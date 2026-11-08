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
    US2_SwitchQuiet,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum US4 {
    US4_ValuedGlob,
    US4_ValuedSince,
    US4_ValuedOut,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US4),
    US3_None,
}
pub struct Mut0 { pub l0: i32 }
fn command_line_argument_count_2() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_1(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_2();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_3(v0);
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
            let v12: Rc<UH0> = arguments_from_1(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn dropped_4(mut v0: i32, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: bool = v0 > 0i32;
                if v4 {
                    let v5: i32 = v0.wrapping_sub(1i32);
                    (v0, v1) = (v5, v3);
                    continue;
                } else {
                    return v1;
                }
            }
            _ => {
                return v1;
            }
        }
    }
}
fn finished_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: Rc<str>, mut v6: Rc<RefCell<Vec<Rc<str>>>>, mut v7: Rc<RefCell<Vec<Rc<str>>>>, mut v8: Rc<RefCell<Vec<Rc<str>>>>, mut v9: Rc<RefCell<Vec<Rc<str>>>>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) {
    (v0, v1, v2, v3, v4, v5, v8, v9)
}
fn valued_applied_8(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: bool, mut v7: Rc<str>, mut v8: Rc<RefCell<Vec<Rc<str>>>>, mut v9: Rc<RefCell<Vec<Rc<str>>>>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) {
    let (mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) = match &v0 {
        US4::US4_ValuedGlob => {
            (v2, v1, v4, v5, v6, v7, v8, v9)
        }
        _ => {
            let (mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) = match &v0 {
                US4::US4_ValuedSince => {
                    (v2, v3, v1, v5, v6, v7, v8, v9)
                }
                _ => {
                    let (mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) = match v0 {
                        US4::US4_ValuedOut => {
                            (v2, v3, v4, v1, v6, v7, v8, v9)
                        }
                        _ => unreachable!(),
                    };
                    (v10, v11, v12, v13, v14, v15, v16, v17)
                }
            };
            (v18, v19, v20, v21, v22, v23, v24, v25)
        }
    };
    (v26, v27, v28, v29, v30, v31, v32, v33)
}
fn method9(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn words_read_6(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: Rc<str>, mut v7: Rc<RefCell<Vec<Rc<str>>>>, mut v8: Rc<RefCell<Vec<Rc<str>>>>, mut v9: Rc<RefCell<Vec<Rc<str>>>>, mut v10: Rc<RefCell<Vec<Rc<str>>>>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) {
    loop {
        match &*v0 {
            UH0::UH0_Nil => {
                return finished_7(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10);
            }
            _ => {
                match Rc::unwrap_or_clone(v0) {
                    UH0::UH0_Cons(v19, v20) => {
                        let v19: Rc<str> = v19;
                        let v20: Rc<UH0> = v20;
                        let v21: bool = &*v19 == "--quiet";
                        let v25: US1 = if v21 {
                            let v22: US2 = US2::US2_SwitchQuiet;
                            US1::US1_Some(v22)
                        } else {
                            US1::US1_None
                        };
                        let v26: bool = &*v19 == "--glob";
                        let v38: US3 = if v26 {
                            let v27: US4 = US4::US4_ValuedGlob;
                            US3::US3_Some(v27)
                        } else {
                            let v29: bool = &*v19 == "--since";
                            if v29 {
                                let v30: US4 = US4::US4_ValuedSince;
                                US3::US3_Some(v30)
                            } else {
                                let v32: bool = &*v19 == "--out";
                                if v32 {
                                    let v33: US4 = US4::US4_ValuedOut;
                                    US3::US3_Some(v33)
                                } else {
                                    US3::US3_None
                                }
                            }
                        };
                        match v25 {
                            US1::US1_Some(v39) => {
                                let v39: US2 = v39;
                                let v40: bool = true;
                                (v0, v5) = (v20, v40);
                                continue;
                            }
                            _ => {
                                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                                let v50: bool = v19.starts_with(&*v49);
                                let v51: bool = v50 == false;
                                if v51 {
                                    let v52: bool = &*v1 == "";
                                    let v55: Rc<str> = if v52 {
                                        v19
                                    } else {
                                        let v54: Rc<str> = string_concat(&[&*v1, "\n", &*v19]);
                                        v54
                                    };
                                    (v0, v1) = (v20, v55);
                                    continue;
                                } else {
                                    match &*v20 {
                                        UH0::UH0_Nil => {
                                            let v65: Rc<str> = string_concat(&["option ", &*v19, " has no value"]);
                                            return finished_7(v1, v2, v3, v4, v5, v65, v7, v8, v9, v10);
                                        }
                                        _ => {
                                            match v38 {
                                                US3::US3_Some(v74) => {
                                                    let v74: US4 = v74;
                                                    match Rc::unwrap_or_clone(v20) {
                                                        UH0::UH0_Cons(v75, v76) => {
                                                            let v75: Rc<str> = v75;
                                                            let v76: Rc<UH0> = v76;
                                                            let (mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) = valued_applied_8(v74, v75, v1, v2, v3, v4, v5, v6, v7, v8);
                                                            (v0, v1, v2, v3, v4, v5, v6, v7, v8) = (v76, v77, v78, v79, v80, v81, v82, v83, v84);
                                                            continue;
                                                        }
                                                        _ => unreachable!(),
                                                    }
                                                }
                                                _ => {
                                                    match Rc::unwrap_or_clone(v20) {
                                                        UH0::UH0_Cons(v101, v102) => {
                                                            let v101: Rc<str> = v101;
                                                            let v102: Rc<UH0> = v102;
                                                            let v103: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v19]));
                                                            let v104: i32 = (v9.clone().borrow().len() as i32);
                                                            let v105: i32 = (v103.clone().borrow().len() as i32);
                                                            let v106: i32 = v104.wrapping_add(v105);
                                                            let mut v107: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v106 as usize]));
                                                            let mut v108: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                                                            while method9(v106, &v108) {
                                                                let v110: i32 = v108.borrow().l0;
                                                                let v111: bool = v110 < v104;
                                                                let v115: Rc<str> = if v111 {
                                                                    let v112: Rc<str> = v9.borrow()[v110 as usize].clone();
                                                                    v112
                                                                } else {
                                                                    let v113: i32 = v110.wrapping_sub(v104);
                                                                    let v114: Rc<str> = v103.borrow()[v113 as usize].clone();
                                                                    v114
                                                                };
                                                                v107.borrow_mut()[v110 as usize] = v115;
                                                                let v116: i32 = v110.wrapping_add(1i32);
                                                                v108.borrow_mut().l0 = v116;
                                                                ()
                                                            };
                                                            let v117: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v101]));
                                                            let v118: i32 = (v10.clone().borrow().len() as i32);
                                                            let v119: i32 = (v117.clone().borrow().len() as i32);
                                                            let v120: i32 = v118.wrapping_add(v119);
                                                            let mut v121: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v120 as usize]));
                                                            let mut v122: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                                                            while method9(v120, &v122) {
                                                                let v124: i32 = v122.borrow().l0;
                                                                let v125: bool = v124 < v118;
                                                                let v129: Rc<str> = if v125 {
                                                                    let v126: Rc<str> = v10.borrow()[v124 as usize].clone();
                                                                    v126
                                                                } else {
                                                                    let v127: i32 = v124.wrapping_sub(v118);
                                                                    let v128: Rc<str> = v117.borrow()[v127 as usize].clone();
                                                                    v128
                                                                };
                                                                v121.borrow_mut()[v124 as usize] = v129;
                                                                let v130: i32 = v124.wrapping_add(1i32);
                                                                v122.borrow_mut().l0 = v130;
                                                                ()
                                                            };
                                                            (v0, v9, v10) = (v102, v107, v121);
                                                            continue;
                                                        }
                                                        _ => unreachable!(),
                                                    }
                                                }
                                            }
                                        }
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
fn args_of_5(mut v0: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) {
    let v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: bool = false;
    let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) = words_read_6(v0, v4.clone(), v4.clone(), v4.clone(), v4.clone(), v5, v4, v1.clone(), v1, v2, v3);
    (v6, v7, v8, v9, v10, v11, v12, v13)
}
fn logs_args_read_0(mut v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    let v1: i32 = 0i32;
    let v2: Rc<UH0> = arguments_from_1(v1);
    let v3: Rc<UH0> = dropped_4(v0, v2);
    let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) = args_of_5(v3);
    Rc::new(RefCell::new(vec![(v4, v5, v6, v7, v8, v9, v10, v11)]))
}
fn closure0() -> Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>> = Rc::new(move |mut v0: i32| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
        logs_args_read_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_table_logs_args_read(v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    closure0()(v0)
}
pub type SpiralShape_2617b4e49837 = Mut0;
