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
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(i32, Rc<str>, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<UH3>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<UH0>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
fn method2(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH1> = method2(v0.clone(), v4, v2);
            let v7: Rc<str> = string_concat(&[&*v0, "/", &*v3]);
            let v8: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v7).unwrap_or_default().as_str());
            Rc::new(UH1::UH1_Cons(v3, v8, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method3(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<UH1> = v4;
            let v5: Rc<UH0> = method3(v4, v1);
            Rc::new(UH0::UH0_Cons(v3, v5))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method4(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn method6(mut v0: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, i32)>>, mut v1: i32, mut v2: Rc<UH3>) -> Rc<UH3> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7, mut v8, mut v9): (Rc<str>, Rc<str>, Rc<str>, i32, i32) = (v0)[v4].clone();
            let mut v10: i32 = v1.wrapping_sub(1i32);
            let v11: Rc<UH3> = Rc::new(UH3::UH3_Cons(v5, v6, v7, v8, v9, v2));
            (v1, v2) = (v10, v11);
            continue;
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method12(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method12(v7, &v9) {
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
        method11(v16, v17)
    }
}
fn last_char_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = &*v0 == "";
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: i32 = v3.wrapping_sub(1i32);
        let v5: usize = ((v4) as usize);
        let v6: i32 = v4.wrapping_add(1i32);
        let v7: usize = ((v6) as usize);
        let v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
        v8
    }
}
fn word_char_15(mut v0: Rc<str>) -> bool {
    let mut v1: bool = &*v0 != "";
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_'"); } LIT.with(|lit| lit.clone()) };
        let v3: bool = v2.contains(&*v0);
        v3
    } else {
        false
    }
}
fn first_char_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = &*v0 == "";
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: usize = ((0i32) as usize);
        let v4: usize = ((1i32) as usize);
        let v5: Rc<str> = Rc::<str>::from(&(v0)[v3..v4]);
        v5
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: i32, mut v2: Rc<str>) -> (i32, Rc<str>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: Rc<str> = last_char_14(v2);
                let v6: bool = word_char_15(v5);
                let mut v9: bool = if v6 {
                    true
                } else {
                    let v7: Rc<str> = first_char_16(v3.clone());
                    word_char_15(v7)
                };
                let v11: i32 = if v9 {
                    v1
                } else {
                    let mut v10: i32 = v1.wrapping_add(1i32);
                    v10
                };
                (v0, v1, v2) = (v4, v11, v3);
                continue;
            }
            UH0::UH0_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn uses_in_9(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: bool = v0.contains(&*v1);
    let v3: bool = v2 == false;
    if v3 {
        0i32
    } else {
        let v4: Rc<UH0> = split_list_10(v1, v0);
        match Rc::unwrap_or_clone(v4) {
            UH0::UH0_Cons(v5, v6) => {
                let v5: Rc<str> = v5;
                let v6: Rc<UH0> = v6;
                let v7: i32 = 0i32;
                let (mut v8, mut v9): (i32, Rc<str>) = method13(v6, v7, v5);
                v8
            }
            UH0::UH0_Nil => {
                0i32
            }
        }
    }
}
fn method18(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_17(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method18(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn scored_of_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: i32, mut v6: i32) -> (i32, Rc<str>) {
    let v7: i32 = uses_in_9(v1, v2.clone());
    let v8: i32 = v7.wrapping_sub(1i32);
    let mut v9: bool = v8 <= 1i32;
    let v12: i32 = if v9 {
        2i32
    } else {
        let mut v10: bool = v8 >= 3i32;
        if v10 {
            -1i32
        } else {
            0i32
        }
    };
    let v13: i32 = v6.wrapping_add(v12);
    let v15: Rc<str> = format_real_17(v5);
    let v17: Rc<str> = format_real_17(v13);
    let v25: Rc<str> = format_real_17(v8);
    let mut v28: Rc<str> = string_concat(&[&*v17, " ", &*v0, ":", &*v15, " ", &*v2, " ", &*v3, " callers=", &*v25, " -> ", &*v4]);
    (v13, v28)
}
fn inserted_19(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<UH2>) -> Rc<UH2> {
    match &*v2 {
        UH2::UH2_Cons(v5, v6, v7) => {
            let v5: i32 = *v5;
            let v6: Rc<str> = v6.clone();
            let v7: Rc<UH2> = v7.clone();
            let v8: bool = v0 > v5;
            if v8 {
                Rc::new(UH2::UH2_Cons(v0, v1, v2))
            } else {
                let mut v10: Rc<UH2> = inserted_19(v0, v1, v7);
                Rc::new(UH2::UH2_Cons(v5, v6, v10))
            }
        }
        UH2::UH2_Nil => {
            let v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_Cons(v0, v1, v3))
        }
    }
}
fn method7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH3>, mut v3: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH3::UH3_Cons(v4, v5, v6, v7, v8, v9) => {
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<str> = v6;
                let v7: i32 = v7;
                let v8: i32 = v8;
                let mut v9: Rc<UH3> = v9;
                let (mut v10, mut v11): (i32, Rc<str>) = scored_of_8(v1.clone(), v0.clone(), v4, v5, v6, v7, v8);
                let v12: Rc<UH2> = inserted_19(v10, v11, v3);
                (v2, v3) = (v9, v12);
                continue;
            }
            UH3::UH3_Nil => {
                return v3;
            }
        }
    }
}
fn method5(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, i32)>>> = rig_wrapper_score_model::rig_wrapper_rows(&v4);
                let v7: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, i32)>> = Rc::new((v6).borrow().clone());
                let v8: usize = (v7).len();
                let mut v9: i32 = ((v8) as i32);
                let mut v10: i32 = v9.wrapping_sub(1i32);
                let v11: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                let v12: Rc<UH3> = method6(v7, v10, v11);
                let v13: Rc<UH2> = method7(v0.clone(), v3, v12, v2);
                (v1, v2) = (v5, v13);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn method20(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3, v4) => {
            let mut v2: i32 = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<UH2> = v4;
            let v5: Rc<UH2> = method20(v4, v1);
            let v6: bool = v2 >= 5i32;
            if v6 {
                Rc::new(UH2::UH2_Cons(v2, v3, v5))
            } else {
                v5
            }
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn taken_21(mut v0: i32, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v1) {
        UH2::UH2_Cons(v2, v3, v4) => {
            let mut v2: i32 = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<UH2> = v4;
            let v5: bool = v0 > 0i32;
            if v5 {
                let v6: i32 = v0.wrapping_sub(1i32);
                let v7: Rc<UH2> = taken_21(v6, v4);
                Rc::new(UH2::UH2_Cons(v2, v3, v7))
            } else {
                { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
            }
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method22(mut v0: Rc<UH2>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v1, v2, v3) => {
                let mut v1: i32 = v1;
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                println!("{}", v2);
                v0 = v3;
                continue;
            }
            UH2::UH2_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method23(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method24(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4) => {
                let mut v2: i32 = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<UH2> = v4;
                let v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4, v5);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn wrappers_listed_1(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v0.replace(&*v3, &*v4));
    let v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH1> = method2(v5, v1.clone(), v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method3(v7.clone(), v8);
    let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v11: i32 = 0i32;
    let v12: i32 = method4(v10.clone(), v9, v11);
    let v13: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
    let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v13).as_ref().clone()));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from((*v14.borrow()).join(&*v15));
    let v17: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v18: Rc<UH2> = method5(v16, v7, v17);
    let v19: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v20: Rc<UH2> = method20(v18.clone(), v19);
    let v21: Rc<UH2> = taken_21(v2, v18.clone());
    method22(v21);
    let v22: i32 = 0i32;
    let v23: i32 = method23(v1, v22);
    let v24: Rc<str> = format_real_17(v23);
    let v27: i32 = 0i32;
    let mut v28: i32 = method24(v18, v27);
    let v29: Rc<str> = format_real_17(v28);
    let v32: i32 = 0i32;
    let v33: i32 = method24(v20, v32);
    let v34: Rc<str> = format_real_17(v33);
    let v35: Rc<str> = string_concat(&["rig standards wrappers files=", &*v24, " wrappers=", &*v29, " high=", &*v34]);
    println!("{}", v35);
    0i32
}
fn command_line_argument_count_26() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_27(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_25(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_26();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_27(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let mut v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_25(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn method29(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
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
fn method30(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method30(v3, v1);
            let v5: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method31(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method31(v3, v1);
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
fn listed_files_28(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v1)));
    let v3: bool = &*v0 == "";
    let v11: Rc<UH0> = if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v4, v5))
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v8));
        Rc::new(UH0::UH0_Cons(v0, v9))
    };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ls-files"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--cached"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--others"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--exclude-standard"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*.spi"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*.spir"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v19));
    let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17, v20));
    let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v21));
    let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v22));
    let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v14, v23));
    let v25: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v24));
    let v26: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v25));
    let v27: Rc<UH0> = method11(v11, v26);
    method29(v2.clone(), v27);
    let mut v28: Rc<std::io::Result<std::process::Output>> = Rc::new(v2.borrow_mut().output());
    let v29: bool = v28.is_ok();
    let (mut v34, mut v35, mut v36): (bool, i32, Rc<str>) = if v29 {
        let v30: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v28).as_ref().unwrap().stdout).as_ref());
        let v31: Rc<str> = string_concat(&[&*v30, ""]);
        let v32: i32 = (&*v28).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v32, v31)
    } else {
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v33)
    };
    let v38: bool = if v34 {
        let v37: bool = v35 == 0i32;
        v37
    } else {
        false
    };
    if v38 {
        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v42: Rc<str> = Rc::<str>::from(v36.replace(&*v40, &*v41));
        let v43: Rc<UH0> = split_list_10(v39, v42);
        let v44: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v45: Rc<UH0> = method30(v43, v44);
        let v46: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v47: Rc<UH0> = method31(v45, v46);
        US1::US1_Some(v47)
    } else {
        US1::US1_None
    }
}
fn method33(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure1() -> Rc<dyn Fn((i32)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US2> = Rc::new(move |mut v0: (i32)| -> US2 {
        let mut v1: i32 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn top_flag_32(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let v4: Rc<str> = v4.clone();
                        let v5: Rc<UH0> = v5.clone();
                        let v6: bool = &*v2 == "--top";
                        if v6 {
                            let v7: Option<i32> = (v4).trim().parse::<i32>().ok();
                            let v8: Option<i32> = method33(v7);
                            let mut v9: Rc<dyn Fn((i32)) -> US2> = closure1();
                            let mut v10: Option<US2> = v8.map(|x| v9(x));
                            let v12: US2 = US2::US2_None;
                            let v13: US2 = v12.clone();
                            let v14: US2 = v10.unwrap_or(v13);
                            match v14 {
                                US2::US2_Some(v15) => {
                                    let v15: i32 = v15;
                                    let v16: bool = v15 > 0i32;
                                    if v16 {
                                        (v0, v1) = (v5, v15);
                                        continue;
                                    } else {
                                        v0 = v5;
                                        continue;
                                    }
                                }
                                _ => {
                                    v0 = v5;
                                    continue;
                                }
                            }
                        } else {
                            v0 = v3;
                            continue;
                        }
                    }
                    _ => {
                        v0 = v3;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn wrappers_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_25(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            let v8: US1 = listed_files_28(v6.clone());
                            match v8 {
                                US1::US1_None => {
                                    let v13: Rc<str> = string_concat(&["rig error: standards --wrappers: git could not list ", &*v6]);
                                    eprintln!("{}", v13);
                                    2i32
                                }
                                US1::US1_Some(v9) => {
                                    let mut v9: Rc<UH0> = v9;
                                    let mut v10: i32 = 20i32;
                                    let v11: i32 = top_flag_32(v7, v10);
                                    wrappers_listed_1(v6, v9, v11)
                                }
                            }
                        }
                        _ => {
                            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: standards --wrappers <repo> [--top n]"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v16);
                            2i32
                        }
                    }
                }
                _ => {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: standards --wrappers <repo> [--top n]"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v18);
                    2i32
                }
            }
        }
        _ => {
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: standards --wrappers <repo> [--top n]"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v20);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        wrappers_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_standards_wrappers_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
