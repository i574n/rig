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
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_PathFirst(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US1, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<UH2>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH2> }
#[derive(Clone)]
enum US2 {
    US2_Matches,
    US2_Shadowed(Rc<str>),
    US2_Elsewhere(Rc<str>),
    US2_Missing,
}
pub struct Mut1 { pub l0: Rc<str> }
fn env_refused_1(mut v0: Rc<str>) -> i32 {
    let v3: Rc<str> = string_concat(&["kit error: env check ", &*v0, "; usage: env check [<tool>=<pinned-path>...] (each tool is looked up on PATH in order; the first copy found must be the pinned one; with no pins given, the PathFirst rows of <kit config>/path_first.spi are the pins)"]);
    eprintln!("{}", v3);
    2i32
}
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
fn method5(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: US1 = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH0> = method5(v3, v1);
            let (mut v17, mut v18): (Rc<str>, Rc<str>) = match v2 {
                US1::US1_PathFirst(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let mut v6: Rc<str> = v6;
                    let mut v7: Rc<str> = string_concat(&[&*v6, "/"]);
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                    let mut v9: bool = v5.contains(&*v8);
                    let v15: Rc<str> = if v9 {
                        v5.clone()
                    } else {
                        let v10: bool = cfg!(windows);
                        let v13: Rc<str> = if v10 {
                            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".exe"); } LIT.with(|lit| lit.clone()) };
                            v11
                        } else {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v12
                        };
                        let v14: Rc<str> = string_concat(&[&*v5, &*v13]);
                        v14
                    };
                    let v16: Rc<str> = string_concat(&[&*v7, &*v15]);
                    (v5, v16)
                }
            };
            Rc::new(UH0::UH0_Cons(v17, v18, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn config_choice_7(mut v0: Rc<str>, mut v1: bool, mut v2: bool) -> Rc<str> {
    let v4: bool = if v1 {
        true
    } else {
        let v3: bool = v2 == false;
        v3
    };
    if v4 {
        let v5: Rc<str> = string_concat(&[&*v0, "/rig"]);
        v5
    } else {
        let mut v6: Rc<str> = string_concat(&[&*v0, "/kit"]);
        v6
    }
}
fn get_environment_variable_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn directory_exists_9(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
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
fn config_dir_6() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
    let v1: Rc<str> = get_environment_variable_8(v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("APPDATA"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = get_environment_variable_8(v2);
    let v4: bool = &*v3 != "";
    let mut v8: Rc<str> = if v4 {
        v3
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HOME"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = get_environment_variable_8(v5);
        let mut v7: Rc<str> = string_concat(&[&*v6, "/.config"]);
        v7
    };
    let mut v9: Rc<str> = string_concat(&[&*v8, "/rig"]);
    let v10: bool = directory_exists_9(v9);
    let v11: bool = &*v1 != "";
    if v11 {
        v1
    } else {
        let v12: Rc<str> = string_concat(&[&*v8, "/kit"]);
        let v13: bool = directory_exists_9(v12);
        config_choice_7(v8, v10, v13)
    }
}
fn method12(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method14(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn method15(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH2> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH2::UH2_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method15(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH2> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH2> = Rc::new(UH2::UH2_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH2> = v9.borrow().l1.clone();
        let v17: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        method14(v16, v17)
    }
}
fn method16(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: US1 = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method16(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method11(mut v0: Rc<UH2>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = PathFirst ("); } LIT.with(|lit| lit.clone()) };
                let v5: Option<usize> = v2.find(&*v4);
                let mut v6: Option<usize> = method12(v5);
                let mut v7: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v8: Option<i32> = v6.map(|x| v7(x));
                let v10: i32 = v8.unwrap_or(-1i32);
                let v11: bool = v10 < 0i32;
                let v42: Rc<UH1> = if v11 {
                    v1
                } else {
                    let v12: i32 = v10.wrapping_add(14i32);
                    let v13: i32 = (v2.clone().len() as i32);
                    let v14: i32 = v13.wrapping_sub(v10);
                    let v15: i32 = v14.wrapping_sub(14i32);
                    let v16: usize = ((v12) as usize);
                    let v17: i32 = v12.wrapping_add(v15);
                    let v18: usize = ((v17) as usize);
                    let v19: Rc<str> = Rc::<str>::from(&(v2)[v16..v18]);
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<UH2> = split_list_13(v20, v19);
                    match Rc::unwrap_or_clone(v21) {
                        UH2::UH2_Cons(v22, v23) => {
                            let v22: Rc<str> = v22;
                            let v23: Rc<UH2> = v23;
                            match Rc::unwrap_or_clone(v23) {
                                UH2::UH2_Cons(v24, v25) => {
                                    let mut v24: Rc<str> = v24;
                                    let mut v25: Rc<UH2> = v25;
                                    match Rc::unwrap_or_clone(v25) {
                                        UH2::UH2_Cons(v26, v27) => {
                                            let v26: Rc<str> = v26;
                                            let v27: Rc<UH2> = v27;
                                            match Rc::unwrap_or_clone(v27) {
                                                UH2::UH2_Cons(v28, v29) => {
                                                    let v28: Rc<str> = v28;
                                                    let v29: Rc<UH2> = v29;
                                                    let v30: bool = &*v24 != "";
                                                    let v32: bool = if v30 {
                                                        let v31: bool = &*v28 != "";
                                                        v31
                                                    } else {
                                                        false
                                                    };
                                                    if v32 {
                                                        let v33: US1 = US1::US1_PathFirst(v24, v28);
                                                        let v34: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                                                        let v35: Rc<UH1> = Rc::new(UH1::UH1_Cons(v33, v34));
                                                        method16(v1, v35)
                                                    } else {
                                                        v1
                                                    }
                                                }
                                                _ => {
                                                    v1
                                                }
                                            }
                                        }
                                        _ => {
                                            v1
                                        }
                                    }
                                }
                                _ => {
                                    v1
                                }
                            }
                        }
                        _ => {
                            v1
                        }
                    }
                };
                (v0, v1) = (v3, v42);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn path_first_rows_10(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH2> = split_list_13(v1, v4);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method11(v5, v6)
}
fn config_pins_4() -> Rc<UH0> {
    let mut v0: Rc<str> = config_dir_6();
    let v1: Rc<str> = string_concat(&[&*v0, "/path_first.spi"]);
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v1).unwrap_or_default().as_str());
    let v3: Rc<UH1> = path_first_rows_10(v2);
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method5(v3, v4)
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v1));
                (v0, v1) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn pins_from_17(mut v0: i32, mut v1: i32, mut v2: Rc<UH0>, mut v3: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    loop {
        let v4: bool = v0 >= v1;
        let mut v6: bool = if v4 {
            true
        } else {
            let v5: bool = &*v3 != "";
            v5
        };
        if v6 {
            let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v8: Rc<UH0> = method18(v2, v7);
            return (v8, v3);
        } else {
            let mut v9: bool = v0 >= 0i32;
            let v12: bool = if v9 {
                let v10: i32 = command_line_argument_count_2();
                let v11: bool = v0 < v10;
                v11
            } else {
                false
            };
            let v16: US0 = if v12 {
                let v13: Rc<str> = command_line_argument_3(v0);
                US0::US0_Some(v13)
            } else {
                US0::US0_None
            };
            let v20: Rc<str> = match v16 {
                US0::US0_None => {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v18
                }
                US0::US0_Some(v17) => {
                    let v17: Rc<str> = v17;
                    v17
                }
            };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
            let v22: Option<usize> = v20.find(&*v21);
            let v23: Option<usize> = method12(v22);
            let mut v24: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v25: Option<i32> = v23.map(|x| v24(x));
            let v27: i32 = v25.unwrap_or(-1i32);
            let v28: bool = v27 <= 0i32;
            let (mut v41, mut v42): (Rc<str>, Rc<str>) = if v28 {
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v20, v29)
            } else {
                let v30: usize = ((0i32) as usize);
                let v31: usize = ((v27) as usize);
                let v32: Rc<str> = Rc::<str>::from(&(v20)[v30..v31]);
                let v33: i32 = v27.wrapping_add(1i32);
                let v34: i32 = (v20.clone().len() as i32);
                let v35: i32 = v34.wrapping_sub(v27);
                let v36: i32 = v35.wrapping_sub(1i32);
                let v37: usize = ((v33) as usize);
                let v38: i32 = v33.wrapping_add(v36);
                let v39: usize = ((v38) as usize);
                let v40: Rc<str> = Rc::<str>::from(&(v20)[v37..v39]);
                (v32, v40)
            };
            let v43: bool = &*v42 == "";
            if v43 {
                let v46: bool = if v9 {
                    let v44: i32 = command_line_argument_count_2();
                    let v45: bool = v0 < v44;
                    v45
                } else {
                    false
                };
                let v50: US0 = if v46 {
                    let v47: Rc<str> = command_line_argument_3(v0);
                    US0::US0_Some(v47)
                } else {
                    US0::US0_None
                };
                let v54: Rc<str> = match v50 {
                    US0::US0_None => {
                        let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v52
                    }
                    US0::US0_Some(v51) => {
                        let v51: Rc<str> = v51;
                        v51
                    }
                };
                let v55: Rc<str> = string_concat(&["expects tool=pinned-path, received ", &*v54]);
                return (v2, v55);
            } else {
                let v56: i32 = v0.wrapping_add(1i32);
                let v57: Rc<UH0> = Rc::new(UH0::UH0_Cons(v41, v42, v2));
                let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v0, v2, v3) = (v56, v57, v58);
                continue;
            }
        }
    }
}
fn loop_21(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(1i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_22(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(1i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn file_exists_23(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn found_on_path_20(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<UH2>) -> Rc<UH2> {
    loop {
        let v4: bool = v1 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: Rc<str> = loop_21(v5);
            let mut v7: Rc<str> = loop_22(v6);
            let mut v9: Rc<str> = string_concat(&[&*v7, "/", &*v2]);
            let v10: i32 = v1.wrapping_sub(1i32);
            let v11: bool = &*v7 != "";
            let v13: bool = if v11 {
                file_exists_23(v9.clone())
            } else {
                false
            };
            let v15: Rc<UH2> = if v13 {
                Rc::new(UH2::UH2_Cons(v9, v3))
            } else {
                v3
            };
            (v1, v3) = (v10, v15);
            continue;
        }
    }
}
fn listed_25(mut v0: Rc<UH2>, mut v1: Rc<str>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = Rc::<str>::from(v2.replace(&*v4, &*v5));
                let mut v7: Rc<str> = Rc::<str>::from(v6.to_lowercase());
                let mut v8: Rc<str> = Rc::<str>::from(v1.replace(&*v4, &*v5));
                let mut v9: Rc<str> = Rc::<str>::from(v8.to_lowercase());
                let v10: bool = &*v7 == &*v9;
                if v10 {
                    return true;
                } else {
                    v0 = v3;
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return false;
            }
        }
    }
}
fn drift_of_24(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH2>) -> US2 {
    match &*v2 {
        UH2::UH2_Cons(v4, v5) => {
            let v4: Rc<str> = v4.clone();
            let v5: Rc<UH2> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = Rc::<str>::from(v4.replace(&*v6, &*v7));
            let mut v9: Rc<str> = Rc::<str>::from(v8.to_lowercase());
            let v10: Rc<str> = Rc::<str>::from(v1.replace(&*v6, &*v7));
            let v11: Rc<str> = Rc::<str>::from(v10.to_lowercase());
            let v12: bool = &*v9 == &*v11;
            if v12 {
                US2::US2_Matches
            } else {
                let v14: bool = listed_25(v2, v1);
                if v14 {
                    US2::US2_Shadowed(v4)
                } else {
                    US2::US2_Elsewhere(v4)
                }
            }
        }
        UH2::UH2_Nil => {
            US2::US2_Missing
        }
    }
}
fn checked_19(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: u64) -> (Rc<str>, u64) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5, v6) => {
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let mut v6: Rc<UH0> = v6;
                let mut v7: i32 = (v0.clone().borrow().len() as i32);
                let mut v8: i32 = v7.wrapping_sub(1i32);
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v4.contains(&*v9);
                let v16: Rc<str> = if v10 {
                    v4.clone()
                } else {
                    let v11: bool = cfg!(windows);
                    let v14: Rc<str> = if v11 {
                        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".exe"); } LIT.with(|lit| lit.clone()) };
                        v12
                    } else {
                        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v13
                    };
                    let v15: Rc<str> = string_concat(&[&*v4, &*v14]);
                    v15
                };
                let v17: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                let v18: Rc<UH2> = found_on_path_20(v0, v8, v16, v17);
                let v19: US2 = drift_of_24(&v4, v5.clone(), v18);
                let v40: Rc<str> = match &v19 {
                    US2::US2_Elsewhere(v27) => {
                        let v27: Rc<str> = v27.clone();
                        let v32: Rc<str> = string_concat(&[&*v4, " unpinned: PATH finds ", &*v27, ", and the pinned ", &*v5, " is not on PATH"]);
                        v32
                    }
                    US2::US2_Matches => {
                        let v21: Rc<str> = string_concat(&[&*v4, " ok ", &*v5]);
                        v21
                    }
                    US2::US2_Missing => {
                        let v36: Rc<str> = string_concat(&[&*v4, " missing: no ", &*v4, " on PATH, pinned ", &*v5]);
                        v36
                    }
                    US2::US2_Shadowed(v22) => {
                        let v22: Rc<str> = v22.clone();
                        let v26: Rc<str> = string_concat(&[&*v4, " shadowed: ", &*v22, " comes first on PATH, pinned ", &*v5]);
                        v26
                    }
                };
                let v42: Rc<str> = string_concat(&[&*v2, &*v40, "\n"]);
                let v43: bool = match v19 {
                    US2::US2_Matches => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v45: u64 = if v43 {
                    v3
                } else {
                    let v44: u64 = v3.wrapping_add(1u64);
                    v44
                };
                (v1, v2, v3) = (v6, v42, v45);
                continue;
            }
            UH0::UH0_Nil => {
                return (v2, v3);
            }
        }
    }
}
fn loop_26(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(1i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method27(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method29(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_28(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method29(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_30(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method29(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn env_run_0() -> i32 {
    let mut v0: i32 = command_line_argument_count_2();
    let v1: i32 = command_line_argument_count_2();
    let v2: bool = 1i32 < v1;
    let mut v7: US0 = if v2 {
        let v3: i32 = 1i32;
        let v4: Rc<str> = command_line_argument_3(v3);
        US0::US0_Some(v4)
    } else {
        US0::US0_None
    };
    let v11: Rc<str> = match v7 {
        US0::US0_None => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v9
        }
        US0::US0_Some(v8) => {
            let mut v8: Rc<str> = v8;
            v8
        }
    };
    let v12: bool = &*v11 != "check";
    if v12 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: env usage: env check [<tool>=<pinned-path>...] (each tool is looked up on PATH in order; the first copy found must be the pinned one; with no pins given, the PathFirst rows of <kit config>/path_first.spi are the pins)"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v13);
        2i32
    } else {
        let v14: bool = v0 < 3i32;
        let (mut v22, mut v23): (Rc<UH0>, Rc<str>) = if v14 {
            let v15: Rc<UH0> = config_pins_4();
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v15, v16)
        } else {
            let v17: i32 = 2i32;
            let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            pins_from_17(v17, v0, v18, v19)
        };
        let mut v24: bool = match &*v22 {
            UH0::UH0_Nil => {
                true
            }
            _ => {
                false
            }
        };
        let v26: bool = if v24 {
            let mut v25: bool = &*v23 == "";
            v25
        } else {
            false
        };
        if v26 {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no pins given and no PathFirst rows in the kit config"); } LIT.with(|lit| lit.clone()) };
            env_refused_1(v27)
        } else {
            let v29: bool = &*v23 != "";
            if v29 {
                env_refused_1(v23)
            } else {
                let v31: bool = cfg!(windows);
                let v34: Rc<str> = if v31 {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
                    v32
                } else {
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                    v33
                };
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PATH"); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = get_environment_variable_8(v35);
                let v37: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v36.split(&*v34).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v39: u64 = 0u64;
                let (mut v40, mut v41): (Rc<str>, u64) = checked_19(&v37, v22.clone(), v38, v39);
                let v42: Rc<str> = loop_26(v40);
                println!("{}", v42);
                let v43: bool = v41 == 0u64;
                let v46: Rc<str> = if v43 {
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                    v44
                } else {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("drift"); } LIT.with(|lit| lit.clone()) };
                    v45
                };
                let v49: i32 = 0i32;
                let v50: i32 = method27(v22, v49);
                let v51: Rc<str> = format_real_28(v50);
                let v54: Rc<str> = format_real_30(v41);
                let v55: Rc<str> = string_concat(&["kit env check ", &*v46, " pins=", &*v51, " drifted=", &*v54]);
                eprintln!("{}", v55);
                if v43 {
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
        env_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_env_run() -> i32 {
    closure0()()
}
pub type SpiralShape_0408665020a9 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
