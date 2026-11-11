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
enum US0 {
    US0_Ok,
    US0_Error(Rc<str>),
}
enum US1 {
    US1_Ok,
    US1_Error(std::io::Error),
}
#[derive(Clone)]
enum US2 {
    US2_SpiralSource,
    US2_Manifest,
    US2_Glue,
    US2_Junk,
}
#[derive(Clone)]
enum US3 {
    US3_PromoteTo(Rc<str>),
    US3_PromoteRefused(Rc<str>),
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>),
    US4_None,
}
fn plain_words_2(mut v0: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
                match &*v2 {
                    UH0::UH0_Cons(v3, v4) => {
                        let mut v3: Rc<str> = v3.clone();
                        let v4: Rc<UH0> = v4.clone();
                        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                        let mut v6: bool = v1.starts_with(&*v5);
                        if v6 {
                            v0 = v4;
                            continue;
                        } else {
                            let v8: Rc<UH0> = plain_words_2(v2);
                            return Rc::new(UH0::UH0_Cons(v1, v8));
                        }
                    }
                    _ => {
                        let mut v11: Rc<UH0> = plain_words_2(v2);
                        return Rc::new(UH0::UH0_Cons(v1, v11));
                    }
                }
            }
            UH0::UH0_Nil => {
                return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn get_environment_variable_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn slash_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    v3
}
fn store_root_4() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = get_environment_variable_5(v0);
    let v2: Rc<str> = slash_6(v1);
    let mut v3: Rc<str> = string_concat(&[&*v2, "/kit/drafts"]);
    v3
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
fn leaf_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = slash_6(v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Option<usize> = v1.rfind(&*v2);
    let v4: Option<usize> = method8(v3);
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let v8: i32 = v6.unwrap_or(-1i32);
    let v9: bool = v8 < 0i32;
    if v9 {
        v1
    } else {
        let v10: i32 = v8.wrapping_add(1i32);
        let mut v11: i32 = (v1.clone().len() as i32);
        let mut v12: i32 = v11.wrapping_sub(v8);
        let mut v13: i32 = v12.wrapping_sub(1i32);
        let mut v14: usize = ((v10) as usize);
        let v15: i32 = v10.wrapping_add(v13);
        let v16: usize = ((v15) as usize);
        let mut v17: Rc<str> = Rc::<str>::from(&(v1)[v14..v16]);
        v17
    }
}
fn draft_dir_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = store_root_4();
    let v4: Rc<str> = leaf_7(v0);
    let mut v7: Rc<str> = string_concat(&[&*v2, "/", &*v4, "/", &*v1]);
    v7
}
fn directory_exists_11(mut v0: Rc<str>) -> bool {
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
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method12(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v7: Rc<str> = string_concat(&[&*v0, "/", &*v4]);
                let v8: bool = directory_exists_11(v7.clone());
                let v16: Rc<UH0> = if v8 {
                    let v10: Rc<str> = string_concat(&[&*v1, &*v4, "/"]);
                    files_under_9(v7, v10, v3)
                } else {
                    let mut v12: Rc<str> = string_concat(&[&*v1, &*v4]);
                    let mut v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v13));
                    method12(v3, v14)
                };
                (v2, v3) = (v5, v16);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn names_in_13(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: bool = directory_exists_11(v0.clone());
    let v2: bool = v1 == false;
    if v2 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v5: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v7: Vec<std::string::String> = v5.unwrap();
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v10: bool = true; let _vec_fold_ = v7.into_iter().fold(v8, |acc, x| { //;
        let mut v12: Rc<UH0> = acc;
        let mut v14: std::string::String = x;
        let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
        let mut v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v12));
        let v19: bool = true; v18 });
        let v21: Rc<UH0> = _vec_fold_;
        let mut v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method14(v21, v22)
    }
}
fn files_under_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> Rc<UH0> {
    let mut v3: Rc<UH0> = names_in_13(v0.clone());
    method10(v0, v1, v3, v2)
}
fn closure2() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> Rc<str>> = Rc::new(move |mut v0: std::io::Error| -> Rc<str> {
        let mut v1: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v0)).s() });
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    closure2()
}
fn ok_c3() -> Rc<dyn Fn() -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US0> = Rc::new(move || -> US0 {
        US0::US0_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method19() -> Rc<dyn Fn() -> US0> {
    ok_c3()
}
fn error_c4() -> Rc<dyn Fn(Rc<str>) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US0> = Rc::new(move |mut v0: Rc<str>| -> US0 {
        US0::US0_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method20() -> Rc<dyn Fn(Rc<str>) -> US0> {
    error_c4()
}
fn ok_c5() -> Rc<dyn Fn() -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US1> = Rc::new(move || -> US1 {
        US1::US1_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method21() -> Rc<dyn Fn() -> US1> {
    ok_c5()
}
fn error_c6() -> Rc<dyn Fn(std::io::Error) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US1> = Rc::new(move |mut v0: std::io::Error| -> US1 {
        US1::US1_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method22() -> Rc<dyn Fn(std::io::Error) -> US1> {
    error_c6()
}
fn moved_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Option<usize> = v1.rfind(&*v2);
    let v4: Option<usize> = method8(v3);
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let v8: i32 = v6.unwrap_or(-1i32);
    let v9: usize = ((0i32) as usize);
    let v10: usize = ((v8) as usize);
    let mut v11: Rc<str> = Rc::<str>::from(&(v1)[v9..v10]);
    let mut v13: Result<(), std::io::Error> = std::fs::create_dir_all(&*v11);
    let mut v14: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method18();
    let v16: Result<(), Rc<str>> = v13.map_err(|x| v14(x));
    let mut v17: Rc<dyn Fn() -> US0> = method19();
    let mut v18: Rc<dyn Fn(Rc<str>) -> US0> = method20();
    let v19: US0 = match v16 { Ok(()) => v17(), Err(e) => v18(e) };
    let v21: Result<(), std::io::Error> = std::fs::rename(&*v0, &*v1);
    let mut v22: Rc<dyn Fn() -> US1> = method21();
    let mut v23: Rc<dyn Fn(std::io::Error) -> US1> = method22();
    let v24: US1 = match v21 { Ok(()) => v22(), Err(e) => v23(e) };
    match v24 {
        US1::US1_Error(v26) => {
            let v26: std::io::Error = v26;
            let v29: Rc<str> = string_concat(&["could not move ", &*v0, " to ", &*v1]);
            v29
        }
        US1::US1_Ok => {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v25
        }
    }
}
fn promote_one_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = slash_6(v0.clone());
    let mut v6: Rc<str> = string_concat(&[&*v4, "/", &*v2]);
    let mut v7: bool = directory_exists_11(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v9: Option<usize> = v3.rfind(&*v8);
    let v10: Option<usize> = method8(v9);
    let mut v11: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v12: Option<i32> = v10.map(|x| v11(x));
    let mut v14: i32 = v12.unwrap_or(-1i32);
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = Rc::<str>::from(v3.replace(&*v15, &*v16));
    let mut v18: Option<usize> = v17.rfind(&*v16);
    let v19: Option<usize> = method8(v18);
    let mut v20: Option<i32> = v19.map(|x| v11(x));
    let mut v22: i32 = v20.unwrap_or(-1i32);
    let mut v23: bool = v14 > v22;
    let v34: Rc<str> = if v23 {
        let v24: i32 = v14.wrapping_add(1i32);
        let v25: i32 = (v3.clone().len() as i32);
        let v26: i32 = v25.wrapping_sub(v14);
        let v27: i32 = v26.wrapping_sub(1i32);
        let v28: usize = ((v24) as usize);
        let v29: i32 = v24.wrapping_add(v27);
        let v30: usize = ((v29) as usize);
        let v31: Rc<str> = Rc::<str>::from(&(v3)[v28..v30]);
        let v32: Rc<str> = Rc::<str>::from(v31.to_lowercase());
        v32
    } else {
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v33
    };
    let v35: bool = &*v34 == "spi";
    let v37: bool = if v35 {
        true
    } else {
        let v36: bool = &*v34 == "spir";
        v36
    };
    let v56: US2 = if v37 {
        US2::US2_SpiralSource
    } else {
        let v39: bool = &*v34 == "spiproj";
        let v41: bool = if v39 {
            true
        } else {
            let v40: bool = &*v34 == "toml";
            v40
        };
        if v41 {
            US2::US2_Manifest
        } else {
            let v43: bool = &*v34 == "rs";
            let v45: bool = if v43 {
                true
            } else {
                let v44: bool = &*v34 == "gleam";
                v44
            };
            let v47: bool = if v45 {
                true
            } else {
                let v46: bool = &*v34 == "erl";
                v46
            };
            let v49: bool = if v47 {
                true
            } else {
                let v48: bool = &*v34 == "c";
                v48
            };
            let v51: bool = if v49 {
                true
            } else {
                let v50: bool = &*v34 == "h";
                v50
            };
            if v51 {
                US2::US2_Glue
            } else {
                US2::US2_Junk
            }
        }
    };
    let v69: US3 = match v56 {
        US2::US2_Junk => {
            let v57: Rc<str> = string_concat(&[&*v3, " is not typed Spiral, a manifest or thin glue; archive it or rewrite it in Spiral"]);
            US3::US3_PromoteRefused(v57)
        }
        _ => {
            let v59: bool = &*v2 == "";
            if v59 {
                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("promote needs a decided landing: an existing package or app dir, or one named on the item"); } LIT.with(|lit| lit.clone()) };
                US3::US3_PromoteRefused(v60)
            } else {
                let v62: bool = v7 == false;
                if v62 {
                    let v64: Rc<str> = string_concat(&["the landing ", &*v2, " does not exist; name it on the item or create the package first"]);
                    US3::US3_PromoteRefused(v64)
                } else {
                    US3::US3_PromoteTo(v2)
                }
            }
        }
    };
    match v69 {
        US3::US3_PromoteRefused(v70) => {
            let v70: Rc<str> = v70;
            v70
        }
        US3::US3_PromoteTo(v71) => {
            let v71: Rc<str> = v71;
            let v72: Rc<str> = draft_dir_3(v0.clone(), v1);
            let v74: Rc<str> = string_concat(&[&*v72, "/", &*v3]);
            let v75: Rc<str> = slash_6(v0);
            let v79: Rc<str> = leaf_7(v3);
            let v80: Rc<str> = string_concat(&[&*v75, "/", &*v71, "/", &*v79]);
            moved_17(v74, v80)
        }
    }
}
fn method15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v3) {
        UH0::UH0_Cons(v5, v6) => {
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<UH0> = v6;
            let mut v7: Rc<UH0> = method15(v0.clone(), v1.clone(), v2.clone(), v6, v4);
            let v8: Rc<str> = promote_one_16(v0, v1, v2, v5);
            Rc::new(UH0::UH0_Cons(v8, v7))
        }
        UH0::UH0_Nil => {
            v4
        }
    }
}
fn method23(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method23(v3, v1);
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
fn method24(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
                let mut v3: Rc<str> = string_concat(&["drafts promote refused: ", &*v1]);
                println!("{}", v3);
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
fn method25(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn verdict_word_26(mut v0: &Rc<UH0>) -> Rc<str> {
    let mut v1: bool = match &**v0 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
        v3
    }
}
fn method28(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_27(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method28(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn promote_run_1(mut v0: Rc<UH0>) -> i32 {
    let mut v1: Rc<UH0> = plain_words_2(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let mut v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            let v8: Rc<str> = draft_dir_3(v2.clone(), v4.clone());
                            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let mut v11: Rc<UH0> = files_under_9(v8, v9, v10);
                            let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let mut v13: Rc<UH0> = method15(v2, v4, v6, v11.clone(), v12);
                            let mut v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v15: Rc<UH0> = method23(v13, v14);
                            method24(v15.clone());
                            let v16: i32 = 0i32;
                            let mut v17: i32 = method25(v11, v16);
                            let mut v18: i32 = 0i32;
                            let v19: i32 = method25(v15.clone(), v18);
                            let mut v20: i32 = v17.wrapping_sub(v19);
                            let v21: Rc<str> = verdict_word_26(&v15);
                            let v24: Rc<str> = format_real_27(v20);
                            let v25: Rc<str> = string_concat(&["rig drafts promote ", &*v21, " landed=", &*v24]);
                            println!("{}", v25);
                            let v26: bool = match Rc::unwrap_or_clone(v15) {
                                UH0::UH0_Nil => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            };
                            if v26 {
                                0i32
                            } else {
                                1i32
                            }
                        }
                        _ => {
                            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: drafts promote <repo root> <ITEM> <landing dir in the repo>"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v28);
                            2i32
                        }
                    }
                }
                _ => {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: drafts promote <repo root> <ITEM> <landing dir in the repo>"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v30);
                    2i32
                }
            }
        }
        _ => {
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: drafts promote <repo root> <ITEM> <landing dir in the repo>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v32);
            2i32
        }
    }
}
fn command_line_argument_count_31() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_32(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_30(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_31();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US4 = if v4 {
        let mut v5: Rc<str> = command_line_argument_32(v0);
        US4::US4_Some(v5)
    } else {
        US4::US4_None
    };
    match v8 {
        US4::US4_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US4::US4_Some(v10) => {
            let v10: Rc<str> = v10;
            let mut v11: i32 = v0.wrapping_add(1i32);
            let mut v12: Rc<UH0> = arguments_from_30(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn verb_words_29() -> Rc<UH0> {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_30(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let mut v5: Rc<UH0> = v5;
                    v5
                }
                _ => {
                    { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                }
            }
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn promote_entry_0() -> i32 {
    let mut v0: Rc<UH0> = verb_words_29();
    promote_run_1(v0)
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        promote_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_drafts_promote_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
