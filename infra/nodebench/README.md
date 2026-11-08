# nodebench

One self-contained Rust exe that measures a node's raw compute/memory/disk (and optionally GPU) and
prints a single key=value line. It never ranks or rates — the fleet-relative **0-1000 rating + rank** is
computed centrally by the fleet coordinator (its copy of `rate.ps1`), the only place that sees all nodes.

## Build (two profiles from one source — the "typefeatureset")

| profile | command | deps | where |
|---|---|---|---|
| **lean** (default) | `rustc -C opt-level=3 -C lto=fat -C codegen-units=1 --edition 2021 src/main.rs -o nodebench` | none | every node — offline, seconds to build (used on the no-internet 245 and the deep-relay nodes) |
| **gpu** | `cargo build --release --features gpu` | wgpu, pollster | nodes where the GPU line matters (real-GPU: box/moon; or to classify) |

The lean path compiles under plain `rustc` because the GPU code is behind `#[cfg(feature = "gpu")]`,
which is simply inactive without cargo features. No cargo, no registry, no network required for lean.

**The lean flags are not optional.** Plain `rustc -O` is `opt-level=2`; the ST/MT sqrt loop vectorizes only at
`opt-level=3`, so `-O` measured **~1.9× slower** (ST 279 vs 146 on the same box). Lean must use the same
`opt-level=3 + lto=fat + codegen-units=1` as `[profile.release]`, or a fleet mixing lean- and cargo-built numbers
ranks the build flags instead of the hardware.

## Output line

```
CORES=N ST_MS=.. MT_MS=.. MEM_GBPS=.. DWR_MBPS=.. DRD_MBPS=.. DSYNC_MS=.. DIOPS=.. GPU_CLASS=.. [GPU_BACKEND=.. GPU_MS=.. GPU_NAME=".."]
```

| key | meaning | statistic |
|---|---|---|
| `CORES` | logical CPUs (`available_parallelism`) | — |
| `ST_MS` | single-thread: 50M `sqrt` accumulate | **best of 3** (+warmup), ns-timed |
| `MT_MS` | all-core: `CORES` threads each doing the full ST work | best of 3 |
| `MEM_GBPS` | 64MB `copy_from_slice` × 20, GB/s | best of 5 |
| `DWR_MBPS` | sequential write of 256MB + `sync_all` (durable), MB/s | single clean run |
| `DRD_MBPS` | sequential read-back, MB/s (**cache-warm** — see caveat) | single |
| `DSYNC_MS` | latency of one durable `fsync` (4K append + `sync_all`) | single |
| `DIOPS` | random 4K writes each `fsync`'d, ops/sec (durable small-write rate) | 128 ops |
| `GPU_CLASS` | `discrete` / `integrated` / `virtual` / `software` / `none` / `skip` (lean build) | — |
| `GPU_MS` | 50× WGSL compute dispatch (256-iter sqrt over 1M elems), ms | — |

### Caveats / edge cases handled
- **ST/MT/MEM use best-of-N (min)** — a throughput probe wants the least-interfered sample; the mean
  absorbed Defender/scheduler noise (an earth read swung 311→450ms as a mean, stable as a min).
- **ns timing** throughout — integer `as_millis()` hid sub-ms gaps between fast nodes (89 vs 87).
- **mem buffer is 64MB** (128MB working set) so it never OOMs the smallest (1 GB) node. On a huge-L3 node (the largest, 28-core
  one) part of this sits in cache, so `MEM_GBPS` there reads high — read it as "cache+RAM", not pure DRAM.
- **disk temp dir is RAII-cleaned** (`TempGuard: Drop`) — removed on normal exit *and* early return, so a
  failed probe leaves nothing behind (teardown-sweeps-every-layer).
- **`DRD_MBPS` is cache-warm** (the file was just written). Portable cache-drop/O_DIRECT isn't available on
  all nodes, so read-back over-reads on nodes with spare RAM. `DWR_MBPS` + `DSYNC_MS` (both durable) are the
  trustworthy disk numbers; treat read as an upper bound.

## Remote storage — dogfooding, gated
`DWR/DSYNC/DIOPS` are the local-disk analogues of what the object store
pays per commit. The remote PUT/GET/latency probe is **not** in this exe (it needs S3
creds + network egress); it runs only where object-store credentials are configured, via the storage lane's
tooling, and its numbers fold into the disk dimension for those nodes only. See STORE-NO-DANGLING (team:management).

## Rating (computed centrally, not here)
See the fleet coordinator's copy of `rate.ps1`. Each dimension is scored **log-relative** to the fleet (fleet-min→0, fleet-max→1000
on a `ln` axis, so a fixed *ratio* is a fixed number of points), then blended:
`parallel 0.28 · single 0.24 · mem 0.18 · disk 0.22 · gpu 0.08` (gpu weight redistributes when no hardware GPU).
Disk is `0.5·DWR + 0.3·DIOPS + 0.2·(1/DSYNC)` before the log step. The overall 0-1000 + rank is stored at the
fleet coordinator and printed next to the node's name and encoding.
