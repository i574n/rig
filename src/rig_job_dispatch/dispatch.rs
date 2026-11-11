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
    US0_Dispatched(Rc<str>, Rc<str>),
    US0_RemoteUnreachable(Rc<str>),
    US0_RemoteRefused(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_RequestRead(Rc<str>, Rc<str>, Rc<UH0>),
    US1_RequestRefused(Rc<str>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
fn outcome_code_1(mut v0: US0) -> i32 {
    match v0 {
        US0::US0_Dispatched(v1, v2) => {
            let mut v1: Rc<str> = v1;
            let v2: Rc<str> = v2;
            0i32
        }
        US0::US0_RemoteRefused(v4) => {
            let mut v4: Rc<str> = v4;
            2i32
        }
        US0::US0_RemoteUnreachable(v3) => {
            let v3: Rc<str> = v3;
            4i32
        }
    }
}
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn words_from_3(mut v0: i32, mut v1: i32) -> Rc<UH0> {
    let v2: bool = v1 <= v0;
    if v2 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
        let mut v5: i32 = v0.wrapping_add(1i32);
        let mut v6: Rc<UH0> = words_from_3(v5, v1);
        Rc::new(UH0::UH0_Cons(v4, v6))
    }
}
fn plain_argument_6(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.contains(&*v1);
    let v3: bool = v2 == false;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v0.contains(&*v4);
        let mut v6: bool = v5 == false;
        v6
    } else {
        false
    }
}
fn method5(mut v0: Rc<UH0>, mut v1: bool) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v5: bool = if v1 {
                    plain_argument_6(v2)
                } else {
                    false
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
fn request_of_4(mut v0: &Rc<UH0>) -> US1 {
    match &**v0 {
        UH0::UH0_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let v2: Rc<UH0> = v2.clone();
            let (mut v15, mut v16): (Rc<str>, Rc<UH0>) = match &*v2 {
                UH0::UH0_Cons(v3, v4) => {
                    let v3: Rc<str> = v3.clone();
                    let mut v4: Rc<UH0> = v4.clone();
                    match Rc::unwrap_or_clone(v4) {
                        UH0::UH0_Cons(v5, v6) => {
                            let mut v5: Rc<str> = v5;
                            let mut v6: Rc<UH0> = v6;
                            let v7: bool = &*v3 == "--bin";
                            if v7 {
                                (v5, v6)
                            } else {
                                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                                (v8, v2)
                            }
                        }
                        _ => {
                            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                            (v11, v2)
                        }
                    }
                }
                _ => {
                    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                    (v14, v2)
                }
            };
            let v17: bool = &*v1 == "";
            let v20: bool = if v17 {
                true
            } else {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let v19: bool = v1.starts_with(&*v18);
                v19
            };
            if v20 {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--on needs a machine (an ssh alias)"); } LIT.with(|lit| lit.clone()) };
                US1::US1_RequestRefused(v21)
            } else {
                let v23: bool = match &*v16 {
                    UH0::UH0_Nil => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                if v23 {
                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--on <machine> needs the job: [options] <label> <timeout-ms> <program> [args...]"); } LIT.with(|lit| lit.clone()) };
                    US1::US1_RequestRefused(v24)
                } else {
                    let v26: bool = true;
                    let v27: bool = method5(v16.clone(), v26);
                    let v28: bool = v27 == false;
                    if v28 {
                        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--on forwards arguments without quote characters (they would be re-split by the remote shell)"); } LIT.with(|lit| lit.clone()) };
                        US1::US1_RequestRefused(v29)
                    } else {
                        US1::US1_RequestRead(v1, v15, v16)
                    }
                }
            }
        }
        UH0::UH0_Nil => {
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--on needs a machine (an ssh alias)"); } LIT.with(|lit| lit.clone()) };
            US1::US1_RequestRefused(v35)
        }
    }
}
fn machine_binary_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = rig_job_bin::rig_job_machine_binary(&v0, &v1, &v2);
    v3
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method9(v3, v1);
            let mut v6: Rc<str> = string_concat(&["\"", &*v2, "\""]);
            Rc::new(UH0::UH0_Cons(v6, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method10(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn remote_line_8(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> Rc<str> {
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH0> = method9(v2, v4);
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v7: i32 = 0i32;
    let v8: i32 = method10(v6.clone(), v5, v7);
    let mut v9: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v9).as_ref().clone()));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from((*v10.borrow()).join(&*v11));
    let v13: Rc<str> = string_concat(&[&*v1, " job start ", &*v12]);
    v13
}
fn connect_of_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = rig_job_bin::rig_job_machine_connect(&v0, &v1);
    v2
}
fn ssh_with_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<UH0> {
    let v3: Rc<str> = string_concat(&["ConnectTimeout=", &*v0]);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-o"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BatchMode=yes"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ServerAliveInterval=15"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v7));
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v8));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v9));
    let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v10));
    let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v11));
    let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v12));
    let mut v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v13));
    Rc::new(UH0::UH0_Cons(v4, v14))
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v2 == "";
                let mut v6: Rc<UH0> = if v4 {
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
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method15(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
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
fn method18(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method20(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
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
        while method20(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method14(v16, v17)
    }
}
fn receipt_id_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig job start ok id="); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method18(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    } else {
        let v10: i32 = v7.wrapping_add(20i32);
        let v11: i32 = (v0.clone().len() as i32);
        let v12: i32 = v11.wrapping_sub(v7);
        let v13: i32 = v12.wrapping_sub(20i32);
        let mut v14: usize = ((v10) as usize);
        let v15: i32 = v10.wrapping_add(v13);
        let v16: usize = ((v15) as usize);
        let v17: Rc<str> = Rc::<str>::from(&(v0)[v14..v16]);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = Rc::<str>::from(v17.replace(&*v19, &*v18));
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v22));
        let mut v24: Rc<UH0> = split_list_19(v18, v23);
        match Rc::unwrap_or_clone(v24) {
            UH0::UH0_Cons(v25, v26) => {
                let v25: Rc<str> = v25;
                let v26: Rc<UH0> = v26;
                v25
            }
            UH0::UH0_Nil => {
                v22
            }
        }
    }
}
fn method22(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_21(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method22(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn outcome_of_16(mut v0: Rc<str>, mut v1: bool, mut v2: i32, mut v3: Rc<str>) -> US0 {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let v7: Rc<str> = Rc::<str>::from((v6).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v8: Rc<str> = receipt_id_17(v3);
    let mut v9: bool = v1 == false;
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh did not run"); } LIT.with(|lit| lit.clone()) };
        US0::US0_RemoteUnreachable(v10)
    } else {
        let v12: bool = &*v8 != "";
        if v12 {
            US0::US0_Dispatched(v0, v8)
        } else {
            let mut v14: bool = v2 == 255i32;
            let v16: bool = if v14 {
                true
            } else {
                let v15: bool = &*v7 == "";
                v15
            };
            if v16 {
                let v17: bool = &*v7 == "";
                let v20: Rc<str> = if v17 {
                    let v18: Rc<str> = format_real_21(v2);
                    let v19: Rc<str> = string_concat(&["ssh exit ", &*v18]);
                    v19
                } else {
                    v7
                };
                US0::US0_RemoteUnreachable(v20)
            } else {
                US0::US0_RemoteRefused(v7)
            }
        }
    }
}
fn joined_24(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v6: Rc<str> = string_concat(&[&*v4, "/", &*v1]);
    v6
}
fn label_of_id_25(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH0> = split_list_19(v1.clone(), v0.clone());
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH0::UH0_Cons(v5, v6) => {
                    let mut v5: Rc<str> = v5;
                    let mut v6: Rc<UH0> = v6;
                    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v8: i32 = 0i32;
                    let mut v9: i32 = method10(v7.clone(), v6, v8);
                    let v10: Rc<Vec<Rc<str>>> = Rc::new(v7.borrow().clone());
                    let v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v10).as_ref().clone()));
                    let v12: Rc<str> = Rc::<str>::from((*v11.borrow()).join(&*v1));
                    v12
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
fn format_real_26(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method22(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn recorded_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> () {
    let mut v5: Rc<str> = joined_24(v0, v2.clone());
    let v7: Result<(), std::io::Error> = std::fs::create_dir_all(&*v5);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placement"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = joined_24(v5.clone(), v8);
    let v10: Rc<str> = string_concat(&["on ", &*v1]);
    std::fs::write(&*v9, &*v10).unwrap();
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = joined_24(v5.clone(), v12);
    let mut v14: Rc<str> = label_of_id_25(v2);
    std::fs::write(&*v13, &*v14).unwrap();
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = joined_24(v5.clone(), v16);
    let v18: Rc<str> = format_real_26(v4);
    std::fs::write(&*v17, &*v18).unwrap();
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("remote-bin"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = joined_24(v5, v20);
    std::fs::write(&*v21, &*v3).unwrap();
    ()
}
fn outcome_line_27(mut v0: US0, mut v1: Rc<str>) -> Rc<str> {
    match v0 {
        US0::US0_Dispatched(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v7: Rc<str> = string_concat(&["rig job start ok id=", &*v3, " machine=", &*v2, " placement=remote"]);
            v7
        }
        US0::US0_RemoteRefused(v15) => {
            let v15: Rc<str> = v15;
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = Rc::<str>::from(v15.replace(&*v18, &*v19));
            let v21: Rc<str> = string_concat(&["rig job start refused machine=", &*v1, " reason=", &*v20]);
            v21
        }
        US0::US0_RemoteUnreachable(v8) => {
            let v8: Rc<str> = v8;
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = Rc::<str>::from(v8.replace(&*v11, &*v12));
            let mut v14: Rc<str> = string_concat(&["rig job start unreachable machine=", &*v1, " reason=", &*v13]);
            v14
        }
    }
}
fn dispatch_run_0(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = 4i32;
    let v2: i32 = command_line_argument_count_2();
    let v3: i32 = v2.wrapping_add(1i32);
    let mut v4: Rc<UH0> = words_from_3(v1, v3);
    let mut v5: US1 = request_of_4(&v4);
    match v5 {
        US1::US1_RequestRead(v8, v9, v10) => {
            let v8: Rc<str> = v8;
            let mut v9: Rc<str> = v9;
            let v10: Rc<UH0> = v10;
            let v11: Rc<str> = machine_binary_7(v0.clone(), v8.clone(), v9);
            let v12: Rc<str> = remote_line_8(&v8, v11.clone(), v10);
            let v13: Rc<RefCell<Vec<(bool, Rc<str>, Rc<str>)>>> = rig_job_remote_gate::rig_job_remote_gated_line(&v0, &v8, &v12);
            let (mut v14, mut v15, mut v16): (bool, Rc<str>, Rc<str>) = v13.borrow()[0i32 as usize].clone();
            let v17: bool = v14 == false;
            if v17 {
                let v20: Rc<str> = string_concat(&["rig job start busy machine=", &*v8, " reason=", &*v16]);
                println!("{}", v20);
                3i32
            } else {
                let v21: Rc<str> = connect_of_11(v0.clone(), v8.clone());
                let v22: Rc<UH0> = ssh_with_12(v21, v8.clone(), v15);
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh"); } LIT.with(|lit| lit.clone()) };
                let mut v24: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v23)));
                let v25: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v26: Rc<UH0> = method13(v22, v25);
                let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v28: Rc<UH0> = method14(v26, v27);
                method15(v24.clone(), v28);
                let mut v29: Rc<std::io::Result<std::process::Output>> = Rc::new(v24.borrow_mut().output());
                let v30: bool = v29.is_ok();
                let (mut v36, mut v37, mut v38): (bool, i32, Rc<str>) = if v30 {
                    let v31: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v29).as_ref().unwrap().stderr).as_ref());
                    let v32: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v29).as_ref().unwrap().stdout).as_ref());
                    let v33: Rc<str> = string_concat(&[&*v32, &*v31]);
                    let mut v34: i32 = (&*v29).as_ref().unwrap().status.code().unwrap_or(-2);
                    (true, v34, v33)
                } else {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (false, -1i32, v35)
                };
                let mut v39: US0 = outcome_of_16(v8.clone(), v36, v37, v38);
                match &v39 {
                    US0::US0_Dispatched(v40, v41) => {
                        let v40: Rc<str> = v40.clone();
                        let v41: Rc<str> = v41.clone();
                        let v42: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                        recorded_23(v0, v40, v41, v11, v42)
                    }
                    _ => {
                        ()
                    }
                };
                let v43: Rc<str> = outcome_line_27(v39.clone(), v8);
                println!("{}", v43);
                outcome_code_1(v39)
            }
        }
        US1::US1_RequestRefused(v6) => {
            let mut v6: Rc<str> = v6;
            let v7: Rc<str> = string_concat(&["rig error: job start ", &*v6]);
            eprintln!("{}", v7);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        dispatch_run_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn file_exists_29(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method30(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method31(mut v0: Rc<UH0>, mut v1: bool) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v5: bool = if v1 {
                    plain_argument_6(v2)
                } else {
                    false
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
fn method33(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method33(v3, v1);
            let mut v6: Rc<str> = string_concat(&["\"", &*v2, "\""]);
            Rc::new(UH0::UH0_Cons(v6, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn forward_line_32(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<str> {
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = method33(v1, v3);
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v6: i32 = 0i32;
    let v7: i32 = method10(v5.clone(), v4, v6);
    let v8: Rc<Vec<Rc<str>>> = Rc::new(v5.borrow().clone());
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
    let v12: Rc<str> = string_concat(&[&*v0, " job ", &*v11]);
    v12
}
fn forward_code_34(mut v0: bool, mut v1: i32) -> i32 {
    let v2: bool = v0 == false;
    let mut v4: bool = if v2 {
        true
    } else {
        let v3: bool = v1 == 255i32;
        v3
    };
    if v4 {
        4i32
    } else {
        v1
    }
}
fn forward_run_28(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: i32 = 3i32;
    let v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v2 as usize).unwrap_or_default());
    let mut v4: Rc<str> = joined_24(v1.clone(), v3);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("remote-bin"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = joined_24(v4, v5);
    let v7: bool = file_exists_29(v6.clone());
    let v21: Rc<str> = if v7 {
        let mut v9: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v6);
        let v11: Vec<u8> = v9.unwrap();
        let v12: Vec<u8> = method30(v11);
        let mut v14: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v12);
        let v16: std::string::String = v14.unwrap();
        let v18: Rc<str> = Rc::<str>::from(String::as_str(&v16));
        let v19: Rc<str> = Rc::<str>::from((v18).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        v19
    } else {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
        v20
    };
    let v22: i32 = 2i32;
    let v23: i32 = command_line_argument_count_2();
    let mut v24: i32 = v23.wrapping_add(1i32);
    let v25: Rc<UH0> = words_from_3(v22, v24);
    let v26: bool = true;
    let v27: bool = method31(v25.clone(), v26);
    let v28: bool = v27 == false;
    if v28 {
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: job forwarding refuses quote characters in arguments"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v29);
        2i32
    } else {
        let v30: Rc<str> = connect_of_11(v1, v0.clone());
        let v31: Rc<str> = forward_line_32(v21, v25);
        let v32: Rc<UH0> = ssh_with_12(v30, v0.clone(), v31);
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh"); } LIT.with(|lit| lit.clone()) };
        let mut v34: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v33)));
        let v35: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v36: Rc<UH0> = method13(v32, v35);
        let v37: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v38: Rc<UH0> = method14(v36, v37);
        method15(v34.clone(), v38);
        let mut v39: Rc<std::io::Result<std::process::Output>> = Rc::new(v34.borrow_mut().output());
        let v40: bool = v39.is_ok();
        let (mut v46, mut v47, mut v48): (bool, i32, Rc<str>) = if v40 {
            let v41: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v39).as_ref().unwrap().stderr).as_ref());
            let v42: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v39).as_ref().unwrap().stdout).as_ref());
            let v43: Rc<str> = string_concat(&[&*v42, &*v41]);
            let v44: i32 = (&*v39).as_ref().unwrap().status.code().unwrap_or(-2);
            (true, v44, v43)
        } else {
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, -1i32, v45)
        };
        let v49: i32 = forward_code_34(v46, v47);
        let v50: bool = v49 == 4i32;
        let v53: bool = if v50 {
            let v51: Rc<str> = Rc::<str>::from((v48).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let v52: bool = &*v51 == "";
            v52
        } else {
            false
        };
        let v56: Rc<str> = if v53 {
            let v54: Rc<str> = string_concat(&["rig job unreachable machine=", &*v0]);
            v54
        } else {
            let v55: Rc<str> = Rc::<str>::from((v48).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            v55
        };
        println!("{}", v56);
        v49
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        forward_run_28(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn listed_on_36(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>) -> i32 {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v3));
    let mut v6: Rc<str> = forward_line_32(v2, v5);
    let v7: Rc<UH0> = ssh_with_12(v0, v1.clone(), v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v8)));
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH0> = method13(v7, v10);
    let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH0> = method14(v11, v12);
    method15(v9.clone(), v13);
    let mut v14: Rc<std::io::Result<std::process::Output>> = Rc::new(v9.borrow_mut().output());
    let v15: bool = v14.is_ok();
    let (mut v21, mut v22, mut v23): (bool, i32, Rc<str>) = if v15 {
        let v16: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v14).as_ref().unwrap().stderr).as_ref());
        let v17: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v14).as_ref().unwrap().stdout).as_ref());
        let v18: Rc<str> = string_concat(&[&*v17, &*v16]);
        let v19: i32 = (&*v14).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v19, v18)
    } else {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v20)
    };
    let mut v24: i32 = forward_code_34(v21, v22);
    let v25: bool = v24 == 4i32;
    let v28: bool = if v25 {
        let v26: Rc<str> = Rc::<str>::from((v23).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        let v27: bool = &*v26 == "";
        v27
    } else {
        false
    };
    let v31: Rc<str> = if v28 {
        let mut v29: Rc<str> = string_concat(&["rig job unreachable machine=", &*v1]);
        v29
    } else {
        let v30: Rc<str> = Rc::<str>::from((v23).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        v30
    };
    println!("{}", v31);
    v24
}
fn list_on_run_35(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = 4i32;
    let v2: i32 = command_line_argument_count_2();
    let v3: i32 = v2.wrapping_add(1i32);
    let mut v4: Rc<UH0> = words_from_3(v1, v3);
    match Rc::unwrap_or_clone(v4) {
        UH0::UH0_Cons(v5, v6) => {
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<UH0> = v6;
            match &*v6 {
                UH0::UH0_Cons(v7, v8) => {
                    let v7: Rc<str> = v7.clone();
                    let v8: Rc<UH0> = v8.clone();
                    match Rc::unwrap_or_clone(v8) {
                        UH0::UH0_Cons(v9, v10) => {
                            let mut v9: Rc<str> = v9;
                            let v10: Rc<UH0> = v10;
                            let v11: bool = &*v7 == "--bin";
                            if v11 {
                                let v12: Rc<str> = connect_of_11(v0.clone(), v5.clone());
                                let v13: Rc<str> = machine_binary_7(v0, v5.clone(), v9);
                                listed_on_36(v12, v5, v13, v10)
                            } else {
                                let v15: Rc<str> = connect_of_11(v0.clone(), v5.clone());
                                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                                let v17: Rc<str> = machine_binary_7(v0, v5.clone(), v16);
                                listed_on_36(v15, v5, v17, v6)
                            }
                        }
                        _ => {
                            let v20: Rc<str> = connect_of_11(v0.clone(), v5.clone());
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                            let v22: Rc<str> = machine_binary_7(v0, v5.clone(), v21);
                            listed_on_36(v20, v5, v22, v6)
                        }
                    }
                }
                _ => {
                    let v25: Rc<str> = connect_of_11(v0.clone(), v5.clone());
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = machine_binary_7(v0, v5.clone(), v26);
                    listed_on_36(v25, v5, v27, v6)
                }
            }
        }
        UH0::UH0_Nil => {
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: job list --on needs a machine (an ssh alias)"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v30);
            2i32
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        list_on_run_35(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_job_dispatch_run(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_job_forward_run(v0: &str, v1: &str) -> i32 {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_job_list_on(v0: &str) -> i32 {
    closure3()(Rc::<str>::from(v0))
}
use rig_job_bin::SpiralShape_97df63ebabc3 as Mut1;
pub type SpiralShape_652accfc6a8d = Mut0;
