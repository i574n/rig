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
    US0_Formatted(i32),
    US0_Unformatted(Rc<UH0>),
    US0_NoConfig(Rc<str>),
    US0_NoTaplo,
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn verdict_code_1(mut v0: US0) -> i32 {
    match v0 {
        US0::US0_Formatted(v1) => {
            let mut v1: i32 = v1;
            0i32
        }
        US0::US0_Unformatted(v2) => {
            let mut v2: Rc<UH0> = v2;
            1i32
        }
        _ => {
            2i32
        }
    }
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_2(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_3();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US1 = if v4 {
        let v5: Rc<str> = command_line_argument_4(v0);
        US1::US1_Some(v5)
    } else {
        US1::US1_None
    };
    match v8 {
        US1::US1_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US1::US1_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_2(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn get_environment_variable_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn file_exists_7(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn taplo_config_8(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>) -> Rc<str> {
    if v1 {
        v0
    } else {
        v2
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method10(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method13(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method14(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method15(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: i32 = (v2.clone().len() as i32);
                let v5: i32 = 0i32;
                let mut v6: i32 = method13(v2.clone(), v4, v5);
                let mut v7: i32 = v4.wrapping_sub(1i32);
                let mut v8: Rc<str> = string_slice(&v2, v6 as i64, v7 as i64);
                let mut v9: i32 = (v8.clone().len() as i32);
                let v10: i32 = method14(v8.clone(), v9);
                let v11: Rc<str> = string_slice(&v8, 0i32 as i64, v10 as i64);
                let v12: bool = &*v11 == "";
                let v15: bool = if v12 {
                    true
                } else {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("manifest.toml"); } LIT.with(|lit| lit.clone()) };
                    let v14: bool = v11.ends_with(&*v13);
                    v14
                };
                let v19: Rc<UH0> = if v15 {
                    v1
                } else {
                    let mut v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v16));
                    method15(v1, v17)
                };
                (v0, v1) = (v3, v19);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method17(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
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
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method9(v16, v17)
    }
}
fn toml_files_11(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_16(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method12(v5, v6)
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: bool = &*v2 == "";
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
fn method21(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: bool = &*v1 == "";
                let v5: Rc<str> = if v4 {
                    v2
                } else {
                    v1
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
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    Rc::new(move |mut v2: Rc<UH0>| -> US2 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US2> = method23(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method23(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    closure2(v0, v1)
}
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("not properly formatted path=\""); } LIT.with(|lit| lit.clone()) };
                let v5: Option<usize> = v2.find(&*v4);
                let mut v6: Option<usize> = method21(v5);
                let mut v7: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v8: Option<i32> = v6.map(|x| v7(x));
                let v10: i32 = v8.unwrap_or(-1i32);
                let v11: bool = v10 < 0i32;
                let v35: Rc<UH0> = if v11 {
                    v1
                } else {
                    let v12: i32 = v10.wrapping_add(29i32);
                    let v13: i32 = (v2.clone().len() as i32);
                    let v14: i32 = v13.wrapping_sub(v10);
                    let v15: i32 = v14.wrapping_sub(29i32);
                    let mut v16: usize = ((v12) as usize);
                    let v17: i32 = v12.wrapping_add(v15);
                    let v18: usize = ((v17) as usize);
                    let v19: Rc<str> = Rc::<str>::from(&(v2)[v16..v18]);
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<UH0> = split_list_16(v20, v19);
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = method22(v21, v22);
                    let v24: bool = &*v23 == "";
                    let v30: bool = if v24 {
                        true
                    } else {
                        let v25: i32 = 0i32;
                        let mut v26: Rc<dyn Fn(Rc<UH0>) -> US2> = method23(v23.clone(), v25);
                        let v27: US2 = v26(v1.clone());
                        let v28: bool = match v27 {
                            US2::US2_None => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v29: bool = v28 != true;
                        v29
                    };
                    if v30 {
                        v1
                    } else {
                        let v31: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let mut v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v31));
                        method15(v1, v32)
                    }
                };
                (v0, v1) = (v3, v35);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn unformatted_paths_19(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_16(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method20(v5, v6)
}
fn method24(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
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
fn toml_verdict_of_5(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = string_concat(&[&*v3, "/.taplo.toml"]);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TAPLO_CONFIG"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = get_environment_variable_6(v5);
    let mut v7: Rc<str> = Rc::<str>::from(v6.replace(&*v1, &*v2));
    let mut v8: bool = file_exists_7(v4.clone());
    let mut v9: Rc<str> = taplo_config_8(v4, v8, v7);
    let v10: bool = &*v9 == "";
    let v13: bool = if v10 {
        true
    } else {
        let v11: bool = file_exists_7(v9.clone());
        let v12: bool = v11 == false;
        v12
    };
    if v13 {
        US0::US0_NoConfig(v0)
    } else {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git"); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v15)));
        let v17: bool = &*v0 == "";
        let v25: Rc<UH0> = if v17 {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v18, v19))
        } else {
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v21, v22));
            Rc::new(UH0::UH0_Cons(v0, v23))
        };
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ls-files"); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*.toml"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v29: Rc<UH0> = Rc::new(UH0::UH0_Cons(v27, v28));
        let v30: Rc<UH0> = Rc::new(UH0::UH0_Cons(v26, v29));
        let v31: Rc<UH0> = method9(v25, v30);
        method10(v16.clone(), v31);
        let mut v32: Rc<std::io::Result<std::process::Output>> = Rc::new(v16.borrow_mut().output());
        let v33: bool = v32.is_ok();
        let (mut v38, mut v39, mut v40): (bool, i32, Rc<str>) = if v33 {
            let v34: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v32).as_ref().unwrap().stdout).as_ref());
            let v35: Rc<str> = string_concat(&[&*v34, ""]);
            let v36: i32 = (&*v32).as_ref().unwrap().status.code().unwrap_or(-2);
            (true, v36, v35)
        } else {
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, -1i32, v37)
        };
        let v41: Rc<UH0> = toml_files_11(v40);
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("taplo"); } LIT.with(|lit| lit.clone()) };
        let mut v43: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v42)));
        let v59: Rc<UH0> = if v10 {
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--config"); } LIT.with(|lit| lit.clone()) };
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--check"); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fmt"); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v46, v47));
            let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v45, v48));
            Rc::new(UH0::UH0_Cons(v44, v49))
        } else {
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--config"); } LIT.with(|lit| lit.clone()) };
            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--check"); } LIT.with(|lit| lit.clone()) };
            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fmt"); } LIT.with(|lit| lit.clone()) };
            let v54: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v55: Rc<UH0> = Rc::new(UH0::UH0_Cons(v53, v54));
            let v56: Rc<UH0> = Rc::new(UH0::UH0_Cons(v52, v55));
            let v57: Rc<UH0> = Rc::new(UH0::UH0_Cons(v51, v56));
            Rc::new(UH0::UH0_Cons(v9, v57))
        };
        let v60: Rc<UH0> = method18(v41.clone(), v59);
        let v61: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v62: Rc<UH0> = method9(v60, v61);
        method10(v43.clone(), v62);
        let mut v63: Rc<std::io::Result<std::process::Output>> = Rc::new(v43.borrow_mut().output());
        let v64: bool = v63.is_ok();
        let (mut v70, mut v71, mut v72): (bool, i32, Rc<str>) = if v64 {
            let v65: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v63).as_ref().unwrap().stderr).as_ref());
            let v66: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v63).as_ref().unwrap().stdout).as_ref());
            let v67: Rc<str> = string_concat(&[&*v66, &*v65]);
            let v68: i32 = (&*v63).as_ref().unwrap().status.code().unwrap_or(-2);
            (true, v68, v67)
        } else {
            let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, -1i32, v69)
        };
        let v73: bool = v70 == false;
        if v73 {
            US0::US0_NoTaplo
        } else {
            let v75: Rc<UH0> = unformatted_paths_19(v72);
            match &*v75 {
                UH0::UH0_Nil => {
                    let v76: i32 = 0i32;
                    let v77: i32 = method24(v41, v76);
                    US0::US0_Formatted(v77)
                }
                _ => {
                    US0::US0_Unformatted(v75)
                }
            }
        }
    }
}
fn method27(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_26(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method27(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method28(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method28(v3, v1);
            let v5: Rc<str> = string_concat(&[&*v2, ":1  toml-format  not formatted by the taplo config"]);
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn verdict_lines_25(mut v0: US0) -> Rc<UH0> {
    match v0 {
        US0::US0_Formatted(v1) => {
            let mut v1: i32 = v1;
            let mut v2: Rc<str> = format_real_26(v1);
            let v4: Rc<str> = string_concat(&["kit standards toml ok files=", &*v2, " unformatted=0"]);
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v4, v5))
        }
        US0::US0_NoConfig(v17) => {
            let v17: Rc<str> = v17;
            let v19: Rc<str> = string_concat(&["kit standards toml: no .taplo.toml at ", &*v17, " and TAPLO_CONFIG names no file"]);
            let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v19, v20))
        }
        US0::US0_NoTaplo => {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit standards toml: taplo is not on PATH"); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v22, v23))
        }
        US0::US0_Unformatted(v7) => {
            let mut v7: Rc<UH0> = v7;
            let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v9: Rc<UH0> = method28(v7.clone(), v8);
            let v10: i32 = 0i32;
            let v11: i32 = method24(v7, v10);
            let v12: Rc<str> = format_real_26(v11);
            let v13: Rc<str> = string_concat(&["kit standards toml refused unformatted=", &*v12]);
            let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v14));
            method15(v9, v15)
        }
    }
}
fn method29(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<UH0> = v2;
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
fn toml_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_2(v0);
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
                            let mut v8: US0 = toml_verdict_of_5(v6);
                            let mut v9: Rc<UH0> = verdict_lines_25(v8.clone());
                            method29(v9);
                            verdict_code_1(v8)
                        }
                        _ => {
                            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: standards --toml <repo>"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v11);
                            2i32
                        }
                    }
                }
                _ => {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: standards --toml <repo>"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v13);
                    2i32
                }
            }
        }
        _ => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: standards --toml <repo>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v15);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        toml_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_standards_toml_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
