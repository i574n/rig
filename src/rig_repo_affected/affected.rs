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
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH0>, bool, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<UH0>, bool),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
pub struct Mut0 { pub l0: Rc<str> }
fn command_line_argument_count_1() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_2(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn arguments_from_3(mut v0: i32, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: bool = v0 >= v1;
        if v3 {
            let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method4(v2, v4);
        } else {
            let mut v6: i32 = v0.wrapping_add(1i32);
            let mut v7: bool = v0 >= 0i32;
            let v10: bool = if v7 {
                let v8: i32 = command_line_argument_count_1();
                let mut v9: bool = v0 < v8;
                v9
            } else {
                false
            };
            let v14: US0 = if v10 {
                let v11: Rc<str> = command_line_argument_2(v0);
                US0::US0_Some(v11)
            } else {
                US0::US0_None
            };
            let v18: Rc<str> = match v14 {
                US0::US0_None => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v16
                }
                US0::US0_Some(v15) => {
                    let v15: Rc<str> = v15;
                    v15
                }
            };
            let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v2));
            (v0, v2) = (v6, v19);
            continue;
        }
    }
}
fn directory_exists_6(mut v0: Rc<str>) -> bool {
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
fn directory_names_5(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: bool = directory_exists_6(v0.clone());
    if v1 {
        let mut v3: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v5: Vec<std::string::String> = v3.unwrap();
        let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v8: bool = true; let _vec_fold_ = v5.into_iter().fold(v6, |acc, x| { //;
        let v10: Rc<UH0> = acc;
        let mut v12: std::string::String = x;
        let v14: Rc<str> = Rc::<str>::from(String::as_str(&v12));
        let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v14, v10));
        let v17: bool = true; v16 });
        let v19: Rc<UH0> = _vec_fold_;
        let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method4(v19, v20)
    } else {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    }
}
fn method8(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: bool = v4;
                let mut v5: Rc<UH1> = v5;
                let mut v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn file_exists_10(mut v0: Rc<str>) -> bool {
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
fn method11(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn loop_14(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(1i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn section_uses_13(mut v0: Rc<UH0>, mut v1: bool, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v5, v6) => {
                let mut v5: Rc<str> = v5;
                let mut v6: Rc<UH0> = v6;
                let mut v7: Rc<str> = Rc::<str>::from((v5).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v8: bool = &*v7 == "packages:";
                if v8 {
                    let mut v9: bool = true;
                    (v0, v1) = (v6, v9);
                    continue;
                } else {
                    let v11: bool = &*v7 == "modules:";
                    if v11 {
                        let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        return method4(v2, v12);
                    } else {
                        let v15: bool = if v1 {
                            let v14: bool = &*v7 != "";
                            v14
                        } else {
                            false
                        };
                        if v15 {
                            let v16: bool = true;
                            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                            let v18: bool = v7.starts_with(&*v17);
                            let v25: Rc<str> = if v18 {
                                let v19: i32 = (v7.clone().len() as i32);
                                let v20: i32 = v19.wrapping_sub(1i32);
                                let v21: usize = ((1i32) as usize);
                                let v22: i32 = 1i32.wrapping_add(v20);
                                let v23: usize = ((v22) as usize);
                                let v24: Rc<str> = Rc::<str>::from(&(v7)[v21..v23]);
                                v24
                            } else {
                                v7
                            };
                            let v26: Rc<str> = loop_14(v25);
                            let v27: Rc<UH0> = Rc::new(UH0::UH0_Cons(v26, v2));
                            (v0, v1, v2) = (v6, v16, v27);
                            continue;
                        } else {
                            v0 = v6;
                            continue;
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method4(v2, v3);
            }
        }
    }
}
fn lines_list_15(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let mut v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v4, v6);
            continue;
        }
    }
}
fn project_uses_12(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: i32 = v3.wrapping_sub(1i32);
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = lines_list_15(&v2, v4, v5);
    let mut v7: bool = false;
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    section_uses_13(v6, v7, v8)
}
fn node_at_9(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let mut v3: Rc<str> = string_concat(&[&*v0, "/", &*v1]);
    let v4: Rc<str> = string_concat(&[&*v3, "/package.spiproj"]);
    let mut v5: bool = file_exists_10(v4.clone());
    if v5 {
        let mut v7: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v4);
        let mut v9: Vec<u8> = v7.unwrap();
        let v10: Vec<u8> = method11(v9);
        let mut v12: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v10);
        let v14: std::string::String = v12.unwrap();
        let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
        let v17: Rc<UH0> = project_uses_12(v16);
        let v18: Rc<str> = string_concat(&[&*v3, "/Cargo.toml"]);
        let v19: bool = file_exists_10(v18);
        US1::US1_Some(v1, v17, v19)
    } else {
        US1::US1_None
    }
}
fn nodes_of_7(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v5, v6) => {
                let mut v5: Rc<str> = v5;
                let mut v6: Rc<UH0> = v6;
                let mut v7: US1 = node_at_9(v0.clone(), v5);
                match v7 {
                    US1::US1_None => {
                        v1 = v6;
                        continue;
                    }
                    US1::US1_Some(v8, v9, v10) => {
                        let v8: Rc<str> = v8;
                        let mut v9: Rc<UH0> = v9;
                        let v10: bool = v10;
                        let v11: Rc<UH1> = Rc::new(UH1::UH1_Cons(v8, v9, v10, v2));
                        (v1, v2) = (v6, v11);
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let mut v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                return method8(v2, v3);
            }
        }
    }
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    Rc::new(move |mut v2: Rc<UH0>| -> US2 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US2> = method18(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method18(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    closure1(v0, v1)
}
fn node_named_19(mut v0: Rc<str>, mut v1: Rc<UH1>) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let mut v5: bool = v5;
                let mut v6: Rc<UH1> = v6;
                let mut v7: bool = &*v3 == &*v0;
                if v7 {
                    return US1::US1_Some(v3, v4, v5);
                } else {
                    v1 = v6;
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US1::US1_None;
            }
        }
    }
}
fn prepended_20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = prepended_20(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn reaches_17(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: i32 = 0i32;
                let mut v7: Rc<dyn Fn(Rc<UH0>) -> US2> = method18(v4.clone(), v6);
                let v8: US2 = v7(v1.clone());
                let mut v9: bool = match v8 {
                    US2::US2_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v10: bool = v9 != true;
                if v10 {
                    return true;
                } else {
                    let v11: i32 = 0i32;
                    let mut v12: Rc<dyn Fn(Rc<UH0>) -> US2> = method18(v4.clone(), v11);
                    let v13: US2 = v12(v2.clone());
                    let v14: bool = match v13 {
                        US2::US2_None => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v15: bool = v14 != true;
                    if v15 {
                        v3 = v5;
                        continue;
                    } else {
                        let v17: US1 = node_named_19(v4.clone(), v0.clone());
                        match v17 {
                            US1::US1_None => {
                                let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v2));
                                (v2, v3) = (v24, v5);
                                continue;
                            }
                            US1::US1_Some(v18, v19, v20) => {
                                let v18: Rc<str> = v18;
                                let v19: Rc<UH0> = v19;
                                let v20: bool = v20;
                                let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v2));
                                let v22: Rc<UH0> = prepended_20(v19, v5);
                                (v2, v3) = (v21, v22);
                                continue;
                            }
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn affected_crates_16(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v6, v7, v8, v9) => {
                let mut v6: Rc<str> = v6;
                let mut v7: Rc<UH0> = v7;
                let v8: bool = v8;
                let mut v9: Rc<UH1> = v9;
                let v14: bool = if v8 {
                    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6.clone(), v11));
                    reaches_17(v1.clone(), v2.clone(), v10, v12)
                } else {
                    false
                };
                let v16: Rc<UH0> = if v14 {
                    Rc::new(UH0::UH0_Cons(v6, v3))
                } else {
                    v3
                };
                (v0, v3) = (v9, v16);
                continue;
            }
            UH1::UH1_Nil => {
                let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method4(v3, v4);
            }
        }
    }
}
fn lines_printed_21(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                println!("{}", v2);
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method23(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_22(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method23(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method24(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: bool = v4;
                let mut v5: Rc<UH1> = v5;
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn affected_run_0() -> i32 {
    let mut v0: i32 = command_line_argument_count_1();
    let mut v1: i32 = command_line_argument_count_1();
    let v2: bool = 2i32 < v1;
    let mut v7: US0 = if v2 {
        let mut v3: i32 = 2i32;
        let v4: Rc<str> = command_line_argument_2(v3);
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
            let v8: Rc<str> = v8;
            v8
        }
    };
    let mut v12: i32 = 3i32;
    let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v14: Rc<UH0> = arguments_from_3(v12, v0, v13);
    match &*v14 {
        UH0::UH0_Nil => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: repo usage: repo affected <src> <package>... (crates whose package graph reaches a changed package)"); } LIT.with(|lit| lit.clone()) };
            println!("{}", v15);
            2i32
        }
        _ => {
            let v16: Rc<UH0> = directory_names_5(v11.clone());
            let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v18: Rc<UH1> = nodes_of_7(v11.clone(), v16, v17);
            let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v20: Rc<UH0> = affected_crates_16(v18.clone(), v18.clone(), v14, v19);
            let v21: i32 = 0i32;
            let v22: i32 = lines_printed_21(v20, v21);
            let v23: Rc<str> = format_real_22(v22);
            let v26: i32 = 0i32;
            let v27: i32 = method24(v18, v26);
            let v28: Rc<str> = format_real_22(v27);
            let v31: Rc<str> = string_concat(&["rig repo affected ok crates=", &*v23, " packages=", &*v28, " src=", &*v11]);
            println!("{}", v31);
            0i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        affected_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_repo_affected_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
