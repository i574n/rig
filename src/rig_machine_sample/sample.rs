#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_concat(parts: &[&str]) -> Rc<str> {
    let mut joined = Rc::<[u8]>::new_uninit_slice(parts.iter().map(|part| part.len()).sum());
    let bytes = Rc::get_mut(&mut joined).unwrap().as_mut_ptr() as *mut u8;
    let mut at = 0;
    for part in parts {
        unsafe { std::ptr::copy_nonoverlapping(part.as_ptr(), bytes.add(at), part.len()) };
        at += part.len();
    }
    unsafe { Rc::from_raw(Rc::into_raw(joined.assume_init()) as *const str) }
}
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, u64, bool, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(u64, Rc<UH2>),
}
fn live_parent_2(mut v0: std::rc::Rc<std::cell::RefCell<sysinfo::System>>, mut v1: u32, mut v2: u64) -> u32 {
    let v3: u32 = v0.borrow().process(sysinfo::Pid::from_u32(v1)).and_then(sysinfo::Process::parent).map_or(0, sysinfo::Pid::as_u32);
    let mut v4: u64 = v0.borrow().process(sysinfo::Pid::from_u32(v1)).map_or(0, sysinfo::Process::start_time);
    let v5: u64 = v0.borrow().process(sysinfo::Pid::from_u32(v3)).map_or(0, sysinfo::Process::start_time);
    let mut v6: bool = v3 <= 4u32;
    let v8: bool = if v6 {
        true
    } else {
        let v7: bool = v4 == 0u64;
        v7
    };
    let mut v10: bool = if v8 {
        true
    } else {
        let mut v9: bool = v5 == 0u64;
        v9
    };
    let v12: bool = if v10 {
        true
    } else {
        let v11: bool = v5 <= v4;
        v11
    };
    let mut v14: bool = if v12 {
        true
    } else {
        let v13: bool = v5 > v2;
        v13
    };
    if v14 {
        v3
    } else {
        0u32
    }
}
fn process_private_bytes_3(mut v0: u64) -> u64 {
    let mut v1: u64 = rig_os_launch::rig_process_private_bytes(v0);
    v1
}
fn rows_from_1(mut v0: std::rc::Rc<std::cell::RefCell<sysinfo::System>>, mut v1: std::rc::Rc<Vec<u32>>, mut v2: std::rc::Rc<std::cell::RefCell<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>>, mut v3: std::rc::Rc<std::collections::HashMap<u32, u64>>, mut v4: i32, mut v5: i32, mut v6: f64) -> () {
    loop {
        let v7: bool = v4 < v5;
        if v7 {
            let v8: usize = ((v4) as usize);
            let mut v9: u32 = v1[v8];
            let mut v10: u64 = v0.borrow().process(sysinfo::Pid::from_u32(v9)).map_or(0, sysinfo::Process::accumulated_cpu_time);
            let v11: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_secs());
            let v12: u64 = (v9 as u64);
            let v13: u32 = live_parent_2(v0.clone(), v9, v11);
            let mut v14: u64 = (v13 as u64);
            let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.borrow().process(sysinfo::Pid::from_u32(v9)).map(|process| process.name().to_string_lossy().trim_end_matches(".exe").to_owned()).unwrap_or_default());
            let v16: Rc<str> = std::rc::Rc::<str>::from(v0.borrow().process(sysinfo::Pid::from_u32(v9)).map(|process| process.cmd().iter().map(|part| part.to_string_lossy().into_owned()).collect::<Vec<_>>().join(" ")).unwrap_or_default());
            let v17: u64 = v3.get(&v9).copied().unwrap_or(0);
            let v18: bool = v10 > v17;
            let v20: bool = if v18 {
                let v19: bool = v6 > 0.0f64;
                v19
            } else {
                false
            };
            let v25: f64 = if v20 {
                let v21: u64 = v10.wrapping_sub(v17);
                let v22: f64 = (v21 as f64);
                let v23: f64 = v6 * 1000.0f64;
                let v24: f64 = v22 / v23;
                v24
            } else {
                0.0f64
            };
            let mut v26: u64 = v0.borrow().process(sysinfo::Pid::from_u32(v9)).map_or(0, sysinfo::Process::run_time);
            let v27: u64 = v26.wrapping_div(60u64);
            let v28: u64 = v10.wrapping_div(60000u64);
            let mut v29: u64 = v0.borrow().process(sysinfo::Pid::from_u32(v9)).map_or(0, sysinfo::Process::memory);
            let v30: u64 = v29.wrapping_div(1048576u64);
            let v31: u64 = process_private_bytes_3(v12);
            let v32: u64 = v31.wrapping_div(1048576u64);
            v2.borrow_mut().push((v12, v14, v15, v16, v25, v27, v28, v30, v32));
            let mut v33: i32 = v4.wrapping_add(1i32);
            v4 = v33;
            continue;
        }
        return ();
    }
}
fn rows_sampled_0(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> {
    let mut v1: sysinfo::ProcessRefreshKind = sysinfo::ProcessRefreshKind::nothing().with_cpu().with_memory().with_cmd(sysinfo::UpdateKind::Always);
    let mut v2: std::rc::Rc<std::cell::RefCell<sysinfo::System>> = std::rc::Rc::new(std::cell::RefCell::new(sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(v1.clone()))));
    let v3: std::rc::Rc<std::collections::HashMap<u32, u64>> = std::rc::Rc::new(v2.borrow().processes().iter().map(|(pid, process)| (pid.as_u32(), process.accumulated_cpu_time())).collect());
    let mut v4: std::time::Instant = std::time::Instant::now();
    std::thread::sleep(std::time::Duration::from_millis(v0));
    let v5: usize = v2.borrow_mut().refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, true, v1);
    let mut v6: f64 = (v4).elapsed().as_secs_f64();
    let v7: std::rc::Rc<Vec<u32>> = std::rc::Rc::new(v2.borrow().processes().keys().copied().map(sysinfo::Pid::as_u32).collect::<Vec<_>>());
    let v8: std::rc::Rc<std::cell::RefCell<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let mut v9: i32 = 0i32;
    let mut v10: usize = v7.len();
    let v11: i32 = ((v10) as i32);
    rows_from_1(v2, v7, v8.clone(), v3, v9, v11, v6);
    let v12: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = std::rc::Rc::new(v8.borrow().clone());
    v12
}
fn closure0() -> Rc<dyn Fn(u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>> = Rc::new(move |mut v0: u64| -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> {
        rows_sampled_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn cells_vm_command_4(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((&*v0).to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.notebook"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v1.contains(&*v2);
    let mut v6: bool = if v3 {
        true
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral-cells"); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v1.contains(&*v4);
        v5
    };
    let mut v9: bool = if v6 {
        true
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-build"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v1.contains(&*v7);
        v8
    };
    if v9 {
        true
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.cells.compilerdaemon"); } LIT.with(|lit| lit.clone()) };
        let v11: bool = v1.contains(&*v10);
        v11
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        cells_vm_command_4(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn joined_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_10() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_JOBS_HOME").or_else(|| std::env::var_os("KIT_JOBS_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_9(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_10();
    joined_8(v1, v0)
}
fn job_file_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_9(v0);
    joined_8(v2, v1)
}
fn text_or_empty_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method12(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure3() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn job_left_from_6(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: u64, mut v3: u64) -> Rc<str> {
    loop {
        let mut v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v6;
        } else {
            let v7: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = job_file_7(v7.clone(), v8);
            let mut v10: Rc<str> = text_or_empty_11(v9);
            let v11: Rc<str> = Rc::<str>::from((v10).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let v12: Option<u64> = (v11).trim().parse::<u64>().ok();
            let v13: Option<u64> = method12(v12);
            let mut v14: Rc<dyn Fn((u64)) -> US0> = closure3();
            let mut v15: Option<US0> = v13.map(|x| v14(x));
            let v17: US0 = US0::US0_None;
            let v18: US0 = v17.clone();
            let v19: US0 = v15.unwrap_or(v18);
            let v22: u64 = match v19 {
                US0::US0_None => {
                    0u64
                }
                US0::US0_Some(v20) => {
                    let v20: u64 = v20;
                    v20
                }
            };
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = job_file_7(v7.clone(), v23);
            let v25: Rc<str> = text_or_empty_11(v24);
            let mut v26: Rc<str> = Rc::<str>::from((v25).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let v27: Option<u64> = (v26).trim().parse::<u64>().ok();
            let v28: Option<u64> = method12(v27);
            let mut v29: Option<US0> = v28.map(|x| v14(x));
            let v31: US0 = US0::US0_None;
            let v32: US0 = v31.clone();
            let mut v33: US0 = v29.unwrap_or(v32);
            let mut v36: u64 = match v33 {
                US0::US0_None => {
                    0u64
                }
                US0::US0_Some(v34) => {
                    let v34: u64 = v34;
                    v34
                }
            };
            let mut v37: bool = &*v7 != "";
            let v41: bool = if v37 {
                let v38: bool = v22 == v2;
                if v38 {
                    true
                } else {
                    let mut v39: bool = v36 == v2;
                    v39
                }
            } else {
                false
            };
            if v41 {
                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                let v43: Rc<str> = job_file_7(v7.clone(), v42);
                let v44: Rc<str> = text_or_empty_11(v43);
                let v45: Rc<str> = Rc::<str>::from((v44).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v46: bool = &*v45 != "";
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
                let v48: Rc<str> = job_file_7(v7.clone(), v47);
                let v49: Rc<str> = text_or_empty_11(v48);
                let v50: Rc<str> = Rc::<str>::from((v49).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v51: Option<u64> = (v50).trim().parse::<u64>().ok();
                let v52: Option<u64> = method12(v51);
                let mut v53: Option<US0> = v52.map(|x| v14(x));
                let v55: US0 = US0::US0_None;
                let v56: US0 = v55.clone();
                let v57: US0 = v53.unwrap_or(v56);
                let v60: u64 = match v57 {
                    US0::US0_None => {
                        0u64
                    }
                    US0::US0_Some(v58) => {
                        let v58: u64 = v58;
                        v58
                    }
                };
                let v62: bool = if v46 {
                    let v61: bool = v2 > 4u64;
                    v61
                } else {
                    false
                };
                let v66: bool = if v62 {
                    let v63: bool = v2 == v22;
                    if v63 {
                        true
                    } else {
                        let v64: bool = v2 == v36;
                        v64
                    }
                } else {
                    false
                };
                let v69: bool = if v66 {
                    let v67: u64 = v60.wrapping_div(1000u64);
                    let v68: bool = v3 >= v67;
                    v68
                } else {
                    false
                };
                if v69 {
                    return v7;
                } else {
                    let v70: i32 = v1.wrapping_add(1i32);
                    v1 = v70;
                    continue;
                }
            } else {
                let v73: i32 = v1.wrapping_add(1i32);
                v1 = v73;
                continue;
            }
        }
    }
}
fn job_left_5(mut v0: u64, mut v1: u64) -> Rc<str> {
    let mut v2: bool = v0 <= 4u64;
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let mut v4: Rc<str> = jobs_root_10();
        let v5: u64 = 100u64;
        let mut v6: Rc<str> = { let mut names = std::fs::read_dir(&*v4).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with("job-")).collect::<Vec<_>>()).unwrap_or_default(); names.sort(); names.reverse(); names.truncate(v5 as usize); std::rc::Rc::<str>::from(names.join("
")) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v6.split(&*v7).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v9: i32 = 0i32;
        let mut v10: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_secs());
        let v11: u64 = v1.wrapping_mul(60u64);
        let v12: u64 = v10.wrapping_sub(v11);
        job_left_from_6(v8, v9, v0, v12)
    }
}
fn closure2() -> Rc<dyn Fn(u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, u64) -> Rc<str>> = Rc::new(move |mut v0: u64, mut v1: u64| -> Rc<str> {
        job_left_5(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn job_seen_of_15(mut v0: Rc<str>, mut v1: u64) -> (Rc<str>, Rc<str>, u64, bool) {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = job_file_7(v0.clone(), v2);
    let mut v4: Rc<str> = text_or_empty_11(v3);
    let v5: Rc<str> = Rc::<str>::from((v4).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v6: Rc<str> = Rc::<str>::from((v5).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v7: Option<u64> = (v6).trim().parse::<u64>().ok();
    let v8: Option<u64> = method12(v7);
    let mut v9: Rc<dyn Fn((u64)) -> US0> = closure3();
    let mut v10: Option<US0> = v8.map(|x| v9(x));
    let v12: US0 = US0::US0_None;
    let v13: US0 = v12.clone();
    let mut v14: US0 = v10.unwrap_or(v13);
    let v17: u64 = match v14 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v15) => {
            let mut v15: u64 = v15;
            v15
        }
    };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = job_file_7(v0.clone(), v18);
    let v20: Rc<str> = text_or_empty_11(v19);
    let v21: Rc<str> = Rc::<str>::from((v20).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v21.split(&*v22).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v24: Rc<RefCell<Vec<Rc<str>>>> = v23 ;
    let v25: i32 = (v24.clone().borrow().len() as i32);
    let mut v26: bool = 1i32 < v25;
    let v30: US1 = if v26 {
        let v27: Rc<str> = v24.borrow()[1i32 as usize].clone();
        US1::US1_Some(v27)
    } else {
        US1::US1_None
    };
    let v34: Rc<str> = match v30 {
        US1::US1_None => {
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v32
        }
        US1::US1_Some(v31) => {
            let v31: Rc<str> = v31;
            v31
        }
    };
    let v35: bool = &*v21 == "";
    let v47: u64 = if v35 {
        v1
    } else {
        let mut v36: Rc<str> = Rc::<str>::from((v34).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        let mut v37: Option<u64> = (v36).trim().parse::<u64>().ok();
        let v38: Option<u64> = method12(v37);
        let mut v39: Option<US0> = v38.map(|x| v9(x));
        let v41: US0 = US0::US0_None;
        let v42: US0 = v41.clone();
        let v43: US0 = v39.unwrap_or(v42);
        match v43 {
            US0::US0_None => {
                0u64
            }
            US0::US0_Some(v44) => {
                let v44: u64 = v44;
                v44
            }
        }
    };
    let v48: bool = v47 > v17;
    let v50: u64 = if v48 {
        let v49: u64 = v47.wrapping_sub(v17);
        v49
    } else {
        0u64
    };
    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
    let v52: Rc<str> = job_file_7(v0.clone(), v51);
    let mut v53: Rc<str> = text_or_empty_11(v52);
    let v54: Rc<str> = Rc::<str>::from((v53).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v55: bool = &*v21 != "";
    (v0, v54, v50, v55)
}
fn jobs_seen_14(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: u64, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            return v3;
        } else {
            let mut v6: Rc<str> = v0.borrow()[v1 as usize].clone();
            let (mut v7, mut v8, mut v9, mut v10): (Rc<str>, Rc<str>, u64, bool) = job_seen_of_15(v6, v2);
            let v11: i32 = v1.wrapping_add(1i32);
            let v12: bool = &*v8 == "";
            let mut v14: Rc<UH0> = if v12 {
                v3
            } else {
                Rc::new(UH0::UH0_Cons(v7, v8, v9, v10, v3))
            };
            (v1, v3) = (v11, v14);
            continue;
        }
    }
}
fn method18(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let v5: u64 = v5;
                let mut v6: bool = v6;
                let v7: Rc<UH0> = v7;
                let mut v9: bool = if v6 {
                    let v8: bool = &*v4 == &*v0;
                    v8
                } else {
                    false
                };
                let v11: Rc<UH2> = if v9 {
                    Rc::new(UH2::UH2_Cons(v5, v2))
                } else {
                    v2
                };
                (v1, v2) = (v7, v11);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method20(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: u64 = v2;
                let v3: Rc<UH2> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn nth_of_22(mut v0: Rc<UH2>, mut v1: i32) -> u64 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: u64 = v2;
                let v3: Rc<UH2> = v3;
                let mut v4: bool = v1 <= 0i32;
                if v4 {
                    return v2;
                } else {
                    let v5: i32 = v1.wrapping_sub(1i32);
                    (v0, v1) = (v3, v5);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return 0u64;
            }
        }
    }
}
fn sorted_in_24(mut v0: u64, mut v1: Rc<UH2>) -> Rc<UH2> {
    match &*v1 {
        UH2::UH2_Cons(v4, v5) => {
            let mut v4: u64 = *v4;
            let v5: Rc<UH2> = v5.clone();
            let mut v6: bool = v0 <= v4;
            if v6 {
                Rc::new(UH2::UH2_Cons(v0, v1))
            } else {
                let v8: Rc<UH2> = sorted_in_24(v0, v5);
                Rc::new(UH2::UH2_Cons(v4, v8))
            }
        }
        UH2::UH2_Nil => {
            let mut v2: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_Cons(v0, v2))
        }
    }
}
fn method23(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: u64 = v2;
                let v3: Rc<UH2> = v3;
                let mut v4: Rc<UH2> = sorted_in_24(v2, v1);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn usual_ms_21(mut v0: Rc<UH2>) -> u64 {
    let mut v1: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH2> = method23(v0.clone(), v1);
    let v3: i32 = 0i32;
    let mut v4: i32 = method20(v0, v3);
    let v5: i32 = v4.wrapping_div(2i32);
    nth_of_22(v2, v5)
}
fn stalled_by_19(mut v0: u64, mut v1: Rc<UH2>) -> bool {
    let mut v2: i32 = 0i32;
    let v3: i32 = method20(v1.clone(), v2);
    let mut v4: bool = v3 >= 3i32;
    if v4 {
        let v5: u64 = usual_ms_21(v1);
        let mut v6: u64 = 2u64.wrapping_mul(v5);
        let v7: bool = v0 > v6;
        v7
    } else {
        false
    }
}
fn stalled_step_17(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: bool) -> Rc<UH1> {
    let mut v6: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH2> = method18(v3, v0, v6);
    let mut v10: bool = if v5 {
        true
    } else {
        let v8: bool = stalled_by_19(v4, v7.clone());
        let mut v9: bool = v8 == false;
        v9
    };
    if v10 {
        v1
    } else {
        let v11: i32 = 0i32;
        let v12: i32 = method20(v7.clone(), v11);
        let v13: Rc<str> = std::rc::Rc::<str>::from((v12).to_string());
        let mut v14: u64 = usual_ms_21(v7);
        let mut v15: u64 = v14.wrapping_div(60000u64);
        let v16: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
        let v19: u64 = v4.wrapping_div(60000u64);
        let v20: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
        let mut v26: Rc<str> = string_concat(&["  job ", &*v2, " running ", &*v20, " min, usual ", &*v16, " min over ", &*v13, " runs  stalled"]);
        Rc::new(UH1::UH1_Cons(v26, v1))
    }
}
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let v5: u64 = v5;
                let mut v6: bool = v6;
                let v7: Rc<UH0> = v7;
                let v8: Rc<UH1> = stalled_step_17(v0.clone(), v2, v3, v4, v5, v6);
                (v1, v2) = (v7, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method25(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH1> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn stalled_jobs_13() -> Rc<str> {
    let mut v0: Rc<str> = jobs_root_10();
    let mut v1: u64 = 300u64;
    let mut v2: Rc<str> = { let mut names = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with("job-")).collect::<Vec<_>>()).unwrap_or_default(); names.sort(); names.reverse(); names.truncate(v1 as usize); std::rc::Rc::<str>::from(names.join("
")) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v2.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v5: i32 = 0i32;
    let mut v6: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_secs());
    let v7: u64 = v6.wrapping_mul(1000u64);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = jobs_seen_14(&v4, v5, v7, v8);
    let mut v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH1> = method16(v9.clone(), v9.clone(), v10);
    let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v13: i32 = 0i32;
    let mut v14: i32 = method25(v12.clone(), v11, v13);
    let mut v15: Rc<Vec<Rc<str>>> = Rc::new(v12.borrow().clone());
    let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
    let v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v3));
    v17
}
fn closure4() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        stalled_jobs_13()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn get_environment_variable_27(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn file_exists_28(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn history_appended_26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = get_environment_variable_27(v1);
    let v3: Rc<str> = string_concat(&[&*v2, "\\kit\\machine-watch"]);
    let mut v4: Rc<str> = string_concat(&[&*v3, "\\health.tsv"]);
    let mut v6: Result<(), std::io::Error> = std::fs::create_dir_all(&*v3);
    let v7: bool = file_exists_28(v4.clone());
    let v23: Rc<str> = if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v8
    } else {
        let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unix"); } LIT.with(|lit| lit.clone()) };
        v9.borrow_mut().push(v10);
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tools"); } LIT.with(|lit| lit.clone()) };
        v9.borrow_mut().push(v11);
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        v9.borrow_mut().push(v12);
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unowned"); } LIT.with(|lit| lit.clone()) };
        v9.borrow_mut().push(v13);
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dangling"); } LIT.with(|lit| lit.clone()) };
        v9.borrow_mut().push(v14);
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("leftover"); } LIT.with(|lit| lit.clone()) };
        v9.borrow_mut().push(v15);
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hogs"); } LIT.with(|lit| lit.clone()) };
        v9.borrow_mut().push(v16);
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stalled"); } LIT.with(|lit| lit.clone()) };
        v9.borrow_mut().push(v17);
        let v18: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
        let v19: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v18).as_ref().clone()));
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = Rc::<str>::from((*v19.borrow()).join(&*v20));
        let v22: Rc<str> = string_concat(&[&*v21, "\n"]);
        v22
    };
    let v24: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_secs());
    let v25: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
    let mut v26: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v26.borrow_mut().push(v25);
    v26.borrow_mut().push(v0);
    let v27: Rc<Vec<Rc<str>>> = Rc::new(v26.borrow().clone());
    let v28: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v27).as_ref().clone()));
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = Rc::<str>::from((*v28.borrow()).join(&*v29));
    let v32: Rc<str> = string_concat(&[&*v23, &*v30, "\n"]);
    let mut v33: std::io::Result<std::fs::File> = std::fs::OpenOptions::new().create(true).append(true).open(&*v4);
    let v34: bool = v33.is_ok();
    let v35: bool = v34 == false;
    if v35 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the file could not be opened"); } LIT.with(|lit| lit.clone()) }))
    };
    let mut v36: std::fs::File = v33.unwrap();
    let mut v37: std::io::Result<()> = std::io::Write::write_all(&mut v36, (v32).as_bytes());
    let v38: bool = v37.is_ok();
    let mut v39: bool = v38 == false;
    if v39 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the write failed"); } LIT.with(|lit| lit.clone()) }))
    };
    v4
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        history_appended_26(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_machine_sampled(v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> {
    closure0()(v0)
}
pub fn rig_machine_cells_vm(v0: &str) -> bool {
    closure1()(Rc::<str>::from(v0))
}
pub fn rig_machine_job_left(v0: u64, v1: u64) -> Rc<str> {
    closure2()(v0, v1)
}
pub fn rig_machine_stalled_jobs() -> Rc<str> {
    closure4()()
}
pub fn rig_machine_history_appended(v0: &str) -> Rc<str> {
    closure5()(Rc::<str>::from(v0))
}
