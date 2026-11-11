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
#[derive(Clone)]
enum US0 {
    US0_HeadLine,
    US0_TagsLine,
    US0_PlanningLine,
    US0_MilestonesLine,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(i32, Rc<UH2>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, i32),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<UH0>, i32),
    US3_None,
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: bool = &*v1 == "";
                let mut v5: Rc<str> = if v4 {
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
fn marked_4(mut v0: US0, mut v1: Rc<str>) -> bool {
    match v0 {
        US0::US0_HeadLine => {
            false
        }
        US0::US0_MilestonesLine => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MILESTONES:"); } LIT.with(|lit| lit.clone()) };
            let v7: bool = v1.contains(&*v6);
            v7
        }
        US0::US0_PlanningLine => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning:"); } LIT.with(|lit| lit.clone()) };
            let mut v5: bool = v1.contains(&*v4);
            v5
        }
        US0::US0_TagsLine => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags:"); } LIT.with(|lit| lit.clone()) };
            let mut v3: bool = v1.starts_with(&*v2);
            v3
        }
    }
}
fn method3(mut v0: US0, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = marked_4(v0.clone(), v3.clone());
                let v6: Rc<str> = if v5 {
                    v3
                } else {
                    v2
                };
                (v1, v2) = (v4, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn method7(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
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
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method6(v16, v17)
    }
}
fn line_of_1(mut v0: Rc<str>, mut v1: US0) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(v0.replace(&*v3, &*v4));
    let v6: Rc<UH0> = split_list_5(v2, v5);
    match &v1 {
        US0::US0_HeadLine => {
            method2(v6, v4)
        }
        _ => {
            method3(v1, v6, v4)
        }
    }
}
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
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
fn method9(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
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
fn method11(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: bool = &*v1 == "";
                let mut v5: Rc<str> = if v4 {
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
fn method14(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure2() -> Rc<dyn Fn((i32)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US1> = Rc::new(move |mut v0: (i32)| -> US1 {
        let mut v1: i32 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn number_of_13(mut v0: Rc<str>) -> US1 {
    let mut v1: Option<i32> = (v0).trim().parse::<i32>().ok();
    let mut v2: Option<i32> = method14(v1);
    let mut v3: Rc<dyn Fn((i32)) -> US1> = closure2();
    let mut v4: Option<US1> = v2.map(|x| v3(x));
    let v6: US1 = US1::US1_None;
    let v7: US1 = v6.clone();
    let v8: US1 = v4.unwrap_or(v7);
    v8
}
fn estimate_in_10(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate="); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method11(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        0i32
    } else {
        let mut v9: i32 = v7.wrapping_add(9i32);
        let v10: i32 = (v0.clone().len() as i32);
        let v11: i32 = v10.wrapping_sub(1i32);
        let v12: Rc<str> = string_slice(&v0, v9 as i64, v11 as i64);
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<UH0> = split_list_5(v13, v12);
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = method12(v14, v15);
        let mut v17: US1 = number_of_13(v16);
        match v17 {
            US1::US1_None => {
                0i32
            }
            US1::US1_Some(v18) => {
                let v18: i32 = v18;
                v18
            }
        }
    }
}
fn nth_word_16(mut v0: Rc<UH0>, mut v1: i32) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: bool = v1 == 0i32;
                if v4 {
                    return v2;
                } else {
                    let mut v5: i32 = v1.wrapping_sub(1i32);
                    (v0, v1) = (v3, v5);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v8;
            }
        }
    }
}
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method17(v3, v1);
            let mut v5: bool = &*v2 != "";
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
fn word_at_15(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<UH0> = split_list_5(v2, v0);
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH0> = method17(v3, v4);
    nth_word_16(v5, v1)
}
fn shown_fields_of_0(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, i32) {
    let mut v1: US0 = US0::US0_HeadLine;
    let mut v2: Rc<str> = line_of_1(v0.clone(), v1);
    let mut v3: US0 = US0::US0_TagsLine;
    let mut v4: Rc<str> = line_of_1(v0.clone(), v3);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags:"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v6));
    let v8: i32 = (v7.clone().len() as i32);
    let mut v9: i32 = 0i32;
    let v10: i32 = method8(v7.clone(), v8, v9);
    let v11: i32 = v8.wrapping_sub(1i32);
    let v12: Rc<str> = string_slice(&v7, v10 as i64, v11 as i64);
    let v13: i32 = (v12.clone().len() as i32);
    let v14: i32 = method9(v12.clone(), v13);
    let v15: Rc<str> = string_slice(&v12, 0i32 as i64, v14 as i64);
    let v16: US0 = US0::US0_PlanningLine;
    let mut v17: Rc<str> = line_of_1(v0, v16);
    let v18: i32 = estimate_in_10(v17);
    let v19: i32 = 2i32;
    let v20: Rc<str> = word_at_15(v2.clone(), v19);
    let v21: i32 = 3i32;
    let v22: Rc<str> = word_at_15(v2, v21);
    (v20, v22, v15, v18)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, i32)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, i32)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>, i32) {
        let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, Rc<str>, Rc<str>, i32) = shown_fields_of_0(v0);
        (v1, v2, v3, v4)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method20(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let mut v3: i32 = v3;
                let mut v4: Rc<UH1> = v4;
                let mut v5: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v1));
                (v0, v1) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method22(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: i32 = v2;
                let mut v3: Rc<UH2> = v3;
                let mut v4: bool = v2 < v1;
                let mut v5: i32 = if v4 {
                    v2
                } else {
                    v1
                };
                (v0, v1) = (v3, v5);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn milestones_text_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MILESTONES:"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method11(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    } else {
        let v10: i32 = v7.wrapping_add(11i32);
        let v11: i32 = (v0.clone().len() as i32);
        let v12: i32 = v11.wrapping_sub(1i32);
        let v13: Rc<str> = string_slice(&v0, v10 as i64, v12 as i64);
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(". "); } LIT.with(|lit| lit.clone()) };
        let v15: Option<usize> = v13.find(&*v14);
        let v16: Option<usize> = method11(v15);
        let mut v17: Option<i32> = v16.map(|x| v4(x));
        let v19: i32 = v17.unwrap_or(-1i32);
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v21: Option<usize> = v13.find(&*v20);
        let v22: Option<usize> = method11(v21);
        let mut v23: Option<i32> = v22.map(|x| v4(x));
        let v25: i32 = v23.unwrap_or(-1i32);
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("NEXT STEP"); } LIT.with(|lit| lit.clone()) };
        let v27: Option<usize> = v13.find(&*v26);
        let v28: Option<usize> = method11(v27);
        let mut v29: Option<i32> = v28.map(|x| v4(x));
        let v31: i32 = v29.unwrap_or(-1i32);
        let v32: bool = v19 >= 0i32;
        let v36: Rc<UH2> = if v32 {
            let v33: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_Cons(v19, v33))
        } else {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
        };
        let v37: bool = v25 >= 0i32;
        let v39: Rc<UH2> = if v37 {
            Rc::new(UH2::UH2_Cons(v25, v36))
        } else {
            v36
        };
        let v40: bool = v31 >= 0i32;
        let v42: Rc<UH2> = if v40 {
            Rc::new(UH2::UH2_Cons(v31, v39))
        } else {
            v39
        };
        let v43: i32 = (v13.clone().len() as i32);
        let v44: i32 = method22(v42, v43);
        let v45: i32 = v44.wrapping_sub(1i32);
        let v46: Rc<str> = string_slice(&v13, 0i32 as i64, v45 as i64);
        v46
    }
}
fn method25(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method25(v3, v1);
            let mut v5: bool = &*v2 != "";
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
fn minutes_in_26(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> US3 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let mut v4: Rc<str> = v4.clone();
                        let mut v5: Rc<UH0> = v5.clone();
                        let v6: bool = &*v4 == "m";
                        let v10: bool = if v6 {
                            let v7: US1 = number_of_13(v2.clone());
                            let v8: bool = match v7 {
                                US1::US1_None => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            };
                            let mut v9: bool = v8 != true;
                            v9
                        } else {
                            false
                        };
                        if v10 {
                            let v11: US1 = number_of_13(v2);
                            let v14: i32 = match v11 {
                                US1::US1_None => {
                                    0i32
                                }
                                US1::US1_Some(v12) => {
                                    let v12: i32 = v12;
                                    v12
                                }
                            };
                            return US3::US3_Some(v1, v14);
                        } else {
                            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                            let mut v17: bool = v2.ends_with(&*v16);
                            let v24: bool = if v17 {
                                let v18: i32 = (v2.clone().len() as i32);
                                let v19: i32 = v18.wrapping_sub(2i32);
                                let v20: Rc<str> = string_slice(&v2.clone(), 0i32 as i64, v19 as i64);
                                let v21: US1 = number_of_13(v20);
                                let v22: bool = match v21 {
                                    US1::US1_None => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                };
                                let mut v23: bool = v22 != true;
                                v23
                            } else {
                                false
                            };
                            if v24 {
                                let v25: i32 = (v2.clone().len() as i32);
                                let v26: i32 = v25.wrapping_sub(2i32);
                                let v27: Rc<str> = string_slice(&v2, 0i32 as i64, v26 as i64);
                                let v28: US1 = number_of_13(v27);
                                let v31: i32 = match v28 {
                                    US1::US1_None => {
                                        0i32
                                    }
                                    US1::US1_Some(v29) => {
                                        let mut v29: i32 = v29;
                                        v29
                                    }
                                };
                                return US3::US3_Some(v1, v31);
                            } else {
                                let v33: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                                (v0, v1) = (v3, v33);
                                continue;
                            }
                        }
                    }
                    _ => {
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                        let v38: bool = v2.ends_with(&*v37);
                        let v45: bool = if v38 {
                            let v39: i32 = (v2.clone().len() as i32);
                            let v40: i32 = v39.wrapping_sub(2i32);
                            let v41: Rc<str> = string_slice(&v2.clone(), 0i32 as i64, v40 as i64);
                            let v42: US1 = number_of_13(v41);
                            let v43: bool = match v42 {
                                US1::US1_None => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            };
                            let v44: bool = v43 != true;
                            v44
                        } else {
                            false
                        };
                        if v45 {
                            let v46: i32 = (v2.clone().len() as i32);
                            let v47: i32 = v46.wrapping_sub(2i32);
                            let v48: Rc<str> = string_slice(&v2, 0i32 as i64, v47 as i64);
                            let v49: US1 = number_of_13(v48);
                            let v52: i32 = match v49 {
                                US1::US1_None => {
                                    0i32
                                }
                                US1::US1_Some(v50) => {
                                    let v50: i32 = v50;
                                    v50
                                }
                            };
                            return US3::US3_Some(v1, v52);
                        } else {
                            let v54: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                            (v0, v1) = (v3, v54);
                            continue;
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                return US3::US3_None;
            }
        }
    }
}
fn method27(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn milestone_of_24(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v2));
    let v6: Rc<UH0> = split_list_5(v2.clone(), v5);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = method25(v6, v7);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: US3 = minutes_in_26(v8, v9);
    match v10 {
        US3::US3_Some(v11, v12) => {
            let v11: Rc<UH0> = v11;
            let v12: i32 = v12;
            let v13: bool = v12 > 0i32;
            let v16: bool = if v13 {
                let v14: bool = match &*v11 {
                    UH0::UH0_Nil => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v15: bool = v14 != true;
                v15
            } else {
                false
            };
            if v16 {
                let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v18: Rc<UH0> = method6(v11, v17);
                let v19: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v20: i32 = 0i32;
                let v21: i32 = method27(v19.clone(), v18, v20);
                let v22: Rc<Vec<Rc<str>>> = Rc::new(v19.borrow().clone());
                let mut v23: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v22).as_ref().clone()));
                let v24: Rc<str> = Rc::<str>::from((*v23.borrow()).join(&*v2));
                US2::US2_Some(v24, v12)
            } else {
                US2::US2_None
            }
        }
        _ => {
            US2::US2_None
        }
    }
}
fn method23(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: US2 = milestone_of_24(v2.clone());
                let v12: Rc<UH1> = match v4 {
                    US2::US2_Some(v5, v6) => {
                        let mut v5: Rc<str> = v5;
                        let v6: i32 = v6;
                        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(done)"); } LIT.with(|lit| lit.clone()) };
                        let v8: bool = v2.contains(&*v7);
                        let mut v9: bool = v8 == false;
                        if v9 {
                            Rc::new(UH1::UH1_Cons(v5, v6, v1))
                        } else {
                            v1
                        }
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v3, v12);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn milestones_of_19(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = milestones_text_21(v0);
    let mut v3: Rc<UH0> = split_list_5(v1, v2);
    let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH1> = method23(v3, v4);
    let v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method20(v5, v6)
}
fn method28(mut v0: Rc<RefCell<Vec<(Rc<str>, i32)>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5) => {
                let mut v3: Rc<str> = v3;
                let mut v4: i32 = v4;
                let mut v5: Rc<UH1> = v5;
                v0.borrow_mut().push((v3, v4));
                let v6: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v5, v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn shown_steps_18(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i32)>>> {
    let mut v1: US0 = US0::US0_MilestonesLine;
    let mut v2: Rc<str> = line_of_1(v0, v1);
    let mut v3: Rc<UH1> = milestones_of_19(v2);
    let mut v4: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v5: i32 = 0i32;
    let v6: i32 = method28(v4.clone(), v3, v5);
    let v7: Rc<Vec<(Rc<str>, i32)>> = Rc::new(v4.borrow().clone());
    let v8: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    v8
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i32)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i32)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, i32)>>> {
        shown_steps_18(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_split_fields(v0: &str) -> (Rc<str>, Rc<str>, Rc<str>, i32) {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_agile_split_steps(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, i32)>>> {
    closure3()(Rc::<str>::from(v0))
}
pub type SpiralShape_652accfc6a8d = Mut0;
