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
    US0_Replaced,
    US0_TemporaryWriteFailed,
    US0_RenameFailed,
    US0_RenameFailedTemporaryKept,
    US0_ReadbackMismatch,
    US0_ChangedSinceRead,
}
#[derive(Clone)]
enum US1 {
    US1_StateFile(Rc<str>),
    US1_HistoryFile(Rc<str>),
    US1_ArchiveFile(Rc<str>),
    US1_OtherFile,
}
#[derive(Clone)]
enum US2 {
    US2_WordsRoot,
    US2_SymbolsRoot,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US4 {
    US4_HeadInl,
    US4_HeadUnion,
    US4_HeadOpen,
    US4_HeadType,
}
#[derive(Clone)]
enum US3 {
    US3_KeywordHead(US4),
    US3_SymbolHead(US4),
    US3_OtherLine,
}
#[derive(Clone)]
enum US5 {
    US5_MigrationDryRun,
    US5_MigrationApply,
}
#[derive(Clone)]
enum US6 {
    US6_FileReadable(Rc<str>),
    US6_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US7 {
    US7_ScanPlain,
    US7_ScanQuoted,
    US7_ScanEscaped,
}
#[derive(Clone)]
enum US8 {
    US8_StepText(Rc<str>),
    US8_StepRejected(Rc<str>),
}
#[derive(Clone)]
enum US9 {
    US9_PlainTag,
    US9_PlanningTag(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum US10 {
    US10_Valid(US5),
    US10_Invalid,
}
fn migration_receipt_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: u64) -> i32 {
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v14: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let v17: u64 = v5.len() as u64;
    let v18: u64 = v1.len() as u64;
    let mut v19: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v6);
    let v20: Rc<str> = std::rc::Rc::<str>::from((v17).to_string());
    let mut v23: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
    let v26: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
    let v28: Rc<str> = string_concat(&["rig agile migrate-tags ", &*v0, " tasks=", &*v8, " tags_moved=", &*v11, " rejected=", &*v14, " bytes_before=", &*v20, " bytes_after=", &*v23, " elapsed_ms=", &*v26]);
    let v29: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v28.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v29
}
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
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
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(6i32);
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
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_9(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
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
fn method12(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method12(v7, &v9) {
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
        method11(v16, v17)
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method13(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let v15: US3 = if v6 {
                let v7: US4 = US4::US4_HeadType;
                US3::US3_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US4 = US4::US4_HeadType;
                    US3::US3_SymbolHead(v11)
                } else {
                    US3::US3_OtherLine
                }
            };
            let v27: US3 = match &v15 {
                US3::US3_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US4 = US4::US4_HeadInl;
                        US3::US3_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let mut v22: US4 = US4::US4_HeadInl;
                            US3::US3_SymbolHead(v22)
                        } else {
                            US3::US3_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US3 = match &v27 {
                US3::US3_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US4 = US4::US4_HeadUnion;
                        US3::US3_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US4 = US4::US4_HeadUnion;
                            US3::US3_SymbolHead(v34)
                        } else {
                            US3::US3_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US3 = match &v39 {
                US3::US3_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US4 = US4::US4_HeadOpen;
                        US3::US3_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US4 = US4::US4_HeadOpen;
                            US3::US3_SymbolHead(v46)
                        } else {
                            US3::US3_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US3::US3_KeywordHead(v52) => {
                    let v52: US4 = v52;
                    let v59: Rc<str> = match &v52 {
                        US4::US4_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US4::US4_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US4::US4_HeadType => {
                            v5
                        }
                        US4::US4_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v60: i32 = (v59.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US4::US4_HeadInl => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v61
                        }
                        US4::US4_HeadOpen => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                        US4::US4_HeadType => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US4::US4_HeadUnion => {
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
fn renamed_within_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn replace_records_unchanged_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let v3: bool = rig_agile_records::rig_agile_records_unchanged(&*v0, &*v1);
    if v3 {
        let mut v4: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v6));
        let mut v8: Rc<str> = loop_5(v7);
        let mut v9: Option<usize> = v8.rfind(&*v6);
        let v10: Option<usize> = method6(v9);
        let mut v11: Rc<dyn Fn((usize)) -> i32> = closure1();
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
        let v40: US1 = if v19 {
            US1::US1_OtherFile
        } else {
            let v21: usize = ((0i32) as usize);
            let mut v22: usize = ((v14) as usize);
            let mut v23: Rc<str> = Rc::<str>::from(&(v8)[v21..v22]);
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v25: bool = v23.ends_with(&*v24);
            if v25 {
                let v26: Rc<str> = loop_7(v23);
                US1::US1_StateFile(v26)
            } else {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v29: bool = v23.ends_with(&*v28);
                if v29 {
                    let v30: Rc<str> = loop_8(v23);
                    US1::US1_HistoryFile(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v23.ends_with(&*v32);
                    if v33 {
                        let v34: Rc<str> = loop_9(v23);
                        US1::US1_ArchiveFile(v34)
                    } else {
                        US1::US1_OtherFile
                    }
                }
            }
        };
        let v63: US2 = match v40 {
            US1::US1_ArchiveFile(v53) => {
                let v53: Rc<str> = v53;
                let v54: Rc<str> = string_concat(&[&*v53, "/state/spiral.toml"]);
                let v55: bool = std::path::Path::new(&*v54).is_file();
                if v55 {
                    US2::US2_SymbolsRoot
                } else {
                    US2::US2_WordsRoot
                }
            }
            US1::US1_HistoryFile(v47) => {
                let v47: Rc<str> = v47;
                let v48: Rc<str> = string_concat(&[&*v47, "/state/spiral.toml"]);
                let v49: bool = std::path::Path::new(&*v48).is_file();
                if v49 {
                    US2::US2_SymbolsRoot
                } else {
                    US2::US2_WordsRoot
                }
            }
            US1::US1_OtherFile => {
                US2::US2_WordsRoot
            }
            US1::US1_StateFile(v41) => {
                let v41: Rc<str> = v41;
                let v42: Rc<str> = string_concat(&[&*v41, "/state/spiral.toml"]);
                let v43: bool = std::path::Path::new(&*v42).is_file();
                if v43 {
                    US2::US2_SymbolsRoot
                } else {
                    US2::US2_WordsRoot
                }
            }
        };
        let v75: Rc<str> = match v63 {
            US2::US2_SymbolsRoot => {
                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v65: Rc<UH0> = split_list_10(v64.clone(), v2.clone());
                let v66: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v67: Rc<UH0> = method13(v65, v66);
                let v68: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v69: i32 = 0i32;
                let v70: i32 = method14(v68.clone(), v67, v69);
                let v71: Rc<Vec<Rc<str>>> = Rc::new(v68.borrow().clone());
                let v72: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v71).as_ref().clone()));
                let v73: Rc<str> = Rc::<str>::from((*v72.borrow()).join(&*v64));
                v73
            }
            US2::US2_WordsRoot => {
                v2.clone()
            }
        };
        let v76: bool = std::fs::write(&*v4, &*v75).is_ok();
        if v76 {
            let v77: i32 = 40i32;
            let v78: bool = renamed_within_15(v0.clone(), v4.clone(), v77);
            if v78 {
                let v79: Rc<str> = Rc::<str>::from(v0.replace(&*v5, &*v6));
                let mut v80: Rc<str> = loop_5(v79);
                let v81: Option<usize> = v80.rfind(&*v6);
                let v82: Option<usize> = method6(v81);
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
                        let v97: Rc<str> = loop_7(v94);
                        US1::US1_StateFile(v97)
                    } else {
                        let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v100: bool = v94.ends_with(&*v99);
                        if v100 {
                            let v101: Rc<str> = loop_8(v94);
                            US1::US1_HistoryFile(v101)
                        } else {
                            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v104: bool = v94.ends_with(&*v103);
                            if v104 {
                                let v105: Rc<str> = loop_9(v94);
                                US1::US1_ArchiveFile(v105)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                let v134: US2 = match v111 {
                    US1::US1_ArchiveFile(v124) => {
                        let v124: Rc<str> = v124;
                        let v125: Rc<str> = string_concat(&[&*v124, "/state/spiral.toml"]);
                        let v126: bool = std::path::Path::new(&*v125).is_file();
                        if v126 {
                            US2::US2_SymbolsRoot
                        } else {
                            US2::US2_WordsRoot
                        }
                    }
                    US1::US1_HistoryFile(v118) => {
                        let v118: Rc<str> = v118;
                        let v119: Rc<str> = string_concat(&[&*v118, "/state/spiral.toml"]);
                        let v120: bool = std::path::Path::new(&*v119).is_file();
                        if v120 {
                            US2::US2_SymbolsRoot
                        } else {
                            US2::US2_WordsRoot
                        }
                    }
                    US1::US1_OtherFile => {
                        US2::US2_WordsRoot
                    }
                    US1::US1_StateFile(v112) => {
                        let v112: Rc<str> = v112;
                        let v113: Rc<str> = string_concat(&[&*v112, "/state/spiral.toml"]);
                        let v114: bool = std::path::Path::new(&*v113).is_file();
                        if v114 {
                            US2::US2_SymbolsRoot
                        } else {
                            US2::US2_WordsRoot
                        }
                    }
                };
                let v146: Rc<str> = match v134 {
                    US2::US2_SymbolsRoot => {
                        let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v136: Rc<UH0> = split_list_10(v135.clone(), v2);
                        let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v138: Rc<UH0> = method13(v136, v137);
                        let v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v140: i32 = 0i32;
                        let v141: i32 = method14(v139.clone(), v138, v140);
                        let v142: Rc<Vec<Rc<str>>> = Rc::new(v139.borrow().clone());
                        let v143: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v142).as_ref().clone()));
                        let v144: Rc<str> = Rc::<str>::from((*v143.borrow()).join(&*v135));
                        v144
                    }
                    US2::US2_WordsRoot => {
                        v2
                    }
                };
                let v147: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v146);
                if v147 {
                    US0::US0_Replaced
                } else {
                    US0::US0_ReadbackMismatch
                }
            } else {
                let v151: bool = std::fs::remove_file(&*v4).is_ok() || !std::path::Path::new(&*v4).exists();
                if v151 {
                    US0::US0_RenameFailed
                } else {
                    US0::US0_RenameFailedTemporaryKept
                }
            }
        } else {
            US0::US0_TemporaryWriteFailed
        }
    } else {
        US0::US0_ChangedSinceRead
    }
}
fn migration_written_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: u64) -> i32 {
    let v7: bool = v2 == 0i32;
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok unchanged"); } LIT.with(|lit| lit.clone()) };
        migration_receipt_3(v8, v1, v2, v3, v4, v5, v6)
    } else {
        let v10: US0 = replace_records_unchanged_4(v0, v5.clone(), v1.clone());
        match &v10 {
            US0::US0_Replaced => {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                migration_receipt_3(v11, v1, v2, v3, v4, v5, v6)
            }
            _ => {
                let mut v22: Rc<str> = match v10 {
                    US0::US0_ChangedSinceRead => {
                        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v17
                    }
                    US0::US0_ReadbackMismatch => {
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v16
                    }
                    US0::US0_RenameFailed => {
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v14
                    }
                    US0::US0_RenameFailedTemporaryKept => {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v15
                    }
                    US0::US0_TemporaryWriteFailed => {
                        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v13
                    }
                    _ => unreachable!(),
                };
                let mut v23: Rc<str> = string_concat(&["agile migrate-tags rejected: ", &*v22]);
                let v24: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v23 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v24
            }
        }
    }
}
fn concat5_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method17(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let v15: US3 = if v6 {
                let v7: US4 = US4::US4_HeadType;
                US3::US3_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US4 = US4::US4_HeadType;
                    US3::US3_SymbolHead(v11)
                } else {
                    US3::US3_OtherLine
                }
            };
            let v27: US3 = match &v15 {
                US3::US3_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US4 = US4::US4_HeadInl;
                        US3::US3_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let mut v22: US4 = US4::US4_HeadInl;
                            US3::US3_SymbolHead(v22)
                        } else {
                            US3::US3_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US3 = match &v27 {
                US3::US3_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US4 = US4::US4_HeadUnion;
                        US3::US3_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US4 = US4::US4_HeadUnion;
                            US3::US3_SymbolHead(v34)
                        } else {
                            US3::US3_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US3 = match &v39 {
                US3::US3_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US4 = US4::US4_HeadOpen;
                        US3::US3_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US4 = US4::US4_HeadOpen;
                            US3::US3_SymbolHead(v46)
                        } else {
                            US3::US3_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US3::US3_SymbolHead(v52) => {
                    let v52: US4 = v52;
                    let v60: Rc<str> = match &v52 {
                        US4::US4_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US4::US4_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US4::US4_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US4::US4_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US4::US4_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US4::US4_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US4::US4_HeadType => {
                            v5
                        }
                        US4::US4_HeadUnion => {
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
fn field_end_20(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US7) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US7::US7_ScanPlain => {
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
                let v25: US7 = match v3 {
                    US7::US7_ScanEscaped => {
                        US7::US7_ScanQuoted
                    }
                    US7::US7_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US7::US7_ScanQuoted
                        } else {
                            US7::US7_ScanPlain
                        }
                    }
                    US7::US7_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US7::US7_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US7::US7_ScanPlain
                            } else {
                                US7::US7_ScanQuoted
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
fn skip_spaces_21(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn top_level_field_count_19(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: US7 = US7::US7_ScanPlain;
        let v5: i32 = field_end_20(v0.clone(), v1, v2, v4);
        let v6: bool = v2 <= v5;
        if v6 {
            return -1i32;
        } else {
            let v7: i32 = v0.as_bytes().get(v5 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v8: bool = v7 == 41i32;
            if v8 {
                let mut v9: i32 = v3.wrapping_add(1i32);
                return v9;
            } else {
                let v10: i32 = v5.wrapping_add(1i32);
                let mut v11: i32 = skip_spaces_21(v0.clone(), v10);
                let mut v12: i32 = v3.wrapping_add(1i32);
                (v1, v3) = (v11, v12);
                continue;
            }
        }
    }
}
fn first_malformed_task_18(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let mut v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return -1i32;
        } else {
            let v6: i32 = v4.wrapping_add(6i32);
            let v7: i32 = v0.get(..v4 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v7 as usize..v4 as usize).unwrap_or(""));
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v10: bool = v8.starts_with(&*v9);
            if v10 {
                let mut v11: i32 = 0i32;
                let mut v12: i32 = top_level_field_count_19(v0.clone(), v6, v2, v11);
                let v13: bool = v12 == 11i32;
                if v13 {
                    v1 = v6;
                    continue;
                } else {
                    return v4;
                }
            } else {
                v1 = v6;
                continue;
            }
        }
    }
}
fn fresh_migration_22(mut v0: Rc<str>) -> (Rc<str>, i32, i32, i32) {
    (v0, 0i32, 0i32, 0i32)
}
fn field_start_24(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US7 = US7::US7_ScanPlain;
            let v6: i32 = field_end_20(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_21(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn fresh_tag_pass_25(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, i32, Rc<str>, i32) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0, v1.clone(), 0i32, v1, 0i32)
}
fn rewritten_29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US8 {
    let (mut v4, mut v5): (i32, Rc<str>) = agile_planning_rewrite::rig_agile_planning_rewrite(&*v0, &*v1, &*v2, &*v3);
    let v6: bool = v4 == 0i32;
    if v6 {
        US8::US8_StepText(v5)
    } else {
        US8::US8_StepRejected(v5)
    }
}
fn tag_moved_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>) -> (Rc<str>, Rc<str>, i32, Rc<str>, i32) {
    let mut v9: US8 = rewritten_29(v0.clone(), v5, v7, v8);
    let (mut v27, mut v28, mut v29, mut v30, mut v31): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = match &v9 {
        US8::US8_StepText(v10) => {
            let v10: Rc<str> = v10.clone();
            let mut v11: i32 = v2.wrapping_add(1i32);
            (v10, v1, v11, v3, v4)
        }
        _ => {
            let (mut v22, mut v23, mut v24, mut v25, mut v26): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = match v9 {
                US8::US8_StepRejected(v12) => {
                    let mut v12: Rc<str> = v12;
                    let v13: bool = &*v1 == "";
                    let v16: Rc<str> = if v13 {
                        v6.clone()
                    } else {
                        let v15: Rc<str> = string_concat(&[&*v1, ",", &*v6]);
                        v15
                    };
                    let v17: bool = &*v3 == "";
                    let v20: Rc<str> = if v17 {
                        v6
                    } else {
                        let mut v19: Rc<str> = string_concat(&[&*v3, ",", &*v6]);
                        v19
                    };
                    let v21: i32 = v4.wrapping_add(1i32);
                    (v0, v16, v2, v20, v21)
                }
                _ => unreachable!(),
            };
            (v22, v23, v24, v25, v26)
        }
    };
    (v27, v28, v29, v30, v31)
}
fn tag_kind_of_30(mut v0: Rc<str>) -> US9 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("est:"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    if v2 {
        let v3: i32 = (v1.len() as i32);
        let mut v4: i32 = (v0.clone().len() as i32);
        let v5: Rc<str> = std::rc::Rc::<str>::from(v0.get(v3 as usize..v4 as usize).unwrap_or(""));
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate"); } LIT.with(|lit| lit.clone()) };
        US9::US9_PlanningTag(v6, v5)
    } else {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load:"); } LIT.with(|lit| lit.clone()) };
        let mut v9: bool = v0.starts_with(&*v8);
        if v9 {
            let v10: i32 = (v8.len() as i32);
            let mut v11: i32 = (v0.clone().len() as i32);
            let mut v12: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v11 as usize).unwrap_or(""));
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load"); } LIT.with(|lit| lit.clone()) };
            US9::US9_PlanningTag(v13, v12)
        } else {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agent:"); } LIT.with(|lit| lit.clone()) };
            let v16: bool = v0.starts_with(&*v15);
            if v16 {
                let v17: i32 = (v15.len() as i32);
                let v18: i32 = (v0.clone().len() as i32);
                let mut v19: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v18 as usize).unwrap_or(""));
                let v20: Rc<str> = agile_planning_values::rig_agile_planning_legacy_agent_value(&*v19);
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("assignee"); } LIT.with(|lit| lit.clone()) };
                US9::US9_PlanningTag(v21, v20)
            } else {
                US9::US9_PlainTag
            }
        }
    }
}
fn value_token_31(mut v0: Rc<str>, mut v1: Rc<str>) -> US8 {
    let (mut v2, mut v3): (i32, Rc<str>) = agile_planning_values::rig_agile_planning_value_token(&*v0, &*v1);
    let mut v4: bool = v2 == 0i32;
    if v4 {
        US8::US8_StepText(v3)
    } else {
        US8::US8_StepRejected(v3)
    }
}
fn tag_applied_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>) -> (Rc<str>, Rc<str>, i32, Rc<str>, i32) {
    let v7: US9 = tag_kind_of_30(v6.clone());
    let (mut v46, mut v47, mut v48, mut v49, mut v50): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = match &v7 {
        US9::US9_PlainTag => {
            let mut v8: bool = &*v1 == "";
            let mut v11: Rc<str> = if v8 {
                v6
            } else {
                let v10: Rc<str> = string_concat(&[&*v1, ",", &*v6]);
                v10
            };
            (v0, v11, v2, v3, v4)
        }
        _ => {
            let (mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = match v7 {
                US9::US9_PlanningTag(v12, v13) => {
                    let mut v12: Rc<str> = v12;
                    let v13: Rc<str> = v13;
                    let v14: US8 = value_token_31(v12.clone(), v13);
                    match &v14 {
                        US8::US8_StepText(v15) => {
                            let v15: Rc<str> = v15.clone();
                            tag_moved_28(v0, v1, v2, v3, v4, v5, v6, v12, v15)
                        }
                        _ => {
                            let (mut v31, mut v32, mut v33, mut v34, mut v35): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = match v14 {
                                US8::US8_StepRejected(v21) => {
                                    let v21: Rc<str> = v21;
                                    let mut v22: bool = &*v1 == "";
                                    let v25: Rc<str> = if v22 {
                                        v6.clone()
                                    } else {
                                        let v24: Rc<str> = string_concat(&[&*v1, ",", &*v6]);
                                        v24
                                    };
                                    let v26: bool = &*v3 == "";
                                    let v29: Rc<str> = if v26 {
                                        v6
                                    } else {
                                        let v28: Rc<str> = string_concat(&[&*v3, ",", &*v6]);
                                        v28
                                    };
                                    let v30: i32 = v4.wrapping_add(1i32);
                                    (v0, v25, v2, v29, v30)
                                }
                                _ => unreachable!(),
                            };
                            (v31, v32, v33, v34, v35)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v41, v42, v43, v44, v45)
        }
    };
    (v46, v47, v48, v49, v50)
}
fn tags_applied_26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: i32) -> (Rc<str>, Rc<str>, i32, Rc<str>, i32) {
    loop {
        let mut v8: i32 = (v6.clone().len() as i32);
        let mut v9: bool = v8 <= v7;
        if v9 {
            return (v0, v1, v2, v3, v4);
        } else {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v11: i32 = v6.get(v7 as usize..).and_then(|rest| rest.find(&*v10)).map(|found| (found + v7 as usize) as i32).unwrap_or(-1);
            let mut v12: bool = v11 < 0i32;
            let v13: i32 = if v12 {
                v8
            } else {
                v11
            };
            let v14: Rc<str> = std::rc::Rc::<str>::from(v6.get(v7 as usize..v13 as usize).unwrap_or(""));
            let v15: i32 = (v14.clone().len() as i32);
            let v16: bool = v15 == 0i32;
            let (mut v22, mut v23, mut v24, mut v25, mut v26): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = if v16 {
                (v0, v1, v2, v3, v4)
            } else {
                let (mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = tag_applied_27(v0, v1, v2, v3, v4, v5.clone(), v14);
                (v17, v18, v19, v20, v21)
            };
            let v27: i32 = v13.wrapping_add(1i32);
            (v0, v1, v2, v3, v4, v7) = (v22, v23, v24, v25, v26, v27);
            continue;
        }
    }
}
fn tags_replaced_33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: i32 = (v0.clone().len() as i32);
    let v5: Rc<str> = string_concat(&["Task (\"", &*v1, "\","]);
    let v6: i32 = v0.find(&*v5).map(|offset| offset as i32).unwrap_or(-1);
    let v7: i32 = v6.wrapping_add(6i32);
    let mut v8: i32 = 8i32;
    let mut v9: i32 = field_start_24(v0.clone(), v7, v3, v8);
    let v10: US7 = US7::US7_ScanPlain;
    let mut v11: i32 = field_end_20(v0.clone(), v9, v3, v10);
    let mut v12: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v9 as usize).unwrap_or(""));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(v2.replace(&*v14, &*v15));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(v16.replace(&*v13, &*v17));
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(v18.replace(&*v19, &*v20));
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = Rc::<str>::from(v21.replace(&*v22, &*v23));
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = Rc::<str>::from(v24.replace(&*v25, &*v26));
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = concat5_16(v13.clone(), v27, v13.clone(), v28.clone(), v28.clone());
    let v31: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v3 as usize).unwrap_or(""));
    let v32: Rc<str> = string_concat(&[&*v12, &*v29, &*v31]);
    v32
}
fn task_report_34(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: i32) -> i32 {
    let v7: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = string_concat(&[&*v0, " moved=", &*v7, " "]);
    let v10: Rc<str> = agile_planning_fields::rig_agile_planning_summary(&*v1, &*v0);
    let v13: Rc<str> = string_concat(&[&*v9, &*v10, " tags=", &*v2]);
    let v14: bool = v5 == 0i32;
    let v17: Rc<str> = if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v15
    } else {
        let v16: Rc<str> = string_concat(&[" rejected=", &*v4]);
        v16
    };
    let v18: Rc<str> = string_concat(&[&*v13, &*v17]);
    let mut v19: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v18.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v19
}
fn task_migrated_32(mut v0: &Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: i32, mut v8: Rc<str>, mut v9: i32) -> (Rc<str>, i32, i32, i32) {
    let v10: Rc<str> = tags_replaced_33(v5, v4.clone(), v6.clone());
    let mut v11: i32 = task_report_34(v4, v10.clone(), v6, v7, v8, v9);
    let mut v12: i32 = v1.wrapping_add(1i32);
    let v13: i32 = v2.wrapping_add(v7);
    let v14: i32 = v3.wrapping_add(v9);
    (v10, v12, v13, v14)
}
fn task_with_rejections_35(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: i32, mut v8: Rc<str>, mut v9: i32) -> (Rc<str>, i32, i32, i32) {
    let v10: i32 = task_report_34(v4, v5, v6, v7, v8, v9);
    let mut v11: i32 = v3.wrapping_add(v9);
    (v0, v1, v2, v11)
}
fn migrated_from_23(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> (Rc<str>, i32, i32, i32) {
    loop {
        let v5: i32 = (v0.clone().len() as i32);
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v7: i32 = v0.get(v4 as usize..).and_then(|rest| rest.find(&*v6)).map(|found| (found + v4 as usize) as i32).unwrap_or(-1);
        let mut v8: bool = v7 < 0i32;
        if v8 {
            return (v0, v1, v2, v3);
        } else {
            let mut v9: i32 = v7.wrapping_add(6i32);
            let v10: i32 = v0.get(..v7 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v11: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v7 as usize).unwrap_or(""));
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v13: bool = v11.starts_with(&*v12);
            if v13 {
                let v14: i32 = 0i32;
                let v15: i32 = field_start_24(v0.clone(), v9, v5, v14);
                let v16: US7 = US7::US7_ScanPlain;
                let v17: i32 = field_end_20(v0.clone(), v15, v5, v16);
                let v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(v15 as usize..v17 as usize).unwrap_or(""));
                let mut v19: i32 = (v18.clone().len() as i32);
                let v20: i32 = v19.wrapping_sub(1i32);
                let v21: Rc<str> = std::rc::Rc::<str>::from(v18.get(1i32 as usize..v20 as usize).unwrap_or(""));
                let (mut v22, mut v23, mut v24, mut v25, mut v26): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = fresh_tag_pass_25(v0.clone());
                let v27: i32 = 8i32;
                let v28: i32 = field_start_24(v0.clone(), v9, v5, v27);
                let v29: US7 = US7::US7_ScanPlain;
                let v30: i32 = field_end_20(v0.clone(), v28, v5, v29);
                let v31: Rc<str> = std::rc::Rc::<str>::from(v0.get(v28 as usize..v30 as usize).unwrap_or(""));
                let v32: i32 = (v31.clone().len() as i32);
                let v33: i32 = v32.wrapping_sub(1i32);
                let v34: Rc<str> = std::rc::Rc::<str>::from(v31.get(1i32 as usize..v33 as usize).unwrap_or(""));
                let v35: i32 = 0i32;
                let (mut v36, mut v37, mut v38, mut v39, mut v40): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = tags_applied_26(v22, v23, v24, v25, v26, v21.clone(), v34, v35);
                let v41: bool = 1i32 <= v38;
                if v41 {
                    let (mut v42, mut v43, mut v44, mut v45): (Rc<str>, i32, i32, i32) = task_migrated_32(&v0, v1, v2, v3, v21.clone(), v36, v37, v38, v39, v40);
                    let v47: Rc<str> = string_concat(&["Task (\"", &*v21, "\","]);
                    let v48: i32 = v42.find(&*v47).map(|offset| offset as i32).unwrap_or(-1);
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v50: i32 = v42.get(v48 as usize..).and_then(|rest| rest.find(&*v49)).map(|found| (found + v48 as usize) as i32).unwrap_or(-1);
                    let v51: bool = v50 < 0i32;
                    let v53: i32 = if v51 {
                        let v52: i32 = (v42.clone().len() as i32);
                        v52
                    } else {
                        v50
                    };
                    (v0, v1, v2, v3, v4) = (v42, v43, v44, v45, v53);
                    continue;
                } else {
                    let v58: bool = 1i32 <= v40;
                    if v58 {
                        let (mut v59, mut v60, mut v61, mut v62): (Rc<str>, i32, i32, i32) = task_with_rejections_35(v0, v1, v2, v3, v21, v36, v37, v38, v39, v40);
                        (v0, v1, v2, v3, v4) = (v59, v60, v61, v62, v9);
                        continue;
                    } else {
                        v4 = v9;
                        continue;
                    }
                }
            } else {
                v4 = v9;
                continue;
            }
        }
    }
}
fn tags_migration_1(mut v0: Rc<str>, mut v1: US5) -> i32 {
    let mut v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_16(v0, v3, v4.clone(), v4.clone(), v4.clone());
    let v6: bool = std::path::Path::new(&*v5).is_file();
    let mut v9: US6 = if v6 {
        US6::US6_FileReadable(v5.clone())
    } else {
        US6::US6_FileMissing(v5.clone())
    };
    match v9 {
        US6::US6_FileMissing(v10) => {
            let v10: Rc<str> = v10;
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = string_concat(&["agile migrate-tags rejected: ", &*v11]);
            let v13: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v12 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v13
        }
        US6::US6_FileReadable(v14) => {
            let v14: Rc<str> = v14;
            let v15: Rc<str> = std::fs::read_to_string(&*v14).unwrap_or_default().into();
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(v14.replace(&*v16, &*v17));
            let mut v19: Rc<str> = loop_5(v18);
            let v20: Option<usize> = v19.rfind(&*v17);
            let v21: Option<usize> = method6(v20);
            let mut v22: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v23: Option<i32> = v21.map(|x| v22(x));
            let v25: i32 = v23.unwrap_or(-1i32);
            let v26: bool = v25 < 0i32;
            let v30: bool = if v26 {
                true
            } else {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v28: bool = v19.ends_with(&*v27);
                let v29: bool = v28 == false;
                v29
            };
            let v51: US1 = if v30 {
                US1::US1_OtherFile
            } else {
                let v32: usize = ((0i32) as usize);
                let v33: usize = ((v25) as usize);
                let v34: Rc<str> = Rc::<str>::from(&(v19)[v32..v33]);
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v36: bool = v34.ends_with(&*v35);
                if v36 {
                    let v37: Rc<str> = loop_7(v34);
                    US1::US1_StateFile(v37)
                } else {
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v40: bool = v34.ends_with(&*v39);
                    if v40 {
                        let v41: Rc<str> = loop_8(v34);
                        US1::US1_HistoryFile(v41)
                    } else {
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v44: bool = v34.ends_with(&*v43);
                        if v44 {
                            let v45: Rc<str> = loop_9(v34);
                            US1::US1_ArchiveFile(v45)
                        } else {
                            US1::US1_OtherFile
                        }
                    }
                }
            };
            let v62: Rc<str> = match v51 {
                US1::US1_OtherFile => {
                    v15
                }
                _ => {
                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v53: Rc<UH0> = split_list_10(v52.clone(), v15);
                    let v54: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v55: Rc<UH0> = method17(v53, v54);
                    let v56: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v57: i32 = 0i32;
                    let v58: i32 = method14(v56.clone(), v55, v57);
                    let v59: Rc<Vec<Rc<str>>> = Rc::new(v56.borrow().clone());
                    let v60: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v59).as_ref().clone()));
                    let v61: Rc<str> = Rc::<str>::from((*v60.borrow()).join(&*v52));
                    v61
                }
            };
            let v63: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v62);
            let v64: i32 = 0i32;
            let v65: i32 = (v63.clone().len() as i32);
            let v66: i32 = first_malformed_task_18(v63.clone(), v64, v65);
            let v67: bool = v66 < 0i32;
            if v67 {
                let (mut v68, mut v69, mut v70, mut v71): (Rc<str>, i32, i32, i32) = fresh_migration_22(v63.clone());
                let v72: i32 = 0i32;
                let (mut v73, mut v74, mut v75, mut v76): (Rc<str>, i32, i32, i32) = migrated_from_23(v68, v69, v70, v71, v72);
                match v1 {
                    US5::US5_MigrationApply => {
                        migration_written_2(v5, v73, v74, v75, v76, v63, v2)
                    }
                    US5::US5_MigrationDryRun => {
                        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview"); } LIT.with(|lit| lit.clone()) };
                        migration_receipt_3(v77, v73, v74, v75, v76, v63, v2)
                    }
                }
            } else {
                let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed Task fields"); } LIT.with(|lit| lit.clone()) };
                let mut v83: Rc<str> = string_concat(&["agile migrate-tags rejected: ", &*v82]);
                let v84: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v83 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v84
            }
        }
    }
}
fn command_line_argument_count_37() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_36() -> i32 {
    let mut v0: i32 = command_line_argument_count_37();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn tags_migration_entry_0() -> i32 {
    let mut v0: i32 = argument_count_36();
    let mut v1: bool = v0 == 3i32;
    if v1 {
        let mut v2: i32 = 1i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v4: bool = &*v3 == "apply";
        let mut v12: US10 = if v4 {
            let v5: US5 = US5::US5_MigrationApply;
            US10::US10_Valid(v5)
        } else {
            let v7: bool = &*v3 == "preview";
            if v7 {
                let mut v8: US5 = US5::US5_MigrationDryRun;
                US10::US10_Valid(v8)
            } else {
                US10::US10_Invalid
            }
        };
        match v12 {
            US10::US10_Invalid => {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-tags expects preview <root> or apply <root>"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = string_concat(&["agile migrate-tags rejected: ", &*v17]);
                let mut v19: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v18 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v19
            }
            US10::US10_Valid(v13) => {
                let v13: US5 = v13;
                let v14: i32 = 2i32;
                let v15: Rc<str> = usize::try_from(v14).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                tags_migration_1(v15, v13)
            }
        }
    } else {
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-tags expects preview <root> or apply <root>"); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = string_concat(&["agile migrate-tags rejected: ", &*v22]);
        let v24: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v23 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v24
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        tags_migration_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_migrate_tags_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
