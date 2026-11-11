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
    US5_RootsEdited(Rc<str>),
    US5_RootsRefused(Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_FileReadable(Rc<str>),
    US6_FileMissing(Rc<str>),
}
fn concat5_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn harvest_roots_from_2(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
        let mut v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return v2;
        } else {
            let v6: i32 = v4.wrapping_add(19i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = v0.get(v6 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v6 as usize) as i32).unwrap_or(-1);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v6 as usize..v8 as usize).unwrap_or(""));
            let v10: bool = &*v2 == "";
            let v14: Rc<str> = if v10 {
                v9
            } else {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                concat5_3(v2, v11, v9, v12.clone(), v12.clone())
            };
            let v15: i32 = v8.wrapping_add(1i32);
            (v1, v2) = (v15, v14);
            continue;
        }
    }
}
fn roots_printed_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v3 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v6: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v5)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
            let v7: bool = v6 < 0i32;
            let mut v8: i32 = if v7 {
                v3
            } else {
                v6
            };
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v8 as usize).unwrap_or(""));
            let v10: Rc<str> = string_concat(&["root ", &*v9]);
            let mut v11: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v10.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            let mut v12: i32 = v8.wrapping_add(1i32);
            let v13: i32 = v2.wrapping_add(1i32);
            (v1, v2) = (v12, v13);
            continue;
        }
    }
}
fn roots_receipt_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: i32 = 0i32;
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = harvest_roots_from_2(v1, v3, v4);
    let v6: i32 = 0i32;
    let v7: i32 = 0i32;
    let mut v8: i32 = roots_printed_4(v5, v6, v7);
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
    let v13: Rc<str> = string_concat(&["rig agile roots ok action=", &*v0, " roots=", &*v11, &*v2]);
    let v14: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v13.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v14
}
fn loop_8(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method9(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_10(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(6i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_11(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
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
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method15(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
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
        while method15(v7, &v9) {
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
        method14(v16, v17)
    }
}
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method16(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
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
fn method17(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn renamed_within_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn replace_records_unchanged_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let v3: bool = rig_agile_records::rig_agile_records_unchanged(&*v0, &*v1);
    if v3 {
        let mut v4: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v6));
        let mut v8: Rc<str> = loop_8(v7);
        let mut v9: Option<usize> = v8.rfind(&*v6);
        let v10: Option<usize> = method9(v9);
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
                let v26: Rc<str> = loop_10(v23);
                US1::US1_StateFile(v26)
            } else {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v29: bool = v23.ends_with(&*v28);
                if v29 {
                    let v30: Rc<str> = loop_11(v23);
                    US1::US1_HistoryFile(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v23.ends_with(&*v32);
                    if v33 {
                        let v34: Rc<str> = loop_12(v23);
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
                let v65: Rc<UH0> = split_list_13(v64.clone(), v2.clone());
                let v66: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v67: Rc<UH0> = method16(v65, v66);
                let v68: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v69: i32 = 0i32;
                let v70: i32 = method17(v68.clone(), v67, v69);
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
            let v78: bool = renamed_within_18(v0.clone(), v4.clone(), v77);
            if v78 {
                let v79: Rc<str> = Rc::<str>::from(v0.replace(&*v5, &*v6));
                let mut v80: Rc<str> = loop_8(v79);
                let v81: Option<usize> = v80.rfind(&*v6);
                let v82: Option<usize> = method9(v81);
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
                        let v97: Rc<str> = loop_10(v94);
                        US1::US1_StateFile(v97)
                    } else {
                        let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v100: bool = v94.ends_with(&*v99);
                        if v100 {
                            let v101: Rc<str> = loop_11(v94);
                            US1::US1_HistoryFile(v101)
                        } else {
                            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v104: bool = v94.ends_with(&*v103);
                            if v104 {
                                let v105: Rc<str> = loop_12(v94);
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
                        let v136: Rc<UH0> = split_list_13(v135.clone(), v2);
                        let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v138: Rc<UH0> = method16(v136, v137);
                        let v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v140: i32 = 0i32;
                        let v141: i32 = method17(v139.clone(), v138, v140);
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
fn roots_written_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> i32 {
    let mut v5: US0 = replace_records_unchanged_7(v0, v2.clone(), v3.clone());
    match &v5 {
        US0::US0_Replaced => {
            let v6: u64 = v2.len() as u64;
            let v7: u64 = v3.len() as u64;
            let mut v8: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from((v6).to_string());
            let mut v12: Rc<str> = std::rc::Rc::<str>::from((v7).to_string());
            let v15: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
            let v16: Rc<str> = string_concat(&[" bytes_before=", &*v9, " bytes_after=", &*v12, " elapsed_ms=", &*v15]);
            roots_receipt_1(v1, v3, v16)
        }
        _ => {
            let v27: Rc<str> = match v5 {
                US0::US0_ChangedSinceRead => {
                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                    v22
                }
                US0::US0_ReadbackMismatch => {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                    v21
                }
                US0::US0_RenameFailed => {
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                    v19
                }
                US0::US0_RenameFailedTemporaryKept => {
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                    v20
                }
                US0::US0_TemporaryWriteFailed => {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                    v18
                }
                _ => unreachable!(),
            };
            let v28: Rc<str> = string_concat(&["agile roots rejected: ", &*v27]);
            let v29: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v28 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v29
        }
    }
}
fn command_line_argument_count_20() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_19() -> i32 {
    let mut v0: i32 = command_line_argument_count_20();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn root_key_25(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method9(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let mut v8: bool = v7 == 1i32;
    if v8 {
        let mut v9: Rc<str> = Rc::<str>::from(v0.to_lowercase());
        v9
    } else {
        v0
    }
}
fn method24(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: Rc<str> = root_key_25(v3.clone());
                let v6: bool = &*v2 != "";
                let mut v8: bool = if v6 {
                    true
                } else {
                    let v7: bool = &*v3 == "";
                    v7
                };
                let v17: Rc<str> = if v8 {
                    v2
                } else {
                    let mut v9: bool = &*v5 == &*v0;
                    let mut v12: bool = if v9 {
                        true
                    } else {
                        let v10: Rc<str> = string_concat(&[&*v5, "/"]);
                        let mut v11: bool = v0.starts_with(&*v10);
                        v11
                    };
                    let v15: bool = if v12 {
                        true
                    } else {
                        let v13: Rc<str> = string_concat(&[&*v0, "/"]);
                        let v14: bool = v5.starts_with(&*v13);
                        v14
                    };
                    if v15 {
                        v3
                    } else {
                        v2
                    }
                };
                (v1, v2) = (v4, v17);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn overlapping_root_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = root_key_25(v1);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: i32 = 0i32;
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = harvest_roots_from_2(v0, v4, v5.clone());
    let v7: Rc<UH0> = split_list_13(v3, v6);
    method24(v2, v7, v5)
}
fn overlap_reason_26(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = root_key_25(v0.clone());
    let v3: Rc<str> = root_key_25(v1.clone());
    let mut v4: bool = &*v2 == &*v3;
    if v4 {
        let mut v5: Rc<str> = string_concat(&[&*v0, " is already a harvest root"]);
        v5
    } else {
        let mut v8: Rc<str> = string_concat(&[&*v0, " overlaps harvest root ", &*v1, " (one root per tree, so every file is scanned once)"]);
        v8
    }
}
fn rows_end_from_27(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v3: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v2)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        let v7: i32 = if v4 {
            let mut v5: i32 = (v0.clone().len() as i32);
            v5
        } else {
            let v6: i32 = v3.wrapping_add(1i32);
            v6
        };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v7 as usize).unwrap_or(""));
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
        let v10: bool = v8.contains(&*v9);
        if v10 {
            v1 = v7;
            continue;
        } else {
            return v1;
        }
    }
}
fn root_added_22(mut v0: Rc<str>, mut v1: Rc<str>) -> US5 {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from({ let forward = (v1).trim().replace(char::from(92u8), "/"); let trimmed = forward.trim_end_matches(char::from(47u8)); if trimmed.is_empty() { forward } else { trimmed.to_owned() } });
    let v3: bool = &*v2 == "";
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a harvest root needs a path"); } LIT.with(|lit| lit.clone()) };
        US5::US5_RootsRefused(v4)
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v7: bool = v2.contains(&*v6);
        if v7 {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a harvest root cannot contain a double quote"); } LIT.with(|lit| lit.clone()) };
            US5::US5_RootsRefused(v8)
        } else {
            let v10: Rc<str> = overlapping_root_23(v0.clone(), v2.clone());
            let mut v11: bool = &*v10 != "";
            if v11 {
                let mut v12: Rc<str> = overlapping_root_23(v0, v2.clone());
                let v13: Rc<str> = overlap_reason_26(v2, v12);
                US5::US5_RootsRefused(v13)
            } else {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union harvest_root ="); } LIT.with(|lit| lit.clone()) };
                let v16: bool = v0.contains(&*v15);
                let v32: Rc<str> = if v16 {
                    v0
                } else {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v18: bool = v0.starts_with(&*v17);
                    let v25: i32 = if v18 {
                        0i32
                    } else {
                        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\ninl "); } LIT.with(|lit| lit.clone()) };
                        let v20: i32 = v0.find(&*v19).map(|offset| offset as i32).unwrap_or(-1);
                        let v21: bool = v20 < 0i32;
                        if v21 {
                            let mut v22: i32 = (v0.clone().len() as i32);
                            v22
                        } else {
                            let mut v23: i32 = v20.wrapping_add(1i32);
                            v23
                        }
                    };
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union harvest_root =\n    | HarvestRoot :: string -> harvest_root\n\n"); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v25 as usize).unwrap_or(""));
                    let v28: i32 = (v0.clone().len() as i32);
                    let v29: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v28 as usize).unwrap_or(""));
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_3(v27, v26, v29, v30.clone(), v30.clone())
                };
                let v33: i32 = v32.find(&*v15).map(|offset| offset as i32).unwrap_or(-1);
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
                let v35: i32 = v32.get(v33 as usize..).and_then(|rest| rest.find(&*v34)).map(|found| (found + v33 as usize) as i32).unwrap_or(-1);
                let v36: i32 = v35.wrapping_add(2i32);
                let v37: i32 = rows_end_from_27(v32.clone(), v36);
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                let v39: Rc<str> = std::rc::Rc::<str>::from(format!("harvest_root_{}", (v2).to_ascii_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { char::from(95u8) }).collect::<String>().trim_matches(char::from(95u8))));
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\n"); } LIT.with(|lit| lit.clone()) };
                let v42: Rc<str> = concat5_3(v38, v39, v40, v2, v41);
                let v43: Rc<str> = std::rc::Rc::<str>::from(v32.get(0i32 as usize..v37 as usize).unwrap_or(""));
                let v44: i32 = (v32.clone().len() as i32);
                let v45: Rc<str> = std::rc::Rc::<str>::from(v32.get(v37 as usize..v44 as usize).unwrap_or(""));
                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v47: Rc<str> = concat5_3(v43, v42, v45, v46.clone(), v46.clone());
                US5::US5_RootsEdited(v47)
            }
        }
    }
}
fn root_dropped_28(mut v0: Rc<str>, mut v1: Rc<str>) -> US5 {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from({ let forward = (v1).trim().replace(char::from(92u8), "/"); let trimmed = forward.trim_end_matches(char::from(47u8)); if trimmed.is_empty() { forward } else { trimmed.to_owned() } });
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(format!("harvest_root_{}", (v2).to_ascii_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { char::from(95u8) }).collect::<String>().trim_matches(char::from(95u8))));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\n"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = concat5_3(v3, v4, v5, v2.clone(), v6);
    let mut v8: i32 = v0.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
    let mut v9: bool = v8 < 0i32;
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not a harvest root"); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = concat5_3(v2, v10, v11.clone(), v11.clone(), v11.clone());
        US5::US5_RootsRefused(v12)
    } else {
        let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v8 as usize).unwrap_or(""));
        let v15: i32 = (v7.len() as i32);
        let v16: i32 = v8.wrapping_add(v15);
        let v17: i32 = (v0.clone().len() as i32);
        let v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(v16 as usize..v17 as usize).unwrap_or(""));
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = concat5_3(v14, v18, v19.clone(), v19.clone(), v19.clone());
        US5::US5_RootsEdited(v20)
    }
}
fn edits_applied_21(mut v0: Rc<str>, mut v1: bool, mut v2: i32, mut v3: i32) -> US5 {
    loop {
        let mut v4: bool = v3 <= v2;
        if v4 {
            return US5::US5_RootsEdited(v0);
        } else {
            let v6: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v9: US5 = if v1 {
                root_added_22(v0, v6)
            } else {
                root_dropped_28(v0, v6)
            };
            match v9 {
                US5::US5_RootsEdited(v10) => {
                    let v10: Rc<str> = v10;
                    let mut v11: i32 = v2.wrapping_add(1i32);
                    (v0, v2) = (v10, v11);
                    continue;
                }
                US5::US5_RootsRefused(v13) => {
                    let v13: Rc<str> = v13;
                    return US5::US5_RootsRefused(v13);
                }
            }
        }
    }
}
fn roots_edited_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> i32 {
    let mut v4: i32 = argument_count_19();
    let mut v5: bool = v4 <= 3i32;
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add and drop need at least one path"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = string_concat(&["agile roots rejected: ", &*v6]);
        let mut v8: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v7 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v8
    } else {
        let mut v9: bool = &*v1 == "add";
        let v10: i32 = 3i32;
        let mut v11: i32 = argument_count_19();
        let mut v12: US5 = edits_applied_21(v2.clone(), v9, v10, v11);
        match v12 {
            US5::US5_RootsEdited(v16) => {
                let v16: Rc<str> = v16;
                roots_written_6(v0, v1, v2, v16, v3)
            }
            US5::US5_RootsRefused(v13) => {
                let v13: Rc<str> = v13;
                let v14: Rc<str> = string_concat(&["agile roots rejected: ", &*v13]);
                let v15: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v14 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v15
            }
        }
    }
}
fn method29(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method29(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
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
fn roots_entry_0() -> i32 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: i32 = 1i32;
    let mut v2: Rc<str> = usize::try_from(v1).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = concat5_3(v2, v3, v4.clone(), v4.clone(), v4.clone());
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
            let mut v12: Rc<str> = string_concat(&["agile roots rejected: ", &*v11]);
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
            let mut v19: Rc<str> = loop_8(v18);
            let v20: Option<usize> = v19.rfind(&*v17);
            let v21: Option<usize> = method9(v20);
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
                    let v37: Rc<str> = loop_10(v34);
                    US1::US1_StateFile(v37)
                } else {
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v40: bool = v34.ends_with(&*v39);
                    if v40 {
                        let v41: Rc<str> = loop_11(v34);
                        US1::US1_HistoryFile(v41)
                    } else {
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v44: bool = v34.ends_with(&*v43);
                        if v44 {
                            let v45: Rc<str> = loop_12(v34);
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
                    let v53: Rc<UH0> = split_list_13(v52.clone(), v15);
                    let v54: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v55: Rc<UH0> = method29(v53, v54);
                    let v56: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v57: i32 = 0i32;
                    let v58: i32 = method17(v56.clone(), v55, v57);
                    let v59: Rc<Vec<Rc<str>>> = Rc::new(v56.borrow().clone());
                    let v60: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v59).as_ref().clone()));
                    let v61: Rc<str> = Rc::<str>::from((*v60.borrow()).join(&*v52));
                    v61
                }
            };
            let v63: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v62);
            let v64: i32 = 2i32;
            let v65: Rc<str> = usize::try_from(v64).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v66: bool = &*v65 == "";
            if v66 {
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                let v68: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                let v69: Rc<str> = std::rc::Rc::<str>::from((v68).to_string());
                let v70: Rc<str> = string_concat(&[" elapsed_ms=", &*v69]);
                roots_receipt_1(v67, v63, v70)
            } else {
                let v72: bool = &*v65 == "add";
                let v74: bool = if v72 {
                    true
                } else {
                    let v73: bool = &*v65 == "drop";
                    v73
                };
                if v74 {
                    roots_edited_5(v5, v65, v63, v0)
                } else {
                    let v78: Rc<str> = string_concat(&["agile roots rejected: unknown action ", &*v65, " (expected add or drop)"]);
                    let v79: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v78 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v79
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        roots_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_roots_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
