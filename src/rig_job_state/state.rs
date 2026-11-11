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
fn state_of_parts_3(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US0 {
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
fn job_state_of_2(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US0 {
    let mut v4: bool = &*v0 != "";
    if v4 {
        let v5: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v6: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
        let v7: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
        state_of_parts_3(v5, v6, v7)
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
fn joined_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_8() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_JOBS_HOME").or_else(|| std::env::var_os("KIT_JOBS_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_8();
    joined_6(v1, v0)
}
fn job_file_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_7(v0);
    joined_6(v2, v1)
}
fn text_or_empty_9(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method11(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_10(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method11(v1);
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
fn now_ms_12() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_4(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_5(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_9(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = job_file_5(v0.clone(), v4);
    let mut v6: Rc<str> = text_or_empty_9(v5);
    let v7: u64 = decimal_u64_10(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = job_file_5(v0, v8);
    let v10: Rc<str> = text_or_empty_9(v9);
    let v11: u64 = decimal_u64_10(v10);
    let v12: u64 = now_ms_12();
    (v3, v7, v11, v12)
}
fn file_exists_13(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn start_request_path_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-home"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_5(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_9(v2);
    let mut v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v5: bool = &*v4 == "";
    let v13: Rc<str> = if v5 {
        let mut v6: Rc<str> = jobs_root_8();
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shell-host"); } LIT.with(|lit| lit.clone()) };
        joined_6(v6, v7)
    } else {
        let v9: Rc<str> = jobs_root_8();
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hosts"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = joined_6(v9, v10);
        joined_6(v11, v4)
    };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("requests"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = joined_6(v13, v14);
    let v16: Rc<str> = string_concat(&[&*v0, ".start"]);
    joined_6(v15, v16)
}
fn process_alive_15(mut v0: u64) -> bool {
    let v1: bool = rig_process_alive::rig_process_alive(v0);
    v1
}
fn concat5_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn text_written_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2
}
fn job_state_now_1(mut v0: Rc<str>) -> US0 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_4(v0.clone());
    let v5: US0 = job_state_of_2(v1.clone(), v2, v3, v4);
    match &v5 {
        US0::US0_Live(v6) => {
            let mut v6: US1 = v6.clone();
            match v6 {
                US1::US1_JobRunning => {
                    let v7: Rc<str> = jobs_root_8();
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let v9: Rc<str> = joined_6(v7, v8);
                    let v10: Rc<str> = joined_6(v9, v0);
                    let v11: bool = file_exists_13(v10);
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
                    let v18: Rc<str> = job_file_5(v0.clone(), v17);
                    let v19: Rc<str> = text_or_empty_9(v18);
                    let v20: u64 = decimal_u64_10(v19);
                    let v21: Rc<str> = jobs_root_8();
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = joined_6(v21, v22);
                    let v24: Rc<str> = joined_6(v23, v0.clone());
                    let v25: bool = file_exists_13(v24);
                    let v28: bool = if v25 {
                        true
                    } else {
                        let v26: Rc<str> = start_request_path_14(v0.clone());
                        file_exists_13(v26)
                    };
                    if v28 {
                        let v29: US1 = US1::US1_JobPending;
                        US0::US0_Live(v29)
                    } else {
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placement"); } LIT.with(|lit| lit.clone()) };
                        let v32: Rc<str> = job_file_5(v0.clone(), v31);
                        let v33: bool = file_exists_13(v32);
                        if v33 {
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placed-exit"); } LIT.with(|lit| lit.clone()) };
                            let v35: Rc<str> = job_file_5(v0, v34);
                            let v36: Rc<str> = text_or_empty_9(v35);
                            let v37: bool = &*v36 == "";
                            if v37 {
                                let v38: US1 = US1::US1_JobPlaced;
                                US0::US0_Live(v38)
                            } else {
                                job_state_of_2(v36, v2, v3, v4)
                            }
                        } else {
                            let v42: bool = v20 != 0u64;
                            let v44: bool = if v42 {
                                process_alive_15(v20)
                            } else {
                                false
                            };
                            if v44 {
                                let v45: US1 = US1::US1_JobRunning;
                                US0::US0_Live(v45)
                            } else {
                                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                                let v48: Rc<str> = job_file_5(v0.clone(), v47);
                                let v49: Rc<str> = text_or_empty_9(v48);
                                let v50: u64 = decimal_u64_10(v49);
                                let v51: bool = v50 == 0u64;
                                let v54: bool = if v51 {
                                    true
                                } else {
                                    let v52: bool = process_alive_15(v50);
                                    let v53: bool = v52 == false;
                                    v53
                                };
                                let v55: bool = &*v1 == "";
                                let v56: bool = v55 && v54;
                                let v72: Rc<str> = if v56 {
                                    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                                    let v58: Rc<str> = job_file_5(v0, v57);
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
                                    let v68: Rc<str> = concat5_16(v62, v65.clone(), v61, v66, v67);
                                    let v69: Rc<str> = concat5_16(v63, v65.clone(), v64, v65, v68);
                                    text_written_17(v58, v69)
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
fn live_word_19(mut v0: US1) -> Rc<str> {
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
fn state_word_18(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_Live(v1) => {
            let v1: US1 = v1;
            live_word_19(v1)
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
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
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
fn settled_exit_21(mut v0: US2) -> i32 {
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
fn state_exit_20(mut v0: US0) -> i32 {
    match v0 {
        US0::US0_Live(v1) => {
            let v1: US1 = v1;
            3i32
        }
        US0::US0_Settled(v2) => {
            let v2: US2 = v2;
            settled_exit_21(v2)
        }
    }
}
fn state_detail_22(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_Live(v1) => {
            let v1: US1 = v1;
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_Settled(v3) => {
            let mut v3: US2 = v3;
            match v3 {
                US2::US2_JobSkipped(v5) => {
                    let v5: Rc<str> = v5;
                    v5
                }
                US2::US2_JobSpawnFailed(v4) => {
                    let mut v4: Rc<str> = v4;
                    v4
                }
                _ => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v6
                }
            }
        }
    }
}
fn state_parts_0(mut v0: Rc<str>) -> (Rc<str>, i32, Rc<str>) {
    let v1: bool = rig_job_orphans::rig_job_orphan_settled(&v0);
    let v2: US0 = job_state_now_1(v0);
    let mut v3: Rc<str> = state_word_18(v2.clone());
    let mut v4: i32 = state_exit_20(v2.clone());
    let v5: Rc<str> = state_detail_22(v2);
    (v3, v4, v5)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, i32, Rc<str>) {
        state_parts_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_job_state_parts(v0: &str) -> (Rc<str>, i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0))
}
