#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
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
    UH0_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_UrgentAsk,
    US0_HighAsk,
    US0_LowerAsk,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, US0, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<UH2>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH2> }
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, US0),
    US1_None,
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, US0, Rc<str>, i64, Rc<UH3>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, Rc<str>, Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(i32),
    US3_None,
}
fn method1(mut v0: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>)>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>) = (v0)[v4].clone();
            let v8: i32 = v1.wrapping_sub(1i32);
            let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v6, v7, v2));
            (v1, v2) = (v8, v9);
            continue;
        }
    }
}
fn method3(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let v3: US0 = v3;
                let v4: Rc<UH1> = v4;
                let v5: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v1));
                (v0, v1) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method5(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method6(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH2> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH2::UH2_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method6(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH2> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH2> = Rc::new(UH2::UH2_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH2> = v9.borrow().l1.clone();
        let v17: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        method5(v16, v17)
    }
}
fn method10(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method11(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method9(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH2> = method9(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let v6: i32 = 0i32;
            let v7: i32 = method10(v2.clone(), v5, v6);
            let v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
            let v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method11(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            Rc::new(UH2::UH2_Cons(v12, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn priority_of_13(mut v0: Rc<str>) -> US0 {
    let mut v1: bool = &*v0 == "P0";
    if v1 {
        US0::US0_UrgentAsk
    } else {
        let v3: bool = &*v0 == "P1";
        if v3 {
            US0::US0_HighAsk
        } else {
            US0::US0_LowerAsk
        }
    }
}
fn method12(mut v0: Rc<UH2>, mut v1: US0) -> US0 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let v4: bool = &*v2 == "P0";
                let v6: bool = if v4 {
                    true
                } else {
                    let v5: bool = &*v2 == "P1";
                    v5
                };
                let v8: US0 = if v6 {
                    priority_of_13(v2)
                } else {
                    v1
                };
                (v0, v1) = (v3, v8);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn task_priority_8(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v5: bool = v0.starts_with(&*v4);
    let v6: bool = v5 == false;
    let v10: bool = if v6 {
        true
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() = Task ("); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v0.contains(&*v7);
        let mut v9: bool = v8 == false;
        v9
    };
    let v12: bool = if v10 {
        true
    } else {
        let v11: bool = v3 < 10i32;
        v11
    };
    if v12 {
        US1::US1_None
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v15: i32 = v3.wrapping_sub(9i32);
        let v16: Rc<str> = v2.borrow()[v15 as usize].clone();
        let v17: Rc<UH2> = split_list_4(v14, v16);
        let v18: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let v19: Rc<UH2> = method9(v17, v18);
        let v20: Rc<str> = v2.borrow()[1i32 as usize].clone();
        let v21: US0 = US0::US0_LowerAsk;
        let v22: US0 = method12(v19, v21);
        US1::US1_Some(v20, v22)
    }
}
fn method7(mut v0: Rc<UH2>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let v4: US1 = task_priority_8(v2);
                let mut v9: Rc<UH1> = match v4 {
                    US1::US1_None => {
                        v1
                    }
                    US1::US1_Some(v5, v6) => {
                        let v5: Rc<str> = v5;
                        let v6: US0 = v6;
                        Rc::new(UH1::UH1_Cons(v5, v6, v1))
                    }
                };
                (v0, v1) = (v3, v9);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn priorities_of_2(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH2> = split_list_4(v1, v4);
    let v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH1> = method7(v5, v6);
    let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method3(v7, v8)
}
fn method16(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: US0) -> US0 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5) => {
                let v3: Rc<str> = v3;
                let v4: US0 = v4;
                let v5: Rc<UH1> = v5;
                let v6: bool = &*v3 == &*v0;
                let v7: US0 = if v6 {
                    v4
                } else {
                    v2
                };
                (v1, v2) = (v5, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn priority_in_15(mut v0: Rc<UH1>, mut v1: Rc<str>) -> US0 {
    let mut v2: US0 = US0::US0_LowerAsk;
    method16(v1, v0, v2)
}
fn method17(mut v0: Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: i64, mut v4: Rc<UH0>, mut v5: US2) -> US2 {
    loop {
        match Rc::unwrap_or_clone(v4) {
            UH0::UH0_Cons(v6, v7, v8, v9) => {
                let v6: Rc<str> = v6;
                let v7: Rc<str> = v7;
                let v8: Rc<str> = v8;
                let mut v9: Rc<UH0> = v9;
                let v10: bool = &*v6 == &*v0;
                let v12: US2 = if v10 {
                    US2::US2_Some(v6, v7, v8)
                } else {
                    v5
                };
                (v4, v5) = (v9, v12);
                continue;
            }
            UH0::UH0_Nil => {
                return v5;
            }
        }
    }
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH2>) -> US3> {
    Rc::new(move |mut v2: Rc<UH2>| -> US3 {
        match Rc::unwrap_or_clone(v2) {
            UH2::UH2_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH2> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US3::US3_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH2>) -> US3> = method19(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH2::UH2_Nil => {
                US3::US3_None
            }
        }
    })
}
fn method19(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH2>) -> US3> {
    closure1(v0, v1)
}
fn team_tagged_18(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = &*v0 == "";
    if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<UH2> = split_list_4(v3, v1);
        let v5: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let v6: Rc<UH2> = method9(v4, v5);
        let v7: Rc<str> = string_concat(&["team:", &*v0]);
        let v8: i32 = 0i32;
        let mut v9: Rc<dyn Fn(Rc<UH2>) -> US3> = method19(v7, v8);
        let v10: US3 = v9(v6);
        let v11: bool = match v10 {
            US3::US3_None => {
                true
            }
            _ => {
                false
            }
        };
        let v12: bool = v11 != true;
        v12
    }
}
fn neglected_from_14(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>>, mut v1: i32, mut v2: Rc<UH1>, mut v3: Rc<UH0>, mut v4: i64, mut v5: i64, mut v6: Rc<str>, mut v7: Rc<UH3>) -> Rc<UH3> {
    loop {
        let v8: bool = v1 < 0i32;
        if v8 {
            return v7;
        } else {
            let (mut v9, mut v10, mut v11, mut v12): (Rc<str>, Rc<str>, Rc<str>, i64) = v0.borrow()[v1 as usize].clone();
            let v13: US0 = priority_in_15(v2.clone(), v9.clone());
            let v14: US2 = US2::US2_None;
            let v15: US2 = method17(v9.clone(), &v10, &v11, v12, v3.clone(), v14);
            let v34: Rc<UH3> = match v15 {
                US2::US2_Some(v16, v17, v18) => {
                    let v16: Rc<str> = v16;
                    let v17: Rc<str> = v17;
                    let v18: Rc<str> = v18;
                    let v19: bool = &*v10 == "Planned";
                    let v21: bool = if v19 {
                        match &v13 {
                            US0::US0_LowerAsk => {
                                false
                            }
                            _ => {
                                true
                            }
                        }
                    } else {
                        false
                    };
                    let v23: bool = if v21 {
                        team_tagged_18(v6.clone(), v11)
                    } else {
                        false
                    };
                    if v23 {
                        let v24: Rc<str> = string_concat(&[&*v17, "T00:00Z"]);
                        let v25: i64 = agile_quality_state::rig_agile_stamp_hour(&v24);
                        let v26: bool = v12 > v25;
                        let v27: i64 = if v26 {
                            v12
                        } else {
                            v25
                        };
                        let v28: i64 = v4.wrapping_sub(v27);
                        let v29: i64 = v5.wrapping_mul(24i64);
                        let v30: bool = v28 >= v29;
                        if v30 {
                            Rc::new(UH3::UH3_Cons(v9, v13, v17, v28, v7))
                        } else {
                            v7
                        }
                    } else {
                        v7
                    }
                }
                _ => {
                    v7
                }
            };
            let v35: i32 = v1.wrapping_sub(1i32);
            (v1, v7) = (v35, v34);
            continue;
        }
    }
}
fn method23(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_22(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method23(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn ask_line_21(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: i64) -> Rc<str> {
    let v5: Rc<str> = string_concat(&["rig agile cadence neglected-ask id=", &*v0, " priority="]);
    let v11: Rc<str> = match v1 {
        US0::US0_HighAsk => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US0::US0_LowerAsk => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lower"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US0::US0_UrgentAsk => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
            v6
        }
    };
    let v16: i64 = v3.wrapping_div(24i64);
    let v17: Rc<str> = format_real_22(v16);
    let v18: Rc<str> = string_concat(&[&*v5, &*v11, " asked=", &*v2, " idle_days=", &*v17]);
    v18
}
fn method20(mut v0: Rc<UH3>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_Cons(v2, v3, v4, v5, v6) => {
            let mut v2: Rc<str> = v2;
            let v3: US0 = v3;
            let v4: Rc<str> = v4;
            let v5: i64 = v5;
            let v6: Rc<UH3> = v6;
            let v7: Rc<UH2> = method20(v6, v1);
            let v8: Rc<str> = ask_line_21(v2, v3, v4, v5);
            Rc::new(UH2::UH2_Cons(v8, v7))
        }
        UH3::UH3_Nil => {
            v1
        }
    }
}
fn method24(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH2>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH2> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn asks_report_0(mut v0: Rc<str>, mut v1: i64, mut v2: i64, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v5: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>> = agile_quality_state::rig_agile_cadence_rows(&v4);
    let v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = agile_task_asks::rig_agile_asks(&v3);
    let v7: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>)>> = Rc::new((v6).borrow().clone());
    let v8: usize = (v7).len();
    let mut v9: i32 = ((v8) as i32);
    let v10: i32 = v9.wrapping_sub(1i32);
    let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH0> = method1(v7, v10, v11);
    let v13: Rc<UH1> = priorities_of_2(v4);
    let v14: i32 = (v5.clone().borrow().len() as i32);
    let v15: i32 = v14.wrapping_sub(1i32);
    let v16: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH3> = neglected_from_14(&v5, v15, v13, v12, v2, v1, v0, v16);
    let v18: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v19: Rc<UH2> = method20(v17, v18);
    let v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v21: i32 = 0i32;
    let v22: i32 = method24(v20.clone(), v19, v21);
    let v23: Rc<Vec<Rc<str>>> = Rc::new(v20.borrow().clone());
    let v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v23).as_ref().clone()));
    v24
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, i64, i64, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, i64, i64, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i64, mut v3: i64, mut v4: Rc<str>| -> Rc<RefCell<Vec<Rc<str>>>> {
        asks_report_0(v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_cadence_asks_report(v0: &str, v1: &str, v2: i64, v3: i64, v4: &str) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2, v3, Rc::<str>::from(v4))
}
use agile_task_asks::SpiralShape_97df63ebabc3 as Mut1;
pub type SpiralShape_0408665020a9 = Mut0;
