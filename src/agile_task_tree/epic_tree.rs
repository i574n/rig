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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_StateFile(Rc<str>),
    US1_HistoryFile(Rc<str>),
    US1_ArchiveFile(Rc<str>),
    US1_OtherFile,
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
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>, Rc<str>),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_Some(Rc<str>, Rc<str>),
    US5_None,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<str>, Rc<UH3>),
}
#[derive(Clone)]
enum US6 {
    US6_ScanPlain,
    US6_ScanQuoted,
    US6_ScanEscaped,
}
#[derive(Clone)]
enum US7 {
    US7_Some(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US7_None,
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US8 {
    US8_AllTeams,
    US8_OneTeam(Rc<str>),
    US8_TeamMissing,
}
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(Rc<UH0>, Rc<UH4>),
}
#[derive(Clone)]
enum US9 {
    US9_Some(i32),
    US9_None,
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_2(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_3();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US0 = if v4 {
        let mut v5: Rc<str> = command_line_argument_4(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let mut v12: Rc<UH0> = arguments_from_2(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn loop_6(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method7(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_8(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(6i32);
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
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_10(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method13(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method13(v7, &v9) {
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
        method12(v16, v17)
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method14(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US2 = if v6 {
                let mut v7: US3 = US3::US3_HeadType;
                US2::US2_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US3 = US3::US3_HeadType;
                    US2::US2_SymbolHead(v11)
                } else {
                    US2::US2_OtherLine
                }
            };
            let v27: US2 = match &v15 {
                US2::US2_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
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
fn method15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
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
fn file_text_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = std::path::Path::new(&*v0).is_file();
    if v1 {
        let (mut v2, mut v3): (bool, Rc<str>) = match std::fs::read(&*v0) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v2 {
            let v4: Rc<str> = string_concat(&["rig state read failed: ", &*v0]);
            std::panic::panic_any::<std::string::String>(format!("{}", v4))
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
            let mut v9: Rc<str> = loop_6(v8);
            let v10: Option<usize> = v9.rfind(&*v7);
            let v11: Option<usize> = method7(v10);
            let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v13: Option<i32> = v11.map(|x| v12(x));
            let v15: i32 = v13.unwrap_or(-1i32);
            let v16: bool = v15 < 0i32;
            let v20: bool = if v16 {
                true
            } else {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v18: bool = v9.ends_with(&*v17);
                let v19: bool = v18 == false;
                v19
            };
            let v41: US1 = if v20 {
                US1::US1_OtherFile
            } else {
                let v22: usize = ((0i32) as usize);
                let v23: usize = ((v15) as usize);
                let v24: Rc<str> = Rc::<str>::from(&(v9)[v22..v23]);
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v26: bool = v24.ends_with(&*v25);
                if v26 {
                    let v27: Rc<str> = loop_8(v24);
                    US1::US1_StateFile(v27)
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v30: bool = v24.ends_with(&*v29);
                    if v30 {
                        let v31: Rc<str> = loop_9(v24);
                        US1::US1_HistoryFile(v31)
                    } else {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v34: bool = v24.ends_with(&*v33);
                        if v34 {
                            let v35: Rc<str> = loop_10(v24);
                            US1::US1_ArchiveFile(v35)
                        } else {
                            US1::US1_OtherFile
                        }
                    }
                }
            };
            match v41 {
                US1::US1_OtherFile => {
                    v3
                }
                _ => {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<UH0> = split_list_11(v42.clone(), v3);
                    let v44: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v45: Rc<UH0> = method14(v43, v44);
                    let v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v47: i32 = 0i32;
                    let v48: i32 = method15(v46.clone(), v45, v47);
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
fn method17(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                let mut v5: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v1));
                (v0, v1) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
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
fn loop_21(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(2i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn quoted_pair_20(mut v0: Rc<str>) -> US5 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = split_list_11(v1, v0);
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH0::UH0_Cons(v5, v6) => {
                    let mut v5: Rc<str> = v5;
                    let mut v6: Rc<UH0> = v6;
                    match Rc::unwrap_or_clone(v6) {
                        UH0::UH0_Nil => {
                            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
                            let mut v8: bool = v5.ends_with(&*v7);
                            if v8 {
                                let mut v9: Rc<str> = loop_21(v5);
                                let v10: bool = &*v3 == "";
                                let mut v12: bool = if v10 {
                                    true
                                } else {
                                    let v11: bool = &*v9 == "";
                                    v11
                                };
                                if v12 {
                                    US5::US5_None
                                } else {
                                    US5::US5_Some(v3, v9)
                                }
                            } else {
                                US5::US5_None
                            }
                        }
                        _ => {
                            US5::US5_None
                        }
                    }
                }
                _ => {
                    US5::US5_None
                }
            }
        }
        _ => {
            US5::US5_None
        }
    }
}
fn link_of_line_19(mut v0: Rc<str>) -> US4 {
    let mut v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Parent (\""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Option<usize> = v1.find(&*v2);
    let v4: Option<usize> = method7(v3);
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let mut v8: i32 = v6.unwrap_or(-1i32);
    let mut v9: bool = v8 < 0i32;
    if v9 {
        US4::US4_None
    } else {
        let v11: i32 = v8.wrapping_add(12i32);
        let mut v12: i32 = (v1.clone().len() as i32);
        let mut v13: i32 = v12.wrapping_sub(1i32);
        let v14: Rc<str> = string_slice(&v1, v11 as i64, v13 as i64);
        let v15: US5 = quoted_pair_20(v14);
        match v15 {
            US5::US5_None => {
                US4::US4_None
            }
            US5::US5_Some(v16, v17) => {
                let v16: Rc<str> = v16;
                let v17: Rc<str> = v17;
                US4::US4_Some(v16, v17)
            }
        }
    }
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: US4 = link_of_line_19(v2);
                let mut v9: Rc<UH1> = match v4 {
                    US4::US4_None => {
                        v1
                    }
                    US4::US4_Some(v5, v6) => {
                        let mut v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        Rc::new(UH1::UH1_Cons(v5, v6, v1))
                    }
                };
                (v0, v1) = (v3, v9);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn links_of_text_16(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_11(v1, v4);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH1> = method18(v5, v6);
    let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method17(v7, v8)
}
fn method22(mut v0: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>, mut v1: i32, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = (v0)[v4].clone();
            let v14: i32 = v1.wrapping_sub(1i32);
            let v15: Rc<UH2> = Rc::new(UH2::UH2_Cons(v5, v6, v7, v8, v9, v10, v11, v12, v13, v2));
            (v1, v2) = (v14, v15);
            continue;
        }
    }
}
fn field_end_26(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US6) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US6::US6_ScanPlain => {
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
                let v10: i32 = v1.wrapping_add(1i32);
                let v25: US6 = match v3 {
                    US6::US6_ScanEscaped => {
                        US6::US6_ScanQuoted
                    }
                    US6::US6_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US6::US6_ScanQuoted
                        } else {
                            US6::US6_ScanPlain
                        }
                    }
                    US6::US6_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US6::US6_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US6::US6_ScanPlain
                            } else {
                                US6::US6_ScanQuoted
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
fn skip_spaces_27(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let mut v3: bool = v2 == 32i32;
        if v3 {
            let v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_25(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US6 = US6::US6_ScanPlain;
            let mut v6: i32 = field_end_26(v0.clone(), v1, v2, v5);
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_27(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn quoted_value_28(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v5: bool = if v3 {
        let v4: bool = v1.ends_with(&*v2);
        v4
    } else {
        false
    };
    let mut v9: Rc<str> = if v5 {
        let mut v6: i32 = (v1.clone().len() as i32);
        let mut v7: i32 = v6.wrapping_sub(2i32);
        let mut v8: Rc<str> = string_slice(&v1, 1i32 as i64, v7 as i64);
        v8
    } else {
        v1
    };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v2));
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v13));
    v14
}
fn method24(mut v0: Rc<UH0>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                let mut v5: Option<usize> = v2.find(&*v4);
                let mut v6: Option<usize> = method7(v5);
                let mut v7: Rc<dyn Fn((usize)) -> i32> = closure2();
                let mut v8: Option<i32> = v6.map(|x| v7(x));
                let v10: i32 = v8.unwrap_or(-1i32);
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                let mut v12: bool = v2.starts_with(&*v11);
                let v14: bool = if v12 {
                    let mut v13: bool = v10 >= 0i32;
                    v13
                } else {
                    false
                };
                let v30: Rc<UH3> = if v14 {
                    let v15: i32 = v10.wrapping_add(6i32);
                    let v16: i32 = (v2.clone().len() as i32);
                    let v17: i32 = 0i32;
                    let v18: i32 = field_start_25(v2.clone(), v15, v16, v17);
                    let v19: US6 = US6::US6_ScanPlain;
                    let v20: i32 = field_end_26(v2.clone(), v18, v16, v19);
                    let v21: Rc<str> = std::rc::Rc::<str>::from(v2.get(v18 as usize..v20 as usize).unwrap_or(""));
                    let v22: Rc<str> = quoted_value_28(v21);
                    let v23: i32 = 1i32;
                    let v24: i32 = field_start_25(v2.clone(), v15, v16, v23);
                    let v25: US6 = US6::US6_ScanPlain;
                    let v26: i32 = field_end_26(v2.clone(), v24, v16, v25);
                    let v27: Rc<str> = std::rc::Rc::<str>::from(v2.get(v24 as usize..v26 as usize).unwrap_or(""));
                    let v28: Rc<str> = quoted_value_28(v27);
                    Rc::new(UH3::UH3_Cons(v22, v28, v1))
                } else {
                    v1
                };
                (v0, v1) = (v3, v30);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn titles_of_23(mut v0: Rc<str>) -> Rc<UH3> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_11(v1, v4);
    let mut v6: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    method24(v5, v6)
}
fn method30(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let v4: Rc<UH1> = v4;
            let mut v5: Rc<UH0> = method30(v4, v1);
            Rc::new(UH0::UH0_Cons(v2, v5))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method31(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let mut v5: Rc<UH1> = v5;
            let mut v6: Rc<UH1> = method31(v0.clone(), v5, v2);
            let mut v7: bool = &*v4 == &*v0;
            if v7 {
                Rc::new(UH1::UH1_Cons(v3, v4, v6))
            } else {
                v6
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn children_of_29(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH1> = method31(v1, v0, v2);
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method30(v3, v4)
}
fn method35(mut v0: Rc<str>, mut v1: Rc<UH2>, mut v2: US7) -> US7 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
                let mut v6: i32 = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<str> = v11;
                let mut v12: Rc<UH2> = v12;
                let mut v13: bool = &*v3 == &*v0;
                let v15: US7 = if v13 {
                    US7::US7_Some(v3, v4, v5, v6, v7, v8, v9, v10, v11)
                } else {
                    v2
                };
                (v1, v2) = (v12, v15);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn fact_for_34(mut v0: Rc<UH2>, mut v1: Rc<str>) -> US7 {
    let mut v2: US7 = US7::US7_None;
    method35(v1, v0, v2)
}
fn method37(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_36(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method37(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn child_line_33(mut v0: Rc<UH2>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: US7 = fact_for_34(v0, v1.clone());
    match v2 {
        US7::US7_None => {
            let v20: Rc<str> = string_concat(&["  ", &*v1, "  unknown"]);
            v20
        }
        US7::US7_Some(v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let mut v5: Rc<str> = v5;
            let mut v6: i32 = v6;
            let mut v7: Rc<str> = v7;
            let mut v8: Rc<str> = v8;
            let mut v9: Rc<str> = v9;
            let v10: Rc<str> = v10;
            let v11: Rc<str> = v11;
            let v16: Rc<str> = format_real_36(v6);
            let v18: Rc<str> = string_concat(&["  ", &*v1, "  ", &*v4, " ", &*v16, "m"]);
            v18
        }
    }
}
fn method40(mut v0: Rc<str>, mut v1: Rc<UH3>, mut v2: US0) -> US0 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH3::UH3_Cons(v3, v4, v5) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH3> = v5;
                let mut v6: bool = &*v3 == &*v0;
                let mut v8: US0 = if v6 {
                    US0::US0_Some(v4)
                } else {
                    v2
                };
                (v1, v2) = (v5, v8);
                continue;
            }
            UH3::UH3_Nil => {
                return v2;
            }
        }
    }
}
fn title_for_39(mut v0: Rc<UH3>, mut v1: Rc<str>) -> US0 {
    let mut v2: US0 = US0::US0_None;
    method40(v1, v0, v2)
}
fn titled_line_38(mut v0: Rc<UH3>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: US0 = title_for_39(v0, v1);
    match v3 {
        US0::US0_None => {
            v2
        }
        US0::US0_Some(v4) => {
            let v4: Rc<str> = v4;
            let mut v6: Rc<str> = string_concat(&[&*v2, "  ", &*v4]);
            v6
        }
    }
}
fn method32(mut v0: Rc<UH2>, mut v1: Rc<UH3>, mut v2: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let mut v5: Rc<str> = child_line_33(v0.clone(), v3.clone());
                let mut v6: Rc<str> = titled_line_38(v1.clone(), v3, v5);
                println!("{}", v6);
                v2 = v4;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method41(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn parent_run_1(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = 0i32;
    let mut v2: Rc<UH0> = arguments_from_2(v1);
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH0::UH0_Cons(v5, v6) => {
                    let mut v5: Rc<str> = v5;
                    let mut v6: Rc<UH0> = v6;
                    match Rc::unwrap_or_clone(v6) {
                        UH0::UH0_Cons(v7, v8) => {
                            let mut v7: Rc<str> = v7;
                            let mut v8: Rc<UH0> = v8;
                            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                            let v11: Rc<str> = Rc::<str>::from(v7.replace(&*v9, &*v10));
                            let mut v12: Rc<str> = string_concat(&[&*v11, "/state/parents.spi"]);
                            let mut v13: Rc<str> = file_text_5(v12);
                            let v14: Rc<UH1> = links_of_text_16(v13);
                            let v15: Rc<str> = string_concat(&[&*v11, "/state/agile.spi"]);
                            let v16: Rc<str> = file_text_5(v15);
                            let v17: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = agile_quality_facts::rig_agile_fact_rows(&v16);
                            let v18: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new((v17).borrow().clone());
                            let v19: usize = (v18).len();
                            let v20: i32 = ((v19) as i32);
                            let v21: i32 = v20.wrapping_sub(1i32);
                            let v22: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                            let v23: Rc<UH2> = method22(v18, v21, v22);
                            let v24: Rc<UH3> = titles_of_23(v16);
                            let v25: Rc<UH0> = children_of_29(v14, v0.clone());
                            method32(v23, v24, v25.clone());
                            let v26: i32 = 0i32;
                            let v27: i32 = method41(v25, v26);
                            let v28: Rc<str> = format_real_36(v27);
                            let v31: Rc<str> = string_concat(&["rig agile list ok parent=", &*v0, " children=", &*v28]);
                            println!("{}", v31);
                            0i32
                        }
                        _ => {
                            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile list <root> --parent <EPIC>"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v32);
                            2i32
                        }
                    }
                }
                _ => {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile list <root> --parent <EPIC>"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v34);
                    2i32
                }
            }
        }
        _ => {
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile list <root> --parent <EPIC>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v36);
            2i32
        }
    }
}
fn team_after_43(mut v0: Rc<UH0>) -> US8 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<UH0> = v2;
                let mut v3: bool = "--team" == &*v1;
                if v3 {
                    match Rc::unwrap_or_clone(v2) {
                        UH0::UH0_Cons(v4, v5) => {
                            let v4: Rc<str> = v4;
                            let mut v5: Rc<UH0> = v5;
                            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                            let mut v7: bool = v4.starts_with(&*v6);
                            let mut v8: bool = v7 == false;
                            if v8 {
                                return US8::US8_OneTeam(v4);
                            } else {
                                return US8::US8_TeamMissing;
                            }
                        }
                        _ => {
                            return US8::US8_TeamMissing;
                        }
                    }
                } else {
                    v0 = v2;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US8::US8_AllTeams;
            }
        }
    }
}
fn method46(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method46(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method45(mut v0: Rc<UH4>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH4::UH4_Cons(v2, v3) => {
                let mut v2: Rc<UH0> = v2;
                let mut v3: Rc<UH4> = v3;
                let v4: Rc<UH0> = method46(v1, v2);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH4::UH4_Nil => {
                return v1;
            }
        }
    }
}
fn closure3(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US9> {
    Rc::new(move |mut v2: Rc<UH0>| -> US9 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US9::US9_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US9> = method49(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US9::US9_None
            }
        }
    })
}
fn method49(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US9> {
    closure3(v0, v1)
}
fn method48(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                let mut v5: i32 = 0i32;
                let mut v6: Rc<dyn Fn(Rc<UH0>) -> US9> = method49(v3.clone(), v5);
                let mut v7: US9 = v6(v1.clone());
                let mut v8: bool = match v7 {
                    US9::US9_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v9: bool = v8 != true;
                let mut v13: Rc<UH0> = if v9 {
                    v1
                } else {
                    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v10));
                    method46(v1, v11)
                };
                (v0, v1) = (v4, v13);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn epics_of_47(mut v0: Rc<UH1>) -> Rc<UH0> {
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method48(v0, v1)
}
fn epic_tags_51(mut v0: Rc<UH2>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: US7 = fact_for_34(v0, v1);
    match v2 {
        US7::US7_None => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v12
        }
        US7::US7_Some(v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let mut v5: Rc<str> = v5;
            let mut v6: i32 = v6;
            let mut v7: Rc<str> = v7;
            let mut v8: Rc<str> = v8;
            let mut v9: Rc<str> = v9;
            let v10: Rc<str> = v10;
            let v11: Rc<str> = v11;
            v5
        }
    }
}
fn method53(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method53(v3, v1);
            let mut v5: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn in_scope_52(mut v0: US8, mut v1: Rc<str>) -> bool {
    match v0 {
        US8::US8_OneTeam(v2) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v4: Rc<UH0> = split_list_11(v3, v1);
            let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v6: Rc<UH0> = method53(v4, v5);
            let mut v7: Rc<str> = string_concat(&["team:", &*v2]);
            let mut v8: i32 = 0i32;
            let mut v9: Rc<dyn Fn(Rc<UH0>) -> US9> = method49(v7, v8);
            let v10: US9 = v9(v6);
            let v11: bool = match v10 {
                US9::US9_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let mut v12: bool = v11 != true;
            v12
        }
        _ => {
            true
        }
    }
}
fn method50(mut v0: Rc<UH2>, mut v1: US8, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v4, v5) => {
            let v4: Rc<str> = v4;
            let mut v5: Rc<UH0> = v5;
            let mut v6: Rc<UH0> = method50(v0.clone(), v1.clone(), v5, v3);
            let mut v7: Rc<str> = epic_tags_51(v0, v4.clone());
            let mut v8: bool = in_scope_52(v1, v7);
            if v8 {
                Rc::new(UH0::UH0_Cons(v4, v6))
            } else {
                v6
            }
        }
        UH0::UH0_Nil => {
            v3
        }
    }
}
fn method56(mut v0: Rc<UH2>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32) -> (i32, i32) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: US7 = fact_for_34(v0.clone(), v4);
                let (mut v21, mut v22): (i32, i32) = match v6 {
                    US7::US7_Some(v7, v8, v9, v10, v11, v12, v13, v14, v15) => {
                        let mut v7: Rc<str> = v7;
                        let mut v8: Rc<str> = v8;
                        let mut v9: Rc<str> = v9;
                        let v10: i32 = v10;
                        let v11: Rc<str> = v11;
                        let mut v12: Rc<str> = v12;
                        let mut v13: Rc<str> = v13;
                        let v14: Rc<str> = v14;
                        let v15: Rc<str> = v15;
                        let v16: bool = &*v8 == "Done";
                        if v16 {
                            let v17: i32 = v2.wrapping_add(1i32);
                            let v18: i32 = v3.wrapping_add(v10);
                            (v17, v18)
                        } else {
                            (v2, v3)
                        }
                    }
                    _ => {
                        (v2, v3)
                    }
                };
                (v1, v2, v3) = (v5, v21, v22);
                continue;
            }
            UH0::UH0_Nil => {
                return (v2, v3);
            }
        }
    }
}
fn method57(mut v0: Rc<UH2>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32) -> (i32, i32) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: US7 = fact_for_34(v0.clone(), v4);
                let (mut v18, mut v19): (i32, i32) = match v6 {
                    US7::US7_Some(v7, v8, v9, v10, v11, v12, v13, v14, v15) => {
                        let mut v7: Rc<str> = v7;
                        let mut v8: Rc<str> = v8;
                        let mut v9: Rc<str> = v9;
                        let v10: i32 = v10;
                        let v11: Rc<str> = v11;
                        let mut v12: Rc<str> = v12;
                        let mut v13: Rc<str> = v13;
                        let v14: Rc<str> = v14;
                        let v15: Rc<str> = v15;
                        let v16: i32 = v2.wrapping_add(1i32);
                        let v17: i32 = v3.wrapping_add(v10);
                        (v16, v17)
                    }
                    _ => {
                        (v2, v3)
                    }
                };
                (v1, v2, v3) = (v5, v18, v19);
                continue;
            }
            UH0::UH0_Nil => {
                return (v2, v3);
            }
        }
    }
}
fn epic_line_55(mut v0: Rc<UH2>, mut v1: Rc<UH1>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<UH0> = children_of_29(v1.clone(), v2.clone());
    let v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let (mut v6, mut v7): (i32, i32) = method56(v0.clone(), v3, v4, v5);
    let mut v8: Rc<UH0> = children_of_29(v1, v2.clone());
    let mut v9: i32 = 0i32;
    let v10: i32 = 0i32;
    let (mut v11, mut v12): (i32, i32) = method57(v0, v8, v9, v10);
    let mut v13: Rc<str> = format_real_36(v6);
    let v16: Rc<str> = format_real_36(v11);
    let v20: Rc<str> = format_real_36(v7);
    let v23: Rc<str> = format_real_36(v12);
    let v24: Rc<str> = string_concat(&[&*v2, "  done=", &*v13, "/", &*v16, " minutes=", &*v20, "/", &*v23]);
    v24
}
fn method58(mut v0: Rc<UH2>, mut v1: Rc<UH3>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v4, v5) => {
            let v4: Rc<str> = v4;
            let mut v5: Rc<UH0> = v5;
            let mut v6: Rc<UH0> = method58(v0.clone(), v1.clone(), v5, v3);
            let mut v7: Rc<str> = child_line_33(v0, v4.clone());
            let mut v8: Rc<str> = titled_line_38(v1, v4, v7);
            Rc::new(UH0::UH0_Cons(v8, v6))
        }
        UH0::UH0_Nil => {
            v3
        }
    }
}
fn method54(mut v0: Rc<UH2>, mut v1: Rc<UH3>, mut v2: Rc<UH1>, mut v3: Rc<UH2>, mut v4: Rc<UH3>, mut v5: Rc<UH1>, mut v6: Rc<UH0>, mut v7: Rc<UH4>) -> Rc<UH4> {
    match Rc::unwrap_or_clone(v6) {
        UH0::UH0_Cons(v8, v9) => {
            let mut v8: Rc<str> = v8;
            let mut v9: Rc<UH0> = v9;
            let v10: Rc<UH4> = method54(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v9, v7);
            let v11: Rc<str> = epic_line_55(v0, v2, v8.clone());
            let mut v12: Rc<str> = titled_line_38(v1, v8.clone(), v11);
            let mut v13: Rc<UH0> = children_of_29(v5, v8);
            let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v15: Rc<UH0> = method58(v3, v4, v13, v14);
            let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v15));
            Rc::new(UH4::UH4_Cons(v16, v10))
        }
        UH0::UH0_Nil => {
            v7
        }
    }
}
fn tree_text_of_44(mut v0: Rc<UH2>, mut v1: Rc<UH3>, mut v2: US8, mut v3: Rc<UH1>) -> Rc<UH0> {
    let v4: Rc<UH0> = epics_of_47(v3.clone());
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = method50(v0.clone(), v2, v4, v5);
    let mut v7: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH4> = method54(v0.clone(), v1.clone(), v3.clone(), v0, v1, v3, v6, v7);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method45(v8, v9)
}
fn method59(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<UH0> = v2;
                println!("{}", v1);
                v0 = v2;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method60(mut v0: Rc<UH2>, mut v1: US8, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v4, v5) => {
            let v4: Rc<str> = v4;
            let mut v5: Rc<UH0> = v5;
            let mut v6: Rc<UH0> = method60(v0.clone(), v1.clone(), v5, v3);
            let mut v7: Rc<str> = epic_tags_51(v0, v4.clone());
            let mut v8: bool = in_scope_52(v1, v7);
            if v8 {
                Rc::new(UH0::UH0_Cons(v4, v6))
            } else {
                v6
            }
        }
        UH0::UH0_Nil => {
            v3
        }
    }
}
fn method61(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let mut v5: Rc<UH1> = v5;
            let mut v6: Rc<UH1> = method61(v0.clone(), v5, v2);
            let mut v7: i32 = 0i32;
            let mut v8: Rc<dyn Fn(Rc<UH0>) -> US9> = method49(v4.clone(), v7);
            let mut v9: US9 = v8(v0);
            let v10: bool = match v9 {
                US9::US9_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v11: bool = v10 != true;
            if v11 {
                Rc::new(UH1::UH1_Cons(v3, v4, v6))
            } else {
                v6
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn method62(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                let mut v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn tree_run_42() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_2(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let mut v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                            let v10: Rc<str> = Rc::<str>::from(v6.replace(&*v8, &*v9));
                            let v11: Rc<str> = string_concat(&[&*v10, "/state/parents.spi"]);
                            let mut v12: Rc<str> = file_text_5(v11);
                            let mut v13: Rc<UH1> = links_of_text_16(v12);
                            let v14: Rc<str> = string_concat(&[&*v10, "/state/agile.spi"]);
                            let v15: Rc<str> = file_text_5(v14);
                            let v16: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = agile_quality_facts::rig_agile_fact_rows(&v15);
                            let v17: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new((v16).borrow().clone());
                            let v18: usize = (v17).len();
                            let v19: i32 = ((v18) as i32);
                            let v20: i32 = v19.wrapping_sub(1i32);
                            let v21: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                            let v22: Rc<UH2> = method22(v17, v20, v21);
                            let v23: i32 = 0i32;
                            let v24: Rc<UH0> = arguments_from_2(v23);
                            let v25: US8 = team_after_43(v24);
                            match &v25 {
                                US8::US8_TeamMissing => {
                                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile list <root> --tree [--team <team>]"); } LIT.with(|lit| lit.clone()) };
                                    eprintln!("{}", v26);
                                    2i32
                                }
                                _ => {
                                    let v27: Rc<UH3> = titles_of_23(v15);
                                    let v28: Rc<UH0> = tree_text_of_44(v22.clone(), v27, v25.clone(), v13.clone());
                                    method59(v28);
                                    let v29: Rc<UH0> = epics_of_47(v13.clone());
                                    let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v31: Rc<UH0> = method60(v22, v25, v29, v30);
                                    let v32: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                                    let v33: Rc<UH1> = method61(v31.clone(), v13, v32);
                                    let v34: i32 = 0i32;
                                    let v35: i32 = method41(v31, v34);
                                    let v36: Rc<str> = format_real_36(v35);
                                    let v37: i32 = 0i32;
                                    let v38: i32 = method62(v33, v37);
                                    let v39: Rc<str> = format_real_36(v38);
                                    let v42: Rc<str> = string_concat(&["rig agile list ok tree epics=", &*v36, " links=", &*v39]);
                                    println!("{}", v42);
                                    0i32
                                }
                            }
                        }
                        _ => {
                            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile list <root> --tree"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v44);
                            2i32
                        }
                    }
                }
                _ => {
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile list <root> --tree"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v46);
                    2i32
                }
            }
        }
        _ => {
            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile list <root> --tree"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v48);
            2i32
        }
    }
}
fn parent_after_63(mut v0: Rc<UH0>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<UH0> = v2;
                match &*v2 {
                    UH0::UH0_Cons(v3, v4) => {
                        let mut v3: Rc<str> = v3.clone();
                        let v4: Rc<UH0> = v4.clone();
                        let mut v5: bool = &*v1 == "--parent";
                        if v5 {
                            return v3;
                        } else {
                            v0 = v2;
                            continue;
                        }
                    }
                    _ => {
                        v0 = v2;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v10;
            }
        }
    }
}
fn closure4(mut v0: i32) -> Rc<dyn Fn(Rc<UH0>) -> US9> {
    Rc::new(move |mut v1: Rc<UH0>| -> US9 {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let mut v5: bool = &*v3 == "--tree";
                if v5 {
                    US9::US9_Some(v0)
                } else {
                    let mut v7: i32 = v0.wrapping_add(1i32);
                    let mut v8: Rc<dyn Fn(Rc<UH0>) -> US9> = method65(v7);
                    v8(v4)
                }
            }
            UH0::UH0_Nil => {
                US9::US9_None
            }
        }
    })
}
fn method65(mut v0: i32) -> Rc<dyn Fn(Rc<UH0>) -> US9> {
    closure4(v0)
}
fn tree_wanted_64() -> bool {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_2(v0);
    let mut v2: i32 = 0i32;
    let mut v3: Rc<dyn Fn(Rc<UH0>) -> US9> = method65(v2);
    let v4: US9 = v3(v1);
    let mut v5: bool = match v4 {
        US9::US9_None => {
            true
        }
        _ => {
            false
        }
    };
    let mut v6: bool = v5 != true;
    v6
}
fn list_routed_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_2(v0);
    let mut v2: Rc<str> = parent_after_63(v1);
    let mut v3: bool = &*v2 != "";
    if v3 {
        parent_run_1(v2)
    } else {
        let mut v5: bool = tree_wanted_64();
        if v5 {
            tree_run_42()
        } else {
            let mut v7: i32 = agile_task_listing::rig_agile_list_run();
            v7
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        list_routed_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_list_routed() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
