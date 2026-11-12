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
    US0_TaskLine(Rc<str>, Rc<str>, Rc<str>),
    US0_PlanningLine(Rc<str>, i32, Rc<str>, Rc<str>),
    US0_NoteLine(Rc<str>, Rc<str>),
    US0_WaitLine(Rc<str>, Rc<str>),
    US0_MarkerLine(Rc<str>),
    US0_OtherLine,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH1>),
}
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn method7(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method7(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
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
        method6(v16, v17)
    }
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method8(v3, v1);
            let v5: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method9(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure1() -> Rc<dyn Fn((i32)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US1> = Rc::new(move |mut v0: (i32)| -> US1 {
        let v1: i32 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_line_4(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = v0.borrow()[2i32 as usize].clone();
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let v6: Rc<UH0> = split_list_5(v1, v5);
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = method8(v6, v7);
    match Rc::unwrap_or_clone(v8) {
        UH0::UH0_Cons(v9, v10) => {
            let mut v9: Rc<str> = v9;
            let v10: Rc<UH0> = v10;
            match Rc::unwrap_or_clone(v10) {
                UH0::UH0_Cons(v11, v12) => {
                    let v11: Rc<str> = v11;
                    let v12: Rc<UH0> = v12;
                    match Rc::unwrap_or_clone(v12) {
                        UH0::UH0_Cons(v13, v14) => {
                            let v13: Rc<str> = v13;
                            let v14: Rc<UH0> = v14;
                            match Rc::unwrap_or_clone(v14) {
                                UH0::UH0_Cons(v15, v16) => {
                                    let v15: Rc<str> = v15;
                                    let v16: Rc<UH0> = v16;
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Minutes "); } LIT.with(|lit| lit.clone()) };
                                    let v18: bool = v11.starts_with(&*v17);
                                    let v20: Rc<str> = if v18 {
                                        let v19: Rc<str> = Rc::<str>::from(v11.replace(&*v17, &*v4));
                                        v19
                                    } else {
                                        v4.clone()
                                    };
                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                                    let v22: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v4));
                                    let v23: Option<i32> = (v22).trim().parse::<i32>().ok();
                                    let v24: Option<i32> = method9(v23);
                                    let mut v25: Rc<dyn Fn((i32)) -> US1> = closure1();
                                    let mut v26: Option<US1> = v24.map(|x| v25(x));
                                    let v28: US1 = US1::US1_None;
                                    let v29: US1 = v28.clone();
                                    let v30: US1 = v26.unwrap_or(v29);
                                    let v33: i32 = match v30 {
                                        US1::US1_None => {
                                            0i32
                                        }
                                        US1::US1_Some(v31) => {
                                            let v31: i32 = v31;
                                            v31
                                        }
                                    };
                                    let v34: Rc<str> = v0.borrow()[1i32 as usize].clone();
                                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load "); } LIT.with(|lit| lit.clone()) };
                                    let v36: bool = v13.starts_with(&*v35);
                                    let mut v38: Rc<str> = if v36 {
                                        let v37: Rc<str> = Rc::<str>::from(v13.replace(&*v35, &*v4));
                                        v37
                                    } else {
                                        v4.clone()
                                    };
                                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) };
                                    let v40: bool = v15.starts_with(&*v39);
                                    let v42: Rc<str> = if v40 {
                                        let v41: Rc<str> = Rc::<str>::from(v15.replace(&*v39, &*v4));
                                        v41
                                    } else {
                                        v4
                                    };
                                    US0::US0_PlanningLine(v34, v33, v38, v42)
                                }
                                _ => {
                                    US0::US0_OtherLine
                                }
                            }
                        }
                        _ => {
                            US0::US0_OtherLine
                        }
                    }
                }
                _ => {
                    US0::US0_OtherLine
                }
            }
        }
        _ => {
            US0::US0_OtherLine
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method11(v3, v1);
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                (v0, v1) = (v3, v2);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn task_line_10(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> US0 {
    let v1: i32 = (v0.clone().borrow().len() as i32);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = v1.wrapping_sub(9i32);
    let v4: Rc<str> = v0.borrow()[v3 as usize].clone();
    let v5: Rc<UH0> = split_list_5(v2, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method8(v5, v6);
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method11(v7, v8);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = method12(v9, v10);
    let v12: Rc<str> = v0.borrow()[1i32 as usize].clone();
    let v13: i32 = v1.wrapping_sub(6i32);
    let v14: Rc<str> = v0.borrow()[v13 as usize].clone();
    US0::US0_TaskLine(v12, v11, v14)
}
fn state_line_of_3(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 < 3i32;
    if v4 {
        US0::US0_OtherLine
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl note_of_"); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v0.starts_with(&*v6);
        if v7 {
            let mut v8: Rc<str> = v2.borrow()[1i32 as usize].clone();
            US0::US0_NoteLine(v8, v0)
        } else {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl wait_of_"); } LIT.with(|lit| lit.clone()) };
            let v11: bool = v0.starts_with(&*v10);
            let v14: bool = if v11 {
                true
            } else {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= wait_of_"); } LIT.with(|lit| lit.clone()) };
                let v13: bool = v0.starts_with(&*v12);
                v13
            };
            let v16: bool = if v14 {
                let v15: bool = v3 >= 7i32;
                v15
            } else {
                false
            };
            if v16 {
                let v17: Rc<str> = v2.borrow()[1i32 as usize].clone();
                let v18: Rc<str> = v2.borrow()[3i32 as usize].clone();
                let v20: Rc<str> = v2.borrow()[5i32 as usize].clone();
                let v21: Rc<str> = string_concat(&[&*v18, ":", &*v20]);
                US0::US0_WaitLine(v17, v21)
            } else {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Marker (\""); } LIT.with(|lit| lit.clone()) };
                let v24: bool = v0.contains(&*v23);
                let mut v26: bool = if v24 {
                    let mut v25: bool = v3 >= 5i32;
                    v25
                } else {
                    false
                };
                if v26 {
                    let v27: Rc<str> = v2.borrow()[1i32 as usize].clone();
                    let v29: Rc<str> = v2.borrow()[3i32 as usize].clone();
                    let v30: Rc<str> = string_concat(&[&*v27, "=", &*v29]);
                    US0::US0_MarkerLine(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl planning_of_"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v0.starts_with(&*v32);
                    if v33 {
                        planning_line_4(v2)
                    } else {
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                        let v36: bool = v0.starts_with(&*v35);
                        let v39: bool = if v36 {
                            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() = Task ("); } LIT.with(|lit| lit.clone()) };
                            let mut v38: bool = v0.contains(&*v37);
                            v38
                        } else {
                            false
                        };
                        let v41: bool = if v39 {
                            let v40: bool = v3 >= 10i32;
                            v40
                        } else {
                            false
                        };
                        if v41 {
                            task_line_10(v2)
                        } else {
                            US0::US0_OtherLine
                        }
                    }
                }
            }
        }
    }
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: US0 = state_line_of_3(v2);
                let v10: Rc<str> = match v4 {
                    US0::US0_MarkerLine(v5) => {
                        let v5: Rc<str> = v5;
                        let v6: bool = &*v1 == "";
                        if v6 {
                            v5
                        } else {
                            let mut v8: Rc<str> = string_concat(&[&*v1, "\n", &*v5]);
                            v8
                        }
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v3, v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn declared_markers_1(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_5(v1, v4);
    method2(v5, v3)
}
fn method16(mut v0: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>) -> Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    v0
}
fn closure2() -> Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US2> = Rc::new(move |mut v0: ((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))| -> US2 {
        let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = (v0);
        US2::US2_Some(v1, v2, v3, v4, v5, v6, v7, v8)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn folded_15(mut v0: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>, mut v1: US0) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> {
    match v1 {
        US0::US0_MarkerLine(v175) => {
            let v175: Rc<str> = v175;
            v0
        }
        US0::US0_NoteLine(v87, v88) => {
            let v87: Rc<str> = v87;
            let v88: Rc<str> = v88;
            let v89: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v87).cloned();
            let v90: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method16(v89);
            let mut v91: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US2> = closure2();
            let mut v92: Option<US2> = v90.map(|x| v91(x));
            let v94: US2 = US2::US2_None;
            let v95: US2 = v94.clone();
            let v96: US2 = v92.unwrap_or(v95);
            let (mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v96 {
                US2::US2_Some(v97, v98, v99, v100, v101, v102, v103, v104) => {
                    let v97: Rc<str> = v97.clone();
                    let v98: Rc<str> = v98.clone();
                    let v99: i32 = *v99;
                    let v100: Rc<str> = v100.clone();
                    let v101: Rc<str> = v101.clone();
                    let v102: Rc<str> = v102.clone();
                    let v103: Rc<str> = v103.clone();
                    let v104: Rc<str> = v104.clone();
                    (v97, v98, v99, v100, v101, v102, v103, v104)
                }
                _ => {
                    let (mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v96 {
                        US2::US2_None => {
                            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v105.clone(), v105.clone(), 0i32, v105.clone(), v105.clone(), v105.clone(), v105.clone(), v105.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v106, v107, v108, v109, v110, v111, v112, v113)
                }
            };
            let v123: Rc<str> = string_concat(&[&*v119, "\n", &*v88]);
            let mut v124: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v126: bool = true; // 1;
            let mut v124 = v124;
            let v128: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v124.insert(v87, (v114, v115, v116, v117, v118, v123, v120, v121));
            let v129: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v124);
            v129
        }
        US0::US0_OtherLine => {
            v0
        }
        US0::US0_PlanningLine(v44, v45, v46, v47) => {
            let v44: Rc<str> = v44;
            let v45: i32 = v45;
            let v46: Rc<str> = v46;
            let v47: Rc<str> = v47;
            let v48: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v44).cloned();
            let v49: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method16(v48);
            let mut v50: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US2> = closure2();
            let mut v51: Option<US2> = v49.map(|x| v50(x));
            let v53: US2 = US2::US2_None;
            let v54: US2 = v53.clone();
            let v55: US2 = v51.unwrap_or(v54);
            let (mut v73, mut v74, mut v78, mut v79, mut v80): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v55 {
                US2::US2_Some(v56, v57, v58, v59, v60, v61, v62, v63) => {
                    let v56: Rc<str> = v56.clone();
                    let v57: Rc<str> = v57.clone();
                    let v58: i32 = *v58;
                    let v59: Rc<str> = v59.clone();
                    let v60: Rc<str> = v60.clone();
                    let v61: Rc<str> = v61.clone();
                    let v62: Rc<str> = v62.clone();
                    let v63: Rc<str> = v63.clone();
                    (v56, v57, v61, v62, v63)
                }
                _ => {
                    let (mut v65, mut v66, mut v70, mut v71, mut v72): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v55 {
                        US2::US2_None => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v64.clone(), v64.clone(), v64.clone(), v64.clone(), v64.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v65, v66, v70, v71, v72)
                }
            };
            let mut v81: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v83: bool = true; // 1;
            let mut v81 = v81;
            let v85: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v81.insert(v44, (v73, v74, v45, v46, v47, v78, v79, v80));
            let v86: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v81);
            v86
        }
        US0::US0_TaskLine(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v2).cloned();
            let v6: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method16(v5);
            let mut v7: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US2> = closure2();
            let mut v8: Option<US2> = v6.map(|x| v7(x));
            let v10: US2 = US2::US2_None;
            let v11: US2 = v10.clone();
            let v12: US2 = v8.unwrap_or(v11);
            let (mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                US2::US2_Some(v13, v14, v15, v16, v17, v18, v19, v20) => {
                    let v13: Rc<str> = v13.clone();
                    let v14: Rc<str> = v14.clone();
                    let v15: i32 = *v15;
                    let v16: Rc<str> = v16.clone();
                    let v17: Rc<str> = v17.clone();
                    let v18: Rc<str> = v18.clone();
                    let v19: Rc<str> = v19.clone();
                    let v20: Rc<str> = v20.clone();
                    (v15, v16, v17, v18, v19, v20)
                }
                _ => {
                    let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v12 {
                        US2::US2_None => {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (0i32, v21.clone(), v21.clone(), v21.clone(), v21.clone(), v21.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v24, v25, v26, v27, v28, v29)
                }
            };
            let mut v38: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v40: bool = true; // 1;
            let mut v38 = v38;
            let v42: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v38.insert(v2, (v3, v4, v32, v33, v34, v35, v36, v37));
            let v43: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v38);
            v43
        }
        US0::US0_WaitLine(v130, v131) => {
            let v130: Rc<str> = v130;
            let v131: Rc<str> = v131;
            let v132: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v130).cloned();
            let v133: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method16(v132);
            let mut v134: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US2> = closure2();
            let mut v135: Option<US2> = v133.map(|x| v134(x));
            let v137: US2 = US2::US2_None;
            let v138: US2 = v137.clone();
            let v139: US2 = v135.unwrap_or(v138);
            let (mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v139 {
                US2::US2_Some(v140, v141, v142, v143, v144, v145, v146, v147) => {
                    let v140: Rc<str> = v140.clone();
                    let v141: Rc<str> = v141.clone();
                    let v142: i32 = *v142;
                    let v143: Rc<str> = v143.clone();
                    let v144: Rc<str> = v144.clone();
                    let v145: Rc<str> = v145.clone();
                    let v146: Rc<str> = v146.clone();
                    let v147: Rc<str> = v147.clone();
                    (v140, v141, v142, v143, v144, v145, v146, v147)
                }
                _ => {
                    let (mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v139 {
                        US2::US2_None => {
                            let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v148.clone(), v148.clone(), 0i32, v148.clone(), v148.clone(), v148.clone(), v148.clone(), v148.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v149, v150, v151, v152, v153, v154, v155, v156)
                }
            };
            let v165: bool = &*v163 == "";
            let v168: Rc<str> = if v165 {
                v131
            } else {
                let v167: Rc<str> = string_concat(&[&*v163, ",", &*v131]);
                v167
            };
            let mut v169: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v171: bool = true; // 1;
            let mut v169 = v169;
            let v173: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v169.insert(v130, (v157, v158, v159, v160, v161, v162, v168, v164));
            let v174: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v169);
            v174
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: US0 = state_line_of_3(v2);
                let v5: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = folded_15(v1, v4);
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn facts_of_13(mut v0: Rc<str>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_5(v1, v4);
    let mut v7: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::collections::BTreeMap::new();
    let mut v8: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v7);
    method14(v5, v8)
}
fn method17(mut v0: Rc<str>, mut v1: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>, mut v2: Rc<UH0>, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v1.get(&v4).cloned();
                let mut v7: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method16(v6);
                let mut v8: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US2> = closure2();
                let mut v9: Option<US2> = v7.map(|x| v8(x));
                let v11: US2 = US2::US2_None;
                let v12: US2 = v11.clone();
                let v13: US2 = v9.unwrap_or(v12);
                let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v13 {
                    US2::US2_Some(v14, v15, v16, v17, v18, v19, v20, v21) => {
                        let v14: Rc<str> = v14.clone();
                        let v15: Rc<str> = v15.clone();
                        let v16: i32 = *v16;
                        let v17: Rc<str> = v17.clone();
                        let v18: Rc<str> = v18.clone();
                        let v19: Rc<str> = v19.clone();
                        let v20: Rc<str> = v20.clone();
                        let v21: Rc<str> = v21.clone();
                        (v14, v15, v16, v17, v18, v19, v20, v21)
                    }
                    _ => {
                        let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v13 {
                            US2::US2_None => {
                                std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dict.present: the key is not in the dict"); } LIT.with(|lit| lit.clone()) }))
                            }
                            _ => unreachable!(),
                        };
                        (v30, v31, v32, v33, v34, v35, v36, v37)
                    }
                };
                let v46: bool = &*v38 == "";
                let v48: Rc<UH1> = if v46 {
                    v3
                } else {
                    Rc::new(UH1::UH1_Cons(v4, v38, v39, v40, v41, v42, v43, v44, v0.clone(), v3))
                };
                (v2, v3) = (v5, v48);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn method18(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: i32 = v5;
                let v6: Rc<str> = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<UH1> = v11;
                let v12: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10, v1));
                (v0, v1) = (v11, v12);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method19(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: i32 = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<str> = v11;
                let v12: Rc<UH1> = v12;
                v0.borrow_mut().push((v3, v4, v5, v6, v7, v8, v9, v10, v11));
                let v13: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v12, v13);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn fact_rows_of_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    let v1: Rc<str> = declared_markers_1(v0.clone());
    let mut v2: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = facts_of_13(v0);
    let mut v3: Vec<Rc<str>> = v2.keys().cloned().collect::<Vec<_>>();
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: bool = true; let _vec_fold_ = v3.into_iter().fold(v4, |acc, x| { //;
    let mut v8: Rc<UH0> = acc;
    let v10: Rc<str> = x;
    let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v8));
    let v13: bool = true; v12 });
    let v15: Rc<UH0> = _vec_fold_;
    let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH0> = method6(v15, v16);
    let v18: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v19: Rc<UH1> = method17(v1, v2, v17, v18);
    let v20: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v21: Rc<UH1> = method18(v19, v20);
    let v22: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v23: i32 = 0i32;
    let v24: i32 = method19(v22.clone(), v21, v23);
    let mut v25: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new(v22.borrow().clone());
    let mut v26: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v25).as_ref().clone()));
    v26
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
        fact_rows_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_fact_rows(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub type SpiralShape_652accfc6a8d = Mut0;
