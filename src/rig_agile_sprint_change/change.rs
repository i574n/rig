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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_HeadInl,
    US2_HeadUnion,
    US2_HeadOpen,
    US2_HeadType,
}
#[derive(Clone)]
enum US1 {
    US1_KeywordHead(US2),
    US1_SymbolHead(US2),
    US1_OtherLine,
}
#[derive(Clone)]
enum US3 {
    US3_ItemOpen,
    US3_ItemDone,
    US3_ItemUnknown,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, US3, Rc<UH1>),
}
#[derive(Clone)]
enum US6 {
    US6_SprintOpen,
    US6_SprintClosed,
}
#[derive(Clone)]
enum US7 {
    US7_Committed,
    US7_Dropped,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, i32, US7, Rc<UH2>),
}
#[derive(Clone)]
enum US5 {
    US5_Started(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US6, Rc<UH2>),
    US5_AnotherSprintOpen(Rc<str>),
    US5_ItemNotOpen(Rc<str>),
    US5_ItemUnestimated(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(US5),
    US4_None,
}
#[derive(Clone)]
enum US8 {
    US8_WordsRoot,
    US8_SymbolsRoot,
}
#[derive(Clone)]
enum US9 {
    US9_Some(Rc<str>),
    US9_None,
}
fn loop_2(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
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
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(6i32);
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
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
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
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
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
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method9(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
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
            let v6: bool = v2.starts_with(&*v5);
            let mut v15: US1 = if v6 {
                let v7: US2 = US2::US2_HeadType;
                US1::US1_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US2 = US2::US2_HeadType;
                    US1::US1_SymbolHead(v11)
                } else {
                    US1::US1_OtherLine
                }
            };
            let v27: US1 = match &v15 {
                US1::US1_OtherLine => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let mut v18: US2 = US2::US2_HeadInl;
                        US1::US1_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US2 = US2::US2_HeadInl;
                            US1::US1_SymbolHead(v22)
                        } else {
                            US1::US1_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US1 = match &v27 {
                US1::US1_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US2 = US2::US2_HeadUnion;
                        US1::US1_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US2 = US2::US2_HeadUnion;
                            US1::US1_SymbolHead(v34)
                        } else {
                            US1::US1_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US1 = match &v39 {
                US1::US1_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US2 = US2::US2_HeadOpen;
                        US1::US1_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US2 = US2::US2_HeadOpen;
                            US1::US1_SymbolHead(v46)
                        } else {
                            US1::US1_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US1::US1_SymbolHead(v52) => {
                    let v52: US2 = v52;
                    let v60: Rc<str> = match &v52 {
                        US2::US2_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US2::US2_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US2::US2_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US2::US2_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US2::US2_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US2::US2_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US2::US2_HeadType => {
                            v5
                        }
                        US2::US2_HeadUnion => {
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
fn sprint_has_12(mut v0: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v2 >= v3;
        if v4 {
            return false;
        } else {
            let (mut v5, mut v6, mut v7): (Rc<str>, i32, bool) = v0.borrow()[v2 as usize].clone();
            let v8: bool = &*v5 == &*v1;
            if v8 {
                return true;
            } else {
                let mut v9: i32 = v2.wrapping_add(1i32);
                v2 = v9;
                continue;
            }
        }
    }
}
fn standing_of_14(mut v0: Rc<str>) -> US3 {
    let mut v1: bool = &*v0 == "";
    if v1 {
        US3::US3_ItemUnknown
    } else {
        let v3: bool = &*v0 == "Done";
        if v3 {
            US3::US3_ItemDone
        } else {
            US3::US3_ItemOpen
        }
    }
}
fn candidate_of_13(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, i32, US3) {
    let mut v2: Rc<str> = agile_task_census::rig_agile_task_status_of(&v0, &v1);
    let v3: i32 = agile_planning_fields::rig_agile_planning_minutes_of(&v0, &v1);
    let mut v4: US3 = standing_of_14(v2);
    (v1, v3, v4)
}
fn method16(mut v0: Rc<UH1>, mut v1: US4) -> US4 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let mut v4: US3 = v4;
                let v5: Rc<UH1> = v5;
                let mut v18: US4 = match &v1 {
                    US4::US4_None => {
                        let v7: bool = match v4 {
                            US3::US3_ItemOpen => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v8: bool = v7 != true;
                        if v8 {
                            let mut v9: US5 = US5::US5_ItemNotOpen(v2);
                            US4::US4_Some(v9)
                        } else {
                            let mut v11: bool = v3 <= 0i32;
                            if v11 {
                                let mut v12: US5 = US5::US5_ItemUnestimated(v2);
                                US4::US4_Some(v12)
                            } else {
                                US4::US4_None
                            }
                        }
                    }
                    US4::US4_Some(v6) => {
                        let v6: US5 = v6.clone();
                        v1
                    }
                };
                (v0, v1) = (v5, v18);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn first_refusal_15(mut v0: Rc<UH1>) -> US4 {
    let mut v1: US4 = US4::US4_None;
    method16(v0, v1)
}
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method17(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let mut v15: US1 = if v6 {
                let v7: US2 = US2::US2_HeadType;
                US1::US1_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US2 = US2::US2_HeadType;
                    US1::US1_SymbolHead(v11)
                } else {
                    US1::US1_OtherLine
                }
            };
            let v27: US1 = match &v15 {
                US1::US1_OtherLine => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let mut v18: US2 = US2::US2_HeadInl;
                        US1::US1_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US2 = US2::US2_HeadInl;
                            US1::US1_SymbolHead(v22)
                        } else {
                            US1::US1_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US1 = match &v27 {
                US1::US1_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US2 = US2::US2_HeadUnion;
                        US1::US1_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US2 = US2::US2_HeadUnion;
                            US1::US1_SymbolHead(v34)
                        } else {
                            US1::US1_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US1 = match &v39 {
                US1::US1_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US2 = US2::US2_HeadOpen;
                        US1::US1_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US2 = US2::US2_HeadOpen;
                            US1::US1_SymbolHead(v46)
                        } else {
                            US1::US1_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US1::US1_KeywordHead(v52) => {
                    let v52: US2 = v52;
                    let v59: Rc<str> = match &v52 {
                        US2::US2_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US2::US2_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US2::US2_HeadType => {
                            v5
                        }
                        US2::US2_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v60: i32 = (v59.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US2::US2_HeadInl => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v61
                        }
                        US2::US2_HeadOpen => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                        US2::US2_HeadType => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US2::US2_HeadUnion => {
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
fn sprint_changed_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool) -> i32 {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
    let v7: Rc<str> = string_concat(&[&*v6, "/state"]);
    let v8: Rc<str> = string_concat(&[&*v7, "/sprints.spi"]);
    let mut v9: bool = std::path::Path::new(&*v8).is_file();
    let v61: Rc<str> = if v9 {
        let (mut v10, mut v11): (bool, Rc<str>) = match std::fs::read(&*v8) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v10 {
            let mut v12: Rc<str> = string_concat(&["rig state read failed: ", &*v8]);
            std::panic::panic_any::<std::string::String>(format!("{}", v12))
        } else {
            let v14: Rc<str> = Rc::<str>::from(v8.replace(&*v4, &*v5));
            let mut v15: Rc<str> = loop_2(v14);
            let mut v16: Option<usize> = v15.rfind(&*v5);
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
                let v25: bool = v24 == false;
                v25
            };
            let v47: US0 = if v26 {
                US0::US0_OtherFile
            } else {
                let v28: usize = ((0i32) as usize);
                let v29: usize = ((v21) as usize);
                let v30: Rc<str> = Rc::<str>::from(&(v15)[v28..v29]);
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v32: bool = v30.ends_with(&*v31);
                if v32 {
                    let v33: Rc<str> = loop_4(v30);
                    US0::US0_StateFile(v33)
                } else {
                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v36: bool = v30.ends_with(&*v35);
                    if v36 {
                        let v37: Rc<str> = loop_5(v30);
                        US0::US0_HistoryFile(v37)
                    } else {
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v40: bool = v30.ends_with(&*v39);
                        if v40 {
                            let v41: Rc<str> = loop_6(v30);
                            US0::US0_ArchiveFile(v41)
                        } else {
                            US0::US0_OtherFile
                        }
                    }
                }
            };
            match v47 {
                US0::US0_OtherFile => {
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
            }
        }
    } else {
        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v60
    };
    let v62: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>> = rig_agile_sprint_state::rig_agile_sprint_head(&v61, &v1);
    let v65: Rc<str> = if v3 {
        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("drop"); } LIT.with(|lit| lit.clone()) };
        v63
    } else {
        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
        v64
    };
    let v66: i32 = (v62.clone().borrow().len() as i32);
    let v67: bool = v66 == 0i32;
    if v67 {
        let v71: Rc<str> = string_concat(&["rig error: agile sprint ", &*v65, " refused: no sprint ", &*v1]);
        eprintln!("{}", v71);
        1i32
    } else {
        let (mut v72, mut v73, mut v74, mut v75, mut v76): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) = v62.borrow()[0i32 as usize].clone();
        let v77: bool = v76 == false;
        if v77 {
            let mut v82: Rc<str> = string_concat(&["rig error: agile sprint ", &*v65, " refused: sprint ", &*v1, " is closed"]);
            eprintln!("{}", v82);
            1i32
        } else {
            let mut v83: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> = rig_agile_sprint_state::rig_agile_sprint_items(&v61, &v1);
            let v84: i32 = 0i32;
            let v85: bool = sprint_has_12(&v83, v2.clone(), v84);
            let v86: Rc<str> = string_concat(&[&*v7, "/agile.spi"]);
            let v87: bool = std::path::Path::new(&*v86).is_file();
            let v139: Rc<str> = if v87 {
                let (mut v88, mut v89): (bool, Rc<str>) = match std::fs::read(&*v86) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                if v88 {
                    let v90: Rc<str> = string_concat(&["rig state read failed: ", &*v86]);
                    std::panic::panic_any::<std::string::String>(format!("{}", v90))
                } else {
                    let v92: Rc<str> = Rc::<str>::from(v86.replace(&*v4, &*v5));
                    let mut v93: Rc<str> = loop_2(v92);
                    let v94: Option<usize> = v93.rfind(&*v5);
                    let v95: Option<usize> = method3(v94);
                    let mut v96: Rc<dyn Fn((usize)) -> i32> = closure1();
                    let mut v97: Option<i32> = v95.map(|x| v96(x));
                    let v99: i32 = v97.unwrap_or(-1i32);
                    let v100: bool = v99 < 0i32;
                    let v104: bool = if v100 {
                        true
                    } else {
                        let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                        let v102: bool = v93.ends_with(&*v101);
                        let v103: bool = v102 == false;
                        v103
                    };
                    let v125: US0 = if v104 {
                        US0::US0_OtherFile
                    } else {
                        let v106: usize = ((0i32) as usize);
                        let v107: usize = ((v99) as usize);
                        let v108: Rc<str> = Rc::<str>::from(&(v93)[v106..v107]);
                        let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                        let v110: bool = v108.ends_with(&*v109);
                        if v110 {
                            let v111: Rc<str> = loop_4(v108);
                            US0::US0_StateFile(v111)
                        } else {
                            let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                            let v114: bool = v108.ends_with(&*v113);
                            if v114 {
                                let v115: Rc<str> = loop_5(v108);
                                US0::US0_HistoryFile(v115)
                            } else {
                                let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                let v118: bool = v108.ends_with(&*v117);
                                if v118 {
                                    let v119: Rc<str> = loop_6(v108);
                                    US0::US0_ArchiveFile(v119)
                                } else {
                                    US0::US0_OtherFile
                                }
                            }
                        }
                    };
                    match v125 {
                        US0::US0_OtherFile => {
                            v89
                        }
                        _ => {
                            let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let v127: Rc<UH0> = split_list_7(v126.clone(), v89);
                            let v128: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v129: Rc<UH0> = method10(v127, v128);
                            let v130: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v131: i32 = 0i32;
                            let mut v132: i32 = method11(v130.clone(), v129, v131);
                            let v133: Rc<Vec<Rc<str>>> = Rc::new(v130.borrow().clone());
                            let v134: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v133).as_ref().clone()));
                            let mut v135: Rc<str> = Rc::<str>::from((*v134.borrow()).join(&*v126));
                            v135
                        }
                    }
                }
            } else {
                let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v138
            };
            let (mut v140, mut v141, mut v142): (Rc<str>, i32, US3) = candidate_of_13(v139, v2.clone());
            let v163: Rc<str> = if v3 {
                if v85 {
                    let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v143
                } else {
                    let v146: Rc<str> = string_concat(&["item ", &*v2, " is not in sprint ", &*v1]);
                    v146
                }
            } else {
                let v148: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v149: Rc<UH1> = Rc::new(UH1::UH1_Cons(v140, v141, v142, v148));
                let v150: US4 = first_refusal_15(v149);
                match v150 {
                    US4::US4_Some(v151) => {
                        let v151: US5 = v151;
                        match v151 {
                            US5::US5_ItemNotOpen(v152) => {
                                let v152: Rc<str> = v152;
                                let v154: Rc<str> = string_concat(&["item ", &*v2, " is unknown or Done"]);
                                v154
                            }
                            US5::US5_ItemUnestimated(v155) => {
                                let v155: Rc<str> = v155;
                                let v157: Rc<str> = string_concat(&["item ", &*v2, " has no estimate"]);
                                v157
                            }
                            _ => {
                                let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v158
                            }
                        }
                    }
                    _ => {
                        let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v161
                    }
                }
            };
            let v164: bool = &*v163 != "";
            if v164 {
                let v167: Rc<str> = string_concat(&["rig error: agile sprint ", &*v65, " refused: ", &*v163]);
                eprintln!("{}", v167);
                1i32
            } else {
                let v168: Rc<str> = rig_agile_sprint_append::rig_agile_sprint_item_appended(&v61, &v1, &v2, v141, v3);
                let v169: Rc<str> = Rc::<str>::from(v8.replace(&*v4, &*v5));
                let mut v170: Rc<str> = loop_2(v169);
                let v171: Option<usize> = v170.rfind(&*v5);
                let v172: Option<usize> = method3(v171);
                let mut v173: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v174: Option<i32> = v172.map(|x| v173(x));
                let v176: i32 = v174.unwrap_or(-1i32);
                let v177: bool = v176 < 0i32;
                let v181: bool = if v177 {
                    true
                } else {
                    let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v179: bool = v170.ends_with(&*v178);
                    let v180: bool = v179 == false;
                    v180
                };
                let v202: US0 = if v181 {
                    US0::US0_OtherFile
                } else {
                    let v183: usize = ((0i32) as usize);
                    let v184: usize = ((v176) as usize);
                    let v185: Rc<str> = Rc::<str>::from(&(v170)[v183..v184]);
                    let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v187: bool = v185.ends_with(&*v186);
                    if v187 {
                        let v188: Rc<str> = loop_4(v185);
                        US0::US0_StateFile(v188)
                    } else {
                        let v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v191: bool = v185.ends_with(&*v190);
                        if v191 {
                            let v192: Rc<str> = loop_5(v185);
                            US0::US0_HistoryFile(v192)
                        } else {
                            let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v195: bool = v185.ends_with(&*v194);
                            if v195 {
                                let v196: Rc<str> = loop_6(v185);
                                US0::US0_ArchiveFile(v196)
                            } else {
                                US0::US0_OtherFile
                            }
                        }
                    }
                };
                let v225: US8 = match v202 {
                    US0::US0_ArchiveFile(v215) => {
                        let v215: Rc<str> = v215;
                        let v216: Rc<str> = string_concat(&[&*v215, "/state/spiral.toml"]);
                        let v217: bool = std::path::Path::new(&*v216).is_file();
                        if v217 {
                            US8::US8_SymbolsRoot
                        } else {
                            US8::US8_WordsRoot
                        }
                    }
                    US0::US0_HistoryFile(v209) => {
                        let v209: Rc<str> = v209;
                        let v210: Rc<str> = string_concat(&[&*v209, "/state/spiral.toml"]);
                        let v211: bool = std::path::Path::new(&*v210).is_file();
                        if v211 {
                            US8::US8_SymbolsRoot
                        } else {
                            US8::US8_WordsRoot
                        }
                    }
                    US0::US0_OtherFile => {
                        US8::US8_WordsRoot
                    }
                    US0::US0_StateFile(v203) => {
                        let v203: Rc<str> = v203;
                        let v204: Rc<str> = string_concat(&[&*v203, "/state/spiral.toml"]);
                        let v205: bool = std::path::Path::new(&*v204).is_file();
                        if v205 {
                            US8::US8_SymbolsRoot
                        } else {
                            US8::US8_WordsRoot
                        }
                    }
                };
                let v237: Rc<str> = match v225 {
                    US8::US8_SymbolsRoot => {
                        let v226: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v227: Rc<UH0> = split_list_7(v226.clone(), v168);
                        let v228: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v229: Rc<UH0> = method17(v227, v228);
                        let v230: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v231: i32 = 0i32;
                        let v232: i32 = method11(v230.clone(), v229, v231);
                        let v233: Rc<Vec<Rc<str>>> = Rc::new(v230.borrow().clone());
                        let v234: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v233).as_ref().clone()));
                        let v235: Rc<str> = Rc::<str>::from((*v234.borrow()).join(&*v226));
                        v235
                    }
                    US8::US8_WordsRoot => {
                        v168
                    }
                };
                let v238: bool = std::fs::write(&*v8, &*v237).is_ok();
                if v238 {
                    ()
                } else {
                    let v239: Rc<str> = string_concat(&["rig state write failed: ", &*v8]);
                    std::panic::panic_any::<std::string::String>(format!("{}", v239))
                };
                let v244: Rc<str> = string_concat(&["rig agile sprint ", &*v65, " ok id=", &*v1, " item=", &*v2]);
                println!("{}", v244);
                0i32
            }
        }
    }
}
fn changed_0(mut v0: bool, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    sprint_changed_1(v3, v2, v1, v0)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool| -> i32 {
        changed_0(v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method21(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn finished_from_20(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v2: i32, mut v3: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        let mut v4: i32 = (v1.clone().borrow().len() as i32);
        let v5: bool = v2 >= v4;
        if v5 {
            return v3;
        } else {
            let (mut v6, mut v7, mut v8): (Rc<str>, i32, bool) = v1.borrow()[v2 as usize].clone();
            let mut v9: Rc<str> = agile_task_census::rig_agile_task_status_of(&v0, &v6);
            let v10: bool = &*v9 == "Done";
            let mut v11: i32 = v2.wrapping_add(1i32);
            let v26: Rc<RefCell<Vec<Rc<str>>>> = if v10 {
                let mut v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v6]));
                let mut v13: i32 = (v3.clone().borrow().len() as i32);
                let v14: i32 = (v12.clone().borrow().len() as i32);
                let mut v15: i32 = v13.wrapping_add(v14);
                let mut v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v15 as usize]));
                let mut v17: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                while method21(v15, &v17) {
                    let mut v19: i32 = v17.borrow().l0;
                    let v20: bool = v19 < v13;
                    let v24: Rc<str> = if v20 {
                        let v21: Rc<str> = v3.borrow()[v19 as usize].clone();
                        v21
                    } else {
                        let v22: i32 = v19.wrapping_sub(v13);
                        let v23: Rc<str> = v12.borrow()[v22 as usize].clone();
                        v23
                    };
                    v16.borrow_mut()[v19 as usize] = v24;
                    let v25: i32 = v19.wrapping_add(1i32);
                    v17.borrow_mut().l0 = v25;
                    ()
                };
                v16
            } else {
                v3
            };
            (v2, v3) = (v11, v26);
            continue;
        }
    }
}
fn method23(mut v0: &Rc<RefCell<Mut2>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_22(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method23(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn carried_from_24(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: i32 = (v1.clone().borrow().len() as i32);
        let v5: bool = v2 >= v4;
        if v5 {
            let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method8(v3, v6);
        } else {
            let (mut v8, mut v9, mut v10): (Rc<str>, i32, bool) = v1.borrow()[v2 as usize].clone();
            let mut v11: Rc<str> = agile_task_census::rig_agile_task_status_of(&v0, &v8);
            let mut v12: bool = &*v11 == "Done";
            let mut v13: i32 = v2.wrapping_add(1i32);
            let mut v15: bool = if v10 {
                let v14: bool = v12 == false;
                v14
            } else {
                false
            };
            let mut v17: Rc<UH0> = if v15 {
                Rc::new(UH0::UH0_Cons(v8, v3))
            } else {
                v3
            };
            (v2, v3) = (v13, v17);
            continue;
        }
    }
}
fn method27(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                drop(v0.borrow_mut().arg(&*v2));
                v1 = v3;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH0::UH0_Cons(v5, v6) => {
                let v5: Rc<str> = v5;
                let v6: Rc<UH0> = v6;
                let v8: Rc<str> = string_concat(&["sprint ", &*v1, " closed with this item not Done; it carries over to the next sprint"]);
                let mut v9: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v2)));
                let v10: bool = &*v0 == "";
                let v22: Rc<UH0> = if v10 {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
                    let mut v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v13));
                    Rc::new(UH0::UH0_Cons(v11, v14))
                } else {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
                    let mut v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17, v18));
                    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v19));
                    Rc::new(UH0::UH0_Cons(v0.clone(), v20))
                };
                let v23: bool = &*v5 == "";
                let v25: Rc<UH0> = if v23 {
                    v22
                } else {
                    Rc::new(UH0::UH0_Cons(v5, v22))
                };
                let v26: bool = &*v8 == "";
                let v28: Rc<UH0> = if v26 {
                    v25
                } else {
                    Rc::new(UH0::UH0_Cons(v8, v25))
                };
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-sprint"); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v30, v31));
                let v33: Rc<UH0> = Rc::new(UH0::UH0_Cons(v29, v32));
                let v34: Rc<UH0> = method8(v28, v33);
                method27(v9.clone(), v34);
                let mut v35: Rc<std::io::Result<std::process::Output>> = Rc::new(v9.borrow_mut().output());
                let v36: bool = v35.is_ok();
                let (mut v42, mut v43, mut v44): (bool, i32, Rc<str>) = if v36 {
                    let v37: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v35).as_ref().unwrap().stderr).as_ref());
                    let v38: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v35).as_ref().unwrap().stdout).as_ref());
                    let v39: Rc<str> = string_concat(&[&*v38, &*v37]);
                    let v40: i32 = (&*v35).as_ref().unwrap().status.code().unwrap_or(-2);
                    (true, v40, v39)
                } else {
                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (false, -1i32, v41)
                };
                let v46: bool = if v42 {
                    let v45: bool = v43 == 0i32;
                    v45
                } else {
                    false
                };
                let v48: i32 = if v46 {
                    let v47: i32 = v4.wrapping_add(1i32);
                    v47
                } else {
                    v4
                };
                (v3, v4) = (v6, v48);
                continue;
            }
            UH0::UH0_Nil => {
                return v4;
            }
        }
    }
}
fn method29(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure3() -> Rc<dyn Fn((Rc<str>)) -> US9> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US9> = Rc::new(move |mut v0: (Rc<str>)| -> US9 {
        let mut v1: Rc<str> = (v0);
        US9::US9_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn current_executable_28() -> Rc<str> {
    let mut v0: Option<std::path::PathBuf> = std::env::current_exe().ok();
    let mut v2: bool = true; let _optionm_map_ = v0.map(|x| { //;
    let mut v4: std::path::PathBuf = x;
    let v5: Rc<str> = Rc::<str>::from(v4.to_string_lossy().as_ref());
    let v7: bool = true; v5 });
    let mut v9: Option<Rc<str>> = _optionm_map_;
    let v10: Option<Rc<str>> = method29(v9);
    let mut v11: Rc<dyn Fn((Rc<str>)) -> US9> = closure3();
    let mut v12: Option<US9> = v10.map(|x| v11(x));
    let v14: US9 = US9::US9_None;
    let mut v15: US9 = v14.clone();
    let mut v16: US9 = v12.unwrap_or(v15);
    match v16 {
        US9::US9_None => {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v18
        }
        US9::US9_Some(v17) => {
            let mut v17: Rc<str> = v17;
            v17
        }
    }
}
fn carried_noted_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> i32 {
    let v3: Rc<str> = current_executable_28();
    let mut v4: i32 = 0i32;
    method26(v0, v1, v3, v2, v4)
}
fn sprint_closed_19(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = string_concat(&[&*v4, "/state"]);
    let v6: Rc<str> = string_concat(&[&*v5, "/sprints.spi"]);
    let v7: bool = std::path::Path::new(&*v6).is_file();
    let v59: Rc<str> = if v7 {
        let (mut v8, mut v9): (bool, Rc<str>) = match std::fs::read(&*v6) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v8 {
            let v10: Rc<str> = string_concat(&["rig state read failed: ", &*v6]);
            std::panic::panic_any::<std::string::String>(format!("{}", v10))
        } else {
            let mut v12: Rc<str> = Rc::<str>::from(v6.replace(&*v2, &*v3));
            let mut v13: Rc<str> = loop_2(v12);
            let v14: Option<usize> = v13.rfind(&*v3);
            let mut v15: Option<usize> = method3(v14);
            let mut v16: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v17: Option<i32> = v15.map(|x| v16(x));
            let mut v19: i32 = v17.unwrap_or(-1i32);
            let v20: bool = v19 < 0i32;
            let v24: bool = if v20 {
                true
            } else {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v22: bool = v13.ends_with(&*v21);
                let v23: bool = v22 == false;
                v23
            };
            let v45: US0 = if v24 {
                US0::US0_OtherFile
            } else {
                let v26: usize = ((0i32) as usize);
                let v27: usize = ((v19) as usize);
                let v28: Rc<str> = Rc::<str>::from(&(v13)[v26..v27]);
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v30: bool = v28.ends_with(&*v29);
                if v30 {
                    let v31: Rc<str> = loop_4(v28);
                    US0::US0_StateFile(v31)
                } else {
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v34: bool = v28.ends_with(&*v33);
                    if v34 {
                        let mut v35: Rc<str> = loop_5(v28);
                        US0::US0_HistoryFile(v35)
                    } else {
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v38: bool = v28.ends_with(&*v37);
                        if v38 {
                            let v39: Rc<str> = loop_6(v28);
                            US0::US0_ArchiveFile(v39)
                        } else {
                            US0::US0_OtherFile
                        }
                    }
                }
            };
            match v45 {
                US0::US0_OtherFile => {
                    v9
                }
                _ => {
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v47: Rc<UH0> = split_list_7(v46.clone(), v9);
                    let v48: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v49: Rc<UH0> = method10(v47, v48);
                    let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v51: i32 = 0i32;
                    let v52: i32 = method11(v50.clone(), v49, v51);
                    let v53: Rc<Vec<Rc<str>>> = Rc::new(v50.borrow().clone());
                    let v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v53).as_ref().clone()));
                    let v55: Rc<str> = Rc::<str>::from((*v54.borrow()).join(&*v46));
                    v55
                }
            }
        }
    } else {
        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v58
    };
    let v60: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>> = rig_agile_sprint_state::rig_agile_sprint_head(&v59, &v1);
    let v61: i32 = (v60.clone().borrow().len() as i32);
    let v62: bool = v61 == 0i32;
    let v69: bool = if v62 {
        true
    } else {
        let (mut v63, mut v64, mut v65, mut v66, mut v67): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) = v60.borrow()[0i32 as usize].clone();
        let v68: bool = v67 == false;
        v68
    };
    if v69 {
        let v70: Rc<str> = string_concat(&["rig error: agile sprint close refused: no open sprint ", &*v1]);
        eprintln!("{}", v70);
        1i32
    } else {
        let v71: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> = rig_agile_sprint_state::rig_agile_sprint_items(&v59, &v1);
        let v72: Rc<str> = string_concat(&[&*v5, "/agile.spi"]);
        let v73: bool = std::path::Path::new(&*v72).is_file();
        let v125: Rc<str> = if v73 {
            let (mut v74, mut v75): (bool, Rc<str>) = match std::fs::read(&*v72) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v74 {
                let v76: Rc<str> = string_concat(&["rig state read failed: ", &*v72]);
                std::panic::panic_any::<std::string::String>(format!("{}", v76))
            } else {
                let v78: Rc<str> = Rc::<str>::from(v72.replace(&*v2, &*v3));
                let mut v79: Rc<str> = loop_2(v78);
                let v80: Option<usize> = v79.rfind(&*v3);
                let v81: Option<usize> = method3(v80);
                let mut v82: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v83: Option<i32> = v81.map(|x| v82(x));
                let v85: i32 = v83.unwrap_or(-1i32);
                let v86: bool = v85 < 0i32;
                let v90: bool = if v86 {
                    true
                } else {
                    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v88: bool = v79.ends_with(&*v87);
                    let v89: bool = v88 == false;
                    v89
                };
                let v111: US0 = if v90 {
                    US0::US0_OtherFile
                } else {
                    let v92: usize = ((0i32) as usize);
                    let mut v93: usize = ((v85) as usize);
                    let v94: Rc<str> = Rc::<str>::from(&(v79)[v92..v93]);
                    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let mut v96: bool = v94.ends_with(&*v95);
                    if v96 {
                        let mut v97: Rc<str> = loop_4(v94);
                        US0::US0_StateFile(v97)
                    } else {
                        let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v100: bool = v94.ends_with(&*v99);
                        if v100 {
                            let v101: Rc<str> = loop_5(v94);
                            US0::US0_HistoryFile(v101)
                        } else {
                            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v104: bool = v94.ends_with(&*v103);
                            if v104 {
                                let v105: Rc<str> = loop_6(v94);
                                US0::US0_ArchiveFile(v105)
                            } else {
                                US0::US0_OtherFile
                            }
                        }
                    }
                };
                match v111 {
                    US0::US0_OtherFile => {
                        v75
                    }
                    _ => {
                        let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v113: Rc<UH0> = split_list_7(v112.clone(), v75);
                        let v114: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v115: Rc<UH0> = method10(v113, v114);
                        let v116: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v117: i32 = 0i32;
                        let v118: i32 = method11(v116.clone(), v115, v117);
                        let v119: Rc<Vec<Rc<str>>> = Rc::new(v116.borrow().clone());
                        let v120: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v119).as_ref().clone()));
                        let v121: Rc<str> = Rc::<str>::from((*v120.borrow()).join(&*v112));
                        v121
                    }
                }
            }
        } else {
            let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v124
        };
        let v126: i32 = 0i32;
        let v127: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
        let v128: Rc<RefCell<Vec<Rc<str>>>> = finished_from_20(v125.clone(), &v71, v126, v127);
        let v129: Rc<RefCell<Vec<(i32, i32, i32)>>> = rig_agile_sprint_review::rig_agile_sprint_review(&v59, &v1, v128.clone());
        let v130: Rc<str> = rig_agile_sprint_review::rig_agile_sprint_closed(&v59, &v1, v128);
        let v131: Rc<str> = Rc::<str>::from(v6.replace(&*v2, &*v3));
        let mut v132: Rc<str> = loop_2(v131);
        let v133: Option<usize> = v132.rfind(&*v3);
        let v134: Option<usize> = method3(v133);
        let mut v135: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v136: Option<i32> = v134.map(|x| v135(x));
        let v138: i32 = v136.unwrap_or(-1i32);
        let v139: bool = v138 < 0i32;
        let v143: bool = if v139 {
            true
        } else {
            let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v141: bool = v132.ends_with(&*v140);
            let v142: bool = v141 == false;
            v142
        };
        let v164: US0 = if v143 {
            US0::US0_OtherFile
        } else {
            let v145: usize = ((0i32) as usize);
            let v146: usize = ((v138) as usize);
            let v147: Rc<str> = Rc::<str>::from(&(v132)[v145..v146]);
            let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v149: bool = v147.ends_with(&*v148);
            if v149 {
                let v150: Rc<str> = loop_4(v147);
                US0::US0_StateFile(v150)
            } else {
                let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v153: bool = v147.ends_with(&*v152);
                if v153 {
                    let v154: Rc<str> = loop_5(v147);
                    US0::US0_HistoryFile(v154)
                } else {
                    let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v157: bool = v147.ends_with(&*v156);
                    if v157 {
                        let v158: Rc<str> = loop_6(v147);
                        US0::US0_ArchiveFile(v158)
                    } else {
                        US0::US0_OtherFile
                    }
                }
            }
        };
        let v187: US8 = match v164 {
            US0::US0_ArchiveFile(v177) => {
                let v177: Rc<str> = v177;
                let v178: Rc<str> = string_concat(&[&*v177, "/state/spiral.toml"]);
                let v179: bool = std::path::Path::new(&*v178).is_file();
                if v179 {
                    US8::US8_SymbolsRoot
                } else {
                    US8::US8_WordsRoot
                }
            }
            US0::US0_HistoryFile(v171) => {
                let v171: Rc<str> = v171;
                let v172: Rc<str> = string_concat(&[&*v171, "/state/spiral.toml"]);
                let mut v173: bool = std::path::Path::new(&*v172).is_file();
                if v173 {
                    US8::US8_SymbolsRoot
                } else {
                    US8::US8_WordsRoot
                }
            }
            US0::US0_OtherFile => {
                US8::US8_WordsRoot
            }
            US0::US0_StateFile(v165) => {
                let v165: Rc<str> = v165;
                let v166: Rc<str> = string_concat(&[&*v165, "/state/spiral.toml"]);
                let v167: bool = std::path::Path::new(&*v166).is_file();
                if v167 {
                    US8::US8_SymbolsRoot
                } else {
                    US8::US8_WordsRoot
                }
            }
        };
        let v199: Rc<str> = match v187 {
            US8::US8_SymbolsRoot => {
                let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v189: Rc<UH0> = split_list_7(v188.clone(), v130);
                let v190: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v191: Rc<UH0> = method17(v189, v190);
                let v192: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v193: i32 = 0i32;
                let v194: i32 = method11(v192.clone(), v191, v193);
                let v195: Rc<Vec<Rc<str>>> = Rc::new(v192.borrow().clone());
                let v196: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v195).as_ref().clone()));
                let v197: Rc<str> = Rc::<str>::from((*v196.borrow()).join(&*v188));
                v197
            }
            US8::US8_WordsRoot => {
                v130
            }
        };
        let v200: bool = std::fs::write(&*v6, &*v199).is_ok();
        if v200 {
            ()
        } else {
            let v201: Rc<str> = string_concat(&["rig state write failed: ", &*v6]);
            std::panic::panic_any::<std::string::String>(format!("{}", v201))
        };
        let (mut v202, mut v203, mut v204): (i32, i32, i32) = v129.borrow()[0i32 as usize].clone();
        let v205: Rc<str> = format_real_22(v202);
        let v208: Rc<str> = format_real_22(v203);
        let v211: Rc<str> = format_real_22(v204);
        let v213: Rc<str> = string_concat(&[" done=", &*v205, "m carried=", &*v208, "m dropped=", &*v211, "m"]);
        let v214: bool = v203 == 0i32;
        let v217: Rc<str> = if v214 {
            let v215: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
            v215
        } else {
            let v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no"); } LIT.with(|lit| lit.clone()) };
            v216
        };
        let v218: i32 = 0i32;
        let v219: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v220: Rc<UH0> = carried_from_24(v125, &v71, v218, v219);
        let v221: i32 = carried_noted_25(v0, v1.clone(), v220);
        let v222: Rc<str> = format_real_22(v221);
        let v228: Rc<str> = string_concat(&["rig agile sprint close ok id=", &*v1, &*v213, " goal_met=", &*v217, " carried_notes=", &*v222]);
        println!("{}", v228);
        0i32
    }
}
fn closed_18(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    sprint_closed_19(v1, v0)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        closed_18(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn sprint_velocity_31(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = string_concat(&[&*v4, "/state/sprints.spi"]);
    let v6: bool = std::path::Path::new(&*v5).is_file();
    let v58: Rc<str> = if v6 {
        let (mut v7, mut v8): (bool, Rc<str>) = match std::fs::read(&*v5) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v7 {
            let mut v9: Rc<str> = string_concat(&["rig state read failed: ", &*v5]);
            std::panic::panic_any::<std::string::String>(format!("{}", v9))
        } else {
            let mut v11: Rc<str> = Rc::<str>::from(v5.replace(&*v2, &*v3));
            let mut v12: Rc<str> = loop_2(v11);
            let mut v13: Option<usize> = v12.rfind(&*v3);
            let v14: Option<usize> = method3(v13);
            let mut v15: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v16: Option<i32> = v14.map(|x| v15(x));
            let mut v18: i32 = v16.unwrap_or(-1i32);
            let mut v19: bool = v18 < 0i32;
            let v23: bool = if v19 {
                true
            } else {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v21: bool = v12.ends_with(&*v20);
                let v22: bool = v21 == false;
                v22
            };
            let v44: US0 = if v23 {
                US0::US0_OtherFile
            } else {
                let v25: usize = ((0i32) as usize);
                let v26: usize = ((v18) as usize);
                let v27: Rc<str> = Rc::<str>::from(&(v12)[v25..v26]);
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v29: bool = v27.ends_with(&*v28);
                if v29 {
                    let v30: Rc<str> = loop_4(v27);
                    US0::US0_StateFile(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v27.ends_with(&*v32);
                    if v33 {
                        let v34: Rc<str> = loop_5(v27);
                        US0::US0_HistoryFile(v34)
                    } else {
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v37: bool = v27.ends_with(&*v36);
                        if v37 {
                            let v38: Rc<str> = loop_6(v27);
                            US0::US0_ArchiveFile(v38)
                        } else {
                            US0::US0_OtherFile
                        }
                    }
                }
            };
            match v44 {
                US0::US0_OtherFile => {
                    v8
                }
                _ => {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v46: Rc<UH0> = split_list_7(v45.clone(), v8);
                    let v47: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v48: Rc<UH0> = method10(v46, v47);
                    let v49: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v50: i32 = 0i32;
                    let v51: i32 = method11(v49.clone(), v48, v50);
                    let v52: Rc<Vec<Rc<str>>> = Rc::new(v49.borrow().clone());
                    let v53: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v52).as_ref().clone()));
                    let v54: Rc<str> = Rc::<str>::from((*v53.borrow()).join(&*v45));
                    v54
                }
            }
        }
    } else {
        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v57
    };
    let v59: i32 = rig_agile_sprint_state::rig_agile_sprint_velocity(&v58, v1);
    let v60: Rc<str> = format_real_22(v1);
    let v63: Rc<str> = format_real_22(v59);
    let v64: Rc<str> = string_concat(&["rig agile sprint velocity ok last=", &*v60, " minutes=", &*v63]);
    println!("{}", v64);
    0i32
}
fn velocity_shown_30(mut v0: i32, mut v1: Rc<str>) -> i32 {
    sprint_velocity_31(v1, v0)
}
fn closure4() -> Rc<dyn Fn(Rc<str>, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: i32| -> i32 {
        velocity_shown_30(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_sprint_changed(v0: &str, v1: &str, v2: &str, v3: bool) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3)
}
pub fn rig_agile_sprint_closed_run(v0: &str, v1: &str) -> i32 {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_agile_sprint_velocity_shown(v0: &str, v1: i32) -> i32 {
    closure4()(Rc::<str>::from(v0), v1)
}
use rig_agile_sprint_state::SpiralShape_2617b4e49837 as Mut1;
use agile_task_census::SpiralShape_97df63ebabc3 as Mut2;
pub type SpiralShape_652accfc6a8d = Mut0;
