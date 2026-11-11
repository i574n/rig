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
enum US0 {
    US0_RustTarget,
    US0_GleamTarget,
    US0_ZigTarget,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, i32, Rc<str>, Rc<UH1>),
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_CrateWithin,
    US2_CrateOver,
    US2_CrateUnbuilt,
}
#[derive(Clone)]
enum US3 {
    US3_GatePassed,
    US3_GateFailedOver(i32),
    US3_GateFailedMissing(i32),
    US3_GateFailedVacuous,
}
#[derive(Clone)]
enum US4 {
    US4_Some(US0),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_Some(Rc<str>),
    US5_None,
}
#[derive(Clone)]
enum US6 {
    US6_FlagEnforce,
    US6_FlagLimit,
    US6_FlagBackend,
}
fn get_environment_variable_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn target_limits_text_3() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = get_environment_variable_4(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("APPDATA"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = get_environment_variable_4(v2);
    let v4: bool = &*v1 != "";
    let v11: Rc<str> = if v4 {
        v1
    } else {
        let v5: bool = &*v3 != "";
        if v5 {
            let mut v6: Rc<str> = string_concat(&[&*v3, "/kit"]);
            v6
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HOME"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = get_environment_variable_4(v7);
            let mut v9: Rc<str> = string_concat(&[&*v8, "/.config/kit"]);
            v9
        }
    };
    let v12: Rc<str> = string_concat(&[&*v11, "/target_limits.spi"]);
    let v13: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v12).unwrap_or_default().as_str());
    v13
}
fn method7(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
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
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
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
fn method10(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure2() -> Rc<dyn Fn((i32)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US1> = Rc::new(move |mut v0: (i32)| -> US1 {
        let mut v1: i32 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method6(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: Option<usize> = v3.find(&*v0);
                let mut v6: Option<usize> = method7(v5);
                let mut v7: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v8: Option<i32> = v6.map(|x| v7(x));
                let v10: i32 = v8.unwrap_or(-1i32);
                let v11: bool = v10 < 0i32;
                let v44: i32 = if v11 {
                    v2
                } else {
                    let v12: i32 = (v0.clone().len() as i32);
                    let v13: i32 = v10.wrapping_add(v12);
                    let v14: i32 = (v3.clone().len() as i32);
                    let v15: i32 = v14.wrapping_sub(v10);
                    let v16: i32 = v15.wrapping_sub(v12);
                    let v17: usize = ((v13) as usize);
                    let v18: i32 = v13.wrapping_add(v16);
                    let v19: usize = ((v18) as usize);
                    let v20: Rc<str> = Rc::<str>::from(&(v3)[v17..v19]);
                    let v21: i32 = (v20.clone().len() as i32);
                    let v22: i32 = 0i32;
                    let v23: i32 = method8(v20.clone(), v21, v22);
                    let v24: i32 = v21.wrapping_sub(1i32);
                    let v25: Rc<str> = string_slice(&v20, v23 as i64, v24 as i64);
                    let v26: i32 = (v25.clone().len() as i32);
                    let v27: i32 = method9(v25.clone(), v26);
                    let v28: Rc<str> = string_slice(&v25, 0i32 as i64, v27 as i64);
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32)"); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v31: Rc<str> = Rc::<str>::from(v28.replace(&*v29, &*v30));
                    let v32: Option<i32> = (v31).trim().parse::<i32>().ok();
                    let v33: Option<i32> = method10(v32);
                    let mut v34: Rc<dyn Fn((i32)) -> US1> = closure2();
                    let mut v35: Option<US1> = v33.map(|x| v34(x));
                    let v37: US1 = US1::US1_None;
                    let v38: US1 = v37.clone();
                    let v39: US1 = v35.unwrap_or(v38);
                    match v39 {
                        US1::US1_Some(v40) => {
                            let v40: i32 = v40;
                            let v41: bool = v40 > 0i32;
                            if v41 {
                                v40
                            } else {
                                v2
                            }
                        }
                        _ => {
                            v2
                        }
                    }
                };
                (v1, v2) = (v4, v44);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method13(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
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
fn target_limit_of_5(mut v0: Rc<str>, mut v1: US0) -> i32 {
    let mut v7: Rc<str> = match v1 {
        US0::US0_GleamTarget => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_RustTarget => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_ZigTarget => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
            v4
        }
    };
    let mut v9: Rc<str> = string_concat(&[" = TargetLimit (\"", &*v7, "\", "]);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(v0.replace(&*v11, &*v12));
    let v14: Rc<UH0> = split_list_11(v10, v13);
    let v15: i32 = 1000i32;
    method6(v9, v14, v15)
}
fn method15(mut v0: Option<char>) -> Option<char> {
    v0
}
fn closure3() -> Rc<dyn Fn((char)) -> u32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> u32> = Rc::new(move |mut v0: (char)| -> u32 {
        let mut v1: char = (v0);
        let mut v2: u32 = u32::from(v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn digits_value_14(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: usize = ((v1) as usize);
            let mut v6: Option<char> = (&*v0)[v5..].chars().next();
            let mut v7: Option<char> = method15(v6);
            let mut v8: Rc<dyn Fn((char)) -> u32> = closure3();
            let mut v9: Option<u32> = v7.map(|x| v8(x));
            let v11: u32 = v9.unwrap_or(0u32);
            let v12: bool = 48u32 <= v11;
            let v14: bool = if v12 {
                let v13: bool = v11 <= 57u32;
                v13
            } else {
                false
            };
            if v14 {
                let v15: i32 = v1.wrapping_add(1i32);
                let v16: i32 = v2.wrapping_mul(10i32);
                let v17: u32 = v11.wrapping_sub(48u32);
                let v18: i32 = (v17 as i32);
                let v19: i32 = v16.wrapping_add(v18);
                (v1, v2) = (v15, v19);
                continue;
            } else {
                return -1i32;
            }
        }
    }
}
fn directory_exists_16(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn rows_from_18(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let (mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, i32, Rc<str>) = v0.borrow()[v1 as usize].clone();
            let mut v9: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v6, v7, v8, v2));
            (v1, v2) = (v4, v9);
            continue;
        }
    }
}
fn rows_listed_17(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>>) -> Rc<UH1> {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    rows_from_18(v0, v2, v3)
}
fn method22(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_21(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method22(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn verdict_of_24(mut v0: i32, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: i32, mut v4: &Rc<str>) -> US2 {
    let v5: bool = v3 < 0i32;
    if v5 {
        US2::US2_CrateUnbuilt
    } else {
        let mut v7: bool = v3 > v0;
        if v7 {
            US2::US2_CrateOver
        } else {
            US2::US2_CrateWithin
        }
    }
}
fn verdict_text_23(mut v0: i32, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: i32, mut v4: &Rc<str>) -> Rc<str> {
    let v5: US2 = verdict_of_24(v0, v1, v2, v3, v4);
    match v5 {
        US2::US2_CrateOver => {
            let mut v7: Rc<str> = format_real_21(v0);
            let mut v8: Rc<str> = string_concat(&["OVER ", &*v7]);
            v8
        }
        US2::US2_CrateUnbuilt => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no generated output yet"); } LIT.with(|lit| lit.clone()) };
            v9
        }
        US2::US2_CrateWithin => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v6
        }
    }
}
fn measure_line_20(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    let mut v9: Rc<str> = format_real_21(v3);
    let v11: Rc<str> = string_concat(&["gate / ", &*v1, " ", &*v2, " ", &*v9, " lines "]);
    let v12: Rc<str> = verdict_text_23(v0, &v1, &v2, v3, &v4);
    let v14: Rc<str> = string_concat(&[&*v11, &*v12, &*v4]);
    v14
}
fn lines_printed_19(mut v0: i32, mut v1: Rc<UH1>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: i32 = v4;
                let v5: Rc<str> = v5;
                let mut v6: Rc<UH1> = v6;
                let mut v7: Rc<str> = measure_line_20(v0, v2, v3, v4, v5);
                println!("{}", v7);
                v1 = v6;
                continue;
            }
            UH1::UH1_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method26(mut v0: i32, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: i32 = v5;
                let mut v6: Rc<str> = v6;
                let mut v7: Rc<UH1> = v7;
                let mut v8: US2 = verdict_of_24(v0, &v3, &v4, v5, &v6);
                let v10: i32 = match v8 {
                    US2::US2_CrateOver => {
                        let mut v9: i32 = v2.wrapping_add(1i32);
                        v9
                    }
                    _ => {
                        v2
                    }
                };
                (v1, v2) = (v7, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn over_count_25(mut v0: i32, mut v1: Rc<UH1>) -> i32 {
    let mut v2: i32 = 0i32;
    method26(v0, v1, v2)
}
fn method27(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: i32 = v4;
                let v5: Rc<str> = v5;
                let mut v6: Rc<UH1> = v6;
                let mut v7: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method30(mut v0: i32, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: i32 = v5;
                let mut v6: Rc<str> = v6;
                let mut v7: Rc<UH1> = v7;
                let mut v8: US2 = verdict_of_24(v0, &v3, &v4, v5, &v6);
                let v10: i32 = match v8 {
                    US2::US2_CrateUnbuilt => {
                        let mut v9: i32 = v2.wrapping_add(1i32);
                        v9
                    }
                    _ => {
                        v2
                    }
                };
                (v1, v2) = (v7, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn missing_count_29(mut v0: i32, mut v1: Rc<UH1>) -> i32 {
    let mut v2: i32 = 0i32;
    method30(v0, v1, v2)
}
fn gate_outcome_of_28(mut v0: bool, mut v1: i32, mut v2: Rc<UH1>) -> US3 {
    let v3: i32 = over_count_25(v1, v2.clone());
    let v4: i32 = missing_count_29(v1, v2.clone());
    let mut v8: bool = if v0 {
        let v5: i32 = 0i32;
        let mut v6: i32 = method27(v2, v5);
        let mut v7: bool = v6 == 0i32;
        v7
    } else {
        false
    };
    if v8 {
        US3::US3_GateFailedVacuous
    } else {
        let v11: bool = if v0 {
            let v10: bool = v3 > 0i32;
            v10
        } else {
            false
        };
        if v11 {
            US3::US3_GateFailedOver(v3)
        } else {
            let v14: bool = if v0 {
                let v13: bool = v4 > 0i32;
                v13
            } else {
                false
            };
            if v14 {
                US3::US3_GateFailedMissing(v4)
            } else {
                US3::US3_GatePassed
            }
        }
    }
}
fn outcome_text_31(mut v0: US0, mut v1: US3, mut v2: Rc<str>) -> (i32, Rc<str>) {
    match v1 {
        US3::US3_GateFailedMissing(v9) => {
            let mut v9: i32 = v9;
            let v10: Rc<str> = format_real_21(v9);
            let v13: Rc<str> = string_concat(&["rig crate gate refused: ", &*v10, " crate(s) have no generated output, so generation did not run or failed;", &*v2]);
            (1i32, v13)
        }
        US3::US3_GateFailedOver(v4) => {
            let v4: i32 = v4;
            let v5: Rc<str> = format_real_21(v4);
            let mut v8: Rc<str> = string_concat(&["rig crate gate refused: ", &*v5, " crate(s) over the limit;", &*v2]);
            (1i32, v8)
        }
        US3::US3_GateFailedVacuous => {
            let v21: Rc<str> = match &v0 {
                US0::US0_RustTarget => {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no crate with [package.metadata.spiral] under the root"); } LIT.with(|lit| lit.clone()) };
                    v14
                }
                _ => {
                    let v18: Rc<str> = match v0 {
                        US0::US0_GleamTarget => {
                            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".gleam"); } LIT.with(|lit| lit.clone()) };
                            v15
                        }
                        US0::US0_ZigTarget => {
                            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".zig"); } LIT.with(|lit| lit.clone()) };
                            v16
                        }
                        _ => unreachable!(),
                    };
                    let v20: Rc<str> = string_concat(&["no ", &*v18, " file under the root"]);
                    v20
                }
            };
            let v24: Rc<str> = string_concat(&["rig crate gate refused: ", &*v21, ", so nothing was measured;", &*v2]);
            (1i32, v24)
        }
        US3::US3_GatePassed => {
            let v3: Rc<str> = string_concat(&["rig crate gate ok", &*v2]);
            (0i32, v3)
        }
    }
}
fn gate_checked_2(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US0) -> i32 {
    let v5: bool = &*v2 == "";
    let v11: i32 = if v5 {
        let mut v6: Rc<str> = target_limits_text_3();
        target_limit_of_5(v6, v4.clone())
    } else {
        let mut v8: i32 = 0i32;
        let mut v9: i32 = 0i32;
        digits_value_14(v2.clone(), v8, v9)
    };
    let v12: bool = &*v0 == "";
    let v15: bool = if v12 {
        true
    } else {
        let v13: bool = directory_exists_16(v0.clone());
        let v14: bool = v13 == false;
        v14
    };
    if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: crate usage: crate gate <root> [--enforce] [--limit n] [--backend rust|gleam|zig] (rust: generated output of every Cargo.toml with [package.metadata.spiral] under root; gleam and zig: every .gleam or .zig file under root)"); } LIT.with(|lit| lit.clone()) };
        println!("{}", v16);
        2i32
    } else {
        let v17: bool = v11 <= 0i32;
        if v17 {
            let v18: Rc<str> = string_concat(&["rig error: crate gate: --limit expects a positive whole number, got ", &*v2]);
            println!("{}", v18);
            2i32
        } else {
            let v27: Rc<UH1> = match &v4 {
                US0::US0_RustTarget => {
                    let v19: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>> = rig_crate_walk::rig_crate_measures(&v0);
                    rows_listed_17(v19)
                }
                _ => {
                    let v24: Rc<str> = match &v4 {
                        US0::US0_GleamTarget => {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".gleam"); } LIT.with(|lit| lit.clone()) };
                            v21
                        }
                        US0::US0_ZigTarget => {
                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".zig"); } LIT.with(|lit| lit.clone()) };
                            v22
                        }
                        _ => unreachable!(),
                    };
                    let v25: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>> = rig_crate_walk::rig_output_measures(&v0, &v24);
                    rows_listed_17(v25)
                }
            };
            lines_printed_19(v11, v27.clone());
            let v28: i32 = over_count_25(v11, v27.clone());
            let v29: i32 = 0i32;
            let v30: i32 = method27(v27.clone(), v29);
            let v31: Rc<str> = format_real_21(v30);
            let mut v34: Rc<str> = format_real_21(v28);
            let v37: Rc<str> = format_real_21(v11);
            let v39: Rc<str> = string_concat(&[" crates=", &*v31, " over=", &*v34, " limit=", &*v37, " backend="]);
            let v45: Rc<str> = match &v4 {
                US0::US0_GleamTarget => {
                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
                    v41
                }
                US0::US0_RustTarget => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust"); } LIT.with(|lit| lit.clone()) };
                    v40
                }
                US0::US0_ZigTarget => {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
                    v42
                }
            };
            let v48: Rc<str> = string_concat(&[&*v39, &*v45, " root=", &*v0]);
            let v49: US3 = gate_outcome_of_28(v1, v11, v27);
            let (mut v50, mut v51): (i32, Rc<str>) = outcome_text_31(v4, v49, v48);
            println!("{}", v51);
            v50
        }
    }
}
fn backend_named_32(mut v0: Rc<str>) -> US4 {
    let mut v1: bool = &*v0 == "rust";
    if v1 {
        let mut v2: US0 = US0::US0_RustTarget;
        US4::US4_Some(v2)
    } else {
        let v4: bool = &*v0 == "gleam";
        if v4 {
            let v5: US0 = US0::US0_GleamTarget;
            US4::US4_Some(v5)
        } else {
            let mut v7: bool = &*v0 == "zig";
            if v7 {
                let mut v8: US0 = US0::US0_ZigTarget;
                US4::US4_Some(v8)
            } else {
                US4::US4_None
            }
        }
    }
}
fn gate_started_1(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let v4: bool = &*v3 == "";
    let mut v6: Rc<str> = if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust"); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        v3.clone()
    };
    let mut v7: US4 = backend_named_32(v6);
    match v7 {
        US4::US4_None => {
            let v10: Rc<str> = string_concat(&["rig error: crate gate: --backend expects rust, gleam or zig, got ", &*v3]);
            println!("{}", v10);
            2i32
        }
        US4::US4_Some(v8) => {
            let mut v8: US0 = v8;
            gate_checked_2(v0, v1, v2, v3, v8)
        }
    }
}
fn command_line_argument_count_34() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_35(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_33(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_34();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US5 = if v4 {
        let v5: Rc<str> = command_line_argument_35(v0);
        US5::US5_Some(v5)
    } else {
        US5::US5_None
    };
    match v8 {
        US5::US5_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US5::US5_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_33(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn flag_word_37(mut v0: US6) -> Rc<str> {
    let v4: Rc<str> = match v0 {
        US6::US6_FlagBackend => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backend"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US6::US6_FlagEnforce => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("enforce"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US6::US6_FlagLimit => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("limit"); } LIT.with(|lit| lit.clone()) };
            v2
        }
    };
    let v5: Rc<str> = string_concat(&["--", &*v4]);
    v5
}
fn gate_flags_read_36(mut v0: Rc<UH0>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> (bool, Rc<str>, Rc<str>) {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let mut v6: US6 = US6::US6_FlagEnforce;
                let mut v7: Rc<str> = flag_word_37(v6);
                let mut v8: bool = &*v4 == &*v7;
                if v8 {
                    let mut v9: bool = true;
                    (v0, v1) = (v5, v9);
                    continue;
                } else {
                    match &*v5 {
                        UH0::UH0_Cons(v13, v14) => {
                            let v13: Rc<str> = v13.clone();
                            let v14: Rc<UH0> = v14.clone();
                            let v15: US6 = US6::US6_FlagLimit;
                            let v16: Rc<str> = flag_word_37(v15);
                            let v17: bool = &*v4 == &*v16;
                            if v17 {
                                (v0, v2) = (v14, v13);
                                continue;
                            } else {
                                let v21: US6 = US6::US6_FlagBackend;
                                let v22: Rc<str> = flag_word_37(v21);
                                let v23: bool = &*v4 == &*v22;
                                if v23 {
                                    (v0, v3) = (v14, v13);
                                    continue;
                                } else {
                                    v0 = v5;
                                    continue;
                                }
                            }
                        }
                        _ => {
                            v0 = v5;
                            continue;
                        }
                    }
                }
            }
            _ => {
                let (mut v45, mut v46, mut v47): (bool, Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v0) {
                    UH0::UH0_Nil => {
                        (v1, v2, v3)
                    }
                    _ => unreachable!(),
                };
                return (v45, v46, v47);
            }
        }
    }
}
fn gate_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_33(v0);
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
                            let mut v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            let mut v8: bool = false;
                            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let (mut v10, mut v11, mut v12): (bool, Rc<str>, Rc<str>) = gate_flags_read_36(v7, v8, v9.clone(), v9);
                            gate_started_1(v6, v10, v11, v12)
                        }
                        _ => {
                            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v15: bool = false;
                            gate_started_1(v14.clone(), v15, v14.clone(), v14)
                        }
                    }
                }
                _ => {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v19: bool = false;
                    gate_started_1(v18.clone(), v19, v18.clone(), v18)
                }
            }
        }
        _ => {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v23: bool = false;
            gate_started_1(v22.clone(), v23, v22.clone(), v22)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        gate_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_crate_gate_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
