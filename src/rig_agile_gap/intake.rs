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
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_FileReadable(Rc<str>),
    US2_FileMissing(Rc<str>),
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
#[derive(Clone)]
enum US6 {
    US6_AddedText(Rc<str>),
    US6_AddedRefused(Rc<str>),
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
#[derive(Clone)]
enum US8 {
    US8_WordsRoot,
    US8_SymbolsRoot,
}
#[derive(Clone)]
enum US9 {
    US9_RequestRead(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US9_RequestRefused(Rc<str>),
}
#[derive(Clone)]
enum US11 {
    US11_FlagVerb,
    US11_FlagExit,
    US11_FlagArgv,
    US11_FlagCwd,
    US11_FlagAs,
}
#[derive(Clone)]
enum US10 {
    US10_FlagKnown(US11),
    US10_FlagUnknown,
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method4(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method4(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method3(v16, v17)
    }
}
fn method6(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US0 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v6: Rc<str> = string_concat(&["Marker (\"", &*v0, "\", \""]);
                let mut v7: bool = v3.contains(&*v6);
                if v7 {
                    return US0::US0_Some(v3);
                } else {
                    v1 = v4;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US0::US0_None;
            }
        }
    }
}
fn declared_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH0> = split_list_2(v2, v0);
    let mut v4: US0 = method6(v1, v3);
    match v4 {
        US0::US0_None => {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24
        }
        US0::US0_Some(v5) => {
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<UH0> = split_list_2(v6, v5);
            match Rc::unwrap_or_clone(v7) {
                UH0::UH0_Cons(v8, v9) => {
                    let mut v8: Rc<str> = v8;
                    let mut v9: Rc<UH0> = v9;
                    match Rc::unwrap_or_clone(v9) {
                        UH0::UH0_Cons(v10, v11) => {
                            let mut v10: Rc<str> = v10;
                            let mut v11: Rc<UH0> = v11;
                            match Rc::unwrap_or_clone(v11) {
                                UH0::UH0_Cons(v12, v13) => {
                                    let mut v12: Rc<str> = v12;
                                    let v13: Rc<UH0> = v13;
                                    match Rc::unwrap_or_clone(v13) {
                                        UH0::UH0_Cons(v14, v15) => {
                                            let v14: Rc<str> = v14;
                                            let v15: Rc<UH0> = v15;
                                            v14
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
            }
        }
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method9(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() = Task (\""); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.contains(&*v5);
            if v6 {
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
fn task_rows_8(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = split_list_2(v1, v0);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method9(v2, v3)
}
fn tagged_with_11(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = string_concat(&["team:", &*v0]);
    let v3: Rc<str> = string_concat(&[&*v2, "\""]);
    let mut v4: bool = v1.contains(&*v3);
    if v4 {
        true
    } else {
        let mut v5: Rc<str> = string_concat(&[&*v2, ","]);
        let mut v6: bool = v1.contains(&*v5);
        v6
    }
}
fn closure1(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        tagged_with_11(v0.clone(), v1)
    })
}
fn method10(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure1(v0)
}
fn method12(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: Rc<UH0>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = v1(v3);
                if v5 {
                    return true;
                } else {
                    let mut v6: Rc<dyn Fn(Rc<str>) -> bool> = method10(v0.clone());
                    (v1, v2) = (v6, v4);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn lane_team_7(mut v0: &Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: &Rc<str>, mut v4: &Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> Rc<str> {
    let mut v7: Rc<UH0> = task_rows_8(v6);
    let mut v8: Rc<dyn Fn(Rc<str>) -> bool> = method10(v5.clone());
    let mut v9: bool = method12(v5.clone(), v8, v7);
    if v9 {
        v5
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<UH0> = split_list_2(v10, v5);
        match Rc::unwrap_or_clone(v11) {
            UH0::UH0_Cons(v12, v13) => {
                let mut v12: Rc<str> = v12;
                let v13: Rc<UH0> = v13;
                v12
            }
            UH0::UH0_Nil => {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v14
            }
        }
    }
}
fn method16(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn id_prefix_of_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() = Task (\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method16(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let mut v8: i32 = v7.wrapping_add(12i32);
    let mut v9: bool = v7 < 0i32;
    let v17: Rc<str> = if v9 {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v10
    } else {
        let mut v11: i32 = (v0.clone().len() as i32);
        let mut v12: i32 = v11.wrapping_sub(v8);
        let v13: usize = ((v8) as usize);
        let v14: i32 = v8.wrapping_add(v12);
        let v15: usize = ((v14) as usize);
        let v16: Rc<str> = Rc::<str>::from(&(v0)[v13..v15]);
        v16
    };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Option<usize> = v17.find(&*v18);
    let v20: Option<usize> = method16(v19);
    let mut v21: Option<i32> = v20.map(|x| v4(x));
    let v23: i32 = v21.unwrap_or(-1i32);
    let v24: bool = v23 <= 0i32;
    if v24 {
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v25
    } else {
        let v26: usize = ((0i32) as usize);
        let v27: usize = ((v23) as usize);
        let v28: Rc<str> = Rc::<str>::from(&(v17)[v26..v27]);
        v28
    }
}
fn method17(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure3() -> Rc<dyn Fn((i32)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US1> = Rc::new(move |mut v0: (i32)| -> US1 {
        let mut v1: i32 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method14(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i32>>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, i32>> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = tagged_with_11(v0.clone(), v3.clone());
                let v25: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i32>> = if v5 {
                    let mut v6: Rc<str> = id_prefix_of_15(v3);
                    let mut v7: Option<i32> = v2.get(&v6).cloned();
                    let mut v8: Option<i32> = method17(v7);
                    let mut v9: Rc<dyn Fn((i32)) -> US1> = closure3();
                    let mut v10: Option<US1> = v8.map(|x| v9(x));
                    let mut v12: US1 = US1::US1_None;
                    let v13: US1 = v12.clone();
                    let v14: US1 = v10.unwrap_or(v13);
                    let v17: i32 = match v14 {
                        US1::US1_None => {
                            0i32
                        }
                        US1::US1_Some(v15) => {
                            let v15: i32 = v15;
                            v15
                        }
                    };
                    let v18: i32 = v17.wrapping_add(1i32);
                    let mut v19: std::collections::BTreeMap<Rc<str>, i32> = std::rc::Rc::unwrap_or_clone(v2);
                    let mut v21: bool = true; // 1;
                    let mut v19 = v19;
                    let v23: Option<i32> = v19.insert(v6, v18);
                    let v24: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i32>> = std::rc::Rc::new(v19);
                    v24
                } else {
                    v2
                };
                (v1, v2) = (v4, v25);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method18(mut v0: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i32>>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: i32) -> (Rc<str>, i32) {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let mut v6: Option<i32> = v0.get(&v4).cloned();
                let mut v7: Option<i32> = method17(v6);
                let mut v8: Rc<dyn Fn((i32)) -> US1> = closure3();
                let mut v9: Option<US1> = v7.map(|x| v8(x));
                let mut v11: US1 = US1::US1_None;
                let mut v12: US1 = v11.clone();
                let v13: US1 = v9.unwrap_or(v12);
                let v17: i32 = match v13 {
                    US1::US1_None => {
                        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dict.present: the key is not in the dict"); } LIT.with(|lit| lit.clone()) }))
                    }
                    US1::US1_Some(v14) => {
                        let v14: i32 = v14;
                        v14
                    }
                };
                let v18: bool = &*v4 != "";
                let v20: bool = if v18 {
                    let mut v19: bool = v17 > v3;
                    v19
                } else {
                    false
                };
                let (mut v21, mut v22): (Rc<str>, i32) = if v20 {
                    (v4, v17)
                } else {
                    (v2, v3)
                };
                (v1, v2, v3) = (v5, v21, v22);
                continue;
            }
            _ => {
                let (mut v25, mut v26): (Rc<str>, i32) = match Rc::unwrap_or_clone(v1) {
                    UH0::UH0_Nil => {
                        (v2, v3)
                    }
                    _ => unreachable!(),
                };
                return (v25, v26);
            }
        }
    }
}
fn team_prefix_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<UH0> = task_rows_8(v0);
    let mut v4: std::collections::BTreeMap<Rc<str>, i32> = std::collections::BTreeMap::new();
    let mut v5: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i32>> = std::rc::Rc::new(v4);
    let mut v6: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i32>> = method14(v1.clone(), v2, v5);
    let mut v7: Vec<Rc<str>> = v6.keys().cloned().collect::<Vec<_>>();
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v10: bool = true; let _vec_fold_ = v7.into_iter().fold(v8, |acc, x| { //;
    let mut v12: Rc<UH0> = acc;
    let v14: Rc<str> = x;
    let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v14, v12));
    let v17: bool = true; v16 });
    let mut v19: Rc<UH0> = _vec_fold_;
    let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v21: Rc<UH0> = method3(v19, v20);
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v23: i32 = 0i32;
    let (mut v24, mut v25): (Rc<str>, i32) = method18(v6, v21, v22, v23);
    let v26: bool = &*v24 == "";
    if v26 {
        let v27: Rc<str> = Rc::<str>::from(v1.to_uppercase());
        v27
    } else {
        v24
    }
}
fn trim_dashes_20(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.starts_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(1i32);
            let mut v5: usize = ((1i32) as usize);
            let mut v6: i32 = 1i32.wrapping_add(v4);
            let mut v7: usize = ((v6) as usize);
            let mut v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
            v0 = v8;
            continue;
        } else {
            return v0;
        }
    }
}
fn upper_words_19(mut v0: Rc<UH0>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<str> = trim_dashes_20(v2);
            let mut v5: Rc<str> = Rc::<str>::from(v4.to_uppercase());
            let mut v6: Rc<str> = upper_words_19(v3);
            let mut v7: bool = &*v5 == "";
            if v7 {
                v6
            } else {
                let mut v8: bool = &*v6 == "";
                if v8 {
                    v5
                } else {
                    let mut v10: Rc<str> = string_concat(&[&*v5, "-", &*v6]);
                    v10
                }
            }
        }
        UH0::UH0_Nil => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
    }
}
fn free_id_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    loop {
        let v3: bool = v2 == 1i32;
        let mut v7: Rc<str> = if v3 {
            v1.clone()
        } else {
            let mut v5: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let mut v6: Rc<str> = string_concat(&[&*v1, "-", &*v5]);
            v6
        };
        let mut v9: Rc<str> = string_concat(&["Task (\"", &*v7, "\","]);
        let mut v10: bool = v0.contains(&*v9);
        if v10 {
            let mut v11: i32 = v2.wrapping_add(1i32);
            v2 = v11;
            continue;
        } else {
            return v7;
        }
    }
}
fn gap_team_22(mut v0: &Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: &Rc<str>, mut v4: &Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("GapTeam"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = declared_5(v7, v8);
    let mut v10: bool = &*v9 != "";
    if v10 {
        v9
    } else {
        lane_team_7(v0, v1, v2, v3, v4, v5, v6)
    }
}
fn concat5_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn loop_24(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
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
            let mut v4: i32 = v3.wrapping_sub(6i32);
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
            let mut v4: i32 = v3.wrapping_sub(8i32);
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
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method28(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method28(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US4 = if v6 {
                let mut v7: US5 = US5::US5_HeadType;
                US4::US4_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US5 = US5::US5_HeadType;
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
                        let mut v21: bool = v2.starts_with(&*v20);
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
                        let v30: US5 = US5::US5_HeadUnion;
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
            Rc::new(UH0::UH0_Cons(v76, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method29(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn extras_applied_30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: &Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> US6 {
    let (mut v8, mut v9): (i32, Rc<str>) = agile_task_notes::rig_agile_groomed(&*v0, &*v1, &*v2, &*v3, &*v4, &*v6, &*v7);
    let mut v10: bool = v8 == 0i32;
    if v10 {
        US6::US6_AddedText(v9)
    } else {
        US6::US6_AddedRefused(v9)
    }
}
fn method32(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method32(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US4 = if v6 {
                let mut v7: US5 = US5::US5_HeadType;
                US4::US4_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US5 = US5::US5_HeadType;
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
                        let mut v21: bool = v2.starts_with(&*v20);
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
                        let v30: US5 = US5::US5_HeadUnion;
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
            Rc::new(UH0::UH0_Cons(v76, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn renamed_within_33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: bool = std::fs::rename(&*v1, &*v0).is_ok();
        if v3 {
            return true;
        } else {
            let mut v4: bool = v2 <= 1i32;
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
fn replace_records_unchanged_31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US7 {
    let v3: bool = rig_agile_records::rig_agile_records_unchanged(&*v0, &*v1);
    if v3 {
        let mut v4: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v6));
        let mut v8: Rc<str> = loop_24(v7);
        let mut v9: Option<usize> = v8.rfind(&*v6);
        let mut v10: Option<usize> = method16(v9);
        let mut v11: Rc<dyn Fn((usize)) -> i32> = closure4();
        let mut v12: Option<i32> = v10.map(|x| v11(x));
        let v14: i32 = v12.unwrap_or(-1i32);
        let v15: bool = v14 < 0i32;
        let mut v19: bool = if v15 {
            true
        } else {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v17: bool = v8.ends_with(&*v16);
            let v18: bool = v17 == false;
            v18
        };
        let v40: US3 = if v19 {
            US3::US3_OtherFile
        } else {
            let mut v21: usize = ((0i32) as usize);
            let v22: usize = ((v14) as usize);
            let v23: Rc<str> = Rc::<str>::from(&(v8)[v21..v22]);
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v25: bool = v23.ends_with(&*v24);
            if v25 {
                let v26: Rc<str> = loop_25(v23);
                US3::US3_StateFile(v26)
            } else {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v29: bool = v23.ends_with(&*v28);
                if v29 {
                    let v30: Rc<str> = loop_26(v23);
                    US3::US3_HistoryFile(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v23.ends_with(&*v32);
                    if v33 {
                        let v34: Rc<str> = loop_27(v23);
                        US3::US3_ArchiveFile(v34)
                    } else {
                        US3::US3_OtherFile
                    }
                }
            }
        };
        let v63: US8 = match v40 {
            US3::US3_ArchiveFile(v53) => {
                let v53: Rc<str> = v53;
                let v54: Rc<str> = string_concat(&[&*v53, "/state/spiral.toml"]);
                let v55: bool = std::path::Path::new(&*v54).is_file();
                if v55 {
                    US8::US8_SymbolsRoot
                } else {
                    US8::US8_WordsRoot
                }
            }
            US3::US3_HistoryFile(v47) => {
                let v47: Rc<str> = v47;
                let v48: Rc<str> = string_concat(&[&*v47, "/state/spiral.toml"]);
                let v49: bool = std::path::Path::new(&*v48).is_file();
                if v49 {
                    US8::US8_SymbolsRoot
                } else {
                    US8::US8_WordsRoot
                }
            }
            US3::US3_OtherFile => {
                US8::US8_WordsRoot
            }
            US3::US3_StateFile(v41) => {
                let v41: Rc<str> = v41;
                let v42: Rc<str> = string_concat(&[&*v41, "/state/spiral.toml"]);
                let v43: bool = std::path::Path::new(&*v42).is_file();
                if v43 {
                    US8::US8_SymbolsRoot
                } else {
                    US8::US8_WordsRoot
                }
            }
        };
        let v75: Rc<str> = match v63 {
            US8::US8_SymbolsRoot => {
                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v65: Rc<UH0> = split_list_2(v64.clone(), v2.clone());
                let v66: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v67: Rc<UH0> = method32(v65, v66);
                let v68: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v69: i32 = 0i32;
                let v70: i32 = method29(v68.clone(), v67, v69);
                let v71: Rc<Vec<Rc<str>>> = Rc::new(v68.borrow().clone());
                let v72: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v71).as_ref().clone()));
                let v73: Rc<str> = Rc::<str>::from((*v72.borrow()).join(&*v64));
                v73
            }
            US8::US8_WordsRoot => {
                v2.clone()
            }
        };
        let v76: bool = std::fs::write(&*v4, &*v75).is_ok();
        if v76 {
            let v77: i32 = 40i32;
            let v78: bool = renamed_within_33(v0.clone(), v4.clone(), v77);
            if v78 {
                let mut v79: Rc<str> = Rc::<str>::from(v0.replace(&*v5, &*v6));
                let mut v80: Rc<str> = loop_24(v79);
                let v81: Option<usize> = v80.rfind(&*v6);
                let mut v82: Option<usize> = method16(v81);
                let mut v83: Option<i32> = v82.map(|x| v11(x));
                let v85: i32 = v83.unwrap_or(-1i32);
                let v86: bool = v85 < 0i32;
                let v90: bool = if v86 {
                    true
                } else {
                    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v88: bool = v80.ends_with(&*v87);
                    let v89: bool = v88 == false;
                    v89
                };
                let v111: US3 = if v90 {
                    US3::US3_OtherFile
                } else {
                    let v92: usize = ((0i32) as usize);
                    let v93: usize = ((v85) as usize);
                    let v94: Rc<str> = Rc::<str>::from(&(v80)[v92..v93]);
                    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v96: bool = v94.ends_with(&*v95);
                    if v96 {
                        let v97: Rc<str> = loop_25(v94);
                        US3::US3_StateFile(v97)
                    } else {
                        let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v100: bool = v94.ends_with(&*v99);
                        if v100 {
                            let v101: Rc<str> = loop_26(v94);
                            US3::US3_HistoryFile(v101)
                        } else {
                            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v104: bool = v94.ends_with(&*v103);
                            if v104 {
                                let v105: Rc<str> = loop_27(v94);
                                US3::US3_ArchiveFile(v105)
                            } else {
                                US3::US3_OtherFile
                            }
                        }
                    }
                };
                let v134: US8 = match v111 {
                    US3::US3_ArchiveFile(v124) => {
                        let v124: Rc<str> = v124;
                        let v125: Rc<str> = string_concat(&[&*v124, "/state/spiral.toml"]);
                        let v126: bool = std::path::Path::new(&*v125).is_file();
                        if v126 {
                            US8::US8_SymbolsRoot
                        } else {
                            US8::US8_WordsRoot
                        }
                    }
                    US3::US3_HistoryFile(v118) => {
                        let v118: Rc<str> = v118;
                        let v119: Rc<str> = string_concat(&[&*v118, "/state/spiral.toml"]);
                        let v120: bool = std::path::Path::new(&*v119).is_file();
                        if v120 {
                            US8::US8_SymbolsRoot
                        } else {
                            US8::US8_WordsRoot
                        }
                    }
                    US3::US3_OtherFile => {
                        US8::US8_WordsRoot
                    }
                    US3::US3_StateFile(v112) => {
                        let v112: Rc<str> = v112;
                        let v113: Rc<str> = string_concat(&[&*v112, "/state/spiral.toml"]);
                        let v114: bool = std::path::Path::new(&*v113).is_file();
                        if v114 {
                            US8::US8_SymbolsRoot
                        } else {
                            US8::US8_WordsRoot
                        }
                    }
                };
                let v146: Rc<str> = match v134 {
                    US8::US8_SymbolsRoot => {
                        let mut v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v136: Rc<UH0> = split_list_2(v135.clone(), v2);
                        let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v138: Rc<UH0> = method32(v136, v137);
                        let v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v140: i32 = 0i32;
                        let v141: i32 = method29(v139.clone(), v138, v140);
                        let v142: Rc<Vec<Rc<str>>> = Rc::new(v139.borrow().clone());
                        let v143: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v142).as_ref().clone()));
                        let v144: Rc<str> = Rc::<str>::from((*v143.borrow()).join(&*v135));
                        v144
                    }
                    US8::US8_WordsRoot => {
                        v2
                    }
                };
                let v147: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v146);
                if v147 {
                    US7::US7_Replaced
                } else {
                    US7::US7_ReadbackMismatch
                }
            } else {
                let v151: bool = std::fs::remove_file(&*v4).is_ok() || !std::path::Path::new(&*v4).exists();
                if v151 {
                    US7::US7_RenameFailed
                } else {
                    US7::US7_RenameFailedTemporaryKept
                }
            }
        } else {
            US7::US7_TemporaryWriteFailed
        }
    } else {
        US7::US7_ChangedSinceRead
    }
}
fn gap_filed_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> i32 {
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v8));
    let mut v10: Rc<str> = string_concat(&[&*v9, "/state/agile.spi"]);
    let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v10).unwrap_or_default().as_str());
    let mut v12: bool = &*v11 == "";
    if v12 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing or empty"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = string_concat(&["agile gap rejected: ", &*v13]);
        let v15: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v14 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v15
    } else {
        let v16: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v8));
        let v17: Rc<str> = string_concat(&[&*v16, "/state/markers.spi"]);
        let v18: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v17).unwrap_or_default().as_str());
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<UH0> = split_list_2(v19.clone(), v2.clone());
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("GapPrefix"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = declared_5(v18.clone(), v21);
        let v23: bool = &*v22 != "";
        let v26: Rc<str> = if v23 {
            v22
        } else {
            let v24: Rc<str> = lane_team_7(&v1, &v2, &v3, &v4, &v5, v6.clone(), v11.clone());
            team_prefix_13(v11.clone(), v24)
        };
        let v28: Rc<str> = upper_words_19(v20);
        let v29: Rc<str> = string_concat(&[&*v26, "-GAP-", &*v28]);
        let v30: i32 = 1i32;
        let v31: Rc<str> = free_id_21(v11.clone(), v29, v30);
        let v32: Rc<UH0> = split_list_2(v19, v2.clone());
        let v38: Rc<str> = match Rc::unwrap_or_clone(v32) {
            UH0::UH0_Cons(v33, v34) => {
                let v33: Rc<str> = v33;
                let v34: Rc<UH0> = v34;
                trim_dashes_20(v33)
            }
            UH0::UH0_Nil => {
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v36
            }
        };
        let v41: Rc<str> = gap_team_22(&v1, &v2, &v3, &v4, &v5, v6.clone(), v11, v18);
        let v42: Rc<str> = string_concat(&["rig,gap,", &*v38, ",team:", &*v41]);
        let v44: Rc<str> = string_concat(&["FAILED CALL: verb=", &*v2, " exit="]);
        let v45: bool = &*v3 == "";
        let v47: Rc<str> = if v45 {
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v46
        } else {
            v3
        };
        let v49: Rc<str> = string_concat(&[&*v44, &*v47, " cwd="]);
        let v50: bool = &*v5 == "";
        let v52: Rc<str> = if v50 {
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v51
        } else {
            v5
        };
        let v54: Rc<str> = string_concat(&[&*v49, &*v52, " argv="]);
        let v55: bool = &*v4 == "";
        let v57: Rc<str> = if v55 {
            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v56
        } else {
            v4
        };
        let v61: Rc<str> = string_concat(&[&*v54, &*v57, ". DoD: the call works in one step, or refuses with intent, usage, valid values and a corrected example. Next step: reproduce the call, fix it in the lane that hit it, add a spec/one-call case. TypeFeatures: none yet."]);
        let v62: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v65: Rc<str> = concat5_23(v0, v63, v64.clone(), v64.clone(), v64.clone());
        let v66: bool = std::path::Path::new(&*v65).is_file();
        let v69: US2 = if v66 {
            US2::US2_FileReadable(v65.clone())
        } else {
            US2::US2_FileMissing(v65.clone())
        };
        match v69 {
            US2::US2_FileMissing(v70) => {
                let v70: Rc<str> = v70;
                let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v74: Rc<str> = concat5_23(v72, v31, v73, v71, v64);
                let v75: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v74 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v75
            }
            US2::US2_FileReadable(v76) => {
                let v76: Rc<str> = v76;
                let v77: Rc<str> = std::fs::read_to_string(&*v76).unwrap_or_default().into();
                let v78: Rc<str> = Rc::<str>::from(v76.replace(&*v7, &*v8));
                let mut v79: Rc<str> = loop_24(v78);
                let mut v80: Option<usize> = v79.rfind(&*v8);
                let v81: Option<usize> = method16(v80);
                let mut v82: Rc<dyn Fn((usize)) -> i32> = closure4();
                let mut v83: Option<i32> = v81.map(|x| v82(x));
                let v85: i32 = v83.unwrap_or(-1i32);
                let v86: bool = v85 < 0i32;
                let v90: bool = if v86 {
                    true
                } else {
                    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v88: bool = v79.ends_with(&*v87);
                    let v89: bool = v88 == false;
                    v89
                };
                let v111: US3 = if v90 {
                    US3::US3_OtherFile
                } else {
                    let v92: usize = ((0i32) as usize);
                    let v93: usize = ((v85) as usize);
                    let v94: Rc<str> = Rc::<str>::from(&(v79)[v92..v93]);
                    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v96: bool = v94.ends_with(&*v95);
                    if v96 {
                        let v97: Rc<str> = loop_25(v94);
                        US3::US3_StateFile(v97)
                    } else {
                        let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v100: bool = v94.ends_with(&*v99);
                        if v100 {
                            let v101: Rc<str> = loop_26(v94);
                            US3::US3_HistoryFile(v101)
                        } else {
                            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v104: bool = v94.ends_with(&*v103);
                            if v104 {
                                let v105: Rc<str> = loop_27(v94);
                                US3::US3_ArchiveFile(v105)
                            } else {
                                US3::US3_OtherFile
                            }
                        }
                    }
                };
                let v122: Rc<str> = match v111 {
                    US3::US3_OtherFile => {
                        v77
                    }
                    _ => {
                        let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v113: Rc<UH0> = split_list_2(v112.clone(), v77);
                        let v114: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v115: Rc<UH0> = method28(v113, v114);
                        let v116: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v117: i32 = 0i32;
                        let v118: i32 = method29(v116.clone(), v115, v117);
                        let v119: Rc<Vec<Rc<str>>> = Rc::new(v116.borrow().clone());
                        let v120: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v119).as_ref().clone()));
                        let v121: Rc<str> = Rc::<str>::from((*v120.borrow()).join(&*v112));
                        v121
                    }
                };
                let mut v123: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v122);
                let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                let v126: Rc<str> = concat5_23(v124, v31.clone(), v125, v64.clone(), v64.clone());
                let v127: bool = v123.contains(&*v126);
                if v127 {
                    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task with this id already exists"); } LIT.with(|lit| lit.clone()) };
                    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                    let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let v131: Rc<str> = concat5_23(v129, v31, v130, v128, v64);
                    let v132: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v131 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v132
                } else {
                    let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v135: Rc<str> = Rc::<str>::from(v31.replace(&*v133, &*v134));
                    let v136: Rc<str> = std::rc::Rc::<str>::from(v135.to_ascii_lowercase());
                    let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                    let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                    let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                    let v140: Rc<str> = agile_task_creation_text::rig_agile_task_line(&v31, &v136, &v137, &v138, &v139, &v1, &v42, &v64);
                    let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v142: Rc<str> = std::rc::Rc::<str>::from(v123.trim_end_matches(&*v141));
                    let v145: Rc<str> = string_concat(&[&*v142, "\n", &*v140, "\n"]);
                    let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("30"); } LIT.with(|lit| lit.clone()) };
                    let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("light"); } LIT.with(|lit| lit.clone()) };
                    let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agent"); } LIT.with(|lit| lit.clone()) };
                    let v149: US6 = extras_applied_30(v145, v31.clone(), v146.clone(), v147.clone(), v148.clone(), &v64, v61.clone(), v6);
                    match v149 {
                        US6::US6_AddedRefused(v150) => {
                            let v150: Rc<str> = v150;
                            let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                            let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                            let v153: Rc<str> = concat5_23(v151, v31, v152, v150, v64);
                            let v154: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v153 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v154
                        }
                        US6::US6_AddedText(v155) => {
                            let v155: Rc<str> = v155;
                            let v156: US7 = replace_records_unchanged_31(v65, v123.clone(), v155.clone());
                            match &v156 {
                                US7::US7_Replaced => {
                                    let mut v157: Rc<str> = Rc::<str>::from(v31.replace(&*v133, &*v134));
                                    let v158: Rc<str> = std::rc::Rc::<str>::from(v157.to_ascii_lowercase());
                                    let v159: u64 = v123.len() as u64;
                                    let v160: u64 = v155.len() as u64;
                                    let v161: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v62);
                                    let v162: Rc<str> = agile_task_creation_text::rig_agile_add_receipt(&v31, &v158, &v146, &v147, &v148, &v64, &v61);
                                    let v163: Rc<str> = std::rc::Rc::<str>::from((v159).to_string());
                                    let v166: Rc<str> = std::rc::Rc::<str>::from((v160).to_string());
                                    let v169: Rc<str> = std::rc::Rc::<str>::from((v161).to_string());
                                    let v171: Rc<str> = string_concat(&[&*v162, " bytes_before=", &*v163, " bytes_after=", &*v166, " elapsed_ms=", &*v169]);
                                    let v172: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v171.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                    v172
                                }
                                _ => {
                                    let v182: Rc<str> = match v156 {
                                        US7::US7_ChangedSinceRead => {
                                            let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                            v177
                                        }
                                        US7::US7_ReadbackMismatch => {
                                            let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                            v176
                                        }
                                        US7::US7_RenameFailed => {
                                            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                            v174
                                        }
                                        US7::US7_RenameFailedTemporaryKept => {
                                            let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                            v175
                                        }
                                        US7::US7_TemporaryWriteFailed => {
                                            let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                            v173
                                        }
                                        _ => unreachable!(),
                                    };
                                    let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                    let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                    let v185: Rc<str> = concat5_23(v183, v31, v184, v182, v64);
                                    let v186: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v185 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v186
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn command_line_argument_count_35() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_36(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_34(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_35();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US0 = if v4 {
        let mut v5: Rc<str> = command_line_argument_36(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let mut v10: Rc<str> = v10;
            let mut v11: i32 = v0.wrapping_add(1i32);
            let mut v12: Rc<UH0> = arguments_from_34(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn flag_of_word_38(mut v0: Rc<str>) -> US10 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Verb"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(v1.to_lowercase());
    let v3: Rc<str> = string_concat(&["--", &*v2]);
    let mut v4: bool = &*v0 == &*v3;
    if v4 {
        let mut v5: US11 = US11::US11_FlagVerb;
        US10::US10_FlagKnown(v5)
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Exit"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
        let mut v9: Rc<str> = string_concat(&["--", &*v8]);
        let mut v10: bool = &*v0 == &*v9;
        if v10 {
            let mut v11: US11 = US11::US11_FlagExit;
            US10::US10_FlagKnown(v11)
        } else {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Argv"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = Rc::<str>::from(v13.to_lowercase());
            let v15: Rc<str> = string_concat(&["--", &*v14]);
            let v16: bool = &*v0 == &*v15;
            if v16 {
                let v17: US11 = US11::US11_FlagArgv;
                US10::US10_FlagKnown(v17)
            } else {
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Cwd"); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = Rc::<str>::from(v19.to_lowercase());
                let mut v21: Rc<str> = string_concat(&["--", &*v20]);
                let v22: bool = &*v0 == &*v21;
                if v22 {
                    let v23: US11 = US11::US11_FlagCwd;
                    US10::US10_FlagKnown(v23)
                } else {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = Rc::<str>::from(v25.to_lowercase());
                    let v27: Rc<str> = string_concat(&["--", &*v26]);
                    let v28: bool = &*v0 == &*v27;
                    if v28 {
                        let v29: US11 = US11::US11_FlagAs;
                        US10::US10_FlagKnown(v29)
                    } else {
                        US10::US10_FlagUnknown
                    }
                }
            }
        }
    }
}
fn request_from_37(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> US9 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v8, v9) => {
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<UH0> = v9;
                let mut v10: US10 = flag_of_word_38(v8.clone());
                match v10 {
                    US10::US10_FlagKnown(v41) => {
                        let v41: US11 = v41;
                        match Rc::unwrap_or_clone(v9) {
                            UH0::UH0_Cons(v44, v45) => {
                                let v44: Rc<str> = v44;
                                let v45: Rc<UH0> = v45;
                                let v46: Rc<str> = Rc::<str>::from((v44).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v41 {
                                    US11::US11_FlagVerb => {
                                        (v1, v46, v3, v4, v5, v6)
                                    }
                                    _ => {
                                        let (mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v41 {
                                            US11::US11_FlagExit => {
                                                (v1, v2, v46, v4, v5, v6)
                                            }
                                            _ => {
                                                let (mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v41 {
                                                    US11::US11_FlagArgv => {
                                                        (v1, v2, v3, v46, v5, v6)
                                                    }
                                                    _ => {
                                                        let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v41 {
                                                            US11::US11_FlagCwd => {
                                                                (v1, v2, v3, v4, v46, v6)
                                                            }
                                                            _ => {
                                                                let (mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v41 {
                                                                    US11::US11_FlagAs => {
                                                                        (v1, v2, v3, v4, v5, v46)
                                                                    }
                                                                    _ => unreachable!(),
                                                                };
                                                                (v47, v48, v49, v50, v51, v52)
                                                            }
                                                        };
                                                        (v53, v54, v55, v56, v57, v58)
                                                    }
                                                };
                                                (v59, v60, v61, v62, v63, v64)
                                            }
                                        };
                                        (v65, v66, v67, v68, v69, v70)
                                    }
                                };
                                (v0, v1, v2, v3, v4, v5, v6) = (v45, v71, v72, v73, v74, v75, v76);
                                continue;
                            }
                            UH0::UH0_Nil => {
                                let v42: Rc<str> = string_concat(&[&*v8, " needs a value"]);
                                return US9::US9_RequestRefused(v42);
                            }
                        }
                    }
                    US10::US10_FlagUnknown => {
                        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Verb"); } LIT.with(|lit| lit.clone()) };
                        let v14: Rc<str> = Rc::<str>::from(v13.to_lowercase());
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Exit"); } LIT.with(|lit| lit.clone()) };
                        let mut v19: Rc<str> = Rc::<str>::from(v18.to_lowercase());
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Argv"); } LIT.with(|lit| lit.clone()) };
                        let v24: Rc<str> = Rc::<str>::from(v23.to_lowercase());
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Cwd"); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = Rc::<str>::from(v28.to_lowercase());
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                        let v34: Rc<str> = Rc::<str>::from(v33.to_lowercase());
                        let v39: Rc<str> = string_concat(&["unknown option ", &*v8, " (expected --", &*v14, ", --", &*v19, ", --", &*v24, ", --", &*v29, ", --", &*v34, ")"]);
                        return US9::US9_RequestRefused(v39);
                    }
                }
            }
            UH0::UH0_Nil => {
                return US9::US9_RequestRead(v1, v2, v3, v4, v5, v6);
            }
        }
    }
}
fn bytes_within_40(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v1.clone().len() as i32);
        let mut v4: bool = v3 <= v2;
        if v4 {
            return true;
        } else {
            let mut v5: usize = ((v2) as usize);
            let mut v6: i32 = v2.wrapping_add(1i32);
            let mut v7: usize = ((v6) as usize);
            let mut v8: Rc<str> = Rc::<str>::from(&(v1)[v5..v7]);
            let mut v9: bool = v0.contains(&*v8);
            if v9 {
                v2 = v6;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn request_checked_39(mut v0: US9) -> US9 {
    match &v0 {
        US9::US9_RequestRead(v2, v3, v4, v5, v6, v7) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<str> = v3.clone();
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = v6.clone();
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<UH0> = split_list_2(v8, v7);
            let v14: Rc<str> = match Rc::unwrap_or_clone(v9) {
                UH0::UH0_Cons(v10, v11) => {
                    let mut v10: Rc<str> = v10;
                    let mut v11: Rc<UH0> = v11;
                    v10
                }
                UH0::UH0_Nil => {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v12
                }
            };
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyz0123456789- "); } LIT.with(|lit| lit.clone()) };
            let v16: i32 = 0i32;
            let v17: bool = bytes_within_40(v15, v3.clone(), v16);
            let v18: bool = &*v14 != "";
            let v22: bool = if v18 {
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyz0123456789"); } LIT.with(|lit| lit.clone()) };
                let v20: i32 = 0i32;
                bytes_within_40(v19, v14, v20)
            } else {
                false
            };
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) };
            let v24: i32 = 0i32;
            let v25: bool = bytes_within_40(v23, v4, v24);
            let v26: bool = &*v2 == "";
            if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a gap needs its title: what the call needed and did not get"); } LIT.with(|lit| lit.clone()) };
                US9::US9_RequestRefused(v27)
            } else {
                let v29: bool = &*v3 == "";
                if v29 {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--verb names the rig verb that failed or was missing, e.g. --verb 'job chain'"); } LIT.with(|lit| lit.clone()) };
                    US9::US9_RequestRefused(v30)
                } else {
                    let v32: bool = v17 == false;
                    if v32 {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--verb takes lower-case words, digits and dashes, e.g. --verb 'help agile add'"); } LIT.with(|lit| lit.clone()) };
                        US9::US9_RequestRefused(v33)
                    } else {
                        let v35: bool = v22 == false;
                        if v35 {
                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as <lane> names who hit the gap; its first word is the team, e.g. --as rig-fc1568a6"); } LIT.with(|lit| lit.clone()) };
                            US9::US9_RequestRefused(v36)
                        } else {
                            let v38: bool = v25 == false;
                            if v38 {
                                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--exit takes the failed call's exit code as digits, e.g. --exit 2"); } LIT.with(|lit| lit.clone()) };
                                US9::US9_RequestRefused(v39)
                            } else {
                                v0
                            }
                        }
                    }
                }
            }
        }
        US9::US9_RequestRefused(v1) => {
            let mut v1: Rc<str> = v1.clone();
            v0
        }
    }
}
fn gap_entry_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_34(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let mut v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH0::UH0_Cons(v8, v9) => {
                                    let mut v8: Rc<str> = v8;
                                    let mut v9: Rc<UH0> = v9;
                                    let mut v10: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v12: US9 = request_from_37(v9, v10, v11.clone(), v11.clone(), v11.clone(), v11.clone(), v11.clone());
                                    let v13: US9 = request_checked_39(v12);
                                    match v13 {
                                        US9::US9_RequestRead(v17, v18, v19, v20, v21, v22) => {
                                            let v17: Rc<str> = v17;
                                            let v18: Rc<str> = v18;
                                            let mut v19: Rc<str> = v19;
                                            let v20: Rc<str> = v20;
                                            let mut v21: Rc<str> = v21;
                                            let v22: Rc<str> = v22;
                                            gap_filed_1(v6, v17, v18, v19, v20, v21, v22)
                                        }
                                        US9::US9_RequestRefused(v14) => {
                                            let v14: Rc<str> = v14;
                                            let v15: Rc<str> = string_concat(&["agile gap rejected: ", &*v14]);
                                            let v16: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v15 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                            v16
                                        }
                                    }
                                }
                                _ => {
                                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile gap <root> <title> --verb <verb> --as <lane> [--exit <n>] [--argv <text>] [--cwd <dir>]"); } LIT.with(|lit| lit.clone()) };
                                    let v27: Rc<str> = string_concat(&["agile gap rejected: ", &*v26]);
                                    let v28: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v27 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v28
                                }
                            }
                        }
                        _ => {
                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile gap <root> <title> --verb <verb> --as <lane> [--exit <n>] [--argv <text>] [--cwd <dir>]"); } LIT.with(|lit| lit.clone()) };
                            let v31: Rc<str> = string_concat(&["agile gap rejected: ", &*v30]);
                            let v32: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v31 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v32
                        }
                    }
                }
                _ => {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile gap <root> <title> --verb <verb> --as <lane> [--exit <n>] [--argv <text>] [--cwd <dir>]"); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = string_concat(&["agile gap rejected: ", &*v34]);
                    let v36: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v35 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v36
                }
            }
        }
        _ => {
            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile gap <root> <title> --verb <verb> --as <lane> [--exit <n>] [--argv <text>] [--cwd <dir>]"); } LIT.with(|lit| lit.clone()) };
            let v39: Rc<str> = string_concat(&["agile gap rejected: ", &*v38]);
            let v40: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v39 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v40
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        gap_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_gap_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
