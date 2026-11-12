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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
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
    US4_ScanPlain,
    US4_ScanQuoted,
    US4_ScanEscaped,
}
#[derive(Clone)]
enum US5 {
    US5_TokenRendered(Rc<str>),
    US5_TokenRejected,
}
#[derive(Clone)]
enum US6 {
    US6_Replaced,
    US6_TemporaryWriteFailed,
    US6_RenameFailed,
    US6_RenameFailedTemporaryKept,
    US6_ReadbackMismatch,
    US6_ChangedSinceRead,
}
#[derive(Clone)]
enum US7 {
    US7_WordsRoot,
    US7_SymbolsRoot,
}
fn agile_edit_failure_1(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v4: Rc<str> = string_concat(&["agile edit ", &*v1, " rejected: ", &*v0]);
    let v5: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v4 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
    v5
}
fn argument_owned_2(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    v1
}
fn concat5_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn loop_4(mut v0: Rc<str>) -> Rc<str> {
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
fn method5(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_6(mut v0: Rc<str>) -> Rc<str> {
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
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
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
fn loop_8(mut v0: Rc<str>) -> Rc<str> {
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
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method11(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
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
        while method11(v7, &v9) {
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
        method10(v16, v17)
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method12(v3, v1);
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
                        let mut v30: US3 = US3::US3_HeadUnion;
                        US2::US2_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let mut v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let mut v34: US3 = US3::US3_HeadUnion;
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
fn method13(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn field_end_15(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US4) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US4::US4_ScanPlain => {
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
                let v25: US4 = match v3 {
                    US4::US4_ScanEscaped => {
                        US4::US4_ScanQuoted
                    }
                    US4::US4_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US4::US4_ScanQuoted
                        } else {
                            US4::US4_ScanPlain
                        }
                    }
                    US4::US4_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US4::US4_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US4::US4_ScanPlain
                            } else {
                                US4::US4_ScanQuoted
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
            let v5: US4 = US4::US4_ScanPlain;
            let v6: i32 = field_end_15(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_16(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
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
                        let mut v30: US3 = US3::US3_HeadUnion;
                        US2::US2_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let mut v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let mut v34: US3 = US3::US3_HeadUnion;
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
fn replace_records_unchanged_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US6 {
    let v3: bool = rig_agile_records::rig_agile_records_unchanged(&*v0, &*v1);
    if v3 {
        let v4: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v6));
        let mut v8: Rc<str> = loop_4(v7);
        let mut v9: Option<usize> = v8.rfind(&*v6);
        let v10: Option<usize> = method5(v9);
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
                let v26: Rc<str> = loop_6(v23);
                US1::US1_StateFile(v26)
            } else {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v29: bool = v23.ends_with(&*v28);
                if v29 {
                    let mut v30: Rc<str> = loop_7(v23);
                    US1::US1_HistoryFile(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let mut v33: bool = v23.ends_with(&*v32);
                    if v33 {
                        let mut v34: Rc<str> = loop_8(v23);
                        US1::US1_ArchiveFile(v34)
                    } else {
                        US1::US1_OtherFile
                    }
                }
            }
        };
        let v63: US7 = match v40 {
            US1::US1_ArchiveFile(v53) => {
                let v53: Rc<str> = v53;
                let v54: Rc<str> = string_concat(&[&*v53, "/state/spiral.toml"]);
                let v55: bool = std::path::Path::new(&*v54).is_file();
                if v55 {
                    US7::US7_SymbolsRoot
                } else {
                    US7::US7_WordsRoot
                }
            }
            US1::US1_HistoryFile(v47) => {
                let v47: Rc<str> = v47;
                let v48: Rc<str> = string_concat(&[&*v47, "/state/spiral.toml"]);
                let v49: bool = std::path::Path::new(&*v48).is_file();
                if v49 {
                    US7::US7_SymbolsRoot
                } else {
                    US7::US7_WordsRoot
                }
            }
            US1::US1_OtherFile => {
                US7::US7_WordsRoot
            }
            US1::US1_StateFile(v41) => {
                let v41: Rc<str> = v41;
                let v42: Rc<str> = string_concat(&[&*v41, "/state/spiral.toml"]);
                let v43: bool = std::path::Path::new(&*v42).is_file();
                if v43 {
                    US7::US7_SymbolsRoot
                } else {
                    US7::US7_WordsRoot
                }
            }
        };
        let v75: Rc<str> = match v63 {
            US7::US7_SymbolsRoot => {
                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v65: Rc<UH0> = split_list_9(v64.clone(), v2.clone());
                let v66: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v67: Rc<UH0> = method18(v65, v66);
                let v68: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v69: i32 = 0i32;
                let v70: i32 = method13(v68.clone(), v67, v69);
                let v71: Rc<Vec<Rc<str>>> = Rc::new(v68.borrow().clone());
                let v72: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v71).as_ref().clone()));
                let v73: Rc<str> = Rc::<str>::from((*v72.borrow()).join(&*v64));
                v73
            }
            US7::US7_WordsRoot => {
                v2.clone()
            }
        };
        let v76: bool = std::fs::write(&*v4, &*v75).is_ok();
        if v76 {
            let v77: i32 = 40i32;
            let v78: bool = renamed_within_19(v0.clone(), v4.clone(), v77);
            if v78 {
                let v79: Rc<str> = Rc::<str>::from(v0.replace(&*v5, &*v6));
                let mut v80: Rc<str> = loop_4(v79);
                let v81: Option<usize> = v80.rfind(&*v6);
                let v82: Option<usize> = method5(v81);
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
                        let v97: Rc<str> = loop_6(v94);
                        US1::US1_StateFile(v97)
                    } else {
                        let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v100: bool = v94.ends_with(&*v99);
                        if v100 {
                            let v101: Rc<str> = loop_7(v94);
                            US1::US1_HistoryFile(v101)
                        } else {
                            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v104: bool = v94.ends_with(&*v103);
                            if v104 {
                                let v105: Rc<str> = loop_8(v94);
                                US1::US1_ArchiveFile(v105)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                let v134: US7 = match v111 {
                    US1::US1_ArchiveFile(v124) => {
                        let v124: Rc<str> = v124;
                        let v125: Rc<str> = string_concat(&[&*v124, "/state/spiral.toml"]);
                        let v126: bool = std::path::Path::new(&*v125).is_file();
                        if v126 {
                            US7::US7_SymbolsRoot
                        } else {
                            US7::US7_WordsRoot
                        }
                    }
                    US1::US1_HistoryFile(v118) => {
                        let v118: Rc<str> = v118;
                        let v119: Rc<str> = string_concat(&[&*v118, "/state/spiral.toml"]);
                        let v120: bool = std::path::Path::new(&*v119).is_file();
                        if v120 {
                            US7::US7_SymbolsRoot
                        } else {
                            US7::US7_WordsRoot
                        }
                    }
                    US1::US1_OtherFile => {
                        US7::US7_WordsRoot
                    }
                    US1::US1_StateFile(v112) => {
                        let v112: Rc<str> = v112;
                        let v113: Rc<str> = string_concat(&[&*v112, "/state/spiral.toml"]);
                        let v114: bool = std::path::Path::new(&*v113).is_file();
                        if v114 {
                            US7::US7_SymbolsRoot
                        } else {
                            US7::US7_WordsRoot
                        }
                    }
                };
                let v146: Rc<str> = match v134 {
                    US7::US7_SymbolsRoot => {
                        let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v136: Rc<UH0> = split_list_9(v135.clone(), v2);
                        let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v138: Rc<UH0> = method18(v136, v137);
                        let v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v140: i32 = 0i32;
                        let v141: i32 = method13(v139.clone(), v138, v140);
                        let v142: Rc<Vec<Rc<str>>> = Rc::new(v139.borrow().clone());
                        let v143: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v142).as_ref().clone()));
                        let v144: Rc<str> = Rc::<str>::from((*v143.borrow()).join(&*v135));
                        v144
                    }
                    US7::US7_WordsRoot => {
                        v2
                    }
                };
                let v147: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v146);
                if v147 {
                    US6::US6_Replaced
                } else {
                    US6::US6_ReadbackMismatch
                }
            } else {
                let v151: bool = std::fs::remove_file(&*v4).is_ok() || !std::path::Path::new(&*v4).exists();
                if v151 {
                    US6::US6_RenameFailed
                } else {
                    US6::US6_RenameFailedTemporaryKept
                }
            }
        } else {
            US6::US6_TemporaryWriteFailed
        }
    } else {
        US6::US6_ChangedSinceRead
    }
}
fn agile_edit_entry_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = argument_owned_2(v0);
    let mut v2: i32 = agile_field_rendering::rig_agile_edit_field_position(&*v1);
    let v3: bool = v2 < 0i32;
    if v3 {
        let v4: Rc<str> = agile_field_rendering::rig_agile_edit_field_rule(&*v1);
        let v5: i32 = 2i32;
        let v6: Rc<str> = argument_owned_2(v5);
        agile_edit_failure_1(v4, v6)
    } else {
        let mut v8: i32 = 1i32;
        let mut v9: Rc<str> = argument_owned_2(v8);
        let v10: i32 = 2i32;
        let mut v11: Rc<str> = argument_owned_2(v10);
        let mut v12: i32 = 4i32;
        let v13: Rc<str> = argument_owned_2(v12);
        let v14: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = concat5_3(v9, v15, v16.clone(), v16.clone(), v16.clone());
        let v18: bool = std::path::Path::new(&*v17).is_file();
        let v21: US0 = if v18 {
            US0::US0_FileReadable(v17.clone())
        } else {
            US0::US0_FileMissing(v17.clone())
        };
        match v21 {
            US0::US0_FileMissing(v22) => {
                let v22: Rc<str> = v22;
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                agile_edit_failure_1(v23, v11)
            }
            US0::US0_FileReadable(v25) => {
                let v25: Rc<str> = v25;
                let v26: Rc<str> = std::fs::read_to_string(&*v25).unwrap_or_default().into();
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let v29: Rc<str> = Rc::<str>::from(v25.replace(&*v27, &*v28));
                let mut v30: Rc<str> = loop_4(v29);
                let v31: Option<usize> = v30.rfind(&*v28);
                let v32: Option<usize> = method5(v31);
                let mut v33: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v34: Option<i32> = v32.map(|x| v33(x));
                let v36: i32 = v34.unwrap_or(-1i32);
                let v37: bool = v36 < 0i32;
                let v41: bool = if v37 {
                    true
                } else {
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v39: bool = v30.ends_with(&*v38);
                    let v40: bool = v39 == false;
                    v40
                };
                let v62: US1 = if v41 {
                    US1::US1_OtherFile
                } else {
                    let v43: usize = ((0i32) as usize);
                    let v44: usize = ((v36) as usize);
                    let v45: Rc<str> = Rc::<str>::from(&(v30)[v43..v44]);
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v47: bool = v45.ends_with(&*v46);
                    if v47 {
                        let v48: Rc<str> = loop_6(v45);
                        US1::US1_StateFile(v48)
                    } else {
                        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v51: bool = v45.ends_with(&*v50);
                        if v51 {
                            let v52: Rc<str> = loop_7(v45);
                            US1::US1_HistoryFile(v52)
                        } else {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v55: bool = v45.ends_with(&*v54);
                            if v55 {
                                let v56: Rc<str> = loop_8(v45);
                                US1::US1_ArchiveFile(v56)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                let v73: Rc<str> = match v62 {
                    US1::US1_OtherFile => {
                        v26
                    }
                    _ => {
                        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v64: Rc<UH0> = split_list_9(v63.clone(), v26);
                        let v65: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v66: Rc<UH0> = method12(v64, v65);
                        let v67: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v68: i32 = 0i32;
                        let v69: i32 = method13(v67.clone(), v66, v68);
                        let v70: Rc<Vec<Rc<str>>> = Rc::new(v67.borrow().clone());
                        let v71: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v70).as_ref().clone()));
                        let v72: Rc<str> = Rc::<str>::from((*v71.borrow()).join(&*v63));
                        v72
                    }
                };
                let v74: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v73);
                let v76: Rc<str> = string_concat(&["Task (\"", &*v11, "\","]);
                let v77: i32 = v74.find(&*v76).map(|offset| offset as i32).unwrap_or(-1);
                let v78: bool = v77 < 0i32;
                if v78 {
                    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                    agile_edit_failure_1(v79, v11)
                } else {
                    let v81: i32 = (v74.clone().len() as i32);
                    let v82: i32 = v77.wrapping_add(6i32);
                    let mut v83: i32 = field_start_14(v74.clone(), v82, v81, v2);
                    let v84: US4 = US4::US4_ScanPlain;
                    let v85: i32 = field_end_15(v74.clone(), v83, v81, v84);
                    let v86: Rc<str> = std::rc::Rc::<str>::from(v74.get(v83 as usize..v85 as usize).unwrap_or(""));
                    let v87: i32 = 5i32;
                    let v88: Rc<str> = argument_owned_2(v87);
                    let v89: bool = &*v88 == "--expect";
                    let v92: Rc<str> = if v89 {
                        let v90: i32 = 6i32;
                        argument_owned_2(v90)
                    } else {
                        v16
                    };
                    let v93: Rc<str> = agile_field_rendering::rig_agile_edit_field_token(&*v1, &*v86, &*v92);
                    let v94: bool = &*v93 == "";
                    let v97: US5 = if v94 {
                        US5::US5_TokenRejected
                    } else {
                        US5::US5_TokenRendered(v93)
                    };
                    let v98: bool = &*v92 == "";
                    let v102: bool = if v98 {
                        true
                    } else {
                        match v97 {
                            US5::US5_TokenRendered(v99) => {
                                let v99: Rc<str> = v99;
                                let v100: bool = &*v99 == &*v86;
                                v100
                            }
                            _ => {
                                false
                            }
                        }
                    };
                    let v105: Rc<str> = string_concat(&["field ", &*v1, " changed since it was read; it now reads ", &*v86]);
                    let v106: bool = v102 == false;
                    if v106 {
                        agile_edit_failure_1(v105, v11)
                    } else {
                        let v108: Rc<str> = agile_field_rendering::rig_agile_edit_field_token(&*v1, &*v86, &*v13);
                        let v109: bool = &*v108 == "";
                        let v112: US5 = if v109 {
                            US5::US5_TokenRejected
                        } else {
                            US5::US5_TokenRendered(v108)
                        };
                        match v112 {
                            US5::US5_TokenRejected => {
                                let v113: Rc<str> = agile_field_rendering::rig_agile_edit_field_rule(&*v1);
                                agile_edit_failure_1(v113, v11)
                            }
                            US5::US5_TokenRendered(v115) => {
                                let v115: Rc<str> = v115;
                                let v116: Rc<str> = std::rc::Rc::<str>::from(v74.get(0i32 as usize..v83 as usize).unwrap_or(""));
                                let v118: Rc<str> = std::rc::Rc::<str>::from(v74.get(v85 as usize..v81 as usize).unwrap_or(""));
                                let v119: Rc<str> = string_concat(&[&*v116, &*v115, &*v118]);
                                let v120: US6 = replace_records_unchanged_17(v17, v74.clone(), v119.clone());
                                match &v120 {
                                    US6::US6_Replaced => {
                                        let v121: u64 = v74.len() as u64;
                                        let v122: u64 = v119.len() as u64;
                                        let v123: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v14);
                                        let v127: Rc<str> = std::rc::Rc::<str>::from((v121).to_string());
                                        let v130: Rc<str> = std::rc::Rc::<str>::from((v122).to_string());
                                        let v133: Rc<str> = std::rc::Rc::<str>::from((v123).to_string());
                                        let v135: Rc<str> = string_concat(&["rig agile edit ok id=", &*v11, " field=", &*v1, " bytes_before=", &*v127, " bytes_after=", &*v130, " elapsed_ms=", &*v133]);
                                        let v136: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v135.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                        v136
                                    }
                                    _ => {
                                        let v146: Rc<str> = match v120 {
                                            US6::US6_ChangedSinceRead => {
                                                let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                                v141
                                            }
                                            US6::US6_ReadbackMismatch => {
                                                let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                v140
                                            }
                                            US6::US6_RenameFailed => {
                                                let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                v138
                                            }
                                            US6::US6_RenameFailedTemporaryKept => {
                                                let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                                v139
                                            }
                                            US6::US6_TemporaryWriteFailed => {
                                                let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                v137
                                            }
                                            _ => unreachable!(),
                                        };
                                        agile_edit_failure_1(v146, v11)
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
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_edit_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_edit_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
