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
enum US1 {
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_AgentSession(Rc<str>, u64),
    US2_KitJob(Rc<str>),
    US2_CellsVm(u64),
    US2_JobLeftover(Rc<str>),
    US2_Orphan,
    US2_NotATool,
}
#[derive(Clone)]
enum US3 {
    US3_Some(i32),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Healthy,
    US4_Unowned,
    US4_Dangling,
    US4_Leftover,
    US4_Hog,
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_2(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_3();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_4(v0);
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
            let v12: Rc<UH0> = arguments_from_2(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn value_in_5(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: bool = &*v3 == &*v1;
                if v5 {
                    match Rc::unwrap_or_clone(v4) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            return v6;
                        }
                        UH0::UH0_Nil => {
                            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            return v8;
                        }
                    }
                } else {
                    v0 = v4;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v2;
            }
        }
    }
}
fn method6(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let mut v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn free_bytes_on_7(mut v0: std::rc::Rc<sysinfo::Disks>, mut v1: Rc<str>, mut v2: i32) -> u64 {
    loop {
        let v3: usize = ((v2) as usize);
        let v4: usize = v0.list().len();
        let v5: i32 = ((v4) as i32);
        let mut v6: bool = v2 >= v5;
        if v6 {
            return 0u64;
        } else {
            let mut v7: bool = v0.list()[v3.clone()].mount_point().eq(std::path::Path::new(&*v1));
            if v7 {
                let mut v8: u64 = v0.list()[v3].available_space();
                return v8;
            } else {
                let mut v9: i32 = v2.wrapping_add(1i32);
                v2 = v9;
                continue;
            }
        }
    }
}
fn process_rows_sampled_8(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> {
    let mut v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = rig_machine_sample::rig_machine_sampled(v0);
    v1
}
fn cores_sum_9(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: i32, mut v2: f64) -> f64 {
    loop {
        let v3: i32 = v0.len() as i32;
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v1.wrapping_add(1i32);
            let mut v6: f64 = v0[v1 as usize].4;
            let mut v7: f64 = v2 + v6;
            (v1, v2) = (v5, v7);
            continue;
        }
    }
}
fn gate_run_1() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_2(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-disk-gb"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = value_in_5(v1.clone(), v2);
    let v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v5: Option<u64> = (v4).trim().parse::<u64>().ok();
    let mut v6: Option<u64> = method6(v5);
    let mut v7: Rc<dyn Fn((u64)) -> US1> = closure1();
    let mut v8: Option<US1> = v6.map(|x| v7(x));
    let v10: US1 = US1::US1_None;
    let v11: US1 = v10.clone();
    let v12: US1 = v8.unwrap_or(v11);
    let v15: u64 = match v12 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v13) => {
            let v13: u64 = v13;
            v13
        }
    };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = value_in_5(v1.clone(), v16);
    let v18: Rc<str> = Rc::<str>::from((v17).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v19: Option<u64> = (v18).trim().parse::<u64>().ok();
    let v20: Option<u64> = method6(v19);
    let mut v21: Option<US1> = v20.map(|x| v7(x));
    let v23: US1 = US1::US1_None;
    let v24: US1 = v23.clone();
    let v25: US1 = v21.unwrap_or(v24);
    let v28: u64 = match v25 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v26) => {
            let v26: u64 = v26;
            v26
        }
    };
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max-busy-pct"); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = value_in_5(v1.clone(), v29);
    let v31: Rc<str> = Rc::<str>::from((v30).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v32: Option<u64> = (v31).trim().parse::<u64>().ok();
    let v33: Option<u64> = method6(v32);
    let mut v34: Option<US1> = v33.map(|x| v7(x));
    let v36: US1 = US1::US1_None;
    let v37: US1 = v36.clone();
    let mut v38: US1 = v34.unwrap_or(v37);
    let v41: u64 = match v38 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v39) => {
            let v39: u64 = v39;
            v39
        }
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--mount"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = value_in_5(v1, v42);
    let v44: bool = &*v43 != "";
    let v49: Rc<str> = if v44 {
        v43
    } else {
        let v45: bool = cfg!(windows);
        if v45 {
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("C:\\"); } LIT.with(|lit| lit.clone()) };
            v46
        } else {
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            v47
        }
    };
    let v50: std::rc::Rc<sysinfo::Disks> = std::rc::Rc::new(sysinfo::Disks::new_with_refreshed_list());
    let v51: i32 = 0i32;
    let v52: u64 = free_bytes_on_7(v50, v49, v51);
    let v53: u64 = v52.wrapping_div(1073741824u64);
    let v54: sysinfo::MemoryRefreshKind = sysinfo::MemoryRefreshKind::nothing().with_ram();
    let v55: sysinfo::RefreshKind = sysinfo::RefreshKind::nothing().with_memory(v54);
    let mut v56: std::rc::Rc<sysinfo::System> = std::rc::Rc::new(sysinfo::System::new_with_specifics(v55));
    let v57: u64 = v56.available_memory();
    let v58: u64 = v57.wrapping_div(1048576u64);
    let v59: u64 = 2000u64;
    let v60: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = process_rows_sampled_8(v59);
    let v61: usize = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let v62: i32 = ((v61) as i32);
    let v63: f64 = (v62 as f64);
    let v64: i32 = 0i32;
    let v65: f64 = 0.0f64;
    let v66: f64 = cores_sum_9(v60, v64, v65);
    let v67: f64 = v66 / v63;
    let v68: f64 = v67 * 100.0f64;
    let v69: u64 = (v68 as u64);
    let v70: bool = v53 >= v15;
    let v72: bool = if v70 {
        let v71: bool = v58 >= v28;
        v71
    } else {
        false
    };
    let v74: bool = if v72 {
        let v73: bool = v69 < v41;
        v73
    } else {
        false
    };
    let v75: bool = v53 < v15;
    let v96: Rc<str> = if v75 {
        let v76: Rc<str> = std::rc::Rc::<str>::from((v53).to_string());
        let v79: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
        let v80: Rc<str> = string_concat(&["disk-", &*v76, "gb<", &*v79]);
        v80
    } else {
        let v81: bool = v58 < v28;
        if v81 {
            let v82: Rc<str> = std::rc::Rc::<str>::from((v58).to_string());
            let v85: Rc<str> = std::rc::Rc::<str>::from((v28).to_string());
            let v86: Rc<str> = string_concat(&["ram-", &*v82, "mb<", &*v85]);
            v86
        } else {
            let v87: bool = v69 >= v41;
            if v87 {
                let v88: Rc<str> = std::rc::Rc::<str>::from((v69).to_string());
                let v91: Rc<str> = std::rc::Rc::<str>::from((v41).to_string());
                let v92: Rc<str> = string_concat(&["busy-", &*v88, "pct>=", &*v91]);
                v92
            } else {
                let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                v93
            }
        }
    };
    let v99: Rc<str> = if v74 {
        let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        v97
    } else {
        let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("refused"); } LIT.with(|lit| lit.clone()) };
        v98
    };
    let v104: Rc<str> = std::rc::Rc::<str>::from((v53).to_string());
    let v107: Rc<str> = std::rc::Rc::<str>::from((v58).to_string());
    let v110: Rc<str> = std::rc::Rc::<str>::from((v69).to_string());
    let v112: Rc<str> = string_concat(&["rig machine health gate=", &*v99, " reason=", &*v96, " disk=", &*v104, "gb ram=", &*v107, "mb busy=", &*v110, "pct"]);
    println!("{}", v112);
    if v74 {
        0i32
    } else {
        1i32
    }
}
fn closure2(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: bool = &*v1 == &*v0;
        v2
    })
}
fn method11(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure2(v0)
}
fn method12(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: Rc<UH0>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: bool = v1(v3);
                if v5 {
                    return true;
                } else {
                    let mut v6: Rc<dyn Fn(Rc<str>) -> bool> = method11(v0.clone());
                    (v1, v2) = (v6, v4);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn job_left_by_19(mut v0: u64, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = rig_machine_sample::rig_machine_job_left(v0, v1);
    v2
}
fn left_or_orphan_18(mut v0: u64, mut v1: u64) -> US2 {
    let mut v2: Rc<str> = job_left_by_19(v0, v1);
    let v3: bool = &*v2 == "";
    if v3 {
        US2::US2_Orphan
    } else {
        US2::US2_JobLeftover(v2)
    }
}
fn closure3(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US3> {
    Rc::new(move |mut v2: Rc<UH0>| -> US3 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US3::US3_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US3> = method21(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US3::US3_None
            }
        }
    })
}
fn method21(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US3> {
    closure3(v0, v1)
}
fn agent_client_20(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = 0i32;
    let mut v2: Rc<dyn Fn(Rc<UH0>) -> US3> = method21(v0, v1);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("codex"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5));
    let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v6));
    let mut v8: US3 = v2(v7);
    let mut v9: bool = match v8 {
        US3::US3_None => {
            true
        }
        _ => {
            false
        }
    };
    let v10: bool = v9 != true;
    v10
}
fn cells_vm_at_22(mut v0: Rc<str>) -> bool {
    let mut v1: bool = rig_machine_sample::rig_machine_cells_vm(&*v0);
    v1
}
fn owner_walk_17(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: i32, mut v2: i32) -> US2 {
    loop {
        let v3: bool = v1 < 0i32;
        let v5: bool = if v3 {
            true
        } else {
            let v4: bool = v2 >= 40i32;
            v4
        };
        if v5 {
            return US2::US2_Orphan;
        } else {
            let mut v7: Rc<str> = v0[v1 as usize].2.clone();
            let mut v8: Rc<str> = v0[v1 as usize].3.clone();
            let mut v9: Rc<str> = std::rc::Rc::<str>::from((v8).split_whitespace().skip_while(|word| *word != "supervise").nth(1).filter(|word| word.starts_with("job-")).unwrap_or(""));
            let v10: bool = agent_client_20(v7.clone());
            if v10 {
                let v11: u64 = v0[v1 as usize].0;
                return US2::US2_AgentSession(v7, v11);
            } else {
                let v13: bool = &*v9 != "";
                if v13 {
                    return US2::US2_KitJob(v9);
                } else {
                    let v15: u64 = v0[v1 as usize].1;
                    let v16: i32 = v0.iter().position(|row| row.0 == v15).map(|at| at as i32).unwrap_or(-1);
                    let v17: bool = v16 < 0i32;
                    if v17 {
                        let v18: bool = &*v7 == "erl";
                        let mut v21: bool = if v18 {
                            let v19: Rc<str> = v0[v1 as usize].3.clone();
                            cells_vm_at_22(v19)
                        } else {
                            false
                        };
                        if v21 {
                            let v22: u64 = v0[v1 as usize].0;
                            return US2::US2_CellsVm(v22);
                        } else {
                            let v24: u64 = v0[v1 as usize].1;
                            let v25: u64 = v0[v1 as usize].5;
                            return left_or_orphan_18(v24, v25);
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
fn owner_of_16(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: i32) -> US2 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dotnet|rustc|cargo|erl|beam.smp|pwsh|powershell|node|bun|gcc|cc1|cc1plus|ld|zig|lean|java|swiftc|swift-frontend|python|kotlinc|gleam|wasmtime|link|clang|mix|escript|make|git"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = v0[v1 as usize].2.clone();
    let v4: bool = v2.split("|").any(|item| item == &*v3);
    if v4 {
        let v5: i32 = 0i32;
        owner_walk_17(v0, v1, v5)
    } else {
        US2::US2_NotATool
    }
}
fn owner_text_23(mut v0: US2) -> Rc<str> {
    match v0 {
        US2::US2_AgentSession(v1, v2) => {
            let mut v1: Rc<str> = v1;
            let mut v2: u64 = v2;
            let v4: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let v5: Rc<str> = string_concat(&[&*v1, ":", &*v4]);
            v5
        }
        US2::US2_CellsVm(v8) => {
            let mut v8: u64 = v8;
            let mut v9: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
            let v10: Rc<str> = string_concat(&["cells-vm:", &*v9]);
            v10
        }
        US2::US2_JobLeftover(v11) => {
            let v11: Rc<str> = v11;
            let v12: Rc<str> = string_concat(&["left-by:", &*v11]);
            v12
        }
        US2::US2_KitJob(v6) => {
            let mut v6: Rc<str> = v6;
            let mut v7: Rc<str> = string_concat(&["kit:", &*v6]);
            v7
        }
        US2::US2_NotATool => {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v14
        }
        US2::US2_Orphan => {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan"); } LIT.with(|lit| lit.clone()) };
            v13
        }
    }
}
fn tallied_15(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v5: bool, mut v6: i32, mut v7: Rc<UH0>, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64) -> (Rc<UH0>, u64, u64, u64, u64, u64, u64) {
    loop {
        let v14: i32 = v4.len() as i32;
        let v15: bool = v6 >= v14;
        if v15 {
            return (v7, v8, v9, v10, v11, v12, v13);
        } else {
            let v16: US2 = owner_of_16(v4.clone(), v6);
            match &v16 {
                US2::US2_NotATool => {
                    let v17: i32 = v6.wrapping_add(1i32);
                    v6 = v17;
                    continue;
                }
                _ => {
                    let v25: u64 = v4[v6 as usize].7;
                    let v26: f64 = v4[v6 as usize].4;
                    let v27: u64 = v4[v6 as usize].5;
                    let v41: US4 = match &v16 {
                        US2::US2_JobLeftover(v28) => {
                            let v28: Rc<str> = v28.clone();
                            US4::US4_Leftover
                        }
                        US2::US2_Orphan => {
                            let v30: bool = v26 >= v2;
                            let v32: bool = if v30 {
                                let v31: bool = v27 >= v1;
                                v31
                            } else {
                                false
                            };
                            if v32 {
                                US4::US4_Dangling
                            } else {
                                US4::US4_Unowned
                            }
                        }
                        _ => {
                            let v36: bool = v25 >= v3;
                            if v36 {
                                US4::US4_Hog
                            } else {
                                US4::US4_Healthy
                            }
                        }
                    };
                    let v43: bool = match &v41 {
                        US4::US4_Healthy => {
                            v5
                        }
                        US4::US4_Unowned => {
                            v5
                        }
                        _ => {
                            true
                        }
                    };
                    let (mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = match &v41 {
                        US4::US4_Healthy => {
                            let v44: u64 = v8.wrapping_add(1u64);
                            let v45: u64 = v9.wrapping_add(1u64);
                            (v7, v44, v45, v10, v11, v12, v13)
                        }
                        _ => {
                            let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = match &v41 {
                                US4::US4_Unowned => {
                                    let v46: u64 = v8.wrapping_add(1u64);
                                    let v47: u64 = v10.wrapping_add(1u64);
                                    (v7, v46, v9, v47, v11, v12, v13)
                                }
                                _ => {
                                    let (mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = match &v41 {
                                        US4::US4_Dangling => {
                                            let v48: u64 = v8.wrapping_add(1u64);
                                            let v49: u64 = v11.wrapping_add(1u64);
                                            (v7, v48, v9, v10, v49, v12, v13)
                                        }
                                        _ => {
                                            let (mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = match &v41 {
                                                US4::US4_Leftover => {
                                                    let v50: u64 = v8.wrapping_add(1u64);
                                                    let v51: u64 = v12.wrapping_add(1u64);
                                                    (v7, v50, v9, v10, v11, v51, v13)
                                                }
                                                _ => {
                                                    let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = match &v41 {
                                                        US4::US4_Hog => {
                                                            let v52: u64 = v8.wrapping_add(1u64);
                                                            let v53: u64 = v13.wrapping_add(1u64);
                                                            (v7, v52, v9, v10, v11, v12, v53)
                                                        }
                                                        _ => unreachable!(),
                                                    };
                                                    (v54, v55, v56, v57, v58, v59, v60)
                                                }
                                            };
                                            (v61, v62, v63, v64, v65, v66, v67)
                                        }
                                    };
                                    (v68, v69, v70, v71, v72, v73, v74)
                                }
                            };
                            (v75, v76, v77, v78, v79, v80, v81)
                        }
                    };
                    let (mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = if v43 {
                        let v89: u64 = v4[v6 as usize].0;
                        let v90: Rc<str> = std::rc::Rc::<str>::from((v89).to_string());
                        let v93: Rc<str> = v4[v6 as usize].2.clone();
                        let v96: f64 = v4[v6 as usize].4;
                        let v97: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v96));
                        let v100: u64 = v4[v6 as usize].7;
                        let v101: Rc<str> = std::rc::Rc::<str>::from((v100).to_string());
                        let v104: u64 = v4[v6 as usize].5;
                        let v105: Rc<str> = std::rc::Rc::<str>::from((v104).to_string());
                        let v108: Rc<str> = owner_text_23(v16);
                        let v110: Rc<str> = string_concat(&["  ", &*v90, "  ", &*v93, "  ", &*v97, " cores  ", &*v101, " MB  ", &*v105, " min  ", &*v108, "  "]);
                        let v120: Rc<str> = match v41 {
                            US4::US4_Dangling => {
                                let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dangling"); } LIT.with(|lit| lit.clone()) };
                                v113
                            }
                            US4::US4_Healthy => {
                                let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                                v111
                            }
                            US4::US4_Hog => {
                                let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hog"); } LIT.with(|lit| lit.clone()) };
                                v115
                            }
                            US4::US4_Leftover => {
                                let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("leftover"); } LIT.with(|lit| lit.clone()) };
                                v114
                            }
                            US4::US4_Unowned => {
                                let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unowned"); } LIT.with(|lit| lit.clone()) };
                                v112
                            }
                        };
                        let v121: Rc<str> = string_concat(&[&*v110, &*v120]);
                        let v122: Rc<UH0> = Rc::new(UH0::UH0_Cons(v121, v82));
                        (v122, v83, v84, v85, v86, v87, v88)
                    } else {
                        (v82, v83, v84, v85, v86, v87, v88)
                    };
                    let v130: i32 = v6.wrapping_add(1i32);
                    (v6, v7, v8, v9, v10, v11, v12, v13) = (v130, v123, v124, v125, v126, v127, v128, v129);
                    continue;
                }
            }
        }
    }
}
fn method24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method25(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<UH0> = v2;
                println!("{}", v1);
                v0 = v2;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn stalled_jobs_listed_26() -> Rc<str> {
    let mut v0: Rc<str> = rig_machine_sample::rig_machine_stalled_jobs();
    v0
}
fn history_written_27(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = rig_machine_sample::rig_machine_history_appended(&*v0);
    v1
}
fn health_pass_14(mut v0: u64, mut v1: bool) -> bool {
    let mut v2: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = process_rows_sampled_8(v0);
    let v3: u64 = 60u64;
    let v4: u64 = 30u64;
    let v5: f64 = 0.8f64;
    let mut v6: u64 = 8192u64;
    let mut v7: i32 = 0i32;
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: u64 = 0u64;
    let v10: u64 = 0u64;
    let v11: u64 = 0u64;
    let v12: u64 = 0u64;
    let v13: u64 = 0u64;
    let v14: u64 = 0u64;
    let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = tallied_15(v3, v4, v5, v6, v2, v1, v7, v8, v9, v10, v11, v12, v13, v14);
    let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v23: Rc<UH0> = method24(v15, v22);
    method25(v23);
    let v24: Rc<str> = stalled_jobs_listed_26();
    let v25: bool = &*v24 == "";
    if v25 {
        ()
    } else {
        println!("{}", v24);
        ()
    };
    let v30: u64 = if v25 {
        0u64
    } else {
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v24.split(&*v26).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v28: i32 = (v27.borrow().len() as i32);
        let v29: u64 = (v28 as u64);
        v29
    };
    let v31: Rc<str> = std::rc::Rc::<str>::from((v16).to_string());
    let v32: Rc<str> = std::rc::Rc::<str>::from((v17).to_string());
    let v33: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
    let mut v34: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
    let v35: Rc<str> = std::rc::Rc::<str>::from((v20).to_string());
    let v36: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
    let v37: Rc<str> = std::rc::Rc::<str>::from((v30).to_string());
    let mut v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v38.borrow_mut().push(v31);
    v38.borrow_mut().push(v32);
    v38.borrow_mut().push(v33);
    v38.borrow_mut().push(v34);
    v38.borrow_mut().push(v35);
    v38.borrow_mut().push(v36);
    v38.borrow_mut().push(v37);
    let v39: Rc<Vec<Rc<str>>> = Rc::new(v38.borrow().clone());
    let v40: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v39).as_ref().clone()));
    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v42: Rc<str> = Rc::<str>::from((*v40.borrow()).join(&*v41));
    let v43: Rc<str> = history_written_27(v42);
    let v44: u64 = v19.wrapping_add(v20);
    let v45: bool = v44 > 0u64;
    let v48: Rc<str> = if v45 {
        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
        v46
    } else {
        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        v47
    };
    let v51: Rc<str> = std::rc::Rc::<str>::from((v16).to_string());
    let v54: Rc<str> = std::rc::Rc::<str>::from((v17).to_string());
    let v57: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
    let v60: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
    let v63: Rc<str> = std::rc::Rc::<str>::from((v20).to_string());
    let v66: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
    let v69: Rc<str> = std::rc::Rc::<str>::from((v30).to_string());
    let v72: Rc<str> = string_concat(&["rig machine health ", &*v48, " tools=", &*v51, " ok=", &*v54, " unowned=", &*v57, " dangling=", &*v60, " leftover=", &*v63, " hogs=", &*v66, " stalled=", &*v69, " history=", &*v43]);
    println!("{}", v72);
    v45
}
fn health_passes_13(mut v0: u64, mut v1: bool, mut v2: u64, mut v3: bool) -> bool {
    loop {
        let v4: bool = v2 == 0u64;
        if v4 {
            return v3;
        } else {
            let v5: u64 = v2.wrapping_sub(1u64);
            let mut v6: bool = health_pass_14(v0, v1);
            (v2, v3) = (v5, v6);
            continue;
        }
    }
}
fn health_normal_run_10() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_2(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--every"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = value_in_5(v1.clone(), v2);
    let v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v5: Option<u64> = (v4).trim().parse::<u64>().ok();
    let mut v6: Option<u64> = method6(v5);
    let mut v7: Rc<dyn Fn((u64)) -> US1> = closure1();
    let mut v8: Option<US1> = v6.map(|x| v7(x));
    let v10: US1 = US1::US1_None;
    let v11: US1 = v10.clone();
    let v12: US1 = v8.unwrap_or(v11);
    let v15: u64 = match v12 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v13) => {
            let v13: u64 = v13;
            v13
        }
    };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--times"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = value_in_5(v1, v16);
    let v18: Rc<str> = Rc::<str>::from((v17).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v19: Option<u64> = (v18).trim().parse::<u64>().ok();
    let v20: Option<u64> = method6(v19);
    let mut v21: Option<US1> = v20.map(|x| v7(x));
    let v23: US1 = US1::US1_None;
    let v24: US1 = v23.clone();
    let v25: US1 = v21.unwrap_or(v24);
    let v28: u64 = match v25 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v26) => {
            let v26: u64 = v26;
            v26
        }
    };
    let v29: bool = v15 == 0u64;
    let v31: u64 = if v29 {
        3000u64
    } else {
        let v30: u64 = v15.wrapping_mul(1000u64);
        v30
    };
    let mut v34: u64 = if v29 {
        1u64
    } else {
        let v32: bool = v28 == 0u64;
        if v32 {
            1u64
        } else {
            v28
        }
    };
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--all"); } LIT.with(|lit| lit.clone()) };
    let v36: i32 = 0i32;
    let v37: Rc<UH0> = arguments_from_2(v36);
    let mut v38: Rc<dyn Fn(Rc<str>) -> bool> = method11(v35.clone());
    let v39: bool = method12(v35, v38, v37);
    let v40: bool = false;
    let v41: bool = health_passes_13(v31, v39, v34, v40);
    if v41 {
        1i32
    } else {
        0i32
    }
}
fn health_run_0() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--gate"); } LIT.with(|lit| lit.clone()) };
    let mut v1: i32 = 0i32;
    let mut v2: Rc<UH0> = arguments_from_2(v1);
    let v3: Rc<dyn Fn(Rc<str>) -> bool> = method11(v0.clone());
    let v4: bool = method12(v0, v3, v2);
    if v4 {
        gate_run_1()
    } else {
        health_normal_run_10()
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        health_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_machine_health_run() -> i32 {
    closure0()()
}
