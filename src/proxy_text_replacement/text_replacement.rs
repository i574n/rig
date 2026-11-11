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
    US0_ReplacementReceipt(Rc<str>),
    US0_ReplacementRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_CheckReplacement,
    US2_ApplyReplacement,
}
#[derive(Clone)]
enum US1 {
    US1_Valid(US2),
    US1_Invalid,
}
#[derive(Clone)]
enum US4 {
    US4_ExactlyOneMatch,
    US4_ExactlyMatches(u64),
    US4_EveryMatch,
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US4),
    US3_Invalid,
}
#[derive(Clone)]
enum US5 {
    US5_StateFile(Rc<str>),
    US5_HistoryFile(Rc<str>),
    US5_ArchiveFile(Rc<str>),
    US5_OtherFile,
}
#[derive(Clone)]
enum US6 {
    US6_WordsRoot,
    US6_SymbolsRoot,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US8 {
    US8_HeadInl,
    US8_HeadUnion,
    US8_HeadOpen,
    US8_HeadType,
}
#[derive(Clone)]
enum US7 {
    US7_KeywordHead(US8),
    US7_SymbolHead(US8),
    US7_OtherLine,
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
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn proxy_argument_count_2() -> i32 {
    let mut v0: i32 = command_line_argument_count_3();
    let v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn regular_file_4(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
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
fn method6(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
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
fn loop_8(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
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
fn loop_9(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
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
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method12(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method12(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method11(v16, v17)
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method13(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US7 = if v6 {
                let v7: US8 = US8::US8_HeadType;
                US7::US7_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US8 = US8::US8_HeadType;
                    US7::US7_SymbolHead(v11)
                } else {
                    US7::US7_OtherLine
                }
            };
            let v27: US7 = match &v15 {
                US7::US7_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US8 = US8::US8_HeadInl;
                        US7::US7_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US8 = US8::US8_HeadInl;
                            US7::US7_SymbolHead(v22)
                        } else {
                            US7::US7_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US7 = match &v27 {
                US7::US7_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US8 = US8::US8_HeadUnion;
                        US7::US7_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US8 = US8::US8_HeadUnion;
                            US7::US7_SymbolHead(v34)
                        } else {
                            US7::US7_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US7 = match &v39 {
                US7::US7_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US8 = US8::US8_HeadOpen;
                        US7::US7_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US8 = US8::US8_HeadOpen;
                            US7::US7_SymbolHead(v46)
                        } else {
                            US7::US7_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US7::US7_KeywordHead(v52) => {
                    let v52: US8 = v52;
                    let mut v59: Rc<str> = match &v52 {
                        US8::US8_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US8::US8_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US8::US8_HeadType => {
                            v5
                        }
                        US8::US8_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v60: i32 = (v59.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US8::US8_HeadInl => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v61
                        }
                        US8::US8_HeadOpen => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                        US8::US8_HeadType => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US8::US8_HeadUnion => {
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
fn method14(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn renamed_within_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn text_replace_outcome_1() -> US0 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
    let mut v2: bool = &*v1 == "check";
    let v10: US1 = if v2 {
        let v3: US2 = US2::US2_CheckReplacement;
        US1::US1_Valid(v3)
    } else {
        let v5: bool = &*v1 == "apply";
        if v5 {
            let mut v6: US2 = US2::US2_ApplyReplacement;
            US1::US1_Valid(v6)
        } else {
            US1::US1_Invalid
        }
    };
    let v11: i32 = proxy_argument_count_2();
    let v12: bool = 6i32 == v11;
    let v37: US3 = if v12 {
        let v13: US4 = US4::US4_ExactlyOneMatch;
        US3::US3_Valid(v13)
    } else {
        let v15: bool = 7i32 == v11;
        if v15 {
            let v16: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 6i32 as usize).unwrap_or_default());
            let v17: bool = &*v16 == "--all";
            if v17 {
                let v18: US4 = US4::US4_EveryMatch;
                US3::US3_Valid(v18)
            } else {
                US3::US3_Invalid
            }
        } else {
            let v22: bool = 8i32 == v11;
            if v22 {
                let v23: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 7i32 as usize).unwrap_or_default());
                let v24: u64 = v23.parse::<u64>().unwrap_or(0);
                let v25: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 6i32 as usize).unwrap_or_default());
                let v26: bool = &*v25 == "--expect";
                if v26 {
                    let v27: bool = 0u64 < v24;
                    if v27 {
                        let v28: US4 = US4::US4_ExactlyMatches(v24);
                        US3::US3_Valid(v28)
                    } else {
                        US3::US3_Invalid
                    }
                } else {
                    US3::US3_Invalid
                }
            } else {
                US3::US3_Invalid
            }
        }
    };
    match &v10 {
        US1::US1_Invalid => {
            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace mode must be check or apply"); } LIT.with(|lit| lit.clone()) };
            US0::US0_ReplacementRejected(v38)
        }
        _ => {
            match &v37 {
                US3::US3_Invalid => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace expects <check|apply> <root> <relative> <old> <new> [--expect N | --all]"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_ReplacementRejected(v40)
                }
                _ => {
                    match v10 {
                        US1::US1_Valid(v42) => {
                            let v42: US2 = v42;
                            match v37 {
                                US3::US3_Valid(v43) => {
                                    let v43: US4 = v43;
                                    let v44: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 4i32 as usize).unwrap_or_default());
                                    let v45: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 5i32 as usize).unwrap_or_default());
                                    let v46: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
                                    let v47: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 3i32 as usize).unwrap_or_default());
                                    let v48: Rc<str> = rig_rust_std_fs::rooted_path(std::path::Path::new(&*v46), &*v47).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                    let v49: bool = &*v48 == "";
                                    if v49 {
                                        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace target escapes the root"); } LIT.with(|lit| lit.clone()) };
                                        US0::US0_ReplacementRejected(v50)
                                    } else {
                                        let v52: bool = &*v44 == "";
                                        if v52 {
                                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace old text is empty"); } LIT.with(|lit| lit.clone()) };
                                            US0::US0_ReplacementRejected(v53)
                                        } else {
                                            let v55: bool = &*v44 == &*v45;
                                            if v55 {
                                                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace old and new text are identical"); } LIT.with(|lit| lit.clone()) };
                                                US0::US0_ReplacementRejected(v56)
                                            } else {
                                                let v58: bool = regular_file_4(v48.clone());
                                                if v58 {
                                                    let mut v59: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v48).unwrap_or_default());
                                                    let v60: u64 = v59.match_indices(&*v44).count() as u64;
                                                    let v67: bool = match &v43 {
                                                        US4::US4_EveryMatch => {
                                                            let v64: bool = 0u64 < v60;
                                                            v64
                                                        }
                                                        US4::US4_ExactlyMatches(v62) => {
                                                            let v62: u64 = *v62;
                                                            let v63: bool = v60 == v62;
                                                            v63
                                                        }
                                                        US4::US4_ExactlyOneMatch => {
                                                            let v61: bool = v60 == 1u64;
                                                            v61
                                                        }
                                                    };
                                                    if v67 {
                                                        let v70: Rc<str> = match &v43 {
                                                            US4::US4_EveryMatch => {
                                                                let v68: Rc<str> = Rc::<str>::from(v59.replace(&*v44, &*v45));
                                                                v68
                                                            }
                                                            _ => {
                                                                let v69: Rc<str> = std::rc::Rc::<str>::from(v59.replacen(&*v44, &*v45, v60 as usize));
                                                                v69
                                                            }
                                                        };
                                                        match v42 {
                                                            US2::US2_ApplyReplacement => {
                                                                let v97: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v48).unwrap_or_default());
                                                                let v98: bool = &*v97 != &*v59;
                                                                if v98 {
                                                                    let v100: Rc<str> = string_concat(&["text-replace: ", &*v48, " changed since it was read; nothing written, run it again"]);
                                                                    US0::US0_ReplacementRejected(v100)
                                                                } else {
                                                                    let v102: Rc<str> = string_concat(&[&*v48, ".pending"]);
                                                                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                                                    let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                                                                    let v105: Rc<str> = Rc::<str>::from(v102.replace(&*v103, &*v104));
                                                                    let mut v106: Rc<str> = loop_5(v105);
                                                                    let v107: Option<usize> = v106.rfind(&*v104);
                                                                    let v108: Option<usize> = method6(v107);
                                                                    let mut v109: Rc<dyn Fn((usize)) -> i32> = closure1();
                                                                    let mut v110: Option<i32> = v108.map(|x| v109(x));
                                                                    let v112: i32 = v110.unwrap_or(-1i32);
                                                                    let v113: bool = v112 < 0i32;
                                                                    let v117: bool = if v113 {
                                                                        true
                                                                    } else {
                                                                        let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                                        let v115: bool = v106.ends_with(&*v114);
                                                                        let v116: bool = v115 == false;
                                                                        v116
                                                                    };
                                                                    let v138: US5 = if v117 {
                                                                        US5::US5_OtherFile
                                                                    } else {
                                                                        let v119: usize = ((0i32) as usize);
                                                                        let v120: usize = ((v112) as usize);
                                                                        let v121: Rc<str> = Rc::<str>::from(&(v106)[v119..v120]);
                                                                        let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                                        let v123: bool = v121.ends_with(&*v122);
                                                                        if v123 {
                                                                            let v124: Rc<str> = loop_7(v121);
                                                                            US5::US5_StateFile(v124)
                                                                        } else {
                                                                            let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                                            let v127: bool = v121.ends_with(&*v126);
                                                                            if v127 {
                                                                                let v128: Rc<str> = loop_8(v121);
                                                                                US5::US5_HistoryFile(v128)
                                                                            } else {
                                                                                let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                                                let v131: bool = v121.ends_with(&*v130);
                                                                                if v131 {
                                                                                    let v132: Rc<str> = loop_9(v121);
                                                                                    US5::US5_ArchiveFile(v132)
                                                                                } else {
                                                                                    US5::US5_OtherFile
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    let v161: US6 = match v138 {
                                                                        US5::US5_ArchiveFile(v151) => {
                                                                            let v151: Rc<str> = v151;
                                                                            let v152: Rc<str> = string_concat(&[&*v151, "/state/spiral.toml"]);
                                                                            let v153: bool = std::path::Path::new(&*v152).is_file();
                                                                            if v153 {
                                                                                US6::US6_SymbolsRoot
                                                                            } else {
                                                                                US6::US6_WordsRoot
                                                                            }
                                                                        }
                                                                        US5::US5_HistoryFile(v145) => {
                                                                            let v145: Rc<str> = v145;
                                                                            let v146: Rc<str> = string_concat(&[&*v145, "/state/spiral.toml"]);
                                                                            let v147: bool = std::path::Path::new(&*v146).is_file();
                                                                            if v147 {
                                                                                US6::US6_SymbolsRoot
                                                                            } else {
                                                                                US6::US6_WordsRoot
                                                                            }
                                                                        }
                                                                        US5::US5_OtherFile => {
                                                                            US6::US6_WordsRoot
                                                                        }
                                                                        US5::US5_StateFile(v139) => {
                                                                            let v139: Rc<str> = v139;
                                                                            let v140: Rc<str> = string_concat(&[&*v139, "/state/spiral.toml"]);
                                                                            let v141: bool = std::path::Path::new(&*v140).is_file();
                                                                            if v141 {
                                                                                US6::US6_SymbolsRoot
                                                                            } else {
                                                                                US6::US6_WordsRoot
                                                                            }
                                                                        }
                                                                    };
                                                                    let v173: Rc<str> = match v161 {
                                                                        US6::US6_SymbolsRoot => {
                                                                            let v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                                            let v163: Rc<UH0> = split_list_10(v162.clone(), v70.clone());
                                                                            let v164: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                                            let v165: Rc<UH0> = method13(v163, v164);
                                                                            let v166: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                                            let v167: i32 = 0i32;
                                                                            let v168: i32 = method14(v166.clone(), v165, v167);
                                                                            let v169: Rc<Vec<Rc<str>>> = Rc::new(v166.borrow().clone());
                                                                            let v170: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v169).as_ref().clone()));
                                                                            let v171: Rc<str> = Rc::<str>::from((*v170.borrow()).join(&*v162));
                                                                            v171
                                                                        }
                                                                        US6::US6_WordsRoot => {
                                                                            v70.clone()
                                                                        }
                                                                    };
                                                                    let v174: bool = std::fs::write(&*v102, &*v173).is_ok();
                                                                    let v255: US9 = if v174 {
                                                                        let v175: i32 = 40i32;
                                                                        let v176: bool = renamed_within_15(v48.clone(), v102.clone(), v175);
                                                                        if v176 {
                                                                            let v177: Rc<str> = Rc::<str>::from(v48.replace(&*v103, &*v104));
                                                                            let mut v178: Rc<str> = loop_5(v177);
                                                                            let v179: Option<usize> = v178.rfind(&*v104);
                                                                            let v180: Option<usize> = method6(v179);
                                                                            let mut v181: Option<i32> = v180.map(|x| v109(x));
                                                                            let v183: i32 = v181.unwrap_or(-1i32);
                                                                            let v184: bool = v183 < 0i32;
                                                                            let v188: bool = if v184 {
                                                                                true
                                                                            } else {
                                                                                let v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                                                let v186: bool = v178.ends_with(&*v185);
                                                                                let v187: bool = v186 == false;
                                                                                v187
                                                                            };
                                                                            let v209: US5 = if v188 {
                                                                                US5::US5_OtherFile
                                                                            } else {
                                                                                let v190: usize = ((0i32) as usize);
                                                                                let v191: usize = ((v183) as usize);
                                                                                let v192: Rc<str> = Rc::<str>::from(&(v178)[v190..v191]);
                                                                                let v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                                                let v194: bool = v192.ends_with(&*v193);
                                                                                if v194 {
                                                                                    let v195: Rc<str> = loop_7(v192);
                                                                                    US5::US5_StateFile(v195)
                                                                                } else {
                                                                                    let v197: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                                                    let v198: bool = v192.ends_with(&*v197);
                                                                                    if v198 {
                                                                                        let v199: Rc<str> = loop_8(v192);
                                                                                        US5::US5_HistoryFile(v199)
                                                                                    } else {
                                                                                        let v201: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                                                        let v202: bool = v192.ends_with(&*v201);
                                                                                        if v202 {
                                                                                            let v203: Rc<str> = loop_9(v192);
                                                                                            US5::US5_ArchiveFile(v203)
                                                                                        } else {
                                                                                            US5::US5_OtherFile
                                                                                        }
                                                                                    }
                                                                                }
                                                                            };
                                                                            let v232: US6 = match v209 {
                                                                                US5::US5_ArchiveFile(v222) => {
                                                                                    let v222: Rc<str> = v222;
                                                                                    let v223: Rc<str> = string_concat(&[&*v222, "/state/spiral.toml"]);
                                                                                    let v224: bool = std::path::Path::new(&*v223).is_file();
                                                                                    if v224 {
                                                                                        US6::US6_SymbolsRoot
                                                                                    } else {
                                                                                        US6::US6_WordsRoot
                                                                                    }
                                                                                }
                                                                                US5::US5_HistoryFile(v216) => {
                                                                                    let v216: Rc<str> = v216;
                                                                                    let v217: Rc<str> = string_concat(&[&*v216, "/state/spiral.toml"]);
                                                                                    let v218: bool = std::path::Path::new(&*v217).is_file();
                                                                                    if v218 {
                                                                                        US6::US6_SymbolsRoot
                                                                                    } else {
                                                                                        US6::US6_WordsRoot
                                                                                    }
                                                                                }
                                                                                US5::US5_OtherFile => {
                                                                                    US6::US6_WordsRoot
                                                                                }
                                                                                US5::US5_StateFile(v210) => {
                                                                                    let v210: Rc<str> = v210;
                                                                                    let v211: Rc<str> = string_concat(&[&*v210, "/state/spiral.toml"]);
                                                                                    let v212: bool = std::path::Path::new(&*v211).is_file();
                                                                                    if v212 {
                                                                                        US6::US6_SymbolsRoot
                                                                                    } else {
                                                                                        US6::US6_WordsRoot
                                                                                    }
                                                                                }
                                                                            };
                                                                            let v244: Rc<str> = match v232 {
                                                                                US6::US6_SymbolsRoot => {
                                                                                    let v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                                                    let v234: Rc<UH0> = split_list_10(v233.clone(), v70.clone());
                                                                                    let v235: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                                                    let v236: Rc<UH0> = method13(v234, v235);
                                                                                    let v237: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                                                    let v238: i32 = 0i32;
                                                                                    let v239: i32 = method14(v237.clone(), v236, v238);
                                                                                    let v240: Rc<Vec<Rc<str>>> = Rc::new(v237.borrow().clone());
                                                                                    let v241: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v240).as_ref().clone()));
                                                                                    let v242: Rc<str> = Rc::<str>::from((*v241.borrow()).join(&*v233));
                                                                                    v242
                                                                                }
                                                                                US6::US6_WordsRoot => {
                                                                                    v70.clone()
                                                                                }
                                                                            };
                                                                            let v245: bool = std::fs::read_to_string(&*v48).is_ok_and(|value| value == *v244);
                                                                            if v245 {
                                                                                US9::US9_Replaced
                                                                            } else {
                                                                                US9::US9_ReadbackMismatch
                                                                            }
                                                                        } else {
                                                                            let v249: bool = std::fs::remove_file(&*v102).is_ok() || !std::path::Path::new(&*v102).exists();
                                                                            if v249 {
                                                                                US9::US9_RenameFailed
                                                                            } else {
                                                                                US9::US9_RenameFailedTemporaryKept
                                                                            }
                                                                        }
                                                                    } else {
                                                                        US9::US9_TemporaryWriteFailed
                                                                    };
                                                                    match &v255 {
                                                                        US9::US9_Replaced => {
                                                                            let v256: u64 = v59.len() as u64;
                                                                            let v257: u64 = v70.len() as u64;
                                                                            let v259: Rc<str> = string_concat(&["rig proxy text-replace ok mode=apply path=", &*v48, " matches="]);
                                                                            let v260: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                                                                            let v261: Rc<str> = string_concat(&[&*v260, " expected="]);
                                                                            let v268: Rc<str> = match v43 {
                                                                                US4::US4_EveryMatch => {
                                                                                    let v265: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                                    v265
                                                                                }
                                                                                US4::US4_ExactlyMatches(v263) => {
                                                                                    let v263: u64 = v263;
                                                                                    let v264: Rc<str> = std::rc::Rc::<str>::from((v263).to_string());
                                                                                    v264
                                                                                }
                                                                                US4::US4_ExactlyOneMatch => {
                                                                                    let v262: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                                    v262
                                                                                }
                                                                            };
                                                                            let v271: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                                                            let v272: Rc<str> = std::rc::Rc::<str>::from((v256).to_string());
                                                                            let v275: Rc<str> = std::rc::Rc::<str>::from((v257).to_string());
                                                                            let v278: Rc<str> = std::rc::Rc::<str>::from((v271).to_string());
                                                                            let v280: Rc<str> = string_concat(&[&*v259, &*v261, &*v268, " bytes_before=", &*v272, " bytes_after=", &*v275, " elapsed_ms=", &*v278]);
                                                                            US0::US0_ReplacementReceipt(v280)
                                                                        }
                                                                        _ => {
                                                                            let v291: Rc<str> = match v255 {
                                                                                US9::US9_ChangedSinceRead => {
                                                                                    let v286: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                                                                    v286
                                                                                }
                                                                                US9::US9_ReadbackMismatch => {
                                                                                    let v285: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                                    v285
                                                                                }
                                                                                US9::US9_RenameFailed => {
                                                                                    let v283: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                                    v283
                                                                                }
                                                                                US9::US9_RenameFailedTemporaryKept => {
                                                                                    let v284: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                                                                    v284
                                                                                }
                                                                                US9::US9_TemporaryWriteFailed => {
                                                                                    let v282: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                                    v282
                                                                                }
                                                                                _ => unreachable!(),
                                                                            };
                                                                            US0::US0_ReplacementRejected(v291)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            US2::US2_CheckReplacement => {
                                                                let v71: u64 = v59.len() as u64;
                                                                let v72: u64 = v70.len() as u64;
                                                                let v74: Rc<str> = string_concat(&["rig proxy text-replace ok mode=check path=", &*v48, " matches="]);
                                                                let v75: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                                                                let v76: Rc<str> = string_concat(&[&*v75, " expected="]);
                                                                let v83: Rc<str> = match v43 {
                                                                    US4::US4_EveryMatch => {
                                                                        let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                        v80
                                                                    }
                                                                    US4::US4_ExactlyMatches(v78) => {
                                                                        let v78: u64 = v78;
                                                                        let v79: Rc<str> = std::rc::Rc::<str>::from((v78).to_string());
                                                                        v79
                                                                    }
                                                                    US4::US4_ExactlyOneMatch => {
                                                                        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                        v77
                                                                    }
                                                                };
                                                                let v86: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                                                let v87: Rc<str> = std::rc::Rc::<str>::from((v71).to_string());
                                                                let v90: Rc<str> = std::rc::Rc::<str>::from((v72).to_string());
                                                                let v93: Rc<str> = std::rc::Rc::<str>::from((v86).to_string());
                                                                let v95: Rc<str> = string_concat(&[&*v74, &*v76, &*v83, " bytes_before=", &*v87, " bytes_after=", &*v90, " elapsed_ms=", &*v93]);
                                                                US0::US0_ReplacementReceipt(v95)
                                                            }
                                                        }
                                                    } else {
                                                        let v303: Rc<str> = match v43 {
                                                            US4::US4_EveryMatch => {
                                                                let v300: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                v300
                                                            }
                                                            US4::US4_ExactlyMatches(v298) => {
                                                                let v298: u64 = v298;
                                                                let v299: Rc<str> = std::rc::Rc::<str>::from((v298).to_string());
                                                                v299
                                                            }
                                                            US4::US4_ExactlyOneMatch => {
                                                                let v297: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                v297
                                                            }
                                                        };
                                                        let v306: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                                                        let v308: Rc<str> = string_concat(&["text-replace expected ", &*v303, " match(es), observed matches=", &*v306, " (use --expect N or --all for repeated text)"]);
                                                        US0::US0_ReplacementRejected(v308)
                                                    }
                                                } else {
                                                    let v311: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace target is not a regular file"); } LIT.with(|lit| lit.clone()) };
                                                    US0::US0_ReplacementRejected(v311)
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}
fn text_replace_report_0() -> (i32, Rc<str>) {
    let mut v0: US0 = text_replace_outcome_1();
    match v0 {
        US0::US0_ReplacementReceipt(v1) => {
            let v1: Rc<str> = v1;
            (0i32, v1)
        }
        US0::US0_ReplacementRejected(v2) => {
            let mut v2: Rc<str> = v2;
            (2i32, v2)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (i32, Rc<str>)> = Rc::new(move || -> (i32, Rc<str>) {
        text_replace_report_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_text_replace_outcome() -> (i32, Rc<str>) {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
