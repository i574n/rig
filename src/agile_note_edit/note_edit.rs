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
    US0_ReplaceText,
    US0_AppendText,
}
#[derive(Clone)]
enum US1 {
    US1_NoGuard,
    US1_ExpectedAt(Rc<str>),
    US1_ExpectedPrefix(Rc<str>),
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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
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
    US6_ScanPlain,
    US6_ScanQuoted,
    US6_ScanEscaped,
}
#[derive(Clone)]
enum US7 {
    US7_StateWritten,
    US7_StateChangedSinceRead,
    US7_StateWriteFailed(Rc<str>),
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
    US10_Some(i32),
    US10_None,
}
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn loop_3(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
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
fn method4(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
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
fn loop_6(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
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
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
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
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method9(v16, v17)
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method11(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let v15: US4 = if v6 {
                let v7: US5 = US5::US5_HeadType;
                US4::US4_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
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
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
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
                    let mut v29: bool = v2.starts_with(&*v28);
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
                    let mut v74: Rc<str> = Rc::<str>::from(&(v2)[v71..v73]);
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
fn method12(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn field_end_15(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US6) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US6::US6_ScanPlain => {
                    let v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let v10: i32 = v1.wrapping_add(1i32);
                let v25: US6 = match v3 {
                    US6::US6_ScanEscaped => {
                        US6::US6_ScanQuoted
                    }
                    US6::US6_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US6::US6_ScanQuoted
                        } else {
                            US6::US6_ScanPlain
                        }
                    }
                    US6::US6_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US6::US6_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US6::US6_ScanPlain
                            } else {
                                US6::US6_ScanQuoted
                            }
                        }
                    }
                };
                (v1, v3) = (v10, v25);
                continue;
            }
        }
    }
}
fn skip_spaces_16(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_14(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US6 = US6::US6_ScanPlain;
            let v6: i32 = field_end_15(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_16(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn unescaped_from_18(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 <= v1;
        if v5 {
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return concat5_2(v4, v6, v7.clone(), v7.clone(), v7.clone());
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v10: bool = v9 == 92i32;
            if v10 {
                let mut v11: i32 = v1.wrapping_add(1i32);
                let mut v12: i32 = v11.wrapping_add(1i32);
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v15: bool = v14 == 110i32;
                let v24: Rc<str> = if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16
                } else {
                    let v17: bool = v14 == 114i32;
                    if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18
                    } else {
                        let v19: bool = v14 == 116i32;
                        if v19 {
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20
                        } else {
                            let mut v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21
                        }
                    }
                };
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = concat5_2(v4, v13, v24, v25.clone(), v25.clone());
                (v1, v2, v4) = (v12, v12, v26);
                continue;
            } else {
                let v28: i32 = v1.wrapping_add(1i32);
                v1 = v28;
                continue;
            }
        }
    }
}
fn unquoted_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_18(v3, v4, v5, v6, v7)
}
fn note_at_13(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1.wrapping_add(6i32);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = 2i32;
    let v5: i32 = field_start_14(v0.clone(), v2, v3, v4);
    let v6: US6 = US6::US6_ScanPlain;
    let v7: i32 = field_end_15(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_17(v8);
    let v10: i32 = 3i32;
    let mut v11: i32 = field_start_14(v0.clone(), v2, v3, v10);
    let mut v12: US6 = US6::US6_ScanPlain;
    let mut v13: i32 = field_end_15(v0.clone(), v11, v3, v12);
    let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v13 as usize).unwrap_or(""));
    let v15: Rc<str> = unquoted_17(v14);
    let v16: i32 = 4i32;
    let v17: i32 = field_start_14(v0.clone(), v2, v3, v16);
    let v18: US6 = US6::US6_ScanPlain;
    let v19: i32 = field_end_15(v0.clone(), v17, v3, v18);
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let mut v21: Rc<str> = unquoted_17(v20);
    (v9, v15, v21)
}
fn note_row_19(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v8));
    let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = concat5_2(v6, v10, v8, v11, v12.clone());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note ("); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(v0.replace(&*v16, &*v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = Rc::<str>::from(v18.replace(&*v15, &*v19));
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v22));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = Rc::<str>::from(v23.replace(&*v24, &*v25));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = Rc::<str>::from(v26.replace(&*v27, &*v28));
    let mut v30: Rc<str> = concat5_2(v15.clone(), v29, v15.clone(), v12.clone(), v12.clone());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = concat5_2(v5, v13, v14, v30, v31.clone());
    let v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = Rc::<str>::from(v2.replace(&*v16, &*v17));
    let v36: Rc<str> = Rc::<str>::from(v35.replace(&*v15, &*v19));
    let v37: Rc<str> = Rc::<str>::from(v36.replace(&*v21, &*v22));
    let v38: Rc<str> = Rc::<str>::from(v37.replace(&*v24, &*v25));
    let v39: Rc<str> = Rc::<str>::from(v38.replace(&*v27, &*v28));
    let v40: Rc<str> = concat5_2(v15.clone(), v39, v15.clone(), v12.clone(), v12.clone());
    let v41: Rc<str> = Rc::<str>::from(v3.replace(&*v16, &*v17));
    let v42: Rc<str> = Rc::<str>::from(v41.replace(&*v15, &*v19));
    let v43: Rc<str> = Rc::<str>::from(v42.replace(&*v21, &*v22));
    let v44: Rc<str> = Rc::<str>::from(v43.replace(&*v24, &*v25));
    let v45: Rc<str> = Rc::<str>::from(v44.replace(&*v27, &*v28));
    let v46: Rc<str> = concat5_2(v15.clone(), v45, v15.clone(), v12.clone(), v12.clone());
    let v47: Rc<str> = concat5_2(v33, v34, v40, v31.clone(), v46);
    let v48: Rc<str> = Rc::<str>::from(v4.replace(&*v16, &*v17));
    let v49: Rc<str> = Rc::<str>::from(v48.replace(&*v15, &*v19));
    let v50: Rc<str> = Rc::<str>::from(v49.replace(&*v21, &*v22));
    let v51: Rc<str> = Rc::<str>::from(v50.replace(&*v24, &*v25));
    let v52: Rc<str> = Rc::<str>::from(v51.replace(&*v27, &*v28));
    let v53: Rc<str> = concat5_2(v15.clone(), v52, v15, v12.clone(), v12);
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    concat5_2(v32, v47, v31, v53, v54)
}
fn method21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method21(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let v15: US4 = if v6 {
                let v7: US5 = US5::US5_HeadType;
                US4::US4_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
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
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
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
                    let mut v29: bool = v2.starts_with(&*v28);
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
                    let mut v74: Rc<str> = Rc::<str>::from(&(v2)[v71..v73]);
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
fn renamed_within_22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn state_written_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US7 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_2(v0.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    std::env::var("RIG_AGILE_TEST_PAUSE_MS").or_else(|_| std::env::var("KIT_AGILE_TEST_PAUSE_MS")).ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
    let mut v9: Rc<str> = loop_3(v8);
    let v10: Option<usize> = v9.rfind(&*v7);
    let mut v11: Option<usize> = method4(v10);
    let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v13: Option<i32> = v11.map(|x| v12(x));
    let v15: i32 = v13.unwrap_or(-1i32);
    let v16: bool = v15 < 0i32;
    let mut v20: bool = if v16 {
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
        let mut v26: bool = v24.ends_with(&*v25);
        if v26 {
            let v27: Rc<str> = loop_5(v24);
            US3::US3_StateFile(v27)
        } else {
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
            let mut v30: bool = v24.ends_with(&*v29);
            if v30 {
                let v31: Rc<str> = loop_6(v24);
                US3::US3_HistoryFile(v31)
            } else {
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                let v34: bool = v24.ends_with(&*v33);
                if v34 {
                    let v35: Rc<str> = loop_7(v24);
                    US3::US3_ArchiveFile(v35)
                } else {
                    US3::US3_OtherFile
                }
            }
        }
    };
    let v64: US8 = match v41 {
        US3::US3_ArchiveFile(v54) => {
            let v54: Rc<str> = v54;
            let v55: Rc<str> = string_concat(&[&*v54, "/state/spiral.toml"]);
            let v56: bool = std::path::Path::new(&*v55).is_file();
            if v56 {
                US8::US8_SymbolsRoot
            } else {
                US8::US8_WordsRoot
            }
        }
        US3::US3_HistoryFile(v48) => {
            let v48: Rc<str> = v48;
            let v49: Rc<str> = string_concat(&[&*v48, "/state/spiral.toml"]);
            let v50: bool = std::path::Path::new(&*v49).is_file();
            if v50 {
                US8::US8_SymbolsRoot
            } else {
                US8::US8_WordsRoot
            }
        }
        US3::US3_OtherFile => {
            US8::US8_WordsRoot
        }
        US3::US3_StateFile(v42) => {
            let v42: Rc<str> = v42;
            let v43: Rc<str> = string_concat(&[&*v42, "/state/spiral.toml"]);
            let v44: bool = std::path::Path::new(&*v43).is_file();
            if v44 {
                US8::US8_SymbolsRoot
            } else {
                US8::US8_WordsRoot
            }
        }
    };
    let v76: Rc<str> = match v64 {
        US8::US8_SymbolsRoot => {
            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v66: Rc<UH0> = split_list_8(v65.clone(), v1);
            let v67: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v68: Rc<UH0> = method21(v66, v67);
            let v69: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v70: i32 = 0i32;
            let v71: i32 = method12(v69.clone(), v68, v70);
            let v72: Rc<Vec<Rc<str>>> = Rc::new(v69.borrow().clone());
            let v73: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v72).as_ref().clone()));
            let mut v74: Rc<str> = Rc::<str>::from((*v73.borrow()).join(&*v65));
            v74
        }
        US8::US8_WordsRoot => {
            v1
        }
    };
    let v77: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v76);
    if v77 {
        let v78: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let v79: Rc<str> = Rc::<str>::from(v78.replace(&*v6, &*v7));
        let mut v80: Rc<str> = loop_3(v79);
        let v81: Option<usize> = v80.rfind(&*v7);
        let v82: Option<usize> = method4(v81);
        let mut v83: Option<i32> = v82.map(|x| v12(x));
        let v85: i32 = v83.unwrap_or(-1i32);
        let v86: bool = v85 < 0i32;
        let v90: bool = if v86 {
            true
        } else {
            let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
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
                let v97: Rc<str> = loop_5(v94);
                US3::US3_StateFile(v97)
            } else {
                let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v100: bool = v94.ends_with(&*v99);
                if v100 {
                    let v101: Rc<str> = loop_6(v94);
                    US3::US3_HistoryFile(v101)
                } else {
                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v104: bool = v94.ends_with(&*v103);
                    if v104 {
                        let v105: Rc<str> = loop_7(v94);
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
                let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v136: Rc<UH0> = split_list_8(v135.clone(), v2.clone());
                let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v138: Rc<UH0> = method21(v136, v137);
                let v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v140: i32 = 0i32;
                let v141: i32 = method12(v139.clone(), v138, v140);
                let v142: Rc<Vec<Rc<str>>> = Rc::new(v139.borrow().clone());
                let v143: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v142).as_ref().clone()));
                let v144: Rc<str> = Rc::<str>::from((*v143.borrow()).join(&*v135));
                v144
            }
            US8::US8_WordsRoot => {
                v2.clone()
            }
        };
        let v147: bool = std::fs::write(&*v78, &*v146).is_ok();
        let v228: US9 = if v147 {
            let v148: i32 = 40i32;
            let v149: bool = renamed_within_22(v0.clone(), v78.clone(), v148);
            if v149 {
                let v150: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
                let mut v151: Rc<str> = loop_3(v150);
                let v152: Option<usize> = v151.rfind(&*v7);
                let v153: Option<usize> = method4(v152);
                let mut v154: Option<i32> = v153.map(|x| v12(x));
                let v156: i32 = v154.unwrap_or(-1i32);
                let v157: bool = v156 < 0i32;
                let v161: bool = if v157 {
                    true
                } else {
                    let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v159: bool = v151.ends_with(&*v158);
                    let v160: bool = v159 == false;
                    v160
                };
                let v182: US3 = if v161 {
                    US3::US3_OtherFile
                } else {
                    let v163: usize = ((0i32) as usize);
                    let v164: usize = ((v156) as usize);
                    let v165: Rc<str> = Rc::<str>::from(&(v151)[v163..v164]);
                    let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v167: bool = v165.ends_with(&*v166);
                    if v167 {
                        let v168: Rc<str> = loop_5(v165);
                        US3::US3_StateFile(v168)
                    } else {
                        let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v171: bool = v165.ends_with(&*v170);
                        if v171 {
                            let v172: Rc<str> = loop_6(v165);
                            US3::US3_HistoryFile(v172)
                        } else {
                            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v175: bool = v165.ends_with(&*v174);
                            if v175 {
                                let v176: Rc<str> = loop_7(v165);
                                US3::US3_ArchiveFile(v176)
                            } else {
                                US3::US3_OtherFile
                            }
                        }
                    }
                };
                let v205: US8 = match v182 {
                    US3::US3_ArchiveFile(v195) => {
                        let v195: Rc<str> = v195;
                        let v196: Rc<str> = string_concat(&[&*v195, "/state/spiral.toml"]);
                        let v197: bool = std::path::Path::new(&*v196).is_file();
                        if v197 {
                            US8::US8_SymbolsRoot
                        } else {
                            US8::US8_WordsRoot
                        }
                    }
                    US3::US3_HistoryFile(v189) => {
                        let v189: Rc<str> = v189;
                        let v190: Rc<str> = string_concat(&[&*v189, "/state/spiral.toml"]);
                        let v191: bool = std::path::Path::new(&*v190).is_file();
                        if v191 {
                            US8::US8_SymbolsRoot
                        } else {
                            US8::US8_WordsRoot
                        }
                    }
                    US3::US3_OtherFile => {
                        US8::US8_WordsRoot
                    }
                    US3::US3_StateFile(v183) => {
                        let v183: Rc<str> = v183;
                        let v184: Rc<str> = string_concat(&[&*v183, "/state/spiral.toml"]);
                        let v185: bool = std::path::Path::new(&*v184).is_file();
                        if v185 {
                            US8::US8_SymbolsRoot
                        } else {
                            US8::US8_WordsRoot
                        }
                    }
                };
                let v217: Rc<str> = match v205 {
                    US8::US8_SymbolsRoot => {
                        let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v207: Rc<UH0> = split_list_8(v206.clone(), v2);
                        let v208: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v209: Rc<UH0> = method21(v207, v208);
                        let v210: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v211: i32 = 0i32;
                        let v212: i32 = method12(v210.clone(), v209, v211);
                        let v213: Rc<Vec<Rc<str>>> = Rc::new(v210.borrow().clone());
                        let v214: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v213).as_ref().clone()));
                        let v215: Rc<str> = Rc::<str>::from((*v214.borrow()).join(&*v206));
                        v215
                    }
                    US8::US8_WordsRoot => {
                        v2
                    }
                };
                let v218: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v217);
                if v218 {
                    US9::US9_Replaced
                } else {
                    US9::US9_ReadbackMismatch
                }
            } else {
                let v222: bool = std::fs::remove_file(&*v78).is_ok() || !std::path::Path::new(&*v78).exists();
                if v222 {
                    US9::US9_RenameFailed
                } else {
                    US9::US9_RenameFailedTemporaryKept
                }
            }
        } else {
            US9::US9_TemporaryWriteFailed
        };
        match &v228 {
            US9::US9_Replaced => {
                US7::US7_StateWritten
            }
            _ => {
                let v239: Rc<str> = match v228 {
                    US9::US9_ChangedSinceRead => {
                        let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v234
                    }
                    US9::US9_ReadbackMismatch => {
                        let v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v233
                    }
                    US9::US9_RenameFailed => {
                        let v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v231
                    }
                    US9::US9_RenameFailedTemporaryKept => {
                        let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v232
                    }
                    US9::US9_TemporaryWriteFailed => {
                        let v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v230
                    }
                    _ => unreachable!(),
                };
                US7::US7_StateWriteFailed(v239)
            }
        }
    } else {
        US7::US7_StateChangedSinceRead
    }
}
fn note_edited_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US0, mut v6: US1) -> i32 {
    let v7: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = concat5_2(v0, v8, v9.clone(), v9.clone(), v9.clone());
    let mut v11: bool = std::path::Path::new(&*v10).is_file();
    let v14: US2 = if v11 {
        US2::US2_FileReadable(v10.clone())
    } else {
        US2::US2_FileMissing(v10.clone())
    };
    match v14 {
        US2::US2_FileMissing(v15) => {
            let v15: Rc<str> = v15;
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = string_concat(&["agile note-edit ", &*v1, " rejected: ", &*v16]);
            let mut v20: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v19 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v20
        }
        US2::US2_FileReadable(v21) => {
            let mut v21: Rc<str> = v21;
            let v22: Rc<str> = std::fs::read_to_string(&*v21).unwrap_or_default().into();
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = Rc::<str>::from(v21.replace(&*v23, &*v24));
            let mut v26: Rc<str> = loop_3(v25);
            let v27: Option<usize> = v26.rfind(&*v24);
            let v28: Option<usize> = method4(v27);
            let mut v29: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v30: Option<i32> = v28.map(|x| v29(x));
            let v32: i32 = v30.unwrap_or(-1i32);
            let v33: bool = v32 < 0i32;
            let v37: bool = if v33 {
                true
            } else {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v35: bool = v26.ends_with(&*v34);
                let v36: bool = v35 == false;
                v36
            };
            let v58: US3 = if v37 {
                US3::US3_OtherFile
            } else {
                let v39: usize = ((0i32) as usize);
                let v40: usize = ((v32) as usize);
                let v41: Rc<str> = Rc::<str>::from(&(v26)[v39..v40]);
                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v43: bool = v41.ends_with(&*v42);
                if v43 {
                    let v44: Rc<str> = loop_5(v41);
                    US3::US3_StateFile(v44)
                } else {
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v47: bool = v41.ends_with(&*v46);
                    if v47 {
                        let v48: Rc<str> = loop_6(v41);
                        US3::US3_HistoryFile(v48)
                    } else {
                        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v51: bool = v41.ends_with(&*v50);
                        if v51 {
                            let v52: Rc<str> = loop_7(v41);
                            US3::US3_ArchiveFile(v52)
                        } else {
                            US3::US3_OtherFile
                        }
                    }
                }
            };
            let v69: Rc<str> = match v58 {
                US3::US3_OtherFile => {
                    v22
                }
                _ => {
                    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v60: Rc<UH0> = split_list_8(v59.clone(), v22);
                    let v61: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v62: Rc<UH0> = method11(v60, v61);
                    let v63: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v64: i32 = 0i32;
                    let v65: i32 = method12(v63.clone(), v62, v64);
                    let v66: Rc<Vec<Rc<str>>> = Rc::new(v63.borrow().clone());
                    let v67: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v66).as_ref().clone()));
                    let v68: Rc<str> = Rc::<str>::from((*v67.borrow()).join(&*v59));
                    v68
                }
            };
            let v70: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v69);
            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
            let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let mut v74: Rc<str> = Rc::<str>::from(v1.replace(&*v72, &*v73));
            let v75: Rc<str> = std::rc::Rc::<str>::from(v74.to_ascii_lowercase());
            let v76: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let v77: Rc<str> = concat5_2(v71.clone(), v75, v73.clone(), v76, v9.clone());
            let v79: Rc<str> = string_concat(&["inl ", &*v77, " () = "]);
            let mut v80: i32 = v70.find(&*v79).map(|offset| offset as i32).unwrap_or(-1);
            let v81: bool = v80 < 0i32;
            if v81 {
                let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no note with this number"); } LIT.with(|lit| lit.clone()) };
                let v85: Rc<str> = string_concat(&["agile note-edit ", &*v1, " rejected: ", &*v82]);
                let v86: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v85 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v86
            } else {
                let mut v87: Rc<str> = Rc::<str>::from(v1.replace(&*v72, &*v73));
                let v88: Rc<str> = std::rc::Rc::<str>::from(v87.to_ascii_lowercase());
                let v89: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                let v90: Rc<str> = concat5_2(v71, v88, v73, v89, v9.clone());
                let v92: Rc<str> = string_concat(&["inl ", &*v90, " () = "]);
                let v93: i32 = (v92.len() as i32);
                let v94: i32 = v80.wrapping_add(v93);
                let (mut v95, mut v96, mut v97): (Rc<str>, Rc<str>, Rc<str>) = note_at_13(v70.clone(), v94);
                let v115: Rc<str> = match v6 {
                    US1::US1_ExpectedAt(v98) => {
                        let v98: Rc<str> = v98;
                        let v99: bool = &*v95 == &*v98;
                        if v99 {
                            v9.clone()
                        } else {
                            let v100: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                            let v105: Rc<str> = string_concat(&["note ", &*v100, " was written at ", &*v95, ", not ", &*v98]);
                            v105
                        }
                    }
                    US1::US1_ExpectedPrefix(v107) => {
                        let v107: Rc<str> = v107;
                        let v108: bool = v97.starts_with(&*v107);
                        if v108 {
                            v9.clone()
                        } else {
                            let v109: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                            let v111: Rc<str> = string_concat(&["note ", &*v109, " does not start with the expected text"]);
                            v111
                        }
                    }
                    US1::US1_NoGuard => {
                        v9.clone()
                    }
                };
                let v116: bool = &*v115 != "";
                if v116 {
                    let v119: Rc<str> = string_concat(&["agile note-edit ", &*v1, " rejected: ", &*v115]);
                    let v120: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v119 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v120
                } else {
                    let v121: bool = &*v4 == "";
                    let v123: bool = if v121 {
                        true
                    } else {
                        let v122: bool = &*v96 == &*v4;
                        v122
                    };
                    if v123 {
                        let v127: Rc<str> = match v5 {
                            US0::US0_AppendText => {
                                let v125: Rc<str> = string_concat(&[&*v97, " ", &*v3]);
                                v125
                            }
                            US0::US0_ReplaceText => {
                                v3
                            }
                        };
                        let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v129: i32 = v70.get(v80 as usize..).and_then(|rest| rest.find(&*v128)).map(|found| (found + v80 as usize) as i32).unwrap_or(-1);
                        let v130: bool = v129 < 0i32;
                        let v132: i32 = if v130 {
                            let v131: i32 = (v70.clone().len() as i32);
                            v131
                        } else {
                            v129
                        };
                        let v133: Rc<str> = note_row_19(v1.clone(), v2, v95.clone(), v96.clone(), v127.clone());
                        let v134: Rc<str> = std::rc::Rc::<str>::from(v70.get(0i32 as usize..v80 as usize).unwrap_or(""));
                        let v135: i32 = (v70.clone().len() as i32);
                        let v136: Rc<str> = std::rc::Rc::<str>::from(v70.get(v132 as usize..v135 as usize).unwrap_or(""));
                        let v137: Rc<str> = concat5_2(v134, v133, v136, v9.clone(), v9.clone());
                        let v138: US7 = state_written_20(v10, v69, v137.clone());
                        match &v138 {
                            US7::US7_StateWritten => {
                                let v139: u64 = v70.len() as u64;
                                let v140: u64 = v137.len() as u64;
                                let v141: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v7);
                                let v144: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                                let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                let v150: Rc<str> = Rc::<str>::from(v96.replace(&*v23, &*v149));
                                let mut v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                let v152: Rc<str> = Rc::<str>::from(v150.replace(&*v148, &*v151));
                                let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v154: Rc<str> = Rc::<str>::from(v152.replace(&*v128, &*v153));
                                let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                let v157: Rc<str> = Rc::<str>::from(v154.replace(&*v155, &*v156));
                                let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                let v160: Rc<str> = Rc::<str>::from(v157.replace(&*v158, &*v159));
                                let v161: Rc<str> = concat5_2(v148.clone(), v160, v148.clone(), v9.clone(), v9.clone());
                                let v164: i32 = (v97.clone().len() as i32);
                                let v165: Rc<str> = std::rc::Rc::<str>::from((v164).to_string());
                                let v169: i32 = (v127.len() as i32);
                                let v170: Rc<str> = std::rc::Rc::<str>::from((v169).to_string());
                                let v173: Rc<str> = { let text = &*v97; if text.chars().count() <= 80i32 as usize { std::rc::Rc::<str>::from(text) } else { std::rc::Rc::<str>::from(text.chars().take((80i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
                                let v174: Rc<str> = Rc::<str>::from(v173.replace(&*v23, &*v149));
                                let v175: Rc<str> = Rc::<str>::from(v174.replace(&*v148, &*v151));
                                let v176: Rc<str> = Rc::<str>::from(v175.replace(&*v128, &*v153));
                                let v177: Rc<str> = Rc::<str>::from(v176.replace(&*v155, &*v156));
                                let v178: Rc<str> = Rc::<str>::from(v177.replace(&*v158, &*v159));
                                let v179: Rc<str> = concat5_2(v148.clone(), v178, v148.clone(), v9.clone(), v9.clone());
                                let v181: Rc<str> = std::rc::Rc::<str>::from((v139).to_string());
                                let v184: Rc<str> = std::rc::Rc::<str>::from((v140).to_string());
                                let v187: Rc<str> = std::rc::Rc::<str>::from((v141).to_string());
                                let v189: Rc<str> = string_concat(&["rig agile note-edit ok id=", &*v1, " note=", &*v144, " at=", &*v95, " author=", &*v161, " text_bytes=", &*v165, "->", &*v170, " replaced=", &*v179, " bytes_before=", &*v181, " bytes_after=", &*v184, " elapsed_ms=", &*v187]);
                                let v190: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v189.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                v190
                            }
                            _ => {
                                let v194: Rc<str> = match v138 {
                                    US7::US7_StateChangedSinceRead => {
                                        let v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                        v191
                                    }
                                    US7::US7_StateWriteFailed(v192) => {
                                        let v192: Rc<str> = v192;
                                        v192
                                    }
                                    _ => unreachable!(),
                                };
                                let v197: Rc<str> = string_concat(&["agile note-edit ", &*v1, " rejected: ", &*v194]);
                                let v198: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v197 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v198
                            }
                        }
                    } else {
                        let v200: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                        let v207: Rc<str> = string_concat(&["agile note-edit ", &*v1, " rejected: note ", &*v200, " was written by ", &*v96, "; only its author edits it"]);
                        let v208: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v207 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v208
                    }
                }
            }
        }
    }
}
fn method23(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
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
fn method24(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
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
fn method25(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure2() -> Rc<dyn Fn((i32)) -> US10> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US10> = Rc::new(move |mut v0: (i32)| -> US10 {
        let mut v1: i32 = (v0);
        US10::US10_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_text_of_26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method23(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method24(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let mut v9: bool = &*v8 == "-";
    let mut v21: Rc<str> = if v9 {
        let v10: std::io::Stdin = std::io::stdin();
        let mut v11: Result<std::string::String, std::io::Error> = std::io::read_to_string(v10);
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v14: &str = &*v12;
        let v16: std::string::String = String::from(v14);
        let v18: std::string::String = v11.unwrap_or(v16);
        let mut v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
        v20
    } else {
        v0
    };
    let v22: i32 = (v21.clone().len() as i32);
    let v23: i32 = 0i32;
    let v24: i32 = method23(v21.clone(), v22, v23);
    let v25: i32 = v22.wrapping_sub(1i32);
    let mut v26: Rc<str> = string_slice(&v21, v24 as i64, v25 as i64);
    let v27: i32 = (v26.clone().len() as i32);
    let v28: i32 = method24(v26.clone(), v27);
    let mut v29: Rc<str> = string_slice(&v26, 0i32 as i64, v28 as i64);
    v29
}
fn options_of_27(mut v0: &Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: &Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> (Rc<str>, US0, US1, bool) {
    let mut v9: bool = &*v6 != "";
    let v15: US1 = if v9 {
        US1::US1_ExpectedAt(v6)
    } else {
        let mut v11: bool = &*v7 != "";
        if v11 {
            US1::US1_ExpectedPrefix(v7)
        } else {
            US1::US1_NoGuard
        }
    };
    let v18: US0 = if v5 {
        US0::US0_AppendText
    } else {
        US0::US0_ReplaceText
    };
    (v4, v18, v15, v8)
}
fn note_edit_entry_0() -> i32 {
    let mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool)>>> = agile_note_edit_args::agile_note_edit_args_read(2i32);
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = v0.borrow()[0i32 as usize].clone();
    let v10: i32 = (v3.clone().len() as i32);
    let mut v11: i32 = 0i32;
    let mut v12: i32 = method23(v3.clone(), v10, v11);
    let mut v13: i32 = v10.wrapping_sub(1i32);
    let v14: Rc<str> = string_slice(&v3.clone(), v12 as i64, v13 as i64);
    let v15: i32 = (v14.clone().len() as i32);
    let v16: i32 = method24(v14.clone(), v15);
    let v17: Rc<str> = string_slice(&v14, 0i32 as i64, v16 as i64);
    let v18: Option<i32> = (v17).trim().parse::<i32>().ok();
    let v19: Option<i32> = method25(v18);
    let mut v20: Rc<dyn Fn((i32)) -> US10> = closure2();
    let mut v21: Option<US10> = v19.map(|x| v20(x));
    let v23: US10 = US10::US10_None;
    let v24: US10 = v23.clone();
    let v25: US10 = v21.unwrap_or(v24);
    let v28: i32 = match v25 {
        US10::US10_None => {
            -1i32
        }
        US10::US10_Some(v26) => {
            let mut v26: i32 = v26;
            v26
        }
    };
    let mut v29: Rc<str> = note_text_of_26(v4.clone());
    let mut v30: bool = v28 < 1i32;
    if v30 {
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note number must be a positive integer"); } LIT.with(|lit| lit.clone()) };
        let v34: Rc<str> = string_concat(&["agile note-edit ", &*v2, " rejected: ", &*v31]);
        let v35: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v34 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v35
    } else {
        let v36: bool = &*v29 == "";
        if v36 {
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note text is empty"); } LIT.with(|lit| lit.clone()) };
            let v40: Rc<str> = string_concat(&["agile note-edit ", &*v2, " rejected: ", &*v37]);
            let v41: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v40 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v41
        } else {
            let (mut v42, mut v43, mut v44, mut v45): (Rc<str>, US0, US1, bool) = options_of_27(&v1, &v2, &v3, &v4, v5, v6, v7, v8, v9);
            if v45 {
                note_edited_1(v1, v2, v28, v29, v42, v43, v44)
            } else {
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>]"); } LIT.with(|lit| lit.clone()) };
                let v50: Rc<str> = string_concat(&["agile note-edit ", &*v2, " rejected: ", &*v47]);
                let v51: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v50 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v51
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        note_edit_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_note_edit_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
