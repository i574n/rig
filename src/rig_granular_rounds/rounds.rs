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
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_CountRow(Rc<str>, u64, Rc<str>, i32),
    US2_OtherLine,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US2, Rc<UH2>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
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
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_2();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let mut v5: Rc<str> = command_line_argument_3(v0);
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
            let v12: Rc<UH0> = arguments_from_1(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn method4(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
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
fn method9(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH1>, mut v3: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v4, v5, v6) => {
            let mut v4: Rc<str> = v4;
            let mut v5: i32 = v5;
            let mut v6: Rc<UH1> = v6;
            let v7: Rc<UH1> = method9(v0.clone(), v1, v6, v3);
            let v8: bool = &*v4 == &*v0;
            let (mut v10, mut v11): (Rc<str>, i32) = if v8 {
                let mut v9: i32 = v5.wrapping_add(v1);
                (v4, v9)
            } else {
                (v4, v5)
            };
            Rc::new(UH1::UH1_Cons(v10, v11, v7))
        }
        UH1::UH1_Nil => {
            v3
        }
    }
}
fn method10(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let v2: Rc<str> = v2;
            let v3: i32 = v3;
            let mut v4: Rc<UH1> = v4;
            let mut v5: Rc<UH1> = method10(v4, v1);
            Rc::new(UH1::UH1_Cons(v2, v3, v5))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method11(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let v2: Rc<str> = v2;
            let v3: i32 = v3;
            let mut v4: Rc<UH1> = v4;
            let mut v5: Rc<UH0> = method11(v4, v1);
            Rc::new(UH0::UH0_Cons(v2, v5))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    Rc::new(move |mut v2: Rc<UH0>| -> US1 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US1::US1_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US1> = method12(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US1::US1_None
            }
        }
    })
}
fn method12(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    closure1(v0, v1)
}
fn added_8(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32) -> Rc<UH1> {
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = method11(v0.clone(), v3);
    let mut v5: i32 = 0i32;
    let mut v6: Rc<dyn Fn(Rc<UH0>) -> US1> = method12(v1.clone(), v5);
    let v7: US1 = v6(v4);
    let v8: bool = match v7 {
        US1::US1_None => {
            true
        }
        _ => {
            false
        }
    };
    let mut v9: bool = v8 != true;
    if v9 {
        let mut v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method9(v1, v2, v0, v10)
    } else {
        let v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let v13: Rc<UH1> = Rc::new(UH1::UH1_Cons(v1, v2, v12));
        method10(v0, v13)
    }
}
fn summed_7(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let (mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, i32, i32) = v0.borrow()[v1 as usize].clone();
            let mut v9: i32 = v1.wrapping_add(1i32);
            let mut v10: Rc<UH1> = added_8(v2, v6, v7);
            (v1, v2) = (v9, v10);
            continue;
        }
    }
}
fn packages_summed_6(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let mut v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = rig_style_measure::rig_style_measured(&v5);
            let v7: i32 = v1.wrapping_add(1i32);
            let v8: i32 = 0i32;
            let mut v9: Rc<UH1> = summed_7(&v6, v8, v2);
            (v1, v2) = (v7, v9);
            continue;
        }
    }
}
fn totals_of_5(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<UH1> {
    let v1: Rc<RefCell<Vec<Rc<str>>>> = rig_style_measure::rig_style_packages_under(v0);
    let v2: i32 = 0i32;
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    packages_summed_6(v1, v2, v3)
}
fn get_environment_variable_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn config_path_13() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
    let v1: Rc<str> = get_environment_variable_14(v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("APPDATA"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = get_environment_variable_14(v2);
    let mut v4: bool = &*v1 != "";
    let v11: Rc<str> = if v4 {
        v1
    } else {
        let mut v5: bool = &*v3 != "";
        if v5 {
            let mut v6: Rc<str> = string_concat(&[&*v3, "/kit"]);
            v6
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HOME"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = get_environment_variable_14(v7);
            let mut v9: Rc<str> = string_concat(&[&*v8, "/.config/kit"]);
            v9
        }
    };
    let v12: Rc<str> = string_concat(&[&*v11, "/granular_rounds.spi"]);
    v12
}
fn method16(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let v2: Rc<str> = v2;
                let v3: i32 = v3;
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
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn method19(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method19(v7, &v9) {
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
        method18(v16, v17)
    }
}
fn method22(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method23(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure3() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method24(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure4() -> Rc<dyn Fn((i32)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US1> = Rc::new(move |mut v0: (i32)| -> US1 {
        let v1: i32 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn line_of_21(mut v0: Rc<str>) -> US2 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = RoundCount (\""); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method22(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        US2::US2_OtherLine
    } else {
        let mut v10: i32 = v7.wrapping_add(16i32);
        let v11: i32 = v10.wrapping_sub(1i32);
        let v12: i32 = (v0.clone().len() as i32);
        let v13: i32 = v12.wrapping_sub(2i32);
        let v14: Rc<str> = string_slice(&v0, v11 as i64, v13 as i64);
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v14.split(&*v15).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v17: i32 = (v16.clone().borrow().len() as i32);
        let v18: bool = v17 > 1i32;
        let v24: Rc<str> = if v18 {
            let v19: Rc<str> = v16.borrow()[1i32 as usize].clone();
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = Rc::<str>::from(v19.replace(&*v20, &*v21));
            v22
        } else {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v23
        };
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = Rc::<str>::from(v24.replace(&*v25, &*v26));
        let v28: Option<u64> = (v27).trim().parse::<u64>().ok();
        let v29: Option<u64> = method23(v28);
        let mut v30: Rc<dyn Fn((u64)) -> US3> = closure3();
        let mut v31: Option<US3> = v29.map(|x| v30(x));
        let v33: US3 = US3::US3_None;
        let v34: US3 = v33.clone();
        let v35: US3 = v31.unwrap_or(v34);
        let v38: u64 = match v35 {
            US3::US3_None => {
                0u64
            }
            US3::US3_Some(v36) => {
                let v36: u64 = v36;
                v36
            }
        };
        let v39: bool = v17 > 3i32;
        let v43: Rc<str> = if v39 {
            let v40: Rc<str> = v16.borrow()[3i32 as usize].clone();
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v42: Rc<str> = Rc::<str>::from(v40.replace(&*v41, &*v26));
            v42
        } else {
            v26.clone()
        };
        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32"); } LIT.with(|lit| lit.clone()) };
        let v45: Rc<str> = Rc::<str>::from(v43.replace(&*v44, &*v26));
        let v46: Option<i32> = (v45).trim().parse::<i32>().ok();
        let v47: Option<i32> = method24(v46);
        let mut v48: Rc<dyn Fn((i32)) -> US1> = closure4();
        let mut v49: Option<US1> = v47.map(|x| v48(x));
        let v51: US1 = US1::US1_None;
        let v52: US1 = v51.clone();
        let v53: US1 = v49.unwrap_or(v52);
        let v56: i32 = match v53 {
            US1::US1_None => {
                0i32
            }
            US1::US1_Some(v54) => {
                let v54: i32 = v54;
                v54
            }
        };
        let v57: bool = v17 == 4i32;
        if v57 {
            let v58: bool = v17 > 0i32;
            let v62: Rc<str> = if v58 {
                let v59: Rc<str> = v16.borrow()[0i32 as usize].clone();
                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                let v61: Rc<str> = Rc::<str>::from(v59.replace(&*v60, &*v26));
                v61
            } else {
                v26.clone()
            };
            let v63: bool = v17 > 2i32;
            let v67: Rc<str> = if v63 {
                let v64: Rc<str> = v16.borrow()[2i32 as usize].clone();
                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                let v66: Rc<str> = Rc::<str>::from(v64.replace(&*v65, &*v26));
                v66
            } else {
                v26
            };
            US2::US2_CountRow(v62, v38, v67, v56)
        } else {
            US2::US2_OtherLine
        }
    }
}
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH2> = method20(v3, v1);
            let mut v5: US2 = line_of_21(v2);
            Rc::new(UH2::UH2_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method25(mut v0: Rc<str>, mut v1: Rc<UH2>, mut v2: u64) -> u64 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: US2 = v3;
                let mut v4: Rc<UH2> = v4;
                let v13: u64 = match v3 {
                    US2::US2_CountRow(v5, v6, v7, v8) => {
                        let mut v5: Rc<str> = v5;
                        let mut v6: u64 = v6;
                        let v7: Rc<str> = v7;
                        let v8: i32 = v8;
                        let mut v9: bool = &*v5 == &*v0;
                        let v11: bool = if v9 {
                            let mut v10: bool = v6 > v2;
                            v10
                        } else {
                            false
                        };
                        if v11 {
                            v6
                        } else {
                            v2
                        }
                    }
                    _ => {
                        v2
                    }
                };
                (v1, v2) = (v4, v13);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn method26(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<UH2>, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH2::UH2_Cons(v4, v5) => {
                let mut v4: US2 = v4;
                let mut v5: Rc<UH2> = v5;
                let v15: Rc<UH1> = match v4 {
                    US2::US2_CountRow(v6, v7, v8, v9) => {
                        let mut v6: Rc<str> = v6;
                        let v7: u64 = v7;
                        let v8: Rc<str> = v8;
                        let mut v9: i32 = v9;
                        let mut v10: bool = &*v6 == &*v0;
                        let v12: bool = if v10 {
                            let v11: bool = v7 == v1;
                            v11
                        } else {
                            false
                        };
                        if v12 {
                            Rc::new(UH1::UH1_Cons(v8, v9, v3))
                        } else {
                            v3
                        }
                    }
                    _ => {
                        v3
                    }
                };
                (v2, v3) = (v5, v15);
                continue;
            }
            UH2::UH2_Nil => {
                return v3;
            }
        }
    }
}
fn previous_of_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(v0.replace(&*v3, &*v4));
    let mut v6: Rc<UH0> = split_list_17(v2, v5);
    let v7: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH2> = method20(v6, v7);
    let mut v9: u64 = 0u64;
    let mut v10: u64 = method25(v1.clone(), v8.clone(), v9);
    let v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH1> = method26(v1, v10, v8, v11);
    let v13: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method16(v12, v13)
}
fn method29(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH1>, mut v3: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v4, v5, v6) => {
                let mut v4: Rc<str> = v4;
                let mut v5: i32 = v5;
                let mut v6: Rc<UH1> = v6;
                let v7: bool = &*v4 == &*v0;
                let v8: i32 = if v7 {
                    v5
                } else {
                    v3
                };
                (v2, v3) = (v6, v8);
                continue;
            }
            UH1::UH1_Nil => {
                return v3;
            }
        }
    }
}
fn method31(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_30(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method31(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn delta_line_28(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    let v3: i32 = -1i32;
    let mut v4: i32 = method29(v1.clone(), v2, v0, v3);
    let mut v5: i32 = v2.wrapping_sub(v4);
    let mut v6: bool = v4 < 0i32;
    let v13: Rc<str> = if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("new"); } LIT.with(|lit| lit.clone()) };
        v7
    } else {
        let v8: bool = v5 >= 0i32;
        if v8 {
            let mut v9: Rc<str> = format_real_30(v5);
            let mut v10: Rc<str> = string_concat(&["+", &*v9]);
            v10
        } else {
            format_real_30(v5)
        }
    };
    let v16: Rc<str> = format_real_30(v2);
    let v20: Rc<str> = string_concat(&["  ", &*v1, "=", &*v16, " (", &*v13, ")"]);
    v20
}
fn method27(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5) => {
            let v3: Rc<str> = v3;
            let mut v4: i32 = v4;
            let mut v5: Rc<UH1> = v5;
            let mut v6: Rc<UH0> = method27(v0.clone(), v5, v2);
            let v7: Rc<str> = delta_line_28(v0, v3, v4);
            Rc::new(UH0::UH0_Cons(v7, v6))
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn written_32(mut v0: Rc<UH0>) -> () {
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
fn slug_of_35(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v3));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(v6.replace(&*v7, &*v3));
    v8
}
fn format_real_36(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method31(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn row_text_34(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: i32) -> Rc<str> {
    let mut v4: Rc<str> = slug_of_35(v0.clone());
    let v7: Rc<str> = format_real_36(v1);
    let mut v10: Rc<str> = slug_of_35(v2.clone());
    let v15: Rc<str> = format_real_36(v1);
    let v20: Rc<str> = format_real_30(v3);
    let v22: Rc<str> = string_concat(&["inl round_", &*v4, "_", &*v7, "_", &*v10, " () = RoundCount (\"", &*v0, "\", ", &*v15, "u64, \"", &*v2, "\", ", &*v20, "i32)"]);
    v22
}
fn method33(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<UH1>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v4, v5, v6) => {
            let mut v4: Rc<str> = v4;
            let mut v5: i32 = v5;
            let mut v6: Rc<UH1> = v6;
            let v7: Rc<UH0> = method33(v0.clone(), v1, v6, v3);
            let v8: Rc<str> = row_text_34(v0, v1, v4, v5);
            let mut v9: Rc<str> = string_concat(&[&*v8, "\n"]);
            Rc::new(UH0::UH0_Cons(v9, v7))
        }
        UH1::UH1_Nil => {
            v3
        }
    }
}
fn method37(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let v2: Rc<str> = v2;
                let v3: i32 = v3;
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
fn round_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_1(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let mut v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH0::UH0_Cons(v8, v9) => {
                                    let v8: Rc<str> = v8;
                                    let mut v9: Rc<UH0> = v9;
                                    let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                    v10.borrow_mut().push(v8);
                                    let v11: i32 = 1i32;
                                    let v12: i32 = method4(v10.clone(), v9, v11);
                                    let v13: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
                                    let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v13).as_ref().clone()));
                                    let v15: Rc<UH1> = totals_of_5(v14);
                                    let v16: Rc<str> = config_path_13();
                                    let v17: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v16).unwrap_or_default().as_str());
                                    let v18: Rc<UH1> = previous_of_15(v17.clone(), v6.clone());
                                    let v19: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                                    let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v21: Rc<UH0> = method27(v18, v15.clone(), v20);
                                    written_32(v21);
                                    let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v23: Rc<UH0> = method33(v6.clone(), v19, v15.clone(), v22);
                                    let v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                    let v25: i32 = 0i32;
                                    let v26: i32 = method4(v24.clone(), v23, v25);
                                    let v27: Rc<Vec<Rc<str>>> = Rc::new(v24.borrow().clone());
                                    let v28: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v27).as_ref().clone()));
                                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v30: Rc<str> = Rc::<str>::from((*v28.borrow()).join(&*v29));
                                    let mut v31: Rc<str> = string_concat(&[&*v17, &*v30]);
                                    std::fs::write(&*v16, &*v31).unwrap();
                                    let v33: i32 = 0i32;
                                    let v34: i32 = method37(v15, v33);
                                    let v35: Rc<str> = format_real_30(v34);
                                    let v40: Rc<str> = string_concat(&["rig standards round ok repo=", &*v6, " words=", &*v35, " file=", &*v16]);
                                    println!("{}", v40);
                                    0i32
                                }
                                _ => {
                                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: standards --round <repo-label> <dir>..."); } LIT.with(|lit| lit.clone()) };
                                    eprintln!("{}", v41);
                                    2i32
                                }
                            }
                        }
                        _ => {
                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: standards --round <repo-label> <dir>..."); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v43);
                            2i32
                        }
                    }
                }
                _ => {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: standards --round <repo-label> <dir>..."); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v45);
                    2i32
                }
            }
        }
        _ => {
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: standards --round <repo-label> <dir>..."); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v47);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        round_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_granular_round_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
