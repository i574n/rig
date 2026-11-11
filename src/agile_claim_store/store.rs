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
    US4_WordsRoot,
    US4_SymbolsRoot,
}
#[derive(Clone)]
enum US5 {
    US5_StateWritten,
    US5_StateChangedSinceRead,
    US5_StateWriteFailed(Rc<str>),
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
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, u64, Rc<UH0>, Rc<UH1>),
}
#[derive(Clone)]
enum US11 {
    US11_Some(Rc<str>, Rc<str>, u64, Rc<UH0>),
    US11_None,
}
#[derive(Clone)]
enum US12 {
    US12_Some(u64),
    US12_None,
}
#[derive(Clone)]
enum US13 {
    US13_ChangeUpsert,
    US13_ChangeRemove,
    US13_ChangeNone,
}
pub struct Mut1 { pub l0: Rc<str> }
fn loop_1(mut v0: Rc<str>) -> Rc<str> {
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
fn method2(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_3(mut v0: Rc<str>) -> Rc<str> {
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
fn loop_4(mut v0: Rc<str>) -> Rc<str> {
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
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
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
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method8(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method8(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method7(v16, v17)
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method9(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let mut v15: US2 = if v6 {
                let v7: US3 = US3::US3_HeadType;
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
                    let mut v17: bool = v2.starts_with(&*v16);
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
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let mut v29: bool = v2.starts_with(&*v28);
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
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let mut v41: bool = v2.starts_with(&*v40);
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
fn method10(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn claims_loaded_0(mut v0: Rc<str>) -> (bool, Rc<str>) {
    let mut v1: Rc<str> = string_concat(&[&*v0, "/state/claims.spi"]);
    let mut v2: bool = std::path::Path::new(&*v1).is_file();
    let v5: US0 = if v2 {
        US0::US0_FileReadable(v1)
    } else {
        US0::US0_FileMissing(v1)
    };
    match v5 {
        US0::US0_FileMissing(v6) => {
            let mut v6: Rc<str> = v6;
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, v7)
        }
        US0::US0_FileReadable(v8) => {
            let mut v8: Rc<str> = v8;
            let mut v9: Rc<str> = std::fs::read_to_string(&*v8).unwrap_or_default().into();
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = Rc::<str>::from(v8.replace(&*v10, &*v11));
            let mut v13: Rc<str> = loop_1(v12);
            let mut v14: Option<usize> = v13.rfind(&*v11);
            let mut v15: Option<usize> = method2(v14);
            let mut v16: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v17: Option<i32> = v15.map(|x| v16(x));
            let v19: i32 = v17.unwrap_or(-1i32);
            let v20: bool = v19 < 0i32;
            let v24: bool = if v20 {
                true
            } else {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v22: bool = v13.ends_with(&*v21);
                let v23: bool = v22 == false;
                v23
            };
            let v45: US1 = if v24 {
                US1::US1_OtherFile
            } else {
                let mut v26: usize = ((0i32) as usize);
                let v27: usize = ((v19) as usize);
                let mut v28: Rc<str> = Rc::<str>::from(&(v13)[v26..v27]);
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v30: bool = v28.ends_with(&*v29);
                if v30 {
                    let v31: Rc<str> = loop_3(v28);
                    US1::US1_StateFile(v31)
                } else {
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v34: bool = v28.ends_with(&*v33);
                    if v34 {
                        let v35: Rc<str> = loop_4(v28);
                        US1::US1_HistoryFile(v35)
                    } else {
                        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v38: bool = v28.ends_with(&*v37);
                        if v38 {
                            let v39: Rc<str> = loop_5(v28);
                            US1::US1_ArchiveFile(v39)
                        } else {
                            US1::US1_OtherFile
                        }
                    }
                }
            };
            let v56: Rc<str> = match v45 {
                US1::US1_OtherFile => {
                    v9
                }
                _ => {
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v47: Rc<UH0> = split_list_6(v46.clone(), v9);
                    let v48: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v49: Rc<UH0> = method9(v47, v48);
                    let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v51: i32 = 0i32;
                    let v52: i32 = method10(v50.clone(), v49, v51);
                    let v53: Rc<Vec<Rc<str>>> = Rc::new(v50.borrow().clone());
                    let v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v53).as_ref().clone()));
                    let v55: Rc<str> = Rc::<str>::from((*v54.borrow()).join(&*v46));
                    v55
                }
            };
            (true, v56)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (bool, Rc<str>) {
        claims_loaded_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method14(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let mut v15: US2 = if v6 {
                let v7: US3 = US3::US3_HeadType;
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
                    let mut v17: bool = v2.starts_with(&*v16);
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
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let mut v29: bool = v2.starts_with(&*v28);
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
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let mut v41: bool = v2.starts_with(&*v40);
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
fn method18(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
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
                        let mut v8: bool = v4 == b'\n';
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
fn method19(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
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
                        let mut v8: bool = v4 == b'\n';
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
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method17(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let mut v6: i32 = 0i32;
            let v7: i32 = method18(v2.clone(), v5, v6);
            let mut v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2.clone(), v7 as i64, v8 as i64);
            let mut v10: i32 = (v9.clone().len() as i32);
            let mut v11: i32 = method19(v9.clone(), v10);
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
fn section_lines_21(mut v0: US7, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> US8 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v8, v9) => {
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH0> = v9.clone();
                let mut v26: bool = match &v0 {
                    US7::US7_SpiprojProject => {
                        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
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
                    US7::US7_TomlProject => {
                        let mut v16: i32 = (v8.clone().len() as i32);
                        let mut v17: i32 = 0i32;
                        let v18: i32 = method18(v8.clone(), v16, v17);
                        let v19: i32 = v16.wrapping_sub(1i32);
                        let v20: Rc<str> = string_slice(&v8.clone(), v18 as i64, v19 as i64);
                        let v21: i32 = (v20.clone().len() as i32);
                        let v22: i32 = method19(v20.clone(), v21);
                        let v23: Rc<str> = string_slice(&v20, 0i32 as i64, v22 as i64);
                        let v24: bool = &*v23 == "'''";
                        v24
                    }
                };
                if v26 {
                    let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v28: Rc<UH0> = method7(v3, v27);
                    return US8::US8_SplitProject(v2, v28, v1);
                } else {
                    let v30: i32 = (v8.clone().len() as i32);
                    let v31: i32 = 0i32;
                    let v32: i32 = method18(v8.clone(), v30, v31);
                    let v33: i32 = v30.wrapping_sub(1i32);
                    let v34: Rc<str> = string_slice(&v8, v32 as i64, v33 as i64);
                    let v35: i32 = (v34.clone().len() as i32);
                    let v36: i32 = method19(v34.clone(), v35);
                    let mut v37: Rc<str> = string_slice(&v34, 0i32 as i64, v36 as i64);
                    let v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v37, v3));
                    (v1, v3) = (v9, v38);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH0> = method7(v3, v4);
                let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return US8::US8_SplitProject(v2, v5, v6);
            }
        }
    }
}
fn split_lines_20(mut v0: US7, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> US8 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: i32 = (v4.clone().len() as i32);
                let v7: i32 = 0i32;
                let mut v8: i32 = method18(v4.clone(), v6, v7);
                let mut v9: i32 = v6.wrapping_sub(1i32);
                let mut v10: Rc<str> = string_slice(&v4.clone(), v8 as i64, v9 as i64);
                let mut v11: i32 = (v10.clone().len() as i32);
                let mut v12: i32 = method19(v10.clone(), v11);
                let mut v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
                let mut v17: Rc<str> = match &v0 {
                    US7::US7_SpiprojProject => {
                        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                        v14
                    }
                    US7::US7_TomlProject => {
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                        v15
                    }
                };
                let v18: bool = &*v13 == &*v17;
                if v18 {
                    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v19));
                    let v21: Rc<UH0> = method7(v2, v20);
                    let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    return section_lines_21(v0, v5, v21, v22);
                } else {
                    let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v2));
                    (v1, v2) = (v5, v24);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US8::US8_NoModules;
            }
        }
    }
}
fn closure3(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US10> {
    Rc::new(move |mut v2: Rc<UH0>| -> US10 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US10::US10_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US10> = method22(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US10::US10_None
            }
        }
    })
}
fn method22(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US10> {
    closure3(v0, v1)
}
fn method23(mut v0: US7, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method23(v0.clone(), v4, v2);
            let mut v13: Rc<str> = match v0 {
                US7::US7_SpiprojProject => {
                    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                    v6.borrow_mut().push(v7);
                    v6.borrow_mut().push(v3);
                    let mut v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
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
fn method24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method24(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn module_registered_16(mut v0: Rc<str>) -> US5 {
    let mut v1: Rc<str> = string_concat(&[&*v0, "/state"]);
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v2.borrow_mut().push(v1.clone());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.toml"); } LIT.with(|lit| lit.clone()) };
    v2.borrow_mut().push(v3.clone());
    let v4: Rc<Vec<Rc<str>>> = Rc::new(v2.borrow().clone());
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v4).as_ref().clone()));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from((*v5.borrow()).join(&*v6));
    let mut v8: bool = std::path::Path::new(&*v7).is_file();
    let mut v11: US7 = if v8 {
        US7::US7_TomlProject
    } else {
        US7::US7_SpiprojProject
    };
    let mut v14: Rc<str> = match v11 {
        US7::US7_SpiprojProject => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
            v12
        }
        US7::US7_TomlProject => {
            v3.clone()
        }
    };
    let mut v15: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v15.borrow_mut().push(v1.clone());
    v15.borrow_mut().push(v14);
    let mut v16: Rc<Vec<Rc<str>>> = Rc::new(v15.borrow().clone());
    let mut v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v16).as_ref().clone()));
    let v18: Rc<str> = Rc::<str>::from((*v17.borrow()).join(&*v6));
    let v19: bool = std::path::Path::new(&*v18).is_file();
    let v22: US0 = if v19 {
        US0::US0_FileReadable(v18.clone())
    } else {
        US0::US0_FileMissing(v18.clone())
    };
    match v22 {
        US0::US0_FileMissing(v23) => {
            let v23: Rc<str> = v23;
            US5::US5_StateWritten
        }
        US0::US0_FileReadable(v25) => {
            let v25: Rc<str> = v25;
            let mut v26: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v26.borrow_mut().push(v1);
            v26.borrow_mut().push(v3);
            let v27: Rc<Vec<Rc<str>>> = Rc::new(v26.borrow().clone());
            let mut v28: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v27).as_ref().clone()));
            let mut v29: Rc<str> = Rc::<str>::from((*v28.borrow()).join(&*v6));
            let v30: bool = std::path::Path::new(&*v29).is_file();
            let v33: US7 = if v30 {
                US7::US7_TomlProject
            } else {
                US7::US7_SpiprojProject
            };
            let v34: Rc<str> = std::fs::read_to_string(&*v25).unwrap_or_default().into();
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = Rc::<str>::from(v25.replace(&*v35, &*v6));
            let mut v37: Rc<str> = loop_1(v36);
            let v38: Option<usize> = v37.rfind(&*v6);
            let v39: Option<usize> = method2(v38);
            let mut v40: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v41: Option<i32> = v39.map(|x| v40(x));
            let v43: i32 = v41.unwrap_or(-1i32);
            let v44: bool = v43 < 0i32;
            let v48: bool = if v44 {
                true
            } else {
                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v46: bool = v37.ends_with(&*v45);
                let v47: bool = v46 == false;
                v47
            };
            let v69: US1 = if v48 {
                US1::US1_OtherFile
            } else {
                let v50: usize = ((0i32) as usize);
                let v51: usize = ((v43) as usize);
                let v52: Rc<str> = Rc::<str>::from(&(v37)[v50..v51]);
                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v54: bool = v52.ends_with(&*v53);
                if v54 {
                    let v55: Rc<str> = loop_3(v52);
                    US1::US1_StateFile(v55)
                } else {
                    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v58: bool = v52.ends_with(&*v57);
                    if v58 {
                        let v59: Rc<str> = loop_4(v52);
                        US1::US1_HistoryFile(v59)
                    } else {
                        let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v62: bool = v52.ends_with(&*v61);
                        if v62 {
                            let v63: Rc<str> = loop_5(v52);
                            US1::US1_ArchiveFile(v63)
                        } else {
                            US1::US1_OtherFile
                        }
                    }
                }
            };
            let v80: Rc<str> = match v69 {
                US1::US1_OtherFile => {
                    v34
                }
                _ => {
                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v71: Rc<UH0> = split_list_6(v70.clone(), v34);
                    let v72: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v73: Rc<UH0> = method9(v71, v72);
                    let v74: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v75: i32 = 0i32;
                    let v76: i32 = method10(v74.clone(), v73, v75);
                    let v77: Rc<Vec<Rc<str>>> = Rc::new(v74.borrow().clone());
                    let v78: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v77).as_ref().clone()));
                    let v79: Rc<str> = Rc::<str>::from((*v78.borrow()).join(&*v70));
                    v79
                }
            };
            let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claims"); } LIT.with(|lit| lit.clone()) };
            let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v85: Rc<str> = Rc::<str>::from(v80.replace(&*v83, &*v84));
            let mut v86: Rc<UH0> = split_list_6(v82.clone(), v85);
            let v87: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v88: Rc<UH0> = method17(v86, v87);
            let mut v89: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v90: US8 = split_lines_20(v33.clone(), v88, v89);
            let v126: US9 = match v90 {
                US8::US8_NoModules => {
                    US9::US9_NoModulesSection
                }
                US8::US8_SplitProject(v92, v93, v94) => {
                    let v92: Rc<UH0> = v92;
                    let v93: Rc<UH0> = v93;
                    let v94: Rc<UH0> = v94;
                    let v95: i32 = 0i32;
                    let mut v96: Rc<dyn Fn(Rc<UH0>) -> US10> = method22(v81.clone(), v95);
                    let v97: US10 = v96(v93.clone());
                    let v98: bool = match v97 {
                        US10::US10_None => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v99: bool = v98 != true;
                    if v99 {
                        US9::US9_ModuleListed
                    } else {
                        let v101: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v102: Rc<UH0> = method23(v33.clone(), v93, v101);
                        let v109: Rc<str> = match v33 {
                            US7::US7_SpiprojProject => {
                                let mut v103: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                                v103.borrow_mut().push(v104);
                                v103.borrow_mut().push(v81);
                                let v105: Rc<Vec<Rc<str>>> = Rc::new(v103.borrow().clone());
                                let v106: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v105).as_ref().clone()));
                                let v107: Rc<str> = Rc::<str>::from((*v106.borrow()).join(&*v84));
                                v107
                            }
                            US7::US7_TomlProject => {
                                v81
                            }
                        };
                        let v110: Rc<UH0> = method24(v102, v94);
                        let v111: Rc<UH0> = Rc::new(UH0::UH0_Cons(v109, v110));
                        let v112: Rc<UH0> = method24(v92, v111);
                        let v113: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v114: i32 = 0i32;
                        let v115: i32 = method10(v113.clone(), v112, v114);
                        let v116: Rc<Vec<Rc<str>>> = Rc::new(v113.borrow().clone());
                        let v117: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v116).as_ref().clone()));
                        let v118: Rc<str> = Rc::<str>::from((*v117.borrow()).join(&*v82));
                        let mut v119: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        v119.borrow_mut().push(v118);
                        v119.borrow_mut().push(v82.clone());
                        let v120: Rc<Vec<Rc<str>>> = Rc::new(v119.borrow().clone());
                        let v121: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v120).as_ref().clone()));
                        let v122: Rc<str> = Rc::<str>::from((*v121.borrow()).join(&*v84));
                        US9::US9_ModuleAdded(v122)
                    }
                }
            };
            match v126 {
                US9::US9_ModuleAdded(v130) => {
                    let v130: Rc<str> = v130;
                    let v131: Rc<str> = string_concat(&[&*v18, ".pending"]);
                    let v132: Rc<str> = Rc::<str>::from(v131.replace(&*v35, &*v6));
                    let mut v133: Rc<str> = loop_1(v132);
                    let v134: Option<usize> = v133.rfind(&*v6);
                    let v135: Option<usize> = method2(v134);
                    let mut v136: Option<i32> = v135.map(|x| v40(x));
                    let v138: i32 = v136.unwrap_or(-1i32);
                    let v139: bool = v138 < 0i32;
                    let v143: bool = if v139 {
                        true
                    } else {
                        let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                        let v141: bool = v133.ends_with(&*v140);
                        let v142: bool = v141 == false;
                        v142
                    };
                    let v164: US1 = if v143 {
                        US1::US1_OtherFile
                    } else {
                        let v145: usize = ((0i32) as usize);
                        let v146: usize = ((v138) as usize);
                        let v147: Rc<str> = Rc::<str>::from(&(v133)[v145..v146]);
                        let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                        let v149: bool = v147.ends_with(&*v148);
                        if v149 {
                            let v150: Rc<str> = loop_3(v147);
                            US1::US1_StateFile(v150)
                        } else {
                            let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                            let v153: bool = v147.ends_with(&*v152);
                            if v153 {
                                let v154: Rc<str> = loop_4(v147);
                                US1::US1_HistoryFile(v154)
                            } else {
                                let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                let v157: bool = v147.ends_with(&*v156);
                                if v157 {
                                    let mut v158: Rc<str> = loop_5(v147);
                                    US1::US1_ArchiveFile(v158)
                                } else {
                                    US1::US1_OtherFile
                                }
                            }
                        }
                    };
                    let v187: US4 = match v164 {
                        US1::US1_ArchiveFile(v177) => {
                            let v177: Rc<str> = v177;
                            let v178: Rc<str> = string_concat(&[&*v177, "/state/spiral.toml"]);
                            let v179: bool = std::path::Path::new(&*v178).is_file();
                            if v179 {
                                US4::US4_SymbolsRoot
                            } else {
                                US4::US4_WordsRoot
                            }
                        }
                        US1::US1_HistoryFile(v171) => {
                            let v171: Rc<str> = v171;
                            let v172: Rc<str> = string_concat(&[&*v171, "/state/spiral.toml"]);
                            let v173: bool = std::path::Path::new(&*v172).is_file();
                            if v173 {
                                US4::US4_SymbolsRoot
                            } else {
                                US4::US4_WordsRoot
                            }
                        }
                        US1::US1_OtherFile => {
                            US4::US4_WordsRoot
                        }
                        US1::US1_StateFile(v165) => {
                            let v165: Rc<str> = v165;
                            let v166: Rc<str> = string_concat(&[&*v165, "/state/spiral.toml"]);
                            let v167: bool = std::path::Path::new(&*v166).is_file();
                            if v167 {
                                US4::US4_SymbolsRoot
                            } else {
                                US4::US4_WordsRoot
                            }
                        }
                    };
                    let v198: Rc<str> = match v187 {
                        US4::US4_SymbolsRoot => {
                            let v188: Rc<UH0> = split_list_6(v82.clone(), v130.clone());
                            let v189: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v190: Rc<UH0> = method14(v188, v189);
                            let v191: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v192: i32 = 0i32;
                            let v193: i32 = method10(v191.clone(), v190, v192);
                            let v194: Rc<Vec<Rc<str>>> = Rc::new(v191.borrow().clone());
                            let v195: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v194).as_ref().clone()));
                            let v196: Rc<str> = Rc::<str>::from((*v195.borrow()).join(&*v82));
                            v196
                        }
                        US4::US4_WordsRoot => {
                            v130.clone()
                        }
                    };
                    let v199: bool = std::fs::write(&*v131, &*v198).is_ok();
                    let v279: US6 = if v199 {
                        let v200: i32 = 40i32;
                        let v201: bool = renamed_within_15(v18.clone(), v131.clone(), v200);
                        if v201 {
                            let v202: Rc<str> = Rc::<str>::from(v18.replace(&*v35, &*v6));
                            let mut v203: Rc<str> = loop_1(v202);
                            let v204: Option<usize> = v203.rfind(&*v6);
                            let v205: Option<usize> = method2(v204);
                            let mut v206: Option<i32> = v205.map(|x| v40(x));
                            let v208: i32 = v206.unwrap_or(-1i32);
                            let v209: bool = v208 < 0i32;
                            let v213: bool = if v209 {
                                true
                            } else {
                                let v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                let v211: bool = v203.ends_with(&*v210);
                                let v212: bool = v211 == false;
                                v212
                            };
                            let v234: US1 = if v213 {
                                US1::US1_OtherFile
                            } else {
                                let v215: usize = ((0i32) as usize);
                                let v216: usize = ((v208) as usize);
                                let v217: Rc<str> = Rc::<str>::from(&(v203)[v215..v216]);
                                let v218: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                let v219: bool = v217.ends_with(&*v218);
                                if v219 {
                                    let v220: Rc<str> = loop_3(v217);
                                    US1::US1_StateFile(v220)
                                } else {
                                    let v222: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                    let v223: bool = v217.ends_with(&*v222);
                                    if v223 {
                                        let v224: Rc<str> = loop_4(v217);
                                        US1::US1_HistoryFile(v224)
                                    } else {
                                        let v226: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                        let v227: bool = v217.ends_with(&*v226);
                                        if v227 {
                                            let v228: Rc<str> = loop_5(v217);
                                            US1::US1_ArchiveFile(v228)
                                        } else {
                                            US1::US1_OtherFile
                                        }
                                    }
                                }
                            };
                            let v257: US4 = match v234 {
                                US1::US1_ArchiveFile(v247) => {
                                    let v247: Rc<str> = v247;
                                    let v248: Rc<str> = string_concat(&[&*v247, "/state/spiral.toml"]);
                                    let v249: bool = std::path::Path::new(&*v248).is_file();
                                    if v249 {
                                        US4::US4_SymbolsRoot
                                    } else {
                                        US4::US4_WordsRoot
                                    }
                                }
                                US1::US1_HistoryFile(v241) => {
                                    let v241: Rc<str> = v241;
                                    let v242: Rc<str> = string_concat(&[&*v241, "/state/spiral.toml"]);
                                    let v243: bool = std::path::Path::new(&*v242).is_file();
                                    if v243 {
                                        US4::US4_SymbolsRoot
                                    } else {
                                        US4::US4_WordsRoot
                                    }
                                }
                                US1::US1_OtherFile => {
                                    US4::US4_WordsRoot
                                }
                                US1::US1_StateFile(v235) => {
                                    let v235: Rc<str> = v235;
                                    let v236: Rc<str> = string_concat(&[&*v235, "/state/spiral.toml"]);
                                    let v237: bool = std::path::Path::new(&*v236).is_file();
                                    if v237 {
                                        US4::US4_SymbolsRoot
                                    } else {
                                        US4::US4_WordsRoot
                                    }
                                }
                            };
                            let v268: Rc<str> = match v257 {
                                US4::US4_SymbolsRoot => {
                                    let v258: Rc<UH0> = split_list_6(v82.clone(), v130);
                                    let v259: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v260: Rc<UH0> = method14(v258, v259);
                                    let v261: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                    let v262: i32 = 0i32;
                                    let v263: i32 = method10(v261.clone(), v260, v262);
                                    let v264: Rc<Vec<Rc<str>>> = Rc::new(v261.borrow().clone());
                                    let v265: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v264).as_ref().clone()));
                                    let v266: Rc<str> = Rc::<str>::from((*v265.borrow()).join(&*v82));
                                    v266
                                }
                                US4::US4_WordsRoot => {
                                    v130
                                }
                            };
                            let v269: bool = std::fs::read_to_string(&*v18).is_ok_and(|value| value == *v268);
                            if v269 {
                                US6::US6_Replaced
                            } else {
                                US6::US6_ReadbackMismatch
                            }
                        } else {
                            let v273: bool = std::fs::remove_file(&*v131).is_ok() || !std::path::Path::new(&*v131).exists();
                            if v273 {
                                US6::US6_RenameFailed
                            } else {
                                US6::US6_RenameFailedTemporaryKept
                            }
                        }
                    } else {
                        US6::US6_TemporaryWriteFailed
                    };
                    match &v279 {
                        US6::US6_Replaced => {
                            US5::US5_StateWritten
                        }
                        _ => {
                            let v290: Rc<str> = match v279 {
                                US6::US6_ChangedSinceRead => {
                                    let v285: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                    v285
                                }
                                US6::US6_ReadbackMismatch => {
                                    let v284: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                    v284
                                }
                                US6::US6_RenameFailed => {
                                    let v282: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                    v282
                                }
                                US6::US6_RenameFailedTemporaryKept => {
                                    let v283: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                    v283
                                }
                                US6::US6_TemporaryWriteFailed => {
                                    let v281: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                    v281
                                }
                                _ => unreachable!(),
                            };
                            US5::US5_StateWriteFailed(v290)
                        }
                    }
                }
                US9::US9_ModuleListed => {
                    US5::US5_StateWritten
                }
                US9::US9_NoModulesSection => {
                    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the state project has no modules section to list claims in"); } LIT.with(|lit| lit.clone()) };
                    US5::US5_StateWriteFailed(v128)
                }
            }
        }
    }
}
fn claims_saved_13(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = string_concat(&[&*v0, "/state/claims.spi"]);
    let v79: bool = if v1 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v6));
        let mut v8: Rc<str> = loop_1(v7);
        let mut v9: Option<usize> = v8.rfind(&*v6);
        let mut v10: Option<usize> = method2(v9);
        let mut v11: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v12: Option<i32> = v10.map(|x| v11(x));
        let mut v14: i32 = v12.unwrap_or(-1i32);
        let mut v15: bool = v14 < 0i32;
        let v19: bool = if v15 {
            true
        } else {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let mut v17: bool = v8.ends_with(&*v16);
            let v18: bool = v17 == false;
            v18
        };
        let mut v40: US1 = if v19 {
            US1::US1_OtherFile
        } else {
            let v21: usize = ((0i32) as usize);
            let v22: usize = ((v14) as usize);
            let v23: Rc<str> = Rc::<str>::from(&(v8)[v21..v22]);
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v25: bool = v23.ends_with(&*v24);
            if v25 {
                let mut v26: Rc<str> = loop_3(v23);
                US1::US1_StateFile(v26)
            } else {
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let mut v29: bool = v23.ends_with(&*v28);
                if v29 {
                    let v30: Rc<str> = loop_4(v23);
                    US1::US1_HistoryFile(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v23.ends_with(&*v32);
                    if v33 {
                        let v34: Rc<str> = loop_5(v23);
                        US1::US1_ArchiveFile(v34)
                    } else {
                        US1::US1_OtherFile
                    }
                }
            }
        };
        let v63: US4 = match v40 {
            US1::US1_ArchiveFile(v53) => {
                let v53: Rc<str> = v53;
                let v54: Rc<str> = string_concat(&[&*v53, "/state/spiral.toml"]);
                let v55: bool = std::path::Path::new(&*v54).is_file();
                if v55 {
                    US4::US4_SymbolsRoot
                } else {
                    US4::US4_WordsRoot
                }
            }
            US1::US1_HistoryFile(v47) => {
                let v47: Rc<str> = v47;
                let v48: Rc<str> = string_concat(&[&*v47, "/state/spiral.toml"]);
                let v49: bool = std::path::Path::new(&*v48).is_file();
                if v49 {
                    US4::US4_SymbolsRoot
                } else {
                    US4::US4_WordsRoot
                }
            }
            US1::US1_OtherFile => {
                US4::US4_WordsRoot
            }
            US1::US1_StateFile(v41) => {
                let mut v41: Rc<str> = v41;
                let v42: Rc<str> = string_concat(&[&*v41, "/state/spiral.toml"]);
                let v43: bool = std::path::Path::new(&*v42).is_file();
                if v43 {
                    US4::US4_SymbolsRoot
                } else {
                    US4::US4_WordsRoot
                }
            }
        };
        let v75: Rc<str> = match v63 {
            US4::US4_SymbolsRoot => {
                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v65: Rc<UH0> = split_list_6(v64.clone(), v2);
                let v66: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v67: Rc<UH0> = method14(v65, v66);
                let v68: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v69: i32 = 0i32;
                let v70: i32 = method10(v68.clone(), v67, v69);
                let v71: Rc<Vec<Rc<str>>> = Rc::new(v68.borrow().clone());
                let v72: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v71).as_ref().clone()));
                let v73: Rc<str> = Rc::<str>::from((*v72.borrow()).join(&*v64));
                v73
            }
            US4::US4_WordsRoot => {
                v2
            }
        };
        let v76: bool = std::fs::read_to_string(&*v4).is_ok_and(|value| value == *v75);
        v76
    } else {
        let v77: bool = std::path::Path::new(&*v4).is_file();
        let v78: bool = v77 == false;
        v78
    };
    let v80: bool = v79 == false;
    let v249: US5 = if v80 {
        US5::US5_StateChangedSinceRead
    } else {
        let v82: Rc<str> = string_concat(&[&*v4, ".pending"]);
        let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v85: Rc<str> = Rc::<str>::from(v82.replace(&*v83, &*v84));
        let mut v86: Rc<str> = loop_1(v85);
        let v87: Option<usize> = v86.rfind(&*v84);
        let v88: Option<usize> = method2(v87);
        let mut v89: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v90: Option<i32> = v88.map(|x| v89(x));
        let v92: i32 = v90.unwrap_or(-1i32);
        let v93: bool = v92 < 0i32;
        let v97: bool = if v93 {
            true
        } else {
            let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v95: bool = v86.ends_with(&*v94);
            let mut v96: bool = v95 == false;
            v96
        };
        let v118: US1 = if v97 {
            US1::US1_OtherFile
        } else {
            let v99: usize = ((0i32) as usize);
            let v100: usize = ((v92) as usize);
            let v101: Rc<str> = Rc::<str>::from(&(v86)[v99..v100]);
            let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let mut v103: bool = v101.ends_with(&*v102);
            if v103 {
                let v104: Rc<str> = loop_3(v101);
                US1::US1_StateFile(v104)
            } else {
                let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v107: bool = v101.ends_with(&*v106);
                if v107 {
                    let v108: Rc<str> = loop_4(v101);
                    US1::US1_HistoryFile(v108)
                } else {
                    let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v111: bool = v101.ends_with(&*v110);
                    if v111 {
                        let v112: Rc<str> = loop_5(v101);
                        US1::US1_ArchiveFile(v112)
                    } else {
                        US1::US1_OtherFile
                    }
                }
            }
        };
        let v141: US4 = match v118 {
            US1::US1_ArchiveFile(v131) => {
                let v131: Rc<str> = v131;
                let v132: Rc<str> = string_concat(&[&*v131, "/state/spiral.toml"]);
                let mut v133: bool = std::path::Path::new(&*v132).is_file();
                if v133 {
                    US4::US4_SymbolsRoot
                } else {
                    US4::US4_WordsRoot
                }
            }
            US1::US1_HistoryFile(v125) => {
                let v125: Rc<str> = v125;
                let v126: Rc<str> = string_concat(&[&*v125, "/state/spiral.toml"]);
                let v127: bool = std::path::Path::new(&*v126).is_file();
                if v127 {
                    US4::US4_SymbolsRoot
                } else {
                    US4::US4_WordsRoot
                }
            }
            US1::US1_OtherFile => {
                US4::US4_WordsRoot
            }
            US1::US1_StateFile(v119) => {
                let mut v119: Rc<str> = v119;
                let v120: Rc<str> = string_concat(&[&*v119, "/state/spiral.toml"]);
                let v121: bool = std::path::Path::new(&*v120).is_file();
                if v121 {
                    US4::US4_SymbolsRoot
                } else {
                    US4::US4_WordsRoot
                }
            }
        };
        let v153: Rc<str> = match v141 {
            US4::US4_SymbolsRoot => {
                let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v143: Rc<UH0> = split_list_6(v142.clone(), v3.clone());
                let v144: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v145: Rc<UH0> = method14(v143, v144);
                let v146: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v147: i32 = 0i32;
                let v148: i32 = method10(v146.clone(), v145, v147);
                let v149: Rc<Vec<Rc<str>>> = Rc::new(v146.borrow().clone());
                let v150: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v149).as_ref().clone()));
                let v151: Rc<str> = Rc::<str>::from((*v150.borrow()).join(&*v142));
                v151
            }
            US4::US4_WordsRoot => {
                v3.clone()
            }
        };
        let v154: bool = std::fs::write(&*v82, &*v153).is_ok();
        let v235: US6 = if v154 {
            let v155: i32 = 40i32;
            let v156: bool = renamed_within_15(v4.clone(), v82.clone(), v155);
            if v156 {
                let v157: Rc<str> = Rc::<str>::from(v4.replace(&*v83, &*v84));
                let mut v158: Rc<str> = loop_1(v157);
                let v159: Option<usize> = v158.rfind(&*v84);
                let v160: Option<usize> = method2(v159);
                let mut v161: Option<i32> = v160.map(|x| v89(x));
                let v163: i32 = v161.unwrap_or(-1i32);
                let v164: bool = v163 < 0i32;
                let v168: bool = if v164 {
                    true
                } else {
                    let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v166: bool = v158.ends_with(&*v165);
                    let v167: bool = v166 == false;
                    v167
                };
                let v189: US1 = if v168 {
                    US1::US1_OtherFile
                } else {
                    let v170: usize = ((0i32) as usize);
                    let v171: usize = ((v163) as usize);
                    let v172: Rc<str> = Rc::<str>::from(&(v158)[v170..v171]);
                    let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v174: bool = v172.ends_with(&*v173);
                    if v174 {
                        let v175: Rc<str> = loop_3(v172);
                        US1::US1_StateFile(v175)
                    } else {
                        let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v178: bool = v172.ends_with(&*v177);
                        if v178 {
                            let v179: Rc<str> = loop_4(v172);
                            US1::US1_HistoryFile(v179)
                        } else {
                            let v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v182: bool = v172.ends_with(&*v181);
                            if v182 {
                                let v183: Rc<str> = loop_5(v172);
                                US1::US1_ArchiveFile(v183)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                let v212: US4 = match v189 {
                    US1::US1_ArchiveFile(v202) => {
                        let v202: Rc<str> = v202;
                        let mut v203: Rc<str> = string_concat(&[&*v202, "/state/spiral.toml"]);
                        let v204: bool = std::path::Path::new(&*v203).is_file();
                        if v204 {
                            US4::US4_SymbolsRoot
                        } else {
                            US4::US4_WordsRoot
                        }
                    }
                    US1::US1_HistoryFile(v196) => {
                        let v196: Rc<str> = v196;
                        let v197: Rc<str> = string_concat(&[&*v196, "/state/spiral.toml"]);
                        let v198: bool = std::path::Path::new(&*v197).is_file();
                        if v198 {
                            US4::US4_SymbolsRoot
                        } else {
                            US4::US4_WordsRoot
                        }
                    }
                    US1::US1_OtherFile => {
                        US4::US4_WordsRoot
                    }
                    US1::US1_StateFile(v190) => {
                        let v190: Rc<str> = v190;
                        let v191: Rc<str> = string_concat(&[&*v190, "/state/spiral.toml"]);
                        let v192: bool = std::path::Path::new(&*v191).is_file();
                        if v192 {
                            US4::US4_SymbolsRoot
                        } else {
                            US4::US4_WordsRoot
                        }
                    }
                };
                let v224: Rc<str> = match v212 {
                    US4::US4_SymbolsRoot => {
                        let v213: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v214: Rc<UH0> = split_list_6(v213.clone(), v3);
                        let v215: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v216: Rc<UH0> = method14(v214, v215);
                        let v217: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v218: i32 = 0i32;
                        let v219: i32 = method10(v217.clone(), v216, v218);
                        let v220: Rc<Vec<Rc<str>>> = Rc::new(v217.borrow().clone());
                        let v221: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v220).as_ref().clone()));
                        let v222: Rc<str> = Rc::<str>::from((*v221.borrow()).join(&*v213));
                        v222
                    }
                    US4::US4_WordsRoot => {
                        v3
                    }
                };
                let v225: bool = std::fs::read_to_string(&*v4).is_ok_and(|value| value == *v224);
                if v225 {
                    US6::US6_Replaced
                } else {
                    US6::US6_ReadbackMismatch
                }
            } else {
                let v229: bool = std::fs::remove_file(&*v82).is_ok() || !std::path::Path::new(&*v82).exists();
                if v229 {
                    US6::US6_RenameFailed
                } else {
                    US6::US6_RenameFailedTemporaryKept
                }
            }
        } else {
            US6::US6_TemporaryWriteFailed
        };
        match &v235 {
            US6::US6_Replaced => {
                module_registered_16(v0)
            }
            _ => {
                let v246: Rc<str> = match v235 {
                    US6::US6_ChangedSinceRead => {
                        let v241: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v241
                    }
                    US6::US6_ReadbackMismatch => {
                        let v240: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v240
                    }
                    US6::US6_RenameFailed => {
                        let v238: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v238
                    }
                    US6::US6_RenameFailedTemporaryKept => {
                        let v239: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v239
                    }
                    US6::US6_TemporaryWriteFailed => {
                        let v237: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v237
                    }
                    _ => unreachable!(),
                };
                US5::US5_StateWriteFailed(v246)
            }
        }
    };
    match v249 {
        US5::US5_StateChangedSinceRead => {
            let v251: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/claims.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
            v251
        }
        US5::US5_StateWriteFailed(v252) => {
            let v252: Rc<str> = v252;
            v252
        }
        US5::US5_StateWritten => {
            let v250: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v250
        }
    }
}
fn method26(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: u64 = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<UH1> = v6;
                let v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v1));
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn closure4() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method29(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure5() -> Rc<dyn Fn((u64)) -> US12> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US12> = Rc::new(move |mut v0: (u64)| -> US12 {
        let mut v1: u64 = (v0);
        US12::US12_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn globs_from_30(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().borrow().len() as i32);
    let v3: bool = v2 <= v1;
    if v3 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
        let mut v6: i32 = v1.wrapping_add(2i32);
        let v7: Rc<UH0> = globs_from_30(v0, v6);
        Rc::new(UH0::UH0_Cons(v5, v7))
    }
}
fn claim_of_row_28(mut v0: Rc<str>) -> US11 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 < 5i32;
    let mut v8: bool = if v4 {
        true
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl claim_"); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = v0.starts_with(&*v5);
        let v7: bool = v6 == false;
        v7
    };
    if v8 {
        US11::US11_None
    } else {
        let mut v10: Rc<str> = v2.borrow()[4i32 as usize].clone();
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Option<usize> = v10.find(&*v11);
        let mut v13: Option<usize> = method2(v12);
        let mut v14: Rc<dyn Fn((usize)) -> i32> = closure4();
        let mut v15: Option<i32> = v13.map(|x| v14(x));
        let mut v17: i32 = v15.unwrap_or(-1i32);
        let v18: bool = v17 > 2i32;
        let v25: Rc<str> = if v18 {
            let v19: i32 = v17.wrapping_sub(2i32);
            let v20: usize = ((2i32) as usize);
            let v21: i32 = 2i32.wrapping_add(v19);
            let v22: usize = ((v21) as usize);
            let v23: Rc<str> = Rc::<str>::from(&(v10)[v20..v22]);
            v23
        } else {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24
        };
        let mut v26: Option<u64> = (v25).trim().parse::<u64>().ok();
        let v27: Option<u64> = method29(v26);
        let mut v28: Rc<dyn Fn((u64)) -> US12> = closure5();
        let mut v29: Option<US12> = v27.map(|x| v28(x));
        let v31: US12 = US12::US12_None;
        let v32: US12 = v31.clone();
        let v33: US12 = v29.unwrap_or(v32);
        match v33 {
            US12::US12_None => {
                US11::US11_None
            }
            US12::US12_Some(v34) => {
                let v34: u64 = v34;
                let v35: Rc<str> = v2.borrow()[1i32 as usize].clone();
                let v36: Rc<str> = v2.borrow()[3i32 as usize].clone();
                let mut v37: i32 = 5i32;
                let v38: Rc<UH0> = globs_from_30(&v2, v37);
                US11::US11_Some(v35, v36, v34, v38)
            }
        }
    }
}
fn method27(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: US11 = claim_of_row_28(v2);
                let mut v11: Rc<UH1> = match v4 {
                    US11::US11_None => {
                        v1
                    }
                    US11::US11_Some(v5, v6, v7, v8) => {
                        let v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        let v7: u64 = v7;
                        let mut v8: Rc<UH0> = v8;
                        Rc::new(UH1::UH1_Cons(v5, v6, v7, v8, v1))
                    }
                };
                (v0, v1) = (v3, v11);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn claims_of_text_25(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_6(v1, v4);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH1> = method27(v5, v6);
    let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method26(v7, v8)
}
fn method31(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method31(v3, v1);
            let v5: bool = &*v2 != "";
            if v5 {
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
fn change_word_33(mut v0: US13) -> Rc<str> {
    let v4: Rc<str> = match v0 {
        US13::US13_ChangeNone => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US13::US13_ChangeRemove => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Remove"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US13::US13_ChangeUpsert => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Upsert"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    };
    let v5: Rc<str> = Rc::<str>::from(v4.to_lowercase());
    v5
}
fn change_of_32(mut v0: Rc<str>) -> US13 {
    let mut v1: US13 = US13::US13_ChangeUpsert;
    let mut v2: Rc<str> = change_word_33(v1);
    let v3: bool = &*v2 == &*v0;
    if v3 {
        US13::US13_ChangeUpsert
    } else {
        let v5: US13 = US13::US13_ChangeRemove;
        let mut v6: Rc<str> = change_word_33(v5);
        let v7: bool = &*v6 == &*v0;
        if v7 {
            US13::US13_ChangeRemove
        } else {
            let mut v9: US13 = US13::US13_ChangeNone;
            let mut v10: Rc<str> = change_word_33(v9);
            US13::US13_ChangeNone
        }
    }
}
fn method35(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: u64 = v5;
            let mut v6: Rc<UH0> = v6;
            let v7: Rc<UH1> = v7;
            let mut v8: Rc<UH1> = method35(v0.clone(), v7, v2);
            let mut v9: bool = &*v3 != &*v0;
            if v9 {
                Rc::new(UH1::UH1_Cons(v3, v4, v5, v6, v8))
            } else {
                v8
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn released_34(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method35(v1, v0, v2)
}
fn method37(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: u64 = v4;
            let v5: Rc<UH0> = v5;
            let mut v6: Rc<UH1> = v6;
            let v7: Rc<UH1> = method37(v6, v1);
            Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v7))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn decliner_39(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: &Rc<UH0>) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
    let v5: bool = v1.starts_with(&*v4);
    if v5 {
        let mut v6: i32 = (v1.clone().len() as i32);
        let v7: i32 = v6.wrapping_sub(1i32);
        let mut v8: usize = ((1i32) as usize);
        let mut v9: i32 = 1i32.wrapping_add(v7);
        let mut v10: usize = ((v9) as usize);
        let mut v11: Rc<str> = Rc::<str>::from(&(v1)[v8..v10]);
        v11
    } else {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    }
}
fn method38(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: &Rc<UH0>, mut v4: Rc<UH1>, mut v5: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v4) {
        UH1::UH1_Cons(v6, v7, v8, v9, v10) => {
            let mut v6: Rc<str> = v6;
            let v7: Rc<str> = v7;
            let mut v8: u64 = v8;
            let mut v9: Rc<UH0> = v9;
            let mut v10: Rc<UH1> = v10;
            let mut v11: Rc<UH1> = method38(v0.clone(), v1.clone(), v2, v3, v10, v5);
            let mut v12: bool = &*v6 != &*v0;
            let v22: bool = if v12 {
                true
            } else {
                let mut v13: Rc<str> = decliner_39(&v0, v1.clone(), v2, v3);
                let mut v14: bool = &*v13 != "";
                if v14 {
                    let mut v15: bool = &*v7 != &*v1;
                    if v15 {
                        let mut v16: Rc<str> = decliner_39(&v0, v1, v2, v3);
                        let mut v17: bool = &*v7 != &*v16;
                        v17
                    } else {
                        false
                    }
                } else {
                    let v19: Rc<str> = decliner_39(&v6, v7.clone(), v8, &v9);
                    let v20: bool = &*v19 != "";
                    v20
                }
            };
            if v22 {
                Rc::new(UH1::UH1_Cons(v6, v7, v8, v9, v11))
            } else {
                v11
            }
        }
        UH1::UH1_Nil => {
            v5
        }
    }
}
fn upserted_36(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: Rc<UH0>) -> Rc<UH1> {
    let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH1> = method38(v1.clone(), v2.clone(), v3, &v4, v0, v5);
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH1> = Rc::new(UH1::UH1_Cons(v1, v2, v3, v4, v7));
    method37(v6, v8)
}
fn method40(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: u64 = v5;
            let mut v6: Rc<UH0> = v6;
            let v7: Rc<UH1> = v7;
            let mut v8: Rc<UH1> = method40(v0.clone(), v7, v2);
            let mut v9: i32 = 0i32;
            let mut v10: Rc<dyn Fn(Rc<UH0>) -> US10> = method22(v3.clone(), v9);
            let mut v11: US10 = v10(v0);
            let mut v12: bool = match v11 {
                US10::US10_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let mut v13: bool = v12 != true;
            if v13 {
                Rc::new(UH1::UH1_Cons(v3, v4, v5, v6, v8))
            } else {
                v8
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn quoted_44(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let mut v8: Rc<str> = string_concat(&["\"", &*v6, "\""]);
    v8
}
fn method46(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_45(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method46(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn paths_term_47(mut v0: Rc<UH0>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<str> = quoted_44(v2);
            let v7: Rc<str> = paths_term_47(v3);
            let mut v9: Rc<str> = string_concat(&["PathThen (", &*v4, ", ", &*v7, ")"]);
            v9
        }
        UH0::UH0_Nil => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("NoPaths"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    }
}
fn claim_row_43(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH0>) -> Rc<str> {
    let v4: Rc<str> = decliner_39(&v0, v1.clone(), v2, &v3);
    let v5: bool = &*v4 == "";
    let mut v13: Rc<str> = if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        let v7: Rc<str> = decliner_39(&v0, v1.clone(), v2, &v3);
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v9));
        let mut v11: Rc<str> = Rc::<str>::from(v10.to_lowercase());
        let mut v12: Rc<str> = string_concat(&["__declined_", &*v11]);
        v12
    };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = Rc::<str>::from(v0.replace(&*v14, &*v15));
    let mut v17: Rc<str> = Rc::<str>::from(v16.to_lowercase());
    let v21: Rc<str> = quoted_44(v0);
    let v24: Rc<str> = quoted_44(v1);
    let v27: Rc<str> = format_real_45(v2);
    let v30: Rc<str> = paths_term_47(v3);
    let v32: Rc<str> = string_concat(&["inl claim_", &*v17, &*v13, " () = Claim (", &*v21, ", ", &*v24, ", ", &*v27, "u64, ", &*v30, ")"]);
    v32
}
fn method42(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: u64 = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<UH1> = v6;
                let v7: Rc<str> = claim_row_43(v2, v3, v4, v5);
                let mut v9: Rc<str> = string_concat(&[&*v1, &*v7, "\n"]);
                (v0, v1) = (v6, v9);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn claims_text_41(mut v0: Rc<UH1>) -> Rc<str> {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v2);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union rec claim_paths ="); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v3);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    | NoPaths :: claim_paths"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v4);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    | PathThen :: string * claim_paths -> claim_paths"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v5);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union agile_claim ="); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v6);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    | Claim :: string * string * u64 * claim_paths -> agile_claim"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v7);
    let mut v8: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
    let mut v12: Rc<str> = string_concat(&[&*v11, "\n"]);
    method42(v0, v12)
}
fn claims_applied_12(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>) -> Rc<str> {
    let mut v9: Rc<UH1> = claims_of_text_25(v2.clone());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<UH0> = split_list_6(v10, v8);
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
    let mut v13: bool = v5.starts_with(&*v12);
    let v19: Rc<UH0> = if v13 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<UH0> = split_list_6(v15, v7);
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method31(v16, v17)
    };
    let v20: US13 = change_of_32(v3.clone());
    let v25: Rc<UH1> = match v20 {
        US13::US13_ChangeNone => {
            v9
        }
        US13::US13_ChangeRemove => {
            released_34(v9, v4)
        }
        US13::US13_ChangeUpsert => {
            upserted_36(v9, v4, v5, v6, v19)
        }
    };
    let mut v26: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v27: Rc<UH1> = method40(v11, v25, v26);
    let mut v28: US13 = change_of_32(v3);
    let mut v29: bool = match v28 {
        US13::US13_ChangeNone => {
            true
        }
        _ => {
            false
        }
    };
    if v29 {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v30
    } else {
        let v31: Rc<str> = claims_text_41(v27);
        claims_saved_13(v0, v1, v2, v31)
    }
}
fn applied_claims_11(mut v0: Rc<RefCell<Vec<(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>)>>>) -> Rc<str> {
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9): (Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>) = v0.borrow()[0i32 as usize].clone();
    claims_applied_12(v1, v2, v3, v4, v5, v6, v7, v8, v9)
}
fn closure2() -> Rc<dyn Fn(Rc<RefCell<Vec<(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>)>>>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<RefCell<Vec<(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>)>>>) -> Rc<str>> = Rc::new(move |mut v0: Rc<RefCell<Vec<(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>)>>>| -> Rc<str> {
        applied_claims_11(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_claims_loaded(v0: &str) -> (bool, Rc<str>) {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_claims_applied(v0: Rc<RefCell<Vec<(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>)>>>) -> Rc<str> {
    closure2()(v0)
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
