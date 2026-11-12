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
    US3_Verified,
    US3_Missing,
    US3_Duplicated(i32),
    US3_Superseded,
    US3_Refused,
    US3_NotReceipt,
}
pub struct Mut1 { pub l0: Rc<str> }
fn loop_1(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
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
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_3(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(6i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_4(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
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
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
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
fn method8(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
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
        while method8(v7, &v9) {
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
        method7(v16, v17)
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method9(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let v15: US1 = if v6 {
                let v7: US2 = US2::US2_HeadType;
                US1::US1_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US2 = US2::US2_HeadType;
                    US1::US1_SymbolHead(v11)
                } else {
                    US1::US1_OtherLine
                }
            };
            let v27: US1 = match &v15 {
                US1::US1_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US2 = US2::US2_HeadInl;
                        US1::US1_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let mut v22: US2 = US2::US2_HeadInl;
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
            let mut v76: Rc<str> = match v51 {
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
fn method10(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
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
fn tallied_13(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: US3) -> (i32, i32, i32, i32, i32) {
    let (mut v37, mut v38, mut v39, mut v40, mut v41): (i32, i32, i32, i32, i32) = match &v5 {
        US3::US3_Verified => {
            let v6: i32 = v0.wrapping_add(1i32);
            (v6, v1, v2, v3, v4)
        }
        _ => {
            let (mut v32, mut v33, mut v34, mut v35, mut v36): (i32, i32, i32, i32, i32) = match &v5 {
                US3::US3_Refused => {
                    let v7: i32 = v1.wrapping_add(1i32);
                    (v0, v7, v2, v3, v4)
                }
                _ => {
                    let (mut v27, mut v28, mut v29, mut v30, mut v31): (i32, i32, i32, i32, i32) = match &v5 {
                        US3::US3_Superseded => {
                            let v8: i32 = v2.wrapping_add(1i32);
                            (v0, v1, v8, v3, v4)
                        }
                        _ => {
                            let (mut v22, mut v23, mut v24, mut v25, mut v26): (i32, i32, i32, i32, i32) = match &v5 {
                                US3::US3_Missing => {
                                    let mut v9: i32 = v3.wrapping_add(1i32);
                                    (v0, v1, v2, v9, v4)
                                }
                                _ => {
                                    let (mut v17, mut v18, mut v19, mut v20, mut v21): (i32, i32, i32, i32, i32) = match &v5 {
                                        US3::US3_Duplicated(v10) => {
                                            let mut v10: i32 = *v10;
                                            let v11: i32 = v4.wrapping_add(1i32);
                                            (v0, v1, v2, v3, v11)
                                        }
                                        _ => {
                                            let (mut v12, mut v13, mut v14, mut v15, mut v16): (i32, i32, i32, i32, i32) = match v5 {
                                                US3::US3_NotReceipt => {
                                                    (v0, v1, v2, v3, v4)
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v12, v13, v14, v15, v16)
                                        }
                                    };
                                    (v17, v18, v19, v20, v21)
                                }
                            };
                            (v22, v23, v24, v25, v26)
                        }
                    };
                    (v27, v28, v29, v30, v31)
                }
            };
            (v32, v33, v34, v35, v36)
        }
    };
    (v37, v38, v39, v40, v41)
}
fn counted_15(mut v0: i32) -> US3 {
    let v1: bool = v0 == 1i32;
    if v1 {
        US3::US3_Verified
    } else {
        let v3: bool = v0 == 0i32;
        if v3 {
            US3::US3_Missing
        } else {
            US3::US3_Duplicated(v0)
        }
    }
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn word_of_17(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method2(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        v0
    } else {
        let mut v9: usize = ((0i32) as usize);
        let mut v10: usize = ((v7) as usize);
        let v11: Rc<str> = Rc::<str>::from(&(v0)[v9..v10]);
        v11
    }
}
fn after_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Option<usize> = v1.find(&*v0);
    let v3: Option<usize> = method2(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    } else {
        let mut v10: i32 = (v0.len() as i32);
        let v11: i32 = v7.wrapping_add(v10);
        let v12: i32 = (v1.clone().len() as i32);
        let v13: i32 = v12.wrapping_sub(v11);
        let v14: usize = ((v11) as usize);
        let v15: i32 = v11.wrapping_add(v13);
        let v16: usize = ((v15) as usize);
        let v17: Rc<str> = Rc::<str>::from(&(v1)[v14..v16]);
        v17
    }
}
fn verb_of_16(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig agile "); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    if v2 {
        let v3: Rc<str> = after_18(v1, v0);
        word_of_17(v3)
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    }
}
fn field_of_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = string_concat(&[" ", &*v0, "="]);
    let mut v4: Rc<str> = after_18(v3, v1);
    word_of_17(v4)
}
fn method21(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn occurrences_20(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<UH0> = split_list_6(v0, v1);
    let v3: i32 = 0i32;
    let mut v4: i32 = method21(v2, v3);
    let mut v5: i32 = v4.wrapping_sub(1i32);
    v5
}
fn task_key_22(mut v0: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = string_concat(&["() = Task (\"", &*v0, "\","]);
    v2
}
fn current_c3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v2: Rc<str>| -> bool {
        let v3: Rc<str> = task_key_22(v0.clone());
        let mut v4: bool = v2.contains(&*v3);
        if v4 {
            let v6: Rc<str> = string_concat(&[", ", &*v1, ","]);
            let v7: bool = v2.contains(&*v6);
            v7
        } else {
            false
        }
    })
}
fn method25(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    current_c3(v0, v1)
}
fn method24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<dyn Fn(Rc<str>) -> bool>, mut v3: Rc<UH0>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let v6: bool = v2(v4);
                if v6 {
                    return true;
                } else {
                    let v7: Rc<dyn Fn(Rc<str>) -> bool> = method25(v0.clone(), v1.clone());
                    (v2, v3) = (v7, v5);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn status_current_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<UH0> = split_list_6(v3, v0);
    let mut v5: Rc<dyn Fn(Rc<str>) -> bool> = method25(v1.clone(), v2.clone());
    method24(v1, v2, v5, v4)
}
fn verdict_of_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US3 {
    let mut v4: Rc<str> = verb_of_16(v3.clone());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = field_of_19(v5, v3.clone());
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("child"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = field_of_19(v7, v3.clone());
    let mut v9: bool = &*v4 != "";
    let v13: bool = if v9 {
        let v11: Rc<str> = string_concat(&["rig agile ", &*v4, " ok "]);
        let v12: bool = v3.contains(&*v11);
        v12
    } else {
        false
    };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error"); } LIT.with(|lit| lit.clone()) };
    let v15: bool = v3.starts_with(&*v14);
    if v15 {
        US3::US3_Refused
    } else {
        let v17: bool = v13 == false;
        if v17 {
            US3::US3_NotReceipt
        } else {
            let v19: bool = &*v4 == "note";
            let v21: bool = if v19 {
                true
            } else {
                let v20: bool = &*v4 == "note-edit";
                v20
            };
            if v21 {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
                let mut v25: Rc<str> = field_of_19(v24, v3);
                let v27: Rc<str> = string_concat(&["Note (\"", &*v6, "\", ", &*v25, "u32,"]);
                let v28: i32 = occurrences_20(v27, v0);
                counted_15(v28)
            } else {
                let v30: bool = &*v4 == "add";
                if v30 {
                    let v31: Rc<str> = task_key_22(v6);
                    let v32: i32 = occurrences_20(v31, v0);
                    counted_15(v32)
                } else {
                    let v34: bool = &*v4 == "set";
                    if v34 {
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
                        let v36: Rc<str> = field_of_19(v35, v3);
                        let v37: bool = status_current_23(v0, v6, v36);
                        if v37 {
                            US3::US3_Verified
                        } else {
                            US3::US3_Superseded
                        }
                    } else {
                        let v41: bool = &*v4 == "edit";
                        let v43: bool = if v41 {
                            let v42: bool = &*v8 != "";
                            v42
                        } else {
                            false
                        };
                        if v43 {
                            let v45: Rc<str> = string_concat(&["Parent (\"", &*v8, "\","]);
                            let v46: i32 = occurrences_20(v45, v2);
                            counted_15(v46)
                        } else {
                            let v50: bool = if v41 {
                                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waits="); } LIT.with(|lit| lit.clone()) };
                                let v49: bool = v3.contains(&*v48);
                                v49
                            } else {
                                false
                            };
                            if v50 {
                                let v52: Rc<str> = string_concat(&["Waits (\"", &*v6, "\","]);
                                let v53: i32 = occurrences_20(v52, v1);
                                let v54: bool = v53 > 0i32;
                                if v54 {
                                    US3::US3_Verified
                                } else {
                                    US3::US3_Missing
                                }
                            } else {
                                if v41 {
                                    let v58: Rc<str> = task_key_22(v6);
                                    let v59: i32 = occurrences_20(v58, v0);
                                    counted_15(v59)
                                } else {
                                    US3::US3_NotReceipt
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method27(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_26(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method27(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn reported_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: Rc<str>) -> (i32, i32, i32, i32, i32) {
    let mut v9: US3 = verdict_of_14(v0, v1, v2, v8.clone());
    match &v9 {
        US3::US3_Duplicated(v11) => {
            let v11: i32 = *v11;
            let v12: Rc<str> = format_real_26(v11);
            let v15: Rc<str> = string_concat(&["duplicated x", &*v12, " ", &*v8]);
            println!("{}", v15);
            ()
        }
        US3::US3_Missing => {
            let mut v10: Rc<str> = string_concat(&["missing ", &*v8]);
            println!("{}", v10);
            ()
        }
        _ => {
            ()
        }
    };
    let (mut v16, mut v17, mut v18, mut v19, mut v20): (i32, i32, i32, i32, i32) = tallied_13(v3, v4, v5, v6, v7, v9);
    (v16, v17, v18, v19, v20)
}
fn method11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32) -> (i32, i32, i32, i32, i32) {
    loop {
        match &*v3 {
            UH0::UH0_Cons(v9, v10) => {
                let mut v9: Rc<str> = v9.clone();
                let mut v10: Rc<UH0> = v10.clone();
                let (mut v11, mut v12, mut v13, mut v14, mut v15): (i32, i32, i32, i32, i32) = reported_12(v0.clone(), v1.clone(), v2.clone(), v4, v5, v6, v7, v8, v9);
                (v3, v4, v5, v6, v7, v8) = (v10, v11, v12, v13, v14, v15);
                continue;
            }
            _ => {
                let (mut v21, mut v22, mut v23, mut v24, mut v25): (i32, i32, i32, i32, i32) = match Rc::unwrap_or_clone(v3) {
                    UH0::UH0_Nil => {
                        (v4, v5, v6, v7, v8)
                    }
                    _ => unreachable!(),
                };
                return (v21, v22, v23, v24, v25);
            }
        }
    }
}
fn summary_of_28(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> Rc<str> {
    let mut v5: Rc<str> = format_real_26(v0);
    let v6: Rc<str> = format_real_26(v1);
    let v7: Rc<str> = format_real_26(v2);
    let v8: Rc<str> = format_real_26(v3);
    let mut v9: Rc<str> = format_real_26(v4);
    let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig agile receipts verified="); } LIT.with(|lit| lit.clone()) };
    v10.borrow_mut().push(v11);
    v10.borrow_mut().push(v5);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" refused="); } LIT.with(|lit| lit.clone()) };
    v10.borrow_mut().push(v12);
    v10.borrow_mut().push(v6);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" superseded="); } LIT.with(|lit| lit.clone()) };
    v10.borrow_mut().push(v13);
    v10.borrow_mut().push(v7);
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" missing="); } LIT.with(|lit| lit.clone()) };
    v10.borrow_mut().push(v14);
    v10.borrow_mut().push(v8);
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" duplicated="); } LIT.with(|lit| lit.clone()) };
    v10.borrow_mut().push(v15);
    v10.borrow_mut().push(v9);
    let v16: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
    let v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v16).as_ref().clone()));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from((*v17.borrow()).join(&*v18));
    v19
}
fn verify_run_0() -> i32 {
    let mut v0: i32 = 1i32;
    let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 3i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: i32 = 2i32;
    let mut v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v6: bool = &*v5 != "verify";
    let v8: bool = if v6 {
        true
    } else {
        let v7: bool = &*v3 == "";
        v7
    };
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: rig agile receipts <root> verify <file> (one receipt or rig error line per row)"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v9);
        2i32
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = Rc::<str>::from(v1.replace(&*v10, &*v11));
        let v14: Rc<str> = string_concat(&[&*v12, "/state/"]);
        let v15: Rc<str> = string_concat(&[&*v14, "agile.spi"]);
        let v16: bool = std::path::Path::new(&*v15).is_file();
        let v68: Rc<str> = if v16 {
            let (mut v17, mut v18): (bool, Rc<str>) = match std::fs::read(&*v15) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v17 {
                let v19: Rc<str> = string_concat(&["rig state read failed: ", &*v15]);
                std::panic::panic_any::<std::string::String>(format!("{}", v19))
            } else {
                let v21: Rc<str> = Rc::<str>::from(v15.replace(&*v10, &*v11));
                let mut v22: Rc<str> = loop_1(v21);
                let v23: Option<usize> = v22.rfind(&*v11);
                let v24: Option<usize> = method2(v23);
                let mut v25: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v26: Option<i32> = v24.map(|x| v25(x));
                let v28: i32 = v26.unwrap_or(-1i32);
                let v29: bool = v28 < 0i32;
                let v33: bool = if v29 {
                    true
                } else {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v31: bool = v22.ends_with(&*v30);
                    let v32: bool = v31 == false;
                    v32
                };
                let v54: US0 = if v33 {
                    US0::US0_OtherFile
                } else {
                    let v35: usize = ((0i32) as usize);
                    let v36: usize = ((v28) as usize);
                    let v37: Rc<str> = Rc::<str>::from(&(v22)[v35..v36]);
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v39: bool = v37.ends_with(&*v38);
                    if v39 {
                        let v40: Rc<str> = loop_3(v37);
                        US0::US0_StateFile(v40)
                    } else {
                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v43: bool = v37.ends_with(&*v42);
                        if v43 {
                            let v44: Rc<str> = loop_4(v37);
                            US0::US0_HistoryFile(v44)
                        } else {
                            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v47: bool = v37.ends_with(&*v46);
                            if v47 {
                                let v48: Rc<str> = loop_5(v37);
                                US0::US0_ArchiveFile(v48)
                            } else {
                                US0::US0_OtherFile
                            }
                        }
                    }
                };
                match v54 {
                    US0::US0_OtherFile => {
                        v18
                    }
                    _ => {
                        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v56: Rc<UH0> = split_list_6(v55.clone(), v18);
                        let v57: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v58: Rc<UH0> = method9(v56, v57);
                        let v59: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v60: i32 = 0i32;
                        let v61: i32 = method10(v59.clone(), v58, v60);
                        let v62: Rc<Vec<Rc<str>>> = Rc::new(v59.borrow().clone());
                        let v63: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v62).as_ref().clone()));
                        let v64: Rc<str> = Rc::<str>::from((*v63.borrow()).join(&*v55));
                        v64
                    }
                }
            }
        } else {
            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v67
        };
        let v69: Rc<str> = string_concat(&[&*v14, "waits.spi"]);
        let v70: bool = std::path::Path::new(&*v69).is_file();
        let v122: Rc<str> = if v70 {
            let (mut v71, mut v72): (bool, Rc<str>) = match std::fs::read(&*v69) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v71 {
                let v73: Rc<str> = string_concat(&["rig state read failed: ", &*v69]);
                std::panic::panic_any::<std::string::String>(format!("{}", v73))
            } else {
                let v75: Rc<str> = Rc::<str>::from(v69.replace(&*v10, &*v11));
                let mut v76: Rc<str> = loop_1(v75);
                let v77: Option<usize> = v76.rfind(&*v11);
                let v78: Option<usize> = method2(v77);
                let mut v79: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v80: Option<i32> = v78.map(|x| v79(x));
                let v82: i32 = v80.unwrap_or(-1i32);
                let v83: bool = v82 < 0i32;
                let v87: bool = if v83 {
                    true
                } else {
                    let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v85: bool = v76.ends_with(&*v84);
                    let v86: bool = v85 == false;
                    v86
                };
                let v108: US0 = if v87 {
                    US0::US0_OtherFile
                } else {
                    let v89: usize = ((0i32) as usize);
                    let v90: usize = ((v82) as usize);
                    let v91: Rc<str> = Rc::<str>::from(&(v76)[v89..v90]);
                    let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v93: bool = v91.ends_with(&*v92);
                    if v93 {
                        let v94: Rc<str> = loop_3(v91);
                        US0::US0_StateFile(v94)
                    } else {
                        let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v97: bool = v91.ends_with(&*v96);
                        if v97 {
                            let v98: Rc<str> = loop_4(v91);
                            US0::US0_HistoryFile(v98)
                        } else {
                            let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v101: bool = v91.ends_with(&*v100);
                            if v101 {
                                let v102: Rc<str> = loop_5(v91);
                                US0::US0_ArchiveFile(v102)
                            } else {
                                US0::US0_OtherFile
                            }
                        }
                    }
                };
                match v108 {
                    US0::US0_OtherFile => {
                        v72
                    }
                    _ => {
                        let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v110: Rc<UH0> = split_list_6(v109.clone(), v72);
                        let v111: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v112: Rc<UH0> = method9(v110, v111);
                        let v113: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v114: i32 = 0i32;
                        let v115: i32 = method10(v113.clone(), v112, v114);
                        let v116: Rc<Vec<Rc<str>>> = Rc::new(v113.borrow().clone());
                        let v117: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v116).as_ref().clone()));
                        let v118: Rc<str> = Rc::<str>::from((*v117.borrow()).join(&*v109));
                        v118
                    }
                }
            }
        } else {
            let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v121
        };
        let v123: Rc<str> = string_concat(&[&*v14, "parents.spi"]);
        let v124: bool = std::path::Path::new(&*v123).is_file();
        let v176: Rc<str> = if v124 {
            let (mut v125, mut v126): (bool, Rc<str>) = match std::fs::read(&*v123) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v125 {
                let v127: Rc<str> = string_concat(&["rig state read failed: ", &*v123]);
                std::panic::panic_any::<std::string::String>(format!("{}", v127))
            } else {
                let v129: Rc<str> = Rc::<str>::from(v123.replace(&*v10, &*v11));
                let mut v130: Rc<str> = loop_1(v129);
                let v131: Option<usize> = v130.rfind(&*v11);
                let v132: Option<usize> = method2(v131);
                let mut v133: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v134: Option<i32> = v132.map(|x| v133(x));
                let v136: i32 = v134.unwrap_or(-1i32);
                let v137: bool = v136 < 0i32;
                let v141: bool = if v137 {
                    true
                } else {
                    let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v139: bool = v130.ends_with(&*v138);
                    let v140: bool = v139 == false;
                    v140
                };
                let v162: US0 = if v141 {
                    US0::US0_OtherFile
                } else {
                    let v143: usize = ((0i32) as usize);
                    let v144: usize = ((v136) as usize);
                    let v145: Rc<str> = Rc::<str>::from(&(v130)[v143..v144]);
                    let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v147: bool = v145.ends_with(&*v146);
                    if v147 {
                        let v148: Rc<str> = loop_3(v145);
                        US0::US0_StateFile(v148)
                    } else {
                        let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v151: bool = v145.ends_with(&*v150);
                        if v151 {
                            let v152: Rc<str> = loop_4(v145);
                            US0::US0_HistoryFile(v152)
                        } else {
                            let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v155: bool = v145.ends_with(&*v154);
                            if v155 {
                                let v156: Rc<str> = loop_5(v145);
                                US0::US0_ArchiveFile(v156)
                            } else {
                                US0::US0_OtherFile
                            }
                        }
                    }
                };
                match v162 {
                    US0::US0_OtherFile => {
                        v126
                    }
                    _ => {
                        let v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v164: Rc<UH0> = split_list_6(v163.clone(), v126);
                        let v165: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v166: Rc<UH0> = method9(v164, v165);
                        let v167: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v168: i32 = 0i32;
                        let v169: i32 = method10(v167.clone(), v166, v168);
                        let v170: Rc<Vec<Rc<str>>> = Rc::new(v167.borrow().clone());
                        let v171: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v170).as_ref().clone()));
                        let v172: Rc<str> = Rc::<str>::from((*v171.borrow()).join(&*v163));
                        v172
                    }
                }
            }
        } else {
            let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v175
        };
        let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v178: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v3).unwrap_or_default().as_str());
        let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v181: Rc<str> = Rc::<str>::from(v178.replace(&*v179, &*v180));
        let v182: Rc<UH0> = split_list_6(v177, v181);
        let v183: i32 = 0i32;
        let v184: i32 = 0i32;
        let v185: i32 = 0i32;
        let v186: i32 = 0i32;
        let v187: i32 = 0i32;
        let (mut v188, mut v189, mut v190, mut v191, mut v192): (i32, i32, i32, i32, i32) = method11(v68, v122, v176, v182, v183, v184, v185, v186, v187);
        let v193: Rc<str> = summary_of_28(v188, v189, v190, v191, v192);
        println!("{}", v193);
        let v194: i32 = v191.wrapping_add(v192);
        let v195: bool = v194 > 0i32;
        if v195 {
            1i32
        } else {
            0i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        verify_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_receipts_verify_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
