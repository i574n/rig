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
enum UH1 {
    UH1_NoTasks,
    UH1_TaskThen(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US4 {
    US4_ScanPlain,
    US4_ScanQuoted,
    US4_ScanEscaped,
}
#[derive(Clone)]
enum US5 {
    US5_MainAgent,
    US5_Subagent,
}
#[derive(Clone)]
enum UH2 {
    UH2_NoRanked,
    UH2_RankedThen(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<UH2>),
}
fn concat5_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn loop_2(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn method3(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_4(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn loop_6(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method9(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method9(v7, &v9) {
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
        method8(v16, v17)
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method10(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let v15: US2 = if v6 {
                let v7: US3 = US3::US3_HeadType;
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
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let mut v18: US3 = US3::US3_HeadInl;
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
fn method11(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn field_end_15(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US4) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US4::US4_ScanPlain => {
                    let v6: bool = v5 == 44i32;
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
                let v25: US4 = match v3 {
                    US4::US4_ScanEscaped => {
                        US4::US4_ScanQuoted
                    }
                    US4::US4_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US4::US4_ScanQuoted
                        } else {
                            US4::US4_ScanPlain
                        }
                    }
                    US4::US4_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US4::US4_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US4::US4_ScanPlain
                            } else {
                                US4::US4_ScanQuoted
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
fn skip_spaces_16(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_14(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US4 = US4::US4_ScanPlain;
            let v6: i32 = field_end_15(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_16(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn backlog_task_at_13(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>) {
    let mut v3: i32 = 0i32;
    let v4: i32 = field_start_14(v0.clone(), v1, v2, v3);
    let v5: US4 = US4::US4_ScanPlain;
    let v6: i32 = field_end_15(v0.clone(), v4, v2, v5);
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let v8: i32 = (v7.clone().len() as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let v11: i32 = 1i32;
    let v12: i32 = field_start_14(v0.clone(), v1, v2, v11);
    let v13: US4 = US4::US4_ScanPlain;
    let mut v14: i32 = field_end_15(v0.clone(), v12, v2, v13);
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: i32 = (v15.clone().len() as i32);
    let mut v17: i32 = v16.wrapping_sub(1i32);
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v15.get(1i32 as usize..v17 as usize).unwrap_or(""));
    let v19: i32 = 2i32;
    let v20: i32 = field_start_14(v0.clone(), v1, v2, v19);
    let v21: US4 = US4::US4_ScanPlain;
    let v22: i32 = field_end_15(v0.clone(), v20, v2, v21);
    let v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let v24: i32 = 3i32;
    let v25: i32 = field_start_14(v0.clone(), v1, v2, v24);
    let v26: US4 = US4::US4_ScanPlain;
    let v27: i32 = field_end_15(v0.clone(), v25, v2, v26);
    let v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let v29: i32 = (v28).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
    let v30: i32 = 4i32;
    let v31: i32 = field_start_14(v0.clone(), v1, v2, v30);
    let v32: US4 = US4::US4_ScanPlain;
    let v33: i32 = field_end_15(v0.clone(), v31, v2, v32);
    let v34: Rc<str> = std::rc::Rc::<str>::from(v0.get(v31 as usize..v33 as usize).unwrap_or(""));
    let v35: i32 = (v34).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
    let v36: i32 = 6i32;
    let v37: i32 = field_start_14(v0.clone(), v1, v2, v36);
    let v38: US4 = US4::US4_ScanPlain;
    let v39: i32 = field_end_15(v0.clone(), v37, v2, v38);
    let v40: Rc<str> = std::rc::Rc::<str>::from(v0.get(v37 as usize..v39 as usize).unwrap_or(""));
    let v41: i32 = 7i32;
    let v42: i32 = field_start_14(v0.clone(), v1, v2, v41);
    let v43: US4 = US4::US4_ScanPlain;
    let v44: i32 = field_end_15(v0.clone(), v42, v2, v43);
    let v45: Rc<str> = std::rc::Rc::<str>::from(v0.get(v42 as usize..v44 as usize).unwrap_or(""));
    let v46: i32 = (v45.clone().len() as i32);
    let v47: i32 = v46.wrapping_sub(1i32);
    let v48: Rc<str> = std::rc::Rc::<str>::from(v45.get(1i32 as usize..v47 as usize).unwrap_or(""));
    let v49: i32 = 8i32;
    let v50: i32 = field_start_14(v0.clone(), v1, v2, v49);
    let v51: US4 = US4::US4_ScanPlain;
    let v52: i32 = field_end_15(v0.clone(), v50, v2, v51);
    let v53: Rc<str> = std::rc::Rc::<str>::from(v0.get(v50 as usize..v52 as usize).unwrap_or(""));
    let v54: i32 = (v53.clone().len() as i32);
    let v55: i32 = v54.wrapping_sub(1i32);
    let v56: Rc<str> = std::rc::Rc::<str>::from(v53.get(1i32 as usize..v55 as usize).unwrap_or(""));
    let v57: i32 = agile_planning_fields::rig_agile_planning_minutes_near(&v0, &v10, v1);
    let v58: i32 = 5i32;
    let v59: i32 = field_start_14(v0.clone(), v1, v2, v58);
    let v60: US4 = US4::US4_ScanPlain;
    let v61: i32 = field_end_15(v0.clone(), v59, v2, v60);
    let v62: Rc<str> = std::rc::Rc::<str>::from(v0.get(v59 as usize..v61 as usize).unwrap_or(""));
    let v63: i32 = (v62).trim().trim_end_matches("u32").parse::<i32>().unwrap_or(0);
    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v10, v18, v23, v29, v35, v40, v48, v56, v57, v63, 0i32, false, v64)
}
fn tasks_from_12(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v4)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v6: bool = v5 < 0i32;
        if v6 {
            return v3;
        } else {
            let v7: i32 = v5.wrapping_add(6i32);
            let v8: i32 = v0.get(..v5 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v5 as usize).unwrap_or(""));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v11: bool = v9.starts_with(&*v10);
            if v11 {
                let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24): (Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>) = backlog_task_at_13(v0.clone(), v7, v2);
                let v25: Rc<UH1> = Rc::new(UH1::UH1_TaskThen(v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v3));
                (v1, v3) = (v7, v25);
                continue;
            } else {
                v1 = v7;
                continue;
            }
        }
    }
}
fn agents_take_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32, mut v10: i32, mut v11: i32, mut v12: bool, mut v13: Rc<str>) -> bool {
    let mut v14: bool = &*v3 != "Epic";
    if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user"); } LIT.with(|lit| lit.clone()) };
        let mut v17: bool = agile_planning_filters::rig_agile_planning_selected(&v0, &v1, &v15, &v16);
        let mut v18: bool = v17 == false;
        v18
    } else {
        false
    }
}
fn closure2(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32, mut v10: i32, mut v11: i32, mut v12: bool, mut v13: Rc<str>| -> bool {
        agents_take_18(v0.clone(), v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13)
    })
}
fn task_open_20(mut v0: &Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: &Rc<str>, mut v7: &Rc<str>, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: &Rc<str>) -> bool {
    let v13: bool = &*v5 == "Planned";
    if v13 {
        true
    } else {
        let mut v14: bool = &*v5 == "Active";
        if v14 {
            true
        } else {
            let v15: bool = &*v5 == "Blocked";
            if v15 {
                false
            } else {
                let v16: bool = &*v5 == "Paused";
                if v16 {
                    false
                } else {
                    false
                }
            }
        }
    }
}
fn open_child_of_21(mut v0: Rc<UH1>, mut v1: Rc<str>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_NoTasks => {
                return false;
            }
            UH1::UH1_TaskThen(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: i32 = v5;
                let v6: i32 = v6;
                let v7: Rc<str> = v7;
                let v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: i32 = v10;
                let v11: i32 = v11;
                let v12: i32 = v12;
                let v13: bool = v13;
                let mut v14: Rc<str> = v14;
                let v15: Rc<UH1> = v15;
                let v16: bool = &*v14 == &*v1;
                let mut v18: bool = if v16 {
                    let mut v17: bool = &*v7 != "Done";
                    v17
                } else {
                    false
                };
                if v18 {
                    return true;
                } else {
                    v0 = v15;
                    continue;
                }
            }
        }
    }
}
fn status_of_23(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_NoTasks => {
                let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v2;
            }
            UH1::UH1_TaskThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: i32 = v6;
                let v7: i32 = v7;
                let v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: i32 = v11;
                let v12: i32 = v12;
                let v13: i32 = v13;
                let mut v14: bool = v14;
                let v15: Rc<str> = v15;
                let v16: Rc<UH1> = v16;
                let mut v17: bool = &*v3 == &*v1;
                if v17 {
                    return v8;
                } else {
                    v0 = v16;
                    continue;
                }
            }
        }
    }
}
fn deps_settled_22(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: i32 = (v1.clone().len() as i32);
        let v4: bool = v3 <= v2;
        if v4 {
            return true;
        } else {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v6: i32 = v1.get(v2 as usize..).and_then(|rest| rest.find(&*v5)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let v7: bool = v6 < 0i32;
            let v8: i32 = if v7 {
                v3
            } else {
                v6
            };
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v1.get(v2 as usize..v8 as usize).unwrap_or(""));
            let v10: Rc<str> = std::rc::Rc::<str>::from(v9.trim());
            let v11: Rc<str> = status_of_23(v0.clone(), v10);
            let v12: bool = &*v11 == "";
            let mut v14: bool = if v12 {
                true
            } else {
                let v13: bool = &*v11 == "Done";
                v13
            };
            if v14 {
                let v15: i32 = v8.wrapping_add(1i32);
                v2 = v15;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn open_deps_26(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = (v1.clone().len() as i32);
        let v5: bool = v4 <= v2;
        if v5 {
            return v3;
        } else {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v7: i32 = v1.get(v2 as usize..).and_then(|rest| rest.find(&*v6)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let v8: bool = v7 < 0i32;
            let mut v9: i32 = if v8 {
                v4
            } else {
                v7
            };
            let v10: Rc<str> = std::rc::Rc::<str>::from(v1.get(v2 as usize..v9 as usize).unwrap_or(""));
            let v11: Rc<str> = std::rc::Rc::<str>::from(v10.trim());
            let v12: Rc<str> = status_of_23(v0.clone(), v11);
            let v13: i32 = v9.wrapping_add(1i32);
            let mut v14: bool = &*v12 == "";
            let v16: bool = if v14 {
                true
            } else {
                let v15: bool = &*v12 == "Done";
                v15
            };
            let mut v18: i32 = if v16 {
                v3
            } else {
                let mut v17: i32 = v3.wrapping_add(1i32);
                v17
            };
            (v2, v3) = (v13, v18);
            continue;
        }
    }
}
fn unblocked_weight_25(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_NoTasks => {
                return v2;
            }
            UH1::UH1_TaskThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: Rc<str> = v5.clone();
                let v6: i32 = *v6;
                let v7: i32 = *v7;
                let v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let v10: Rc<str> = v10.clone();
                let v11: i32 = *v11;
                let v12: i32 = *v12;
                let v13: i32 = *v13;
                let mut v14: bool = *v14;
                let v15: Rc<str> = v15.clone();
                let v16: Rc<UH1> = v16.clone();
                let mut v17: bool = task_open_20(&v3, &v4, &v5, v6, v7, v8, &v9, &v10, v11, v12, v13, v14, &v15);
                let v24: bool = if v17 {
                    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                    let v20: Rc<str> = Rc::<str>::from(v9.replace(&*v18, &*v19));
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v19));
                    let v23: bool = v22.split("|").any(|item| item == &*v1);
                    v23
                } else {
                    false
                };
                let v32: i32 = if v24 {
                    let v25: i32 = 0i32;
                    let v26: i32 = 0i32;
                    let v27: i32 = open_deps_26(v0, v9, v25, v26);
                    let v28: i32 = 1i32 << (2 * (5 - (v6).clamp(0, 5)));
                    let v29: bool = v27 < 1i32;
                    let v30: i32 = if v29 {
                        1i32
                    } else {
                        v27
                    };
                    let v31: i32 = v28.wrapping_div(v30);
                    v31
                } else {
                    0i32
                };
                let v33: i32 = v2.wrapping_add(v32);
                (v0, v2) = (v16, v33);
                continue;
            }
        }
    }
}
fn ranked_of_24(mut v0: US5, mut v1: Rc<UH1>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: &Rc<str>, mut v5: i32, mut v6: i32, mut v7: &Rc<str>, mut v8: &Rc<str>, mut v9: &Rc<str>, mut v10: i32, mut v11: i32, mut v12: i32, mut v13: bool, mut v14: Rc<str>) -> (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>) {
    let v15: bool = 0i32 < v10;
    let v33: i32 = if v15 {
        v10
    } else {
        let v16: bool = 0i32 == v6;
        let v31: i32 = if v16 {
            15i32
        } else {
            let mut v17: bool = 1i32 == v6;
            if v17 {
                30i32
            } else {
                let mut v18: bool = 2i32 == v6;
                if v18 {
                    60i32
                } else {
                    let v19: bool = 3i32 == v6;
                    if v19 {
                        180i32
                    } else {
                        let v20: bool = 4i32 == v6;
                        if v20 {
                            300i32
                        } else {
                            let v21: bool = 5i32 == v6;
                            if v21 {
                                600i32
                            } else {
                                let v22: bool = 6i32 == v6;
                                if v22 {
                                    1200i32
                                } else {
                                    let v23: bool = 7i32 == v6;
                                    if v23 {
                                        2400i32
                                    } else {
                                        3600i32
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        let v32: i32 = ((v31 as i64) * (100 - (v11).clamp(0, 100)) as i64 / 100).max(15) as i32;
        v32
    };
    let v34: i32 = 0i32;
    let v35: i32 = unblocked_weight_25(v1, v2.clone(), v34);
    let v36: i32 = 1i32 << (2 * (5 - (v5).clamp(0, 5)));
    let v37: i32 = v36.wrapping_add(v35);
    let v43: i32 = match &v0 {
        US5::US5_MainAgent => {
            let v38: i32 = ((v37 as i64) * 6000 / (v33).max(1) as i64).min(i32::MAX as i64) as i32;
            v38
        }
        US5::US5_Subagent => {
            let v39: i32 = v33.wrapping_mul(8i32);
            let v40: i32 = v39.wrapping_add(5i32);
            let v41: i32 = v40.wrapping_sub(v5);
            v41
        }
    };
    let v55: i32 = match v0 {
        US5::US5_MainAgent => {
            let v44: bool = v12 < 14i32;
            let v45: i32 = if v44 {
                v12
            } else {
                14i32
            };
            let v46: i32 = 7i32.wrapping_add(v45);
            let v47: i32 = if v13 {
                2i32
            } else {
                1i32
            };
            let v48: i32 = v46.wrapping_mul(v47);
            let v49: i32 = 2147483647i32.wrapping_div(v48);
            let v50: bool = v43 > v49;
            if v50 {
                2147483647i32
            } else {
                let v51: i32 = v43.wrapping_mul(v48);
                let v52: i32 = v51.wrapping_div(7i32);
                v52
            }
        }
        US5::US5_Subagent => {
            v43
        }
    };
    (v2, v3, v5, v6, v33, v15, v35, v55, v14)
}
fn inserted_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: bool, mut v6: i32, mut v7: i32, mut v8: Rc<str>, mut v9: Rc<UH2>) -> Rc<UH2> {
    match &*v9 {
        UH2::UH2_NoRanked => {
            let v10: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_NoRanked); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_RankedThen(v0, v1, v2, v3, v4, v5, v6, v7, v8, v10))
        }
        UH2::UH2_RankedThen(v12, v13, v14, v15, v16, v17, v18, v19, v20, v21) => {
            let v12: Rc<str> = v12.clone();
            let v13: Rc<str> = v13.clone();
            let mut v14: i32 = *v14;
            let v15: i32 = *v15;
            let v16: i32 = *v16;
            let mut v17: bool = *v17;
            let mut v18: i32 = *v18;
            let v19: i32 = *v19;
            let v20: Rc<str> = v20.clone();
            let v21: Rc<UH2> = v21.clone();
            let v22: bool = v19 < v7;
            if v22 {
                Rc::new(UH2::UH2_RankedThen(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9))
            } else {
                let v24: Rc<UH2> = inserted_27(v0, v1, v2, v3, v4, v5, v6, v7, v8, v21);
                Rc::new(UH2::UH2_RankedThen(v12, v13, v14, v15, v16, v17, v18, v19, v20, v24))
            }
        }
    }
}
fn ranked_from_19(mut v0: US5, mut v1: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>) -> bool>, mut v2: Rc<UH1>, mut v3: Rc<UH1>, mut v4: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH1::UH1_NoTasks => {
                return v4;
            }
            UH1::UH1_TaskThen(v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18) => {
                let v5: Rc<str> = v5;
                let v6: Rc<str> = v6;
                let v7: Rc<str> = v7;
                let v8: i32 = v8;
                let mut v9: i32 = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<str> = v11;
                let v12: Rc<str> = v12;
                let v13: i32 = v13;
                let mut v14: i32 = v14;
                let v15: i32 = v15;
                let v16: bool = v16;
                let mut v17: Rc<str> = v17;
                let mut v18: Rc<UH1> = v18;
                let v19: bool = task_open_20(&v5, &v6, &v7, v8, v9, v10.clone(), &v11, &v12, v13, v14, v15, v16, &v17);
                let v23: bool = if v19 {
                    match &v0 {
                        US5::US5_MainAgent => {
                            true
                        }
                        US5::US5_Subagent => {
                            let v20: bool = &*v7 != "Epic";
                            v20
                        }
                    }
                } else {
                    false
                };
                let v28: bool = if v23 {
                    let v24: bool = &*v7 == "Epic";
                    let v26: bool = if v24 {
                        open_child_of_21(v2.clone(), v5.clone())
                    } else {
                        false
                    };
                    let v27: bool = v26 == false;
                    v27
                } else {
                    false
                };
                let v37: bool = if v28 {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user-batch"); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                    let v32: Rc<str> = Rc::<str>::from(v12.replace(&*v30, &*v31));
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v34: Rc<str> = Rc::<str>::from(v32.replace(&*v33, &*v31));
                    let v35: bool = v34.split("|").any(|item| item == &*v29);
                    let v36: bool = v35 == false;
                    v36
                } else {
                    false
                };
                let v40: bool = if v37 {
                    let v38: i32 = 0i32;
                    deps_settled_22(v2.clone(), v11.clone(), v38)
                } else {
                    false
                };
                let v42: bool = if v40 {
                    v1(v5.clone(), v6.clone(), v7.clone(), v8, v9, v10.clone(), v11.clone(), v12.clone(), v13, v14, v15, v16, v17.clone())
                } else {
                    false
                };
                if v42 {
                    let (mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>) = ranked_of_24(v0.clone(), v2.clone(), v5, v6, &v7, v8, v9, &v10, &v11, &v12, v13, v14, v15, v16, v17);
                    let v52: Rc<UH2> = inserted_27(v43, v44, v45, v46, v47, v48, v49, v50, v51, v4);
                    (v3, v4) = (v18, v52);
                    continue;
                } else {
                    v3 = v18;
                    continue;
                }
            }
        }
    }
}
fn ranked_words_28(mut v0: Rc<UH2>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_NoRanked => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        UH2::UH2_RankedThen(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let v4: i32 = v4;
            let v5: i32 = v5;
            let v6: i32 = v6;
            let v7: bool = v7;
            let v8: i32 = v8;
            let mut v9: i32 = v9;
            let v10: Rc<str> = v10;
            let v11: Rc<UH2> = v11;
            let v12: Rc<UH0> = ranked_words_28(v11);
            Rc::new(UH0::UH0_Cons(v2, v12))
        }
    }
}
fn method29(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method29(v0.clone(), v4, v2);
            let v6: Rc<str> = status_of_23(v0, v3.clone());
            let v7: bool = &*v6 == "Active";
            if v7 {
                Rc::new(UH0::UH0_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn open_words_30(mut v0: Rc<UH1>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_NoTasks => {
                return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            }
            UH1::UH1_TaskThen(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: i32 = v5;
                let v6: i32 = v6;
                let v7: Rc<str> = v7;
                let v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: i32 = v10;
                let v11: i32 = v11;
                let v12: i32 = v12;
                let v13: bool = v13;
                let mut v14: Rc<str> = v14;
                let v15: Rc<UH1> = v15;
                let v16: bool = &*v7 == "Done";
                if v16 {
                    v0 = v15;
                    continue;
                } else {
                    let mut v18: Rc<UH0> = open_words_30(v15);
                    return Rc::new(UH0::UH0_Cons(v2, v18));
                }
            }
        }
    }
}
fn ranking_of_17(mut v0: Rc<str>, mut v1: Rc<UH1>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: US5 = US5::US5_MainAgent;
    let mut v3: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>) -> bool> = closure2(v0);
    let v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_NoRanked); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH2> = ranked_from_19(v2, v3, v1.clone(), v1.clone(), v4);
    let v6: Rc<UH0> = ranked_words_28(v5);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = method29(v1.clone(), v6.clone(), v7);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v10: i32 = 0i32;
    let v11: i32 = method11(v9.clone(), v6, v10);
    let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
    let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v14));
    let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v17: i32 = 0i32;
    let mut v18: i32 = method11(v16.clone(), v8, v17);
    let v19: Rc<Vec<Rc<str>>> = Rc::new(v16.borrow().clone());
    let v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v19).as_ref().clone()));
    let v21: Rc<str> = Rc::<str>::from((*v20.borrow()).join(&*v14));
    let v22: Rc<UH0> = open_words_30(v1);
    let v23: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v24: i32 = 0i32;
    let v25: i32 = method11(v23.clone(), v22, v24);
    let v26: Rc<Vec<Rc<str>>> = Rc::new(v23.borrow().clone());
    let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v26).as_ref().clone()));
    let v28: Rc<str> = Rc::<str>::from((*v27.borrow()).join(&*v14));
    (v15, v21, v28)
}
fn claim_ranking_0(mut v0: Rc<str>) -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = concat5_1(v0, v1, v2.clone(), v2.clone(), v2.clone());
    let v4: bool = std::path::Path::new(&*v3).is_file();
    let v7: US0 = if v4 {
        US0::US0_FileReadable(v3)
    } else {
        US0::US0_FileMissing(v3)
    };
    match v7 {
        US0::US0_FileMissing(v8) => {
            let v8: Rc<str> = v8;
            (false, v2.clone(), v2.clone(), v2.clone(), v2.clone())
        }
        US0::US0_FileReadable(v9) => {
            let mut v9: Rc<str> = v9;
            let v10: Rc<str> = std::fs::read_to_string(&*v9).unwrap_or_default().into();
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = Rc::<str>::from(v9.replace(&*v11, &*v12));
            let mut v14: Rc<str> = loop_2(v13);
            let v15: Option<usize> = v14.rfind(&*v12);
            let v16: Option<usize> = method3(v15);
            let mut v17: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v18: Option<i32> = v16.map(|x| v17(x));
            let v20: i32 = v18.unwrap_or(-1i32);
            let v21: bool = v20 < 0i32;
            let v25: bool = if v21 {
                true
            } else {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v23: bool = v14.ends_with(&*v22);
                let v24: bool = v23 == false;
                v24
            };
            let v46: US1 = if v25 {
                US1::US1_OtherFile
            } else {
                let v27: usize = ((0i32) as usize);
                let v28: usize = ((v20) as usize);
                let v29: Rc<str> = Rc::<str>::from(&(v14)[v27..v28]);
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v31: bool = v29.ends_with(&*v30);
                if v31 {
                    let v32: Rc<str> = loop_4(v29);
                    US1::US1_StateFile(v32)
                } else {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v35: bool = v29.ends_with(&*v34);
                    if v35 {
                        let v36: Rc<str> = loop_5(v29);
                        US1::US1_HistoryFile(v36)
                    } else {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v39: bool = v29.ends_with(&*v38);
                        if v39 {
                            let v40: Rc<str> = loop_6(v29);
                            US1::US1_ArchiveFile(v40)
                        } else {
                            US1::US1_OtherFile
                        }
                    }
                }
            };
            let v57: Rc<str> = match v46 {
                US1::US1_OtherFile => {
                    v10
                }
                _ => {
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<UH0> = split_list_7(v47.clone(), v10);
                    let v49: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v50: Rc<UH0> = method10(v48, v49);
                    let v51: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v52: i32 = 0i32;
                    let v53: i32 = method11(v51.clone(), v50, v52);
                    let v54: Rc<Vec<Rc<str>>> = Rc::new(v51.borrow().clone());
                    let v55: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v54).as_ref().clone()));
                    let v56: Rc<str> = Rc::<str>::from((*v55.borrow()).join(&*v47));
                    v56
                }
            };
            let v58: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v57);
            let v59: i32 = 0i32;
            let v60: i32 = (v58.clone().len() as i32);
            let v61: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoTasks); } CASE.with(|case| case.clone()) };
            let v62: Rc<UH1> = tasks_from_12(v58.clone(), v59, v60, v61);
            let (mut v63, mut v64, mut v65): (Rc<str>, Rc<str>, Rc<str>) = ranking_of_17(v58, v62);
            let v66: Rc<str> = rig_job_names::rig_job_live_items(&v63);
            (true, v63, v64, v65, v66)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
        claim_ranking_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_claim_ranking(v0: &str) -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    closure0()(Rc::<str>::from(v0))
}
pub type SpiralShape_652accfc6a8d = Mut0;
