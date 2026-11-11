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
    UH0_Cons(i32, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum US0 {
    US0_Some(i32),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_ConsNilSpelled(i32),
    US1_LineTooWide(i32, i32),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(US1, Rc<UH3>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<UH3>, Rc<UH2>),
}
pub struct Mut1 { pub l0: Rc<str> }
fn spiral_source_1(mut v0: Rc<str>) -> bool {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.ends_with(&*v1);
    if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spir"); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.ends_with(&*v3);
        v4
    }
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4) => {
                let v2: i32 = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v1));
                (v0, v1) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method6(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method7(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method7(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH1> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method6(v16, v17)
    }
}
fn lines_of_4(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    split_list_5(v1, v4)
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US0> {
    Rc::new(move |mut v2: Rc<UH1>| -> US0 {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US0::US0_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US0> = method9(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH1::UH1_Nil => {
                US0::US0_None
            }
        }
    })
}
fn method9(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US0> {
    closure1(v0, v1)
}
fn method8(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: i32, mut v3: Rc<UH0>) -> (i32, Rc<UH0>) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let v6: i32 = 0i32;
                let mut v7: Rc<dyn Fn(Rc<UH1>) -> US0> = method9(v4.clone(), v6);
                let v8: US0 = v7(v0.clone());
                let mut v9: bool = match v8 {
                    US0::US0_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v10: bool = v9 != true;
                let v12: Rc<UH0> = if v10 {
                    v3
                } else {
                    Rc::new(UH0::UH0_Cons(v2, v4, v3))
                };
                let v13: i32 = v2.wrapping_add(1i32);
                (v1, v2, v3) = (v5, v13, v12);
                continue;
            }
            UH1::UH1_Nil => {
                return (v2, v3);
            }
        }
    }
}
fn added_lines_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: Rc<UH1> = lines_of_4(v0);
    let v3: Rc<UH1> = lines_of_4(v1);
    let v4: i32 = 1i32;
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v6, mut v7): (i32, Rc<UH0>) = method8(v2, v3, v4, v5);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method3(v7, v8)
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        let v1: bool = &*v0 == "Cons";
        if v1 {
            true
        } else {
            let v2: bool = &*v0 == "Nil";
            v2
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method14() -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure2()
}
fn method13(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<UH1>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: bool = v0(v2);
                if v4 {
                    return true;
                } else {
                    let v5: Rc<dyn Fn(Rc<str>) -> bool> = method14();
                    (v0, v1) = (v5, v3);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return false;
            }
        }
    }
}
fn words_of_15(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v2));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v2));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v2));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v2));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v2));
    split_list_5(v2, v13)
}
fn spells_cons_nil_12(mut v0: Rc<str>) -> bool {
    let v1: Rc<UH1> = words_of_15(v0);
    let v2: Rc<dyn Fn(Rc<str>) -> bool> = method14();
    method13(v2, v1)
}
fn findings_of_11(mut v0: Rc<str>, mut v1: i32) -> Rc<UH3> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 > 120i32;
    let v8: Rc<UH3> = if v3 {
        let v4: US1 = US1::US1_LineTooWide(v1, v2);
        let v5: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH3::UH3_Cons(v4, v5))
    } else {
        { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) }
    };
    let mut v9: bool = spells_cons_nil_12(v0);
    if v9 {
        let v10: US1 = US1::US1_ConsNilSpelled(v1);
        Rc::new(UH3::UH3_Cons(v10, v8))
    } else {
        v8
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4) => {
            let v2: i32 = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH2> = method10(v4, v1);
            let v6: Rc<UH3> = findings_of_11(v3, v2);
            Rc::new(UH2::UH2_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method17(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_Cons(v2, v3) => {
            let v2: US1 = v2;
            let v3: Rc<UH3> = v3;
            let v4: Rc<UH3> = method17(v3, v1);
            Rc::new(UH3::UH3_Cons(v2, v4))
        }
        UH3::UH3_Nil => {
            v1
        }
    }
}
fn method16(mut v0: Rc<UH2>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let v2: Rc<UH3> = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH3> = method16(v3, v1);
            method17(v2, v4)
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method21(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_20(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method21(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn finding_text_19(mut v0: US1) -> Rc<str> {
    match v0 {
        US1::US1_ConsNilSpelled(v1) => {
            let v1: i32 = v1;
            let v2: Rc<str> = format_real_20(v1);
            let v4: Rc<str> = string_concat(&["line ", &*v2, " spells Cons/Nil: write :: and []"]);
            v4
        }
        US1::US1_LineTooWide(v5, v6) => {
            let v5: i32 = v5;
            let v6: i32 = v6;
            let mut v7: Rc<str> = format_real_20(v5);
            let v10: Rc<str> = format_real_20(v6);
            let v13: i32 = 120i32;
            let v14: Rc<str> = format_real_20(v13);
            let v15: Rc<str> = string_concat(&["line ", &*v7, " is ", &*v10, " columns, over ", &*v14]);
            v15
        }
    }
}
fn method18(mut v0: Rc<UH3>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_Cons(v2, v3) => {
            let v2: US1 = v2;
            let v3: Rc<UH3> = v3;
            let v4: Rc<UH1> = method18(v3, v1);
            let v5: Rc<str> = finding_text_19(v2);
            Rc::new(UH1::UH1_Cons(v5, v4))
        }
        UH3::UH3_Nil => {
            v1
        }
    }
}
fn method22(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn slop_problem_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: bool = spiral_source_1(v2);
    let v4: bool = v3 == false;
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let v6: Rc<UH0> = added_lines_2(v1, v0);
        let mut v7: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let v8: Rc<UH2> = method10(v6, v7);
        let mut v9: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let v10: Rc<UH3> = method16(v8, v9);
        let v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let v12: Rc<UH1> = method18(v10, v11);
        let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v14: i32 = 0i32;
        let v15: i32 = method22(v13.clone(), v12, v14);
        let v16: Rc<Vec<Rc<str>>> = Rc::new(v13.borrow().clone());
        let v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v16).as_ref().clone()));
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = Rc::<str>::from((*v17.borrow()).join(&*v18));
        v19
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> Rc<str> {
        slop_problem_0(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_do_slop_problem(v0: &str, v1: &str, v2: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub type SpiralShape_accbf6de6842 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
