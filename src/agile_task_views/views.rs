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
    US4_StateWritten,
    US4_StateChangedSinceRead,
    US4_StateWriteFailed(Rc<str>),
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
    US7_SpiprojProject,
    US7_TomlProject,
}
#[derive(Clone)]
enum US8 {
    US8_SplitProject(Rc<UH0>, Rc<UH0>, Rc<UH0>),
    US8_NoModules,
}
#[derive(Clone)]
enum US9 {
    US9_ModuleListed,
    US9_ModuleAdded(Rc<str>),
    US9_NoModulesSection,
}
#[derive(Clone)]
enum US10 {
    US10_Some(i32),
    US10_None,
}
#[derive(Clone)]
enum US12 {
    US12_ViewList,
    US12_ViewNext,
}
#[derive(Clone)]
enum US11 {
    US11_VerbRead(US12),
    US11_VerbRefused,
}
#[derive(Clone)]
enum US13 {
    US13_ScanPlain,
    US13_ScanQuoted,
    US13_ScanEscaped,
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US15 {
    US15_ViewsCommand,
    US15_ViewCommand,
    US15_ViewAddCommand,
}
#[derive(Clone)]
enum US14 {
    US14_CommandRead(US15),
    US14_CommandUnknown,
}
fn loop_4(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
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
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
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
fn loop_8(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
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
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method11(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
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
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US3 = US3::US3_HeadInl;
                        US2::US2_KeywordHead(v18)
                    } else {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let mut v21: bool = v2.starts_with(&*v20);
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
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
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
                        let mut v42: US3 = US3::US3_HeadOpen;
                        US2::US2_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let mut v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let mut v46: US3 = US3::US3_HeadOpen;
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
fn views_loaded_3(mut v0: Rc<str>) -> (bool, Rc<str>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/views.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v2, &*v2]);
    let mut v7: bool = std::path::Path::new(&*v6).is_file();
    let mut v10: US0 = if v7 {
        US0::US0_FileReadable(v6)
    } else {
        US0::US0_FileMissing(v6)
    };
    let (mut v63, mut v64): (bool, Rc<str>) = match &v10 {
        US0::US0_FileMissing(v11) => {
            let mut v11: Rc<str> = v11.clone();
            (false, v2)
        }
        _ => {
            let (mut v61, mut v62): (bool, Rc<str>) = match v10 {
                US0::US0_FileReadable(v12) => {
                    let v12: Rc<str> = v12;
                    let v13: Rc<str> = std::fs::read_to_string(&*v12).unwrap_or_default().into();
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let v16: Rc<str> = Rc::<str>::from(v12.replace(&*v14, &*v15));
                    let mut v17: Rc<str> = loop_4(v16);
                    let v18: Option<usize> = v17.rfind(&*v15);
                    let mut v19: Option<usize> = method5(v18);
                    let mut v20: Rc<dyn Fn((usize)) -> i32> = closure1();
                    let mut v21: Option<i32> = v19.map(|x| v20(x));
                    let v23: i32 = v21.unwrap_or(-1i32);
                    let v24: bool = v23 < 0i32;
                    let v28: bool = if v24 {
                        true
                    } else {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                        let v26: bool = v17.ends_with(&*v25);
                        let v27: bool = v26 == false;
                        v27
                    };
                    let v49: US1 = if v28 {
                        US1::US1_OtherFile
                    } else {
                        let v30: usize = ((0i32) as usize);
                        let mut v31: usize = ((v23) as usize);
                        let v32: Rc<str> = Rc::<str>::from(&(v17)[v30..v31]);
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                        let v34: bool = v32.ends_with(&*v33);
                        if v34 {
                            let v35: Rc<str> = loop_6(v32);
                            US1::US1_StateFile(v35)
                        } else {
                            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                            let v38: bool = v32.ends_with(&*v37);
                            if v38 {
                                let v39: Rc<str> = loop_7(v32);
                                US1::US1_HistoryFile(v39)
                            } else {
                                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                let mut v42: bool = v32.ends_with(&*v41);
                                if v42 {
                                    let v43: Rc<str> = loop_8(v32);
                                    US1::US1_ArchiveFile(v43)
                                } else {
                                    US1::US1_OtherFile
                                }
                            }
                        }
                    };
                    let v60: Rc<str> = match v49 {
                        US1::US1_OtherFile => {
                            v13
                        }
                        _ => {
                            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let v51: Rc<UH0> = split_list_9(v50.clone(), v13);
                            let v52: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v53: Rc<UH0> = method12(v51, v52);
                            let v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v55: i32 = 0i32;
                            let v56: i32 = method13(v54.clone(), v53, v55);
                            let v57: Rc<Vec<Rc<str>>> = Rc::new(v54.borrow().clone());
                            let v58: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v57).as_ref().clone()));
                            let v59: Rc<str> = Rc::<str>::from((*v58.borrow()).join(&*v50));
                            v59
                        }
                    };
                    (true, v60)
                }
                _ => unreachable!(),
            };
            (v61, v62)
        }
    };
    (v63, v64)
}
fn concat5_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn declared_from_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
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
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method17(v3, v1);
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
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US3 = US3::US3_HeadInl;
                        US2::US2_KeywordHead(v18)
                    } else {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let mut v21: bool = v2.starts_with(&*v20);
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
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
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
                        let mut v42: US3 = US3::US3_HeadOpen;
                        US2::US2_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let mut v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let mut v46: US3 = US3::US3_HeadOpen;
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
fn renamed_within_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn views_written_16(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> US4 {
    let v78: bool = if v1 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
        let mut v7: Rc<str> = loop_4(v6);
        let v8: Option<usize> = v7.rfind(&*v5);
        let mut v9: Option<usize> = method5(v8);
        let mut v10: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v11: Option<i32> = v9.map(|x| v10(x));
        let v13: i32 = v11.unwrap_or(-1i32);
        let v14: bool = v13 < 0i32;
        let v18: bool = if v14 {
            true
        } else {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v16: bool = v7.ends_with(&*v15);
            let mut v17: bool = v16 == false;
            v17
        };
        let v39: US1 = if v18 {
            US1::US1_OtherFile
        } else {
            let mut v20: usize = ((0i32) as usize);
            let mut v21: usize = ((v13) as usize);
            let v22: Rc<str> = Rc::<str>::from(&(v7)[v20..v21]);
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v24: bool = v22.ends_with(&*v23);
            if v24 {
                let v25: Rc<str> = loop_6(v22);
                US1::US1_StateFile(v25)
            } else {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v28: bool = v22.ends_with(&*v27);
                if v28 {
                    let v29: Rc<str> = loop_7(v22);
                    US1::US1_HistoryFile(v29)
                } else {
                    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v32: bool = v22.ends_with(&*v31);
                    if v32 {
                        let v33: Rc<str> = loop_8(v22);
                        US1::US1_ArchiveFile(v33)
                    } else {
                        US1::US1_OtherFile
                    }
                }
            }
        };
        let v62: US5 = match v39 {
            US1::US1_ArchiveFile(v52) => {
                let v52: Rc<str> = v52;
                let v53: Rc<str> = string_concat(&[&*v52, "/state/spiral.toml"]);
                let v54: bool = std::path::Path::new(&*v53).is_file();
                if v54 {
                    US5::US5_SymbolsRoot
                } else {
                    US5::US5_WordsRoot
                }
            }
            US1::US1_HistoryFile(v46) => {
                let mut v46: Rc<str> = v46;
                let v47: Rc<str> = string_concat(&[&*v46, "/state/spiral.toml"]);
                let v48: bool = std::path::Path::new(&*v47).is_file();
                if v48 {
                    US5::US5_SymbolsRoot
                } else {
                    US5::US5_WordsRoot
                }
            }
            US1::US1_OtherFile => {
                US5::US5_WordsRoot
            }
            US1::US1_StateFile(v40) => {
                let v40: Rc<str> = v40;
                let v41: Rc<str> = string_concat(&[&*v40, "/state/spiral.toml"]);
                let mut v42: bool = std::path::Path::new(&*v41).is_file();
                if v42 {
                    US5::US5_SymbolsRoot
                } else {
                    US5::US5_WordsRoot
                }
            }
        };
        let v74: Rc<str> = match v62 {
            US5::US5_SymbolsRoot => {
                let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v64: Rc<UH0> = split_list_9(v63.clone(), v2);
                let v65: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v66: Rc<UH0> = method17(v64, v65);
                let v67: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v68: i32 = 0i32;
                let v69: i32 = method13(v67.clone(), v66, v68);
                let v70: Rc<Vec<Rc<str>>> = Rc::new(v67.borrow().clone());
                let v71: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v70).as_ref().clone()));
                let v72: Rc<str> = Rc::<str>::from((*v71.borrow()).join(&*v63));
                v72
            }
            US5::US5_WordsRoot => {
                v2
            }
        };
        let v75: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v74);
        v75
    } else {
        let v76: bool = std::path::Path::new(&*v0).is_file();
        let v77: bool = v76 == false;
        v77
    };
    if v78 {
        let v79: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v82: Rc<str> = Rc::<str>::from(v79.replace(&*v80, &*v81));
        let mut v83: Rc<str> = loop_4(v82);
        let v84: Option<usize> = v83.rfind(&*v81);
        let v85: Option<usize> = method5(v84);
        let mut v86: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v87: Option<i32> = v85.map(|x| v86(x));
        let v89: i32 = v87.unwrap_or(-1i32);
        let v90: bool = v89 < 0i32;
        let v94: bool = if v90 {
            true
        } else {
            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v92: bool = v83.ends_with(&*v91);
            let v93: bool = v92 == false;
            v93
        };
        let v115: US1 = if v94 {
            US1::US1_OtherFile
        } else {
            let v96: usize = ((0i32) as usize);
            let v97: usize = ((v89) as usize);
            let v98: Rc<str> = Rc::<str>::from(&(v83)[v96..v97]);
            let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v100: bool = v98.ends_with(&*v99);
            if v100 {
                let v101: Rc<str> = loop_6(v98);
                US1::US1_StateFile(v101)
            } else {
                let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v104: bool = v98.ends_with(&*v103);
                if v104 {
                    let v105: Rc<str> = loop_7(v98);
                    US1::US1_HistoryFile(v105)
                } else {
                    let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v108: bool = v98.ends_with(&*v107);
                    if v108 {
                        let v109: Rc<str> = loop_8(v98);
                        US1::US1_ArchiveFile(v109)
                    } else {
                        US1::US1_OtherFile
                    }
                }
            }
        };
        let v138: US5 = match v115 {
            US1::US1_ArchiveFile(v128) => {
                let v128: Rc<str> = v128;
                let v129: Rc<str> = string_concat(&[&*v128, "/state/spiral.toml"]);
                let v130: bool = std::path::Path::new(&*v129).is_file();
                if v130 {
                    US5::US5_SymbolsRoot
                } else {
                    US5::US5_WordsRoot
                }
            }
            US1::US1_HistoryFile(v122) => {
                let mut v122: Rc<str> = v122;
                let v123: Rc<str> = string_concat(&[&*v122, "/state/spiral.toml"]);
                let v124: bool = std::path::Path::new(&*v123).is_file();
                if v124 {
                    US5::US5_SymbolsRoot
                } else {
                    US5::US5_WordsRoot
                }
            }
            US1::US1_OtherFile => {
                US5::US5_WordsRoot
            }
            US1::US1_StateFile(v116) => {
                let v116: Rc<str> = v116;
                let v117: Rc<str> = string_concat(&[&*v116, "/state/spiral.toml"]);
                let v118: bool = std::path::Path::new(&*v117).is_file();
                if v118 {
                    US5::US5_SymbolsRoot
                } else {
                    US5::US5_WordsRoot
                }
            }
        };
        let v150: Rc<str> = match v138 {
            US5::US5_SymbolsRoot => {
                let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v140: Rc<UH0> = split_list_9(v139.clone(), v3.clone());
                let v141: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v142: Rc<UH0> = method17(v140, v141);
                let v143: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v144: i32 = 0i32;
                let v145: i32 = method13(v143.clone(), v142, v144);
                let v146: Rc<Vec<Rc<str>>> = Rc::new(v143.borrow().clone());
                let v147: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v146).as_ref().clone()));
                let v148: Rc<str> = Rc::<str>::from((*v147.borrow()).join(&*v139));
                v148
            }
            US5::US5_WordsRoot => {
                v3.clone()
            }
        };
        let v151: bool = std::fs::write(&*v79, &*v150).is_ok();
        let v232: US6 = if v151 {
            let v152: i32 = 40i32;
            let v153: bool = renamed_within_18(v0.clone(), v79.clone(), v152);
            if v153 {
                let v154: Rc<str> = Rc::<str>::from(v0.replace(&*v80, &*v81));
                let mut v155: Rc<str> = loop_4(v154);
                let v156: Option<usize> = v155.rfind(&*v81);
                let v157: Option<usize> = method5(v156);
                let mut v158: Option<i32> = v157.map(|x| v86(x));
                let v160: i32 = v158.unwrap_or(-1i32);
                let v161: bool = v160 < 0i32;
                let v165: bool = if v161 {
                    true
                } else {
                    let v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v163: bool = v155.ends_with(&*v162);
                    let v164: bool = v163 == false;
                    v164
                };
                let v186: US1 = if v165 {
                    US1::US1_OtherFile
                } else {
                    let v167: usize = ((0i32) as usize);
                    let v168: usize = ((v160) as usize);
                    let v169: Rc<str> = Rc::<str>::from(&(v155)[v167..v168]);
                    let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v171: bool = v169.ends_with(&*v170);
                    if v171 {
                        let v172: Rc<str> = loop_6(v169);
                        US1::US1_StateFile(v172)
                    } else {
                        let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v175: bool = v169.ends_with(&*v174);
                        if v175 {
                            let v176: Rc<str> = loop_7(v169);
                            US1::US1_HistoryFile(v176)
                        } else {
                            let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v179: bool = v169.ends_with(&*v178);
                            if v179 {
                                let v180: Rc<str> = loop_8(v169);
                                US1::US1_ArchiveFile(v180)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                let v209: US5 = match v186 {
                    US1::US1_ArchiveFile(v199) => {
                        let v199: Rc<str> = v199;
                        let v200: Rc<str> = string_concat(&[&*v199, "/state/spiral.toml"]);
                        let v201: bool = std::path::Path::new(&*v200).is_file();
                        if v201 {
                            US5::US5_SymbolsRoot
                        } else {
                            US5::US5_WordsRoot
                        }
                    }
                    US1::US1_HistoryFile(v193) => {
                        let v193: Rc<str> = v193;
                        let v194: Rc<str> = string_concat(&[&*v193, "/state/spiral.toml"]);
                        let v195: bool = std::path::Path::new(&*v194).is_file();
                        if v195 {
                            US5::US5_SymbolsRoot
                        } else {
                            US5::US5_WordsRoot
                        }
                    }
                    US1::US1_OtherFile => {
                        US5::US5_WordsRoot
                    }
                    US1::US1_StateFile(v187) => {
                        let v187: Rc<str> = v187;
                        let v188: Rc<str> = string_concat(&[&*v187, "/state/spiral.toml"]);
                        let v189: bool = std::path::Path::new(&*v188).is_file();
                        if v189 {
                            US5::US5_SymbolsRoot
                        } else {
                            US5::US5_WordsRoot
                        }
                    }
                };
                let v221: Rc<str> = match v209 {
                    US5::US5_SymbolsRoot => {
                        let v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v211: Rc<UH0> = split_list_9(v210.clone(), v3);
                        let v212: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v213: Rc<UH0> = method17(v211, v212);
                        let v214: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v215: i32 = 0i32;
                        let v216: i32 = method13(v214.clone(), v213, v215);
                        let v217: Rc<Vec<Rc<str>>> = Rc::new(v214.borrow().clone());
                        let v218: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v217).as_ref().clone()));
                        let v219: Rc<str> = Rc::<str>::from((*v218.borrow()).join(&*v210));
                        v219
                    }
                    US5::US5_WordsRoot => {
                        v3
                    }
                };
                let v222: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v221);
                if v222 {
                    US6::US6_Replaced
                } else {
                    US6::US6_ReadbackMismatch
                }
            } else {
                let v226: bool = std::fs::remove_file(&*v79).is_ok() || !std::path::Path::new(&*v79).exists();
                if v226 {
                    US6::US6_RenameFailed
                } else {
                    US6::US6_RenameFailedTemporaryKept
                }
            }
        } else {
            US6::US6_TemporaryWriteFailed
        };
        match &v232 {
            US6::US6_Replaced => {
                US4::US4_StateWritten
            }
            _ => {
                let v243: Rc<str> = match v232 {
                    US6::US6_ChangedSinceRead => {
                        let v238: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v238
                    }
                    US6::US6_ReadbackMismatch => {
                        let v237: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v237
                    }
                    US6::US6_RenameFailed => {
                        let v235: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v235
                    }
                    US6::US6_RenameFailedTemporaryKept => {
                        let v236: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v236
                    }
                    US6::US6_TemporaryWriteFailed => {
                        let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v234
                    }
                    _ => unreachable!(),
                };
                US4::US4_StateWriteFailed(v243)
            }
        }
    } else {
        US4::US4_StateChangedSinceRead
    }
}
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method20(v3, v1);
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
fn section_lines_22(mut v0: US7, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> US8 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v8, v9) => {
                let v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH0> = v9.clone();
                let mut v19: bool = match &v0 {
                    US7::US7_SpiprojProject => {
                        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let mut v11: bool = v8.starts_with(&*v10);
                        let v14: bool = if v11 {
                            true
                        } else {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let v13: bool = v8.starts_with(&*v12);
                            v13
                        };
                        let v15: bool = v14 == false;
                        v15
                    }
                    US7::US7_TomlProject => {
                        let v16: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                        let mut v17: bool = &*v16 == "'''";
                        v17
                    }
                };
                if v19 {
                    let mut v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v21: Rc<UH0> = method10(v3, v20);
                    return US8::US8_SplitProject(v2, v21, v1);
                } else {
                    let v23: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                    let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v3));
                    (v1, v3) = (v9, v24);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH0> = method10(v3, v4);
                let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return US8::US8_SplitProject(v2, v5, v6);
            }
        }
    }
}
fn split_lines_21(mut v0: US7, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> US8 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<str> = Rc::<str>::from((v4).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let mut v10: Rc<str> = match &v0 {
                    US7::US7_SpiprojProject => {
                        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                        v7
                    }
                    US7::US7_TomlProject => {
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                        v8
                    }
                };
                let mut v11: bool = &*v6 == &*v10;
                if v11 {
                    let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v12));
                    let v14: Rc<UH0> = method10(v2, v13);
                    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    return section_lines_22(v0, v5, v14, v15);
                } else {
                    let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v2));
                    (v1, v2) = (v5, v17);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US8::US8_NoModules;
            }
        }
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US10> {
    Rc::new(move |mut v2: Rc<UH0>| -> US10 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US10::US10_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US10> = method23(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US10::US10_None
            }
        }
    })
}
fn method23(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US10> {
    closure2(v0, v1)
}
fn method24(mut v0: US7, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method24(v0.clone(), v4, v2);
            let v13: Rc<str> = match v0 {
                US7::US7_SpiprojProject => {
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
                US7::US7_TomlProject => {
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
fn method25(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method25(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn views_module_registered_19(mut v0: Rc<str>) -> US4 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v2, &*v2]);
    let mut v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v7.borrow_mut().push(v6.clone());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.toml"); } LIT.with(|lit| lit.clone()) };
    v7.borrow_mut().push(v8.clone());
    let mut v9: Rc<Vec<Rc<str>>> = Rc::new(v7.borrow().clone());
    let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v9).as_ref().clone()));
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from((*v10.borrow()).join(&*v11));
    let v13: bool = std::path::Path::new(&*v12).is_file();
    let v16: US7 = if v13 {
        US7::US7_TomlProject
    } else {
        US7::US7_SpiprojProject
    };
    let mut v19: Rc<str> = match v16 {
        US7::US7_SpiprojProject => {
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
            v17
        }
        US7::US7_TomlProject => {
            v8.clone()
        }
    };
    let mut v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v20.borrow_mut().push(v6.clone());
    v20.borrow_mut().push(v19);
    let mut v21: Rc<Vec<Rc<str>>> = Rc::new(v20.borrow().clone());
    let v22: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v21).as_ref().clone()));
    let v23: Rc<str> = Rc::<str>::from((*v22.borrow()).join(&*v11));
    let v24: bool = std::path::Path::new(&*v23).is_file();
    let v27: US0 = if v24 {
        US0::US0_FileReadable(v23.clone())
    } else {
        US0::US0_FileMissing(v23.clone())
    };
    match v27 {
        US0::US0_FileMissing(v28) => {
            let v28: Rc<str> = v28;
            US4::US4_StateWritten
        }
        US0::US0_FileReadable(v30) => {
            let v30: Rc<str> = v30;
            let mut v31: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v31.borrow_mut().push(v6);
            v31.borrow_mut().push(v8);
            let v32: Rc<Vec<Rc<str>>> = Rc::new(v31.borrow().clone());
            let v33: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v32).as_ref().clone()));
            let v34: Rc<str> = Rc::<str>::from((*v33.borrow()).join(&*v11));
            let v35: bool = std::path::Path::new(&*v34).is_file();
            let v38: US7 = if v35 {
                US7::US7_TomlProject
            } else {
                US7::US7_SpiprojProject
            };
            let v39: Rc<str> = std::fs::read_to_string(&*v30).unwrap_or_default().into();
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v41: Rc<str> = Rc::<str>::from(v30.replace(&*v40, &*v11));
            let mut v42: Rc<str> = loop_4(v41);
            let v43: Option<usize> = v42.rfind(&*v11);
            let v44: Option<usize> = method5(v43);
            let mut v45: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v46: Option<i32> = v44.map(|x| v45(x));
            let v48: i32 = v46.unwrap_or(-1i32);
            let v49: bool = v48 < 0i32;
            let v53: bool = if v49 {
                true
            } else {
                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v51: bool = v42.ends_with(&*v50);
                let v52: bool = v51 == false;
                v52
            };
            let v73: US1 = if v53 {
                US1::US1_OtherFile
            } else {
                let v55: usize = ((0i32) as usize);
                let v56: usize = ((v48) as usize);
                let v57: Rc<str> = Rc::<str>::from(&(v42)[v55..v56]);
                let v58: bool = v57.ends_with(&*v1);
                if v58 {
                    let v59: Rc<str> = loop_6(v57);
                    US1::US1_StateFile(v59)
                } else {
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v62: bool = v57.ends_with(&*v61);
                    if v62 {
                        let v63: Rc<str> = loop_7(v57);
                        US1::US1_HistoryFile(v63)
                    } else {
                        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v66: bool = v57.ends_with(&*v65);
                        if v66 {
                            let v67: Rc<str> = loop_8(v57);
                            US1::US1_ArchiveFile(v67)
                        } else {
                            US1::US1_OtherFile
                        }
                    }
                }
            };
            let v84: Rc<str> = match v73 {
                US1::US1_OtherFile => {
                    v39
                }
                _ => {
                    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v75: Rc<UH0> = split_list_9(v74.clone(), v39);
                    let v76: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v77: Rc<UH0> = method12(v75, v76);
                    let v78: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v79: i32 = 0i32;
                    let v80: i32 = method13(v78.clone(), v77, v79);
                    let v81: Rc<Vec<Rc<str>>> = Rc::new(v78.borrow().clone());
                    let v82: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v81).as_ref().clone()));
                    let mut v83: Rc<str> = Rc::<str>::from((*v82.borrow()).join(&*v74));
                    v83
                }
            };
            let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("views"); } LIT.with(|lit| lit.clone()) };
            let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v88: Rc<str> = Rc::<str>::from(v84.replace(&*v87, &*v2));
            let v89: Rc<UH0> = split_list_9(v86.clone(), v88);
            let v90: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v91: Rc<UH0> = method20(v89, v90);
            let v92: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v93: US8 = split_lines_21(v38.clone(), v91, v92);
            let v129: US9 = match v93 {
                US8::US8_NoModules => {
                    US9::US9_NoModulesSection
                }
                US8::US8_SplitProject(v95, v96, v97) => {
                    let v95: Rc<UH0> = v95;
                    let v96: Rc<UH0> = v96;
                    let v97: Rc<UH0> = v97;
                    let v98: i32 = 0i32;
                    let mut v99: Rc<dyn Fn(Rc<UH0>) -> US10> = method23(v85.clone(), v98);
                    let v100: US10 = v99(v96.clone());
                    let v101: bool = match v100 {
                        US10::US10_None => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v102: bool = v101 != true;
                    if v102 {
                        US9::US9_ModuleListed
                    } else {
                        let v104: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v105: Rc<UH0> = method24(v38.clone(), v96, v104);
                        let v112: Rc<str> = match v38 {
                            US7::US7_SpiprojProject => {
                                let mut v106: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                                v106.borrow_mut().push(v107);
                                v106.borrow_mut().push(v85);
                                let v108: Rc<Vec<Rc<str>>> = Rc::new(v106.borrow().clone());
                                let v109: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v108).as_ref().clone()));
                                let v110: Rc<str> = Rc::<str>::from((*v109.borrow()).join(&*v2));
                                v110
                            }
                            US7::US7_TomlProject => {
                                v85
                            }
                        };
                        let v113: Rc<UH0> = method25(v105, v97);
                        let v114: Rc<UH0> = Rc::new(UH0::UH0_Cons(v112, v113));
                        let v115: Rc<UH0> = method25(v95, v114);
                        let v116: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v117: i32 = 0i32;
                        let v118: i32 = method13(v116.clone(), v115, v117);
                        let v119: Rc<Vec<Rc<str>>> = Rc::new(v116.borrow().clone());
                        let v120: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v119).as_ref().clone()));
                        let v121: Rc<str> = Rc::<str>::from((*v120.borrow()).join(&*v86));
                        let mut v122: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        v122.borrow_mut().push(v121);
                        v122.borrow_mut().push(v86.clone());
                        let v123: Rc<Vec<Rc<str>>> = Rc::new(v122.borrow().clone());
                        let v124: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v123).as_ref().clone()));
                        let v125: Rc<str> = Rc::<str>::from((*v124.borrow()).join(&*v2));
                        US9::US9_ModuleAdded(v125)
                    }
                }
            };
            match v129 {
                US9::US9_ModuleAdded(v133) => {
                    let v133: Rc<str> = v133;
                    let v134: Rc<str> = string_concat(&[&*v23, ".pending"]);
                    let v135: Rc<str> = Rc::<str>::from(v134.replace(&*v40, &*v11));
                    let mut v136: Rc<str> = loop_4(v135);
                    let v137: Option<usize> = v136.rfind(&*v11);
                    let v138: Option<usize> = method5(v137);
                    let mut v139: Option<i32> = v138.map(|x| v45(x));
                    let v141: i32 = v139.unwrap_or(-1i32);
                    let v142: bool = v141 < 0i32;
                    let v146: bool = if v142 {
                        true
                    } else {
                        let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                        let v144: bool = v136.ends_with(&*v143);
                        let v145: bool = v144 == false;
                        v145
                    };
                    let v166: US1 = if v146 {
                        US1::US1_OtherFile
                    } else {
                        let v148: usize = ((0i32) as usize);
                        let v149: usize = ((v141) as usize);
                        let v150: Rc<str> = Rc::<str>::from(&(v136)[v148..v149]);
                        let v151: bool = v150.ends_with(&*v1);
                        if v151 {
                            let v152: Rc<str> = loop_6(v150);
                            US1::US1_StateFile(v152)
                        } else {
                            let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                            let mut v155: bool = v150.ends_with(&*v154);
                            if v155 {
                                let v156: Rc<str> = loop_7(v150);
                                US1::US1_HistoryFile(v156)
                            } else {
                                let mut v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                let v159: bool = v150.ends_with(&*v158);
                                if v159 {
                                    let v160: Rc<str> = loop_8(v150);
                                    US1::US1_ArchiveFile(v160)
                                } else {
                                    US1::US1_OtherFile
                                }
                            }
                        }
                    };
                    let v189: US5 = match v166 {
                        US1::US1_ArchiveFile(v179) => {
                            let v179: Rc<str> = v179;
                            let v180: Rc<str> = string_concat(&[&*v179, "/state/spiral.toml"]);
                            let v181: bool = std::path::Path::new(&*v180).is_file();
                            if v181 {
                                US5::US5_SymbolsRoot
                            } else {
                                US5::US5_WordsRoot
                            }
                        }
                        US1::US1_HistoryFile(v173) => {
                            let v173: Rc<str> = v173;
                            let v174: Rc<str> = string_concat(&[&*v173, "/state/spiral.toml"]);
                            let v175: bool = std::path::Path::new(&*v174).is_file();
                            if v175 {
                                US5::US5_SymbolsRoot
                            } else {
                                US5::US5_WordsRoot
                            }
                        }
                        US1::US1_OtherFile => {
                            US5::US5_WordsRoot
                        }
                        US1::US1_StateFile(v167) => {
                            let v167: Rc<str> = v167;
                            let v168: Rc<str> = string_concat(&[&*v167, "/state/spiral.toml"]);
                            let v169: bool = std::path::Path::new(&*v168).is_file();
                            if v169 {
                                US5::US5_SymbolsRoot
                            } else {
                                US5::US5_WordsRoot
                            }
                        }
                    };
                    let v200: Rc<str> = match v189 {
                        US5::US5_SymbolsRoot => {
                            let v190: Rc<UH0> = split_list_9(v86.clone(), v133.clone());
                            let v191: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v192: Rc<UH0> = method17(v190, v191);
                            let v193: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v194: i32 = 0i32;
                            let v195: i32 = method13(v193.clone(), v192, v194);
                            let v196: Rc<Vec<Rc<str>>> = Rc::new(v193.borrow().clone());
                            let v197: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v196).as_ref().clone()));
                            let v198: Rc<str> = Rc::<str>::from((*v197.borrow()).join(&*v86));
                            v198
                        }
                        US5::US5_WordsRoot => {
                            v133.clone()
                        }
                    };
                    let v201: bool = std::fs::write(&*v134, &*v200).is_ok();
                    let v280: US6 = if v201 {
                        let v202: i32 = 40i32;
                        let v203: bool = renamed_within_18(v23.clone(), v134.clone(), v202);
                        if v203 {
                            let v204: Rc<str> = Rc::<str>::from(v23.replace(&*v40, &*v11));
                            let mut v205: Rc<str> = loop_4(v204);
                            let v206: Option<usize> = v205.rfind(&*v11);
                            let v207: Option<usize> = method5(v206);
                            let mut v208: Option<i32> = v207.map(|x| v45(x));
                            let v210: i32 = v208.unwrap_or(-1i32);
                            let v211: bool = v210 < 0i32;
                            let v215: bool = if v211 {
                                true
                            } else {
                                let v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                let v213: bool = v205.ends_with(&*v212);
                                let v214: bool = v213 == false;
                                v214
                            };
                            let v235: US1 = if v215 {
                                US1::US1_OtherFile
                            } else {
                                let v217: usize = ((0i32) as usize);
                                let v218: usize = ((v210) as usize);
                                let v219: Rc<str> = Rc::<str>::from(&(v205)[v217..v218]);
                                let v220: bool = v219.ends_with(&*v1);
                                if v220 {
                                    let v221: Rc<str> = loop_6(v219);
                                    US1::US1_StateFile(v221)
                                } else {
                                    let v223: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                    let v224: bool = v219.ends_with(&*v223);
                                    if v224 {
                                        let v225: Rc<str> = loop_7(v219);
                                        US1::US1_HistoryFile(v225)
                                    } else {
                                        let v227: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                        let v228: bool = v219.ends_with(&*v227);
                                        if v228 {
                                            let v229: Rc<str> = loop_8(v219);
                                            US1::US1_ArchiveFile(v229)
                                        } else {
                                            US1::US1_OtherFile
                                        }
                                    }
                                }
                            };
                            let v258: US5 = match v235 {
                                US1::US1_ArchiveFile(v248) => {
                                    let v248: Rc<str> = v248;
                                    let v249: Rc<str> = string_concat(&[&*v248, "/state/spiral.toml"]);
                                    let v250: bool = std::path::Path::new(&*v249).is_file();
                                    if v250 {
                                        US5::US5_SymbolsRoot
                                    } else {
                                        US5::US5_WordsRoot
                                    }
                                }
                                US1::US1_HistoryFile(v242) => {
                                    let v242: Rc<str> = v242;
                                    let v243: Rc<str> = string_concat(&[&*v242, "/state/spiral.toml"]);
                                    let v244: bool = std::path::Path::new(&*v243).is_file();
                                    if v244 {
                                        US5::US5_SymbolsRoot
                                    } else {
                                        US5::US5_WordsRoot
                                    }
                                }
                                US1::US1_OtherFile => {
                                    US5::US5_WordsRoot
                                }
                                US1::US1_StateFile(v236) => {
                                    let v236: Rc<str> = v236;
                                    let v237: Rc<str> = string_concat(&[&*v236, "/state/spiral.toml"]);
                                    let v238: bool = std::path::Path::new(&*v237).is_file();
                                    if v238 {
                                        US5::US5_SymbolsRoot
                                    } else {
                                        US5::US5_WordsRoot
                                    }
                                }
                            };
                            let v269: Rc<str> = match v258 {
                                US5::US5_SymbolsRoot => {
                                    let v259: Rc<UH0> = split_list_9(v86.clone(), v133);
                                    let v260: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v261: Rc<UH0> = method17(v259, v260);
                                    let v262: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                    let v263: i32 = 0i32;
                                    let v264: i32 = method13(v262.clone(), v261, v263);
                                    let v265: Rc<Vec<Rc<str>>> = Rc::new(v262.borrow().clone());
                                    let v266: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v265).as_ref().clone()));
                                    let v267: Rc<str> = Rc::<str>::from((*v266.borrow()).join(&*v86));
                                    v267
                                }
                                US5::US5_WordsRoot => {
                                    v133
                                }
                            };
                            let v270: bool = std::fs::read_to_string(&*v23).is_ok_and(|value| value == *v269);
                            if v270 {
                                US6::US6_Replaced
                            } else {
                                US6::US6_ReadbackMismatch
                            }
                        } else {
                            let v274: bool = std::fs::remove_file(&*v134).is_ok() || !std::path::Path::new(&*v134).exists();
                            if v274 {
                                US6::US6_RenameFailed
                            } else {
                                US6::US6_RenameFailedTemporaryKept
                            }
                        }
                    } else {
                        US6::US6_TemporaryWriteFailed
                    };
                    match &v280 {
                        US6::US6_Replaced => {
                            US4::US4_StateWritten
                        }
                        _ => {
                            let v291: Rc<str> = match v280 {
                                US6::US6_ChangedSinceRead => {
                                    let v286: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                    v286
                                }
                                US6::US6_ReadbackMismatch => {
                                    let v285: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                    v285
                                }
                                US6::US6_RenameFailed => {
                                    let v283: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                    v283
                                }
                                US6::US6_RenameFailedTemporaryKept => {
                                    let v284: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                    v284
                                }
                                US6::US6_TemporaryWriteFailed => {
                                    let v282: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                    v282
                                }
                                _ => unreachable!(),
                            };
                            US4::US4_StateWriteFailed(v291)
                        }
                    }
                }
                US9::US9_ModuleListed => {
                    US4::US4_StateWritten
                }
                US9::US9_NoModulesSection => {
                    let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the state project has no modules section to list views in"); } LIT.with(|lit| lit.clone()) };
                    US4::US4_StateWriteFailed(v131)
                }
            }
        }
    }
}
fn view_stored_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let (mut v4, mut v5): (bool, Rc<str>) = views_loaded_3(v0.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View ("); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = Rc::<str>::from(v1.replace(&*v8, &*v9));
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(v10.replace(&*v7, &*v11));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(v12.replace(&*v13, &*v14));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(v15.replace(&*v16, &*v17));
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = Rc::<str>::from(v18.replace(&*v19, &*v20));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = concat5_14(v7.clone(), v21, v7.clone(), v22.clone(), v22.clone());
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = string_concat(&[&*v6, &*v23, &*v24, &*v22, &*v22]);
    let v29: i32 = 0i32;
    let v30: i32 = declared_from_15(v5.clone(), v28, v29);
    let mut v31: bool = v30 < 0i32;
    let v32: bool = v31 == false;
    let v130: Rc<str> = if v32 {
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view_"); } LIT.with(|lit| lit.clone()) };
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = Rc::<str>::from(v1.replace(&*v35, &*v36));
        let v41: Rc<str> = string_concat(&[&*v34, &*v37, &*v22, &*v22, &*v22]);
        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = View ("); } LIT.with(|lit| lit.clone()) };
        let v43: Rc<str> = Rc::<str>::from(v1.replace(&*v8, &*v9));
        let v44: Rc<str> = Rc::<str>::from(v43.replace(&*v7, &*v11));
        let mut v45: Rc<str> = Rc::<str>::from(v44.replace(&*v13, &*v14));
        let mut v46: Rc<str> = Rc::<str>::from(v45.replace(&*v16, &*v17));
        let v47: Rc<str> = Rc::<str>::from(v46.replace(&*v19, &*v20));
        let v48: Rc<str> = concat5_14(v7.clone(), v47, v7.clone(), v22.clone(), v22.clone());
        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v54: Rc<str> = Rc::<str>::from(v2.replace(&*v8, &*v9));
        let v55: Rc<str> = Rc::<str>::from(v54.replace(&*v7, &*v11));
        let v56: Rc<str> = Rc::<str>::from(v55.replace(&*v13, &*v14));
        let v57: Rc<str> = Rc::<str>::from(v56.replace(&*v16, &*v17));
        let v58: Rc<str> = Rc::<str>::from(v57.replace(&*v19, &*v20));
        let v59: Rc<str> = concat5_14(v7.clone(), v58, v7.clone(), v22.clone(), v22.clone());
        let v60: Rc<str> = Rc::<str>::from(v3.replace(&*v8, &*v9));
        let v61: Rc<str> = Rc::<str>::from(v60.replace(&*v7, &*v11));
        let v62: Rc<str> = Rc::<str>::from(v61.replace(&*v13, &*v14));
        let v63: Rc<str> = Rc::<str>::from(v62.replace(&*v16, &*v17));
        let v64: Rc<str> = Rc::<str>::from(v63.replace(&*v19, &*v20));
        let v65: Rc<str> = concat5_14(v7.clone(), v64, v7.clone(), v22.clone(), v22.clone());
        let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v70: Rc<str> = string_concat(&[&*v33, &*v41, &*v42, &*v48, &*v49, &*v59, &*v49, &*v65, &*v66]);
        let v71: i32 = v5.get(..v30 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
        let v72: i32 = v5.get(v30 as usize..).and_then(|rest| rest.find(&*v13)).map(|found| (found + v30 as usize) as i32).unwrap_or(-1);
        let v73: bool = v72 < 0i32;
        let v75: i32 = if v73 {
            let v74: i32 = (v5.clone().len() as i32);
            v74
        } else {
            v72
        };
        let v76: Rc<str> = std::rc::Rc::<str>::from(v5.get(0i32 as usize..v71 as usize).unwrap_or(""));
        let v77: i32 = (v5.clone().len() as i32);
        let v78: Rc<str> = std::rc::Rc::<str>::from(v5.get(v75 as usize..v77 as usize).unwrap_or(""));
        let v82: Rc<str> = string_concat(&[&*v76, &*v70, &*v78, &*v22, &*v22]);
        v82
    } else {
        let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view_"); } LIT.with(|lit| lit.clone()) };
        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
        let mut v87: Rc<str> = Rc::<str>::from(v1.replace(&*v85, &*v86));
        let v91: Rc<str> = string_concat(&[&*v84, &*v87, &*v22, &*v22, &*v22]);
        let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = View ("); } LIT.with(|lit| lit.clone()) };
        let v93: Rc<str> = Rc::<str>::from(v1.replace(&*v8, &*v9));
        let v94: Rc<str> = Rc::<str>::from(v93.replace(&*v7, &*v11));
        let v95: Rc<str> = Rc::<str>::from(v94.replace(&*v13, &*v14));
        let v96: Rc<str> = Rc::<str>::from(v95.replace(&*v16, &*v17));
        let v97: Rc<str> = Rc::<str>::from(v96.replace(&*v19, &*v20));
        let v98: Rc<str> = concat5_14(v7.clone(), v97, v7.clone(), v22.clone(), v22.clone());
        let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v104: Rc<str> = Rc::<str>::from(v2.replace(&*v8, &*v9));
        let v105: Rc<str> = Rc::<str>::from(v104.replace(&*v7, &*v11));
        let mut v106: Rc<str> = Rc::<str>::from(v105.replace(&*v13, &*v14));
        let v107: Rc<str> = Rc::<str>::from(v106.replace(&*v16, &*v17));
        let v108: Rc<str> = Rc::<str>::from(v107.replace(&*v19, &*v20));
        let v109: Rc<str> = concat5_14(v7.clone(), v108, v7.clone(), v22.clone(), v22.clone());
        let v110: Rc<str> = Rc::<str>::from(v3.replace(&*v8, &*v9));
        let v111: Rc<str> = Rc::<str>::from(v110.replace(&*v7, &*v11));
        let v112: Rc<str> = Rc::<str>::from(v111.replace(&*v13, &*v14));
        let v113: Rc<str> = Rc::<str>::from(v112.replace(&*v16, &*v17));
        let v114: Rc<str> = Rc::<str>::from(v113.replace(&*v19, &*v20));
        let v115: Rc<str> = concat5_14(v7.clone(), v114, v7.clone(), v22.clone(), v22.clone());
        let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v120: Rc<str> = string_concat(&[&*v83, &*v91, &*v92, &*v98, &*v99, &*v109, &*v99, &*v115, &*v116]);
        let v121: bool = &*v5 == "";
        let v123: Rc<str> = if v121 {
            let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion agile_view =\n    | View :: string * string * string -> agile_view\n"); } LIT.with(|lit| lit.clone()) };
            v122
        } else {
            v5.clone()
        };
        let v124: bool = v123.ends_with(&*v13);
        let v125: Rc<str> = if v124 {
            v22.clone()
        } else {
            v13.clone()
        };
        let v129: Rc<str> = string_concat(&[&*v123, &*v125, &*v120, &*v13, &*v22]);
        v129
    };
    let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/views.spi"); } LIT.with(|lit| lit.clone()) };
    let v135: Rc<str> = string_concat(&[&*v0, &*v131, &*v22, &*v22, &*v22]);
    let mut v136: US4 = views_written_16(v135, v4, v5, v130);
    let v138: US4 = match &v136 {
        US4::US4_StateWritten => {
            views_module_registered_19(v0)
        }
        _ => {
            v136
        }
    };
    match &v138 {
        US4::US4_StateWritten => {
            let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" verb="); } LIT.with(|lit| lit.clone()) };
            let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" flags='"); } LIT.with(|lit| lit.clone()) };
            let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
            let v145: Rc<str> = string_concat(&[&*v139, &*v2, &*v140, &*v3, &*v141]);
            let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig agile view-add ok name="); } LIT.with(|lit| lit.clone()) };
            let v149: Rc<str> = if v32 {
                let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" replaced=yes"); } LIT.with(|lit| lit.clone()) };
                v147
            } else {
                let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" replaced=no"); } LIT.with(|lit| lit.clone()) };
                v148
            };
            let v153: Rc<str> = string_concat(&[&*v146, &*v1, &*v145, &*v149, &*v22]);
            let v154: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v153.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            v154
        }
        _ => {
            let mut v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add "); } LIT.with(|lit| lit.clone()) };
            let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let v160: Rc<str> = match v138 {
                US4::US4_StateChangedSinceRead => {
                    let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                    v157
                }
                US4::US4_StateWriteFailed(v158) => {
                    let mut v158: Rc<str> = v158;
                    v158
                }
                _ => unreachable!(),
            };
            let v164: Rc<str> = string_concat(&[&*v155, &*v1, &*v156, &*v160, &*v22]);
            let v165: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v164 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v165
        }
    }
}
fn view_add_run_1() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v4: bool = !(v3).is_empty() && (v3).bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == 45u8);
    if v4 {
        let v5: i32 = 3i32;
        let mut v6: Rc<str> = usize::try_from(v5).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v7: bool = "list" == &*v6;
        let v15: US11 = if v7 {
            let v8: US12 = US12::US12_ViewList;
            US11::US11_VerbRead(v8)
        } else {
            let mut v10: bool = "next" == &*v6;
            if v10 {
                let mut v11: US12 = US12::US12_ViewNext;
                US11::US11_VerbRead(v11)
            } else {
                US11::US11_VerbRefused
            }
        };
        match v15 {
            US11::US11_VerbRead(v24) => {
                let v24: US12 = v24;
                let v25: bool = std::env::args().skip(6i32 as usize).all(|argument| !argument.is_empty() && !argument.contains(char::is_whitespace));
                if v25 {
                    let v29: Rc<str> = match v24 {
                        US12::US12_ViewList => {
                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                            v26
                        }
                        US12::US12_ViewNext => {
                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) };
                            v27
                        }
                    };
                    let v30: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(6i32 as usize).collect::<Vec<_>>().join(" ").as_str());
                    view_stored_2(v1, v3, v29, v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: each stored flag must be one word without spaces"); } LIT.with(|lit| lit.clone()) };
                    let v33: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v32 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v33
                }
            }
            US11::US11_VerbRefused => {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: view verbs are "); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list|next"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v22: Rc<str> = string_concat(&[&*v16, &*v17, &*v18, &*v18, &*v18]);
                let v23: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v22 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v23
            }
        }
    } else {
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: view names are lowercase letters, digits and hyphens"); } LIT.with(|lit| lit.clone()) };
        let v38: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v37 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v38
    }
}
fn field_end_29(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US13) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US13::US13_ScanPlain => {
                    let mut v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let mut v7: bool = v5 == 41i32;
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
                let mut v10: i32 = v1.wrapping_add(1i32);
                let v25: US13 = match v3 {
                    US13::US13_ScanEscaped => {
                        US13::US13_ScanQuoted
                    }
                    US13::US13_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US13::US13_ScanQuoted
                        } else {
                            US13::US13_ScanPlain
                        }
                    }
                    US13::US13_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US13::US13_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US13::US13_ScanPlain
                            } else {
                                US13::US13_ScanQuoted
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
fn skip_spaces_30(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_28(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US13 = US13::US13_ScanPlain;
            let mut v6: i32 = field_end_29(v0.clone(), v1, v2, v5);
            let mut v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_30(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn unescaped_from_32(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return concat5_14(v4, v6, v7.clone(), v7.clone(), v7.clone());
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v10: bool = v9 == 92i32;
            if v10 {
                let mut v11: i32 = v1.wrapping_add(1i32);
                let v12: i32 = v11.wrapping_add(1i32);
                let v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v15: bool = v14 == 110i32;
                let v24: Rc<str> = if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16
                } else {
                    let mut v17: bool = v14 == 114i32;
                    if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18
                    } else {
                        let mut v19: bool = v14 == 116i32;
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
                let v26: Rc<str> = concat5_14(v4, v13, v24, v25.clone(), v25.clone());
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
fn unquoted_31(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_32(v3, v4, v5, v6, v7)
}
fn view_at_27(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1.wrapping_add(6i32);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = 0i32;
    let v5: i32 = field_start_28(v0.clone(), v2, v3, v4);
    let mut v6: US13 = US13::US13_ScanPlain;
    let mut v7: i32 = field_end_29(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_31(v8);
    let mut v10: i32 = 1i32;
    let mut v11: i32 = field_start_28(v0.clone(), v2, v3, v10);
    let v12: US13 = US13::US13_ScanPlain;
    let v13: i32 = field_end_29(v0.clone(), v11, v3, v12);
    let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v13 as usize).unwrap_or(""));
    let v15: Rc<str> = unquoted_31(v14);
    let v16: i32 = 2i32;
    let mut v17: i32 = field_start_28(v0.clone(), v2, v3, v16);
    let v18: US13 = US13::US13_ScanPlain;
    let mut v19: i32 = field_end_29(v0.clone(), v17, v3, v18);
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let mut v21: Rc<str> = unquoted_31(v20);
    (v9, v15, v21)
}
fn view_run_26() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v4: bool = &*v3 == "";
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile view <root> <name> [extra flags...]"); } LIT.with(|lit| lit.clone()) };
        let mut v6: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v5 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v6
    } else {
        let (mut v7, mut v8): (bool, Rc<str>) = views_loaded_3(v1.clone());
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View ("); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = Rc::<str>::from(v3.replace(&*v11, &*v12));
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = Rc::<str>::from(v13.replace(&*v10, &*v14));
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = Rc::<str>::from(v15.replace(&*v16, &*v17));
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = Rc::<str>::from(v18.replace(&*v19, &*v20));
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = Rc::<str>::from(v21.replace(&*v22, &*v23));
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = concat5_14(v10.clone(), v24, v10.clone(), v25.clone(), v25.clone());
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = string_concat(&[&*v9, &*v26, &*v27, &*v25, &*v25]);
        let v32: i32 = 0i32;
        let v33: i32 = declared_from_15(v8.clone(), v31, v32);
        let v34: bool = v33 < 0i32;
        if v34 {
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view: no view named "); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/views.spi"); } LIT.with(|lit| lit.clone()) };
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; agile views <root> lists them"); } LIT.with(|lit| lit.clone()) };
            let mut v46: Rc<str> = string_concat(&[&*v35, &*v3, &*v36, &*v1, &*v37, &*v25, &*v25, &*v25, &*v42]);
            let v47: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v46 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v47
        } else {
            let (mut v48, mut v49, mut v50): (Rc<str>, Rc<str>, Rc<str>) = view_at_27(v8, v33);
            let v51: i32 = { let extra = std::env::args().skip(5i32 as usize).collect::<Vec<_>>(); match std::env::current_exe().and_then(|exe| std::process::Command::new(exe).arg("agile").arg(&*v49).arg(&*v1).args((v50).split_whitespace()).args(&extra).status()) { Ok(status) => status.code().unwrap_or(1), Err(error) => { eprintln!("rig error: agile view could not start: {}", error); 2 } } };
            v51
        }
    }
}
fn views_listed_34(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View ("); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = string_concat(&[&*v3, &*v4, &*v5, &*v5, &*v5]);
        let mut v10: i32 = declared_from_15(v0.clone(), v9, v1);
        let mut v11: bool = v10 < 0i32;
        if v11 {
            return v2;
        } else {
            let (mut v12, mut v13, mut v14): (Rc<str>, Rc<str>, Rc<str>) = view_at_27(v0.clone(), v10);
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = string_concat(&[&*v12, &*v15, &*v13, &*v15, &*v14]);
            let mut v20: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v19.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            let mut v21: i32 = v10.wrapping_add(1i32);
            let v22: i32 = v2.wrapping_add(1i32);
            (v1, v2) = (v21, v22);
            continue;
        }
    }
}
fn method36(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_35(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method36(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn views_run_33() -> i32 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: i32 = 1i32;
    let mut v2: Rc<str> = usize::try_from(v1).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let (mut v3, mut v4): (bool, Rc<str>) = views_loaded_3(v2.clone());
    let v5: i32 = 0i32;
    let mut v6: i32 = 0i32;
    let mut v7: i32 = views_listed_34(v4, v5, v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig agile views ok views="); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from((v7).to_string().as_str());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/views.spi"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = string_concat(&[&*v2, &*v11, &*v12, &*v12, &*v12]);
    let mut v17: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
    let v18: Rc<str> = format_real_35(v17);
    let mut v19: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    v19.borrow_mut().push(v20);
    v19.borrow_mut().push(v18);
    let mut v21: Rc<Vec<Rc<str>>> = Rc::new(v19.borrow().clone());
    let v22: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v21).as_ref().clone()));
    let v23: Rc<str> = Rc::<str>::from((*v22.borrow()).join(&*v12));
    let v27: Rc<str> = string_concat(&[&*v8, &*v9, &*v10, &*v16, &*v23]);
    let v28: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v27.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v28
}
fn views_entry_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: bool = "views" == &*v1;
    let v14: US14 = if v2 {
        let v3: US15 = US15::US15_ViewsCommand;
        US14::US14_CommandRead(v3)
    } else {
        let v5: bool = "view" == &*v1;
        if v5 {
            let mut v6: US15 = US15::US15_ViewCommand;
            US14::US14_CommandRead(v6)
        } else {
            let v8: bool = "view-add" == &*v1;
            if v8 {
                let mut v9: US15 = US15::US15_ViewAddCommand;
                US14::US14_CommandRead(v9)
            } else {
                US14::US14_CommandUnknown
            }
        }
    };
    match v14 {
        US14::US14_CommandRead(v15) => {
            let v15: US15 = v15;
            match v15 {
                US15::US15_ViewAddCommand => {
                    view_add_run_1()
                }
                US15::US15_ViewCommand => {
                    view_run_26()
                }
                US15::US15_ViewsCommand => {
                    views_run_33()
                }
            }
        }
        US14::US14_CommandUnknown => {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile views: expected views, view or view-add"); } LIT.with(|lit| lit.clone()) };
            let v23: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v22 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v23
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        views_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_views_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
