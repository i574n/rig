// Fleet node benchmark — one self-contained exe. CPU (single/all-core) + memory bandwidth +
// local disk (seq write/read, fsync latency, random 4K IOPS) + optional GPU/WGSL.
//
// Two build profiles from ONE source (the "typefeatureset"):
//   lean (default): `rustc -C opt-level=3 -C lto=fat -C codegen-units=1 --edition 2021 src/main.rs -o nodebench`
//                   -- zero deps, offline, seconds, every node. These flags MATTER: plain `rustc -O` is opt-level=2
//                      and the sqrt loop does not vectorize, giving ST ~1.9x slower than the release profile, so
//                      a lean-vs-cargo fleet would rank build flags, not hardware. Must match [profile.release] below.
//   gpu           : `cargo build --release --features gpu` -- pulls wgpu/pollster, adds the GPU line (same opt profile).
//
// Raw metrics only: the exe never ranks or rates. Fleet-relative 0-1000 rating + rank is computed
// centrally by the fleet coordinator (rate.ps1), because that is the only place that sees all nodes.
//
// Design notes (lessons baked in):
//  - ns timing (not truncated ms): fast nodes differ by <1ms, integer ms hid it.
//  - best-of-N (min): a throughput probe wants the least-interfered run, not the mean (killed the earth 311->450 noise).
//  - disk temp dir cleaned via RAII Drop guard: teardown sweeps every layer even on early return.
//  - mem buffer kept small enough not to OOM the smallest (1 GB) node; large-L3 caveat noted in README.

use std::hint::black_box;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::time::Instant;
use std::{fs, thread};

/// Run `f` `reps` times after one warmup; return the fastest elapsed seconds (least interference).
fn best_secs<F: FnMut()>(warmup: bool, reps: u32, mut f: F) -> f64 {
    if warmup {
        f();
    }
    let mut best = f64::INFINITY;
    for _ in 0..reps.max(1) {
        let t = Instant::now();
        f();
        let s = t.elapsed().as_secs_f64();
        if s < best {
            best = s;
        }
    }
    best
}

fn cpu_work(iters: u64) -> f64 {
    let mut s = 0.0f64;
    for i in 0..iters {
        s += (i as f64).sqrt();
    }
    s
}

/// A directory that deletes itself (and everything under it) when dropped — on normal exit OR early return.
struct TempGuard(PathBuf);
impl Drop for TempGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Local disk: sequential write+fsync throughput, a single durable fsync latency, read-back throughput,
/// and random 4K fsync'd write IOPS. Returns (write_mbps, read_mbps, fsync_ms, iops). (-1 on any failure.)
fn disk_probe() -> (f64, f64, f64, f64) {
    let dir = std::env::temp_dir().join(format!("nodebench-{}", std::process::id()));
    if fs::create_dir_all(&dir).is_err() {
        return (-1.0, -1.0, -1.0, -1.0);
    }
    let _guard = TempGuard(dir.clone());
    let path = dir.join("blob.bin");

    let file_mb: usize = 256; // trivial even on 25 GB and 32 GB disks
    let chunk = vec![0xA5u8; 1024 * 1024];

    // --- sequential write + final fsync ---
    let write_secs;
    {
        let mut f = match fs::File::create(&path) {
            Ok(f) => f,
            Err(_) => return (-1.0, -1.0, -1.0, -1.0),
        };
        let t = Instant::now();
        for _ in 0..file_mb {
            if f.write_all(&chunk).is_err() {
                return (-1.0, -1.0, -1.0, -1.0);
            }
        }
        let _ = f.sync_all(); // durability barrier is part of a storage write
        write_secs = t.elapsed().as_secs_f64();
    }
    let write_mbps = file_mb as f64 / write_secs;

    // --- durable fsync latency: median of 32 (4K append + sync_all). A writeback-cached VM returns ~0 here
    //     (fsync not honored durably) -> the central rating clamps that so cache can't masquerade as storage. ---
    let fsync_ms;
    {
        let mut f = fs::OpenOptions::new().append(true).open(&path).unwrap();
        let mut s = Vec::with_capacity(32);
        for _ in 0..32 {
            let _ = f.write_all(&[0u8; 4096]);
            let t = Instant::now();
            let _ = f.sync_all();
            s.push(t.elapsed().as_secs_f64() * 1000.0);
        }
        s.sort_by(|a, b| a.partial_cmp(b).unwrap());
        fsync_ms = s[s.len() / 2];
    }

    // --- sequential read-back (cache-warm; see README caveat) ---
    let read_secs;
    {
        let mut f = fs::File::open(&path).unwrap();
        let mut buf = vec![0u8; 1024 * 1024];
        let t = Instant::now();
        loop {
            match f.read(&mut buf) {
                Ok(0) => break,
                Ok(_) => {
                    black_box(&buf[0]);
                }
                Err(_) => break,
            }
        }
        read_secs = t.elapsed().as_secs_f64();
    }
    let read_mbps = (file_mb as f64 + 0.004) / read_secs;

    // --- random 4K fsync'd write IOPS (durable small-write rate) ---
    let iops;
    {
        let mut f = fs::OpenOptions::new().write(true).open(&path).unwrap();
        let blocks = (file_mb * 1024 / 4) as u64; // number of 4K slots in the file
        let small = [0x5Au8; 4096];
        let ops = 128u64;
        let mut lcg: u64 = 0x9E3779B97F4A7C15; // deterministic offset stream, no rand dep
        let t = Instant::now();
        for _ in 0..ops {
            lcg = lcg.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let slot = lcg % blocks;
            if f.seek(SeekFrom::Start(slot * 4096)).is_err() {
                break;
            }
            let _ = f.write_all(&small);
            let _ = f.sync_all();
        }
        let s = t.elapsed().as_secs_f64();
        iops = ops as f64 / s;
    }

    (write_mbps, read_mbps, fsync_ms, iops)
}

