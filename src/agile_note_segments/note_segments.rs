#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
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
#[derive(Clone)]
enum US0 {
    US0_StateFile(Rc<str>),
    US0_HistoryFile(Rc<str>),
    US0_ArchiveFile(Rc<str>),
    US0_OtherFile,
}
#[derive(Clone)]
enum US1 {
    US1_WordsRoot,
    US1_SymbolsRoot,
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
    US4_Replaced,
    US4_TemporaryWriteFailed,
    US4_RenameFailed,
    US4_RenameFailedTemporaryKept,
    US4_ReadbackMismatch,
    US4_ChangedSinceRead,
}
#[derive(Clone)]
enum US5 {
    US5_SpiprojProject,
    US5_TomlProject,
}
#[derive(Clone)]
enum US6 {
    US6_SplitProject(Rc<UH0>, Rc<UH0>, Rc<UH0>),
    US6_NoModules,
}
#[derive(Clone)]
enum US7 {
    US7_Some(i32),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_StateWritten,
    US8_StateChangedSinceRead,
    US8_StateWriteFailed(Rc<str>),
}
fn file_exists_2(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn loop_3(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
            let mut v3: i32 = (v0.clone().len() as i32);
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
            let mut v3: i32 = (v0.clone().len() as i32);
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
            let mut v3: i32 = (v0.clone().len() as i32);
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
                let mut v3: Rc<UH0> = v3;
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
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method9(v16, v17)
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method11(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US2 = if v6 {
                let mut v7: US3 = US3::US3_HeadType;
                US2::US2_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US3 = US3::US3_HeadType;
                    US2::US2_SymbolHead(v11)
                } else {
                    US2::US2_OtherLine
                }
            };
            let v27: US2 = match &v15 {
                US2::US2_OtherLine => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
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
                        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
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
fn method12(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
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
fn renamed_within_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = std::fs::rename(&*v1, &*v0).is_ok();
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
fn written_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: bool = file_exists_2(v0.clone());
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
        let mut v7: Rc<str> = loop_3(v6);
        let v8: Option<usize> = v7.rfind(&*v5);
        let mut v9: Option<usize> = method4(v8);
        let mut v10: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v11: Option<i32> = v9.map(|x| v10(x));
        let mut v13: i32 = v11.unwrap_or(-1i32);
        let mut v14: bool = v13 < 0i32;
        let v18: bool = if v14 {
            true
        } else {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let mut v16: bool = v7.ends_with(&*v15);
            let v17: bool = v16 == false;
            v17
        };
        let v39: US0 = if v18 {
            US0::US0_OtherFile
        } else {
            let v20: usize = ((0i32) as usize);
            let v21: usize = ((v13) as usize);
            let v22: Rc<str> = Rc::<str>::from(&(v7)[v20..v21]);
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v24: bool = v22.ends_with(&*v23);
            if v24 {
                let mut v25: Rc<str> = loop_5(v22);
                US0::US0_StateFile(v25)
            } else {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v28: bool = v22.ends_with(&*v27);
                if v28 {
                    let v29: Rc<str> = loop_6(v22);
                    US0::US0_HistoryFile(v29)
                } else {
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let mut v32: bool = v22.ends_with(&*v31);
                    if v32 {
                        let v33: Rc<str> = loop_7(v22);
                        US0::US0_ArchiveFile(v33)
                    } else {
                        US0::US0_OtherFile
                    }
                }
            }
        };
        let v62: US1 = match v39 {
            US0::US0_ArchiveFile(v52) => {
                let v52: Rc<str> = v52;
                let v53: Rc<str> = string_concat(&[&*v52, "/state/spiral.toml"]);
                let v54: bool = std::path::Path::new(&*v53).is_file();
                if v54 {
                    US1::US1_SymbolsRoot
                } else {
                    US1::US1_WordsRoot
                }
            }
            US0::US0_HistoryFile(v46) => {
                let v46: Rc<str> = v46;
                let mut v47: Rc<str> = string_concat(&[&*v46, "/state/spiral.toml"]);
                let v48: bool = std::path::Path::new(&*v47).is_file();
                if v48 {
                    US1::US1_SymbolsRoot
                } else {
                    US1::US1_WordsRoot
                }
            }
            US0::US0_OtherFile => {
                US1::US1_WordsRoot
            }
            US0::US0_StateFile(v40) => {
                let v40: Rc<str> = v40;
                let v41: Rc<str> = string_concat(&[&*v40, "/state/spiral.toml"]);
                let v42: bool = std::path::Path::new(&*v41).is_file();
                if v42 {
                    US1::US1_SymbolsRoot
                } else {
                    US1::US1_WordsRoot
                }
            }
        };
        let v74: Rc<str> = match v62 {
            US1::US1_SymbolsRoot => {
                let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v64: Rc<UH0> = split_list_8(v63.clone(), v1);
                let v65: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v66: Rc<UH0> = method11(v64, v65);
                let v67: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v68: i32 = 0i32;
                let v69: i32 = method12(v67.clone(), v66, v68);
                let v70: Rc<Vec<Rc<str>>> = Rc::new(v67.borrow().clone());
                let v71: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v70).as_ref().clone()));
                let v72: Rc<str> = Rc::<str>::from((*v71.borrow()).join(&*v63));
                v72
            }
            US1::US1_WordsRoot => {
                v1
            }
        };
        let v75: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v74);
        let v228: US4 = if v75 {
            let v76: Rc<str> = string_concat(&[&*v0, ".pending"]);
            let v77: Rc<str> = Rc::<str>::from(v76.replace(&*v4, &*v5));
            let mut v78: Rc<str> = loop_3(v77);
            let v79: Option<usize> = v78.rfind(&*v5);
            let mut v80: Option<usize> = method4(v79);
            let mut v81: Option<i32> = v80.map(|x| v10(x));
            let mut v83: i32 = v81.unwrap_or(-1i32);
            let v84: bool = v83 < 0i32;
            let v88: bool = if v84 {
                true
            } else {
                let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v86: bool = v78.ends_with(&*v85);
                let v87: bool = v86 == false;
                v87
            };
            let v109: US0 = if v88 {
                US0::US0_OtherFile
            } else {
                let v90: usize = ((0i32) as usize);
                let v91: usize = ((v83) as usize);
                let v92: Rc<str> = Rc::<str>::from(&(v78)[v90..v91]);
                let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v94: bool = v92.ends_with(&*v93);
                if v94 {
                    let v95: Rc<str> = loop_5(v92);
                    US0::US0_StateFile(v95)
                } else {
                    let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v98: bool = v92.ends_with(&*v97);
                    if v98 {
                        let v99: Rc<str> = loop_6(v92);
                        US0::US0_HistoryFile(v99)
                    } else {
                        let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v102: bool = v92.ends_with(&*v101);
                        if v102 {
                            let v103: Rc<str> = loop_7(v92);
                            US0::US0_ArchiveFile(v103)
                        } else {
                            US0::US0_OtherFile
                        }
                    }
                }
            };
            let v132: US1 = match v109 {
                US0::US0_ArchiveFile(v122) => {
                    let v122: Rc<str> = v122;
                    let v123: Rc<str> = string_concat(&[&*v122, "/state/spiral.toml"]);
                    let v124: bool = std::path::Path::new(&*v123).is_file();
                    if v124 {
                        US1::US1_SymbolsRoot
                    } else {
                        US1::US1_WordsRoot
                    }
                }
                US0::US0_HistoryFile(v116) => {
                    let v116: Rc<str> = v116;
                    let v117: Rc<str> = string_concat(&[&*v116, "/state/spiral.toml"]);
                    let v118: bool = std::path::Path::new(&*v117).is_file();
                    if v118 {
                        US1::US1_SymbolsRoot
                    } else {
                        US1::US1_WordsRoot
                    }
                }
                US0::US0_OtherFile => {
                    US1::US1_WordsRoot
                }
                US0::US0_StateFile(v110) => {
                    let v110: Rc<str> = v110;
                    let v111: Rc<str> = string_concat(&[&*v110, "/state/spiral.toml"]);
                    let v112: bool = std::path::Path::new(&*v111).is_file();
                    if v112 {
                        US1::US1_SymbolsRoot
                    } else {
                        US1::US1_WordsRoot
                    }
                }
            };
            let v144: Rc<str> = match v132 {
                US1::US1_SymbolsRoot => {
                    let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v134: Rc<UH0> = split_list_8(v133.clone(), v2.clone());
                    let v135: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v136: Rc<UH0> = method11(v134, v135);
                    let v137: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v138: i32 = 0i32;
                    let v139: i32 = method12(v137.clone(), v136, v138);
                    let v140: Rc<Vec<Rc<str>>> = Rc::new(v137.borrow().clone());
                    let v141: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v140).as_ref().clone()));
                    let v142: Rc<str> = Rc::<str>::from((*v141.borrow()).join(&*v133));
                    v142
                }
                US1::US1_WordsRoot => {
                    v2.clone()
                }
            };
            let v145: bool = std::fs::write(&*v76, &*v144).is_ok();
            if v145 {
                let v146: i32 = 40i32;
                let v147: bool = renamed_within_13(v0.clone(), v76.clone(), v146);
                if v147 {
                    let v148: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
                    let mut v149: Rc<str> = loop_3(v148);
                    let v150: Option<usize> = v149.rfind(&*v5);
                    let mut v151: Option<usize> = method4(v150);
                    let mut v152: Option<i32> = v151.map(|x| v10(x));
                    let mut v154: i32 = v152.unwrap_or(-1i32);
                    let v155: bool = v154 < 0i32;
                    let v159: bool = if v155 {
                        true
                    } else {
                        let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                        let v157: bool = v149.ends_with(&*v156);
                        let v158: bool = v157 == false;
                        v158
                    };
                    let v180: US0 = if v159 {
                        US0::US0_OtherFile
                    } else {
                        let v161: usize = ((0i32) as usize);
                        let v162: usize = ((v154) as usize);
                        let v163: Rc<str> = Rc::<str>::from(&(v149)[v161..v162]);
                        let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                        let v165: bool = v163.ends_with(&*v164);
                        if v165 {
                            let v166: Rc<str> = loop_5(v163);
                            US0::US0_StateFile(v166)
                        } else {
                            let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                            let v169: bool = v163.ends_with(&*v168);
                            if v169 {
                                let v170: Rc<str> = loop_6(v163);
                                US0::US0_HistoryFile(v170)
                            } else {
                                let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                let v173: bool = v163.ends_with(&*v172);
                                if v173 {
                                    let v174: Rc<str> = loop_7(v163);
                                    US0::US0_ArchiveFile(v174)
                                } else {
                                    US0::US0_OtherFile
                                }
                            }
                        }
                    };
                    let v203: US1 = match v180 {
                        US0::US0_ArchiveFile(v193) => {
                            let v193: Rc<str> = v193;
                            let v194: Rc<str> = string_concat(&[&*v193, "/state/spiral.toml"]);
                            let v195: bool = std::path::Path::new(&*v194).is_file();
                            if v195 {
                                US1::US1_SymbolsRoot
                            } else {
                                US1::US1_WordsRoot
                            }
                        }
                        US0::US0_HistoryFile(v187) => {
                            let v187: Rc<str> = v187;
                            let v188: Rc<str> = string_concat(&[&*v187, "/state/spiral.toml"]);
                            let v189: bool = std::path::Path::new(&*v188).is_file();
                            if v189 {
                                US1::US1_SymbolsRoot
                            } else {
                                US1::US1_WordsRoot
                            }
                        }
                        US0::US0_OtherFile => {
                            US1::US1_WordsRoot
                        }
                        US0::US0_StateFile(v181) => {
                            let v181: Rc<str> = v181;
                            let v182: Rc<str> = string_concat(&[&*v181, "/state/spiral.toml"]);
                            let v183: bool = std::path::Path::new(&*v182).is_file();
                            if v183 {
                                US1::US1_SymbolsRoot
                            } else {
                                US1::US1_WordsRoot
                            }
                        }
                    };
                    let v215: Rc<str> = match v203 {
                        US1::US1_SymbolsRoot => {
                            let v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let v205: Rc<UH0> = split_list_8(v204.clone(), v2);
                            let v206: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v207: Rc<UH0> = method11(v205, v206);
                            let v208: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v209: i32 = 0i32;
                            let v210: i32 = method12(v208.clone(), v207, v209);
                            let v211: Rc<Vec<Rc<str>>> = Rc::new(v208.borrow().clone());
                            let v212: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v211).as_ref().clone()));
                            let v213: Rc<str> = Rc::<str>::from((*v212.borrow()).join(&*v204));
                            v213
                        }
                        US1::US1_WordsRoot => {
                            v2
                        }
                    };
                    let v216: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v215);
                    if v216 {
                        US4::US4_Replaced
                    } else {
                        US4::US4_ReadbackMismatch
                    }
                } else {
                    let v220: bool = std::fs::remove_file(&*v76).is_ok() || !std::path::Path::new(&*v76).exists();
                    if v220 {
                        US4::US4_RenameFailed
                    } else {
                        US4::US4_RenameFailedTemporaryKept
                    }
                }
            } else {
                US4::US4_TemporaryWriteFailed
            }
        } else {
            US4::US4_ChangedSinceRead
        };
        match v228 {
            US4::US4_ChangedSinceRead => {
                let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                v234
            }
            US4::US4_ReadbackMismatch => {
                let v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                v233
            }
            US4::US4_RenameFailed => {
                let v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                v231
            }
            US4::US4_RenameFailedTemporaryKept => {
                let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                v232
            }
            US4::US4_Replaced => {
                let v229: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v229
            }
            US4::US4_TemporaryWriteFailed => {
                let v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                v230
            }
        }
    } else {
        let v241: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v243: Rc<str> = Rc::<str>::from(v0.replace(&*v241, &*v242));
        let mut v244: Rc<str> = loop_3(v243);
        let v245: Option<usize> = v244.rfind(&*v242);
        let v246: Option<usize> = method4(v245);
        let mut v247: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v248: Option<i32> = v246.map(|x| v247(x));
        let v250: i32 = v248.unwrap_or(-1i32);
        let v251: bool = v250 < 0i32;
        let v255: bool = if v251 {
            true
        } else {
            let v252: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v253: bool = v244.ends_with(&*v252);
            let v254: bool = v253 == false;
            v254
        };
        let v276: US0 = if v255 {
            US0::US0_OtherFile
        } else {
            let v257: usize = ((0i32) as usize);
            let v258: usize = ((v250) as usize);
            let v259: Rc<str> = Rc::<str>::from(&(v244)[v257..v258]);
            let v260: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v261: bool = v259.ends_with(&*v260);
            if v261 {
                let v262: Rc<str> = loop_5(v259);
                US0::US0_StateFile(v262)
            } else {
                let v264: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v265: bool = v259.ends_with(&*v264);
                if v265 {
                    let v266: Rc<str> = loop_6(v259);
                    US0::US0_HistoryFile(v266)
                } else {
                    let v268: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v269: bool = v259.ends_with(&*v268);
                    if v269 {
                        let v270: Rc<str> = loop_7(v259);
                        US0::US0_ArchiveFile(v270)
                    } else {
                        US0::US0_OtherFile
                    }
                }
            }
        };
        let v299: US1 = match v276 {
            US0::US0_ArchiveFile(v289) => {
                let v289: Rc<str> = v289;
                let v290: Rc<str> = string_concat(&[&*v289, "/state/spiral.toml"]);
                let v291: bool = std::path::Path::new(&*v290).is_file();
                if v291 {
                    US1::US1_SymbolsRoot
                } else {
                    US1::US1_WordsRoot
                }
            }
            US0::US0_HistoryFile(v283) => {
                let v283: Rc<str> = v283;
                let v284: Rc<str> = string_concat(&[&*v283, "/state/spiral.toml"]);
                let v285: bool = std::path::Path::new(&*v284).is_file();
                if v285 {
                    US1::US1_SymbolsRoot
                } else {
                    US1::US1_WordsRoot
                }
            }
            US0::US0_OtherFile => {
                US1::US1_WordsRoot
            }
            US0::US0_StateFile(v277) => {
                let v277: Rc<str> = v277;
                let v278: Rc<str> = string_concat(&[&*v277, "/state/spiral.toml"]);
                let v279: bool = std::path::Path::new(&*v278).is_file();
                if v279 {
                    US1::US1_SymbolsRoot
                } else {
                    US1::US1_WordsRoot
                }
            }
        };
        let v311: Rc<str> = match v299 {
            US1::US1_SymbolsRoot => {
                let v300: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v301: Rc<UH0> = split_list_8(v300.clone(), v2);
                let v302: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v303: Rc<UH0> = method11(v301, v302);
                let v304: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v305: i32 = 0i32;
                let v306: i32 = method12(v304.clone(), v303, v305);
                let v307: Rc<Vec<Rc<str>>> = Rc::new(v304.borrow().clone());
                let v308: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v307).as_ref().clone()));
                let v309: Rc<str> = Rc::<str>::from((*v308.borrow()).join(&*v300));
                v309
            }
            US1::US1_WordsRoot => {
                v2
            }
        };
        let v312: bool = std::fs::write(&*v0, &*v311).is_ok();
        if v312 {
            ()
        } else {
            let v313: Rc<str> = string_concat(&["rig state write failed: ", &*v0]);
            std::panic::panic_any::<std::string::String>(format!("{}", v313))
        };
        let v314: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v314
    }
}
fn segment_name_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..4i32 as usize).unwrap_or(""));
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.get(5i32 as usize..7i32 as usize).unwrap_or(""));
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes_"); } LIT.with(|lit| lit.clone()) };
    v3.borrow_mut().push(v4);
    v3.borrow_mut().push(v1);
    v3.borrow_mut().push(v2);
    let v5: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from((*v6.borrow()).join(&*v7));
    v8
}
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method16(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US2 = if v6 {
                let mut v7: US3 = US3::US3_HeadType;
                US2::US2_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US3 = US3::US3_HeadType;
                    US2::US2_SymbolHead(v11)
                } else {
                    US2::US2_OtherLine
                }
            };
            let v27: US2 = match &v15 {
                US2::US2_OtherLine => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
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
                        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
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
fn text_or_empty_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = std::path::Path::new(&*v0).is_file();
    if v1 {
        let (mut v2, mut v3): (bool, Rc<str>) = match std::fs::read(&*v0) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v2 {
            let mut v4: Rc<str> = string_concat(&["rig state read failed: ", &*v0]);
            std::panic::panic_any::<std::string::String>(format!("{}", v4))
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
            let mut v9: Rc<str> = loop_3(v8);
            let mut v10: Option<usize> = v9.rfind(&*v7);
            let mut v11: Option<usize> = method4(v10);
            let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v13: Option<i32> = v11.map(|x| v12(x));
            let v15: i32 = v13.unwrap_or(-1i32);
            let mut v16: bool = v15 < 0i32;
            let v20: bool = if v16 {
                true
            } else {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v18: bool = v9.ends_with(&*v17);
                let mut v19: bool = v18 == false;
                v19
            };
            let v41: US0 = if v20 {
                US0::US0_OtherFile
            } else {
                let v22: usize = ((0i32) as usize);
                let v23: usize = ((v15) as usize);
                let v24: Rc<str> = Rc::<str>::from(&(v9)[v22..v23]);
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v26: bool = v24.ends_with(&*v25);
                if v26 {
                    let v27: Rc<str> = loop_5(v24);
                    US0::US0_StateFile(v27)
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v30: bool = v24.ends_with(&*v29);
                    if v30 {
                        let v31: Rc<str> = loop_6(v24);
                        US0::US0_HistoryFile(v31)
                    } else {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v34: bool = v24.ends_with(&*v33);
                        if v34 {
                            let v35: Rc<str> = loop_7(v24);
                            US0::US0_ArchiveFile(v35)
                        } else {
                            US0::US0_OtherFile
                        }
                    }
                }
            };
            match v41 {
                US0::US0_OtherFile => {
                    v3
                }
                _ => {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<UH0> = split_list_8(v42.clone(), v3);
                    let v44: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v45: Rc<UH0> = method16(v43, v44);
                    let v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let mut v47: i32 = 0i32;
                    let v48: i32 = method12(v46.clone(), v45, v47);
                    let v49: Rc<Vec<Rc<str>>> = Rc::new(v46.borrow().clone());
                    let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v49).as_ref().clone()));
                    let v51: Rc<str> = Rc::<str>::from((*v50.borrow()).join(&*v42));
                    v51
                }
            }
        }
    } else {
        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v54
    }
}
fn method19(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
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
fn method20(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
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
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method18(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let mut v6: i32 = 0i32;
            let mut v7: i32 = method19(v2.clone(), v5, v6);
            let v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2.clone(), v7 as i64, v8 as i64);
            let mut v10: i32 = (v9.clone().len() as i32);
            let mut v11: i32 = method20(v9.clone(), v10);
            let mut v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            let mut v13: bool = &*v12 != "";
            if v13 {
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
fn section_lines_22(mut v0: US5, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> US6 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v8, v9) => {
                let v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH0> = v9.clone();
                let v26: bool = match &v0 {
                    US5::US5_SpiprojProject => {
                        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let mut v11: bool = v8.starts_with(&*v10);
                        let mut v14: bool = if v11 {
                            true
                        } else {
                            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v13: bool = v8.starts_with(&*v12);
                            v13
                        };
                        let v15: bool = v14 == false;
                        v15
                    }
                    US5::US5_TomlProject => {
                        let mut v16: i32 = (v8.clone().len() as i32);
                        let v17: i32 = 0i32;
                        let v18: i32 = method19(v8.clone(), v16, v17);
                        let mut v19: i32 = v16.wrapping_sub(1i32);
                        let v20: Rc<str> = string_slice(&v8.clone(), v18 as i64, v19 as i64);
                        let v21: i32 = (v20.clone().len() as i32);
                        let v22: i32 = method20(v20.clone(), v21);
                        let v23: Rc<str> = string_slice(&v20, 0i32 as i64, v22 as i64);
                        let v24: bool = &*v23 == "'''";
                        v24
                    }
                };
                if v26 {
                    let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v28: Rc<UH0> = method9(v3, v27);
                    return US6::US6_SplitProject(v2, v28, v1);
                } else {
                    let v30: i32 = (v8.clone().len() as i32);
                    let v31: i32 = 0i32;
                    let mut v32: i32 = method19(v8.clone(), v30, v31);
                    let v33: i32 = v30.wrapping_sub(1i32);
                    let v34: Rc<str> = string_slice(&v8, v32 as i64, v33 as i64);
                    let v35: i32 = (v34.clone().len() as i32);
                    let mut v36: i32 = method20(v34.clone(), v35);
                    let v37: Rc<str> = string_slice(&v34, 0i32 as i64, v36 as i64);
                    let v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v37, v3));
                    (v1, v3) = (v9, v38);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH0> = method9(v3, v4);
                let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return US6::US6_SplitProject(v2, v5, v6);
            }
        }
    }
}
fn split_lines_21(mut v0: US5, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> US6 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: i32 = (v4.clone().len() as i32);
                let mut v7: i32 = 0i32;
                let v8: i32 = method19(v4.clone(), v6, v7);
                let mut v9: i32 = v6.wrapping_sub(1i32);
                let mut v10: Rc<str> = string_slice(&v4.clone(), v8 as i64, v9 as i64);
                let mut v11: i32 = (v10.clone().len() as i32);
                let mut v12: i32 = method20(v10.clone(), v11);
                let mut v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
                let v17: Rc<str> = match &v0 {
                    US5::US5_SpiprojProject => {
                        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                        v14
                    }
                    US5::US5_TomlProject => {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                        v15
                    }
                };
                let v18: bool = &*v13 == &*v17;
                if v18 {
                    let mut v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v19));
                    let v21: Rc<UH0> = method9(v2, v20);
                    let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    return section_lines_22(v0, v5, v21, v22);
                } else {
                    let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v2));
                    (v1, v2) = (v5, v24);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US6::US6_NoModules;
            }
        }
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US7> {
    Rc::new(move |mut v2: Rc<UH0>| -> US7 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US7::US7_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US7> = method23(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US7::US7_None
            }
        }
    })
}
fn method23(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US7> {
    closure2(v0, v1)
}
fn registered_17(mut v0: US5, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v1.replace(&*v4, &*v5));
    let mut v7: Rc<UH0> = split_list_8(v3, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method18(v7, v8);
    let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v11: US6 = split_lines_21(v0, v9, v10);
    let v17: Rc<UH0> = match v11 {
        US6::US6_NoModules => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US6::US6_SplitProject(v12, v13, v14) => {
            let mut v12: Rc<UH0> = v12;
            let mut v13: Rc<UH0> = v13;
            let mut v14: Rc<UH0> = v14;
            v13
        }
    };
    let v18: i32 = 0i32;
    let mut v19: Rc<dyn Fn(Rc<UH0>) -> US7> = method23(v2, v18);
    let v20: US7 = v19(v17);
    let v21: bool = match v20 {
        US7::US7_None => {
            true
        }
        _ => {
            false
        }
    };
    let v22: bool = v21 != true;
    v22
}
fn segment_after_agile_25(mut v0: bool, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v5, v6) => {
            let v5: Rc<str> = v5.clone();
            let mut v6: Rc<UH0> = v6.clone();
            let mut v7: bool = &*v5 == "agile";
            let mut v11: bool = if v7 {
                true
            } else {
                if v0 {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes_"); } LIT.with(|lit| lit.clone()) };
                    let mut v9: bool = v5.starts_with(&*v8);
                    v9
                } else {
                    false
                }
            };
            if v11 {
                let mut v12: bool = true;
                let mut v13: Rc<UH0> = segment_after_agile_25(v12, v6, v2);
                Rc::new(UH0::UH0_Cons(v5, v13))
            } else {
                if v0 {
                    Rc::new(UH0::UH0_Cons(v2, v1))
                } else {
                    let mut v16: bool = false;
                    let v17: Rc<UH0> = segment_after_agile_25(v16, v6, v2);
                    Rc::new(UH0::UH0_Cons(v5, v17))
                }
            }
        }
        UH0::UH0_Nil => {
            let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v2, v3))
        }
    }
}
fn method26(mut v0: US5, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method26(v0.clone(), v4, v2);
            let mut v13: Rc<str> = match v0 {
                US5::US5_SpiprojProject => {
                    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                    v6.borrow_mut().push(v7);
                    v6.borrow_mut().push(v3);
                    let v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
                    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
                    v11
                }
                US5::US5_TomlProject => {
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
fn method27(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method27(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn manifest_with_segment_24(mut v0: US5, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v1.replace(&*v4, &*v5));
    let mut v7: Rc<UH0> = split_list_8(v3.clone(), v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method18(v7, v8);
    let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v11: US6 = split_lines_21(v0.clone(), v9, v10);
    match v11 {
        US6::US6_NoModules => {
            v1
        }
        US6::US6_SplitProject(v12, v13, v14) => {
            let mut v12: Rc<UH0> = v12;
            let mut v13: Rc<UH0> = v13;
            let mut v14: Rc<UH0> = v14;
            let v15: i32 = 0i32;
            let mut v16: Rc<dyn Fn(Rc<UH0>) -> US7> = method23(v2.clone(), v15);
            let v17: US7 = v16(v13.clone());
            let v18: bool = match v17 {
                US7::US7_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let mut v19: bool = v18 != true;
            if v19 {
                v1
            } else {
                let v20: bool = false;
                let v21: Rc<UH0> = segment_after_agile_25(v20, v13, v2);
                let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v23: Rc<UH0> = method26(v0, v21, v22);
                let v24: Rc<UH0> = method27(v23, v14);
                let mut v25: Rc<UH0> = method27(v12, v24);
                let v26: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v27: i32 = 0i32;
                let v28: i32 = method12(v26.clone(), v25, v27);
                let v29: Rc<Vec<Rc<str>>> = Rc::new(v26.borrow().clone());
                let v30: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v29).as_ref().clone()));
                let v31: Rc<str> = Rc::<str>::from((*v30.borrow()).join(&*v3));
                let mut v32: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v32.borrow_mut().push(v31);
                v32.borrow_mut().push(v3);
                let v33: Rc<Vec<Rc<str>>> = Rc::new(v32.borrow().clone());
                let v34: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v33).as_ref().clone()));
                let v35: Rc<str> = Rc::<str>::from((*v34.borrow()).join(&*v5));
                v35
            }
        }
    }
}
fn state_path_28(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v2.borrow_mut().push(v0);
    v2.borrow_mut().push(v1);
    let mut v3: Rc<Vec<Rc<str>>> = Rc::new(v2.borrow().clone());
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v3).as_ref().clone()));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from((*v4.borrow()).join(&*v5));
    v6
}
fn note_appended_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = segment_name_14(v2);
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v4.borrow_mut().push(v0.clone());
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.toml"); } LIT.with(|lit| lit.clone()) };
    v4.borrow_mut().push(v5.clone());
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new(v4.borrow().clone());
    let mut v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from((*v7.borrow()).join(&*v8));
    let mut v10: bool = std::path::Path::new(&*v9).is_file();
    let mut v13: US5 = if v10 {
        US5::US5_TomlProject
    } else {
        US5::US5_SpiprojProject
    };
    let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v14.borrow_mut().push(v0.clone());
    v14.borrow_mut().push(v5.clone());
    let v15: Rc<Vec<Rc<str>>> = Rc::new(v14.borrow().clone());
    let mut v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
    let v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v8));
    let v18: bool = std::path::Path::new(&*v17).is_file();
    let v21: US5 = if v18 {
        US5::US5_TomlProject
    } else {
        US5::US5_SpiprojProject
    };
    let v24: Rc<str> = match v21 {
        US5::US5_SpiprojProject => {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
            v22
        }
        US5::US5_TomlProject => {
            v5
        }
    };
    let mut v25: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v25.borrow_mut().push(v0.clone());
    v25.borrow_mut().push(v24);
    let v26: Rc<Vec<Rc<str>>> = Rc::new(v25.borrow().clone());
    let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v26).as_ref().clone()));
    let v28: Rc<str> = Rc::<str>::from((*v27.borrow()).join(&*v8));
    let v29: Rc<str> = text_or_empty_15(v28.clone());
    let v30: bool = registered_17(v13.clone(), v29.clone(), v3.clone());
    let v34: Rc<str> = if v30 {
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v31
    } else {
        let mut v32: Rc<str> = manifest_with_segment_24(v13, v29.clone(), v3.clone());
        written_1(v28, v29, v32)
    };
    let v35: bool = &*v34 != "";
    if v35 {
        v34
    } else {
        let mut v36: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        v36.borrow_mut().push(v3);
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        v36.borrow_mut().push(v37);
        let v38: Rc<Vec<Rc<str>>> = Rc::new(v36.borrow().clone());
        let v39: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v38).as_ref().clone()));
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v41: Rc<str> = Rc::<str>::from((*v39.borrow()).join(&*v40));
        let v42: Rc<str> = state_path_28(v0, v41);
        let v43: Rc<str> = text_or_empty_15(v42.clone());
        let v44: bool = &*v43 == "";
        let v46: Rc<str> = if v44 {
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open agile\n"); } LIT.with(|lit| lit.clone()) };
            v45
        } else {
            v43.clone()
        };
        let mut v47: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        v47.borrow_mut().push(v46);
        v47.borrow_mut().push(v1);
        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        v47.borrow_mut().push(v48);
        let v49: Rc<Vec<Rc<str>>> = Rc::new(v47.borrow().clone());
        let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v49).as_ref().clone()));
        let v51: Rc<str> = Rc::<str>::from((*v50.borrow()).join(&*v40));
        written_1(v42, v43, v51)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> Rc<str> {
        note_appended_0(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn get_environment_variable_30(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn concat5_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn declared_from_34(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v6.starts_with(&*v7);
            if v8 {
                return v3;
            } else {
                let mut v9: i32 = v3.wrapping_add(1i32);
                v2 = v9;
                continue;
            }
        }
    }
}
fn last_declared_35(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: i32 = declared_from_34(v0.clone(), v1.clone(), v2);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            (v2, v3) = (v6, v4);
            continue;
        }
    }
}
fn notes_insertion_33(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = concat5_32(v2, v3.clone(), v1.clone(), v4.clone(), v5.clone());
    let mut v7: i32 = 0i32;
    let v8: i32 = declared_from_34(v0.clone(), v6, v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = concat5_32(v9, v3.clone(), v1.clone(), v4.clone(), v5.clone());
    let mut v11: i32 = 0i32;
    let mut v12: i32 = declared_from_34(v0.clone(), v10, v11);
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = concat5_32(v13, v3, v1, v4, v5);
    let v15: i32 = 0i32;
    let mut v16: i32 = -1i32;
    let v17: i32 = last_declared_35(v0.clone(), v14, v15, v16);
    let v18: bool = v12 <= v17;
    let mut v19: i32 = if v18 {
        v17
    } else {
        v12
    };
    let v20: bool = v8 <= v19;
    let v21: i32 = if v20 {
        v19
    } else {
        v8
    };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v23: i32 = v0.get(v21 as usize..).and_then(|rest| rest.find(&*v22)).map(|found| (found + v21 as usize) as i32).unwrap_or(-1);
    let v24: bool = v23 < 0i32;
    if v24 {
        let mut v25: i32 = (v0.len() as i32);
        v25
    } else {
        v23
    }
}
fn notes_appended_31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: i32 = notes_insertion_33(v0.clone(), v1);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = concat5_32(v4, v2, v5.clone(), v5.clone(), v5.clone());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v3 as usize).unwrap_or(""));
    let v8: i32 = (v0.clone().len() as i32);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v3 as usize..v8 as usize).unwrap_or(""));
    concat5_32(v7, v6, v9, v5.clone(), v5)
}
fn state_written_36(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US8 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_32(v0.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    std::env::var("RIG_AGILE_TEST_PAUSE_MS").or_else(|_| std::env::var("KIT_AGILE_TEST_PAUSE_MS")).ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
    let mut v9: Rc<str> = loop_3(v8);
    let mut v10: Option<usize> = v9.rfind(&*v7);
    let mut v11: Option<usize> = method4(v10);
    let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v13: Option<i32> = v11.map(|x| v12(x));
    let v15: i32 = v13.unwrap_or(-1i32);
    let mut v16: bool = v15 < 0i32;
    let v20: bool = if v16 {
        true
    } else {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v18: bool = v9.ends_with(&*v17);
        let mut v19: bool = v18 == false;
        v19
    };
    let v41: US0 = if v20 {
        US0::US0_OtherFile
    } else {
        let v22: usize = ((0i32) as usize);
        let v23: usize = ((v15) as usize);
        let v24: Rc<str> = Rc::<str>::from(&(v9)[v22..v23]);
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v26: bool = v24.ends_with(&*v25);
        if v26 {
            let v27: Rc<str> = loop_5(v24);
            US0::US0_StateFile(v27)
        } else {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
            let v30: bool = v24.ends_with(&*v29);
            if v30 {
                let v31: Rc<str> = loop_6(v24);
                US0::US0_HistoryFile(v31)
            } else {
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                let v34: bool = v24.ends_with(&*v33);
                if v34 {
                    let v35: Rc<str> = loop_7(v24);
                    US0::US0_ArchiveFile(v35)
                } else {
                    US0::US0_OtherFile
                }
            }
        }
    };
    let v64: US1 = match v41 {
        US0::US0_ArchiveFile(v54) => {
            let v54: Rc<str> = v54;
            let v55: Rc<str> = string_concat(&[&*v54, "/state/spiral.toml"]);
            let v56: bool = std::path::Path::new(&*v55).is_file();
            if v56 {
                US1::US1_SymbolsRoot
            } else {
                US1::US1_WordsRoot
            }
        }
        US0::US0_HistoryFile(v48) => {
            let v48: Rc<str> = v48;
            let v49: Rc<str> = string_concat(&[&*v48, "/state/spiral.toml"]);
            let v50: bool = std::path::Path::new(&*v49).is_file();
            if v50 {
                US1::US1_SymbolsRoot
            } else {
                US1::US1_WordsRoot
            }
        }
        US0::US0_OtherFile => {
            US1::US1_WordsRoot
        }
        US0::US0_StateFile(v42) => {
            let v42: Rc<str> = v42;
            let v43: Rc<str> = string_concat(&[&*v42, "/state/spiral.toml"]);
            let v44: bool = std::path::Path::new(&*v43).is_file();
            if v44 {
                US1::US1_SymbolsRoot
            } else {
                US1::US1_WordsRoot
            }
        }
    };
    let v76: Rc<str> = match v64 {
        US1::US1_SymbolsRoot => {
            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v66: Rc<UH0> = split_list_8(v65.clone(), v1);
            let v67: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v68: Rc<UH0> = method11(v66, v67);
            let v69: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v70: i32 = 0i32;
            let v71: i32 = method12(v69.clone(), v68, v70);
            let v72: Rc<Vec<Rc<str>>> = Rc::new(v69.borrow().clone());
            let v73: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v72).as_ref().clone()));
            let v74: Rc<str> = Rc::<str>::from((*v73.borrow()).join(&*v65));
            v74
        }
        US1::US1_WordsRoot => {
            v1
        }
    };
    let v77: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v76);
    if v77 {
        let mut v78: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let v79: Rc<str> = Rc::<str>::from(v78.replace(&*v6, &*v7));
        let mut v80: Rc<str> = loop_3(v79);
        let mut v81: Option<usize> = v80.rfind(&*v7);
        let v82: Option<usize> = method4(v81);
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
        let v111: US0 = if v90 {
            US0::US0_OtherFile
        } else {
            let v92: usize = ((0i32) as usize);
            let v93: usize = ((v85) as usize);
            let v94: Rc<str> = Rc::<str>::from(&(v80)[v92..v93]);
            let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v96: bool = v94.ends_with(&*v95);
            if v96 {
                let v97: Rc<str> = loop_5(v94);
                US0::US0_StateFile(v97)
            } else {
                let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v100: bool = v94.ends_with(&*v99);
                if v100 {
                    let v101: Rc<str> = loop_6(v94);
                    US0::US0_HistoryFile(v101)
                } else {
                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v104: bool = v94.ends_with(&*v103);
                    if v104 {
                        let v105: Rc<str> = loop_7(v94);
                        US0::US0_ArchiveFile(v105)
                    } else {
                        US0::US0_OtherFile
                    }
                }
            }
        };
        let v134: US1 = match v111 {
            US0::US0_ArchiveFile(v124) => {
                let v124: Rc<str> = v124;
                let v125: Rc<str> = string_concat(&[&*v124, "/state/spiral.toml"]);
                let v126: bool = std::path::Path::new(&*v125).is_file();
                if v126 {
                    US1::US1_SymbolsRoot
                } else {
                    US1::US1_WordsRoot
                }
            }
            US0::US0_HistoryFile(v118) => {
                let v118: Rc<str> = v118;
                let v119: Rc<str> = string_concat(&[&*v118, "/state/spiral.toml"]);
                let v120: bool = std::path::Path::new(&*v119).is_file();
                if v120 {
                    US1::US1_SymbolsRoot
                } else {
                    US1::US1_WordsRoot
                }
            }
            US0::US0_OtherFile => {
                US1::US1_WordsRoot
            }
            US0::US0_StateFile(v112) => {
                let v112: Rc<str> = v112;
                let v113: Rc<str> = string_concat(&[&*v112, "/state/spiral.toml"]);
                let v114: bool = std::path::Path::new(&*v113).is_file();
                if v114 {
                    US1::US1_SymbolsRoot
                } else {
                    US1::US1_WordsRoot
                }
            }
        };
        let v146: Rc<str> = match v134 {
            US1::US1_SymbolsRoot => {
                let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v136: Rc<UH0> = split_list_8(v135.clone(), v2.clone());
                let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v138: Rc<UH0> = method11(v136, v137);
                let v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v140: i32 = 0i32;
                let v141: i32 = method12(v139.clone(), v138, v140);
                let v142: Rc<Vec<Rc<str>>> = Rc::new(v139.borrow().clone());
                let v143: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v142).as_ref().clone()));
                let v144: Rc<str> = Rc::<str>::from((*v143.borrow()).join(&*v135));
                v144
            }
            US1::US1_WordsRoot => {
                v2.clone()
            }
        };
        let v147: bool = std::fs::write(&*v78, &*v146).is_ok();
        let v228: US4 = if v147 {
            let v148: i32 = 40i32;
            let mut v149: bool = renamed_within_13(v0.clone(), v78.clone(), v148);
            if v149 {
                let v150: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
                let mut v151: Rc<str> = loop_3(v150);
                let mut v152: Option<usize> = v151.rfind(&*v7);
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
                let v182: US0 = if v161 {
                    US0::US0_OtherFile
                } else {
                    let v163: usize = ((0i32) as usize);
                    let v164: usize = ((v156) as usize);
                    let v165: Rc<str> = Rc::<str>::from(&(v151)[v163..v164]);
                    let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v167: bool = v165.ends_with(&*v166);
                    if v167 {
                        let v168: Rc<str> = loop_5(v165);
                        US0::US0_StateFile(v168)
                    } else {
                        let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v171: bool = v165.ends_with(&*v170);
                        if v171 {
                            let v172: Rc<str> = loop_6(v165);
                            US0::US0_HistoryFile(v172)
                        } else {
                            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v175: bool = v165.ends_with(&*v174);
                            if v175 {
                                let v176: Rc<str> = loop_7(v165);
                                US0::US0_ArchiveFile(v176)
                            } else {
                                US0::US0_OtherFile
                            }
                        }
                    }
                };
                let v205: US1 = match v182 {
                    US0::US0_ArchiveFile(v195) => {
                        let v195: Rc<str> = v195;
                        let v196: Rc<str> = string_concat(&[&*v195, "/state/spiral.toml"]);
                        let v197: bool = std::path::Path::new(&*v196).is_file();
                        if v197 {
                            US1::US1_SymbolsRoot
                        } else {
                            US1::US1_WordsRoot
                        }
                    }
                    US0::US0_HistoryFile(v189) => {
                        let v189: Rc<str> = v189;
                        let v190: Rc<str> = string_concat(&[&*v189, "/state/spiral.toml"]);
                        let v191: bool = std::path::Path::new(&*v190).is_file();
                        if v191 {
                            US1::US1_SymbolsRoot
                        } else {
                            US1::US1_WordsRoot
                        }
                    }
                    US0::US0_OtherFile => {
                        US1::US1_WordsRoot
                    }
                    US0::US0_StateFile(v183) => {
                        let v183: Rc<str> = v183;
                        let v184: Rc<str> = string_concat(&[&*v183, "/state/spiral.toml"]);
                        let v185: bool = std::path::Path::new(&*v184).is_file();
                        if v185 {
                            US1::US1_SymbolsRoot
                        } else {
                            US1::US1_WordsRoot
                        }
                    }
                };
                let v217: Rc<str> = match v205 {
                    US1::US1_SymbolsRoot => {
                        let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v207: Rc<UH0> = split_list_8(v206.clone(), v2);
                        let v208: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v209: Rc<UH0> = method11(v207, v208);
                        let v210: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v211: i32 = 0i32;
                        let v212: i32 = method12(v210.clone(), v209, v211);
                        let v213: Rc<Vec<Rc<str>>> = Rc::new(v210.borrow().clone());
                        let v214: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v213).as_ref().clone()));
                        let v215: Rc<str> = Rc::<str>::from((*v214.borrow()).join(&*v206));
                        v215
                    }
                    US1::US1_WordsRoot => {
                        v2
                    }
                };
                let v218: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v217);
                if v218 {
                    US4::US4_Replaced
                } else {
                    US4::US4_ReadbackMismatch
                }
            } else {
                let v222: bool = std::fs::remove_file(&*v78).is_ok() || !std::path::Path::new(&*v78).exists();
                if v222 {
                    US4::US4_RenameFailed
                } else {
                    US4::US4_RenameFailedTemporaryKept
                }
            }
        } else {
            US4::US4_TemporaryWriteFailed
        };
        match &v228 {
            US4::US4_Replaced => {
                US8::US8_StateWritten
            }
            _ => {
                let v239: Rc<str> = match v228 {
                    US4::US4_ChangedSinceRead => {
                        let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v234
                    }
                    US4::US4_ReadbackMismatch => {
                        let v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v233
                    }
                    US4::US4_RenameFailed => {
                        let v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v231
                    }
                    US4::US4_RenameFailedTemporaryKept => {
                        let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v232
                    }
                    US4::US4_TemporaryWriteFailed => {
                        let v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v230
                    }
                    _ => unreachable!(),
                };
                US8::US8_StateWriteFailed(v239)
            }
        }
    } else {
        US8::US8_StateChangedSinceRead
    }
}
fn note_persisted_29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> (Rc<str>, u64) {
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RIG_AGILE_NOTE_SEGMENTS"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = get_environment_variable_30(v7);
    let mut v9: bool = &*v8 == "1";
    let mut v13: bool = if v9 {
        true
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AGILE_NOTE_SEGMENTS"); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = get_environment_variable_30(v10);
        let mut v12: bool = &*v11 == "1";
        v12
    };
    if v13 {
        let mut v14: Rc<str> = note_appended_0(v0, v5, v6);
        let v15: i32 = (v2.len() as i32);
        let mut v16: u64 = (v15 as u64);
        (v14, v16)
    } else {
        let v17: Rc<str> = notes_appended_31(v3, v4, v5);
        let v18: US8 = state_written_36(v1, v2, v17.clone());
        match &v18 {
            US8::US8_StateWritten => {
                let mut v19: i32 = (v17.len() as i32);
                let v20: u64 = (v19 as u64);
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v21, v20)
            }
            _ => {
                let mut v25: Rc<str> = match v18 {
                    US8::US8_StateChangedSinceRead => {
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v22
                    }
                    US8::US8_StateWriteFailed(v23) => {
                        let v23: Rc<str> = v23;
                        v23
                    }
                    _ => unreachable!(),
                };
                (v25, 0u64)
            }
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, u64)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>| -> (Rc<str>, u64) {
        note_persisted_29(v0, v1, v2, v3, v4, v5, v6)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_note_appended(v0: &str, v1: &str, v2: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub fn rig_agile_note_persisted(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str, v5: &str, v6: &str) -> (Rc<str>, u64) {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), Rc::<str>::from(v5), Rc::<str>::from(v6))
}
pub type SpiralShape_652accfc6a8d = Mut0;
