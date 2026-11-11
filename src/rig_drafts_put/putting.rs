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
    US2_PutAccepted,
    US2_PutRefused(Rc<str>),
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>),
    US3_None,
}
fn plain_words_2(mut v0: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
                match &*v2 {
                    UH0::UH0_Cons(v3, v4) => {
                        let v3: Rc<str> = v3.clone();
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
                        let v11: Rc<UH0> = plain_words_2(v2);
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
fn flag_value_3(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let v4: Rc<str> = v4.clone();
                        let mut v5: Rc<UH0> = v5.clone();
                        let mut v6: bool = &*v2 == &*v1;
                        if v6 {
                            return v4;
                        } else {
                            v0 = v3;
                            continue;
                        }
                    }
                    _ => {
                        v0 = v3;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v11;
            }
        }
    }
}
fn slash_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
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
fn closure2() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> Rc<str>> = Rc::new(move |mut v0: std::io::Error| -> Rc<str> {
        let mut v1: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v0)).s() });
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    closure2()
}
fn ok_c3() -> Rc<dyn Fn() -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US0> = Rc::new(move || -> US0 {
        US0::US0_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10() -> Rc<dyn Fn() -> US0> {
    ok_c3()
}
fn error_c4() -> Rc<dyn Fn(Rc<str>) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US0> = Rc::new(move |mut v0: Rc<str>| -> US0 {
        US0::US0_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method11() -> Rc<dyn Fn(Rc<str>) -> US0> {
    error_c4()
}
fn ok_c5() -> Rc<dyn Fn() -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US1> = Rc::new(move || -> US1 {
        US1::US1_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method12() -> Rc<dyn Fn() -> US1> {
    ok_c5()
}
fn error_c6() -> Rc<dyn Fn(std::io::Error) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US1> = Rc::new(move |mut v0: std::io::Error| -> US1 {
        US1::US1_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method13() -> Rc<dyn Fn(std::io::Error) -> US1> {
    error_c6()
}
fn moved_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Option<usize> = v1.rfind(&*v2);
    let v4: Option<usize> = method8(v3);
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let v8: i32 = v6.unwrap_or(-1i32);
    let v9: usize = ((0i32) as usize);
    let v10: usize = ((v8) as usize);
    let v11: Rc<str> = Rc::<str>::from(&(v1)[v9..v10]);
    let mut v13: Result<(), std::io::Error> = std::fs::create_dir_all(&*v11);
    let mut v14: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method9();
    let v16: Result<(), Rc<str>> = v13.map_err(|x| v14(x));
    let mut v17: Rc<dyn Fn() -> US0> = method10();
    let mut v18: Rc<dyn Fn(Rc<str>) -> US0> = method11();
    let v19: US0 = match v16 { Ok(()) => v17(), Err(e) => v18(e) };
    let v21: Result<(), std::io::Error> = std::fs::rename(&*v0, &*v1);
    let mut v22: Rc<dyn Fn() -> US1> = method12();
    let mut v23: Rc<dyn Fn(std::io::Error) -> US1> = method13();
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
fn get_environment_variable_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn store_root_15() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = get_environment_variable_16(v0);
    let v2: Rc<str> = slash_4(v1);
    let v3: Rc<str> = string_concat(&[&*v2, "/kit/drafts"]);
    v3
}
fn leaf_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = slash_4(v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Option<usize> = v1.rfind(&*v2);
    let v4: Option<usize> = method8(v3);
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let v8: i32 = v6.unwrap_or(-1i32);
    let v9: bool = v8 < 0i32;
    if v9 {
        v1
    } else {
        let v10: i32 = v8.wrapping_add(1i32);
        let v11: i32 = (v1.clone().len() as i32);
        let v12: i32 = v11.wrapping_sub(v8);
        let mut v13: i32 = v12.wrapping_sub(1i32);
        let mut v14: usize = ((v10) as usize);
        let v15: i32 = v10.wrapping_add(v13);
        let v16: usize = ((v15) as usize);
        let mut v17: Rc<str> = Rc::<str>::from(&(v1)[v14..v16]);
        v17
    }
}
fn draft_dir_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = store_root_15();
    let v4: Rc<str> = leaf_17(v0);
    let v7: Rc<str> = string_concat(&[&*v2, "/", &*v4, "/", &*v1]);
    v7
}
fn put_one_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = slash_4(v2.clone());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v6: bool = v3.contains(&*v5);
    let v9: bool = if v6 {
        true
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v3.starts_with(&*v7);
        v8
    };
    let mut v14: Rc<str> = if v9 {
        slash_4(v3)
    } else {
        let v12: Rc<str> = slash_4(v3);
        let mut v13: Rc<str> = string_concat(&[&*v4, "/", &*v12]);
        v13
    };
    let v16: Rc<str> = string_concat(&["Task (\"", &*v1, "\","]);
    let mut v17: bool = v0.contains(&*v16);
    let mut v18: bool = v17 == false;
    let v36: US2 = if v18 {
        let v20: Rc<str> = string_concat(&["no backlog item ", &*v1, "; a draft is always tied to an item (agile add it first)"]);
        US2::US2_PutRefused(v20)
    } else {
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = Rc::<str>::from(v2.replace(&*v22, &*v23));
        let v25: Rc<str> = Rc::<str>::from(v14.replace(&*v22, &*v23));
        let v26: bool = v24.ends_with(&*v23);
        let v28: Rc<str> = if v26 {
            v24
        } else {
            let v27: Rc<str> = string_concat(&[&*v24, "/"]);
            v27
        };
        let v29: bool = v25.starts_with(&*v28);
        let v30: bool = v29 == false;
        if v30 {
            let v32: Rc<str> = string_concat(&[&*v14, " is not inside the repo ", &*v2]);
            US2::US2_PutRefused(v32)
        } else {
            US2::US2_PutAccepted
        }
    };
    match v36 {
        US2::US2_PutAccepted => {
            let v38: i32 = (v4.len() as i32);
            let v39: i32 = v38.wrapping_add(1i32);
            let v40: Rc<str> = draft_dir_14(v2, v1);
            let v42: i32 = (v14.clone().len() as i32);
            let v43: i32 = v42.wrapping_sub(v39);
            let v44: usize = ((v39) as usize);
            let v45: i32 = v39.wrapping_add(v43);
            let v46: usize = ((v45) as usize);
            let v47: Rc<str> = Rc::<str>::from(&(v14)[v44..v46]);
            let v48: Rc<str> = string_concat(&[&*v40, "/", &*v47]);
            moved_7(v14, v48)
        }
        US2::US2_PutRefused(v37) => {
            let v37: Rc<str> = v37;
            v37
        }
    }
}
fn method5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v3) {
        UH0::UH0_Cons(v5, v6) => {
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<UH0> = v6;
            let v7: Rc<UH0> = method5(v0.clone(), v1.clone(), v2.clone(), v6, v4);
            let v8: Rc<str> = put_one_6(v0, v1, v2, v5);
            Rc::new(UH0::UH0_Cons(v8, v7))
        }
        UH0::UH0_Nil => {
            v4
        }
    }
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method18(v3, v1);
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
fn method19(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
                let v3: Rc<str> = string_concat(&["drafts put refused: ", &*v1]);
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
fn method20(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn verdict_word_21(mut v0: &Rc<UH0>) -> Rc<str> {
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
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
        v3
    }
}
fn method23(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_22(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method23(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn put_run_1(mut v0: Rc<UH0>) -> i32 {
    let mut v1: Rc<UH0> = plain_words_2(v0.clone());
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let mut v5: Rc<UH0> = v5;
                    let mut v6: bool = match &*v5 {
                        UH0::UH0_Nil => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v7: bool = v6 != true;
                    if v7 {
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--backlog"); } LIT.with(|lit| lit.clone()) };
                        let v9: Rc<str> = flag_value_3(v0, v8);
                        let v10: Rc<str> = slash_4(v9);
                        let v11: Rc<str> = string_concat(&[&*v10, "/state/agile.spi"]);
                        let v12: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v11).unwrap_or_default().as_str());
                        let mut v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let mut v14: Rc<UH0> = method5(v12, v4, v2, v5.clone(), v13);
                        let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v16: Rc<UH0> = method18(v14, v15);
                        method19(v16.clone());
                        let mut v17: i32 = 0i32;
                        let mut v18: i32 = method20(v5, v17);
                        let v19: i32 = 0i32;
                        let v20: i32 = method20(v16.clone(), v19);
                        let v21: i32 = v18.wrapping_sub(v20);
                        let mut v22: Rc<str> = verdict_word_21(&v16);
                        let v25: Rc<str> = format_real_22(v21);
                        let v26: Rc<str> = string_concat(&["rig drafts put ", &*v22, " moved=", &*v25]);
                        println!("{}", v26);
                        let v27: bool = match Rc::unwrap_or_clone(v16) {
                            UH0::UH0_Nil => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        if v27 {
                            0i32
                        } else {
                            1i32
                        }
                    } else {
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: drafts put <repo root> <ITEM> <path...> --backlog <root>"); } LIT.with(|lit| lit.clone()) };
                        eprintln!("{}", v29);
                        2i32
                    }
                }
                _ => {
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: drafts put <repo root> <ITEM> <path...> --backlog <root>"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v31);
                    2i32
                }
            }
        }
        _ => {
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: drafts put <repo root> <ITEM> <path...> --backlog <root>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v33);
            2i32
        }
    }
}
fn command_line_argument_count_26() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_27(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_25(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_26();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US3 = if v4 {
        let mut v5: Rc<str> = command_line_argument_27(v0);
        US3::US3_Some(v5)
    } else {
        US3::US3_None
    };
    match v8 {
        US3::US3_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US3::US3_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_25(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn verb_words_24() -> Rc<UH0> {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_25(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
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
fn put_entry_0() -> i32 {
    let mut v0: Rc<UH0> = verb_words_24();
    put_run_1(v0)
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        put_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_drafts_put_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
