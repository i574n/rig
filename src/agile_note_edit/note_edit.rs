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
#[derive(Clone)]
enum US11 {
    US11_Some(Rc<str>),
    US11_None,
}
#[derive(Clone)]
enum US12 {
    US12_RedactPlan,
    US12_RedactApply,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, Rc<str>, Rc<UH0>, Rc<UH1>),
}
#[derive(Clone)]
enum US13 {
    US13_RedactPlanned(Rc<UH1>),
    US13_RedactApplied(Rc<UH1>, u64, u64),
    US13_RedactRefused(Rc<str>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<str>, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<str>, Rc<UH3>),
}
#[derive(Clone)]
enum US14 {
    US14_Some(Rc<str>, Rc<str>),
    US14_None,
}
#[derive(Clone)]
enum US15 {
    US15_Some(Rc<str>, i32, Rc<str>, Rc<UH0>),
    US15_None,
}
#[derive(Clone)]
enum US16 {
    US16_SpiprojProject,
    US16_TomlProject,
}
#[derive(Clone)]
enum US17 {
    US17_SplitProject(Rc<UH0>, Rc<UH0>, Rc<UH0>),
    US17_NoModules,
}
#[derive(Clone)]
enum US18 {
    US18_ModuleListed,
    US18_ModuleAdded(Rc<str>),
    US18_NoModulesSection,
}
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(Rc<str>, i32, Rc<UH4>),
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
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
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
            let mut v4: i32 = v3.wrapping_sub(6i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
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
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
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
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
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
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method9(v16, v17)
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method11(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let mut v15: US4 = if v6 {
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
                    let mut v17: bool = v2.starts_with(&*v16);
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
                    let mut v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let mut v30: US5 = US5::US5_HeadUnion;
                        US4::US4_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let mut v34: US5 = US5::US5_HeadUnion;
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
            let mut v51: US4 = match &v39 {
                US4::US4_OtherLine => {
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
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
                let mut v4: Rc<UH0> = v4;
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
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US6::US6_ScanPlain => {
                    let mut v6: bool = v5 == 44i32;
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
                            let mut v14: bool = v5 == 34i32;
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
            let mut v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_14(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US6 = US6::US6_ScanPlain;
            let mut v6: i32 = field_end_15(v0.clone(), v1, v2, v5);
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
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return concat5_2(v4, v6, v7.clone(), v7.clone(), v7.clone());
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v10: bool = v9 == 92i32;
            if v10 {
                let mut v11: i32 = v1.wrapping_add(1i32);
                let mut v12: i32 = v11.wrapping_add(1i32);
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v15: bool = v14 == 110i32;
                let mut v24: Rc<str> = if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16
                } else {
                    let mut v17: bool = v14 == 114i32;
                    if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18
                    } else {
                        let v19: bool = v14 == 116i32;
                        if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20
                        } else {
                            let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
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
    let mut v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_18(v3, v4, v5, v6, v7)
}
fn note_at_13(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1.wrapping_add(6i32);
    let v3: i32 = (v0.clone().len() as i32);
    let mut v4: i32 = 2i32;
    let v5: i32 = field_start_14(v0.clone(), v2, v3, v4);
    let mut v6: US6 = US6::US6_ScanPlain;
    let v7: i32 = field_end_15(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_17(v8);
    let v10: i32 = 3i32;
    let mut v11: i32 = field_start_14(v0.clone(), v2, v3, v10);
    let mut v12: US6 = US6::US6_ScanPlain;
    let mut v13: i32 = field_end_15(v0.clone(), v11, v3, v12);
    let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v13 as usize).unwrap_or(""));
    let mut v15: Rc<str> = unquoted_17(v14);
    let v16: i32 = 4i32;
    let mut v17: i32 = field_start_14(v0.clone(), v2, v3, v16);
    let v18: US6 = US6::US6_ScanPlain;
    let v19: i32 = field_end_15(v0.clone(), v17, v3, v18);
    let v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let v21: Rc<str> = unquoted_17(v20);
    (v9, v15, v21)
}
fn note_row_19(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v8));
    let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = concat5_2(v6, v10, v8, v11, v12.clone());
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(v0.replace(&*v16, &*v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(v18.replace(&*v15, &*v19));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v22));
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = Rc::<str>::from(v23.replace(&*v24, &*v25));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = Rc::<str>::from(v26.replace(&*v27, &*v28));
    let mut v30: Rc<str> = concat5_2(v15.clone(), v29, v15.clone(), v12.clone(), v12.clone());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = concat5_2(v5, v13, v14, v30, v31.clone());
    let v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = Rc::<str>::from(v2.replace(&*v16, &*v17));
    let v36: Rc<str> = Rc::<str>::from(v35.replace(&*v15, &*v19));
    let mut v37: Rc<str> = Rc::<str>::from(v36.replace(&*v21, &*v22));
    let mut v38: Rc<str> = Rc::<str>::from(v37.replace(&*v24, &*v25));
    let v39: Rc<str> = Rc::<str>::from(v38.replace(&*v27, &*v28));
    let mut v40: Rc<str> = concat5_2(v15.clone(), v39, v15.clone(), v12.clone(), v12.clone());
    let v41: Rc<str> = Rc::<str>::from(v3.replace(&*v16, &*v17));
    let v42: Rc<str> = Rc::<str>::from(v41.replace(&*v15, &*v19));
    let v43: Rc<str> = Rc::<str>::from(v42.replace(&*v21, &*v22));
    let v44: Rc<str> = Rc::<str>::from(v43.replace(&*v24, &*v25));
    let v45: Rc<str> = Rc::<str>::from(v44.replace(&*v27, &*v28));
    let v46: Rc<str> = concat5_2(v15.clone(), v45, v15.clone(), v12.clone(), v12.clone());
    let v47: Rc<str> = concat5_2(v33, v34, v40, v31.clone(), v46);
    let v48: Rc<str> = Rc::<str>::from(v4.replace(&*v16, &*v17));
    let v49: Rc<str> = Rc::<str>::from(v48.replace(&*v15, &*v19));
    let mut v50: Rc<str> = Rc::<str>::from(v49.replace(&*v21, &*v22));
    let mut v51: Rc<str> = Rc::<str>::from(v50.replace(&*v24, &*v25));
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
            let mut v4: Rc<UH0> = method21(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let mut v15: US4 = if v6 {
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
                    let mut v17: bool = v2.starts_with(&*v16);
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
                    let mut v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let mut v30: US5 = US5::US5_HeadUnion;
                        US4::US4_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let mut v34: US5 = US5::US5_HeadUnion;
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
            let mut v51: US4 = match &v39 {
                US4::US4_OtherLine => {
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
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
            let mut v4: bool = v2 <= 1i32;
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
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_2(v0.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    std::env::var("RIG_AGILE_TEST_PAUSE_MS").or_else(|_| std::env::var("KIT_AGILE_TEST_PAUSE_MS")).ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
    let mut v9: Rc<str> = loop_3(v8);
    let v10: Option<usize> = v9.rfind(&*v7);
    let mut v11: Option<usize> = method4(v10);
    let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v13: Option<i32> = v11.map(|x| v12(x));
    let mut v15: i32 = v13.unwrap_or(-1i32);
    let v16: bool = v15 < 0i32;
    let v20: bool = if v16 {
        true
    } else {
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v18: bool = v9.ends_with(&*v17);
        let v19: bool = v18 == false;
        v19
    };
    let v41: US3 = if v20 {
        US3::US3_OtherFile
    } else {
        let v22: usize = ((0i32) as usize);
        let v23: usize = ((v15) as usize);
        let mut v24: Rc<str> = Rc::<str>::from(&(v9)[v22..v23]);
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
                let mut v34: bool = v24.ends_with(&*v33);
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
            let mut v50: bool = std::path::Path::new(&*v49).is_file();
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
        let mut v90: bool = if v86 {
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
            let mut v93: usize = ((v85) as usize);
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
            let mut v149: bool = renamed_within_22(v0.clone(), v78.clone(), v148);
            if v149 {
                let v150: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
                let mut v151: Rc<str> = loop_3(v150);
                let v152: Option<usize> = v151.rfind(&*v7);
                let v153: Option<usize> = method4(v152);
                let mut v154: Option<i32> = v153.map(|x| v12(x));
                let v156: i32 = v154.unwrap_or(-1i32);
                let v157: bool = v156 < 0i32;
                let mut v161: bool = if v157 {
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
                        let mut v171: bool = v165.ends_with(&*v170);
                        if v171 {
                            let v172: Rc<str> = loop_6(v165);
                            US3::US3_HistoryFile(v172)
                        } else {
                            let mut v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
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
    let mut v14: US2 = if v11 {
        US2::US2_FileReadable(v10.clone())
    } else {
        US2::US2_FileMissing(v10.clone())
    };
    match v14 {
        US2::US2_FileMissing(v15) => {
            let mut v15: Rc<str> = v15;
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = string_concat(&["agile note-edit ", &*v1, " rejected: ", &*v16]);
            let v20: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v19 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v20
        }
        US2::US2_FileReadable(v21) => {
            let v21: Rc<str> = v21;
            let v22: Rc<str> = std::fs::read_to_string(&*v21).unwrap_or_default().into();
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = Rc::<str>::from(v21.replace(&*v23, &*v24));
            let mut v26: Rc<str> = loop_3(v25);
            let v27: Option<usize> = v26.rfind(&*v24);
            let v28: Option<usize> = method4(v27);
            let mut v29: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v30: Option<i32> = v28.map(|x| v29(x));
            let v32: i32 = v30.unwrap_or(-1i32);
            let v33: bool = v32 < 0i32;
            let mut v37: bool = if v33 {
                true
            } else {
                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v35: bool = v26.ends_with(&*v34);
                let v36: bool = v35 == false;
                v36
            };
            let v58: US3 = if v37 {
                US3::US3_OtherFile
            } else {
                let v39: usize = ((0i32) as usize);
                let mut v40: usize = ((v32) as usize);
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
                        let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let mut v51: bool = v41.ends_with(&*v50);
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
                let mut v90: Rc<str> = concat5_2(v71, v88, v73, v89, v9.clone());
                let v92: Rc<str> = string_concat(&["inl ", &*v90, " () = "]);
                let mut v93: i32 = (v92.len() as i32);
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
                                let mut v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
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
                                let mut v161: Rc<str> = concat5_2(v148.clone(), v160, v148.clone(), v9.clone(), v9.clone());
                                let v164: i32 = (v97.clone().len() as i32);
                                let v165: Rc<str> = std::rc::Rc::<str>::from((v164).to_string());
                                let v169: i32 = (v127.len() as i32);
                                let v170: Rc<str> = std::rc::Rc::<str>::from((v169).to_string());
                                let v173: Rc<str> = { let text = &*v97; if text.chars().count() <= 80i32 as usize { std::rc::Rc::<str>::from(text) } else { std::rc::Rc::<str>::from(text.chars().take((80i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
                                let mut v174: Rc<str> = Rc::<str>::from(v173.replace(&*v23, &*v149));
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
fn method23(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure2() -> Rc<dyn Fn((i32)) -> US10> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US10> = Rc::new(move |mut v0: (i32)| -> US10 {
        let mut v1: i32 = (v0);
        US10::US10_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_text_of_24(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v2: bool = &*v1 == "-";
    let mut v14: Rc<str> = if v2 {
        let v3: std::io::Stdin = std::io::stdin();
        let mut v4: Result<std::string::String, std::io::Error> = std::io::read_to_string(v3);
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v7: &str = &*v5;
        let mut v9: std::string::String = String::from(v7);
        let mut v11: std::string::String = v4.unwrap_or(v9);
        let mut v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
        v13
    } else {
        v0
    };
    let mut v15: Rc<str> = Rc::<str>::from((v14).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    v15
}
fn options_of_25(mut v0: &Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: &Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> (Rc<str>, US0, US1, bool) {
    let mut v9: bool = &*v6 != "";
    let mut v15: US1 = if v9 {
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
    let v10: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v11: Option<i32> = (v10).trim().parse::<i32>().ok();
    let mut v12: Option<i32> = method23(v11);
    let mut v13: Rc<dyn Fn((i32)) -> US10> = closure2();
    let mut v14: Option<US10> = v12.map(|x| v13(x));
    let v16: US10 = US10::US10_None;
    let mut v17: US10 = v16.clone();
    let v18: US10 = v14.unwrap_or(v17);
    let v21: i32 = match v18 {
        US10::US10_None => {
            -1i32
        }
        US10::US10_Some(v19) => {
            let v19: i32 = v19;
            v19
        }
    };
    let v22: Rc<str> = note_text_of_24(v4.clone());
    let v23: bool = v21 < 1i32;
    if v23 {
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note number must be a positive integer"); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = string_concat(&["agile note-edit ", &*v2, " rejected: ", &*v24]);
        let v28: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v27 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v28
    } else {
        let mut v29: bool = &*v22 == "";
        if v29 {
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note text is empty"); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = string_concat(&["agile note-edit ", &*v2, " rejected: ", &*v30]);
            let mut v34: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v33 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v34
        } else {
            let (mut v35, mut v36, mut v37, mut v38): (Rc<str>, US0, US1, bool) = options_of_25(&v1, &v2, &v3, &v4, v5, v6, v7, v8, v9);
            if v38 {
                note_edited_1(v1, v2, v21, v22, v35, v36, v37)
            } else {
                let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>]"); } LIT.with(|lit| lit.clone()) };
                let v43: Rc<str> = string_concat(&["agile note-edit ", &*v2, " rejected: ", &*v40]);
                let v44: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v43 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v44
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        note_edit_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn command_line_argument_count_28() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_29(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_27(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_28();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US11 = if v4 {
        let v5: Rc<str> = command_line_argument_29(v0);
        US11::US11_Some(v5)
    } else {
        US11::US11_None
    };
    match v8 {
        US11::US11_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US11::US11_Some(v10) => {
            let v10: Rc<str> = v10;
            let mut v11: i32 = v0.wrapping_add(1i32);
            let mut v12: Rc<UH0> = arguments_from_27(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn words_after_30(mut v0: i32, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v4: bool = v0 > 0i32;
                if v4 {
                    let v5: i32 = v0.wrapping_sub(1i32);
                    (v0, v1) = (v5, v3);
                    continue;
                } else {
                    return v1;
                }
            }
            _ => {
                return v1;
            }
        }
    }
}
fn request_flags_32(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US12, mut v5: bool) -> (Rc<str>, Rc<str>, Rc<str>, US12, bool) {
    loop {
        match &*v0 {
            UH0::UH0_Nil => {
                return (v1, v2, v3, v4, v5);
            }
            _ => {
                match Rc::unwrap_or_clone(v0) {
                    UH0::UH0_Cons(v6, v7) => {
                        let mut v6: Rc<str> = v6;
                        let v7: Rc<UH0> = v7;
                        match &*v7 {
                            UH0::UH0_Cons(v8, v9) => {
                                let v8: Rc<str> = v8.clone();
                                let mut v9: Rc<UH0> = v9.clone();
                                let v10: bool = &*v6 == "--terms";
                                let mut v13: bool = if v10 {
                                    let mut v11: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                    let mut v12: bool = &*v11 != "";
                                    v12
                                } else {
                                    false
                                };
                                if v13 {
                                    let mut v14: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                    (v0, v2) = (v9, v14);
                                    continue;
                                } else {
                                    let v20: bool = &*v6 == "--as";
                                    let v23: bool = if v20 {
                                        let v21: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                        let v22: bool = &*v21 != "";
                                        v22
                                    } else {
                                        false
                                    };
                                    if v23 {
                                        let mut v24: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                        (v0, v3) = (v9, v24);
                                        continue;
                                    } else {
                                        let mut v30: bool = &*v6 == "--apply";
                                        if v30 {
                                            let v31: US12 = US12::US12_RedactApply;
                                            (v0, v4) = (v7, v31);
                                            continue;
                                        } else {
                                            return (v1, v2, v3, v4, false);
                                        }
                                    }
                                }
                            }
                            _ => {
                                let v52: bool = &*v6 == "--apply";
                                if v52 {
                                    let v53: US12 = US12::US12_RedactApply;
                                    (v0, v4) = (v7, v53);
                                    continue;
                                } else {
                                    return (v1, v2, v3, v4, false);
                                }
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
fn request_of_31(mut v0: &Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>, US12, bool) {
    let (mut v22, mut v23, mut v24, mut v25, mut v26): (Rc<str>, Rc<str>, Rc<str>, US12, bool) = match &**v0 {
        UH0::UH0_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<UH0> = v2.clone();
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v4: US12 = US12::US12_RedactPlan;
            let v5: bool = true;
            let (mut v6, mut v7, mut v8, mut v9, mut v10): (Rc<str>, Rc<str>, Rc<str>, US12, bool) = request_flags_32(v2, v1, v3.clone(), v3, v4, v5);
            let mut v12: bool = if v10 {
                let mut v11: bool = &*v7 != "";
                v11
            } else {
                false
            };
            let mut v14: bool = if v12 {
                let mut v13: bool = &*v8 != "";
                v13
            } else {
                false
            };
            (v6, v7, v8, v9, v14)
        }
        _ => {
            let (mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<str>, Rc<str>, Rc<str>, US12, bool) = match &**v0 {
                UH0::UH0_Nil => {
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v16: US12 = US12::US12_RedactPlan;
                    (v15.clone(), v15.clone(), v15, v16, false)
                }
                _ => unreachable!(),
            };
            (v17, v18, v19, v20, v21)
        }
    };
    (v22, v23, v24, v25, v26)
}
fn term_of_line_36(mut v0: Rc<str>) -> US14 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_8(v1, v4);
    match Rc::unwrap_or_clone(v5) {
        UH0::UH0_Cons(v6, v7) => {
            let mut v6: Rc<str> = v6;
            let v7: Rc<UH0> = v7;
            match Rc::unwrap_or_clone(v7) {
                UH0::UH0_Cons(v8, v9) => {
                    let v8: Rc<str> = v8;
                    let mut v9: Rc<UH0> = v9;
                    match Rc::unwrap_or_clone(v9) {
                        UH0::UH0_Nil => {
                            let v10: Rc<str> = Rc::<str>::from((v6).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                            let mut v11: bool = &*v10 != "";
                            let mut v14: bool = if v11 {
                                let mut v12: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                let mut v13: bool = &*v12 != "";
                                v13
                            } else {
                                false
                            };
                            if v14 {
                                let mut v15: Rc<str> = Rc::<str>::from((v6).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                let v16: Rc<str> = std::rc::Rc::<str>::from(v15.to_ascii_lowercase());
                                let mut v17: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                US14::US14_Some(v16, v17)
                            } else {
                                US14::US14_None
                            }
                        }
                        _ => {
                            US14::US14_None
                        }
                    }
                }
                _ => {
                    US14::US14_None
                }
            }
        }
        _ => {
            US14::US14_None
        }
    }
}
fn terms_of_lines_35(mut v0: Rc<UH0>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: US14 = term_of_line_36(v2);
                match v4 {
                    US14::US14_None => {
                        v0 = v3;
                        continue;
                    }
                    US14::US14_Some(v5, v6) => {
                        let v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        let v7: Rc<UH2> = terms_of_lines_35(v3);
                        return Rc::new(UH2::UH2_Cons(v5, v6, v7));
                    }
                }
            }
            UH0::UH0_Nil => {
                return { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn authors_of_lines_37(mut v0: Rc<UH0>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v7: Rc<str> = Rc::<str>::from(v2.replace(&*v5, &*v6));
                let v8: Rc<UH0> = split_list_8(v4, v7);
                match Rc::unwrap_or_clone(v8) {
                    UH0::UH0_Cons(v9, v10) => {
                        let mut v9: Rc<str> = v9;
                        let v10: Rc<UH0> = v10;
                        match Rc::unwrap_or_clone(v10) {
                            UH0::UH0_Cons(v11, v12) => {
                                let mut v11: Rc<str> = v11;
                                let mut v12: Rc<UH0> = v12;
                                match Rc::unwrap_or_clone(v12) {
                                    UH0::UH0_Cons(v13, v14) => {
                                        let mut v13: Rc<str> = v13;
                                        let mut v14: Rc<UH0> = v14;
                                        match Rc::unwrap_or_clone(v14) {
                                            UH0::UH0_Nil => {
                                                let mut v15: Rc<str> = Rc::<str>::from((v9).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                                let v16: bool = &*v15 == "author";
                                                let v19: bool = if v16 {
                                                    let mut v17: Rc<str> = Rc::<str>::from((v11).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                                    let v18: bool = &*v17 != "";
                                                    v18
                                                } else {
                                                    false
                                                };
                                                let v22: bool = if v19 {
                                                    let v20: Rc<str> = Rc::<str>::from((v13).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                                    let v21: bool = &*v20 != "";
                                                    v21
                                                } else {
                                                    false
                                                };
                                                if v22 {
                                                    let v23: Rc<str> = Rc::<str>::from((v11).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                                    let mut v24: Rc<str> = Rc::<str>::from((v13).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                                    let v25: Rc<UH3> = authors_of_lines_37(v3);
                                                    return Rc::new(UH3::UH3_Cons(v23, v24, v25));
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
                                    _ => {
                                        v0 = v3;
                                        continue;
                                    }
                                }
                            }
                            _ => {
                                v0 = v3;
                                continue;
                            }
                        }
                    }
                    _ => {
                        v0 = v3;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                return { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn rules_of_34(mut v0: Rc<str>) -> (Rc<UH2>, Rc<UH3>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = split_list_8(v1, v0);
    let v3: Rc<UH2> = terms_of_lines_35(v2.clone());
    let mut v4: Rc<UH3> = authors_of_lines_37(v2);
    (v3, v4)
}
fn replaced_from_42(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: i32) -> (Rc<str>, i32) {
    loop {
        let v7: i32 = v1.get(v4 as usize..).and_then(|rest| rest.find(&*v2)).map(|found| (found + v4 as usize) as i32).unwrap_or(-1);
        let v8: bool = v7 < 0i32;
        if v8 {
            let mut v9: i32 = (v0.clone().len() as i32);
            let v10: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v9 as usize).unwrap_or(""));
            let mut v11: Rc<str> = string_concat(&[&*v5, &*v10]);
            return (v11, v6);
        } else {
            let mut v12: i32 = (v2.clone().len() as i32);
            let mut v13: i32 = v7.wrapping_add(v12);
            let mut v14: i32 = v7.wrapping_sub(1i32);
            let mut v15: i32 = v0.as_bytes().get(v14 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v16: bool = v15 >= 48i32;
            let v18: bool = if v16 {
                let mut v17: bool = v15 <= 57i32;
                v17
            } else {
                false
            };
            let v22: bool = if v18 {
                true
            } else {
                let v19: bool = v15 >= 65i32;
                if v19 {
                    let v20: bool = v15 <= 90i32;
                    v20
                } else {
                    false
                }
            };
            let mut v26: bool = if v22 {
                true
            } else {
                let v23: bool = v15 >= 97i32;
                if v23 {
                    let mut v24: bool = v15 <= 122i32;
                    v24
                } else {
                    false
                }
            };
            let v28: bool = if v26 {
                true
            } else {
                let v27: bool = v15 == 95i32;
                v27
            };
            let mut v30: bool = if v28 {
                true
            } else {
                let mut v29: bool = v15 == 45i32;
                v29
            };
            let v47: bool = if v30 {
                true
            } else {
                let v31: i32 = v0.as_bytes().get(v13 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v32: bool = v31 >= 48i32;
                let mut v34: bool = if v32 {
                    let v33: bool = v31 <= 57i32;
                    v33
                } else {
                    false
                };
                let mut v38: bool = if v34 {
                    true
                } else {
                    let v35: bool = v31 >= 65i32;
                    if v35 {
                        let v36: bool = v31 <= 90i32;
                        v36
                    } else {
                        false
                    }
                };
                let v42: bool = if v38 {
                    true
                } else {
                    let v39: bool = v31 >= 97i32;
                    if v39 {
                        let mut v40: bool = v31 <= 122i32;
                        v40
                    } else {
                        false
                    }
                };
                let v44: bool = if v42 {
                    true
                } else {
                    let v43: bool = v31 == 95i32;
                    v43
                };
                if v44 {
                    true
                } else {
                    let v45: bool = v31 == 45i32;
                    v45
                }
            };
            if v47 {
                let v48: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v13 as usize).unwrap_or(""));
                let v49: Rc<str> = string_concat(&[&*v5, &*v48]);
                (v4, v5) = (v13, v49);
                continue;
            } else {
                let v52: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v7 as usize).unwrap_or(""));
                let v54: Rc<str> = string_concat(&[&*v5, &*v52, &*v3]);
                let v55: i32 = v6.wrapping_add(1i32);
                (v4, v5, v6) = (v13, v54, v55);
                continue;
            }
        }
    }
}
fn closure4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v2: Rc<str>| -> bool {
        let v3: bool = &*v2 == &*v1;
        v3
    })
}
fn method43(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure4(v0, v1)
}
fn method44(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<dyn Fn(Rc<str>) -> bool>, mut v3: Rc<UH0>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = v2(v4);
                if v6 {
                    return true;
                } else {
                    let v7: Rc<dyn Fn(Rc<str>) -> bool> = method43(v0.clone(), v1.clone());
                    (v2, v3) = (v7, v5);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn method45(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method45(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn text_redacted_41(mut v0: Rc<UH2>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> (Rc<str>, Rc<UH0>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v3, v4, v5) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH2> = v5;
                let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.to_ascii_lowercase());
                let v7: i32 = 0i32;
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v9: i32 = 0i32;
                let (mut v10, mut v11): (Rc<str>, i32) = replaced_from_42(v1, v6, v3.clone(), v4.clone(), v7, v8, v9);
                let mut v12: Rc<dyn Fn(Rc<str>) -> bool> = method43(v3.clone(), v4.clone());
                let mut v13: bool = method44(v3, v4.clone(), v12, v2.clone());
                let mut v14: bool = v11 > 0i32;
                let v16: bool = if v14 {
                    let mut v15: bool = v13 == false;
                    v15
                } else {
                    false
                };
                let v20: Rc<UH0> = if v16 {
                    let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v17));
                    method45(v2, v18)
                } else {
                    v2
                };
                (v0, v1, v2) = (v5, v10, v20);
                continue;
            }
            UH2::UH2_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn author_renamed_46(mut v0: Rc<UH3>, mut v1: Rc<str>) -> US11 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v3, v4, v5) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH3> = v5;
                let mut v6: bool = &*v3 == &*v1;
                if v6 {
                    return US11::US11_Some(v4);
                } else {
                    v0 = v5;
                    continue;
                }
            }
            UH3::UH3_Nil => {
                return US11::US11_None;
            }
        }
    }
}
fn line_redacted_40(mut v0: Rc<UH2>, mut v1: Rc<UH3>, mut v2: Rc<str>) -> (Rc<str>, US15) {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
    let mut v4: i32 = v2.find(&*v3).map(|offset| offset as i32).unwrap_or(-1);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl note_of_"); } LIT.with(|lit| lit.clone()) };
    let mut v6: bool = v2.starts_with(&*v5);
    let v7: bool = v6 == false;
    let mut v9: bool = if v7 {
        true
    } else {
        let v8: bool = v4 < 0i32;
        v8
    };
    if v9 {
        let v10: US15 = US15::US15_None;
        (v2, v10)
    } else {
        let mut v11: i32 = v4.wrapping_add(6i32);
        let (mut v12, mut v13, mut v14): (Rc<str>, Rc<str>, Rc<str>) = note_at_13(v2.clone(), v11);
        let mut v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let (mut v16, mut v17): (Rc<str>, Rc<UH0>) = text_redacted_41(v0, v14, v15);
        let v18: US11 = author_renamed_46(v1, v13.clone());
        let v21: Rc<str> = match &v18 {
            US11::US11_None => {
                v13
            }
            US11::US11_Some(v19) => {
                let v19: Rc<str> = v19.clone();
                v19
            }
        };
        let v28: Rc<UH0> = match v18 {
            US11::US11_None => {
                v17
            }
            US11::US11_Some(v22) => {
                let v22: Rc<str> = v22;
                let v23: Rc<str> = string_concat(&["author ", &*v22]);
                let mut v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v25: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v24));
                method45(v17, v25)
            }
        };
        let mut v29: bool = match &*v28 {
            UH0::UH0_Nil => {
                true
            }
            _ => {
                false
            }
        };
        if v29 {
            let mut v30: US15 = US15::US15_None;
            (v2, v30)
        } else {
            let v31: i32 = v11.wrapping_add(6i32);
            let v32: i32 = (v2.clone().len() as i32);
            let v33: i32 = 0i32;
            let mut v34: i32 = field_start_14(v2.clone(), v31, v32, v33);
            let v35: US6 = US6::US6_ScanPlain;
            let v36: i32 = field_end_15(v2.clone(), v34, v32, v35);
            let mut v37: Rc<str> = std::rc::Rc::<str>::from(v2.get(v34 as usize..v36 as usize).unwrap_or(""));
            let mut v38: Rc<str> = unquoted_17(v37);
            let v39: i32 = 1i32;
            let mut v40: i32 = field_start_14(v2.clone(), v31, v32, v39);
            let v41: US6 = US6::US6_ScanPlain;
            let v42: i32 = field_end_15(v2.clone(), v40, v32, v41);
            let v43: Rc<str> = std::rc::Rc::<str>::from(v2.get(v40 as usize..v42 as usize).unwrap_or(""));
            let v44: Rc<str> = Rc::<str>::from((v43).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = Rc::<str>::from(v44.replace(&*v45, &*v46));
            let v48: Option<i32> = (v47).trim().parse::<i32>().ok();
            let v49: Option<i32> = method23(v48);
            let mut v50: Rc<dyn Fn((i32)) -> US10> = closure2();
            let mut v51: Option<US10> = v49.map(|x| v50(x));
            let v53: US10 = US10::US10_None;
            let v54: US10 = v53.clone();
            let v55: US10 = v51.unwrap_or(v54);
            let v58: i32 = match v55 {
                US10::US10_None => {
                    -1i32
                }
                US10::US10_Some(v56) => {
                    let v56: i32 = v56;
                    v56
                }
            };
            let v59: Rc<str> = note_row_19(v38.clone(), v58, v12, v21.clone(), v16);
            let v60: US15 = US15::US15_Some(v38, v58, v21, v28);
            (v59, v60)
        }
    }
}
fn lines_redacted_39(mut v0: Rc<UH2>, mut v1: Rc<UH3>, mut v2: Rc<UH0>) -> (Rc<UH0>, Rc<UH1>) {
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v5, v6) => {
            let v5: Rc<str> = v5;
            let mut v6: Rc<UH0> = v6;
            let (mut v7, mut v8): (Rc<str>, US15) = line_redacted_40(v0.clone(), v1.clone(), v5);
            let (mut v9, mut v10): (Rc<UH0>, Rc<UH1>) = lines_redacted_39(v0, v1, v6);
            match v8 {
                US15::US15_None => {
                    let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v9));
                    (v17, v10)
                }
                US15::US15_Some(v11, v12, v13, v14) => {
                    let mut v11: Rc<str> = v11;
                    let mut v12: i32 = v12;
                    let mut v13: Rc<str> = v13;
                    let mut v14: Rc<UH0> = v14;
                    let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v9));
                    let v16: Rc<UH1> = Rc::new(UH1::UH1_Cons(v11, v12, v13, v14, v10));
                    (v15, v16)
                }
            }
        }
        UH0::UH0_Nil => {
            let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            (v3, v4)
        }
    }
}
fn redacted_text_38(mut v0: Rc<UH2>, mut v1: Rc<UH3>, mut v2: Rc<str>) -> (Rc<str>, Rc<UH1>) {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<UH0> = split_list_8(v3.clone(), v2);
    let (mut v5, mut v6): (Rc<UH0>, Rc<UH1>) = lines_redacted_39(v0, v1, v4);
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v8: i32 = 0i32;
    let mut v9: i32 = method12(v7.clone(), v5, v8);
    let v10: Rc<Vec<Rc<str>>> = Rc::new(v7.borrow().clone());
    let mut v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v10).as_ref().clone()));
    let mut v12: Rc<str> = Rc::<str>::from((*v11.borrow()).join(&*v3));
    (v12, v6)
}
fn audit_authors_renamed_47(mut v0: Rc<UH3>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH3> = v4;
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                let v8: Rc<str> = Rc::<str>::from(v2.replace(&*v6, &*v7));
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                let v10: Rc<str> = Rc::<str>::from(v8.replace(&*v5, &*v9));
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = Rc::<str>::from(v10.replace(&*v11, &*v12));
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = Rc::<str>::from(v13.replace(&*v14, &*v15));
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = Rc::<str>::from(v16.replace(&*v17, &*v18));
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = concat5_2(v5.clone(), v19, v5.clone(), v20.clone(), v20.clone());
                let v23: Rc<str> = string_concat(&["u32, ", &*v21, ", "]);
                let mut v24: Rc<str> = Rc::<str>::from(v3.replace(&*v6, &*v7));
                let v25: Rc<str> = Rc::<str>::from(v24.replace(&*v5, &*v9));
                let mut v26: Rc<str> = Rc::<str>::from(v25.replace(&*v11, &*v12));
                let v27: Rc<str> = Rc::<str>::from(v26.replace(&*v14, &*v15));
                let v28: Rc<str> = Rc::<str>::from(v27.replace(&*v17, &*v18));
                let mut v29: Rc<str> = concat5_2(v5.clone(), v28, v5.clone(), v20.clone(), v20.clone());
                let v31: Rc<str> = string_concat(&["u32, ", &*v29, ", "]);
                let v32: Rc<str> = Rc::<str>::from(v1.replace(&*v23, &*v31));
                (v0, v1) = (v4, v32);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn civil_of_days_49(mut v0: u64) -> (u64, u64, u64) {
    let mut v1: u64 = v0.wrapping_add(719468u64);
    let mut v2: u64 = v1.wrapping_div(146097u64);
    let v3: u64 = v2.wrapping_mul(146097u64);
    let mut v4: u64 = v1.wrapping_sub(v3);
    let v5: u64 = v4.wrapping_div(1460u64);
    let mut v6: u64 = v4.wrapping_sub(v5);
    let v7: u64 = v4.wrapping_div(36524u64);
    let v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let v10: u64 = v8.wrapping_sub(v9);
    let mut v11: u64 = v10.wrapping_div(365u64);
    let mut v12: u64 = 365u64.wrapping_mul(v11);
    let mut v13: u64 = v11.wrapping_div(4u64);
    let mut v14: u64 = v12.wrapping_add(v13);
    let mut v15: u64 = v11.wrapping_div(100u64);
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
    let mut v26: bool = v20 < 10u64;
    let mut v29: u64 = if v26 {
        let v27: u64 = v20.wrapping_add(3u64);
        v27
    } else {
        let v28: u64 = v20.wrapping_sub(9u64);
        v28
    };
    let mut v30: u64 = v2.wrapping_mul(400u64);
    let v31: u64 = v11.wrapping_add(v30);
    let v32: bool = v29 < 3u64;
    let v33: u64 = if v32 {
        1u64
    } else {
        0u64
    };
    let mut v34: u64 = v31.wrapping_add(v33);
    (v34, v29, v25)
}
fn utc_stamp_48(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(1000u64);
    let mut v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_49(v2);
    let mut v6: u64 = v1.wrapping_rem(86400u64);
    let v7: u64 = v6.wrapping_div(60u64);
    let v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v4 < 10u64;
    let v16: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v11, v12, v13.clone(), v13.clone(), v13)
    } else {
        let mut v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15
    };
    let mut v17: bool = v5 < 10u64;
    let v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v18, v19, v20.clone(), v20.clone(), v20)
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22
    };
    let mut v24: Rc<str> = concat5_2(v8, v9.clone(), v16, v9, v23);
    let v25: u64 = v7.wrapping_div(60u64);
    let mut v26: bool = v25 < 10u64;
    let v32: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v27, v28, v29.clone(), v29.clone(), v29)
    } else {
        let v31: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        v31
    };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v34: u64 = v7.wrapping_rem(60u64);
    let v35: bool = v34 < 10u64;
    let v41: Rc<str> = if v35 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v37: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v36, v37, v38.clone(), v38.clone(), v38)
    } else {
        let mut v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = concat5_2(v32, v33, v41, v42, v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    concat5_2(v24, v45, v44, v43.clone(), v43)
}
fn redaction_row_52(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<UH0>) -> Rc<str> {
    let v7: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = Rc::<str>::from(v3.replace(&*v11, &*v12));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = Rc::<str>::from(v13.replace(&*v10, &*v14));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(v15.replace(&*v16, &*v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(v18.replace(&*v19, &*v20));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = Rc::<str>::from(v21.replace(&*v22, &*v23));
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = concat5_2(v10.clone(), v24, v10.clone(), v25.clone(), v25.clone());
    let mut v29: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let v31: Rc<str> = Rc::<str>::from(v5.replace(&*v11, &*v12));
    let v32: Rc<str> = Rc::<str>::from(v31.replace(&*v10, &*v14));
    let v33: Rc<str> = Rc::<str>::from(v32.replace(&*v16, &*v17));
    let mut v34: Rc<str> = Rc::<str>::from(v33.replace(&*v19, &*v20));
    let v35: Rc<str> = Rc::<str>::from(v34.replace(&*v22, &*v23));
    let v36: Rc<str> = concat5_2(v10.clone(), v35, v10.clone(), v25.clone(), v25.clone());
    let v39: Rc<str> = Rc::<str>::from(v1.replace(&*v11, &*v12));
    let mut v40: Rc<str> = Rc::<str>::from(v39.replace(&*v10, &*v14));
    let v41: Rc<str> = Rc::<str>::from(v40.replace(&*v16, &*v17));
    let v42: Rc<str> = Rc::<str>::from(v41.replace(&*v19, &*v20));
    let v43: Rc<str> = Rc::<str>::from(v42.replace(&*v22, &*v23));
    let v44: Rc<str> = concat5_2(v10.clone(), v43, v10.clone(), v25.clone(), v25.clone());
    let v48: Rc<str> = Rc::<str>::from(v2.replace(&*v11, &*v12));
    let v49: Rc<str> = Rc::<str>::from(v48.replace(&*v10, &*v14));
    let mut v50: Rc<str> = Rc::<str>::from(v49.replace(&*v16, &*v17));
    let mut v51: Rc<str> = Rc::<str>::from(v50.replace(&*v19, &*v20));
    let v52: Rc<str> = Rc::<str>::from(v51.replace(&*v22, &*v23));
    let v53: Rc<str> = concat5_2(v10.clone(), v52, v10.clone(), v25.clone(), v25.clone());
    let v56: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v57: i32 = 0i32;
    let v58: i32 = method12(v56.clone(), v6, v57);
    let v59: Rc<Vec<Rc<str>>> = Rc::new(v56.borrow().clone());
    let v60: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v59).as_ref().clone()));
    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
    let v62: Rc<str> = Rc::<str>::from((*v60.borrow()).join(&*v61));
    let v63: Rc<str> = Rc::<str>::from(v62.replace(&*v11, &*v12));
    let v64: Rc<str> = Rc::<str>::from(v63.replace(&*v10, &*v14));
    let v65: Rc<str> = Rc::<str>::from(v64.replace(&*v16, &*v17));
    let v66: Rc<str> = Rc::<str>::from(v65.replace(&*v19, &*v20));
    let v67: Rc<str> = Rc::<str>::from(v66.replace(&*v22, &*v23));
    let v68: Rc<str> = concat5_2(v10.clone(), v67, v10, v25.clone(), v25);
    let v70: Rc<str> = string_concat(&["inl redaction_", &*v7, " () = Redacted (", &*v26, ", ", &*v29, "u32, ", &*v36, ", ", &*v44, ", ", &*v53, ", ", &*v68, ")\n"]);
    v70
}
fn rows_text_51(mut v0: Rc<UH1>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v5, v6, v7, v8, v9) => {
                let v5: Rc<str> = v5;
                let mut v6: i32 = v6;
                let v7: Rc<str> = v7;
                let v8: Rc<UH0> = v8;
                let mut v9: Rc<UH1> = v9;
                let v10: i32 = v1.wrapping_add(1i32);
                let mut v11: Rc<str> = redaction_row_52(v1, v2.clone(), v3.clone(), v5, v6, v7, v8);
                let mut v12: Rc<str> = string_concat(&[&*v4, &*v11]);
                (v0, v1, v4) = (v9, v10, v12);
                continue;
            }
            UH1::UH1_Nil => {
                return v4;
            }
        }
    }
}
fn rows_counted_53(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl redaction_"); } LIT.with(|lit| lit.clone()) };
                let v5: bool = v2.starts_with(&*v4);
                let v8: bool = if v5 {
                    true
                } else {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= redaction_"); } LIT.with(|lit| lit.clone()) };
                    let v7: bool = v2.starts_with(&*v6);
                    v7
                };
                let v10: i32 = if v8 {
                    let mut v9: i32 = v1.wrapping_add(1i32);
                    v9
                } else {
                    v1
                };
                (v0, v1) = (v3, v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn audit_text_50(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v5: bool = &*v4 == "";
    let v7: Rc<str> = if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion task_redaction =\n    | Redacted :: string * u32 * string * string * string * string -> task_redaction\n"); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        v0
    };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v9: bool = v7.ends_with(&*v8);
    let mut v11: Rc<str> = if v9 {
        v7
    } else {
        let v10: Rc<str> = string_concat(&[&*v7, "\n"]);
        v10
    };
    let mut v12: Rc<UH0> = split_list_8(v8, v11.clone());
    let mut v13: i32 = 0i32;
    let mut v14: i32 = rows_counted_53(v12, v13);
    let mut v15: i32 = v14.wrapping_add(1i32);
    rows_text_51(v1, v15, v2, v3, v11)
}
fn method55(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method55(v3, v1);
            let v5: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let mut v6: bool = &*v5 != "";
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
fn section_lines_57(mut v0: US16, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> US17 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v8, v9) => {
                let v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH0> = v9.clone();
                let v19: bool = match &v0 {
                    US16::US16_SpiprojProject => {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let mut v11: bool = v8.starts_with(&*v10);
                        let mut v14: bool = if v11 {
                            true
                        } else {
                            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v13: bool = v8.starts_with(&*v12);
                            v13
                        };
                        let mut v15: bool = v14 == false;
                        v15
                    }
                    US16::US16_TomlProject => {
                        let v16: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                        let mut v17: bool = &*v16 == "'''";
                        v17
                    }
                };
                if v19 {
                    let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v21: Rc<UH0> = method9(v3, v20);
                    return US17::US17_SplitProject(v2, v21, v1);
                } else {
                    let v23: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                    let mut v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v3));
                    (v1, v3) = (v9, v24);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH0> = method9(v3, v4);
                let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return US17::US17_SplitProject(v2, v5, v6);
            }
        }
    }
}
fn split_lines_56(mut v0: US16, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> US17 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<str> = Rc::<str>::from((v4).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v10: Rc<str> = match &v0 {
                    US16::US16_SpiprojProject => {
                        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                        v7
                    }
                    US16::US16_TomlProject => {
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                        v8
                    }
                };
                let mut v11: bool = &*v6 == &*v10;
                if v11 {
                    let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v12));
                    let mut v14: Rc<UH0> = method9(v2, v13);
                    let mut v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    return section_lines_57(v0, v5, v14, v15);
                } else {
                    let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v2));
                    (v1, v2) = (v5, v17);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US17::US17_NoModules;
            }
        }
    }
}
fn closure5(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US10> {
    Rc::new(move |mut v2: Rc<UH0>| -> US10 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US10::US10_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US10> = method58(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US10::US10_None
            }
        }
    })
}
fn method58(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US10> {
    closure5(v0, v1)
}
fn method59(mut v0: US16, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method59(v0.clone(), v4, v2);
            let mut v13: Rc<str> = match v0 {
                US16::US16_SpiprojProject => {
                    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                    v6.borrow_mut().push(v7);
                    v6.borrow_mut().push(v3);
                    let v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
                    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
                    v11
                }
                US16::US16_TomlProject => {
                    v3
                }
            };
            Rc::new(UH0::UH0_Cons(v13, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn module_added_54(mut v0: US16, mut v1: Rc<str>) -> US11 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("redactions"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v1.replace(&*v4, &*v5));
    let v7: Rc<UH0> = split_list_8(v3.clone(), v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method55(v7, v8);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v11: US17 = split_lines_56(v0.clone(), v9, v10);
    let v47: US18 = match v11 {
        US17::US17_NoModules => {
            US18::US18_NoModulesSection
        }
        US17::US17_SplitProject(v13, v14, v15) => {
            let mut v13: Rc<UH0> = v13;
            let mut v14: Rc<UH0> = v14;
            let mut v15: Rc<UH0> = v15;
            let v16: i32 = 0i32;
            let mut v17: Rc<dyn Fn(Rc<UH0>) -> US10> = method58(v2.clone(), v16);
            let v18: US10 = v17(v14.clone());
            let v19: bool = match v18 {
                US10::US10_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v20: bool = v19 != true;
            if v20 {
                US18::US18_ModuleListed
            } else {
                let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v23: Rc<UH0> = method59(v0.clone(), v14, v22);
                let mut v30: Rc<str> = match v0 {
                    US16::US16_SpiprojProject => {
                        let mut v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                        v24.borrow_mut().push(v25);
                        v24.borrow_mut().push(v2);
                        let mut v26: Rc<Vec<Rc<str>>> = Rc::new(v24.borrow().clone());
                        let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v26).as_ref().clone()));
                        let v28: Rc<str> = Rc::<str>::from((*v27.borrow()).join(&*v5));
                        v28
                    }
                    US16::US16_TomlProject => {
                        v2
                    }
                };
                let v31: Rc<UH0> = method45(v23, v15);
                let v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v30, v31));
                let v33: Rc<UH0> = method45(v13, v32);
                let mut v34: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v35: i32 = 0i32;
                let v36: i32 = method12(v34.clone(), v33, v35);
                let mut v37: Rc<Vec<Rc<str>>> = Rc::new(v34.borrow().clone());
                let mut v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v37).as_ref().clone()));
                let v39: Rc<str> = Rc::<str>::from((*v38.borrow()).join(&*v3));
                let mut v40: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v40.borrow_mut().push(v39);
                v40.borrow_mut().push(v3);
                let v41: Rc<Vec<Rc<str>>> = Rc::new(v40.borrow().clone());
                let v42: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v41).as_ref().clone()));
                let v43: Rc<str> = Rc::<str>::from((*v42.borrow()).join(&*v5));
                US18::US18_ModuleAdded(v43)
            }
        }
    };
    match v47 {
        US18::US18_ModuleAdded(v49) => {
            let v49: Rc<str> = v49;
            US11::US11_Some(v49)
        }
        US18::US18_ModuleListed => {
            US11::US11_Some(v1)
        }
        US18::US18_NoModulesSection => {
            US11::US11_None
        }
    }
}
fn redaction_done_33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US12, mut v4: bool) -> US13 {
    let v5: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = concat5_2(v0.clone(), v6, v7.clone(), v7.clone(), v7.clone());
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(std::path::absolute(&*v0).map(|p| p.to_string_lossy().replace(char::from(92u8), "/")).unwrap_or_else(|_| (&*v0).replace(char::from(92u8), "/")).as_str());
    let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::path::absolute(&*v1).map(|p| p.to_string_lossy().replace(char::from(92u8), "/")).unwrap_or_else(|_| (&*v1).replace(char::from(92u8), "/")).as_str());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from(v11.to_ascii_lowercase());
    let mut v13: bool = &*v12 == &*v10;
    let v19: bool = if v13 {
        true
    } else {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v15: bool = v10.ends_with(&*v14);
        let mut v17: Rc<str> = if v15 {
            v10
        } else {
            let v16: Rc<str> = string_concat(&[&*v10, "/"]);
            v16
        };
        let v18: bool = v12.starts_with(&*v17);
        v18
    };
    if v19 {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--terms must live outside the backlog, so the replaced words never enter it"); } LIT.with(|lit| lit.clone()) };
        US13::US13_RedactRefused(v20)
    } else {
        let v22: bool = std::path::Path::new(&*v1).is_file();
        let v25: US2 = if v22 {
            US2::US2_FileReadable(v1)
        } else {
            US2::US2_FileMissing(v1)
        };
        match v25 {
            US2::US2_FileMissing(v26) => {
                let mut v26: Rc<str> = v26;
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the --terms file is missing"); } LIT.with(|lit| lit.clone()) };
                US13::US13_RedactRefused(v27)
            }
            US2::US2_FileReadable(v29) => {
                let mut v29: Rc<str> = v29;
                let mut v30: Rc<str> = std::fs::read_to_string(&*v29).unwrap_or_default().into();
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = Rc::<str>::from(v29.replace(&*v31, &*v32));
                let mut v34: Rc<str> = loop_3(v33);
                let v35: Option<usize> = v34.rfind(&*v32);
                let v36: Option<usize> = method4(v35);
                let mut v37: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v38: Option<i32> = v36.map(|x| v37(x));
                let mut v40: i32 = v38.unwrap_or(-1i32);
                let v41: bool = v40 < 0i32;
                let v45: bool = if v41 {
                    true
                } else {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v43: bool = v34.ends_with(&*v42);
                    let v44: bool = v43 == false;
                    v44
                };
                let v66: US3 = if v45 {
                    US3::US3_OtherFile
                } else {
                    let v47: usize = ((0i32) as usize);
                    let v48: usize = ((v40) as usize);
                    let v49: Rc<str> = Rc::<str>::from(&(v34)[v47..v48]);
                    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let mut v51: bool = v49.ends_with(&*v50);
                    if v51 {
                        let v52: Rc<str> = loop_5(v49);
                        US3::US3_StateFile(v52)
                    } else {
                        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v55: bool = v49.ends_with(&*v54);
                        if v55 {
                            let v56: Rc<str> = loop_6(v49);
                            US3::US3_HistoryFile(v56)
                        } else {
                            let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v59: bool = v49.ends_with(&*v58);
                            if v59 {
                                let v60: Rc<str> = loop_7(v49);
                                US3::US3_ArchiveFile(v60)
                            } else {
                                US3::US3_OtherFile
                            }
                        }
                    }
                };
                let v77: Rc<str> = match v66 {
                    US3::US3_OtherFile => {
                        v30
                    }
                    _ => {
                        let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v68: Rc<UH0> = split_list_8(v67.clone(), v30);
                        let v69: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v70: Rc<UH0> = method11(v68, v69);
                        let v71: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v72: i32 = 0i32;
                        let v73: i32 = method12(v71.clone(), v70, v72);
                        let mut v74: Rc<Vec<Rc<str>>> = Rc::new(v71.borrow().clone());
                        let v75: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v74).as_ref().clone()));
                        let v76: Rc<str> = Rc::<str>::from((*v75.borrow()).join(&*v67));
                        v76
                    }
                };
                let (mut v78, mut v79): (Rc<UH2>, Rc<UH3>) = rules_of_34(v77);
                let mut v80: bool = std::path::Path::new(&*v8).is_file();
                let mut v83: US2 = if v80 {
                    US2::US2_FileReadable(v8.clone())
                } else {
                    US2::US2_FileMissing(v8.clone())
                };
                match v83 {
                    US2::US2_FileMissing(v84) => {
                        let v84: Rc<str> = v84;
                        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                        US13::US13_RedactRefused(v85)
                    }
                    US2::US2_FileReadable(v87) => {
                        let mut v87: Rc<str> = v87;
                        let v88: Rc<str> = std::fs::read_to_string(&*v87).unwrap_or_default().into();
                        let v89: Rc<str> = Rc::<str>::from(v87.replace(&*v31, &*v32));
                        let mut v90: Rc<str> = loop_3(v89);
                        let v91: Option<usize> = v90.rfind(&*v32);
                        let v92: Option<usize> = method4(v91);
                        let mut v93: Option<i32> = v92.map(|x| v37(x));
                        let v95: i32 = v93.unwrap_or(-1i32);
                        let v96: bool = v95 < 0i32;
                        let v100: bool = if v96 {
                            true
                        } else {
                            let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                            let v98: bool = v90.ends_with(&*v97);
                            let v99: bool = v98 == false;
                            v99
                        };
                        let v121: US3 = if v100 {
                            US3::US3_OtherFile
                        } else {
                            let v102: usize = ((0i32) as usize);
                            let v103: usize = ((v95) as usize);
                            let v104: Rc<str> = Rc::<str>::from(&(v90)[v102..v103]);
                            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                            let v106: bool = v104.ends_with(&*v105);
                            if v106 {
                                let v107: Rc<str> = loop_5(v104);
                                US3::US3_StateFile(v107)
                            } else {
                                let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                let v110: bool = v104.ends_with(&*v109);
                                if v110 {
                                    let v111: Rc<str> = loop_6(v104);
                                    US3::US3_HistoryFile(v111)
                                } else {
                                    let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                    let v114: bool = v104.ends_with(&*v113);
                                    if v114 {
                                        let v115: Rc<str> = loop_7(v104);
                                        US3::US3_ArchiveFile(v115)
                                    } else {
                                        US3::US3_OtherFile
                                    }
                                }
                            }
                        };
                        let v132: Rc<str> = match v121 {
                            US3::US3_OtherFile => {
                                v88
                            }
                            _ => {
                                let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let v123: Rc<UH0> = split_list_8(v122.clone(), v88);
                                let v124: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                let v125: Rc<UH0> = method11(v123, v124);
                                let v126: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                let v127: i32 = 0i32;
                                let v128: i32 = method12(v126.clone(), v125, v127);
                                let v129: Rc<Vec<Rc<str>>> = Rc::new(v126.borrow().clone());
                                let v130: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v129).as_ref().clone()));
                                let v131: Rc<str> = Rc::<str>::from((*v130.borrow()).join(&*v122));
                                v131
                            }
                        };
                        let v133: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v132);
                        let (mut v134, mut v135): (Rc<str>, Rc<UH1>) = redacted_text_38(v78.clone(), v79.clone(), v133.clone());
                        let v136: bool = match Rc::unwrap_or_clone(v78) {
                            UH2::UH2_Nil => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v138: bool = if v136 {
                            match &*v79 {
                                UH3::UH3_Nil => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            }
                        } else {
                            false
                        };
                        if v138 {
                            let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the --terms file has no term<TAB>role or author<TAB>old<TAB>new rows"); } LIT.with(|lit| lit.clone()) };
                            US13::US13_RedactRefused(v139)
                        } else {
                            let v141: bool = match v3 {
                                US12::US12_RedactPlan => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            };
                            let v143: bool = if v141 {
                                true
                            } else {
                                match &*v135 {
                                    UH1::UH1_Nil => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                }
                            };
                            if v143 {
                                US13::US13_RedactPlanned(v135)
                            } else {
                                let v145: US7 = state_written_20(v8, v132, v134.clone());
                                match &v145 {
                                    US7::US7_StateWritten => {
                                        let v146: Rc<str> = Rc::<str>::from(v0.replace(&*v31, &*v32));
                                        let v147: Rc<str> = string_concat(&[&*v146, "/state"]);
                                        let v148: Rc<str> = string_concat(&[&*v147, "/redactions.spi"]);
                                        let mut v149: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                        v149.borrow_mut().push(v147.clone());
                                        let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.toml"); } LIT.with(|lit| lit.clone()) };
                                        v149.borrow_mut().push(v150.clone());
                                        let mut v151: Rc<Vec<Rc<str>>> = Rc::new(v149.borrow().clone());
                                        let v152: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v151).as_ref().clone()));
                                        let v153: Rc<str> = Rc::<str>::from((*v152.borrow()).join(&*v32));
                                        let mut v154: bool = std::path::Path::new(&*v153).is_file();
                                        let v157: US16 = if v154 {
                                            US16::US16_TomlProject
                                        } else {
                                            US16::US16_SpiprojProject
                                        };
                                        let v160: Rc<str> = match v157 {
                                            US16::US16_SpiprojProject => {
                                                let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
                                                v158
                                            }
                                            US16::US16_TomlProject => {
                                                v150.clone()
                                            }
                                        };
                                        let mut v161: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                        v161.borrow_mut().push(v147.clone());
                                        v161.borrow_mut().push(v160);
                                        let v162: Rc<Vec<Rc<str>>> = Rc::new(v161.borrow().clone());
                                        let v163: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v162).as_ref().clone()));
                                        let v164: Rc<str> = Rc::<str>::from((*v163.borrow()).join(&*v32));
                                        let v165: bool = std::path::Path::new(&*v148).is_file();
                                        let v215: Rc<str> = if v165 {
                                            let (mut v166, mut v167): (bool, Rc<str>) = match std::fs::read(&*v148) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                                            if v166 {
                                                let v168: Rc<str> = string_concat(&["rig state read failed: ", &*v148]);
                                                std::panic::panic_any::<std::string::String>(format!("{}", v168))
                                            } else {
                                                let v170: Rc<str> = Rc::<str>::from(v148.replace(&*v31, &*v32));
                                                let mut v171: Rc<str> = loop_3(v170);
                                                let v172: Option<usize> = v171.rfind(&*v32);
                                                let v173: Option<usize> = method4(v172);
                                                let mut v174: Option<i32> = v173.map(|x| v37(x));
                                                let v176: i32 = v174.unwrap_or(-1i32);
                                                let v177: bool = v176 < 0i32;
                                                let v181: bool = if v177 {
                                                    true
                                                } else {
                                                    let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                    let v179: bool = v171.ends_with(&*v178);
                                                    let v180: bool = v179 == false;
                                                    v180
                                                };
                                                let v202: US3 = if v181 {
                                                    US3::US3_OtherFile
                                                } else {
                                                    let v183: usize = ((0i32) as usize);
                                                    let v184: usize = ((v176) as usize);
                                                    let v185: Rc<str> = Rc::<str>::from(&(v171)[v183..v184]);
                                                    let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                    let v187: bool = v185.ends_with(&*v186);
                                                    if v187 {
                                                        let v188: Rc<str> = loop_5(v185);
                                                        US3::US3_StateFile(v188)
                                                    } else {
                                                        let v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                        let v191: bool = v185.ends_with(&*v190);
                                                        if v191 {
                                                            let v192: Rc<str> = loop_6(v185);
                                                            US3::US3_HistoryFile(v192)
                                                        } else {
                                                            let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                            let v195: bool = v185.ends_with(&*v194);
                                                            if v195 {
                                                                let v196: Rc<str> = loop_7(v185);
                                                                US3::US3_ArchiveFile(v196)
                                                            } else {
                                                                US3::US3_OtherFile
                                                            }
                                                        }
                                                    }
                                                };
                                                match v202 {
                                                    US3::US3_OtherFile => {
                                                        v167
                                                    }
                                                    _ => {
                                                        let v203: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                        let v204: Rc<UH0> = split_list_8(v203.clone(), v167);
                                                        let v205: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                        let v206: Rc<UH0> = method11(v204, v205);
                                                        let v207: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                        let v208: i32 = 0i32;
                                                        let v209: i32 = method12(v207.clone(), v206, v208);
                                                        let v210: Rc<Vec<Rc<str>>> = Rc::new(v207.borrow().clone());
                                                        let v211: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v210).as_ref().clone()));
                                                        let v212: Rc<str> = Rc::<str>::from((*v211.borrow()).join(&*v203));
                                                        v212
                                                    }
                                                }
                                            }
                                        } else {
                                            v7.clone()
                                        };
                                        let v216: Rc<str> = audit_authors_renamed_47(v79, v215);
                                        let v217: Rc<str> = utc_stamp_48(v5);
                                        let v218: Rc<str> = audit_text_50(v216, v135.clone(), v2, v217);
                                        let v219: Rc<str> = string_concat(&[&*v148, ".pending"]);
                                        let v220: Rc<str> = Rc::<str>::from(v219.replace(&*v31, &*v32));
                                        let mut v221: Rc<str> = loop_3(v220);
                                        let v222: Option<usize> = v221.rfind(&*v32);
                                        let v223: Option<usize> = method4(v222);
                                        let mut v224: Option<i32> = v223.map(|x| v37(x));
                                        let v226: i32 = v224.unwrap_or(-1i32);
                                        let v227: bool = v226 < 0i32;
                                        let v231: bool = if v227 {
                                            true
                                        } else {
                                            let v228: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                            let v229: bool = v221.ends_with(&*v228);
                                            let v230: bool = v229 == false;
                                            v230
                                        };
                                        let v252: US3 = if v231 {
                                            US3::US3_OtherFile
                                        } else {
                                            let v233: usize = ((0i32) as usize);
                                            let v234: usize = ((v226) as usize);
                                            let v235: Rc<str> = Rc::<str>::from(&(v221)[v233..v234]);
                                            let v236: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                            let v237: bool = v235.ends_with(&*v236);
                                            if v237 {
                                                let v238: Rc<str> = loop_5(v235);
                                                US3::US3_StateFile(v238)
                                            } else {
                                                let v240: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                let v241: bool = v235.ends_with(&*v240);
                                                if v241 {
                                                    let v242: Rc<str> = loop_6(v235);
                                                    US3::US3_HistoryFile(v242)
                                                } else {
                                                    let v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                    let v245: bool = v235.ends_with(&*v244);
                                                    if v245 {
                                                        let v246: Rc<str> = loop_7(v235);
                                                        US3::US3_ArchiveFile(v246)
                                                    } else {
                                                        US3::US3_OtherFile
                                                    }
                                                }
                                            }
                                        };
                                        let v275: US8 = match v252 {
                                            US3::US3_ArchiveFile(v265) => {
                                                let v265: Rc<str> = v265;
                                                let v266: Rc<str> = string_concat(&[&*v265, "/state/spiral.toml"]);
                                                let v267: bool = std::path::Path::new(&*v266).is_file();
                                                if v267 {
                                                    US8::US8_SymbolsRoot
                                                } else {
                                                    US8::US8_WordsRoot
                                                }
                                            }
                                            US3::US3_HistoryFile(v259) => {
                                                let v259: Rc<str> = v259;
                                                let v260: Rc<str> = string_concat(&[&*v259, "/state/spiral.toml"]);
                                                let v261: bool = std::path::Path::new(&*v260).is_file();
                                                if v261 {
                                                    US8::US8_SymbolsRoot
                                                } else {
                                                    US8::US8_WordsRoot
                                                }
                                            }
                                            US3::US3_OtherFile => {
                                                US8::US8_WordsRoot
                                            }
                                            US3::US3_StateFile(v253) => {
                                                let v253: Rc<str> = v253;
                                                let v254: Rc<str> = string_concat(&[&*v253, "/state/spiral.toml"]);
                                                let v255: bool = std::path::Path::new(&*v254).is_file();
                                                if v255 {
                                                    US8::US8_SymbolsRoot
                                                } else {
                                                    US8::US8_WordsRoot
                                                }
                                            }
                                        };
                                        let v287: Rc<str> = match v275 {
                                            US8::US8_SymbolsRoot => {
                                                let v276: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                let v277: Rc<UH0> = split_list_8(v276.clone(), v218.clone());
                                                let v278: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                let v279: Rc<UH0> = method21(v277, v278);
                                                let v280: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                let v281: i32 = 0i32;
                                                let v282: i32 = method12(v280.clone(), v279, v281);
                                                let v283: Rc<Vec<Rc<str>>> = Rc::new(v280.borrow().clone());
                                                let v284: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v283).as_ref().clone()));
                                                let v285: Rc<str> = Rc::<str>::from((*v284.borrow()).join(&*v276));
                                                v285
                                            }
                                            US8::US8_WordsRoot => {
                                                v218.clone()
                                            }
                                        };
                                        let v288: bool = std::fs::write(&*v219, &*v287).is_ok();
                                        let v369: US9 = if v288 {
                                            let v289: i32 = 40i32;
                                            let v290: bool = renamed_within_22(v148.clone(), v219.clone(), v289);
                                            if v290 {
                                                let v291: Rc<str> = Rc::<str>::from(v148.replace(&*v31, &*v32));
                                                let mut v292: Rc<str> = loop_3(v291);
                                                let v293: Option<usize> = v292.rfind(&*v32);
                                                let v294: Option<usize> = method4(v293);
                                                let mut v295: Option<i32> = v294.map(|x| v37(x));
                                                let v297: i32 = v295.unwrap_or(-1i32);
                                                let v298: bool = v297 < 0i32;
                                                let v302: bool = if v298 {
                                                    true
                                                } else {
                                                    let v299: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                    let v300: bool = v292.ends_with(&*v299);
                                                    let v301: bool = v300 == false;
                                                    v301
                                                };
                                                let v323: US3 = if v302 {
                                                    US3::US3_OtherFile
                                                } else {
                                                    let v304: usize = ((0i32) as usize);
                                                    let v305: usize = ((v297) as usize);
                                                    let v306: Rc<str> = Rc::<str>::from(&(v292)[v304..v305]);
                                                    let v307: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                    let v308: bool = v306.ends_with(&*v307);
                                                    if v308 {
                                                        let v309: Rc<str> = loop_5(v306);
                                                        US3::US3_StateFile(v309)
                                                    } else {
                                                        let v311: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                        let v312: bool = v306.ends_with(&*v311);
                                                        if v312 {
                                                            let v313: Rc<str> = loop_6(v306);
                                                            US3::US3_HistoryFile(v313)
                                                        } else {
                                                            let v315: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                            let v316: bool = v306.ends_with(&*v315);
                                                            if v316 {
                                                                let v317: Rc<str> = loop_7(v306);
                                                                US3::US3_ArchiveFile(v317)
                                                            } else {
                                                                US3::US3_OtherFile
                                                            }
                                                        }
                                                    }
                                                };
                                                let v346: US8 = match v323 {
                                                    US3::US3_ArchiveFile(v336) => {
                                                        let v336: Rc<str> = v336;
                                                        let v337: Rc<str> = string_concat(&[&*v336, "/state/spiral.toml"]);
                                                        let v338: bool = std::path::Path::new(&*v337).is_file();
                                                        if v338 {
                                                            US8::US8_SymbolsRoot
                                                        } else {
                                                            US8::US8_WordsRoot
                                                        }
                                                    }
                                                    US3::US3_HistoryFile(v330) => {
                                                        let v330: Rc<str> = v330;
                                                        let v331: Rc<str> = string_concat(&[&*v330, "/state/spiral.toml"]);
                                                        let v332: bool = std::path::Path::new(&*v331).is_file();
                                                        if v332 {
                                                            US8::US8_SymbolsRoot
                                                        } else {
                                                            US8::US8_WordsRoot
                                                        }
                                                    }
                                                    US3::US3_OtherFile => {
                                                        US8::US8_WordsRoot
                                                    }
                                                    US3::US3_StateFile(v324) => {
                                                        let v324: Rc<str> = v324;
                                                        let v325: Rc<str> = string_concat(&[&*v324, "/state/spiral.toml"]);
                                                        let v326: bool = std::path::Path::new(&*v325).is_file();
                                                        if v326 {
                                                            US8::US8_SymbolsRoot
                                                        } else {
                                                            US8::US8_WordsRoot
                                                        }
                                                    }
                                                };
                                                let v358: Rc<str> = match v346 {
                                                    US8::US8_SymbolsRoot => {
                                                        let v347: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                        let v348: Rc<UH0> = split_list_8(v347.clone(), v218);
                                                        let v349: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                        let v350: Rc<UH0> = method21(v348, v349);
                                                        let v351: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                        let v352: i32 = 0i32;
                                                        let v353: i32 = method12(v351.clone(), v350, v352);
                                                        let v354: Rc<Vec<Rc<str>>> = Rc::new(v351.borrow().clone());
                                                        let v355: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v354).as_ref().clone()));
                                                        let v356: Rc<str> = Rc::<str>::from((*v355.borrow()).join(&*v347));
                                                        v356
                                                    }
                                                    US8::US8_WordsRoot => {
                                                        v218
                                                    }
                                                };
                                                let v359: bool = std::fs::read_to_string(&*v148).is_ok_and(|value| value == *v358);
                                                if v359 {
                                                    US9::US9_Replaced
                                                } else {
                                                    US9::US9_ReadbackMismatch
                                                }
                                            } else {
                                                let v363: bool = std::fs::remove_file(&*v219).is_ok() || !std::path::Path::new(&*v219).exists();
                                                if v363 {
                                                    US9::US9_RenameFailed
                                                } else {
                                                    US9::US9_RenameFailedTemporaryKept
                                                }
                                            }
                                        } else {
                                            US9::US9_TemporaryWriteFailed
                                        };
                                        let mut v370: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                        v370.borrow_mut().push(v147);
                                        v370.borrow_mut().push(v150);
                                        let v371: Rc<Vec<Rc<str>>> = Rc::new(v370.borrow().clone());
                                        let v372: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v371).as_ref().clone()));
                                        let v373: Rc<str> = Rc::<str>::from((*v372.borrow()).join(&*v32));
                                        let v374: bool = std::path::Path::new(&*v373).is_file();
                                        let v377: US16 = if v374 {
                                            US16::US16_TomlProject
                                        } else {
                                            US16::US16_SpiprojProject
                                        };
                                        let v378: bool = std::path::Path::new(&*v164).is_file();
                                        let v428: Rc<str> = if v378 {
                                            let (mut v379, mut v380): (bool, Rc<str>) = match std::fs::read(&*v164) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                                            if v379 {
                                                let v381: Rc<str> = string_concat(&["rig state read failed: ", &*v164]);
                                                std::panic::panic_any::<std::string::String>(format!("{}", v381))
                                            } else {
                                                let v383: Rc<str> = Rc::<str>::from(v164.replace(&*v31, &*v32));
                                                let mut v384: Rc<str> = loop_3(v383);
                                                let v385: Option<usize> = v384.rfind(&*v32);
                                                let v386: Option<usize> = method4(v385);
                                                let mut v387: Option<i32> = v386.map(|x| v37(x));
                                                let v389: i32 = v387.unwrap_or(-1i32);
                                                let v390: bool = v389 < 0i32;
                                                let v394: bool = if v390 {
                                                    true
                                                } else {
                                                    let v391: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                    let v392: bool = v384.ends_with(&*v391);
                                                    let v393: bool = v392 == false;
                                                    v393
                                                };
                                                let v415: US3 = if v394 {
                                                    US3::US3_OtherFile
                                                } else {
                                                    let v396: usize = ((0i32) as usize);
                                                    let v397: usize = ((v389) as usize);
                                                    let v398: Rc<str> = Rc::<str>::from(&(v384)[v396..v397]);
                                                    let v399: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                    let v400: bool = v398.ends_with(&*v399);
                                                    if v400 {
                                                        let v401: Rc<str> = loop_5(v398);
                                                        US3::US3_StateFile(v401)
                                                    } else {
                                                        let v403: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                        let v404: bool = v398.ends_with(&*v403);
                                                        if v404 {
                                                            let v405: Rc<str> = loop_6(v398);
                                                            US3::US3_HistoryFile(v405)
                                                        } else {
                                                            let v407: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                            let v408: bool = v398.ends_with(&*v407);
                                                            if v408 {
                                                                let v409: Rc<str> = loop_7(v398);
                                                                US3::US3_ArchiveFile(v409)
                                                            } else {
                                                                US3::US3_OtherFile
                                                            }
                                                        }
                                                    }
                                                };
                                                match v415 {
                                                    US3::US3_OtherFile => {
                                                        v380
                                                    }
                                                    _ => {
                                                        let v416: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                        let v417: Rc<UH0> = split_list_8(v416.clone(), v380);
                                                        let v418: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                        let v419: Rc<UH0> = method11(v417, v418);
                                                        let v420: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                        let v421: i32 = 0i32;
                                                        let v422: i32 = method12(v420.clone(), v419, v421);
                                                        let v423: Rc<Vec<Rc<str>>> = Rc::new(v420.borrow().clone());
                                                        let v424: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v423).as_ref().clone()));
                                                        let v425: Rc<str> = Rc::<str>::from((*v424.borrow()).join(&*v416));
                                                        v425
                                                    }
                                                }
                                            }
                                        } else {
                                            v7.clone()
                                        };
                                        let v429: US11 = module_added_54(v377, v428);
                                        match &v369 {
                                            US9::US9_Replaced => {
                                                match v429 {
                                                    US11::US11_None => {
                                                        let v652: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the state project has no modules section to list redactions in"); } LIT.with(|lit| lit.clone()) };
                                                        US13::US13_RedactRefused(v652)
                                                    }
                                                    US11::US11_Some(v430) => {
                                                        let v430: Rc<str> = v430;
                                                        let v431: bool = std::path::Path::new(&*v164).is_file();
                                                        let v481: Rc<str> = if v431 {
                                                            let (mut v432, mut v433): (bool, Rc<str>) = match std::fs::read(&*v164) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                                                            if v432 {
                                                                let v434: Rc<str> = string_concat(&["rig state read failed: ", &*v164]);
                                                                std::panic::panic_any::<std::string::String>(format!("{}", v434))
                                                            } else {
                                                                let v436: Rc<str> = Rc::<str>::from(v164.replace(&*v31, &*v32));
                                                                let mut v437: Rc<str> = loop_3(v436);
                                                                let v438: Option<usize> = v437.rfind(&*v32);
                                                                let v439: Option<usize> = method4(v438);
                                                                let mut v440: Option<i32> = v439.map(|x| v37(x));
                                                                let v442: i32 = v440.unwrap_or(-1i32);
                                                                let v443: bool = v442 < 0i32;
                                                                let v447: bool = if v443 {
                                                                    true
                                                                } else {
                                                                    let v444: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                                    let v445: bool = v437.ends_with(&*v444);
                                                                    let v446: bool = v445 == false;
                                                                    v446
                                                                };
                                                                let v468: US3 = if v447 {
                                                                    US3::US3_OtherFile
                                                                } else {
                                                                    let v449: usize = ((0i32) as usize);
                                                                    let v450: usize = ((v442) as usize);
                                                                    let v451: Rc<str> = Rc::<str>::from(&(v437)[v449..v450]);
                                                                    let v452: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                                    let v453: bool = v451.ends_with(&*v452);
                                                                    if v453 {
                                                                        let v454: Rc<str> = loop_5(v451);
                                                                        US3::US3_StateFile(v454)
                                                                    } else {
                                                                        let v456: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                                        let v457: bool = v451.ends_with(&*v456);
                                                                        if v457 {
                                                                            let v458: Rc<str> = loop_6(v451);
                                                                            US3::US3_HistoryFile(v458)
                                                                        } else {
                                                                            let v460: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                                            let v461: bool = v451.ends_with(&*v460);
                                                                            if v461 {
                                                                                let v462: Rc<str> = loop_7(v451);
                                                                                US3::US3_ArchiveFile(v462)
                                                                            } else {
                                                                                US3::US3_OtherFile
                                                                            }
                                                                        }
                                                                    }
                                                                };
                                                                match v468 {
                                                                    US3::US3_OtherFile => {
                                                                        v433
                                                                    }
                                                                    _ => {
                                                                        let v469: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                                        let v470: Rc<UH0> = split_list_8(v469.clone(), v433);
                                                                        let v471: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                                        let v472: Rc<UH0> = method11(v470, v471);
                                                                        let v473: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                                        let v474: i32 = 0i32;
                                                                        let v475: i32 = method12(v473.clone(), v472, v474);
                                                                        let v476: Rc<Vec<Rc<str>>> = Rc::new(v473.borrow().clone());
                                                                        let v477: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v476).as_ref().clone()));
                                                                        let v478: Rc<str> = Rc::<str>::from((*v477.borrow()).join(&*v469));
                                                                        v478
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            v7
                                                        };
                                                        let v482: bool = &*v430 == &*v481;
                                                        let v635: US9 = if v482 {
                                                            US9::US9_Replaced
                                                        } else {
                                                            let v484: Rc<str> = string_concat(&[&*v164, ".pending"]);
                                                            let v485: Rc<str> = Rc::<str>::from(v484.replace(&*v31, &*v32));
                                                            let mut v486: Rc<str> = loop_3(v485);
                                                            let v487: Option<usize> = v486.rfind(&*v32);
                                                            let v488: Option<usize> = method4(v487);
                                                            let mut v489: Option<i32> = v488.map(|x| v37(x));
                                                            let v491: i32 = v489.unwrap_or(-1i32);
                                                            let v492: bool = v491 < 0i32;
                                                            let v496: bool = if v492 {
                                                                true
                                                            } else {
                                                                let v493: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                                let v494: bool = v486.ends_with(&*v493);
                                                                let v495: bool = v494 == false;
                                                                v495
                                                            };
                                                            let v517: US3 = if v496 {
                                                                US3::US3_OtherFile
                                                            } else {
                                                                let v498: usize = ((0i32) as usize);
                                                                let v499: usize = ((v491) as usize);
                                                                let v500: Rc<str> = Rc::<str>::from(&(v486)[v498..v499]);
                                                                let v501: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                                let v502: bool = v500.ends_with(&*v501);
                                                                if v502 {
                                                                    let v503: Rc<str> = loop_5(v500);
                                                                    US3::US3_StateFile(v503)
                                                                } else {
                                                                    let v505: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                                    let v506: bool = v500.ends_with(&*v505);
                                                                    if v506 {
                                                                        let v507: Rc<str> = loop_6(v500);
                                                                        US3::US3_HistoryFile(v507)
                                                                    } else {
                                                                        let v509: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                                        let v510: bool = v500.ends_with(&*v509);
                                                                        if v510 {
                                                                            let v511: Rc<str> = loop_7(v500);
                                                                            US3::US3_ArchiveFile(v511)
                                                                        } else {
                                                                            US3::US3_OtherFile
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            let v540: US8 = match v517 {
                                                                US3::US3_ArchiveFile(v530) => {
                                                                    let v530: Rc<str> = v530;
                                                                    let v531: Rc<str> = string_concat(&[&*v530, "/state/spiral.toml"]);
                                                                    let v532: bool = std::path::Path::new(&*v531).is_file();
                                                                    if v532 {
                                                                        US8::US8_SymbolsRoot
                                                                    } else {
                                                                        US8::US8_WordsRoot
                                                                    }
                                                                }
                                                                US3::US3_HistoryFile(v524) => {
                                                                    let v524: Rc<str> = v524;
                                                                    let v525: Rc<str> = string_concat(&[&*v524, "/state/spiral.toml"]);
                                                                    let v526: bool = std::path::Path::new(&*v525).is_file();
                                                                    if v526 {
                                                                        US8::US8_SymbolsRoot
                                                                    } else {
                                                                        US8::US8_WordsRoot
                                                                    }
                                                                }
                                                                US3::US3_OtherFile => {
                                                                    US8::US8_WordsRoot
                                                                }
                                                                US3::US3_StateFile(v518) => {
                                                                    let v518: Rc<str> = v518;
                                                                    let v519: Rc<str> = string_concat(&[&*v518, "/state/spiral.toml"]);
                                                                    let v520: bool = std::path::Path::new(&*v519).is_file();
                                                                    if v520 {
                                                                        US8::US8_SymbolsRoot
                                                                    } else {
                                                                        US8::US8_WordsRoot
                                                                    }
                                                                }
                                                            };
                                                            let v552: Rc<str> = match v540 {
                                                                US8::US8_SymbolsRoot => {
                                                                    let v541: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                                    let v542: Rc<UH0> = split_list_8(v541.clone(), v430.clone());
                                                                    let v543: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                                    let v544: Rc<UH0> = method21(v542, v543);
                                                                    let v545: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                                    let v546: i32 = 0i32;
                                                                    let v547: i32 = method12(v545.clone(), v544, v546);
                                                                    let v548: Rc<Vec<Rc<str>>> = Rc::new(v545.borrow().clone());
                                                                    let v549: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v548).as_ref().clone()));
                                                                    let v550: Rc<str> = Rc::<str>::from((*v549.borrow()).join(&*v541));
                                                                    v550
                                                                }
                                                                US8::US8_WordsRoot => {
                                                                    v430.clone()
                                                                }
                                                            };
                                                            let v553: bool = std::fs::write(&*v484, &*v552).is_ok();
                                                            if v553 {
                                                                let v554: i32 = 40i32;
                                                                let v555: bool = renamed_within_22(v164.clone(), v484.clone(), v554);
                                                                if v555 {
                                                                    let v556: Rc<str> = Rc::<str>::from(v164.replace(&*v31, &*v32));
                                                                    let mut v557: Rc<str> = loop_3(v556);
                                                                    let v558: Option<usize> = v557.rfind(&*v32);
                                                                    let v559: Option<usize> = method4(v558);
                                                                    let mut v560: Option<i32> = v559.map(|x| v37(x));
                                                                    let v562: i32 = v560.unwrap_or(-1i32);
                                                                    let v563: bool = v562 < 0i32;
                                                                    let v567: bool = if v563 {
                                                                        true
                                                                    } else {
                                                                        let v564: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                                        let v565: bool = v557.ends_with(&*v564);
                                                                        let v566: bool = v565 == false;
                                                                        v566
                                                                    };
                                                                    let v588: US3 = if v567 {
                                                                        US3::US3_OtherFile
                                                                    } else {
                                                                        let v569: usize = ((0i32) as usize);
                                                                        let v570: usize = ((v562) as usize);
                                                                        let v571: Rc<str> = Rc::<str>::from(&(v557)[v569..v570]);
                                                                        let v572: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                                        let v573: bool = v571.ends_with(&*v572);
                                                                        if v573 {
                                                                            let v574: Rc<str> = loop_5(v571);
                                                                            US3::US3_StateFile(v574)
                                                                        } else {
                                                                            let v576: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                                            let v577: bool = v571.ends_with(&*v576);
                                                                            if v577 {
                                                                                let v578: Rc<str> = loop_6(v571);
                                                                                US3::US3_HistoryFile(v578)
                                                                            } else {
                                                                                let v580: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                                                let v581: bool = v571.ends_with(&*v580);
                                                                                if v581 {
                                                                                    let v582: Rc<str> = loop_7(v571);
                                                                                    US3::US3_ArchiveFile(v582)
                                                                                } else {
                                                                                    US3::US3_OtherFile
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    let v611: US8 = match v588 {
                                                                        US3::US3_ArchiveFile(v601) => {
                                                                            let v601: Rc<str> = v601;
                                                                            let v602: Rc<str> = string_concat(&[&*v601, "/state/spiral.toml"]);
                                                                            let v603: bool = std::path::Path::new(&*v602).is_file();
                                                                            if v603 {
                                                                                US8::US8_SymbolsRoot
                                                                            } else {
                                                                                US8::US8_WordsRoot
                                                                            }
                                                                        }
                                                                        US3::US3_HistoryFile(v595) => {
                                                                            let v595: Rc<str> = v595;
                                                                            let v596: Rc<str> = string_concat(&[&*v595, "/state/spiral.toml"]);
                                                                            let v597: bool = std::path::Path::new(&*v596).is_file();
                                                                            if v597 {
                                                                                US8::US8_SymbolsRoot
                                                                            } else {
                                                                                US8::US8_WordsRoot
                                                                            }
                                                                        }
                                                                        US3::US3_OtherFile => {
                                                                            US8::US8_WordsRoot
                                                                        }
                                                                        US3::US3_StateFile(v589) => {
                                                                            let v589: Rc<str> = v589;
                                                                            let v590: Rc<str> = string_concat(&[&*v589, "/state/spiral.toml"]);
                                                                            let v591: bool = std::path::Path::new(&*v590).is_file();
                                                                            if v591 {
                                                                                US8::US8_SymbolsRoot
                                                                            } else {
                                                                                US8::US8_WordsRoot
                                                                            }
                                                                        }
                                                                    };
                                                                    let v623: Rc<str> = match v611 {
                                                                        US8::US8_SymbolsRoot => {
                                                                            let v612: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                                            let v613: Rc<UH0> = split_list_8(v612.clone(), v430);
                                                                            let v614: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                                            let v615: Rc<UH0> = method21(v613, v614);
                                                                            let v616: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                                            let v617: i32 = 0i32;
                                                                            let v618: i32 = method12(v616.clone(), v615, v617);
                                                                            let v619: Rc<Vec<Rc<str>>> = Rc::new(v616.borrow().clone());
                                                                            let v620: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v619).as_ref().clone()));
                                                                            let v621: Rc<str> = Rc::<str>::from((*v620.borrow()).join(&*v612));
                                                                            v621
                                                                        }
                                                                        US8::US8_WordsRoot => {
                                                                            v430
                                                                        }
                                                                    };
                                                                    let v624: bool = std::fs::read_to_string(&*v164).is_ok_and(|value| value == *v623);
                                                                    if v624 {
                                                                        US9::US9_Replaced
                                                                    } else {
                                                                        US9::US9_ReadbackMismatch
                                                                    }
                                                                } else {
                                                                    let v628: bool = std::fs::remove_file(&*v484).is_ok() || !std::path::Path::new(&*v484).exists();
                                                                    if v628 {
                                                                        US9::US9_RenameFailed
                                                                    } else {
                                                                        US9::US9_RenameFailedTemporaryKept
                                                                    }
                                                                }
                                                            } else {
                                                                US9::US9_TemporaryWriteFailed
                                                            }
                                                        };
                                                        match &v635 {
                                                            US9::US9_Replaced => {
                                                                let v636: u64 = v133.len() as u64;
                                                                let v637: u64 = v134.len() as u64;
                                                                US13::US13_RedactApplied(v135, v636, v637)
                                                            }
                                                            _ => {
                                                                let v648: Rc<str> = match v635 {
                                                                    US9::US9_ChangedSinceRead => {
                                                                        let v643: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                                                        v643
                                                                    }
                                                                    US9::US9_ReadbackMismatch => {
                                                                        let v642: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                        v642
                                                                    }
                                                                    US9::US9_RenameFailed => {
                                                                        let v640: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                        v640
                                                                    }
                                                                    US9::US9_RenameFailedTemporaryKept => {
                                                                        let v641: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                                                        v641
                                                                    }
                                                                    US9::US9_TemporaryWriteFailed => {
                                                                        let v639: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                        v639
                                                                    }
                                                                    _ => unreachable!(),
                                                                };
                                                                let v649: Rc<str> = string_concat(&["the state project was not updated: ", &*v648]);
                                                                US13::US13_RedactRefused(v649)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                let v665: Rc<str> = match v369 {
                                                    US9::US9_ChangedSinceRead => {
                                                        let v660: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                                        v660
                                                    }
                                                    US9::US9_ReadbackMismatch => {
                                                        let v659: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                        v659
                                                    }
                                                    US9::US9_RenameFailed => {
                                                        let v657: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                        v657
                                                    }
                                                    US9::US9_RenameFailedTemporaryKept => {
                                                        let v658: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                                        v658
                                                    }
                                                    US9::US9_TemporaryWriteFailed => {
                                                        let v656: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                        v656
                                                    }
                                                    _ => unreachable!(),
                                                };
                                                let v666: Rc<str> = string_concat(&["state/redactions.spi was not written: ", &*v665]);
                                                US13::US13_RedactRefused(v666)
                                            }
                                        }
                                    }
                                    _ => {
                                        let v672: Rc<str> = match v145 {
                                            US7::US7_StateChangedSinceRead => {
                                                let v669: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                                v669
                                            }
                                            US7::US7_StateWriteFailed(v670) => {
                                                let v670: Rc<str> = v670;
                                                v670
                                            }
                                            _ => unreachable!(),
                                        };
                                        US13::US13_RedactRefused(v672)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn tally_add_62(mut v0: Rc<str>, mut v1: Rc<UH4>) -> Rc<UH4> {
    match Rc::unwrap_or_clone(v1) {
        UH4::UH4_Cons(v4, v5, v6) => {
            let mut v4: Rc<str> = v4;
            let v5: i32 = v5;
            let mut v6: Rc<UH4> = v6;
            let v7: bool = &*v4 == &*v0;
            if v7 {
                let v8: i32 = v5.wrapping_add(1i32);
                Rc::new(UH4::UH4_Cons(v4, v8, v6))
            } else {
                let v10: Rc<UH4> = tally_add_62(v0, v6);
                Rc::new(UH4::UH4_Cons(v4, v5, v10))
            }
        }
        UH4::UH4_Nil => {
            let mut v2: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH4::UH4_Cons(v0, 1i32, v2))
        }
    }
}
fn method61(mut v0: Rc<UH1>, mut v1: Rc<UH4>) -> Rc<UH4> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<UH1> = v6;
                let v7: Rc<UH4> = tally_add_62(v4, v1);
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method64(mut v0: Rc<UH0>, mut v1: Rc<UH4>) -> Rc<UH4> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<UH4> = tally_add_62(v2, v1);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method63(mut v0: Rc<UH1>, mut v1: Rc<UH4>) -> Rc<UH4> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<UH1> = v6;
                let v7: Rc<UH4> = method64(v5, v1);
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method65(mut v0: Rc<UH1>, mut v1: Rc<UH4>) -> Rc<UH4> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<UH1> = v6;
                let v7: Rc<UH4> = tally_add_62(v2, v1);
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method66(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<UH1> = v6;
                let v7: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method67(mut v0: Rc<UH4>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH4::UH4_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let mut v4: Rc<UH4> = v4;
                let v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4, v5);
                continue;
            }
            UH4::UH4_Nil => {
                return v1;
            }
        }
    }
}
fn method69(mut v0: Rc<UH4>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH4::UH4_Cons(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let v3: i32 = v3;
            let mut v4: Rc<UH4> = v4;
            let v5: Rc<UH0> = method69(v4, v1);
            let v7: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
            let v8: Rc<str> = string_concat(&[&*v2, "=", &*v7]);
            Rc::new(UH0::UH0_Cons(v8, v5))
        }
        UH4::UH4_Nil => {
            v1
        }
    }
}
fn tally_text_68(mut v0: Rc<UH4>) -> Rc<str> {
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH0> = method69(v0, v1);
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v4: i32 = 0i32;
    let v5: i32 = method12(v3.clone(), v2, v4);
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from((*v7.borrow()).join(&*v8));
    v9
}
fn summary_60(mut v0: Rc<UH1>) -> Rc<str> {
    let mut v1: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH4> = method61(v0.clone(), v1);
    let v3: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH4> = method63(v0.clone(), v3);
    let v5: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH4> = method65(v0.clone(), v5);
    let v7: i32 = 0i32;
    let v8: i32 = method66(v0, v7);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
    let mut v12: i32 = 0i32;
    let mut v13: i32 = method67(v6, v12);
    let mut v14: Rc<str> = std::rc::Rc::<str>::from((v13).to_string());
    let mut v17: Rc<str> = tally_text_68(v2);
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(v17.replace(&*v19, &*v20));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(v21.replace(&*v18, &*v22));
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = Rc::<str>::from(v23.replace(&*v24, &*v25));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = Rc::<str>::from(v26.replace(&*v27, &*v28));
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = Rc::<str>::from(v29.replace(&*v30, &*v31));
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = concat5_2(v18.clone(), v32, v18.clone(), v33.clone(), v33.clone());
    let mut v37: Rc<str> = tally_text_68(v4);
    let mut v38: Rc<str> = Rc::<str>::from(v37.replace(&*v19, &*v20));
    let v39: Rc<str> = Rc::<str>::from(v38.replace(&*v18, &*v22));
    let mut v40: Rc<str> = Rc::<str>::from(v39.replace(&*v24, &*v25));
    let v41: Rc<str> = Rc::<str>::from(v40.replace(&*v27, &*v28));
    let v42: Rc<str> = Rc::<str>::from(v41.replace(&*v30, &*v31));
    let v43: Rc<str> = concat5_2(v18.clone(), v42, v18, v33.clone(), v33);
    let v44: Rc<str> = string_concat(&["notes=", &*v9, " items=", &*v14, " by_author=", &*v34, " roles=", &*v43]);
    v44
}
fn redact_entry_26() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: i32 = 0i32;
    let mut v2: Rc<UH0> = arguments_from_27(v1);
    let v3: Rc<UH0> = words_after_30(v0, v2);
    let (mut v4, mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, Rc<str>, US12, bool) = request_of_31(&v3);
    let mut v9: bool = v8 == false;
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile redact <root> --terms <file outside the backlog> --as <lane> [--apply]"); } LIT.with(|lit| lit.clone()) };
        let mut v11: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v10 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v11
    } else {
        let mut v12: US13 = redaction_done_33(v4, v5, v6, v7, v8);
        match v12 {
            US13::US13_RedactApplied(v18, v19, v20) => {
                let v18: Rc<UH1> = v18;
                let v19: u64 = v19;
                let v20: u64 = v20;
                let v21: Rc<str> = summary_60(v18);
                let mut v24: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
                let v27: Rc<str> = std::rc::Rc::<str>::from((v20).to_string());
                let mut v30: Rc<str> = std::rc::Rc::<str>::from((0u64).to_string());
                let v32: Rc<str> = string_concat(&["rig agile redact ok ", &*v21, " audit=state/redactions.spi bytes_before=", &*v24, " bytes_after=", &*v27, " elapsed_ms=", &*v30]);
                let v33: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v32.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                v33
            }
            US13::US13_RedactPlanned(v13) => {
                let mut v13: Rc<UH1> = v13;
                let mut v14: Rc<str> = summary_60(v13);
                let v16: Rc<str> = string_concat(&["rig agile redact plan ", &*v14, " apply=--apply"]);
                let mut v17: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v16.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                v17
            }
            US13::US13_RedactRefused(v34) => {
                let mut v34: Rc<str> = v34;
                let v35: Rc<str> = string_concat(&["agile redact rejected: ", &*v34]);
                let v36: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v35 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v36
            }
        }
    }
}
fn closure3() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        redact_entry_26()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_note_edit_run() -> i32 {
    closure0()()
}
pub fn rig_agile_redact_run() -> i32 {
    closure3()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
