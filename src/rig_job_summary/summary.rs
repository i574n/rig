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
    US1_JobPending,
    US1_JobRunning,
    US1_JobPlaced,
}
#[derive(Clone)]
enum US2 {
    US2_JobFinished(i32),
    US2_JobTimedOut,
    US2_JobStopped,
    US2_JobSpawnFailed(Rc<str>),
    US2_JobSkipped(Rc<str>),
    US2_JobLost,
    US2_JobMemoryCapped,
    US2_JobExpiredInQueue,
}
#[derive(Clone)]
enum US0 {
    US0_Live(US1),
    US0_Settled(US2),
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_NotWaiting,
    US4_WaitingAfter(Rc<str>),
    US4_WaitingWindow(Rc<str>, Rc<str>),
    US4_WaitingSlot,
}
#[derive(Clone)]
enum US6 {
    US6_Machine(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_Local,
    US5_OnMachine(US6),
}
#[derive(Clone)]
enum US7 {
    US7_Some(US5),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_Some(US6),
    US8_None,
}
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn state_of_parts_5(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US0 {
    let mut v3: bool = &*v0 == "pending";
    if v3 {
        let mut v4: US1 = US1::US1_JobPending;
        US0::US0_Live(v4)
    } else {
        let mut v6: bool = &*v0 == "running";
        if v6 {
            let v7: US1 = US1::US1_JobRunning;
            US0::US0_Live(v7)
        } else {
            let v9: bool = &*v0 == "placed";
            if v9 {
                let v10: US1 = US1::US1_JobPlaced;
                US0::US0_Live(v10)
            } else {
                let v12: bool = &*v0 == "finished";
                if v12 {
                    let v13: US2 = US2::US2_JobFinished(v1);
                    US0::US0_Settled(v13)
                } else {
                    let v15: bool = &*v0 == "timed-out";
                    if v15 {
                        let v16: US2 = US2::US2_JobTimedOut;
                        US0::US0_Settled(v16)
                    } else {
                        let v18: bool = &*v0 == "stopped";
                        if v18 {
                            let v19: US2 = US2::US2_JobStopped;
                            US0::US0_Settled(v19)
                        } else {
                            let v21: bool = &*v0 == "lost";
                            if v21 {
                                let v22: US2 = US2::US2_JobLost;
                                US0::US0_Settled(v22)
                            } else {
                                let v24: bool = &*v0 == "memory-capped";
                                if v24 {
                                    let v25: US2 = US2::US2_JobMemoryCapped;
                                    US0::US0_Settled(v25)
                                } else {
                                    let v27: bool = &*v0 == "expired-in-queue";
                                    if v27 {
                                        let v28: US2 = US2::US2_JobExpiredInQueue;
                                        US0::US0_Settled(v28)
                                    } else {
                                        let v30: bool = &*v0 == "skipped";
                                        if v30 {
                                            let v31: US2 = US2::US2_JobSkipped(v2);
                                            US0::US0_Settled(v31)
                                        } else {
                                            let v33: US2 = US2::US2_JobSpawnFailed(v2);
                                            US0::US0_Settled(v33)
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
fn job_state_of_4(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US0 {
    let mut v4: bool = &*v0 != "";
    if v4 {
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v6: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
        let v7: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
        state_of_parts_5(v5, v6, v7)
    } else {
        let v9: bool = v1 > 0u64;
        if v9 {
            let v10: u64 = v1.wrapping_add(15000u64);
            let v11: bool = v10 < v3;
            if v11 {
                let v12: US2 = US2::US2_JobLost;
                US0::US0_Settled(v12)
            } else {
                let v14: US1 = US1::US1_JobRunning;
                US0::US0_Live(v14)
            }
        } else {
            let v17: u64 = v2.wrapping_add(15000u64);
            let v18: bool = v17 < v3;
            if v18 {
                let v19: US2 = US2::US2_JobLost;
                US0::US0_Settled(v19)
            } else {
                let v21: US1 = US1::US1_JobPending;
                US0::US0_Live(v21)
            }
        }
    }
}
fn joined_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_10() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_JOBS_HOME").or_else(|| std::env::var_os("KIT_JOBS_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_9(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_10();
    joined_8(v1, v0)
}
fn job_file_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_9(v0);
    joined_8(v2, v1)
}
fn text_or_empty_11(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method13(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_12(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method13(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let mut v6: US3 = US3::US3_None;
    let v7: US3 = v6.clone();
    let v8: US3 = v4.unwrap_or(v7);
    match v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn now_ms_14() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_6(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_7(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_11(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = job_file_7(v0.clone(), v4);
    let mut v6: Rc<str> = text_or_empty_11(v5);
    let v7: u64 = decimal_u64_12(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = job_file_7(v0, v8);
    let v10: Rc<str> = text_or_empty_11(v9);
    let v11: u64 = decimal_u64_12(v10);
    let v12: u64 = now_ms_14();
    (v3, v7, v11, v12)
}
fn file_exists_15(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn start_request_path_16(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-home"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_7(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_11(v2);
    let mut v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v5: bool = &*v4 == "";
    let v13: Rc<str> = if v5 {
        let mut v6: Rc<str> = jobs_root_10();
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shell-host"); } LIT.with(|lit| lit.clone()) };
        joined_8(v6, v7)
    } else {
        let v9: Rc<str> = jobs_root_10();
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hosts"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = joined_8(v9, v10);
        joined_8(v11, v4)
    };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("requests"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = joined_8(v13, v14);
    let v16: Rc<str> = string_concat(&[&*v0, ".start"]);
    joined_8(v15, v16)
}
fn process_alive_17(mut v0: u64) -> bool {
    let v1: bool = rig_process_alive::rig_process_alive(v0);
    v1
}
fn text_written_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2
}
fn job_state_now_3(mut v0: Rc<str>) -> US0 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_6(v0.clone());
    let mut v5: US0 = job_state_of_4(v1.clone(), v2, v3, v4);
    match &v5 {
        US0::US0_Live(v6) => {
            let mut v6: US1 = v6.clone();
            match v6 {
                US1::US1_JobRunning => {
                    let v7: Rc<str> = jobs_root_10();
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let v9: Rc<str> = joined_8(v7, v8);
                    let v10: Rc<str> = joined_8(v9, v0);
                    let v11: bool = file_exists_15(v10);
                    if v11 {
                        let v12: US1 = US1::US1_JobPending;
                        US0::US0_Live(v12)
                    } else {
                        v5
                    }
                }
                _ => {
                    v5
                }
            }
        }
        US0::US0_Settled(v16) => {
            let v16: US2 = v16.clone();
            match v16 {
                US2::US2_JobLost => {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
                    let v18: Rc<str> = job_file_7(v0.clone(), v17);
                    let v19: Rc<str> = text_or_empty_11(v18);
                    let v20: u64 = decimal_u64_12(v19);
                    let v21: Rc<str> = jobs_root_10();
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = joined_8(v21, v22);
                    let v24: Rc<str> = joined_8(v23, v0.clone());
                    let v25: bool = file_exists_15(v24);
                    let v28: bool = if v25 {
                        true
                    } else {
                        let v26: Rc<str> = start_request_path_16(v0.clone());
                        file_exists_15(v26)
                    };
                    if v28 {
                        let v29: US1 = US1::US1_JobPending;
                        US0::US0_Live(v29)
                    } else {
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placement"); } LIT.with(|lit| lit.clone()) };
                        let v32: Rc<str> = job_file_7(v0.clone(), v31);
                        let v33: bool = file_exists_15(v32);
                        if v33 {
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placed-exit"); } LIT.with(|lit| lit.clone()) };
                            let v35: Rc<str> = job_file_7(v0, v34);
                            let v36: Rc<str> = text_or_empty_11(v35);
                            let v37: bool = &*v36 == "";
                            if v37 {
                                let v38: US1 = US1::US1_JobPlaced;
                                US0::US0_Live(v38)
                            } else {
                                job_state_of_4(v36, v2, v3, v4)
                            }
                        } else {
                            let v42: bool = v20 != 0u64;
                            let v44: bool = if v42 {
                                process_alive_17(v20)
                            } else {
                                false
                            };
                            if v44 {
                                let v45: US1 = US1::US1_JobRunning;
                                US0::US0_Live(v45)
                            } else {
                                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                                let v48: Rc<str> = job_file_7(v0.clone(), v47);
                                let v49: Rc<str> = text_or_empty_11(v48);
                                let v50: u64 = decimal_u64_12(v49);
                                let v51: bool = v50 == 0u64;
                                let v54: bool = if v51 {
                                    true
                                } else {
                                    let v52: bool = process_alive_17(v50);
                                    let v53: bool = v52 == false;
                                    v53
                                };
                                let v55: bool = &*v1 == "";
                                let v56: bool = v55 && v54;
                                let v72: Rc<str> = if v56 {
                                    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                                    let v58: Rc<str> = job_file_7(v0, v57);
                                    let v59: bool = v2 > 0u64;
                                    let v60: u64 = if v59 {
                                        v2
                                    } else {
                                        v3
                                    };
                                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the supervisor stopped beating: it was killed (with its caller, by the memory reaper) or crashed, and its job tree ended with it"); } LIT.with(|lit| lit.clone()) };
                                    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                                    let v63: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
                                    let v64: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                                    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v68: Rc<str> = concat5_2(v62, v65.clone(), v61, v66, v67);
                                    let v69: Rc<str> = concat5_2(v63, v65.clone(), v64, v65, v68);
                                    text_written_18(v58, v69)
                                } else {
                                    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v71
                                };
                                v5
                            }
                        }
                    }
                }
                _ => {
                    v5
                }
            }
        }
    }
}
fn state_settled_19(mut v0: US0) -> bool {
    match v0 {
        US0::US0_Live(v1) => {
            let v1: US1 = v1;
            false
        }
        US0::US0_Settled(v2) => {
            let v2: US2 = v2;
            true
        }
    }
}
fn method22(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn window_reason_21(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason="); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method22(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    } else {
        let v10: i32 = v7.wrapping_add(1i32);
        let v11: i32 = (v0.clone().len() as i32);
        let v12: i32 = v11.wrapping_sub(v7);
        let v13: i32 = v12.wrapping_sub(1i32);
        let v14: usize = ((v10) as usize);
        let v15: i32 = v10.wrapping_add(v13);
        let v16: usize = ((v15) as usize);
        let v17: Rc<str> = Rc::<str>::from(&(v0)[v14..v16]);
        let v18: Rc<str> = Rc::<str>::from((v17).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        v18
    }
}
fn gate_wait_of_20(mut v0: Rc<str>) -> US4 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_7(v0.clone(), v1.clone());
    let mut v3: bool = file_exists_15(v2);
    let v8: bool = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = job_file_7(v0.clone(), v4);
        let mut v6: bool = file_exists_15(v5);
        let v7: bool = v6 == false;
        v7
    } else {
        false
    };
    if v8 {
        let v9: Rc<str> = job_file_7(v0, v1);
        let v10: Rc<str> = text_or_empty_11(v9);
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v10.split(&*v11).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v13: Rc<str> = v12.borrow()[0i32 as usize].clone();
        let v14: Rc<str> = Rc::<str>::from((v13).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        US4::US4_WaitingAfter(v14)
    } else {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = job_file_7(v0.clone(), v16.clone());
        let v18: bool = file_exists_15(v17);
        if v18 {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = job_file_7(v0.clone(), v19);
            let v21: Rc<str> = text_or_empty_11(v20);
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            let v23: bool = v21.starts_with(&*v22);
            if v23 {
                US4::US4_NotWaiting
            } else {
                let v25: Rc<str> = job_file_7(v0, v16);
                let v26: Rc<str> = text_or_empty_11(v25);
                let v27: Rc<str> = Rc::<str>::from((v26).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v28: Rc<str> = window_reason_21(v21);
                US4::US4_WaitingWindow(v27, v28)
            }
        } else {
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = job_file_7(v0.clone(), v31);
            let v33: bool = file_exists_15(v32);
            let v40: bool = if v33 {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = job_file_7(v0, v34);
                let v36: Rc<str> = text_or_empty_11(v35);
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Invoke-Heavy: acquired slot"); } LIT.with(|lit| lit.clone()) };
                let v38: bool = v36.contains(&*v37);
                let v39: bool = v38 == false;
                v39
            } else {
                false
            };
            if v40 {
                US4::US4_WaitingSlot
            } else {
                US4::US4_NotWaiting
            }
        }
    }
}
fn live_word_24(mut v0: US1) -> Rc<str> {
    match v0 {
        US1::US1_JobPending => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US1::US1_JobPlaced => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placed"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US1::US1_JobRunning => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v2
        }
    }
}
fn state_word_23(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_Live(v1) => {
            let v1: US1 = v1;
            live_word_24(v1)
        }
        US0::US0_Settled(v3) => {
            let mut v3: US2 = v3;
            match v3 {
                US2::US2_JobExpiredInQueue => {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                    v14
                }
                US2::US2_JobFinished(v4) => {
                    let mut v4: i32 = v4;
                    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
                    v5
                }
                US2::US2_JobLost => {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                    v12
                }
                US2::US2_JobMemoryCapped => {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
                    v13
                }
                US2::US2_JobSkipped(v10) => {
                    let v10: Rc<str> = v10;
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
                    v11
                }
                US2::US2_JobSpawnFailed(v8) => {
                    let v8: Rc<str> = v8;
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
                    v9
                }
                US2::US2_JobStopped => {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
                    v7
                }
                US2::US2_JobTimedOut => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
                    v6
                }
            }
        }
    }
}
fn settled_exit_25(mut v0: US2) -> i32 {
    match v0 {
        US2::US2_JobExpiredInQueue => {
            75i32
        }
        US2::US2_JobFinished(v1) => {
            let v1: i32 = v1;
            v1
        }
        US2::US2_JobLost => {
            125i32
        }
        US2::US2_JobMemoryCapped => {
            137i32
        }
        US2::US2_JobSkipped(v3) => {
            let mut v3: Rc<str> = v3;
            125i32
        }
        US2::US2_JobSpawnFailed(v2) => {
            let v2: Rc<str> = v2;
            125i32
        }
        US2::US2_JobStopped => {
            143i32
        }
        US2::US2_JobTimedOut => {
            124i32
        }
    }
}
fn elapsed_of_26(mut v0: Rc<str>) -> u64 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_6(v0);
    let mut v5: bool = &*v1 == "";
    let v8: u64 = if v5 {
        v4
    } else {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.split(char::from(9u8)).nth(1).unwrap_or("0"));
        decimal_u64_12(v6)
    };
    let v9: bool = v8 > v3;
    if v9 {
        let v10: u64 = v8.wrapping_sub(v3);
        v10
    } else {
        0u64
    }
}
fn method29(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_28(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method29(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn decimal_text_27(mut v0: u64) -> Rc<str> {
    format_real_28(v0)
}
fn file_bytes_30(mut v0: Rc<str>) -> u64 {
    let v1: u64 = std::fs::metadata(&*v0).map(|metadata| metadata.len()).unwrap_or(0);
    v1
}
fn job_placement_31(mut v0: Rc<str>) -> US5 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placement"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_7(v0, v1);
    let mut v3: Rc<str> = text_or_empty_11(v2);
    let mut v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v5: bool = &*v4 == "";
    let v7: bool = if v5 {
        true
    } else {
        let mut v6: bool = &*v4 == "local";
        v6
    };
    let v35: US7 = if v7 {
        let v8: US5 = US5::US5_Local;
        US7::US7_Some(v8)
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
            let v26: US8 = if v22 {
                US8::US8_None
            } else {
                let v24: US6 = US6::US6_Machine(v18);
                US8::US8_Some(v24)
            };
            match v26 {
                US8::US8_None => {
                    US7::US7_None
                }
                US8::US8_Some(v27) => {
                    let v27: US6 = v27;
                    let v28: US5 = US5::US5_OnMachine(v27);
                    US7::US7_Some(v28)
                }
            }
        } else {
            US7::US7_None
        }
    };
    match v35 {
        US7::US7_None => {
            US5::US5_Local
        }
        US7::US7_Some(v36) => {
            let v36: US5 = v36;
            v36
        }
    }
}
fn job_summary_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: US0 = job_state_now_3(v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ok id="); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state="); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = concat5_2(v3, v0, v4, v1.clone(), v5);
    let v7: bool = state_settled_19(v2.clone());
    let v10: US4 = if v7 {
        US4::US4_NotWaiting
    } else {
        gate_wait_of_20(v1.clone())
    };
    let v11: bool = match &v10 {
        US4::US4_NotWaiting => {
            true
        }
        _ => {
            false
        }
    };
    let v12: bool = v11 != true;
    let v15: bool = if v12 {
        let v13: bool = state_settled_19(v2.clone());
        let v14: bool = v13 == false;
        v14
    } else {
        false
    };
    let v18: Rc<str> = if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
        v16
    } else {
        state_word_23(v2.clone())
    };
    let v34: Rc<str> = match v10 {
        US4::US4_NotWaiting => {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v19
        }
        US4::US4_WaitingAfter(v20) => {
            let v20: Rc<str> = v20;
            let v21: Rc<str> = string_concat(&[" waiting_for=after:", &*v20]);
            v21
        }
        US4::US4_WaitingSlot => {
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waiting_for=heavy-slot"); } LIT.with(|lit| lit.clone()) };
            v30
        }
        US4::US4_WaitingWindow(v22, v23) => {
            let v22: Rc<str> = v22;
            let v23: Rc<str> = v23;
            let v24: Rc<str> = string_concat(&[" waiting_for=window:", &*v22]);
            let v25: bool = &*v23 == "";
            let v28: Rc<str> = if v25 {
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v26
            } else {
                let v27: Rc<str> = string_concat(&[" ", &*v23]);
                v27
            };
            let v29: Rc<str> = string_concat(&[&*v24, &*v28]);
            v29
        }
    };
    let v36: Rc<str> = string_concat(&[&*v18, &*v34, " exit="]);
    let v43: Rc<str> = match v2 {
        US0::US0_Live(v37) => {
            let v37: US1 = v37;
            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v38
        }
        US0::US0_Settled(v39) => {
            let v39: US2 = v39;
            let v40: i32 = settled_exit_25(v39);
            let v41: Rc<str> = std::rc::Rc::<str>::from((v40).to_string());
            v41
        }
    };
    let v46: u64 = elapsed_of_26(v1.clone());
    let v47: Rc<str> = decimal_text_27(v46);
    let v48: Rc<str> = string_concat(&[&*v36, &*v43, " elapsed_ms=", &*v47]);
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stdout_bytes="); } LIT.with(|lit| lit.clone()) };
    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
    let v51: Rc<str> = job_file_7(v1.clone(), v50);
    let v52: u64 = file_bytes_30(v51);
    let v53: Rc<str> = decimal_text_27(v52);
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stderr_bytes="); } LIT.with(|lit| lit.clone()) };
    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
    let v56: Rc<str> = job_file_7(v1.clone(), v55);
    let v57: u64 = file_bytes_30(v56);
    let v58: Rc<str> = decimal_text_27(v57);
    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v60: Rc<str> = concat5_2(v49, v53, v54, v58, v59.clone());
    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let v62: Rc<str> = job_file_7(v1.clone(), v61);
    let v63: Rc<str> = text_or_empty_11(v62);
    let v64: Rc<str> = std::rc::Rc::<str>::from(v63.split(char::from(9u8)).nth(3).unwrap_or("").trim());
    let v65: bool = &*v64 == "";
    let v69: Rc<str> = if v65 {
        v59.clone()
    } else {
        let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" detail=\""); } LIT.with(|lit| lit.clone()) };
        let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v66, v64, v67, v59.clone(), v59.clone())
    };
    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label="); } LIT.with(|lit| lit.clone()) };
    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
    let v72: Rc<str> = job_file_7(v1.clone(), v71);
    let v73: Rc<str> = text_or_empty_11(v72);
    let v74: Rc<str> = concat5_2(v70, v73, v59.clone(), v59.clone(), v59.clone());
    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_mb"); } LIT.with(|lit| lit.clone()) };
    let v76: Rc<str> = job_file_7(v1.clone(), v75);
    let v77: Rc<str> = text_or_empty_11(v76);
    let v78: bool = &*v77 == "";
    let v89: Rc<str> = if v78 {
        v59.clone()
    } else {
        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" max_private_mb="); } LIT.with(|lit| lit.clone()) };
        let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" peak_private_mb="); } LIT.with(|lit| lit.clone()) };
        let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("peak_private_mb"); } LIT.with(|lit| lit.clone()) };
        let v82: Rc<str> = job_file_7(v1.clone(), v81.clone());
        let v83: bool = file_exists_15(v82);
        let v87: Rc<str> = if v83 {
            let v84: Rc<str> = job_file_7(v1.clone(), v81);
            text_or_empty_11(v84)
        } else {
            let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
            v86
        };
        concat5_2(v79, v77, v80, v87, v59.clone())
    };
    let v90: US5 = job_placement_31(v1);
    let v96: Rc<str> = match v90 {
        US5::US5_Local => {
            v59.clone()
        }
        US5::US5_OnMachine(v91) => {
            let v91: US6 = v91;
            let v93: Rc<str> = match v91 {
                US6::US6_Machine(v92) => {
                    let v92: Rc<str> = v92;
                    v92
                }
            };
            let v94: Rc<str> = string_concat(&[" placement=", &*v93]);
            v94
        }
    };
    let v97: Rc<str> = concat5_2(v89, v74, v96, v59.clone(), v59);
    concat5_2(v6, v48, v60, v69, v97)
}
fn summary_settled_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = rig_job_orphans::rig_job_orphan_settled(&v0);
    job_summary_1(v1, v0)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        summary_settled_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_job_summary(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
use rig_process_alive::SpiralShape_97df63ebabc3 as Mut0;
