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
    US0_MainAgent,
    US0_Subagent,
}
#[derive(Clone)]
enum US1 {
    US1_FileReadable(Rc<str>),
    US1_FileMissing(Rc<str>),
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
    US5_ScanPlain,
    US5_ScanQuoted,
    US5_ScanEscaped,
}
#[derive(Clone)]
enum UH1 {
    UH1_NoTasks,
    UH1_TaskThen(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_NoRanked,
    UH2_RankedThen(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<str>, Rc<UH2>),
}
pub struct Mut1 { pub l0: i32 }
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
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
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
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
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
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
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
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
            let v6: bool = v2.starts_with(&*v5);
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
            let mut v27: US3 = match &v15 {
                US3::US3_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US4 = US4::US4_HeadInl;
                        US3::US3_KeywordHead(v18)
                    } else {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
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
fn field_end_15(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US5) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US5::US5_ScanPlain => {
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
                let v25: US5 = match v3 {
                    US5::US5_ScanEscaped => {
                        US5::US5_ScanQuoted
                    }
                    US5::US5_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US5::US5_ScanQuoted
                        } else {
                            US5::US5_ScanPlain
                        }
                    }
                    US5::US5_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US5::US5_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US5::US5_ScanPlain
                            } else {
                                US5::US5_ScanQuoted
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
fn top_level_field_count_14(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: US5 = US5::US5_ScanPlain;
        let v5: i32 = field_end_15(v0.clone(), v1, v2, v4);
        let v6: bool = v2 <= v5;
        if v6 {
            return -1i32;
        } else {
            let v7: i32 = v0.as_bytes().get(v5 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v8: bool = v7 == 41i32;
            if v8 {
                let mut v9: i32 = v3.wrapping_add(1i32);
                return v9;
            } else {
                let v10: i32 = v5.wrapping_add(1i32);
                let v11: i32 = skip_spaces_16(v0.clone(), v10);
                let v12: i32 = v3.wrapping_add(1i32);
                (v1, v3) = (v11, v12);
                continue;
            }
        }
    }
}
fn first_malformed_task_13(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return -1i32;
        } else {
            let v6: i32 = v4.wrapping_add(6i32);
            let v7: i32 = v0.get(..v4 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v7 as usize..v4 as usize).unwrap_or(""));
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v10: bool = v8.starts_with(&*v9);
            if v10 {
                let v11: i32 = 0i32;
                let v12: i32 = top_level_field_count_14(v0.clone(), v6, v2, v11);
                let v13: bool = v12 == 11i32;
                if v13 {
                    v1 = v6;
                    continue;
                } else {
                    return v4;
                }
            } else {
                v1 = v6;
                continue;
            }
        }
    }
}
fn field_start_19(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US5 = US5::US5_ScanPlain;
            let v6: i32 = field_end_15(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_16(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn backlog_task_at_18(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>, Rc<str>) {
    let mut v3: i32 = 0i32;
    let v4: i32 = field_start_19(v0.clone(), v1, v2, v3);
    let v5: US5 = US5::US5_ScanPlain;
    let v6: i32 = field_end_15(v0.clone(), v4, v2, v5);
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let v8: i32 = (v7.clone().len() as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let v11: i32 = 1i32;
    let v12: i32 = field_start_19(v0.clone(), v1, v2, v11);
    let v13: US5 = US5::US5_ScanPlain;
    let v14: i32 = field_end_15(v0.clone(), v12, v2, v13);
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: i32 = (v15.clone().len() as i32);
    let v17: i32 = v16.wrapping_sub(1i32);
    let v18: Rc<str> = std::rc::Rc::<str>::from(v15.get(1i32 as usize..v17 as usize).unwrap_or(""));
    let mut v19: i32 = 2i32;
    let mut v20: i32 = field_start_19(v0.clone(), v1, v2, v19);
    let v21: US5 = US5::US5_ScanPlain;
    let v22: i32 = field_end_15(v0.clone(), v20, v2, v21);
    let v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let mut v24: i32 = 3i32;
    let v25: i32 = field_start_19(v0.clone(), v1, v2, v24);
    let v26: US5 = US5::US5_ScanPlain;
    let mut v27: i32 = field_end_15(v0.clone(), v25, v2, v26);
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let v29: i32 = (v28).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
    let v30: i32 = 4i32;
    let v31: i32 = field_start_19(v0.clone(), v1, v2, v30);
    let v32: US5 = US5::US5_ScanPlain;
    let v33: i32 = field_end_15(v0.clone(), v31, v2, v32);
    let v34: Rc<str> = std::rc::Rc::<str>::from(v0.get(v31 as usize..v33 as usize).unwrap_or(""));
    let v35: i32 = (v34).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
    let v36: i32 = 6i32;
    let v37: i32 = field_start_19(v0.clone(), v1, v2, v36);
    let v38: US5 = US5::US5_ScanPlain;
    let v39: i32 = field_end_15(v0.clone(), v37, v2, v38);
    let v40: Rc<str> = std::rc::Rc::<str>::from(v0.get(v37 as usize..v39 as usize).unwrap_or(""));
    let v41: i32 = 7i32;
    let v42: i32 = field_start_19(v0.clone(), v1, v2, v41);
    let v43: US5 = US5::US5_ScanPlain;
    let v44: i32 = field_end_15(v0.clone(), v42, v2, v43);
    let v45: Rc<str> = std::rc::Rc::<str>::from(v0.get(v42 as usize..v44 as usize).unwrap_or(""));
    let v46: i32 = (v45.clone().len() as i32);
    let v47: i32 = v46.wrapping_sub(1i32);
    let v48: Rc<str> = std::rc::Rc::<str>::from(v45.get(1i32 as usize..v47 as usize).unwrap_or(""));
    let v49: i32 = 8i32;
    let v50: i32 = field_start_19(v0.clone(), v1, v2, v49);
    let v51: US5 = US5::US5_ScanPlain;
    let v52: i32 = field_end_15(v0.clone(), v50, v2, v51);
    let v53: Rc<str> = std::rc::Rc::<str>::from(v0.get(v50 as usize..v52 as usize).unwrap_or(""));
    let v54: i32 = (v53.clone().len() as i32);
    let v55: i32 = v54.wrapping_sub(1i32);
    let v56: Rc<str> = std::rc::Rc::<str>::from(v53.get(1i32 as usize..v55 as usize).unwrap_or(""));
    let v57: i32 = agile_planning_fields::rig_agile_planning_minutes_near(&v0, &v10, v1);
    let v58: i32 = 5i32;
    let v59: i32 = field_start_19(v0.clone(), v1, v2, v58);
    let v60: US5 = US5::US5_ScanPlain;
    let v61: i32 = field_end_15(v0.clone(), v59, v2, v60);
    let v62: Rc<str> = std::rc::Rc::<str>::from(v0.get(v59 as usize..v61 as usize).unwrap_or(""));
    let v63: i32 = (v62).trim().trim_end_matches("u32").parse::<i32>().unwrap_or(0);
    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v10, v18, v23, v29, v35, v40, v48, v56, v57, v63, 0i32, false, v64.clone(), v64.clone())
}
fn tasks_from_17(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH1>) -> Rc<UH1> {
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
                let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>, Rc<str>) = backlog_task_at_18(v0.clone(), v7, v2);
                let v26: Rc<UH1> = Rc::new(UH1::UH1_TaskThen(v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v3));
                (v1, v3) = (v7, v26);
                continue;
            } else {
                v1 = v7;
                continue;
            }
        }
    }
}
fn annotated_from_21(mut v0: Rc<RefCell<Vec<(Rc<str>, i32, Rc<str>, bool, Rc<str>)>>>, mut v1: Rc<UH1>, mut v2: i32) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_NoTasks => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoTasks); } CASE.with(|case| case.clone()) }
        }
        UH1::UH1_TaskThen(v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18) => {
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: i32 = v7;
            let v8: i32 = v8;
            let mut v9: Rc<str> = v9;
            let v10: Rc<str> = v10;
            let v11: Rc<str> = v11;
            let v12: i32 = v12;
            let v13: i32 = v13;
            let v14: i32 = v14;
            let v15: bool = v15;
            let v16: Rc<str> = v16;
            let v17: Rc<str> = v17;
            let v18: Rc<UH1> = v18;
            let (mut v19, mut v20, mut v21, mut v22, mut v23): (Rc<str>, i32, Rc<str>, bool, Rc<str>) = v0.borrow()[v2 as usize].clone();
            let mut v24: i32 = v2.wrapping_add(1i32);
            let v25: Rc<UH1> = annotated_from_21(v0, v18, v24);
            Rc::new(UH1::UH1_TaskThen(v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v20, v22, v21, v23, v25))
        }
    }
}
fn ids_text_22(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_NoTasks => {
                return v1;
            }
            UH1::UH1_TaskThen(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) => {
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
                let v14: Rc<str> = v14;
                let v15: Rc<str> = v15;
                let v16: Rc<UH1> = v16;
                let v17: bool = &*v1 == "";
                let mut v20: Rc<str> = if v17 {
                    v2
                } else {
                    let mut v19: Rc<str> = string_concat(&[&*v1, "\n", &*v2]);
                    v19
                };
                (v0, v1) = (v16, v20);
                continue;
            }
        }
    }
}
fn annotated_tasks_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i64, mut v3: Rc<UH1>) -> Rc<UH1> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = ids_text_22(v3.clone(), v4);
    let v6: Rc<RefCell<Vec<(Rc<str>, i32, Rc<str>, bool, Rc<str>)>>> = agile_task_annotation::rig_agile_task_annotations(&v0, &v1, &v5, v2);
    let v7: i32 = 0i32;
    annotated_from_21(v6, v3, v7)
}
fn load_words_23(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    v2
}
fn loads_pass_24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<RefCell<Vec<Rc<str>>>>, mut v3: i32) -> bool {
    loop {
        let v4: i32 = (v2.clone().borrow().len() as i32);
        let v5: bool = v4 <= v3;
        if v5 {
            return false;
        } else {
            let v6: Rc<str> = v2.borrow()[v3 as usize].clone();
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            let v8: bool = agile_planning_filters::rig_agile_planning_selected(&v0, &v1, &v6, &v7);
            if v8 {
                return true;
            } else {
                let mut v9: i32 = v3.wrapping_add(1i32);
                v3 = v9;
                continue;
            }
        }
    }
}
fn listed_at_25(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v2 >= v3;
        if v4 {
            return false;
        } else {
            let v5: Rc<str> = v0.borrow()[v2 as usize].clone();
            let v6: bool = &*v5 == &*v1;
            if v6 {
                return true;
            } else {
                let v7: i32 = v2.wrapping_add(1i32);
                v2 = v7;
                continue;
            }
        }
    }
}
fn closure2(mut v0: US0, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: Rc<str>, mut v6: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>, Rc<str>) -> bool> {
    Rc::new(move |mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: i32, mut v11: i32, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: i32, mut v16: i32, mut v17: i32, mut v18: bool, mut v19: Rc<str>, mut v20: Rc<str>| -> bool {
        let v21: bool = &*v2 == "";
        let v23: bool = if v21 {
            true
        } else {
            let v22: bool = agile_task_query::rig_query_passes(&*v2, &*v7, &*v8, &*v14, &*v12, true);
            v22
        };
        let mut v27: bool = if v23 {
            let mut v24: Rc<RefCell<Vec<Rc<str>>>> = load_words_23(v3.clone());
            let v25: i32 = 0i32;
            loads_pass_24(v5.clone(), v7.clone(), v24, v25)
        } else {
            false
        };
        if v27 {
            let mut v28: i32 = 0i32;
            let v29: bool = listed_at_25(&v6, v7, v28);
            let v30: bool = v29 == false;
            v30
        } else {
            false
        }
    })
}
fn task_open_27(mut v0: &Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: &Rc<str>, mut v7: &Rc<str>, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: &Rc<str>, mut v13: &Rc<str>) -> bool {
    let v14: bool = &*v5 == "Planned";
    if v14 {
        true
    } else {
        let v15: bool = &*v5 == "Active";
        if v15 {
            true
        } else {
            let v16: bool = &*v5 == "Blocked";
            if v16 {
                false
            } else {
                let v17: bool = &*v5 == "Paused";
                if v17 {
                    false
                } else {
                    let v18: bool = &*v5 == "Done";
                    if v18 {
                        false
                    } else {
                        false
                    }
                }
            }
        }
    }
}
fn open_child_of_28(mut v0: Rc<UH1>, mut v1: Rc<str>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_NoTasks => {
                return false;
            }
            UH1::UH1_TaskThen(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) => {
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
                let v14: Rc<str> = v14;
                let v15: Rc<str> = v15;
                let v16: Rc<UH1> = v16;
                let v17: bool = &*v14 == &*v1;
                let v22: bool = if v17 {
                    let v18: bool = &*v7 == "Done";
                    let mut v20: bool = if v18 {
                        true
                    } else {
                        let mut v19: bool = &*v7 == "Dropped";
                        v19
                    };
                    let v21: bool = v20 == false;
                    v21
                } else {
                    false
                };
                if v22 {
                    return true;
                } else {
                    v0 = v16;
                    continue;
                }
            }
        }
    }
}
fn status_of_30(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_NoTasks => {
                let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v2;
            }
            UH1::UH1_TaskThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17) => {
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
                let v14: bool = v14;
                let v15: Rc<str> = v15;
                let v16: Rc<str> = v16;
                let v17: Rc<UH1> = v17;
                let v18: bool = &*v3 == &*v1;
                if v18 {
                    return v8;
                } else {
                    v0 = v17;
                    continue;
                }
            }
        }
    }
}
fn deps_settled_29(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
            let v11: Rc<str> = status_of_30(v0.clone(), v10);
            let v12: bool = &*v11 == "";
            let v16: bool = if v12 {
                true
            } else {
                let v13: bool = &*v11 == "Done";
                if v13 {
                    true
                } else {
                    let v14: bool = &*v11 == "Dropped";
                    v14
                }
            };
            if v16 {
                let v17: i32 = v8.wrapping_add(1i32);
                v2 = v17;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn open_deps_33(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
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
            let v12: Rc<str> = status_of_30(v0.clone(), v11);
            let v13: bool = &*v12 == "";
            let v17: bool = if v13 {
                true
            } else {
                let v14: bool = &*v12 == "Done";
                if v14 {
                    true
                } else {
                    let v15: bool = &*v12 == "Dropped";
                    v15
                }
            };
            let v18: i32 = v9.wrapping_add(1i32);
            let mut v20: i32 = if v17 {
                v3
            } else {
                let mut v19: i32 = v3.wrapping_add(1i32);
                v19
            };
            (v2, v3) = (v18, v20);
            continue;
        }
    }
}
fn unblocked_weight_32(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_NoTasks => {
                return v2;
            }
            UH1::UH1_TaskThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17) => {
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
                let v14: bool = *v14;
                let v15: Rc<str> = v15.clone();
                let v16: Rc<str> = v16.clone();
                let v17: Rc<UH1> = v17.clone();
                let v18: bool = task_open_27(&v3, &v4, &v5, v6, v7, v8, &v9, &v10, v11, v12, v13, v14, &v15, &v16);
                let v25: bool = if v18 {
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<str> = Rc::<str>::from(v9.replace(&*v19, &*v20));
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = Rc::<str>::from(v21.replace(&*v22, &*v20));
                    let mut v24: bool = v23.split("|").any(|item| item == &*v1);
                    v24
                } else {
                    false
                };
                let v33: i32 = if v25 {
                    let v26: i32 = 0i32;
                    let mut v27: i32 = 0i32;
                    let mut v28: i32 = open_deps_33(v0, v9, v26, v27);
                    let v29: i32 = 1i32 << (2 * (5 - (v6).clamp(0, 5)));
                    let v30: bool = v28 < 1i32;
                    let v31: i32 = if v30 {
                        1i32
                    } else {
                        v28
                    };
                    let v32: i32 = v29.wrapping_div(v31);
                    v32
                } else {
                    0i32
                };
                let v34: i32 = v2.wrapping_add(v33);
                (v0, v2) = (v17, v34);
                continue;
            }
        }
    }
}
fn after_waiting_34(mut v0: Rc<UH1>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = status_of_30(v0, v1.clone());
    let mut v3: bool = &*v1 != "";
    let v5: bool = if v3 {
        let v4: bool = &*v2 != "";
        v4
    } else {
        false
    };
    if v5 {
        let v6: bool = &*v2 == "Done";
        let v8: bool = if v6 {
            true
        } else {
            let v7: bool = &*v2 == "Dropped";
            v7
        };
        let mut v9: bool = v8 == false;
        v9
    } else {
        false
    }
}
fn after_shown_35(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = status_of_30(v0, v1.clone());
    let mut v3: bool = &*v1 == "";
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let v5: bool = &*v2 == "";
        if v5 {
            let v6: Rc<str> = string_concat(&[&*v1, ":gone"]);
            v6
        } else {
            let v7: bool = &*v2 == "Done";
            let mut v9: bool = if v7 {
                true
            } else {
                let v8: bool = &*v2 == "Dropped";
                v8
            };
            if v9 {
                let v11: Rc<str> = Rc::<str>::from(v2.to_lowercase());
                let v12: Rc<str> = string_concat(&[&*v1, ":", &*v11]);
                v12
            } else {
                v1
            }
        }
    }
}
fn ranked_of_31(mut v0: US0, mut v1: Rc<UH1>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: &Rc<str>, mut v5: i32, mut v6: i32, mut v7: &Rc<str>, mut v8: &Rc<str>, mut v9: &Rc<str>, mut v10: i32, mut v11: i32, mut v12: i32, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>) -> (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<str>) {
    let v16: bool = 0i32 < v10;
    let v34: i32 = if v16 {
        v10
    } else {
        let v17: bool = 0i32 == v6;
        let v32: i32 = if v17 {
            15i32
        } else {
            let v18: bool = 1i32 == v6;
            if v18 {
                30i32
            } else {
                let mut v19: bool = 2i32 == v6;
                if v19 {
                    60i32
                } else {
                    let mut v20: bool = 3i32 == v6;
                    if v20 {
                        180i32
                    } else {
                        let v21: bool = 4i32 == v6;
                        if v21 {
                            300i32
                        } else {
                            let v22: bool = 5i32 == v6;
                            if v22 {
                                600i32
                            } else {
                                let v23: bool = 6i32 == v6;
                                if v23 {
                                    1200i32
                                } else {
                                    let mut v24: bool = 7i32 == v6;
                                    if v24 {
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
        let v33: i32 = ((v32 as i64) * (100 - (v11).clamp(0, 100)) as i64 / 100).max(15) as i32;
        v33
    };
    let v35: i32 = 0i32;
    let v36: i32 = unblocked_weight_32(v1.clone(), v2.clone(), v35);
    let v37: i32 = 1i32 << (2 * (5 - (v5).clamp(0, 5)));
    let v38: i32 = v37.wrapping_add(v36);
    let v44: i32 = match &v0 {
        US0::US0_MainAgent => {
            let v39: i32 = ((v38 as i64) * 6000 / (v34).max(1) as i64).min(i32::MAX as i64) as i32;
            v39
        }
        US0::US0_Subagent => {
            let v40: i32 = v34.wrapping_mul(8i32);
            let v41: i32 = v40.wrapping_add(5i32);
            let v42: i32 = v41.wrapping_sub(v5);
            v42
        }
    };
    let v56: i32 = match v0 {
        US0::US0_MainAgent => {
            let v45: bool = v12 < 14i32;
            let v46: i32 = if v45 {
                v12
            } else {
                14i32
            };
            let v47: i32 = 7i32.wrapping_add(v46);
            let v48: i32 = if v13 {
                2i32
            } else {
                1i32
            };
            let v49: i32 = v47.wrapping_mul(v48);
            let v50: i32 = 2147483647i32.wrapping_div(v49);
            let v51: bool = v44 > v50;
            if v51 {
                2147483647i32
            } else {
                let v52: i32 = v44.wrapping_mul(v49);
                let v53: i32 = v52.wrapping_div(7i32);
                v53
            }
        }
        US0::US0_Subagent => {
            v44
        }
    };
    let v57: bool = after_waiting_34(v1.clone(), v15.clone());
    let v59: i32 = if v57 {
        let v58: i32 = v56.wrapping_div(1000i32);
        v58
    } else {
        v56
    };
    let v60: Rc<str> = after_shown_35(v1, v15);
    (v2, v3, v5, v6, v34, v16, v36, v59, v14, v60)
}
fn inserted_36(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: bool, mut v6: i32, mut v7: i32, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<UH2>) -> Rc<UH2> {
    match &*v10 {
        UH2::UH2_NoRanked => {
            let v11: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_NoRanked); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_RankedThen(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v11))
        }
        UH2::UH2_RankedThen(v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23) => {
            let v13: Rc<str> = v13.clone();
            let v14: Rc<str> = v14.clone();
            let v15: i32 = *v15;
            let v16: i32 = *v16;
            let v17: i32 = *v17;
            let v18: bool = *v18;
            let mut v19: i32 = *v19;
            let mut v20: i32 = *v20;
            let v21: Rc<str> = v21.clone();
            let v22: Rc<str> = v22.clone();
            let v23: Rc<UH2> = v23.clone();
            let mut v24: bool = v20 < v7;
            if v24 {
                Rc::new(UH2::UH2_RankedThen(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10))
            } else {
                let v26: Rc<UH2> = inserted_36(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v23);
                Rc::new(UH2::UH2_RankedThen(v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v26))
            }
        }
    }
}
fn ranked_from_26(mut v0: US0, mut v1: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>, Rc<str>) -> bool>, mut v2: Rc<UH1>, mut v3: Rc<UH1>, mut v4: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH1::UH1_NoTasks => {
                return v4;
            }
            UH1::UH1_TaskThen(v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19) => {
                let v5: Rc<str> = v5;
                let v6: Rc<str> = v6;
                let v7: Rc<str> = v7;
                let v8: i32 = v8;
                let mut v9: i32 = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<str> = v11;
                let v12: Rc<str> = v12;
                let v13: i32 = v13;
                let v14: i32 = v14;
                let v15: i32 = v15;
                let v16: bool = v16;
                let v17: Rc<str> = v17;
                let v18: Rc<str> = v18;
                let mut v19: Rc<UH1> = v19;
                let mut v20: bool = task_open_27(&v5, &v6, &v7, v8, v9, v10.clone(), &v11, &v12, v13, v14, v15, v16, &v17, &v18);
                let mut v24: bool = if v20 {
                    match &v0 {
                        US0::US0_MainAgent => {
                            true
                        }
                        US0::US0_Subagent => {
                            let v21: bool = &*v7 != "Epic";
                            v21
                        }
                    }
                } else {
                    false
                };
                let v29: bool = if v24 {
                    let v25: bool = &*v7 == "Epic";
                    let mut v27: bool = if v25 {
                        open_child_of_28(v2.clone(), v5.clone())
                    } else {
                        false
                    };
                    let mut v28: bool = v27 == false;
                    v28
                } else {
                    false
                };
                let v38: bool = if v29 {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user-batch"); } LIT.with(|lit| lit.clone()) };
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                    let v33: Rc<str> = Rc::<str>::from(v12.replace(&*v31, &*v32));
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = Rc::<str>::from(v33.replace(&*v34, &*v32));
                    let v36: bool = v35.split("|").any(|item| item == &*v30);
                    let v37: bool = v36 == false;
                    v37
                } else {
                    false
                };
                let v41: bool = if v38 {
                    let v39: i32 = 0i32;
                    deps_settled_29(v2.clone(), v11.clone(), v39)
                } else {
                    false
                };
                let v43: bool = if v41 {
                    v1(v5.clone(), v6.clone(), v7.clone(), v8, v9, v10.clone(), v11.clone(), v12.clone(), v13, v14, v15, v16, v17.clone(), v18.clone())
                } else {
                    false
                };
                if v43 {
                    let (mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<str>) = ranked_of_31(v0.clone(), v2.clone(), v5, v6, &v7, v8, v9, &v10, &v11, &v12, v13, v14, v15, v16, v17, v18);
                    let v54: Rc<UH2> = inserted_36(v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, v4);
                    (v3, v4) = (v19, v54);
                    continue;
                } else {
                    v3 = v19;
                    continue;
                }
            }
        }
    }
}
fn method38(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn rows_of_37(mut v0: Rc<UH2>, mut v1: i32, mut v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<str>)>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_NoRanked => {
                return v2;
            }
            UH2::UH2_RankedThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: i32 = v5;
                let v6: i32 = v6;
                let v7: i32 = v7;
                let v8: bool = v8;
                let mut v9: i32 = v9;
                let v10: i32 = v10;
                let v11: Rc<str> = v11;
                let v12: Rc<str> = v12;
                let v13: Rc<UH2> = v13;
                let v14: i32 = (v2.clone().borrow().len() as i32);
                let v15: bool = v14 >= v1;
                if v15 {
                    return v2;
                } else {
                    let v16: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(vec![(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)]));
                    let v17: i32 = (v16.clone().borrow().len() as i32);
                    let v18: i32 = v14.wrapping_add(v17);
                    let mut v19: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<str>)>::default(); v18 as usize]));
                    let mut v20: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                    while method38(v18, &v20) {
                        let v22: i32 = v20.borrow().l0;
                        let v23: bool = v22 < v14;
                        let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<str>) = if v23 {
                            let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<str>) = v2.borrow()[v22 as usize].clone();
                            (v24, v25, v26, v27, v28, v29, v30, v31, v32, v33)
                        } else {
                            let v34: i32 = v22.wrapping_sub(v14);
                            let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<str>) = v16.borrow()[v34 as usize].clone();
                            (v35, v36, v37, v38, v39, v40, v41, v42, v43, v44)
                        };
                        v19.borrow_mut()[v22 as usize] = (v45, v46, v47, v48, v49, v50, v51, v52, v53, v54);
                        let v55: i32 = v22.wrapping_add(1i32);
                        v20.borrow_mut().l0 = v55;
                        ()
                    };
                    (v0, v2) = (v13, v19);
                    continue;
                }
            }
        }
    }
}
fn ranked_count_39(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_NoRanked => {
                return v1;
            }
            UH2::UH2_RankedThen(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: i32 = v4;
                let v5: i32 = v5;
                let v6: i32 = v6;
                let v7: bool = v7;
                let v8: i32 = v8;
                let mut v9: i32 = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<str> = v11;
                let v12: Rc<UH2> = v12;
                let v13: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v12, v13);
                continue;
            }
        }
    }
}
fn agile_next_1(mut v0: Rc<str>, mut v1: US0, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool) -> i32 {
    let v6: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = concat5_2(v0.clone(), v7, v8.clone(), v8.clone(), v8.clone());
    let v10: bool = std::path::Path::new(&*v9).is_file();
    let v13: US1 = if v10 {
        US1::US1_FileReadable(v9)
    } else {
        US1::US1_FileMissing(v9)
    };
    match v13 {
        US1::US1_FileMissing(v14) => {
            let v14: Rc<str> = v14;
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile next rejected: "); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = concat5_2(v16, v15, v8.clone(), v8.clone(), v8.clone());
            let v18: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v17 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v18
        }
        US1::US1_FileReadable(v19) => {
            let mut v19: Rc<str> = v19;
            let mut v20: Rc<str> = std::fs::read_to_string(&*v19).unwrap_or_default().into();
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = Rc::<str>::from(v19.replace(&*v21, &*v22));
            let mut v24: Rc<str> = loop_3(v23);
            let v25: Option<usize> = v24.rfind(&*v22);
            let v26: Option<usize> = method4(v25);
            let mut v27: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v28: Option<i32> = v26.map(|x| v27(x));
            let v30: i32 = v28.unwrap_or(-1i32);
            let v31: bool = v30 < 0i32;
            let v35: bool = if v31 {
                true
            } else {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v33: bool = v24.ends_with(&*v32);
                let v34: bool = v33 == false;
                v34
            };
            let v56: US2 = if v35 {
                US2::US2_OtherFile
            } else {
                let v37: usize = ((0i32) as usize);
                let v38: usize = ((v30) as usize);
                let v39: Rc<str> = Rc::<str>::from(&(v24)[v37..v38]);
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v41: bool = v39.ends_with(&*v40);
                if v41 {
                    let v42: Rc<str> = loop_5(v39);
                    US2::US2_StateFile(v42)
                } else {
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v45: bool = v39.ends_with(&*v44);
                    if v45 {
                        let v46: Rc<str> = loop_6(v39);
                        US2::US2_HistoryFile(v46)
                    } else {
                        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v49: bool = v39.ends_with(&*v48);
                        if v49 {
                            let v50: Rc<str> = loop_7(v39);
                            US2::US2_ArchiveFile(v50)
                        } else {
                            US2::US2_OtherFile
                        }
                    }
                }
            };
            let v67: Rc<str> = match v56 {
                US2::US2_OtherFile => {
                    v20
                }
                _ => {
                    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v58: Rc<UH0> = split_list_8(v57.clone(), v20);
                    let v59: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v60: Rc<UH0> = method11(v58, v59);
                    let v61: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v62: i32 = 0i32;
                    let v63: i32 = method12(v61.clone(), v60, v62);
                    let v64: Rc<Vec<Rc<str>>> = Rc::new(v61.borrow().clone());
                    let v65: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v64).as_ref().clone()));
                    let v66: Rc<str> = Rc::<str>::from((*v65.borrow()).join(&*v57));
                    v66
                }
            };
            let v68: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v67);
            let v69: i32 = (v68.clone().len() as i32);
            let v70: i32 = 0i32;
            let v71: i32 = first_malformed_task_13(v68.clone(), v70, v69);
            let v72: bool = v71 < 0i32;
            if v72 {
                let v73: Rc<RefCell<Vec<Rc<str>>>> = agile_waits_hold::rig_agile_waits_holding(&v0, &v68);
                let v74: u64 = v6.wrapping_div(3600000u64);
                let v75: i64 = (v74 as i64);
                let v76: i32 = 0i32;
                let v77: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoTasks); } CASE.with(|case| case.clone()) };
                let v78: Rc<UH1> = tasks_from_17(v68.clone(), v76, v69, v77);
                let v79: Rc<UH1> = annotated_tasks_20(v0.clone(), v68.clone(), v75, v78);
                let v80: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>, Rc<str>) -> bool> = closure2(v1.clone(), v2, v3.clone(), v4.clone(), v5, v68.clone(), v73);
                let v81: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_NoRanked); } CASE.with(|case| case.clone()) };
                let v82: Rc<UH2> = ranked_from_26(v1.clone(), v80, v79.clone(), v79.clone(), v81);
                let v83: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(vec![]));
                let v84: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<str>)>>> = rows_of_37(v82.clone(), v2, v83);
                let v85: bool = match &v1 {
                    US0::US0_MainAgent => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v86: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v6);
                let v90: Rc<str> = match v1 {
                    US0::US0_MainAgent => {
                        let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("main"); } LIT.with(|lit| lit.clone()) };
                        v87
                    }
                    US0::US0_Subagent => {
                        let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("subagent"); } LIT.with(|lit| lit.clone()) };
                        v88
                    }
                };
                let v91: i32 = 0i32;
                let v92: i32 = ranked_count_39(v82, v91);
                let v93: i32 = agile_next_print::rig_agile_next_printed(v84, &v90, v85, v5, &v4, v86, v92);
                let v95: i32 = if v5 {
                    0i32
                } else {
                    let v94: i32 = agile_waits_hold::rig_agile_lane_waits_printed(&v0, &v3, &v68);
                    v94
                };
                v93
            } else {
                let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed Task fields at byte "); } LIT.with(|lit| lit.clone()) };
                let v97: Rc<str> = std::rc::Rc::<str>::from((v71).to_string());
                let v98: Rc<str> = concat5_2(v96, v97, v8.clone(), v8.clone(), v8.clone());
                let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile next rejected: "); } LIT.with(|lit| lit.clone()) };
                let v100: Rc<str> = concat5_2(v99, v98, v8.clone(), v8.clone(), v8.clone());
                let v101: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v100 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v101
            }
        }
    }
}
fn agile_next_entry_0() -> i32 {
    let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (bool, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, bool) = rig_agile_next_args::rig_agile_next_request();
    if v0 {
        let v7: bool = &*v2 == "subagent";
        let v10: US0 = if v7 {
            US0::US0_Subagent
        } else {
            US0::US0_MainAgent
        };
        let v11: i32 = 1i32;
        let v12: Rc<str> = usize::try_from(v11).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        agile_next_1(v12, v10, v3, v4, v5, v6)
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile next rejected: "); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = concat5_2(v14, v1, v15.clone(), v15.clone(), v15.clone());
        let v17: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v16 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v17
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_next_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_next_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_2617b4e49837 = Mut1;
