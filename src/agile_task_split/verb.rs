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
enum US1 {
    US1_HeadLine,
    US1_TagsLine,
    US1_PlanningLine,
    US1_MilestonesLine,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, Rc<UH1>),
}
#[derive(Clone)]
enum US3 {
    US3_SplitInto(Rc<UH1>),
    US3_NoMilestones,
    US3_MilestoneTooBig(Rc<str>, i32),
    US3_AlreadySmall(i32),
}
pub struct Mut1 { pub l0: Rc<str> }
fn method3(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure1() -> Rc<dyn Fn((Rc<str>)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US0> = Rc::new(move |mut v0: (Rc<str>)| -> US0 {
        let mut v1: Rc<str> = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn current_executable_2() -> Rc<str> {
    let mut v0: Option<std::path::PathBuf> = std::env::current_exe().ok();
    let mut v2: bool = true; let _optionm_map_ = v0.map(|x| { //;
    let mut v4: std::path::PathBuf = x;
    let mut v5: Rc<str> = Rc::<str>::from(v4.to_string_lossy().as_ref());
    let mut v7: bool = true; v5 });
    let mut v9: Option<Rc<str>> = _optionm_map_;
    let v10: Option<Rc<str>> = method3(v9);
    let mut v11: Rc<dyn Fn((Rc<str>)) -> US0> = closure1();
    let mut v12: Option<US0> = v10.map(|x| v11(x));
    let v14: US0 = US0::US0_None;
    let v15: US0 = v14.clone();
    let v16: US0 = v12.unwrap_or(v15);
    match v16 {
        US0::US0_None => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v18
        }
        US0::US0_Some(v17) => {
            let v17: Rc<str> = v17;
            v17
        }
    }
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn method5(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method6(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn rig_said_1(mut v0: Rc<UH0>) -> (bool, Rc<str>) {
    let mut v1: Rc<str> = current_executable_2();
    let mut v2: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v1)));
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = method4(v0, v3);
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = method5(v4, v5);
    method6(v2.clone(), v6);
    let mut v7: Rc<std::io::Result<std::process::Output>> = Rc::new(v2.borrow_mut().output());
    let v8: bool = v7.is_ok();
    let (mut v14, mut v15, mut v16): (bool, i32, Rc<str>) = if v8 {
        let mut v9: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v7).as_ref().unwrap().stderr).as_ref());
        let v10: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v7).as_ref().unwrap().stdout).as_ref());
        let mut v11: Rc<str> = string_concat(&[&*v10, &*v9]);
        let mut v12: i32 = (&*v7).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v12, v11)
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v13)
    };
    let v18: bool = if v14 {
        let v17: bool = v15 == 0i32;
        v17
    } else {
        false
    };
    (v18, v16)
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
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
fn marked_11(mut v0: US1, mut v1: Rc<str>) -> bool {
    match v0 {
        US1::US1_HeadLine => {
            false
        }
        US1::US1_MilestonesLine => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MILESTONES:"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = v1.contains(&*v6);
            v7
        }
        US1::US1_PlanningLine => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning:"); } LIT.with(|lit| lit.clone()) };
            let mut v5: bool = v1.contains(&*v4);
            v5
        }
        US1::US1_TagsLine => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags:"); } LIT.with(|lit| lit.clone()) };
            let mut v3: bool = v1.starts_with(&*v2);
            v3
        }
    }
}
fn method10(mut v0: US1, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = marked_11(v0.clone(), v3.clone());
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
fn method13(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method13(v7, &v9) {
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
        method5(v16, v17)
    }
}
fn line_of_8(mut v0: Rc<str>, mut v1: US1) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(v0.replace(&*v3, &*v4));
    let v6: Rc<UH0> = split_list_12(v2, v5);
    match &v1 {
        US1::US1_HeadLine => {
            method9(v6, v4)
        }
        _ => {
            method10(v1, v6, v4)
        }
    }
}
fn method14(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method15(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
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
fn method17(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
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
fn method20(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure3() -> Rc<dyn Fn((i32)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US2> = Rc::new(move |mut v0: (i32)| -> US2 {
        let mut v1: i32 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn number_of_19(mut v0: Rc<str>) -> US2 {
    let mut v1: Option<i32> = (v0).trim().parse::<i32>().ok();
    let mut v2: Option<i32> = method20(v1);
    let mut v3: Rc<dyn Fn((i32)) -> US2> = closure3();
    let mut v4: Option<US2> = v2.map(|x| v3(x));
    let v6: US2 = US2::US2_None;
    let mut v7: US2 = v6.clone();
    let v8: US2 = v4.unwrap_or(v7);
    v8
}
fn estimate_in_16(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate="); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method17(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        0i32
    } else {
        let mut v9: i32 = v7.wrapping_add(9i32);
        let v10: i32 = (v0.clone().len() as i32);
        let mut v11: i32 = v10.wrapping_sub(1i32);
        let mut v12: Rc<str> = string_slice(&v0, v9 as i64, v11 as i64);
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<UH0> = split_list_12(v13, v12);
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = method18(v14, v15);
        let v17: US2 = number_of_19(v16);
        match v17 {
            US2::US2_None => {
                0i32
            }
            US2::US2_Some(v18) => {
                let v18: i32 = v18;
                v18
            }
        }
    }
}
fn nth_word_22(mut v0: Rc<UH0>, mut v1: i32) -> Rc<str> {
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
fn method23(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method23(v3, v1);
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
fn word_at_21(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<UH0> = split_list_12(v2, v0);
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH0> = method23(v3, v4);
    nth_word_22(v5, v1)
}
fn shown_fields_of_7(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, i32) {
    let mut v1: US1 = US1::US1_HeadLine;
    let mut v2: Rc<str> = line_of_8(v0.clone(), v1);
    let mut v3: US1 = US1::US1_TagsLine;
    let mut v4: Rc<str> = line_of_8(v0.clone(), v3);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags:"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v6));
    let v8: i32 = (v7.clone().len() as i32);
    let mut v9: i32 = 0i32;
    let v10: i32 = method14(v7.clone(), v8, v9);
    let mut v11: i32 = v8.wrapping_sub(1i32);
    let mut v12: Rc<str> = string_slice(&v7, v10 as i64, v11 as i64);
    let v13: i32 = (v12.clone().len() as i32);
    let v14: i32 = method15(v12.clone(), v13);
    let v15: Rc<str> = string_slice(&v12, 0i32 as i64, v14 as i64);
    let v16: US1 = US1::US1_PlanningLine;
    let v17: Rc<str> = line_of_8(v0, v16);
    let v18: i32 = estimate_in_16(v17);
    let v19: i32 = 2i32;
    let v20: Rc<str> = word_at_21(v2.clone(), v19);
    let v21: i32 = 3i32;
    let v22: Rc<str> = word_at_21(v2, v21);
    (v20, v22, v15, v18)
}
fn gathered_25(mut v0: Rc<RefCell<Vec<(Rc<str>, i32)>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let (mut v4, mut v5): (Rc<str>, i32) = v0.borrow()[v1 as usize].clone();
            let v6: i32 = v1.wrapping_sub(1i32);
            let mut v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v4, v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn steps_of_24(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<RefCell<Vec<(Rc<str>, i32)>>> = agile_split_plan::rig_agile_split_steps(&v0);
    let mut v2: i32 = (v1.clone().borrow().len() as i32);
    let mut v3: i32 = v2.wrapping_sub(1i32);
    let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    gathered_25(v1, v3, v4)
}
fn method27(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let mut v3: i32 = v3;
            let mut v4: Rc<UH1> = v4;
            let mut v5: Rc<UH1> = method27(v4, v1);
            let v6: bool = v3 > 120i32;
            if v6 {
                Rc::new(UH1::UH1_Cons(v2, v3, v5))
            } else {
                v5
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn split_plan_of_26(mut v0: i32, mut v1: Rc<UH1>) -> US3 {
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH1> = method27(v1.clone(), v2);
    let mut v4: bool = v0 <= 240i32;
    if v4 {
        US3::US3_AlreadySmall(v0)
    } else {
        match Rc::unwrap_or_clone(v3) {
            UH1::UH1_Cons(v6, v7, v8) => {
                let v6: Rc<str> = v6;
                let mut v7: i32 = v7;
                let v8: Rc<UH1> = v8;
                US3::US3_MilestoneTooBig(v6, v7)
            }
            UH1::UH1_Nil => {
                let v10: bool = match &*v1 {
                    UH1::UH1_Nil => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                if v10 {
                    US3::US3_NoMilestones
                } else {
                    US3::US3_SplitInto(v1)
                }
            }
        }
    }
}
fn method29(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_28(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method29(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn child_id_31(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v3: Rc<str> = format_real_28(v1);
    let mut v4: Rc<str> = string_concat(&[&*v0, "-M", &*v3]);
    v4
}
fn child_note_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> Rc<str> {
    let v6: Rc<str> = format_real_28(v3);
    let v10: Rc<str> = string_concat(&["split from ", &*v0, " by kit agile split (milestone ", &*v6, "). DoD: ", &*v1, ", proven in the parent item notes. NEXT STEP: read the parent latest note for this milestone."]);
    v10
}
fn method33(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method33(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn children_added_30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: &Rc<str>, mut v4: Rc<str>, mut v5: i32, mut v6: Rc<UH1>, mut v7: i32, mut v8: Rc<str>) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v6) {
            UH1::UH1_Cons(v10, v11, v12) => {
                let v10: Rc<str> = v10;
                let mut v11: i32 = v11;
                let mut v12: Rc<UH1> = v12;
                let v13: Rc<str> = child_id_31(v1.clone(), v7);
                let v14: bool = &*v8 == "";
                let v20: Rc<UH0> = if v14 {
                    { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                } else {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--deps"); } LIT.with(|lit| lit.clone()) };
                    let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v17));
                    Rc::new(UH0::UH0_Cons(v16, v18))
                };
                let v22: Rc<str> = format_real_28(v7);
                let v25: Rc<str> = string_concat(&[&*v1, " milestone ", &*v22, ": ", &*v10]);
                let v26: Rc<str> = format_real_28(v11);
                let v27: Rc<str> = child_note_32(v1.clone(), v10, v11, v7);
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--note"); } LIT.with(|lit| lit.clone()) };
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-split"); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v30, v31));
                let v33: Rc<UH0> = Rc::new(UH0::UH0_Cons(v29, v32));
                let v34: Rc<UH0> = Rc::new(UH0::UH0_Cons(v27, v33));
                let v35: Rc<UH0> = Rc::new(UH0::UH0_Cons(v28, v34));
                let v36: Rc<UH0> = method33(v20, v35);
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--estimate"); } LIT.with(|lit| lit.clone()) };
                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--load"); } LIT.with(|lit| lit.clone()) };
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("light"); } LIT.with(|lit| lit.clone()) };
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--assignee"); } LIT.with(|lit| lit.clone()) };
                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agent"); } LIT.with(|lit| lit.clone()) };
                let v46: Rc<UH0> = Rc::new(UH0::UH0_Cons(v45, v36));
                let v47: Rc<UH0> = Rc::new(UH0::UH0_Cons(v44, v46));
                let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v43, v47));
                let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v42, v48));
                let v50: Rc<UH0> = Rc::new(UH0::UH0_Cons(v26, v49));
                let v51: Rc<UH0> = Rc::new(UH0::UH0_Cons(v41, v50));
                let v52: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v51));
                let v53: Rc<UH0> = Rc::new(UH0::UH0_Cons(v25, v52));
                let v54: Rc<UH0> = Rc::new(UH0::UH0_Cons(v40, v53));
                let v55: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v54));
                let v56: Rc<UH0> = Rc::new(UH0::UH0_Cons(v39, v55));
                let v57: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v56));
                let v58: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0.clone(), v57));
                let v59: Rc<UH0> = Rc::new(UH0::UH0_Cons(v38, v58));
                let v60: Rc<UH0> = Rc::new(UH0::UH0_Cons(v37.clone(), v59));
                let (mut v61, mut v62): (bool, Rc<str>) = rig_said_1(v60);
                let (mut v75, mut v76): (bool, Rc<str>) = if v61 {
                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--parent"); } LIT.with(|lit| lit.clone()) };
                    let v65: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v66: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1.clone(), v65));
                    let v67: Rc<UH0> = Rc::new(UH0::UH0_Cons(v64, v66));
                    let v68: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v67));
                    let v69: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0.clone(), v68));
                    let v70: Rc<UH0> = Rc::new(UH0::UH0_Cons(v63, v69));
                    let v71: Rc<UH0> = Rc::new(UH0::UH0_Cons(v37, v70));
                    rig_said_1(v71)
                } else {
                    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (false, v74)
                };
                let v77: bool = v61 && v75;
                if v77 {
                    let v78: i32 = v7.wrapping_add(1i32);
                    (v6, v7, v8) = (v12, v78, v13);
                    continue;
                } else {
                    let v80: i32 = v7.wrapping_sub(1i32);
                    return v80;
                }
            }
            UH1::UH1_Nil => {
                let mut v9: i32 = v7.wrapping_sub(1i32);
                return v9;
            }
        }
    }
}
fn method34(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let mut v3: i32 = v3;
                let mut v4: Rc<UH1> = v4;
                let mut v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn split_run_0() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v6));
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1.clone(), v7));
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v8));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v9));
    let (mut v11, mut v12): (bool, Rc<str>) = rig_said_1(v10);
    let v13: bool = v11 == false;
    let v15: bool = if v13 {
        true
    } else {
        let v14: bool = &*v3 == "";
        v14
    };
    if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile edit <root> <id> --split milestones (needs MILESTONES:)"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v16);
        2i32
    } else {
        let (mut v17, mut v18, mut v19, mut v20): (Rc<str>, Rc<str>, Rc<str>, i32) = shown_fields_of_7(v12.clone());
        let v21: Rc<UH1> = steps_of_24(v12);
        let v22: US3 = split_plan_of_26(v20, v21);
        match v22 {
            US3::US3_AlreadySmall(v23) => {
                let v23: i32 = v23;
                let v24: Rc<str> = format_real_28(v23);
                let v27: Rc<str> = string_concat(&["kit agile split ok id=", &*v3, " children=0 reason=already-small estimate=", &*v24]);
                println!("{}", v27);
                0i32
            }
            US3::US3_MilestoneTooBig(v30, v31) => {
                let v30: Rc<str> = v30;
                let v31: i32 = v31;
                let v32: Rc<str> = format_real_28(v31);
                let v36: Rc<str> = string_concat(&["kit error: agile split refused: milestone ", &*v30, " is ", &*v32, " m; split it to 120 m or less first"]);
                eprintln!("{}", v36);
                1i32
            }
            US3::US3_NoMilestones => {
                let v29: Rc<str> = string_concat(&["kit error: agile split refused: ", &*v3, " has no note with MILESTONES:"]);
                eprintln!("{}", v29);
                1i32
            }
            US3::US3_SplitInto(v37) => {
                let v37: Rc<UH1> = v37;
                let v38: i32 = 1i32;
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v40: i32 = children_added_30(v1.clone(), v3.clone(), v17, &v18, v19, v20, v37.clone(), v38, v39);
                let v41: Rc<str> = format_real_28(v40);
                let v43: Rc<str> = string_concat(&["split into ", &*v41, " chained children by kit agile split"]);
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-split"); } LIT.with(|lit| lit.clone()) };
                let v47: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v46, v47));
                let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v45, v48));
                let v50: Rc<UH0> = Rc::new(UH0::UH0_Cons(v43, v49));
                let v51: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v50));
                let v52: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v51));
                let v53: Rc<UH0> = Rc::new(UH0::UH0_Cons(v44, v52));
                let v54: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v53));
                let (mut v55, mut v56): (bool, Rc<str>) = rig_said_1(v54);
                let v59: Rc<str> = format_real_28(v40);
                let v60: Rc<str> = string_concat(&["kit agile split ok id=", &*v3, " children=", &*v59]);
                println!("{}", v60);
                let v61: i32 = 0i32;
                let v62: i32 = method34(v37, v61);
                let v63: bool = v40 == v62;
                if v63 {
                    0i32
                } else {
                    1i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        split_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_split_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
