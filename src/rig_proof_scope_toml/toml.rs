#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
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
    US0_PackageDirKey,
    US0_PackagesKey,
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
fn method3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
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
fn method4(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
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
fn method2(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = &*v2 == "";
                let v19: bool = if v5 {
                    let mut v6: i32 = (v3.clone().len() as i32);
                    let v7: i32 = 0i32;
                    let v8: i32 = method3(v3.clone(), v6, v7);
                    let mut v9: i32 = v6.wrapping_sub(1i32);
                    let v10: Rc<str> = string_slice(&v3.clone(), v8 as i64, v9 as i64);
                    let mut v11: i32 = (v10.clone().len() as i32);
                    let v12: i32 = method4(v10.clone(), v11);
                    let v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
                    let v14: Rc<str> = string_concat(&[&*v0, " "]);
                    let v15: bool = v13.starts_with(&*v14);
                    if v15 {
                        true
                    } else {
                        let v16: Rc<str> = string_concat(&[&*v0, "="]);
                        let v17: bool = v13.starts_with(&*v16);
                        v17
                    }
                } else {
                    false
                };
                let v28: Rc<str> = if v19 {
                    let v20: i32 = (v3.clone().len() as i32);
                    let v21: i32 = 0i32;
                    let v22: i32 = method3(v3.clone(), v20, v21);
                    let v23: i32 = v20.wrapping_sub(1i32);
                    let v24: Rc<str> = string_slice(&v3, v22 as i64, v23 as i64);
                    let v25: i32 = (v24.clone().len() as i32);
                    let v26: i32 = method4(v24.clone(), v25);
                    let v27: Rc<str> = string_slice(&v24, 0i32 as i64, v26 as i64);
                    v27
                } else {
                    v2
                };
                (v1, v2) = (v4, v28);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method6(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let mut v5: Rc<str> = (v0)[v4].clone();
            let mut v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn text_lines_5(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new((v5).borrow().clone());
    let v7: usize = (v6).len();
    let v8: i32 = ((v7) as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method6(v6, v9, v10)
}
fn key_line_1(mut v0: US0, mut v1: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = match v0 {
        US0::US0_PackageDirKey => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packageDir"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_PackagesKey => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packages"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let mut v6: Rc<UH0> = text_lines_5(v1);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method2(v5, v6, v7)
}
fn method8(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn after_equals_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method8(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    } else {
        let v10: i32 = v7.wrapping_add(1i32);
        let mut v11: i32 = (v0.clone().len() as i32);
        let v12: i32 = v11.wrapping_sub(1i32);
        let v13: Rc<str> = string_slice(&v0, v10 as i64, v12 as i64);
        let v14: i32 = (v13.clone().len() as i32);
        let v15: i32 = 0i32;
        let v16: i32 = method3(v13.clone(), v14, v15);
        let v17: i32 = v14.wrapping_sub(1i32);
        let v18: Rc<str> = string_slice(&v13, v16 as i64, v17 as i64);
        let v19: i32 = (v18.clone().len() as i32);
        let v20: i32 = method4(v18.clone(), v19);
        let v21: Rc<str> = string_slice(&v18, 0i32 as i64, v20 as i64);
        v21
    }
}
fn unquoted_9(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method3(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0, v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method4(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(v8.replace(&*v9, &*v10));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v10));
    v13
}
fn directory_exists_12(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    Rc::new(move |mut v2: Rc<UH0>| -> US1 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US1::US1_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US1> = method13(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US1::US1_None
            }
        }
    })
}
fn method13(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    closure2(v0, v1)
}
fn method11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<UH0>, mut v4: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH0::UH0_Cons(v5, v6) => {
                let mut v5: Rc<str> = v5;
                let mut v6: Rc<UH0> = v6;
                let v8: Rc<str> = string_concat(&[&*v0, "/", &*v5]);
                let mut v9: bool = directory_exists_12(v8.clone());
                let v10: i32 = 0i32;
                let mut v11: Rc<dyn Fn(Rc<UH0>) -> US1> = method13(v5.clone(), v10);
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target"); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".git"); } LIT.with(|lit| lit.clone()) };
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules"); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".cache"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_build"); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bin"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("obj"); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19, v20));
                let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v21));
                let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17, v22));
                let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v23));
                let v25: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v24));
                let v26: Rc<UH0> = Rc::new(UH0::UH0_Cons(v14, v25));
                let v27: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v26));
                let v28: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v27));
                let v29: US1 = v11(v28);
                let v30: bool = match v29 {
                    US1::US1_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v31: bool = v30 != true;
                let v34: bool = if v31 {
                    true
                } else {
                    if v9 {
                        let v32: bool = v2 >= 6i32;
                        v32
                    } else {
                        false
                    }
                };
                let v48: Rc<UH0> = if v34 {
                    v4
                } else {
                    if v9 {
                        let v36: Rc<str> = string_concat(&[&*v1, &*v5, "/"]);
                        let v37: i32 = v2.wrapping_add(1i32);
                        spi_modules_in_10(v8, v36, v37, v4)
                    } else {
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                        let v40: bool = v5.ends_with(&*v39);
                        if v40 {
                            let v41: i32 = (v5.clone().len() as i32);
                            let v42: i32 = v41.wrapping_sub(5i32);
                            let v43: Rc<str> = string_slice(&v5, 0i32 as i64, v42 as i64);
                            let v44: Rc<str> = string_concat(&[&*v1, &*v43]);
                            Rc::new(UH0::UH0_Cons(v44, v4))
                        } else {
                            v4
                        }
                    }
                };
                (v3, v4) = (v6, v48);
                continue;
            }
            UH0::UH0_Nil => {
                return v4;
            }
        }
    }
}
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn names_in_14(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Vec<std::string::String> = v2.unwrap();
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: bool = true; let _vec_fold_ = v4.into_iter().fold(v5, |acc, x| { //;
    let mut v9: Rc<UH0> = acc;
    let mut v11: std::string::String = x;
    let v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v9));
    let v16: bool = true; v15 });
    let v18: Rc<UH0> = _vec_fold_;
    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method15(v18, v19)
}
fn spi_modules_in_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    let mut v4: Rc<UH0> = names_in_14(v0.clone());
    method11(v0, v1, v2, v4, v3)
}
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method17(v3, v1);
            let mut v5: bool = &*v2 != "";
            if v5 {
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
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method18(v3, v1);
            let mut v5: Rc<str> = unquoted_9(v2);
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn key_list_16(mut v0: US0, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: Rc<str> = key_line_1(v0, v1);
    let v3: Rc<str> = after_equals_7(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(v6.replace(&*v7, &*v5));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v8.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v11: Rc<Vec<Rc<str>>> = Rc::new((v10).borrow().clone());
    let v12: usize = (v11).len();
    let v13: i32 = ((v12) as i32);
    let v14: i32 = v13.wrapping_sub(1i32);
    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH0> = method6(v11, v14, v15);
    let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v18: Rc<UH0> = method18(v16, v17);
    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method17(v18, v19)
}
fn method20(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn boxed_19(mut v0: Rc<UH0>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v2: i32 = 0i32;
    let v3: i32 = method20(v1.clone(), v0, v2);
    let mut v4: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v4).as_ref().clone()));
    v5
}
fn toml_package_of_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    let mut v2: US0 = US0::US0_PackageDirKey;
    let v3: Rc<str> = key_line_1(v2, v0.clone());
    let mut v4: Rc<str> = after_equals_7(v3);
    let mut v5: Rc<str> = unquoted_9(v4);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: i32 = 0i32;
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = spi_modules_in_10(v1, v6, v7, v8);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v11: Rc<UH0> = method15(v9, v10);
    let v12: US0 = US0::US0_PackagesKey;
    let v13: Rc<UH0> = key_list_16(v12, v0);
    let v14: Rc<RefCell<Vec<Rc<str>>>> = boxed_19(v13);
    let v15: Rc<RefCell<Vec<Rc<str>>>> = boxed_19(v11);
    Rc::new(RefCell::new(vec![(v5, v14, v15)]))
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
        toml_package_of_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_proof_scope_toml_package(v0: &str, v1: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
