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
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_ReapedGroup,
    US2_NoGroupLeft,
    US2_SkippedPidReused,
    US2_SkippedNoChildPid,
    US2_SkippedWindows,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>),
    US3_None,
}
fn process_alive_0(mut v0: u64) -> bool {
    let v1: bool = v0 == 0u64;
    if v1 {
        true
    } else {
        let v2: u32 = ((v0) as u32);
        let v3: sysinfo::RefreshKind = sysinfo::RefreshKind::nothing().with_processes(Default::default());
        let mut v4: std::rc::Rc<sysinfo::System> = std::rc::Rc::new(sysinfo::System::new_with_specifics(v3));
        let mut v5: sysinfo::Pid = sysinfo::Pid::from_u32(v2);
        let v6: Option<sysinfo::ProcessStatus> = v4.process(v5).map(|p| p.status());
        let v7: Option<sysinfo::ProcessStatus> = None;
        let v8: bool = v6.eq(&v7);
        let mut v11: bool = if v8 {
            true
        } else {
            let mut v9: Option<sysinfo::ProcessStatus> = Some(sysinfo::ProcessStatus::Zombie);
            let mut v10: bool = v6.eq(&v9);
            v10
        };
        let v14: bool = if v11 {
            true
        } else {
            let v12: Option<sysinfo::ProcessStatus> = Some(sysinfo::ProcessStatus::Dead);
            let v13: bool = v6.eq(&v12);
            v13
        };
        let v15: bool = v14 == false;
        v15
    }
}
fn closure0() -> Rc<dyn Fn(u64) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64) -> bool> = Rc::new(move |mut v0: u64| -> bool {
        process_alive_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn owner_running_1(mut v0: u64, mut v1: u64) -> bool {
    let v2: bool = v1 > 0u64;
    if v2 {
        let v3: u32 = ((v1) as u32);
        let mut v4: sysinfo::RefreshKind = sysinfo::RefreshKind::nothing().with_processes(Default::default());
        let mut v5: std::rc::Rc<sysinfo::System> = std::rc::Rc::new(sysinfo::System::new_with_specifics(v4));
        let v6: sysinfo::Pid = sysinfo::Pid::from_u32(v3);
        let v7: u64 = v5.process(v6).map(|p| p.start_time()).unwrap_or(0);
        let v8: u64 = v7.wrapping_mul(1000u64);
        let mut v9: bool = v8 > 0u64;
        let v12: bool = if v9 {
            let mut v10: u64 = v8.wrapping_add(2000u64);
            let mut v11: bool = v10 > v0;
            v11
        } else {
            false
        };
        if v12 {
            let v13: u64 = v0.wrapping_add(2000u64);
            let v14: bool = v13 > v8;
            v14
        } else {
            false
        }
    } else {
        false
    }
}
fn closure1() -> Rc<dyn Fn(u64, u64) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, u64) -> bool> = Rc::new(move |mut v0: u64, mut v1: u64| -> bool {
        owner_running_1(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
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
fn method5(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method5(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method4(v16, v17)
    }
}
fn method6(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure3() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn home_abandoned_2(mut v0: u64, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH0> = split_list_3(v2, v1);
    match Rc::unwrap_or_clone(v3) {
        UH0::UH0_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4;
            let mut v5: Rc<UH0> = v5;
            match Rc::unwrap_or_clone(v5) {
                UH0::UH0_Cons(v6, v7) => {
                    let v6: Rc<str> = v6;
                    let v7: Rc<UH0> = v7;
                    match Rc::unwrap_or_clone(v7) {
                        UH0::UH0_Nil => {
                            let v8: Option<u64> = (v4).trim().parse::<u64>().ok();
                            let mut v9: Option<u64> = method6(v8);
                            let mut v10: Rc<dyn Fn((u64)) -> US0> = closure3();
                            let mut v11: Option<US0> = v9.map(|x| v10(x));
                            let v13: US0 = US0::US0_None;
                            let v14: US0 = v13.clone();
                            let v15: US0 = v11.unwrap_or(v14);
                            let v18: u64 = match v15 {
                                US0::US0_None => {
                                    0u64
                                }
                                US0::US0_Some(v16) => {
                                    let v16: u64 = v16;
                                    v16
                                }
                            };
                            let v19: Option<u64> = (v6).trim().parse::<u64>().ok();
                            let v20: Option<u64> = method6(v19);
                            let mut v21: Option<US0> = v20.map(|x| v10(x));
                            let v23: US0 = US0::US0_None;
                            let v24: US0 = v23.clone();
                            let v25: US0 = v21.unwrap_or(v24);
                            let v28: u64 = match v25 {
                                US0::US0_None => {
                                    0u64
                                }
                                US0::US0_Some(v26) => {
                                    let v26: u64 = v26;
                                    v26
                                }
                            };
                            let v29: bool = owner_running_1(v28, v18);
                            let v30: bool = v29 == false;
                            if v30 {
                                let v31: bool = v0 == 0u64;
                                if v31 {
                                    true
                                } else {
                                    let v32: bool = process_alive_0(v0);
                                    let v33: bool = v32 == false;
                                    v33
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
        _ => {
            false
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, u64) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> bool {
        home_abandoned_2(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn reap_skip_8(mut v0: bool, mut v1: bool, mut v2: u64, mut v3: bool) -> US1 {
    if v3 {
        let mut v4: US2 = US2::US2_SkippedWindows;
        US1::US1_Some(v4)
    } else {
        let v6: bool = v2 <= 1u64;
        if v6 {
            let v7: US2 = US2::US2_SkippedNoChildPid;
            US1::US1_Some(v7)
        } else {
            let mut v10: bool = if v1 {
                let mut v9: bool = v0 == false;
                v9
            } else {
                false
            };
            if v10 {
                let mut v11: US2 = US2::US2_SkippedPidReused;
                US1::US1_Some(v11)
            } else {
                US1::US1_None
            }
        }
    }
}
fn method10(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_9(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method10(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn lost_group_reaped_7(mut v0: u64, mut v1: u64) -> Rc<str> {
    let v2: bool = v1 > 1u64;
    let mut v4: bool = if v2 {
        process_alive_0(v1)
    } else {
        false
    };
    let mut v5: bool = cfg!(windows);
    let v6: bool = owner_running_1(v0, v1);
    let v7: US1 = reap_skip_8(v6, v4, v1, v5);
    let v22: US2 = match v7 {
        US1::US1_None => {
            let mut v9: Rc<str> = format_real_9(v1);
            let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v10.borrow_mut().push(v11);
            v10.borrow_mut().push(v9);
            let v12: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
            let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v14));
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-KILL"); } LIT.with(|lit| lit.clone()) };
            let v17: bool = std::process::Command::new("kill").args([&*v16, "--", &*v15]).output().is_ok_and(|o| o.status.success());
            if v17 {
                US2::US2_ReapedGroup
            } else {
                US2::US2_NoGroupLeft
            }
        }
        US1::US1_Some(v8) => {
            let v8: US2 = v8;
            v8
        }
    };
    ;
    ;
    ;
    ;
    ;
    let v23: bool = match &v22 {
        US2::US2_ReapedGroup => {
            true
        }
        _ => {
            false
        }
    };
    let v27: US3 = if v23 {
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ReapedGroup"); } LIT.with(|lit| lit.clone()) };
        US3::US3_Some(v24)
    } else {
        US3::US3_None
    };
    let v68: US3 = match v27 {
        US3::US3_None => {
            let v30: bool = match &v22 {
                US2::US2_NoGroupLeft => {
                    true
                }
                _ => {
                    false
                }
            };
            let v34: US3 = if v30 {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("NoGroupLeft"); } LIT.with(|lit| lit.clone()) };
                US3::US3_Some(v31)
            } else {
                US3::US3_None
            };
            match v34 {
                US3::US3_None => {
                    let v37: bool = match &v22 {
                        US2::US2_SkippedPidReused => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v41: US3 = if v37 {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SkippedPidReused"); } LIT.with(|lit| lit.clone()) };
                        US3::US3_Some(v38)
                    } else {
                        US3::US3_None
                    };
                    match v41 {
                        US3::US3_None => {
                            let v44: bool = match &v22 {
                                US2::US2_SkippedNoChildPid => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            };
                            let v48: US3 = if v44 {
                                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SkippedNoChildPid"); } LIT.with(|lit| lit.clone()) };
                                US3::US3_Some(v45)
                            } else {
                                US3::US3_None
                            };
                            match v48 {
                                US3::US3_None => {
                                    let v51: bool = match v22 {
                                        US2::US2_SkippedWindows => {
                                            true
                                        }
                                        _ => {
                                            false
                                        }
                                    };
                                    let v55: US3 = if v51 {
                                        let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SkippedWindows"); } LIT.with(|lit| lit.clone()) };
                                        US3::US3_Some(v52)
                                    } else {
                                        US3::US3_None
                                    };
                                    match v55 {
                                        US3::US3_None => {
                                            US3::US3_None
                                        }
                                        US3::US3_Some(v56) => {
                                            let v56: Rc<str> = v56;
                                            US3::US3_Some(v56)
                                        }
                                    }
                                }
                                US3::US3_Some(v49) => {
                                    let v49: Rc<str> = v49;
                                    US3::US3_Some(v49)
                                }
                            }
                        }
                        US3::US3_Some(v42) => {
                            let v42: Rc<str> = v42;
                            US3::US3_Some(v42)
                        }
                    }
                }
                US3::US3_Some(v35) => {
                    let v35: Rc<str> = v35;
                    US3::US3_Some(v35)
                }
            }
        }
        US3::US3_Some(v28) => {
            let v28: Rc<str> = v28;
            US3::US3_Some(v28)
        }
    };
    match v68 {
        US3::US3_None => {
            std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Option does not have a value."); } LIT.with(|lit| lit.clone()) }))
        }
        US3::US3_Some(v69) => {
            let v69: Rc<str> = v69;
            v69
        }
    }
}
fn closure4() -> Rc<dyn Fn(u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, u64) -> Rc<str>> = Rc::new(move |mut v0: u64, mut v1: u64| -> Rc<str> {
        lost_group_reaped_7(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_process_alive(v0: u64) -> bool {
    closure0()(v0)
}
pub fn rig_process_owner_running(v0: u64, v1: u64) -> bool {
    closure1()(v0, v1)
}
pub fn rig_process_home_abandoned(v0: &str, v1: u64) -> bool {
    closure2()(Rc::<str>::from(v0), v1)
}
pub fn rig_process_lost_group_reaped(v0: u64, v1: u64) -> Rc<str> {
    closure4()(v0, v1)
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
