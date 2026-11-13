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
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_Some(i64),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_NoteRow,
    US1_ParentRow,
    US1_AskRow,
    US1_AfterRow,
}
#[derive(Clone)]
enum US2 {
    US2_StateFile(Rc<str>),
    US2_HistoryFile(Rc<str>),
    US2_ArchiveFile(Rc<str>),
    US2_OtherFile,
}
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
    US5_Some(Rc<str>),
    US5_None,
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method5(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method5(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method4(v16, v17)
    }
}
fn method7(mut v0: Option<i64>) -> Option<i64> {
    v0
}
fn closure1() -> Rc<dyn Fn((i64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i64)) -> US0> = Rc::new(move |mut v0: (i64)| -> US0 {
        let v1: i64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn stamp_hour_6(mut v0: Rc<str>) -> i64 {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: bool = v1 < 13i32;
    if v2 {
        0i64
    } else {
        let v3: usize = ((0i32) as usize);
        let v4: usize = ((4i32) as usize);
        let v5: Rc<str> = Rc::<str>::from(&(v0)[v3..v4]);
        let v6: Option<i64> = (v5).trim().parse::<i64>().ok();
        let mut v7: Option<i64> = method7(v6);
        let mut v8: Rc<dyn Fn((i64)) -> US0> = closure1();
        let mut v9: Option<US0> = v7.map(|x| v8(x));
        let v11: US0 = US0::US0_None;
        let mut v12: US0 = v11.clone();
        let v13: US0 = v9.unwrap_or(v12);
        let mut v16: i64 = match v13 {
            US0::US0_None => {
                0i64
            }
            US0::US0_Some(v14) => {
                let v14: i64 = v14;
                v14
            }
        };
        let v17: usize = ((5i32) as usize);
        let v18: usize = ((7i32) as usize);
        let v19: Rc<str> = Rc::<str>::from(&(v0)[v17..v18]);
        let mut v20: Option<i64> = (v19).trim().parse::<i64>().ok();
        let v21: Option<i64> = method7(v20);
        let mut v22: Option<US0> = v21.map(|x| v8(x));
        let mut v24: US0 = US0::US0_None;
        let mut v25: US0 = v24.clone();
        let mut v26: US0 = v22.unwrap_or(v25);
        let v29: i64 = match v26 {
            US0::US0_None => {
                0i64
            }
            US0::US0_Some(v27) => {
                let mut v27: i64 = v27;
                v27
            }
        };
        let v30: usize = ((8i32) as usize);
        let v31: usize = ((10i32) as usize);
        let v32: Rc<str> = Rc::<str>::from(&(v0)[v30..v31]);
        let v33: Option<i64> = (v32).trim().parse::<i64>().ok();
        let v34: Option<i64> = method7(v33);
        let mut v35: Option<US0> = v34.map(|x| v8(x));
        let v37: US0 = US0::US0_None;
        let mut v38: US0 = v37.clone();
        let v39: US0 = v35.unwrap_or(v38);
        let v42: i64 = match v39 {
            US0::US0_None => {
                0i64
            }
            US0::US0_Some(v40) => {
                let v40: i64 = v40;
                v40
            }
        };
        let v43: bool = v29 <= 2i64;
        let v45: i64 = if v43 {
            let v44: i64 = v16.wrapping_sub(1i64);
            v44
        } else {
            v16
        };
        let v46: i64 = v45.wrapping_div(400i64);
        let v47: i64 = v45.wrapping_rem(400i64);
        let v48: bool = v47 != 0i64;
        let mut v50: bool = if v48 {
            let v49: bool = v45 < 0i64;
            v49
        } else {
            false
        };
        let v52: i64 = if v50 {
            let v51: i64 = v46.wrapping_sub(1i64);
            v51
        } else {
            v46
        };
        let v53: i64 = v52.wrapping_mul(400i64);
        let v54: i64 = v45.wrapping_sub(v53);
        let v55: i64 = v54.wrapping_mul(365i64);
        let v56: i64 = v54.wrapping_div(4i64);
        let v57: i64 = v55.wrapping_add(v56);
        let v58: i64 = v54.wrapping_div(100i64);
        let v59: i64 = v57.wrapping_sub(v58);
        let v60: bool = v29 > 2i64;
        let v63: i64 = if v60 {
            let v61: i64 = v29.wrapping_sub(3i64);
            v61
        } else {
            let v62: i64 = v29.wrapping_add(9i64);
            v62
        };
        let v64: i64 = 153i64.wrapping_mul(v63);
        let v65: i64 = v64.wrapping_add(2i64);
        let v66: i64 = v65.wrapping_div(5i64);
        let v67: i64 = v66.wrapping_add(v42);
        let v68: i64 = v67.wrapping_sub(1i64);
        let v69: i64 = v59.wrapping_add(v68);
        let v70: i64 = v52.wrapping_mul(146097i64);
        let v71: i64 = v70.wrapping_add(v69);
        let v72: i64 = v71.wrapping_sub(719468i64);
        let v73: i64 = v72.wrapping_mul(24i64);
        let v74: usize = ((11i32) as usize);
        let v75: usize = ((13i32) as usize);
        let v76: Rc<str> = Rc::<str>::from(&(v0)[v74..v75]);
        let v77: Option<i64> = (v76).trim().parse::<i64>().ok();
        let v78: Option<i64> = method7(v77);
        let mut v79: Option<US0> = v78.map(|x| v8(x));
        let v81: US0 = US0::US0_None;
        let v82: US0 = v81.clone();
        let v83: US0 = v79.unwrap_or(v82);
        let v86: i64 = match v83 {
            US0::US0_None => {
                0i64
            }
            US0::US0_Some(v84) => {
                let v84: i64 = v84;
                v84
            }
        };
        let v87: i64 = v73.wrapping_add(v86);
        v87
    }
}
fn later_note_2(mut v0: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i64>>, mut v1: Rc<str>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, i64>> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH0> = split_list_3(v2, v1);
    match Rc::unwrap_or_clone(v3) {
        UH0::UH0_Cons(v4, v5) => {
            let v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            match Rc::unwrap_or_clone(v5) {
                UH0::UH0_Cons(v6, v7) => {
                    let v6: Rc<str> = v6;
                    let mut v7: Rc<UH0> = v7;
                    match Rc::unwrap_or_clone(v7) {
                        UH0::UH0_Cons(v8, v9) => {
                            let mut v8: Rc<str> = v8;
                            let mut v9: Rc<UH0> = v9;
                            match Rc::unwrap_or_clone(v9) {
                                UH0::UH0_Cons(v10, v11) => {
                                    let v10: Rc<str> = v10;
                                    let v11: Rc<UH0> = v11;
                                    let mut v12: i64 = stamp_hour_6(v10);
                                    let v13: Option<i64> = v0.get(&v6).cloned();
                                    let v14: Option<i64> = method7(v13);
                                    let mut v15: Rc<dyn Fn((i64)) -> US0> = closure1();
                                    let mut v16: Option<US0> = v14.map(|x| v15(x));
                                    let v18: US0 = US0::US0_None;
                                    let v19: US0 = v18.clone();
                                    let mut v20: US0 = v16.unwrap_or(v19);
                                    let mut v23: i64 = match v20 {
                                        US0::US0_None => {
                                            0i64
                                        }
                                        US0::US0_Some(v21) => {
                                            let v21: i64 = v21;
                                            v21
                                        }
                                    };
                                    let mut v24: bool = v12 > v23;
                                    if v24 {
                                        let mut v25: std::collections::BTreeMap<Rc<str>, i64> = std::rc::Rc::unwrap_or_clone(v0);
                                        let mut v27: bool = true; // 1;
                                        let mut v25 = v25;
                                        let v29: Option<i64> = v25.insert(v6, v12);
                                        let v30: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i64>> = std::rc::Rc::new(v25);
                                        v30
                                    } else {
                                        v0
                                    }
                                }
                                _ => {
                                    v0
                                }
                            }
                        }
                        _ => {
                            v0
                        }
                    }
                }
                _ => {
                    v0
                }
            }
        }
        _ => {
            v0
        }
    }
}
fn note_hours_from_1(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i64>>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, i64>> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl note_of_"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = v5.starts_with(&*v6);
            let mut v9: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i64>> = if v7 {
                later_note_2(v2, v5)
            } else {
                v2
            };
            let v10: i32 = v1.wrapping_add(1i32);
            (v1, v2) = (v10, v9);
            continue;
        }
    }
}
fn pair_of_line_10(mut v0: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v1: Rc<str>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH0> = split_list_3(v2, v1);
    match Rc::unwrap_or_clone(v3) {
        UH0::UH0_Cons(v4, v5) => {
            let v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            match Rc::unwrap_or_clone(v5) {
                UH0::UH0_Cons(v6, v7) => {
                    let v6: Rc<str> = v6;
                    let mut v7: Rc<UH0> = v7;
                    match Rc::unwrap_or_clone(v7) {
                        UH0::UH0_Cons(v8, v9) => {
                            let mut v8: Rc<str> = v8;
                            let mut v9: Rc<UH0> = v9;
                            match Rc::unwrap_or_clone(v9) {
                                UH0::UH0_Cons(v10, v11) => {
                                    let v10: Rc<str> = v10;
                                    let v11: Rc<UH0> = v11;
                                    let mut v12: std::collections::BTreeMap<Rc<str>, Rc<str>> = std::rc::Rc::unwrap_or_clone(v0);
                                    let v14: bool = true; // 1;
                                    let mut v12 = v12;
                                    let mut v16: Option<Rc<str>> = v12.insert(v6, v10);
                                    let v17: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> = std::rc::Rc::new(v12);
                                    v17
                                }
                                _ => {
                                    v0
                                }
                            }
                        }
                        _ => {
                            v0
                        }
                    }
                }
                _ => {
                    v0
                }
            }
        }
        _ => {
            v0
        }
    }
}
fn pairs_from_9(mut v0: US1, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> {
    loop {
        let v4: i32 = (v1.clone().borrow().len() as i32);
        let v5: bool = v4 <= v2;
        if v5 {
            return v3;
        } else {
            let v6: Rc<str> = v1.borrow()[v2 as usize].clone();
            let v14: Rc<str> = match &v0 {
                US1::US1_AfterRow => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl after_of_"); } LIT.with(|lit| lit.clone()) };
                    v10
                }
                US1::US1_AskRow => {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl asked_of_"); } LIT.with(|lit| lit.clone()) };
                    v9
                }
                US1::US1_NoteRow => {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl note_of_"); } LIT.with(|lit| lit.clone()) };
                    v7
                }
                US1::US1_ParentRow => {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl parent_of_"); } LIT.with(|lit| lit.clone()) };
                    v8
                }
            };
            let mut v15: bool = v6.starts_with(&*v14);
            let v17: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> = if v15 {
                pair_of_line_10(v3, v6)
            } else {
                v3
            };
            let v18: i32 = v2.wrapping_add(1i32);
            (v2, v3) = (v18, v17);
            continue;
        }
    }
}
fn loop_11(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
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
fn method12(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_13(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
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
fn loop_14(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
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
fn loop_15(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
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
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method16(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let mut v15: US3 = if v6 {
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
            let mut v27: US3 = match &v15 {
                US3::US3_OtherLine => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US4 = US4::US4_HeadInl;
                        US3::US3_KeywordHead(v18)
                    } else {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
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
fn method17(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn state_pairs_8(mut v0: Rc<str>, mut v1: US1) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> {
    let v2: Rc<str> = string_concat(&[&*v0, "/state/"]);
    let v10: Rc<str> = match &v1 {
        US1::US1_AfterRow => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("afters.spi"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US1::US1_AskRow => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("asks.spi"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US1::US1_NoteRow => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile.spi"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US1::US1_ParentRow => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("parents.spi"); } LIT.with(|lit| lit.clone()) };
            v4
        }
    };
    let v11: Rc<str> = string_concat(&[&*v2, &*v10]);
    let mut v12: bool = std::path::Path::new(&*v11).is_file();
    let v66: Rc<str> = if v12 {
        let (mut v13, mut v14): (bool, Rc<str>) = match std::fs::read(&*v11) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v13 {
            let mut v15: Rc<str> = string_concat(&["rig state read failed: ", &*v11]);
            std::panic::panic_any::<std::string::String>(format!("{}", v15))
        } else {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = Rc::<str>::from(v11.replace(&*v17, &*v18));
            let mut v20: Rc<str> = loop_11(v19);
            let v21: Option<usize> = v20.rfind(&*v18);
            let mut v22: Option<usize> = method12(v21);
            let mut v23: Rc<dyn Fn((usize)) -> i32> = closure2();
            let mut v24: Option<i32> = v22.map(|x| v23(x));
            let mut v26: i32 = v24.unwrap_or(-1i32);
            let mut v27: bool = v26 < 0i32;
            let v31: bool = if v27 {
                true
            } else {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v29: bool = v20.ends_with(&*v28);
                let v30: bool = v29 == false;
                v30
            };
            let v52: US2 = if v31 {
                US2::US2_OtherFile
            } else {
                let v33: usize = ((0i32) as usize);
                let v34: usize = ((v26) as usize);
                let mut v35: Rc<str> = Rc::<str>::from(&(v20)[v33..v34]);
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v37: bool = v35.ends_with(&*v36);
                if v37 {
                    let mut v38: Rc<str> = loop_13(v35);
                    US2::US2_StateFile(v38)
                } else {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v35.ends_with(&*v40);
                    if v41 {
                        let v42: Rc<str> = loop_14(v35);
                        US2::US2_HistoryFile(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v35.ends_with(&*v44);
                        if v45 {
                            let v46: Rc<str> = loop_15(v35);
                            US2::US2_ArchiveFile(v46)
                        } else {
                            US2::US2_OtherFile
                        }
                    }
                }
            };
            match v52 {
                US2::US2_OtherFile => {
                    v14
                }
                _ => {
                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v54: Rc<UH0> = split_list_3(v53.clone(), v14);
                    let v55: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v56: Rc<UH0> = method16(v54, v55);
                    let v57: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v58: i32 = 0i32;
                    let v59: i32 = method17(v57.clone(), v56, v58);
                    let v60: Rc<Vec<Rc<str>>> = Rc::new(v57.borrow().clone());
                    let v61: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v60).as_ref().clone()));
                    let v62: Rc<str> = Rc::<str>::from((*v61.borrow()).join(&*v53));
                    v62
                }
            }
        }
    } else {
        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v65
    };
    let v67: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v66.lines().map(Rc::<str>::from).collect::<Vec<Rc<str>>>()));
    let v68: i32 = 0i32;
    let v70: std::collections::BTreeMap<Rc<str>, Rc<str>> = std::collections::BTreeMap::new();
    let v71: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> = std::rc::Rc::new(v70);
    pairs_from_9(v1, v67, v68, v71)
}
fn method20(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure3() -> Rc<dyn Fn((Rc<str>)) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US5> = Rc::new(move |mut v0: (Rc<str>)| -> US5 {
        let v1: Rc<str> = (v0);
        US5::US5_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn annotation_for_19(mut v0: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i64>>, mut v1: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v2: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v3: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v4: i64, mut v5: Rc<str>) -> (Rc<str>, i32, Rc<str>, bool, Rc<str>) {
    let v6: Option<i64> = v0.get(&v5).cloned();
    let mut v7: Option<i64> = method7(v6);
    let mut v8: Rc<dyn Fn((i64)) -> US0> = closure1();
    let mut v9: Option<US0> = v7.map(|x| v8(x));
    let v11: US0 = US0::US0_None;
    let mut v12: US0 = v11.clone();
    let v13: US0 = v9.unwrap_or(v12);
    let mut v16: i64 = match v13 {
        US0::US0_None => {
            0i64
        }
        US0::US0_Some(v14) => {
            let v14: i64 = v14;
            v14
        }
    };
    let v17: bool = v16 <= 0i64;
    let v19: bool = if v17 {
        true
    } else {
        let v18: bool = v4 <= v16;
        v18
    };
    let mut v23: i32 = if v19 {
        0i32
    } else {
        let mut v20: i64 = v4.wrapping_sub(v16);
        let v21: i64 = v20.wrapping_div(24i64);
        let mut v22: i32 = (v21 as i32);
        v22
    };
    let mut v24: Option<Rc<str>> = v1.get(&v5).cloned();
    let mut v25: Option<Rc<str>> = method20(v24);
    let mut v26: Rc<dyn Fn((Rc<str>)) -> US5> = closure3();
    let mut v27: Option<US5> = v25.map(|x| v26(x));
    let v29: US5 = US5::US5_None;
    let v30: US5 = v29.clone();
    let v31: US5 = v27.unwrap_or(v30);
    let mut v35: Rc<str> = match v31 {
        US5::US5_None => {
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v33
        }
        US5::US5_Some(v32) => {
            let v32: Rc<str> = v32;
            v32
        }
    };
    let v36: Option<Rc<str>> = v2.get(&v5).cloned();
    let v37: Option<Rc<str>> = method20(v36);
    let mut v38: Option<US5> = v37.map(|x| v26(x));
    let v40: US5 = US5::US5_None;
    let v41: US5 = v40.clone();
    let v42: US5 = v38.unwrap_or(v41);
    let v46: Rc<str> = match v42 {
        US5::US5_None => {
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v44
        }
        US5::US5_Some(v43) => {
            let v43: Rc<str> = v43;
            v43
        }
    };
    let v47: bool = &*v46 != "";
    let v48: Option<Rc<str>> = v3.get(&v5).cloned();
    let v49: Option<Rc<str>> = method20(v48);
    let mut v50: Option<US5> = v49.map(|x| v26(x));
    let v52: US5 = US5::US5_None;
    let v53: US5 = v52.clone();
    let v54: US5 = v50.unwrap_or(v53);
    let v58: Rc<str> = match v54 {
        US5::US5_None => {
            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v56
        }
        US5::US5_Some(v55) => {
            let v55: Rc<str> = v55;
            v55
        }
    };
    (v5, v23, v35, v47, v58)
}
fn annotate_into_18(mut v0: &Rc<RefCell<Vec<(Rc<str>, i32, Rc<str>, bool, Rc<str>)>>>, mut v1: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i64>>, mut v2: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v3: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v4: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v5: i64, mut v6: &Rc<RefCell<Vec<Rc<str>>>>, mut v7: i32) -> () {
    loop {
        let mut v8: i32 = (v6.clone().borrow().len() as i32);
        let mut v9: bool = v8 <= v7;
        if v9 {
            return ();
        } else {
            let v10: Rc<str> = v6.borrow()[v7 as usize].clone();
            let (mut v11, mut v12, mut v13, mut v14, mut v15): (Rc<str>, i32, Rc<str>, bool, Rc<str>) = annotation_for_19(v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5, v10);
            v0.borrow_mut()[v7 as usize] = (v11, v12, v13, v14, v15);
            let mut v16: i32 = v7.wrapping_add(1i32);
            v7 = v16;
            continue;
        }
        return ();
    }
}
fn annotations_of_0(mut v0: i64, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i32, Rc<str>, bool, Rc<str>)>>> {
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v2.lines().map(Rc::<str>::from).collect::<Vec<Rc<str>>>()));
    let v5: i32 = 0i32;
    let mut v7: std::collections::BTreeMap<Rc<str>, i64> = std::collections::BTreeMap::new();
    let mut v8: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i64>> = std::rc::Rc::new(v7);
    let mut v9: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i64>> = note_hours_from_1(&v4, v5, v8);
    let v10: US1 = US1::US1_ParentRow;
    let v11: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> = state_pairs_8(v3.clone(), v10);
    let mut v12: US1 = US1::US1_AskRow;
    let v13: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> = state_pairs_8(v3.clone(), v12);
    let v14: US1 = US1::US1_AfterRow;
    let mut v15: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> = state_pairs_8(v3, v14);
    let mut v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.lines().map(Rc::<str>::from).collect::<Vec<Rc<str>>>()));
    let v17: i32 = (v16.clone().borrow().len() as i32);
    let v18: Rc<RefCell<Vec<(Rc<str>, i32, Rc<str>, bool, Rc<str>)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, i32, Rc<str>, bool, Rc<str>)>::default(); v17 as usize]));
    let v19: i32 = 0i32;
    annotate_into_18(&v18, v9, v11, v13, v15, v0, &v16, v19);
    v18
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i64) -> Rc<RefCell<Vec<(Rc<str>, i32, Rc<str>, bool, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i64) -> Rc<RefCell<Vec<(Rc<str>, i32, Rc<str>, bool, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i64| -> Rc<RefCell<Vec<(Rc<str>, i32, Rc<str>, bool, Rc<str>)>>> {
        annotations_of_0(v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_task_annotations(v0: &str, v1: &str, v2: &str, v3: i64) -> Rc<RefCell<Vec<(Rc<str>, i32, Rc<str>, bool, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3)
}
pub type SpiralShape_652accfc6a8d = Mut0;
