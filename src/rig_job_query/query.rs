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
enum US1 {
    US1_Machine(Rc<str>),
}
#[derive(Clone)]
enum US0 {
    US0_Local,
    US0_OnMachine(US1),
}
#[derive(Clone)]
enum US2 {
    US2_Some(US0),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US1),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_JobFinished(i32),
    US4_JobTimedOut,
    US4_JobStopped,
    US4_JobSpawnFailed(Rc<str>),
    US4_JobSkipped(Rc<str>),
    US4_JobLost,
    US4_JobMemoryCapped,
    US4_JobExpiredInQueue,
}
#[derive(Clone)]
enum US6 {
    US6_JobPending,
    US6_JobRunning,
    US6_JobPlaced,
}
#[derive(Clone)]
enum US5 {
    US5_Live(US6),
    US5_Settled(US4),
}
#[derive(Clone)]
enum US7 {
    US7_Some(u64),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_WaitFinished,
    US8_WaitTimedOut,
    US8_WaitCallerGone,
}
fn joined_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_6() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_JOBS_HOME").or_else(|| std::env::var_os("KIT_JOBS_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_6();
    joined_4(v1, v0)
}
fn job_file_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_5(v0);
    joined_4(v2, v1)
}
fn text_or_empty_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn job_placement_2(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placement"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_3(v0, v1);
    let mut v3: Rc<str> = text_or_empty_7(v2);
    let mut v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v5: bool = &*v4 == "";
    let v7: bool = if v5 {
        true
    } else {
        let mut v6: bool = &*v4 == "local";
        v6
    };
    let v35: US2 = if v7 {
        let v8: US0 = US0::US0_Local;
        US2::US2_Some(v8)
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("on "); } LIT.with(|lit| lit.clone()) };
        let v11: bool = v4.starts_with(&*v10);
        if v11 {
            let v12: i32 = (v4.clone().len() as i32);
            let v13: i32 = v12.wrapping_sub(3i32);
            let v14: usize = ((3i32) as usize);
            let v15: i32 = 3i32.wrapping_add(v13);
            let v16: usize = ((v15) as usize);
            let v17: Rc<str> = Rc::<str>::from(&(v4)[v14..v16]);
            let v18: Rc<str> = Rc::<str>::from((v17).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let v19: bool = &*v18 == "";
            let v22: bool = if v19 {
                true
            } else {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v21: bool = v18.contains(&*v20);
                v21
            };
            let v26: US3 = if v22 {
                US3::US3_None
            } else {
                let v24: US1 = US1::US1_Machine(v18);
                US3::US3_Some(v24)
            };
            match v26 {
                US3::US3_None => {
                    US2::US2_None
                }
                US3::US3_Some(v27) => {
                    let v27: US1 = v27;
                    let v28: US0 = US0::US0_OnMachine(v27);
                    US2::US2_Some(v28)
                }
            }
        } else {
            US2::US2_None
        }
    };
    match v35 {
        US2::US2_None => {
            US0::US0_Local
        }
        US2::US2_Some(v36) => {
            let v36: US0 = v36;
            v36
        }
    }
}
fn placed_1(mut v0: Rc<str>, mut v1: Rc<dyn Fn() -> i32>) -> i32 {
    let v2: US0 = job_placement_2(v0);
    match v2 {
        US0::US0_Local => {
            v1()
        }
        US0::US0_OnMachine(v3) => {
            let mut v3: US1 = v3;
            let mut v4: Rc<str> = jobs_root_6();
            let mut v6: Rc<str> = match v3 {
                US1::US1_Machine(v5) => {
                    let v5: Rc<str> = v5;
                    v5
                }
            };
            let v7: i32 = rig_job_dispatch::rig_job_forward_run(&v4, &v6);
            v7
        }
    }
}
fn concat5_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn stderr_line_11(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn rejected_9(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: job "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = concat5_10(v1, v0, v2.clone(), v2.clone(), v2);
    stderr_line_11(v3);
    2i32
}
fn directory_exists_12(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn shared_summary_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = rig_job_summary::rig_job_summary(&*v0, &*v1);
    v2
}
fn stdout_line_14(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn job_status_8(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    let v5: bool = if v2 {
        let mut v3: Rc<str> = job_directory_5(v0.clone());
        directory_exists_12(v3)
    } else {
        false
    };
    if v5 {
        let mut v6: Rc<str> = rig_job_host_request::rig_job_host_revive();
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = shared_summary_13(v7, v0);
        stdout_line_14(v8);
        0i32
    } else {
        let v9: Rc<str> = string_concat(&["status: unknown job id ", &*v0]);
        rejected_9(v9)
    }
}
fn closure1(mut v0: Rc<str>) -> Rc<dyn Fn() -> i32> {
    Rc::new(move || -> i32 {
        job_status_8(v0.clone())
    })
}
fn status_placed_0(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<dyn Fn() -> i32> = closure1(v0.clone());
    placed_1(v0, v1)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        status_placed_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn settled_exit_18(mut v0: US4) -> i32 {
    match v0 {
        US4::US4_JobExpiredInQueue => {
            75i32
        }
        US4::US4_JobFinished(v1) => {
            let mut v1: i32 = v1;
            v1
        }
        US4::US4_JobLost => {
            125i32
        }
        US4::US4_JobMemoryCapped => {
            137i32
        }
        US4::US4_JobSkipped(v3) => {
            let mut v3: Rc<str> = v3;
            125i32
        }
        US4::US4_JobSpawnFailed(v2) => {
            let v2: Rc<str> = v2;
            125i32
        }
        US4::US4_JobStopped => {
            143i32
        }
        US4::US4_JobTimedOut => {
            124i32
        }
    }
}
fn state_exit_17(mut v0: US5) -> i32 {
    match v0 {
        US5::US5_Live(v1) => {
            let mut v1: US6 = v1;
            3i32
        }
        US5::US5_Settled(v2) => {
            let v2: US4 = v2;
            settled_exit_18(v2)
        }
    }
}
fn now_ms_19() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn method21(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure4() -> Rc<dyn Fn((u64)) -> US7> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US7> = Rc::new(move |mut v0: (u64)| -> US7 {
        let mut v1: u64 = (v0);
        US7::US7_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_20(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method21(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US7> = closure4();
    let mut v4: Option<US7> = v2.map(|x| v3(x));
    let mut v6: US7 = US7::US7_None;
    let v7: US7 = v6.clone();
    let v8: US7 = v4.unwrap_or(v7);
    match v8 {
        US7::US7_None => {
            0u64
        }
        US7::US7_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn parent_pid_23() -> u64 {
    let v0: u64 = { #[cfg(windows)] fn parent() -> u64 { #[repr(C)] struct Entry { size: u32, usage: u32, pid: u32, heap: usize, module: u32, threads: u32, parent: u32, base: i32, flags: u32, exe: [u16; 260] } unsafe extern "system" { fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> *mut std::ffi::c_void; fn Process32FirstW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn Process32NextW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } let me = std::process::id(); unsafe { let snapshot = CreateToolhelp32Snapshot(2, 0); if snapshot as isize == -1 { return 0; } let mut entry: Entry = std::mem::zeroed(); entry.size = std::mem::size_of::<Entry>() as u32; let mut found = 0u64; let mut more = Process32FirstW(snapshot, &mut entry) != 0; while more { if entry.pid == me { found = entry.parent as u64; break; } more = Process32NextW(snapshot, &mut entry) != 0; } CloseHandle(snapshot); found } } #[cfg(not(windows))] fn parent() -> u64 { std::os::unix::process::parent_id() as u64 } parent() };
    v0
}
fn watched_caller_22() -> u64 {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JOB_WATCH"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = string_concat(&["RIG_", &*v0]);
    let v2: Rc<str> = std::env::var(&*v1).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
    let mut v3: bool = &*v2 != "";
    let mut v6: Rc<str> = if v3 {
        v2
    } else {
        let mut v4: Rc<str> = string_concat(&["KIT_", &*v0]);
        let v5: Rc<str> = std::env::var(&*v4).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
        v5
    };
    let v7: u64 = decimal_u64_20(v6);
    let v8: bool = v7 == 0u64;
    if v8 {
        parent_pid_23()
    } else {
        v7
    }
}
fn state_of_parts_27(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US5 {
    let mut v3: bool = &*v0 == "pending";
    if v3 {
        let mut v4: US6 = US6::US6_JobPending;
        US5::US5_Live(v4)
    } else {
        let mut v6: bool = &*v0 == "running";
        if v6 {
            let v7: US6 = US6::US6_JobRunning;
            US5::US5_Live(v7)
        } else {
            let v9: bool = &*v0 == "placed";
            if v9 {
                let v10: US6 = US6::US6_JobPlaced;
                US5::US5_Live(v10)
            } else {
                let v12: bool = &*v0 == "finished";
                if v12 {
                    let v13: US4 = US4::US4_JobFinished(v1);
                    US5::US5_Settled(v13)
                } else {
                    let v15: bool = &*v0 == "timed-out";
                    if v15 {
                        let v16: US4 = US4::US4_JobTimedOut;
                        US5::US5_Settled(v16)
                    } else {
                        let v18: bool = &*v0 == "stopped";
                        if v18 {
                            let v19: US4 = US4::US4_JobStopped;
                            US5::US5_Settled(v19)
                        } else {
                            let v21: bool = &*v0 == "lost";
                            if v21 {
                                let v22: US4 = US4::US4_JobLost;
                                US5::US5_Settled(v22)
                            } else {
                                let v24: bool = &*v0 == "memory-capped";
                                if v24 {
                                    let v25: US4 = US4::US4_JobMemoryCapped;
                                    US5::US5_Settled(v25)
                                } else {
                                    let v27: bool = &*v0 == "expired-in-queue";
                                    if v27 {
                                        let v28: US4 = US4::US4_JobExpiredInQueue;
                                        US5::US5_Settled(v28)
                                    } else {
                                        let v30: bool = &*v0 == "skipped";
                                        if v30 {
                                            let v31: US4 = US4::US4_JobSkipped(v2);
                                            US5::US5_Settled(v31)
                                        } else {
                                            let v33: US4 = US4::US4_JobSpawnFailed(v2);
                                            US5::US5_Settled(v33)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn shared_state_26(mut v0: Rc<str>) -> US5 {
    let (mut v1, mut v2, mut v3): (Rc<str>, i32, Rc<str>) = rig_job_state::rig_job_state_parts(&*v0);
    state_of_parts_27(v1, v2, v3)
}
fn state_settled_28(mut v0: US5) -> bool {
    match v0 {
        US5::US5_Live(v1) => {
            let mut v1: US6 = v1;
            false
        }
        US5::US5_Settled(v2) => {
            let v2: US4 = v2;
            true
        }
    }
}
fn process_alive_29(mut v0: u64) -> bool {
    let mut v1: bool = rig_process_alive::rig_process_alive(v0);
    v1
}
fn next_nap_30(mut v0: u64) -> u64 {
    let mut v1: bool = v0 >= 2500u64;
    if v1 {
        5000u64
    } else {
        let v2: u64 = v0.wrapping_mul(2u64);
        v2
    }
}
fn waited_napping_25(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US5 {
    loop {
        let mut v4: US5 = shared_state_26(v0.clone());
        let v5: bool = state_settled_28(v4.clone());
        let v8: bool = if v5 {
            true
        } else {
            let mut v6: u64 = now_ms_19();
            let v7: bool = v6 > v1;
            v7
        };
        let v11: bool = if v8 {
            true
        } else {
            let v9: bool = process_alive_29(v2);
            let v10: bool = v9 == false;
            v10
        };
        if v11 {
            return v4;
        } else {
            std::thread::sleep(std::time::Duration::from_millis(v3));
            let v12: u64 = next_nap_30(v3);
            v3 = v12;
            continue;
        }
    }
}
fn waited_24(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> US5 {
    let mut v3: u64 = 250u64;
    waited_napping_25(v0, v1, v2, v3)
}
fn wait_end_of_31(mut v0: US5, mut v1: u64) -> US8 {
    let v2: bool = state_settled_28(v0);
    if v2 {
        US8::US8_WaitFinished
    } else {
        let mut v4: bool = process_alive_29(v1);
        let v5: bool = v4 == false;
        if v5 {
            US8::US8_WaitCallerGone
        } else {
            US8::US8_WaitTimedOut
        }
    }
}
fn kebab_from_33(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v2 >= v1;
        if v4 {
            return v3;
        } else {
            let v5: usize = ((v2) as usize);
            let mut v6: i32 = v2.wrapping_add(1i32);
            let v7: usize = ((v6) as usize);
            let v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
            let v9: Rc<str> = Rc::<str>::from(v8.to_lowercase());
            let v10: bool = v2 > 0i32;
            let v12: bool = if v10 {
                let v11: bool = &*v8 != &*v9;
                v11
            } else {
                false
            };
            let v15: Rc<str> = if v12 {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                v13
            } else {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v14
            };
            let v17: Rc<str> = string_concat(&[&*v3, &*v15, &*v9]);
            (v2, v3) = (v6, v17);
            continue;
        }
    }
}
fn wait_end_word_32(mut v0: US8) -> Rc<str> {
    let mut v4: Rc<str> = match v0 {
        US8::US8_WaitCallerGone => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CallerGone"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US8::US8_WaitFinished => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Finished"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US8::US8_WaitTimedOut => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TimedOut"); } LIT.with(|lit| lit.clone()) };
            v2
        }
    };
    let v5: i32 = (v4.clone().len() as i32);
    let mut v6: i32 = 0i32;
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    kebab_from_33(v4, v5, v6, v7)
}
fn job_waited_16(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v0.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_5(v0.clone());
        directory_exists_12(v4)
    } else {
        false
    };
    if v6 {
        let v7: Rc<str> = rig_job_host_request::rig_job_host_revive();
        let v8: u64 = now_ms_19();
        let v9: bool = &*v1 == "";
        let v11: u64 = if v9 {
            864000000u64
        } else {
            decimal_u64_20(v1)
        };
        let v12: u64 = v8.wrapping_add(v11);
        let v13: u64 = watched_caller_22();
        let v14: US5 = waited_24(v0.clone(), v12, v13);
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = shared_summary_13(v15, v0);
        let v18: US8 = wait_end_of_31(v14.clone(), v13);
        let v19: Rc<str> = wait_end_word_32(v18);
        let v20: Rc<str> = string_concat(&[&*v16, " end=", &*v19]);
        stdout_line_14(v20);
        state_exit_17(v14)
    } else {
        let v22: Rc<str> = string_concat(&["wait: unknown job id ", &*v0]);
        rejected_9(v22)
    }
}
fn closure3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn() -> i32> {
    Rc::new(move || -> i32 {
        job_waited_16(v1.clone(), v0.clone())
    })
}
fn wait_placed_15(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: Rc<dyn Fn() -> i32> = closure3(v0, v1.clone());
    placed_1(v1, v2)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        wait_placed_15(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let v0: i32 = rig_job_control::rig_job_tail_run();
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn tail_placed_34(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<dyn Fn() -> i32> = closure6();
    placed_1(v0, v1)
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        tail_placed_34(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let v0: i32 = rig_job_control::rig_job_stop_run();
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn stop_placed_35(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<dyn Fn() -> i32> = closure8();
    placed_1(v0, v1)
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        stop_placed_35(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_job_status_placed(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_job_wait_placed(v0: &str, v1: &str) -> i32 {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_job_tail_placed(v0: &str) -> i32 {
    closure5()(Rc::<str>::from(v0))
}
pub fn rig_job_stop_placed(v0: &str) -> i32 {
    closure7()(Rc::<str>::from(v0))
}
