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
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
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
    let mut v8: US0 = if v4 {
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
fn flag_value_5(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let v4: Rc<str> = v4.clone();
                        let v5: Rc<UH0> = v5.clone();
                        let v6: bool = &*v2 == &*v1;
                        if v6 {
                            return v4;
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
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v11;
            }
        }
    }
}
fn terms_of_4(mut v0: Rc<str>, mut v1: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = flag_value_5(v1.clone(), v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = flag_value_5(v1, v4);
    (v0, v3, v5)
}
fn method6(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let v6: i32 = v1.wrapping_sub(1i32);
            let mut v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method7(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH0> = method7(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method10(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method13(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method13(v7, &v9) {
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
        method12(v16, v17)
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: bool = &*v1 == "";
                let v5: Rc<str> = if v4 {
                    v2
                } else {
                    v1
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
fn method15(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method15(v0.clone(), v4, v2);
            let v6: bool = &*v3 != &*v0;
            if v6 {
                Rc::new(UH0::UH0_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method16(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Note (\""); } LIT.with(|lit| lit.clone()) };
                let v5: Option<usize> = v2.find(&*v4);
                let v6: Option<usize> = method10(v5);
                let mut v7: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v8: Option<i32> = v6.map(|x| v7(x));
                let v10: i32 = v8.unwrap_or(-1i32);
                let v11: bool = v10 < 0i32;
                let v33: Rc<UH0> = if v11 {
                    v1
                } else {
                    let v12: i32 = v10.wrapping_add(10i32);
                    let v13: i32 = (v2.clone().len() as i32);
                    let v14: i32 = v13.wrapping_sub(v10);
                    let v15: i32 = v14.wrapping_sub(10i32);
                    let v16: usize = ((v12) as usize);
                    let v17: i32 = v12.wrapping_add(v15);
                    let v18: usize = ((v17) as usize);
                    let v19: Rc<str> = Rc::<str>::from(&(v2)[v16..v18]);
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<UH0> = split_list_11(v20, v19);
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = method14(v21, v22);
                    let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v25: Rc<UH0> = method15(v23.clone(), v1, v24);
                    let v26: Rc<str> = Rc::<str>::from(v2.to_lowercase());
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waits on install: train"); } LIT.with(|lit| lit.clone()) };
                    let v28: bool = v26.contains(&*v27);
                    if v28 {
                        let v29: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v30: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v29));
                        method16(v25, v30)
                    } else {
                        v25
                    }
                };
                (v0, v1) = (v3, v33);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn install_waiting_8(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH0> = split_list_11(v1, v0);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method9(v2, v3)
}
fn ids_where_17(mut v0: &Rc<RefCell<Vec<(Rc<str>, bool)>>>, mut v1: bool, mut v2: i32) -> Rc<UH0> {
    let v3: bool = v2 < 0i32;
    if v3 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let (mut v5, mut v6): (Rc<str>, bool) = v0.borrow()[v2 as usize].clone();
        let mut v7: i32 = v2.wrapping_sub(1i32);
        let mut v8: Rc<UH0> = ids_where_17(v0, v1, v7);
        let mut v9: bool = v6 == v1;
        if v9 {
            let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v10));
            method16(v8, v11)
        } else {
            v8
        }
    }
}
fn method18(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method18(v0.clone(), v4, v2);
            let mut v7: Rc<str> = string_concat(&["Waits (\"", &*v3, "\","]);
            let mut v8: bool = v0.contains(&*v7);
            if v8 {
                Rc::new(UH0::UH0_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    Rc::new(move |mut v2: Rc<UH0>| -> US1 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US1::US1_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US1> = method20(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US1::US1_None
            }
        }
    })
}
fn method20(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    closure2(v0, v1)
}
fn method19(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method19(v0.clone(), v4, v2);
            let v6: i32 = 0i32;
            let mut v7: Rc<dyn Fn(Rc<UH0>) -> US1> = method20(v3.clone(), v6);
            let mut v8: US1 = v7(v0);
            let mut v9: bool = match v8 {
                US1::US1_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v10: bool = v9 != true;
            let v11: bool = v10 == false;
            if v11 {
                Rc::new(UH0::UH0_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn taken_21(mut v0: i32, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: bool = v0 > 0i32;
            if v4 {
                let v5: i32 = v0.wrapping_sub(1i32);
                let v6: Rc<UH0> = taken_21(v5, v3);
                Rc::new(UH0::UH0_Cons(v2, v6))
            } else {
                { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
            }
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method22(v0.clone(), v4, v2);
            let v6: i32 = 0i32;
            let mut v7: Rc<dyn Fn(Rc<UH0>) -> US1> = method20(v3.clone(), v6);
            let mut v8: US1 = v7(v0);
            let mut v9: bool = match v8 {
                US1::US1_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v10: bool = v9 != true;
            if v10 {
                Rc::new(UH0::UH0_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method24(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn method26(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_25(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method26(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method28(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn ready_line_27(mut v0: Rc<str>, mut v1: &Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>) -> Rc<str> {
    let v4: bool = match &*v3 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v10: i32 = 0i32;
        let v11: i32 = method28(v9.clone(), v3, v10);
        let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
        let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v14));
        let v20: Rc<str> = string_concat(&["Your team's top ready light items: ", &*v15, ". Claim one explicitly (kit agile claim ", &*v0, " <ID> --as ", &*v2, ") and start it now; while a run or train you started is pending, work another of these instead of stopping."]);
        v20
    }
}
fn parked_line_29(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: &Rc<str>, mut v3: Rc<UH0>) -> Rc<str> {
    let v4: bool = match &*v3 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let mut v9: i32 = 0i32;
        let v10: i32 = method28(v8.clone(), v3, v9);
        let v11: Rc<Vec<Rc<str>>> = Rc::new(v8.borrow().clone());
        let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v11).as_ref().clone()));
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = Rc::<str>::from((*v12.borrow()).join(&*v13));
        let v16: Rc<str> = string_concat(&["Paused P0/P1 items for team ", &*v1, " (re-decide each: resume as Planned, or keep Paused only with a reason AND a resume condition): ", &*v14, "."]);
        v16
    }
}
fn waiting_line_30(mut v0: Rc<UH0>) -> Rc<str> {
    let v1: bool = match &*v0 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v4: i32 = 0i32;
        let v5: i32 = method28(v3.clone(), v0, v4);
        let v6: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
        let mut v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = Rc::<str>::from((*v7.borrow()).join(&*v8));
        let v11: Rc<str> = string_concat(&["Built and waiting on a train install (not ready work): ", &*v9, "."]);
        v11
    }
}
fn gate_lines_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: Rc<UH0>, mut v5: Rc<UH0>) -> Rc<UH0> {
    let mut v8: i32 = 0i32;
    let mut v9: i32 = method24(v3.clone(), v8);
    let v10: Rc<str> = format_real_25(v9);
    let v13: i32 = 0i32;
    let v14: i32 = method24(v4.clone(), v13);
    let v15: Rc<str> = format_real_25(v14);
    let v18: i32 = 0i32;
    let v19: i32 = method24(v5.clone(), v18);
    let v20: Rc<str> = format_real_25(v19);
    let v21: Rc<str> = string_concat(&["kit agile gate ok team=", &*v1, " ready=", &*v10, " paused=", &*v15, " waiting=", &*v20]);
    let v22: Rc<str> = ready_line_27(v0.clone(), &v1, v2.clone(), v3);
    let v23: Rc<str> = parked_line_29(&v0, v1, &v2, v4);
    let v24: Rc<str> = waiting_line_30(v5);
    let v25: bool = &*v21 != "";
    let v29: Rc<UH0> = if v25 {
        let v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v21, v26))
    } else {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    };
    let v30: bool = &*v24 != "";
    let v32: Rc<UH0> = if v30 {
        Rc::new(UH0::UH0_Cons(v24, v29))
    } else {
        v29
    };
    let v33: bool = &*v23 != "";
    let v35: Rc<UH0> = if v33 {
        Rc::new(UH0::UH0_Cons(v23, v32))
    } else {
        v32
    };
    let v36: bool = &*v22 != "";
    if v36 {
        Rc::new(UH0::UH0_Cons(v22, v35))
    } else {
        v35
    }
}
fn method31(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
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
fn gate_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_1(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                            let mut v9: bool = v6.starts_with(&*v8);
                            let v10: bool = v9 == false;
                            if v10 {
                                let (mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>) = terms_of_4(v6.clone(), v7);
                                let v14: bool = &*v12 == "";
                                let v16: bool = if v14 {
                                    true
                                } else {
                                    let v15: bool = &*v13 == "";
                                    v15
                                };
                                if v16 {
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                                    eprintln!("{}", v17);
                                    2i32
                                } else {
                                    let v18: Rc<RefCell<Vec<(Rc<str>, bool)>>> = agile_team_work::rig_agile_gate_rows(&v6, &v12);
                                    let v19: i32 = (v18.clone().borrow().len() as i32);
                                    let v20: i32 = v19.wrapping_sub(1i32);
                                    let v21: Rc<RefCell<Vec<Rc<str>>>> = agile_team_lane::rig_agile_team_paused(&v6, &v12);
                                    let v22: Rc<Vec<Rc<str>>> = Rc::new((v21).borrow().clone());
                                    let v23: usize = (v22).len();
                                    let v24: i32 = ((v23) as i32);
                                    let v25: i32 = v24.wrapping_sub(1i32);
                                    let v26: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                                    let v27: Rc<UH1> = method6(v22, v25, v26);
                                    let v28: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v29: Rc<UH0> = method7(v27, v28);
                                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                                    let v32: Rc<str> = Rc::<str>::from(v6.replace(&*v30, &*v31));
                                    let v33: Rc<str> = string_concat(&[&*v32, "/state/agile.spi"]);
                                    let v34: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v33).unwrap_or_default().as_str());
                                    let v35: Rc<str> = Rc::<str>::from(v6.replace(&*v30, &*v31));
                                    let v36: Rc<str> = string_concat(&[&*v35, "/state/waits.spi"]);
                                    let v37: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v36).unwrap_or_default().as_str());
                                    let v38: Rc<UH0> = install_waiting_8(v34);
                                    let v39: bool = false;
                                    let v40: Rc<UH0> = ids_where_17(&v18, v39, v20);
                                    let v41: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v42: Rc<UH0> = method18(v37, v40, v41);
                                    let v43: Rc<UH0> = method16(v38, v42);
                                    let v44: bool = false;
                                    let v45: Rc<UH0> = ids_where_17(&v18, v44, v20);
                                    let v46: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v47: Rc<UH0> = method19(v43.clone(), v45.clone(), v46);
                                    let v48: i32 = 3i32;
                                    let v49: Rc<UH0> = taken_21(v48, v47);
                                    let v50: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v51: Rc<UH0> = method22(v43, v45, v50);
                                    let v52: Rc<UH0> = gate_lines_23(v11, v12, v13, v49, v29, v51);
                                    method31(v52);
                                    0i32
                                }
                            } else {
                                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                                eprintln!("{}", v54);
                                2i32
                            }
                        }
                        _ => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v56);
                            2i32
                        }
                    }
                }
                _ => {
                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v58);
                    2i32
                }
            }
        }
        _ => {
            let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v60);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        gate_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_gate_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
