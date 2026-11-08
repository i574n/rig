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
    US0_ItemOpen,
    US0_ItemDone,
    US0_ItemUnknown,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, i32, US0, Rc<UH0>),
}
#[derive(Clone)]
enum US3 {
    US3_SprintOpen,
    US3_SprintClosed,
}
#[derive(Clone)]
enum US4 {
    US4_Committed,
    US4_Dropped,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, US4, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_Started(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US3, Rc<UH1>),
    US2_AnotherSprintOpen(Rc<str>),
    US2_ItemNotOpen(Rc<str>),
    US2_ItemUnestimated(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<UH2>),
}
#[derive(Clone)]
enum US5 {
    US5_Some(Rc<str>),
    US5_None,
}
fn sprint_has_2(mut v0: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v2 >= v3;
        if v4 {
            return false;
        } else {
            let (mut v5, mut v6, mut v7): (Rc<str>, i32, bool) = v0.borrow()[v2 as usize].clone();
            let v8: bool = &*v5 == &*v1;
            if v8 {
                return true;
            } else {
                let mut v9: i32 = v2.wrapping_add(1i32);
                v2 = v9;
                continue;
            }
        }
    }
}
fn standing_of_4(mut v0: Rc<str>) -> US0 {
    let mut v1: bool = &*v0 == "";
    if v1 {
        US0::US0_ItemUnknown
    } else {
        let v3: bool = &*v0 == "Done";
        if v3 {
            US0::US0_ItemDone
        } else {
            US0::US0_ItemOpen
        }
    }
}
fn candidate_of_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, i32, US0) {
    let mut v2: Rc<str> = agile_task_census::rig_agile_task_status_of(&v0, &v1);
    let v3: i32 = agile_planning_fields::rig_agile_planning_minutes_of(&v0, &v1);
    let mut v4: US0 = standing_of_4(v2);
    (v1, v3, v4)
}
fn method6(mut v0: Rc<UH0>, mut v1: US1) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let mut v4: US0 = v4;
                let v5: Rc<UH0> = v5;
                let v18: US1 = match &v1 {
                    US1::US1_None => {
                        let v7: bool = match v4 {
                            US0::US0_ItemOpen => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v8: bool = v7 != true;
                        if v8 {
                            let mut v9: US2 = US2::US2_ItemNotOpen(v2);
                            US1::US1_Some(v9)
                        } else {
                            let mut v11: bool = v3 <= 0i32;
                            if v11 {
                                let mut v12: US2 = US2::US2_ItemUnestimated(v2);
                                US1::US1_Some(v12)
                            } else {
                                US1::US1_None
                            }
                        }
                    }
                    US1::US1_Some(v6) => {
                        let v6: US2 = v6.clone();
                        v1
                    }
                };
                (v0, v1) = (v5, v18);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn first_refusal_5(mut v0: Rc<UH0>) -> US1 {
    let mut v1: US1 = US1::US1_None;
    method6(v0, v1)
}
fn sprint_changed_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool) -> i32 {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
    let v7: Rc<str> = string_concat(&[&*v6, "/state"]);
    let v8: Rc<str> = string_concat(&[&*v7, "/sprints.spi"]);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v8).unwrap_or_default().as_str());
    let v10: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>> = rig_agile_sprint_state::rig_agile_sprint_head(&v9, &v1);
    let v13: Rc<str> = if v3 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("drop"); } LIT.with(|lit| lit.clone()) };
        v11
    } else {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
        v12
    };
    let v14: i32 = (v10.clone().borrow().len() as i32);
    let v15: bool = v14 == 0i32;
    if v15 {
        let v19: Rc<str> = string_concat(&["kit error: agile sprint ", &*v13, " refused: no sprint ", &*v1]);
        eprintln!("{}", v19);
        1i32
    } else {
        let (mut v20, mut v21, mut v22, mut v23, mut v24): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) = v10.borrow()[0i32 as usize].clone();
        let v25: bool = v24 == false;
        if v25 {
            let v30: Rc<str> = string_concat(&["kit error: agile sprint ", &*v13, " refused: sprint ", &*v1, " is closed"]);
            eprintln!("{}", v30);
            1i32
        } else {
            let v31: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> = rig_agile_sprint_state::rig_agile_sprint_items(&v9, &v1);
            let v32: i32 = 0i32;
            let v33: bool = sprint_has_2(&v31, v2.clone(), v32);
            let v34: Rc<str> = string_concat(&[&*v7, "/agile.spi"]);
            let mut v35: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v34).unwrap_or_default().as_str());
            let (mut v36, mut v37, mut v38): (Rc<str>, i32, US0) = candidate_of_3(v35, v2.clone());
            let v59: Rc<str> = if v3 {
                if v33 {
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v39
                } else {
                    let v42: Rc<str> = string_concat(&["item ", &*v2, " is not in sprint ", &*v1]);
                    v42
                }
            } else {
                let v44: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v45: Rc<UH0> = Rc::new(UH0::UH0_Cons(v36, v37, v38, v44));
                let v46: US1 = first_refusal_5(v45);
                match v46 {
                    US1::US1_Some(v47) => {
                        let v47: US2 = v47;
                        match v47 {
                            US2::US2_ItemNotOpen(v48) => {
                                let v48: Rc<str> = v48;
                                let v50: Rc<str> = string_concat(&["item ", &*v2, " is unknown or Done"]);
                                v50
                            }
                            US2::US2_ItemUnestimated(v51) => {
                                let v51: Rc<str> = v51;
                                let v53: Rc<str> = string_concat(&["item ", &*v2, " has no estimate"]);
                                v53
                            }
                            _ => {
                                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v54
                            }
                        }
                    }
                    _ => {
                        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v57
                    }
                }
            };
            let v60: bool = &*v59 != "";
            if v60 {
                let v63: Rc<str> = string_concat(&["kit error: agile sprint ", &*v13, " refused: ", &*v59]);
                eprintln!("{}", v63);
                1i32
            } else {
                let v64: Rc<str> = rig_agile_sprint_append::rig_agile_sprint_item_appended(&v9, &v1, &v2, v37, v3);
                std::fs::write(&*v8, &*v64).unwrap();
                let v70: Rc<str> = string_concat(&["kit agile sprint ", &*v13, " ok id=", &*v1, " item=", &*v2]);
                println!("{}", v70);
                0i32
            }
        }
    }
}
fn changed_0(mut v0: bool, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    sprint_changed_1(v3, v2, v1, v0)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool| -> i32 {
        changed_0(v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn finished_from_9(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v2: i32, mut v3: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        let mut v4: i32 = (v1.clone().borrow().len() as i32);
        let v5: bool = v2 >= v4;
        if v5 {
            return v3;
        } else {
            let (mut v6, mut v7, mut v8): (Rc<str>, i32, bool) = v1.borrow()[v2 as usize].clone();
            let mut v9: Rc<str> = agile_task_census::rig_agile_task_status_of(&v0, &v6);
            let v10: bool = &*v9 == "Done";
            let mut v11: i32 = v2.wrapping_add(1i32);
            let v26: Rc<RefCell<Vec<Rc<str>>>> = if v10 {
                let mut v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v6]));
                let v13: i32 = (v3.clone().borrow().len() as i32);
                let v14: i32 = (v12.clone().borrow().len() as i32);
                let v15: i32 = v13.wrapping_add(v14);
                let mut v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v15 as usize]));
                let mut v17: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method10(v15, &v17) {
                    let v19: i32 = v17.borrow().l0;
                    let v20: bool = v19 < v13;
                    let v24: Rc<str> = if v20 {
                        let v21: Rc<str> = v3.borrow()[v19 as usize].clone();
                        v21
                    } else {
                        let v22: i32 = v19.wrapping_sub(v13);
                        let v23: Rc<str> = v12.borrow()[v22 as usize].clone();
                        v23
                    };
                    v16.borrow_mut()[v19 as usize] = v24;
                    let v25: i32 = v19.wrapping_add(1i32);
                    v17.borrow_mut().l0 = v25;
                    ()
                };
                v16
            } else {
                v3
            };
            (v2, v3) = (v11, v26);
            continue;
        }
    }
}
fn method12(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_11(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method12(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method14(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let mut v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn carried_from_13(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v2: i32, mut v3: Rc<UH2>) -> Rc<UH2> {
    loop {
        let mut v4: i32 = (v1.clone().borrow().len() as i32);
        let v5: bool = v2 >= v4;
        if v5 {
            let v6: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            return method14(v3, v6);
        } else {
            let (mut v8, mut v9, mut v10): (Rc<str>, i32, bool) = v1.borrow()[v2 as usize].clone();
            let mut v11: Rc<str> = agile_task_census::rig_agile_task_status_of(&v0, &v8);
            let mut v12: bool = &*v11 == "Done";
            let v13: i32 = v2.wrapping_add(1i32);
            let v15: bool = if v10 {
                let v14: bool = v12 == false;
                v14
            } else {
                false
            };
            let mut v17: Rc<UH2> = if v15 {
                Rc::new(UH2::UH2_Cons(v8, v3))
            } else {
                v3
            };
            (v2, v3) = (v13, v17);
            continue;
        }
    }
}
fn method17(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH2>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                drop(v0.borrow_mut().arg(&*v2));
                v1 = v3;
                continue;
            }
            UH2::UH2_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH2>, mut v4: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH2::UH2_Cons(v5, v6) => {
                let v5: Rc<str> = v5;
                let v6: Rc<UH2> = v6;
                let v8: Rc<str> = string_concat(&["sprint ", &*v1, " closed with this item not Done; it carries over to the next sprint"]);
                let mut v9: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v2)));
                let v10: bool = &*v0 == "";
                let v22: Rc<UH2> = if v10 {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
                    let v13: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let v14: Rc<UH2> = Rc::new(UH2::UH2_Cons(v12, v13));
                    Rc::new(UH2::UH2_Cons(v11, v14))
                } else {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
                    let v18: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let v19: Rc<UH2> = Rc::new(UH2::UH2_Cons(v17, v18));
                    let v20: Rc<UH2> = Rc::new(UH2::UH2_Cons(v16, v19));
                    Rc::new(UH2::UH2_Cons(v0.clone(), v20))
                };
                let v23: bool = &*v5 == "";
                let v25: Rc<UH2> = if v23 {
                    v22
                } else {
                    Rc::new(UH2::UH2_Cons(v5, v22))
                };
                let v26: bool = &*v8 == "";
                let v28: Rc<UH2> = if v26 {
                    v25
                } else {
                    Rc::new(UH2::UH2_Cons(v8, v25))
                };
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-sprint"); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                let v32: Rc<UH2> = Rc::new(UH2::UH2_Cons(v30, v31));
                let v33: Rc<UH2> = Rc::new(UH2::UH2_Cons(v29, v32));
                let v34: Rc<UH2> = method14(v28, v33);
                method17(v9.clone(), v34);
                let mut v35: Rc<std::io::Result<std::process::Output>> = Rc::new(v9.borrow_mut().output());
                let v36: bool = v35.is_ok();
                let (mut v42, mut v43, mut v44): (bool, i32, Rc<str>) = if v36 {
                    let v37: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v35).as_ref().unwrap().stderr).as_ref());
                    let v38: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v35).as_ref().unwrap().stdout).as_ref());
                    let v39: Rc<str> = string_concat(&[&*v38, &*v37]);
                    let v40: i32 = (&*v35).as_ref().unwrap().status.code().unwrap_or(-2);
                    (true, v40, v39)
                } else {
                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (false, -1i32, v41)
                };
                let v46: bool = if v42 {
                    let v45: bool = v43 == 0i32;
                    v45
                } else {
                    false
                };
                let v48: i32 = if v46 {
                    let v47: i32 = v4.wrapping_add(1i32);
                    v47
                } else {
                    v4
                };
                (v3, v4) = (v6, v48);
                continue;
            }
            UH2::UH2_Nil => {
                return v4;
            }
        }
    }
}
fn method19(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure2() -> Rc<dyn Fn((Rc<str>)) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US5> = Rc::new(move |mut v0: (Rc<str>)| -> US5 {
        let mut v1: Rc<str> = (v0);
        US5::US5_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn current_executable_18() -> Rc<str> {
    let mut v0: Option<std::path::PathBuf> = std::env::current_exe().ok();
    let mut v2: bool = true; let _optionm_map_ = v0.map(|x| { //;
    let mut v4: std::path::PathBuf = x;
    let v5: Rc<str> = Rc::<str>::from(v4.to_string_lossy().as_ref());
    let v7: bool = true; v5 });
    let mut v9: Option<Rc<str>> = _optionm_map_;
    let v10: Option<Rc<str>> = method19(v9);
    let mut v11: Rc<dyn Fn((Rc<str>)) -> US5> = closure2();
    let mut v12: Option<US5> = v10.map(|x| v11(x));
    let v14: US5 = US5::US5_None;
    let v15: US5 = v14.clone();
    let mut v16: US5 = v12.unwrap_or(v15);
    match v16 {
        US5::US5_None => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v18
        }
        US5::US5_Some(v17) => {
            let mut v17: Rc<str> = v17;
            v17
        }
    }
}
fn carried_noted_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH2>) -> i32 {
    let v3: Rc<str> = current_executable_18();
    let mut v4: i32 = 0i32;
    method16(v0, v1, v3, v2, v4)
}
fn sprint_closed_8(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = string_concat(&[&*v4, "/state"]);
    let v6: Rc<str> = string_concat(&[&*v5, "/sprints.spi"]);
    let v7: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v6).unwrap_or_default().as_str());
    let v8: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>> = rig_agile_sprint_state::rig_agile_sprint_head(&v7, &v1);
    let mut v9: i32 = (v8.clone().borrow().len() as i32);
    let v10: bool = v9 == 0i32;
    let mut v17: bool = if v10 {
        true
    } else {
        let (mut v11, mut v12, mut v13, mut v14, mut v15): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) = v8.borrow()[0i32 as usize].clone();
        let mut v16: bool = v15 == false;
        v16
    };
    if v17 {
        let v18: Rc<str> = string_concat(&["kit error: agile sprint close refused: no open sprint ", &*v1]);
        eprintln!("{}", v18);
        1i32
    } else {
        let v19: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> = rig_agile_sprint_state::rig_agile_sprint_items(&v7, &v1);
        let v20: Rc<str> = string_concat(&[&*v5, "/agile.spi"]);
        let v21: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v20).unwrap_or_default().as_str());
        let v22: i32 = 0i32;
        let v23: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
        let v24: Rc<RefCell<Vec<Rc<str>>>> = finished_from_9(v21.clone(), &v19, v22, v23);
        let v25: Rc<RefCell<Vec<(i32, i32, i32)>>> = rig_agile_sprint_review::rig_agile_sprint_review(&v7, &v1, v24.clone());
        let v26: Rc<str> = rig_agile_sprint_review::rig_agile_sprint_closed(&v7, &v1, v24);
        std::fs::write(&*v6, &*v26).unwrap();
        let (mut v28, mut v29, mut v30): (i32, i32, i32) = v25.borrow()[0i32 as usize].clone();
        let v31: Rc<str> = format_real_11(v28);
        let v34: Rc<str> = format_real_11(v29);
        let v37: Rc<str> = format_real_11(v30);
        let v39: Rc<str> = string_concat(&[" done=", &*v31, "m carried=", &*v34, "m dropped=", &*v37, "m"]);
        let v40: bool = v29 == 0i32;
        let v43: Rc<str> = if v40 {
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
            v41
        } else {
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no"); } LIT.with(|lit| lit.clone()) };
            v42
        };
        let v44: i32 = 0i32;
        let v45: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let v46: Rc<UH2> = carried_from_13(v21, &v19, v44, v45);
        let v47: i32 = carried_noted_15(v0, v1.clone(), v46);
        let v48: Rc<str> = format_real_11(v47);
        let v54: Rc<str> = string_concat(&["kit agile sprint close ok id=", &*v1, &*v39, " goal_met=", &*v43, " carried_notes=", &*v48]);
        println!("{}", v54);
        0i32
    }
}
fn closed_7(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    sprint_closed_8(v1, v0)
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        closed_7(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn sprint_velocity_21(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = string_concat(&[&*v4, "/state/sprints.spi"]);
    let v6: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v5).unwrap_or_default().as_str());
    let v7: i32 = rig_agile_sprint_state::rig_agile_sprint_velocity(&v6, v1);
    let v8: Rc<str> = format_real_11(v1);
    let mut v11: Rc<str> = format_real_11(v7);
    let mut v12: Rc<str> = string_concat(&["kit agile sprint velocity ok last=", &*v8, " minutes=", &*v11]);
    println!("{}", v12);
    0i32
}
fn velocity_shown_20(mut v0: i32, mut v1: Rc<str>) -> i32 {
    sprint_velocity_21(v1, v0)
}
fn closure3() -> Rc<dyn Fn(Rc<str>, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: i32| -> i32 {
        velocity_shown_20(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_sprint_changed(v0: &str, v1: &str, v2: &str, v3: bool) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3)
}
pub fn rig_agile_sprint_closed_run(v0: &str, v1: &str) -> i32 {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_agile_sprint_velocity_shown(v0: &str, v1: i32) -> i32 {
    closure3()(Rc::<str>::from(v0), v1)
}
use rig_agile_sprint_state::SpiralShape_2617b4e49837 as Mut0;
use agile_task_census::SpiralShape_97df63ebabc3 as Mut1;