#[cfg(feature = "gpu")]
fn gpu_probe() -> String {
    let instance = wgpu::Instance::default();
    let adapters = instance.enumerate_adapters(wgpu::Backends::all());
    if adapters.is_empty() {
        return "GPU_CLASS=none".to_string();
    }
    let rank = |d: wgpu::DeviceType| match d {
        wgpu::DeviceType::DiscreteGpu => 4,
        wgpu::DeviceType::IntegratedGpu => 3,
        wgpu::DeviceType::VirtualGpu => 2,
        wgpu::DeviceType::Cpu => 1,
        wgpu::DeviceType::Other => 0,
    };
    let best = adapters
        .iter()
        .max_by_key(|a| rank(a.get_info().device_type))
        .unwrap();
    let info = best.get_info();
    let class = match info.device_type {
        wgpu::DeviceType::DiscreteGpu => "discrete",
        wgpu::DeviceType::IntegratedGpu => "integrated",
        wgpu::DeviceType::VirtualGpu => "virtual",
        wgpu::DeviceType::Cpu => "software",
        wgpu::DeviceType::Other => "other",
    };
    let gpu_ms = pollster::block_on(gpu_compute(best)).unwrap_or(-1.0);
    format!(
        "GPU_CLASS={} GPU_BACKEND={:?} GPU_MS={:.2} GPU_NAME=\"{}\"",
        class, info.backend, gpu_ms, info.name
    )
}

#[cfg(feature = "gpu")]
async fn gpu_compute(adapter: &wgpu::Adapter) -> Option<f64> {
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .ok()?;
    let n: u32 = 1 << 20;
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(
            r#"
@group(0) @binding(0) var<storage, read_write> data: array<f32>;
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    var x = f32(i);
    for (var k = 0u; k < 256u; k = k + 1u) { x = sqrt(x * 1.0001 + 1.0); }
    data[i] = x;
}
"#
            .into(),
        ),
    });
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (n as u64) * 4,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buf.as_entire_binding(),
        }],
    });
    let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bgl],
        push_constant_ranges: &[],
    });
    let pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&pl),
        module: &shader,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });
    let reps = 50;
    let t = Instant::now();
    for _ in 0..reps {
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut cp = enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            cp.set_pipeline(&pipe);
            cp.set_bind_group(0, &bg, &[]);
            cp.dispatch_workgroups(n / 64, 1, 1);
        }
        queue.submit(Some(enc.finish()));
    }
    device.poll(wgpu::Maintain::Wait);
    Some(t.elapsed().as_secs_f64() * 1000.0)
}

#[cfg(not(feature = "gpu"))]
fn gpu_probe() -> String {
    "GPU_CLASS=skip".to_string()
}

fn main() {
    let iters: u64 = 50_000_000;

    // single-thread: best of 3 (+warmup)
    let st_ms = best_secs(true, 3, || {
        black_box(cpu_work(black_box(iters)));
    }) * 1000.0;

    // all-core: n threads each doing full work, best of 3
    let n = thread::available_parallelism().map(|x| x.get()).unwrap_or(1);
    let mt_ms = best_secs(true, 3, || {
        let h: Vec<_> = (0..n)
            .map(|_| thread::spawn(move || black_box(cpu_work(black_box(iters)))))
            .collect();
        for x in h {
            let _ = x.join();
        }
    }) * 1000.0;

    // memory bandwidth: 64MB copy, best of 5 (working set 128MB — safe on the 1GB node)
    let size = 64usize * 1024 * 1024;
    let src = vec![1u8; size];
    let mut dst = vec![0u8; size];
    let reps = 20u64;
    let mem_secs = best_secs(true, 5, || {
        for _ in 0..reps {
            dst.copy_from_slice(black_box(&src));
            black_box(&dst[0]);
        }
    });
    let mem_gbps = (size as f64 * reps as f64) / mem_secs / 1e9;

    let (dwr, drd, dsync, diops) = disk_probe();
    let gpu = gpu_probe();

    println!(
        "CORES={} ST_MS={:.2} MT_MS={:.2} MEM_GBPS={:.2} DWR_MBPS={:.1} DRD_MBPS={:.1} DSYNC_MS={:.3} DIOPS={:.0} {}",
        n, st_ms, mt_ms, mem_gbps, dwr, drd, dsync, diops, gpu
    );
}
