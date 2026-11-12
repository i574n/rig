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
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_JobHost,
    US1_JobSupervisor(Rc<str>),
    US1_JobWaiter(Rc<str>),
    US1_WindowHolder,
    US1_OtherCall,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(u64, bool, US1, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_IdleOrphanHost(u64),
    US2_OrphanCompilerServer(u64, u64),
    US2_SettledButSupervised(u64, Rc<str>),
    US2_WaitOnSettled(u64, Rc<str>),
    US2_WaitOnMissing(u64, Rc<str>),
    US2_ChainToMissing(Rc<str>, Rc<str>),
    US2_HostEnvironmentDiffers(Rc<str>),
    US2_UnwatchedJob(u64, Rc<str>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US2, Rc<UH2>),
}
#[derive(Clone)]
enum US3 {
    US3_JobLive,
    US3_JobSettling,
    US3_JobSettled,
    US3_JobMissing,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum US4 {
    US4_Some(u64),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_Some(i32),
    US5_None,
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<str>, US3, Rc<UH3>),
}
#[derive(Clone)]
enum US6 {
    US6_Some(US2),
    US6_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn now_ms_2() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: u64 = v2;
                let v3: bool = v3;
                let v4: US1 = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method8(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn facts_from_6(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method7(v3, v5);
        } else {
            let mut v7: Rc<str> = v0[v1 as usize].2.clone();
            let v8: bool = &*v7 == "rig";
            let v10: bool = if v8 {
                true
            } else {
                let mut v9: bool = &*v7 == "kit";
                v9
            };
            let v13: bool = if v10 {
                true
            } else {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-"); } LIT.with(|lit| lit.clone()) };
                let v12: bool = v7.starts_with(&*v11);
                v12
            };
            let v16: bool = if v13 {
                true
            } else {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-"); } LIT.with(|lit| lit.clone()) };
                let v15: bool = v7.starts_with(&*v14);
                v15
            };
            let mut v17: bool = v16 == false;
            if v17 {
                let v18: i32 = v1.wrapping_add(1i32);
                v1 = v18;
                continue;
            } else {
                let v20: u64 = v0[v1 as usize].0;
                let v21: u64 = v0[v1 as usize].1;
                let v22: i32 = v0.iter().position(|row| row.0 == v21).map(|at| at as i32).unwrap_or(-1);
                let v23: bool = v22 >= 0i32;
                let v24: Rc<str> = v0[v1 as usize].3.clone();
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job host"); } LIT.with(|lit| lit.clone()) };
                let v26: bool = v24.contains(&*v25);
                let v91: US1 = if v26 {
                    US1::US1_JobHost
                } else {
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job supervise "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v24.contains(&*v28);
                    if v29 {
                        let v30: Option<usize> = v24.find(&*v28);
                        let v31: Option<usize> = method8(v30);
                        let mut v32: Rc<dyn Fn((usize)) -> i32> = closure1();
                        let mut v33: Option<i32> = v31.map(|x| v32(x));
                        let v35: i32 = v33.unwrap_or(-1i32);
                        let v36: bool = v35 < 0i32;
                        let v54: Rc<str> = if v36 {
                            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v37
                        } else {
                            let v38: i32 = (v28.len() as i32);
                            let v39: i32 = v35.wrapping_add(v38);
                            let v40: i32 = (v24.clone().len() as i32);
                            let v41: i32 = v40.wrapping_sub(v35);
                            let v42: i32 = v41.wrapping_sub(v38);
                            let v43: usize = ((v39) as usize);
                            let v44: i32 = v39.wrapping_add(v42);
                            let v45: usize = ((v44) as usize);
                            let v46: Rc<str> = Rc::<str>::from(&(v24)[v43..v45]);
                            let v47: Rc<str> = Rc::<str>::from((v46).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                            let v49: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v47.split(&*v48).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                            let v50: Rc<str> = v49.borrow()[0i32 as usize].clone();
                            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v53: Rc<str> = Rc::<str>::from(v50.replace(&*v51, &*v52));
                            v53
                        };
                        US1::US1_JobSupervisor(v54)
                    } else {
                        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job wait "); } LIT.with(|lit| lit.clone()) };
                        let v57: bool = v24.contains(&*v56);
                        if v57 {
                            let v58: Option<usize> = v24.find(&*v56);
                            let v59: Option<usize> = method8(v58);
                            let mut v60: Rc<dyn Fn((usize)) -> i32> = closure1();
                            let mut v61: Option<i32> = v59.map(|x| v60(x));
                            let v63: i32 = v61.unwrap_or(-1i32);
                            let v64: bool = v63 < 0i32;
                            let v82: Rc<str> = if v64 {
                                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v65
                            } else {
                                let v66: i32 = (v56.len() as i32);
                                let v67: i32 = v63.wrapping_add(v66);
                                let v68: i32 = (v24.clone().len() as i32);
                                let v69: i32 = v68.wrapping_sub(v63);
                                let v70: i32 = v69.wrapping_sub(v66);
                                let v71: usize = ((v67) as usize);
                                let v72: i32 = v67.wrapping_add(v70);
                                let v73: usize = ((v72) as usize);
                                let v74: Rc<str> = Rc::<str>::from(&(v24)[v71..v73]);
                                let v75: Rc<str> = Rc::<str>::from((v74).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                let v77: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v75.split(&*v76).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                                let v78: Rc<str> = v77.borrow()[0i32 as usize].clone();
                                let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v81: Rc<str> = Rc::<str>::from(v78.replace(&*v79, &*v80));
                                v81
                            };
                            US1::US1_JobWaiter(v82)
                        } else {
                            let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" window run "); } LIT.with(|lit| lit.clone()) };
                            let v85: bool = v24.contains(&*v84);
                            if v85 {
                                US1::US1_WindowHolder
                            } else {
                                US1::US1_OtherCall
                            }
                        }
                    }
                };
                let v92: i32 = v1.wrapping_add(1i32);
                let v93: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20, v23, v91, v3));
                (v1, v3) = (v92, v93);
                continue;
            }
        }
    }
}
fn process_rows_sampled_9(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> {
    let mut v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = rig_machine_sample::rig_machine_sampled(v0);
    v1
}
fn process_facts_5() -> Rc<UH0> {
    let mut v0: u64 = 50u64;
    let mut v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = process_rows_sampled_9(v0);
    let v2: i32 = 0i32;
    let v3: i32 = v1.len() as i32;
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    facts_from_6(v1, v2, v3, v4)
}
fn method10(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: u64 = v2;
                let v3: bool = v3;
                let v4: US1 = v4;
                let v5: Rc<UH0> = v5;
                let v8: i32 = match v4 {
                    US1::US1_JobSupervisor(v6) => {
                        let mut v6: Rc<str> = v6;
                        let mut v7: i32 = v1.wrapping_add(1i32);
                        v7
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v5, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: u64 = v2;
                let v3: bool = v3;
                let v4: US1 = v4;
                let v5: Rc<UH0> = v5;
                let v8: Rc<UH1> = match v4 {
                    US1::US1_JobWaiter(v6) => {
                        let mut v6: Rc<str> = v6;
                        Rc::new(UH1::UH1_Cons(v6, v1))
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v5, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn joined_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_16() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_JOBS_HOME").or_else(|| std::env::var_os("KIT_JOBS_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_16();
    joined_15(v1, v0)
}
fn directory_exists_17(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn job_file_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_14(v0);
    joined_15(v2, v1)
}
fn file_exists_19(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn text_or_empty_20(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method22(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
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
fn method23(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method23(v7, &v9) {
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
        let mut v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method22(v16, v17)
    }
}
fn method24(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let mut v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn standing_of_13(mut v0: Rc<str>) -> US3 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    let v5: bool = if v2 {
        let v3: Rc<str> = job_directory_14(v0.clone());
        directory_exists_17(v3)
    } else {
        false
    };
    let mut v6: bool = v5 == false;
    if v6 {
        US3::US3_JobMissing
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = job_file_18(v0.clone(), v8.clone());
        let v10: bool = file_exists_19(v9);
        if v10 {
            let v11: Rc<str> = job_file_18(v0, v8);
            let v12: Rc<str> = text_or_empty_20(v11);
            let v13: u64 = now_ms_2();
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<UH1> = split_list_21(v14, v12);
            let v24: Rc<str> = match Rc::unwrap_or_clone(v15) {
                UH1::UH1_Cons(v16, v17) => {
                    let v16: Rc<str> = v16;
                    let mut v17: Rc<UH1> = v17;
                    match Rc::unwrap_or_clone(v17) {
                        UH1::UH1_Cons(v18, v19) => {
                            let v18: Rc<str> = v18;
                            let v19: Rc<UH1> = v19;
                            let v20: Rc<str> = Rc::<str>::from((v18).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                            v20
                        }
                        _ => {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v21
                        }
                    }
                }
                _ => {
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v23
                }
            };
            let v25: Option<u64> = (v24).trim().parse::<u64>().ok();
            let v26: Option<u64> = method24(v25);
            let mut v27: Rc<dyn Fn((u64)) -> US4> = closure2();
            let mut v28: Option<US4> = v26.map(|x| v27(x));
            let v30: US4 = US4::US4_None;
            let v31: US4 = v30.clone();
            let mut v32: US4 = v28.unwrap_or(v31);
            let v35: u64 = match v32 {
                US4::US4_None => {
                    0u64
                }
                US4::US4_Some(v33) => {
                    let mut v33: u64 = v33;
                    v33
                }
            };
            let v36: bool = v35 > 0u64;
            let v39: bool = if v36 {
                let v37: u64 = v35.wrapping_add(30000u64);
                let v38: bool = v13 < v37;
                v38
            } else {
                false
            };
            if v39 {
                US3::US3_JobSettling
            } else {
                US3::US3_JobSettled
            }
        } else {
            US3::US3_JobLive
        }
    }
}
fn closure3(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US5> {
    Rc::new(move |mut v2: Rc<UH1>| -> US5 {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US5::US5_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US5> = method25(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH1::UH1_Nil => {
                US5::US5_None
            }
        }
    })
}
fn method25(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US5> {
    closure3(v0, v1)
}
fn method12(mut v0: bool, mut v1: Rc<UH1>, mut v2: Rc<UH0>, mut v3: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5, v6, v7) => {
                let v4: u64 = v4;
                let v5: bool = v5;
                let mut v6: US1 = v6;
                let mut v7: Rc<UH0> = v7;
                let v40: Rc<UH2> = match v6 {
                    US1::US1_JobHost => {
                        let v8: bool = v5 || v0;
                        if v8 {
                            v3
                        } else {
                            let mut v9: US2 = US2::US2_IdleOrphanHost(v4);
                            Rc::new(UH2::UH2_Cons(v9, v3))
                        }
                    }
                    US1::US1_JobSupervisor(v12) => {
                        let v12: Rc<str> = v12;
                        let v13: US3 = standing_of_13(v12.clone());
                        match v13 {
                            US3::US3_JobLive => {
                                let v16: i32 = 0i32;
                                let mut v17: Rc<dyn Fn(Rc<UH1>) -> US5> = method25(v12.clone(), v16);
                                let v18: US5 = v17(v1.clone());
                                let v19: bool = match v18 {
                                    US5::US5_None => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                };
                                let v20: bool = v19 != true;
                                if v20 {
                                    v3
                                } else {
                                    let v21: US2 = US2::US2_UnwatchedJob(v4, v12);
                                    Rc::new(UH2::UH2_Cons(v21, v3))
                                }
                            }
                            US3::US3_JobMissing => {
                                v3
                            }
                            US3::US3_JobSettled => {
                                let v14: US2 = US2::US2_SettledButSupervised(v4, v12);
                                Rc::new(UH2::UH2_Cons(v14, v3))
                            }
                            US3::US3_JobSettling => {
                                v3
                            }
                        }
                    }
                    US1::US1_JobWaiter(v28) => {
                        let mut v28: Rc<str> = v28;
                        let v29: US3 = standing_of_13(v28.clone());
                        match v29 {
                            US3::US3_JobLive => {
                                v3
                            }
                            US3::US3_JobMissing => {
                                let mut v32: US2 = US2::US2_WaitOnMissing(v4, v28);
                                Rc::new(UH2::UH2_Cons(v32, v3))
                            }
                            US3::US3_JobSettled => {
                                let v30: US2 = US2::US2_WaitOnSettled(v4, v28);
                                Rc::new(UH2::UH2_Cons(v30, v3))
                            }
                            US3::US3_JobSettling => {
                                v3
                            }
                        }
                    }
                    _ => {
                        v3
                    }
                };
                (v2, v3) = (v7, v40);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn method26(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: US2 = v2;
                let v3: Rc<UH2> = v3;
                let v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method29(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method28(mut v0: Rc<UH1>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                let v5: Rc<str> = job_file_18(v2.clone(), v4);
                let mut v6: bool = file_exists_19(v5.clone());
                let v11: bool = if v6 {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
                    let v8: Rc<str> = job_file_18(v2.clone(), v7);
                    let mut v9: bool = file_exists_19(v8);
                    let v10: bool = v9 == false;
                    v10
                } else {
                    false
                };
                let v16: bool = if v11 {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                    let v13: Rc<str> = job_file_18(v2.clone(), v12);
                    let v14: bool = file_exists_19(v13);
                    let v15: bool = v14 == false;
                    v15
                } else {
                    false
                };
                let v34: Rc<UH3> = if v16 {
                    let v18: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v5);
                    let v20: Vec<u8> = v18.unwrap();
                    let v21: Vec<u8> = method29(v20);
                    let v23: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v21);
                    let v25: std::string::String = v23.unwrap();
                    let mut v27: Rc<str> = Rc::<str>::from(String::as_str(&v25));
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v29: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v27.split(&*v28).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                    let v30: Rc<str> = v29.borrow()[0i32 as usize].clone();
                    let v31: Rc<str> = Rc::<str>::from((v30).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                    let mut v32: US3 = standing_of_13(v31.clone());
                    Rc::new(UH3::UH3_Cons(v2, v31, v32, v1))
                } else {
                    v1
                };
                (v0, v1) = (v3, v34);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method31(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method31(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            if v6 {
                Rc::new(UH1::UH1_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn directory_exists_32(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn job_names_30() -> Rc<UH1> {
    let mut v0: Rc<str> = jobs_root_16();
    let mut v1: bool = directory_exists_32(v0.clone());
    let v2: bool = v1 == false;
    if v2 {
        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v5: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v7: Vec<std::string::String> = v5.unwrap();
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let v10: bool = true; let _vec_fold_ = v7.into_iter().fold(v8, |acc, x| { //;
        let v12: Rc<UH1> = acc;
        let v14: std::string::String = x;
        let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
        let v18: Rc<UH1> = Rc::new(UH1::UH1_Cons(v16, v12));
        let v19: bool = true; v18 });
        let v21: Rc<UH1> = _vec_fold_;
        let v22: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method31(v21, v22)
    }
}
fn chain_facts_27() -> Rc<UH3> {
    let mut v0: Rc<UH1> = job_names_30();
    let mut v1: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    method28(v0, v1)
}
fn method33(mut v0: Rc<UH3>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: US3 = v4;
                let v5: Rc<UH3> = v5;
                let v8: Rc<UH2> = match v4 {
                    US3::US3_JobMissing => {
                        let mut v6: US2 = US2::US2_ChainToMissing(v2, v3);
                        Rc::new(UH2::UH2_Cons(v6, v1))
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v5, v8);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn method34(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: US2 = v2;
                let v3: Rc<UH2> = v3;
                let v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn environment_finding_35(mut v0: Rc<dyn Fn() -> (bool, Rc<str>)>) -> US6 {
    let (mut v1, mut v2): (bool, Rc<str>) = v0();
    let v3: bool = v1 == false;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(an environment listing did not run)"); } LIT.with(|lit| lit.clone()) };
        let v5: US2 = US2::US2_HostEnvironmentDiffers(v4);
        US6::US6_Some(v5)
    } else {
        let mut v7: bool = &*v2 == "";
        if v7 {
            US6::US6_None
        } else {
            let mut v9: US2 = US2::US2_HostEnvironmentDiffers(v2);
            US6::US6_Some(v9)
        }
    }
}
fn method38(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_37(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method38(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method36(mut v0: Rc<UH2>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v1, v2) => {
                let mut v1: US2 = v1;
                let v2: Rc<UH2> = v2;
                let v54: Rc<str> = match v1 {
                    US2::US2_ChainToMissing(v33, v34) => {
                        let mut v33: Rc<str> = v33;
                        let v34: Rc<str> = v34;
                        let v37: Rc<str> = string_concat(&["chain-to-missing job=", &*v33, " after=", &*v34]);
                        v37
                    }
                    US2::US2_HostEnvironmentDiffers(v38) => {
                        let v38: Rc<str> = v38;
                        let v39: Rc<str> = string_concat(&["host-environment-differs ", &*v38]);
                        v39
                    }
                    US2::US2_IdleOrphanHost(v3) => {
                        let v3: u64 = v3;
                        let v4: Rc<str> = format_real_37(v3);
                        let mut v6: Rc<str> = string_concat(&["idle-orphan-host pid=", &*v4, " (its parent is gone and it serves no job)"]);
                        v6
                    }
                    US2::US2_OrphanCompilerServer(v7, v8) => {
                        let mut v7: u64 = v7;
                        let v8: u64 = v8;
                        let mut v9: Rc<str> = format_real_37(v7);
                        let v12: Rc<str> = format_real_37(v8);
                        let v14: Rc<str> = string_concat(&["orphan-compiler-server pid=", &*v9, " owner=", &*v12, " (the erl named in its socket is gone; send quit on the socket or stop the pid)"]);
                        v14
                    }
                    US2::US2_SettledButSupervised(v15, v16) => {
                        let v15: u64 = v15;
                        let v16: Rc<str> = v16;
                        let mut v17: Rc<str> = format_real_37(v15);
                        let v20: Rc<str> = string_concat(&["settled-but-supervised pid=", &*v17, " job=", &*v16]);
                        v20
                    }
                    US2::US2_UnwatchedJob(v40, v41) => {
                        let v40: u64 = v40;
                        let v41: Rc<str> = v41;
                        let v42: Rc<str> = format_real_37(v40);
                        let v46: Rc<str> = string_concat(&["unwatched-job pid=", &*v42, " job=", &*v41, " (live, but no rig job wait holds it; nothing will notice when it ends)"]);
                        v46
                    }
                    US2::US2_WaitOnMissing(v27, v28) => {
                        let mut v27: u64 = v27;
                        let mut v28: Rc<str> = v28;
                        let v29: Rc<str> = format_real_37(v27);
                        let mut v32: Rc<str> = string_concat(&["wait-on-missing pid=", &*v29, " job=", &*v28]);
                        v32
                    }
                    US2::US2_WaitOnSettled(v21, v22) => {
                        let v21: u64 = v21;
                        let v22: Rc<str> = v22;
                        let v23: Rc<str> = format_real_37(v21);
                        let v26: Rc<str> = string_concat(&["wait-on-settled pid=", &*v23, " job=", &*v22]);
                        v26
                    }
                };
                println!("{}", v54);
                v0 = v2;
                continue;
            }
            UH2::UH2_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method39(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: US2 = v2;
                let v3: Rc<UH2> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method40(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: u64 = v2;
                let v3: bool = v3;
                let v4: US1 = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn format_real_41(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method38(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn doctor_run_1(mut v0: Rc<dyn Fn() -> (bool, Rc<str>)>, mut v1: Rc<dyn Fn() -> (i32, Rc<str>)>) -> i32 {
    let v2: u64 = now_ms_2();
    let v3: i32 = command_line_argument_count_3();
    let v4: bool = 2i32 < v3;
    let mut v9: US0 = if v4 {
        let v5: i32 = 2i32;
        let mut v6: Rc<str> = command_line_argument_4(v5);
        US0::US0_Some(v6)
    } else {
        US0::US0_None
    };
    let v13: Rc<str> = match v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            v10
        }
    };
    let v14: bool = &*v13 == "--env";
    let v15: Rc<UH0> = process_facts_5();
    let v16: u64 = now_ms_2();
    let mut v17: i32 = 0i32;
    let v18: i32 = method10(v15.clone(), v17);
    let v19: bool = v18 > 0i32;
    let v20: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v21: Rc<UH1> = method11(v15.clone(), v20);
    let v22: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v23: Rc<UH2> = method12(v19, v21, v15.clone(), v22);
    let v24: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v25: Rc<UH2> = method26(v23, v24);
    let v26: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let mut v27: Rc<UH2> = method26(v25, v26);
    let mut v28: Rc<UH3> = chain_facts_27();
    let v29: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v30: Rc<UH2> = method33(v28, v29);
    let v31: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let mut v32: Rc<UH2> = method26(v30, v31);
    let mut v33: Rc<UH2> = method34(v32, v27);
    let v34: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v35: Rc<UH2> = method26(v33, v34);
    let v36: bool = v14 == false;
    let v46: Rc<UH2> = if v36 {
        v35
    } else {
        let v37: US6 = environment_finding_35(v0);
        match v37 {
            US6::US6_None => {
                v35
            }
            US6::US6_Some(v38) => {
                let v38: US2 = v38;
                let v39: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                let v40: Rc<UH2> = method26(v35, v39);
                let v41: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                let v42: Rc<UH2> = Rc::new(UH2::UH2_Cons(v38, v41));
                method26(v40, v42)
            }
        }
    };
    method36(v46.clone());
    let (mut v47, mut v48): (i32, Rc<str>) = v1();
    let v49: bool = &*v48 != "";
    if v49 {
        println!("{}", v48);
        ()
    };
    let v50: i32 = 0i32;
    let v51: i32 = method39(v46, v50);
    let v52: i32 = v51.wrapping_add(v47);
    let v53: i32 = 0i32;
    let v54: i32 = method40(v15, v53);
    let v55: Rc<str> = format_real_41(v54);
    let v58: Rc<str> = format_real_41(v52);
    let mut v60: u64 = now_ms_2();
    let mut v61: u64 = v60.wrapping_sub(v2);
    let v62: Rc<str> = format_real_37(v61);
    let v65: u64 = v16.wrapping_sub(v2);
    let v66: Rc<str> = format_real_37(v65);
    let v68: Rc<str> = string_concat(&[" processes=", &*v55, " findings=", &*v58, " ms=", &*v62, " sample_ms=", &*v66]);
    let v69: bool = v52 == 0i32;
    if v69 {
        let v70: Rc<str> = string_concat(&["rig job doctor ok", &*v68]);
        println!("{}", v70);
        0i32
    } else {
        let v71: Rc<str> = string_concat(&["rig job doctor refused:", &*v68]);
        println!("{}", v71);
        1i32
    }
}
fn env_drift_42() -> (bool, Rc<str>) {
    let (mut v0, mut v1): (bool, Rc<str>) = rig_jobs_doctor_env::rig_job_doctor_env_drift();
    (v0, v1)
}
fn closure4() -> Rc<dyn Fn() -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (bool, Rc<str>)> = Rc::new(move || -> (bool, Rc<str>) {
        env_drift_42()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn server_report_43() -> (i32, Rc<str>) {
    let (mut v0, mut v1): (i32, Rc<str>) = rig_jobs_doctor_servers::rig_job_doctor_server_report();
    (v0, v1)
}
fn closure5() -> Rc<dyn Fn() -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (i32, Rc<str>)> = Rc::new(move || -> (i32, Rc<str>) {
        server_report_43()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn doctor_checked_0() -> i32 {
    let mut v0: Rc<dyn Fn() -> (bool, Rc<str>)> = closure4();
    let mut v1: Rc<dyn Fn() -> (i32, Rc<str>)> = closure5();
    doctor_run_1(v0, v1)
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        doctor_checked_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_job_doctor_run() -> i32 {
    closure0()()
}
pub type SpiralShape_accbf6de6842 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
