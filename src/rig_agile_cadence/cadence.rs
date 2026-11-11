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
enum US4 {
    US4_Some(i64),
    US4_None,
}
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_1(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
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
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_1(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn loop_4(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method5(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_6(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(6i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_8(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method11(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method11(v7, &v9) {
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
        method10(v16, v17)
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method12(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
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
                            let mut v22: US3 = US3::US3_HeadInl;
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
fn method13(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn text_after_15(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let v4: Rc<str> = v4.clone();
                        let v5: Rc<UH0> = v5.clone();
                        let mut v6: bool = &*v2 == &*v1;
                        if v6 {
                            return v4;
                        } else {
                            v0 = v3;
                            continue;
                        }
                    }
                    _ => {
                        v0 = v3;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v11;
            }
        }
    }
}
fn method16(mut v0: Option<i64>) -> Option<i64> {
    v0
}
fn closure2() -> Rc<dyn Fn((i64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i64)) -> US4> = Rc::new(move |mut v0: (i64)| -> US4 {
        let v1: i64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn number_after_14(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i64) -> i64 {
    let v3: Rc<str> = text_after_15(v0, v1);
    let v4: Option<i64> = (v3).trim().parse::<i64>().ok();
    let v5: Option<i64> = method16(v4);
    let mut v6: Rc<dyn Fn((i64)) -> US4> = closure2();
    let mut v7: Option<US4> = v5.map(|x| v6(x));
    let mut v9: US4 = US4::US4_None;
    let v10: US4 = v9.clone();
    let v11: US4 = v7.unwrap_or(v10);
    match v11 {
        US4::US4_None => {
            v2
        }
        US4::US4_Some(v12) => {
            let v12: i64 = v12;
            v12
        }
    }
}
fn day_of_date_17(mut v0: Rc<str>) -> US4 {
    let v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v2: Rc<str> = string_concat(&[&*v1, "T00:00Z"]);
    let v3: i64 = agile_quality_state::rig_agile_stamp_hour(&v2);
    let v4: bool = v3 > 0i64;
    if v4 {
        let v5: i64 = v3.wrapping_div(24i64);
        US4::US4_Some(v5)
    } else {
        US4::US4_None
    }
}
fn method19(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let mut v6: i32 = v1.wrapping_sub(1i32);
            let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn listed_18(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<UH0> {
    let v1: Rc<Vec<Rc<str>>> = Rc::new((v0).borrow().clone());
    let v2: usize = (v1).len();
    let v3: i32 = ((v2) as i32);
    let v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method19(v1, v4, v5)
}
fn method20(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
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
fn method22(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_21(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method22(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method24(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig agile cadence done "); } LIT.with(|lit| lit.clone()) };
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
fn method25(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method25(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig agile cadence silent "); } LIT.with(|lit| lit.clone()) };
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
fn format_real_26(mut v0: i64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method22(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn dating_text_27(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = StatusChanged (\""); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.contains(&*v1);
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (done dated by status change, else last note)"); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (done dated by last note)"); } LIT.with(|lit| lit.clone()) };
        v4
    }
}
fn summary_of_23(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i64, mut v3: Rc<str>) -> Rc<str> {
    let v4: i32 = 0i32;
    let v5: i32 = method20(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method24(v0.clone(), v6);
    let v8: i32 = 0i32;
    let mut v9: i32 = method20(v7, v8);
    let v10: Rc<str> = format_real_21(v9);
    let v13: Rc<str> = format_real_21(v5);
    let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH0> = method25(v0, v16);
    let v18: i32 = 0i32;
    let v19: i32 = method20(v17, v18);
    let v20: Rc<str> = format_real_21(v19);
    let v23: Rc<str> = format_real_26(v2);
    let mut v25: Rc<str> = dating_text_27(v3);
    let mut v26: Rc<str> = string_concat(&["rig agile cadence ok done_rows=", &*v10, " activity_rows=", &*v13, " silent_active=", &*v20, " window_hours=", &*v23, &*v25]);
    v26
}
fn method28(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method28(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method29(mut v0: Rc<UH0>) -> () {
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
fn cadence_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_1(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                            let mut v9: bool = v6.starts_with(&*v8);
                            let v10: bool = v9 == false;
                            if v10 {
                                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                                let v13: Rc<str> = Rc::<str>::from(v6.replace(&*v11, &*v12));
                                let v14: Rc<str> = string_concat(&[&*v13, "/state"]);
                                let v15: Rc<str> = string_concat(&[&*v14, "/agile.spi"]);
                                let v16: bool = std::path::Path::new(&*v15).is_file();
                                let v68: Rc<str> = if v16 {
                                    let (mut v17, mut v18): (bool, Rc<str>) = match std::fs::read(&*v15) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                                    if v17 {
                                        let v19: Rc<str> = string_concat(&["rig state read failed: ", &*v15]);
                                        std::panic::panic_any::<std::string::String>(format!("{}", v19))
                                    } else {
                                        let v21: Rc<str> = Rc::<str>::from(v15.replace(&*v11, &*v12));
                                        let mut v22: Rc<str> = loop_4(v21);
                                        let v23: Option<usize> = v22.rfind(&*v12);
                                        let v24: Option<usize> = method5(v23);
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
                                        let v54: US1 = if v33 {
                                            US1::US1_OtherFile
                                        } else {
                                            let v35: usize = ((0i32) as usize);
                                            let v36: usize = ((v28) as usize);
                                            let v37: Rc<str> = Rc::<str>::from(&(v22)[v35..v36]);
                                            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                            let v39: bool = v37.ends_with(&*v38);
                                            if v39 {
                                                let v40: Rc<str> = loop_6(v37);
                                                US1::US1_StateFile(v40)
                                            } else {
                                                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                let v43: bool = v37.ends_with(&*v42);
                                                if v43 {
                                                    let v44: Rc<str> = loop_7(v37);
                                                    US1::US1_HistoryFile(v44)
                                                } else {
                                                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                    let v47: bool = v37.ends_with(&*v46);
                                                    if v47 {
                                                        let v48: Rc<str> = loop_8(v37);
                                                        US1::US1_ArchiveFile(v48)
                                                    } else {
                                                        US1::US1_OtherFile
                                                    }
                                                }
                                            }
                                        };
                                        match v54 {
                                            US1::US1_OtherFile => {
                                                v18
                                            }
                                            _ => {
                                                let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                let v56: Rc<UH0> = split_list_9(v55.clone(), v18);
                                                let v57: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                let v58: Rc<UH0> = method12(v56, v57);
                                                let v59: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                let v60: i32 = 0i32;
                                                let v61: i32 = method13(v59.clone(), v58, v60);
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
                                let v69: Rc<str> = Rc::<str>::from(v6.replace(&*v11, &*v12));
                                let v70: Rc<str> = string_concat(&[&*v69, "/history/status_changes.spi"]);
                                let v71: bool = std::path::Path::new(&*v70).is_file();
                                let v123: Rc<str> = if v71 {
                                    let (mut v72, mut v73): (bool, Rc<str>) = match std::fs::read(&*v70) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                                    if v72 {
                                        let v74: Rc<str> = string_concat(&["rig state read failed: ", &*v70]);
                                        std::panic::panic_any::<std::string::String>(format!("{}", v74))
                                    } else {
                                        let v76: Rc<str> = Rc::<str>::from(v70.replace(&*v11, &*v12));
                                        let mut v77: Rc<str> = loop_4(v76);
                                        let v78: Option<usize> = v77.rfind(&*v12);
                                        let v79: Option<usize> = method5(v78);
                                        let mut v80: Rc<dyn Fn((usize)) -> i32> = closure1();
                                        let mut v81: Option<i32> = v79.map(|x| v80(x));
                                        let v83: i32 = v81.unwrap_or(-1i32);
                                        let v84: bool = v83 < 0i32;
                                        let v88: bool = if v84 {
                                            true
                                        } else {
                                            let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                            let v86: bool = v77.ends_with(&*v85);
                                            let v87: bool = v86 == false;
                                            v87
                                        };
                                        let v109: US1 = if v88 {
                                            US1::US1_OtherFile
                                        } else {
                                            let v90: usize = ((0i32) as usize);
                                            let v91: usize = ((v83) as usize);
                                            let v92: Rc<str> = Rc::<str>::from(&(v77)[v90..v91]);
                                            let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                            let v94: bool = v92.ends_with(&*v93);
                                            if v94 {
                                                let v95: Rc<str> = loop_6(v92);
                                                US1::US1_StateFile(v95)
                                            } else {
                                                let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                let v98: bool = v92.ends_with(&*v97);
                                                if v98 {
                                                    let v99: Rc<str> = loop_7(v92);
                                                    US1::US1_HistoryFile(v99)
                                                } else {
                                                    let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                    let v102: bool = v92.ends_with(&*v101);
                                                    if v102 {
                                                        let v103: Rc<str> = loop_8(v92);
                                                        US1::US1_ArchiveFile(v103)
                                                    } else {
                                                        US1::US1_OtherFile
                                                    }
                                                }
                                            }
                                        };
                                        match v109 {
                                            US1::US1_OtherFile => {
                                                v73
                                            }
                                            _ => {
                                                let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                let v111: Rc<UH0> = split_list_9(v110.clone(), v73);
                                                let v112: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                let v113: Rc<UH0> = method12(v111, v112);
                                                let v114: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                let v115: i32 = 0i32;
                                                let v116: i32 = method13(v114.clone(), v113, v115);
                                                let v117: Rc<Vec<Rc<str>>> = Rc::new(v114.borrow().clone());
                                                let v118: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v117).as_ref().clone()));
                                                let v119: Rc<str> = Rc::<str>::from((*v118.borrow()).join(&*v110));
                                                v119
                                            }
                                        }
                                    }
                                } else {
                                    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v122
                                };
                                let v124: Rc<str> = string_concat(&[&*v14, "/asks.spi"]);
                                let v125: bool = std::path::Path::new(&*v124).is_file();
                                let v177: Rc<str> = if v125 {
                                    let (mut v126, mut v127): (bool, Rc<str>) = match std::fs::read(&*v124) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                                    if v126 {
                                        let v128: Rc<str> = string_concat(&["rig state read failed: ", &*v124]);
                                        std::panic::panic_any::<std::string::String>(format!("{}", v128))
                                    } else {
                                        let v130: Rc<str> = Rc::<str>::from(v124.replace(&*v11, &*v12));
                                        let mut v131: Rc<str> = loop_4(v130);
                                        let v132: Option<usize> = v131.rfind(&*v12);
                                        let v133: Option<usize> = method5(v132);
                                        let mut v134: Rc<dyn Fn((usize)) -> i32> = closure1();
                                        let mut v135: Option<i32> = v133.map(|x| v134(x));
                                        let v137: i32 = v135.unwrap_or(-1i32);
                                        let v138: bool = v137 < 0i32;
                                        let v142: bool = if v138 {
                                            true
                                        } else {
                                            let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                            let v140: bool = v131.ends_with(&*v139);
                                            let v141: bool = v140 == false;
                                            v141
                                        };
                                        let v163: US1 = if v142 {
                                            US1::US1_OtherFile
                                        } else {
                                            let v144: usize = ((0i32) as usize);
                                            let v145: usize = ((v137) as usize);
                                            let v146: Rc<str> = Rc::<str>::from(&(v131)[v144..v145]);
                                            let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                            let v148: bool = v146.ends_with(&*v147);
                                            if v148 {
                                                let v149: Rc<str> = loop_6(v146);
                                                US1::US1_StateFile(v149)
                                            } else {
                                                let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                let v152: bool = v146.ends_with(&*v151);
                                                if v152 {
                                                    let v153: Rc<str> = loop_7(v146);
                                                    US1::US1_HistoryFile(v153)
                                                } else {
                                                    let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                    let v156: bool = v146.ends_with(&*v155);
                                                    if v156 {
                                                        let v157: Rc<str> = loop_8(v146);
                                                        US1::US1_ArchiveFile(v157)
                                                    } else {
                                                        US1::US1_OtherFile
                                                    }
                                                }
                                            }
                                        };
                                        match v163 {
                                            US1::US1_OtherFile => {
                                                v127
                                            }
                                            _ => {
                                                let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                let v165: Rc<UH0> = split_list_9(v164.clone(), v127);
                                                let v166: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                let v167: Rc<UH0> = method12(v165, v166);
                                                let v168: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                let v169: i32 = 0i32;
                                                let v170: i32 = method13(v168.clone(), v167, v169);
                                                let v171: Rc<Vec<Rc<str>>> = Rc::new(v168.borrow().clone());
                                                let v172: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v171).as_ref().clone()));
                                                let v173: Rc<str> = Rc::<str>::from((*v172.borrow()).join(&*v164));
                                                v173
                                            }
                                        }
                                    }
                                } else {
                                    let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v176
                                };
                                let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ask-days"); } LIT.with(|lit| lit.clone()) };
                                let v179: i64 = 3i64;
                                let v180: i64 = number_after_14(v7.clone(), v178, v179);
                                let v181: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                                let v182: i64 = (v181 as i64);
                                let v183: i64 = v182.wrapping_div(3600000i64);
                                let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--hours"); } LIT.with(|lit| lit.clone()) };
                                let v185: i64 = 48i64;
                                let v186: i64 = number_after_14(v7.clone(), v184, v185);
                                let v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--since"); } LIT.with(|lit| lit.clone()) };
                                let v188: Rc<str> = text_after_15(v7.clone(), v187);
                                let v189: US4 = day_of_date_17(v188);
                                let v197: i64 = match v189 {
                                    US4::US4_None => {
                                        let v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--days"); } LIT.with(|lit| lit.clone()) };
                                        let v194: i64 = 7i64;
                                        number_after_14(v7.clone(), v193, v194)
                                    }
                                    US4::US4_Some(v190) => {
                                        let v190: i64 = v190;
                                        let v191: i64 = v183.wrapping_div(24i64);
                                        let v192: i64 = v191.wrapping_sub(v190);
                                        v192
                                    }
                                };
                                let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--round"); } LIT.with(|lit| lit.clone()) };
                                let v199: i64 = 0i64;
                                let v200: i64 = number_after_14(v7.clone(), v198, v199);
                                let v201: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team"); } LIT.with(|lit| lit.clone()) };
                                let v202: Rc<str> = text_after_15(v7, v201);
                                let v203: bool = v200 > 0i64;
                                let v224: Rc<UH0> = if v203 {
                                    let v204: Rc<RefCell<Vec<Rc<str>>>> = rig_cadence_activity::rig_cadence_round_report(&v68, v183, v200);
                                    listed_18(v204)
                                } else {
                                    let v206: Rc<RefCell<Vec<Rc<str>>>> = rig_cadence_done::rig_cadence_done_report(&v68, &v123, v183, v186, v197, &v202);
                                    let v207: Rc<UH0> = listed_18(v206);
                                    let v208: i64 = v183.wrapping_div(24i64);
                                    let v209: Rc<RefCell<Vec<Rc<str>>>> = rig_cadence_activity::rig_cadence_activity_report(&v68, v208, v197, &v202);
                                    let v210: Rc<UH0> = listed_18(v209);
                                    let v211: Rc<RefCell<Vec<Rc<str>>>> = rig_cadence_asks::rig_cadence_asks_report(&v68, &v177, v183, v180, &v202);
                                    let v212: Rc<UH0> = listed_18(v211);
                                    let v213: i32 = 0i32;
                                    let v214: i32 = method20(v212.clone(), v213);
                                    let v215: Rc<str> = format_real_21(v214);
                                    let v217: Rc<str> = summary_of_23(v207.clone(), v210.clone(), v186, v123);
                                    let v218: Rc<str> = string_concat(&[&*v217, " neglected_asks=", &*v215]);
                                    let v219: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v220: Rc<UH0> = Rc::new(UH0::UH0_Cons(v218, v219));
                                    let v221: Rc<UH0> = method28(v212, v220);
                                    let v222: Rc<UH0> = method28(v210, v221);
                                    method28(v207, v222)
                                };
                                method29(v224);
                                0i32
                            } else {
                                let v225: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: rig agile cadence <root> [--hours n] [--days n | --since yyyy-mm-dd] [--team t] [--round h] [--ask-days n]"); } LIT.with(|lit| lit.clone()) };
                                eprintln!("{}", v225);
                                2i32
                            }
                        }
                        _ => {
                            let v227: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: rig agile cadence <root> [--hours n] [--days n | --since yyyy-mm-dd] [--team t] [--round h] [--ask-days n]"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v227);
                            2i32
                        }
                    }
                }
                _ => {
                    let v229: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: rig agile cadence <root> [--hours n] [--days n | --since yyyy-mm-dd] [--team t] [--round h] [--ask-days n]"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v229);
                    2i32
                }
            }
        }
        _ => {
            let v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: rig agile cadence <root> [--hours n] [--days n | --since yyyy-mm-dd] [--team t] [--round h] [--ask-days n]"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v231);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        cadence_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_cadence_run() -> i32 {
    closure0()()
}
use rig_cadence_activity::SpiralShape_97df63ebabc3 as Mut1;
pub type SpiralShape_652accfc6a8d = Mut0;
