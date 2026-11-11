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
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_NextWork,
    US1_RecentJobs,
    US1_MachineWindows,
    US1_AuthorNotes,
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
    US7_ScanPlain,
    US7_ScanQuoted,
    US7_ScanEscaped,
}
pub struct Mut1 { pub l0: Rc<str> }
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_1(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let v3: i32 = command_line_argument_count_2();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US0 = if v5 {
        let mut v6: Rc<str> = command_line_argument_3(v1);
        US0::US0_Some(v6)
    } else {
        US0::US0_None
    };
    match v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            v10
        }
    }
}
fn loop_9(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
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
fn method10(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_11(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
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
fn loop_12(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
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
fn loop_13(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
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
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method16(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method16(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method15(v16, v17)
    }
}
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method17(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US3 = if v6 {
                let v7: US4 = US4::US4_HeadType;
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
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let mut v18: US4 = US4::US4_HeadInl;
                        US3::US3_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US4 = US4::US4_HeadInl;
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
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let mut v29: bool = v2.starts_with(&*v28);
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
                    let mut v72: i32 = v61.wrapping_add(v70);
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
fn method18(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn state_file_text_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v2.borrow_mut().push(v0);
    v2.borrow_mut().push(v1);
    let v3: Rc<Vec<Rc<str>>> = Rc::new(v2.borrow().clone());
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v3).as_ref().clone()));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from((*v4.borrow()).join(&*v5));
    let v7: bool = std::path::Path::new(&*v6).is_file();
    if v7 {
        let (mut v8, mut v9): (bool, Rc<str>) = match std::fs::read(&*v6) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v8 {
            let v10: Rc<str> = string_concat(&["rig state read failed: ", &*v6]);
            std::panic::panic_any::<std::string::String>(format!("{}", v10))
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = Rc::<str>::from(v6.replace(&*v12, &*v5));
            let mut v14: Rc<str> = loop_9(v13);
            let v15: Option<usize> = v14.rfind(&*v5);
            let v16: Option<usize> = method10(v15);
            let mut v17: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v18: Option<i32> = v16.map(|x| v17(x));
            let v20: i32 = v18.unwrap_or(-1i32);
            let v21: bool = v20 < 0i32;
            let mut v25: bool = if v21 {
                true
            } else {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v23: bool = v14.ends_with(&*v22);
                let v24: bool = v23 == false;
                v24
            };
            let v46: US2 = if v25 {
                US2::US2_OtherFile
            } else {
                let v27: usize = ((0i32) as usize);
                let mut v28: usize = ((v20) as usize);
                let mut v29: Rc<str> = Rc::<str>::from(&(v14)[v27..v28]);
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v31: bool = v29.ends_with(&*v30);
                if v31 {
                    let v32: Rc<str> = loop_11(v29);
                    US2::US2_StateFile(v32)
                } else {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v35: bool = v29.ends_with(&*v34);
                    if v35 {
                        let v36: Rc<str> = loop_12(v29);
                        US2::US2_HistoryFile(v36)
                    } else {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v39: bool = v29.ends_with(&*v38);
                        if v39 {
                            let v40: Rc<str> = loop_13(v29);
                            US2::US2_ArchiveFile(v40)
                        } else {
                            US2::US2_OtherFile
                        }
                    }
                }
            };
            match v46 {
                US2::US2_OtherFile => {
                    v9
                }
                _ => {
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<UH0> = split_list_14(v47.clone(), v9);
                    let v49: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v50: Rc<UH0> = method17(v48, v49);
                    let v51: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v52: i32 = 0i32;
                    let v53: i32 = method18(v51.clone(), v50, v52);
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
    }
}
fn method21(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
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
fn method22(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
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
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method20(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let mut v6: i32 = 0i32;
            let v7: i32 = method21(v2.clone(), v5, v6);
            let v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2.clone(), v7 as i64, v8 as i64);
            let v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method22(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
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
fn section_lines_24(mut v0: US5, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> US6 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v8, v9) => {
                let v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH0> = v9.clone();
                let v26: bool = match &v0 {
                    US5::US5_SpiprojProject => {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v11: bool = v8.starts_with(&*v10);
                        let mut v14: bool = if v11 {
                            true
                        } else {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v13: bool = v8.starts_with(&*v12);
                            v13
                        };
                        let v15: bool = v14 == false;
                        v15
                    }
                    US5::US5_TomlProject => {
                        let v16: i32 = (v8.clone().len() as i32);
                        let mut v17: i32 = 0i32;
                        let mut v18: i32 = method21(v8.clone(), v16, v17);
                        let v19: i32 = v16.wrapping_sub(1i32);
                        let v20: Rc<str> = string_slice(&v8.clone(), v18 as i64, v19 as i64);
                        let v21: i32 = (v20.clone().len() as i32);
                        let v22: i32 = method22(v20.clone(), v21);
                        let v23: Rc<str> = string_slice(&v20, 0i32 as i64, v22 as i64);
                        let v24: bool = &*v23 == "'''";
                        v24
                    }
                };
                if v26 {
                    let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v28: Rc<UH0> = method15(v3, v27);
                    return US6::US6_SplitProject(v2, v28, v1);
                } else {
                    let v30: i32 = (v8.clone().len() as i32);
                    let v31: i32 = 0i32;
                    let v32: i32 = method21(v8.clone(), v30, v31);
                    let v33: i32 = v30.wrapping_sub(1i32);
                    let v34: Rc<str> = string_slice(&v8, v32 as i64, v33 as i64);
                    let v35: i32 = (v34.clone().len() as i32);
                    let v36: i32 = method22(v34.clone(), v35);
                    let v37: Rc<str> = string_slice(&v34, 0i32 as i64, v36 as i64);
                    let v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v37, v3));
                    (v1, v3) = (v9, v38);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH0> = method15(v3, v4);
                let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return US6::US6_SplitProject(v2, v5, v6);
            }
        }
    }
}
fn split_lines_23(mut v0: US5, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> US6 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: i32 = (v4.clone().len() as i32);
                let v7: i32 = 0i32;
                let v8: i32 = method21(v4.clone(), v6, v7);
                let mut v9: i32 = v6.wrapping_sub(1i32);
                let v10: Rc<str> = string_slice(&v4.clone(), v8 as i64, v9 as i64);
                let v11: i32 = (v10.clone().len() as i32);
                let v12: i32 = method22(v10.clone(), v11);
                let mut v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
                let mut v17: Rc<str> = match &v0 {
                    US5::US5_SpiprojProject => {
                        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                        v14
                    }
                    US5::US5_TomlProject => {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                        v15
                    }
                };
                let mut v18: bool = &*v13 == &*v17;
                if v18 {
                    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v19));
                    let v21: Rc<UH0> = method15(v2, v20);
                    let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    return section_lines_24(v0, v5, v21, v22);
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
fn method25(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method25(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes_"); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
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
fn method26(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method26(v0.clone(), v4, v2);
            let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v6.borrow_mut().push(v3);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            v6.borrow_mut().push(v7);
            let v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
            let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
            let v12: Rc<str> = state_file_text_8(v0, v11);
            let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            v13.borrow_mut().push(v14);
            v13.borrow_mut().push(v12);
            let v15: Rc<Vec<Rc<str>>> = Rc::new(v13.borrow().clone());
            let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
            let mut v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v10));
            Rc::new(UH0::UH0_Cons(v17, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn state_segments_text_19(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v1.borrow_mut().push(v0.clone());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.toml"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v2.clone());
    let v3: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v3).as_ref().clone()));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from((*v4.borrow()).join(&*v5));
    let v7: bool = std::path::Path::new(&*v6).is_file();
    let v10: US5 = if v7 {
        US5::US5_TomlProject
    } else {
        US5::US5_SpiprojProject
    };
    let mut v13: Rc<str> = match v10 {
        US5::US5_SpiprojProject => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US5::US5_TomlProject => {
            v2.clone()
        }
    };
    let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v14.borrow_mut().push(v0.clone());
    v14.borrow_mut().push(v13);
    let v15: Rc<Vec<Rc<str>>> = Rc::new(v14.borrow().clone());
    let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
    let mut v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v5));
    let mut v18: bool = std::path::Path::new(&*v17).is_file();
    let v71: Rc<str> = if v18 {
        let (mut v19, mut v20): (bool, Rc<str>) = match std::fs::read(&*v17) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v19 {
            let v21: Rc<str> = string_concat(&["rig state read failed: ", &*v17]);
            std::panic::panic_any::<std::string::String>(format!("{}", v21))
        } else {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = Rc::<str>::from(v17.replace(&*v23, &*v5));
            let mut v25: Rc<str> = loop_9(v24);
            let v26: Option<usize> = v25.rfind(&*v5);
            let v27: Option<usize> = method10(v26);
            let mut v28: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v29: Option<i32> = v27.map(|x| v28(x));
            let v31: i32 = v29.unwrap_or(-1i32);
            let v32: bool = v31 < 0i32;
            let v36: bool = if v32 {
                true
            } else {
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v34: bool = v25.ends_with(&*v33);
                let v35: bool = v34 == false;
                v35
            };
            let v57: US2 = if v36 {
                US2::US2_OtherFile
            } else {
                let v38: usize = ((0i32) as usize);
                let v39: usize = ((v31) as usize);
                let v40: Rc<str> = Rc::<str>::from(&(v25)[v38..v39]);
                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v42: bool = v40.ends_with(&*v41);
                if v42 {
                    let v43: Rc<str> = loop_11(v40);
                    US2::US2_StateFile(v43)
                } else {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v46: bool = v40.ends_with(&*v45);
                    if v46 {
                        let v47: Rc<str> = loop_12(v40);
                        US2::US2_HistoryFile(v47)
                    } else {
                        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v50: bool = v40.ends_with(&*v49);
                        if v50 {
                            let v51: Rc<str> = loop_13(v40);
                            US2::US2_ArchiveFile(v51)
                        } else {
                            US2::US2_OtherFile
                        }
                    }
                }
            };
            match v57 {
                US2::US2_OtherFile => {
                    v20
                }
                _ => {
                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v59: Rc<UH0> = split_list_14(v58.clone(), v20);
                    let v60: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v61: Rc<UH0> = method17(v59, v60);
                    let v62: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v63: i32 = 0i32;
                    let v64: i32 = method18(v62.clone(), v61, v63);
                    let v65: Rc<Vec<Rc<str>>> = Rc::new(v62.borrow().clone());
                    let v66: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v65).as_ref().clone()));
                    let v67: Rc<str> = Rc::<str>::from((*v66.borrow()).join(&*v58));
                    v67
                }
            }
        }
    } else {
        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v70
    };
    let mut v72: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v72.borrow_mut().push(v0.clone());
    v72.borrow_mut().push(v2);
    let v73: Rc<Vec<Rc<str>>> = Rc::new(v72.borrow().clone());
    let v74: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v73).as_ref().clone()));
    let v75: Rc<str> = Rc::<str>::from((*v74.borrow()).join(&*v5));
    let v76: bool = std::path::Path::new(&*v75).is_file();
    let v79: US5 = if v76 {
        US5::US5_TomlProject
    } else {
        US5::US5_SpiprojProject
    };
    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v83: Rc<str> = Rc::<str>::from(v71.replace(&*v81, &*v82));
    let v84: Rc<UH0> = split_list_14(v80, v83);
    let v85: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v86: Rc<UH0> = method20(v84, v85);
    let v87: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v88: US6 = split_lines_23(v79, v86, v87);
    let v94: Rc<UH0> = match v88 {
        US6::US6_NoModules => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US6::US6_SplitProject(v89, v90, v91) => {
            let v89: Rc<UH0> = v89;
            let v90: Rc<UH0> = v90;
            let v91: Rc<UH0> = v91;
            v90
        }
    };
    let v95: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v96: Rc<UH0> = method25(v94, v95);
    let v97: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v98: Rc<UH0> = method26(v0, v96, v97);
    let v99: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v100: i32 = 0i32;
    let v101: i32 = method18(v99.clone(), v98, v100);
    let v102: Rc<Vec<Rc<str>>> = Rc::new(v99.borrow().clone());
    let v103: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v102).as_ref().clone()));
    let v104: Rc<str> = Rc::<str>::from((*v103.borrow()).join(&*v82));
    v104
}
fn state_text_with_notes_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = state_file_text_8(v0.clone(), v1);
    let v3: Rc<str> = state_segments_text_19(v0);
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v4.borrow_mut().push(v2);
    v4.borrow_mut().push(v3);
    let v5: Rc<Vec<Rc<str>>> = Rc::new(v4.borrow().clone());
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from((*v6.borrow()).join(&*v7));
    v8
}
fn state_text_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v1.borrow_mut().push(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v2);
    let v3: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v3).as_ref().clone()));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from((*v4.borrow()).join(&*v5));
    state_text_with_notes_7(v6)
}
fn declared_from_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
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
fn field_end_31(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US7) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US7::US7_ScanPlain => {
                    let mut v6: bool = v5 == 44i32;
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
                let mut v25: US7 = match v3 {
                    US7::US7_ScanEscaped => {
                        US7::US7_ScanQuoted
                    }
                    US7::US7_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US7::US7_ScanQuoted
                        } else {
                            US7::US7_ScanPlain
                        }
                    }
                    US7::US7_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US7::US7_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
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
fn skip_spaces_32(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_30(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US7 = US7::US7_ScanPlain;
            let mut v6: i32 = field_end_31(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_32(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn concat5_35(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn unescaped_from_34(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return concat5_35(v4, v6, v7.clone(), v7.clone(), v7.clone());
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v10: bool = v9 == 92i32;
            if v10 {
                let v11: i32 = v1.wrapping_add(1i32);
                let v12: i32 = v11.wrapping_add(1i32);
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v15: bool = v14 == 110i32;
                let v24: Rc<str> = if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16
                } else {
                    let mut v17: bool = v14 == 114i32;
                    if v17 {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18
                    } else {
                        let v19: bool = v14 == 116i32;
                        if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20
                        } else {
                            let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21
                        }
                    }
                };
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = concat5_35(v4, v13, v24, v25.clone(), v25.clone());
                (v1, v2, v4) = (v12, v12, v26);
                continue;
            } else {
                let mut v28: i32 = v1.wrapping_add(1i32);
                v1 = v28;
                continue;
            }
        }
    }
}
fn unquoted_33(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let mut v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_34(v3, v4, v5, v6, v7)
}
fn note_at_29(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1.wrapping_add(6i32);
    let v3: i32 = (v0.clone().len() as i32);
    let mut v4: i32 = 2i32;
    let v5: i32 = field_start_30(v0.clone(), v2, v3, v4);
    let mut v6: US7 = US7::US7_ScanPlain;
    let v7: i32 = field_end_31(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_33(v8);
    let v10: i32 = 3i32;
    let v11: i32 = field_start_30(v0.clone(), v2, v3, v10);
    let v12: US7 = US7::US7_ScanPlain;
    let mut v13: i32 = field_end_31(v0.clone(), v11, v3, v12);
    let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v13 as usize).unwrap_or(""));
    let v15: Rc<str> = unquoted_33(v14);
    let v16: i32 = 4i32;
    let mut v17: i32 = field_start_30(v0.clone(), v2, v3, v16);
    let mut v18: US7 = US7::US7_ScanPlain;
    let v19: i32 = field_end_31(v0.clone(), v17, v3, v18);
    let v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let v21: Rc<str> = unquoted_33(v20);
    (v9, v15, v21)
}
fn authored_row_36(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v5: i32 = v1.wrapping_add(6i32);
    let mut v6: i32 = (v0.clone().len() as i32);
    let v7: i32 = 0i32;
    let v8: i32 = field_start_30(v0.clone(), v5, v6, v7);
    let mut v9: US7 = US7::US7_ScanPlain;
    let v10: i32 = field_end_31(v0.clone(), v8, v6, v9);
    let v11: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v10 as usize).unwrap_or(""));
    let v12: Rc<str> = unquoted_33(v11);
    let v15: Rc<str> = std::rc::Rc::<str>::from((v2).replace(char::from(10u8), " ").chars().take(160i32 as usize).collect::<String>().as_str());
    let v16: Rc<str> = string_concat(&[&*v3, "  ", &*v12, "  ", &*v15]);
    v16
}
fn authored_rows_from_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: i32 = declared_from_28(v0.clone(), v1.clone(), v3);
        let mut v6: bool = v5 < 0i32;
        if v6 {
            return v4;
        } else {
            let (mut v7, mut v8, mut v9): (Rc<str>, Rc<str>, Rc<str>) = note_at_29(v0.clone(), v5);
            let v10: bool = &*v8 == &*v2;
            let mut v14: Rc<str> = if v10 {
                let v12: Rc<str> = authored_row_36(v0.clone(), v5, v9, v7);
                let mut v13: Rc<str> = string_concat(&[&*v4, "\n", &*v12]);
                v13
            } else {
                v4
            };
            let v15: i32 = v5.wrapping_add(1i32);
            (v3, v4) = (v15, v14);
            continue;
        }
    }
}
fn author_notes_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = state_text_6(v0);
    let mut v4: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note (\""); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = 0i32;
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = authored_rows_from_27(v4, v5, v2.clone(), v6, v7);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from({ let mut rows = (v8).split(char::from(10u8)).filter(|row| !row.is_empty()).collect::<Vec<_>>(); rows.sort_by(|a, b| b.cmp(a)); rows.truncate(8i32 as usize); rows.join(&char::from(10u8).to_string()) }.as_str());
    let v10: bool = &*v9 == "";
    if v10 {
        let v11: Rc<str> = string_concat(&["no notes by ", &*v2]);
        v11
    } else {
        v9
    }
}
fn section_shown_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US1, mut v4: i32) -> i32 {
    let v7: bool = match &v3 {
        US1::US1_AuthorNotes => {
            let v5: bool = &*v2 == "";
            let mut v6: bool = v5 == false;
            v6
        }
        _ => {
            true
        }
    };
    if v7 {
        let v15: Rc<str> = match &v3 {
            US1::US1_AuthorNotes => {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== recent notes by author (agile notes --as)"); } LIT.with(|lit| lit.clone()) };
                v11
            }
            US1::US1_MachineWindows => {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== machine windows (window list)"); } LIT.with(|lit| lit.clone()) };
                v10
            }
            US1::US1_NextWork => {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== next work (agile next, WSJF)"); } LIT.with(|lit| lit.clone()) };
                v8
            }
            US1::US1_RecentJobs => {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== recent jobs (job list)"); } LIT.with(|lit| lit.clone()) };
                v9
            }
        };
        println!("{}", v15);
        let v30: Rc<str> = match &v3 {
            US1::US1_AuthorNotes => {
                author_notes_5(v0, v1, v2)
            }
            _ => {
                let mut v28: Rc<str> = match v3 {
                    US1::US1_MachineWindows => {
                        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window\nlist"); } LIT.with(|lit| lit.clone()) };
                        v25
                    }
                    US1::US1_NextWork => {
                        let mut v18: Rc<str> = string_concat(&["agile\nnext\n", &*v0, "\n8"]);
                        let v19: bool = &*v1 == "";
                        let v22: Rc<str> = if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v20
                        } else {
                            let v21: Rc<str> = string_concat(&["\n--team\n", &*v1]);
                            v21
                        };
                        let v23: Rc<str> = string_concat(&[&*v18, &*v22]);
                        v23
                    }
                    US1::US1_RecentJobs => {
                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job\nlist\n12"); } LIT.with(|lit| lit.clone()) };
                        v24
                    }
                    _ => unreachable!(),
                };
                let mut v29: Rc<str> = std::rc::Rc::<str>::from(std::env::current_exe().ok().and_then(|exe| std::process::Command::new(exe).args((v28).split(char::from(10u8)).filter(|item| !item.is_empty())).output().ok()).map(|out| String::from_utf8_lossy(&out.stdout).trim_end().to_owned()).unwrap_or_default().as_str());
                v29
            }
        };
        println!("{}", v30);
        let v31: i32 = v4.wrapping_add(1i32);
        v31
    } else {
        v4
    }
}
fn method38(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_37(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method38(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn resume_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = argument_1(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().collect::<Vec<_>>(); args.iter().position(|argument| argument == &*v2).and_then(|at| args.get(at + 1)).cloned().unwrap_or_default() }.as_str());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().collect::<Vec<_>>(); args.iter().position(|argument| argument == &*v4).and_then(|at| args.get(at + 1)).cloned().unwrap_or_default() }.as_str());
    let mut v6: bool = &*v1 == "";
    let mut v9: bool = if v6 {
        true
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v1.starts_with(&*v7);
        v8
    };
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: kit resume <backlog-root> [--team <team>] [--as <author>]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v10);
        2i32
    } else {
        let v11: US1 = US1::US1_NextWork;
        let v12: i32 = 0i32;
        let mut v13: i32 = section_shown_4(v1.clone(), v3.clone(), v5.clone(), v11, v12);
        let mut v14: US1 = US1::US1_RecentJobs;
        let v15: i32 = section_shown_4(v1.clone(), v3.clone(), v5.clone(), v14, v13);
        let v16: US1 = US1::US1_MachineWindows;
        let mut v17: i32 = section_shown_4(v1.clone(), v3.clone(), v5.clone(), v16, v15);
        let mut v18: US1 = US1::US1_AuthorNotes;
        let v19: i32 = section_shown_4(v1, v3.clone(), v5.clone(), v18, v17);
        let v20: Rc<str> = format_real_37(v19);
        let v21: Rc<str> = string_concat(&["rig resume ok sections=", &*v20]);
        let v22: bool = &*v3 == "";
        let mut v25: Rc<str> = if v22 {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v23
        } else {
            let v24: Rc<str> = string_concat(&[" team=", &*v3]);
            v24
        };
        let v26: bool = &*v5 == "";
        let mut v29: Rc<str> = if v26 {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v27
        } else {
            let mut v28: Rc<str> = string_concat(&[" author=", &*v5]);
            v28
        };
        let v33: Rc<str> = std::rc::Rc::<str>::from(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0).to_string().as_str());
        let v34: Rc<str> = string_concat(&[&*v21, &*v25, &*v29, " unix=", &*v33]);
        println!("{}", v34);
        0i32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        resume_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_resume_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
