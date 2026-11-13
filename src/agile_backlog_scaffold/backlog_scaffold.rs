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
enum US5 {
    US5_WordsDialect,
    US5_SymbolsDialect,
}
#[derive(Clone)]
enum US4 {
    US4_DialectChosen(US5),
    US4_DialectUnknown,
}
#[derive(Clone)]
enum US6 {
    US6_StatePackage,
    US6_StateToml,
    US6_BacklogSchema,
    US6_LeasePrompt,
}
#[derive(Clone)]
enum US7 {
    US7_DialectConverted(i32),
    US7_DialectAlready,
    US7_DialectRefused(Rc<str>),
}
#[derive(Clone)]
enum US8 {
    US8_SpiprojProject,
    US8_TomlProject,
}
#[derive(Clone)]
enum US9 {
    US9_FileUnchanged,
    US9_FileRewritten(Rc<str>, Rc<str>),
    US9_FileUnreadable(Rc<str>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US9, Rc<UH1>),
}
#[derive(Clone)]
enum US10 {
    US10_SplitProject(Rc<UH0>, Rc<UH0>, Rc<UH0>),
    US10_NoModules,
}
#[derive(Clone)]
enum US11 {
    US11_ProjectConverted(Rc<str>),
    US11_ProjectNotConvertible(Rc<str>),
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
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method8(v7, &v9) {
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
            let v15: US2 = if v6 {
                let mut v7: US3 = US3::US3_HeadType;
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
            let mut v39: US2 = match &v27 {
                US2::US2_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
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
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US3 = US3::US3_HeadOpen;
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
                    let mut v52: US3 = v52;
                    let v59: Rc<str> = match &v52 {
                        US3::US3_HeadInl => {
                            let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
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
                    let mut v71: usize = ((v60) as usize);
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
fn agile_init_entry_0() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let v3: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v2 ;
    if v3 {
        let v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let v5: Rc<str> = string_concat(&[&*v1, "/state/agile.spi"]);
        let mut v6: bool = std::path::Path::new(&*v5).exists();
        if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi already exists"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v7]);
            let mut v11: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v10 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v11
        } else {
            let mut v12: Rc<str> = string_concat(&[&*v1, "/state/package.spiproj"]);
            let v13: bool = std::path::Path::new(&*v12).exists();
            if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj already exists"); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v14]);
                let v18: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v17 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v18
            } else {
                let v19: Rc<str> = string_concat(&[&*v1, "/state/spiral.toml"]);
                let v20: bool = std::path::Path::new(&*v19).exists();
                if v20 {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/spiral.toml already exists"); } LIT.with(|lit| lit.clone()) };
                    let mut v24: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v21]);
                    let v25: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v24 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v25
                } else {
                    let v26: Rc<str> = string_concat(&[&*v1, "/state/prompt.spi"]);
                    let v27: bool = std::path::Path::new(&*v26).exists();
                    if v27 {
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/prompt.spi already exists"); } LIT.with(|lit| lit.clone()) };
                        let mut v31: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v28]);
                        let v32: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v31 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v32
                    } else {
                        let v33: Rc<str> = string_concat(&[&*v1, "/state"]);
                        let mut v35: Result<(), std::io::Error> = std::fs::create_dir_all(&*v33);
                        let v37: bool = v35.is_err();
                        let v38: bool = v37 == false;
                        if v38 {
                            let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n    agile\n    prompt\n"); } LIT.with(|lit| lit.clone()) };
                            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                            let v42: Rc<str> = Rc::<str>::from(v12.replace(&*v40, &*v41));
                            let mut v43: Rc<str> = loop_1(v42);
                            let v44: Option<usize> = v43.rfind(&*v41);
                            let mut v45: Option<usize> = method2(v44);
                            let mut v46: Rc<dyn Fn((usize)) -> i32> = closure1();
                            let mut v47: Option<i32> = v45.map(|x| v46(x));
                            let v49: i32 = v47.unwrap_or(-1i32);
                            let v50: bool = v49 < 0i32;
                            let v54: bool = if v50 {
                                true
                            } else {
                                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                let mut v52: bool = v43.ends_with(&*v51);
                                let mut v53: bool = v52 == false;
                                v53
                            };
                            let v75: US0 = if v54 {
                                US0::US0_OtherFile
                            } else {
                                let v56: usize = ((0i32) as usize);
                                let v57: usize = ((v49) as usize);
                                let v58: Rc<str> = Rc::<str>::from(&(v43)[v56..v57]);
                                let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                let v60: bool = v58.ends_with(&*v59);
                                if v60 {
                                    let v61: Rc<str> = loop_3(v58);
                                    US0::US0_StateFile(v61)
                                } else {
                                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                    let v64: bool = v58.ends_with(&*v63);
                                    if v64 {
                                        let v65: Rc<str> = loop_4(v58);
                                        US0::US0_HistoryFile(v65)
                                    } else {
                                        let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                        let v68: bool = v58.ends_with(&*v67);
                                        if v68 {
                                            let v69: Rc<str> = loop_5(v58);
                                            US0::US0_ArchiveFile(v69)
                                        } else {
                                            US0::US0_OtherFile
                                        }
                                    }
                                }
                            };
                            let v98: US1 = match v75 {
                                US0::US0_ArchiveFile(v88) => {
                                    let mut v88: Rc<str> = v88;
                                    let v89: Rc<str> = string_concat(&[&*v88, "/state/spiral.toml"]);
                                    let v90: bool = std::path::Path::new(&*v89).is_file();
                                    if v90 {
                                        US1::US1_SymbolsRoot
                                    } else {
                                        US1::US1_WordsRoot
                                    }
                                }
                                US0::US0_HistoryFile(v82) => {
                                    let v82: Rc<str> = v82;
                                    let v83: Rc<str> = string_concat(&[&*v82, "/state/spiral.toml"]);
                                    let v84: bool = std::path::Path::new(&*v83).is_file();
                                    if v84 {
                                        US1::US1_SymbolsRoot
                                    } else {
                                        US1::US1_WordsRoot
                                    }
                                }
                                US0::US0_OtherFile => {
                                    US1::US1_WordsRoot
                                }
                                US0::US0_StateFile(v76) => {
                                    let v76: Rc<str> = v76;
                                    let v77: Rc<str> = string_concat(&[&*v76, "/state/spiral.toml"]);
                                    let v78: bool = std::path::Path::new(&*v77).is_file();
                                    if v78 {
                                        US1::US1_SymbolsRoot
                                    } else {
                                        US1::US1_WordsRoot
                                    }
                                }
                            };
                            let v110: Rc<str> = match v98 {
                                US1::US1_SymbolsRoot => {
                                    let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    let v100: Rc<UH0> = split_list_6(v99.clone(), v39.clone());
                                    let v101: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v102: Rc<UH0> = method9(v100, v101);
                                    let v103: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                    let v104: i32 = 0i32;
                                    let v105: i32 = method10(v103.clone(), v102, v104);
                                    let v106: Rc<Vec<Rc<str>>> = Rc::new(v103.borrow().clone());
                                    let v107: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v106).as_ref().clone()));
                                    let v108: Rc<str> = Rc::<str>::from((*v107.borrow()).join(&*v99));
                                    v108
                                }
                                US1::US1_WordsRoot => {
                                    v39.clone()
                                }
                            };
                            let v111: bool = std::fs::write(&*v12, &*v110).is_ok();
                            if v111 {
                                let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\nunion task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status | Dropped :: task_status\n\nunion agile_item =\n    | Task :: string * string * task_kind * priority * difficulty * u32 * task_status * string * string * string * string -> agile_item\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                                let v113: Rc<str> = Rc::<str>::from(v5.replace(&*v40, &*v41));
                                let mut v114: Rc<str> = loop_1(v113);
                                let v115: Option<usize> = v114.rfind(&*v41);
                                let v116: Option<usize> = method2(v115);
                                let mut v117: Option<i32> = v116.map(|x| v46(x));
                                let v119: i32 = v117.unwrap_or(-1i32);
                                let v120: bool = v119 < 0i32;
                                let v124: bool = if v120 {
                                    true
                                } else {
                                    let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                    let v122: bool = v114.ends_with(&*v121);
                                    let v123: bool = v122 == false;
                                    v123
                                };
                                let v145: US0 = if v124 {
                                    US0::US0_OtherFile
                                } else {
                                    let v126: usize = ((0i32) as usize);
                                    let v127: usize = ((v119) as usize);
                                    let v128: Rc<str> = Rc::<str>::from(&(v114)[v126..v127]);
                                    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                    let v130: bool = v128.ends_with(&*v129);
                                    if v130 {
                                        let v131: Rc<str> = loop_3(v128);
                                        US0::US0_StateFile(v131)
                                    } else {
                                        let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                        let v134: bool = v128.ends_with(&*v133);
                                        if v134 {
                                            let v135: Rc<str> = loop_4(v128);
                                            US0::US0_HistoryFile(v135)
                                        } else {
                                            let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                            let v138: bool = v128.ends_with(&*v137);
                                            if v138 {
                                                let v139: Rc<str> = loop_5(v128);
                                                US0::US0_ArchiveFile(v139)
                                            } else {
                                                US0::US0_OtherFile
                                            }
                                        }
                                    }
                                };
                                let v168: US1 = match v145 {
                                    US0::US0_ArchiveFile(v158) => {
                                        let v158: Rc<str> = v158;
                                        let v159: Rc<str> = string_concat(&[&*v158, "/state/spiral.toml"]);
                                        let v160: bool = std::path::Path::new(&*v159).is_file();
                                        if v160 {
                                            US1::US1_SymbolsRoot
                                        } else {
                                            US1::US1_WordsRoot
                                        }
                                    }
                                    US0::US0_HistoryFile(v152) => {
                                        let v152: Rc<str> = v152;
                                        let v153: Rc<str> = string_concat(&[&*v152, "/state/spiral.toml"]);
                                        let v154: bool = std::path::Path::new(&*v153).is_file();
                                        if v154 {
                                            US1::US1_SymbolsRoot
                                        } else {
                                            US1::US1_WordsRoot
                                        }
                                    }
                                    US0::US0_OtherFile => {
                                        US1::US1_WordsRoot
                                    }
                                    US0::US0_StateFile(v146) => {
                                        let mut v146: Rc<str> = v146;
                                        let v147: Rc<str> = string_concat(&[&*v146, "/state/spiral.toml"]);
                                        let v148: bool = std::path::Path::new(&*v147).is_file();
                                        if v148 {
                                            US1::US1_SymbolsRoot
                                        } else {
                                            US1::US1_WordsRoot
                                        }
                                    }
                                };
                                let v180: Rc<str> = match v168 {
                                    US1::US1_SymbolsRoot => {
                                        let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                        let v170: Rc<UH0> = split_list_6(v169.clone(), v112.clone());
                                        let v171: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                        let mut v172: Rc<UH0> = method9(v170, v171);
                                        let v173: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                        let v174: i32 = 0i32;
                                        let v175: i32 = method10(v173.clone(), v172, v174);
                                        let v176: Rc<Vec<Rc<str>>> = Rc::new(v173.borrow().clone());
                                        let v177: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v176).as_ref().clone()));
                                        let v178: Rc<str> = Rc::<str>::from((*v177.borrow()).join(&*v169));
                                        v178
                                    }
                                    US1::US1_WordsRoot => {
                                        v112.clone()
                                    }
                                };
                                let v181: bool = std::fs::write(&*v5, &*v180).is_ok();
                                if v181 {
                                    let v182: u64 = v4.wrapping_sub(5100000u64);
                                    let v183: Rc<str> = std::rc::Rc::<str>::from((v182).to_string());
                                    let v186: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
                                    let mut v189: Rc<str> = string_concat(&["///- --no-respell\nunion lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\ninl current () : prompt_lease = PromptLease (\"init\", \"scaffolded by agile init; agile begin opens a lease\", 4500000u64, 600000u64, ", &*v183, "u64, ", &*v186, "u64, LeaseActive)\ninl main () : i32 = 0i32\n"]);
                                    let v190: Rc<str> = Rc::<str>::from(v26.replace(&*v40, &*v41));
                                    let mut v191: Rc<str> = loop_1(v190);
                                    let v192: Option<usize> = v191.rfind(&*v41);
                                    let v193: Option<usize> = method2(v192);
                                    let mut v194: Option<i32> = v193.map(|x| v46(x));
                                    let v196: i32 = v194.unwrap_or(-1i32);
                                    let mut v197: bool = v196 < 0i32;
                                    let v201: bool = if v197 {
                                        true
                                    } else {
                                        let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                        let v199: bool = v191.ends_with(&*v198);
                                        let v200: bool = v199 == false;
                                        v200
                                    };
                                    let v222: US0 = if v201 {
                                        US0::US0_OtherFile
                                    } else {
                                        let v203: usize = ((0i32) as usize);
                                        let v204: usize = ((v196) as usize);
                                        let mut v205: Rc<str> = Rc::<str>::from(&(v191)[v203..v204]);
                                        let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                        let v207: bool = v205.ends_with(&*v206);
                                        if v207 {
                                            let v208: Rc<str> = loop_3(v205);
                                            US0::US0_StateFile(v208)
                                        } else {
                                            let v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                            let v211: bool = v205.ends_with(&*v210);
                                            if v211 {
                                                let v212: Rc<str> = loop_4(v205);
                                                US0::US0_HistoryFile(v212)
                                            } else {
                                                let v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                let v215: bool = v205.ends_with(&*v214);
                                                if v215 {
                                                    let v216: Rc<str> = loop_5(v205);
                                                    US0::US0_ArchiveFile(v216)
                                                } else {
                                                    US0::US0_OtherFile
                                                }
                                            }
                                        }
                                    };
                                    let v245: US1 = match v222 {
                                        US0::US0_ArchiveFile(v235) => {
                                            let v235: Rc<str> = v235;
                                            let v236: Rc<str> = string_concat(&[&*v235, "/state/spiral.toml"]);
                                            let v237: bool = std::path::Path::new(&*v236).is_file();
                                            if v237 {
                                                US1::US1_SymbolsRoot
                                            } else {
                                                US1::US1_WordsRoot
                                            }
                                        }
                                        US0::US0_HistoryFile(v229) => {
                                            let v229: Rc<str> = v229;
                                            let v230: Rc<str> = string_concat(&[&*v229, "/state/spiral.toml"]);
                                            let v231: bool = std::path::Path::new(&*v230).is_file();
                                            if v231 {
                                                US1::US1_SymbolsRoot
                                            } else {
                                                US1::US1_WordsRoot
                                            }
                                        }
                                        US0::US0_OtherFile => {
                                            US1::US1_WordsRoot
                                        }
                                        US0::US0_StateFile(v223) => {
                                            let v223: Rc<str> = v223;
                                            let v224: Rc<str> = string_concat(&[&*v223, "/state/spiral.toml"]);
                                            let v225: bool = std::path::Path::new(&*v224).is_file();
                                            if v225 {
                                                US1::US1_SymbolsRoot
                                            } else {
                                                US1::US1_WordsRoot
                                            }
                                        }
                                    };
                                    let v257: Rc<str> = match v245 {
                                        US1::US1_SymbolsRoot => {
                                            let v246: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                            let v247: Rc<UH0> = split_list_6(v246.clone(), v189);
                                            let v248: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                            let v249: Rc<UH0> = method9(v247, v248);
                                            let v250: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                            let v251: i32 = 0i32;
                                            let v252: i32 = method10(v250.clone(), v249, v251);
                                            let v253: Rc<Vec<Rc<str>>> = Rc::new(v250.borrow().clone());
                                            let v254: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v253).as_ref().clone()));
                                            let v255: Rc<str> = Rc::<str>::from((*v254.borrow()).join(&*v246));
                                            v255
                                        }
                                        US1::US1_WordsRoot => {
                                            v189
                                        }
                                    };
                                    let v258: bool = std::fs::write(&*v26, &*v257).is_ok();
                                    if v258 {
                                        let v259: u64 = v39.len() as u64;
                                        let v260: u64 = v112.len() as u64;
                                        let v261: Rc<str> = std::rc::Rc::<str>::from((v182).to_string());
                                        let v264: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
                                        let v267: Rc<str> = string_concat(&["///- --no-respell\nunion lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\ninl current () : prompt_lease = PromptLease (\"init\", \"scaffolded by agile init; agile begin opens a lease\", 4500000u64, 600000u64, ", &*v261, "u64, ", &*v264, "u64, LeaseActive)\ninl main () : i32 = 0i32\n"]);
                                        let v268: u64 = v267.len() as u64;
                                        let v269: u64 = v259 + v260 + v268;
                                        let v270: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
                                        let v273: Rc<str> = std::rc::Rc::<str>::from((v269).to_string());
                                        let v276: Rc<str> = std::rc::Rc::<str>::from((v270).to_string());
                                        let v278: Rc<str> = string_concat(&["rig agile init ok root=", &*v1, " files=3 bytes=", &*v273, " elapsed_ms=", &*v276, " next=kit agile begin <root> <title>"]);
                                        let v279: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v278.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                        v279
                                    } else {
                                        let v280: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/prompt.spi"); } LIT.with(|lit| lit.clone()) };
                                        let v283: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v280]);
                                        let v284: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v283 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                        v284
                                    }
                                } else {
                                    let v286: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                                    let v289: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v286]);
                                    let v290: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v289 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v290
                                }
                            } else {
                                let v292: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the state project"); } LIT.with(|lit| lit.clone()) };
                                let v295: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v292]);
                                let v296: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v295 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v296
                            }
                        } else {
                            let v298: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not create the state directory"); } LIT.with(|lit| lit.clone()) };
                            let v301: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v298]);
                            let v302: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v301 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v302
                        }
                    }
                }
            }
        }
    } else {
        let v308: i32 = 4i32;
        let v309: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v308 ;
        let v313: bool = if v309 {
            let v310: i32 = 2i32;
            let v311: Rc<str> = usize::try_from(v310).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v312: bool = &*v311 == "--dialect";
            v312
        } else {
            false
        };
        if v313 {
            let v314: i32 = 3i32;
            let v315: Rc<str> = usize::try_from(v314).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v316: bool = &*v315 == "symbols";
            let v324: US4 = if v316 {
                let v317: US5 = US5::US5_SymbolsDialect;
                US4::US4_DialectChosen(v317)
            } else {
                let v319: bool = &*v315 == "words";
                if v319 {
                    let v320: US5 = US5::US5_WordsDialect;
                    US4::US4_DialectChosen(v320)
                } else {
                    US4::US4_DialectUnknown
                }
            };
            match v324 {
                US4::US4_DialectChosen(v325) => {
                    let v325: US5 = v325;
                    let v326: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                    let v330: US6 = match v325 {
                        US5::US5_SymbolsDialect => {
                            US6::US6_StateToml
                        }
                        US5::US5_WordsDialect => {
                            US6::US6_StatePackage
                        }
                    };
                    let v331: Rc<str> = string_concat(&[&*v1, "/state/agile.spi"]);
                    let v332: bool = std::path::Path::new(&*v331).exists();
                    if v332 {
                        let v333: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi already exists"); } LIT.with(|lit| lit.clone()) };
                        let v336: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v333]);
                        let v337: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v336 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v337
                    } else {
                        let v338: Rc<str> = string_concat(&[&*v1, "/state/package.spiproj"]);
                        let v339: bool = std::path::Path::new(&*v338).exists();
                        if v339 {
                            let v340: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj already exists"); } LIT.with(|lit| lit.clone()) };
                            let v343: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v340]);
                            let v344: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v343 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v344
                        } else {
                            let v345: Rc<str> = string_concat(&[&*v1, "/state/spiral.toml"]);
                            let v346: bool = std::path::Path::new(&*v345).exists();
                            if v346 {
                                let v347: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/spiral.toml already exists"); } LIT.with(|lit| lit.clone()) };
                                let v350: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v347]);
                                let v351: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v350 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v351
                            } else {
                                let v352: Rc<str> = string_concat(&[&*v1, "/state/prompt.spi"]);
                                let v353: bool = std::path::Path::new(&*v352).exists();
                                if v353 {
                                    let v354: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/prompt.spi already exists"); } LIT.with(|lit| lit.clone()) };
                                    let v357: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v354]);
                                    let v358: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v357 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v358
                                } else {
                                    let v359: Rc<str> = string_concat(&[&*v1, "/state"]);
                                    let mut v361: Result<(), std::io::Error> = std::fs::create_dir_all(&*v359);
                                    let v363: bool = v361.is_err();
                                    let v364: bool = v363 == false;
                                    if v364 {
                                        let v372: Rc<str> = match &v330 {
                                            US6::US6_BacklogSchema => {
                                                let v367: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                                                v367
                                            }
                                            US6::US6_LeasePrompt => {
                                                let v368: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/prompt.spi"); } LIT.with(|lit| lit.clone()) };
                                                v368
                                            }
                                            US6::US6_StatePackage => {
                                                let v365: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/package.spiproj"); } LIT.with(|lit| lit.clone()) };
                                                v365
                                            }
                                            US6::US6_StateToml => {
                                                let v366: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/spiral.toml"); } LIT.with(|lit| lit.clone()) };
                                                v366
                                            }
                                        };
                                        let v373: Rc<str> = string_concat(&[&*v1, &*v372]);
                                        let v388: Rc<str> = match &v330 {
                                            US6::US6_BacklogSchema => {
                                                let v376: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\nunion task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status | Dropped :: task_status\n\nunion agile_item =\n    | Task :: string * string * task_kind * priority * difficulty * u32 * task_status * string * string * string * string -> agile_item\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                                                v376
                                            }
                                            US6::US6_LeasePrompt => {
                                                let v377: u64 = v326.wrapping_sub(5100000u64);
                                                let v378: Rc<str> = std::rc::Rc::<str>::from((v377).to_string());
                                                let v381: Rc<str> = std::rc::Rc::<str>::from((v326).to_string());
                                                let v384: Rc<str> = string_concat(&["///- --no-respell\nunion lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\ninl current () : prompt_lease = PromptLease (\"init\", \"scaffolded by agile init; agile begin opens a lease\", 4500000u64, 600000u64, ", &*v378, "u64, ", &*v381, "u64, LeaseActive)\ninl main () : i32 = 0i32\n"]);
                                                v384
                                            }
                                            US6::US6_StatePackage => {
                                                let v374: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n    agile\n    prompt\n"); } LIT.with(|lit| lit.clone()) };
                                                v374
                                            }
                                            US6::US6_StateToml => {
                                                let v375: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''\nagile\nprompt\n'''\n"); } LIT.with(|lit| lit.clone()) };
                                                v375
                                            }
                                        };
                                        let v389: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                        let v390: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                                        let v391: Rc<str> = Rc::<str>::from(v373.replace(&*v389, &*v390));
                                        let mut v392: Rc<str> = loop_1(v391);
                                        let v393: Option<usize> = v392.rfind(&*v390);
                                        let v394: Option<usize> = method2(v393);
                                        let mut v395: Rc<dyn Fn((usize)) -> i32> = closure1();
                                        let mut v396: Option<i32> = v394.map(|x| v395(x));
                                        let v398: i32 = v396.unwrap_or(-1i32);
                                        let v399: bool = v398 < 0i32;
                                        let v403: bool = if v399 {
                                            true
                                        } else {
                                            let v400: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                            let v401: bool = v392.ends_with(&*v400);
                                            let v402: bool = v401 == false;
                                            v402
                                        };
                                        let v424: US0 = if v403 {
                                            US0::US0_OtherFile
                                        } else {
                                            let v405: usize = ((0i32) as usize);
                                            let v406: usize = ((v398) as usize);
                                            let v407: Rc<str> = Rc::<str>::from(&(v392)[v405..v406]);
                                            let v408: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                            let v409: bool = v407.ends_with(&*v408);
                                            if v409 {
                                                let v410: Rc<str> = loop_3(v407);
                                                US0::US0_StateFile(v410)
                                            } else {
                                                let v412: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                let v413: bool = v407.ends_with(&*v412);
                                                if v413 {
                                                    let v414: Rc<str> = loop_4(v407);
                                                    US0::US0_HistoryFile(v414)
                                                } else {
                                                    let v416: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                    let v417: bool = v407.ends_with(&*v416);
                                                    if v417 {
                                                        let v418: Rc<str> = loop_5(v407);
                                                        US0::US0_ArchiveFile(v418)
                                                    } else {
                                                        US0::US0_OtherFile
                                                    }
                                                }
                                            }
                                        };
                                        let v447: US1 = match v424 {
                                            US0::US0_ArchiveFile(v437) => {
                                                let v437: Rc<str> = v437;
                                                let v438: Rc<str> = string_concat(&[&*v437, "/state/spiral.toml"]);
                                                let v439: bool = std::path::Path::new(&*v438).is_file();
                                                if v439 {
                                                    US1::US1_SymbolsRoot
                                                } else {
                                                    US1::US1_WordsRoot
                                                }
                                            }
                                            US0::US0_HistoryFile(v431) => {
                                                let v431: Rc<str> = v431;
                                                let v432: Rc<str> = string_concat(&[&*v431, "/state/spiral.toml"]);
                                                let v433: bool = std::path::Path::new(&*v432).is_file();
                                                if v433 {
                                                    US1::US1_SymbolsRoot
                                                } else {
                                                    US1::US1_WordsRoot
                                                }
                                            }
                                            US0::US0_OtherFile => {
                                                US1::US1_WordsRoot
                                            }
                                            US0::US0_StateFile(v425) => {
                                                let v425: Rc<str> = v425;
                                                let v426: Rc<str> = string_concat(&[&*v425, "/state/spiral.toml"]);
                                                let v427: bool = std::path::Path::new(&*v426).is_file();
                                                if v427 {
                                                    US1::US1_SymbolsRoot
                                                } else {
                                                    US1::US1_WordsRoot
                                                }
                                            }
                                        };
                                        let v459: Rc<str> = match v447 {
                                            US1::US1_SymbolsRoot => {
                                                let v448: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                let v449: Rc<UH0> = split_list_6(v448.clone(), v388);
                                                let v450: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                let v451: Rc<UH0> = method9(v449, v450);
                                                let v452: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                let v453: i32 = 0i32;
                                                let v454: i32 = method10(v452.clone(), v451, v453);
                                                let v455: Rc<Vec<Rc<str>>> = Rc::new(v452.borrow().clone());
                                                let v456: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v455).as_ref().clone()));
                                                let v457: Rc<str> = Rc::<str>::from((*v456.borrow()).join(&*v448));
                                                v457
                                            }
                                            US1::US1_WordsRoot => {
                                                v388
                                            }
                                        };
                                        let v460: bool = std::fs::write(&*v373, &*v459).is_ok();
                                        if v460 {
                                            let v461: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\nunion task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status | Dropped :: task_status\n\nunion agile_item =\n    | Task :: string * string * task_kind * priority * difficulty * u32 * task_status * string * string * string * string -> agile_item\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                                            let v462: Rc<str> = Rc::<str>::from(v331.replace(&*v389, &*v390));
                                            let mut v463: Rc<str> = loop_1(v462);
                                            let v464: Option<usize> = v463.rfind(&*v390);
                                            let v465: Option<usize> = method2(v464);
                                            let mut v466: Option<i32> = v465.map(|x| v395(x));
                                            let v468: i32 = v466.unwrap_or(-1i32);
                                            let v469: bool = v468 < 0i32;
                                            let v473: bool = if v469 {
                                                true
                                            } else {
                                                let v470: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                let v471: bool = v463.ends_with(&*v470);
                                                let v472: bool = v471 == false;
                                                v472
                                            };
                                            let v494: US0 = if v473 {
                                                US0::US0_OtherFile
                                            } else {
                                                let v475: usize = ((0i32) as usize);
                                                let v476: usize = ((v468) as usize);
                                                let v477: Rc<str> = Rc::<str>::from(&(v463)[v475..v476]);
                                                let v478: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                let v479: bool = v477.ends_with(&*v478);
                                                if v479 {
                                                    let v480: Rc<str> = loop_3(v477);
                                                    US0::US0_StateFile(v480)
                                                } else {
                                                    let v482: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                    let v483: bool = v477.ends_with(&*v482);
                                                    if v483 {
                                                        let v484: Rc<str> = loop_4(v477);
                                                        US0::US0_HistoryFile(v484)
                                                    } else {
                                                        let v486: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                        let v487: bool = v477.ends_with(&*v486);
                                                        if v487 {
                                                            let v488: Rc<str> = loop_5(v477);
                                                            US0::US0_ArchiveFile(v488)
                                                        } else {
                                                            US0::US0_OtherFile
                                                        }
                                                    }
                                                }
                                            };
                                            let v517: US1 = match v494 {
                                                US0::US0_ArchiveFile(v507) => {
                                                    let v507: Rc<str> = v507;
                                                    let v508: Rc<str> = string_concat(&[&*v507, "/state/spiral.toml"]);
                                                    let v509: bool = std::path::Path::new(&*v508).is_file();
                                                    if v509 {
                                                        US1::US1_SymbolsRoot
                                                    } else {
                                                        US1::US1_WordsRoot
                                                    }
                                                }
                                                US0::US0_HistoryFile(v501) => {
                                                    let v501: Rc<str> = v501;
                                                    let v502: Rc<str> = string_concat(&[&*v501, "/state/spiral.toml"]);
                                                    let v503: bool = std::path::Path::new(&*v502).is_file();
                                                    if v503 {
                                                        US1::US1_SymbolsRoot
                                                    } else {
                                                        US1::US1_WordsRoot
                                                    }
                                                }
                                                US0::US0_OtherFile => {
                                                    US1::US1_WordsRoot
                                                }
                                                US0::US0_StateFile(v495) => {
                                                    let v495: Rc<str> = v495;
                                                    let v496: Rc<str> = string_concat(&[&*v495, "/state/spiral.toml"]);
                                                    let v497: bool = std::path::Path::new(&*v496).is_file();
                                                    if v497 {
                                                        US1::US1_SymbolsRoot
                                                    } else {
                                                        US1::US1_WordsRoot
                                                    }
                                                }
                                            };
                                            let v529: Rc<str> = match v517 {
                                                US1::US1_SymbolsRoot => {
                                                    let v518: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                    let v519: Rc<UH0> = split_list_6(v518.clone(), v461.clone());
                                                    let v520: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                    let v521: Rc<UH0> = method9(v519, v520);
                                                    let v522: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                    let v523: i32 = 0i32;
                                                    let v524: i32 = method10(v522.clone(), v521, v523);
                                                    let v525: Rc<Vec<Rc<str>>> = Rc::new(v522.borrow().clone());
                                                    let v526: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v525).as_ref().clone()));
                                                    let v527: Rc<str> = Rc::<str>::from((*v526.borrow()).join(&*v518));
                                                    v527
                                                }
                                                US1::US1_WordsRoot => {
                                                    v461.clone()
                                                }
                                            };
                                            let v530: bool = std::fs::write(&*v331, &*v529).is_ok();
                                            if v530 {
                                                let v531: u64 = v326.wrapping_sub(5100000u64);
                                                let v532: Rc<str> = std::rc::Rc::<str>::from((v531).to_string());
                                                let v535: Rc<str> = std::rc::Rc::<str>::from((v326).to_string());
                                                let v538: Rc<str> = string_concat(&["///- --no-respell\nunion lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\ninl current () : prompt_lease = PromptLease (\"init\", \"scaffolded by agile init; agile begin opens a lease\", 4500000u64, 600000u64, ", &*v532, "u64, ", &*v535, "u64, LeaseActive)\ninl main () : i32 = 0i32\n"]);
                                                let v539: Rc<str> = Rc::<str>::from(v352.replace(&*v389, &*v390));
                                                let mut v540: Rc<str> = loop_1(v539);
                                                let v541: Option<usize> = v540.rfind(&*v390);
                                                let v542: Option<usize> = method2(v541);
                                                let mut v543: Option<i32> = v542.map(|x| v395(x));
                                                let v545: i32 = v543.unwrap_or(-1i32);
                                                let v546: bool = v545 < 0i32;
                                                let v550: bool = if v546 {
                                                    true
                                                } else {
                                                    let v547: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                    let v548: bool = v540.ends_with(&*v547);
                                                    let v549: bool = v548 == false;
                                                    v549
                                                };
                                                let v571: US0 = if v550 {
                                                    US0::US0_OtherFile
                                                } else {
                                                    let v552: usize = ((0i32) as usize);
                                                    let v553: usize = ((v545) as usize);
                                                    let v554: Rc<str> = Rc::<str>::from(&(v540)[v552..v553]);
                                                    let v555: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                    let v556: bool = v554.ends_with(&*v555);
                                                    if v556 {
                                                        let v557: Rc<str> = loop_3(v554);
                                                        US0::US0_StateFile(v557)
                                                    } else {
                                                        let v559: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                        let v560: bool = v554.ends_with(&*v559);
                                                        if v560 {
                                                            let v561: Rc<str> = loop_4(v554);
                                                            US0::US0_HistoryFile(v561)
                                                        } else {
                                                            let v563: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                            let v564: bool = v554.ends_with(&*v563);
                                                            if v564 {
                                                                let v565: Rc<str> = loop_5(v554);
                                                                US0::US0_ArchiveFile(v565)
                                                            } else {
                                                                US0::US0_OtherFile
                                                            }
                                                        }
                                                    }
                                                };
                                                let v594: US1 = match v571 {
                                                    US0::US0_ArchiveFile(v584) => {
                                                        let v584: Rc<str> = v584;
                                                        let v585: Rc<str> = string_concat(&[&*v584, "/state/spiral.toml"]);
                                                        let v586: bool = std::path::Path::new(&*v585).is_file();
                                                        if v586 {
                                                            US1::US1_SymbolsRoot
                                                        } else {
                                                            US1::US1_WordsRoot
                                                        }
                                                    }
                                                    US0::US0_HistoryFile(v578) => {
                                                        let v578: Rc<str> = v578;
                                                        let v579: Rc<str> = string_concat(&[&*v578, "/state/spiral.toml"]);
                                                        let v580: bool = std::path::Path::new(&*v579).is_file();
                                                        if v580 {
                                                            US1::US1_SymbolsRoot
                                                        } else {
                                                            US1::US1_WordsRoot
                                                        }
                                                    }
                                                    US0::US0_OtherFile => {
                                                        US1::US1_WordsRoot
                                                    }
                                                    US0::US0_StateFile(v572) => {
                                                        let v572: Rc<str> = v572;
                                                        let v573: Rc<str> = string_concat(&[&*v572, "/state/spiral.toml"]);
                                                        let v574: bool = std::path::Path::new(&*v573).is_file();
                                                        if v574 {
                                                            US1::US1_SymbolsRoot
                                                        } else {
                                                            US1::US1_WordsRoot
                                                        }
                                                    }
                                                };
                                                let v606: Rc<str> = match v594 {
                                                    US1::US1_SymbolsRoot => {
                                                        let v595: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                        let v596: Rc<UH0> = split_list_6(v595.clone(), v538);
                                                        let v597: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                        let v598: Rc<UH0> = method9(v596, v597);
                                                        let v599: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                        let v600: i32 = 0i32;
                                                        let v601: i32 = method10(v599.clone(), v598, v600);
                                                        let v602: Rc<Vec<Rc<str>>> = Rc::new(v599.borrow().clone());
                                                        let v603: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v602).as_ref().clone()));
                                                        let v604: Rc<str> = Rc::<str>::from((*v603.borrow()).join(&*v595));
                                                        v604
                                                    }
                                                    US1::US1_WordsRoot => {
                                                        v538
                                                    }
                                                };
                                                let v607: bool = std::fs::write(&*v352, &*v606).is_ok();
                                                if v607 {
                                                    let v620: Rc<str> = match v330 {
                                                        US6::US6_BacklogSchema => {
                                                            v461.clone()
                                                        }
                                                        US6::US6_LeasePrompt => {
                                                            let v610: Rc<str> = std::rc::Rc::<str>::from((v531).to_string());
                                                            let v613: Rc<str> = std::rc::Rc::<str>::from((v326).to_string());
                                                            let v616: Rc<str> = string_concat(&["///- --no-respell\nunion lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\ninl current () : prompt_lease = PromptLease (\"init\", \"scaffolded by agile init; agile begin opens a lease\", 4500000u64, 600000u64, ", &*v610, "u64, ", &*v613, "u64, LeaseActive)\ninl main () : i32 = 0i32\n"]);
                                                            v616
                                                        }
                                                        US6::US6_StatePackage => {
                                                            let v608: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n    agile\n    prompt\n"); } LIT.with(|lit| lit.clone()) };
                                                            v608
                                                        }
                                                        US6::US6_StateToml => {
                                                            let v609: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''\nagile\nprompt\n'''\n"); } LIT.with(|lit| lit.clone()) };
                                                            v609
                                                        }
                                                    };
                                                    let v621: u64 = v620.len() as u64;
                                                    let v622: u64 = v461.len() as u64;
                                                    let v623: Rc<str> = std::rc::Rc::<str>::from((v531).to_string());
                                                    let v626: Rc<str> = std::rc::Rc::<str>::from((v326).to_string());
                                                    let v629: Rc<str> = string_concat(&["///- --no-respell\nunion lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\ninl current () : prompt_lease = PromptLease (\"init\", \"scaffolded by agile init; agile begin opens a lease\", 4500000u64, 600000u64, ", &*v623, "u64, ", &*v626, "u64, LeaseActive)\ninl main () : i32 = 0i32\n"]);
                                                    let v630: u64 = v629.len() as u64;
                                                    let v631: u64 = v621 + v622 + v630;
                                                    let v632: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v326);
                                                    let v635: Rc<str> = std::rc::Rc::<str>::from((v631).to_string());
                                                    let v638: Rc<str> = std::rc::Rc::<str>::from((v632).to_string());
                                                    let v640: Rc<str> = string_concat(&["rig agile init ok root=", &*v1, " files=3 bytes=", &*v635, " elapsed_ms=", &*v638, " next=kit agile begin <root> <title>"]);
                                                    let v641: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v640.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                                    v641
                                                } else {
                                                    let v642: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/prompt.spi"); } LIT.with(|lit| lit.clone()) };
                                                    let v645: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v642]);
                                                    let v646: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v645 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                    v646
                                                }
                                            } else {
                                                let v648: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                                                let v651: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v648]);
                                                let v652: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v651 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                v652
                                            }
                                        } else {
                                            let v654: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the state project"); } LIT.with(|lit| lit.clone()) };
                                            let v657: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v654]);
                                            let v658: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v657 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                            v658
                                        }
                                    } else {
                                        let v660: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not create the state directory"); } LIT.with(|lit| lit.clone()) };
                                        let v663: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v660]);
                                        let v664: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v663 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                        v664
                                    }
                                }
                            }
                        }
                    }
                }
                US4::US4_DialectUnknown => {
                    let v670: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dialect takes symbols or words"); } LIT.with(|lit| lit.clone()) };
                    let v673: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v670]);
                    let v674: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v673 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v674
                }
            }
        } else {
            let v677: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile init <root> [--dialect symbols|words]"); } LIT.with(|lit| lit.clone()) };
            let v680: Rc<str> = string_concat(&["agile init ", &*v1, " rejected: ", &*v677]);
            let v681: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v680 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v681
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_init_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method15(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn directory_exists_17(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn names_in_16(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: bool = directory_exists_17(v0.clone());
    let mut v2: bool = v1 == false;
    if v2 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v5: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v7: Vec<std::string::String> = v5.unwrap();
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v10: bool = true; let _vec_fold_ = v7.into_iter().fold(v8, |acc, x| { //;
        let mut v12: Rc<UH0> = acc;
        let v14: std::string::String = x;
        let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
        let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v12));
        let v19: bool = true; v18 });
        let v21: Rc<UH0> = _vec_fold_;
        v21
    }
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method18(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.ends_with(&*v5);
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
fn method19(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method19(v0.clone(), v4, v2);
            let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v6.borrow_mut().push(v0);
            v6.borrow_mut().push(v3);
            let mut v7: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
            let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from((*v8.borrow()).join(&*v9));
            Rc::new(UH0::UH0_Cons(v10, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn spi_files_14(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v1.borrow_mut().push(v0.clone());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v2);
    let v3: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v3).as_ref().clone()));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from((*v4.borrow()).join(&*v5));
    let mut v7: Rc<UH0> = names_in_16(v6.clone());
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method18(v7, v8);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v11: Rc<UH0> = method19(v6, v9, v10);
    let mut v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v12.borrow_mut().push(v0.clone());
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("history"); } LIT.with(|lit| lit.clone()) };
    v12.borrow_mut().push(v13);
    let v14: Rc<Vec<Rc<str>>> = Rc::new(v12.borrow().clone());
    let v15: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v14).as_ref().clone()));
    let v16: Rc<str> = Rc::<str>::from((*v15.borrow()).join(&*v5));
    let mut v17: Rc<UH0> = names_in_16(v16.clone());
    let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v19: Rc<UH0> = method18(v17, v18);
    let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v21: Rc<UH0> = method19(v16, v19, v20);
    let v22: Rc<UH0> = method15(v11, v21);
    let mut v23: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v23.borrow_mut().push(v0);
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("archive"); } LIT.with(|lit| lit.clone()) };
    v23.borrow_mut().push(v24);
    let v25: Rc<Vec<Rc<str>>> = Rc::new(v23.borrow().clone());
    let v26: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v25).as_ref().clone()));
    let v27: Rc<str> = Rc::<str>::from((*v26.borrow()).join(&*v5));
    let v28: Rc<UH0> = names_in_16(v27.clone());
    let mut v29: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v30: Rc<UH0> = method18(v28, v29);
    let mut v31: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v32: Rc<UH0> = method19(v27, v30, v31);
    method15(v22, v32)
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method22(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US2 = if v6 {
                let mut v7: US3 = US3::US3_HeadType;
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
            let mut v39: US2 = match &v27 {
                US2::US2_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
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
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US3 = US3::US3_HeadOpen;
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
                    let mut v52: US3 = v52;
                    let v60: Rc<str> = match &v52 {
                        US3::US3_HeadInl => {
                            let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
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
                    let mut v71: usize = ((v61) as usize);
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
fn planned_21(mut v0: US5, mut v1: Rc<str>) -> US9 {
    let (mut v2, mut v3): (bool, Rc<str>) = match std::fs::read(&*v1) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
    if v2 {
        US9::US9_FileUnreadable(v1)
    } else {
        let v26: Rc<str> = match v0 {
            US5::US5_SymbolsDialect => {
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<UH0> = split_list_6(v5.clone(), v3.clone());
                let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v8: Rc<UH0> = method9(v6, v7);
                let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v10: i32 = 0i32;
                let mut v11: i32 = method10(v9.clone(), v8, v10);
                let mut v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
                let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
                let v14: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v5));
                v14
            }
            US5::US5_WordsDialect => {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<UH0> = split_list_6(v15.clone(), v3.clone());
                let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v18: Rc<UH0> = method22(v16, v17);
                let v19: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v20: i32 = 0i32;
                let v21: i32 = method10(v19.clone(), v18, v20);
                let v22: Rc<Vec<Rc<str>>> = Rc::new(v19.borrow().clone());
                let mut v23: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v22).as_ref().clone()));
                let mut v24: Rc<str> = Rc::<str>::from((*v23.borrow()).join(&*v15));
                v24
            }
        };
        let v27: bool = &*v26 == &*v3;
        if v27 {
            US9::US9_FileUnchanged
        } else {
            US9::US9_FileRewritten(v1, v26)
        }
    }
}
fn method20(mut v0: US5, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH1> = method20(v0.clone(), v4, v2);
            let mut v6: US9 = planned_21(v0, v3);
            Rc::new(UH1::UH1_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method24(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: US9 = v2;
                let v3: Rc<UH1> = v3;
                let mut v6: Rc<UH0> = match v2 {
                    US9::US9_FileUnreadable(v4) => {
                        let v4: Rc<str> = v4;
                        Rc::new(UH0::UH0_Cons(v4, v1))
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v3, v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn unreadable_of_23(mut v0: Rc<UH1>) -> Rc<UH0> {
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method24(v0, v1)
}
fn method26(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: US9 = v2;
                let v3: Rc<UH1> = v3;
                let mut v9: Rc<UH0> = match v2 {
                    US9::US9_FileRewritten(v4, v5) => {
                        let v4: Rc<str> = v4;
                        let v5: Rc<str> = v5;
                        let mut v6: bool = std::fs::write(&*v4, &*v5).is_ok();
                        if v6 {
                            v1
                        } else {
                            Rc::new(UH0::UH0_Cons(v4, v1))
                        }
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v3, v9);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn unwritten_of_25(mut v0: Rc<UH1>) -> Rc<UH0> {
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method26(v0, v1)
}
fn method28(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: US9 = v2;
                let v3: Rc<UH1> = v3;
                let mut v7: i32 = match v2 {
                    US9::US9_FileRewritten(v4, v5) => {
                        let v4: Rc<str> = v4;
                        let v5: Rc<str> = v5;
                        let mut v6: i32 = v1.wrapping_add(1i32);
                        v6
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v3, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn rewritten_count_27(mut v0: Rc<UH1>) -> i32 {
    let mut v1: i32 = 0i32;
    method28(v0, v1)
}
fn method29(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method29(v3, v1);
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
fn section_lines_31(mut v0: US8, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> US10 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v8, v9) => {
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH0> = v9.clone();
                let v19: bool = match &v0 {
                    US8::US8_SpiprojProject => {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let mut v11: bool = v8.starts_with(&*v10);
                        let v14: bool = if v11 {
                            true
                        } else {
                            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let v13: bool = v8.starts_with(&*v12);
                            v13
                        };
                        let v15: bool = v14 == false;
                        v15
                    }
                    US8::US8_TomlProject => {
                        let v16: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                        let mut v17: bool = &*v16 == "'''";
                        v17
                    }
                };
                if v19 {
                    let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v21: Rc<UH0> = method7(v3, v20);
                    return US10::US10_SplitProject(v2, v21, v1);
                } else {
                    let mut v23: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                    let mut v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v3));
                    (v1, v3) = (v9, v24);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH0> = method7(v3, v4);
                let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return US10::US10_SplitProject(v2, v5, v6);
            }
        }
    }
}
fn split_lines_30(mut v0: US8, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> US10 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<str> = Rc::<str>::from((v4).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v10: Rc<str> = match &v0 {
                    US8::US8_SpiprojProject => {
                        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                        v7
                    }
                    US8::US8_TomlProject => {
                        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                        v8
                    }
                };
                let mut v11: bool = &*v6 == &*v10;
                if v11 {
                    let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v12));
                    let v14: Rc<UH0> = method7(v2, v13);
                    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    return section_lines_31(v0, v5, v14, v15);
                } else {
                    let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v2));
                    (v1, v2) = (v5, v17);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US10::US10_NoModules;
            }
        }
    }
}
fn method32(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> bool {
    loop {
        match (Rc::unwrap_or_clone(v1), Rc::unwrap_or_clone(v0)) {
            (UH0::UH0_Cons(v2, v3), UH0::UH0_Cons(v4, v5)) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v2 == &*v4;
                if v6 {
                    (v0, v1) = (v5, v3);
                    continue;
                } else {
                    return false;
                }
            }
            (UH0::UH0_Nil, UH0::UH0_Nil) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}
fn method33(mut v0: US8, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method33(v0.clone(), v4, v2);
            let v13: Rc<str> = match v0 {
                US8::US8_SpiprojProject => {
                    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                    v6.borrow_mut().push(v7);
                    v6.borrow_mut().push(v3);
                    let mut v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
                    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
                    v11
                }
                US8::US8_TomlProject => {
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
fn method34(mut v0: US5, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH1> = method34(v0.clone(), v4, v2);
            let mut v6: US9 = planned_21(v0, v3);
            Rc::new(UH1::UH1_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn dialect_converted_13(mut v0: Rc<str>, mut v1: US5) -> US7 {
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v2.borrow_mut().push(v0.clone());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    v2.borrow_mut().push(v3);
    let v4: Rc<Vec<Rc<str>>> = Rc::new(v2.borrow().clone());
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v4).as_ref().clone()));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from((*v5.borrow()).join(&*v6));
    let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v8.borrow_mut().push(v7.clone());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.toml"); } LIT.with(|lit| lit.clone()) };
    v8.borrow_mut().push(v9.clone());
    let v10: Rc<Vec<Rc<str>>> = Rc::new(v8.borrow().clone());
    let mut v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v10).as_ref().clone()));
    let mut v12: Rc<str> = Rc::<str>::from((*v11.borrow()).join(&*v6));
    let v13: bool = std::path::Path::new(&*v12).is_file();
    let v16: US8 = if v13 {
        US8::US8_TomlProject
    } else {
        US8::US8_SpiprojProject
    };
    let v20: US8 = match &v1 {
        US5::US5_SymbolsDialect => {
            US8::US8_TomlProject
        }
        US5::US5_WordsDialect => {
            US8::US8_SpiprojProject
        }
    };
    let mut v23: Rc<str> = match &v16 {
        US8::US8_SpiprojProject => {
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
            v21
        }
        US8::US8_TomlProject => {
            v9.clone()
        }
    };
    let mut v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v24.borrow_mut().push(v7.clone());
    v24.borrow_mut().push(v23);
    let v25: Rc<Vec<Rc<str>>> = Rc::new(v24.borrow().clone());
    let v26: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v25).as_ref().clone()));
    let v27: Rc<str> = Rc::<str>::from((*v26.borrow()).join(&*v6));
    let v30: Rc<str> = match &v20 {
        US8::US8_SpiprojProject => {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
            v28
        }
        US8::US8_TomlProject => {
            v9
        }
    };
    let mut v31: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v31.borrow_mut().push(v7);
    v31.borrow_mut().push(v30);
    let v32: Rc<Vec<Rc<str>>> = Rc::new(v31.borrow().clone());
    let v33: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v32).as_ref().clone()));
    let v34: Rc<str> = Rc::<str>::from((*v33.borrow()).join(&*v6));
    let mut v35: bool = std::path::Path::new(&*v27).is_file();
    let v36: bool = v35 == false;
    if v36 {
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no state project (package.spiproj or spiral.toml) under the root"); } LIT.with(|lit| lit.clone()) };
        US7::US7_DialectRefused(v37)
    } else {
        let mut v39: bool = match (&v16, &v20) {
            (US8::US8_SpiprojProject, US8::US8_SpiprojProject) => {
                true
            }
            (US8::US8_TomlProject, US8::US8_TomlProject) => {
                true
            }
            _ => {
                false
            }
        };
        if v39 {
            let v40: Rc<UH0> = spi_files_14(v0);
            let v41: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v42: Rc<UH1> = method20(v1, v40, v41);
            let mut v43: Rc<UH0> = unreadable_of_23(v42.clone());
            let v44: bool = match &*v43 {
                UH0::UH0_Nil => {
                    true
                }
                _ => {
                    false
                }
            };
            let mut v45: bool = v44 != true;
            if v45 {
                let mut v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let mut v47: i32 = 0i32;
                let v48: i32 = method10(v46.clone(), v43, v47);
                let v49: Rc<Vec<Rc<str>>> = Rc::new(v46.borrow().clone());
                let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v49).as_ref().clone()));
                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                let mut v52: Rc<str> = Rc::<str>::from((*v50.borrow()).join(&*v51));
                let mut v53: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not read "); } LIT.with(|lit| lit.clone()) };
                v53.borrow_mut().push(v54);
                v53.borrow_mut().push(v52);
                let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; nothing written"); } LIT.with(|lit| lit.clone()) };
                v53.borrow_mut().push(v55);
                let v56: Rc<Vec<Rc<str>>> = Rc::new(v53.borrow().clone());
                let v57: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v56).as_ref().clone()));
                let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v59: Rc<str> = Rc::<str>::from((*v57.borrow()).join(&*v58));
                US7::US7_DialectRefused(v59)
            } else {
                let v61: Rc<UH0> = unwritten_of_25(v42.clone());
                let v62: bool = match &*v61 {
                    UH0::UH0_Nil => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v63: bool = v62 != true;
                if v63 {
                    let v64: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v65: i32 = 0i32;
                    let v66: i32 = method10(v64.clone(), v61, v65);
                    let v67: Rc<Vec<Rc<str>>> = Rc::new(v64.borrow().clone());
                    let v68: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v67).as_ref().clone()));
                    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let v70: Rc<str> = Rc::<str>::from((*v68.borrow()).join(&*v69));
                    let mut v71: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write "); } LIT.with(|lit| lit.clone()) };
                    v71.borrow_mut().push(v72);
                    v71.borrow_mut().push(v70);
                    let v73: Rc<Vec<Rc<str>>> = Rc::new(v71.borrow().clone());
                    let v74: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v73).as_ref().clone()));
                    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v76: Rc<str> = Rc::<str>::from((*v74.borrow()).join(&*v75));
                    US7::US7_DialectRefused(v76)
                } else {
                    let v78: i32 = rewritten_count_27(v42.clone());
                    let v79: bool = v78 == 0i32;
                    if v79 {
                        US7::US7_DialectAlready
                    } else {
                        let v81: i32 = rewritten_count_27(v42);
                        US7::US7_DialectConverted(v81)
                    }
                }
            }
        } else {
            let (mut v86, mut v87): (bool, Rc<str>) = match std::fs::read(&*v27) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v86 {
                let mut v88: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not read "); } LIT.with(|lit| lit.clone()) };
                v88.borrow_mut().push(v89);
                v88.borrow_mut().push(v27);
                let v90: Rc<Vec<Rc<str>>> = Rc::new(v88.borrow().clone());
                let v91: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v90).as_ref().clone()));
                let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v93: Rc<str> = Rc::<str>::from((*v91.borrow()).join(&*v92));
                US7::US7_DialectRefused(v93)
            } else {
                let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v98: Rc<str> = Rc::<str>::from(v87.replace(&*v96, &*v97));
                let v99: Rc<UH0> = split_list_6(v95.clone(), v98);
                let v100: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v101: Rc<UH0> = method29(v99, v100);
                let v102: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v103: US10 = split_lines_30(v16.clone(), v101, v102);
                let v155: US11 = match v103 {
                    US10::US10_NoModules => {
                        let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the state project has no modules section"); } LIT.with(|lit| lit.clone()) };
                        US11::US11_ProjectNotConvertible(v104)
                    }
                    US10::US10_SplitProject(v106, v107, v108) => {
                        let v106: Rc<UH0> = v106;
                        let v107: Rc<UH0> = v107;
                        let v108: Rc<UH0> = v108;
                        let v112: Rc<str> = match &v16 {
                            US8::US8_SpiprojProject => {
                                let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                                v109
                            }
                            US8::US8_TomlProject => {
                                let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                                v110
                            }
                        };
                        let v118: bool = match Rc::unwrap_or_clone(v106) {
                            UH0::UH0_Cons(v113, v114) => {
                                let v113: Rc<str> = v113;
                                let mut v114: Rc<UH0> = v114;
                                let v115: bool = &*v113 == &*v112;
                                if v115 {
                                    match Rc::unwrap_or_clone(v114) {
                                        UH0::UH0_Nil => {
                                            true
                                        }
                                        _ => {
                                            false
                                        }
                                    }
                                } else {
                                    false
                                }
                            }
                            _ => {
                                false
                            }
                        };
                        let v126: bool = if v118 {
                            let v124: Rc<UH0> = match v16 {
                                US8::US8_SpiprojProject => {
                                    { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                                }
                                US8::US8_TomlProject => {
                                    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'''"); } LIT.with(|lit| lit.clone()) };
                                    let v121: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    Rc::new(UH0::UH0_Cons(v120, v121))
                                }
                            };
                            method32(v124, v108)
                        } else {
                            false
                        };
                        if v126 {
                            let v130: Rc<str> = match &v20 {
                                US8::US8_SpiprojProject => {
                                    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                                    v127
                                }
                                US8::US8_TomlProject => {
                                    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                                    v128
                                }
                            };
                            let v136: Rc<UH0> = match &v20 {
                                US8::US8_SpiprojProject => {
                                    { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                                }
                                US8::US8_TomlProject => {
                                    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'''"); } LIT.with(|lit| lit.clone()) };
                                    let v133: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    Rc::new(UH0::UH0_Cons(v132, v133))
                                }
                            };
                            let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v138: Rc<UH0> = method33(v20, v107, v137);
                            let v139: Rc<UH0> = method15(v138, v136);
                            let mut v140: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            v140.borrow_mut().push(v130);
                            let v141: i32 = 1i32;
                            let v142: i32 = method10(v140.clone(), v139, v141);
                            let v143: Rc<Vec<Rc<str>>> = Rc::new(v140.borrow().clone());
                            let v144: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v143).as_ref().clone()));
                            let v145: Rc<str> = Rc::<str>::from((*v144.borrow()).join(&*v95));
                            let mut v146: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            v146.borrow_mut().push(v145);
                            v146.borrow_mut().push(v95);
                            let v147: Rc<Vec<Rc<str>>> = Rc::new(v146.borrow().clone());
                            let v148: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v147).as_ref().clone()));
                            let v149: Rc<str> = Rc::<str>::from((*v148.borrow()).join(&*v97));
                            US11::US11_ProjectConverted(v149)
                        } else {
                            let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the state project holds keys besides modules; convert it by hand"); } LIT.with(|lit| lit.clone()) };
                            US11::US11_ProjectNotConvertible(v151)
                        }
                    }
                };
                match v155 {
                    US11::US11_ProjectConverted(v158) => {
                        let v158: Rc<str> = v158;
                        let v159: Rc<UH0> = spi_files_14(v0);
                        let v160: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v161: Rc<UH1> = method34(v1, v159, v160);
                        let v162: Rc<UH0> = unreadable_of_23(v161.clone());
                        let v163: bool = match &*v162 {
                            UH0::UH0_Nil => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v164: bool = v163 != true;
                        if v164 {
                            let v165: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v166: i32 = 0i32;
                            let v167: i32 = method10(v165.clone(), v162, v166);
                            let v168: Rc<Vec<Rc<str>>> = Rc::new(v165.borrow().clone());
                            let v169: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v168).as_ref().clone()));
                            let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                            let v171: Rc<str> = Rc::<str>::from((*v169.borrow()).join(&*v170));
                            let mut v172: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not read "); } LIT.with(|lit| lit.clone()) };
                            v172.borrow_mut().push(v173);
                            v172.borrow_mut().push(v171);
                            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; nothing written"); } LIT.with(|lit| lit.clone()) };
                            v172.borrow_mut().push(v174);
                            let v175: Rc<Vec<Rc<str>>> = Rc::new(v172.borrow().clone());
                            let v176: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v175).as_ref().clone()));
                            let v177: Rc<str> = Rc::<str>::from((*v176.borrow()).join(&*v97));
                            US7::US7_DialectRefused(v177)
                        } else {
                            let v179: Rc<UH0> = unwritten_of_25(v161.clone());
                            let v180: bool = match &*v179 {
                                UH0::UH0_Nil => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            };
                            let v181: bool = v180 != true;
                            if v181 {
                                let v182: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                let v183: i32 = 0i32;
                                let v184: i32 = method10(v182.clone(), v179, v183);
                                let v185: Rc<Vec<Rc<str>>> = Rc::new(v182.borrow().clone());
                                let v186: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v185).as_ref().clone()));
                                let v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                                let v188: Rc<str> = Rc::<str>::from((*v186.borrow()).join(&*v187));
                                let mut v189: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                let v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write "); } LIT.with(|lit| lit.clone()) };
                                v189.borrow_mut().push(v190);
                                v189.borrow_mut().push(v188);
                                let mut v191: Rc<Vec<Rc<str>>> = Rc::new(v189.borrow().clone());
                                let v192: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v191).as_ref().clone()));
                                let v193: Rc<str> = Rc::<str>::from((*v192.borrow()).join(&*v97));
                                US7::US7_DialectRefused(v193)
                            } else {
                                let v195: bool = std::fs::write(&*v34, &*v158).is_ok();
                                let v196: bool = v195 == false;
                                if v196 {
                                    let mut v197: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                    let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write "); } LIT.with(|lit| lit.clone()) };
                                    v197.borrow_mut().push(v198);
                                    v197.borrow_mut().push(v34);
                                    let v199: Rc<Vec<Rc<str>>> = Rc::new(v197.borrow().clone());
                                    let v200: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v199).as_ref().clone()));
                                    let v201: Rc<str> = Rc::<str>::from((*v200.borrow()).join(&*v97));
                                    US7::US7_DialectRefused(v201)
                                } else {
                                    let v203: bool = std::fs::remove_file(&*v27).is_ok() || !std::path::Path::new(&*v27).exists();
                                    let v204: bool = v203 == false;
                                    if v204 {
                                        let mut v205: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                        let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not remove "); } LIT.with(|lit| lit.clone()) };
                                        v205.borrow_mut().push(v206);
                                        v205.borrow_mut().push(v27);
                                        let v207: Rc<Vec<Rc<str>>> = Rc::new(v205.borrow().clone());
                                        let v208: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v207).as_ref().clone()));
                                        let v209: Rc<str> = Rc::<str>::from((*v208.borrow()).join(&*v97));
                                        US7::US7_DialectRefused(v209)
                                    } else {
                                        let v211: i32 = rewritten_count_27(v161);
                                        US7::US7_DialectConverted(v211)
                                    }
                                }
                            }
                        }
                    }
                    US11::US11_ProjectNotConvertible(v156) => {
                        let v156: Rc<str> = v156;
                        US7::US7_DialectRefused(v156)
                    }
                }
            }
        }
    }
}
fn method36(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_35(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method36(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn agile_dialect_12(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: bool = &*v1 == "symbols";
    let v10: US4 = if v2 {
        let v3: US5 = US5::US5_SymbolsDialect;
        US4::US4_DialectChosen(v3)
    } else {
        let v5: bool = &*v1 == "words";
        if v5 {
            let mut v6: US5 = US5::US5_WordsDialect;
            US4::US4_DialectChosen(v6)
        } else {
            US4::US4_DialectUnknown
        }
    };
    match v10 {
        US4::US4_DialectChosen(v24) => {
            let mut v24: US5 = v24;
            let v28: Rc<str> = match &v24 {
                US5::US5_SymbolsDialect => {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("symbols"); } LIT.with(|lit| lit.clone()) };
                    v26
                }
                US5::US5_WordsDialect => {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("words"); } LIT.with(|lit| lit.clone()) };
                    v25
                }
            };
            let mut v29: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig agile dialect ok root="); } LIT.with(|lit| lit.clone()) };
            v29.borrow_mut().push(v30);
            v29.borrow_mut().push(v0.clone());
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" dialect="); } LIT.with(|lit| lit.clone()) };
            v29.borrow_mut().push(v31);
            v29.borrow_mut().push(v28);
            let v32: Rc<Vec<Rc<str>>> = Rc::new(v29.borrow().clone());
            let v33: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v32).as_ref().clone()));
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = Rc::<str>::from((*v33.borrow()).join(&*v34));
            let v36: US7 = dialect_converted_13(v0.clone(), v24);
            match v36 {
                US7::US7_DialectAlready => {
                    let mut v45: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    v45.borrow_mut().push(v35);
                    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files=0 unchanged=already"); } LIT.with(|lit| lit.clone()) };
                    v45.borrow_mut().push(v46);
                    let mut v47: Rc<Vec<Rc<str>>> = Rc::new(v45.borrow().clone());
                    let v48: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v47).as_ref().clone()));
                    let v49: Rc<str> = Rc::<str>::from((*v48.borrow()).join(&*v34));
                    let v50: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v49.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v50
                }
                US7::US7_DialectConverted(v37) => {
                    let v37: i32 = v37;
                    let v38: Rc<str> = format_real_35(v37);
                    let mut v39: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    v39.borrow_mut().push(v35);
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files="); } LIT.with(|lit| lit.clone()) };
                    v39.borrow_mut().push(v40);
                    v39.borrow_mut().push(v38);
                    let v41: Rc<Vec<Rc<str>>> = Rc::new(v39.borrow().clone());
                    let v42: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v41).as_ref().clone()));
                    let mut v43: Rc<str> = Rc::<str>::from((*v42.borrow()).join(&*v34));
                    let v44: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v43.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v44
                }
                US7::US7_DialectRefused(v51) => {
                    let v51: Rc<str> = v51;
                    let mut v52: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile dialect "); } LIT.with(|lit| lit.clone()) };
                    v52.borrow_mut().push(v53);
                    v52.borrow_mut().push(v0);
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    v52.borrow_mut().push(v54);
                    v52.borrow_mut().push(v51);
                    let v55: Rc<Vec<Rc<str>>> = Rc::new(v52.borrow().clone());
                    let v56: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v55).as_ref().clone()));
                    let v57: Rc<str> = Rc::<str>::from((*v56.borrow()).join(&*v34));
                    let v58: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v57 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v58
                }
            }
        }
        US4::US4_DialectUnknown => {
            let mut v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the dialect is symbols or words, not "); } LIT.with(|lit| lit.clone()) };
            v11.borrow_mut().push(v12);
            v11.borrow_mut().push(v1);
            let v13: Rc<Vec<Rc<str>>> = Rc::new(v11.borrow().clone());
            let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v13).as_ref().clone()));
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = Rc::<str>::from((*v14.borrow()).join(&*v15));
            let mut v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile dialect "); } LIT.with(|lit| lit.clone()) };
            v17.borrow_mut().push(v18);
            v17.borrow_mut().push(v0);
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            v17.borrow_mut().push(v19);
            v17.borrow_mut().push(v16);
            let v20: Rc<Vec<Rc<str>>> = Rc::new(v17.borrow().clone());
            let v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v20).as_ref().clone()));
            let v22: Rc<str> = Rc::<str>::from((*v21.borrow()).join(&*v15));
            let mut v23: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v22 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v23
        }
    }
}
fn agile_dialect_entry_11() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    agile_dialect_12(v1, v3)
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_dialect_entry_11()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_init_run() -> i32 {
    closure0()()
}
pub fn rig_agile_dialect_run() -> i32 {
    closure2()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
