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
    US3_WaitsTrain(Rc<str>),
    US3_WaitsItem(Rc<str>),
    US3_WaitsLane(Rc<str>),
    US3_WaitsUser(Rc<str>),
    US3_ResumeWhen(Rc<str>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, US3, Rc<UH1>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>, US3),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_Some(US3),
    US5_None,
}
#[derive(Clone)]
enum US6 {
    US6_Some(Rc<str>),
    US6_None,
}
#[derive(Clone)]
enum US7 {
    US7_Some(i32),
    US7_None,
}
fn loop_1(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
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
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_3(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(6i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
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
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
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
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
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
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
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
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method7(v16, v17)
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method9(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let mut v15: US1 = if v6 {
                let mut v7: US2 = US2::US2_HeadType;
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
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US2 = US2::US2_HeadInl;
                        US1::US1_KeywordHead(v18)
                    } else {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
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
fn method10(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
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
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn nth_quoted_14(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    loop {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v3: Option<usize> = v0.find(&*v2);
        let mut v4: Option<usize> = method2(v3);
        let mut v5: Rc<dyn Fn((usize)) -> i32> = closure2();
        let mut v6: Option<i32> = v4.map(|x| v5(x));
        let mut v8: i32 = v6.unwrap_or(-1i32);
        let mut v9: bool = v8 < 0i32;
        let mut v19: Rc<str> = if v9 {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v10
        } else {
            let mut v11: i32 = v8.wrapping_add(1i32);
            let mut v12: i32 = (v0.clone().len() as i32);
            let v13: i32 = v12.wrapping_sub(v8);
            let mut v14: i32 = v13.wrapping_sub(1i32);
            let mut v15: usize = ((v11) as usize);
            let mut v16: i32 = v11.wrapping_add(v14);
            let v17: usize = ((v16) as usize);
            let v18: Rc<str> = Rc::<str>::from(&(v0)[v15..v17]);
            v18
        };
        let mut v20: Option<usize> = v19.find(&*v2);
        let v21: Option<usize> = method2(v20);
        let mut v22: Option<i32> = v21.map(|x| v5(x));
        let v24: i32 = v22.unwrap_or(-1i32);
        let v26: bool = if v9 {
            true
        } else {
            let v25: bool = v24 < 0i32;
            v25
        };
        if v26 {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v27;
        } else {
            let v28: bool = v1 == 0i32;
            if v28 {
                let v29: usize = ((0i32) as usize);
                let v30: usize = ((v24) as usize);
                let v31: Rc<str> = Rc::<str>::from(&(v19)[v29..v30]);
                return v31;
            } else {
                let v32: i32 = v24.wrapping_add(1i32);
                let v33: i32 = (v19.clone().len() as i32);
                let v34: i32 = v33.wrapping_sub(v24);
                let v35: i32 = v34.wrapping_sub(1i32);
                let v36: usize = ((v32) as usize);
                let v37: i32 = v32.wrapping_add(v35);
                let v38: usize = ((v37) as usize);
                let v39: Rc<str> = Rc::<str>::from(&(v19)[v36..v38]);
                let v40: i32 = v1.wrapping_sub(1i32);
                (v0, v1) = (v39, v40);
                continue;
            }
        }
    }
}
fn kind_of_15(mut v0: Rc<str>, mut v1: Rc<str>) -> US5 {
    let v2: bool = &*v1 == "";
    if v2 {
        US5::US5_None
    } else {
        let mut v4: bool = &*v0 == "train";
        if v4 {
            let mut v5: US3 = US3::US3_WaitsTrain(v1);
            US5::US5_Some(v5)
        } else {
            let mut v7: bool = &*v0 == "item";
            if v7 {
                let mut v8: US3 = US3::US3_WaitsItem(v1);
                US5::US5_Some(v8)
            } else {
                let v10: bool = &*v0 == "lane";
                if v10 {
                    let mut v11: US3 = US3::US3_WaitsLane(v1);
                    US5::US5_Some(v11)
                } else {
                    let v13: bool = &*v0 == "user";
                    if v13 {
                        let mut v14: US3 = US3::US3_WaitsUser(v1);
                        US5::US5_Some(v14)
                    } else {
                        let mut v16: bool = &*v0 == "when";
                        if v16 {
                            let v17: US3 = US3::US3_ResumeWhen(v1);
                            US5::US5_Some(v17)
                        } else {
                            US5::US5_None
                        }
                    }
                }
            }
        }
    }
}
fn wait_of_line_13(mut v0: Rc<str>) -> US4 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Waits (\""); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method2(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let mut v8: bool = v7 < 0i32;
    if v8 {
        US4::US4_None
    } else {
        let v10: i32 = v7.wrapping_add(3i32);
        let mut v11: i32 = (v0.clone().len() as i32);
        let mut v12: i32 = v11.wrapping_sub(v7);
        let v13: i32 = v12.wrapping_sub(3i32);
        let mut v14: usize = ((v10) as usize);
        let mut v15: i32 = v10.wrapping_add(v13);
        let mut v16: usize = ((v15) as usize);
        let v17: Rc<str> = Rc::<str>::from(&(v0)[v14..v16]);
        let v18: i32 = 0i32;
        let mut v19: Rc<str> = nth_quoted_14(v17.clone(), v18);
        let mut v20: i32 = 1i32;
        let v21: Rc<str> = nth_quoted_14(v17.clone(), v20);
        let mut v22: i32 = 2i32;
        let v23: Rc<str> = nth_quoted_14(v17, v22);
        let v24: US5 = kind_of_15(v21, v23);
        match v24 {
            US5::US5_Some(v25) => {
                let v25: US3 = v25;
                let v26: bool = &*v19 != "";
                if v26 {
                    US4::US4_Some(v19, v25)
                } else {
                    US4::US4_None
                }
            }
            _ => {
                US4::US4_None
            }
        }
    }
}
fn method16(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let v2: Rc<str> = v2;
            let mut v3: US3 = v3;
            let mut v4: Rc<UH1> = v4;
            let mut v5: Rc<UH1> = method16(v4, v1);
            Rc::new(UH1::UH1_Cons(v2, v3, v5))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: US4 = wait_of_line_13(v2);
                let mut v11: Rc<UH1> = match v4 {
                    US4::US4_None => {
                        v1
                    }
                    US4::US4_Some(v5, v6) => {
                        let mut v5: Rc<str> = v5;
                        let mut v6: US3 = v6;
                        let mut v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let mut v8: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v6, v7));
                        method16(v1, v8)
                    }
                };
                (v0, v1) = (v3, v11);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn waits_of_text_11(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_6(v1, v4);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method12(v5, v6)
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= Task (\""); } LIT.with(|lit| lit.clone()) };
                let mut v5: Option<usize> = v2.find(&*v4);
                let mut v6: Option<usize> = method2(v5);
                let mut v7: Rc<dyn Fn((usize)) -> i32> = closure2();
                let mut v8: Option<i32> = v6.map(|x| v7(x));
                let v10: i32 = v8.unwrap_or(-1i32);
                let mut v11: bool = v10 < 0i32;
                let v21: Rc<str> = if v11 {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v12
                } else {
                    let v13: i32 = v10.wrapping_add(8i32);
                    let mut v14: i32 = (v2.clone().len() as i32);
                    let mut v15: i32 = v14.wrapping_sub(v10);
                    let mut v16: i32 = v15.wrapping_sub(8i32);
                    let v17: usize = ((v13) as usize);
                    let v18: i32 = v13.wrapping_add(v16);
                    let mut v19: usize = ((v18) as usize);
                    let mut v20: Rc<str> = Rc::<str>::from(&(v2)[v17..v19]);
                    v20
                };
                let mut v22: i32 = 0i32;
                let v23: Rc<str> = nth_quoted_14(v21, v22);
                let v24: bool = &*v23 != "";
                let v27: bool = if v24 {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, Done, "); } LIT.with(|lit| lit.clone()) };
                    let v26: bool = v2.contains(&*v25);
                    v26
                } else {
                    false
                };
                let v29: Rc<UH0> = if v27 {
                    Rc::new(UH0::UH0_Cons(v23, v1))
                } else {
                    v1
                };
                (v0, v1) = (v3, v29);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn done_ids_17(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_6(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method18(v5, v6)
}
fn method21(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure3() -> Rc<dyn Fn((Rc<str>)) -> US6> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US6> = Rc::new(move |mut v0: (Rc<str>)| -> US6 {
        let v1: Rc<str> = (v0);
        US6::US6_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn current_executable_20() -> Rc<str> {
    let mut v0: Option<std::path::PathBuf> = std::env::current_exe().ok();
    let v2: bool = true; let _optionm_map_ = v0.map(|x| { //;
    let mut v4: std::path::PathBuf = x;
    let mut v5: Rc<str> = Rc::<str>::from(v4.to_string_lossy().as_ref());
    let mut v7: bool = true; v5 });
    let mut v9: Option<Rc<str>> = _optionm_map_;
    let v10: Option<Rc<str>> = method21(v9);
    let mut v11: Rc<dyn Fn((Rc<str>)) -> US6> = closure3();
    let mut v12: Option<US6> = v10.map(|x| v11(x));
    let mut v14: US6 = US6::US6_None;
    let mut v15: US6 = v14.clone();
    let mut v16: US6 = v12.unwrap_or(v15);
    match v16 {
        US6::US6_None => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v18
        }
        US6::US6_Some(v17) => {
            let v17: Rc<str> = v17;
            v17
        }
    }
}
fn installed_train_19() -> Rc<str> {
    let mut v0: Rc<str> = current_executable_20();
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Option<usize> = v3.rfind(&*v2);
    let mut v5: Option<usize> = method2(v4);
    let mut v6: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v7: Option<i32> = v5.map(|x| v6(x));
    let mut v9: i32 = v7.unwrap_or(-1i32);
    let v10: bool = v9 < 0i32;
    let mut v15: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        v11
    } else {
        let mut v12: usize = ((0i32) as usize);
        let v13: usize = ((v9) as usize);
        let mut v14: Rc<str> = Rc::<str>::from(&(v3)[v12..v13]);
        v14
    };
    let mut v16: Rc<str> = string_concat(&[&*v15, "/installed-train.txt"]);
    let v17: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v16).unwrap_or_default().as_str());
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = Rc::<str>::from(v17.replace(&*v18, &*v19));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v19));
    v22
}
fn method24(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let v2: Rc<str> = v2;
            let mut v3: US3 = v3;
            let mut v4: Rc<UH1> = v4;
            let mut v5: Rc<UH0> = method24(v4, v1);
            Rc::new(UH0::UH0_Cons(v2, v5))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn waiting_ids_23(mut v0: Rc<UH1>) -> Rc<UH0> {
    let v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method24(v0, v1)
}
fn closure4(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US7> {
    Rc::new(move |mut v2: Rc<UH0>| -> US7 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US7::US7_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US7> = method27(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US7::US7_None
            }
        }
    })
}
fn method27(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US7> {
    closure4(v0, v1)
}
fn digits_from_29(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 < v3;
        let v10: Rc<str> = if v4 {
            let mut v5: usize = ((v1) as usize);
            let mut v6: i32 = v1.wrapping_add(1i32);
            let mut v7: usize = ((v6) as usize);
            let mut v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
            v8
        } else {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v9
        };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Option<usize> = v11.find(&*v10);
        let v13: Option<usize> = method2(v12);
        let mut v14: Rc<dyn Fn((usize)) -> i32> = closure2();
        let mut v15: Option<i32> = v13.map(|x| v14(x));
        let v17: i32 = v15.unwrap_or(-1i32);
        let v18: bool = &*v10 != "";
        let mut v20: bool = if v18 {
            let mut v19: bool = v17 >= 0i32;
            v19
        } else {
            false
        };
        if v20 {
            let v21: i32 = v1.wrapping_add(1i32);
            let mut v22: i32 = v2.wrapping_mul(10i32);
            let v23: i32 = v22.wrapping_add(v17);
            (v1, v2) = (v21, v23);
            continue;
        } else {
            return v2;
        }
    }
}
fn train_number_28(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TRAIN"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    let mut v5: bool = if v2 {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v3 > 5i32;
        v4
    } else {
        false
    };
    let mut v11: bool = if v5 {
        let mut v6: usize = ((5i32) as usize);
        let mut v7: usize = ((6i32) as usize);
        let mut v8: Rc<str> = Rc::<str>::from(&(v0)[v6..v7]);
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) };
        let v10: bool = v9.contains(&*v8);
        v10
    } else {
        false
    };
    if v11 {
        let mut v12: i32 = 5i32;
        let v13: i32 = 0i32;
        digits_from_29(v0, v12, v13)
    } else {
        -1i32
    }
}
fn wait_holds_26(mut v0: &Rc<str>, mut v1: US3, mut v2: Rc<UH0>, mut v3: Rc<str>) -> bool {
    match v1 {
        US3::US3_WaitsItem(v4) => {
            let mut v4: Rc<str> = v4;
            let mut v5: i32 = 0i32;
            let mut v6: Rc<dyn Fn(Rc<UH0>) -> US7> = method27(v4, v5);
            let mut v7: US7 = v6(v2);
            let mut v8: bool = match v7 {
                US7::US7_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let mut v9: bool = v8 != true;
            let v10: bool = v9 == false;
            v10
        }
        US3::US3_WaitsTrain(v11) => {
            let mut v11: Rc<str> = v11;
            let mut v12: i32 = train_number_28(v11);
            let v13: i32 = train_number_28(v3);
            let mut v14: bool = v12 < 0i32;
            if v14 {
                true
            } else {
                let mut v15: bool = v13 < v12;
                v15
            }
        }
        _ => {
            true
        }
    }
}
fn method25(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<UH1>, mut v3: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v4, v5, v6) => {
            let mut v4: Rc<str> = v4;
            let mut v5: US3 = v5;
            let mut v6: Rc<UH1> = v6;
            let mut v7: Rc<UH1> = method25(v0.clone(), v1.clone(), v6, v3);
            let mut v8: bool = wait_holds_26(&v4, v5.clone(), v0, v1);
            if v8 {
                Rc::new(UH1::UH1_Cons(v4, v5, v7))
            } else {
                v7
            }
        }
        UH1::UH1_Nil => {
            v3
        }
    }
}
fn holding_of_22(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<UH0> {
    let mut v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH1> = method25(v1, v2, v0, v3);
    waiting_ids_23(v4)
}
fn holding_ids_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v5: Rc<str> = string_concat(&[&*v4, "/state/waits.spi"]);
    let mut v6: bool = std::path::Path::new(&*v5).is_file();
    let v58: Rc<str> = if v6 {
        let (mut v7, mut v8): (bool, Rc<str>) = match std::fs::read(&*v5) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v7 {
            let mut v9: Rc<str> = string_concat(&["rig state read failed: ", &*v5]);
            std::panic::panic_any::<std::string::String>(format!("{}", v9))
        } else {
            let mut v11: Rc<str> = Rc::<str>::from(v5.replace(&*v2, &*v3));
            let mut v12: Rc<str> = loop_1(v11);
            let v13: Option<usize> = v12.rfind(&*v3);
            let mut v14: Option<usize> = method2(v13);
            let mut v15: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v16: Option<i32> = v14.map(|x| v15(x));
            let v18: i32 = v16.unwrap_or(-1i32);
            let mut v19: bool = v18 < 0i32;
            let v23: bool = if v19 {
                true
            } else {
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v21: bool = v12.ends_with(&*v20);
                let mut v22: bool = v21 == false;
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
                    let v30: Rc<str> = loop_3(v27);
                    US0::US0_StateFile(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v27.ends_with(&*v32);
                    if v33 {
                        let v34: Rc<str> = loop_4(v27);
                        US0::US0_HistoryFile(v34)
                    } else {
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v37: bool = v27.ends_with(&*v36);
                        if v37 {
                            let v38: Rc<str> = loop_5(v27);
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
                    let v46: Rc<UH0> = split_list_6(v45.clone(), v8);
                    let v47: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v48: Rc<UH0> = method9(v46, v47);
                    let v49: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v50: i32 = 0i32;
                    let v51: i32 = method10(v49.clone(), v48, v50);
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
    let v59: bool = &*v58 == "";
    let v65: Rc<UH0> = if v59 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v61: Rc<UH1> = waits_of_text_11(v58);
        let v62: Rc<UH0> = done_ids_17(v0);
        let v63: Rc<str> = installed_train_19();
        holding_of_22(v61, v62, v63)
    };
    let v66: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v67: i32 = 0i32;
    let v68: i32 = method10(v66.clone(), v65, v67);
    let v69: Rc<Vec<Rc<str>>> = Rc::new(v66.borrow().clone());
    let v70: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v69).as_ref().clone()));
    v70
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<RefCell<Vec<Rc<str>>>> {
        holding_ids_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method31(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn team_after_32(mut v0: Rc<UH0>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
                match Rc::unwrap_or_clone(v2) {
                    UH0::UH0_Cons(v3, v4) => {
                        let mut v3: Rc<str> = v3;
                        let mut v4: Rc<UH0> = v4;
                        let mut v5: bool = &*v1 == "--team";
                        if v5 {
                            return v3;
                        } else {
                            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v4));
                            v0 = v6;
                            continue;
                        }
                    }
                    _ => {
                        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        return v9;
                    }
                }
            }
            _ => {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v11;
            }
        }
    }
}
fn method34(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5) => {
            let mut v3: Rc<str> = v3;
            let mut v4: US3 = v4;
            let mut v5: Rc<UH1> = v5;
            let mut v6: Rc<UH0> = method34(v0.clone(), v5, v2);
            let v10: Rc<str> = string_concat(&["unblock  ", &*v3, "  waits on team ", &*v0, ": answer or pull it"]);
            Rc::new(UH0::UH0_Cons(v10, v6))
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn lane_name_37(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team:"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    if v2 {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: i32 = v3.wrapping_sub(5i32);
        let mut v5: usize = ((5i32) as usize);
        let mut v6: i32 = 5i32.wrapping_add(v4);
        let mut v7: usize = ((v6) as usize);
        let mut v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
        v8
    } else {
        v0
    }
}
fn routed_to_36(mut v0: Rc<str>, mut v1: &Rc<str>, mut v2: US3) -> bool {
    match v2 {
        US3::US3_WaitsLane(v3) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<str> = lane_name_37(v3);
            let mut v5: bool = &*v4 == &*v0;
            v5
        }
        _ => {
            false
        }
    }
}
fn method35(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5) => {
            let mut v3: Rc<str> = v3;
            let mut v4: US3 = v4;
            let mut v5: Rc<UH1> = v5;
            let mut v6: Rc<UH1> = method35(v0.clone(), v5, v2);
            let mut v7: bool = routed_to_36(v0, &v3, v4.clone());
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
fn method38(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5) => {
            let mut v3: Rc<str> = v3;
            let mut v4: US3 = v4;
            let mut v5: Rc<UH1> = v5;
            let mut v6: Rc<UH1> = method38(v0.clone(), v5, v2);
            let mut v7: i32 = 0i32;
            let mut v8: Rc<dyn Fn(Rc<UH0>) -> US7> = method27(v3.clone(), v7);
            let mut v9: US7 = v8(v0);
            let v10: bool = match v9 {
                US7::US7_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let mut v11: bool = v10 != true;
            let mut v12: bool = v11 == false;
            if v12 {
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
fn unblock_lines_33(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<UH0> {
    let mut v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH1> = method35(v2.clone(), v0, v3);
    let mut v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH1> = method38(v1, v4, v5);
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method34(v2, v6, v7)
}
fn method39(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
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
fn lane_waits_printed_30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<UH0> = split_list_6(v3, v1);
    let mut v5: Rc<str> = team_after_32(v4);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(v2.replace(&*v6, &*v7));
    let mut v9: Rc<str> = string_concat(&[&*v8, "/state/waits.spi"]);
    let v10: bool = std::path::Path::new(&*v9).is_file();
    let v62: Rc<str> = if v10 {
        let (mut v11, mut v12): (bool, Rc<str>) = match std::fs::read(&*v9) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v11 {
            let v13: Rc<str> = string_concat(&["rig state read failed: ", &*v9]);
            std::panic::panic_any::<std::string::String>(format!("{}", v13))
        } else {
            let mut v15: Rc<str> = Rc::<str>::from(v9.replace(&*v6, &*v7));
            let mut v16: Rc<str> = loop_1(v15);
            let v17: Option<usize> = v16.rfind(&*v7);
            let v18: Option<usize> = method2(v17);
            let mut v19: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v20: Option<i32> = v18.map(|x| v19(x));
            let mut v22: i32 = v20.unwrap_or(-1i32);
            let v23: bool = v22 < 0i32;
            let v27: bool = if v23 {
                true
            } else {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v25: bool = v16.ends_with(&*v24);
                let v26: bool = v25 == false;
                v26
            };
            let v48: US0 = if v27 {
                US0::US0_OtherFile
            } else {
                let v29: usize = ((0i32) as usize);
                let v30: usize = ((v22) as usize);
                let v31: Rc<str> = Rc::<str>::from(&(v16)[v29..v30]);
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v33: bool = v31.ends_with(&*v32);
                if v33 {
                    let v34: Rc<str> = loop_3(v31);
                    US0::US0_StateFile(v34)
                } else {
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v37: bool = v31.ends_with(&*v36);
                    if v37 {
                        let v38: Rc<str> = loop_4(v31);
                        US0::US0_HistoryFile(v38)
                    } else {
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v41: bool = v31.ends_with(&*v40);
                        if v41 {
                            let v42: Rc<str> = loop_5(v31);
                            US0::US0_ArchiveFile(v42)
                        } else {
                            US0::US0_OtherFile
                        }
                    }
                }
            };
            match v48 {
                US0::US0_OtherFile => {
                    v12
                }
                _ => {
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v50: Rc<UH0> = split_list_6(v49.clone(), v12);
                    let v51: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v52: Rc<UH0> = method9(v50, v51);
                    let v53: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v54: i32 = 0i32;
                    let v55: i32 = method10(v53.clone(), v52, v54);
                    let v56: Rc<Vec<Rc<str>>> = Rc::new(v53.borrow().clone());
                    let v57: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v56).as_ref().clone()));
                    let v58: Rc<str> = Rc::<str>::from((*v57.borrow()).join(&*v49));
                    v58
                }
            }
        }
    } else {
        let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v61
    };
    let v63: bool = &*v5 == "";
    let v65: bool = if v63 {
        true
    } else {
        let v64: bool = &*v62 == "";
        v64
    };
    if v65 {
        0i32
    } else {
        let v66: Rc<UH1> = waits_of_text_11(v62);
        let v67: Rc<UH0> = done_ids_17(v0);
        let v68: Rc<UH0> = unblock_lines_33(v66, v67, v5);
        method39(v68.clone());
        let v69: i32 = 0i32;
        method31(v68, v69)
    }
}
fn closure5() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> i32 {
        lane_waits_printed_30(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_waits_holding(v0: &str, v1: &str) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_agile_lane_waits_printed(v0: &str, v1: &str, v2: &str) -> i32 {
    closure5()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub type SpiralShape_652accfc6a8d = Mut0;
