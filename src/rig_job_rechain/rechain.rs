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
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_AfterAny,
    US2_AfterOk,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum US4 {
    US4_JobPending,
    US4_JobRunning,
    US4_JobPlaced,
}
#[derive(Clone)]
enum US5 {
    US5_JobFinished(i32),
    US5_JobTimedOut,
    US5_JobStopped,
    US5_JobSpawnFailed(Rc<str>),
    US5_JobSkipped(Rc<str>),
    US5_JobLost,
    US5_JobMemoryCapped,
    US5_JobExpiredInQueue,
}
#[derive(Clone)]
enum US3 {
    US3_Live(US4),
    US3_Settled(US5),
}
#[derive(Clone)]
enum US6 {
    US6_Some(u64),
    US6_None,
}
#[derive(Clone)]
enum US8 {
    US8_UnknownJob,
    US8_UnknownPredecessor,
    US8_SelfAfter,
    US8_NotChained,
    US8_NotWaiting(Rc<str>),
    US8_WouldCycle,
}
#[derive(Clone)]
enum US7 {
    US7_Repointed(Rc<str>),
    US7_Refused(US8),
}
fn command_line_argument_count_1() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_2(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn state_of_parts_6(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US3 {
    let mut v3: bool = &*v0 == "pending";
    if v3 {
        let mut v4: US4 = US4::US4_JobPending;
        US3::US3_Live(v4)
    } else {
        let mut v6: bool = &*v0 == "running";
        if v6 {
            let v7: US4 = US4::US4_JobRunning;
            US3::US3_Live(v7)
        } else {
            let v9: bool = &*v0 == "placed";
            if v9 {
                let v10: US4 = US4::US4_JobPlaced;
                US3::US3_Live(v10)
            } else {
                let v12: bool = &*v0 == "finished";
                if v12 {
                    let v13: US5 = US5::US5_JobFinished(v1);
                    US3::US3_Settled(v13)
                } else {
                    let v15: bool = &*v0 == "timed-out";
                    if v15 {
                        let v16: US5 = US5::US5_JobTimedOut;
                        US3::US3_Settled(v16)
                    } else {
                        let v18: bool = &*v0 == "stopped";
                        if v18 {
                            let v19: US5 = US5::US5_JobStopped;
                            US3::US3_Settled(v19)
                        } else {
                            let v21: bool = &*v0 == "lost";
                            if v21 {
                                let v22: US5 = US5::US5_JobLost;
                                US3::US3_Settled(v22)
                            } else {
                                let v24: bool = &*v0 == "memory-capped";
                                if v24 {
                                    let v25: US5 = US5::US5_JobMemoryCapped;
                                    US3::US3_Settled(v25)
                                } else {
                                    let v27: bool = &*v0 == "expired-in-queue";
                                    if v27 {
                                        let v28: US5 = US5::US5_JobExpiredInQueue;
                                        US3::US3_Settled(v28)
                                    } else {
                                        let v30: bool = &*v0 == "skipped";
                                        if v30 {
                                            let v31: US5 = US5::US5_JobSkipped(v2);
                                            US3::US3_Settled(v31)
                                        } else {
                                            let v33: US5 = US5::US5_JobSpawnFailed(v2);
                                            US3::US3_Settled(v33)
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
fn job_state_of_5(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US3 {
    let mut v4: bool = &*v0 != "";
    if v4 {
        let v5: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v6: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
        let v7: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
        state_of_parts_6(v5, v6, v7)
    } else {
        let v9: bool = v1 > 0u64;
        if v9 {
            let v10: u64 = v1.wrapping_add(15000u64);
            let v11: bool = v10 < v3;
            if v11 {
                let v12: US5 = US5::US5_JobLost;
                US3::US3_Settled(v12)
            } else {
                let v14: US4 = US4::US4_JobRunning;
                US3::US3_Live(v14)
            }
        } else {
            let v17: u64 = v2.wrapping_add(15000u64);
            let v18: bool = v17 < v3;
            if v18 {
                let v19: US5 = US5::US5_JobLost;
                US3::US3_Settled(v19)
            } else {
                let v21: US4 = US4::US4_JobPending;
                US3::US3_Live(v21)
            }
        }
    }
}
fn joined_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_11() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_JOBS_HOME").or_else(|| std::env::var_os("KIT_JOBS_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_10(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_11();
    joined_9(v1, v0)
}
fn job_file_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_10(v0);
    joined_9(v2, v1)
}
fn text_or_empty_12(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method14(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US6> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US6> = Rc::new(move |mut v0: (u64)| -> US6 {
        let v1: u64 = (v0);
        US6::US6_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_13(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method14(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US6> = closure1();
    let mut v4: Option<US6> = v2.map(|x| v3(x));
    let mut v6: US6 = US6::US6_None;
    let v7: US6 = v6.clone();
    let v8: US6 = v4.unwrap_or(v7);
    match v8 {
        US6::US6_None => {
            0u64
        }
        US6::US6_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn now_ms_15() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_7(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_8(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_12(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = job_file_8(v0.clone(), v4);
    let mut v6: Rc<str> = text_or_empty_12(v5);
    let v7: u64 = decimal_u64_13(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = job_file_8(v0, v8);
    let v10: Rc<str> = text_or_empty_12(v9);
    let v11: u64 = decimal_u64_13(v10);
    let v12: u64 = now_ms_15();
    (v3, v7, v11, v12)
}
fn file_exists_16(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn start_request_path_17(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-home"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_8(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_12(v2);
    let mut v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v5: bool = &*v4 == "";
    let v13: Rc<str> = if v5 {
        let mut v6: Rc<str> = jobs_root_11();
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shell-host"); } LIT.with(|lit| lit.clone()) };
        joined_9(v6, v7)
    } else {
        let v9: Rc<str> = jobs_root_11();
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hosts"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = joined_9(v9, v10);
        joined_9(v11, v4)
    };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("requests"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = joined_9(v13, v14);
    let v16: Rc<str> = string_concat(&[&*v0, ".start"]);
    joined_9(v15, v16)
}
fn process_alive_18(mut v0: u64) -> bool {
    let v1: bool = rig_process_alive::rig_process_alive(v0);
    v1
}
fn concat5_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn text_written_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2
}
fn job_state_now_4(mut v0: Rc<str>) -> US3 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_7(v0.clone());
    let v5: US3 = job_state_of_5(v1.clone(), v2, v3, v4);
    match &v5 {
        US3::US3_Live(v6) => {
            let mut v6: US4 = v6.clone();
            match v6 {
                US4::US4_JobRunning => {
                    let v7: Rc<str> = jobs_root_11();
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let v9: Rc<str> = joined_9(v7, v8);
                    let v10: Rc<str> = joined_9(v9, v0);
                    let v11: bool = file_exists_16(v10);
                    if v11 {
                        let v12: US4 = US4::US4_JobPending;
                        US3::US3_Live(v12)
                    } else {
                        v5
                    }
                }
                _ => {
                    v5
                }
            }
        }
        US3::US3_Settled(v16) => {
            let v16: US5 = v16.clone();
            match v16 {
                US5::US5_JobLost => {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
                    let v18: Rc<str> = job_file_8(v0.clone(), v17);
                    let v19: Rc<str> = text_or_empty_12(v18);
                    let v20: u64 = decimal_u64_13(v19);
                    let v21: Rc<str> = jobs_root_11();
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = joined_9(v21, v22);
                    let v24: Rc<str> = joined_9(v23, v0.clone());
                    let v25: bool = file_exists_16(v24);
                    let v28: bool = if v25 {
                        true
                    } else {
                        let v26: Rc<str> = start_request_path_17(v0.clone());
                        file_exists_16(v26)
                    };
                    if v28 {
                        let v29: US4 = US4::US4_JobPending;
                        US3::US3_Live(v29)
                    } else {
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placement"); } LIT.with(|lit| lit.clone()) };
                        let v32: Rc<str> = job_file_8(v0.clone(), v31);
                        let v33: bool = file_exists_16(v32);
                        if v33 {
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placed-exit"); } LIT.with(|lit| lit.clone()) };
                            let v35: Rc<str> = job_file_8(v0, v34);
                            let v36: Rc<str> = text_or_empty_12(v35);
                            let v37: bool = &*v36 == "";
                            if v37 {
                                let v38: US4 = US4::US4_JobPlaced;
                                US3::US3_Live(v38)
                            } else {
                                job_state_of_5(v36, v2, v3, v4)
                            }
                        } else {
                            let v42: bool = v20 != 0u64;
                            let v44: bool = if v42 {
                                process_alive_18(v20)
                            } else {
                                false
                            };
                            if v44 {
                                let v45: US4 = US4::US4_JobRunning;
                                US3::US3_Live(v45)
                            } else {
                                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                                let v48: Rc<str> = job_file_8(v0.clone(), v47);
                                let v49: Rc<str> = text_or_empty_12(v48);
                                let v50: u64 = decimal_u64_13(v49);
                                let v51: bool = v50 == 0u64;
                                let v54: bool = if v51 {
                                    true
                                } else {
                                    let v52: bool = process_alive_18(v50);
                                    let v53: bool = v52 == false;
                                    v53
                                };
                                let v55: bool = &*v1 == "";
                                let v56: bool = v55 && v54;
                                let v72: Rc<str> = if v56 {
                                    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                                    let v58: Rc<str> = job_file_8(v0, v57);
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
                                    let v68: Rc<str> = concat5_19(v62, v65.clone(), v61, v66, v67);
                                    let v69: Rc<str> = concat5_19(v63, v65.clone(), v64, v65, v68);
                                    text_written_20(v58, v69)
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
fn directory_exists_21(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn file_exists_22(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn state_settled_23(mut v0: US3) -> bool {
    match v0 {
        US3::US3_Live(v1) => {
            let v1: US4 = v1;
            false
        }
        US3::US3_Settled(v2) => {
            let v2: US5 = v2;
            true
        }
    }
}
fn live_word_25(mut v0: US4) -> Rc<str> {
    match v0 {
        US4::US4_JobPending => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US4::US4_JobPlaced => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placed"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US4::US4_JobRunning => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v2
        }
    }
}
fn state_word_24(mut v0: US3) -> Rc<str> {
    match v0 {
        US3::US3_Live(v1) => {
            let v1: US4 = v1;
            live_word_25(v1)
        }
        US3::US3_Settled(v3) => {
            let mut v3: US5 = v3;
            match v3 {
                US5::US5_JobExpiredInQueue => {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                    v14
                }
                US5::US5_JobFinished(v4) => {
                    let mut v4: i32 = v4;
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
                    v5
                }
                US5::US5_JobLost => {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                    v12
                }
                US5::US5_JobMemoryCapped => {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
                    v13
                }
                US5::US5_JobSkipped(v10) => {
                    let v10: Rc<str> = v10;
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
                    v11
                }
                US5::US5_JobSpawnFailed(v8) => {
                    let v8: Rc<str> = v8;
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
                    v9
                }
                US5::US5_JobStopped => {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
                    v7
                }
                US5::US5_JobTimedOut => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
                    v6
                }
            }
        }
    }
}
fn method26(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn chain_reaches_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = &*v0 == &*v1;
        if v3 {
            return true;
        } else {
            let mut v4: bool = v2 <= 0i32;
            let v14: bool = if v4 {
                true
            } else {
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = job_file_8(v0.clone(), v5);
                let v7: bool = file_exists_22(v6);
                let v12: bool = if v7 {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
                    let v9: Rc<str> = job_file_8(v0.clone(), v8);
                    let v10: bool = file_exists_22(v9);
                    let v11: bool = v10 == false;
                    v11
                } else {
                    false
                };
                let v13: bool = v12 == false;
                v13
            };
            if v14 {
                return false;
            } else {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = job_file_8(v0, v15);
                let v17: bool = file_exists_22(v16.clone());
                let v30: Rc<str> = if v17 {
                    let v19: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v16);
                    let v21: Vec<u8> = v19.unwrap();
                    let v22: Vec<u8> = method26(v21);
                    let v24: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v22);
                    let v26: std::string::String = v24.unwrap();
                    let v28: Rc<str> = Rc::<str>::from(String::as_str(&v26));
                    v28
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v29
                };
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v30.split(&*v31).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let v33: Rc<str> = v32.borrow()[0i32 as usize].clone();
                let v34: Rc<str> = Rc::<str>::from((v33).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v35: i32 = v2.wrapping_sub(1i32);
                (v0, v2) = (v34, v35);
                continue;
            }
        }
    }
}
fn facts_of_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (bool, bool, bool, bool, bool, Rc<str>, Rc<str>, bool) {
    let v2: US3 = job_state_now_4(v0.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v0.starts_with(&*v3);
    let v7: bool = if v4 {
        let v5: Rc<str> = job_directory_10(v0.clone());
        directory_exists_21(v5)
    } else {
        false
    };
    let v8: bool = v1.starts_with(&*v3);
    let v11: bool = if v8 {
        let v9: Rc<str> = job_directory_10(v1.clone());
        directory_exists_21(v9)
    } else {
        false
    };
    let v12: bool = &*v0 == &*v1;
    let v16: bool = if v7 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = job_file_8(v0.clone(), v13);
        file_exists_22(v14)
    } else {
        false
    };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = job_file_8(v0.clone(), v17.clone());
    let v19: bool = file_exists_22(v18);
    let v24: bool = if v19 {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = job_file_8(v0.clone(), v20);
        let v22: bool = file_exists_22(v21);
        let v23: bool = v22 == false;
        v23
    } else {
        false
    };
    let v27: bool = if v24 {
        let v25: bool = state_settled_23(v2.clone());
        let v26: bool = v25 == false;
        v26
    } else {
        false
    };
    let v28: Rc<str> = state_word_24(v2);
    let v29: Rc<str> = job_file_8(v0.clone(), v17);
    let v30: bool = file_exists_22(v29.clone());
    let v43: Rc<str> = if v30 {
        let v32: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v29);
        let v34: Vec<u8> = v32.unwrap();
        let v35: Vec<u8> = method26(v34);
        let v37: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v35);
        let v39: std::string::String = v37.unwrap();
        let v41: Rc<str> = Rc::<str>::from(String::as_str(&v39));
        v41
    } else {
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v42
    };
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v45: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v43.split(&*v44).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v46: Rc<str> = v45.borrow()[0i32 as usize].clone();
    let v47: Rc<str> = Rc::<str>::from((v46).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v48: i32 = 64i32;
    let v49: bool = chain_reaches_27(v1, v0, v48);
    (v7, v11, v12, v16, v27, v28, v47, v49)
}
fn rechain_run_0() -> i32 {
    let v0: i32 = command_line_argument_count_1();
    let v1: bool = 2i32 < v0;
    let mut v6: US0 = if v1 {
        let v2: i32 = 2i32;
        let mut v3: Rc<str> = command_line_argument_2(v2);
        US0::US0_Some(v3)
    } else {
        US0::US0_None
    };
    let v10: Rc<str> = match v6 {
        US0::US0_None => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US0::US0_Some(v7) => {
            let v7: Rc<str> = v7;
            v7
        }
    };
    let v11: i32 = command_line_argument_count_1();
    let v12: bool = 3i32 < v11;
    let v17: US0 = if v12 {
        let v13: i32 = 3i32;
        let v14: Rc<str> = command_line_argument_2(v13);
        US0::US0_Some(v14)
    } else {
        US0::US0_None
    };
    let v21: Rc<str> = match v17 {
        US0::US0_None => {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v19
        }
        US0::US0_Some(v18) => {
            let v18: Rc<str> = v18;
            v18
        }
    };
    let v22: bool = &*v10 == "";
    let v24: bool = if v22 {
        true
    } else {
        let v23: bool = &*v21 == "";
        v23
    };
    let v46: US1 = if v24 {
        US1::US1_None
    } else {
        let v26: i32 = command_line_argument_count_1();
        let v27: bool = 4i32 < v26;
        let v32: US0 = if v27 {
            let v28: i32 = 4i32;
            let v29: Rc<str> = command_line_argument_2(v28);
            US0::US0_Some(v29)
        } else {
            US0::US0_None
        };
        let v36: Rc<str> = match v32 {
            US0::US0_None => {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v34
            }
            US0::US0_Some(v33) => {
                let v33: Rc<str> = v33;
                v33
            }
        };
        let v37: bool = &*v36 == "";
        if v37 {
            let v38: US2 = US2::US2_AfterAny;
            US1::US1_Some(v38)
        } else {
            let v40: bool = &*v36 == "--ok";
            if v40 {
                let v41: US2 = US2::US2_AfterOk;
                US1::US1_Some(v41)
            } else {
                US1::US1_None
            }
        }
    };
    match v46 {
        US1::US1_None => {
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: job usage: job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept)"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v47);
            2i32
        }
        US1::US1_Some(v48) => {
            let v48: US2 = v48;
            let (mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (bool, bool, bool, bool, bool, Rc<str>, Rc<str>, bool) = facts_of_3(v10.clone(), v21.clone());
            let v57: bool = v49 == false;
            let v79: US7 = if v57 {
                let v58: US8 = US8::US8_UnknownJob;
                US7::US7_Refused(v58)
            } else {
                let v60: bool = v50 == false;
                if v60 {
                    let v61: US8 = US8::US8_UnknownPredecessor;
                    US7::US7_Refused(v61)
                } else {
                    if v51 {
                        let v63: US8 = US8::US8_SelfAfter;
                        US7::US7_Refused(v63)
                    } else {
                        let v65: bool = v52 == false;
                        if v65 {
                            let v66: US8 = US8::US8_NotChained;
                            US7::US7_Refused(v66)
                        } else {
                            let v68: bool = v53 == false;
                            if v68 {
                                let v69: US8 = US8::US8_NotWaiting(v54);
                                US7::US7_Refused(v69)
                            } else {
                                if v56 {
                                    let v71: US8 = US8::US8_WouldCycle;
                                    US7::US7_Refused(v71)
                                } else {
                                    US7::US7_Repointed(v55)
                                }
                            }
                        }
                    }
                }
            };
            match v79 {
                US7::US7_Refused(v80) => {
                    let v80: US8 = v80;
                    let v97: Rc<str> = match v80 {
                        US8::US8_NotChained => {
                            let v84: Rc<str> = string_concat(&[&*v10, " was not started with --after or --after-ok; stop it and start it again with --after"]);
                            v84
                        }
                        US8::US8_NotWaiting(v85) => {
                            let v85: Rc<str> = v85;
                            let v88: Rc<str> = string_concat(&[&*v10, " is ", &*v85, ", no longer waiting for its predecessor; nothing changed"]);
                            v88
                        }
                        US8::US8_SelfAfter => {
                            let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a job cannot wait for itself"); } LIT.with(|lit| lit.clone()) };
                            v83
                        }
                        US8::US8_UnknownJob => {
                            let v81: Rc<str> = string_concat(&["unknown job id ", &*v10]);
                            v81
                        }
                        US8::US8_UnknownPredecessor => {
                            let v82: Rc<str> = string_concat(&["unknown predecessor id ", &*v21]);
                            v82
                        }
                        US8::US8_WouldCycle => {
                            let v91: Rc<str> = string_concat(&[&*v21, " already waits, directly or down its chain, for ", &*v10, "; nothing changed"]);
                            v91
                        }
                    };
                    let v98: Rc<str> = string_concat(&["rig error: job after refused: ", &*v97]);
                    eprintln!("{}", v98);
                    2i32
                }
                US7::US7_Repointed(v99) => {
                    let v99: Rc<str> = v99;
                    let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                    let v101: Rc<str> = job_file_8(v10.clone(), v100);
                    let v102: Rc<str> = string_concat(&[&*v21, "\t"]);
                    let v106: Rc<str> = match &v48 {
                        US2::US2_AfterAny => {
                            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("any"); } LIT.with(|lit| lit.clone()) };
                            v103
                        }
                        US2::US2_AfterOk => {
                            let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                            v104
                        }
                    };
                    let v107: Rc<str> = string_concat(&[&*v102, &*v106]);
                    std::fs::write(&*v101, &*v107).unwrap();
                    let v112: Rc<str> = string_concat(&["rig job after ok id=", &*v10, " after=", &*v21, " after_mode="]);
                    let v116: Rc<str> = match v48 {
                        US2::US2_AfterAny => {
                            let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("any"); } LIT.with(|lit| lit.clone()) };
                            v113
                        }
                        US2::US2_AfterOk => {
                            let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                            v114
                        }
                    };
                    let v119: Rc<str> = string_concat(&[&*v112, &*v116, " previous=", &*v99]);
                    println!("{}", v119);
                    0i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        rechain_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_job_rechain_run() -> i32 {
    closure0()()
}
