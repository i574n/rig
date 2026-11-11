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
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_MacroLong,
    US0_MacroBlock,
    US0_MacroFlow,
    US0_MacroCast,
    US0_MacroArith,
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<str>, Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(US0),
    US2_None,
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn file_exists_3(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn directory_exists_4(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn entry_names_5(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Vec<std::string::String> = v2.unwrap();
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: bool = true; let _vec_fold_ = v4.into_iter().fold(v5, |acc, x| { //;
    let v9: Rc<UH0> = acc;
    let mut v11: std::string::String = x;
    let v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v9));
    let mut v16: bool = true; v15 });
    let v18: Rc<UH0> = _vec_fold_;
    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method2(v18, v19)
}
fn method8(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: &str = &*v0;
    let mut v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let v9: &str = &*v1;
    let mut v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method8(v13);
    let mut v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn method6(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                let mut v6: bool = v3.starts_with(&*v5);
                let v8: bool = if v6 {
                    true
                } else {
                    let mut v7: bool = &*v3 == "target";
                    v7
                };
                let mut v10: bool = if v8 {
                    true
                } else {
                    let v9: bool = &*v3 == "node_modules";
                    v9
                };
                let v12: bool = if v10 {
                    true
                } else {
                    let mut v11: bool = &*v3 == "_build";
                    v11
                };
                let mut v14: bool = if v12 {
                    true
                } else {
                    let v13: bool = &*v3 == "deps";
                    v13
                };
                let mut v16: bool = if v14 {
                    true
                } else {
                    let v15: bool = &*v3 == "bin";
                    v15
                };
                let v18: bool = if v16 {
                    true
                } else {
                    let mut v17: bool = &*v3 == "obj";
                    v17
                };
                let v21: Rc<UH0> = if v18 {
                    v2
                } else {
                    let v19: Rc<str> = method7(v0.clone(), v3);
                    Rc::new(UH0::UH0_Cons(v19, v2))
                };
                (v1, v2) = (v4, v21);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn spi_files_1(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = file_exists_3(v4.clone());
                if v6 {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v8: bool = v4.ends_with(&*v7);
                    let mut v10: Rc<UH0> = if v8 {
                        Rc::new(UH0::UH0_Cons(v4, v1))
                    } else {
                        v1
                    };
                    (v0, v1) = (v5, v10);
                    continue;
                } else {
                    let v12: bool = directory_exists_4(v4.clone());
                    if v12 {
                        let v13: Rc<UH0> = entry_names_5(v4.clone());
                        let mut v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v15: Rc<UH0> = method2(v13, v14);
                        let mut v16: Rc<UH0> = method6(v4, v15, v5);
                        v0 = v16;
                        continue;
                    } else {
                        v0 = v5;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method2(v1, v2);
            }
        }
    }
}
fn method9(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<str>>>>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                v0.borrow_mut().push(v2);
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
fn files_under_0(mut v0: Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    let mut v1: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v2));
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH0> = spi_files_1(v3, v4);
    method9(v1.clone(), v5);
    let mut v6: std::rc::Rc<Vec<std::rc::Rc<str>>> = std::rc::Rc::new(v1.borrow().clone());
    v6
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>>> = Rc::new(move |mut v0: Rc<str>| -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
        files_under_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method13(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: US0, mut v4: &Rc<str>, mut v5: Rc<UH1>) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v5) {
            UH1::UH1_Cons(v7, v8, v9, v10) => {
                let mut v7: Rc<str> = v7;
                let v8: Rc<str> = v8;
                let v9: Rc<str> = v9;
                let mut v10: Rc<UH1> = v10;
                let mut v11: bool = &*v7 == &*v0;
                let v13: bool = if v11 {
                    let v12: bool = &*v8 == &*v2;
                    v12
                } else {
                    false
                };
                let v21: bool = if v13 {
                    let v19: Rc<str> = match &v3 {
                        US0::US0_MacroArith => {
                            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arith"); } LIT.with(|lit| lit.clone()) };
                            v18
                        }
                        US0::US0_MacroBlock => {
                            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("block"); } LIT.with(|lit| lit.clone()) };
                            v15
                        }
                        US0::US0_MacroCast => {
                            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cast"); } LIT.with(|lit| lit.clone()) };
                            v17
                        }
                        US0::US0_MacroFlow => {
                            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("flow"); } LIT.with(|lit| lit.clone()) };
                            v16
                        }
                        US0::US0_MacroLong => {
                            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("long"); } LIT.with(|lit| lit.clone()) };
                            v14
                        }
                    };
                    let v20: bool = &*v9 == &*v19;
                    v20
                } else {
                    false
                };
                if v21 {
                    return US1::US1_Some(v7, v8, v9);
                } else {
                    v5 = v10;
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US1::US1_None;
            }
        }
    }
}
fn known_12(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: US0, mut v5: Rc<str>) -> bool {
    let mut v6: US1 = method13(v1, v2, v3, v4, &v5, v0);
    let mut v7: bool = match v6 {
        US1::US1_None => {
            true
        }
        _ => {
            false
        }
    };
    let v8: bool = v7 != true;
    v8
}
fn method15(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn words_in_17(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v3 <= v1;
        if v4 {
            let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method2(v2, v5);
        } else {
            let mut v7: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v8: i32 = v1.wrapping_add(1i32);
            let v9: bool = &*v7 == "";
            let mut v11: Rc<UH0> = if v9 {
                v2
            } else {
                Rc::new(UH0::UH0_Cons(v7, v2))
            };
            (v1, v2) = (v8, v11);
            continue;
        }
    }
}
fn closure3() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn known_of_16(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v4: i32 = 0i32;
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = words_in_17(&v3, v4, v5);
    match Rc::unwrap_or_clone(v6) {
        UH0::UH0_Cons(v7, v8) => {
            let mut v7: Rc<str> = v7;
            let v8: Rc<UH0> = v8;
            match Rc::unwrap_or_clone(v8) {
                UH0::UH0_Cons(v9, v10) => {
                    let v9: Rc<str> = v9;
                    let mut v10: Rc<UH0> = v10;
                    match Rc::unwrap_or_clone(v10) {
                        UH0::UH0_Cons(v11, v12) => {
                            let mut v11: Rc<str> = v11;
                            let v12: Rc<UH0> = v12;
                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                            let mut v14: Option<usize> = v7.rfind(&*v13);
                            let v15: Option<usize> = method15(v14);
                            let mut v16: Rc<dyn Fn((usize)) -> i32> = closure3();
                            let mut v17: Option<i32> = v15.map(|x| v16(x));
                            let v19: i32 = v17.unwrap_or(-1i32);
                            let v20: bool = &*v9 == "long";
                            let v28: bool = if v20 {
                                true
                            } else {
                                let v21: bool = &*v9 == "block";
                                if v21 {
                                    true
                                } else {
                                    let v22: bool = &*v9 == "flow";
                                    if v22 {
                                        true
                                    } else {
                                        let v23: bool = &*v9 == "cast";
                                        if v23 {
                                            true
                                        } else {
                                            let v24: bool = &*v9 == "arith";
                                            v24
                                        }
                                    }
                                }
                            };
                            let v39: bool = if v28 {
                                let v29: bool = &*v11 == "long";
                                let v37: bool = if v29 {
                                    true
                                } else {
                                    let v30: bool = &*v11 == "block";
                                    if v30 {
                                        true
                                    } else {
                                        let v31: bool = &*v11 == "flow";
                                        if v31 {
                                            true
                                        } else {
                                            let v32: bool = &*v11 == "cast";
                                            if v32 {
                                                true
                                            } else {
                                                let v33: bool = &*v11 == "arith";
                                                v33
                                            }
                                        }
                                    }
                                };
                                let v38: bool = v37 == false;
                                v38
                            } else {
                                false
                            };
                            let v40: bool = v19 <= 0i32;
                            if v40 {
                                US1::US1_None
                            } else {
                                let v42: usize = ((0i32) as usize);
                                let v43: usize = ((v19) as usize);
                                let v44: Rc<str> = Rc::<str>::from(&(v7)[v42..v43]);
                                if v39 {
                                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    US1::US1_Some(v44, v45, v9)
                                } else {
                                    US1::US1_Some(v44, v9, v11)
                                }
                            }
                        }
                        _ => {
                            US1::US1_None
                        }
                    }
                }
                _ => {
                    US1::US1_None
                }
            }
        }
        _ => {
            US1::US1_None
        }
    }
}
fn rows_of_file_14(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v4: i32 = (v1.clone().borrow().len() as i32);
        let mut v5: bool = v2 >= v4;
        if v5 {
            return v3;
        } else {
            let mut v6: Rc<str> = v1.borrow()[v2 as usize].clone();
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v8: Option<usize> = v6.find(&*v7);
            let v9: Option<usize> = method15(v8);
            let mut v10: Rc<dyn Fn((usize)) -> i32> = closure2();
            let mut v11: Option<i32> = v9.map(|x| v10(x));
            let v13: i32 = v11.unwrap_or(-1i32);
            let mut v14: bool = v13 < 0i32;
            let v18: Rc<str> = if v14 {
                v6
            } else {
                let v15: usize = ((0i32) as usize);
                let mut v16: usize = ((v13) as usize);
                let mut v17: Rc<str> = Rc::<str>::from(&(v6)[v15..v16]);
                v17
            };
            let v20: Rc<str> = string_concat(&[&*v0, ":", &*v18]);
            let v21: US1 = known_of_16(v20);
            match v21 {
                US1::US1_None => {
                    let v28: i32 = v2.wrapping_add(1i32);
                    v2 = v28;
                    continue;
                }
                US1::US1_Some(v22, v23, v24) => {
                    let v22: Rc<str> = v22;
                    let v23: Rc<str> = v23;
                    let v24: Rc<str> = v24;
                    let v25: i32 = v2.wrapping_add(1i32);
                    let v26: Rc<UH1> = Rc::new(UH1::UH1_Cons(v22, v23, v24, v3));
                    (v2, v3) = (v25, v26);
                    continue;
                }
            }
        }
    }
}
fn known_text_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: US0, mut v5: Rc<str>) -> bool {
    let mut v7: Rc<str> = string_concat(&["\n", &*v1, ":"]);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = Rc::<str>::from(v0.replace(&*v8, &*v9));
    let mut v11: Rc<str> = string_concat(&["\n", &*v10]);
    let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v11.split(&*v7).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v13: i32 = 1i32;
    let mut v14: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v15: Rc<UH1> = rows_of_file_14(v1.clone(), &v12, v13, v14);
    known_12(v15, v1, v2, v3, v4, v5)
}
fn known_in_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32) -> bool {
    let mut v4: bool = v3 == 0i32;
    let v24: US2 = if v4 {
        let mut v5: US0 = US0::US0_MacroLong;
        US2::US2_Some(v5)
    } else {
        let mut v7: bool = v3 == 1i32;
        if v7 {
            let v8: US0 = US0::US0_MacroBlock;
            US2::US2_Some(v8)
        } else {
            let mut v10: bool = v3 == 2i32;
            if v10 {
                let mut v11: US0 = US0::US0_MacroFlow;
                US2::US2_Some(v11)
            } else {
                let v13: bool = v3 == 3i32;
                if v13 {
                    let mut v14: US0 = US0::US0_MacroCast;
                    US2::US2_Some(v14)
                } else {
                    let mut v16: bool = v3 == 4i32;
                    if v16 {
                        let mut v17: US0 = US0::US0_MacroArith;
                        US2::US2_Some(v17)
                    } else {
                        US2::US2_None
                    }
                }
            }
        }
    };
    match v24 {
        US2::US2_None => {
            false
        }
        US2::US2_Some(v25) => {
            let v25: US0 = v25;
            let v26: i32 = 0i32;
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            known_text_11(v0, v1, v26, v2, v25, v27)
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32| -> bool {
        known_in_10(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_lint_walk_files(v0: &str) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_lint_known(v0: &str, v1: &str, v2: &str, v3: i32) -> bool {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3)
}
