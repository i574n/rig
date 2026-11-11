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
    US1_AfterAny,
    US1_AfterOk,
}
#[derive(Clone)]
enum US2 {
    US2_PredecessorLive,
    US2_PredecessorSettled(Rc<str>, i32),
}
#[derive(Clone)]
enum US0 {
    US0_NoChain,
    US0_Chained(Rc<str>, US1, US2),
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
    US6_KeepWaiting,
    US6_StartSupervisor,
    US6_SettleStopped(Rc<str>),
    US6_SettleSkipped(Rc<str>),
}
pub struct Mut0 { pub l0: Rc<str> }
fn joined_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_5() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_JOBS_HOME").or_else(|| std::env::var_os("KIT_JOBS_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_5();
    joined_3(v1, v0)
}
fn job_file_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_4(v0);
    joined_3(v2, v1)
}
fn text_or_empty_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn file_exists_7(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn state_of_parts_9(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US3 {
    let v3: bool = &*v0 == "pending";
    if v3 {
        let v4: US4 = US4::US4_JobPending;
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
fn shared_state_8(mut v0: Rc<str>) -> US3 {
    let (mut v1, mut v2, mut v3): (Rc<str>, i32, Rc<str>) = rig_job_state::rig_job_state_parts(&*v0);
    state_of_parts_9(v1, v2, v3)
}
fn state_settled_10(mut v0: US3) -> bool {
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
fn live_word_12(mut v0: US4) -> Rc<str> {
    match v0 {
        US4::US4_JobPending => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US4::US4_JobPlaced => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placed"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US4::US4_JobRunning => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v2
        }
    }
}
fn state_word_11(mut v0: US3) -> Rc<str> {
    match v0 {
        US3::US3_Live(v1) => {
            let v1: US4 = v1;
            live_word_12(v1)
        }
        US3::US3_Settled(v3) => {
            let v3: US5 = v3;
            match v3 {
                US5::US5_JobExpiredInQueue => {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                    v14
                }
                US5::US5_JobFinished(v4) => {
                    let v4: i32 = v4;
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
fn settled_exit_14(mut v0: US5) -> i32 {
    match v0 {
        US5::US5_JobExpiredInQueue => {
            75i32
        }
        US5::US5_JobFinished(v1) => {
            let v1: i32 = v1;
            v1
        }
        US5::US5_JobLost => {
            125i32
        }
        US5::US5_JobMemoryCapped => {
            137i32
        }
        US5::US5_JobSkipped(v3) => {
            let v3: Rc<str> = v3;
            125i32
        }
        US5::US5_JobSpawnFailed(v2) => {
            let v2: Rc<str> = v2;
            125i32
        }
        US5::US5_JobStopped => {
            143i32
        }
        US5::US5_JobTimedOut => {
            124i32
        }
    }
}
fn state_exit_13(mut v0: US3) -> i32 {
    match v0 {
        US3::US3_Live(v1) => {
            let v1: US4 = v1;
            3i32
        }
        US3::US3_Settled(v2) => {
            let v2: US5 = v2;
            settled_exit_14(v2)
        }
    }
}
fn queued_of_1(mut v0: Rc<str>) -> (Rc<str>, bool, US0) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_2(v0.clone(), v1);
    let v3: Rc<str> = text_or_empty_6(v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = job_file_2(v0.clone(), v4);
    let mut v6: bool = file_exists_7(v5);
    let v7: bool = &*v3 == "";
    let (mut v36, mut v37, mut v38): (Rc<str>, bool, US0) = if v7 {
        let v8: US0 = US0::US0_NoChain;
        (v0, v6, v8)
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v11: i32 = (v10.clone().borrow().len() as i32);
        let v12: bool = 0i32 < v11;
        let v16: Rc<str> = if v12 {
            let v13: Rc<str> = v10.borrow()[0i32 as usize].clone();
            let v14: Rc<str> = Rc::<str>::from((v13).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            v14
        } else {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v15
        };
        let v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v18: i32 = (v17.clone().borrow().len() as i32);
        let v19: bool = 1i32 < v18;
        let v23: Rc<str> = if v19 {
            let v20: Rc<str> = v17.borrow()[1i32 as usize].clone();
            let v21: Rc<str> = Rc::<str>::from((v20).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            v21
        } else {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v22
        };
        let v24: bool = &*v23 == "ok";
        let v27: US1 = if v24 {
            US1::US1_AfterOk
        } else {
            US1::US1_AfterAny
        };
        let v28: US3 = shared_state_8(v16.clone());
        let v29: bool = state_settled_10(v28.clone());
        let v34: US2 = if v29 {
            let v30: Rc<str> = state_word_11(v28.clone());
            let v31: i32 = state_exit_13(v28);
            US2::US2_PredecessorSettled(v30, v31)
        } else {
            US2::US2_PredecessorLive
        };
        let v35: US0 = US0::US0_Chained(v16, v27, v34);
        (v0, v6, v35)
    };
    (v36, v37, v38)
}
fn method16(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_15(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method16(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn now_ms_18() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn concat5_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn text_written_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2
}
fn settled_in_queue_17(mut v0: Rc<str>, mut v1: US6) -> () {
    match v1 {
        US6::US6_SettleSkipped(v15) => {
            let v15: Rc<str> = v15;
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = job_file_2(v0, v16);
            let v18: u64 = now_ms_18();
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
            let v21: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = concat5_19(v19, v22.clone(), v15, v23, v24);
            let v26: Rc<str> = concat5_19(v20, v22.clone(), v21, v22, v25);
            let v27: Rc<str> = text_written_20(v17, v26);
            ()
        }
        US6::US6_SettleStopped(v2) => {
            let v2: Rc<str> = v2;
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let v4: Rc<str> = job_file_2(v0, v3);
            let v5: u64 = now_ms_18();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
            let v8: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = concat5_19(v6, v9.clone(), v2, v10, v11);
            let v13: Rc<str> = concat5_19(v7, v9.clone(), v8, v9, v12);
            let v14: Rc<str> = text_written_20(v4, v13);
            ()
        }
        _ => {
            ()
        }
    }
}
fn queue_decide_0(mut v0: Rc<str>) -> i32 {
    let (mut v1, mut v2, mut v3): (Rc<str>, bool, US0) = queued_of_1(v0.clone());
    let v36: US6 = if v2 {
        let v9: Rc<str> = match v3 {
            US0::US0_Chained(v5, v6, v7) => {
                let v5: Rc<str> = v5;
                let mut v6: US1 = v6;
                let v7: US2 = v7;
                v5
            }
            US0::US0_NoChain => {
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a slot"); } LIT.with(|lit| lit.clone()) };
                v4
            }
        };
        let v10: Rc<str> = string_concat(&["stopped on request while waiting for ", &*v9]);
        US6::US6_SettleStopped(v10)
    } else {
        match v3 {
            US0::US0_Chained(v13, v14, v15) => {
                let v13: Rc<str> = v13;
                let v14: US1 = v14;
                let v15: US2 = v15;
                match &v15 {
                    US2::US2_PredecessorLive => {
                        US6::US6_KeepWaiting
                    }
                    _ => {
                        match v14 {
                            US1::US1_AfterAny => {
                                US6::US6_StartSupervisor
                            }
                            US1::US1_AfterOk => {
                                match v15 {
                                    US2::US2_PredecessorSettled(v18, v19) => {
                                        let v18: Rc<str> = v18;
                                        let v19: i32 = v19;
                                        let v20: bool = v19 == 0i32;
                                        if v20 {
                                            US6::US6_StartSupervisor
                                        } else {
                                            let v26: Rc<str> = format_real_15(v19);
                                            let v27: Rc<str> = string_concat(&["--after-ok: ", &*v13, " ended ", &*v18, " exit ", &*v26]);
                                            US6::US6_SettleSkipped(v27)
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                    }
                }
            }
            US0::US0_NoChain => {
                US6::US6_StartSupervisor
            }
        }
    };
    match &v36 {
        US6::US6_KeepWaiting => {
            1i32
        }
        US6::US6_StartSupervisor => {
            let v37: i32 = rig_job_slot::rig_job_slot_held_step(&v0);
            let v38: bool = v37 != 0i32;
            if v38 {
                v37
            } else {
                let v39: i32 = rig_job_gate::rig_job_gate_held_step(&v0);
                v39
            }
        }
        _ => {
            settled_in_queue_17(v0.clone(), v36);
            rig_job_slot::rig_job_slot_ticket_dropped(&v0);
            rig_job_gate::rig_job_gate_window_dropped(&v0);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        queue_decide_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_job_queue_decide(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
