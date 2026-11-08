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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, u64, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, Rc<str>, u64),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_ChangeAppended,
    US2_ChangeUnchanged,
    US2_ChangeRefused(Rc<str>),
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US3 {
    US3_Replaced,
    US3_TemporaryWriteFailed,
    US3_RenameFailed,
    US3_RenameFailedTemporaryKept,
    US3_ReadbackMismatch,
    US3_ChangedSinceRead,
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: u64 = v4;
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
fn method4(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method5(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method5(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH1> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method4(v16, v17)
    }
}
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                let mut v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method9(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn method10(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method11(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let mut v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn change_of_line_7(mut v0: Rc<str>) -> US0 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method8(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0, v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method9(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = StatusChanged (\""); } LIT.with(|lit| lit.clone()) };
    let v10: Option<usize> = v8.find(&*v9);
    let v11: Option<usize> = method10(v10);
    let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v13: Option<i32> = v11.map(|x| v12(x));
    let v15: i32 = v13.unwrap_or(-1i32);
    let v16: bool = v15 < 0i32;
    let v20: bool = if v16 {
        true
    } else {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64)"); } LIT.with(|lit| lit.clone()) };
        let v18: bool = v8.ends_with(&*v17);
        let v19: bool = v18 == false;
        v19
    };
    if v20 {
        US0::US0_None
    } else {
        let v22: i32 = v15.wrapping_add(19i32);
        let v23: i32 = (v8.clone().len() as i32);
        let v24: i32 = v23.wrapping_sub(5i32);
        let v25: Rc<str> = string_slice(&v8, v22 as i64, v24 as i64);
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<UH1> = split_list_3(v26, v25);
        match Rc::unwrap_or_clone(v27) {
            UH1::UH1_Cons(v28, v29) => {
                let v28: Rc<str> = v28;
                let v29: Rc<UH1> = v29;
                match Rc::unwrap_or_clone(v29) {
                    UH1::UH1_Cons(v30, v31) => {
                        let v30: Rc<str> = v30;
                        let v31: Rc<UH1> = v31;
                        match Rc::unwrap_or_clone(v31) {
                            UH1::UH1_Cons(v32, v33) => {
                                let v32: Rc<str> = v32;
                                let v33: Rc<UH1> = v33;
                                match Rc::unwrap_or_clone(v33) {
                                    UH1::UH1_Nil => {
                                        let v34: bool = &*v28 != "";
                                        if v34 {
                                            let v35: Option<u64> = (v32).trim().parse::<u64>().ok();
                                            let v36: Option<u64> = method11(v35);
                                            let mut v37: Rc<dyn Fn((u64)) -> US1> = closure2();
                                            let mut v38: Option<US1> = v36.map(|x| v37(x));
                                            let v40: US1 = US1::US1_None;
                                            let v41: US1 = v40.clone();
                                            let v42: US1 = v38.unwrap_or(v41);
                                            match v42 {
                                                US1::US1_None => {
                                                    US0::US0_None
                                                }
                                                US1::US1_Some(v43) => {
                                                    let v43: u64 = v43;
                                                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    let v46: Rc<str> = Rc::<str>::from(v30.replace(&*v44, &*v45));
                                                    US0::US0_Some(v28, v46, v43)
                                                }
                                            }
                                        } else {
                                            US0::US0_None
                                        }
                                    }
                                    _ => {
                                        US0::US0_None
                                    }
                                }
                            }
                            _ => {
                                US0::US0_None
                            }
                        }
                    }
                    _ => {
                        US0::US0_None
                    }
                }
            }
            _ => {
                US0::US0_None
            }
        }
    }
}
fn method6(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: US0 = change_of_line_7(v2);
                let v10: Rc<UH0> = match v4 {
                    US0::US0_None => {
                        v1
                    }
                    US0::US0_Some(v5, v6, v7) => {
                        let v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        let v7: u64 = v7;
                        Rc::new(UH0::UH0_Cons(v5, v6, v7, v1))
                    }
                };
                (v0, v1) = (v3, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn changes_of_text_1(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH1> = split_list_3(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method6(v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method2(v7, v8)
}
fn method12(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: u64 = v5;
                let mut v6: Rc<UH0> = v6;
                v0.borrow_mut().push((v3, v4, v5));
                let v7: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v6, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn changes_of_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>> {
    let mut v1: Rc<UH0> = changes_of_text_1(v0);
    let mut v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>> = Rc::new(RefCell::new(Vec::new()));
    let v3: i32 = 0i32;
    let v4: i32 = method12(v2.clone(), v1, v3);
    let v5: Rc<Vec<(Rc<str>, Rc<str>, u64)>> = Rc::new(v2.borrow().clone());
    let mut v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    v6
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>> {
        changes_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn history_path_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = string_concat(&[&*v3, "/history/status_changes.spi"]);
    v4
}
fn file_exists_17(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn method18(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn text_of_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = file_exists_17(v0.clone());
    if v1 {
        let v3: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let v5: Vec<u8> = v3.unwrap();
        let mut v6: Vec<u8> = method18(v5);
        let v8: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v6);
        let v10: std::string::String = v8.unwrap();
        let mut v12: Rc<str> = Rc::<str>::from(String::as_str(&v10));
        v12
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    }
}
fn method20(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: u64 = v5;
                let mut v6: Rc<UH0> = v6;
                let v7: bool = &*v3 == &*v0;
                let v8: Rc<str> = if v7 {
                    v4
                } else {
                    v2
                };
                (v1, v2) = (v6, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn last_status_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<UH0> = changes_of_text_1(v0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method20(v1, v2, v3)
}
fn slug_of_23(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v3));
    v6
}
fn method25(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_24(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method25(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn row_of_change_22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> Rc<str> {
    let v3: Rc<str> = slug_of_23(v0.clone());
    let mut v6: Rc<str> = format_real_24(v2);
    let mut v13: Rc<str> = format_real_24(v2);
    let v15: Rc<str> = string_concat(&["inl status_changed_", &*v3, "_", &*v6, " () = StatusChanged (\"", &*v0, "\", \"", &*v1, "\", ", &*v13, "u64)"]);
    v15
}
fn appended_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> Rc<str> {
    let v4: bool = &*v0 == "";
    let mut v6: Rc<str> = if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion status_change_row =\n    | StatusChanged :: string * string * u64 -> status_change_row\n"); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        v0
    };
    let v7: bool = &*v6 == "";
    let v10: bool = if v7 {
        true
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v9: bool = v6.ends_with(&*v8);
        v9
    };
    let mut v12: Rc<str> = if v10 {
        v6
    } else {
        let v11: Rc<str> = string_concat(&[&*v6, "\n"]);
        v11
    };
    let mut v13: Rc<str> = row_of_change_22(v1, v2, v3);
    let v15: Rc<str> = string_concat(&[&*v12, &*v13, "\n"]);
    v15
}
fn renamed_within_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: bool = std::fs::rename(&*v1, &*v0).is_ok();
        if v3 {
            return true;
        } else {
            let v4: bool = v2 <= 1i32;
            if v4 {
                return false;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(50u64));
                let v5: i32 = v2.wrapping_sub(1i32);
                v2 = v5;
                continue;
            }
        }
    }
}
fn replaced_26(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = string_concat(&[&*v0, ".pending"]);
    let v3: bool = std::fs::write(&*v2, &*v1).is_ok();
    let v16: US3 = if v3 {
        let v4: i32 = 40i32;
        let v5: bool = renamed_within_27(v0.clone(), v2.clone(), v4);
        if v5 {
            let mut v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
            if v6 {
                US3::US3_Replaced
            } else {
                US3::US3_ReadbackMismatch
            }
        } else {
            let v10: bool = std::fs::remove_file(&*v2).is_ok() || !std::path::Path::new(&*v2).exists();
            if v10 {
                US3::US3_RenameFailed
            } else {
                US3::US3_RenameFailedTemporaryKept
            }
        }
    } else {
        US3::US3_TemporaryWriteFailed
    };
    match v16 {
        US3::US3_Replaced => {
            true
        }
        _ => {
            false
        }
    }
}
fn change_written_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> US2 {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
    let v7: Rc<str> = history_path_15(v0);
    let v8: Rc<str> = text_of_16(v7.clone());
    let mut v9: Rc<str> = string_concat(&[&*v6, "/state/agile.spi"]);
    let v10: Rc<str> = text_of_16(v9);
    let v11: bool = &*v10 == "";
    if v11 {
        let mut v13: Rc<str> = string_concat(&["no backlog state at ", &*v6, "/state"]);
        US2::US2_ChangeRefused(v13)
    } else {
        let v15: Rc<str> = last_status_19(v8.clone(), v1.clone());
        let v16: bool = &*v15 == &*v2;
        if v16 {
            US2::US2_ChangeUnchanged
        } else {
            let v18: Rc<str> = string_concat(&[&*v6, "/history"]);
            let v20: Result<(), std::io::Error> = std::fs::create_dir_all(&*v18);
            let v21: Rc<str> = appended_21(v8, v1, v2, v3);
            let v22: bool = replaced_26(v7, v21);
            if v22 {
                US2::US2_ChangeAppended
            } else {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("history/status_changes.spi could not be written"); } LIT.with(|lit| lit.clone()) };
                US2::US2_ChangeRefused(v24)
            }
        }
    }
}
fn change_recorded_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v4: US2 = change_written_14(v2, v1.clone(), v0, v3);
    match v4 {
        US2::US2_ChangeAppended => {
            0i32
        }
        US2::US2_ChangeRefused(v5) => {
            let v5: Rc<str> = v5;
            let v8: Rc<str> = string_concat(&["kit agile status change not recorded id=", &*v1, ": ", &*v5]);
            eprintln!("{}", v8);
            1i32
        }
        US2::US2_ChangeUnchanged => {
            0i32
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> i32 {
        change_recorded_13(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_status_changes(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_agile_status_change_recorded(v0: &str, v1: &str, v2: &str) -> i32 {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub type SpiralShape_accbf6de6842 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
