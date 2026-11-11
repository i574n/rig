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
enum US2 {
    US2_JobPending,
    US2_JobRunning,
    US2_JobPlaced,
}
#[derive(Clone)]
enum US3 {
    US3_JobFinished(i32),
    US3_JobTimedOut,
    US3_JobStopped,
    US3_JobSpawnFailed(Rc<str>),
    US3_JobSkipped(Rc<str>),
    US3_JobLost,
    US3_JobMemoryCapped,
    US3_JobExpiredInQueue,
}
#[derive(Clone)]
enum US1 {
    US1_Live(US2),
    US1_Settled(US3),
}
#[derive(Clone)]
enum US4 {
    US4_Ok,
    US4_Error(Rc<str>),
}
fn joined_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_5() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_JOBS_HOME").or_else(|| std::env::var_os("KIT_JOBS_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_5();
    joined_3(v1, v0)
}
fn job_file_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_4(v0);
    joined_3(v2, v1)
}
fn text_or_empty_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method8(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_7(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method8(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US0> = closure1();
    let mut v4: Option<US0> = v2.map(|x| v3(x));
    let mut v6: US0 = US0::US0_None;
    let mut v7: US0 = v6.clone();
    let mut v8: US0 = v4.unwrap_or(v7);
    match v8 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn now_ms_9() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_1(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_2(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_6(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = job_file_2(v0.clone(), v4);
    let mut v6: Rc<str> = text_or_empty_6(v5);
    let mut v7: u64 = decimal_u64_7(v6);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = job_file_2(v0, v8);
    let v10: Rc<str> = text_or_empty_6(v9);
    let v11: u64 = decimal_u64_7(v10);
    let v12: u64 = now_ms_9();
    (v3, v7, v11, v12)
}
fn state_of_parts_11(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US1 {
    let mut v3: bool = &*v0 == "pending";
    if v3 {
        let mut v4: US2 = US2::US2_JobPending;
        US1::US1_Live(v4)
    } else {
        let mut v6: bool = &*v0 == "running";
        if v6 {
            let mut v7: US2 = US2::US2_JobRunning;
            US1::US1_Live(v7)
        } else {
            let v9: bool = &*v0 == "placed";
            if v9 {
                let v10: US2 = US2::US2_JobPlaced;
                US1::US1_Live(v10)
            } else {
                let v12: bool = &*v0 == "finished";
                if v12 {
                    let v13: US3 = US3::US3_JobFinished(v1);
                    US1::US1_Settled(v13)
                } else {
                    let v15: bool = &*v0 == "timed-out";
                    if v15 {
                        let v16: US3 = US3::US3_JobTimedOut;
                        US1::US1_Settled(v16)
                    } else {
                        let v18: bool = &*v0 == "stopped";
                        if v18 {
                            let v19: US3 = US3::US3_JobStopped;
                            US1::US1_Settled(v19)
                        } else {
                            let v21: bool = &*v0 == "lost";
                            if v21 {
                                let v22: US3 = US3::US3_JobLost;
                                US1::US1_Settled(v22)
                            } else {
                                let v24: bool = &*v0 == "memory-capped";
                                if v24 {
                                    let v25: US3 = US3::US3_JobMemoryCapped;
                                    US1::US1_Settled(v25)
                                } else {
                                    let v27: bool = &*v0 == "expired-in-queue";
                                    if v27 {
                                        let v28: US3 = US3::US3_JobExpiredInQueue;
                                        US1::US1_Settled(v28)
                                    } else {
                                        let v30: bool = &*v0 == "skipped";
                                        if v30 {
                                            let v31: US3 = US3::US3_JobSkipped(v2);
                                            US1::US1_Settled(v31)
                                        } else {
                                            let v33: US3 = US3::US3_JobSpawnFailed(v2);
                                            US1::US1_Settled(v33)
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
fn job_state_of_10(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US1 {
    let mut v4: bool = &*v0 != "";
    if v4 {
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v6: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
        state_of_parts_11(v5, v6, v7)
    } else {
        let v9: bool = v1 > 0u64;
        if v9 {
            let v10: u64 = v1.wrapping_add(15000u64);
            let v11: bool = v10 < v3;
            if v11 {
                let v12: US3 = US3::US3_JobLost;
                US1::US1_Settled(v12)
            } else {
                let v14: US2 = US2::US2_JobRunning;
                US1::US1_Live(v14)
            }
        } else {
            let v17: u64 = v2.wrapping_add(15000u64);
            let v18: bool = v17 < v3;
            if v18 {
                let v19: US3 = US3::US3_JobLost;
                US1::US1_Settled(v19)
            } else {
                let v21: US2 = US2::US2_JobPending;
                US1::US1_Live(v21)
            }
        }
    }
}
fn process_alive_12(mut v0: u64) -> bool {
    let mut v1: bool = rig_process_alive::rig_process_alive(v0);
    v1
}
fn file_exists_13(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn last_write_ms_14(mut v0: Rc<str>) -> u64 {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v0);
    let v2: bool = v1.is_ok();
    let mut v3: bool = v2 == false;
    if v3 {
        0u64
    } else {
        let mut v4: std::fs::Metadata = v1.unwrap();
        let mut v5: std::io::Result<std::time::SystemTime> = v4.modified();
        let mut v6: bool = v5.is_ok();
        if v6 {
            let mut v7: std::time::SystemTime = v5.unwrap();
            let mut v8: std::time::Duration = v7.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
            let v9: u64 = v8.as_secs();
            let v10: u64 = v9.wrapping_mul(1000u64);
            let v11: u32 = v8.subsec_millis();
            let v12: u64 = (v11 as u64);
            let v13: u64 = v10.wrapping_add(v12);
            v13
        } else {
            0u64
        }
    }
}
fn text_written_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2
}
fn concat5_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn file_exists_17(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn closure2() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> Rc<str>> = Rc::new(move |mut v0: std::io::Error| -> Rc<str> {
        let mut v1: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v0)).s() });
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    closure2()
}
fn ok_c3() -> Rc<dyn Fn() -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US4> = Rc::new(move || -> US4 {
        US4::US4_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method19() -> Rc<dyn Fn() -> US4> {
    ok_c3()
}
fn error_c4() -> Rc<dyn Fn(Rc<str>) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US4> = Rc::new(move |mut v0: Rc<str>| -> US4 {
        US4::US4_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method20() -> Rc<dyn Fn(Rc<str>) -> US4> {
    error_c4()
}
fn hold_orphaned_21(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-home"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_2(v0, v1);
    let mut v3: Rc<str> = text_or_empty_6(v2);
    let mut v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v5: Rc<str> = jobs_root_5();
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hosts"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = joined_3(v5, v6);
    let mut v8: Rc<str> = joined_3(v7, v4.clone());
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = joined_3(v8, v9);
    let v11: Rc<str> = text_or_empty_6(v10);
    let v12: u64 = decimal_u64_7(v11);
    let v13: bool = rig_process_alive::rig_process_home_abandoned(&v4, v12);
    v13
}
fn orphan_settled_0(mut v0: Rc<str>) -> bool {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_1(v0.clone());
    let mut v5: US1 = job_state_of_10(v1.clone(), v2, v3, v4);
    match v5 {
        US1::US1_Settled(v6) => {
            let mut v6: US3 = v6;
            match v6 {
                US3::US3_JobLost => {
                    let mut v7: bool = &*v1 == "";
                    if v7 {
                        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
                        let v9: Rc<str> = job_file_2(v0.clone(), v8);
                        let v10: Rc<str> = text_or_empty_6(v9);
                        let v11: u64 = decimal_u64_7(v10);
                        let v12: Rc<str> = jobs_root_5();
                        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                        let v14: Rc<str> = joined_3(v12, v13);
                        let v15: Rc<str> = joined_3(v14, v0.clone());
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
                        let v17: Rc<str> = job_file_2(v0.clone(), v16);
                        let v18: Rc<str> = text_or_empty_6(v17);
                        let v19: u64 = decimal_u64_7(v18);
                        let v20: bool = v11 == 0u64;
                        let v23: bool = if v20 {
                            true
                        } else {
                            let v21: bool = process_alive_12(v11);
                            let v22: bool = v21 == false;
                            v22
                        };
                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reaped"); } LIT.with(|lit| lit.clone()) };
                        let v25: Rc<str> = job_file_2(v0.clone(), v24);
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                        let v27: Rc<str> = job_file_2(v0.clone(), v26);
                        let v30: bool = if v23 {
                            let v28: bool = file_exists_13(v25.clone());
                            let v29: bool = v28 == false;
                            v29
                        } else {
                            false
                        };
                        let v37: Rc<str> = if v30 {
                            let v31: Rc<str> = text_or_empty_6(v27.clone());
                            let v32: u64 = decimal_u64_7(v31);
                            let v33: u64 = last_write_ms_14(v27);
                            let v34: Rc<str> = rig_process_alive::rig_process_lost_group_reaped(v32, v33);
                            text_written_15(v25, v34)
                        } else {
                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v36
                        };
                        let v38: bool = v19 > 0u64;
                        let v41: bool = if v38 {
                            let v39: u64 = v19.wrapping_add(60000u64);
                            let v40: bool = v39 < v4;
                            v40
                        } else {
                            false
                        };
                        let v42: bool = v41 && v23;
                        if v42 {
                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                            let v44: Rc<str> = job_file_2(v0, v43);
                            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop requested and no live supervisor or host acted on it within 60 s"); } LIT.with(|lit| lit.clone()) };
                            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
                            let v47: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
                            let v48: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
                            let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v52: Rc<str> = concat5_16(v46, v49.clone(), v45, v50, v51);
                            let v53: Rc<str> = concat5_16(v47, v49.clone(), v48, v49.clone(), v52);
                            let v54: Rc<str> = text_written_15(v44, v53);
                            let v55: bool = file_exists_13(v15.clone());
                            if v55 {
                                let v56: bool = file_exists_17(v15.clone());
                                let mut v60: Result<(), std::io::Error> = if v56 {
                                    let v58: Result<(), std::io::Error> = std::fs::remove_file(&*v15);
                                    v58
                                } else {
                                    let v59: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
                                    v59
                                };
                                let mut v61: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method18();
                                let v63: Result<(), Rc<str>> = v60.map_err(|x| v61(x));
                                let mut v64: Rc<dyn Fn() -> US4> = method19();
                                let mut v65: Rc<dyn Fn(Rc<str>) -> US4> = method20();
                                let v66: US4 = match v63 { Ok(()) => v64(), Err(e) => v65(e) };
                                match v66 {
                                    US4::US4_Error(v67) => {
                                        let v67: Rc<str> = v67;
                                        let v68: Rc<str> = string_concat(&["file_system.file_delete: ", &*v67]);
                                        eprintln!("{}", v68);
                                        ()
                                    }
                                    US4::US4_Ok => {
                                        ()
                                    }
                                }
                            };
                            true
                        } else {
                            let v69: bool = file_exists_13(v15.clone());
                            let v71: bool = if v69 {
                                hold_orphaned_21(v0.clone())
                            } else {
                                false
                            };
                            if v71 {
                                let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                                let v73: Rc<str> = job_file_2(v0, v72);
                                let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held by a host whose owner session and host process are gone; it never started"); } LIT.with(|lit| lit.clone()) };
                                let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                                let v76: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
                                let v77: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
                                let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v81: Rc<str> = concat5_16(v75, v78.clone(), v74, v79, v80);
                                let v82: Rc<str> = concat5_16(v76, v78.clone(), v77, v78.clone(), v81);
                                let v83: Rc<str> = text_written_15(v73, v82);
                                let v84: bool = file_exists_17(v15.clone());
                                let mut v88: Result<(), std::io::Error> = if v84 {
                                    let v86: Result<(), std::io::Error> = std::fs::remove_file(&*v15);
                                    v86
                                } else {
                                    let v87: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
                                    v87
                                };
                                let mut v89: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method18();
                                let v91: Result<(), Rc<str>> = v88.map_err(|x| v89(x));
                                let mut v92: Rc<dyn Fn() -> US4> = method19();
                                let mut v93: Rc<dyn Fn(Rc<str>) -> US4> = method20();
                                let v94: US4 = match v91 { Ok(()) => v92(), Err(e) => v93(e) };
                                match v94 {
                                    US4::US4_Error(v95) => {
                                        let v95: Rc<str> = v95;
                                        let v96: Rc<str> = string_concat(&["file_system.file_delete: ", &*v95]);
                                        eprintln!("{}", v96);
                                        ()
                                    }
                                    US4::US4_Ok => {
                                        ()
                                    }
                                };
                                true
                            } else {
                                false
                            }
                        }
                    } else {
                        false
                    }
                }
                _ => {
                    false
                }
            }
        }
        _ => {
            false
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        orphan_settled_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_job_orphan_settled(v0: &str) -> bool {
    closure0()(Rc::<str>::from(v0))
}
