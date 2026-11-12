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
    UH0_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, Rc<str>, Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_AsksWritten(i32),
    US1_AsksRefused(Rc<str>),
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US3 {
    US3_AskSet(Rc<str>, Rc<str>, Rc<str>),
    US3_AskCleared(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, US3, Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US4 {
    US4_StateFile(Rc<str>),
    US4_HistoryFile(Rc<str>),
    US4_ArchiveFile(Rc<str>),
    US4_OtherFile,
}
#[derive(Clone)]
enum US6 {
    US6_HeadInl,
    US6_HeadUnion,
    US6_HeadOpen,
    US6_HeadType,
}
#[derive(Clone)]
enum US5 {
    US5_KeywordHead(US6),
    US5_SymbolHead(US6),
    US5_OtherLine,
}
#[derive(Clone)]
enum US7 {
    US7_WordsRoot,
    US7_SymbolsRoot,
}
#[derive(Clone)]
enum US8 {
    US8_Replaced,
    US8_TemporaryWriteFailed,
    US8_RenameFailed,
    US8_RenameFailedTemporaryKept,
    US8_ReadbackMismatch,
    US8_ChangedSinceRead,
}
#[derive(Clone)]
enum US9 {
    US9_SpiprojProject,
    US9_TomlProject,
}
#[derive(Clone)]
enum US10 {
    US10_ModuleListed,
    US10_ModuleAdded(Rc<str>),
    US10_NoModulesSection,
}
#[derive(Clone)]
enum US11 {
    US11_SplitProject(Rc<UH1>, Rc<UH1>, Rc<UH1>),
    US11_NoModules,
}
#[derive(Clone)]
enum US12 {
    US12_Some(i32),
    US12_None,
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
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
            let mut v11: i32 = v9.borrow().l0;
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
fn loop_9(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(2i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn quoted_triple_8(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH1> = split_list_3(v1, v0);
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH1> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH1::UH1_Cons(v5, v6) => {
                    let mut v5: Rc<str> = v5;
                    let mut v6: Rc<UH1> = v6;
                    match Rc::unwrap_or_clone(v6) {
                        UH1::UH1_Cons(v7, v8) => {
                            let v7: Rc<str> = v7;
                            let v8: Rc<UH1> = v8;
                            match Rc::unwrap_or_clone(v8) {
                                UH1::UH1_Nil => {
                                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
                                    let mut v10: bool = v7.ends_with(&*v9);
                                    if v10 {
                                        let mut v11: Rc<str> = loop_9(v7);
                                        let mut v12: bool = &*v3 == "";
                                        let v14: bool = if v12 {
                                            true
                                        } else {
                                            let mut v13: bool = &*v5 == "";
                                            v13
                                        };
                                        if v14 {
                                            US0::US0_None
                                        } else {
                                            US0::US0_Some(v3, v5, v11)
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
fn ask_of_line_7(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Asked (\""); } LIT.with(|lit| lit.clone()) };
    let v3: Option<usize> = v1.find(&*v2);
    let v4: Option<usize> = method10(v3);
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let v8: i32 = v6.unwrap_or(-1i32);
    let mut v9: bool = v8 < 0i32;
    if v9 {
        US0::US0_None
    } else {
        let mut v11: i32 = v8.wrapping_add(11i32);
        let mut v12: i32 = (v1.clone().len() as i32);
        let mut v13: i32 = v12.wrapping_sub(1i32);
        let v14: Rc<str> = string_slice(&v1, v11 as i64, v13 as i64);
        quoted_triple_8(v14)
    }
}
fn method6(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: US0 = ask_of_line_7(v2);
                let mut v10: Rc<UH0> = match v4 {
                    US0::US0_None => {
                        v1
                    }
                    US0::US0_Some(v5, v6, v7) => {
                        let mut v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        let v7: Rc<str> = v7;
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
fn asks_of_text_1(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH1> = split_list_3(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method6(v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method2(v7, v8)
}
fn method11(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
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
fn asks_of_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    let mut v1: Rc<UH0> = asks_of_text_1(v0);
    let mut v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v3: i32 = 0i32;
    let v4: i32 = method11(v2.clone(), v1, v3);
    let mut v5: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>)>> = Rc::new(v2.borrow().clone());
    let mut v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    v6
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
        asks_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_14(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn reported_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1) -> i32 {
    match v4 {
        US1::US1_AsksRefused(v15) => {
            let v15: Rc<str> = v15;
            let v18: Rc<str> = string_concat(&["rig error: agile ", &*v0, ": ", &*v15]);
            eprintln!("{}", v18);
            2i32
        }
        US1::US1_AsksWritten(v5) => {
            let mut v5: i32 = v5;
            let v8: Rc<str> = format_real_14(v5);
            let v14: Rc<str> = string_concat(&["rig agile ", &*v0, " ok id=", &*v1, " asked=", &*v2, " asks=", &*v8, &*v3]);
            println!("{}", v14);
            0i32
        }
    }
}
fn marked_c3(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: bool = v0.contains(&*v1);
        v2
    })
}
fn method20(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    marked_c3(v0)
}
fn method19(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: Rc<UH1>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                let mut v5: bool = v1(v3);
                if v5 {
                    return true;
                } else {
                    let mut v6: Rc<dyn Fn(Rc<str>) -> bool> = method20(v0.clone());
                    (v1, v2) = (v6, v4);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return false;
            }
        }
    }
}
fn method22(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method22(v3, v1);
            let mut v5: bool = &*v2 != "";
            if v5 {
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
fn loop_24(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
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
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_25(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
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
fn loop_26(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
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
fn loop_27(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
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
fn method28(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method28(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US5 = if v6 {
                let v7: US6 = US6::US6_HeadType;
                US5::US5_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US6 = US6::US6_HeadType;
                    US5::US5_SymbolHead(v11)
                } else {
                    US5::US5_OtherLine
                }
            };
            let v27: US5 = match &v15 {
                US5::US5_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US6 = US6::US6_HeadInl;
                        US5::US5_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US6 = US6::US6_HeadInl;
                            US5::US5_SymbolHead(v22)
                        } else {
                            US5::US5_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US5 = match &v27 {
                US5::US5_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US6 = US6::US6_HeadUnion;
                        US5::US5_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US6 = US6::US6_HeadUnion;
                            US5::US5_SymbolHead(v34)
                        } else {
                            US5::US5_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US5 = match &v39 {
                US5::US5_OtherLine => {
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US6 = US6::US6_HeadOpen;
                        US5::US5_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US6 = US6::US6_HeadOpen;
                            US5::US5_SymbolHead(v46)
                        } else {
                            US5::US5_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US5::US5_SymbolHead(v52) => {
                    let v52: US6 = v52;
                    let v60: Rc<str> = match &v52 {
                        US6::US6_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US6::US6_HeadOpen => {
                            let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US6::US6_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US6::US6_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US6::US6_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US6::US6_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US6::US6_HeadType => {
                            v5
                        }
                        US6::US6_HeadUnion => {
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
fn method29(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
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
fn text_of_23(mut v0: Rc<str>) -> Rc<str> {
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
            let mut v9: Rc<str> = loop_24(v8);
            let mut v10: Option<usize> = v9.rfind(&*v7);
            let mut v11: Option<usize> = method10(v10);
            let mut v12: Rc<dyn Fn((usize)) -> i32> = closure4();
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
            let v41: US4 = if v20 {
                US4::US4_OtherFile
            } else {
                let v22: usize = ((0i32) as usize);
                let v23: usize = ((v15) as usize);
                let mut v24: Rc<str> = Rc::<str>::from(&(v9)[v22..v23]);
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v26: bool = v24.ends_with(&*v25);
                if v26 {
                    let v27: Rc<str> = loop_25(v24);
                    US4::US4_StateFile(v27)
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v30: bool = v24.ends_with(&*v29);
                    if v30 {
                        let v31: Rc<str> = loop_26(v24);
                        US4::US4_HistoryFile(v31)
                    } else {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v34: bool = v24.ends_with(&*v33);
                        if v34 {
                            let v35: Rc<str> = loop_27(v24);
                            US4::US4_ArchiveFile(v35)
                        } else {
                            US4::US4_OtherFile
                        }
                    }
                }
            };
            match v41 {
                US4::US4_OtherFile => {
                    v3
                }
                _ => {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v43: Rc<UH1> = split_list_3(v42.clone(), v3);
                    let v44: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v45: Rc<UH1> = method28(v43, v44);
                    let v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v47: i32 = 0i32;
                    let v48: i32 = method29(v46.clone(), v45, v47);
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
fn method30(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method30(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Marker (\"DoneDefinition\", \""); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.contains(&*v5);
            if v6 {
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
fn method31(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method31(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<UH1> = split_list_3(v5, v2);
            let v23: Rc<str> = match Rc::unwrap_or_clone(v6) {
                UH1::UH1_Cons(v7, v8) => {
                    let v7: Rc<str> = v7;
                    let v8: Rc<UH1> = v8;
                    match Rc::unwrap_or_clone(v8) {
                        UH1::UH1_Cons(v9, v10) => {
                            let mut v9: Rc<str> = v9;
                            let mut v10: Rc<UH1> = v10;
                            match Rc::unwrap_or_clone(v10) {
                                UH1::UH1_Cons(v11, v12) => {
                                    let mut v11: Rc<str> = v11;
                                    let mut v12: Rc<UH1> = v12;
                                    match Rc::unwrap_or_clone(v12) {
                                        UH1::UH1_Cons(v13, v14) => {
                                            let mut v13: Rc<str> = v13;
                                            let v14: Rc<UH1> = v14;
                                            let v15: Rc<str> = Rc::<str>::from(v13.to_lowercase());
                                            v15
                                        }
                                        _ => {
                                            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            v16
                                        }
                                    }
                                }
                                _ => {
                                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v18
                                }
                            }
                        }
                        _ => {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v20
                        }
                    }
                }
                _ => {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v22
                }
            };
            Rc::new(UH1::UH1_Cons(v23, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn declared_done_21(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<str> = string_concat(&[&*v4, "/state/markers.spi"]);
    let mut v6: Rc<str> = text_of_23(v5);
    let v7: Rc<UH1> = split_list_3(v1, v6);
    let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH1> = method30(v7, v8);
    let mut v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v11: Rc<UH1> = method31(v9, v10);
    let mut v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method22(v11, v12)
}
fn distilled_18(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = Rc::<str>::from(v1.to_lowercase());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done when"); } LIT.with(|lit| lit.clone()) };
    let v4: bool = v2.contains(&*v3);
    if v4 {
        true
    } else {
        let mut v5: Rc<UH1> = declared_done_21(v0);
        let mut v6: Rc<dyn Fn(Rc<str>) -> bool> = method20(v2.clone());
        method19(v2, v6, v5)
    }
}
fn ask_flag_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let v3: bool = &*v1 == "--ask";
    let mut v5: bool = if v3 {
        true
    } else {
        let v4: bool = &*v1 == "--quote";
        v4
    };
    if v5 {
        distilled_18(v0, v2)
    } else {
        false
    }
}
fn digits_from_33(mut v0: Rc<str>, mut v1: i32) -> bool {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 >= v2;
        if v3 {
            return true;
        } else {
            let v4: usize = ((v1) as usize);
            let mut v5: i32 = v1.wrapping_add(1i32);
            let mut v6: usize = ((v5) as usize);
            let v7: Rc<str> = Rc::<str>::from(&(v0)[v4..v6]);
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Option<usize> = v8.find(&*v7);
            let mut v10: Option<usize> = method10(v9);
            let mut v11: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v12: Option<i32> = v10.map(|x| v11(x));
            let v14: i32 = v12.unwrap_or(-1i32);
            let v15: bool = v14 >= 0i32;
            if v15 {
                v1 = v5;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn day_shaped_32(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: i32 = (v0.clone().len() as i32);
    let mut v5: bool = v4 == 10i32;
    let mut v10: bool = if v5 {
        let mut v6: usize = ((4i32) as usize);
        let v7: usize = ((5i32) as usize);
        let v8: Rc<str> = Rc::<str>::from(&(v0)[v6..v7]);
        let mut v9: bool = &*v8 == "-";
        v9
    } else {
        false
    };
    let v15: bool = if v10 {
        let mut v11: usize = ((7i32) as usize);
        let mut v12: usize = ((8i32) as usize);
        let mut v13: Rc<str> = Rc::<str>::from(&(v0)[v11..v12]);
        let v14: bool = &*v13 == "-";
        v14
    } else {
        false
    };
    let v18: bool = if v15 {
        let v16: i32 = (v3.clone().len() as i32);
        let mut v17: bool = v16 == 8i32;
        v17
    } else {
        false
    };
    if v18 {
        let v19: i32 = 0i32;
        digits_from_33(v3, v19)
    } else {
        false
    }
}
fn ask_request_16() -> US2 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v4: i32 = 4i32;
    let mut v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v6: i32 = 6i32;
    let v7: Rc<str> = usize::try_from(v6).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v8: i32 = 3i32;
    let mut v9: Rc<str> = usize::try_from(v8).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v10: bool = &*v9 != "--asked";
    if v10 {
        US2::US2_None
    } else {
        let mut v12: bool = &*v5 == "--clear";
        let v16: bool = if v12 {
            let mut v13: i32 = 5i32;
            let v14: Rc<str> = usize::try_from(v13).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v15: bool = &*v14 == "";
            v15
        } else {
            false
        };
        if v16 {
            let mut v17: US3 = US3::US3_AskCleared(v3);
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
            US2::US2_Some(v1, v17, v18)
        } else {
            let v20: i32 = 5i32;
            let v21: Rc<str> = usize::try_from(v20).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v22: bool = ask_flag_17(v1.clone(), v21, v7.clone());
            let v26: bool = if v22 {
                let v23: i32 = 7i32;
                let mut v24: Rc<str> = usize::try_from(v23).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v25: bool = &*v24 == "";
                v25
            } else {
                false
            };
            let v28: bool = if v26 {
                day_shaped_32(v5.clone())
            } else {
                false
            };
            let v31: bool = if v28 {
                let v29: Rc<str> = Rc::<str>::from((v7).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v30: bool = &*v29 != "";
                v30
            } else {
                false
            };
            if v31 {
                let v32: US3 = US3::US3_AskSet(v3, v5.clone(), v7);
                US2::US2_Some(v1, v32, v5)
            } else {
                US2::US2_None
            }
        }
    }
}
fn method36(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn occurrences_of_35(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<UH1> = split_list_3(v0, v1);
    let v3: i32 = 0i32;
    let v4: i32 = method36(v2, v3);
    let mut v5: i32 = v4.wrapping_sub(1i32);
    v5
}
fn rewrites_of_34(mut v0: US3) -> Rc<str> {
    match v0 {
        US3::US3_AskCleared(v23) => {
            let v23: Rc<str> = v23;
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24
        }
        US3::US3_AskSet(v1, v2, v3) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v5: i32 = occurrences_of_35(v4, v3.clone());
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v7: i32 = occurrences_of_35(v6, v3.clone());
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v9: i32 = occurrences_of_35(v8, v3);
            let mut v10: i32 = v5.wrapping_add(v7);
            let mut v11: i32 = v10.wrapping_add(v9);
            let mut v12: bool = v11 == 0i32;
            if v12 {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v13
            } else {
                let v14: Rc<str> = format_real_14(v5);
                let mut v17: Rc<str> = format_real_14(v7);
                let v20: Rc<str> = format_real_14(v9);
                let v21: Rc<str> = string_concat(&[" rewrote=quotes:", &*v14, ",backslashes:", &*v17, ",newlines:", &*v20]);
                v21
            }
        }
    }
}
fn method39(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4, v5) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let mut v5: Rc<UH0> = v5;
            let mut v6: Rc<UH0> = method39(v5, v1);
            Rc::new(UH0::UH0_Cons(v2, v3, v4, v6))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method40(mut v0: US3, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4, v5, v6) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<UH0> = v6;
            let v7: Rc<UH0> = method40(v0.clone(), v6, v2);
            let mut v13: Rc<str> = match v0 {
                US3::US3_AskCleared(v11) => {
                    let mut v11: Rc<str> = v11;
                    v11
                }
                US3::US3_AskSet(v8, v9, v10) => {
                    let v8: Rc<str> = v8;
                    let mut v9: Rc<str> = v9;
                    let mut v10: Rc<str> = v10;
                    v8
                }
            };
            let v14: bool = &*v3 != &*v13;
            if v14 {
                Rc::new(UH0::UH0_Cons(v3, v4, v5, v7))
            } else {
                v7
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn asks_changed_38(mut v0: Rc<UH0>, mut v1: US3) -> Rc<UH0> {
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH0> = method40(v1.clone(), v0, v2);
    match v1 {
        US3::US3_AskCleared(v10) => {
            let mut v10: Rc<str> = v10;
            v3
        }
        US3::US3_AskSet(v4, v5, v6) => {
            let v4: Rc<str> = v4;
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<str> = v6;
            let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5, v6, v7));
            method39(v3, v8)
        }
    }
}
fn task_known_41(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v3: Rc<str> = string_concat(&["Task (\"", &*v1, "\""]);
    let v4: bool = v0.contains(&*v3);
    v4
}
fn slug_of_45(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v3));
    v6
}
fn quote_text_46(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v6.replace(&*v7, &*v8));
    v9
}
fn row_of_ask_44(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = slug_of_45(v0.clone());
    let mut v10: Rc<str> = quote_text_46(v2);
    let mut v12: Rc<str> = string_concat(&["inl asked_of_", &*v3, " () = Asked (\"", &*v0, "\", \"", &*v1, "\", \"", &*v10, "\")"]);
    v12
}
fn method43(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4, v5) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let mut v5: Rc<UH0> = v5;
            let mut v6: Rc<UH1> = method43(v5, v1);
            let v7: Rc<str> = row_of_ask_44(v2, v3, v4);
            let v8: Rc<str> = string_concat(&[&*v7, "\n"]);
            Rc::new(UH1::UH1_Cons(v8, v6))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn asks_text_42(mut v0: Rc<UH0>) -> Rc<str> {
    let mut v1: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH1> = method43(v0, v1);
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v4: i32 = 0i32;
    let mut v5: i32 = method29(v3.clone(), v2, v4);
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from((*v7.borrow()).join(&*v8));
    let mut v10: Rc<str> = string_concat(&["///- --no-respell\nunion task_ask =\n    | Asked :: string * string * string -> task_ask\n", &*v9]);
    v10
}
fn method48(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method48(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US5 = if v6 {
                let v7: US6 = US6::US6_HeadType;
                US5::US5_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US6 = US6::US6_HeadType;
                    US5::US5_SymbolHead(v11)
                } else {
                    US5::US5_OtherLine
                }
            };
            let v27: US5 = match &v15 {
                US5::US5_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US6 = US6::US6_HeadInl;
                        US5::US5_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US6 = US6::US6_HeadInl;
                            US5::US5_SymbolHead(v22)
                        } else {
                            US5::US5_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US5 = match &v27 {
                US5::US5_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US6 = US6::US6_HeadUnion;
                        US5::US5_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US6 = US6::US6_HeadUnion;
                            US5::US5_SymbolHead(v34)
                        } else {
                            US5::US5_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US5 = match &v39 {
                US5::US5_OtherLine => {
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US6 = US6::US6_HeadOpen;
                        US5::US5_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US6 = US6::US6_HeadOpen;
                            US5::US5_SymbolHead(v46)
                        } else {
                            US5::US5_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US5::US5_KeywordHead(v52) => {
                    let v52: US6 = v52;
                    let mut v59: Rc<str> = match &v52 {
                        US6::US6_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US6::US6_HeadOpen => {
                            let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US6::US6_HeadType => {
                            v5
                        }
                        US6::US6_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v60: i32 = (v59.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US6::US6_HeadInl => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v61
                        }
                        US6::US6_HeadOpen => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                        US6::US6_HeadType => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US6::US6_HeadUnion => {
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
fn renamed_within_49(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn replaced_47(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = string_concat(&[&*v0, ".pending"]);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let mut v6: Rc<str> = loop_24(v5);
    let v7: Option<usize> = v6.rfind(&*v4);
    let v8: Option<usize> = method10(v7);
    let mut v9: Rc<dyn Fn((usize)) -> i32> = closure4();
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
    let v38: US4 = if v17 {
        US4::US4_OtherFile
    } else {
        let v19: usize = ((0i32) as usize);
        let v20: usize = ((v12) as usize);
        let v21: Rc<str> = Rc::<str>::from(&(v6)[v19..v20]);
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v23: bool = v21.ends_with(&*v22);
        if v23 {
            let mut v24: Rc<str> = loop_25(v21);
            US4::US4_StateFile(v24)
        } else {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
            let v27: bool = v21.ends_with(&*v26);
            if v27 {
                let v28: Rc<str> = loop_26(v21);
                US4::US4_HistoryFile(v28)
            } else {
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                let v31: bool = v21.ends_with(&*v30);
                if v31 {
                    let v32: Rc<str> = loop_27(v21);
                    US4::US4_ArchiveFile(v32)
                } else {
                    US4::US4_OtherFile
                }
            }
        }
    };
    let v61: US7 = match v38 {
        US4::US4_ArchiveFile(v51) => {
            let v51: Rc<str> = v51;
            let v52: Rc<str> = string_concat(&[&*v51, "/state/spiral.toml"]);
            let v53: bool = std::path::Path::new(&*v52).is_file();
            if v53 {
                US7::US7_SymbolsRoot
            } else {
                US7::US7_WordsRoot
            }
        }
        US4::US4_HistoryFile(v45) => {
            let v45: Rc<str> = v45;
            let v46: Rc<str> = string_concat(&[&*v45, "/state/spiral.toml"]);
            let v47: bool = std::path::Path::new(&*v46).is_file();
            if v47 {
                US7::US7_SymbolsRoot
            } else {
                US7::US7_WordsRoot
            }
        }
        US4::US4_OtherFile => {
            US7::US7_WordsRoot
        }
        US4::US4_StateFile(v39) => {
            let v39: Rc<str> = v39;
            let mut v40: Rc<str> = string_concat(&[&*v39, "/state/spiral.toml"]);
            let v41: bool = std::path::Path::new(&*v40).is_file();
            if v41 {
                US7::US7_SymbolsRoot
            } else {
                US7::US7_WordsRoot
            }
        }
    };
    let v73: Rc<str> = match v61 {
        US7::US7_SymbolsRoot => {
            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v63: Rc<UH1> = split_list_3(v62.clone(), v1.clone());
            let v64: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v65: Rc<UH1> = method48(v63, v64);
            let v66: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v67: i32 = 0i32;
            let v68: i32 = method29(v66.clone(), v65, v67);
            let v69: Rc<Vec<Rc<str>>> = Rc::new(v66.borrow().clone());
            let v70: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v69).as_ref().clone()));
            let v71: Rc<str> = Rc::<str>::from((*v70.borrow()).join(&*v62));
            v71
        }
        US7::US7_WordsRoot => {
            v1.clone()
        }
    };
    let v74: bool = std::fs::write(&*v2, &*v73).is_ok();
    let v155: US8 = if v74 {
        let v75: i32 = 40i32;
        let v76: bool = renamed_within_49(v0.clone(), v2.clone(), v75);
        if v76 {
            let v77: Rc<str> = Rc::<str>::from(v0.replace(&*v3, &*v4));
            let mut v78: Rc<str> = loop_24(v77);
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
            let v109: US4 = if v88 {
                US4::US4_OtherFile
            } else {
                let v90: usize = ((0i32) as usize);
                let v91: usize = ((v83) as usize);
                let v92: Rc<str> = Rc::<str>::from(&(v78)[v90..v91]);
                let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v94: bool = v92.ends_with(&*v93);
                if v94 {
                    let v95: Rc<str> = loop_25(v92);
                    US4::US4_StateFile(v95)
                } else {
                    let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v98: bool = v92.ends_with(&*v97);
                    if v98 {
                        let v99: Rc<str> = loop_26(v92);
                        US4::US4_HistoryFile(v99)
                    } else {
                        let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v102: bool = v92.ends_with(&*v101);
                        if v102 {
                            let v103: Rc<str> = loop_27(v92);
                            US4::US4_ArchiveFile(v103)
                        } else {
                            US4::US4_OtherFile
                        }
                    }
                }
            };
            let v132: US7 = match v109 {
                US4::US4_ArchiveFile(v122) => {
                    let v122: Rc<str> = v122;
                    let v123: Rc<str> = string_concat(&[&*v122, "/state/spiral.toml"]);
                    let v124: bool = std::path::Path::new(&*v123).is_file();
                    if v124 {
                        US7::US7_SymbolsRoot
                    } else {
                        US7::US7_WordsRoot
                    }
                }
                US4::US4_HistoryFile(v116) => {
                    let v116: Rc<str> = v116;
                    let v117: Rc<str> = string_concat(&[&*v116, "/state/spiral.toml"]);
                    let v118: bool = std::path::Path::new(&*v117).is_file();
                    if v118 {
                        US7::US7_SymbolsRoot
                    } else {
                        US7::US7_WordsRoot
                    }
                }
                US4::US4_OtherFile => {
                    US7::US7_WordsRoot
                }
                US4::US4_StateFile(v110) => {
                    let v110: Rc<str> = v110;
                    let v111: Rc<str> = string_concat(&[&*v110, "/state/spiral.toml"]);
                    let v112: bool = std::path::Path::new(&*v111).is_file();
                    if v112 {
                        US7::US7_SymbolsRoot
                    } else {
                        US7::US7_WordsRoot
                    }
                }
            };
            let v144: Rc<str> = match v132 {
                US7::US7_SymbolsRoot => {
                    let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v134: Rc<UH1> = split_list_3(v133.clone(), v1);
                    let v135: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v136: Rc<UH1> = method48(v134, v135);
                    let v137: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v138: i32 = 0i32;
                    let v139: i32 = method29(v137.clone(), v136, v138);
                    let v140: Rc<Vec<Rc<str>>> = Rc::new(v137.borrow().clone());
                    let v141: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v140).as_ref().clone()));
                    let v142: Rc<str> = Rc::<str>::from((*v141.borrow()).join(&*v133));
                    v142
                }
                US7::US7_WordsRoot => {
                    v1
                }
            };
            let v145: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v144);
            if v145 {
                US8::US8_Replaced
            } else {
                US8::US8_ReadbackMismatch
            }
        } else {
            let v149: bool = std::fs::remove_file(&*v2).is_ok() || !std::path::Path::new(&*v2).exists();
            if v149 {
                US8::US8_RenameFailed
            } else {
                US8::US8_RenameFailedTemporaryKept
            }
        }
    } else {
        US8::US8_TemporaryWriteFailed
    };
    match v155 {
        US8::US8_Replaced => {
            true
        }
        _ => {
            false
        }
    }
}
fn method50(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method52(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method52(v3, v1);
            let mut v5: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let mut v6: bool = &*v5 != "";
            if v6 {
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
fn section_lines_54(mut v0: US9, mut v1: Rc<UH1>, mut v2: Rc<UH1>, mut v3: Rc<UH1>) -> US11 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v8, v9) => {
                let v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH1> = v9.clone();
                let v19: bool = match &v0 {
                    US9::US9_SpiprojProject => {
                        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let mut v11: bool = v8.starts_with(&*v10);
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
                    US9::US9_TomlProject => {
                        let v16: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                        let mut v17: bool = &*v16 == "'''";
                        v17
                    }
                };
                if v19 {
                    let v20: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v21: Rc<UH1> = method4(v3, v20);
                    return US11::US11_SplitProject(v2, v21, v1);
                } else {
                    let v23: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                    let mut v24: Rc<UH1> = Rc::new(UH1::UH1_Cons(v23, v3));
                    (v1, v3) = (v9, v24);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let mut v5: Rc<UH1> = method4(v3, v4);
                let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                return US11::US11_SplitProject(v2, v5, v6);
            }
        }
    }
}
fn split_lines_53(mut v0: US9, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> US11 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH1> = v5;
                let mut v6: Rc<str> = Rc::<str>::from((v4).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let mut v10: Rc<str> = match &v0 {
                    US9::US9_SpiprojProject => {
                        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                        v7
                    }
                    US9::US9_TomlProject => {
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                        v8
                    }
                };
                let mut v11: bool = &*v6 == &*v10;
                if v11 {
                    let mut v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let mut v13: Rc<UH1> = Rc::new(UH1::UH1_Cons(v4, v12));
                    let v14: Rc<UH1> = method4(v2, v13);
                    let v15: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    return section_lines_54(v0, v5, v14, v15);
                } else {
                    let mut v17: Rc<UH1> = Rc::new(UH1::UH1_Cons(v4, v2));
                    (v1, v2) = (v5, v17);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US11::US11_NoModules;
            }
        }
    }
}
fn closure5(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US12> {
    Rc::new(move |mut v2: Rc<UH1>| -> US12 {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH1> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US12::US12_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US12> = method55(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH1::UH1_Nil => {
                US12::US12_None
            }
        }
    })
}
fn method55(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US12> {
    closure5(v0, v1)
}
fn method56(mut v0: US9, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH1> = v4;
            let mut v5: Rc<UH1> = method56(v0.clone(), v4, v2);
            let mut v13: Rc<str> = match v0 {
                US9::US9_SpiprojProject => {
                    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                    v6.borrow_mut().push(v7);
                    v6.borrow_mut().push(v3);
                    let v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
                    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
                    v11
                }
                US9::US9_TomlProject => {
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
fn method57(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method57(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn module_listed_51(mut v0: US9, mut v1: Rc<str>) -> US10 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("asks"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v1.replace(&*v4, &*v5));
    let v7: Rc<UH1> = split_list_3(v3.clone(), v6);
    let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH1> = method52(v7, v8);
    let mut v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v11: US11 = split_lines_53(v0.clone(), v9, v10);
    match v11 {
        US11::US11_NoModules => {
            US10::US10_NoModulesSection
        }
        US11::US11_SplitProject(v13, v14, v15) => {
            let mut v13: Rc<UH1> = v13;
            let v14: Rc<UH1> = v14;
            let v15: Rc<UH1> = v15;
            let v16: i32 = 0i32;
            let mut v17: Rc<dyn Fn(Rc<UH1>) -> US12> = method55(v2.clone(), v16);
            let v18: US12 = v17(v14.clone());
            let v19: bool = match v18 {
                US12::US12_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v20: bool = v19 != true;
            if v20 {
                US10::US10_ModuleListed
            } else {
                let v22: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v23: Rc<UH1> = method56(v0.clone(), v14, v22);
                let v30: Rc<str> = match v0 {
                    US9::US9_SpiprojProject => {
                        let mut v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                        v24.borrow_mut().push(v25);
                        v24.borrow_mut().push(v2);
                        let v26: Rc<Vec<Rc<str>>> = Rc::new(v24.borrow().clone());
                        let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v26).as_ref().clone()));
                        let v28: Rc<str> = Rc::<str>::from((*v27.borrow()).join(&*v5));
                        v28
                    }
                    US9::US9_TomlProject => {
                        v2
                    }
                };
                let v31: Rc<UH1> = method57(v23, v15);
                let v32: Rc<UH1> = Rc::new(UH1::UH1_Cons(v30, v31));
                let v33: Rc<UH1> = method57(v13, v32);
                let v34: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v35: i32 = 0i32;
                let v36: i32 = method29(v34.clone(), v33, v35);
                let v37: Rc<Vec<Rc<str>>> = Rc::new(v34.borrow().clone());
                let v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v37).as_ref().clone()));
                let v39: Rc<str> = Rc::<str>::from((*v38.borrow()).join(&*v3));
                let mut v40: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v40.borrow_mut().push(v39);
                v40.borrow_mut().push(v3);
                let v41: Rc<Vec<Rc<str>>> = Rc::new(v40.borrow().clone());
                let v42: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v41).as_ref().clone()));
                let mut v43: Rc<str> = Rc::<str>::from((*v42.borrow()).join(&*v5));
                US10::US10_ModuleAdded(v43)
            }
        }
    }
}
fn asks_written_37(mut v0: Rc<str>, mut v1: US3) -> US1 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<str> = string_concat(&[&*v4, "/state"]);
    let mut v6: Rc<str> = string_concat(&[&*v5, "/agile.spi"]);
    let v7: Rc<str> = text_of_23(v6);
    let v8: Rc<str> = string_concat(&[&*v5, "/asks.spi"]);
    let mut v9: Rc<str> = text_of_23(v8.clone());
    let mut v10: Rc<UH0> = asks_of_text_1(v9.clone());
    let mut v11: Rc<UH0> = asks_changed_38(v10, v1.clone());
    let mut v12: bool = &*v7 == "";
    if v12 {
        let mut v13: Rc<str> = string_concat(&["no backlog state at ", &*v5]);
        US1::US1_AsksRefused(v13)
    } else {
        let v20: Rc<str> = match &v1 {
            US3::US3_AskCleared(v18) => {
                let v18: Rc<str> = v18.clone();
                v18
            }
            US3::US3_AskSet(v15, v16, v17) => {
                let v15: Rc<str> = v15.clone();
                let v16: Rc<str> = v16.clone();
                let mut v17: Rc<str> = v17.clone();
                v15
            }
        };
        let v21: bool = task_known_41(v7, v20);
        let v22: bool = v21 == false;
        if v22 {
            let v28: Rc<str> = match v1 {
                US3::US3_AskCleared(v26) => {
                    let v26: Rc<str> = v26;
                    v26
                }
                US3::US3_AskSet(v23, v24, v25) => {
                    let v23: Rc<str> = v23;
                    let mut v24: Rc<str> = v24;
                    let v25: Rc<str> = v25;
                    v23
                }
            };
            let v30: Rc<str> = string_concat(&["no task ", &*v28, " in state/agile.spi"]);
            US1::US1_AsksRefused(v30)
        } else {
            let v32: Rc<str> = text_of_23(v8.clone());
            let v33: bool = &*v32 != &*v9;
            if v33 {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/asks.spi changed since it was read; run again"); } LIT.with(|lit| lit.clone()) };
                US1::US1_AsksRefused(v34)
            } else {
                let v36: Rc<str> = asks_text_42(v11.clone());
                let v37: bool = replaced_47(v8, v36);
                let v38: bool = v37 == false;
                if v38 {
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/asks.spi could not be written"); } LIT.with(|lit| lit.clone()) };
                    US1::US1_AsksRefused(v39)
                } else {
                    let v41: i32 = 0i32;
                    let v42: i32 = method50(v11, v41);
                    let mut v43: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    v43.borrow_mut().push(v5.clone());
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.toml"); } LIT.with(|lit| lit.clone()) };
                    v43.borrow_mut().push(v44.clone());
                    let v45: Rc<Vec<Rc<str>>> = Rc::new(v43.borrow().clone());
                    let v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v45).as_ref().clone()));
                    let v47: Rc<str> = Rc::<str>::from((*v46.borrow()).join(&*v3));
                    let v48: bool = std::path::Path::new(&*v47).is_file();
                    let v51: US9 = if v48 {
                        US9::US9_TomlProject
                    } else {
                        US9::US9_SpiprojProject
                    };
                    let v54: Rc<str> = match v51 {
                        US9::US9_SpiprojProject => {
                            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
                            v52
                        }
                        US9::US9_TomlProject => {
                            v44.clone()
                        }
                    };
                    let mut v55: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    v55.borrow_mut().push(v5.clone());
                    v55.borrow_mut().push(v54);
                    let v56: Rc<Vec<Rc<str>>> = Rc::new(v55.borrow().clone());
                    let v57: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v56).as_ref().clone()));
                    let v58: Rc<str> = Rc::<str>::from((*v57.borrow()).join(&*v3));
                    let mut v59: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    v59.borrow_mut().push(v5);
                    v59.borrow_mut().push(v44);
                    let v60: Rc<Vec<Rc<str>>> = Rc::new(v59.borrow().clone());
                    let v61: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v60).as_ref().clone()));
                    let v62: Rc<str> = Rc::<str>::from((*v61.borrow()).join(&*v3));
                    let v63: bool = std::path::Path::new(&*v62).is_file();
                    let v66: US9 = if v63 {
                        US9::US9_TomlProject
                    } else {
                        US9::US9_SpiprojProject
                    };
                    let v67: Rc<str> = text_of_23(v58.clone());
                    let v68: US10 = module_listed_51(v66, v67);
                    match v68 {
                        US10::US10_ModuleAdded(v72) => {
                            let v72: Rc<str> = v72;
                            let v73: bool = replaced_47(v58, v72);
                            if v73 {
                                US1::US1_AsksWritten(v42)
                            } else {
                                let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the state project was not updated"); } LIT.with(|lit| lit.clone()) };
                                US1::US1_AsksRefused(v75)
                            }
                        }
                        US10::US10_ModuleListed => {
                            US1::US1_AsksWritten(v42)
                        }
                        US10::US10_NoModulesSection => {
                            let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the state project has no modules section to list asks in"); } LIT.with(|lit| lit.clone()) };
                            US1::US1_AsksRefused(v70)
                        }
                    }
                }
            }
        }
    }
}
fn ask_edit_run_12() -> i32 {
    let mut v0: US2 = ask_request_16();
    match v0 {
        US2::US2_None => {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile edit <root> <id> --asked <yyyy-mm-dd> --ask \"<scope>; done when <criterion>\" | --asked --clear (distil the request: scope and done criterion, never a transcript; a DoneDefinition marker declared in state/markers.spi also marks it)"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v14);
            2i32
        }
        US2::US2_Some(v1, v2, v3) => {
            let mut v1: Rc<str> = v1;
            let mut v2: US3 = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = match &v2 {
                US3::US3_AskCleared(v8) => {
                    let v8: Rc<str> = v8.clone();
                    v8
                }
                US3::US3_AskSet(v5, v6, v7) => {
                    let mut v5: Rc<str> = v5.clone();
                    let mut v6: Rc<str> = v6.clone();
                    let v7: Rc<str> = v7.clone();
                    v5
                }
            };
            let mut v11: Rc<str> = rewrites_of_34(v2.clone());
            let mut v12: US1 = asks_written_37(v1, v2);
            reported_13(v4, v10, v3, v11, v12)
        }
    }
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        ask_edit_run_12()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn concat5_61(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn civil_of_days_62(mut v0: u64) -> (u64, u64, u64) {
    let mut v1: u64 = v0.wrapping_add(719468u64);
    let mut v2: u64 = v1.wrapping_div(146097u64);
    let v3: u64 = v2.wrapping_mul(146097u64);
    let v4: u64 = v1.wrapping_sub(v3);
    let mut v5: u64 = v4.wrapping_div(1460u64);
    let mut v6: u64 = v4.wrapping_sub(v5);
    let v7: u64 = v4.wrapping_div(36524u64);
    let v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let mut v10: u64 = v8.wrapping_sub(v9);
    let mut v11: u64 = v10.wrapping_div(365u64);
    let mut v12: u64 = 365u64.wrapping_mul(v11);
    let mut v13: u64 = v11.wrapping_div(4u64);
    let v14: u64 = v12.wrapping_add(v13);
    let v15: u64 = v11.wrapping_div(100u64);
    let v16: u64 = v14.wrapping_sub(v15);
    let mut v17: u64 = v4.wrapping_sub(v16);
    let v18: u64 = 5u64.wrapping_mul(v17);
    let v19: u64 = v18.wrapping_add(2u64);
    let v20: u64 = v19.wrapping_div(153u64);
    let v21: u64 = 153u64.wrapping_mul(v20);
    let v22: u64 = v21.wrapping_add(2u64);
    let v23: u64 = v22.wrapping_div(5u64);
    let mut v24: u64 = v17.wrapping_sub(v23);
    let v25: u64 = v24.wrapping_add(1u64);
    let v26: bool = v20 < 10u64;
    let v29: u64 = if v26 {
        let v27: u64 = v20.wrapping_add(3u64);
        v27
    } else {
        let v28: u64 = v20.wrapping_sub(9u64);
        v28
    };
    let v30: u64 = v2.wrapping_mul(400u64);
    let v31: u64 = v11.wrapping_add(v30);
    let v32: bool = v29 < 3u64;
    let v33: u64 = if v32 {
        1u64
    } else {
        0u64
    };
    let v34: u64 = v31.wrapping_add(v33);
    (v34, v29, v25)
}
fn utc_stamp_60(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(1000u64);
    let mut v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_62(v2);
    let mut v6: u64 = v1.wrapping_rem(86400u64);
    let v7: u64 = v6.wrapping_div(60u64);
    let v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v10: bool = v4 < 10u64;
    let v16: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_61(v11, v12, v13.clone(), v13.clone(), v13)
    } else {
        let v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15
    };
    let mut v17: bool = v5 < 10u64;
    let v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_61(v18, v19, v20.clone(), v20.clone(), v20)
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22
    };
    let mut v24: Rc<str> = concat5_61(v8, v9.clone(), v16, v9, v23);
    let v25: u64 = v7.wrapping_div(60u64);
    let v26: bool = v25 < 10u64;
    let v32: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_61(v27, v28, v29.clone(), v29.clone(), v29)
    } else {
        let v31: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        v31
    };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v34: u64 = v7.wrapping_rem(60u64);
    let v35: bool = v34 < 10u64;
    let v41: Rc<str> = if v35 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_61(v36, v37, v38.clone(), v38.clone(), v38)
    } else {
        let mut v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = concat5_61(v32, v33, v41, v42, v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    concat5_61(v24, v45, v44, v43.clone(), v43)
}
fn today_59(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = utc_stamp_60(v0);
    let mut v2: usize = ((0i32) as usize);
    let v3: usize = ((10i32) as usize);
    let v4: Rc<str> = Rc::<str>::from(&(v1)[v2..v3]);
    v4
}
fn ask_now_run_58() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v4: i32 = 3i32;
    let mut v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v6: bool = &*v3 != "";
    let mut v10: bool = if v6 {
        let v7: i32 = 4i32;
        let v8: Rc<str> = usize::try_from(v7).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v9: bool = &*v8 == "";
        v9
    } else {
        false
    };
    let mut v13: bool = if v10 {
        let mut v11: Rc<str> = Rc::<str>::from((v5).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        let mut v12: bool = &*v11 != "";
        v12
    } else {
        false
    };
    let v15: bool = if v13 {
        distilled_18(v1.clone(), v5.clone())
    } else {
        false
    };
    if v15 {
        let v16: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let mut v17: Rc<str> = today_59(v16);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ask"); } LIT.with(|lit| lit.clone()) };
        let v19: US3 = US3::US3_AskSet(v3.clone(), v17.clone(), v5.clone());
        let v20: Rc<str> = rewrites_of_34(v19);
        let v21: US3 = US3::US3_AskSet(v3.clone(), v17.clone(), v5);
        let v22: US1 = asks_written_37(v1, v21);
        reported_13(v18, v3, v17, v20, v22)
    } else {
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile ask <root> <id> \"<scope>; done when <criterion>\" (records today's date; distil the request: scope and done criterion, never a transcript)"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v24);
        2i32
    }
}
fn closure6() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        ask_now_run_58()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_asks(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_agile_ask_edit_run() -> i32 {
    closure2()()
}
pub fn rig_agile_ask_now_run() -> i32 {
    closure6()()
}
pub type SpiralShape_accbf6de6842 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
