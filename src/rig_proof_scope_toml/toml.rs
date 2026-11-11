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
fn method2(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = &*v2 == "";
                let v12: bool = if v5 {
                    let mut v6: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                    let v7: Rc<str> = string_concat(&[&*v0, " "]);
                    let v8: bool = v6.starts_with(&*v7);
                    if v8 {
                        true
                    } else {
                        let mut v9: Rc<str> = string_concat(&[&*v0, "="]);
                        let v10: bool = v6.starts_with(&*v9);
                        v10
                    }
                } else {
                    false
                };
                let v14: Rc<str> = if v12 {
                    let v13: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                    v13
                } else {
                    v2
                };
                (v1, v2) = (v4, v14);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method4(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
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
fn text_lines_3(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new((v5).borrow().clone());
    let v7: usize = (v6).len();
    let v8: i32 = ((v7) as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method4(v6, v9, v10)
}
fn key_line_1(mut v0: US0, mut v1: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = match v0 {
        US0::US0_PackageDirKey => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packageDir"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_PackagesKey => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packages"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let mut v6: Rc<UH0> = text_lines_3(v1);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method2(v5, v6, v7)
}
fn method6(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn after_equals_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method6(v2);
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
        let v14: Rc<str> = Rc::<str>::from((v13).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        v14
    }
}
fn unquoted_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v3));
    v6
}
fn block_quote_10(mut v0: Rc<str>) -> bool {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'''"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.contains(&*v1);
    if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\"\""); } LIT.with(|lit| lit.clone()) };
        let mut v4: bool = v0.contains(&*v3);
        v4
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method11(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: bool) -> (Rc<UH0>, bool) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let mut v6: bool = v2 == false;
                let mut v9: bool = if v6 {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules"); } LIT.with(|lit| lit.clone()) };
                    let v8: bool = v5.starts_with(&*v7);
                    v8
                } else {
                    false
                };
                let mut v11: bool = if v9 {
                    block_quote_10(v5.clone())
                } else {
                    false
                };
                let (mut v23, mut v24): (Rc<UH0>, bool) = if v11 {
                    (v1, true)
                } else {
                    let v13: bool = if v2 {
                        block_quote_10(v5.clone())
                    } else {
                        false
                    };
                    if v13 {
                        (v1, false)
                    } else {
                        let v15: bool = if v2 {
                            let v14: bool = &*v5 != "";
                            v14
                        } else {
                            false
                        };
                        if v15 {
                            let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v16));
                            let v18: Rc<UH0> = method11(v1, v17);
                            (v18, true)
                        } else {
                            (v1, v2)
                        }
                    }
                };
                (v0, v1, v2) = (v4, v23, v24);
                continue;
            }
            UH0::UH0_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn declared_modules_8(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<UH0> = text_lines_3(v0);
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v3: bool = false;
    let (mut v4, mut v5): (Rc<UH0>, bool) = method9(v1, v2, v3);
    v4
}
fn directory_exists_14(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
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
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US1> = method15(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US1::US1_None
            }
        }
    })
}
fn method15(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    closure2(v0, v1)
}
fn method13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<UH0>, mut v4: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH0::UH0_Cons(v5, v6) => {
                let mut v5: Rc<str> = v5;
                let mut v6: Rc<UH0> = v6;
                let v8: Rc<str> = string_concat(&[&*v0, "/", &*v5]);
                let mut v9: bool = directory_exists_14(v8.clone());
                let v10: i32 = 0i32;
                let mut v11: Rc<dyn Fn(Rc<UH0>) -> US1> = method15(v5.clone(), v10);
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
                        spi_modules_in_12(v8, v36, v37, v4)
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
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn names_in_16(mut v0: Rc<str>) -> Rc<UH0> {
    let v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
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
    method17(v18, v19)
}
fn spi_modules_in_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    let mut v4: Rc<UH0> = names_in_16(v0.clone());
    method13(v0, v1, v2, v4, v3)
}
fn method19(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method19(v3, v1);
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
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method20(v3, v1);
            let mut v5: Rc<str> = unquoted_7(v2);
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn key_list_18(mut v0: US0, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: Rc<str> = key_line_1(v0, v1);
    let v3: Rc<str> = after_equals_5(v2);
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
    let v16: Rc<UH0> = method4(v11, v14, v15);
    let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v18: Rc<UH0> = method20(v16, v17);
    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method19(v18, v19)
}
fn method22(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn boxed_21(mut v0: Rc<UH0>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v2: i32 = 0i32;
    let v3: i32 = method22(v1.clone(), v0, v2);
    let mut v4: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v4).as_ref().clone()));
    v5
}
fn toml_package_of_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    let v2: US0 = US0::US0_PackageDirKey;
    let v3: Rc<str> = key_line_1(v2, v0.clone());
    let mut v4: Rc<str> = after_equals_5(v3);
    let mut v5: Rc<str> = unquoted_7(v4);
    let mut v6: Rc<UH0> = declared_modules_8(v0.clone());
    let v7: bool = match &*v6 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    let v14: Rc<UH0> = if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v9: i32 = 0i32;
        let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v11: Rc<UH0> = spi_modules_in_12(v1, v8, v9, v10);
        let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method17(v11, v12)
    } else {
        v6
    };
    let v15: US0 = US0::US0_PackagesKey;
    let v16: Rc<UH0> = key_list_18(v15, v0);
    let v17: Rc<RefCell<Vec<Rc<str>>>> = boxed_21(v16);
    let v18: Rc<RefCell<Vec<Rc<str>>>> = boxed_21(v14);
    Rc::new(RefCell::new(vec![(v5, v17, v18)]))
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
        toml_package_of_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_proof_scope_toml_package(v0: &str, v1: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
