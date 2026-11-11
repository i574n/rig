#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
enum US1 {
    US1_Ok(Vec<std::string::String>),
    US1_Error(std::io::Error),
}
fn joined_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn text_or_empty_3(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method5(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_4(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method5(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US0> = closure1();
    let mut v4: Option<US0> = v2.map(|x| v3(x));
    let mut v6: US0 = US0::US0_None;
    let mut v7: US0 = v6.clone();
    let v8: US0 = v4.unwrap_or(v7);
    match v8 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v9) => {
            let mut v9: u64 = v9;
            v9
        }
    }
}
fn process_alive_6(mut v0: u64) -> bool {
    let v1: bool = rig_process_alive::rig_process_alive(v0);
    v1
}
fn counted_live_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool, mut v3: Rc<str>) -> u64 {
    let mut v4: bool = &*v1 == &*v0;
    let mut v6: bool = if v4 {
        true
    } else {
        let v5: bool = v2 == false;
        v5
    };
    if v6 {
        0u64
    } else {
        let mut v7: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        let v8: Option<u64> = (v7).trim().parse::<u64>().ok();
        let mut v9: Option<u64> = method5(v8);
        let mut v10: Rc<dyn Fn((u64)) -> US0> = closure1();
        let mut v11: Option<US0> = v9.map(|x| v10(x));
        let v13: US0 = US0::US0_None;
        let v14: US0 = v13.clone();
        let v15: US0 = v11.unwrap_or(v14);
        match v15 {
            US0::US0_None => {
                0u64
            }
            US0::US0_Some(v16) => {
                let v16: u64 = v16;
                v16
            }
        }
    }
}
fn live_elsewhere_1(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: u64) -> u64 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = joined_2(v3.clone(), v5);
                let mut v7: Rc<str> = text_or_empty_3(v6);
                let v8: u64 = decimal_u64_4(v7);
                let mut v9: bool = v8 != 0u64;
                let mut v11: bool = if v9 {
                    process_alive_6(v8)
                } else {
                    false
                };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live"); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = joined_2(v3.clone(), v12);
                let v14: Rc<str> = text_or_empty_3(v13);
                let v15: u64 = counted_live_7(v0.clone(), v3, v11, v14);
                let v16: u64 = v2.wrapping_add(v15);
                (v1, v2) = (v4, v16);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn jobs_root_8() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_JOBS_HOME").or_else(|| std::env::var_os("KIT_JOBS_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn ok_c2() -> Rc<dyn Fn(Vec<std::string::String>) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<std::string::String>) -> US1> = Rc::new(move |mut v0: Vec<std::string::String>| -> US1 {
        US1::US1_Ok(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9() -> Rc<dyn Fn(Vec<std::string::String>) -> US1> {
    ok_c2()
}
fn error_c3() -> Rc<dyn Fn(std::io::Error) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US1> = Rc::new(move |mut v0: std::io::Error| -> US1 {
        US1::US1_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10() -> Rc<dyn Fn(std::io::Error) -> US1> {
    error_c3()
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn method12(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method12(v0.clone(), v4, v2);
            let mut v6: Rc<str> = joined_2(v0, v3);
            Rc::new(UH0::UH0_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn live_elsewhere_of_0(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = jobs_root_8();
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hosts"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = joined_2(v1, v2);
    let v5: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v3).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v6: Rc<dyn Fn(Vec<std::string::String>) -> US1> = method9();
    let mut v7: Rc<dyn Fn(std::io::Error) -> US1> = method10();
    let v8: US1 = match v5 { Ok(x) => v6(x), Err(e) => v7(e) };
    let v29: Rc<UH0> = match v8 {
        US1::US1_Error(v26) => {
            let v26: std::io::Error = v26;
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US1::US1_Ok(v9) => {
            let mut v9: Vec<std::string::String> = v9;
            let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v12: bool = true; let _vec_fold_ = v9.into_iter().fold(v10, |acc, x| { //;
            let v14: Rc<UH0> = acc;
            let v16: std::string::String = x;
            let v18: Rc<str> = Rc::<str>::from(String::as_str(&v16));
            let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v14));
            let v21: bool = true; v20 });
            let v23: Rc<UH0> = _vec_fold_;
            let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            method11(v23, v24)
        }
    };
    let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v31: Rc<UH0> = method12(v3, v29, v30);
    let v32: Rc<str> = jobs_root_8();
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shell-host"); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<str> = joined_2(v32, v33);
    let v35: Rc<UH0> = Rc::new(UH0::UH0_Cons(v34, v31));
    let v36: u64 = 0u64;
    live_elsewhere_1(v0, v35, v36)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        live_elsewhere_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_job_live_elsewhere(v0: &str) -> u64 {
    closure0()(Rc::<str>::from(v0))
}
