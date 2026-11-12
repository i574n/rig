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
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US4 {
    US4_TaskLine(Rc<str>, Rc<str>, Rc<str>),
    US4_PlanningLine(Rc<str>, i32, Rc<str>, Rc<str>),
    US4_NoteLine(Rc<str>, Rc<str>),
    US4_WaitLine(Rc<str>, Rc<str>),
    US4_MarkerLine(Rc<str>),
    US4_OtherLine,
}
#[derive(Clone)]
enum US5 {
    US5_Some(i32),
    US5_None,
}
#[derive(Clone)]
enum US6 {
    US6_Some(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US6_None,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH2>),
}
#[derive(Clone)]
enum US7 {
    US7_Some(Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_Some(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US8_None,
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<str>, u64, Rc<UH3>),
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US9 {
    US9_Some(Rc<str>),
    US9_None,
}
#[derive(Clone)]
enum US10 {
    US10_Some(Rc<str>, Rc<str>, u64),
    US10_None,
}
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(Rc<UH0>, Rc<UH4>),
}
fn loop_1(mut v0: Rc<str>) -> Rc<str> {
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
fn method2(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_3(mut v0: Rc<str>) -> Rc<str> {
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
fn loop_4(mut v0: Rc<str>) -> Rc<str> {
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
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
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
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method8(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
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
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method7(v16, v17)
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method9(v3, v1);
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
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US2 = US2::US2_HeadInl;
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
                    let mut v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let mut v30: US2 = US2::US2_HeadUnion;
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
            let mut v51: US1 = match &v39 {
                US1::US1_OtherLine => {
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US2 = US2::US2_HeadOpen;
                        US1::US1_KeywordHead(v42)
                    } else {
                        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
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
fn concat5_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn civil_of_days_14(mut v0: u64) -> (u64, u64, u64) {
    let mut v1: u64 = v0.wrapping_add(719468u64);
    let mut v2: u64 = v1.wrapping_div(146097u64);
    let mut v3: u64 = v2.wrapping_mul(146097u64);
    let v4: u64 = v1.wrapping_sub(v3);
    let mut v5: u64 = v4.wrapping_div(1460u64);
    let mut v6: u64 = v4.wrapping_sub(v5);
    let mut v7: u64 = v4.wrapping_div(36524u64);
    let mut v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let v10: u64 = v8.wrapping_sub(v9);
    let mut v11: u64 = v10.wrapping_div(365u64);
    let mut v12: u64 = 365u64.wrapping_mul(v11);
    let v13: u64 = v11.wrapping_div(4u64);
    let mut v14: u64 = v12.wrapping_add(v13);
    let mut v15: u64 = v11.wrapping_div(100u64);
    let v16: u64 = v14.wrapping_sub(v15);
    let v17: u64 = v4.wrapping_sub(v16);
    let v18: u64 = 5u64.wrapping_mul(v17);
    let v19: u64 = v18.wrapping_add(2u64);
    let v20: u64 = v19.wrapping_div(153u64);
    let v21: u64 = 153u64.wrapping_mul(v20);
    let v22: u64 = v21.wrapping_add(2u64);
    let v23: u64 = v22.wrapping_div(5u64);
    let v24: u64 = v17.wrapping_sub(v23);
    let mut v25: u64 = v24.wrapping_add(1u64);
    let mut v26: bool = v20 < 10u64;
    let mut v29: u64 = if v26 {
        let v27: u64 = v20.wrapping_add(3u64);
        v27
    } else {
        let v28: u64 = v20.wrapping_sub(9u64);
        v28
    };
    let mut v30: u64 = v2.wrapping_mul(400u64);
    let mut v31: u64 = v11.wrapping_add(v30);
    let v32: bool = v29 < 3u64;
    let v33: u64 = if v32 {
        1u64
    } else {
        0u64
    };
    let v34: u64 = v31.wrapping_add(v33);
    (v34, v29, v25)
}
fn utc_stamp_12(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(1000u64);
    let mut v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_14(v2);
    let mut v6: u64 = v1.wrapping_rem(86400u64);
    let mut v7: u64 = v6.wrapping_div(60u64);
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v4 < 10u64;
    let v16: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_13(v11, v12, v13.clone(), v13.clone(), v13)
    } else {
        let mut v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15
    };
    let v17: bool = v5 < 10u64;
    let v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_13(v18, v19, v20.clone(), v20.clone(), v20)
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22
    };
    let v24: Rc<str> = concat5_13(v8, v9.clone(), v16, v9, v23);
    let mut v25: u64 = v7.wrapping_div(60u64);
    let mut v26: bool = v25 < 10u64;
    let v32: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_13(v27, v28, v29.clone(), v29.clone(), v29)
    } else {
        let mut v31: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        v31
    };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v34: u64 = v7.wrapping_rem(60u64);
    let v35: bool = v34 < 10u64;
    let v41: Rc<str> = if v35 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_13(v36, v37, v38.clone(), v38.clone(), v38)
    } else {
        let mut v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v44: Rc<str> = concat5_13(v32, v33, v41, v42, v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    concat5_13(v24, v45, v44, v43.clone(), v43)
}
fn method16(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn window_minutes_15(mut v0: Rc<str>) -> US3 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: usize = ((0i32) as usize);
    let v4: usize = ((v2) as usize);
    let mut v5: Rc<str> = Rc::<str>::from(&(v0)[v3..v4]);
    let mut v6: Option<u64> = (v5).trim().parse::<u64>().ok();
    let mut v7: Option<u64> = method16(v6);
    let mut v8: Rc<dyn Fn((u64)) -> US3> = closure2();
    let mut v9: Option<US3> = v7.map(|x| v8(x));
    let mut v11: US3 = US3::US3_None;
    let mut v12: US3 = v11.clone();
    let v13: US3 = v9.unwrap_or(v12);
    match v13 {
        US3::US3_Some(v14) => {
            let mut v14: u64 = v14;
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v16: bool = v0.ends_with(&*v15);
            if v16 {
                let v17: u64 = v14.wrapping_mul(60u64);
                US3::US3_Some(v17)
            } else {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v20: bool = v0.ends_with(&*v19);
                if v20 {
                    US3::US3_Some(v14)
                } else {
                    US3::US3_None
                }
            }
        }
        _ => {
            US3::US3_None
        }
    }
}
fn since_of_11(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: US3 = window_minutes_15(v0.clone());
    match v2 {
        US3::US3_None => {
            v0
        }
        US3::US3_Some(v3) => {
            let mut v3: u64 = v3;
            let v4: u64 = v3.wrapping_mul(60000u64);
            let mut v5: u64 = v1.wrapping_sub(v4);
            utc_stamp_12(v5)
        }
    }
}
fn not_after_19(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = (&*v0) <= (&*v1);
    v2
}
fn merged_18(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Nil => {
            v1
        }
        _ => {
            match &*v1 {
                UH1::UH1_Nil => {
                    v0
                }
                _ => {
                    match &*v0 {
                        UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                            let mut v2: Rc<str> = v2.clone();
                            let mut v3: Rc<str> = v3.clone();
                            let v4: Rc<str> = v4.clone();
                            let mut v5: Rc<str> = v5.clone();
                            let mut v6: Rc<UH1> = v6.clone();
                            match &*v1 {
                                UH1::UH1_Cons(v7, v8, v9, v10, v11) => {
                                    let mut v7: Rc<str> = v7.clone();
                                    let mut v8: Rc<str> = v8.clone();
                                    let mut v9: Rc<str> = v9.clone();
                                    let v10: Rc<str> = v10.clone();
                                    let mut v11: Rc<UH1> = v11.clone();
                                    let mut v12: bool = not_after_19(v2.clone(), v7.clone());
                                    if v12 {
                                        let v13: Rc<UH1> = merged_18(v6, v1);
                                        Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v13))
                                    } else {
                                        let mut v15: Rc<UH1> = merged_18(v0, v11);
                                        Rc::new(UH1::UH1_Cons(v7, v8, v9, v10, v15))
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}
fn method25(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method25(v3, v1);
            let mut v5: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method26(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure3() -> Rc<dyn Fn((i32)) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US5> = Rc::new(move |mut v0: (i32)| -> US5 {
        let mut v1: i32 = (v0);
        US5::US5_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_line_24(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> US4 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = v0.borrow()[2i32 as usize].clone();
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let mut v6: Rc<UH0> = split_list_6(v1, v5);
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = method25(v6, v7);
    match Rc::unwrap_or_clone(v8) {
        UH0::UH0_Cons(v9, v10) => {
            let mut v9: Rc<str> = v9;
            let v10: Rc<UH0> = v10;
            match Rc::unwrap_or_clone(v10) {
                UH0::UH0_Cons(v11, v12) => {
                    let mut v11: Rc<str> = v11;
                    let mut v12: Rc<UH0> = v12;
                    match Rc::unwrap_or_clone(v12) {
                        UH0::UH0_Cons(v13, v14) => {
                            let v13: Rc<str> = v13;
                            let mut v14: Rc<UH0> = v14;
                            match Rc::unwrap_or_clone(v14) {
                                UH0::UH0_Cons(v15, v16) => {
                                    let mut v15: Rc<str> = v15;
                                    let v16: Rc<UH0> = v16;
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Minutes "); } LIT.with(|lit| lit.clone()) };
                                    let v18: bool = v11.starts_with(&*v17);
                                    let v20: Rc<str> = if v18 {
                                        let v19: Rc<str> = Rc::<str>::from(v11.replace(&*v17, &*v4));
                                        v19
                                    } else {
                                        v4.clone()
                                    };
                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                                    let v22: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v4));
                                    let v23: Option<i32> = (v22).trim().parse::<i32>().ok();
                                    let v24: Option<i32> = method26(v23);
                                    let mut v25: Rc<dyn Fn((i32)) -> US5> = closure3();
                                    let mut v26: Option<US5> = v24.map(|x| v25(x));
                                    let v28: US5 = US5::US5_None;
                                    let mut v29: US5 = v28.clone();
                                    let mut v30: US5 = v26.unwrap_or(v29);
                                    let v33: i32 = match v30 {
                                        US5::US5_None => {
                                            0i32
                                        }
                                        US5::US5_Some(v31) => {
                                            let mut v31: i32 = v31;
                                            v31
                                        }
                                    };
                                    let v34: Rc<str> = v0.borrow()[1i32 as usize].clone();
                                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load "); } LIT.with(|lit| lit.clone()) };
                                    let v36: bool = v13.starts_with(&*v35);
                                    let mut v38: Rc<str> = if v36 {
                                        let v37: Rc<str> = Rc::<str>::from(v13.replace(&*v35, &*v4));
                                        v37
                                    } else {
                                        v4.clone()
                                    };
                                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) };
                                    let mut v40: bool = v15.starts_with(&*v39);
                                    let v42: Rc<str> = if v40 {
                                        let v41: Rc<str> = Rc::<str>::from(v15.replace(&*v39, &*v4));
                                        v41
                                    } else {
                                        v4
                                    };
                                    US4::US4_PlanningLine(v34, v33, v38, v42)
                                }
                                _ => {
                                    US4::US4_OtherLine
                                }
                            }
                        }
                        _ => {
                            US4::US4_OtherLine
                        }
                    }
                }
                _ => {
                    US4::US4_OtherLine
                }
            }
        }
        _ => {
            US4::US4_OtherLine
        }
    }
}
fn method28(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method28(v3, v1);
            let mut v5: bool = &*v2 != "";
            if v5 {
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
fn method29(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                (v0, v1) = (v3, v2);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn task_line_27(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> US4 {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = v1.wrapping_sub(9i32);
    let v4: Rc<str> = v0.borrow()[v3 as usize].clone();
    let mut v5: Rc<UH0> = split_list_6(v2, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method25(v5, v6);
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method28(v7, v8);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = method29(v9, v10);
    let mut v12: Rc<str> = v0.borrow()[1i32 as usize].clone();
    let v13: i32 = v1.wrapping_sub(6i32);
    let mut v14: Rc<str> = v0.borrow()[v13 as usize].clone();
    US4::US4_TaskLine(v12, v11, v14)
}
fn state_line_of_23(mut v0: Rc<str>) -> US4 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 < 3i32;
    if v4 {
        US4::US4_OtherLine
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl note_of_"); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v0.starts_with(&*v6);
        if v7 {
            let mut v8: Rc<str> = v2.borrow()[1i32 as usize].clone();
            US4::US4_NoteLine(v8, v0)
        } else {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl wait_of_"); } LIT.with(|lit| lit.clone()) };
            let mut v11: bool = v0.starts_with(&*v10);
            let mut v14: bool = if v11 {
                true
            } else {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= wait_of_"); } LIT.with(|lit| lit.clone()) };
                let v13: bool = v0.starts_with(&*v12);
                v13
            };
            let v16: bool = if v14 {
                let mut v15: bool = v3 >= 7i32;
                v15
            } else {
                false
            };
            if v16 {
                let v17: Rc<str> = v2.borrow()[1i32 as usize].clone();
                let v18: Rc<str> = v2.borrow()[3i32 as usize].clone();
                let v20: Rc<str> = v2.borrow()[5i32 as usize].clone();
                let v21: Rc<str> = string_concat(&[&*v18, ":", &*v20]);
                US4::US4_WaitLine(v17, v21)
            } else {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Marker (\""); } LIT.with(|lit| lit.clone()) };
                let v24: bool = v0.contains(&*v23);
                let mut v26: bool = if v24 {
                    let mut v25: bool = v3 >= 5i32;
                    v25
                } else {
                    false
                };
                if v26 {
                    let v27: Rc<str> = v2.borrow()[1i32 as usize].clone();
                    let mut v29: Rc<str> = v2.borrow()[3i32 as usize].clone();
                    let mut v30: Rc<str> = string_concat(&[&*v27, "=", &*v29]);
                    US4::US4_MarkerLine(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl planning_of_"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v0.starts_with(&*v32);
                    if v33 {
                        planning_line_24(v2)
                    } else {
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                        let v36: bool = v0.starts_with(&*v35);
                        let v39: bool = if v36 {
                            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() = Task ("); } LIT.with(|lit| lit.clone()) };
                            let mut v38: bool = v0.contains(&*v37);
                            v38
                        } else {
                            false
                        };
                        let v41: bool = if v39 {
                            let mut v40: bool = v3 >= 10i32;
                            v40
                        } else {
                            false
                        };
                        if v41 {
                            task_line_27(v2)
                        } else {
                            US4::US4_OtherLine
                        }
                    }
                }
            }
        }
    }
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: US4 = state_line_of_23(v2);
                let v10: Rc<str> = match v4 {
                    US4::US4_MarkerLine(v5) => {
                        let mut v5: Rc<str> = v5;
                        let mut v6: bool = &*v1 == "";
                        if v6 {
                            v5
                        } else {
                            let mut v8: Rc<str> = string_concat(&[&*v1, "\n", &*v5]);
                            v8
                        }
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v3, v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn declared_markers_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_6(v1, v4);
    method22(v5, v3)
}
fn method33(mut v0: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>) -> Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    v0
}
fn closure4() -> Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US6> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US6> = Rc::new(move |mut v0: ((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))| -> US6 {
        let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = (v0);
        US6::US6_Some(v1, v2, v3, v4, v5, v6, v7, v8)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn folded_32(mut v0: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>, mut v1: US4) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> {
    match v1 {
        US4::US4_MarkerLine(v175) => {
            let v175: Rc<str> = v175;
            v0
        }
        US4::US4_NoteLine(v87, v88) => {
            let v87: Rc<str> = v87;
            let v88: Rc<str> = v88;
            let v89: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v87).cloned();
            let v90: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method33(v89);
            let mut v91: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US6> = closure4();
            let mut v92: Option<US6> = v90.map(|x| v91(x));
            let mut v94: US6 = US6::US6_None;
            let v95: US6 = v94.clone();
            let v96: US6 = v92.unwrap_or(v95);
            let (mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v96 {
                US6::US6_Some(v97, v98, v99, v100, v101, v102, v103, v104) => {
                    let mut v97: Rc<str> = v97.clone();
                    let mut v98: Rc<str> = v98.clone();
                    let v99: i32 = *v99;
                    let v100: Rc<str> = v100.clone();
                    let v101: Rc<str> = v101.clone();
                    let v102: Rc<str> = v102.clone();
                    let v103: Rc<str> = v103.clone();
                    let v104: Rc<str> = v104.clone();
                    (v97, v98, v99, v100, v101, v102, v103, v104)
                }
                _ => {
                    let (mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v96 {
                        US6::US6_None => {
                            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v105.clone(), v105.clone(), 0i32, v105.clone(), v105.clone(), v105.clone(), v105.clone(), v105.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v106, v107, v108, v109, v110, v111, v112, v113)
                }
            };
            let v123: Rc<str> = string_concat(&[&*v119, "\n", &*v88]);
            let mut v124: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v126: bool = true; // 1;
            let mut v124 = v124;
            let v128: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v124.insert(v87, (v114, v115, v116, v117, v118, v123, v120, v121));
            let v129: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v124);
            v129
        }
        US4::US4_OtherLine => {
            v0
        }
        US4::US4_PlanningLine(v44, v45, v46, v47) => {
            let mut v44: Rc<str> = v44;
            let v45: i32 = v45;
            let v46: Rc<str> = v46;
            let v47: Rc<str> = v47;
            let v48: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v44).cloned();
            let v49: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method33(v48);
            let mut v50: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US6> = closure4();
            let mut v51: Option<US6> = v49.map(|x| v50(x));
            let v53: US6 = US6::US6_None;
            let v54: US6 = v53.clone();
            let v55: US6 = v51.unwrap_or(v54);
            let (mut v73, mut v74, mut v78, mut v79, mut v80): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v55 {
                US6::US6_Some(v56, v57, v58, v59, v60, v61, v62, v63) => {
                    let v56: Rc<str> = v56.clone();
                    let v57: Rc<str> = v57.clone();
                    let v58: i32 = *v58;
                    let v59: Rc<str> = v59.clone();
                    let v60: Rc<str> = v60.clone();
                    let v61: Rc<str> = v61.clone();
                    let v62: Rc<str> = v62.clone();
                    let v63: Rc<str> = v63.clone();
                    (v56, v57, v61, v62, v63)
                }
                _ => {
                    let (mut v65, mut v66, mut v70, mut v71, mut v72): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v55 {
                        US6::US6_None => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v64.clone(), v64.clone(), v64.clone(), v64.clone(), v64.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v65, v66, v70, v71, v72)
                }
            };
            let mut v81: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v83: bool = true; // 1;
            let mut v81 = v81;
            let v85: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v81.insert(v44, (v73, v74, v45, v46, v47, v78, v79, v80));
            let v86: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v81);
            v86
        }
        US4::US4_TaskLine(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let mut v5: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v2).cloned();
            let mut v6: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method33(v5);
            let mut v7: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US6> = closure4();
            let mut v8: Option<US6> = v6.map(|x| v7(x));
            let v10: US6 = US6::US6_None;
            let mut v11: US6 = v10.clone();
            let mut v12: US6 = v8.unwrap_or(v11);
            let (mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                US6::US6_Some(v13, v14, v15, v16, v17, v18, v19, v20) => {
                    let v13: Rc<str> = v13.clone();
                    let mut v14: Rc<str> = v14.clone();
                    let mut v15: i32 = *v15;
                    let v16: Rc<str> = v16.clone();
                    let v17: Rc<str> = v17.clone();
                    let v18: Rc<str> = v18.clone();
                    let v19: Rc<str> = v19.clone();
                    let v20: Rc<str> = v20.clone();
                    (v15, v16, v17, v18, v19, v20)
                }
                _ => {
                    let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v12 {
                        US6::US6_None => {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (0i32, v21.clone(), v21.clone(), v21.clone(), v21.clone(), v21.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v24, v25, v26, v27, v28, v29)
                }
            };
            let mut v38: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let mut v40: bool = true; // 1;
            let mut v38 = v38;
            let v42: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v38.insert(v2, (v3, v4, v32, v33, v34, v35, v36, v37));
            let mut v43: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v38);
            v43
        }
        US4::US4_WaitLine(v130, v131) => {
            let v130: Rc<str> = v130;
            let v131: Rc<str> = v131;
            let v132: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v130).cloned();
            let v133: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method33(v132);
            let mut v134: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US6> = closure4();
            let mut v135: Option<US6> = v133.map(|x| v134(x));
            let v137: US6 = US6::US6_None;
            let v138: US6 = v137.clone();
            let v139: US6 = v135.unwrap_or(v138);
            let (mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v139 {
                US6::US6_Some(v140, v141, v142, v143, v144, v145, v146, v147) => {
                    let v140: Rc<str> = v140.clone();
                    let v141: Rc<str> = v141.clone();
                    let v142: i32 = *v142;
                    let v143: Rc<str> = v143.clone();
                    let v144: Rc<str> = v144.clone();
                    let v145: Rc<str> = v145.clone();
                    let v146: Rc<str> = v146.clone();
                    let v147: Rc<str> = v147.clone();
                    (v140, v141, v142, v143, v144, v145, v146, v147)
                }
                _ => {
                    let (mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v139 {
                        US6::US6_None => {
                            let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v148.clone(), v148.clone(), 0i32, v148.clone(), v148.clone(), v148.clone(), v148.clone(), v148.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v149, v150, v151, v152, v153, v154, v155, v156)
                }
            };
            let v165: bool = &*v163 == "";
            let v168: Rc<str> = if v165 {
                v131
            } else {
                let v167: Rc<str> = string_concat(&[&*v163, ",", &*v131]);
                v167
            };
            let mut v169: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v171: bool = true; // 1;
            let mut v169 = v169;
            let v173: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v169.insert(v130, (v157, v158, v159, v160, v161, v162, v168, v164));
            let v174: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v169);
            v174
        }
    }
}
fn method31(mut v0: Rc<UH0>, mut v1: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: US4 = state_line_of_23(v2);
                let mut v5: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = folded_32(v1, v4);
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn facts_of_30(mut v0: Rc<str>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_6(v1, v4);
    let mut v7: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::collections::BTreeMap::new();
    let mut v8: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v7);
    method31(v5, v8)
}
fn method34(mut v0: Rc<str>, mut v1: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>, mut v2: Rc<UH0>, mut v3: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v1.get(&v4).cloned();
                let mut v7: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method33(v6);
                let mut v8: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US6> = closure4();
                let mut v9: Option<US6> = v7.map(|x| v8(x));
                let mut v11: US6 = US6::US6_None;
                let mut v12: US6 = v11.clone();
                let v13: US6 = v9.unwrap_or(v12);
                let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v13 {
                    US6::US6_Some(v14, v15, v16, v17, v18, v19, v20, v21) => {
                        let mut v14: Rc<str> = v14.clone();
                        let mut v15: Rc<str> = v15.clone();
                        let v16: i32 = *v16;
                        let v17: Rc<str> = v17.clone();
                        let v18: Rc<str> = v18.clone();
                        let v19: Rc<str> = v19.clone();
                        let v20: Rc<str> = v20.clone();
                        let v21: Rc<str> = v21.clone();
                        (v14, v15, v16, v17, v18, v19, v20, v21)
                    }
                    _ => {
                        let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v13 {
                            US6::US6_None => {
                                std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dict.present: the key is not in the dict"); } LIT.with(|lit| lit.clone()) }))
                            }
                            _ => unreachable!(),
                        };
                        (v30, v31, v32, v33, v34, v35, v36, v37)
                    }
                };
                let v46: bool = &*v38 == "";
                let v48: Rc<UH2> = if v46 {
                    v3
                } else {
                    Rc::new(UH2::UH2_Cons(v4, v38, v39, v40, v41, v42, v43, v44, v0.clone(), v3))
                };
                (v2, v3) = (v5, v48);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn method35(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let mut v5: i32 = v5;
                let mut v6: Rc<str> = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let mut v11: Rc<UH2> = v11;
                let mut v12: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10, v1));
                (v0, v1) = (v11, v12);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method36(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: Rc<UH2>, mut v2: i32) -> i32 {
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
                let mut v11: Rc<str> = v11;
                let mut v12: Rc<UH2> = v12;
                v0.borrow_mut().push((v3, v4, v5, v6, v7, v8, v9, v10, v11));
                let v13: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v12, v13);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn fact_rows_of_20(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    let mut v1: Rc<str> = declared_markers_21(v0.clone());
    let mut v2: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = facts_of_30(v0);
    let mut v3: Vec<Rc<str>> = v2.keys().cloned().collect::<Vec<_>>();
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: bool = true; let _vec_fold_ = v3.into_iter().fold(v4, |acc, x| { //;
    let mut v8: Rc<UH0> = acc;
    let v10: Rc<str> = x;
    let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v8));
    let v13: bool = true; v12 });
    let mut v15: Rc<UH0> = _vec_fold_;
    let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH0> = method7(v15, v16);
    let v18: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v19: Rc<UH2> = method34(v1, v2, v17, v18);
    let v20: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v21: Rc<UH2> = method35(v19, v20);
    let v22: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v23: i32 = 0i32;
    let v24: i32 = method36(v22.clone(), v21, v23);
    let mut v25: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new(v22.borrow().clone());
    let mut v26: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v25).as_ref().clone()));
    v26
}
fn method37(mut v0: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>, mut v1: i32, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = (v0)[v4].clone();
            let mut v14: i32 = v1.wrapping_sub(1i32);
            let mut v15: Rc<UH2> = Rc::new(UH2::UH2_Cons(v5, v6, v7, v8, v9, v10, v11, v12, v13, v2));
            (v1, v2) = (v14, v15);
            continue;
        }
    }
}
fn quoted_parts_40(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    split_list_6(v1, v0)
}
fn note_of_line_39(mut v0: Rc<str>) -> US7 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Note (\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.contains(&*v1);
    let mut v3: bool = v2 == false;
    if v3 {
        US7::US7_None
    } else {
        let mut v5: Rc<UH0> = quoted_parts_40(v0);
        match Rc::unwrap_or_clone(v5) {
            UH0::UH0_Cons(v6, v7) => {
                let mut v6: Rc<str> = v6;
                let mut v7: Rc<UH0> = v7;
                match Rc::unwrap_or_clone(v7) {
                    UH0::UH0_Cons(v8, v9) => {
                        let mut v8: Rc<str> = v8;
                        let mut v9: Rc<UH0> = v9;
                        match Rc::unwrap_or_clone(v9) {
                            UH0::UH0_Cons(v10, v11) => {
                                let v10: Rc<str> = v10;
                                let mut v11: Rc<UH0> = v11;
                                match Rc::unwrap_or_clone(v11) {
                                    UH0::UH0_Cons(v12, v13) => {
                                        let mut v12: Rc<str> = v12;
                                        let v13: Rc<UH0> = v13;
                                        match Rc::unwrap_or_clone(v13) {
                                            UH0::UH0_Cons(v14, v15) => {
                                                let mut v14: Rc<str> = v14;
                                                let mut v15: Rc<UH0> = v15;
                                                match Rc::unwrap_or_clone(v15) {
                                                    UH0::UH0_Cons(v16, v17) => {
                                                        let v16: Rc<str> = v16;
                                                        let v17: Rc<UH0> = v17;
                                                        match Rc::unwrap_or_clone(v17) {
                                                            UH0::UH0_Cons(v18, v19) => {
                                                                let v18: Rc<str> = v18;
                                                                let v19: Rc<UH0> = v19;
                                                                match Rc::unwrap_or_clone(v19) {
                                                                    UH0::UH0_Cons(v20, v21) => {
                                                                        let v20: Rc<str> = v20;
                                                                        let v21: Rc<UH0> = v21;
                                                                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                                                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                        let v24: Rc<str> = Rc::<str>::from(v10.replace(&*v22, &*v23));
                                                                        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v26: Rc<str> = Rc::<str>::from(v24.replace(&*v25, &*v23));
                                                                        let v27: Rc<str> = Rc::<str>::from((v26).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                                                        let v28: Rc<str> = { let text = &*v20; if text.chars().count() <= 70i32 as usize { std::rc::Rc::<str>::from(text) } else { std::rc::Rc::<str>::from(text.chars().take((70i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
                                                                        let mut v29: bool = &*v27 == "1";
                                                                        let v33: Rc<str> = if v29 {
                                                                            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("new item, note 1: "); } LIT.with(|lit| lit.clone()) };
                                                                            v30
                                                                        } else {
                                                                            let v32: Rc<str> = string_concat(&["note ", &*v27, ": "]);
                                                                            v32
                                                                        };
                                                                        let v34: Rc<str> = string_concat(&[&*v33, &*v28]);
                                                                        US7::US7_Some(v12, v16, v8, v34)
                                                                    }
                                                                    _ => {
                                                                        US7::US7_None
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                US7::US7_None
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        US7::US7_None
                                                    }
                                                }
                                            }
                                            _ => {
                                                US7::US7_None
                                            }
                                        }
                                    }
                                    _ => {
                                        US7::US7_None
                                    }
                                }
                            }
                            _ => {
                                US7::US7_None
                            }
                        }
                    }
                    _ => {
                        US7::US7_None
                    }
                }
            }
            _ => {
                US7::US7_None
            }
        }
    }
}
fn method38(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: US7 = note_of_line_39(v2);
                let mut v11: Rc<UH1> = match v4 {
                    US7::US7_None => {
                        v1
                    }
                    US7::US7_Some(v5, v6, v7, v8) => {
                        let mut v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        let mut v7: Rc<str> = v7;
                        let mut v8: Rc<str> = v8;
                        Rc::new(UH1::UH1_Cons(v5, v6, v7, v8, v1))
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
fn method41(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
                let mut v6: Rc<UH1> = v6;
                let mut v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v1));
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method43(mut v0: Rc<str>, mut v1: Rc<UH2>) -> US8 {
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
                let mut v11: Rc<str> = v11;
                let mut v12: Rc<UH2> = v12;
                let v13: bool = &*v3 == &*v0;
                if v13 {
                    return US8::US8_Some(v3, v4, v5, v6, v7, v8, v9, v10, v11);
                } else {
                    v1 = v12;
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return US8::US8_None;
            }
        }
    }
}
fn closure5() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn tag_listed_45(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v3: Rc<str> = string_concat(&[",", &*v1, ","]);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
    let mut v8: Rc<str> = string_concat(&[",", &*v6, ","]);
    let mut v9: Option<usize> = v8.find(&*v3);
    let v10: Option<usize> = method2(v9);
    let mut v11: Rc<dyn Fn((usize)) -> i32> = closure5();
    let mut v12: Option<i32> = v10.map(|x| v11(x));
    let mut v14: i32 = v12.unwrap_or(-1i32);
    let mut v15: bool = v14 >= 0i32;
    v15
}
fn team_tagged_46(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = string_concat(&[",", &*v3]);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",team:"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Option<usize> = v4.find(&*v5);
    let mut v7: Option<usize> = method2(v6);
    let mut v8: Rc<dyn Fn((usize)) -> i32> = closure5();
    let mut v9: Option<i32> = v7.map(|x| v8(x));
    let mut v11: i32 = v9.unwrap_or(-1i32);
    let mut v12: bool = v11 >= 0i32;
    v12
}
fn on_team_44(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: bool = team_tagged_46(v2.clone());
    if v3 {
        let v4: Rc<str> = string_concat(&["team:", &*v0]);
        tag_listed_45(v2, v4)
    } else {
        let mut v6: bool = tag_listed_45(v2.clone(), v0.clone());
        let mut v9: bool = if v6 {
            true
        } else {
            let mut v7: Rc<str> = string_concat(&[&*v0, "-team"]);
            tag_listed_45(v2, v7)
        };
        if v9 {
            true
        } else {
            let v10: Rc<str> = Rc::<str>::from(v0.to_uppercase());
            let mut v11: Rc<str> = string_concat(&[&*v10, "-"]);
            let mut v12: bool = v1.starts_with(&*v11);
            v12
        }
    }
}
fn method42(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH2>, mut v3: Rc<UH1>, mut v4: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v3) {
        UH1::UH1_Cons(v5, v6, v7, v8, v9) => {
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<str> = v6;
            let mut v7: Rc<str> = v7;
            let mut v8: Rc<str> = v8;
            let mut v9: Rc<UH1> = v9;
            let v10: Rc<UH1> = method42(v0.clone(), v1.clone(), v2.clone(), v9, v4);
            let mut v11: bool = not_after_19(v0, v5.clone());
            let v28: bool = if v11 {
                let mut v12: bool = &*v1 == "";
                if v12 {
                    true
                } else {
                    let v13: US8 = method43(v7.clone(), v2);
                    let mut v25: Rc<str> = match v13 {
                        US8::US8_None => {
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v23
                        }
                        US8::US8_Some(v14, v15, v16, v17, v18, v19, v20, v21, v22) => {
                            let mut v14: Rc<str> = v14;
                            let mut v15: Rc<str> = v15;
                            let v16: Rc<str> = v16;
                            let v17: i32 = v17;
                            let v18: Rc<str> = v18;
                            let v19: Rc<str> = v19;
                            let v20: Rc<str> = v20;
                            let v21: Rc<str> = v21;
                            let v22: Rc<str> = v22;
                            v16
                        }
                    };
                    on_team_44(v1, v7.clone(), v25)
                }
            } else {
                false
            };
            if v28 {
                Rc::new(UH1::UH1_Cons(v5, v6, v7, v8, v10))
            } else {
                v10
            }
        }
        UH1::UH1_Nil => {
            v4
        }
    }
}
fn method51(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_50(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method51(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method52(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US9 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let mut v5: bool = v3.contains(&*v0);
                if v5 {
                    return US9::US9_Some(v3);
                } else {
                    v1 = v4;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US9::US9_None;
            }
        }
    }
}
fn author_of_49(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> Rc<str> {
    let mut v5: Rc<str> = format_real_50(v2);
    let mut v7: Rc<str> = string_concat(&["StatusAuthor (\"", &*v1, "\", ", &*v5, "u64, \""]);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<UH0> = split_list_6(v8, v0);
    let v10: US9 = method52(v7.clone(), v9);
    match v10 {
        US9::US9_None => {
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v35
        }
        US9::US9_Some(v11) => {
            let mut v11: Rc<str> = v11;
            let mut v12: Option<usize> = v11.find(&*v7);
            let v13: Option<usize> = method2(v12);
            let mut v14: Rc<dyn Fn((usize)) -> i32> = closure5();
            let mut v15: Option<i32> = v13.map(|x| v14(x));
            let v17: i32 = v15.unwrap_or(-1i32);
            let v18: i32 = (v7.len() as i32);
            let v19: i32 = v17.wrapping_add(v18);
            let v20: i32 = (v11.clone().len() as i32);
            let v21: i32 = v20.wrapping_sub(v19);
            let v22: usize = ((v19) as usize);
            let v23: i32 = v19.wrapping_add(v21);
            let v24: usize = ((v23) as usize);
            let mut v25: Rc<str> = Rc::<str>::from(&(v11)[v22..v24]);
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v27: Option<usize> = v25.find(&*v26);
            let v28: Option<usize> = method2(v27);
            let mut v29: Option<i32> = v28.map(|x| v14(x));
            let mut v31: i32 = v29.unwrap_or(-1i32);
            let v32: usize = ((0i32) as usize);
            let v33: usize = ((v31) as usize);
            let v34: Rc<str> = Rc::<str>::from(&(v25)[v32..v33]);
            v34
        }
    }
}
fn method48(mut v0: Rc<str>, mut v1: Rc<UH3>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH3::UH3_Cons(v3, v4, v5, v6) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let mut v5: u64 = v5;
            let mut v6: Rc<UH3> = v6;
            let mut v7: Rc<UH1> = method48(v0.clone(), v6, v2);
            let mut v8: Rc<str> = utc_stamp_12(v5);
            let mut v9: Rc<str> = author_of_49(v0, v3.clone(), v5);
            let v10: Rc<str> = string_concat(&["status -> ", &*v4]);
            Rc::new(UH1::UH1_Cons(v8, v9, v3, v10, v7))
        }
        UH3::UH3_Nil => {
            v2
        }
    }
}
fn method54(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: u64 = v4;
                let mut v5: Rc<UH3> = v5;
                let mut v6: Rc<UH3> = Rc::new(UH3::UH3_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn change_of_line_56(mut v0: Rc<str>) -> US10 {
    let mut v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = StatusChanged (\""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Option<usize> = v1.find(&*v2);
    let v4: Option<usize> = method2(v3);
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure5();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let mut v8: i32 = v6.unwrap_or(-1i32);
    let mut v9: bool = v8 < 0i32;
    let v13: bool = if v9 {
        true
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64)"); } LIT.with(|lit| lit.clone()) };
        let mut v11: bool = v1.ends_with(&*v10);
        let mut v12: bool = v11 == false;
        v12
    };
    if v13 {
        US10::US10_None
    } else {
        let mut v15: i32 = v8.wrapping_add(19i32);
        let v16: i32 = (v1.clone().len() as i32);
        let v17: i32 = v16.wrapping_sub(5i32);
        let v18: Rc<str> = string_slice(&v1, v15 as i64, v17 as i64);
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<UH0> = split_list_6(v19, v18);
        match Rc::unwrap_or_clone(v20) {
            UH0::UH0_Cons(v21, v22) => {
                let v21: Rc<str> = v21;
                let v22: Rc<UH0> = v22;
                match Rc::unwrap_or_clone(v22) {
                    UH0::UH0_Cons(v23, v24) => {
                        let v23: Rc<str> = v23;
                        let v24: Rc<UH0> = v24;
                        match Rc::unwrap_or_clone(v24) {
                            UH0::UH0_Cons(v25, v26) => {
                                let mut v25: Rc<str> = v25;
                                let mut v26: Rc<UH0> = v26;
                                match Rc::unwrap_or_clone(v26) {
                                    UH0::UH0_Nil => {
                                        let v27: bool = &*v21 != "";
                                        if v27 {
                                            let v28: Option<u64> = (v25).trim().parse::<u64>().ok();
                                            let mut v29: Option<u64> = method16(v28);
                                            let mut v30: Rc<dyn Fn((u64)) -> US3> = closure2();
                                            let mut v31: Option<US3> = v29.map(|x| v30(x));
                                            let v33: US3 = US3::US3_None;
                                            let v34: US3 = v33.clone();
                                            let v35: US3 = v31.unwrap_or(v34);
                                            match v35 {
                                                US3::US3_None => {
                                                    US10::US10_None
                                                }
                                                US3::US3_Some(v36) => {
                                                    let v36: u64 = v36;
                                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    let v39: Rc<str> = Rc::<str>::from(v23.replace(&*v37, &*v38));
                                                    US10::US10_Some(v21, v39, v36)
                                                }
                                            }
                                        } else {
                                            US10::US10_None
                                        }
                                    }
                                    _ => {
                                        US10::US10_None
                                    }
                                }
                            }
                            _ => {
                                US10::US10_None
                            }
                        }
                    }
                    _ => {
                        US10::US10_None
                    }
                }
            }
            _ => {
                US10::US10_None
            }
        }
    }
}
fn method55(mut v0: Rc<UH0>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: US10 = change_of_line_56(v2);
                let v10: Rc<UH3> = match v4 {
                    US10::US10_None => {
                        v1
                    }
                    US10::US10_Some(v5, v6, v7) => {
                        let mut v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        let mut v7: u64 = v7;
                        Rc::new(UH3::UH3_Cons(v5, v6, v7, v1))
                    }
                };
                (v0, v1) = (v3, v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn changes_of_text_53(mut v0: Rc<str>) -> Rc<UH3> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_6(v1, v4);
    let mut v6: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH3> = method55(v5, v6);
    let mut v8: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    method54(v7, v8)
}
fn status_rows_47(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<UH3> = changes_of_text_53(v0.clone());
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method48(v0, v1, v2)
}
fn changes_of_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<UH1> {
    let v4: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = fact_rows_of_20(v0.clone());
    let mut v5: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new((v4).borrow().clone());
    let mut v6: usize = (v5).len();
    let mut v7: i32 = ((v6) as i32);
    let mut v8: i32 = v7.wrapping_sub(1i32);
    let mut v9: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH2> = method37(v5, v8, v9);
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<UH0> = split_list_6(v11, v0);
    let v13: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v14: Rc<UH1> = method38(v12, v13);
    let mut v15: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH1> = method41(v14, v15);
    let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v18: Rc<UH1> = method42(v2.clone(), v3.clone(), v10.clone(), v16, v17);
    let v19: Rc<UH1> = status_rows_47(v1);
    let v20: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v21: Rc<UH1> = method42(v2, v3, v10, v19, v20);
    merged_18(v18, v21)
}
fn method59(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method59(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method58(mut v0: Rc<UH4>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH4::UH4_Cons(v2, v3) => {
                let mut v2: Rc<UH0> = v2;
                let mut v3: Rc<UH4> = v3;
                let v4: Rc<UH0> = method59(v1, v2);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH4::UH4_Nil => {
                return v1;
            }
        }
    }
}
fn closure6(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US5> {
    Rc::new(move |mut v2: Rc<UH0>| -> US5 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US5::US5_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US5> = method61(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US5::US5_None
            }
        }
    })
}
fn method61(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US5> {
    closure6(v0, v1)
}
fn method60(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
                let mut v6: Rc<UH1> = v6;
                let mut v7: i32 = 0i32;
                let mut v8: Rc<dyn Fn(Rc<UH0>) -> US5> = method61(v3.clone(), v7);
                let mut v9: US5 = v8(v1.clone());
                let v10: bool = match v9 {
                    US5::US5_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v11: bool = v10 != true;
                let mut v15: Rc<UH0> = if v11 {
                    v1
                } else {
                    let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v12));
                    method59(v1, v13)
                };
                (v0, v1) = (v6, v15);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method63(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<str> = v6;
            let mut v7: Rc<UH1> = v7;
            let mut v8: Rc<UH1> = method63(v0.clone(), v7, v2);
            let mut v9: bool = &*v4 == &*v0;
            if v9 {
                Rc::new(UH1::UH1_Cons(v3, v4, v5, v6, v8))
            } else {
                v8
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn method64(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<UH1> = v6;
            let mut v7: Rc<UH0> = method64(v6, v1);
            let mut v12: Rc<str> = string_concat(&["  ", &*v2, "  ", &*v4, "  ", &*v5]);
            Rc::new(UH0::UH0_Cons(v12, v7))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method62(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: Rc<UH4>) -> Rc<UH4> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let mut v5: Rc<UH4> = method62(v0.clone(), v4, v2);
            let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let mut v7: Rc<UH1> = method63(v3.clone(), v0, v6);
            let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v9: Rc<UH0> = method64(v7, v8);
            let v10: Rc<str> = string_concat(&[&*v3, ":"]);
            let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v9));
            Rc::new(UH4::UH4_Cons(v11, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn lines_of_57(mut v0: Rc<UH1>) -> Rc<UH0> {
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH0> = method60(v0.clone(), v1);
    let mut v3: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH4> = method62(v0, v2, v3);
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method58(v4, v5)
}
fn method65(mut v0: Rc<UH0>) -> () {
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
fn method66(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<UH1> = v6;
            let mut v7: Rc<UH1> = method66(v6, v1);
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
            let mut v9: bool = v5.starts_with(&*v8);
            let v10: bool = v9 == false;
            if v10 {
                Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v7))
            } else {
                v7
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method67(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
                let mut v6: Rc<UH1> = v6;
                let mut v7: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn format_real_68(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method51(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn changes_run_0() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v4: bool = &*v3 == "--since";
    let mut v14: Rc<str> = if v4 {
        let mut v5: i32 = 3i32;
        let mut v6: Rc<str> = usize::try_from(v5).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        v6
    } else {
        let mut v7: i32 = 4i32;
        let mut v8: Rc<str> = usize::try_from(v7).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v9: bool = &*v8 == "--since";
        if v9 {
            let v10: i32 = 5i32;
            let mut v11: Rc<str> = usize::try_from(v10).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            v11
        } else {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v12
        }
    };
    let mut v15: i32 = 2i32;
    let v16: Rc<str> = usize::try_from(v15).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v17: bool = &*v16 == "--team";
    let v27: Rc<str> = if v17 {
        let v18: i32 = 3i32;
        let v19: Rc<str> = usize::try_from(v18).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        v19
    } else {
        let v20: i32 = 4i32;
        let v21: Rc<str> = usize::try_from(v20).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v22: bool = &*v21 == "--team";
        if v22 {
            let v23: i32 = 5i32;
            let v24: Rc<str> = usize::try_from(v23).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            v24
        } else {
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v25
        }
    };
    let v28: bool = &*v14 == "";
    if v28 {
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: rig agile changes <root> --since <8h|30m|yyyy-mm-ddThh:mmZ> [--team <team>]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v29);
        2i32
    } else {
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v32: Rc<str> = Rc::<str>::from(v1.replace(&*v30, &*v31));
        let v33: Rc<str> = string_concat(&[&*v32, "/state/agile.spi"]);
        let v34: bool = std::path::Path::new(&*v33).is_file();
        let v86: Rc<str> = if v34 {
            let (mut v35, mut v36): (bool, Rc<str>) = match std::fs::read(&*v33) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v35 {
                let v37: Rc<str> = string_concat(&["rig state read failed: ", &*v33]);
                std::panic::panic_any::<std::string::String>(format!("{}", v37))
            } else {
                let v39: Rc<str> = Rc::<str>::from(v33.replace(&*v30, &*v31));
                let mut v40: Rc<str> = loop_1(v39);
                let v41: Option<usize> = v40.rfind(&*v31);
                let v42: Option<usize> = method2(v41);
                let mut v43: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v44: Option<i32> = v42.map(|x| v43(x));
                let v46: i32 = v44.unwrap_or(-1i32);
                let v47: bool = v46 < 0i32;
                let mut v51: bool = if v47 {
                    true
                } else {
                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v49: bool = v40.ends_with(&*v48);
                    let mut v50: bool = v49 == false;
                    v50
                };
                let v72: US0 = if v51 {
                    US0::US0_OtherFile
                } else {
                    let v53: usize = ((0i32) as usize);
                    let v54: usize = ((v46) as usize);
                    let v55: Rc<str> = Rc::<str>::from(&(v40)[v53..v54]);
                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v57: bool = v55.ends_with(&*v56);
                    if v57 {
                        let v58: Rc<str> = loop_3(v55);
                        US0::US0_StateFile(v58)
                    } else {
                        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v61: bool = v55.ends_with(&*v60);
                        if v61 {
                            let v62: Rc<str> = loop_4(v55);
                            US0::US0_HistoryFile(v62)
                        } else {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v65: bool = v55.ends_with(&*v64);
                            if v65 {
                                let v66: Rc<str> = loop_5(v55);
                                US0::US0_ArchiveFile(v66)
                            } else {
                                US0::US0_OtherFile
                            }
                        }
                    }
                };
                match v72 {
                    US0::US0_OtherFile => {
                        v36
                    }
                    _ => {
                        let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v74: Rc<UH0> = split_list_6(v73.clone(), v36);
                        let v75: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v76: Rc<UH0> = method9(v74, v75);
                        let v77: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v78: i32 = 0i32;
                        let v79: i32 = method10(v77.clone(), v76, v78);
                        let v80: Rc<Vec<Rc<str>>> = Rc::new(v77.borrow().clone());
                        let mut v81: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v80).as_ref().clone()));
                        let v82: Rc<str> = Rc::<str>::from((*v81.borrow()).join(&*v73));
                        v82
                    }
                }
            }
        } else {
            let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v85
        };
        let v87: Rc<str> = string_concat(&[&*v32, "/history/status_changes.spi"]);
        let v88: bool = std::path::Path::new(&*v87).is_file();
        let v140: Rc<str> = if v88 {
            let (mut v89, mut v90): (bool, Rc<str>) = match std::fs::read(&*v87) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v89 {
                let mut v91: Rc<str> = string_concat(&["rig state read failed: ", &*v87]);
                std::panic::panic_any::<std::string::String>(format!("{}", v91))
            } else {
                let v93: Rc<str> = Rc::<str>::from(v87.replace(&*v30, &*v31));
                let mut v94: Rc<str> = loop_1(v93);
                let v95: Option<usize> = v94.rfind(&*v31);
                let v96: Option<usize> = method2(v95);
                let mut v97: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v98: Option<i32> = v96.map(|x| v97(x));
                let v100: i32 = v98.unwrap_or(-1i32);
                let v101: bool = v100 < 0i32;
                let v105: bool = if v101 {
                    true
                } else {
                    let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v103: bool = v94.ends_with(&*v102);
                    let v104: bool = v103 == false;
                    v104
                };
                let v126: US0 = if v105 {
                    US0::US0_OtherFile
                } else {
                    let v107: usize = ((0i32) as usize);
                    let v108: usize = ((v100) as usize);
                    let v109: Rc<str> = Rc::<str>::from(&(v94)[v107..v108]);
                    let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v111: bool = v109.ends_with(&*v110);
                    if v111 {
                        let v112: Rc<str> = loop_3(v109);
                        US0::US0_StateFile(v112)
                    } else {
                        let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v115: bool = v109.ends_with(&*v114);
                        if v115 {
                            let v116: Rc<str> = loop_4(v109);
                            US0::US0_HistoryFile(v116)
                        } else {
                            let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v119: bool = v109.ends_with(&*v118);
                            if v119 {
                                let v120: Rc<str> = loop_5(v109);
                                US0::US0_ArchiveFile(v120)
                            } else {
                                US0::US0_OtherFile
                            }
                        }
                    }
                };
                match v126 {
                    US0::US0_OtherFile => {
                        v90
                    }
                    _ => {
                        let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v128: Rc<UH0> = split_list_6(v127.clone(), v90);
                        let v129: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v130: Rc<UH0> = method9(v128, v129);
                        let v131: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v132: i32 = 0i32;
                        let v133: i32 = method10(v131.clone(), v130, v132);
                        let mut v134: Rc<Vec<Rc<str>>> = Rc::new(v131.borrow().clone());
                        let mut v135: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v134).as_ref().clone()));
                        let v136: Rc<str> = Rc::<str>::from((*v135.borrow()).join(&*v127));
                        v136
                    }
                }
            }
        } else {
            let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v139
        };
        let v141: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let v142: Rc<str> = since_of_11(v14, v141);
        let v143: Rc<UH1> = changes_of_17(v86, v140, v142.clone(), v27.clone());
        let v144: Rc<UH0> = lines_of_57(v143.clone());
        method65(v144);
        let v145: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let v146: Rc<UH1> = method66(v143.clone(), v145);
        let v147: i32 = 0i32;
        let v148: i32 = method67(v146, v147);
        let v149: i32 = 0i32;
        let v150: i32 = method67(v143, v149);
        let v151: i32 = v150.wrapping_sub(v148);
        let v152: bool = &*v27 == "";
        let v155: Rc<str> = if v152 {
            let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v153
        } else {
            let v154: Rc<str> = string_concat(&[" team=", &*v27]);
            v154
        };
        let v159: Rc<str> = format_real_68(v148);
        let v162: Rc<str> = format_real_68(v151);
        let v163: Rc<str> = string_concat(&["rig agile changes ok since=", &*v142, &*v155, " notes=", &*v159, " status_changes=", &*v162]);
        println!("{}", v163);
        0i32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        changes_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_changes_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
