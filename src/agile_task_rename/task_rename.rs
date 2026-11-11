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
    US4_ParentsFile,
    US4_ClaimsFile,
    US4_WaitsFile,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US4, Rc<UH1>),
}
#[derive(Clone)]
enum US5 {
    US5_Untouched,
    US5_Pointing(Rc<str>),
    US5_Owned(Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_StateWritten,
    US6_StateChangedSinceRead,
    US6_StateWriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US7 {
    US7_WordsRoot,
    US7_SymbolsRoot,
}
#[derive(Clone)]
enum US8 {
    US8_Replaced,
    US8_TemporaryWriteFailed,
    US8_RenameFailed,
    US8_RenameFailedTemporaryKept,
    US8_ReadbackMismatch,
    US8_ChangedSinceRead,
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
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn loop_6(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
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
            let mut v4: Rc<UH0> = method11(v3, v1);
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
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let mut v18: US3 = US3::US3_HeadInl;
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
fn method12(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
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
fn declared_from_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
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
fn binding_declared_14(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v3: Rc<str> = string_concat(&["\ninl ", &*v1, " ("]);
    let v5: Rc<str> = string_concat(&["inl ", &*v1, " ("]);
    let v6: bool = v0.starts_with(&*v5);
    if v6 {
        true
    } else {
        let v7: i32 = v0.find(&*v3).map(|offset| offset as i32).unwrap_or(-1);
        let v8: bool = v7 < 0i32;
        let mut v9: bool = v8 == false;
        v9
    }
}
fn file_name_19(mut v0: US4) -> Rc<str> {
    match v0 {
        US4::US4_ClaimsFile => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claims.spi"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US4::US4_ParentsFile => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("parents.spi"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US4::US4_WaitsFile => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waits.spi"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    }
}
fn side_path_18(mut v0: Rc<str>, mut v1: US4) -> Rc<str> {
    let mut v3: Rc<str> = file_name_19(v1);
    let mut v4: Rc<str> = string_concat(&[&*v0, "/state/", &*v3]);
    v4
}
fn quoted_23(mut v0: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = string_concat(&["\"", &*v0, "\""]);
    v2
}
fn binding_prefix_24(mut v0: US4) -> Rc<str> {
    match v0 {
        US4::US4_ClaimsFile => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl claim_"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US4::US4_ParentsFile => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl parent_of_"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US4::US4_WaitsFile => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl wait_of_"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    }
}
fn slug_of_25(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    v4
}
fn method28(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method28(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method27(mut v0: Rc<UH0>, mut v1: i32, mut v2: Rc<UH0>) -> (i32, Rc<UH0>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let v5: i32 = v1.wrapping_add(1i32);
                let v6: i32 = v1.wrapping_rem(2i32);
                let v7: bool = v6 == 1i32;
                let v11: Rc<UH0> = if v7 {
                    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v8));
                    method28(v2, v9)
                } else {
                    v2
                };
                (v0, v1, v2) = (v4, v5, v11);
                continue;
            }
            UH0::UH0_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn owned_by_26(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<UH0> = split_list_8(v2, v0);
    let mut v4: i32 = 0i32;
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v6, mut v7): (i32, Rc<UH0>) = method27(v3, v4, v5);
    match Rc::unwrap_or_clone(v7) {
        UH0::UH0_Cons(v8, v9) => {
            let v8: Rc<str> = v8;
            let mut v9: Rc<UH0> = v9;
            let v10: bool = &*v8 == &*v1;
            v10
        }
        UH0::UH0_Nil => {
            false
        }
    }
}
fn renamed_row_22(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US5 {
    let mut v4: Rc<str> = quoted_23(v1.clone());
    let v5: Rc<str> = quoted_23(v2.clone());
    let v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let v7: Rc<str> = binding_prefix_24(v0.clone());
    let v8: Rc<str> = slug_of_25(v1.clone());
    let mut v9: Rc<str> = string_concat(&[&*v7, &*v8]);
    let v10: bool = owned_by_26(v3.clone(), v1);
    let mut v12: bool = if v10 {
        let v11: bool = v6.starts_with(&*v9);
        v11
    } else {
        false
    };
    if v12 {
        let mut v13: i32 = (v9.len() as i32);
        let v14: i32 = (v6.clone().len() as i32);
        let v15: i32 = v14.wrapping_sub(v13);
        let v16: usize = ((v13) as usize);
        let v17: i32 = v13.wrapping_add(v15);
        let mut v18: usize = ((v17) as usize);
        let mut v19: Rc<str> = Rc::<str>::from(&(v6)[v16..v18]);
        let v20: Rc<str> = binding_prefix_24(v0);
        let v21: Rc<str> = slug_of_25(v2);
        let v23: Rc<str> = string_concat(&[&*v20, &*v21, &*v19]);
        US5::US5_Owned(v23)
    } else {
        let mut v25: bool = &*v6 != &*v3;
        if v25 {
            US5::US5_Pointing(v6)
        } else {
            US5::US5_Untouched
        }
    }
}
fn line_after_29(mut v0: Rc<str>, mut v1: US5) -> Rc<str> {
    match v1 {
        US5::US5_Owned(v3) => {
            let mut v3: Rc<str> = v3;
            v3
        }
        US5::US5_Pointing(v2) => {
            let mut v2: Rc<str> = v2;
            v2
        }
        US5::US5_Untouched => {
            v0
        }
    }
}
fn counted_30(mut v0: bool, mut v1: i32) -> i32 {
    if v0 {
        let mut v2: i32 = v1.wrapping_add(1i32);
        v2
    } else {
        v1
    }
}
fn owns_binding_31(mut v0: US5) -> bool {
    match v0 {
        US5::US5_Owned(v1) => {
            let mut v1: Rc<str> = v1;
            true
        }
        _ => {
            false
        }
    }
}
fn method21(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: Rc<UH0>, mut v5: i32, mut v6: i32) -> (Rc<UH0>, i32, i32) {
    loop {
        match &*v3 {
            UH0::UH0_Cons(v7, v8) => {
                let v7: Rc<str> = v7.clone();
                let v8: Rc<UH0> = v8.clone();
                let mut v9: US5 = renamed_row_22(v0.clone(), v1.clone(), v2.clone(), v7.clone());
                let v10: Rc<str> = line_after_29(v7, v9.clone());
                let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v11));
                let mut v13: Rc<UH0> = method28(v4, v12);
                let v14: bool = match &v9 {
                    US5::US5_Untouched => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v15: bool = v14 != true;
                let v16: i32 = counted_30(v15, v5);
                let v17: bool = owns_binding_31(v9);
                let mut v18: i32 = counted_30(v17, v6);
                (v3, v4, v5, v6) = (v8, v13, v16, v18);
                continue;
            }
            _ => {
                let (mut v22, mut v23, mut v24): (Rc<UH0>, i32, i32) = match Rc::unwrap_or_clone(v3) {
                    UH0::UH0_Nil => {
                        (v4, v5, v6)
                    }
                    _ => unreachable!(),
                };
                return (v22, v23, v24);
            }
        }
    }
}
fn renamed_side_20(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, i32, i32) {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<UH0> = split_list_8(v4.clone(), v3);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: i32 = 0i32;
    let v8: i32 = 0i32;
    let (mut v9, mut v10, mut v11): (Rc<UH0>, i32, i32) = method21(v0, v1, v2, v5, v6, v7, v8);
    let mut v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v13: i32 = 0i32;
    let v14: i32 = method12(v12.clone(), v9, v13);
    let v15: Rc<Vec<Rc<str>>> = Rc::new(v12.borrow().clone());
    let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
    let v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v4));
    (v17, v10, v11)
}
fn method17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<dyn Fn(Rc<str>) -> Rc<str>>, mut v4: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>>>, mut v5: Rc<UH1>, mut v6: i32, mut v7: i32, mut v8: Rc<UH0>) -> (i32, i32, Rc<UH0>) {
    loop {
        match &*v5 {
            UH1::UH1_Cons(v9, v10) => {
                let mut v9: US4 = v9.clone();
                let v10: Rc<UH1> = v10.clone();
                let v11: Rc<str> = side_path_18(v0.clone(), v9.clone());
                let mut v12: Rc<str> = v3(v11.clone());
                let (mut v13, mut v14, mut v15): (Rc<str>, i32, i32) = renamed_side_20(v9.clone(), v1.clone(), v2.clone(), v12.clone());
                let v16: bool = v14 == 0i32;
                let v21: Rc<str> = if v16 {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v17
                } else {
                    let mut v18: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>> = v4(v11);
                    let mut v19: Rc<dyn Fn(Rc<str>) -> Rc<str>> = v18(v12);
                    v19(v13)
                };
                let mut v22: bool = &*v21 == "";
                let v29: Rc<UH0> = if v22 {
                    v8
                } else {
                    let v23: Rc<str> = file_name_19(v9);
                    let mut v25: Rc<str> = string_concat(&[&*v23, ": ", &*v21]);
                    let mut v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v27: Rc<UH0> = Rc::new(UH0::UH0_Cons(v25, v26));
                    method28(v8, v27)
                };
                let v30: i32 = v6.wrapping_add(v14);
                let v31: i32 = v7.wrapping_add(v15);
                (v5, v6, v7, v8) = (v10, v30, v31, v29);
                continue;
            }
            _ => {
                let (mut v35, mut v36, mut v37): (i32, i32, Rc<UH0>) = match Rc::unwrap_or_clone(v5) {
                    UH1::UH1_Nil => {
                        (v6, v7, v8)
                    }
                    _ => unreachable!(),
                };
                return (v35, v36, v37);
            }
        }
    }
}
fn side_files_32() -> Rc<UH1> {
    let mut v0: US4 = US4::US4_ParentsFile;
    let mut v1: US4 = US4::US4_ClaimsFile;
    let mut v2: US4 = US4::US4_WaitsFile;
    let mut v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3));
    let v5: Rc<UH1> = Rc::new(UH1::UH1_Cons(v1, v4));
    Rc::new(UH1::UH1_Cons(v0, v5))
}
fn renamed_sides_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<dyn Fn(Rc<str>) -> Rc<str>>, mut v4: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>>>) -> (i32, i32, Rc<UH0>) {
    let v5: Rc<UH1> = side_files_32();
    let v6: i32 = 0i32;
    let v7: i32 = 0i32;
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v9, mut v10, mut v11): (i32, i32, Rc<UH0>) = method17(v0, v1, v2, v3, v4, v5, v6, v7, v8);
    (v9, v10, v11)
}
fn id_rename_of_33(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    (v0, v1)
}
fn side_read_34(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default().as_str());
    v1
}
fn read_c2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        side_read_34(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method37(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method37(v3, v1);
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
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let mut v18: US3 = US3::US3_HeadInl;
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
                US2::US2_KeywordHead(v52) => {
                    let v52: US3 = v52;
                    let v59: Rc<str> = match &v52 {
                        US3::US3_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US3::US3_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US3::US3_HeadType => {
                            v5
                        }
                        US3::US3_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v60: i32 = (v59.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US3::US3_HeadInl => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v61
                        }
                        US3::US3_HeadOpen => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                        US3::US3_HeadType => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US3::US3_HeadUnion => {
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
fn renamed_within_38(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = std::fs::rename(&*v1, &*v0).is_ok();
        if v3 {
            return true;
        } else {
            let mut v4: bool = v2 <= 1i32;
            if v4 {
                return false;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(50u64));
                let v5: i32 = v2.wrapping_sub(1i32);
                v2 = v5;
                continue;
            }
        }
    }
}
fn state_written_36(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US6 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_2(v0.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    std::env::var("RIG_AGILE_TEST_PAUSE_MS").or_else(|_| std::env::var("KIT_AGILE_TEST_PAUSE_MS")).ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
    let mut v9: Rc<str> = loop_3(v8);
    let v10: Option<usize> = v9.rfind(&*v7);
    let v11: Option<usize> = method4(v10);
    let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v13: Option<i32> = v11.map(|x| v12(x));
    let v15: i32 = v13.unwrap_or(-1i32);
    let v16: bool = v15 < 0i32;
    let v20: bool = if v16 {
        true
    } else {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let mut v18: bool = v9.ends_with(&*v17);
        let mut v19: bool = v18 == false;
        v19
    };
    let v41: US1 = if v20 {
        US1::US1_OtherFile
    } else {
        let mut v22: usize = ((0i32) as usize);
        let v23: usize = ((v15) as usize);
        let v24: Rc<str> = Rc::<str>::from(&(v9)[v22..v23]);
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v26: bool = v24.ends_with(&*v25);
        if v26 {
            let v27: Rc<str> = loop_5(v24);
            US1::US1_StateFile(v27)
        } else {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
            let v30: bool = v24.ends_with(&*v29);
            if v30 {
                let v31: Rc<str> = loop_6(v24);
                US1::US1_HistoryFile(v31)
            } else {
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                let v34: bool = v24.ends_with(&*v33);
                if v34 {
                    let v35: Rc<str> = loop_7(v24);
                    US1::US1_ArchiveFile(v35)
                } else {
                    US1::US1_OtherFile
                }
            }
        }
    };
    let v64: US7 = match v41 {
        US1::US1_ArchiveFile(v54) => {
            let v54: Rc<str> = v54;
            let v55: Rc<str> = string_concat(&[&*v54, "/state/spiral.toml"]);
            let v56: bool = std::path::Path::new(&*v55).is_file();
            if v56 {
                US7::US7_SymbolsRoot
            } else {
                US7::US7_WordsRoot
            }
        }
        US1::US1_HistoryFile(v48) => {
            let v48: Rc<str> = v48;
            let v49: Rc<str> = string_concat(&[&*v48, "/state/spiral.toml"]);
            let v50: bool = std::path::Path::new(&*v49).is_file();
            if v50 {
                US7::US7_SymbolsRoot
            } else {
                US7::US7_WordsRoot
            }
        }
        US1::US1_OtherFile => {
            US7::US7_WordsRoot
        }
        US1::US1_StateFile(v42) => {
            let v42: Rc<str> = v42;
            let v43: Rc<str> = string_concat(&[&*v42, "/state/spiral.toml"]);
            let v44: bool = std::path::Path::new(&*v43).is_file();
            if v44 {
                US7::US7_SymbolsRoot
            } else {
                US7::US7_WordsRoot
            }
        }
    };
    let v76: Rc<str> = match v64 {
        US7::US7_SymbolsRoot => {
            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v66: Rc<UH0> = split_list_8(v65.clone(), v1);
            let v67: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v68: Rc<UH0> = method37(v66, v67);
            let v69: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v70: i32 = 0i32;
            let v71: i32 = method12(v69.clone(), v68, v70);
            let v72: Rc<Vec<Rc<str>>> = Rc::new(v69.borrow().clone());
            let v73: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v72).as_ref().clone()));
            let v74: Rc<str> = Rc::<str>::from((*v73.borrow()).join(&*v65));
            v74
        }
        US7::US7_WordsRoot => {
            v1
        }
    };
    let v77: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v76);
    if v77 {
        let v78: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let v79: Rc<str> = Rc::<str>::from(v78.replace(&*v6, &*v7));
        let mut v80: Rc<str> = loop_3(v79);
        let v81: Option<usize> = v80.rfind(&*v7);
        let v82: Option<usize> = method4(v81);
        let mut v83: Option<i32> = v82.map(|x| v12(x));
        let v85: i32 = v83.unwrap_or(-1i32);
        let v86: bool = v85 < 0i32;
        let v90: bool = if v86 {
            true
        } else {
            let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v88: bool = v80.ends_with(&*v87);
            let v89: bool = v88 == false;
            v89
        };
        let v111: US1 = if v90 {
            US1::US1_OtherFile
        } else {
            let v92: usize = ((0i32) as usize);
            let v93: usize = ((v85) as usize);
            let mut v94: Rc<str> = Rc::<str>::from(&(v80)[v92..v93]);
            let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v96: bool = v94.ends_with(&*v95);
            if v96 {
                let mut v97: Rc<str> = loop_5(v94);
                US1::US1_StateFile(v97)
            } else {
                let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v100: bool = v94.ends_with(&*v99);
                if v100 {
                    let v101: Rc<str> = loop_6(v94);
                    US1::US1_HistoryFile(v101)
                } else {
                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v104: bool = v94.ends_with(&*v103);
                    if v104 {
                        let v105: Rc<str> = loop_7(v94);
                        US1::US1_ArchiveFile(v105)
                    } else {
                        US1::US1_OtherFile
                    }
                }
            }
        };
        let v134: US7 = match v111 {
            US1::US1_ArchiveFile(v124) => {
                let v124: Rc<str> = v124;
                let v125: Rc<str> = string_concat(&[&*v124, "/state/spiral.toml"]);
                let v126: bool = std::path::Path::new(&*v125).is_file();
                if v126 {
                    US7::US7_SymbolsRoot
                } else {
                    US7::US7_WordsRoot
                }
            }
            US1::US1_HistoryFile(v118) => {
                let v118: Rc<str> = v118;
                let v119: Rc<str> = string_concat(&[&*v118, "/state/spiral.toml"]);
                let v120: bool = std::path::Path::new(&*v119).is_file();
                if v120 {
                    US7::US7_SymbolsRoot
                } else {
                    US7::US7_WordsRoot
                }
            }
            US1::US1_OtherFile => {
                US7::US7_WordsRoot
            }
            US1::US1_StateFile(v112) => {
                let v112: Rc<str> = v112;
                let v113: Rc<str> = string_concat(&[&*v112, "/state/spiral.toml"]);
                let v114: bool = std::path::Path::new(&*v113).is_file();
                if v114 {
                    US7::US7_SymbolsRoot
                } else {
                    US7::US7_WordsRoot
                }
            }
        };
        let v146: Rc<str> = match v134 {
            US7::US7_SymbolsRoot => {
                let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v136: Rc<UH0> = split_list_8(v135.clone(), v2.clone());
                let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v138: Rc<UH0> = method37(v136, v137);
                let v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v140: i32 = 0i32;
                let v141: i32 = method12(v139.clone(), v138, v140);
                let v142: Rc<Vec<Rc<str>>> = Rc::new(v139.borrow().clone());
                let v143: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v142).as_ref().clone()));
                let v144: Rc<str> = Rc::<str>::from((*v143.borrow()).join(&*v135));
                v144
            }
            US7::US7_WordsRoot => {
                v2.clone()
            }
        };
        let v147: bool = std::fs::write(&*v78, &*v146).is_ok();
        let v228: US8 = if v147 {
            let v148: i32 = 40i32;
            let v149: bool = renamed_within_38(v0.clone(), v78.clone(), v148);
            if v149 {
                let v150: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
                let mut v151: Rc<str> = loop_3(v150);
                let v152: Option<usize> = v151.rfind(&*v7);
                let v153: Option<usize> = method4(v152);
                let mut v154: Option<i32> = v153.map(|x| v12(x));
                let v156: i32 = v154.unwrap_or(-1i32);
                let v157: bool = v156 < 0i32;
                let v161: bool = if v157 {
                    true
                } else {
                    let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v159: bool = v151.ends_with(&*v158);
                    let v160: bool = v159 == false;
                    v160
                };
                let v182: US1 = if v161 {
                    US1::US1_OtherFile
                } else {
                    let v163: usize = ((0i32) as usize);
                    let v164: usize = ((v156) as usize);
                    let v165: Rc<str> = Rc::<str>::from(&(v151)[v163..v164]);
                    let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v167: bool = v165.ends_with(&*v166);
                    if v167 {
                        let v168: Rc<str> = loop_5(v165);
                        US1::US1_StateFile(v168)
                    } else {
                        let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v171: bool = v165.ends_with(&*v170);
                        if v171 {
                            let v172: Rc<str> = loop_6(v165);
                            US1::US1_HistoryFile(v172)
                        } else {
                            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v175: bool = v165.ends_with(&*v174);
                            if v175 {
                                let v176: Rc<str> = loop_7(v165);
                                US1::US1_ArchiveFile(v176)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                let v205: US7 = match v182 {
                    US1::US1_ArchiveFile(v195) => {
                        let v195: Rc<str> = v195;
                        let v196: Rc<str> = string_concat(&[&*v195, "/state/spiral.toml"]);
                        let v197: bool = std::path::Path::new(&*v196).is_file();
                        if v197 {
                            US7::US7_SymbolsRoot
                        } else {
                            US7::US7_WordsRoot
                        }
                    }
                    US1::US1_HistoryFile(v189) => {
                        let v189: Rc<str> = v189;
                        let v190: Rc<str> = string_concat(&[&*v189, "/state/spiral.toml"]);
                        let v191: bool = std::path::Path::new(&*v190).is_file();
                        if v191 {
                            US7::US7_SymbolsRoot
                        } else {
                            US7::US7_WordsRoot
                        }
                    }
                    US1::US1_OtherFile => {
                        US7::US7_WordsRoot
                    }
                    US1::US1_StateFile(v183) => {
                        let v183: Rc<str> = v183;
                        let v184: Rc<str> = string_concat(&[&*v183, "/state/spiral.toml"]);
                        let v185: bool = std::path::Path::new(&*v184).is_file();
                        if v185 {
                            US7::US7_SymbolsRoot
                        } else {
                            US7::US7_WordsRoot
                        }
                    }
                };
                let v217: Rc<str> = match v205 {
                    US7::US7_SymbolsRoot => {
                        let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v207: Rc<UH0> = split_list_8(v206.clone(), v2);
                        let v208: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v209: Rc<UH0> = method37(v207, v208);
                        let v210: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v211: i32 = 0i32;
                        let v212: i32 = method12(v210.clone(), v209, v211);
                        let v213: Rc<Vec<Rc<str>>> = Rc::new(v210.borrow().clone());
                        let v214: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v213).as_ref().clone()));
                        let v215: Rc<str> = Rc::<str>::from((*v214.borrow()).join(&*v206));
                        v215
                    }
                    US7::US7_WordsRoot => {
                        v2
                    }
                };
                let v218: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v217);
                if v218 {
                    US8::US8_Replaced
                } else {
                    US8::US8_ReadbackMismatch
                }
            } else {
                let v222: bool = std::fs::remove_file(&*v78).is_ok() || !std::path::Path::new(&*v78).exists();
                if v222 {
                    US8::US8_RenameFailed
                } else {
                    US8::US8_RenameFailedTemporaryKept
                }
            }
        } else {
            US8::US8_TemporaryWriteFailed
        };
        match &v228 {
            US8::US8_Replaced => {
                US6::US6_StateWritten
            }
            _ => {
                let v239: Rc<str> = match v228 {
                    US8::US8_ChangedSinceRead => {
                        let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v234
                    }
                    US8::US8_ReadbackMismatch => {
                        let v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v233
                    }
                    US8::US8_RenameFailed => {
                        let v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v231
                    }
                    US8::US8_RenameFailedTemporaryKept => {
                        let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v232
                    }
                    US8::US8_TemporaryWriteFailed => {
                        let v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v230
                    }
                    _ => unreachable!(),
                };
                US6::US6_StateWriteFailed(v239)
            }
        }
    } else {
        US6::US6_StateChangedSinceRead
    }
}
fn side_write_35(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: US6 = state_written_36(v0, v1, v2);
    match v3 {
        US6::US6_StateChangedSinceRead => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US6::US6_StateWriteFailed(v6) => {
            let v6: Rc<str> = v6;
            v6
        }
        US6::US6_StateWritten => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v4
        }
    }
}
fn closure5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    Rc::new(move |mut v2: Rc<str>| -> Rc<str> {
        side_write_35(v0.clone(), v1.clone(), v2)
    })
}
fn closure4(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>> {
    Rc::new(move |mut v1: Rc<str>| -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
        closure5(v0.clone(), v1)
    })
}
fn write_c3() -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>> {
        closure4(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn sides_renamed_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (i32, i32, Rc<UH0>) {
    let (mut v3, mut v4): (Rc<str>, Rc<str>) = id_rename_of_33(v1, v2);
    let v5: Rc<dyn Fn(Rc<str>) -> Rc<str>> = read_c2();
    let v6: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>>> = write_c3();
    let (mut v7, mut v8, mut v9): (i32, i32, Rc<UH0>) = renamed_sides_16(v0, v3, v4, v5, v6);
    (v7, v8, v9)
}
fn renamed_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let mut v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_2(v0.clone(), v4, v5.clone(), v5.clone(), v5.clone());
    let v7: bool = std::path::Path::new(&*v6).is_file();
    let v10: US0 = if v7 {
        US0::US0_FileReadable(v6.clone())
    } else {
        US0::US0_FileMissing(v6.clone())
    };
    match v10 {
        US0::US0_FileMissing(v11) => {
            let v11: Rc<str> = v11;
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v12]);
            let v16: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v15 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v16
        }
        US0::US0_FileReadable(v17) => {
            let v17: Rc<str> = v17;
            let mut v18: Rc<str> = std::fs::read_to_string(&*v17).unwrap_or_default().into();
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = Rc::<str>::from(v17.replace(&*v19, &*v20));
            let mut v22: Rc<str> = loop_3(v21);
            let v23: Option<usize> = v22.rfind(&*v20);
            let v24: Option<usize> = method4(v23);
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
                    let v40: Rc<str> = loop_5(v37);
                    US1::US1_StateFile(v40)
                } else {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v43: bool = v37.ends_with(&*v42);
                    if v43 {
                        let v44: Rc<str> = loop_6(v37);
                        US1::US1_HistoryFile(v44)
                    } else {
                        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v47: bool = v37.ends_with(&*v46);
                        if v47 {
                            let v48: Rc<str> = loop_7(v37);
                            US1::US1_ArchiveFile(v48)
                        } else {
                            US1::US1_OtherFile
                        }
                    }
                }
            };
            let v65: Rc<str> = match v54 {
                US1::US1_OtherFile => {
                    v18
                }
                _ => {
                    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v56: Rc<UH0> = split_list_8(v55.clone(), v18);
                    let v57: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v58: Rc<UH0> = method11(v56, v57);
                    let v59: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v60: i32 = 0i32;
                    let v61: i32 = method12(v59.clone(), v58, v60);
                    let v62: Rc<Vec<Rc<str>>> = Rc::new(v59.borrow().clone());
                    let v63: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v62).as_ref().clone()));
                    let v64: Rc<str> = Rc::<str>::from((*v63.borrow()).join(&*v55));
                    v64
                }
            };
            let v66: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v65);
            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
            let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
            let v70: Rc<str> = concat5_2(v67.clone(), v68.clone(), v1.clone(), v69.clone(), v5.clone());
            let v71: i32 = 0i32;
            let v72: i32 = declared_from_13(v66.clone(), v70, v71);
            let v73: bool = v72 < 0i32;
            let v74: bool = v73 == false;
            let v75: bool = v74 == false;
            if v75 {
                let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                let v79: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v76]);
                let mut v80: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v79 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v80
            } else {
                let v81: Rc<str> = concat5_2(v67, v68, v2.clone(), v69, v5.clone());
                let v82: i32 = 0i32;
                let mut v83: i32 = declared_from_13(v66.clone(), v81, v82);
                let v84: bool = v83 < 0i32;
                let v85: bool = v84 == false;
                if v85 {
                    let v90: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: the id ", &*v2, " is already taken"]);
                    let v91: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v90 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v91
                } else {
                    let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v94: Rc<str> = Rc::<str>::from(v2.replace(&*v92, &*v93));
                    let v95: Rc<str> = std::rc::Rc::<str>::from(v94.to_ascii_lowercase());
                    let v96: bool = binding_declared_14(v66.clone(), v95);
                    if v96 {
                        let mut v97: Rc<str> = Rc::<str>::from(v2.replace(&*v92, &*v93));
                        let v98: Rc<str> = std::rc::Rc::<str>::from(v97.to_ascii_lowercase());
                        let v103: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: the name inl ", &*v98, " is already declared"]);
                        let v104: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v103 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v104
                    } else {
                        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning_of_"); } LIT.with(|lit| lit.clone()) };
                        let mut v106: Rc<str> = Rc::<str>::from(v2.replace(&*v92, &*v93));
                        let v107: Rc<str> = std::rc::Rc::<str>::from(v106.to_ascii_lowercase());
                        let v108: Rc<str> = concat5_2(v105.clone(), v107, v5.clone(), v5.clone(), v5.clone());
                        let v109: bool = binding_declared_14(v66.clone(), v108);
                        if v109 {
                            let mut v110: Rc<str> = Rc::<str>::from(v2.replace(&*v92, &*v93));
                            let v111: Rc<str> = std::rc::Rc::<str>::from(v110.to_ascii_lowercase());
                            let v112: Rc<str> = concat5_2(v105, v111, v5.clone(), v5.clone(), v5.clone());
                            let v117: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: the name inl ", &*v112, " is already declared"]);
                            let v118: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v117 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v118
                        } else {
                            let (mut v119, mut v120, mut v121): (i32, i32, Rc<UH0>) = sides_renamed_15(v0, v1.clone(), v2.clone());
                            let v122: bool = match &*v121 {
                                UH0::UH0_Nil => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            };
                            let v123: bool = v122 != true;
                            if v123 {
                                let v124: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                let v125: i32 = 0i32;
                                let v126: i32 = method12(v124.clone(), v121, v125);
                                let v127: Rc<Vec<Rc<str>>> = Rc::new(v124.borrow().clone());
                                let v128: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v127).as_ref().clone()));
                                let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                                let v130: Rc<str> = Rc::<str>::from((*v128.borrow()).join(&*v129));
                                let v135: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: side files not finished (", &*v130, "); run it again"]);
                                let v136: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v135 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v136
                            } else {
                                let (mut v137, mut v138, mut v139, mut v140, mut v141, mut v142): (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) = agile_rename_rewrite::rig_agile_renamed_text(&*v66, &*v1, &*v2);
                                let v143: US6 = state_written_36(v6, v65, v137.clone());
                                match &v143 {
                                    US6::US6_StateWritten => {
                                        let v144: u64 = v66.len() as u64;
                                        let v145: u64 = v137.len() as u64;
                                        let v146: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
                                        let v150: Rc<str> = string_concat(&["rig agile rename ok old=", &*v1, " new=", &*v2, " items="]);
                                        let mut v151: Rc<str> = std::rc::Rc::<str>::from((v138).to_string());
                                        let v153: Rc<str> = std::rc::Rc::<str>::from((v139).to_string());
                                        let v155: Rc<str> = string_concat(&[&*v151, " deps_rewritten=", &*v153, " deps_in="]);
                                        let v156: bool = &*v140 == "";
                                        let v157: Rc<str> = if v156 {
                                            v92.clone()
                                        } else {
                                            v140
                                        };
                                        let v158: Rc<str> = string_concat(&[&*v155, &*v157]);
                                        let v159: Rc<str> = std::rc::Rc::<str>::from((v141).to_string());
                                        let v161: Rc<str> = string_concat(&[" titles_mentioning_old=", &*v159, " titles_in="]);
                                        let v162: bool = &*v142 == "";
                                        let v163: Rc<str> = if v162 {
                                            v92
                                        } else {
                                            v142
                                        };
                                        let v165: Rc<str> = std::rc::Rc::<str>::from((v119).to_string());
                                        let v168: Rc<str> = std::rc::Rc::<str>::from((v120).to_string());
                                        let v173: Rc<str> = std::rc::Rc::<str>::from((v144).to_string());
                                        let v176: Rc<str> = std::rc::Rc::<str>::from((v145).to_string());
                                        let v179: Rc<str> = std::rc::Rc::<str>::from((v146).to_string());
                                        let v181: Rc<str> = string_concat(&[&*v150, &*v158, &*v161, &*v163, " side_rows=", &*v165, " side_bindings=", &*v168, " bytes_before=", &*v173, " bytes_after=", &*v176, " elapsed_ms=", &*v179]);
                                        let v182: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v181.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                        v182
                                    }
                                    _ => {
                                        let v186: Rc<str> = match v143 {
                                            US6::US6_StateChangedSinceRead => {
                                                let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                                v183
                                            }
                                            US6::US6_StateWriteFailed(v184) => {
                                                let v184: Rc<str> = v184;
                                                v184
                                            }
                                            _ => unreachable!(),
                                        };
                                        let v189: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v186]);
                                        let v190: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v189 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                        v190
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn command_line_argument_count_40() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_39() -> i32 {
    let mut v0: i32 = command_line_argument_count_40();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn id_bytes_valid_41(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = v2 <= v1;
        if v3 {
            return true;
        } else {
            let mut v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v5: bool = 65i32 <= v4;
            let v11: bool = if v5 {
                let v6: bool = v4 <= 90i32;
                v6
            } else {
                let v7: bool = 48i32 <= v4;
                if v7 {
                    let v8: bool = v4 <= 57i32;
                    v8
                } else {
                    let mut v9: bool = v4 == 45i32;
                    v9
                }
            };
            if v11 {
                let mut v12: i32 = v1.wrapping_add(1i32);
                v1 = v12;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn rename_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 3i32;
    let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: Rc<str> = rig_agile_id_prefix::rig_agile_new_id_refusal(&v3);
    let v5: i32 = argument_count_39();
    let v6: bool = v5 == 4i32;
    let v7: bool = v6 == false;
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile rename <root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v8]);
        let mut v12: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v11 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v12
    } else {
        let mut v13: i32 = (v3.clone().len() as i32);
        let v14: bool = v13 <= 0i32;
        let v17: bool = if v14 {
            false
        } else {
            let v15: i32 = 0i32;
            id_bytes_valid_41(v3.clone(), v15, v13)
        };
        let mut v18: bool = v17 == false;
        if v18 {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task id is upper-case letters, digits and -"); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v19]);
            let v23: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v22 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v23
        } else {
            let v24: i32 = v3.as_bytes().get(0i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v25: bool = 48i32 <= v24;
            let v27: bool = if v25 {
                let mut v26: bool = v24 <= 57i32;
                v26
            } else {
                false
            };
            if v27 {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the new id must start with a letter, so its inl name is a valid identifier"); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v28]);
                let v32: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v31 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v32
            } else {
                let v33: bool = &*v1 == &*v3;
                if v33 {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the new id equals the old one"); } LIT.with(|lit| lit.clone()) };
                    let v37: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v34]);
                    let v38: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v37 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v38
                } else {
                    let v39: bool = &*v4 != "";
                    if v39 {
                        let v42: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v4]);
                        let v43: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v42 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v43
                    } else {
                        let v44: i32 = 1i32;
                        let v45: Rc<str> = usize::try_from(v44).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        renamed_1(v45, v1, v3)
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        rename_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_rename_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
