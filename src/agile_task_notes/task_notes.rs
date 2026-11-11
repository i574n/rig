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
enum US4 {
    US4_AuthorDefault,
    US4_AuthorNamed(Rc<str>),
    US4_AuthorUnnamedAgent,
    US4_AuthorMalformed,
}
#[derive(Clone)]
enum US5 {
    US5_Some(Rc<str>),
    US5_None,
}
#[derive(Clone)]
enum US6 {
    US6_GroomedText(Rc<str>),
    US6_GroomedRefused(Rc<str>),
}
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn loop_3(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
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
fn loop_6(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
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
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
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
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
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
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method11(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let v15: US2 = if v6 {
                let v7: US3 = US3::US3_HeadType;
                US2::US2_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US3 = US3::US3_HeadType;
                    US2::US2_SymbolHead(v11)
                } else {
                    US2::US2_OtherLine
                }
            };
            let mut v27: US2 = match &v15 {
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
fn method12(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn declared_from_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
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
fn note_schema_ensured_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note ="); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = v0.find(&*v1).map(|offset| offset as i32).unwrap_or(-1);
    let v3: bool = v2 < 0i32;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-> task_planning"); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v0.find(&*v4).map(|offset| offset as i32).unwrap_or(-1);
        let v6: bool = v5 < 0i32;
        let mut v9: i32 = if v6 {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-> agile_item"); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v0.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
            v8
        } else {
            v5
        };
        let v10: bool = v9 < 0i32;
        if v10 {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note =\n    | Note :: string * u32 * string * string * string -> task_note"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_2(v11, v12, v0, v13.clone(), v13.clone())
        } else {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v16: i32 = v0.get(v9 as usize..).and_then(|rest| rest.find(&*v15)).map(|found| (found + v9 as usize) as i32).unwrap_or(-1);
            let v17: bool = v16 < 0i32;
            let v19: i32 = if v17 {
                let v18: i32 = (v0.clone().len() as i32);
                v18
            } else {
                v16
            };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note =\n    | Note :: string * u32 * string * string * string -> task_note"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = concat5_2(v20, v21, v22.clone(), v22.clone(), v22.clone());
            let v24: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v19 as usize).unwrap_or(""));
            let v25: i32 = (v0.clone().len() as i32);
            let mut v26: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v25 as usize).unwrap_or(""));
            concat5_2(v24, v23, v26, v22.clone(), v22.clone())
        }
    } else {
        v0
    }
}
fn declared_count_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_13(v0.clone(), v1.clone(), v2);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let v6: i32 = v4.wrapping_add(1i32);
            let v7: i32 = v3.wrapping_add(1i32);
            (v2, v3) = (v6, v7);
            continue;
        }
    }
}
fn civil_of_days_17(mut v0: u64) -> (u64, u64, u64) {
    let mut v1: u64 = v0.wrapping_add(719468u64);
    let mut v2: u64 = v1.wrapping_div(146097u64);
    let v3: u64 = v2.wrapping_mul(146097u64);
    let v4: u64 = v1.wrapping_sub(v3);
    let v5: u64 = v4.wrapping_div(1460u64);
    let v6: u64 = v4.wrapping_sub(v5);
    let v7: u64 = v4.wrapping_div(36524u64);
    let v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let v10: u64 = v8.wrapping_sub(v9);
    let mut v11: u64 = v10.wrapping_div(365u64);
    let v12: u64 = 365u64.wrapping_mul(v11);
    let v13: u64 = v11.wrapping_div(4u64);
    let v14: u64 = v12.wrapping_add(v13);
    let v15: u64 = v11.wrapping_div(100u64);
    let v16: u64 = v14.wrapping_sub(v15);
    let v17: u64 = v4.wrapping_sub(v16);
    let v18: u64 = 5u64.wrapping_mul(v17);
    let v19: u64 = v18.wrapping_add(2u64);
    let v20: u64 = v19.wrapping_div(153u64);
    let v21: u64 = 153u64.wrapping_mul(v20);
    let v22: u64 = v21.wrapping_add(2u64);
    let mut v23: u64 = v22.wrapping_div(5u64);
    let v24: u64 = v17.wrapping_sub(v23);
    let v25: u64 = v24.wrapping_add(1u64);
    let mut v26: bool = v20 < 10u64;
    let v29: u64 = if v26 {
        let mut v27: u64 = v20.wrapping_add(3u64);
        v27
    } else {
        let v28: u64 = v20.wrapping_sub(9u64);
        v28
    };
    let v30: u64 = v2.wrapping_mul(400u64);
    let v31: u64 = v11.wrapping_add(v30);
    let v32: bool = v29 < 3u64;
    let v33: u64 = if v32 {
        1u64
    } else {
        0u64
    };
    let v34: u64 = v31.wrapping_add(v33);
    (v34, v29, v25)
}
fn utc_stamp_16(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(1000u64);
    let mut v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_17(v2);
    let v6: u64 = v1.wrapping_rem(86400u64);
    let v7: u64 = v6.wrapping_div(60u64);
    let v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v4 < 10u64;
    let v16: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v11, v12, v13.clone(), v13.clone(), v13)
    } else {
        let v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15
    };
    let v17: bool = v5 < 10u64;
    let mut v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v18, v19, v20.clone(), v20.clone(), v20)
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22
    };
    let v24: Rc<str> = concat5_2(v8, v9.clone(), v16, v9, v23);
    let v25: u64 = v7.wrapping_div(60u64);
    let mut v26: bool = v25 < 10u64;
    let v32: Rc<str> = if v26 {
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v27, v28, v29.clone(), v29.clone(), v29)
    } else {
        let v31: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        v31
    };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v34: u64 = v7.wrapping_rem(60u64);
    let v35: bool = v34 < 10u64;
    let v41: Rc<str> = if v35 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v36, v37, v38.clone(), v38.clone(), v38)
    } else {
        let v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = concat5_2(v32, v33, v41, v42, v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    concat5_2(v24, v45, v44, v43.clone(), v43)
}
fn note_row_18(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v8));
    let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = concat5_2(v6, v10, v8, v11, v12.clone());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note ("); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(v0.replace(&*v16, &*v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(v18.replace(&*v15, &*v19));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v22));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = Rc::<str>::from(v23.replace(&*v24, &*v25));
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = Rc::<str>::from(v26.replace(&*v27, &*v28));
    let v30: Rc<str> = concat5_2(v15.clone(), v29, v15.clone(), v12.clone(), v12.clone());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = concat5_2(v5, v13, v14, v30, v31.clone());
    let v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = Rc::<str>::from(v2.replace(&*v16, &*v17));
    let v36: Rc<str> = Rc::<str>::from(v35.replace(&*v15, &*v19));
    let v37: Rc<str> = Rc::<str>::from(v36.replace(&*v21, &*v22));
    let v38: Rc<str> = Rc::<str>::from(v37.replace(&*v24, &*v25));
    let v39: Rc<str> = Rc::<str>::from(v38.replace(&*v27, &*v28));
    let v40: Rc<str> = concat5_2(v15.clone(), v39, v15.clone(), v12.clone(), v12.clone());
    let v41: Rc<str> = Rc::<str>::from(v3.replace(&*v16, &*v17));
    let v42: Rc<str> = Rc::<str>::from(v41.replace(&*v15, &*v19));
    let v43: Rc<str> = Rc::<str>::from(v42.replace(&*v21, &*v22));
    let v44: Rc<str> = Rc::<str>::from(v43.replace(&*v24, &*v25));
    let v45: Rc<str> = Rc::<str>::from(v44.replace(&*v27, &*v28));
    let v46: Rc<str> = concat5_2(v15.clone(), v45, v15.clone(), v12.clone(), v12.clone());
    let v47: Rc<str> = concat5_2(v33, v34, v40, v31.clone(), v46);
    let v48: Rc<str> = Rc::<str>::from(v4.replace(&*v16, &*v17));
    let v49: Rc<str> = Rc::<str>::from(v48.replace(&*v15, &*v19));
    let v50: Rc<str> = Rc::<str>::from(v49.replace(&*v21, &*v22));
    let v51: Rc<str> = Rc::<str>::from(v50.replace(&*v24, &*v25));
    let v52: Rc<str> = Rc::<str>::from(v51.replace(&*v27, &*v28));
    let v53: Rc<str> = concat5_2(v15.clone(), v52, v15, v12.clone(), v12);
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    concat5_2(v32, v47, v31, v53, v54)
}
fn note_added_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = concat5_2(v0.clone(), v5, v6.clone(), v6.clone(), v6.clone());
    let v8: bool = std::path::Path::new(&*v7).is_file();
    let mut v11: US0 = if v8 {
        US0::US0_FileReadable(v7.clone())
    } else {
        US0::US0_FileMissing(v7.clone())
    };
    match v11 {
        US0::US0_FileMissing(v12) => {
            let v12: Rc<str> = v12;
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = string_concat(&["agile note ", &*v1, " rejected: ", &*v13]);
            let v17: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v16 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v17
        }
        US0::US0_FileReadable(v18) => {
            let v18: Rc<str> = v18;
            let v19: Rc<str> = std::fs::read_to_string(&*v18).unwrap_or_default().into();
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = Rc::<str>::from(v18.replace(&*v20, &*v21));
            let mut v23: Rc<str> = loop_3(v22);
            let v24: Option<usize> = v23.rfind(&*v21);
            let v25: Option<usize> = method4(v24);
            let mut v26: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v27: Option<i32> = v25.map(|x| v26(x));
            let v29: i32 = v27.unwrap_or(-1i32);
            let v30: bool = v29 < 0i32;
            let v34: bool = if v30 {
                true
            } else {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v32: bool = v23.ends_with(&*v31);
                let v33: bool = v32 == false;
                v33
            };
            let v55: US1 = if v34 {
                US1::US1_OtherFile
            } else {
                let v36: usize = ((0i32) as usize);
                let v37: usize = ((v29) as usize);
                let v38: Rc<str> = Rc::<str>::from(&(v23)[v36..v37]);
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v40: bool = v38.ends_with(&*v39);
                if v40 {
                    let v41: Rc<str> = loop_5(v38);
                    US1::US1_StateFile(v41)
                } else {
                    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v44: bool = v38.ends_with(&*v43);
                    if v44 {
                        let v45: Rc<str> = loop_6(v38);
                        US1::US1_HistoryFile(v45)
                    } else {
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v48: bool = v38.ends_with(&*v47);
                        if v48 {
                            let v49: Rc<str> = loop_7(v38);
                            US1::US1_ArchiveFile(v49)
                        } else {
                            US1::US1_OtherFile
                        }
                    }
                }
            };
            let v66: Rc<str> = match v55 {
                US1::US1_OtherFile => {
                    v19
                }
                _ => {
                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v57: Rc<UH0> = split_list_8(v56.clone(), v19);
                    let v58: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v59: Rc<UH0> = method11(v57, v58);
                    let v60: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v61: i32 = 0i32;
                    let v62: i32 = method12(v60.clone(), v59, v61);
                    let v63: Rc<Vec<Rc<str>>> = Rc::new(v60.borrow().clone());
                    let v64: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v63).as_ref().clone()));
                    let v65: Rc<str> = Rc::<str>::from((*v64.borrow()).join(&*v56));
                    v65
                }
            };
            let v67: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v66);
            let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
            let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
            let v71: Rc<str> = concat5_2(v68, v69.clone(), v1.clone(), v70.clone(), v6.clone());
            let v72: i32 = 0i32;
            let v73: i32 = declared_from_13(v67.clone(), v71, v72);
            let v74: bool = v73 < 0i32;
            let v75: bool = v74 == false;
            if v75 {
                let v76: Rc<str> = note_schema_ensured_14(v67.clone());
                let v77: Rc<str> = string_concat(&[&*v0, "/state"]);
                let v78: Rc<str> = agile_note_listing::rig_agile_state_segments(&*v77);
                let v80: Rc<str> = string_concat(&[&*v76, "\n", &*v78]);
                let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
                let v82: Rc<str> = concat5_2(v81, v69.clone(), v1.clone(), v70, v6.clone());
                let v83: i32 = 0i32;
                let v84: i32 = 0i32;
                let v85: i32 = declared_count_15(v80, v82, v83, v84);
                let v86: i32 = v85.wrapping_add(1i32);
                let v87: Rc<str> = utc_stamp_16(v4);
                let v88: Rc<str> = note_row_18(v1.clone(), v86, v87.clone(), v2.clone(), v3.clone());
                let (mut v89, mut v90): (Rc<str>, u64) = agile_note_segments::rig_agile_note_persisted(&*v77, &*v7, &*v66, &*v76, &*v1, &*v88, &*v87);
                let v91: bool = &*v89 == "";
                if v91 {
                    let v92: u64 = v67.len() as u64;
                    let v93: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
                    let v96: Rc<str> = std::rc::Rc::<str>::from((v86).to_string());
                    let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                    let v101: Rc<str> = Rc::<str>::from(v2.replace(&*v20, &*v100));
                    let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                    let v103: Rc<str> = Rc::<str>::from(v101.replace(&*v69, &*v102));
                    let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                    let v106: Rc<str> = Rc::<str>::from(v103.replace(&*v104, &*v105));
                    let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                    let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                    let v109: Rc<str> = Rc::<str>::from(v106.replace(&*v107, &*v108));
                    let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                    let v112: Rc<str> = Rc::<str>::from(v109.replace(&*v110, &*v111));
                    let v113: Rc<str> = concat5_2(v69.clone(), v112, v69.clone(), v6.clone(), v6.clone());
                    let v116: i32 = (v3.len() as i32);
                    let v117: Rc<str> = std::rc::Rc::<str>::from((v116).to_string());
                    let v120: Rc<str> = std::rc::Rc::<str>::from((v92).to_string());
                    let v123: Rc<str> = std::rc::Rc::<str>::from((v90).to_string());
                    let v126: Rc<str> = std::rc::Rc::<str>::from((v93).to_string());
                    let v128: Rc<str> = string_concat(&["rig agile note ok id=", &*v1, " note=", &*v96, " at=", &*v87, " author=", &*v113, " text_bytes=", &*v117, " bytes_before=", &*v120, " bytes_after=", &*v123, " elapsed_ms=", &*v126]);
                    let v129: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v128.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v129
                } else {
                    let v132: Rc<str> = string_concat(&["agile note ", &*v1, " rejected: ", &*v89]);
                    let v133: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v132 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v133
                }
            } else {
                let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                let v138: Rc<str> = string_concat(&["agile note ", &*v1, " rejected: ", &*v135]);
                let v139: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v138 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v139
            }
        }
    }
}
fn method22(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method23(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn note_text_of_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method22(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method23(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let mut v9: bool = &*v8 == "-";
    let v21: Rc<str> = if v9 {
        let v10: std::io::Stdin = std::io::stdin();
        let mut v11: Result<std::string::String, std::io::Error> = std::io::read_to_string(v10);
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v14: &str = &*v12;
        let v16: std::string::String = String::from(v14);
        let v18: std::string::String = v11.unwrap_or(v16);
        let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
        v20
    } else {
        v0
    };
    let v22: i32 = (v21.clone().len() as i32);
    let mut v23: i32 = 0i32;
    let v24: i32 = method22(v21.clone(), v22, v23);
    let v25: i32 = v22.wrapping_sub(1i32);
    let mut v26: Rc<str> = string_slice(&v21, v24 as i64, v25 as i64);
    let mut v27: i32 = (v26.clone().len() as i32);
    let v28: i32 = method23(v26.clone(), v27);
    let v29: Rc<str> = string_slice(&v26, 0i32 as i64, v28 as i64);
    v29
}
fn note_request_from_20(mut v0: &Rc<UH0>) -> (Rc<str>, US4) {
    let (mut v212, mut v213): (Rc<str>, US4) = match &**v0 {
        UH0::UH0_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<UH0> = v2.clone();
            match Rc::unwrap_or_clone(v2) {
                UH0::UH0_Cons(v3, v4) => {
                    let v3: Rc<str> = v3;
                    let v4: Rc<UH0> = v4;
                    match Rc::unwrap_or_clone(v4) {
                        UH0::UH0_Cons(v5, v6) => {
                            let v5: Rc<str> = v5;
                            let v6: Rc<UH0> = v6;
                            match Rc::unwrap_or_clone(v6) {
                                UH0::UH0_Cons(v7, v8) => {
                                    let v7: Rc<str> = v7;
                                    let v8: Rc<UH0> = v8;
                                    match Rc::unwrap_or_clone(v8) {
                                        UH0::UH0_Cons(v9, v10) => {
                                            let mut v9: Rc<str> = v9;
                                            let v10: Rc<UH0> = v10;
                                            match &*v10 {
                                                UH0::UH0_Nil => {
                                                    let mut v11: Rc<str> = note_text_of_21(v9);
                                                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AUTHOR"); } LIT.with(|lit| lit.clone()) };
                                                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_"); } LIT.with(|lit| lit.clone()) };
                                                    let v14: bool = v12.starts_with(&*v13);
                                                    let v17: bool = if v14 {
                                                        true
                                                    } else {
                                                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RIG_"); } LIT.with(|lit| lit.clone()) };
                                                        let v16: bool = v12.starts_with(&*v15);
                                                        v16
                                                    };
                                                    let v31: Rc<str> = if v17 {
                                                        let v18: i32 = (v12.clone().len() as i32);
                                                        let v19: i32 = v18.wrapping_sub(4i32);
                                                        let v20: usize = ((4i32) as usize);
                                                        let v21: i32 = 4i32.wrapping_add(v19);
                                                        let v22: usize = ((v21) as usize);
                                                        let mut v23: Rc<str> = Rc::<str>::from(&(v12)[v20..v22]);
                                                        let v24: Rc<str> = string_concat(&["RIG_", &*v23]);
                                                        let v25: Rc<str> = std::env::var(&*v24).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                        let mut v26: bool = &*v25 != "";
                                                        if v26 {
                                                            v25
                                                        } else {
                                                            let mut v27: Rc<str> = string_concat(&["KIT_", &*v23]);
                                                            let v28: Rc<str> = std::env::var(&*v27).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                            v28
                                                        }
                                                    } else {
                                                        let v30: Rc<str> = std::env::var(&*v12).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                        v30
                                                    };
                                                    let v32: i32 = (v31.clone().len() as i32);
                                                    let v33: i32 = 0i32;
                                                    let v34: i32 = method22(v31.clone(), v32, v33);
                                                    let v35: i32 = v32.wrapping_sub(1i32);
                                                    let v36: Rc<str> = string_slice(&v31, v34 as i64, v35 as i64);
                                                    let v37: i32 = (v36.clone().len() as i32);
                                                    let v38: i32 = method23(v36.clone(), v37);
                                                    let v39: Rc<str> = string_slice(&v36, 0i32 as i64, v38 as i64);
                                                    let v40: bool = &*v39 == "";
                                                    let v41: bool = v40 == false;
                                                    let v131: US4 = if v41 {
                                                        let v42: bool = v12.starts_with(&*v13);
                                                        let v45: bool = if v42 {
                                                            true
                                                        } else {
                                                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RIG_"); } LIT.with(|lit| lit.clone()) };
                                                            let v44: bool = v12.starts_with(&*v43);
                                                            v44
                                                        };
                                                        let v59: Rc<str> = if v45 {
                                                            let v46: i32 = (v12.clone().len() as i32);
                                                            let v47: i32 = v46.wrapping_sub(4i32);
                                                            let v48: usize = ((4i32) as usize);
                                                            let v49: i32 = 4i32.wrapping_add(v47);
                                                            let v50: usize = ((v49) as usize);
                                                            let v51: Rc<str> = Rc::<str>::from(&(v12)[v48..v50]);
                                                            let v52: Rc<str> = string_concat(&["RIG_", &*v51]);
                                                            let v53: Rc<str> = std::env::var(&*v52).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                            let v54: bool = &*v53 != "";
                                                            if v54 {
                                                                v53
                                                            } else {
                                                                let v55: Rc<str> = string_concat(&["KIT_", &*v51]);
                                                                let v56: Rc<str> = std::env::var(&*v55).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                                v56
                                                            }
                                                        } else {
                                                            let v58: Rc<str> = std::env::var(&*v12).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                            v58
                                                        };
                                                        let v60: i32 = (v59.clone().len() as i32);
                                                        let v61: i32 = 0i32;
                                                        let v62: i32 = method22(v59.clone(), v60, v61);
                                                        let v63: i32 = v60.wrapping_sub(1i32);
                                                        let v64: Rc<str> = string_slice(&v59, v62 as i64, v63 as i64);
                                                        let v65: i32 = (v64.clone().len() as i32);
                                                        let v66: i32 = method23(v64.clone(), v65);
                                                        let v67: Rc<str> = string_slice(&v64, 0i32 as i64, v66 as i64);
                                                        US4::US4_AuthorNamed(v67)
                                                    } else {
                                                        let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("AI_AGENT"); } LIT.with(|lit| lit.clone()) };
                                                        let v70: bool = v69.starts_with(&*v13);
                                                        let v73: bool = if v70 {
                                                            true
                                                        } else {
                                                            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RIG_"); } LIT.with(|lit| lit.clone()) };
                                                            let v72: bool = v69.starts_with(&*v71);
                                                            v72
                                                        };
                                                        let v87: Rc<str> = if v73 {
                                                            let v74: i32 = (v69.clone().len() as i32);
                                                            let v75: i32 = v74.wrapping_sub(4i32);
                                                            let v76: usize = ((4i32) as usize);
                                                            let v77: i32 = 4i32.wrapping_add(v75);
                                                            let v78: usize = ((v77) as usize);
                                                            let v79: Rc<str> = Rc::<str>::from(&(v69)[v76..v78]);
                                                            let v80: Rc<str> = string_concat(&["RIG_", &*v79]);
                                                            let v81: Rc<str> = std::env::var(&*v80).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                            let v82: bool = &*v81 != "";
                                                            if v82 {
                                                                v81
                                                            } else {
                                                                let v83: Rc<str> = string_concat(&["KIT_", &*v79]);
                                                                let v84: Rc<str> = std::env::var(&*v83).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                                v84
                                                            }
                                                        } else {
                                                            let v86: Rc<str> = std::env::var(&*v69).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                            v86
                                                        };
                                                        let v88: i32 = (v87.clone().len() as i32);
                                                        let v89: i32 = 0i32;
                                                        let v90: i32 = method22(v87.clone(), v88, v89);
                                                        let v91: i32 = v88.wrapping_sub(1i32);
                                                        let v92: Rc<str> = string_slice(&v87, v90 as i64, v91 as i64);
                                                        let v93: i32 = (v92.clone().len() as i32);
                                                        let v94: i32 = method23(v92.clone(), v93);
                                                        let v95: Rc<str> = string_slice(&v92, 0i32 as i64, v94 as i64);
                                                        let v96: bool = &*v95 == "";
                                                        let v97: bool = v96 == false;
                                                        let v127: bool = if v97 {
                                                            true
                                                        } else {
                                                            let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CLAUDECODE"); } LIT.with(|lit| lit.clone()) };
                                                            let v99: bool = v98.starts_with(&*v13);
                                                            let v102: bool = if v99 {
                                                                true
                                                            } else {
                                                                let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RIG_"); } LIT.with(|lit| lit.clone()) };
                                                                let v101: bool = v98.starts_with(&*v100);
                                                                v101
                                                            };
                                                            let v116: Rc<str> = if v102 {
                                                                let v103: i32 = (v98.clone().len() as i32);
                                                                let v104: i32 = v103.wrapping_sub(4i32);
                                                                let v105: usize = ((4i32) as usize);
                                                                let v106: i32 = 4i32.wrapping_add(v104);
                                                                let v107: usize = ((v106) as usize);
                                                                let v108: Rc<str> = Rc::<str>::from(&(v98)[v105..v107]);
                                                                let v109: Rc<str> = string_concat(&["RIG_", &*v108]);
                                                                let v110: Rc<str> = std::env::var(&*v109).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                                let v111: bool = &*v110 != "";
                                                                if v111 {
                                                                    v110
                                                                } else {
                                                                    let v112: Rc<str> = string_concat(&["KIT_", &*v108]);
                                                                    let v113: Rc<str> = std::env::var(&*v112).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                                    v113
                                                                }
                                                            } else {
                                                                let v115: Rc<str> = std::env::var(&*v98).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                                v115
                                                            };
                                                            let v117: i32 = (v116.clone().len() as i32);
                                                            let v118: i32 = 0i32;
                                                            let v119: i32 = method22(v116.clone(), v117, v118);
                                                            let v120: i32 = v117.wrapping_sub(1i32);
                                                            let v121: Rc<str> = string_slice(&v116, v119 as i64, v120 as i64);
                                                            let v122: i32 = (v121.clone().len() as i32);
                                                            let v123: i32 = method23(v121.clone(), v122);
                                                            let v124: Rc<str> = string_slice(&v121, 0i32 as i64, v123 as i64);
                                                            let v125: bool = &*v124 == "";
                                                            let v126: bool = v125 == false;
                                                            v126
                                                        };
                                                        if v127 {
                                                            US4::US4_AuthorUnnamedAgent
                                                        } else {
                                                            US4::US4_AuthorDefault
                                                        }
                                                    };
                                                    (v11, v131)
                                                }
                                                _ => {
                                                    let (mut v190, mut v191): (Rc<str>, US4) = match Rc::unwrap_or_clone(v10) {
                                                        UH0::UH0_Cons(v132, v133) => {
                                                            let v132: Rc<str> = v132;
                                                            let v133: Rc<UH0> = v133;
                                                            match Rc::unwrap_or_clone(v133) {
                                                                UH0::UH0_Cons(v134, v135) => {
                                                                    let v134: Rc<str> = v134;
                                                                    let v135: Rc<UH0> = v135;
                                                                    match Rc::unwrap_or_clone(v135) {
                                                                        UH0::UH0_Nil => {
                                                                            let v136: bool = &*v132 == "--as";
                                                                            if v136 {
                                                                                let v137: Rc<str> = note_text_of_21(v9);
                                                                                let v138: i32 = (v134.clone().len() as i32);
                                                                                let v139: i32 = 0i32;
                                                                                let v140: i32 = method22(v134.clone(), v138, v139);
                                                                                let v141: i32 = v138.wrapping_sub(1i32);
                                                                                let v142: Rc<str> = string_slice(&v134.clone(), v140 as i64, v141 as i64);
                                                                                let v143: i32 = (v142.clone().len() as i32);
                                                                                let v144: i32 = method23(v142.clone(), v143);
                                                                                let v145: Rc<str> = string_slice(&v142, 0i32 as i64, v144 as i64);
                                                                                let v146: bool = &*v145 == "";
                                                                                let v155: US4 = if v146 {
                                                                                    US4::US4_AuthorMalformed
                                                                                } else {
                                                                                    let v148: i32 = 0i32;
                                                                                    let v149: i32 = method22(v134.clone(), v138, v148);
                                                                                    let v150: Rc<str> = string_slice(&v134, v149 as i64, v141 as i64);
                                                                                    let v151: i32 = (v150.clone().len() as i32);
                                                                                    let v152: i32 = method23(v150.clone(), v151);
                                                                                    let v153: Rc<str> = string_slice(&v150, 0i32 as i64, v152 as i64);
                                                                                    US4::US4_AuthorNamed(v153)
                                                                                };
                                                                                (v137, v155)
                                                                            } else {
                                                                                let v156: bool = &*v9 == "--as";
                                                                                let (mut v178, mut v179): (Rc<str>, US4) = if v156 {
                                                                                    let v157: Rc<str> = note_text_of_21(v134);
                                                                                    let v158: i32 = (v132.clone().len() as i32);
                                                                                    let v159: i32 = 0i32;
                                                                                    let v160: i32 = method22(v132.clone(), v158, v159);
                                                                                    let v161: i32 = v158.wrapping_sub(1i32);
                                                                                    let v162: Rc<str> = string_slice(&v132.clone(), v160 as i64, v161 as i64);
                                                                                    let v163: i32 = (v162.clone().len() as i32);
                                                                                    let v164: i32 = method23(v162.clone(), v163);
                                                                                    let v165: Rc<str> = string_slice(&v162, 0i32 as i64, v164 as i64);
                                                                                    let v166: bool = &*v165 == "";
                                                                                    let v175: US4 = if v166 {
                                                                                        US4::US4_AuthorMalformed
                                                                                    } else {
                                                                                        let v168: i32 = 0i32;
                                                                                        let v169: i32 = method22(v132.clone(), v158, v168);
                                                                                        let v170: Rc<str> = string_slice(&v132, v169 as i64, v161 as i64);
                                                                                        let v171: i32 = (v170.clone().len() as i32);
                                                                                        let v172: i32 = method23(v170.clone(), v171);
                                                                                        let v173: Rc<str> = string_slice(&v170, 0i32 as i64, v172 as i64);
                                                                                        US4::US4_AuthorNamed(v173)
                                                                                    };
                                                                                    (v157, v175)
                                                                                } else {
                                                                                    let v176: Rc<str> = note_text_of_21(v9);
                                                                                    let v177: US4 = US4::US4_AuthorMalformed;
                                                                                    (v176, v177)
                                                                                };
                                                                                (v178, v179)
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            let v182: Rc<str> = note_text_of_21(v9);
                                                                            let v183: US4 = US4::US4_AuthorMalformed;
                                                                            (v182, v183)
                                                                        }
                                                                    }
                                                                }
                                                                _ => {
                                                                    let v186: Rc<str> = note_text_of_21(v9);
                                                                    let v187: US4 = US4::US4_AuthorMalformed;
                                                                    (v186, v187)
                                                                }
                                                            }
                                                        }
                                                        _ => unreachable!(),
                                                    };
                                                    (v190, v191)
                                                }
                                            }
                                        }
                                        _ => {
                                            let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let v195: US4 = US4::US4_AuthorMalformed;
                                            (v194, v195)
                                        }
                                    }
                                }
                                _ => {
                                    let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v199: US4 = US4::US4_AuthorMalformed;
                                    (v198, v199)
                                }
                            }
                        }
                        _ => {
                            let v202: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v203: US4 = US4::US4_AuthorMalformed;
                            (v202, v203)
                        }
                    }
                }
                _ => {
                    let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v207: US4 = US4::US4_AuthorMalformed;
                    (v206, v207)
                }
            }
        }
        _ => {
            let v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v211: US4 = US4::US4_AuthorMalformed;
            (v210, v211)
        }
    };
    (v212, v213)
}
fn command_line_argument_count_25() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_26(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_24(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_25();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US5 = if v4 {
        let v5: Rc<str> = command_line_argument_26(v0);
        US5::US5_Some(v5)
    } else {
        US5::US5_None
    };
    match v8 {
        US5::US5_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US5::US5_Some(v10) => {
            let v10: Rc<str> = v10;
            let mut v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_24(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn note_request_of_19() -> (Rc<str>, US4) {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_24(v0);
    let (mut v2, mut v3): (Rc<str>, US4) = note_request_from_20(&v1);
    (v2, v3)
}
fn note_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let (mut v2, mut v3): (Rc<str>, US4) = note_request_of_19();
    let v4: bool = &*v2 == "";
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note text is empty"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = string_concat(&["agile note ", &*v1, " rejected: ", &*v5]);
        let mut v9: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v8 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v9
    } else {
        match v3 {
            US4::US4_AuthorDefault => {
                let v10: i32 = 1i32;
                let mut v11: Rc<str> = usize::try_from(v10).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user"); } LIT.with(|lit| lit.clone()) };
                note_added_1(v11, v1, v12, v2)
            }
            US4::US4_AuthorMalformed => {
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile note <root> <id> <text> [--as <author>]"); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = string_concat(&["agile note ", &*v1, " rejected: ", &*v23]);
                let mut v27: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v26 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v27
            }
            US4::US4_AuthorNamed(v14) => {
                let v14: Rc<str> = v14;
                let v15: i32 = 1i32;
                let v16: Rc<str> = usize::try_from(v15).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                note_added_1(v16, v1, v14, v2)
            }
            US4::US4_AuthorUnnamedAgent => {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("an agent session must name the note's author: agile note <root> <id> <text> --as <author> (or set KIT_AUTHOR)"); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = string_concat(&["agile note ", &*v1, " rejected: ", &*v18]);
                let v22: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v21 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v22
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        note_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planned_28(mut v0: US6, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US6 {
    match &v0 {
        US6::US6_GroomedRefused(v4) => {
            let v4: Rc<str> = v4.clone();
            v0
        }
        US6::US6_GroomedText(v5) => {
            let v5: Rc<str> = v5.clone();
            let v6: bool = &*v3 == "";
            if v6 {
                v0
            } else {
                let (mut v7, mut v8): (i32, Rc<str>) = agile_planning_values::rig_agile_planning_value_token(&*v2, &*v3);
                let mut v9: bool = v7 == 0i32;
                let v12: US6 = if v9 {
                    US6::US6_GroomedText(v8)
                } else {
                    US6::US6_GroomedRefused(v8)
                };
                match v12 {
                    US6::US6_GroomedRefused(v13) => {
                        let v13: Rc<str> = v13;
                        US6::US6_GroomedRefused(v13)
                    }
                    US6::US6_GroomedText(v15) => {
                        let v15: Rc<str> = v15;
                        let (mut v16, mut v17): (i32, Rc<str>) = agile_planning_rewrite::rig_agile_planning_rewrite(&*v5, &*v1, &*v2, &*v15);
                        let v18: bool = v16 == 0i32;
                        if v18 {
                            US6::US6_GroomedText(v17)
                        } else {
                            US6::US6_GroomedRefused(v17)
                        }
                    }
                }
            }
        }
    }
}
fn last_declared_31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_13(v0.clone(), v1.clone(), v2);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let v6: i32 = v4.wrapping_add(1i32);
            (v2, v3) = (v6, v4);
            continue;
        }
    }
}
fn notes_insertion_30(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_2(v2, v3.clone(), v1.clone(), v4.clone(), v5.clone());
    let v7: i32 = 0i32;
    let v8: i32 = declared_from_13(v0.clone(), v6, v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = concat5_2(v9, v3.clone(), v1.clone(), v4.clone(), v5.clone());
    let mut v11: i32 = 0i32;
    let v12: i32 = declared_from_13(v0.clone(), v10, v11);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = concat5_2(v13, v3, v1, v4, v5);
    let v15: i32 = 0i32;
    let v16: i32 = -1i32;
    let v17: i32 = last_declared_31(v0.clone(), v14, v15, v16);
    let v18: bool = v12 <= v17;
    let v19: i32 = if v18 {
        v17
    } else {
        v12
    };
    let v20: bool = v8 <= v19;
    let v21: i32 = if v20 {
        v19
    } else {
        v8
    };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v23: i32 = v0.get(v21 as usize..).and_then(|rest| rest.find(&*v22)).map(|found| (found + v21 as usize) as i32).unwrap_or(-1);
    let v24: bool = v23 < 0i32;
    if v24 {
        let v25: i32 = (v0.len() as i32);
        v25
    } else {
        v23
    }
}
fn notes_appended_29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: i32 = notes_insertion_30(v0.clone(), v1);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_2(v4, v2, v5.clone(), v5.clone(), v5.clone());
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v3 as usize).unwrap_or(""));
    let v8: i32 = (v0.clone().len() as i32);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v3 as usize..v8 as usize).unwrap_or(""));
    concat5_2(v7, v6, v9, v5.clone(), v5)
}
fn groomed_text_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> (i32, Rc<str>) {
    let v7: US6 = US6::US6_GroomedText(v0);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate"); } LIT.with(|lit| lit.clone()) };
    let mut v9: US6 = planned_28(v7, v1.clone(), v8, v2);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load"); } LIT.with(|lit| lit.clone()) };
    let mut v11: US6 = planned_28(v9, v1.clone(), v10, v3);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("assignee"); } LIT.with(|lit| lit.clone()) };
    let v13: US6 = planned_28(v11, v1.clone(), v12, v4);
    match v13 {
        US6::US6_GroomedRefused(v14) => {
            let v14: Rc<str> = v14;
            (1i32, v14)
        }
        US6::US6_GroomedText(v15) => {
            let v15: Rc<str> = v15;
            let v16: bool = &*v5 == "";
            if v16 {
                (0i32, v15)
            } else {
                let v17: Rc<str> = note_schema_ensured_14(v15);
                let v18: i32 = 1i32;
                let v19: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                let v20: Rc<str> = utc_stamp_16(v19);
                let v21: Rc<str> = note_row_18(v1.clone(), v18, v20, v6, v5);
                let v22: Rc<str> = notes_appended_29(v17, v1, v21);
                (0i32, v22)
            }
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>| -> (i32, Rc<str>) {
        groomed_text_27(v0, v1, v2, v3, v4, v5, v6)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64) -> Rc<str>> = Rc::new(move |mut v0: u64| -> Rc<str> {
        utc_stamp_16(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_note_run() -> i32 {
    closure0()()
}
pub fn rig_agile_groomed(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str, v5: &str, v6: &str) -> (i32, Rc<str>) {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), Rc::<str>::from(v5), Rc::<str>::from(v6))
}
pub fn rig_utc_stamp(v0: u64) -> Rc<str> {
    closure3()(v0)
}
pub type SpiralShape_652accfc6a8d = Mut0;
