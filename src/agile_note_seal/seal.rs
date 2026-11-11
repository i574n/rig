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
    US0_SealApply,
    US0_SealDryRun,
}
#[derive(Clone)]
enum US1 {
    US1_FileReadable(Rc<str>),
    US1_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_StateFile(Rc<str>),
    US2_HistoryFile(Rc<str>),
    US2_ArchiveFile(Rc<str>),
    US2_OtherFile,
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
    US5_WordsRoot,
    US5_SymbolsRoot,
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
    US7_StateWritten,
    US7_StateChangedSinceRead,
    US7_StateWriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US8 {
    US8_Some(i32),
    US8_None,
}
fn seal_failure_2(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = string_concat(&["agile seal rejected: ", &*v0]);
    let mut v2: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v1 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
    v2
}
fn seal_receipt_3(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<RefCell<Vec<Rc<str>>>>, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: u64, mut v8: u64, mut v9: u64) -> i32 {
    let mut v13: Rc<str> = match v0 {
        US0::US0_SealApply => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v10
        }
        US0::US0_SealDryRun => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dry-run"); } LIT.with(|lit| lit.clone()) };
            v11
        }
    };
    let v14: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let v17: Rc<str> = std::rc::Rc::<str>::from((v6).to_string());
    let mut v21: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v9);
    let mut v22: Rc<str> = std::rc::Rc::<str>::from((v7).to_string());
    let v25: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
    let v28: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
    let v30: Rc<str> = string_concat(&["rig agile seal ", &*v13, " sealed=", &*v14, " new_in_archive=", &*v17, " bytes_before=", &*v22, " bytes_after=", &*v25, " elapsed_ms=", &*v28]);
    let v31: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v30.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v31
}
fn concat5_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
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
fn loop_8(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
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
fn loop_9(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
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
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method12(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
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
        method11(v16, v17)
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method13(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US3 = if v6 {
                let mut v7: US4 = US4::US4_HeadType;
                US3::US3_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US4 = US4::US4_HeadType;
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
                        let mut v18: US4 = US4::US4_HeadInl;
                        US3::US3_KeywordHead(v18)
                    } else {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let mut v21: bool = v2.starts_with(&*v20);
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
                        let mut v45: bool = v2.starts_with(&*v44);
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
fn method14(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn done_ids_15(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = agile_task_census::rig_agile_done_ids(&*v0);
    let mut v2: Rc<str> = string_concat(&["\n", &*v1]);
    v2
}
fn member_18(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = &*v1 != "";
    if v2 {
        let mut v4: Rc<str> = string_concat(&["\n", &*v1, "\n"]);
        let mut v5: bool = v0.contains(&*v4);
        v5
    } else {
        false
    }
}
fn closure3() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_owner_19(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note (\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method6(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure3();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl note_of_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: bool = v0.starts_with(&*v8);
    let v10: bool = v9 == false;
    let mut v12: bool = if v10 {
        true
    } else {
        let v11: bool = v7 < 0i32;
        v11
    };
    if v12 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    } else {
        let v14: i32 = v7.wrapping_add(13i32);
        let v15: i32 = (v0.clone().len() as i32);
        let v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v14 as usize..v15 as usize).unwrap_or(""));
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v18: Option<usize> = v16.find(&*v17);
        let v19: Option<usize> = method6(v18);
        let mut v20: Option<i32> = v19.map(|x| v4(x));
        let mut v22: i32 = v20.unwrap_or(-1i32);
        let v23: bool = v22 < 0i32;
        if v23 {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24
        } else {
            let v25: Rc<str> = std::rc::Rc::<str>::from(v16.get(0i32 as usize..v22 as usize).unwrap_or(""));
            v25
        }
    }
}
fn sealed_line_17(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = note_owner_19(v1);
    member_18(v0, v2)
}
fn closure2(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: bool = sealed_line_17(v0.clone(), v1);
        let v3: bool = v2 == false;
        v3
    })
}
fn method21(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<Vec<Rc<str>>>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: usize = ((v2) as usize);
            let mut v6: Rc<str> = (v1)[v5].clone();
            let mut v7: i32 = v2.wrapping_sub(1i32);
            let v8: bool = v0(v6.clone());
            let v10: Rc<UH0> = if v8 {
                Rc::new(UH0::UH0_Cons(v6, v3))
            } else {
                v3
            };
            (v2, v3) = (v7, v10);
            continue;
        }
    }
}
fn filtered_20(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v2: Rc<Vec<Rc<str>>> = Rc::new((v1).borrow().clone());
    let v3: usize = (v2).len();
    let mut v4: i32 = ((v3) as i32);
    let mut v5: i32 = v4.wrapping_sub(1i32);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method21(v0, v2, v5, v6);
    let v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v9: i32 = 0i32;
    let v10: i32 = method14(v8.clone(), v7, v9);
    let v11: Rc<Vec<Rc<str>>> = Rc::new(v8.borrow().clone());
    let mut v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v11).as_ref().clone()));
    v12
}
fn closure4(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        sealed_line_17(v0.clone(), v1)
    })
}
fn joined_lines_22(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from((*v0).borrow().iter().map(|x| &**x).collect::<Vec<&str>>().join(&*v1).as_str());
    v2
}
fn sealing_of_16(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, i32, i32, i32) {
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.lines().map(Rc::<str>::from).collect::<Vec<Rc<str>>>()));
    let v3: Rc<dyn Fn(Rc<str>) -> bool> = closure2(v0.clone());
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = filtered_20(v3, v2.clone());
    let mut v5: Rc<dyn Fn(Rc<str>) -> bool> = closure4(v0);
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = filtered_20(v5, v2.clone());
    let mut v7: Rc<str> = joined_lines_22(v4.clone());
    let v8: i32 = (v4.borrow().len() as i32);
    let mut v9: i32 = (v6.clone().borrow().len() as i32);
    let v10: i32 = (v2.borrow().len() as i32);
    (v7, v6, v8, v9, v10)
}
fn archive_path_24(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = string_concat(&[&*v0, "/archive/done-notes.spi"]);
    v1
}
fn method25(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn archived_line_27(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v3: Rc<str> = string_concat(&["\n", &*v1, "\n"]);
    let mut v4: Rc<str> = string_concat(&[&*v0, "\n"]);
    let mut v5: bool = v4.contains(&*v3);
    v5
}
fn closure5(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: bool = archived_line_27(v0.clone(), v1);
        let v3: bool = v2 == false;
        v3
    })
}
fn binding_of_30(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = (v0.clone().len() as i32);
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.get(4i32 as usize..v1 as usize).unwrap_or(""));
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Option<usize> = v2.find(&*v3);
    let mut v5: Option<usize> = method6(v4);
    let mut v6: Rc<dyn Fn((usize)) -> i32> = closure3();
    let mut v7: Option<i32> = v5.map(|x| v6(x));
    let mut v9: i32 = v7.unwrap_or(-1i32);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v11: bool = v0.starts_with(&*v10);
    let mut v12: bool = v11 == false;
    let v14: bool = if v12 {
        true
    } else {
        let mut v13: bool = v9 < 0i32;
        v13
    };
    if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v15
    } else {
        let v16: Rc<str> = std::rc::Rc::<str>::from(v2.get(0i32 as usize..v9 as usize).unwrap_or(""));
        v16
    }
}
fn archived_binding_31(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = binding_of_30(v1);
    let mut v4: Rc<str> = string_concat(&["\ninl ", &*v2, " () ="]);
    let mut v5: bool = v0.contains(&*v4);
    v5
}
fn free_binding_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    loop {
        let mut v4: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
        let mut v5: Rc<str> = string_concat(&[&*v1, "_seal", &*v4]);
        let mut v7: Rc<str> = string_concat(&["\ninl ", &*v5, " () ="]);
        let v8: bool = v0.contains(&*v7);
        if v8 {
            let mut v9: i32 = v2.wrapping_add(1i32);
            v2 = v9;
            continue;
        } else {
            return v5;
        }
    }
}
fn archive_row_29(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = binding_of_30(v1.clone());
    let v3: bool = archived_binding_31(v0.clone(), v1.clone());
    let mut v4: bool = v3 == false;
    if v4 {
        v1
    } else {
        let mut v5: i32 = 2i32;
        let mut v6: Rc<str> = free_binding_32(v0, v2.clone(), v5);
        let v8: i32 = (v2.len() as i32);
        let mut v9: i32 = 4i32.wrapping_add(v8);
        let v10: i32 = (v1.clone().len() as i32);
        let v11: Rc<str> = std::rc::Rc::<str>::from(v1.get(v9 as usize..v10 as usize).unwrap_or(""));
        let mut v12: Rc<str> = string_concat(&["inl ", &*v6, &*v11]);
        v12
    }
}
fn method28(mut v0: Rc<str>, mut v1: Rc<Vec<Rc<str>>>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: usize = ((v2) as usize);
            let mut v6: Rc<str> = (v1)[v5].clone();
            let mut v7: i32 = v2.wrapping_sub(1i32);
            let v8: Rc<str> = archive_row_29(v0.clone(), v6);
            let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v3));
            (v2, v3) = (v7, v9);
            continue;
        }
    }
}
fn fresh_rows_26(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v2: Rc<dyn Fn(Rc<str>) -> bool> = closure5(v0.clone());
    let v3: Rc<RefCell<Vec<Rc<str>>>> = filtered_20(v2, v1);
    let mut v4: Rc<Vec<Rc<str>>> = Rc::new((v3).borrow().clone());
    let mut v5: usize = (v4).len();
    let mut v6: i32 = ((v5) as i32);
    let mut v7: i32 = v6.wrapping_sub(1i32);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method28(v0, v4, v7, v8);
    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v11: i32 = 0i32;
    let mut v12: i32 = method14(v10.clone(), v9, v11);
    let mut v13: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
    let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v13).as_ref().clone()));
    v14
}
fn method33(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method33(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US3 = if v6 {
                let mut v7: US4 = US4::US4_HeadType;
                US3::US3_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US4 = US4::US4_HeadType;
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
                        let mut v18: US4 = US4::US4_HeadInl;
                        US3::US3_KeywordHead(v18)
                    } else {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let mut v21: bool = v2.starts_with(&*v20);
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
                        let mut v45: bool = v2.starts_with(&*v44);
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
fn renamed_within_34(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn state_written_35(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US7 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = concat5_4(v0.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    std::env::var("RIG_AGILE_TEST_PAUSE_MS").or_else(|_| std::env::var("KIT_AGILE_TEST_PAUSE_MS")).ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
    let mut v9: Rc<str> = loop_5(v8);
    let v10: Option<usize> = v9.rfind(&*v7);
    let v11: Option<usize> = method6(v10);
    let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v13: Option<i32> = v11.map(|x| v12(x));
    let v15: i32 = v13.unwrap_or(-1i32);
    let v16: bool = v15 < 0i32;
    let mut v20: bool = if v16 {
        true
    } else {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let mut v18: bool = v9.ends_with(&*v17);
        let v19: bool = v18 == false;
        v19
    };
    let v41: US2 = if v20 {
        US2::US2_OtherFile
    } else {
        let mut v22: usize = ((0i32) as usize);
        let v23: usize = ((v15) as usize);
        let v24: Rc<str> = Rc::<str>::from(&(v9)[v22..v23]);
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v26: bool = v24.ends_with(&*v25);
        if v26 {
            let v27: Rc<str> = loop_7(v24);
            US2::US2_StateFile(v27)
        } else {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
            let v30: bool = v24.ends_with(&*v29);
            if v30 {
                let v31: Rc<str> = loop_8(v24);
                US2::US2_HistoryFile(v31)
            } else {
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                let v34: bool = v24.ends_with(&*v33);
                if v34 {
                    let v35: Rc<str> = loop_9(v24);
                    US2::US2_ArchiveFile(v35)
                } else {
                    US2::US2_OtherFile
                }
            }
        }
    };
    let v64: US5 = match v41 {
        US2::US2_ArchiveFile(v54) => {
            let v54: Rc<str> = v54;
            let v55: Rc<str> = string_concat(&[&*v54, "/state/spiral.toml"]);
            let v56: bool = std::path::Path::new(&*v55).is_file();
            if v56 {
                US5::US5_SymbolsRoot
            } else {
                US5::US5_WordsRoot
            }
        }
        US2::US2_HistoryFile(v48) => {
            let mut v48: Rc<str> = v48;
            let mut v49: Rc<str> = string_concat(&[&*v48, "/state/spiral.toml"]);
            let v50: bool = std::path::Path::new(&*v49).is_file();
            if v50 {
                US5::US5_SymbolsRoot
            } else {
                US5::US5_WordsRoot
            }
        }
        US2::US2_OtherFile => {
            US5::US5_WordsRoot
        }
        US2::US2_StateFile(v42) => {
            let v42: Rc<str> = v42;
            let v43: Rc<str> = string_concat(&[&*v42, "/state/spiral.toml"]);
            let v44: bool = std::path::Path::new(&*v43).is_file();
            if v44 {
                US5::US5_SymbolsRoot
            } else {
                US5::US5_WordsRoot
            }
        }
    };
    let v76: Rc<str> = match v64 {
        US5::US5_SymbolsRoot => {
            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v66: Rc<UH0> = split_list_10(v65.clone(), v1);
            let v67: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v68: Rc<UH0> = method33(v66, v67);
            let v69: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v70: i32 = 0i32;
            let v71: i32 = method14(v69.clone(), v68, v70);
            let v72: Rc<Vec<Rc<str>>> = Rc::new(v69.borrow().clone());
            let v73: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v72).as_ref().clone()));
            let v74: Rc<str> = Rc::<str>::from((*v73.borrow()).join(&*v65));
            v74
        }
        US5::US5_WordsRoot => {
            v1
        }
    };
    let v77: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v76);
    if v77 {
        let v78: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let v79: Rc<str> = Rc::<str>::from(v78.replace(&*v6, &*v7));
        let mut v80: Rc<str> = loop_5(v79);
        let v81: Option<usize> = v80.rfind(&*v7);
        let v82: Option<usize> = method6(v81);
        let mut v83: Option<i32> = v82.map(|x| v12(x));
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
        let v111: US2 = if v90 {
            US2::US2_OtherFile
        } else {
            let v92: usize = ((0i32) as usize);
            let mut v93: usize = ((v85) as usize);
            let mut v94: Rc<str> = Rc::<str>::from(&(v80)[v92..v93]);
            let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v96: bool = v94.ends_with(&*v95);
            if v96 {
                let v97: Rc<str> = loop_7(v94);
                US2::US2_StateFile(v97)
            } else {
                let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v100: bool = v94.ends_with(&*v99);
                if v100 {
                    let v101: Rc<str> = loop_8(v94);
                    US2::US2_HistoryFile(v101)
                } else {
                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v104: bool = v94.ends_with(&*v103);
                    if v104 {
                        let v105: Rc<str> = loop_9(v94);
                        US2::US2_ArchiveFile(v105)
                    } else {
                        US2::US2_OtherFile
                    }
                }
            }
        };
        let v134: US5 = match v111 {
            US2::US2_ArchiveFile(v124) => {
                let v124: Rc<str> = v124;
                let v125: Rc<str> = string_concat(&[&*v124, "/state/spiral.toml"]);
                let v126: bool = std::path::Path::new(&*v125).is_file();
                if v126 {
                    US5::US5_SymbolsRoot
                } else {
                    US5::US5_WordsRoot
                }
            }
            US2::US2_HistoryFile(v118) => {
                let v118: Rc<str> = v118;
                let v119: Rc<str> = string_concat(&[&*v118, "/state/spiral.toml"]);
                let mut v120: bool = std::path::Path::new(&*v119).is_file();
                if v120 {
                    US5::US5_SymbolsRoot
                } else {
                    US5::US5_WordsRoot
                }
            }
            US2::US2_OtherFile => {
                US5::US5_WordsRoot
            }
            US2::US2_StateFile(v112) => {
                let v112: Rc<str> = v112;
                let v113: Rc<str> = string_concat(&[&*v112, "/state/spiral.toml"]);
                let v114: bool = std::path::Path::new(&*v113).is_file();
                if v114 {
                    US5::US5_SymbolsRoot
                } else {
                    US5::US5_WordsRoot
                }
            }
        };
        let v146: Rc<str> = match v134 {
            US5::US5_SymbolsRoot => {
                let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v136: Rc<UH0> = split_list_10(v135.clone(), v2.clone());
                let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v138: Rc<UH0> = method33(v136, v137);
                let v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v140: i32 = 0i32;
                let v141: i32 = method14(v139.clone(), v138, v140);
                let v142: Rc<Vec<Rc<str>>> = Rc::new(v139.borrow().clone());
                let v143: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v142).as_ref().clone()));
                let v144: Rc<str> = Rc::<str>::from((*v143.borrow()).join(&*v135));
                v144
            }
            US5::US5_WordsRoot => {
                v2.clone()
            }
        };
        let v147: bool = std::fs::write(&*v78, &*v146).is_ok();
        let v228: US6 = if v147 {
            let v148: i32 = 40i32;
            let v149: bool = renamed_within_34(v0.clone(), v78.clone(), v148);
            if v149 {
                let v150: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
                let mut v151: Rc<str> = loop_5(v150);
                let v152: Option<usize> = v151.rfind(&*v7);
                let v153: Option<usize> = method6(v152);
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
                let v182: US2 = if v161 {
                    US2::US2_OtherFile
                } else {
                    let v163: usize = ((0i32) as usize);
                    let v164: usize = ((v156) as usize);
                    let v165: Rc<str> = Rc::<str>::from(&(v151)[v163..v164]);
                    let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v167: bool = v165.ends_with(&*v166);
                    if v167 {
                        let v168: Rc<str> = loop_7(v165);
                        US2::US2_StateFile(v168)
                    } else {
                        let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v171: bool = v165.ends_with(&*v170);
                        if v171 {
                            let v172: Rc<str> = loop_8(v165);
                            US2::US2_HistoryFile(v172)
                        } else {
                            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v175: bool = v165.ends_with(&*v174);
                            if v175 {
                                let v176: Rc<str> = loop_9(v165);
                                US2::US2_ArchiveFile(v176)
                            } else {
                                US2::US2_OtherFile
                            }
                        }
                    }
                };
                let v205: US5 = match v182 {
                    US2::US2_ArchiveFile(v195) => {
                        let v195: Rc<str> = v195;
                        let v196: Rc<str> = string_concat(&[&*v195, "/state/spiral.toml"]);
                        let v197: bool = std::path::Path::new(&*v196).is_file();
                        if v197 {
                            US5::US5_SymbolsRoot
                        } else {
                            US5::US5_WordsRoot
                        }
                    }
                    US2::US2_HistoryFile(v189) => {
                        let v189: Rc<str> = v189;
                        let v190: Rc<str> = string_concat(&[&*v189, "/state/spiral.toml"]);
                        let v191: bool = std::path::Path::new(&*v190).is_file();
                        if v191 {
                            US5::US5_SymbolsRoot
                        } else {
                            US5::US5_WordsRoot
                        }
                    }
                    US2::US2_OtherFile => {
                        US5::US5_WordsRoot
                    }
                    US2::US2_StateFile(v183) => {
                        let v183: Rc<str> = v183;
                        let v184: Rc<str> = string_concat(&[&*v183, "/state/spiral.toml"]);
                        let v185: bool = std::path::Path::new(&*v184).is_file();
                        if v185 {
                            US5::US5_SymbolsRoot
                        } else {
                            US5::US5_WordsRoot
                        }
                    }
                };
                let v217: Rc<str> = match v205 {
                    US5::US5_SymbolsRoot => {
                        let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v207: Rc<UH0> = split_list_10(v206.clone(), v2);
                        let v208: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v209: Rc<UH0> = method33(v207, v208);
                        let v210: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v211: i32 = 0i32;
                        let v212: i32 = method14(v210.clone(), v209, v211);
                        let v213: Rc<Vec<Rc<str>>> = Rc::new(v210.borrow().clone());
                        let v214: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v213).as_ref().clone()));
                        let v215: Rc<str> = Rc::<str>::from((*v214.borrow()).join(&*v206));
                        v215
                    }
                    US5::US5_WordsRoot => {
                        v2
                    }
                };
                let v218: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v217);
                if v218 {
                    US6::US6_Replaced
                } else {
                    US6::US6_ReadbackMismatch
                }
            } else {
                let v222: bool = std::fs::remove_file(&*v78).is_ok() || !std::path::Path::new(&*v78).exists();
                if v222 {
                    US6::US6_RenameFailed
                } else {
                    US6::US6_RenameFailedTemporaryKept
                }
            }
        } else {
            US6::US6_TemporaryWriteFailed
        };
        match &v228 {
            US6::US6_Replaced => {
                US7::US7_StateWritten
            }
            _ => {
                let v239: Rc<str> = match v228 {
                    US6::US6_ChangedSinceRead => {
                        let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v234
                    }
                    US6::US6_ReadbackMismatch => {
                        let v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v233
                    }
                    US6::US6_RenameFailed => {
                        let v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v231
                    }
                    US6::US6_RenameFailedTemporaryKept => {
                        let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v232
                    }
                    US6::US6_TemporaryWriteFailed => {
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
fn sealed_written_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<RefCell<Vec<Rc<str>>>>, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: Rc<str>) -> Rc<str> {
    let mut v9: Rc<str> = archive_path_24(v0.clone());
    let v10: bool = std::path::Path::new(&*v9).is_file();
    let v23: Rc<str> = if v10 {
        let mut v12: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v9);
        let v14: Vec<u8> = v12.unwrap();
        let v15: Vec<u8> = method25(v14);
        let v17: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v15);
        let v19: std::string::String = v17.unwrap();
        let mut v21: Rc<str> = Rc::<str>::from(String::as_str(&v19));
        v21
    } else {
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v22
    };
    let v24: Rc<str> = string_concat(&["\n", &*v23]);
    let v25: Rc<RefCell<Vec<Rc<str>>>> = fresh_rows_26(v24, v4);
    let v26: Rc<str> = string_concat(&[&*v0, "/archive"]);
    let v28: Result<(), std::io::Error> = std::fs::create_dir_all(&*v26);
    let v29: bool = &*v23 == "";
    let v32: bool = if v29 {
        true
    } else {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v31: bool = v23.ends_with(&*v30);
        v31
    };
    let v34: Rc<str> = if v32 {
        v23.clone()
    } else {
        let v33: Rc<str> = string_concat(&[&*v23, "\n"]);
        v33
    };
    let v35: i32 = (v25.clone().borrow().len() as i32);
    let v36: bool = v35 == 0i32;
    let v40: Rc<str> = if v36 {
        v23
    } else {
        let v37: Rc<str> = joined_lines_22(v25);
        let v39: Rc<str> = string_concat(&[&*v34, &*v37, "\n"]);
        v39
    };
    let v41: Rc<str> = string_concat(&[&*v9, ".pending"]);
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = Rc::<str>::from(v41.replace(&*v42, &*v43));
    let mut v45: Rc<str> = loop_5(v44);
    let v46: Option<usize> = v45.rfind(&*v43);
    let v47: Option<usize> = method6(v46);
    let mut v48: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v49: Option<i32> = v47.map(|x| v48(x));
    let v51: i32 = v49.unwrap_or(-1i32);
    let v52: bool = v51 < 0i32;
    let v56: bool = if v52 {
        true
    } else {
        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v54: bool = v45.ends_with(&*v53);
        let v55: bool = v54 == false;
        v55
    };
    let v77: US2 = if v56 {
        US2::US2_OtherFile
    } else {
        let v58: usize = ((0i32) as usize);
        let v59: usize = ((v51) as usize);
        let v60: Rc<str> = Rc::<str>::from(&(v45)[v58..v59]);
        let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v62: bool = v60.ends_with(&*v61);
        if v62 {
            let v63: Rc<str> = loop_7(v60);
            US2::US2_StateFile(v63)
        } else {
            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
            let v66: bool = v60.ends_with(&*v65);
            if v66 {
                let v67: Rc<str> = loop_8(v60);
                US2::US2_HistoryFile(v67)
            } else {
                let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                let v70: bool = v60.ends_with(&*v69);
                if v70 {
                    let v71: Rc<str> = loop_9(v60);
                    US2::US2_ArchiveFile(v71)
                } else {
                    US2::US2_OtherFile
                }
            }
        }
    };
    let v100: US5 = match v77 {
        US2::US2_ArchiveFile(v90) => {
            let v90: Rc<str> = v90;
            let v91: Rc<str> = string_concat(&[&*v90, "/state/spiral.toml"]);
            let v92: bool = std::path::Path::new(&*v91).is_file();
            if v92 {
                US5::US5_SymbolsRoot
            } else {
                US5::US5_WordsRoot
            }
        }
        US2::US2_HistoryFile(v84) => {
            let v84: Rc<str> = v84;
            let v85: Rc<str> = string_concat(&[&*v84, "/state/spiral.toml"]);
            let v86: bool = std::path::Path::new(&*v85).is_file();
            if v86 {
                US5::US5_SymbolsRoot
            } else {
                US5::US5_WordsRoot
            }
        }
        US2::US2_OtherFile => {
            US5::US5_WordsRoot
        }
        US2::US2_StateFile(v78) => {
            let v78: Rc<str> = v78;
            let v79: Rc<str> = string_concat(&[&*v78, "/state/spiral.toml"]);
            let mut v80: bool = std::path::Path::new(&*v79).is_file();
            if v80 {
                US5::US5_SymbolsRoot
            } else {
                US5::US5_WordsRoot
            }
        }
    };
    let v112: Rc<str> = match v100 {
        US5::US5_SymbolsRoot => {
            let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v102: Rc<UH0> = split_list_10(v101.clone(), v40.clone());
            let v103: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v104: Rc<UH0> = method33(v102, v103);
            let v105: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v106: i32 = 0i32;
            let v107: i32 = method14(v105.clone(), v104, v106);
            let v108: Rc<Vec<Rc<str>>> = Rc::new(v105.borrow().clone());
            let v109: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v108).as_ref().clone()));
            let v110: Rc<str> = Rc::<str>::from((*v109.borrow()).join(&*v101));
            v110
        }
        US5::US5_WordsRoot => {
            v40.clone()
        }
    };
    let v113: bool = std::fs::write(&*v41, &*v112).is_ok();
    let v194: US6 = if v113 {
        let v114: i32 = 40i32;
        let v115: bool = renamed_within_34(v9.clone(), v41.clone(), v114);
        if v115 {
            let v116: Rc<str> = Rc::<str>::from(v9.replace(&*v42, &*v43));
            let mut v117: Rc<str> = loop_5(v116);
            let v118: Option<usize> = v117.rfind(&*v43);
            let v119: Option<usize> = method6(v118);
            let mut v120: Option<i32> = v119.map(|x| v48(x));
            let v122: i32 = v120.unwrap_or(-1i32);
            let v123: bool = v122 < 0i32;
            let v127: bool = if v123 {
                true
            } else {
                let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v125: bool = v117.ends_with(&*v124);
                let v126: bool = v125 == false;
                v126
            };
            let v148: US2 = if v127 {
                US2::US2_OtherFile
            } else {
                let v129: usize = ((0i32) as usize);
                let v130: usize = ((v122) as usize);
                let v131: Rc<str> = Rc::<str>::from(&(v117)[v129..v130]);
                let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v133: bool = v131.ends_with(&*v132);
                if v133 {
                    let v134: Rc<str> = loop_7(v131);
                    US2::US2_StateFile(v134)
                } else {
                    let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v137: bool = v131.ends_with(&*v136);
                    if v137 {
                        let v138: Rc<str> = loop_8(v131);
                        US2::US2_HistoryFile(v138)
                    } else {
                        let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v141: bool = v131.ends_with(&*v140);
                        if v141 {
                            let v142: Rc<str> = loop_9(v131);
                            US2::US2_ArchiveFile(v142)
                        } else {
                            US2::US2_OtherFile
                        }
                    }
                }
            };
            let v171: US5 = match v148 {
                US2::US2_ArchiveFile(v161) => {
                    let v161: Rc<str> = v161;
                    let v162: Rc<str> = string_concat(&[&*v161, "/state/spiral.toml"]);
                    let v163: bool = std::path::Path::new(&*v162).is_file();
                    if v163 {
                        US5::US5_SymbolsRoot
                    } else {
                        US5::US5_WordsRoot
                    }
                }
                US2::US2_HistoryFile(v155) => {
                    let v155: Rc<str> = v155;
                    let v156: Rc<str> = string_concat(&[&*v155, "/state/spiral.toml"]);
                    let v157: bool = std::path::Path::new(&*v156).is_file();
                    if v157 {
                        US5::US5_SymbolsRoot
                    } else {
                        US5::US5_WordsRoot
                    }
                }
                US2::US2_OtherFile => {
                    US5::US5_WordsRoot
                }
                US2::US2_StateFile(v149) => {
                    let v149: Rc<str> = v149;
                    let v150: Rc<str> = string_concat(&[&*v149, "/state/spiral.toml"]);
                    let mut v151: bool = std::path::Path::new(&*v150).is_file();
                    if v151 {
                        US5::US5_SymbolsRoot
                    } else {
                        US5::US5_WordsRoot
                    }
                }
            };
            let v183: Rc<str> = match v171 {
                US5::US5_SymbolsRoot => {
                    let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v173: Rc<UH0> = split_list_10(v172.clone(), v40);
                    let v174: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v175: Rc<UH0> = method33(v173, v174);
                    let v176: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v177: i32 = 0i32;
                    let v178: i32 = method14(v176.clone(), v175, v177);
                    let v179: Rc<Vec<Rc<str>>> = Rc::new(v176.borrow().clone());
                    let v180: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v179).as_ref().clone()));
                    let v181: Rc<str> = Rc::<str>::from((*v180.borrow()).join(&*v172));
                    v181
                }
                US5::US5_WordsRoot => {
                    v40
                }
            };
            let v184: bool = std::fs::read_to_string(&*v9).is_ok_and(|value| value == *v183);
            if v184 {
                US6::US6_Replaced
            } else {
                US6::US6_ReadbackMismatch
            }
        } else {
            let v188: bool = std::fs::remove_file(&*v41).is_ok() || !std::path::Path::new(&*v41).exists();
            if v188 {
                US6::US6_RenameFailed
            } else {
                US6::US6_RenameFailedTemporaryKept
            }
        }
    } else {
        US6::US6_TemporaryWriteFailed
    };
    match &v194 {
        US6::US6_Replaced => {
            let v195: Rc<str> = string_concat(&[&*v3, &*v8]);
            let v196: US7 = state_written_35(v1, v2, v195);
            match &v196 {
                US7::US7_StateWritten => {
                    let v197: Rc<str> = std::rc::Rc::<str>::from((v35).to_string());
                    v197
                }
                _ => {
                    let v201: Rc<str> = match v196 {
                        US7::US7_StateChangedSinceRead => {
                            let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                            v198
                        }
                        US7::US7_StateWriteFailed(v199) => {
                            let v199: Rc<str> = v199;
                            v199
                        }
                        _ => unreachable!(),
                    };
                    let v202: Rc<str> = string_concat(&["!", &*v201]);
                    v202
                }
            }
        }
        _ => {
            let v213: Rc<str> = match v194 {
                US6::US6_ChangedSinceRead => {
                    let v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                    v208
                }
                US6::US6_ReadbackMismatch => {
                    let v207: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                    v207
                }
                US6::US6_RenameFailed => {
                    let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                    v205
                }
                US6::US6_RenameFailedTemporaryKept => {
                    let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                    v206
                }
                US6::US6_TemporaryWriteFailed => {
                    let v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                    v204
                }
                _ => unreachable!(),
            };
            let v214: Rc<str> = string_concat(&["!archive ", &*v213]);
            v214
        }
    }
}
fn method36(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure6() -> Rc<dyn Fn((i32)) -> US8> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US8> = Rc::new(move |mut v0: (i32)| -> US8 {
        let v1: i32 = (v0);
        US8::US8_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn seal_run_1(mut v0: Rc<str>, mut v1: US0) -> i32 {
    let mut v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = concat5_4(v0.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    let mut v6: bool = std::path::Path::new(&*v5).is_file();
    let mut v9: US1 = if v6 {
        US1::US1_FileReadable(v5.clone())
    } else {
        US1::US1_FileMissing(v5.clone())
    };
    match v9 {
        US1::US1_FileMissing(v10) => {
            let v10: Rc<str> = v10;
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            seal_failure_2(v11)
        }
        US1::US1_FileReadable(v13) => {
            let mut v13: Rc<str> = v13;
            let v14: Rc<str> = std::fs::read_to_string(&*v13).unwrap_or_default().into();
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(v13.replace(&*v15, &*v16));
            let mut v18: Rc<str> = loop_5(v17);
            let v19: Option<usize> = v18.rfind(&*v16);
            let mut v20: Option<usize> = method6(v19);
            let mut v21: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v22: Option<i32> = v20.map(|x| v21(x));
            let v24: i32 = v22.unwrap_or(-1i32);
            let v25: bool = v24 < 0i32;
            let v29: bool = if v25 {
                true
            } else {
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v27: bool = v18.ends_with(&*v26);
                let v28: bool = v27 == false;
                v28
            };
            let v50: US2 = if v29 {
                US2::US2_OtherFile
            } else {
                let v31: usize = ((0i32) as usize);
                let v32: usize = ((v24) as usize);
                let v33: Rc<str> = Rc::<str>::from(&(v18)[v31..v32]);
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v35: bool = v33.ends_with(&*v34);
                if v35 {
                    let v36: Rc<str> = loop_7(v33);
                    US2::US2_StateFile(v36)
                } else {
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v39: bool = v33.ends_with(&*v38);
                    if v39 {
                        let v40: Rc<str> = loop_8(v33);
                        US2::US2_HistoryFile(v40)
                    } else {
                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v43: bool = v33.ends_with(&*v42);
                        if v43 {
                            let v44: Rc<str> = loop_9(v33);
                            US2::US2_ArchiveFile(v44)
                        } else {
                            US2::US2_OtherFile
                        }
                    }
                }
            };
            let v61: Rc<str> = match v50 {
                US2::US2_OtherFile => {
                    v14
                }
                _ => {
                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v52: Rc<UH0> = split_list_10(v51.clone(), v14);
                    let v53: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v54: Rc<UH0> = method13(v52, v53);
                    let v55: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v56: i32 = 0i32;
                    let v57: i32 = method14(v55.clone(), v54, v56);
                    let v58: Rc<Vec<Rc<str>>> = Rc::new(v55.borrow().clone());
                    let v59: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v58).as_ref().clone()));
                    let v60: Rc<str> = Rc::<str>::from((*v59.borrow()).join(&*v51));
                    v60
                }
            };
            let v62: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v61);
            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v64: bool = v62.contains(&*v63);
            if v64 {
                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi has CR line endings; sealing keeps LF only"); } LIT.with(|lit| lit.clone()) };
                seal_failure_2(v65)
            } else {
                let v67: Rc<str> = done_ids_15(v62.clone());
                let (mut v68, mut v69, mut v70, mut v71, mut v72): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, i32, i32, i32) = sealing_of_16(v67, v62.clone());
                let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v74: bool = v62.ends_with(&*v73);
                let v75: Rc<str> = if v74 {
                    v73
                } else {
                    v4
                };
                let v76: i32 = v70.wrapping_add(v71);
                let v77: bool = v76 != v72;
                if v77 {
                    let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the split is not a partition of the state lines; nothing written"); } LIT.with(|lit| lit.clone()) };
                    seal_failure_2(v78)
                } else {
                    match &v1 {
                        US0::US0_SealApply => {
                            let v85: Rc<str> = sealed_written_23(v0, v5, v61, v68.clone(), v69.clone(), v70, v71, v72, v75.clone());
                            let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!"); } LIT.with(|lit| lit.clone()) };
                            let v87: bool = v85.starts_with(&*v86);
                            if v87 {
                                let v88: i32 = (v85.clone().len() as i32);
                                let v89: Rc<str> = std::rc::Rc::<str>::from(v85.get(1i32 as usize..v88 as usize).unwrap_or(""));
                                seal_failure_2(v89)
                            } else {
                                let v91: Option<i32> = (v85).trim().parse::<i32>().ok();
                                let v92: Option<i32> = method36(v91);
                                let mut v93: Rc<dyn Fn((i32)) -> US8> = closure6();
                                let mut v94: Option<US8> = v92.map(|x| v93(x));
                                let v96: US8 = US8::US8_None;
                                let v97: US8 = v96.clone();
                                let v98: US8 = v94.unwrap_or(v97);
                                let v101: i32 = match v98 {
                                    US8::US8_None => {
                                        0i32
                                    }
                                    US8::US8_Some(v99) => {
                                        let v99: i32 = v99;
                                        v99
                                    }
                                };
                                let v102: u64 = v62.len() as u64;
                                let v103: Rc<str> = string_concat(&[&*v68, &*v75]);
                                let v104: u64 = v103.len() as u64;
                                seal_receipt_3(v1, v68, v69, v70, v71, v72, v101, v102, v104, v2)
                            }
                        }
                        US0::US0_SealDryRun => {
                            let mut v80: i32 = 0i32;
                            let v81: u64 = v62.len() as u64;
                            let v82: Rc<str> = string_concat(&[&*v68, &*v75]);
                            let mut v83: u64 = v82.len() as u64;
                            seal_receipt_3(v1, v68, v69, v70, v71, v72, v80, v81, v83, v2)
                        }
                    }
                }
            }
        }
    }
}
fn seal_entry_0() -> i32 {
    let mut v0: i32 = 1i32;
    let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: bool = &*v3 == "--dry-run";
    if v4 {
        let mut v5: US0 = US0::US0_SealDryRun;
        seal_run_1(v1, v5)
    } else {
        let mut v7: bool = &*v3 == "";
        if v7 {
            let v8: US0 = US0::US0_SealApply;
            seal_run_1(v1, v8)
        } else {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile seal <root> [--dry-run]"); } LIT.with(|lit| lit.clone()) };
            seal_failure_2(v10)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        seal_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_seal_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
