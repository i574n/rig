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
    US0_Refreshed(Rc<str>),
    US0_DryRun(Rc<str>),
    US0_NotListed,
    US0_OutsideRoot,
    US0_MissingFile,
    US0_WriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_StateFile(Rc<str>),
    US2_HistoryFile(Rc<str>),
    US2_ArchiveFile(Rc<str>),
    US2_OtherFile,
}
#[derive(Clone)]
enum US3 {
    US3_WordsRoot,
    US3_SymbolsRoot,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut1 { pub l0: i32, pub l1: Rc<UH0> }
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
    US6_Replaced,
    US6_TemporaryWriteFailed,
    US6_RenameFailed,
    US6_RenameFailedTemporaryKept,
    US6_ReadbackMismatch,
    US6_ChangedSinceRead,
}
fn stderr_line_1(mut v0: Rc<str>) -> i32 {
    eprintln!("{}", v0);
    2i32
}
fn stdout_line_2(mut v0: Rc<str>) -> i32 {
    println!("{}", v0);
    0i32
}
fn argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    v1
}
fn option_after_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|value| value.as_str() != &*v0).nth(1).unwrap_or_default().as_str());
    v1
}
fn has_flag_5(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::env::args().skip(0i32 as usize).any(|argument| argument == &*v0);
    v1
}
fn file_byte_count_7(mut v0: Rc<str>) -> u64 {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v0);
    let mut v2: bool = v1.is_ok();
    let v3: bool = v2 == false;
    let mut v7: i64 = if v3 {
        0i64
    } else {
        let v4: std::fs::Metadata = v1.unwrap();
        let v5: u64 = v4.len();
        let v6: i64 = (v5 as i64);
        v6
    };
    let mut v8: u64 = (v7 as u64);
    v8
}
fn file_sha256_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(rig_proxy_search::inspection_file_sha256(std::path::Path::new(&*v0)).unwrap_or_default().as_str());
    v1
}
fn method9(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method13(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_14(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn rewritten_row_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("core.EvidenceRef ("); } LIT.with(|lit| lit.clone()) };
    let v5: Option<usize> = v0.find(&*v4);
    let v6: Option<usize> = method13(v5);
    let mut v7: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v8: Option<i32> = v6.map(|x| v7(x));
    let v10: i32 = v8.unwrap_or(-1i32);
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) };
    let v12: Option<usize> = v0.find(&*v11);
    let v13: Option<usize> = method13(v12);
    let mut v14: Option<i32> = v13.map(|x| v7(x));
    let v16: i32 = v14.unwrap_or(-1i32);
    let v17: bool = v10 < 0i32;
    let v19: bool = if v17 {
        true
    } else {
        let v18: bool = v16 < 0i32;
        v18
    };
    if v19 {
        v0
    } else {
        let v20: usize = ((0i32) as usize);
        let v21: usize = ((v10) as usize);
        let v22: Rc<str> = Rc::<str>::from(&(v0)[v20..v21]);
        let v23: i32 = v16.wrapping_add(3i32);
        let v24: i32 = (v0.clone().len() as i32);
        let v25: i32 = v24.wrapping_sub(v16);
        let v26: i32 = v25.wrapping_sub(3i32);
        let v27: usize = ((v23) as usize);
        let v28: i32 = v23.wrapping_add(v26);
        let v29: usize = ((v28) as usize);
        let v30: Rc<str> = Rc::<str>::from(&(v0)[v27..v29]);
        let v37: Rc<str> = format_real_14(v3);
        let v40: Rc<str> = string_concat(&[&*v22, "core.EvidenceRef (\"", &*v1, "\", \"", &*v2, "\", ", &*v37, "u64", &*v30]);
        v40
    }
}
fn joined_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<RefCell<Vec<Rc<str>>>>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: bool) -> US1 {
    loop {
        let mut v9: bool = v6 >= v4;
        if v9 {
            if v8 {
                return US1::US1_Some(v7);
            } else {
                return US1::US1_None;
            }
        } else {
            let v13: Rc<str> = v3.borrow()[v6 as usize].clone();
            let mut v14: bool = v8 == false;
            let v16: bool = if v14 {
                let v15: bool = v13.contains(&*v5);
                v15
            } else {
                false
            };
            let v18: Rc<str> = if v16 {
                rewritten_row_12(v13, v0.clone(), v1.clone(), v2)
            } else {
                v13
            };
            let v19: bool = v6 == 0i32;
            let v22: Rc<str> = if v19 {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v20
            } else {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                v21
            };
            let v23: i32 = v6.wrapping_add(1i32);
            let v25: Rc<str> = string_concat(&[&*v7, &*v22, &*v18]);
            let v26: bool = v8 || v16;
            (v6, v7, v8) = (v23, v25, v26);
            continue;
        }
    }
}
fn refreshed_text_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> US1 {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v7: i32 = (v6.clone().borrow().len() as i32);
    let mut v9: Rc<str> = string_concat(&["core.EvidenceRef (\"", &*v1, "\", "]);
    let v10: i32 = 0i32;
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v12: bool = false;
    joined_11(v2, v3, v4, v6, v7, v9, v10, v11, v12)
}
fn loop_16(mut v0: Rc<str>) -> Rc<str> {
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
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_17(mut v0: Rc<str>) -> Rc<str> {
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
fn loop_18(mut v0: Rc<str>) -> Rc<str> {
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
fn loop_19(mut v0: Rc<str>) -> Rc<str> {
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
fn method21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method22(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method22(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method21(v16, v17)
    }
}
fn method23(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method23(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let v15: US4 = if v6 {
                let mut v7: US5 = US5::US5_HeadType;
                US4::US4_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US5 = US5::US5_HeadType;
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
fn method24(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn renamed_within_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn refresh_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool) -> US0 {
    let v4: bool = &*v2 == "";
    let v5: Rc<str> = if v4 {
        v1.clone()
    } else {
        v2
    };
    let v6: Rc<str> = string_concat(&[&*v0, "/state/evidence.spi"]);
    let mut v8: Rc<str> = string_concat(&[&*v0, "/", &*v5]);
    let mut v9: bool = &*v1 != "";
    let v13: bool = if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".."); } LIT.with(|lit| lit.clone()) };
        let v11: bool = v1.contains(&*v10);
        let v12: bool = v11 == false;
        v12
    } else {
        false
    };
    let v17: bool = if v13 {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let v15: bool = v1.contains(&*v14);
        let v16: bool = v15 == false;
        v16
    } else {
        false
    };
    let v21: bool = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v19: bool = v1.starts_with(&*v18);
        let v20: bool = v19 == false;
        v20
    } else {
        false
    };
    let v25: bool = if v21 {
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v23: bool = v1.starts_with(&*v22);
        let v24: bool = v23 == false;
        v24
    } else {
        false
    };
    let v26: bool = v25 == false;
    let v45: bool = if v26 {
        true
    } else {
        let v27: bool = &*v5 != "";
        let v31: bool = if v27 {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".."); } LIT.with(|lit| lit.clone()) };
            let v29: bool = v5.contains(&*v28);
            let v30: bool = v29 == false;
            v30
        } else {
            false
        };
        let v35: bool = if v31 {
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            let v33: bool = v5.contains(&*v32);
            let v34: bool = v33 == false;
            v34
        } else {
            false
        };
        let v39: bool = if v35 {
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v37: bool = v5.starts_with(&*v36);
            let v38: bool = v37 == false;
            v38
        } else {
            false
        };
        let v43: bool = if v39 {
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v41: bool = v5.starts_with(&*v40);
            let v42: bool = v41 == false;
            v42
        } else {
            false
        };
        let v44: bool = v43 == false;
        v44
    };
    if v45 {
        US0::US0_OutsideRoot
    } else {
        let v47: u64 = file_byte_count_7(v8.clone());
        let v48: bool = v47 == 0u64;
        if v48 {
            US0::US0_MissingFile
        } else {
            let v50: Rc<str> = file_sha256_8(v8.clone());
            let v51: u64 = file_byte_count_7(v8);
            let v53: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v6);
            let v55: Vec<u8> = v53.unwrap();
            let v56: Vec<u8> = method9(v55);
            let v58: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v56);
            let v60: std::string::String = v58.unwrap();
            let v62: Rc<str> = Rc::<str>::from(String::as_str(&v60));
            let v63: US1 = refreshed_text_10(v62, v1, v5.clone(), v50.clone(), v51);
            match v63 {
                US1::US1_None => {
                    US0::US0_NotListed
                }
                US1::US1_Some(v65) => {
                    let v65: Rc<str> = v65;
                    let v71: Rc<str> = format_real_14(v51);
                    let v73: Rc<str> = string_concat(&["core.EvidenceRef (\"", &*v5, "\", \"", &*v50, "\", ", &*v71, "u64"]);
                    if v3 {
                        US0::US0_DryRun(v73)
                    } else {
                        let v75: Rc<str> = string_concat(&[&*v6, ".pending"]);
                        let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                        let v78: Rc<str> = Rc::<str>::from(v75.replace(&*v76, &*v77));
                        let mut v79: Rc<str> = loop_16(v78);
                        let v80: Option<usize> = v79.rfind(&*v77);
                        let v81: Option<usize> = method13(v80);
                        let mut v82: Rc<dyn Fn((usize)) -> i32> = closure2();
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
                        let v111: US2 = if v90 {
                            US2::US2_OtherFile
                        } else {
                            let v92: usize = ((0i32) as usize);
                            let v93: usize = ((v85) as usize);
                            let v94: Rc<str> = Rc::<str>::from(&(v79)[v92..v93]);
                            let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                            let v96: bool = v94.ends_with(&*v95);
                            if v96 {
                                let v97: Rc<str> = loop_17(v94);
                                US2::US2_StateFile(v97)
                            } else {
                                let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                let v100: bool = v94.ends_with(&*v99);
                                if v100 {
                                    let v101: Rc<str> = loop_18(v94);
                                    US2::US2_HistoryFile(v101)
                                } else {
                                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                    let v104: bool = v94.ends_with(&*v103);
                                    if v104 {
                                        let v105: Rc<str> = loop_19(v94);
                                        US2::US2_ArchiveFile(v105)
                                    } else {
                                        US2::US2_OtherFile
                                    }
                                }
                            }
                        };
                        let v134: US3 = match v111 {
                            US2::US2_ArchiveFile(v124) => {
                                let v124: Rc<str> = v124;
                                let v125: Rc<str> = string_concat(&[&*v124, "/state/spiral.toml"]);
                                let v126: bool = std::path::Path::new(&*v125).is_file();
                                if v126 {
                                    US3::US3_SymbolsRoot
                                } else {
                                    US3::US3_WordsRoot
                                }
                            }
                            US2::US2_HistoryFile(v118) => {
                                let v118: Rc<str> = v118;
                                let v119: Rc<str> = string_concat(&[&*v118, "/state/spiral.toml"]);
                                let v120: bool = std::path::Path::new(&*v119).is_file();
                                if v120 {
                                    US3::US3_SymbolsRoot
                                } else {
                                    US3::US3_WordsRoot
                                }
                            }
                            US2::US2_OtherFile => {
                                US3::US3_WordsRoot
                            }
                            US2::US2_StateFile(v112) => {
                                let v112: Rc<str> = v112;
                                let v113: Rc<str> = string_concat(&[&*v112, "/state/spiral.toml"]);
                                let v114: bool = std::path::Path::new(&*v113).is_file();
                                if v114 {
                                    US3::US3_SymbolsRoot
                                } else {
                                    US3::US3_WordsRoot
                                }
                            }
                        };
                        let v146: Rc<str> = match v134 {
                            US3::US3_SymbolsRoot => {
                                let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let v136: Rc<UH0> = split_list_20(v135.clone(), v65.clone());
                                let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                let v138: Rc<UH0> = method23(v136, v137);
                                let v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                let v140: i32 = 0i32;
                                let v141: i32 = method24(v139.clone(), v138, v140);
                                let v142: Rc<Vec<Rc<str>>> = Rc::new(v139.borrow().clone());
                                let v143: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v142).as_ref().clone()));
                                let v144: Rc<str> = Rc::<str>::from((*v143.borrow()).join(&*v135));
                                v144
                            }
                            US3::US3_WordsRoot => {
                                v65.clone()
                            }
                        };
                        let v147: bool = std::fs::write(&*v75, &*v146).is_ok();
                        let v228: US6 = if v147 {
                            let v148: i32 = 40i32;
                            let v149: bool = renamed_within_25(v6.clone(), v75.clone(), v148);
                            if v149 {
                                let v150: Rc<str> = Rc::<str>::from(v6.replace(&*v76, &*v77));
                                let mut v151: Rc<str> = loop_16(v150);
                                let v152: Option<usize> = v151.rfind(&*v77);
                                let v153: Option<usize> = method13(v152);
                                let mut v154: Option<i32> = v153.map(|x| v82(x));
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
                                        let v168: Rc<str> = loop_17(v165);
                                        US2::US2_StateFile(v168)
                                    } else {
                                        let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                        let v171: bool = v165.ends_with(&*v170);
                                        if v171 {
                                            let v172: Rc<str> = loop_18(v165);
                                            US2::US2_HistoryFile(v172)
                                        } else {
                                            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                            let v175: bool = v165.ends_with(&*v174);
                                            if v175 {
                                                let v176: Rc<str> = loop_19(v165);
                                                US2::US2_ArchiveFile(v176)
                                            } else {
                                                US2::US2_OtherFile
                                            }
                                        }
                                    }
                                };
                                let v205: US3 = match v182 {
                                    US2::US2_ArchiveFile(v195) => {
                                        let v195: Rc<str> = v195;
                                        let v196: Rc<str> = string_concat(&[&*v195, "/state/spiral.toml"]);
                                        let v197: bool = std::path::Path::new(&*v196).is_file();
                                        if v197 {
                                            US3::US3_SymbolsRoot
                                        } else {
                                            US3::US3_WordsRoot
                                        }
                                    }
                                    US2::US2_HistoryFile(v189) => {
                                        let v189: Rc<str> = v189;
                                        let v190: Rc<str> = string_concat(&[&*v189, "/state/spiral.toml"]);
                                        let v191: bool = std::path::Path::new(&*v190).is_file();
                                        if v191 {
                                            US3::US3_SymbolsRoot
                                        } else {
                                            US3::US3_WordsRoot
                                        }
                                    }
                                    US2::US2_OtherFile => {
                                        US3::US3_WordsRoot
                                    }
                                    US2::US2_StateFile(v183) => {
                                        let v183: Rc<str> = v183;
                                        let v184: Rc<str> = string_concat(&[&*v183, "/state/spiral.toml"]);
                                        let v185: bool = std::path::Path::new(&*v184).is_file();
                                        if v185 {
                                            US3::US3_SymbolsRoot
                                        } else {
                                            US3::US3_WordsRoot
                                        }
                                    }
                                };
                                let v217: Rc<str> = match v205 {
                                    US3::US3_SymbolsRoot => {
                                        let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                        let v207: Rc<UH0> = split_list_20(v206.clone(), v65);
                                        let v208: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                        let v209: Rc<UH0> = method23(v207, v208);
                                        let v210: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                        let v211: i32 = 0i32;
                                        let v212: i32 = method24(v210.clone(), v209, v211);
                                        let v213: Rc<Vec<Rc<str>>> = Rc::new(v210.borrow().clone());
                                        let v214: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v213).as_ref().clone()));
                                        let v215: Rc<str> = Rc::<str>::from((*v214.borrow()).join(&*v206));
                                        v215
                                    }
                                    US3::US3_WordsRoot => {
                                        v65
                                    }
                                };
                                let v218: bool = std::fs::read_to_string(&*v6).is_ok_and(|value| value == *v217);
                                if v218 {
                                    US6::US6_Replaced
                                } else {
                                    US6::US6_ReadbackMismatch
                                }
                            } else {
                                let v222: bool = std::fs::remove_file(&*v75).is_ok() || !std::path::Path::new(&*v75).exists();
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
                                US0::US0_Refreshed(v73)
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
                                US0::US0_WriteFailed(v239)
                            }
                        }
                    }
                }
            }
        }
    }
}
fn evidence_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = argument_3(v0);
    let mut v2: bool = &*v1 != "refresh";
    let v6: bool = if v2 {
        true
    } else {
        let v3: i32 = 3i32;
        let v4: Rc<str> = argument_3(v3);
        let v5: bool = &*v4 == "";
        v5
    };
    let v10: bool = if v6 {
        true
    } else {
        let mut v7: i32 = 4i32;
        let mut v8: Rc<str> = argument_3(v7);
        let mut v9: bool = &*v8 == "";
        v9
    };
    if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit evidence refresh <root> <relative path> [--as <new relative path>] [--dry-run]"); } LIT.with(|lit| lit.clone()) };
        stderr_line_1(v11)
    } else {
        let v13: i32 = 3i32;
        let mut v14: Rc<str> = argument_3(v13);
        let v15: i32 = 4i32;
        let v16: Rc<str> = argument_3(v15);
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = option_after_4(v17);
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dry-run"); } LIT.with(|lit| lit.clone()) };
        let v20: bool = has_flag_5(v19);
        let v21: US0 = refresh_6(v14, v16, v18, v20);
        match v21 {
            US0::US0_DryRun(v25) => {
                let v25: Rc<str> = v25;
                let v26: Rc<str> = string_concat(&["rig evidence refresh dry-run (nothing written) ", &*v25]);
                stdout_line_2(v26)
            }
            US0::US0_MissingFile => {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig evidence refresh: the file to hash is missing or empty"); } LIT.with(|lit| lit.clone()) };
                stderr_line_1(v32)
            }
            US0::US0_NotListed => {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig evidence refresh: that path has no EvidenceRef row in state/evidence.spi"); } LIT.with(|lit| lit.clone()) };
                stderr_line_1(v28)
            }
            US0::US0_OutsideRoot => {
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig evidence refresh: the path must be relative and stay inside the root"); } LIT.with(|lit| lit.clone()) };
                stderr_line_1(v30)
            }
            US0::US0_Refreshed(v22) => {
                let v22: Rc<str> = v22;
                let v23: Rc<str> = string_concat(&["rig evidence refresh ok ", &*v22]);
                stdout_line_2(v23)
            }
            US0::US0_WriteFailed(v34) => {
                let v34: Rc<str> = v34;
                let v35: Rc<str> = string_concat(&["rig evidence refresh: ", &*v34]);
                stderr_line_1(v35)
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        evidence_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_evidence_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
pub type SpiralShape_652accfc6a8d = Mut1;
