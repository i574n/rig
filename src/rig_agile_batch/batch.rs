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
    US2_NoteOp(Rc<str>, Rc<str>),
    US2_SetOp(Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US2_WaitOp(Rc<str>, Rc<str>),
    US2_EditOp(Rc<str>, Rc<str>, Rc<str>),
    US2_AskOp(Rc<str>, Rc<str>),
    US2_AfterOp(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US2, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Plan(Rc<UH1>),
    US1_PlanRefused(Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US3 {
    US3_Op(US2),
    US3_Skipped,
    US3_LineRefused(Rc<str>),
}
pub struct Mut1 { pub l0: Rc<str> }
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_1(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_2();
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
            let mut v11: i32 = v0.wrapping_add(1i32);
            let mut v12: Rc<UH0> = arguments_from_1(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        let mut v1: bool = &*v0 == "--dry-run";
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method4() -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure1()
}
fn method5(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<UH0>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = v0(v2);
                if v4 {
                    return true;
                } else {
                    let v5: Rc<dyn Fn(Rc<str>) -> bool> = method4();
                    (v0, v1) = (v5, v3);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn flag_in_6(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                match Rc::unwrap_or_clone(v3) {
                    UH0::UH0_Cons(v4, v5) => {
                        let mut v4: Rc<str> = v4;
                        let v5: Rc<UH0> = v5;
                        let v6: bool = &*v2 == &*v1;
                        if v6 {
                            return v4;
                        } else {
                            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5));
                            v0 = v7;
                            continue;
                        }
                    }
                    _ => {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        return v10;
                    }
                }
            }
            _ => {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v12;
            }
        }
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method9(v16, v17)
    }
}
fn op_of_line_13(mut v0: Rc<str>) -> US3 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v5: bool = &*v4 == "";
    let mut v9: bool = if v5 {
        true
    } else {
        let v6: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v6.starts_with(&*v7);
        v8
    };
    if v9 {
        US3::US3_Skipped
    } else {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<UH0> = split_list_8(v11, v3.clone());
        match Rc::unwrap_or_clone(v12) {
            UH0::UH0_Cons(v13, v14) => {
                let v13: Rc<str> = v13;
                let v14: Rc<UH0> = v14;
                match Rc::unwrap_or_clone(v14) {
                    UH0::UH0_Cons(v15, v16) => {
                        let v15: Rc<str> = v15;
                        let v16: Rc<UH0> = v16;
                        match Rc::unwrap_or_clone(v16) {
                            UH0::UH0_Cons(v17, v18) => {
                                let v17: Rc<str> = v17;
                                let mut v18: Rc<UH0> = v18;
                                match Rc::unwrap_or_clone(v18) {
                                    UH0::UH0_Cons(v36, v37) => {
                                        let v36: Rc<str> = v36;
                                        let v37: Rc<UH0> = v37;
                                        match Rc::unwrap_or_clone(v37) {
                                            UH0::UH0_Cons(v47, v48) => {
                                                let v47: Rc<str> = v47;
                                                let v48: Rc<UH0> = v48;
                                                match Rc::unwrap_or_clone(v48) {
                                                    UH0::UH0_Nil => {
                                                        let v49: bool = &*v13 == "set";
                                                        if v49 {
                                                            let v50: US2 = US2::US2_SetOp(v15, v17, v36, v47);
                                                            US3::US3_Op(v50)
                                                        } else {
                                                            US3::US3_LineRefused(v3)
                                                        }
                                                    }
                                                    _ => {
                                                        US3::US3_LineRefused(v3)
                                                    }
                                                }
                                            }
                                            UH0::UH0_Nil => {
                                                let v38: bool = &*v13 == "set";
                                                if v38 {
                                                    let v39: US2 = US2::US2_SetOp(v15, v17, v36, v2);
                                                    US3::US3_Op(v39)
                                                } else {
                                                    let v41: bool = &*v13 == "edit";
                                                    if v41 {
                                                        let v42: US2 = US2::US2_EditOp(v15, v17, v36);
                                                        US3::US3_Op(v42)
                                                    } else {
                                                        US3::US3_LineRefused(v3)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    UH0::UH0_Nil => {
                                        let v19: bool = &*v13 == "note";
                                        if v19 {
                                            let v20: US2 = US2::US2_NoteOp(v15, v17);
                                            US3::US3_Op(v20)
                                        } else {
                                            let v22: bool = &*v13 == "wait";
                                            if v22 {
                                                let v23: US2 = US2::US2_WaitOp(v15, v17);
                                                US3::US3_Op(v23)
                                            } else {
                                                let v25: bool = &*v13 == "ask";
                                                if v25 {
                                                    let v26: US2 = US2::US2_AskOp(v15, v17);
                                                    US3::US3_Op(v26)
                                                } else {
                                                    let v28: bool = &*v13 == "after";
                                                    if v28 {
                                                        let mut v29: US2 = US2::US2_AfterOp(v15, v17);
                                                        US3::US3_Op(v29)
                                                    } else {
                                                        US3::US3_LineRefused(v3)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                US3::US3_LineRefused(v3)
                            }
                        }
                    }
                    _ => {
                        US3::US3_LineRefused(v3)
                    }
                }
            }
            _ => {
                US3::US3_LineRefused(v3)
            }
        }
    }
}
fn method15(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_14(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn line_read_12(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: i32, mut v3: Rc<str>) -> (Rc<UH1>, Rc<UH0>, i32) {
    let mut v4: i32 = v2.wrapping_add(1i32);
    let v5: US3 = op_of_line_13(v3);
    let (mut v20, mut v21, mut v22): (Rc<UH1>, Rc<UH0>, i32) = match &v5 {
        US3::US3_Op(v6) => {
            let v6: US2 = v6.clone();
            let v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v6, v0));
            (v7, v1, v4)
        }
        _ => {
            let (mut v17, mut v18, mut v19): (Rc<UH1>, Rc<UH0>, i32) = match &v5 {
                US3::US3_Skipped => {
                    (v0, v1, v4)
                }
                _ => {
                    let (mut v14, mut v15, mut v16): (Rc<UH1>, Rc<UH0>, i32) = match v5 {
                        US3::US3_LineRefused(v8) => {
                            let v8: Rc<str> = v8;
                            let mut v9: Rc<str> = format_real_14(v2);
                            let mut v12: Rc<str> = string_concat(&["line ", &*v9, ": ", &*v8]);
                            let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v1));
                            (v0, v13, v4)
                        }
                        _ => unreachable!(),
                    };
                    (v14, v15, v16)
                }
            };
            (v17, v18, v19)
        }
    };
    (v20, v21, v22)
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: Rc<UH0>, mut v3: i32) -> (Rc<UH1>, Rc<UH0>, i32) {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let (mut v6, mut v7, mut v8): (Rc<UH1>, Rc<UH0>, i32) = line_read_12(v1, v2, v3, v4);
                (v0, v1, v2, v3) = (v5, v6, v7, v8);
                continue;
            }
            _ => {
                let (mut v12, mut v13, mut v14): (Rc<UH1>, Rc<UH0>, i32) = match Rc::unwrap_or_clone(v0) {
                    UH0::UH0_Nil => {
                        (v1, v2, v3)
                    }
                    _ => unreachable!(),
                };
                return (v12, v13, v14);
            }
        }
    }
}
fn method16(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: US2 = v2;
                let v3: Rc<UH1> = v3;
                let mut v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn plan_of_7(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = split_list_8(v1, v0);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v5: i32 = 1i32;
    let (mut v6, mut v7, mut v8): (Rc<UH1>, Rc<UH0>, i32) = method11(v2, v3, v4, v5);
    match &*v7 {
        UH0::UH0_Nil => {
            let mut v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v10: Rc<UH1> = method16(v6, v9);
            US1::US1_Plan(v10)
        }
        _ => {
            let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v13: Rc<UH0> = method9(v7, v12);
            US1::US1_PlanRefused(v13)
        }
    }
}
fn op_word_18(mut v0: US2) -> Rc<str> {
    match v0 {
        US2::US2_AfterOp(v19, v20) => {
            let v19: Rc<str> = v19;
            let v20: Rc<str> = v20;
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
            v21
        }
        US2::US2_AskOp(v16, v17) => {
            let v16: Rc<str> = v16;
            let v17: Rc<str> = v17;
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ask"); } LIT.with(|lit| lit.clone()) };
            v18
        }
        US2::US2_EditOp(v12, v13, v14) => {
            let mut v12: Rc<str> = v12;
            let v13: Rc<str> = v13;
            let v14: Rc<str> = v14;
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            v15
        }
        US2::US2_NoteOp(v1, v2) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US2::US2_SetOp(v4, v5, v6, v7) => {
            let mut v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: Rc<str> = v7;
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US2::US2_WaitOp(v9, v10) => {
            let mut v9: Rc<str> = v9;
            let v10: Rc<str> = v10;
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
            v11
        }
    }
}
fn op_id_19(mut v0: US2) -> Rc<str> {
    match v0 {
        US2::US2_AfterOp(v14, v15) => {
            let v14: Rc<str> = v14;
            let v15: Rc<str> = v15;
            v14
        }
        US2::US2_AskOp(v12, v13) => {
            let mut v12: Rc<str> = v12;
            let v13: Rc<str> = v13;
            v12
        }
        US2::US2_EditOp(v9, v10, v11) => {
            let mut v9: Rc<str> = v9;
            let v10: Rc<str> = v10;
            let mut v11: Rc<str> = v11;
            v9
        }
        US2::US2_NoteOp(v1, v2) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            v1
        }
        US2::US2_SetOp(v3, v4, v5, v6) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            v3
        }
        US2::US2_WaitOp(v7, v8) => {
            let v7: Rc<str> = v7;
            let v8: Rc<str> = v8;
            v7
        }
    }
}
fn method21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method21(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn argv_of_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US2) -> Rc<UH0> {
    let v3: bool = &*v1 == "";
    let mut v9: Rc<UH0> = if v3 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v6));
        Rc::new(UH0::UH0_Cons(v5, v7))
    };
    match v2 {
        US2::US2_AfterOp(v73, v74) => {
            let v73: Rc<str> = v73;
            let v74: Rc<str> = v74;
            let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after"); } LIT.with(|lit| lit.clone()) };
            let v78: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v79: Rc<UH0> = Rc::new(UH0::UH0_Cons(v74, v78));
            let v80: Rc<UH0> = Rc::new(UH0::UH0_Cons(v77, v79));
            let v81: Rc<UH0> = Rc::new(UH0::UH0_Cons(v73, v80));
            let v82: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v81));
            let v83: Rc<UH0> = Rc::new(UH0::UH0_Cons(v76, v82));
            Rc::new(UH0::UH0_Cons(v75, v83))
        }
        US2::US2_AskOp(v63, v64) => {
            let v63: Rc<str> = v63;
            let v64: Rc<str> = v64;
            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ask"); } LIT.with(|lit| lit.clone()) };
            let v67: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v68: Rc<UH0> = Rc::new(UH0::UH0_Cons(v64, v67));
            let v69: Rc<UH0> = Rc::new(UH0::UH0_Cons(v63, v68));
            let v70: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v69));
            let v71: Rc<UH0> = Rc::new(UH0::UH0_Cons(v66, v70));
            Rc::new(UH0::UH0_Cons(v65, v71))
        }
        US2::US2_EditOp(v51, v52, v53) => {
            let v51: Rc<str> = v51;
            let v52: Rc<str> = v52;
            let v53: Rc<str> = v53;
            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            let v56: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v57: Rc<UH0> = Rc::new(UH0::UH0_Cons(v53, v56));
            let v58: Rc<UH0> = Rc::new(UH0::UH0_Cons(v52, v57));
            let v59: Rc<UH0> = Rc::new(UH0::UH0_Cons(v51, v58));
            let v60: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v59));
            let v61: Rc<UH0> = Rc::new(UH0::UH0_Cons(v55, v60));
            Rc::new(UH0::UH0_Cons(v54, v61))
        }
        US2::US2_NoteOp(v10, v11) => {
            let v10: Rc<str> = v10;
            let mut v11: Rc<str> = v11;
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v9));
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v14));
            let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v15));
            let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v16));
            Rc::new(UH0::UH0_Cons(v12, v17))
        }
        US2::US2_SetOp(v19, v20, v21, v22) => {
            let v19: Rc<str> = v19;
            let v20: Rc<str> = v20;
            let v21: Rc<str> = v21;
            let v22: Rc<str> = v22;
            let v23: bool = &*v22 == "";
            let mut v29: Rc<UH0> = if v23 {
                { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
            } else {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--expect"); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v27: Rc<UH0> = Rc::new(UH0::UH0_Cons(v22, v26));
                Rc::new(UH0::UH0_Cons(v25, v27))
            };
            let v30: Rc<UH0> = method21(v29, v9);
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<UH0> = Rc::new(UH0::UH0_Cons(v21, v30));
            let mut v34: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20, v33));
            let v35: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19, v34));
            let v36: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v35));
            let v37: Rc<UH0> = Rc::new(UH0::UH0_Cons(v32, v36));
            Rc::new(UH0::UH0_Cons(v31, v37))
        }
        US2::US2_WaitOp(v39, v40) => {
            let v39: Rc<str> = v39;
            let v40: Rc<str> = v40;
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--wait"); } LIT.with(|lit| lit.clone()) };
            let v44: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v45: Rc<UH0> = Rc::new(UH0::UH0_Cons(v40, v44));
            let v46: Rc<UH0> = Rc::new(UH0::UH0_Cons(v43, v45));
            let v47: Rc<UH0> = Rc::new(UH0::UH0_Cons(v39, v46));
            let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v47));
            let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v42, v48));
            Rc::new(UH0::UH0_Cons(v41, v49))
        }
    }
}
fn method22(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
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
fn method24(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure2() -> Rc<dyn Fn((Rc<str>)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US0> = Rc::new(move |mut v0: (Rc<str>)| -> US0 {
        let mut v1: Rc<str> = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn current_executable_23() -> Rc<str> {
    let mut v0: Option<std::path::PathBuf> = std::env::current_exe().ok();
    let mut v2: bool = true; let _optionm_map_ = v0.map(|x| { //;
    let mut v4: std::path::PathBuf = x;
    let v5: Rc<str> = Rc::<str>::from(v4.to_string_lossy().as_ref());
    let v7: bool = true; v5 });
    let mut v9: Option<Rc<str>> = _optionm_map_;
    let v10: Option<Rc<str>> = method24(v9);
    let mut v11: Rc<dyn Fn((Rc<str>)) -> US0> = closure2();
    let mut v12: Option<US0> = v10.map(|x| v11(x));
    let v14: US0 = US0::US0_None;
    let v15: US0 = v14.clone();
    let v16: US0 = v12.unwrap_or(v15);
    match v16 {
        US0::US0_None => {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v18
        }
        US0::US0_Some(v17) => {
            let v17: Rc<str> = v17;
            v17
        }
    }
}
fn method25(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v2 == "";
                let v6: Rc<UH0> = if v4 {
                    v1
                } else {
                    Rc::new(UH0::UH0_Cons(v2, v1))
                };
                (v0, v1) = (v3, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method26(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                drop(v0.borrow_mut().arg(&*v2));
                v1 = v3;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method28(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v2 == "";
                let v5: Rc<str> = if v4 {
                    v1
                } else {
                    v2
                };
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method29(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method29(v3, v1);
            let v5: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let v6: bool = &*v5 != "";
            if v6 {
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
fn last_line_27(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_8(v1, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method29(v5, v6);
    method28(v7, v3)
}
fn ops_run_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool, mut v3: Rc<UH1>, mut v4: i32, mut v5: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH1::UH1_Cons(v6, v7) => {
                let v6: US2 = v6;
                let v7: Rc<UH1> = v7;
                let v8: Rc<str> = format_real_14(v4);
                let mut v11: Rc<str> = op_word_18(v6.clone());
                let v14: Rc<str> = op_id_19(v6.clone());
                let v15: Rc<str> = string_concat(&["line ", &*v8, " ", &*v11, " ", &*v14]);
                let v16: Rc<UH0> = argv_of_20(v0.clone(), v1.clone(), v6);
                if v2 {
                    let mut v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v19: i32 = 0i32;
                    let v20: i32 = method22(v18.clone(), v16, v19);
                    let v21: Rc<Vec<Rc<str>>> = Rc::new(v18.borrow().clone());
                    let v22: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v21).as_ref().clone()));
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = Rc::<str>::from((*v22.borrow()).join(&*v23));
                    let v25: Rc<str> = string_concat(&[&*v15, " dry-run: rig ", &*v24]);
                    println!("{}", v25);
                    let v26: i32 = v4.wrapping_add(1i32);
                    (v3, v4) = (v7, v26);
                    continue;
                } else {
                    let v28: Rc<str> = current_executable_23();
                    let mut v29: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v28)));
                    let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v31: Rc<UH0> = method25(v16, v30);
                    let v32: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v33: Rc<UH0> = method9(v31, v32);
                    method26(v29.clone(), v33);
                    let mut v34: Rc<std::io::Result<std::process::Output>> = Rc::new(v29.borrow_mut().output());
                    let v35: bool = v34.is_ok();
                    let (mut v41, mut v42, mut v43): (bool, i32, Rc<str>) = if v35 {
                        let v36: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v34).as_ref().unwrap().stderr).as_ref());
                        let v37: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v34).as_ref().unwrap().stdout).as_ref());
                        let v38: Rc<str> = string_concat(&[&*v37, &*v36]);
                        let v39: i32 = (&*v34).as_ref().unwrap().status.code().unwrap_or(-2);
                        (true, v39, v38)
                    } else {
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        (false, -1i32, v40)
                    };
                    let v44: i32 = if v41 {
                        v42
                    } else {
                        127i32
                    };
                    let v46: Rc<str> = format_real_14(v44);
                    let v49: Rc<str> = last_line_27(v43);
                    let v50: Rc<str> = string_concat(&[&*v15, " exit=", &*v46, ": ", &*v49]);
                    println!("{}", v50);
                    let v51: i32 = v4.wrapping_add(1i32);
                    let v52: bool = v44 == 0i32;
                    let v54: i32 = if v52 {
                        v5
                    } else {
                        let v53: i32 = v5.wrapping_add(1i32);
                        v53
                    };
                    (v3, v4, v5) = (v7, v51, v54);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return v5;
            }
        }
    }
}
fn method30(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: US2 = v2;
                let v3: Rc<UH1> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn lint_tail_31(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = current_executable_23();
    let mut v2: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v1)));
    let v3: bool = &*v0 == "";
    let v15: Rc<UH0> = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lint"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v6));
        Rc::new(UH0::UH0_Cons(v4, v7))
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lint"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v11));
        let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v12));
        Rc::new(UH0::UH0_Cons(v0, v13))
    };
    let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH0> = method9(v15, v16);
    method26(v2.clone(), v17);
    let mut v18: Rc<std::io::Result<std::process::Output>> = Rc::new(v2.borrow_mut().output());
    let v19: bool = v18.is_ok();
    let (mut v25, mut v26, mut v27): (bool, i32, Rc<str>) = if v19 {
        let v20: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v18).as_ref().unwrap().stderr).as_ref());
        let v21: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v18).as_ref().unwrap().stdout).as_ref());
        let v22: Rc<str> = string_concat(&[&*v21, &*v20]);
        let v23: i32 = (&*v18).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v23, v22)
    } else {
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v24)
    };
    last_line_27(v27)
}
fn method32(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<UH0> = v2;
                let v3: Rc<str> = string_concat(&["rig error: agile batch: ", &*v1]);
                eprintln!("{}", v3);
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
fn batch_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_1(v0);
    let mut v2: Rc<dyn Fn(Rc<str>) -> bool> = method4();
    let v3: bool = method5(v2, v1.clone());
    match &*v1 {
        UH0::UH0_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4.clone();
            let v5: Rc<UH0> = v5.clone();
            match Rc::unwrap_or_clone(v5) {
                UH0::UH0_Cons(v6, v7) => {
                    let v6: Rc<str> = v6;
                    let v7: Rc<UH0> = v7;
                    match Rc::unwrap_or_clone(v7) {
                        UH0::UH0_Cons(v8, v9) => {
                            let v8: Rc<str> = v8;
                            let mut v9: Rc<UH0> = v9;
                            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--file"); } LIT.with(|lit| lit.clone()) };
                            let mut v11: Rc<str> = flag_in_6(v1.clone(), v10.clone());
                            let mut v12: bool = &*v11 != "";
                            if v12 {
                                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
                                let v14: Rc<str> = flag_in_6(v1.clone(), v13);
                                let v15: Rc<str> = flag_in_6(v1.clone(), v10.clone());
                                let v16: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v15).unwrap_or_default().as_str());
                                let v17: US1 = plan_of_7(v16);
                                match v17 {
                                    US1::US1_Plan(v20) => {
                                        let v20: Rc<UH1> = v20;
                                        match &*v20 {
                                            UH1::UH1_Nil => {
                                                let v21: Rc<str> = flag_in_6(v1, v10);
                                                let v22: Rc<str> = string_concat(&["rig error: agile batch: no ops in ", &*v21]);
                                                eprintln!("{}", v22);
                                                2i32
                                            }
                                            _ => {
                                                let v23: i32 = 1i32;
                                                let v24: i32 = 0i32;
                                                let v25: i32 = ops_run_17(v8.clone(), v14, v3, v20.clone(), v23, v24);
                                                let v26: i32 = 0i32;
                                                let v27: i32 = method30(v20, v26);
                                                let v31: Rc<str> = if v3 {
                                                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    v28
                                                } else {
                                                    let mut v29: Rc<str> = lint_tail_31(v8);
                                                    let v30: Rc<str> = string_concat(&[" lint: ", &*v29]);
                                                    v30
                                                };
                                                let v32: Rc<str> = format_real_14(v27);
                                                let v35: Rc<str> = format_real_14(v25);
                                                let v36: Rc<str> = string_concat(&[" ops=", &*v32, " failed=", &*v35]);
                                                let v37: bool = v25 == 0i32;
                                                let v40: Rc<str> = if v37 {
                                                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                                                    v38
                                                } else {
                                                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
                                                    v39
                                                };
                                                let v43: Rc<str> = if v3 {
                                                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
                                                    v41
                                                } else {
                                                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no"); } LIT.with(|lit| lit.clone()) };
                                                    v42
                                                };
                                                let v48: Rc<str> = string_concat(&["rig agile batch ", &*v40, &*v36, " dry_run=", &*v43, &*v31]);
                                                println!("{}", v48);
                                                if v37 {
                                                    0i32
                                                } else {
                                                    1i32
                                                }
                                            }
                                        }
                                    }
                                    US1::US1_PlanRefused(v18) => {
                                        let mut v18: Rc<UH0> = v18;
                                        method32(v18);
                                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile batch <root> --file <plan.tsv> [--as <lane>] [--dry-run] (one op per line, fields split by TAB: note <id> <text> | set <id> <progress|-> <status> [<expected status>] | wait <id> <kind:value> | edit <id> <field> <value> | ask <id> <scope; done when ...> | after <id> <other id>)"); } LIT.with(|lit| lit.clone()) };
                                        eprintln!("{}", v19);
                                        2i32
                                    }
                                }
                            } else {
                                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile batch <root> --file <plan.tsv> [--as <lane>] [--dry-run] (one op per line, fields split by TAB: note <id> <text> | set <id> <progress|-> <status> [<expected status>] | wait <id> <kind:value> | edit <id> <field> <value> | ask <id> <scope; done when ...> | after <id> <other id>)"); } LIT.with(|lit| lit.clone()) };
                                eprintln!("{}", v53);
                                2i32
                            }
                        }
                        _ => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile batch <root> --file <plan.tsv> [--as <lane>] [--dry-run] (one op per line, fields split by TAB: note <id> <text> | set <id> <progress|-> <status> [<expected status>] | wait <id> <kind:value> | edit <id> <field> <value> | ask <id> <scope; done when ...> | after <id> <other id>)"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v55);
                            2i32
                        }
                    }
                }
                _ => {
                    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile batch <root> --file <plan.tsv> [--as <lane>] [--dry-run] (one op per line, fields split by TAB: note <id> <text> | set <id> <progress|-> <status> [<expected status>] | wait <id> <kind:value> | edit <id> <field> <value> | ask <id> <scope; done when ...> | after <id> <other id>)"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v57);
                    2i32
                }
            }
        }
        _ => {
            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile batch <root> --file <plan.tsv> [--as <lane>] [--dry-run] (one op per line, fields split by TAB: note <id> <text> | set <id> <progress|-> <status> [<expected status>] | wait <id> <kind:value> | edit <id> <field> <value> | ask <id> <scope; done when ...> | after <id> <other id>)"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v59);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        batch_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_batch_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
