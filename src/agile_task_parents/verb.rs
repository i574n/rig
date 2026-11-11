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
    UH0_Cons(Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_ParentSet(Rc<str>, Rc<str>),
    US3_ParentCleared(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_ParentsWritten(i32),
    US4_ParentsRefused(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_StateFile(Rc<str>),
    US5_HistoryFile(Rc<str>),
    US5_ArchiveFile(Rc<str>),
    US5_OtherFile,
}
#[derive(Clone)]
enum US7 {
    US7_HeadInl,
    US7_HeadUnion,
    US7_HeadOpen,
    US7_HeadType,
}
#[derive(Clone)]
enum US6 {
    US6_KeywordHead(US7),
    US6_SymbolHead(US7),
    US6_OtherLine,
}
#[derive(Clone)]
enum US8 {
    US8_WordsRoot,
    US8_SymbolsRoot,
}
#[derive(Clone)]
enum US9 {
    US9_Replaced,
    US9_TemporaryWriteFailed,
    US9_RenameFailed,
    US9_RenameFailedTemporaryKept,
    US9_ReadbackMismatch,
    US9_ChangedSinceRead,
}
#[derive(Clone)]
enum US10 {
    US10_SpiprojProject,
    US10_TomlProject,
}
#[derive(Clone)]
enum US11 {
    US11_ModuleListed,
    US11_ModuleAdded(Rc<str>),
    US11_NoModulesSection,
}
#[derive(Clone)]
enum US12 {
    US12_SplitProject(Rc<UH1>, Rc<UH1>, Rc<UH1>),
    US12_NoModules,
}
#[derive(Clone)]
enum US13 {
    US13_Some(i32),
    US13_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
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
        let mut v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
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
fn loop_12(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(2i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn quoted_pair_11(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH1> = split_list_3(v1, v0);
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH1> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH1::UH1_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let mut v6: Rc<UH1> = v6;
                    match Rc::unwrap_or_clone(v6) {
                        UH1::UH1_Nil => {
                            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
                            let v8: bool = v5.ends_with(&*v7);
                            if v8 {
                                let mut v9: Rc<str> = loop_12(v5);
                                let mut v10: bool = &*v3 == "";
                                let mut v12: bool = if v10 {
                                    true
                                } else {
                                    let v11: bool = &*v9 == "";
                                    v11
                                };
                                if v12 {
                                    US1::US1_None
                                } else {
                                    US1::US1_Some(v3, v9)
                                }
                            } else {
                                US1::US1_None
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
fn link_of_line_7(mut v0: Rc<str>) -> US0 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method8(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0, v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method9(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Parent (\""); } LIT.with(|lit| lit.clone()) };
    let mut v10: Option<usize> = v8.find(&*v9);
    let v11: Option<usize> = method10(v10);
    let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v13: Option<i32> = v11.map(|x| v12(x));
    let v15: i32 = v13.unwrap_or(-1i32);
    let v16: bool = v15 < 0i32;
    if v16 {
        US0::US0_None
    } else {
        let v18: i32 = v15.wrapping_add(12i32);
        let v19: i32 = (v8.clone().len() as i32);
        let v20: i32 = v19.wrapping_sub(1i32);
        let v21: Rc<str> = string_slice(&v8, v18 as i64, v20 as i64);
        let v22: US1 = quoted_pair_11(v21);
        match v22 {
            US1::US1_None => {
                US0::US0_None
            }
            US1::US1_Some(v23, v24) => {
                let v23: Rc<str> = v23;
                let mut v24: Rc<str> = v24;
                US0::US0_Some(v23, v24)
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
                let v4: US0 = link_of_line_7(v2);
                let mut v9: Rc<UH0> = match v4 {
                    US0::US0_None => {
                        v1
                    }
                    US0::US0_Some(v5, v6) => {
                        let v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        Rc::new(UH0::UH0_Cons(v5, v6, v1))
                    }
                };
                (v0, v1) = (v3, v9);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn links_of_text_1(mut v0: Rc<str>) -> Rc<UH0> {
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
fn method13(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                v0.borrow_mut().push((v3, v4));
                let mut v6: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn links_of_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> {
    let mut v1: Rc<UH0> = links_of_text_1(v0);
    let mut v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v3: i32 = 0i32;
    let v4: i32 = method13(v2.clone(), v1, v3);
    let v5: Rc<Vec<(Rc<str>, Rc<str>)>> = Rc::new(v2.borrow().clone());
    let mut v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    v6
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> {
        links_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn command_line_argument_count_16() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_17(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_15(mut v0: i32) -> Rc<UH1> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_16();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US2 = if v4 {
        let v5: Rc<str> = command_line_argument_17(v0);
        US2::US2_Some(v5)
    } else {
        US2::US2_None
    };
    match v8 {
        US2::US2_None => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
        }
        US2::US2_Some(v10) => {
            let mut v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let mut v12: Rc<UH1> = arguments_from_15(v11);
            Rc::new(UH1::UH1_Cons(v10, v12))
        }
    }
}
fn loop_20(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
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
fn loop_21(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(6i32);
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
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_23(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method24(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method24(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US6 = if v6 {
                let v7: US7 = US7::US7_HeadType;
                US6::US6_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US7 = US7::US7_HeadType;
                    US6::US6_SymbolHead(v11)
                } else {
                    US6::US6_OtherLine
                }
            };
            let v27: US6 = match &v15 {
                US6::US6_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US7 = US7::US7_HeadInl;
                        US6::US6_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US7 = US7::US7_HeadInl;
                            US6::US6_SymbolHead(v22)
                        } else {
                            US6::US6_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US6 = match &v27 {
                US6::US6_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US7 = US7::US7_HeadUnion;
                        US6::US6_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US7 = US7::US7_HeadUnion;
                            US6::US6_SymbolHead(v34)
                        } else {
                            US6::US6_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US6 = match &v39 {
                US6::US6_OtherLine => {
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US7 = US7::US7_HeadOpen;
                        US6::US6_KeywordHead(v42)
                    } else {
                        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US7 = US7::US7_HeadOpen;
                            US6::US6_SymbolHead(v46)
                        } else {
                            US6::US6_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US6::US6_SymbolHead(v52) => {
                    let v52: US7 = v52;
                    let v60: Rc<str> = match &v52 {
                        US7::US7_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US7::US7_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US7::US7_HeadType => {
                            let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US7::US7_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US7::US7_HeadInl => {
                            let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US7::US7_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US7::US7_HeadType => {
                            v5
                        }
                        US7::US7_HeadUnion => {
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
fn method25(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn text_of_19(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = std::path::Path::new(&*v0).is_file();
    if v1 {
        let (mut v2, mut v3): (bool, Rc<str>) = match std::fs::read(&*v0) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v2 {
            let v4: Rc<str> = string_concat(&["rig state read failed: ", &*v0]);
            std::panic::panic_any::<std::string::String>(format!("{}", v4))
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
            let mut v9: Rc<str> = loop_20(v8);
            let mut v10: Option<usize> = v9.rfind(&*v7);
            let v11: Option<usize> = method10(v10);
            let mut v12: Rc<dyn Fn((usize)) -> i32> = closure3();
            let mut v13: Option<i32> = v11.map(|x| v12(x));
            let v15: i32 = v13.unwrap_or(-1i32);
            let v16: bool = v15 < 0i32;
            let v20: bool = if v16 {
                true
            } else {
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v18: bool = v9.ends_with(&*v17);
                let v19: bool = v18 == false;
                v19
            };
            let v41: US5 = if v20 {
                US5::US5_OtherFile
            } else {
                let v22: usize = ((0i32) as usize);
                let v23: usize = ((v15) as usize);
                let mut v24: Rc<str> = Rc::<str>::from(&(v9)[v22..v23]);
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v26: bool = v24.ends_with(&*v25);
                if v26 {
                    let v27: Rc<str> = loop_21(v24);
                    US5::US5_StateFile(v27)
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v30: bool = v24.ends_with(&*v29);
                    if v30 {
                        let v31: Rc<str> = loop_22(v24);
                        US5::US5_HistoryFile(v31)
                    } else {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v34: bool = v24.ends_with(&*v33);
                        if v34 {
                            let v35: Rc<str> = loop_23(v24);
                            US5::US5_ArchiveFile(v35)
                        } else {
                            US5::US5_OtherFile
                        }
                    }
                }
            };
            match v41 {
                US5::US5_OtherFile => {
                    v3
                }
                _ => {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<UH1> = split_list_3(v42.clone(), v3);
                    let mut v44: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v45: Rc<UH1> = method24(v43, v44);
                    let v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v47: i32 = 0i32;
                    let v48: i32 = method25(v46.clone(), v45, v47);
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
fn task_known_26(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v3: Rc<str> = string_concat(&["Task (\"", &*v1, "\""]);
    let v4: bool = v0.contains(&*v3);
    v4
}
fn method28(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method28(v4, v1);
            Rc::new(UH0::UH0_Cons(v2, v3, v5))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method29(mut v0: US3, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4, v5) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            let mut v6: Rc<UH0> = method29(v0.clone(), v5, v2);
            let v11: Rc<str> = match v0 {
                US3::US3_ParentCleared(v9) => {
                    let mut v9: Rc<str> = v9;
                    v9
                }
                US3::US3_ParentSet(v7, v8) => {
                    let v7: Rc<str> = v7;
                    let v8: Rc<str> = v8;
                    v7
                }
            };
            let mut v12: bool = &*v3 != &*v11;
            if v12 {
                Rc::new(UH0::UH0_Cons(v3, v4, v6))
            } else {
                v6
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn links_changed_27(mut v0: Rc<UH0>, mut v1: US3) -> Rc<UH0> {
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH0> = method29(v1.clone(), v0, v2);
    match v1 {
        US3::US3_ParentCleared(v9) => {
            let mut v9: Rc<str> = v9;
            v3
        }
        US3::US3_ParentSet(v4, v5) => {
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5, v6));
            method28(v3, v7)
        }
    }
}
fn slug_of_33(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v3));
    v6
}
fn row_of_link_32(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = slug_of_33(v0.clone());
    let v8: Rc<str> = string_concat(&["inl parent_of_", &*v2, " () = Parent (\"", &*v0, "\", \"", &*v1, "\")"]);
    v8
}
fn method31(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH1> = method31(v4, v1);
            let mut v6: Rc<str> = row_of_link_32(v2, v3);
            let v7: Rc<str> = string_concat(&[&*v6, "\n"]);
            Rc::new(UH1::UH1_Cons(v7, v5))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn parents_text_30(mut v0: Rc<UH0>) -> Rc<str> {
    let mut v1: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH1> = method31(v0, v1);
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v4: i32 = 0i32;
    let v5: i32 = method25(v3.clone(), v2, v4);
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from((*v7.borrow()).join(&*v8));
    let mut v10: Rc<str> = string_concat(&["///- --no-respell\nunion task_parent =\n    | Parent :: string * string -> task_parent\n", &*v9]);
    v10
}
fn method35(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method35(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US6 = if v6 {
                let v7: US7 = US7::US7_HeadType;
                US6::US6_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US7 = US7::US7_HeadType;
                    US6::US6_SymbolHead(v11)
                } else {
                    US6::US6_OtherLine
                }
            };
            let v27: US6 = match &v15 {
                US6::US6_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US7 = US7::US7_HeadInl;
                        US6::US6_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US7 = US7::US7_HeadInl;
                            US6::US6_SymbolHead(v22)
                        } else {
                            US6::US6_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US6 = match &v27 {
                US6::US6_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US7 = US7::US7_HeadUnion;
                        US6::US6_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US7 = US7::US7_HeadUnion;
                            US6::US6_SymbolHead(v34)
                        } else {
                            US6::US6_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US6 = match &v39 {
                US6::US6_OtherLine => {
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US7 = US7::US7_HeadOpen;
                        US6::US6_KeywordHead(v42)
                    } else {
                        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US7 = US7::US7_HeadOpen;
                            US6::US6_SymbolHead(v46)
                        } else {
                            US6::US6_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US6::US6_KeywordHead(v52) => {
                    let v52: US7 = v52;
                    let v59: Rc<str> = match &v52 {
                        US7::US7_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US7::US7_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US7::US7_HeadType => {
                            v5
                        }
                        US7::US7_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v60: i32 = (v59.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US7::US7_HeadInl => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v61
                        }
                        US7::US7_HeadOpen => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                        US7::US7_HeadType => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US7::US7_HeadUnion => {
                            let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
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
fn renamed_within_36(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn replaced_34(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = string_concat(&[&*v0, ".pending"]);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let mut v6: Rc<str> = loop_20(v5);
    let v7: Option<usize> = v6.rfind(&*v4);
    let v8: Option<usize> = method10(v7);
    let mut v9: Rc<dyn Fn((usize)) -> i32> = closure3();
    let mut v10: Option<i32> = v8.map(|x| v9(x));
    let mut v12: i32 = v10.unwrap_or(-1i32);
    let mut v13: bool = v12 < 0i32;
    let mut v17: bool = if v13 {
        true
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v15: bool = v6.ends_with(&*v14);
        let v16: bool = v15 == false;
        v16
    };
    let v38: US5 = if v17 {
        US5::US5_OtherFile
    } else {
        let v19: usize = ((0i32) as usize);
        let v20: usize = ((v12) as usize);
        let v21: Rc<str> = Rc::<str>::from(&(v6)[v19..v20]);
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v23: bool = v21.ends_with(&*v22);
        if v23 {
            let mut v24: Rc<str> = loop_21(v21);
            US5::US5_StateFile(v24)
        } else {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
            let v27: bool = v21.ends_with(&*v26);
            if v27 {
                let v28: Rc<str> = loop_22(v21);
                US5::US5_HistoryFile(v28)
            } else {
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                let v31: bool = v21.ends_with(&*v30);
                if v31 {
                    let v32: Rc<str> = loop_23(v21);
                    US5::US5_ArchiveFile(v32)
                } else {
                    US5::US5_OtherFile
                }
            }
        }
    };
    let v61: US8 = match v38 {
        US5::US5_ArchiveFile(v51) => {
            let v51: Rc<str> = v51;
            let v52: Rc<str> = string_concat(&[&*v51, "/state/spiral.toml"]);
            let v53: bool = std::path::Path::new(&*v52).is_file();
            if v53 {
                US8::US8_SymbolsRoot
            } else {
                US8::US8_WordsRoot
            }
        }
        US5::US5_HistoryFile(v45) => {
            let v45: Rc<str> = v45;
            let v46: Rc<str> = string_concat(&[&*v45, "/state/spiral.toml"]);
            let v47: bool = std::path::Path::new(&*v46).is_file();
            if v47 {
                US8::US8_SymbolsRoot
            } else {
                US8::US8_WordsRoot
            }
        }
        US5::US5_OtherFile => {
            US8::US8_WordsRoot
        }
        US5::US5_StateFile(v39) => {
            let v39: Rc<str> = v39;
            let mut v40: Rc<str> = string_concat(&[&*v39, "/state/spiral.toml"]);
            let v41: bool = std::path::Path::new(&*v40).is_file();
            if v41 {
                US8::US8_SymbolsRoot
            } else {
                US8::US8_WordsRoot
            }
        }
    };
    let v73: Rc<str> = match v61 {
        US8::US8_SymbolsRoot => {
            let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v63: Rc<UH1> = split_list_3(v62.clone(), v1.clone());
            let v64: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v65: Rc<UH1> = method35(v63, v64);
            let v66: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v67: i32 = 0i32;
            let v68: i32 = method25(v66.clone(), v65, v67);
            let v69: Rc<Vec<Rc<str>>> = Rc::new(v66.borrow().clone());
            let v70: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v69).as_ref().clone()));
            let v71: Rc<str> = Rc::<str>::from((*v70.borrow()).join(&*v62));
            v71
        }
        US8::US8_WordsRoot => {
            v1.clone()
        }
    };
    let v74: bool = std::fs::write(&*v2, &*v73).is_ok();
    let v155: US9 = if v74 {
        let v75: i32 = 40i32;
        let v76: bool = renamed_within_36(v0.clone(), v2.clone(), v75);
        if v76 {
            let v77: Rc<str> = Rc::<str>::from(v0.replace(&*v3, &*v4));
            let mut v78: Rc<str> = loop_20(v77);
            let v79: Option<usize> = v78.rfind(&*v4);
            let v80: Option<usize> = method10(v79);
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
            let v109: US5 = if v88 {
                US5::US5_OtherFile
            } else {
                let v90: usize = ((0i32) as usize);
                let v91: usize = ((v83) as usize);
                let v92: Rc<str> = Rc::<str>::from(&(v78)[v90..v91]);
                let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v94: bool = v92.ends_with(&*v93);
                if v94 {
                    let v95: Rc<str> = loop_21(v92);
                    US5::US5_StateFile(v95)
                } else {
                    let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v98: bool = v92.ends_with(&*v97);
                    if v98 {
                        let v99: Rc<str> = loop_22(v92);
                        US5::US5_HistoryFile(v99)
                    } else {
                        let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v102: bool = v92.ends_with(&*v101);
                        if v102 {
                            let v103: Rc<str> = loop_23(v92);
                            US5::US5_ArchiveFile(v103)
                        } else {
                            US5::US5_OtherFile
                        }
                    }
                }
            };
            let v132: US8 = match v109 {
                US5::US5_ArchiveFile(v122) => {
                    let v122: Rc<str> = v122;
                    let v123: Rc<str> = string_concat(&[&*v122, "/state/spiral.toml"]);
                    let v124: bool = std::path::Path::new(&*v123).is_file();
                    if v124 {
                        US8::US8_SymbolsRoot
                    } else {
                        US8::US8_WordsRoot
                    }
                }
                US5::US5_HistoryFile(v116) => {
                    let v116: Rc<str> = v116;
                    let v117: Rc<str> = string_concat(&[&*v116, "/state/spiral.toml"]);
                    let v118: bool = std::path::Path::new(&*v117).is_file();
                    if v118 {
                        US8::US8_SymbolsRoot
                    } else {
                        US8::US8_WordsRoot
                    }
                }
                US5::US5_OtherFile => {
                    US8::US8_WordsRoot
                }
                US5::US5_StateFile(v110) => {
                    let v110: Rc<str> = v110;
                    let v111: Rc<str> = string_concat(&[&*v110, "/state/spiral.toml"]);
                    let v112: bool = std::path::Path::new(&*v111).is_file();
                    if v112 {
                        US8::US8_SymbolsRoot
                    } else {
                        US8::US8_WordsRoot
                    }
                }
            };
            let v144: Rc<str> = match v132 {
                US8::US8_SymbolsRoot => {
                    let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v134: Rc<UH1> = split_list_3(v133.clone(), v1);
                    let v135: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v136: Rc<UH1> = method35(v134, v135);
                    let v137: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v138: i32 = 0i32;
                    let v139: i32 = method25(v137.clone(), v136, v138);
                    let v140: Rc<Vec<Rc<str>>> = Rc::new(v137.borrow().clone());
                    let v141: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v140).as_ref().clone()));
                    let v142: Rc<str> = Rc::<str>::from((*v141.borrow()).join(&*v133));
                    v142
                }
                US8::US8_WordsRoot => {
                    v1
                }
            };
            let v145: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v144);
            if v145 {
                US9::US9_Replaced
            } else {
                US9::US9_ReadbackMismatch
            }
        } else {
            let v149: bool = std::fs::remove_file(&*v2).is_ok() || !std::path::Path::new(&*v2).exists();
            if v149 {
                US9::US9_RenameFailed
            } else {
                US9::US9_RenameFailedTemporaryKept
            }
        }
    } else {
        US9::US9_TemporaryWriteFailed
    };
    match v155 {
        US9::US9_Replaced => {
            true
        }
        _ => {
            false
        }
    }
}
fn method37(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
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
fn method39(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method39(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let mut v6: i32 = 0i32;
            let v7: i32 = method8(v2.clone(), v5, v6);
            let v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2.clone(), v7 as i64, v8 as i64);
            let mut v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method9(v9.clone(), v10);
            let mut v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            let mut v13: bool = &*v12 != "";
            if v13 {
                Rc::new(UH1::UH1_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn section_lines_41(mut v0: US10, mut v1: Rc<UH1>, mut v2: Rc<UH1>, mut v3: Rc<UH1>) -> US12 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v8, v9) => {
                let v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH1> = v9.clone();
                let v26: bool = match &v0 {
                    US10::US10_SpiprojProject => {
                        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v11: bool = v8.starts_with(&*v10);
                        let v14: bool = if v11 {
                            true
                        } else {
                            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v13: bool = v8.starts_with(&*v12);
                            v13
                        };
                        let v15: bool = v14 == false;
                        v15
                    }
                    US10::US10_TomlProject => {
                        let v16: i32 = (v8.clone().len() as i32);
                        let mut v17: i32 = 0i32;
                        let v18: i32 = method8(v8.clone(), v16, v17);
                        let v19: i32 = v16.wrapping_sub(1i32);
                        let v20: Rc<str> = string_slice(&v8.clone(), v18 as i64, v19 as i64);
                        let v21: i32 = (v20.clone().len() as i32);
                        let v22: i32 = method9(v20.clone(), v21);
                        let v23: Rc<str> = string_slice(&v20, 0i32 as i64, v22 as i64);
                        let mut v24: bool = &*v23 == "'''";
                        v24
                    }
                };
                if v26 {
                    let v27: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v28: Rc<UH1> = method4(v3, v27);
                    return US12::US12_SplitProject(v2, v28, v1);
                } else {
                    let v30: i32 = (v8.clone().len() as i32);
                    let v31: i32 = 0i32;
                    let v32: i32 = method8(v8.clone(), v30, v31);
                    let v33: i32 = v30.wrapping_sub(1i32);
                    let v34: Rc<str> = string_slice(&v8, v32 as i64, v33 as i64);
                    let v35: i32 = (v34.clone().len() as i32);
                    let v36: i32 = method9(v34.clone(), v35);
                    let v37: Rc<str> = string_slice(&v34, 0i32 as i64, v36 as i64);
                    let v38: Rc<UH1> = Rc::new(UH1::UH1_Cons(v37, v3));
                    (v1, v3) = (v9, v38);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH1> = method4(v3, v4);
                let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                return US12::US12_SplitProject(v2, v5, v6);
            }
        }
    }
}
fn split_lines_40(mut v0: US10, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> US12 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let mut v6: i32 = (v4.clone().len() as i32);
                let v7: i32 = 0i32;
                let v8: i32 = method8(v4.clone(), v6, v7);
                let mut v9: i32 = v6.wrapping_sub(1i32);
                let mut v10: Rc<str> = string_slice(&v4.clone(), v8 as i64, v9 as i64);
                let v11: i32 = (v10.clone().len() as i32);
                let mut v12: i32 = method9(v10.clone(), v11);
                let mut v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
                let mut v17: Rc<str> = match &v0 {
                    US10::US10_SpiprojProject => {
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                        v14
                    }
                    US10::US10_TomlProject => {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                        v15
                    }
                };
                let v18: bool = &*v13 == &*v17;
                if v18 {
                    let v19: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v20: Rc<UH1> = Rc::new(UH1::UH1_Cons(v4, v19));
                    let v21: Rc<UH1> = method4(v2, v20);
                    let v22: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    return section_lines_41(v0, v5, v21, v22);
                } else {
                    let mut v24: Rc<UH1> = Rc::new(UH1::UH1_Cons(v4, v2));
                    (v1, v2) = (v5, v24);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US12::US12_NoModules;
            }
        }
    }
}
fn closure4(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US13> {
    Rc::new(move |mut v2: Rc<UH1>| -> US13 {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US13::US13_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US13> = method42(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH1::UH1_Nil => {
                US13::US13_None
            }
        }
    })
}
fn method42(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US13> {
    closure4(v0, v1)
}
fn method43(mut v0: US10, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH1> = v4;
            let v5: Rc<UH1> = method43(v0.clone(), v4, v2);
            let mut v13: Rc<str> = match v0 {
                US10::US10_SpiprojProject => {
                    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                    v6.borrow_mut().push(v7);
                    v6.borrow_mut().push(v3);
                    let v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
                    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
                    v11
                }
                US10::US10_TomlProject => {
                    v3
                }
            };
            Rc::new(UH1::UH1_Cons(v13, v5))
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn method44(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method44(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn module_listed_38(mut v0: US10, mut v1: Rc<str>) -> US11 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("parents"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v1.replace(&*v4, &*v5));
    let v7: Rc<UH1> = split_list_3(v3.clone(), v6);
    let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH1> = method39(v7, v8);
    let mut v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v11: US12 = split_lines_40(v0.clone(), v9, v10);
    match v11 {
        US12::US12_NoModules => {
            US11::US11_NoModulesSection
        }
        US12::US12_SplitProject(v13, v14, v15) => {
            let mut v13: Rc<UH1> = v13;
            let v14: Rc<UH1> = v14;
            let v15: Rc<UH1> = v15;
            let v16: i32 = 0i32;
            let mut v17: Rc<dyn Fn(Rc<UH1>) -> US13> = method42(v2.clone(), v16);
            let v18: US13 = v17(v14.clone());
            let v19: bool = match v18 {
                US13::US13_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v20: bool = v19 != true;
            if v20 {
                US11::US11_ModuleListed
            } else {
                let v22: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v23: Rc<UH1> = method43(v0.clone(), v14, v22);
                let v30: Rc<str> = match v0 {
                    US10::US10_SpiprojProject => {
                        let mut v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                        v24.borrow_mut().push(v25);
                        v24.borrow_mut().push(v2);
                        let v26: Rc<Vec<Rc<str>>> = Rc::new(v24.borrow().clone());
                        let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v26).as_ref().clone()));
                        let v28: Rc<str> = Rc::<str>::from((*v27.borrow()).join(&*v5));
                        v28
                    }
                    US10::US10_TomlProject => {
                        v2
                    }
                };
                let v31: Rc<UH1> = method44(v23, v15);
                let v32: Rc<UH1> = Rc::new(UH1::UH1_Cons(v30, v31));
                let v33: Rc<UH1> = method44(v13, v32);
                let v34: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v35: i32 = 0i32;
                let v36: i32 = method25(v34.clone(), v33, v35);
                let v37: Rc<Vec<Rc<str>>> = Rc::new(v34.borrow().clone());
                let v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v37).as_ref().clone()));
                let v39: Rc<str> = Rc::<str>::from((*v38.borrow()).join(&*v3));
                let mut v40: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v40.borrow_mut().push(v39);
                v40.borrow_mut().push(v3);
                let v41: Rc<Vec<Rc<str>>> = Rc::new(v40.borrow().clone());
                let v42: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v41).as_ref().clone()));
                let v43: Rc<str> = Rc::<str>::from((*v42.borrow()).join(&*v5));
                US11::US11_ModuleAdded(v43)
            }
        }
    }
}
fn parents_changed_18(mut v0: Rc<str>, mut v1: US3) -> US4 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = string_concat(&[&*v4, "/state"]);
    let mut v6: Rc<str> = string_concat(&[&*v5, "/agile.spi"]);
    let v7: Rc<str> = text_of_19(v6);
    let v8: Rc<str> = string_concat(&[&*v5, "/parents.spi"]);
    let mut v9: Rc<str> = text_of_19(v8.clone());
    let v25: Rc<str> = match &v1 {
        US3::US3_ParentCleared(v20) => {
            let v20: Rc<str> = v20.clone();
            let v21: bool = task_known_26(v7.clone(), v20.clone());
            if v21 {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v22
            } else {
                v20
            }
        }
        US3::US3_ParentSet(v10, v11) => {
            let mut v10: Rc<str> = v10.clone();
            let v11: Rc<str> = v11.clone();
            let mut v12: bool = task_known_26(v7.clone(), v10.clone());
            let v14: Rc<str> = if v12 {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v13
            } else {
                v10.clone()
            };
            let v15: bool = &*v14 != "";
            if v15 {
                v10
            } else {
                let v16: bool = task_known_26(v7.clone(), v11.clone());
                if v16 {
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v17
                } else {
                    v11
                }
            }
        }
    };
    let v26: Rc<UH0> = links_of_text_1(v9.clone());
    let v27: Rc<UH0> = links_changed_27(v26, v1);
    let v28: bool = &*v7 == "";
    if v28 {
        let v29: Rc<str> = string_concat(&["no backlog state at ", &*v5]);
        US4::US4_ParentsRefused(v29)
    } else {
        let v31: bool = &*v25 != "";
        if v31 {
            let v33: Rc<str> = string_concat(&["no task ", &*v25, " in state/agile.spi"]);
            US4::US4_ParentsRefused(v33)
        } else {
            let v35: Rc<str> = text_of_19(v8.clone());
            let v36: bool = &*v35 != &*v9;
            if v36 {
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/parents.spi changed since it was read; run again"); } LIT.with(|lit| lit.clone()) };
                US4::US4_ParentsRefused(v37)
            } else {
                let v39: Rc<str> = parents_text_30(v27.clone());
                let mut v40: bool = replaced_34(v8, v39);
                let v41: bool = v40 == false;
                if v41 {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/parents.spi could not be written"); } LIT.with(|lit| lit.clone()) };
                    US4::US4_ParentsRefused(v42)
                } else {
                    let mut v44: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    v44.borrow_mut().push(v5.clone());
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.toml"); } LIT.with(|lit| lit.clone()) };
                    v44.borrow_mut().push(v45.clone());
                    let v46: Rc<Vec<Rc<str>>> = Rc::new(v44.borrow().clone());
                    let v47: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v46).as_ref().clone()));
                    let v48: Rc<str> = Rc::<str>::from((*v47.borrow()).join(&*v3));
                    let v49: bool = std::path::Path::new(&*v48).is_file();
                    let v52: US10 = if v49 {
                        US10::US10_TomlProject
                    } else {
                        US10::US10_SpiprojProject
                    };
                    let v55: Rc<str> = match v52 {
                        US10::US10_SpiprojProject => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US10::US10_TomlProject => {
                            v45.clone()
                        }
                    };
                    let mut v56: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    v56.borrow_mut().push(v5.clone());
                    v56.borrow_mut().push(v55);
                    let v57: Rc<Vec<Rc<str>>> = Rc::new(v56.borrow().clone());
                    let v58: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v57).as_ref().clone()));
                    let v59: Rc<str> = Rc::<str>::from((*v58.borrow()).join(&*v3));
                    let v60: i32 = 0i32;
                    let v61: i32 = method37(v27, v60);
                    let mut v62: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    v62.borrow_mut().push(v5);
                    v62.borrow_mut().push(v45);
                    let v63: Rc<Vec<Rc<str>>> = Rc::new(v62.borrow().clone());
                    let v64: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v63).as_ref().clone()));
                    let v65: Rc<str> = Rc::<str>::from((*v64.borrow()).join(&*v3));
                    let v66: bool = std::path::Path::new(&*v65).is_file();
                    let v69: US10 = if v66 {
                        US10::US10_TomlProject
                    } else {
                        US10::US10_SpiprojProject
                    };
                    let v70: Rc<str> = text_of_19(v59.clone());
                    let v71: US11 = module_listed_38(v69, v70);
                    match v71 {
                        US11::US11_ModuleAdded(v75) => {
                            let v75: Rc<str> = v75;
                            let v76: bool = replaced_34(v59, v75);
                            if v76 {
                                US4::US4_ParentsWritten(v61)
                            } else {
                                let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the state project was not updated"); } LIT.with(|lit| lit.clone()) };
                                US4::US4_ParentsRefused(v78)
                            }
                        }
                        US11::US11_ModuleListed => {
                            US4::US4_ParentsWritten(v61)
                        }
                        US11::US11_NoModulesSection => {
                            let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the state project has no modules section to list parents in"); } LIT.with(|lit| lit.clone()) };
                            US4::US4_ParentsRefused(v73)
                        }
                    }
                }
            }
        }
    }
}
fn method46(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_45(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method46(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn parent_edit_run_14() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH1> = arguments_from_15(v0);
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH1::UH1_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH1> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH1::UH1_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let v7: Rc<UH1> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH1::UH1_Cons(v8, v9) => {
                                    let v8: Rc<str> = v8;
                                    let mut v9: Rc<UH1> = v9;
                                    match Rc::unwrap_or_clone(v9) {
                                        UH1::UH1_Cons(v10, v11) => {
                                            let mut v10: Rc<str> = v10;
                                            let v11: Rc<UH1> = v11;
                                            match Rc::unwrap_or_clone(v11) {
                                                UH1::UH1_Cons(v12, v13) => {
                                                    let mut v12: Rc<str> = v12;
                                                    let mut v13: Rc<UH1> = v13;
                                                    match Rc::unwrap_or_clone(v13) {
                                                        UH1::UH1_Nil => {
                                                            let v14: bool = &*v10 == "--parent";
                                                            if v14 {
                                                                let v15: bool = &*v12 == "--clear";
                                                                let v18: US3 = if v15 {
                                                                    US3::US3_ParentCleared(v8.clone())
                                                                } else {
                                                                    US3::US3_ParentSet(v8.clone(), v12.clone())
                                                                };
                                                                let v19: US4 = parents_changed_18(v6, v18);
                                                                match v19 {
                                                                    US4::US4_ParentsRefused(v29) => {
                                                                        let v29: Rc<str> = v29;
                                                                        let v30: Rc<str> = string_concat(&["rig error: agile edit --parent: ", &*v29]);
                                                                        eprintln!("{}", v30);
                                                                        2i32
                                                                    }
                                                                    US4::US4_ParentsWritten(v20) => {
                                                                        let v20: i32 = v20;
                                                                        let v22: Rc<str> = if v15 {
                                                                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
                                                                            v21
                                                                        } else {
                                                                            v12
                                                                        };
                                                                        let v25: Rc<str> = format_real_45(v20);
                                                                        let v28: Rc<str> = string_concat(&["rig agile edit ok child=", &*v8, " parent=", &*v22, " links=", &*v25]);
                                                                        println!("{}", v28);
                                                                        0i32
                                                                    }
                                                                }
                                                            } else {
                                                                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                                                                eprintln!("{}", v33);
                                                                2i32
                                                            }
                                                        }
                                                        _ => {
                                                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                                                            eprintln!("{}", v35);
                                                            2i32
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                                                    eprintln!("{}", v37);
                                                    2i32
                                                }
                                            }
                                        }
                                        _ => {
                                            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                                            eprintln!("{}", v39);
                                            2i32
                                        }
                                    }
                                }
                                _ => {
                                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                                    eprintln!("{}", v41);
                                    2i32
                                }
                            }
                        }
                        _ => {
                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v43);
                            2i32
                        }
                    }
                }
                _ => {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v45);
                    2i32
                }
            }
        }
        _ => {
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v47);
            2i32
        }
    }
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        parent_edit_run_14()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_parent_links(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_agile_parent_edit_run() -> i32 {
    closure2()()
}
pub type SpiralShape_accbf6de6842 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
