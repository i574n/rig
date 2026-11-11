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
    US5_Some(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>),
    US5_None,
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US6 {
    US6_SpiprojProject,
    US6_TomlProject,
}
#[derive(Clone)]
enum US7 {
    US7_SplitProject(Rc<UH0>, Rc<UH0>, Rc<UH0>),
    US7_NoModules,
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
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
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
fn loop_6(mut v0: Rc<str>) -> Rc<str> {
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
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method9(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
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
        while method9(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
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
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method10(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let mut v15: US2 = if v6 {
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
fn method11(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn field_end_15(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US4) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US4::US4_ScanPlain => {
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
                let mut v25: US4 = match v3 {
                    US4::US4_ScanEscaped => {
                        US4::US4_ScanQuoted
                    }
                    US4::US4_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
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
fn field_start_14(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US4 = US4::US4_ScanPlain;
            let mut v6: i32 = field_end_15(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_16(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn backlog_task_at_13(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>) {
    let v3: i32 = 0i32;
    let mut v4: i32 = field_start_14(v0.clone(), v1, v2, v3);
    let v5: US4 = US4::US4_ScanPlain;
    let mut v6: i32 = field_end_15(v0.clone(), v4, v2, v5);
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let v8: i32 = (v7.clone().len() as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let v11: i32 = 1i32;
    let v12: i32 = field_start_14(v0.clone(), v1, v2, v11);
    let mut v13: US4 = US4::US4_ScanPlain;
    let mut v14: i32 = field_end_15(v0.clone(), v12, v2, v13);
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: i32 = (v15.clone().len() as i32);
    let mut v17: i32 = v16.wrapping_sub(1i32);
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v15.get(1i32 as usize..v17 as usize).unwrap_or(""));
    let mut v19: i32 = 2i32;
    let v20: i32 = field_start_14(v0.clone(), v1, v2, v19);
    let v21: US4 = US4::US4_ScanPlain;
    let v22: i32 = field_end_15(v0.clone(), v20, v2, v21);
    let v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let v24: i32 = 3i32;
    let mut v25: i32 = field_start_14(v0.clone(), v1, v2, v24);
    let v26: US4 = US4::US4_ScanPlain;
    let v27: i32 = field_end_15(v0.clone(), v25, v2, v26);
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let mut v29: i32 = (v28).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
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
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v4)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v6: bool = v5 < 0i32;
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
                let mut v25: Rc<UH1> = Rc::new(UH1::UH1_TaskThen(v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v3));
                (v1, v3) = (v7, v25);
                continue;
            } else {
                v1 = v7;
                continue;
            }
        }
    }
}
fn task_found_17(mut v0: Rc<UH1>, mut v1: Rc<str>) -> US5 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_NoTasks => {
                return US5::US5_None;
            }
            UH1::UH1_TaskThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let mut v6: i32 = v6;
                let v7: i32 = v7;
                let v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: i32 = v11;
                let v12: i32 = v12;
                let mut v13: i32 = v13;
                let mut v14: bool = v14;
                let mut v15: Rc<str> = v15;
                let v16: Rc<UH1> = v16;
                let mut v17: bool = &*v3 == &*v1;
                if v17 {
                    return US5::US5_Some(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15);
                } else {
                    v0 = v16;
                    continue;
                }
            }
        }
    }
}
fn method20(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_19(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method20(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v1 == "";
                let v7: bool = if v4 {
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team:"); } LIT.with(|lit| lit.clone()) };
                    let mut v6: bool = v2.starts_with(&*v5);
                    v6
                } else {
                    false
                };
                let mut v14: Rc<str> = if v7 {
                    let v8: i32 = (v2.clone().len() as i32);
                    let mut v9: i32 = v8.wrapping_sub(5i32);
                    let v10: usize = ((5i32) as usize);
                    let v11: i32 = 5i32.wrapping_add(v9);
                    let v12: usize = ((v11) as usize);
                    let mut v13: Rc<str> = Rc::<str>::from(&(v2)[v10..v12]);
                    v13
                } else {
                    v1
                };
                (v0, v1) = (v3, v14);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn team_of_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = split_list_7(v1, v0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = method22(v2, v3);
    let v5: bool = &*v4 == "";
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        v4
    }
}
fn task_line_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: &Rc<str>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: &Rc<str>, mut v7: Rc<str>, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: &Rc<str>) -> Rc<str> {
    let mut v13: Rc<str> = format_real_19(v3);
    let mut v14: Rc<str> = string_concat(&["P", &*v13]);
    let mut v15: Rc<str> = format_real_19(v4);
    let v16: Rc<str> = string_concat(&["D", &*v15]);
    let mut v17: Rc<str> = team_of_21(v7);
    let mut v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("item"); } LIT.with(|lit| lit.clone()) };
    v18.borrow_mut().push(v19);
    v18.borrow_mut().push(v0);
    v18.borrow_mut().push(v14);
    v18.borrow_mut().push(v16);
    v18.borrow_mut().push(v5);
    v18.borrow_mut().push(v17);
    v18.borrow_mut().push(v1);
    let v20: Rc<Vec<Rc<str>>> = Rc::new(v18.borrow().clone());
    let v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v20).as_ref().clone()));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from((*v21.borrow()).join(&*v22));
    v23
}
fn state_file_text_24(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
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
            let mut v14: Rc<str> = loop_2(v13);
            let mut v15: Option<usize> = v14.rfind(&*v5);
            let v16: Option<usize> = method3(v15);
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
            let v46: US1 = if v25 {
                US1::US1_OtherFile
            } else {
                let v27: usize = ((0i32) as usize);
                let mut v28: usize = ((v20) as usize);
                let mut v29: Rc<str> = Rc::<str>::from(&(v14)[v27..v28]);
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
            match v46 {
                US1::US1_OtherFile => {
                    v9
                }
                _ => {
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<UH0> = split_list_7(v47.clone(), v9);
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
            }
        }
    } else {
        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v59
    }
}
fn method26(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method26(v3, v1);
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
fn section_lines_28(mut v0: US6, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> US7 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v8, v9) => {
                let v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH0> = v9.clone();
                let mut v19: bool = match &v0 {
                    US6::US6_SpiprojProject => {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v11: bool = v8.starts_with(&*v10);
                        let mut v14: bool = if v11 {
                            true
                        } else {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v13: bool = v8.starts_with(&*v12);
                            v13
                        };
                        let mut v15: bool = v14 == false;
                        v15
                    }
                    US6::US6_TomlProject => {
                        let v16: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                        let mut v17: bool = &*v16 == "'''";
                        v17
                    }
                };
                if v19 {
                    let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v21: Rc<UH0> = method8(v3, v20);
                    return US7::US7_SplitProject(v2, v21, v1);
                } else {
                    let v23: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                    let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v3));
                    (v1, v3) = (v9, v24);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH0> = method8(v3, v4);
                let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return US7::US7_SplitProject(v2, v5, v6);
            }
        }
    }
}
fn split_lines_27(mut v0: US6, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> US7 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<str> = Rc::<str>::from((v4).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v10: Rc<str> = match &v0 {
                    US6::US6_SpiprojProject => {
                        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                        v7
                    }
                    US6::US6_TomlProject => {
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                        v8
                    }
                };
                let v11: bool = &*v6 == &*v10;
                if v11 {
                    let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v12));
                    let mut v14: Rc<UH0> = method8(v2, v13);
                    let mut v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    return section_lines_28(v0, v5, v14, v15);
                } else {
                    let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v2));
                    (v1, v2) = (v5, v17);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US7::US7_NoModules;
            }
        }
    }
}
fn method29(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method29(v3, v1);
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
fn method30(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method30(v0.clone(), v4, v2);
            let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v6.borrow_mut().push(v3);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            v6.borrow_mut().push(v7);
            let v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
            let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
            let v12: Rc<str> = state_file_text_24(v0, v11);
            let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            v13.borrow_mut().push(v14);
            v13.borrow_mut().push(v12);
            let mut v15: Rc<Vec<Rc<str>>> = Rc::new(v13.borrow().clone());
            let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
            let mut v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v10));
            Rc::new(UH0::UH0_Cons(v17, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn state_segments_text_25(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v1.borrow_mut().push(v0.clone());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.toml"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v2.clone());
    let v3: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v3).as_ref().clone()));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from((*v4.borrow()).join(&*v5));
    let v7: bool = std::path::Path::new(&*v6).is_file();
    let v10: US6 = if v7 {
        US6::US6_TomlProject
    } else {
        US6::US6_SpiprojProject
    };
    let mut v13: Rc<str> = match v10 {
        US6::US6_SpiprojProject => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US6::US6_TomlProject => {
            v2.clone()
        }
    };
    let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v14.borrow_mut().push(v0.clone());
    v14.borrow_mut().push(v13);
    let mut v15: Rc<Vec<Rc<str>>> = Rc::new(v14.borrow().clone());
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
            let mut v25: Rc<str> = loop_2(v24);
            let v26: Option<usize> = v25.rfind(&*v5);
            let v27: Option<usize> = method3(v26);
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
            let v57: US1 = if v36 {
                US1::US1_OtherFile
            } else {
                let v38: usize = ((0i32) as usize);
                let v39: usize = ((v31) as usize);
                let v40: Rc<str> = Rc::<str>::from(&(v25)[v38..v39]);
                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v42: bool = v40.ends_with(&*v41);
                if v42 {
                    let v43: Rc<str> = loop_4(v40);
                    US1::US1_StateFile(v43)
                } else {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v46: bool = v40.ends_with(&*v45);
                    if v46 {
                        let v47: Rc<str> = loop_5(v40);
                        US1::US1_HistoryFile(v47)
                    } else {
                        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v50: bool = v40.ends_with(&*v49);
                        if v50 {
                            let v51: Rc<str> = loop_6(v40);
                            US1::US1_ArchiveFile(v51)
                        } else {
                            US1::US1_OtherFile
                        }
                    }
                }
            };
            match v57 {
                US1::US1_OtherFile => {
                    v20
                }
                _ => {
                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v59: Rc<UH0> = split_list_7(v58.clone(), v20);
                    let v60: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v61: Rc<UH0> = method10(v59, v60);
                    let v62: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v63: i32 = 0i32;
                    let v64: i32 = method11(v62.clone(), v61, v63);
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
    let v79: US6 = if v76 {
        US6::US6_TomlProject
    } else {
        US6::US6_SpiprojProject
    };
    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v83: Rc<str> = Rc::<str>::from(v71.replace(&*v81, &*v82));
    let v84: Rc<UH0> = split_list_7(v80, v83);
    let v85: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v86: Rc<UH0> = method26(v84, v85);
    let v87: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v88: US7 = split_lines_27(v79, v86, v87);
    let v94: Rc<UH0> = match v88 {
        US7::US7_NoModules => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US7::US7_SplitProject(v89, v90, v91) => {
            let v89: Rc<UH0> = v89;
            let v90: Rc<UH0> = v90;
            let v91: Rc<UH0> = v91;
            v90
        }
    };
    let v95: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v96: Rc<UH0> = method29(v94, v95);
    let v97: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v98: Rc<UH0> = method30(v0, v96, v97);
    let v99: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v100: i32 = 0i32;
    let v101: i32 = method11(v99.clone(), v98, v100);
    let v102: Rc<Vec<Rc<str>>> = Rc::new(v99.borrow().clone());
    let v103: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v102).as_ref().clone()));
    let v104: Rc<str> = Rc::<str>::from((*v103.borrow()).join(&*v82));
    v104
}
fn state_text_with_notes_23(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = state_file_text_24(v0.clone(), v1);
    let v3: Rc<str> = state_segments_text_25(v0);
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v4.borrow_mut().push(v2);
    v4.borrow_mut().push(v3);
    let v5: Rc<Vec<Rc<str>>> = Rc::new(v4.borrow().clone());
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from((*v6.borrow()).join(&*v7));
    v8
}
fn note_text_line_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v7));
    let mut v13: Rc<str> = { let text = &*v8; if text.chars().count() <= 300i32 as usize { std::rc::Rc::<str>::from(text) } else { std::rc::Rc::<str>::from(text.chars().take((300i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
    let mut v14: Rc<str> = string_concat(&["last-note ", &*v0, " ", &*v1, " ", &*v13]);
    v14
}
fn declared_from_34(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
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
fn last_declared_33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
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
fn unescaped_from_37(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return concat5_1(v4, v6, v7.clone(), v7.clone(), v7.clone());
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v10: bool = v9 == 92i32;
            if v10 {
                let v11: i32 = v1.wrapping_add(1i32);
                let v12: i32 = v11.wrapping_add(1i32);
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v15: bool = v14 == 110i32;
                let v24: Rc<str> = if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16
                } else {
                    let mut v17: bool = v14 == 114i32;
                    if v17 {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18
                    } else {
                        let mut v19: bool = v14 == 116i32;
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
                let v26: Rc<str> = concat5_1(v4, v13, v24, v25.clone(), v25.clone());
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
fn unquoted_36(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let mut v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_37(v3, v4, v5, v6, v7)
}
fn note_at_35(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1.wrapping_add(6i32);
    let v3: i32 = (v0.clone().len() as i32);
    let mut v4: i32 = 2i32;
    let v5: i32 = field_start_14(v0.clone(), v2, v3, v4);
    let mut v6: US4 = US4::US4_ScanPlain;
    let v7: i32 = field_end_15(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_36(v8);
    let v10: i32 = 3i32;
    let v11: i32 = field_start_14(v0.clone(), v2, v3, v10);
    let v12: US4 = US4::US4_ScanPlain;
    let mut v13: i32 = field_end_15(v0.clone(), v11, v3, v12);
    let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v13 as usize).unwrap_or(""));
    let mut v15: Rc<str> = unquoted_36(v14);
    let v16: i32 = 4i32;
    let mut v17: i32 = field_start_14(v0.clone(), v2, v3, v16);
    let mut v18: US4 = US4::US4_ScanPlain;
    let mut v19: i32 = field_end_15(v0.clone(), v17, v3, v18);
    let v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let v21: Rc<str> = unquoted_36(v20);
    (v9, v15, v21)
}
fn last_note_line_31(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = concat5_1(v2, v3, v1, v4, v5);
    let v7: i32 = 0i32;
    let v8: i32 = -1i32;
    let mut v9: i32 = last_declared_33(v0.clone(), v6, v7, v8);
    let v10: bool = v9 < 0i32;
    if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("last-note none"); } LIT.with(|lit| lit.clone()) };
        v11
    } else {
        let (mut v12, mut v13, mut v14): (Rc<str>, Rc<str>, Rc<str>) = note_at_35(v0, v9);
        note_text_line_32(v12, v13, v14)
    }
}
fn claim_item_lines_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = concat5_1(v0.clone(), v2, v3.clone(), v3.clone(), v3.clone());
    let v5: bool = std::path::Path::new(&*v4).is_file();
    let v8: US0 = if v5 {
        US0::US0_FileReadable(v4)
    } else {
        US0::US0_FileMissing(v4)
    };
    match v8 {
        US0::US0_FileMissing(v9) => {
            let mut v9: Rc<str> = v9;
            v3
        }
        US0::US0_FileReadable(v10) => {
            let v10: Rc<str> = v10;
            let v11: Rc<str> = std::fs::read_to_string(&*v10).unwrap_or_default().into();
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = Rc::<str>::from(v10.replace(&*v12, &*v13));
            let mut v15: Rc<str> = loop_2(v14);
            let v16: Option<usize> = v15.rfind(&*v13);
            let mut v17: Option<usize> = method3(v16);
            let mut v18: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v19: Option<i32> = v17.map(|x| v18(x));
            let v21: i32 = v19.unwrap_or(-1i32);
            let v22: bool = v21 < 0i32;
            let v26: bool = if v22 {
                true
            } else {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v24: bool = v15.ends_with(&*v23);
                let mut v25: bool = v24 == false;
                v25
            };
            let v47: US1 = if v26 {
                US1::US1_OtherFile
            } else {
                let mut v28: usize = ((0i32) as usize);
                let mut v29: usize = ((v21) as usize);
                let v30: Rc<str> = Rc::<str>::from(&(v15)[v28..v29]);
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v32: bool = v30.ends_with(&*v31);
                if v32 {
                    let v33: Rc<str> = loop_4(v30);
                    US1::US1_StateFile(v33)
                } else {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v36: bool = v30.ends_with(&*v35);
                    if v36 {
                        let v37: Rc<str> = loop_5(v30);
                        US1::US1_HistoryFile(v37)
                    } else {
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v40: bool = v30.ends_with(&*v39);
                        if v40 {
                            let v41: Rc<str> = loop_6(v30);
                            US1::US1_ArchiveFile(v41)
                        } else {
                            US1::US1_OtherFile
                        }
                    }
                }
            };
            let v58: Rc<str> = match v47 {
                US1::US1_OtherFile => {
                    v11
                }
                _ => {
                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v49: Rc<UH0> = split_list_7(v48.clone(), v11);
                    let v50: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v51: Rc<UH0> = method10(v49, v50);
                    let v52: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v53: i32 = 0i32;
                    let v54: i32 = method11(v52.clone(), v51, v53);
                    let v55: Rc<Vec<Rc<str>>> = Rc::new(v52.borrow().clone());
                    let v56: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v55).as_ref().clone()));
                    let v57: Rc<str> = Rc::<str>::from((*v56.borrow()).join(&*v48));
                    v57
                }
            };
            let v59: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v58);
            let v60: i32 = 0i32;
            let v61: i32 = (v59.clone().len() as i32);
            let v62: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoTasks); } CASE.with(|case| case.clone()) };
            let v63: Rc<UH1> = tasks_from_12(v59, v60, v61, v62);
            let v64: US5 = task_found_17(v63, v1.clone());
            match v64 {
                US5::US5_None => {
                    v3
                }
                US5::US5_Some(v65, v66, v67, v68, v69, v70, v71, v72, v73, v74, v75, v76, v77) => {
                    let v65: Rc<str> = v65;
                    let v66: Rc<str> = v66;
                    let v67: Rc<str> = v67;
                    let v68: i32 = v68;
                    let v69: i32 = v69;
                    let v70: Rc<str> = v70;
                    let v71: Rc<str> = v71;
                    let mut v72: Rc<str> = v72;
                    let v73: i32 = v73;
                    let v74: i32 = v74;
                    let v75: i32 = v75;
                    let v76: bool = v76;
                    let v77: Rc<str> = v77;
                    let v78: Rc<str> = task_line_18(v65, v66, &v67, v68, v69, v70, &v71, v72, v73, v74, v75, v76, &v77);
                    let v80: Rc<str> = string_concat(&[&*v0, "/state"]);
                    let v81: Rc<str> = state_text_with_notes_23(v80);
                    let v82: Rc<str> = last_note_line_31(v81, v1);
                    let v83: Rc<str> = string_concat(&[&*v78, "\n", &*v82]);
                    v83
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        claim_item_lines_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_claim_item_lines(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
