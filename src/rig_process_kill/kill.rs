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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_SwitchTree,
    US2_SwitchMine,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum US4 {
    US4_ValuedTheirs,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US4),
    US3_None,
}
#[derive(Clone)]
enum US5 {
    US5_AnyProcess,
    US5_MineOnly(std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, u64, Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_Allowed(u64),
    US6_OwnProcess,
    US6_OwnAncestor,
    US6_NotAProcessId,
    US6_NotYours(Rc<str>),
}
#[derive(Clone)]
enum US7 {
    US7_Some(u64),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_YourJob(Rc<str>),
    US8_TheirJob(Rc<str>, Rc<str>),
    US8_NoOwningJob,
}
#[derive(Clone)]
enum US9 {
    US9_AgentSession(Rc<str>, u64),
    US9_KitJob(Rc<str>),
    US9_CellsVm(u64),
    US9_JobLeftover(Rc<str>),
    US9_Orphan,
    US9_NotATool,
}
#[derive(Clone)]
enum US10 {
    US10_Some(i32),
    US10_None,
}
#[derive(Clone)]
enum US11 {
    US11_EveryProcess,
    US11_SessionTree(u64),
    US11_NoSession,
}
#[derive(Clone)]
enum US12 {
    US12_NoSuchProcess,
    US12_StoppedOf(u64, u64),
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US13 {
    US13_Succeeded,
    US13_Failed,
}
fn now_ms_0() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn command_line_argument_count_4() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_5(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_3(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_4();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_5(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_3(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn kill_words_2() -> Rc<UH0> {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_3(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            v3
        }
        UH0::UH0_Nil => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn switch_applied_8(mut v0: US2, mut v1: bool, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<UH0>) -> (bool, bool, Rc<str>, Rc<UH0>) {
    let (mut v9, mut v10, mut v11, mut v12): (bool, bool, Rc<str>, Rc<UH0>) = match &v0 {
        US2::US2_SwitchTree => {
            (true, v2, v3, v4)
        }
        _ => {
            let (mut v5, mut v6, mut v7, mut v8): (bool, bool, Rc<str>, Rc<UH0>) = match v0 {
                US2::US2_SwitchMine => {
                    (v1, true, v3, v4)
                }
                _ => unreachable!(),
            };
            (v5, v6, v7, v8)
        }
    };
    (v9, v10, v11, v12)
}
fn kill_flags_read_6(mut v0: Rc<UH0>, mut v1: bool, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<UH0>) -> (bool, bool, Rc<str>, Rc<UH0>) {
    loop {
        match &*v0 {
            UH0::UH0_Nil => {
                let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v6: Rc<UH0> = method7(v4, v5);
                return (v1, v2, v3, v6);
            }
            _ => {
                match Rc::unwrap_or_clone(v0) {
                    UH0::UH0_Cons(v7, v8) => {
                        let v7: Rc<str> = v7;
                        let v8: Rc<UH0> = v8;
                        let mut v9: bool = &*v7 == "--tree";
                        let v17: US1 = if v9 {
                            let v10: US2 = US2::US2_SwitchTree;
                            US1::US1_Some(v10)
                        } else {
                            let v12: bool = &*v7 == "--mine";
                            if v12 {
                                let v13: US2 = US2::US2_SwitchMine;
                                US1::US1_Some(v13)
                            } else {
                                US1::US1_None
                            }
                        };
                        let v18: bool = &*v7 == "--theirs";
                        let v22: US3 = if v18 {
                            let v19: US4 = US4::US4_ValuedTheirs;
                            US3::US3_Some(v19)
                        } else {
                            US3::US3_None
                        };
                        match v17 {
                            US1::US1_Some(v23) => {
                                let v23: US2 = v23;
                                let (mut v24, mut v25, mut v26, mut v27): (bool, bool, Rc<str>, Rc<UH0>) = switch_applied_8(v23, v1, v2, v3, v4);
                                (v0, v1, v2, v3, v4) = (v8, v24, v25, v26, v27);
                                continue;
                            }
                            _ => {
                                match v22 {
                                    US3::US3_Some(v32) => {
                                        let v32: US4 = v32;
                                        match &*v8 {
                                            UH0::UH0_Cons(v33, v34) => {
                                                let v33: Rc<str> = v33.clone();
                                                let v34: Rc<UH0> = v34.clone();
                                                (v0, v3) = (v34, v33);
                                                continue;
                                            }
                                            _ => {
                                                match Rc::unwrap_or_clone(v8) {
                                                    UH0::UH0_Nil => {
                                                        let v39: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                        v0 = v39;
                                                        continue;
                                                    }
                                                    _ => unreachable!(),
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        let v52: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v4));
                                        (v0, v4) = (v8, v52);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
fn process_rows_sampled_9(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> {
    let v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = rig_machine_sample::rig_machine_sampled(v0);
    v1
}
fn get_environment_variable_11(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn variable_of_10(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = string_concat(&["RIG_", &*v0]);
    let mut v2: Rc<str> = get_environment_variable_11(v1);
    let mut v3: bool = &*v2 != "";
    if v3 {
        v2
    } else {
        let mut v4: Rc<str> = string_concat(&["KIT_", &*v0]);
        get_environment_variable_11(v4)
    }
}
fn ancestor_pids_12() -> Rc<str> {
    let mut v0: Rc<str> = { let system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let mut chain: Vec<u32> = Vec::new(); let mut current = sysinfo::Pid::from_u32(std::process::id()); while let Some(parent) = system.process(current).and_then(|process| process.parent()) { if chain.contains(&parent.as_u32()) { break; } chain.push(parent.as_u32()); current = parent; } std::rc::Rc::<str>::from(chain.iter().map(|pid| pid.to_string()).collect::<Vec<_>>().join(",")) };
    v0
}
fn method16(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US7> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US7> = Rc::new(move |mut v0: (u64)| -> US7 {
        let v1: u64 = (v0);
        US7::US7_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_15(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method16(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US7> = closure1();
    let mut v4: Option<US7> = v2.map(|x| v3(x));
    let v6: US7 = US7::US7_None;
    let v7: US7 = v6.clone();
    let v8: US7 = v4.unwrap_or(v7);
    match v8 {
        US7::US7_None => {
            0u64
        }
        US7::US7_Some(v9) => {
            let mut v9: u64 = v9;
            v9
        }
    }
}
fn stop_refusal_of_14(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>) -> US6 {
    let mut v3: bool = !v0.is_empty() && v0.bytes().all(|byte| byte.is_ascii_digit());
    if v3 {
        let mut v4: u64 = decimal_of_15(v0.clone());
        let v5: bool = v4 == v1;
        if v5 {
            US6::US6_OwnProcess
        } else {
            let v8: Rc<str> = string_concat(&[",", &*v0, ","]);
            let v10: Rc<str> = string_concat(&[",", &*v2, ","]);
            let v11: bool = v10.contains(&*v8);
            if v11 {
                US6::US6_OwnAncestor
            } else {
                US6::US6_Allowed(v4)
            }
        }
    } else {
        US6::US6_NotAProcessId
    }
}
fn mine_verdict_18(mut v0: bool, mut v1: US8, mut v2: Rc<str>) -> US0 {
    match v1 {
        US8::US8_NoOwningJob => {
            if v0 {
                US0::US0_None
            } else {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no kit job started it and it is outside this agent session"); } LIT.with(|lit| lit.clone()) };
                US0::US0_Some(v21)
            }
        }
        US8::US8_TheirJob(v5, v6) => {
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: bool = &*v2 == &*v5;
            if v7 {
                US0::US0_None
            } else {
                let v10: Rc<str> = string_concat(&["it belongs to job ", &*v5, " by "]);
                let v11: bool = &*v6 == "";
                let v13: Rc<str> = if v11 {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no author"); } LIT.with(|lit| lit.clone()) };
                    v12
                } else {
                    v6
                };
                let v17: Rc<str> = string_concat(&[&*v10, &*v13, "; pass --theirs ", &*v5, " to stop it anyway"]);
                US0::US0_Some(v17)
            }
        }
        US8::US8_YourJob(v3) => {
            let mut v3: Rc<str> = v3;
            US0::US0_None
        }
    }
}
fn job_left_by_21(mut v0: u64, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = rig_machine_sample::rig_machine_job_left(v0, v1);
    v2
}
fn left_or_orphan_20(mut v0: u64, mut v1: u64) -> US9 {
    let mut v2: Rc<str> = job_left_by_21(v0, v1);
    let mut v3: bool = &*v2 == "";
    if v3 {
        US9::US9_Orphan
    } else {
        US9::US9_JobLeftover(v2)
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US10> {
    Rc::new(move |mut v2: Rc<UH0>| -> US10 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US10::US10_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US10> = method23(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US10::US10_None
            }
        }
    })
}
fn method23(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US10> {
    closure2(v0, v1)
}
fn agent_client_22(mut v0: Rc<str>) -> bool {
    let v1: i32 = 0i32;
    let mut v2: Rc<dyn Fn(Rc<UH0>) -> US10> = method23(v0, v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("codex"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5));
    let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v6));
    let v8: US10 = v2(v7);
    let mut v9: bool = match v8 {
        US10::US10_None => {
            true
        }
        _ => {
            false
        }
    };
    let v10: bool = v9 != true;
    v10
}
fn cells_vm_at_24(mut v0: Rc<str>) -> bool {
    let v1: bool = rig_machine_sample::rig_machine_cells_vm(&*v0);
    v1
}
fn owner_walk_19(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: i32, mut v2: i32) -> US9 {
    loop {
        let mut v3: bool = v1 < 0i32;
        let v5: bool = if v3 {
            true
        } else {
            let mut v4: bool = v2 >= 40i32;
            v4
        };
        if v5 {
            return US9::US9_Orphan;
        } else {
            let v7: Rc<str> = v0[v1 as usize].2.clone();
            let v8: Rc<str> = v0[v1 as usize].3.clone();
            let mut v9: Rc<str> = std::rc::Rc::<str>::from((v8).split_whitespace().skip_while(|word| *word != "supervise").nth(1).filter(|word| word.starts_with("job-")).unwrap_or(""));
            let v10: bool = agent_client_22(v7.clone());
            if v10 {
                let v11: u64 = v0[v1 as usize].0;
                return US9::US9_AgentSession(v7, v11);
            } else {
                let v13: bool = &*v9 != "";
                if v13 {
                    return US9::US9_KitJob(v9);
                } else {
                    let v15: u64 = v0[v1 as usize].1;
                    let v16: i32 = v0.iter().position(|row| row.0 == v15).map(|at| at as i32).unwrap_or(-1);
                    let v17: bool = v16 < 0i32;
                    if v17 {
                        let v18: bool = &*v7 == "erl";
                        let v21: bool = if v18 {
                            let v19: Rc<str> = v0[v1 as usize].3.clone();
                            cells_vm_at_24(v19)
                        } else {
                            false
                        };
                        if v21 {
                            let v22: u64 = v0[v1 as usize].0;
                            return US9::US9_CellsVm(v22);
                        } else {
                            let v24: u64 = v0[v1 as usize].1;
                            let v25: u64 = v0[v1 as usize].5;
                            return left_or_orphan_20(v24, v25);
                        }
                    } else {
                        let v28: i32 = v2.wrapping_add(1i32);
                        (v1, v2) = (v16, v28);
                        continue;
                    }
                }
            }
        }
    }
}
fn joined_26(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_28() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_JOBS_HOME").or_else(|| std::env::var_os("KIT_JOBS_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_27(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_28();
    joined_26(v1, v0)
}
fn job_file_25(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_27(v0);
    joined_26(v2, v1)
}
fn text_or_empty_29(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn session_root_30(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: u64, mut v2: i32) -> u64 {
    loop {
        let mut v3: i32 = v0.iter().position(|row| row.0 == v1).map(|at| at as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        let v6: bool = if v4 {
            true
        } else {
            let v5: bool = v2 > 32i32;
            v5
        };
        if v6 {
            return 0u64;
        } else {
            let v7: Rc<str> = v0[v3 as usize].2.clone();
            let v8: bool = agent_client_22(v7);
            if v8 {
                return v1;
            } else {
                let mut v9: u64 = v0[v3 as usize].1;
                let v10: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v9, v10);
                continue;
            }
        }
    }
}
fn descends_from_31(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: u64, mut v2: u64, mut v3: i32) -> bool {
    loop {
        let mut v4: bool = v1 == v2;
        if v4 {
            return true;
        } else {
            let v5: bool = v1 == 0u64;
            let v7: bool = if v5 {
                true
            } else {
                let v6: bool = v3 > 64i32;
                v6
            };
            if v7 {
                return false;
            } else {
                let v8: i32 = v0.iter().position(|row| row.0 == v1).map(|at| at as i32).unwrap_or(-1);
                let mut v9: bool = v8 < 0i32;
                if v9 {
                    return false;
                } else {
                    let v10: u64 = v0[v8 as usize].1;
                    let v11: i32 = v3.wrapping_add(1i32);
                    (v1, v3) = (v10, v11);
                    continue;
                }
            }
        }
    }
}
fn job_owner_of_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US8 {
    let mut v3: bool = &*v0 == "";
    if v3 {
        US8::US8_NoOwningJob
    } else {
        let v5: bool = &*v2 != "";
        let v7: bool = if v5 {
            let v6: bool = &*v1 == &*v2;
            v6
        } else {
            false
        };
        if v7 {
            US8::US8_YourJob(v0)
        } else {
            US8::US8_TheirJob(v0, v1)
        }
    }
}
fn guard_refusal_17(mut v0: US5, mut v1: u64) -> US0 {
    match v0 {
        US5::US5_AnyProcess => {
            US0::US0_None
        }
        US5::US5_MineOnly(v3, v4, v5, v6) => {
            let mut v3: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = v3;
            let mut v4: u64 = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: i32 = v3.iter().position(|row| row.0 == v1).map(|at| at as i32).unwrap_or(-1);
            let v8: bool = v7 < 0i32;
            let v15: Rc<str> = if v8 {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v9
            } else {
                let v10: i32 = 0i32;
                let v11: US9 = owner_walk_19(v3.clone(), v7, v10);
                match v11 {
                    US9::US9_KitJob(v12) => {
                        let v12: Rc<str> = v12;
                        v12
                    }
                    _ => {
                        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v13
                    }
                }
            };
            let v16: bool = &*v15 == "";
            let v21: Rc<str> = if v16 {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v17
            } else {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("author"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = job_file_25(v15.clone(), v18);
                text_or_empty_29(v19)
            };
            let v22: i32 = 0i32;
            let v23: u64 = session_root_30(v3.clone(), v4, v22);
            let v24: bool = v23 == 0u64;
            let v27: US11 = if v24 {
                US11::US11_NoSession
            } else {
                US11::US11_SessionTree(v23)
            };
            let v33: bool = match v27 {
                US11::US11_EveryProcess => {
                    true
                }
                US11::US11_NoSession => {
                    false
                }
                US11::US11_SessionTree(v28) => {
                    let v28: u64 = v28;
                    let v29: i32 = 0i32;
                    descends_from_31(v3, v1, v28, v29)
                }
            };
            let v34: US8 = job_owner_of_32(v15, v21, v5);
            mine_verdict_18(v33, v34, v6)
        }
    }
}
fn process_stopped_34(mut v0: u64, mut v1: bool) -> (u64, u64) {
    let mut v2: (u64, u64) = { let system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let root = sysinfo::Pid::from_u32(v0 as u32); match system.process(root) { None => (0u64, 0u64), Some(_) => { let mut order = vec![root]; if v1 { let mut index = 0; while index < order.len() { let parent = order[index]; for (pid, process) in system.processes() { if process.parent() == Some(parent) && !order.contains(pid) { order.push(*pid); } } index += 1; } } for pid in order.iter().rev() { #[cfg(windows)] { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn TerminateProcess(process: *mut std::ffi::c_void, code: u32) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } unsafe { let handle = OpenProcess(1, 0, pid.as_u32()); if !handle.is_null() { TerminateProcess(handle, 1); CloseHandle(handle); } } } #[cfg(not(windows))] { if let Some(process) = system.process(*pid) { process.kill(); } } } std::thread::sleep(std::time::Duration::from_millis(200)); let after = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let survivors = order.iter().filter(|pid| { #[cfg(windows)] { let _ = &after; unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid.as_u32()); if handle.is_null() { false } else { let running = WaitForSingleObject(handle, 2000) == 258; CloseHandle(handle); running } } } #[cfg(not(windows))] { after.process(**pid).is_some_and(|p| p.status() != sysinfo::ProcessStatus::Zombie) } }).count(); ((order.len() - survivors) as u64, order.len() as u64) } } };
    v2
}
fn method36(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_35(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method36(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn stop_once_33(mut v0: Rc<str>, mut v1: u64, mut v2: bool) -> bool {
    let mut v3: (u64, u64) = process_stopped_34(v1, v2);
    let mut v4: u64 = v3.clone().0;
    let v5: u64 = v3.1;
    let v6: bool = v5 == 0u64;
    let mut v9: US12 = if v6 {
        US12::US12_NoSuchProcess
    } else {
        US12::US12_StoppedOf(v4, v5)
    };
    let v34: Rc<str> = match &v9 {
        US12::US12_NoSuchProcess => {
            let v11: Rc<str> = string_concat(&["stop ", &*v0, "  not found"]);
            v11
        }
        US12::US12_StoppedOf(v12, v13) => {
            let v12: u64 = *v12;
            let v13: u64 = *v13;
            let v14: bool = v12 != v13;
            if v14 {
                let v16: Rc<str> = format_real_35(v12);
                let v19: Rc<str> = format_real_35(v13);
                let v22: Rc<str> = string_concat(&["stop ", &*v0, "  stopped ", &*v16, " of ", &*v19, " processes"]);
                v22
            } else {
                let v23: bool = v13 == 1u64;
                if v23 {
                    let v25: Rc<str> = string_concat(&["stop ", &*v0, "  stopped"]);
                    v25
                } else {
                    let v28: Rc<str> = format_real_35(v13);
                    let v30: Rc<str> = string_concat(&["stop ", &*v0, "  stopped (tree: ", &*v28, " processes)"]);
                    v30
                }
            }
        }
    };
    println!("{}", v34);
    match v9 {
        US12::US12_NoSuchProcess => {
            false
        }
        US12::US12_StoppedOf(v35, v36) => {
            let v35: u64 = v35;
            let v36: u64 = v36;
            let v37: bool = v35 == v36;
            v37
        }
    }
}
fn stops_from_13(mut v0: Rc<UH0>, mut v1: bool, mut v2: US5, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64) -> (u64, u64) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v7, v8) => {
                let v7: Rc<str> = v7;
                let v8: Rc<UH0> = v8;
                let mut v9: US6 = stop_refusal_of_14(v7.clone(), v3, v4.clone());
                match &v9 {
                    US6::US6_Allowed(v10) => {
                        let v10: u64 = *v10;
                        let v11: US0 = guard_refusal_17(v2.clone(), v10);
                        match v11 {
                            US0::US0_None => {
                                let v19: bool = stop_once_33(v7, v10, v1);
                                if v19 {
                                    let v20: u64 = v5.wrapping_add(1u64);
                                    (v0, v5) = (v8, v20);
                                    continue;
                                } else {
                                    let v23: u64 = v6.wrapping_add(1u64);
                                    (v0, v6) = (v8, v23);
                                    continue;
                                }
                            }
                            US0::US0_Some(v12) => {
                                let v12: Rc<str> = v12;
                                let v15: Rc<str> = string_concat(&["stop ", &*v7, "  refused: ", &*v12]);
                                println!("{}", v15);
                                let v16: u64 = v6.wrapping_add(1u64);
                                (v0, v6) = (v8, v16);
                                continue;
                            }
                        }
                    }
                    _ => {
                        let v45: Rc<str> = match v9 {
                            US6::US6_NotAProcessId => {
                                let v37: Rc<str> = string_concat(&["stop ", &*v7, "  refused: not a process id"]);
                                v37
                            }
                            US6::US6_NotYours(v38) => {
                                let v38: Rc<str> = v38;
                                let v41: Rc<str> = string_concat(&["stop ", &*v7, "  refused: ", &*v38]);
                                v41
                            }
                            US6::US6_OwnAncestor => {
                                let v35: Rc<str> = string_concat(&["stop ", &*v7, "  refused: that is a parent of this kit (the calling shell)"]);
                                v35
                            }
                            US6::US6_OwnProcess => {
                                let v33: Rc<str> = string_concat(&["stop ", &*v7, "  refused: that is this kit process"]);
                                v33
                            }
                            _ => unreachable!(),
                        };
                        println!("{}", v45);
                        let v46: u64 = v6.wrapping_add(1u64);
                        (v0, v6) = (v8, v46);
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                return (v5, v6);
            }
        }
    }
}
fn decimal_text_37(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn elapsed_text_39(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1
}
fn local_offset_minutes_41() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_40() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_41();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1
}
fn receipt_line_38(mut v0: US13, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match v0 {
        US13::US13_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US13::US13_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let v10: Rc<str> = elapsed_text_39(v2);
    let v13: Rc<str> = clock_text_40();
    let v14: Rc<str> = string_concat(&[&*v6, "  ", &*v1, "  ", &*v10, "  ", &*v13]);
    println!("{}", v14);
    ()
}
fn stop_run_1(mut v0: u64) -> i32 {
    let v1: Rc<UH0> = kill_words_2();
    let mut v2: bool = false;
    let mut v3: bool = false;
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v6, mut v7, mut v8, mut v9): (bool, bool, Rc<str>, Rc<UH0>) = kill_flags_read_6(v1.clone(), v2, v3, v4, v5);
    let v10: bool = match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: rig ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]]"); } LIT.with(|lit| lit.clone()) };
        println!("{}", v11);
        2i32
    } else {
        let v12: u64 = std::process::id() as u64;
        let v19: US5 = if v7 {
            let v13: u64 = 100u64;
            let v14: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = process_rows_sampled_9(v13);
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("AUTHOR"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = variable_of_10(v15);
            US5::US5_MineOnly(v14, v12, v16, v8)
        } else {
            US5::US5_AnyProcess
        };
        let v20: Rc<str> = ancestor_pids_12();
        let v21: u64 = 0u64;
        let v22: u64 = 0u64;
        let (mut v23, mut v24): (u64, u64) = stops_from_13(v9, v6, v19, v12, v20, v21, v22);
        let v25: Rc<str> = decimal_text_37(v23);
        let v28: Rc<str> = decimal_text_37(v24);
        let v29: Rc<str> = string_concat(&["stopped ", &*v25, "  failed ", &*v28]);
        let v30: bool = v24 == 0u64;
        if v30 {
            let v31: US13 = US13::US13_Succeeded;
            receipt_line_38(v31, v29, v0);
            0i32
        } else {
            let v32: US13 = US13::US13_Failed;
            receipt_line_38(v32, v29, v0);
            1i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: u64 = now_ms_0();
        stop_run_1(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_process_kill_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
