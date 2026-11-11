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
pub struct Mut1 { pub l0: Rc<str> }
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_1(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_2();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_3(v0);
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
            let mut v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_1(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
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
fn method6(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
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
fn loop_8(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
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
fn loop_9(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
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
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn method12(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method12(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
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
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method13(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let mut v15: US2 = if v6 {
                let v7: US3 = US3::US3_HeadType;
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
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
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
fn method14(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn state_text_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = string_concat(&[&*v3, "/state/agile.spi"]);
    let v5: bool = std::path::Path::new(&*v4).is_file();
    if v5 {
        let (mut v6, mut v7): (bool, Rc<str>) = match std::fs::read(&*v4) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v6 {
            let v8: Rc<str> = string_concat(&["rig state read failed: ", &*v4]);
            std::panic::panic_any::<std::string::String>(format!("{}", v8))
        } else {
            let v10: Rc<str> = Rc::<str>::from(v4.replace(&*v1, &*v2));
            let mut v11: Rc<str> = loop_5(v10);
            let v12: Option<usize> = v11.rfind(&*v2);
            let v13: Option<usize> = method6(v12);
            let mut v14: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v15: Option<i32> = v13.map(|x| v14(x));
            let v17: i32 = v15.unwrap_or(-1i32);
            let v18: bool = v17 < 0i32;
            let v22: bool = if v18 {
                true
            } else {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v20: bool = v11.ends_with(&*v19);
                let v21: bool = v20 == false;
                v21
            };
            let v43: US1 = if v22 {
                US1::US1_OtherFile
            } else {
                let v24: usize = ((0i32) as usize);
                let v25: usize = ((v17) as usize);
                let v26: Rc<str> = Rc::<str>::from(&(v11)[v24..v25]);
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v28: bool = v26.ends_with(&*v27);
                if v28 {
                    let v29: Rc<str> = loop_7(v26);
                    US1::US1_StateFile(v29)
                } else {
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v32: bool = v26.ends_with(&*v31);
                    if v32 {
                        let v33: Rc<str> = loop_8(v26);
                        US1::US1_HistoryFile(v33)
                    } else {
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v36: bool = v26.ends_with(&*v35);
                        if v36 {
                            let v37: Rc<str> = loop_9(v26);
                            US1::US1_ArchiveFile(v37)
                        } else {
                            US1::US1_OtherFile
                        }
                    }
                }
            };
            match v43 {
                US1::US1_OtherFile => {
                    v7
                }
                _ => {
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v45: Rc<UH0> = split_list_10(v44.clone(), v7);
                    let v46: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v47: Rc<UH0> = method13(v45, v46);
                    let v48: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v49: i32 = 0i32;
                    let v50: i32 = method14(v48.clone(), v47, v49);
                    let v51: Rc<Vec<Rc<str>>> = Rc::new(v48.borrow().clone());
                    let v52: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v51).as_ref().clone()));
                    let v53: Rc<str> = Rc::<str>::from((*v52.borrow()).join(&*v44));
                    v53
                }
            }
        }
    } else {
        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v56
    }
}
fn method17(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_16(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method17(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn census_printed_15(mut v0: &Rc<RefCell<Vec<(Rc<str>, i32)>>>, mut v1: i32) -> () {
    loop {
        let v2: i32 = (v0.clone().borrow().len() as i32);
        let v3: bool = v1 < v2;
        if v3 {
            let (mut v4, mut v5): (Rc<str>, i32) = v0.borrow()[v1 as usize].clone();
            let v8: Rc<str> = format_real_16(v5);
            let mut v9: Rc<str> = string_concat(&["rig agile lint census check=", &*v4, " items=", &*v8]);
            println!("{}", v9);
            let v10: i32 = v1.wrapping_add(1i32);
            v1 = v10;
            continue;
        }
        return ();
    }
}
fn format_real_18(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method17(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn printed_19(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: i32) -> () {
    loop {
        let v2: i32 = (v0.clone().borrow().len() as i32);
        let v3: bool = v1 < v2;
        if v3 {
            let (mut v4, mut v5, mut v6): (Rc<str>, Rc<str>, Rc<str>) = v0.borrow()[v1 as usize].clone();
            let v10: Rc<str> = string_concat(&[&*v4, " ", &*v5, " ", &*v6]);
            println!("{}", v10);
            let mut v11: i32 = v1.wrapping_add(1i32);
            v1 = v11;
            continue;
        }
        return ();
    }
}
fn distinct_ids_20(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: i32, mut v2: Rc<str>, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            return v3;
        } else {
            let (mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, Rc<str>) = v0.borrow()[v1 as usize].clone();
            let mut v9: i32 = v1.wrapping_add(1i32);
            let v10: bool = &*v6 == &*v2;
            let v12: i32 = if v10 {
                v3
            } else {
                let mut v11: i32 = v3.wrapping_add(1i32);
                v11
            };
            (v1, v2, v3) = (v9, v6, v12);
            continue;
        }
    }
}
fn lint_run_0() -> i32 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: i32 = 0i32;
    let v2: Rc<UH0> = arguments_from_1(v1);
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH0::UH0_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let v6: Rc<UH0> = v6;
                    match Rc::unwrap_or_clone(v6) {
                        UH0::UH0_Cons(v7, v8) => {
                            let v7: Rc<str> = v7;
                            let v8: Rc<UH0> = v8;
                            match Rc::unwrap_or_clone(v8) {
                                UH0::UH0_Cons(v9, v10) => {
                                    let mut v9: Rc<str> = v9;
                                    let v10: Rc<UH0> = v10;
                                    let mut v11: bool = &*v9 == "--census";
                                    if v11 {
                                        let v12: Rc<str> = state_text_4(v7);
                                        let v13: Rc<RefCell<Vec<(Rc<str>, i32)>>> = agile_quality_state::rig_agile_lint_census(&v12);
                                        let mut v14: i32 = 0i32;
                                        census_printed_15(&v13, v14);
                                        let mut v15: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                        let v16: Rc<str> = format_real_18(v15);
                                        let v17: Rc<str> = string_concat(&["rig agile lint census ok elapsed_ms=", &*v16]);
                                        println!("{}", v17);
                                        0i32
                                    } else {
                                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                        let v19: bool = v7.starts_with(&*v18);
                                        let v20: bool = v19 == false;
                                        if v20 {
                                            let v27: Rc<str> = match Rc::unwrap_or_clone(v10) {
                                                UH0::UH0_Cons(v21, v22) => {
                                                    let v21: Rc<str> = v21;
                                                    let v22: Rc<UH0> = v22;
                                                    let v23: bool = &*v9 == "--team";
                                                    if v23 {
                                                        v21
                                                    } else {
                                                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        v24
                                                    }
                                                }
                                                _ => {
                                                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    v26
                                                }
                                            };
                                            let v28: Rc<str> = state_text_4(v7);
                                            let v29: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = agile_quality_state::rig_agile_lint_team_failures(&v28, &v27);
                                            let v30: i32 = 0i32;
                                            printed_19(&v29, v30);
                                            let v31: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                            let v32: i32 = (v29.clone().borrow().len() as i32);
                                            let v33: i32 = 0i32;
                                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let v35: i32 = 0i32;
                                            let v36: i32 = distinct_ids_20(&v29, v33, v34.clone(), v35);
                                            let v37: Rc<str> = format_real_16(v36);
                                            let v38: Rc<str> = format_real_16(v32);
                                            let v39: Rc<str> = format_real_18(v31);
                                            let mut v40: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig agile lint ok failing_items="); } LIT.with(|lit| lit.clone()) };
                                            v40.borrow_mut().push(v41);
                                            v40.borrow_mut().push(v37);
                                            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failing_checks="); } LIT.with(|lit| lit.clone()) };
                                            v40.borrow_mut().push(v42);
                                            v40.borrow_mut().push(v38);
                                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (report only; legacy items ratchet) elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                            v40.borrow_mut().push(v43);
                                            v40.borrow_mut().push(v39);
                                            let v44: Rc<Vec<Rc<str>>> = Rc::new(v40.borrow().clone());
                                            let v45: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v44).as_ref().clone()));
                                            let v46: Rc<str> = Rc::<str>::from((*v45.borrow()).join(&*v34));
                                            println!("{}", v46);
                                            0i32
                                        } else {
                                            let v47: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                            let v48: Rc<str> = format_real_18(v47);
                                            let v49: Rc<str> = string_concat(&["usage: rig agile lint <root> [--census | --team <team>] elapsed_ms=", &*v48]);
                                            eprintln!("{}", v49);
                                            2i32
                                        }
                                    }
                                }
                                _ => {
                                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                    let v53: bool = v7.starts_with(&*v52);
                                    let v54: bool = v53 == false;
                                    if v54 {
                                        let v55: Rc<str> = state_text_4(v7);
                                        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v57: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = agile_quality_state::rig_agile_lint_team_failures(&v55, &v56);
                                        let v58: i32 = 0i32;
                                        printed_19(&v57, v58);
                                        let v59: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                        let v60: i32 = (v57.clone().borrow().len() as i32);
                                        let v61: i32 = 0i32;
                                        let v62: i32 = 0i32;
                                        let v63: i32 = distinct_ids_20(&v57, v61, v56.clone(), v62);
                                        let v64: Rc<str> = format_real_16(v63);
                                        let v65: Rc<str> = format_real_16(v60);
                                        let v66: Rc<str> = format_real_18(v59);
                                        let mut v67: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                        let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig agile lint ok failing_items="); } LIT.with(|lit| lit.clone()) };
                                        v67.borrow_mut().push(v68);
                                        v67.borrow_mut().push(v64);
                                        let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failing_checks="); } LIT.with(|lit| lit.clone()) };
                                        v67.borrow_mut().push(v69);
                                        v67.borrow_mut().push(v65);
                                        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (report only; legacy items ratchet) elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                        v67.borrow_mut().push(v70);
                                        v67.borrow_mut().push(v66);
                                        let v71: Rc<Vec<Rc<str>>> = Rc::new(v67.borrow().clone());
                                        let v72: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v71).as_ref().clone()));
                                        let v73: Rc<str> = Rc::<str>::from((*v72.borrow()).join(&*v56));
                                        println!("{}", v73);
                                        0i32
                                    } else {
                                        let v74: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                        let v75: Rc<str> = format_real_18(v74);
                                        let v76: Rc<str> = string_concat(&["usage: rig agile lint <root> [--census | --team <team>] elapsed_ms=", &*v75]);
                                        eprintln!("{}", v76);
                                        2i32
                                    }
                                }
                            }
                        }
                        _ => {
                            let v79: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                            let v80: Rc<str> = format_real_18(v79);
                            let v81: Rc<str> = string_concat(&["usage: rig agile lint <root> [--census | --team <team>] elapsed_ms=", &*v80]);
                            eprintln!("{}", v81);
                            2i32
                        }
                    }
                }
                _ => {
                    let v83: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                    let v84: Rc<str> = format_real_18(v83);
                    let v85: Rc<str> = string_concat(&["usage: rig agile lint <root> [--census | --team <team>] elapsed_ms=", &*v84]);
                    eprintln!("{}", v85);
                    2i32
                }
            }
        }
        _ => {
            let v87: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
            let v88: Rc<str> = format_real_18(v87);
            let v89: Rc<str> = string_concat(&["usage: rig agile lint <root> [--census | --team <team>] elapsed_ms=", &*v88]);
            eprintln!("{}", v89);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        lint_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_lint_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
