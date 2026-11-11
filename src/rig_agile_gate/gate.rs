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
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
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
    US4_Some(i32),
    US4_None,
}
pub struct Mut1 { pub l0: Rc<str> }
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
    let mut v8: US0 = if v4 {
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
fn flag_value_5(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let v4: Rc<str> = v4.clone();
                        let v5: Rc<UH0> = v5.clone();
                        let v6: bool = &*v2 == &*v1;
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
fn terms_of_4(mut v0: Rc<str>, mut v1: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = flag_value_5(v1.clone(), v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = flag_value_5(v1, v4);
    (v0, v3, v5)
}
fn method6(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let v6: i32 = v1.wrapping_sub(1i32);
            let mut v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method7(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH0> = method7(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn loop_8(mut v0: Rc<str>) -> Rc<str> {
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
fn method9(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_10(mut v0: Rc<str>) -> Rc<str> {
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
fn loop_11(mut v0: Rc<str>) -> Rc<str> {
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
fn loop_12(mut v0: Rc<str>) -> Rc<str> {
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
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method15(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
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
        while method15(v7, &v9) {
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
        method14(v16, v17)
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
                        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
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
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method20(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: bool = &*v1 == "";
                let v5: Rc<str> = if v4 {
                    v2
                } else {
                    v1
                };
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method21(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method21(v0.clone(), v4, v2);
            let v6: bool = &*v3 != &*v0;
            if v6 {
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
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method22(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method19(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Note (\""); } LIT.with(|lit| lit.clone()) };
                let v5: Option<usize> = v2.find(&*v4);
                let v6: Option<usize> = method9(v5);
                let mut v7: Rc<dyn Fn((usize)) -> i32> = closure2();
                let mut v8: Option<i32> = v6.map(|x| v7(x));
                let v10: i32 = v8.unwrap_or(-1i32);
                let v11: bool = v10 < 0i32;
                let v33: Rc<UH0> = if v11 {
                    v1
                } else {
                    let v12: i32 = v10.wrapping_add(10i32);
                    let v13: i32 = (v2.clone().len() as i32);
                    let v14: i32 = v13.wrapping_sub(v10);
                    let v15: i32 = v14.wrapping_sub(10i32);
                    let v16: usize = ((v12) as usize);
                    let v17: i32 = v12.wrapping_add(v15);
                    let v18: usize = ((v17) as usize);
                    let v19: Rc<str> = Rc::<str>::from(&(v2)[v16..v18]);
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<UH0> = split_list_13(v20, v19);
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = method20(v21, v22);
                    let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v25: Rc<UH0> = method21(v23.clone(), v1, v24);
                    let v26: Rc<str> = Rc::<str>::from(v2.to_lowercase());
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waits on install: train"); } LIT.with(|lit| lit.clone()) };
                    let v28: bool = v26.contains(&*v27);
                    if v28 {
                        let v29: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v30: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v29));
                        method22(v25, v30)
                    } else {
                        v25
                    }
                };
                (v0, v1) = (v3, v33);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn install_waiting_18(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH0> = split_list_13(v1, v0);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method19(v2, v3)
}
fn ids_where_23(mut v0: &Rc<RefCell<Vec<(Rc<str>, bool)>>>, mut v1: bool, mut v2: i32) -> Rc<UH0> {
    let v3: bool = v2 < 0i32;
    if v3 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let (mut v5, mut v6): (Rc<str>, bool) = v0.borrow()[v2 as usize].clone();
        let mut v7: i32 = v2.wrapping_sub(1i32);
        let mut v8: Rc<UH0> = ids_where_23(v0, v1, v7);
        let mut v9: bool = v6 == v1;
        if v9 {
            let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v10));
            method22(v8, v11)
        } else {
            v8
        }
    }
}
fn method24(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method24(v0.clone(), v4, v2);
            let mut v7: Rc<str> = string_concat(&["Waits (\"", &*v3, "\","]);
            let mut v8: bool = v0.contains(&*v7);
            if v8 {
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
fn closure3(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US4> {
    Rc::new(move |mut v2: Rc<UH0>| -> US4 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US4::US4_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US4> = method26(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US4::US4_None
            }
        }
    })
}
fn method26(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US4> {
    closure3(v0, v1)
}
fn method25(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method25(v0.clone(), v4, v2);
            let v6: i32 = 0i32;
            let mut v7: Rc<dyn Fn(Rc<UH0>) -> US4> = method26(v3.clone(), v6);
            let mut v8: US4 = v7(v0);
            let mut v9: bool = match v8 {
                US4::US4_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v10: bool = v9 != true;
            let v11: bool = v10 == false;
            if v11 {
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
fn taken_27(mut v0: i32, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: bool = v0 > 0i32;
            if v4 {
                let v5: i32 = v0.wrapping_sub(1i32);
                let v6: Rc<UH0> = taken_27(v5, v3);
                Rc::new(UH0::UH0_Cons(v2, v6))
            } else {
                { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
            }
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method28(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method28(v0.clone(), v4, v2);
            let v6: i32 = 0i32;
            let mut v7: Rc<dyn Fn(Rc<UH0>) -> US4> = method26(v3.clone(), v6);
            let mut v8: US4 = v7(v0);
            let mut v9: bool = match v8 {
                US4::US4_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v10: bool = v9 != true;
            if v10 {
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
fn method30(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn method32(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_31(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method32(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn ready_line_33(mut v0: Rc<str>, mut v1: &Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>) -> Rc<str> {
    let v4: bool = match &*v3 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v10: i32 = 0i32;
        let v11: i32 = method17(v9.clone(), v3, v10);
        let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
        let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v14));
        let v20: Rc<str> = string_concat(&["Your team's top ready light items: ", &*v15, ". Claim one explicitly (rig agile claim ", &*v0, " <ID> --as ", &*v2, ") and start it now; while a run or train you started is pending, work another of these instead of stopping."]);
        v20
    }
}
fn parked_line_34(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: &Rc<str>, mut v3: Rc<UH0>) -> Rc<str> {
    let v4: bool = match &*v3 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let mut v9: i32 = 0i32;
        let v10: i32 = method17(v8.clone(), v3, v9);
        let v11: Rc<Vec<Rc<str>>> = Rc::new(v8.borrow().clone());
        let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v11).as_ref().clone()));
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = Rc::<str>::from((*v12.borrow()).join(&*v13));
        let v16: Rc<str> = string_concat(&["Paused P0/P1 items for team ", &*v1, " (re-decide each: resume as Planned, or keep Paused only with a reason AND a resume condition): ", &*v14, "."]);
        v16
    }
}
fn waiting_line_35(mut v0: Rc<UH0>) -> Rc<str> {
    let v1: bool = match &*v0 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v4: i32 = 0i32;
        let v5: i32 = method17(v3.clone(), v0, v4);
        let v6: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
        let mut v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = Rc::<str>::from((*v7.borrow()).join(&*v8));
        let v11: Rc<str> = string_concat(&["Built and waiting on a train install (not ready work): ", &*v9, "."]);
        v11
    }
}
fn gate_lines_29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: Rc<UH0>, mut v5: Rc<UH0>) -> Rc<UH0> {
    let mut v8: i32 = 0i32;
    let mut v9: i32 = method30(v3.clone(), v8);
    let v10: Rc<str> = format_real_31(v9);
    let v13: i32 = 0i32;
    let v14: i32 = method30(v4.clone(), v13);
    let v15: Rc<str> = format_real_31(v14);
    let v18: i32 = 0i32;
    let v19: i32 = method30(v5.clone(), v18);
    let v20: Rc<str> = format_real_31(v19);
    let v21: Rc<str> = string_concat(&["rig agile gate ok team=", &*v1, " ready=", &*v10, " paused=", &*v15, " waiting=", &*v20]);
    let v22: Rc<str> = ready_line_33(v0.clone(), &v1, v2.clone(), v3);
    let v23: Rc<str> = parked_line_34(&v0, v1, &v2, v4);
    let v24: Rc<str> = waiting_line_35(v5);
    let v25: bool = &*v21 != "";
    let v29: Rc<UH0> = if v25 {
        let v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v21, v26))
    } else {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    };
    let v30: bool = &*v24 != "";
    let v32: Rc<UH0> = if v30 {
        Rc::new(UH0::UH0_Cons(v24, v29))
    } else {
        v29
    };
    let v33: bool = &*v23 != "";
    let v35: Rc<UH0> = if v33 {
        Rc::new(UH0::UH0_Cons(v23, v32))
    } else {
        v32
    };
    let v36: bool = &*v22 != "";
    if v36 {
        Rc::new(UH0::UH0_Cons(v22, v35))
    } else {
        v35
    }
}
fn method36(mut v0: Rc<UH0>) -> () {
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
fn gate_run_0() -> i32 {
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
                            let v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                            let mut v9: bool = v6.starts_with(&*v8);
                            let v10: bool = v9 == false;
                            if v10 {
                                let (mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>) = terms_of_4(v6.clone(), v7);
                                let v14: bool = &*v12 == "";
                                let v16: bool = if v14 {
                                    true
                                } else {
                                    let v15: bool = &*v13 == "";
                                    v15
                                };
                                if v16 {
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                                    eprintln!("{}", v17);
                                    2i32
                                } else {
                                    let v18: Rc<RefCell<Vec<(Rc<str>, bool)>>> = agile_team_work::rig_agile_gate_rows(&v6, &v12);
                                    let v19: i32 = (v18.clone().borrow().len() as i32);
                                    let v20: i32 = v19.wrapping_sub(1i32);
                                    let v21: Rc<RefCell<Vec<Rc<str>>>> = agile_team_lane::rig_agile_team_paused(&v6, &v12);
                                    let v22: Rc<Vec<Rc<str>>> = Rc::new((v21).borrow().clone());
                                    let v23: usize = (v22).len();
                                    let v24: i32 = ((v23) as i32);
                                    let v25: i32 = v24.wrapping_sub(1i32);
                                    let v26: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                                    let v27: Rc<UH1> = method6(v22, v25, v26);
                                    let v28: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v29: Rc<UH0> = method7(v27, v28);
                                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                                    let v32: Rc<str> = Rc::<str>::from(v6.replace(&*v30, &*v31));
                                    let v33: Rc<str> = string_concat(&[&*v32, "/state/agile.spi"]);
                                    let v34: bool = std::path::Path::new(&*v33).is_file();
                                    let v86: Rc<str> = if v34 {
                                        let (mut v35, mut v36): (bool, Rc<str>) = match std::fs::read(&*v33) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                                        if v35 {
                                            let v37: Rc<str> = string_concat(&["rig state read failed: ", &*v33]);
                                            std::panic::panic_any::<std::string::String>(format!("{}", v37))
                                        } else {
                                            let v39: Rc<str> = Rc::<str>::from(v33.replace(&*v30, &*v31));
                                            let mut v40: Rc<str> = loop_8(v39);
                                            let v41: Option<usize> = v40.rfind(&*v31);
                                            let v42: Option<usize> = method9(v41);
                                            let mut v43: Rc<dyn Fn((usize)) -> i32> = closure1();
                                            let mut v44: Option<i32> = v42.map(|x| v43(x));
                                            let v46: i32 = v44.unwrap_or(-1i32);
                                            let v47: bool = v46 < 0i32;
                                            let v51: bool = if v47 {
                                                true
                                            } else {
                                                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                let v49: bool = v40.ends_with(&*v48);
                                                let v50: bool = v49 == false;
                                                v50
                                            };
                                            let v72: US1 = if v51 {
                                                US1::US1_OtherFile
                                            } else {
                                                let v53: usize = ((0i32) as usize);
                                                let v54: usize = ((v46) as usize);
                                                let v55: Rc<str> = Rc::<str>::from(&(v40)[v53..v54]);
                                                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                let v57: bool = v55.ends_with(&*v56);
                                                if v57 {
                                                    let v58: Rc<str> = loop_10(v55);
                                                    US1::US1_StateFile(v58)
                                                } else {
                                                    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                    let v61: bool = v55.ends_with(&*v60);
                                                    if v61 {
                                                        let v62: Rc<str> = loop_11(v55);
                                                        US1::US1_HistoryFile(v62)
                                                    } else {
                                                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                        let v65: bool = v55.ends_with(&*v64);
                                                        if v65 {
                                                            let v66: Rc<str> = loop_12(v55);
                                                            US1::US1_ArchiveFile(v66)
                                                        } else {
                                                            US1::US1_OtherFile
                                                        }
                                                    }
                                                }
                                            };
                                            match v72 {
                                                US1::US1_OtherFile => {
                                                    v36
                                                }
                                                _ => {
                                                    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                    let v74: Rc<UH0> = split_list_13(v73.clone(), v36);
                                                    let v75: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                    let v76: Rc<UH0> = method16(v74, v75);
                                                    let v77: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                    let v78: i32 = 0i32;
                                                    let v79: i32 = method17(v77.clone(), v76, v78);
                                                    let v80: Rc<Vec<Rc<str>>> = Rc::new(v77.borrow().clone());
                                                    let v81: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v80).as_ref().clone()));
                                                    let v82: Rc<str> = Rc::<str>::from((*v81.borrow()).join(&*v73));
                                                    v82
                                                }
                                            }
                                        }
                                    } else {
                                        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v85
                                    };
                                    let v87: Rc<str> = Rc::<str>::from(v6.replace(&*v30, &*v31));
                                    let v88: Rc<str> = string_concat(&[&*v87, "/state/waits.spi"]);
                                    let v89: bool = std::path::Path::new(&*v88).is_file();
                                    let v141: Rc<str> = if v89 {
                                        let (mut v90, mut v91): (bool, Rc<str>) = match std::fs::read(&*v88) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                                        if v90 {
                                            let v92: Rc<str> = string_concat(&["rig state read failed: ", &*v88]);
                                            std::panic::panic_any::<std::string::String>(format!("{}", v92))
                                        } else {
                                            let v94: Rc<str> = Rc::<str>::from(v88.replace(&*v30, &*v31));
                                            let mut v95: Rc<str> = loop_8(v94);
                                            let v96: Option<usize> = v95.rfind(&*v31);
                                            let v97: Option<usize> = method9(v96);
                                            let mut v98: Rc<dyn Fn((usize)) -> i32> = closure1();
                                            let mut v99: Option<i32> = v97.map(|x| v98(x));
                                            let v101: i32 = v99.unwrap_or(-1i32);
                                            let v102: bool = v101 < 0i32;
                                            let v106: bool = if v102 {
                                                true
                                            } else {
                                                let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                let v104: bool = v95.ends_with(&*v103);
                                                let v105: bool = v104 == false;
                                                v105
                                            };
                                            let v127: US1 = if v106 {
                                                US1::US1_OtherFile
                                            } else {
                                                let v108: usize = ((0i32) as usize);
                                                let v109: usize = ((v101) as usize);
                                                let v110: Rc<str> = Rc::<str>::from(&(v95)[v108..v109]);
                                                let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                let v112: bool = v110.ends_with(&*v111);
                                                if v112 {
                                                    let v113: Rc<str> = loop_10(v110);
                                                    US1::US1_StateFile(v113)
                                                } else {
                                                    let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                    let v116: bool = v110.ends_with(&*v115);
                                                    if v116 {
                                                        let v117: Rc<str> = loop_11(v110);
                                                        US1::US1_HistoryFile(v117)
                                                    } else {
                                                        let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                        let v120: bool = v110.ends_with(&*v119);
                                                        if v120 {
                                                            let v121: Rc<str> = loop_12(v110);
                                                            US1::US1_ArchiveFile(v121)
                                                        } else {
                                                            US1::US1_OtherFile
                                                        }
                                                    }
                                                }
                                            };
                                            match v127 {
                                                US1::US1_OtherFile => {
                                                    v91
                                                }
                                                _ => {
                                                    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                    let v129: Rc<UH0> = split_list_13(v128.clone(), v91);
                                                    let v130: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                    let v131: Rc<UH0> = method16(v129, v130);
                                                    let v132: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                    let v133: i32 = 0i32;
                                                    let v134: i32 = method17(v132.clone(), v131, v133);
                                                    let v135: Rc<Vec<Rc<str>>> = Rc::new(v132.borrow().clone());
                                                    let v136: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v135).as_ref().clone()));
                                                    let v137: Rc<str> = Rc::<str>::from((*v136.borrow()).join(&*v128));
                                                    v137
                                                }
                                            }
                                        }
                                    } else {
                                        let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v140
                                    };
                                    let v142: Rc<UH0> = install_waiting_18(v86);
                                    let v143: bool = false;
                                    let v144: Rc<UH0> = ids_where_23(&v18, v143, v20);
                                    let v145: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v146: Rc<UH0> = method24(v141, v144, v145);
                                    let v147: Rc<UH0> = method22(v142, v146);
                                    let v148: bool = false;
                                    let v149: Rc<UH0> = ids_where_23(&v18, v148, v20);
                                    let v150: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v151: Rc<UH0> = method25(v147.clone(), v149.clone(), v150);
                                    let v152: i32 = 3i32;
                                    let v153: Rc<UH0> = taken_27(v152, v151);
                                    let v154: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v155: Rc<UH0> = method28(v147, v149, v154);
                                    let v156: Rc<UH0> = gate_lines_29(v11, v12, v13, v153, v29, v155);
                                    method36(v156);
                                    0i32
                                }
                            } else {
                                let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                                eprintln!("{}", v158);
                                2i32
                            }
                        }
                        _ => {
                            let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v160);
                            2i32
                        }
                    }
                }
                _ => {
                    let v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v162);
                    2i32
                }
            }
        }
        _ => {
            let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v164);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        gate_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_gate_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
