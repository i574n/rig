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
#[derive(Clone)]
enum US3 {
    US3_StateFile(Rc<str>),
    US3_HistoryFile(Rc<str>),
    US3_ArchiveFile(Rc<str>),
    US3_OtherFile,
}
#[derive(Clone)]
enum US5 {
    US5_HeadInl,
    US5_HeadUnion,
    US5_HeadOpen,
    US5_HeadType,
}
#[derive(Clone)]
enum US4 {
    US4_KeywordHead(US5),
    US4_SymbolHead(US5),
    US4_OtherLine,
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US6 {
    US6_WordsRoot,
    US6_SymbolsRoot,
}
#[derive(Clone)]
enum US7 {
    US7_Replaced,
    US7_TemporaryWriteFailed,
    US7_RenameFailed,
    US7_RenameFailedTemporaryKept,
    US7_ReadbackMismatch,
    US7_ChangedSinceRead,
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: u64 = v4;
                let mut v5: Rc<UH0> = v5;
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
fn method8(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn change_of_line_7(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = StatusChanged (\""); } LIT.with(|lit| lit.clone()) };
    let v3: Option<usize> = v1.find(&*v2);
    let v4: Option<usize> = method8(v3);
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let v8: i32 = v6.unwrap_or(-1i32);
    let mut v9: bool = v8 < 0i32;
    let mut v13: bool = if v9 {
        true
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64)"); } LIT.with(|lit| lit.clone()) };
        let v11: bool = v1.ends_with(&*v10);
        let mut v12: bool = v11 == false;
        v12
    };
    if v13 {
        US0::US0_None
    } else {
        let v15: i32 = v8.wrapping_add(19i32);
        let v16: i32 = (v1.clone().len() as i32);
        let v17: i32 = v16.wrapping_sub(5i32);
        let v18: Rc<str> = string_slice(&v1, v15 as i64, v17 as i64);
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<UH1> = split_list_3(v19, v18);
        match Rc::unwrap_or_clone(v20) {
            UH1::UH1_Cons(v21, v22) => {
                let v21: Rc<str> = v21;
                let v22: Rc<UH1> = v22;
                match Rc::unwrap_or_clone(v22) {
                    UH1::UH1_Cons(v23, v24) => {
                        let v23: Rc<str> = v23;
                        let v24: Rc<UH1> = v24;
                        match Rc::unwrap_or_clone(v24) {
                            UH1::UH1_Cons(v25, v26) => {
                                let v25: Rc<str> = v25;
                                let v26: Rc<UH1> = v26;
                                match Rc::unwrap_or_clone(v26) {
                                    UH1::UH1_Nil => {
                                        let v27: bool = &*v21 != "";
                                        if v27 {
                                            let v28: Option<u64> = (v25).trim().parse::<u64>().ok();
                                            let v29: Option<u64> = method9(v28);
                                            let mut v30: Rc<dyn Fn((u64)) -> US1> = closure2();
                                            let mut v31: Option<US1> = v29.map(|x| v30(x));
                                            let v33: US1 = US1::US1_None;
                                            let v34: US1 = v33.clone();
                                            let v35: US1 = v31.unwrap_or(v34);
                                            match v35 {
                                                US1::US1_None => {
                                                    US0::US0_None
                                                }
                                                US1::US1_Some(v36) => {
                                                    let v36: u64 = v36;
                                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    let v39: Rc<str> = Rc::<str>::from(v23.replace(&*v37, &*v38));
                                                    US0::US0_Some(v21, v39, v36)
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
                let mut v10: Rc<UH0> = match v4 {
                    US0::US0_None => {
                        v1
                    }
                    US0::US0_Some(v5, v6, v7) => {
                        let mut v5: Rc<str> = v5;
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
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH1> = split_list_3(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method6(v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method2(v7, v8)
}
fn method10(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let mut v5: u64 = v5;
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
    let v1: Rc<UH0> = changes_of_text_1(v0);
    let mut v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>> = Rc::new(RefCell::new(Vec::new()));
    let v3: i32 = 0i32;
    let v4: i32 = method10(v2.clone(), v1, v3);
    let mut v5: Rc<Vec<(Rc<str>, Rc<str>, u64)>> = Rc::new(v2.borrow().clone());
    let mut v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    v6
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>> {
        changes_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn history_path_13(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = string_concat(&[&*v3, "/history/status_changes.spi"]);
    v4
}
fn loop_15(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn closure4() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_16(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(6i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_17(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_18(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method19(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method19(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US4 = if v6 {
                let v7: US5 = US5::US5_HeadType;
                US4::US4_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US5 = US5::US5_HeadType;
                    US4::US4_SymbolHead(v11)
                } else {
                    US4::US4_OtherLine
                }
            };
            let v27: US4 = match &v15 {
                US4::US4_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US5 = US5::US5_HeadInl;
                        US4::US4_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US5 = US5::US5_HeadInl;
                            US4::US4_SymbolHead(v22)
                        } else {
                            US4::US4_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US4 = match &v27 {
                US4::US4_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let mut v30: US5 = US5::US5_HeadUnion;
                        US4::US4_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US5 = US5::US5_HeadUnion;
                            US4::US4_SymbolHead(v34)
                        } else {
                            US4::US4_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US4 = match &v39 {
                US4::US4_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US5 = US5::US5_HeadOpen;
                        US4::US4_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US5 = US5::US5_HeadOpen;
                            US4::US4_SymbolHead(v46)
                        } else {
                            US4::US4_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US4::US4_SymbolHead(v52) => {
                    let v52: US5 = v52;
                    let v60: Rc<str> = match &v52 {
                        US5::US5_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US5::US5_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US5::US5_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US5::US5_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US5::US5_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US5::US5_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US5::US5_HeadType => {
                            v5
                        }
                        US5::US5_HeadUnion => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                    };
                    let v69: i32 = (v2.clone().len() as i32);
                    let v70: i32 = v69.wrapping_sub(v61);
                    let v71: usize = ((v61) as usize);
                    let v72: i32 = v61.wrapping_add(v70);
                    let v73: usize = ((v72) as usize);
                    let v74: Rc<str> = Rc::<str>::from(&(v2)[v71..v73]);
                    let v75: Rc<str> = string_concat(&[&*v68, &*v74]);
                    v75
                }
                _ => {
                    v2
                }
            };
            Rc::new(UH1::UH1_Cons(v76, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method20(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn text_of_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = std::path::Path::new(&*v0).is_file();
    if v1 {
        let (mut v2, mut v3): (bool, Rc<str>) = match std::fs::read(&*v0) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v2 {
            let v4: Rc<str> = string_concat(&["rig state read failed: ", &*v0]);
            std::panic::panic_any::<std::string::String>(format!("{}", v4))
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
            let mut v9: Rc<str> = loop_15(v8);
            let mut v10: Option<usize> = v9.rfind(&*v7);
            let v11: Option<usize> = method8(v10);
            let mut v12: Rc<dyn Fn((usize)) -> i32> = closure4();
            let mut v13: Option<i32> = v11.map(|x| v12(x));
            let v15: i32 = v13.unwrap_or(-1i32);
            let v16: bool = v15 < 0i32;
            let v20: bool = if v16 {
                true
            } else {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v18: bool = v9.ends_with(&*v17);
                let v19: bool = v18 == false;
                v19
            };
            let v41: US3 = if v20 {
                US3::US3_OtherFile
            } else {
                let v22: usize = ((0i32) as usize);
                let v23: usize = ((v15) as usize);
                let v24: Rc<str> = Rc::<str>::from(&(v9)[v22..v23]);
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v26: bool = v24.ends_with(&*v25);
                if v26 {
                    let v27: Rc<str> = loop_16(v24);
                    US3::US3_StateFile(v27)
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let mut v30: bool = v24.ends_with(&*v29);
                    if v30 {
                        let mut v31: Rc<str> = loop_17(v24);
                        US3::US3_HistoryFile(v31)
                    } else {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v34: bool = v24.ends_with(&*v33);
                        if v34 {
                            let v35: Rc<str> = loop_18(v24);
                            US3::US3_ArchiveFile(v35)
                        } else {
                            US3::US3_OtherFile
                        }
                    }
                }
            };
            match v41 {
                US3::US3_OtherFile => {
                    v3
                }
                _ => {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<UH1> = split_list_3(v42.clone(), v3);
                    let v44: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v45: Rc<UH1> = method19(v43, v44);
                    let v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v47: i32 = 0i32;
                    let v48: i32 = method20(v46.clone(), v45, v47);
                    let v49: Rc<Vec<Rc<str>>> = Rc::new(v46.borrow().clone());
                    let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v49).as_ref().clone()));
                    let v51: Rc<str> = Rc::<str>::from((*v50.borrow()).join(&*v42));
                    v51
                }
            }
        }
    } else {
        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v54
    }
}
fn method22(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let mut v5: u64 = v5;
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
fn last_status_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<UH0> = changes_of_text_1(v0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method22(v1, v2, v3)
}
fn slug_of_25(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v3));
    v6
}
fn method27(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_26(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method27(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn row_of_change_24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> Rc<str> {
    let v3: Rc<str> = slug_of_25(v0.clone());
    let mut v6: Rc<str> = format_real_26(v2);
    let mut v13: Rc<str> = format_real_26(v2);
    let v15: Rc<str> = string_concat(&["inl status_changed_", &*v3, "_", &*v6, " () = StatusChanged (\"", &*v0, "\", \"", &*v1, "\", ", &*v13, "u64)"]);
    v15
}
fn appended_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> Rc<str> {
    let v4: bool = &*v0 == "";
    let mut v6: Rc<str> = if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion status_change_row =\n    | StatusChanged :: string * string * u64 -> status_change_row\n"); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        v0
    };
    let v7: bool = &*v6 == "";
    let mut v10: bool = if v7 {
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
    let mut v13: Rc<str> = row_of_change_24(v1, v2, v3);
    let v15: Rc<str> = string_concat(&[&*v12, &*v13, "\n"]);
    v15
}
fn method29(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method29(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US4 = if v6 {
                let v7: US5 = US5::US5_HeadType;
                US4::US4_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US5 = US5::US5_HeadType;
                    US4::US4_SymbolHead(v11)
                } else {
                    US4::US4_OtherLine
                }
            };
            let v27: US4 = match &v15 {
                US4::US4_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US5 = US5::US5_HeadInl;
                        US4::US4_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US5 = US5::US5_HeadInl;
                            US4::US4_SymbolHead(v22)
                        } else {
                            US4::US4_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US4 = match &v27 {
                US4::US4_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let mut v30: US5 = US5::US5_HeadUnion;
                        US4::US4_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US5 = US5::US5_HeadUnion;
                            US4::US4_SymbolHead(v34)
                        } else {
                            US4::US4_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US4 = match &v39 {
                US4::US4_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US5 = US5::US5_HeadOpen;
                        US4::US4_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US5 = US5::US5_HeadOpen;
                            US4::US4_SymbolHead(v46)
                        } else {
                            US4::US4_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US4::US4_KeywordHead(v52) => {
                    let v52: US5 = v52;
                    let v59: Rc<str> = match &v52 {
                        US5::US5_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US5::US5_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US5::US5_HeadType => {
                            v5
                        }
                        US5::US5_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v60: i32 = (v59.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US5::US5_HeadInl => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v61
                        }
                        US5::US5_HeadOpen => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                        US5::US5_HeadType => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US5::US5_HeadUnion => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                    };
                    let v69: i32 = (v2.clone().len() as i32);
                    let v70: i32 = v69.wrapping_sub(v60);
                    let v71: usize = ((v60) as usize);
                    let v72: i32 = v60.wrapping_add(v70);
                    let v73: usize = ((v72) as usize);
                    let v74: Rc<str> = Rc::<str>::from(&(v2)[v71..v73]);
                    let v75: Rc<str> = string_concat(&[&*v68, &*v74]);
                    v75
                }
                _ => {
                    v2
                }
            };
            Rc::new(UH1::UH1_Cons(v76, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn renamed_within_30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
                let mut v5: i32 = v2.wrapping_sub(1i32);
                v2 = v5;
                continue;
            }
        }
    }
}
fn replaced_28(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = string_concat(&[&*v0, ".pending"]);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let mut v6: Rc<str> = loop_15(v5);
    let v7: Option<usize> = v6.rfind(&*v4);
    let v8: Option<usize> = method8(v7);
    let mut v9: Rc<dyn Fn((usize)) -> i32> = closure4();
    let mut v10: Option<i32> = v8.map(|x| v9(x));
    let mut v12: i32 = v10.unwrap_or(-1i32);
    let mut v13: bool = v12 < 0i32;
    let v17: bool = if v13 {
        true
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v15: bool = v6.ends_with(&*v14);
        let v16: bool = v15 == false;
        v16
    };
    let v38: US3 = if v17 {
        US3::US3_OtherFile
    } else {
        let v19: usize = ((0i32) as usize);
        let v20: usize = ((v12) as usize);
        let v21: Rc<str> = Rc::<str>::from(&(v6)[v19..v20]);
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v23: bool = v21.ends_with(&*v22);
        if v23 {
            let v24: Rc<str> = loop_16(v21);
            US3::US3_StateFile(v24)
        } else {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
            let v27: bool = v21.ends_with(&*v26);
            if v27 {
                let v28: Rc<str> = loop_17(v21);
                US3::US3_HistoryFile(v28)
            } else {
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                let mut v31: bool = v21.ends_with(&*v30);
                if v31 {
                    let v32: Rc<str> = loop_18(v21);
                    US3::US3_ArchiveFile(v32)
                } else {
                    US3::US3_OtherFile
                }
            }
        }
    };
    let v61: US6 = match v38 {
        US3::US3_ArchiveFile(v51) => {
            let v51: Rc<str> = v51;
            let v52: Rc<str> = string_concat(&[&*v51, "/state/spiral.toml"]);
            let v53: bool = std::path::Path::new(&*v52).is_file();
            if v53 {
                US6::US6_SymbolsRoot
            } else {
                US6::US6_WordsRoot
            }
        }
        US3::US3_HistoryFile(v45) => {
            let v45: Rc<str> = v45;
            let v46: Rc<str> = string_concat(&[&*v45, "/state/spiral.toml"]);
            let v47: bool = std::path::Path::new(&*v46).is_file();
            if v47 {
                US6::US6_SymbolsRoot
            } else {
                US6::US6_WordsRoot
            }
        }
        US3::US3_OtherFile => {
            US6::US6_WordsRoot
        }
        US3::US3_StateFile(v39) => {
            let v39: Rc<str> = v39;
            let v40: Rc<str> = string_concat(&[&*v39, "/state/spiral.toml"]);
            let v41: bool = std::path::Path::new(&*v40).is_file();
            if v41 {
                US6::US6_SymbolsRoot
            } else {
                US6::US6_WordsRoot
            }
        }
    };
    let v73: Rc<str> = match v61 {
        US6::US6_SymbolsRoot => {
            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v63: Rc<UH1> = split_list_3(v62.clone(), v1.clone());
            let v64: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v65: Rc<UH1> = method29(v63, v64);
            let v66: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v67: i32 = 0i32;
            let v68: i32 = method20(v66.clone(), v65, v67);
            let v69: Rc<Vec<Rc<str>>> = Rc::new(v66.borrow().clone());
            let v70: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v69).as_ref().clone()));
            let v71: Rc<str> = Rc::<str>::from((*v70.borrow()).join(&*v62));
            v71
        }
        US6::US6_WordsRoot => {
            v1.clone()
        }
    };
    let v74: bool = std::fs::write(&*v2, &*v73).is_ok();
    let v155: US7 = if v74 {
        let v75: i32 = 40i32;
        let v76: bool = renamed_within_30(v0.clone(), v2.clone(), v75);
        if v76 {
            let v77: Rc<str> = Rc::<str>::from(v0.replace(&*v3, &*v4));
            let mut v78: Rc<str> = loop_15(v77);
            let v79: Option<usize> = v78.rfind(&*v4);
            let v80: Option<usize> = method8(v79);
            let mut v81: Option<i32> = v80.map(|x| v9(x));
            let v83: i32 = v81.unwrap_or(-1i32);
            let v84: bool = v83 < 0i32;
            let v88: bool = if v84 {
                true
            } else {
                let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v86: bool = v78.ends_with(&*v85);
                let v87: bool = v86 == false;
                v87
            };
            let v109: US3 = if v88 {
                US3::US3_OtherFile
            } else {
                let v90: usize = ((0i32) as usize);
                let v91: usize = ((v83) as usize);
                let v92: Rc<str> = Rc::<str>::from(&(v78)[v90..v91]);
                let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v94: bool = v92.ends_with(&*v93);
                if v94 {
                    let v95: Rc<str> = loop_16(v92);
                    US3::US3_StateFile(v95)
                } else {
                    let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v98: bool = v92.ends_with(&*v97);
                    if v98 {
                        let v99: Rc<str> = loop_17(v92);
                        US3::US3_HistoryFile(v99)
                    } else {
                        let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v102: bool = v92.ends_with(&*v101);
                        if v102 {
                            let v103: Rc<str> = loop_18(v92);
                            US3::US3_ArchiveFile(v103)
                        } else {
                            US3::US3_OtherFile
                        }
                    }
                }
            };
            let v132: US6 = match v109 {
                US3::US3_ArchiveFile(v122) => {
                    let v122: Rc<str> = v122;
                    let v123: Rc<str> = string_concat(&[&*v122, "/state/spiral.toml"]);
                    let v124: bool = std::path::Path::new(&*v123).is_file();
                    if v124 {
                        US6::US6_SymbolsRoot
                    } else {
                        US6::US6_WordsRoot
                    }
                }
                US3::US3_HistoryFile(v116) => {
                    let v116: Rc<str> = v116;
                    let v117: Rc<str> = string_concat(&[&*v116, "/state/spiral.toml"]);
                    let v118: bool = std::path::Path::new(&*v117).is_file();
                    if v118 {
                        US6::US6_SymbolsRoot
                    } else {
                        US6::US6_WordsRoot
                    }
                }
                US3::US3_OtherFile => {
                    US6::US6_WordsRoot
                }
                US3::US3_StateFile(v110) => {
                    let v110: Rc<str> = v110;
                    let v111: Rc<str> = string_concat(&[&*v110, "/state/spiral.toml"]);
                    let v112: bool = std::path::Path::new(&*v111).is_file();
                    if v112 {
                        US6::US6_SymbolsRoot
                    } else {
                        US6::US6_WordsRoot
                    }
                }
            };
            let v144: Rc<str> = match v132 {
                US6::US6_SymbolsRoot => {
                    let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v134: Rc<UH1> = split_list_3(v133.clone(), v1);
                    let v135: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v136: Rc<UH1> = method29(v134, v135);
                    let v137: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v138: i32 = 0i32;
                    let v139: i32 = method20(v137.clone(), v136, v138);
                    let v140: Rc<Vec<Rc<str>>> = Rc::new(v137.borrow().clone());
                    let v141: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v140).as_ref().clone()));
                    let v142: Rc<str> = Rc::<str>::from((*v141.borrow()).join(&*v133));
                    v142
                }
                US6::US6_WordsRoot => {
                    v1
                }
            };
            let v145: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v144);
            if v145 {
                US7::US7_Replaced
            } else {
                US7::US7_ReadbackMismatch
            }
        } else {
            let v149: bool = std::fs::remove_file(&*v2).is_ok() || !std::path::Path::new(&*v2).exists();
            if v149 {
                US7::US7_RenameFailed
            } else {
                US7::US7_RenameFailedTemporaryKept
            }
        }
    } else {
        US7::US7_TemporaryWriteFailed
    };
    match v155 {
        US7::US7_Replaced => {
            true
        }
        _ => {
            false
        }
    }
}
fn change_written_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> US2 {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
    let v7: Rc<str> = history_path_13(v0);
    let v8: Rc<str> = text_of_14(v7.clone());
    let mut v9: Rc<str> = string_concat(&[&*v6, "/state/agile.spi"]);
    let mut v10: Rc<str> = text_of_14(v9);
    let v11: bool = &*v10 == "";
    if v11 {
        let mut v13: Rc<str> = string_concat(&["no backlog state at ", &*v6, "/state"]);
        US2::US2_ChangeRefused(v13)
    } else {
        let v15: Rc<str> = last_status_21(v8.clone(), v1.clone());
        let v16: bool = &*v15 == &*v2;
        if v16 {
            US2::US2_ChangeUnchanged
        } else {
            let v18: Rc<str> = string_concat(&[&*v6, "/history"]);
            let v20: Result<(), std::io::Error> = std::fs::create_dir_all(&*v18);
            let v21: Rc<str> = appended_23(v8, v1, v2, v3);
            let v22: bool = replaced_28(v7, v21);
            if v22 {
                US2::US2_ChangeAppended
            } else {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("history/status_changes.spi could not be written"); } LIT.with(|lit| lit.clone()) };
                US2::US2_ChangeRefused(v24)
            }
        }
    }
}
fn change_recorded_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v4: US2 = change_written_12(v2, v1.clone(), v0, v3);
    match v4 {
        US2::US2_ChangeAppended => {
            0i32
        }
        US2::US2_ChangeRefused(v5) => {
            let mut v5: Rc<str> = v5;
            let v8: Rc<str> = string_concat(&["rig agile status change not recorded id=", &*v1, ": ", &*v5]);
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
        change_recorded_11(v2, v1, v0)
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
