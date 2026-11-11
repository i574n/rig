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
    US0_StateFile(Rc<str>),
    US0_HistoryFile(Rc<str>),
    US0_ArchiveFile(Rc<str>),
    US0_OtherFile,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_HeadInl,
    US2_HeadUnion,
    US2_HeadOpen,
    US2_HeadType,
}
#[derive(Clone)]
enum US1 {
    US1_KeywordHead(US2),
    US1_SymbolHead(US2),
    US1_OtherLine,
}
#[derive(Clone)]
enum US3 {
    US3_MemoryUser,
    US3_MemoryFeedback,
    US3_MemoryProject,
    US3_MemoryReference,
    US3_MemoryUntyped,
}
#[derive(Clone)]
enum US4 {
    US4_CitesOpen,
    US4_CitesDone,
    US4_CitesMissing,
}
#[derive(Clone)]
enum US5 {
    US5_NoFinding,
    US5_DanglingId(Rc<str>, Rc<str>),
    US5_DoneCited(Rc<str>, Rc<str>),
    US5_UnwrittenLink(Rc<str>, Rc<str>),
    US5_UnknownMemory(Rc<str>, Rc<str>),
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US6 {
    US6_Some(Rc<str>),
    US6_None,
}
#[derive(Clone)]
enum US8 {
    US8_SwitchJson,
}
#[derive(Clone)]
enum US7 {
    US7_Some(US8),
    US7_None,
}
#[derive(Clone)]
enum US10 {
    US10_ValuedMemory,
}
#[derive(Clone)]
enum US9 {
    US9_Some(US10),
    US9_None,
}
fn directory_exists_2(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn loop_3(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn loop_6(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
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
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method9(v16, v17)
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method11(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US1 = if v6 {
                let v7: US2 = US2::US2_HeadType;
                US1::US1_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US2 = US2::US2_HeadType;
                    US1::US1_SymbolHead(v11)
                } else {
                    US1::US1_OtherLine
                }
            };
            let mut v27: US1 = match &v15 {
                US1::US1_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let mut v18: US2 = US2::US2_HeadInl;
                        US1::US1_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let mut v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US2 = US2::US2_HeadInl;
                            US1::US1_SymbolHead(v22)
                        } else {
                            US1::US1_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US1 = match &v27 {
                US1::US1_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US2 = US2::US2_HeadUnion;
                        US1::US1_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US2 = US2::US2_HeadUnion;
                            US1::US1_SymbolHead(v34)
                        } else {
                            US1::US1_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US1 = match &v39 {
                US1::US1_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US2 = US2::US2_HeadOpen;
                        US1::US1_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US2 = US2::US2_HeadOpen;
                            US1::US1_SymbolHead(v46)
                        } else {
                            US1::US1_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US1::US1_SymbolHead(v52) => {
                    let v52: US2 = v52;
                    let v60: Rc<str> = match &v52 {
                        US2::US2_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US2::US2_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US2::US2_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US2::US2_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let mut v68: Rc<str> = match v52 {
                        US2::US2_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US2::US2_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US2::US2_HeadType => {
                            v5
                        }
                        US2::US2_HeadUnion => {
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
fn method12(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
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
fn names_from_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = &*v1 == "";
        if v3 {
            return v2;
        } else {
            let v4: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(10u8)).next().unwrap_or(""));
            let v5: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v4).to_string_lossy().as_ref());
            let mut v6: bool = std::path::Path::new(&*v5).is_file();
            let v60: Rc<str> = if v6 {
                let (mut v7, mut v8): (bool, Rc<str>) = match std::fs::read(&*v5) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                if v7 {
                    let mut v9: Rc<str> = string_concat(&["rig state read failed: ", &*v5]);
                    std::panic::panic_any::<std::string::String>(format!("{}", v9))
                } else {
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let v13: Rc<str> = Rc::<str>::from(v5.replace(&*v11, &*v12));
                    let mut v14: Rc<str> = loop_3(v13);
                    let v15: Option<usize> = v14.rfind(&*v12);
                    let v16: Option<usize> = method4(v15);
                    let mut v17: Rc<dyn Fn((usize)) -> i32> = closure1();
                    let mut v18: Option<i32> = v16.map(|x| v17(x));
                    let v20: i32 = v18.unwrap_or(-1i32);
                    let mut v21: bool = v20 < 0i32;
                    let mut v25: bool = if v21 {
                        true
                    } else {
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                        let mut v23: bool = v14.ends_with(&*v22);
                        let mut v24: bool = v23 == false;
                        v24
                    };
                    let v46: US0 = if v25 {
                        US0::US0_OtherFile
                    } else {
                        let mut v27: usize = ((0i32) as usize);
                        let v28: usize = ((v20) as usize);
                        let v29: Rc<str> = Rc::<str>::from(&(v14)[v27..v28]);
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                        let v31: bool = v29.ends_with(&*v30);
                        if v31 {
                            let v32: Rc<str> = loop_5(v29);
                            US0::US0_StateFile(v32)
                        } else {
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                            let v35: bool = v29.ends_with(&*v34);
                            if v35 {
                                let v36: Rc<str> = loop_6(v29);
                                US0::US0_HistoryFile(v36)
                            } else {
                                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                let v39: bool = v29.ends_with(&*v38);
                                if v39 {
                                    let v40: Rc<str> = loop_7(v29);
                                    US0::US0_ArchiveFile(v40)
                                } else {
                                    US0::US0_OtherFile
                                }
                            }
                        }
                    };
                    match v46 {
                        US0::US0_OtherFile => {
                            v8
                        }
                        _ => {
                            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let v48: Rc<UH0> = split_list_8(v47.clone(), v8);
                            let v49: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v50: Rc<UH0> = method11(v48, v49);
                            let v51: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v52: i32 = 0i32;
                            let v53: i32 = method12(v51.clone(), v50, v52);
                            let v54: Rc<Vec<Rc<str>>> = Rc::new(v51.borrow().clone());
                            let v55: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v54).as_ref().clone()));
                            let v56: Rc<str> = Rc::<str>::from((*v55.borrow()).join(&*v47));
                            v56
                        }
                    }
                }
            } else {
                let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v59
            };
            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("name"); } LIT.with(|lit| lit.clone()) };
            let v62: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v60; let front = text.strip_prefix("---").and_then(|rest| rest.split_once("
---").map(|(front, _)| front)).unwrap_or(""); front.lines().find_map(|line| line.trim().strip_prefix(&*v61).and_then(|rest| rest.strip_prefix(char::from(58u8))).map(|value| value.trim().trim_matches(char::from(34u8)).to_owned())).unwrap_or_default() }.as_str());
            let v63: bool = &*v62 == "";
            let v65: Rc<str> = if v63 {
                let v64: Rc<str> = std::rc::Rc::<str>::from((v4).trim_end_matches(".md"));
                v64
            } else {
                v62
            };
            let v66: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let v67: bool = &*v2 == "";
            let v70: Rc<str> = if v67 {
                v65
            } else {
                let v69: Rc<str> = string_concat(&[&*v2, "|", &*v65]);
                v69
            };
            (v1, v2) = (v66, v70);
            continue;
        }
    }
}
fn links_start_14() -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0i32, 0i32, 0i32, 0i32, 0i32, 0i32, v0)
}
fn ids_checked_16(mut v0: Rc<str>, mut v1: US3, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    loop {
        let v11: bool = &*v3 == "";
        if v11 {
            return (v4, v5, v6, v7, v8, v9, v10);
        } else {
            let v12: Rc<str> = std::rc::Rc::<str>::from((v3).split(char::from(10u8)).next().unwrap_or(""));
            let v13: i32 = v5.wrapping_add(1i32);
            let mut v14: Rc<str> = std::rc::Rc::<str>::from((v3).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let v15: Rc<str> = agile_task_census::rig_agile_task_status(&*v0, &*v12);
            let v16: bool = &*v15 == "";
            let v22: US4 = if v16 {
                US4::US4_CitesMissing
            } else {
                let mut v18: bool = &*v15 == "Done";
                if v18 {
                    US4::US4_CitesDone
                } else {
                    US4::US4_CitesOpen
                }
            };
            let v32: US5 = match v22 {
                US4::US4_CitesDone => {
                    let mut v25: bool = match &v1 {
                        US3::US3_MemoryFeedback => {
                            true
                        }
                        US3::US3_MemoryProject => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    if v25 {
                        US5::US5_DoneCited(v2.clone(), v12)
                    } else {
                        US5::US5_NoFinding
                    }
                }
                US4::US4_CitesMissing => {
                    US5::US5_DanglingId(v2.clone(), v12)
                }
                US4::US4_CitesOpen => {
                    US5::US5_NoFinding
                }
            };
            let (mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v32 {
                US5::US5_NoFinding => {
                    (v4, v13, v6, v7, v8, v9, v10)
                }
                _ => {
                    let (mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v32 {
                        US5::US5_DanglingId(v33, v34) => {
                            let v33: Rc<str> = v33.clone();
                            let v34: Rc<str> = v34.clone();
                            let v35: i32 = v6.wrapping_add(1i32);
                            let v40: Rc<str> = string_concat(&[&*v10, "dangling  ", &*v33, "  ", &*v34, "  no backlog item with this id\n"]);
                            (v4, v13, v35, v7, v8, v9, v40)
                        }
                        _ => {
                            let (mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v32 {
                                US5::US5_DoneCited(v41, v42) => {
                                    let v41: Rc<str> = v41.clone();
                                    let v42: Rc<str> = v42.clone();
                                    let v43: i32 = v7.wrapping_add(1i32);
                                    let v48: Rc<str> = string_concat(&[&*v10, "done  ", &*v41, "  ", &*v42, "  the item is Done; check that the rule still holds\n"]);
                                    (v4, v13, v6, v43, v8, v9, v48)
                                }
                                _ => {
                                    let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v32 {
                                        US5::US5_UnwrittenLink(v49, v50) => {
                                            let v49: Rc<str> = v49.clone();
                                            let v50: Rc<str> = v50.clone();
                                            let v51: i32 = v8.wrapping_add(1i32);
                                            let v56: Rc<str> = string_concat(&[&*v10, "unwritten-link  ", &*v49, "  [[", &*v50, "]]  no memory with this name yet\n"]);
                                            (v4, v13, v6, v7, v51, v9, v56)
                                        }
                                        _ => {
                                            let (mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (i32, i32, i32, i32, i32, i32, Rc<str>) = match v32 {
                                                US5::US5_UnknownMemory(v57, v58) => {
                                                    let v57: Rc<str> = v57;
                                                    let v58: Rc<str> = v58;
                                                    let v59: i32 = v9.wrapping_add(1i32);
                                                    let v64: Rc<str> = string_concat(&[&*v10, "unknown-memory  ", &*v57, "  ", &*v58, "  a backlog note cites this memory, which does not exist\n"]);
                                                    (v4, v13, v6, v7, v8, v59, v64)
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v65, v66, v67, v68, v69, v70, v71)
                                        }
                                    };
                                    (v72, v73, v74, v75, v76, v77, v78)
                                }
                            };
                            (v79, v80, v81, v82, v83, v84, v85)
                        }
                    };
                    (v86, v87, v88, v89, v90, v91, v92)
                }
            };
            (v3, v4, v5, v6, v7, v8, v9, v10) = (v14, v93, v94, v95, v96, v97, v98, v99);
            continue;
        }
    }
}
fn method19(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
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
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method20(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
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
fn loop_18(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let v4: bool = v3 >= v2;
        if v4 {
            return false;
        } else {
            let v5: Rc<str> = v1.borrow()[v3 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let v7: i32 = 0i32;
            let v8: i32 = method19(v5.clone(), v6, v7);
            let mut v9: i32 = v6.wrapping_sub(1i32);
            let v10: Rc<str> = string_slice(&v5, v8 as i64, v9 as i64);
            let v11: i32 = (v10.clone().len() as i32);
            let v12: i32 = method20(v10.clone(), v11);
            let v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
            let mut v14: bool = &*v13 == &*v0;
            if v14 {
                return true;
            } else {
                let v15: i32 = v3.wrapping_add(1i32);
                v3 = v15;
                continue;
            }
        }
    }
}
fn links_checked_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    loop {
        let v10: bool = &*v2 == "";
        if v10 {
            return (v3, v4, v5, v6, v7, v8, v9);
        } else {
            let v11: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let v12: Rc<str> = std::rc::Rc::<str>::from((v2).split(char::from(10u8)).next().unwrap_or(""));
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v13).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v15: i32 = (v14.clone().borrow().len() as i32);
            let v16: i32 = 0i32;
            let mut v17: bool = loop_18(v12.clone(), &v14, v15, v16);
            let v20: US5 = if v17 {
                US5::US5_NoFinding
            } else {
                US5::US5_UnwrittenLink(v1.clone(), v12)
            };
            let (mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v20 {
                US5::US5_NoFinding => {
                    (v3, v4, v5, v6, v7, v8, v9)
                }
                _ => {
                    let (mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v20 {
                        US5::US5_DanglingId(v21, v22) => {
                            let mut v21: Rc<str> = v21.clone();
                            let v22: Rc<str> = v22.clone();
                            let mut v23: i32 = v5.wrapping_add(1i32);
                            let v28: Rc<str> = string_concat(&[&*v9, "dangling  ", &*v21, "  ", &*v22, "  no backlog item with this id\n"]);
                            (v3, v4, v23, v6, v7, v8, v28)
                        }
                        _ => {
                            let (mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v20 {
                                US5::US5_DoneCited(v29, v30) => {
                                    let v29: Rc<str> = v29.clone();
                                    let v30: Rc<str> = v30.clone();
                                    let v31: i32 = v6.wrapping_add(1i32);
                                    let v36: Rc<str> = string_concat(&[&*v9, "done  ", &*v29, "  ", &*v30, "  the item is Done; check that the rule still holds\n"]);
                                    (v3, v4, v5, v31, v7, v8, v36)
                                }
                                _ => {
                                    let (mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v20 {
                                        US5::US5_UnwrittenLink(v37, v38) => {
                                            let v37: Rc<str> = v37.clone();
                                            let v38: Rc<str> = v38.clone();
                                            let v39: i32 = v7.wrapping_add(1i32);
                                            let v44: Rc<str> = string_concat(&[&*v9, "unwritten-link  ", &*v37, "  [[", &*v38, "]]  no memory with this name yet\n"]);
                                            (v3, v4, v5, v6, v39, v8, v44)
                                        }
                                        _ => {
                                            let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59): (i32, i32, i32, i32, i32, i32, Rc<str>) = match v20 {
                                                US5::US5_UnknownMemory(v45, v46) => {
                                                    let v45: Rc<str> = v45;
                                                    let v46: Rc<str> = v46;
                                                    let v47: i32 = v8.wrapping_add(1i32);
                                                    let v52: Rc<str> = string_concat(&[&*v9, "unknown-memory  ", &*v45, "  ", &*v46, "  a backlog note cites this memory, which does not exist\n"]);
                                                    (v3, v4, v5, v6, v7, v47, v52)
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v53, v54, v55, v56, v57, v58, v59)
                                        }
                                    };
                                    (v60, v61, v62, v63, v64, v65, v66)
                                }
                            };
                            (v67, v68, v69, v70, v71, v72, v73)
                        }
                    };
                    (v74, v75, v76, v77, v78, v79, v80)
                }
            };
            (v2, v3, v4, v5, v6, v7, v8, v9) = (v11, v81, v82, v83, v84, v85, v86, v87);
            continue;
        }
    }
}
fn memories_checked_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    loop {
        let v12: bool = &*v4 == "";
        if v12 {
            return (v5, v6, v7, v8, v9, v10, v11);
        } else {
            let v13: Rc<str> = std::rc::Rc::<str>::from((v4).split(char::from(10u8)).next().unwrap_or(""));
            let mut v14: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v3).join(&*v13).to_string_lossy().as_ref());
            let v15: bool = std::path::Path::new(&*v14).is_file();
            let v69: Rc<str> = if v15 {
                let (mut v16, mut v17): (bool, Rc<str>) = match std::fs::read(&*v14) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                if v16 {
                    let mut v18: Rc<str> = string_concat(&["rig state read failed: ", &*v14]);
                    std::panic::panic_any::<std::string::String>(format!("{}", v18))
                } else {
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = Rc::<str>::from(v14.replace(&*v20, &*v21));
                    let mut v23: Rc<str> = loop_3(v22);
                    let mut v24: Option<usize> = v23.rfind(&*v21);
                    let mut v25: Option<usize> = method4(v24);
                    let mut v26: Rc<dyn Fn((usize)) -> i32> = closure1();
                    let mut v27: Option<i32> = v25.map(|x| v26(x));
                    let v29: i32 = v27.unwrap_or(-1i32);
                    let v30: bool = v29 < 0i32;
                    let v34: bool = if v30 {
                        true
                    } else {
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                        let v32: bool = v23.ends_with(&*v31);
                        let v33: bool = v32 == false;
                        v33
                    };
                    let v55: US0 = if v34 {
                        US0::US0_OtherFile
                    } else {
                        let v36: usize = ((0i32) as usize);
                        let v37: usize = ((v29) as usize);
                        let v38: Rc<str> = Rc::<str>::from(&(v23)[v36..v37]);
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                        let v40: bool = v38.ends_with(&*v39);
                        if v40 {
                            let v41: Rc<str> = loop_5(v38);
                            US0::US0_StateFile(v41)
                        } else {
                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                            let v44: bool = v38.ends_with(&*v43);
                            if v44 {
                                let v45: Rc<str> = loop_6(v38);
                                US0::US0_HistoryFile(v45)
                            } else {
                                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                let v48: bool = v38.ends_with(&*v47);
                                if v48 {
                                    let v49: Rc<str> = loop_7(v38);
                                    US0::US0_ArchiveFile(v49)
                                } else {
                                    US0::US0_OtherFile
                                }
                            }
                        }
                    };
                    match v55 {
                        US0::US0_OtherFile => {
                            v17
                        }
                        _ => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let v57: Rc<UH0> = split_list_8(v56.clone(), v17);
                            let v58: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v59: Rc<UH0> = method11(v57, v58);
                            let v60: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v61: i32 = 0i32;
                            let v62: i32 = method12(v60.clone(), v59, v61);
                            let v63: Rc<Vec<Rc<str>>> = Rc::new(v60.borrow().clone());
                            let v64: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v63).as_ref().clone()));
                            let v65: Rc<str> = Rc::<str>::from((*v64.borrow()).join(&*v56));
                            v65
                        }
                    }
                }
            } else {
                let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v68
            };
            let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("name"); } LIT.with(|lit| lit.clone()) };
            let v71: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v69; let front = text.strip_prefix("---").and_then(|rest| rest.split_once("
---").map(|(front, _)| front)).unwrap_or(""); front.lines().find_map(|line| line.trim().strip_prefix(&*v70).and_then(|rest| rest.strip_prefix(char::from(58u8))).map(|value| value.trim().trim_matches(char::from(34u8)).to_owned())).unwrap_or_default() }.as_str());
            let v72: bool = &*v71 == "";
            let v74: Rc<str> = if v72 {
                let v73: Rc<str> = std::rc::Rc::<str>::from((v13).trim_end_matches(".md"));
                v73
            } else {
                v71
            };
            let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type"); } LIT.with(|lit| lit.clone()) };
            let v76: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v69; let front = text.strip_prefix("---").and_then(|rest| rest.split_once("
---").map(|(front, _)| front)).unwrap_or(""); front.lines().find_map(|line| line.trim().strip_prefix(&*v75).and_then(|rest| rest.strip_prefix(char::from(58u8))).map(|value| value.trim().trim_matches(char::from(34u8)).to_owned())).unwrap_or_default() }.as_str());
            let v77: bool = &*v76 == "user";
            let v92: US3 = if v77 {
                US3::US3_MemoryUser
            } else {
                let v79: bool = &*v76 == "feedback";
                if v79 {
                    US3::US3_MemoryFeedback
                } else {
                    let v81: bool = &*v76 == "project";
                    if v81 {
                        US3::US3_MemoryProject
                    } else {
                        let v83: bool = &*v76 == "reference";
                        if v83 {
                            US3::US3_MemoryReference
                        } else {
                            US3::US3_MemoryUntyped
                        }
                    }
                }
            };
            let v93: Rc<str> = std::rc::Rc::<str>::from({ let known = (v1).split(char::from(124u8)).collect::<std::collections::HashSet<_>>(); let mut ids = (v69).split(|c: char| !(c.is_ascii_alphanumeric() || c == char::from(45u8) || c == char::from(95u8))).map(|token| token.trim_matches(char::from(45u8))).filter(|token| token.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == char::from(45u8))).filter(|token| token.contains(char::from(45u8)) && token.split(char::from(45u8)).all(|part| !part.is_empty()) && token.starts_with(|c: char| c.is_ascii_uppercase()) && known.contains(token.split(char::from(45u8)).next().unwrap_or(""))).map(str::to_owned).collect::<Vec<_>>(); ids.sort(); ids.dedup(); ids.join("
") }.as_str());
            let v94: i32 = v5.wrapping_add(1i32);
            let (mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101): (i32, i32, i32, i32, i32, i32, Rc<str>) = ids_checked_16(v0.clone(), v92, v74.clone(), v93, v94, v6, v7, v8, v9, v10, v11);
            let v102: Rc<str> = std::rc::Rc::<str>::from((v4).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let v103: Rc<str> = std::rc::Rc::<str>::from({ let mut links = (v69).split("[[").skip(1).filter_map(|rest| rest.split_once("]]").map(|(target, _)| target.trim().to_owned())).filter(|target| !target.is_empty()).collect::<Vec<_>>(); links.sort(); links.dedup(); links.join("
") }.as_str());
            let (mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110): (i32, i32, i32, i32, i32, i32, Rc<str>) = links_checked_17(v2.clone(), v74, v103, v95, v96, v97, v98, v99, v100, v101);
            (v4, v5, v6, v7, v8, v9, v10, v11) = (v102, v104, v105, v106, v107, v108, v109, v110);
            continue;
        }
    }
}
fn cites_in_22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    loop {
        let v11: i32 = (v2).get(v3 as usize..).and_then(|rest| rest.find("memory ")).map(|at| at as i32 + v3).unwrap_or(-1);
        let v12: bool = v11 < 0i32;
        if v12 {
            return (v4, v5, v6, v7, v8, v9, v10);
        } else {
            let v13: i32 = v11.wrapping_add(7i32);
            let mut v14: Rc<str> = std::rc::Rc::<str>::from((v2).get(v13 as usize..).unwrap_or("").chars().take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == char::from(45u8)).collect::<String>().trim_end_matches(char::from(45u8)));
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v15).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let mut v17: i32 = (v16.clone().borrow().len() as i32);
            let mut v18: i32 = 0i32;
            let v19: bool = loop_18(v14.clone(), &v16, v17, v18);
            let mut v26: US5 = if v19 {
                US5::US5_NoFinding
            } else {
                let mut v21: i32 = (v14).matches(char::from(45u8)).count() as i32;
                let v22: bool = v21 >= 2i32;
                if v22 {
                    US5::US5_UnknownMemory(v1.clone(), v14)
                } else {
                    US5::US5_NoFinding
                }
            };
            let (mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v26 {
                US5::US5_NoFinding => {
                    (v4, v5, v6, v7, v8, v9, v10)
                }
                _ => {
                    let (mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v26 {
                        US5::US5_DanglingId(v27, v28) => {
                            let mut v27: Rc<str> = v27.clone();
                            let v28: Rc<str> = v28.clone();
                            let v29: i32 = v6.wrapping_add(1i32);
                            let v34: Rc<str> = string_concat(&[&*v10, "dangling  ", &*v27, "  ", &*v28, "  no backlog item with this id\n"]);
                            (v4, v5, v29, v7, v8, v9, v34)
                        }
                        _ => {
                            let (mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v26 {
                                US5::US5_DoneCited(v35, v36) => {
                                    let v35: Rc<str> = v35.clone();
                                    let v36: Rc<str> = v36.clone();
                                    let v37: i32 = v7.wrapping_add(1i32);
                                    let v42: Rc<str> = string_concat(&[&*v10, "done  ", &*v35, "  ", &*v36, "  the item is Done; check that the rule still holds\n"]);
                                    (v4, v5, v6, v37, v8, v9, v42)
                                }
                                _ => {
                                    let (mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v26 {
                                        US5::US5_UnwrittenLink(v43, v44) => {
                                            let v43: Rc<str> = v43.clone();
                                            let v44: Rc<str> = v44.clone();
                                            let v45: i32 = v8.wrapping_add(1i32);
                                            let v50: Rc<str> = string_concat(&[&*v10, "unwritten-link  ", &*v43, "  [[", &*v44, "]]  no memory with this name yet\n"]);
                                            (v4, v5, v6, v7, v45, v9, v50)
                                        }
                                        _ => {
                                            let (mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65): (i32, i32, i32, i32, i32, i32, Rc<str>) = match v26 {
                                                US5::US5_UnknownMemory(v51, v52) => {
                                                    let v51: Rc<str> = v51;
                                                    let v52: Rc<str> = v52;
                                                    let v53: i32 = v9.wrapping_add(1i32);
                                                    let v58: Rc<str> = string_concat(&[&*v10, "unknown-memory  ", &*v51, "  ", &*v52, "  a backlog note cites this memory, which does not exist\n"]);
                                                    (v4, v5, v6, v7, v8, v53, v58)
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v59, v60, v61, v62, v63, v64, v65)
                                        }
                                    };
                                    (v66, v67, v68, v69, v70, v71, v72)
                                }
                            };
                            (v73, v74, v75, v76, v77, v78, v79)
                        }
                    };
                    (v80, v81, v82, v83, v84, v85, v86)
                }
            };
            (v3, v4, v5, v6, v7, v8, v9, v10) = (v13, v87, v88, v89, v90, v91, v92, v93);
            continue;
        }
    }
}
fn notes_checked_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    loop {
        let v10: i32 = (v1).get(v2 as usize..).and_then(|rest| rest.find("= Note (")).map(|at| at as i32 + v2).unwrap_or(-1);
        let v11: bool = v10 < 0i32;
        if v11 {
            return (v3, v4, v5, v6, v7, v8, v9);
        } else {
            let v12: i32 = (v1).get(v10 as usize..).and_then(|rest| rest.find(char::from(10u8))).map(|at| at as i32 + v10).unwrap_or((v1).len() as i32);
            let v13: Rc<str> = std::rc::Rc::<str>::from((v1).get(v10 as usize..v12 as usize).unwrap_or(""));
            let mut v14: Rc<str> = std::rc::Rc::<str>::from((v13).split(char::from(34u8)).nth(1).unwrap_or(""));
            let v15: Rc<str> = std::rc::Rc::<str>::from((v1).get(v10 as usize..v12 as usize).unwrap_or(""));
            let v16: i32 = 0i32;
            let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23): (i32, i32, i32, i32, i32, i32, Rc<str>) = cites_in_22(v0.clone(), v14, v15, v16, v3, v4, v5, v6, v7, v8, v9);
            (v2, v3, v4, v5, v6, v7, v8, v9) = (v12, v17, v18, v19, v20, v21, v22, v23);
            continue;
        }
    }
}
fn method24(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_23(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method24(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn links_checked_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool) -> i32 {
    let mut v3: bool = &*v1 == "";
    let mut v6: bool = if v3 {
        true
    } else {
        let v4: bool = directory_exists_2(v1.clone());
        let v5: bool = v4 == false;
        v5
    };
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: agile links rejected: --memory <dir> must name a memory directory"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v7);
        2i32
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v8).to_string_lossy().as_ref());
        let v10: Rc<str> = agile_note_listing::rig_agile_state_segments(&*v9);
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile.spi"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v9).join(&*v11).to_string_lossy().as_ref());
        let v13: bool = std::path::Path::new(&*v12).is_file();
        let v67: Rc<str> = if v13 {
            let (mut v14, mut v15): (bool, Rc<str>) = match std::fs::read(&*v12) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v14 {
                let v16: Rc<str> = string_concat(&["rig state read failed: ", &*v12]);
                std::panic::panic_any::<std::string::String>(format!("{}", v16))
            } else {
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = Rc::<str>::from(v12.replace(&*v18, &*v19));
                let mut v21: Rc<str> = loop_3(v20);
                let v22: Option<usize> = v21.rfind(&*v19);
                let mut v23: Option<usize> = method4(v22);
                let mut v24: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v25: Option<i32> = v23.map(|x| v24(x));
                let mut v27: i32 = v25.unwrap_or(-1i32);
                let v28: bool = v27 < 0i32;
                let v32: bool = if v28 {
                    true
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v30: bool = v21.ends_with(&*v29);
                    let v31: bool = v30 == false;
                    v31
                };
                let v53: US0 = if v32 {
                    US0::US0_OtherFile
                } else {
                    let v34: usize = ((0i32) as usize);
                    let v35: usize = ((v27) as usize);
                    let v36: Rc<str> = Rc::<str>::from(&(v21)[v34..v35]);
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v38: bool = v36.ends_with(&*v37);
                    if v38 {
                        let v39: Rc<str> = loop_5(v36);
                        US0::US0_StateFile(v39)
                    } else {
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v42: bool = v36.ends_with(&*v41);
                        if v42 {
                            let v43: Rc<str> = loop_6(v36);
                            US0::US0_HistoryFile(v43)
                        } else {
                            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v46: bool = v36.ends_with(&*v45);
                            if v46 {
                                let v47: Rc<str> = loop_7(v36);
                                US0::US0_ArchiveFile(v47)
                            } else {
                                US0::US0_OtherFile
                            }
                        }
                    }
                };
                match v53 {
                    US0::US0_OtherFile => {
                        v15
                    }
                    _ => {
                        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v55: Rc<UH0> = split_list_8(v54.clone(), v15);
                        let v56: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v57: Rc<UH0> = method11(v55, v56);
                        let v58: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v59: i32 = 0i32;
                        let v60: i32 = method12(v58.clone(), v57, v59);
                        let v61: Rc<Vec<Rc<str>>> = Rc::new(v58.borrow().clone());
                        let v62: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v61).as_ref().clone()));
                        let v63: Rc<str> = Rc::<str>::from((*v62.borrow()).join(&*v54));
                        v63
                    }
                }
            }
        } else {
            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v66
        };
        let mut v68: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        v68.borrow_mut().push(v67);
        v68.borrow_mut().push(v10);
        let v69: Rc<Vec<Rc<str>>> = Rc::new(v68.borrow().clone());
        let v70: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v69).as_ref().clone()));
        let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v72: Rc<str> = Rc::<str>::from((*v70.borrow()).join(&*v71));
        let v73: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v72);
        let v74: Rc<str> = std::rc::Rc::<str>::from({ let mut names = std::fs::read_dir(&*v1).into_iter().flatten().flatten().map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.ends_with(".md") && name != "MEMORY.md").collect::<Vec<_>>(); names.sort(); names.join("
") }.as_str());
        let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v76: Rc<str> = names_from_13(v1.clone(), v74.clone(), v75);
        let v77: i32 = 0i32;
        let v78: Rc<str> = std::rc::Rc::<str>::from({ let mut prefixes = (v73).match_indices("= Task (").filter_map(|(at, marker)| (v73)[at + marker.len()..].strip_prefix(char::from(34u8)).and_then(|rest| rest.split(char::from(45u8)).next()).map(str::to_owned)).collect::<Vec<_>>(); prefixes.sort(); prefixes.dedup(); prefixes.join("|") }.as_str());
        let (mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85): (i32, i32, i32, i32, i32, i32, Rc<str>) = links_start_14();
        let (mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92): (i32, i32, i32, i32, i32, i32, Rc<str>) = memories_checked_15(v73.clone(), v78, v76.clone(), v1, v74, v79, v80, v81, v82, v83, v84, v85);
        let (mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99): (i32, i32, i32, i32, i32, i32, Rc<str>) = notes_checked_21(v76, v73, v77, v86, v87, v88, v89, v90, v91, v92);
        let v100: Rc<str> = format_real_23(v93);
        let v103: Rc<str> = format_real_23(v94);
        let v105: Rc<str> = format_real_23(v95);
        let v108: Rc<str> = format_real_23(v96);
        let v110: Rc<str> = format_real_23(v97);
        let v113: Rc<str> = format_real_23(v98);
        let v116: Rc<str> = string_concat(&["memories=", &*v100, " cited=", &*v103, " dangling=", &*v105, " done=", &*v108, " unwritten_links=", &*v110, " unknown_memories=", &*v113]);
        if v2 {
            let v117: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); (v99).lines().filter_map(|line| { let mut parts = line.splitn(4, "  "); Some(format!("{{{q}kind{q}:{q}{}{q},{q}memory{q}:{q}{}{q},{q}target{q}:{q}{}{q}}}", parts.next()?, parts.next()?, parts.next()?.trim_start_matches("[[").trim_end_matches("]]"))) }).collect::<Vec<_>>().join(",") }.as_str());
            let v120: Rc<str> = format_real_23(v95);
            let v122: Rc<str> = string_concat(&["{\"findings\":[", &*v117, "],\"dangling\":", &*v120, "}"]);
            println!("{}", v122);
            ()
        } else {
            let v123: bool = &*v99 != "";
            if v123 {
                let v124: i32 = (v99.clone().len() as i32);
                let v125: i32 = method20(v99.clone(), v124);
                let v126: Rc<str> = string_slice(&v99, 0i32 as i64, v125 as i64);
                println!("{}", v126);
                ()
            };
            let v127: Rc<str> = string_concat(&["rig agile links ok ", &*v116]);
            println!("{}", v127);
            ()
        };
        let v128: bool = v95 == 0i32;
        if v128 {
            0i32
        } else {
            1i32
        }
    }
}
fn command_line_argument_count_26() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_27(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_25(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_26();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US6 = if v4 {
        let v5: Rc<str> = command_line_argument_27(v0);
        US6::US6_Some(v5)
    } else {
        US6::US6_None
    };
    match v8 {
        US6::US6_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US6::US6_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_25(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn links_flags_read_28(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: bool) -> (Rc<str>, bool) {
    loop {
        match &*v0 {
            UH0::UH0_Nil => {
                return (v1, v2);
            }
            _ => {
                match Rc::unwrap_or_clone(v0) {
                    UH0::UH0_Cons(v3, v4) => {
                        let mut v3: Rc<str> = v3;
                        let v4: Rc<UH0> = v4;
                        let v5: bool = &*v3 == "--json";
                        let mut v9: US7 = if v5 {
                            let mut v6: US8 = US8::US8_SwitchJson;
                            US7::US7_Some(v6)
                        } else {
                            US7::US7_None
                        };
                        let v10: bool = &*v3 == "--memory";
                        let mut v14: US9 = if v10 {
                            let v11: US10 = US10::US10_ValuedMemory;
                            US9::US9_Some(v11)
                        } else {
                            US9::US9_None
                        };
                        match v9 {
                            US7::US7_Some(v15) => {
                                let v15: US8 = v15;
                                let v16: bool = true;
                                (v0, v2) = (v4, v16);
                                continue;
                            }
                            _ => {
                                match v14 {
                                    US9::US9_Some(v19) => {
                                        let v19: US10 = v19;
                                        match &*v4 {
                                            UH0::UH0_Cons(v20, v21) => {
                                                let v20: Rc<str> = v20.clone();
                                                let mut v21: Rc<UH0> = v21.clone();
                                                (v0, v1) = (v21, v20);
                                                continue;
                                            }
                                            _ => {
                                                v0 = v4;
                                                continue;
                                            }
                                        }
                                    }
                                    _ => {
                                        v0 = v4;
                                        continue;
                                    }
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
fn links_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_25(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v9: bool = false;
                            let (mut v10, mut v11): (Rc<str>, bool) = links_flags_read_28(v7, v8, v9);
                            links_checked_1(v6, v10, v11)
                        }
                        _ => {
                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v14: bool = false;
                            links_checked_1(v13.clone(), v13, v14)
                        }
                    }
                }
                _ => {
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v18: bool = false;
                    links_checked_1(v17.clone(), v17, v18)
                }
            }
        }
        _ => {
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v22: bool = false;
            links_checked_1(v21.clone(), v21, v22)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        links_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_links_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
