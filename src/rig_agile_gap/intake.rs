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
    US0_FileReadable(Rc<str>),
    US0_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_StateFile(Rc<str>),
    US1_HistoryFile(Rc<str>),
    US1_ArchiveFile(Rc<str>),
    US1_OtherFile,
}
#[derive(Clone)]
enum US3 {
    US3_HeadInl,
    US3_HeadUnion,
    US3_HeadOpen,
    US3_HeadType,
}
#[derive(Clone)]
enum US2 {
    US2_KeywordHead(US3),
    US2_SymbolHead(US3),
    US2_OtherLine,
}
#[derive(Clone)]
enum US4 {
    US4_AddedText(Rc<str>),
    US4_AddedRefused(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_Replaced,
    US5_TemporaryWriteFailed,
    US5_RenameFailed,
    US5_RenameFailedTemporaryKept,
    US5_ReadbackMismatch,
    US5_ChangedSinceRead,
}
#[derive(Clone)]
enum US6 {
    US6_WordsRoot,
    US6_SymbolsRoot,
}
#[derive(Clone)]
enum US7 {
    US7_Some(Rc<str>),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_RequestRead(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US8_RequestRefused(Rc<str>),
}
#[derive(Clone)]
enum US10 {
    US10_FlagVerb,
    US10_FlagExit,
    US10_FlagArgv,
    US10_FlagCwd,
    US10_FlagAs,
}
#[derive(Clone)]
enum US9 {
    US9_FlagKnown(US10),
    US9_FlagUnknown,
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method4(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
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
fn trim_dashes_6(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.starts_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(1i32);
            let v5: usize = ((1i32) as usize);
            let v6: i32 = 1i32.wrapping_add(v4);
            let v7: usize = ((v6) as usize);
            let mut v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
            v0 = v8;
            continue;
        } else {
            return v0;
        }
    }
}
fn upper_words_5(mut v0: Rc<UH0>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<str> = trim_dashes_6(v2);
            let v5: Rc<str> = Rc::<str>::from(v4.to_uppercase());
            let v6: Rc<str> = upper_words_5(v3);
            let v7: bool = &*v5 == "";
            if v7 {
                v6
            } else {
                let mut v8: bool = &*v6 == "";
                if v8 {
                    v5
                } else {
                    let v10: Rc<str> = string_concat(&[&*v5, "-", &*v6]);
                    v10
                }
            }
        }
        UH0::UH0_Nil => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
    }
}
fn free_id_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    loop {
        let v3: bool = v2 == 1i32;
        let v7: Rc<str> = if v3 {
            v1.clone()
        } else {
            let v5: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let v6: Rc<str> = string_concat(&[&*v1, "-", &*v5]);
            v6
        };
        let mut v9: Rc<str> = string_concat(&["Task (\"", &*v7, "\","]);
        let v10: bool = v0.contains(&*v9);
        if v10 {
            let mut v11: i32 = v2.wrapping_add(1i32);
            v2 = v11;
            continue;
        } else {
            return v7;
        }
    }
}
fn concat5_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn loop_9(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method10(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_11(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(6i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_12(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_13(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method14(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let v15: US2 = if v6 {
                let v7: US3 = US3::US3_HeadType;
                US2::US2_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US3 = US3::US3_HeadType;
                    US2::US2_SymbolHead(v11)
                } else {
                    US2::US2_OtherLine
                }
            };
            let v27: US2 = match &v15 {
                US2::US2_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US3 = US3::US3_HeadInl;
                        US2::US2_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US3 = US3::US3_HeadInl;
                            US2::US2_SymbolHead(v22)
                        } else {
                            US2::US2_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US2 = match &v27 {
                US2::US2_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US3 = US3::US3_HeadUnion;
                        US2::US2_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US3 = US3::US3_HeadUnion;
                            US2::US2_SymbolHead(v34)
                        } else {
                            US2::US2_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US2 = match &v39 {
                US2::US2_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US3 = US3::US3_HeadOpen;
                        US2::US2_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US3 = US3::US3_HeadOpen;
                            US2::US2_SymbolHead(v46)
                        } else {
                            US2::US2_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US2::US2_SymbolHead(v52) => {
                    let v52: US3 = v52;
                    let v60: Rc<str> = match &v52 {
                        US3::US3_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US3::US3_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US3::US3_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US3::US3_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US3::US3_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US3::US3_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US3::US3_HeadType => {
                            v5
                        }
                        US3::US3_HeadUnion => {
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
fn method15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn extras_applied_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: &Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> US4 {
    let (mut v8, mut v9): (i32, Rc<str>) = agile_task_notes::rig_agile_groomed(&*v0, &*v1, &*v2, &*v3, &*v4, &*v6, &*v7);
    let v10: bool = v8 == 0i32;
    if v10 {
        US4::US4_AddedText(v9)
    } else {
        US4::US4_AddedRefused(v9)
    }
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method18(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let v15: US2 = if v6 {
                let v7: US3 = US3::US3_HeadType;
                US2::US2_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US3 = US3::US3_HeadType;
                    US2::US2_SymbolHead(v11)
                } else {
                    US2::US2_OtherLine
                }
            };
            let v27: US2 = match &v15 {
                US2::US2_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US3 = US3::US3_HeadInl;
                        US2::US2_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US3 = US3::US3_HeadInl;
                            US2::US2_SymbolHead(v22)
                        } else {
                            US2::US2_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US2 = match &v27 {
                US2::US2_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US3 = US3::US3_HeadUnion;
                        US2::US2_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US3 = US3::US3_HeadUnion;
                            US2::US2_SymbolHead(v34)
                        } else {
                            US2::US2_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US2 = match &v39 {
                US2::US2_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US3 = US3::US3_HeadOpen;
                        US2::US2_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US3 = US3::US3_HeadOpen;
                            US2::US2_SymbolHead(v46)
                        } else {
                            US2::US2_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US2::US2_KeywordHead(v52) => {
                    let v52: US3 = v52;
                    let v59: Rc<str> = match &v52 {
                        US3::US3_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US3::US3_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US3::US3_HeadType => {
                            v5
                        }
                        US3::US3_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v60: i32 = (v59.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US3::US3_HeadInl => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v61
                        }
                        US3::US3_HeadOpen => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                        US3::US3_HeadType => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US3::US3_HeadUnion => {
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
fn renamed_within_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn replace_records_unchanged_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US5 {
    let v3: bool = rig_agile_records::rig_agile_records_unchanged(&*v0, &*v1);
    if v3 {
        let v4: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v6));
        let mut v8: Rc<str> = loop_9(v7);
        let mut v9: Option<usize> = v8.rfind(&*v6);
        let v10: Option<usize> = method10(v9);
        let mut v11: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v12: Option<i32> = v10.map(|x| v11(x));
        let v14: i32 = v12.unwrap_or(-1i32);
        let v15: bool = v14 < 0i32;
        let v19: bool = if v15 {
            true
        } else {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v17: bool = v8.ends_with(&*v16);
            let v18: bool = v17 == false;
            v18
        };
        let v40: US1 = if v19 {
            US1::US1_OtherFile
        } else {
            let v21: usize = ((0i32) as usize);
            let v22: usize = ((v14) as usize);
            let v23: Rc<str> = Rc::<str>::from(&(v8)[v21..v22]);
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v25: bool = v23.ends_with(&*v24);
            if v25 {
                let v26: Rc<str> = loop_11(v23);
                US1::US1_StateFile(v26)
            } else {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v29: bool = v23.ends_with(&*v28);
                if v29 {
                    let v30: Rc<str> = loop_12(v23);
                    US1::US1_HistoryFile(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v23.ends_with(&*v32);
                    if v33 {
                        let v34: Rc<str> = loop_13(v23);
                        US1::US1_ArchiveFile(v34)
                    } else {
                        US1::US1_OtherFile
                    }
                }
            }
        };
        let v63: US6 = match v40 {
            US1::US1_ArchiveFile(v53) => {
                let v53: Rc<str> = v53;
                let v54: Rc<str> = string_concat(&[&*v53, "/state/spiral.toml"]);
                let v55: bool = std::path::Path::new(&*v54).is_file();
                if v55 {
                    US6::US6_SymbolsRoot
                } else {
                    US6::US6_WordsRoot
                }
            }
            US1::US1_HistoryFile(v47) => {
                let v47: Rc<str> = v47;
                let v48: Rc<str> = string_concat(&[&*v47, "/state/spiral.toml"]);
                let v49: bool = std::path::Path::new(&*v48).is_file();
                if v49 {
                    US6::US6_SymbolsRoot
                } else {
                    US6::US6_WordsRoot
                }
            }
            US1::US1_OtherFile => {
                US6::US6_WordsRoot
            }
            US1::US1_StateFile(v41) => {
                let v41: Rc<str> = v41;
                let v42: Rc<str> = string_concat(&[&*v41, "/state/spiral.toml"]);
                let v43: bool = std::path::Path::new(&*v42).is_file();
                if v43 {
                    US6::US6_SymbolsRoot
                } else {
                    US6::US6_WordsRoot
                }
            }
        };
        let v75: Rc<str> = match v63 {
            US6::US6_SymbolsRoot => {
                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v65: Rc<UH0> = split_list_2(v64.clone(), v2.clone());
                let v66: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v67: Rc<UH0> = method18(v65, v66);
                let v68: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v69: i32 = 0i32;
                let v70: i32 = method15(v68.clone(), v67, v69);
                let v71: Rc<Vec<Rc<str>>> = Rc::new(v68.borrow().clone());
                let v72: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v71).as_ref().clone()));
                let v73: Rc<str> = Rc::<str>::from((*v72.borrow()).join(&*v64));
                v73
            }
            US6::US6_WordsRoot => {
                v2.clone()
            }
        };
        let v76: bool = std::fs::write(&*v4, &*v75).is_ok();
        if v76 {
            let v77: i32 = 40i32;
            let v78: bool = renamed_within_19(v0.clone(), v4.clone(), v77);
            if v78 {
                let v79: Rc<str> = Rc::<str>::from(v0.replace(&*v5, &*v6));
                let mut v80: Rc<str> = loop_9(v79);
                let v81: Option<usize> = v80.rfind(&*v6);
                let v82: Option<usize> = method10(v81);
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
                let v111: US1 = if v90 {
                    US1::US1_OtherFile
                } else {
                    let v92: usize = ((0i32) as usize);
                    let v93: usize = ((v85) as usize);
                    let v94: Rc<str> = Rc::<str>::from(&(v80)[v92..v93]);
                    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v96: bool = v94.ends_with(&*v95);
                    if v96 {
                        let v97: Rc<str> = loop_11(v94);
                        US1::US1_StateFile(v97)
                    } else {
                        let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v100: bool = v94.ends_with(&*v99);
                        if v100 {
                            let v101: Rc<str> = loop_12(v94);
                            US1::US1_HistoryFile(v101)
                        } else {
                            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v104: bool = v94.ends_with(&*v103);
                            if v104 {
                                let v105: Rc<str> = loop_13(v94);
                                US1::US1_ArchiveFile(v105)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                let v134: US6 = match v111 {
                    US1::US1_ArchiveFile(v124) => {
                        let mut v124: Rc<str> = v124;
                        let v125: Rc<str> = string_concat(&[&*v124, "/state/spiral.toml"]);
                        let v126: bool = std::path::Path::new(&*v125).is_file();
                        if v126 {
                            US6::US6_SymbolsRoot
                        } else {
                            US6::US6_WordsRoot
                        }
                    }
                    US1::US1_HistoryFile(v118) => {
                        let v118: Rc<str> = v118;
                        let v119: Rc<str> = string_concat(&[&*v118, "/state/spiral.toml"]);
                        let v120: bool = std::path::Path::new(&*v119).is_file();
                        if v120 {
                            US6::US6_SymbolsRoot
                        } else {
                            US6::US6_WordsRoot
                        }
                    }
                    US1::US1_OtherFile => {
                        US6::US6_WordsRoot
                    }
                    US1::US1_StateFile(v112) => {
                        let v112: Rc<str> = v112;
                        let v113: Rc<str> = string_concat(&[&*v112, "/state/spiral.toml"]);
                        let v114: bool = std::path::Path::new(&*v113).is_file();
                        if v114 {
                            US6::US6_SymbolsRoot
                        } else {
                            US6::US6_WordsRoot
                        }
                    }
                };
                let v146: Rc<str> = match v134 {
                    US6::US6_SymbolsRoot => {
                        let mut v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v136: Rc<UH0> = split_list_2(v135.clone(), v2);
                        let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v138: Rc<UH0> = method18(v136, v137);
                        let v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v140: i32 = 0i32;
                        let v141: i32 = method15(v139.clone(), v138, v140);
                        let v142: Rc<Vec<Rc<str>>> = Rc::new(v139.borrow().clone());
                        let v143: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v142).as_ref().clone()));
                        let v144: Rc<str> = Rc::<str>::from((*v143.borrow()).join(&*v135));
                        v144
                    }
                    US6::US6_WordsRoot => {
                        v2
                    }
                };
                let v147: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v146);
                if v147 {
                    US5::US5_Replaced
                } else {
                    US5::US5_ReadbackMismatch
                }
            } else {
                let v151: bool = std::fs::remove_file(&*v4).is_ok() || !std::path::Path::new(&*v4).exists();
                if v151 {
                    US5::US5_RenameFailed
                } else {
                    US5::US5_RenameFailedTemporaryKept
                }
            }
        } else {
            US5::US5_TemporaryWriteFailed
        }
    } else {
        US5::US5_ChangedSinceRead
    }
}
fn gap_filed_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> i32 {
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v8));
    let v10: Rc<str> = string_concat(&[&*v9, "/state/agile.spi"]);
    let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v10).unwrap_or_default().as_str());
    let mut v12: bool = &*v11 == "";
    if v12 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing or empty"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = string_concat(&["agile gap rejected: ", &*v13]);
        let v15: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v14 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v15
    } else {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<UH0> = split_list_2(v16.clone(), v2.clone());
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<UH0> = split_list_2(v18.clone(), v6.clone());
        let v24: Rc<str> = match Rc::unwrap_or_clone(v19) {
            UH0::UH0_Cons(v20, v21) => {
                let v20: Rc<str> = v20;
                let v21: Rc<UH0> = v21;
                v20
            }
            UH0::UH0_Nil => {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v22
            }
        };
        let v25: Rc<str> = Rc::<str>::from(v24.to_uppercase());
        let v27: Rc<str> = upper_words_5(v17);
        let v28: Rc<str> = string_concat(&[&*v25, "-GAP-", &*v27]);
        let v29: i32 = 1i32;
        let v30: Rc<str> = free_id_7(v11, v28, v29);
        let v31: Rc<UH0> = split_list_2(v16, v2.clone());
        let v37: Rc<str> = match Rc::unwrap_or_clone(v31) {
            UH0::UH0_Cons(v32, v33) => {
                let v32: Rc<str> = v32;
                let v33: Rc<UH0> = v33;
                trim_dashes_6(v32)
            }
            UH0::UH0_Nil => {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v35
            }
        };
        let v39: Rc<str> = string_concat(&["rig,gap,", &*v37, ",team:"]);
        let v40: Rc<UH0> = split_list_2(v18.clone(), v6.clone());
        let v45: Rc<str> = match Rc::unwrap_or_clone(v40) {
            UH0::UH0_Cons(v41, v42) => {
                let v41: Rc<str> = v41;
                let v42: Rc<UH0> = v42;
                v41
            }
            UH0::UH0_Nil => {
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v43
            }
        };
        let v46: Rc<str> = string_concat(&[&*v39, &*v45]);
        let v48: Rc<str> = string_concat(&["FAILED CALL: verb=", &*v2, " exit="]);
        let v49: bool = &*v3 == "";
        let v50: Rc<str> = if v49 {
            v18.clone()
        } else {
            v3
        };
        let v52: Rc<str> = string_concat(&[&*v48, &*v50, " cwd="]);
        let v53: bool = &*v5 == "";
        let v54: Rc<str> = if v53 {
            v18.clone()
        } else {
            v5
        };
        let v56: Rc<str> = string_concat(&[&*v52, &*v54, " argv="]);
        let v57: bool = &*v4 == "";
        let v58: Rc<str> = if v57 {
            v18.clone()
        } else {
            v4
        };
        let v62: Rc<str> = string_concat(&[&*v56, &*v58, ". DoD: the call works in one step, or refuses with intent, usage, valid values and a corrected example. Next step: reproduce the call, fix it in the lane that hit it, add a spec/one-call case. TypeFeatures: none yet."]);
        let v63: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v66: Rc<str> = concat5_8(v0, v64, v65.clone(), v65.clone(), v65.clone());
        let v67: bool = std::path::Path::new(&*v66).is_file();
        let v70: US0 = if v67 {
            US0::US0_FileReadable(v66.clone())
        } else {
            US0::US0_FileMissing(v66.clone())
        };
        match v70 {
            US0::US0_FileMissing(v71) => {
                let v71: Rc<str> = v71;
                let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v75: Rc<str> = concat5_8(v73, v30, v74, v72, v65);
                let v76: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v75 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v76
            }
            US0::US0_FileReadable(v77) => {
                let v77: Rc<str> = v77;
                let v78: Rc<str> = std::fs::read_to_string(&*v77).unwrap_or_default().into();
                let v79: Rc<str> = Rc::<str>::from(v77.replace(&*v7, &*v8));
                let mut v80: Rc<str> = loop_9(v79);
                let v81: Option<usize> = v80.rfind(&*v8);
                let v82: Option<usize> = method10(v81);
                let mut v83: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v84: Option<i32> = v82.map(|x| v83(x));
                let v86: i32 = v84.unwrap_or(-1i32);
                let v87: bool = v86 < 0i32;
                let v91: bool = if v87 {
                    true
                } else {
                    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v89: bool = v80.ends_with(&*v88);
                    let v90: bool = v89 == false;
                    v90
                };
                let v112: US1 = if v91 {
                    US1::US1_OtherFile
                } else {
                    let v93: usize = ((0i32) as usize);
                    let v94: usize = ((v86) as usize);
                    let v95: Rc<str> = Rc::<str>::from(&(v80)[v93..v94]);
                    let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v97: bool = v95.ends_with(&*v96);
                    if v97 {
                        let v98: Rc<str> = loop_11(v95);
                        US1::US1_StateFile(v98)
                    } else {
                        let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v101: bool = v95.ends_with(&*v100);
                        if v101 {
                            let v102: Rc<str> = loop_12(v95);
                            US1::US1_HistoryFile(v102)
                        } else {
                            let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v105: bool = v95.ends_with(&*v104);
                            if v105 {
                                let v106: Rc<str> = loop_13(v95);
                                US1::US1_ArchiveFile(v106)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                let v123: Rc<str> = match v112 {
                    US1::US1_OtherFile => {
                        v78
                    }
                    _ => {
                        let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v114: Rc<UH0> = split_list_2(v113.clone(), v78);
                        let v115: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v116: Rc<UH0> = method14(v114, v115);
                        let v117: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v118: i32 = 0i32;
                        let v119: i32 = method15(v117.clone(), v116, v118);
                        let v120: Rc<Vec<Rc<str>>> = Rc::new(v117.borrow().clone());
                        let v121: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v120).as_ref().clone()));
                        let v122: Rc<str> = Rc::<str>::from((*v121.borrow()).join(&*v113));
                        v122
                    }
                };
                let mut v124: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v123);
                let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                let v127: Rc<str> = concat5_8(v125, v30.clone(), v126, v65.clone(), v65.clone());
                let v128: bool = v124.contains(&*v127);
                if v128 {
                    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task with this id already exists"); } LIT.with(|lit| lit.clone()) };
                    let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                    let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let v132: Rc<str> = concat5_8(v130, v30, v131, v129, v65);
                    let v133: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v132 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v133
                } else {
                    let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v135: Rc<str> = Rc::<str>::from(v30.replace(&*v18, &*v134));
                    let v136: Rc<str> = std::rc::Rc::<str>::from(v135.to_ascii_lowercase());
                    let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                    let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                    let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                    let v140: Rc<str> = agile_task_creation_text::rig_agile_task_line(&v30, &v136, &v137, &v138, &v139, &v1, &v46, &v65);
                    let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v142: Rc<str> = std::rc::Rc::<str>::from(v124.trim_end_matches(&*v141));
                    let v145: Rc<str> = string_concat(&[&*v142, "\n", &*v140, "\n"]);
                    let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("30"); } LIT.with(|lit| lit.clone()) };
                    let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("light"); } LIT.with(|lit| lit.clone()) };
                    let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agent"); } LIT.with(|lit| lit.clone()) };
                    let v149: US4 = extras_applied_16(v145, v30.clone(), v146.clone(), v147.clone(), v148.clone(), &v65, v62.clone(), v6);
                    match v149 {
                        US4::US4_AddedRefused(v150) => {
                            let v150: Rc<str> = v150;
                            let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                            let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                            let v153: Rc<str> = concat5_8(v151, v30, v152, v150, v65);
                            let v154: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v153 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v154
                        }
                        US4::US4_AddedText(v155) => {
                            let v155: Rc<str> = v155;
                            let v156: US5 = replace_records_unchanged_17(v66, v124.clone(), v155.clone());
                            match &v156 {
                                US5::US5_Replaced => {
                                    let mut v157: Rc<str> = Rc::<str>::from(v30.replace(&*v18, &*v134));
                                    let v158: Rc<str> = std::rc::Rc::<str>::from(v157.to_ascii_lowercase());
                                    let v159: u64 = v124.len() as u64;
                                    let v160: u64 = v155.len() as u64;
                                    let v161: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v63);
                                    let v162: Rc<str> = agile_task_creation_text::rig_agile_add_receipt(&v30, &v158, &v146, &v147, &v148, &v65, &v62);
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
                                        US5::US5_ChangedSinceRead => {
                                            let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                            v177
                                        }
                                        US5::US5_ReadbackMismatch => {
                                            let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                            v176
                                        }
                                        US5::US5_RenameFailed => {
                                            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                            v174
                                        }
                                        US5::US5_RenameFailedTemporaryKept => {
                                            let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                            v175
                                        }
                                        US5::US5_TemporaryWriteFailed => {
                                            let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                            v173
                                        }
                                        _ => unreachable!(),
                                    };
                                    let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                    let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                    let v185: Rc<str> = concat5_8(v183, v30, v184, v182, v65);
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
fn command_line_argument_count_21() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_22(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_20(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_21();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US7 = if v4 {
        let v5: Rc<str> = command_line_argument_22(v0);
        US7::US7_Some(v5)
    } else {
        US7::US7_None
    };
    match v8 {
        US7::US7_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US7::US7_Some(v10) => {
            let v10: Rc<str> = v10;
            let mut v11: i32 = v0.wrapping_add(1i32);
            let mut v12: Rc<UH0> = arguments_from_20(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn flag_of_word_24(mut v0: Rc<str>) -> US9 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Verb"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(v1.to_lowercase());
    let v3: Rc<str> = string_concat(&["--", &*v2]);
    let v4: bool = &*v0 == &*v3;
    if v4 {
        let v5: US10 = US10::US10_FlagVerb;
        US9::US9_FlagKnown(v5)
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Exit"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
        let mut v9: Rc<str> = string_concat(&["--", &*v8]);
        let v10: bool = &*v0 == &*v9;
        if v10 {
            let mut v11: US10 = US10::US10_FlagExit;
            US9::US9_FlagKnown(v11)
        } else {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Argv"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = Rc::<str>::from(v13.to_lowercase());
            let v15: Rc<str> = string_concat(&["--", &*v14]);
            let v16: bool = &*v0 == &*v15;
            if v16 {
                let v17: US10 = US10::US10_FlagArgv;
                US9::US9_FlagKnown(v17)
            } else {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Cwd"); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = Rc::<str>::from(v19.to_lowercase());
                let v21: Rc<str> = string_concat(&["--", &*v20]);
                let v22: bool = &*v0 == &*v21;
                if v22 {
                    let v23: US10 = US10::US10_FlagCwd;
                    US9::US9_FlagKnown(v23)
                } else {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = Rc::<str>::from(v25.to_lowercase());
                    let v27: Rc<str> = string_concat(&["--", &*v26]);
                    let v28: bool = &*v0 == &*v27;
                    if v28 {
                        let v29: US10 = US10::US10_FlagAs;
                        US9::US9_FlagKnown(v29)
                    } else {
                        US9::US9_FlagUnknown
                    }
                }
            }
        }
    }
}
fn request_from_23(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> US8 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v8, v9) => {
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<UH0> = v9;
                let v10: US9 = flag_of_word_24(v8.clone());
                match v10 {
                    US9::US9_FlagKnown(v41) => {
                        let v41: US10 = v41;
                        match Rc::unwrap_or_clone(v9) {
                            UH0::UH0_Cons(v44, v45) => {
                                let v44: Rc<str> = v44;
                                let v45: Rc<UH0> = v45;
                                let v46: Rc<str> = Rc::<str>::from((v44).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v41 {
                                    US10::US10_FlagVerb => {
                                        (v1, v46, v3, v4, v5, v6)
                                    }
                                    _ => {
                                        let (mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v41 {
                                            US10::US10_FlagExit => {
                                                (v1, v2, v46, v4, v5, v6)
                                            }
                                            _ => {
                                                let (mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v41 {
                                                    US10::US10_FlagArgv => {
                                                        (v1, v2, v3, v46, v5, v6)
                                                    }
                                                    _ => {
                                                        let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v41 {
                                                            US10::US10_FlagCwd => {
                                                                (v1, v2, v3, v4, v46, v6)
                                                            }
                                                            _ => {
                                                                let (mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v41 {
                                                                    US10::US10_FlagAs => {
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
                                return US8::US8_RequestRefused(v42);
                            }
                        }
                    }
                    US9::US9_FlagUnknown => {
                        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Verb"); } LIT.with(|lit| lit.clone()) };
                        let v14: Rc<str> = Rc::<str>::from(v13.to_lowercase());
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Exit"); } LIT.with(|lit| lit.clone()) };
                        let v19: Rc<str> = Rc::<str>::from(v18.to_lowercase());
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Argv"); } LIT.with(|lit| lit.clone()) };
                        let v24: Rc<str> = Rc::<str>::from(v23.to_lowercase());
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Cwd"); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = Rc::<str>::from(v28.to_lowercase());
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                        let v34: Rc<str> = Rc::<str>::from(v33.to_lowercase());
                        let v39: Rc<str> = string_concat(&["unknown option ", &*v8, " (expected --", &*v14, ", --", &*v19, ", --", &*v24, ", --", &*v29, ", --", &*v34, ")"]);
                        return US8::US8_RequestRefused(v39);
                    }
                }
            }
            UH0::UH0_Nil => {
                return US8::US8_RequestRead(v1, v2, v3, v4, v5, v6);
            }
        }
    }
}
fn bytes_within_26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v1.clone().len() as i32);
        let v4: bool = v3 <= v2;
        if v4 {
            return true;
        } else {
            let v5: usize = ((v2) as usize);
            let v6: i32 = v2.wrapping_add(1i32);
            let v7: usize = ((v6) as usize);
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
fn request_checked_25(mut v0: US8) -> US8 {
    match &v0 {
        US8::US8_RequestRead(v2, v3, v4, v5, v6, v7) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<str> = v3.clone();
            let v4: Rc<str> = v4.clone();
            let v5: Rc<str> = v5.clone();
            let v6: Rc<str> = v6.clone();
            let v7: Rc<str> = v7.clone();
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<UH0> = split_list_2(v8, v7);
            let v14: Rc<str> = match Rc::unwrap_or_clone(v9) {
                UH0::UH0_Cons(v10, v11) => {
                    let v10: Rc<str> = v10;
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
            let v17: bool = bytes_within_26(v15, v3.clone(), v16);
            let v18: bool = &*v14 != "";
            let v22: bool = if v18 {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyz0123456789"); } LIT.with(|lit| lit.clone()) };
                let v20: i32 = 0i32;
                bytes_within_26(v19, v14, v20)
            } else {
                false
            };
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) };
            let v24: i32 = 0i32;
            let v25: bool = bytes_within_26(v23, v4, v24);
            let v26: bool = &*v2 == "";
            if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a gap needs its title: what the call needed and did not get"); } LIT.with(|lit| lit.clone()) };
                US8::US8_RequestRefused(v27)
            } else {
                let v29: bool = &*v3 == "";
                if v29 {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--verb names the rig verb that failed or was missing, e.g. --verb 'job chain'"); } LIT.with(|lit| lit.clone()) };
                    US8::US8_RequestRefused(v30)
                } else {
                    let v32: bool = v17 == false;
                    if v32 {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--verb takes lower-case words, digits and dashes, e.g. --verb 'help agile add'"); } LIT.with(|lit| lit.clone()) };
                        US8::US8_RequestRefused(v33)
                    } else {
                        let v35: bool = v22 == false;
                        if v35 {
                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as <lane> names who hit the gap; its first word is the team, e.g. --as rig-fc1568a6"); } LIT.with(|lit| lit.clone()) };
                            US8::US8_RequestRefused(v36)
                        } else {
                            let v38: bool = v25 == false;
                            if v38 {
                                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--exit takes the failed call's exit code as digits, e.g. --exit 2"); } LIT.with(|lit| lit.clone()) };
                                US8::US8_RequestRefused(v39)
                            } else {
                                v0
                            }
                        }
                    }
                }
            }
        }
        US8::US8_RequestRefused(v1) => {
            let v1: Rc<str> = v1.clone();
            v0
        }
    }
}
fn gap_entry_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_20(v0);
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
                            let v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH0::UH0_Cons(v8, v9) => {
                                    let mut v8: Rc<str> = v8;
                                    let mut v9: Rc<UH0> = v9;
                                    let v10: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v12: US8 = request_from_23(v9, v10, v11.clone(), v11.clone(), v11.clone(), v11.clone(), v11.clone());
                                    let v13: US8 = request_checked_25(v12);
                                    match v13 {
                                        US8::US8_RequestRead(v17, v18, v19, v20, v21, v22) => {
                                            let v17: Rc<str> = v17;
                                            let v18: Rc<str> = v18;
                                            let v19: Rc<str> = v19;
                                            let v20: Rc<str> = v20;
                                            let v21: Rc<str> = v21;
                                            let v22: Rc<str> = v22;
                                            gap_filed_1(v6, v17, v18, v19, v20, v21, v22)
                                        }
                                        US8::US8_RequestRefused(v14) => {
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
