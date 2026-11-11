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
    US1_KitPrefix,
    US1_EoiePrefix,
}
#[derive(Clone)]
enum US0 {
    US0_Some(US1),
    US0_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, Rc<str>),
    US2_None,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US3 {
    US3_Some(i64),
    US3_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn prefix_word_2(mut v0: US1) -> Rc<str> {
    match v0 {
        US1::US1_EoiePrefix => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EOIE-"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US1::US1_KitPrefix => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT-"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    }
}
fn retired_prefix_of_1(mut v0: Rc<str>) -> US0 {
    let v1: US1 = US1::US1_KitPrefix;
    let v2: Rc<str> = prefix_word_2(v1);
    let v3: bool = v0.starts_with(&*v2);
    if v3 {
        let v4: US1 = US1::US1_KitPrefix;
        US0::US0_Some(v4)
    } else {
        let v6: US1 = US1::US1_EoiePrefix;
        let v7: Rc<str> = prefix_word_2(v6);
        let v8: bool = v0.starts_with(&*v7);
        if v8 {
            let mut v9: US1 = US1::US1_EoiePrefix;
            US0::US0_Some(v9)
        } else {
            US0::US0_None
        }
    }
}
fn rig_spelling_3(mut v0: US1, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = prefix_word_2(v0);
    let v3: i32 = (v2.len() as i32);
    let v4: i32 = (v1.clone().len() as i32);
    let v5: i32 = v4.wrapping_sub(v3);
    let v6: usize = ((v3) as usize);
    let v7: i32 = v3.wrapping_add(v5);
    let v8: usize = ((v7) as usize);
    let mut v9: Rc<str> = Rc::<str>::from(&(v1)[v6..v8]);
    let v10: Rc<str> = string_concat(&["RIG-", &*v9]);
    v10
}
fn new_id_refusal_0(mut v0: Rc<str>) -> Rc<str> {
    let v1: US0 = retired_prefix_of_1(v0.clone());
    match v1 {
        US0::US0_None => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US0::US0_Some(v2) => {
            let v2: US1 = v2;
            let v3: Rc<str> = prefix_word_2(v2.clone());
            let v6: Rc<str> = rig_spelling_3(v2, v0);
            let v7: Rc<str> = string_concat(&["the ", &*v3, " id prefix is retired (the tool and team are rig since 11-08); use ", &*v6]);
            v7
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        new_id_refusal_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method9(v16, v17)
    }
}
fn method12(mut v0: Option<i64>) -> Option<i64> {
    v0
}
fn closure2() -> Rc<dyn Fn((i64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i64)) -> US3> = Rc::new(move |mut v0: (i64)| -> US3 {
        let v1: i64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn minute_of_11(mut v0: Rc<str>) -> US3 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v2));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v2));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v2));
    let v10: i32 = (v9.clone().len() as i32);
    let v11: bool = v10 == 12i32;
    if v11 {
        let v12: Option<i64> = (v9).trim().parse::<i64>().ok();
        let v13: Option<i64> = method12(v12);
        let mut v14: Rc<dyn Fn((i64)) -> US3> = closure2();
        let mut v15: Option<US3> = v13.map(|x| v14(x));
        let v17: US3 = US3::US3_None;
        let v18: US3 = v17.clone();
        let v19: US3 = v15.unwrap_or(v18);
        v19
    } else {
        US3::US3_None
    }
}
fn decision_minute_13() -> i64 {
    202611082340i64
}
fn late_id_of_line_7(mut v0: Rc<str>) -> US2 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= Note (\""); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.contains(&*v1);
    let v3: bool = v2 == false;
    if v3 {
        US2::US2_None
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<UH0> = split_list_8(v5, v0);
        match Rc::unwrap_or_clone(v6) {
            UH0::UH0_Cons(v7, v8) => {
                let v7: Rc<str> = v7;
                let v8: Rc<UH0> = v8;
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
                                        let mut v14: Rc<UH0> = v14;
                                        let mut v15: Rc<str> = Rc::<str>::from((v11).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                        let v16: bool = &*v15 == ", 1u32,";
                                        if v16 {
                                            let v17: US0 = retired_prefix_of_1(v9.clone());
                                            let v18: US3 = minute_of_11(v13.clone());
                                            match v17 {
                                                US0::US0_Some(v19) => {
                                                    let v19: US1 = v19;
                                                    match v18 {
                                                        US3::US3_Some(v20) => {
                                                            let v20: i64 = v20;
                                                            let v21: i64 = decision_minute_13();
                                                            let v22: bool = v20 > v21;
                                                            if v22 {
                                                                US2::US2_Some(v9, v13)
                                                            } else {
                                                                US2::US2_None
                                                            }
                                                        }
                                                        _ => {
                                                            US2::US2_None
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    US2::US2_None
                                                }
                                            }
                                        } else {
                                            US2::US2_None
                                        }
                                    }
                                    _ => {
                                        US2::US2_None
                                    }
                                }
                            }
                            _ => {
                                US2::US2_None
                            }
                        }
                    }
                    _ => {
                        US2::US2_None
                    }
                }
            }
            _ => {
                US2::US2_None
            }
        }
    }
}
fn method14(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<UH1> = v4;
            let v5: Rc<UH1> = method14(v4, v1);
            Rc::new(UH1::UH1_Cons(v2, v3, v5))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: US2 = late_id_of_line_7(v2);
                let v11: Rc<UH1> = match v4 {
                    US2::US2_None => {
                        v1
                    }
                    US2::US2_Some(v5, v6) => {
                        let v5: Rc<str> = v5;
                        let v6: Rc<str> = v6;
                        let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v8: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v6, v7));
                        method14(v1, v8)
                    }
                };
                (v0, v1) = (v3, v11);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn late_ids_5(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_8(v1, v4);
    let v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method6(v5, v6)
}
fn late_line_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = string_concat(&[&*v0, " filed ", &*v1, " retired-prefix"]);
    v4
}
fn method15(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<UH1> = v4;
            let v5: Rc<UH0> = method15(v4, v1);
            let v6: Rc<str> = late_line_16(v2, v3);
            Rc::new(UH0::UH0_Cons(v6, v5))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method17(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method19(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_18(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method19(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method20(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method21(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn retired_report_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<UH1> = late_ids_5(v0);
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH0> = method15(v1, v2);
    let v4: i32 = 0i32;
    let v5: i32 = method17(v3.clone(), v4);
    let v6: Rc<str> = format_real_18(v5);
    let v8: Rc<str> = string_concat(&["rig agile lint retired-prefix ids=", &*v6, " (KIT- or EOIE- ids filed after the 11-08 rig decision)"]);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v9));
    let v11: Rc<UH0> = method20(v3, v10);
    let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v13: i32 = 0i32;
    let mut v14: i32 = method21(v12.clone(), v11, v13);
    let mut v15: Rc<Vec<Rc<str>>> = Rc::new(v12.borrow().clone());
    let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v17));
    v18
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        retired_report_4(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_new_id_refusal(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_agile_retired_report(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
