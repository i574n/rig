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
enum US0 {
    US0_Box,
    US0_Node,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US3 {
    US3_Rust,
    US3_Gleam,
    US3_Zig,
    US3_CBackend,
    US3_Lua,
    US3_Wasm,
    US3_Fsharp,
}
#[derive(Clone)]
enum US4 {
    US4_Compile,
    US4_Rustc,
    US4_Run,
    US4_SlotWait,
    US4_DotnetStartup,
    US4_Mount,
    US4_LockWait,
}
#[derive(Clone)]
enum US2 {
    US2_Duration(US3, US4, u64),
    US2_JobDuration(Rc<str>, Rc<str>, u64),
}
#[derive(Clone)]
enum US5 {
    US5_PeakBytes(Rc<str>, u64),
    US5_JobPeak(Rc<str>, Rc<str>, u64),
}
#[derive(Clone)]
enum US6 {
    US6_Count(Rc<str>, u64),
}
#[derive(Clone)]
enum US1 {
    US1_TimedSample(US0, US2),
    US1_SizedSample(US0, US5),
    US1_CountedSample(US0, US6),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US1, Rc<UH1>),
}
#[derive(Clone)]
enum US7 {
    US7_Some(u64),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_Some(US1),
    US8_None,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
fn joined_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_7() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_JOBS_HOME").or_else(|| std::env::var_os("KIT_JOBS_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_7();
    joined_5(v1, v0)
}
fn job_file_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_6(v0);
    joined_5(v2, v1)
}
fn text_or_empty_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method10(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US7> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US7> = Rc::new(move |mut v0: (u64)| -> US7 {
        let v1: u64 = (v0);
        US7::US7_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_9(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method10(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US7> = closure1();
    let mut v4: Option<US7> = v2.map(|x| v3(x));
    let mut v6: US7 = US7::US7_None;
    let mut v7: US7 = v6.clone();
    let mut v8: US7 = v4.unwrap_or(v7);
    match v8 {
        US7::US7_None => {
            0u64
        }
        US7::US7_Some(v9) => {
            let mut v9: u64 = v9;
            v9
        }
    }
}
fn now_ms_11() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_3(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_4(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_8(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = job_file_4(v0.clone(), v4);
    let mut v6: Rc<str> = text_or_empty_8(v5);
    let mut v7: u64 = decimal_u64_9(v6);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = job_file_4(v0, v8);
    let v10: Rc<str> = text_or_empty_8(v9);
    let v11: u64 = decimal_u64_9(v10);
    let v12: u64 = now_ms_11();
    (v3, v7, v11, v12)
}
fn elapsed_of_12(mut v0: Rc<str>) -> u64 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_3(v0);
    let v5: bool = &*v1 == "";
    let mut v8: u64 = if v5 {
        v4
    } else {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.split(char::from(9u8)).nth(1).unwrap_or("0"));
        decimal_u64_9(v6)
    };
    let mut v9: bool = v8 > v3;
    if v9 {
        let v10: u64 = v8.wrapping_sub(v3);
        v10
    } else {
        0u64
    }
}
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method17(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method17(v7, &v9) {
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
        method16(v16, v17)
    }
}
fn lane_of_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH0> = split_list_15(v1, v0.clone());
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH0::UH0_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let mut v6: Rc<UH0> = v6;
                    v3
                }
                _ => {
                    v0
                }
            }
        }
        _ => {
            v0
        }
    }
}
fn job_sample_13(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: bool) -> US8 {
    let mut v6: bool = if v4 {
        let v5: bool = v3 > 0u64;
        v5
    } else {
        false
    };
    if v6 {
        let mut v7: Rc<str> = lane_of_14(v1);
        let mut v8: US2 = US2::US2_JobDuration(v7, v2, v3);
        let mut v9: US1 = US1::US1_TimedSample(v0, v8);
        US8::US8_Some(v9)
    } else {
        US8::US8_None
    }
}
fn peak_mb_of_18(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("peak_private_mb"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_4(v0, v1);
    let mut v3: Rc<str> = text_or_empty_8(v2);
    let mut v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v5: Option<u64> = (v4).trim().parse::<u64>().ok();
    let mut v6: Option<u64> = method10(v5);
    let mut v7: Rc<dyn Fn((u64)) -> US7> = closure1();
    let mut v8: Option<US7> = v6.map(|x| v7(x));
    let v10: US7 = US7::US7_None;
    let v11: US7 = v10.clone();
    let v12: US7 = v8.unwrap_or(v11);
    match v12 {
        US7::US7_None => {
            0u64
        }
        US7::US7_Some(v13) => {
            let v13: u64 = v13;
            v13
        }
    }
}
fn peak_sample_19(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> US8 {
    let mut v4: bool = v3 > 0u64;
    if v4 {
        let v5: Rc<str> = lane_of_14(v1);
        let mut v6: u64 = v3.wrapping_mul(1048576u64);
        let mut v7: US5 = US5::US5_JobPeak(v5, v2, v6);
        let mut v8: US1 = US1::US1_SizedSample(v0, v7);
        US8::US8_Some(v8)
    } else {
        US8::US8_None
    }
}
fn method2(mut v0: US0, mut v1: u64, mut v2: Rc<UH0>, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let (mut v6, mut v7, mut v8, mut v9): (Rc<str>, u64, u64, u64) = job_record_of_3(v4.clone());
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
                let v11: Rc<str> = job_file_4(v4.clone(), v10);
                let v12: Rc<str> = text_or_empty_8(v11);
                let v13: Rc<str> = Rc::<str>::from((v12).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v14: bool = &*v6 != "";
                let v16: bool = if v14 {
                    let v15: bool = v8 >= v1;
                    v15
                } else {
                    false
                };
                let v17: u64 = elapsed_of_12(v4.clone());
                let v18: US8 = job_sample_13(v0.clone(), v13.clone(), v4.clone(), v17, v16);
                let v22: Rc<UH1> = match v18 {
                    US8::US8_None => {
                        v3
                    }
                    US8::US8_Some(v19) => {
                        let v19: US1 = v19;
                        Rc::new(UH1::UH1_Cons(v19, v3))
                    }
                };
                let v26: US8 = if v16 {
                    let v23: u64 = peak_mb_of_18(v4.clone());
                    peak_sample_19(v0.clone(), v13, v4, v23)
                } else {
                    US8::US8_None
                };
                let v30: Rc<UH1> = match v26 {
                    US8::US8_None => {
                        v22
                    }
                    US8::US8_Some(v27) => {
                        let v27: US1 = v27;
                        Rc::new(UH1::UH1_Cons(v27, v22))
                    }
                };
                (v2, v3) = (v5, v30);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn directory_exists_20(mut v0: Rc<str>) -> bool {
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
fn method21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method21(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
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
fn job_samples_1(mut v0: US0, mut v1: u64) -> Rc<UH1> {
    let v2: Rc<str> = jobs_root_7();
    let mut v3: bool = directory_exists_20(v2.clone());
    let mut v4: bool = v3 == false;
    if v4 {
        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v7: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v2).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v9: Vec<std::string::String> = v7.unwrap();
        let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v12: bool = true; let _vec_fold_ = v9.into_iter().fold(v10, |acc, x| { //;
        let v14: Rc<UH0> = acc;
        let v16: std::string::String = x;
        let v18: Rc<str> = Rc::<str>::from(String::as_str(&v16));
        let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v14));
        let v21: bool = true; v20 });
        let v23: Rc<UH0> = _vec_fold_;
        let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v25: Rc<UH0> = method21(v23, v24);
        let v26: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method2(v0, v1, v25, v26)
    }
}
fn method24(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_23(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method24(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method22(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: US1 = v2;
            let mut v3: Rc<UH1> = v3;
            let mut v4: Rc<UH0> = method22(v3, v1);
            let v150: Rc<str> = match v2 {
                US1::US1_CountedSample(v126, v127) => {
                    let v126: US0 = v126;
                    let v127: US6 = v127;
                    let v131: Rc<str> = match v126 {
                        US0::US0_Box => {
                            let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("box"); } LIT.with(|lit| lit.clone()) };
                            v128
                        }
                        US0::US0_Node => {
                            let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node"); } LIT.with(|lit| lit.clone()) };
                            v129
                        }
                    };
                    let v133: Rc<str> = string_concat(&["machine=\"", &*v131, "\""]);
                    match v127 {
                        US6::US6_Count(v134, v135) => {
                            let v134: Rc<str> = v134;
                            let v135: u64 = v135;
                            let v137: Rc<str> = string_concat(&["what=\"", &*v134, "\""]);
                            let v138: Rc<str> = format_real_23(v135);
                            let mut v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig_exec_count{"); } LIT.with(|lit| lit.clone()) };
                            v139.borrow_mut().push(v140);
                            v139.borrow_mut().push(v133);
                            let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            v139.borrow_mut().push(v141);
                            v139.borrow_mut().push(v137);
                            let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v139.borrow_mut().push(v142);
                            v139.borrow_mut().push(v138);
                            let v143: Rc<Vec<Rc<str>>> = Rc::new(v139.borrow().clone());
                            let v144: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v143).as_ref().clone()));
                            let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v146: Rc<str> = Rc::<str>::from((*v144.borrow()).join(&*v145));
                            v146
                        }
                    }
                }
                US1::US1_SizedSample(v83, v84) => {
                    let v83: US0 = v83;
                    let v84: US5 = v84;
                    let v88: Rc<str> = match v83 {
                        US0::US0_Box => {
                            let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("box"); } LIT.with(|lit| lit.clone()) };
                            v85
                        }
                        US0::US0_Node => {
                            let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node"); } LIT.with(|lit| lit.clone()) };
                            v86
                        }
                    };
                    let v90: Rc<str> = string_concat(&["machine=\"", &*v88, "\""]);
                    match v84 {
                        US5::US5_JobPeak(v104, v105, v106) => {
                            let v104: Rc<str> = v104;
                            let v105: Rc<str> = v105;
                            let v106: u64 = v106;
                            let v108: Rc<str> = string_concat(&["lane=\"", &*v104, "\""]);
                            let v110: Rc<str> = string_concat(&["job=\"", &*v105, "\""]);
                            let mut v111: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            v111.borrow_mut().push(v90);
                            v111.borrow_mut().push(v108);
                            v111.borrow_mut().push(v110);
                            let v112: Rc<Vec<Rc<str>>> = Rc::new(v111.borrow().clone());
                            let v113: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v112).as_ref().clone()));
                            let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            let v115: Rc<str> = Rc::<str>::from((*v113.borrow()).join(&*v114));
                            let v116: Rc<str> = format_real_23(v106);
                            let mut v117: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig_job_peak_bytes{"); } LIT.with(|lit| lit.clone()) };
                            v117.borrow_mut().push(v118);
                            v117.borrow_mut().push(v115);
                            let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v117.borrow_mut().push(v119);
                            v117.borrow_mut().push(v116);
                            let v120: Rc<Vec<Rc<str>>> = Rc::new(v117.borrow().clone());
                            let v121: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v120).as_ref().clone()));
                            let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v123: Rc<str> = Rc::<str>::from((*v121.borrow()).join(&*v122));
                            v123
                        }
                        US5::US5_PeakBytes(v91, v92) => {
                            let v91: Rc<str> = v91;
                            let v92: u64 = v92;
                            let v94: Rc<str> = string_concat(&["what=\"", &*v91, "\""]);
                            let v95: Rc<str> = format_real_23(v92);
                            let mut v96: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig_exec_peak_bytes{"); } LIT.with(|lit| lit.clone()) };
                            v96.borrow_mut().push(v97);
                            v96.borrow_mut().push(v90);
                            let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            v96.borrow_mut().push(v98);
                            v96.borrow_mut().push(v94);
                            let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v96.borrow_mut().push(v99);
                            v96.borrow_mut().push(v95);
                            let v100: Rc<Vec<Rc<str>>> = Rc::new(v96.borrow().clone());
                            let v101: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v100).as_ref().clone()));
                            let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v103: Rc<str> = Rc::<str>::from((*v101.borrow()).join(&*v102));
                            v103
                        }
                    }
                }
                US1::US1_TimedSample(v5, v6) => {
                    let v5: US0 = v5;
                    let mut v6: US2 = v6;
                    let v10: Rc<str> = match v5 {
                        US0::US0_Box => {
                            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("box"); } LIT.with(|lit| lit.clone()) };
                            v7
                        }
                        US0::US0_Node => {
                            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node"); } LIT.with(|lit| lit.clone()) };
                            v8
                        }
                    };
                    let v12: Rc<str> = string_concat(&["machine=\"", &*v10, "\""]);
                    match v6 {
                        US2::US2_Duration(v13, v14, v15) => {
                            let v13: US3 = v13;
                            let v14: US4 = v14;
                            let v15: u64 = v15;
                            let v29: Rc<str> = match v13 {
                                US3::US3_CBackend => {
                                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c"); } LIT.with(|lit| lit.clone()) };
                                    v19
                                }
                                US3::US3_Fsharp => {
                                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fsharp"); } LIT.with(|lit| lit.clone()) };
                                    v22
                                }
                                US3::US3_Gleam => {
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
                                    v17
                                }
                                US3::US3_Lua => {
                                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lua"); } LIT.with(|lit| lit.clone()) };
                                    v20
                                }
                                US3::US3_Rust => {
                                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust"); } LIT.with(|lit| lit.clone()) };
                                    v16
                                }
                                US3::US3_Wasm => {
                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wasm"); } LIT.with(|lit| lit.clone()) };
                                    v21
                                }
                                US3::US3_Zig => {
                                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
                                    v18
                                }
                            };
                            let v31: Rc<str> = string_concat(&["backend=\"", &*v29, "\""]);
                            let v45: Rc<str> = match v14 {
                                US4::US4_Compile => {
                                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
                                    v32
                                }
                                US4::US4_DotnetStartup => {
                                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dotnet_startup"); } LIT.with(|lit| lit.clone()) };
                                    v36
                                }
                                US4::US4_LockWait => {
                                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lock_wait"); } LIT.with(|lit| lit.clone()) };
                                    v38
                                }
                                US4::US4_Mount => {
                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mount"); } LIT.with(|lit| lit.clone()) };
                                    v37
                                }
                                US4::US4_Run => {
                                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
                                    v34
                                }
                                US4::US4_Rustc => {
                                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustc"); } LIT.with(|lit| lit.clone()) };
                                    v33
                                }
                                US4::US4_SlotWait => {
                                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_wait"); } LIT.with(|lit| lit.clone()) };
                                    v35
                                }
                            };
                            let v47: Rc<str> = string_concat(&["phase=\"", &*v45, "\""]);
                            let mut v48: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            v48.borrow_mut().push(v12);
                            v48.borrow_mut().push(v31);
                            v48.borrow_mut().push(v47);
                            let v49: Rc<Vec<Rc<str>>> = Rc::new(v48.borrow().clone());
                            let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v49).as_ref().clone()));
                            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            let v52: Rc<str> = Rc::<str>::from((*v50.borrow()).join(&*v51));
                            let v53: Rc<str> = format_real_23(v15);
                            let mut v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig_exec_duration_ms{"); } LIT.with(|lit| lit.clone()) };
                            v54.borrow_mut().push(v55);
                            v54.borrow_mut().push(v52);
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v54.borrow_mut().push(v56);
                            v54.borrow_mut().push(v53);
                            let v57: Rc<Vec<Rc<str>>> = Rc::new(v54.borrow().clone());
                            let v58: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v57).as_ref().clone()));
                            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v60: Rc<str> = Rc::<str>::from((*v58.borrow()).join(&*v59));
                            v60
                        }
                        US2::US2_JobDuration(v61, v62, v63) => {
                            let v61: Rc<str> = v61;
                            let v62: Rc<str> = v62;
                            let v63: u64 = v63;
                            let v65: Rc<str> = string_concat(&["lane=\"", &*v61, "\""]);
                            let v67: Rc<str> = string_concat(&["job=\"", &*v62, "\""]);
                            let mut v68: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            v68.borrow_mut().push(v12);
                            v68.borrow_mut().push(v65);
                            v68.borrow_mut().push(v67);
                            let v69: Rc<Vec<Rc<str>>> = Rc::new(v68.borrow().clone());
                            let v70: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v69).as_ref().clone()));
                            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            let v72: Rc<str> = Rc::<str>::from((*v70.borrow()).join(&*v71));
                            let v73: Rc<str> = format_real_23(v63);
                            let mut v74: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig_job_duration_ms{"); } LIT.with(|lit| lit.clone()) };
                            v74.borrow_mut().push(v75);
                            v74.borrow_mut().push(v72);
                            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v74.borrow_mut().push(v76);
                            v74.borrow_mut().push(v73);
                            let v77: Rc<Vec<Rc<str>>> = Rc::new(v74.borrow().clone());
                            let v78: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v77).as_ref().clone()));
                            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v80: Rc<str> = Rc::<str>::from((*v78.borrow()).join(&*v79));
                            v80
                        }
                    }
                }
            };
            Rc::new(UH0::UH0_Cons(v150, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method25(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
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
fn jobs_prom_0(mut v0: u64, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v1 == "node";
    let v5: US0 = if v2 {
        US0::US0_Node
    } else {
        US0::US0_Box
    };
    let mut v6: Rc<UH1> = job_samples_1(v5, v0);
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = method22(v6, v7);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v10: i32 = 0i32;
    let v11: i32 = method25(v9.clone(), v8, v10);
    let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
    let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v14));
    v15
}
fn closure0() -> Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> Rc<str> {
        jobs_prom_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_exec_jobs_prom(v0: &str, v1: u64) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), v1)
}
use rig_process_alive::SpiralShape_97df63ebabc3 as Mut1;
pub type SpiralShape_652accfc6a8d = Mut0;
