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
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, u64, i64, Rc<UH1>),
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
fn flag_value_2(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let mut v4: Rc<str> = v4.clone();
                        let mut v5: Rc<UH0> = v5.clone();
                        let mut v6: bool = &*v2 == &*v1;
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
fn method4(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn days_of_3(mut v0: Rc<str>, mut v1: i32, mut v2: i64) -> i64 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 < v3;
        let v16: i32 = if v4 {
            let mut v5: usize = ((v1) as usize);
            let mut v6: i32 = v1.wrapping_add(1i32);
            let mut v7: usize = ((v6) as usize);
            let mut v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) };
            let v10: Option<usize> = v9.find(&*v8);
            let v11: Option<usize> = method4(v10);
            let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v13: Option<i32> = v11.map(|x| v12(x));
            let v15: i32 = v13.unwrap_or(-1i32);
            v15
        } else {
            -1i32
        };
        let mut v17: bool = v16 < 0i32;
        if v17 {
            return v2;
        } else {
            let v18: i32 = v1.wrapping_add(1i32);
            let v19: i64 = v2.wrapping_mul(10i64);
            let v20: i64 = (v16 as i64);
            let v21: i64 = v19.wrapping_add(v20);
            (v1, v2) = (v18, v21);
            continue;
        }
    }
}
fn method6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i64, mut v3: Rc<UH1>, mut v4: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v3) {
        UH1::UH1_Cons(v5, v6, v7, v8, v9, v10) => {
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<str> = v6;
            let mut v7: Rc<str> = v7;
            let mut v8: u64 = v8;
            let v9: i64 = v9;
            let v10: Rc<UH1> = v10;
            let v11: Rc<UH1> = method6(v0.clone(), v1.clone(), v2, v10, v4);
            let mut v12: bool = &*v0 == "";
            let v14: bool = if v12 {
                true
            } else {
                let mut v13: bool = &*v0 == &*v5;
                v13
            };
            let v15: bool = &*v1 == "";
            let mut v17: bool = if v15 {
                true
            } else {
                let v16: bool = &*v1 == &*v6;
                v16
            };
            let v18: bool = v14 && v17;
            let v20: bool = if v18 {
                let v19: bool = v9 >= v2;
                v19
            } else {
                false
            };
            if v20 {
                Rc::new(UH1::UH1_Cons(v5, v6, v7, v8, v9, v11))
            } else {
                v11
            }
        }
        UH1::UH1_Nil => {
            v4
        }
    }
}
fn get_environment_variable_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn slash_9(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    v3
}
fn store_root_7() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = get_environment_variable_8(v0);
    let v2: Rc<str> = slash_9(v1);
    let v3: Rc<str> = string_concat(&[&*v2, "/kit/drafts"]);
    v3
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn directory_exists_12(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn names_in_10(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: bool = directory_exists_12(v0.clone());
    let v2: bool = v1 == false;
    if v2 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v5: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v7: Vec<std::string::String> = v5.unwrap();
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v10: bool = true; let _vec_fold_ = v7.into_iter().fold(v8, |acc, x| { //;
        let mut v12: Rc<UH0> = acc;
        let v14: std::string::String = x;
        let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
        let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v12));
        let v19: bool = true; v18 });
        let v21: Rc<UH0> = _vec_fold_;
        let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method11(v21, v22)
    }
}
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method17(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v7: Rc<str> = string_concat(&[&*v0, "/", &*v4]);
                let mut v8: bool = directory_exists_12(v7.clone());
                let v16: Rc<UH0> = if v8 {
                    let v10: Rc<str> = string_concat(&[&*v1, &*v4, "/"]);
                    files_under_15(v7, v10, v3)
                } else {
                    let mut v12: Rc<str> = string_concat(&[&*v1, &*v4]);
                    let mut v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v13));
                    method17(v3, v14)
                };
                (v2, v3) = (v5, v16);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn files_under_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> Rc<UH0> {
    let v3: Rc<UH0> = names_in_10(v0.clone());
    method16(v0, v1, v3, v2)
}
fn last_write_ms_20(mut v0: Rc<str>) -> u64 {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v0);
    let v2: bool = v1.is_ok();
    let v3: bool = v2 == false;
    if v3 {
        0u64
    } else {
        let mut v4: std::fs::Metadata = v1.unwrap();
        let mut v5: std::io::Result<std::time::SystemTime> = v4.modified();
        let mut v6: bool = v5.is_ok();
        if v6 {
            let mut v7: std::time::SystemTime = v5.unwrap();
            let mut v8: std::time::Duration = v7.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
            let v9: u64 = v8.as_secs();
            let v10: u64 = v9.wrapping_mul(1000u64);
            let v11: u32 = v8.subsec_millis();
            let mut v12: u64 = (v11 as u64);
            let mut v13: u64 = v10.wrapping_add(v12);
            v13
        } else {
            0u64
        }
    }
}
fn row_of_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i64) {
    let mut v4: Rc<str> = store_root_7();
    let v10: Rc<str> = string_concat(&[&*v4, "/", &*v0, "/", &*v1, "/", &*v2]);
    let v11: u64 = last_write_ms_20(v10.clone());
    let mut v12: bool = v3 > v11;
    let v16: i64 = if v12 {
        let mut v13: u64 = v3.wrapping_sub(v11);
        let v14: u64 = v13.wrapping_div(86400000u64);
        let v15: i64 = (v14 as i64);
        v15
    } else {
        0i64
    };
    let mut v17: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v10);
    let v18: bool = v17.is_ok();
    let v19: bool = v18 == false;
    let v23: i64 = if v19 {
        0i64
    } else {
        let v20: std::fs::Metadata = v17.unwrap();
        let v21: u64 = v20.len();
        let v22: i64 = (v21 as i64);
        v22
    };
    let v24: u64 = (v23 as u64);
    (v0, v1, v2, v24, v16)
}
fn method18(mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v3) {
        UH0::UH0_Cons(v5, v6) => {
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<UH0> = v6;
            let mut v7: Rc<UH1> = method18(v0, v1.clone(), v2.clone(), v6, v4);
            let (mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<str>, Rc<str>, Rc<str>, u64, i64) = row_of_19(v1, v2, v5, v0);
            Rc::new(UH1::UH1_Cons(v8, v9, v10, v11, v12, v7))
        }
        UH0::UH0_Nil => {
            v4
        }
    }
}
fn method21(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5, v6, v7) => {
            let v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let mut v5: u64 = v5;
            let mut v6: i64 = v6;
            let mut v7: Rc<UH1> = v7;
            let mut v8: Rc<UH1> = method21(v7, v1);
            Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v6, v8))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method14(mut v0: u64, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: Rc<str> = store_root_7();
                let v10: Rc<str> = string_concat(&[&*v6, "/", &*v1, "/", &*v4]);
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v13: Rc<UH0> = files_under_15(v10, v11, v12);
                let v14: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v15: Rc<UH1> = method18(v0, v1.clone(), v4, v13, v14);
                let v16: Rc<UH1> = method21(v3, v15);
                (v2, v3) = (v5, v16);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn method13(mut v0: u64, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: Rc<str> = store_root_7();
                let mut v7: Rc<str> = string_concat(&[&*v5, "/", &*v3]);
                let mut v8: Rc<UH0> = names_in_10(v7);
                let v9: Rc<UH1> = method14(v0, v3, v8, v2);
                (v1, v2) = (v4, v9);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn rows_of_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i64) -> Rc<UH1> {
    let v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v4: Rc<str> = store_root_7();
    let mut v5: Rc<UH0> = names_in_10(v4);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH1> = method13(v3, v5, v6);
    let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method6(v0, v1, v2, v7, v8)
}
fn method24(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_23(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method24(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_25(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method24(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method22(mut v0: Rc<UH1>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v1, v2, v3, v4, v5, v6) => {
                let mut v1: Rc<str> = v1;
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let mut v4: u64 = v4;
                let mut v5: i64 = v5;
                let mut v6: Rc<UH1> = v6;
                let mut v12: Rc<str> = format_real_23(v4);
                let v15: Rc<str> = format_real_25(v5);
                let mut v17: Rc<str> = string_concat(&[&*v1, "  ", &*v2, "  ", &*v3, "  ", &*v12, "  ", &*v15, "d"]);
                println!("{}", v17);
                v0 = v6;
                continue;
            }
            UH1::UH1_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method26(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let mut v5: u64 = v5;
                let mut v6: i64 = v6;
                let mut v7: Rc<UH1> = v7;
                let mut v8: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v7, v8);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn format_real_27(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method24(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn list_run_1(mut v0: Rc<UH0>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--older-than"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = flag_value_2(v0.clone(), v1);
    let v3: i32 = 0i32;
    let mut v4: i64 = 0i64;
    let mut v5: i64 = days_of_3(v2, v3, v4);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--repo"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = flag_value_2(v0.clone(), v6);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--item"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = flag_value_2(v0, v8);
    let v10: Rc<UH1> = rows_of_5(v7, v9, v5);
    method22(v10.clone());
    let v11: i32 = 0i32;
    let mut v12: i32 = method26(v10, v11);
    let mut v13: Rc<str> = format_real_27(v12);
    let v14: Rc<str> = string_concat(&["rig drafts list ok drafts=", &*v13]);
    println!("{}", v14);
    0i32
}
fn command_line_argument_count_30() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_31(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_29(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_30();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US0 = if v4 {
        let mut v5: Rc<str> = command_line_argument_31(v0);
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
            let mut v12: Rc<UH0> = arguments_from_29(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn verb_words_28() -> Rc<UH0> {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_29(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let mut v5: Rc<UH0> = v5;
                    v5
                }
                _ => {
                    { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                }
            }
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn list_entry_0() -> i32 {
    let mut v0: Rc<UH0> = verb_words_28();
    list_run_1(v0)
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        list_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_drafts_list_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
