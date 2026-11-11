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
    US0_Some(i64),
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
    US4_ItemOpen,
    US4_ItemDone,
    US4_ItemUnknown,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, US4, Rc<UH1>),
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
enum US8 {
    US8_Some(US5),
    US8_None,
}
#[derive(Clone)]
enum US9 {
    US9_WordsRoot,
    US9_SymbolsRoot,
}
#[derive(Clone)]
enum US10 {
    US10_SpiprojProject,
    US10_TomlProject,
}
#[derive(Clone)]
enum US11 {
    US11_SplitProject(Rc<UH0>, Rc<UH0>, Rc<UH0>),
    US11_NoModules,
}
#[derive(Clone)]
enum US12 {
    US12_ModuleListed,
    US12_ModuleAdded(Rc<str>),
    US12_NoModulesSection,
}
#[derive(Clone)]
enum US13 {
    US13_Some(i32),
    US13_None,
}
fn method2(mut v0: Option<i64>) -> Option<i64> {
    v0
}
fn closure1() -> Rc<dyn Fn((i64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i64)) -> US0> = Rc::new(move |mut v0: (i64)| -> US0 {
        let mut v1: i64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_3(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
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
fn method4(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
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
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
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
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
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
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
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
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
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
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US2 = if v6 {
                let mut v7: US3 = US3::US3_HeadType;
                US2::US2_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US3 = US3::US3_HeadType;
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
                            let mut v34: US3 = US3::US3_HeadUnion;
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
fn method15(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_14(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn two_digits_16(mut v0: i64) -> Rc<str> {
    let mut v1: bool = v0 < 10i64;
    if v1 {
        let mut v2: Rc<str> = format_real_14(v0);
        let v3: Rc<str> = string_concat(&["0", &*v2]);
        v3
    } else {
        format_real_14(v0)
    }
}
fn civil_date_13(mut v0: i64) -> Rc<str> {
    let mut v1: i64 = v0.wrapping_add(719468i64);
    let mut v2: bool = v1 >= 0i64;
    let v4: i64 = if v2 {
        v1
    } else {
        let v3: i64 = v1.wrapping_sub(146096i64);
        v3
    };
    let v5: i64 = v4.wrapping_div(146097i64);
    let mut v6: i64 = v5.wrapping_mul(146097i64);
    let mut v7: i64 = v1.wrapping_sub(v6);
    let mut v8: i64 = v7.wrapping_div(1460i64);
    let mut v9: i64 = v7.wrapping_sub(v8);
    let mut v10: i64 = v7.wrapping_div(36524i64);
    let mut v11: i64 = v9.wrapping_add(v10);
    let v12: i64 = v7.wrapping_div(146096i64);
    let v13: i64 = v11.wrapping_sub(v12);
    let v14: i64 = v13.wrapping_div(365i64);
    let v15: i64 = 365i64.wrapping_mul(v14);
    let v16: i64 = v14.wrapping_div(4i64);
    let mut v17: i64 = v15.wrapping_add(v16);
    let v18: i64 = v14.wrapping_div(100i64);
    let v19: i64 = v17.wrapping_sub(v18);
    let v20: i64 = v7.wrapping_sub(v19);
    let v21: i64 = 5i64.wrapping_mul(v20);
    let v22: i64 = v21.wrapping_add(2i64);
    let v23: i64 = v22.wrapping_div(153i64);
    let v24: i64 = 153i64.wrapping_mul(v23);
    let v25: i64 = v24.wrapping_add(2i64);
    let v26: i64 = v25.wrapping_div(5i64);
    let v27: i64 = v20.wrapping_sub(v26);
    let v28: i64 = v27.wrapping_add(1i64);
    let v29: bool = v23 < 10i64;
    let v32: i64 = if v29 {
        let v30: i64 = v23.wrapping_add(3i64);
        v30
    } else {
        let mut v31: i64 = v23.wrapping_sub(9i64);
        v31
    };
    let v33: i64 = v5.wrapping_mul(400i64);
    let mut v34: i64 = v14.wrapping_add(v33);
    let mut v35: bool = v32 <= 2i64;
    let v36: i64 = if v35 {
        1i64
    } else {
        0i64
    };
    let v37: i64 = v34.wrapping_add(v36);
    let v38: Rc<str> = format_real_14(v37);
    let v40: Rc<str> = two_digits_16(v32);
    let v43: Rc<str> = two_digits_16(v28);
    let v44: Rc<str> = string_concat(&[&*v38, "-", &*v40, "-", &*v43]);
    v44
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method18(v3, v1);
            let v5: bool = &*v2 != "";
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
fn method20(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
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
fn method21(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
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
fn method19(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method19(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let mut v6: i32 = 0i32;
            let mut v7: i32 = method20(v2.clone(), v5, v6);
            let mut v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
            let mut v10: i32 = (v9.clone().len() as i32);
            let mut v11: i32 = method21(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            Rc::new(UH0::UH0_Cons(v12, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn items_given_17(mut v0: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<UH0> = v2;
                match &*v2 {
                    UH0::UH0_Cons(v3, v4) => {
                        let v3: Rc<str> = v3.clone();
                        let v4: Rc<UH0> = v4.clone();
                        let v5: bool = &*v1 == "--items";
                        if v5 {
                            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            let mut v7: Rc<UH0> = split_list_8(v6, v3);
                            let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let mut v9: Rc<UH0> = method19(v7, v8);
                            let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            return method18(v9, v10);
                        } else {
                            v0 = v2;
                            continue;
                        }
                    }
                    _ => {
                        v0 = v2;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn standing_of_24(mut v0: Rc<str>) -> US4 {
    let mut v1: bool = &*v0 == "";
    if v1 {
        US4::US4_ItemUnknown
    } else {
        let v3: bool = &*v0 == "Done";
        if v3 {
            US4::US4_ItemDone
        } else {
            US4::US4_ItemOpen
        }
    }
}
fn candidate_of_23(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, i32, US4) {
    let mut v2: Rc<str> = agile_task_census::rig_agile_task_status_of(&v0, &v1);
    let v3: i32 = agile_planning_fields::rig_agile_planning_minutes_of(&v0, &v1);
    let v4: US4 = standing_of_24(v2);
    (v1, v3, v4)
}
fn method22(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH1> = method22(v0.clone(), v4, v2);
            let (mut v6, mut v7, mut v8): (Rc<str>, i32, US4) = candidate_of_23(v0, v3);
            Rc::new(UH1::UH1_Cons(v6, v7, v8, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method26(mut v0: Rc<UH1>, mut v1: US8) -> US8 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: US4 = v4;
                let v5: Rc<UH1> = v5;
                let v18: US8 = match &v1 {
                    US8::US8_None => {
                        let mut v7: bool = match v4 {
                            US4::US4_ItemOpen => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let mut v8: bool = v7 != true;
                        if v8 {
                            let mut v9: US5 = US5::US5_ItemNotOpen(v2);
                            US8::US8_Some(v9)
                        } else {
                            let mut v11: bool = v3 <= 0i32;
                            if v11 {
                                let v12: US5 = US5::US5_ItemUnestimated(v2);
                                US8::US8_Some(v12)
                            } else {
                                US8::US8_None
                            }
                        }
                    }
                    US8::US8_Some(v6) => {
                        let mut v6: US5 = v6.clone();
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
fn first_refusal_25(mut v0: Rc<UH1>) -> US8 {
    let mut v1: US8 = US8::US8_None;
    method26(v0, v1)
}
fn method27(mut v0: Rc<UH1>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5) => {
            let mut v2: Rc<str> = v2;
            let v3: i32 = v3;
            let v4: US4 = v4;
            let v5: Rc<UH1> = v5;
            let mut v6: Rc<UH2> = method27(v5, v1);
            let mut v7: US7 = US7::US7_Committed;
            Rc::new(UH2::UH2_Cons(v2, v3, v7, v6))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method29(mut v0: i32, mut v1: &Rc<RefCell<Mut2>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method28(mut v0: Rc<UH1>, mut v1: Rc<RefCell<Vec<(Rc<str>, i32)>>>) -> Rc<RefCell<Vec<(Rc<str>, i32)>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: US4 = v4;
                let v5: Rc<UH1> = v5;
                let mut v6: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new(vec![(v2, v3)]));
                let mut v7: i32 = (v1.clone().borrow().len() as i32);
                let mut v8: i32 = (v6.clone().borrow().len() as i32);
                let mut v9: i32 = v7.wrapping_add(v8);
                let mut v10: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, i32)>::default(); v9 as usize]));
                let mut v11: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: 0i32 }));
                while method29(v9, &v11) {
                    let v13: i32 = v11.borrow().l0;
                    let v14: bool = v13 < v7;
                    let (mut v20, mut v21): (Rc<str>, i32) = if v14 {
                        let (mut v15, mut v16): (Rc<str>, i32) = v1.borrow()[v13 as usize].clone();
                        (v15, v16)
                    } else {
                        let mut v17: i32 = v13.wrapping_sub(v7);
                        let (mut v18, mut v19): (Rc<str>, i32) = v6.borrow()[v17 as usize].clone();
                        (v18, v19)
                    };
                    v10.borrow_mut()[v13 as usize] = (v20, v21);
                    let v22: i32 = v13.wrapping_add(1i32);
                    v11.borrow_mut().l0 = v22;
                    ()
                };
                (v0, v1) = (v5, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method30(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method30(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US2 = if v6 {
                let mut v7: US3 = US3::US3_HeadType;
                US2::US2_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US3 = US3::US3_HeadType;
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
                            let mut v34: US3 = US3::US3_HeadUnion;
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
fn method32(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method32(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let mut v6: i32 = 0i32;
            let mut v7: i32 = method20(v2.clone(), v5, v6);
            let mut v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2.clone(), v7 as i64, v8 as i64);
            let mut v10: i32 = (v9.clone().len() as i32);
            let mut v11: i32 = method21(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            let v13: bool = &*v12 != "";
            if v13 {
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
fn section_lines_34(mut v0: US10, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> US11 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v8, v9) => {
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH0> = v9.clone();
                let v26: bool = match &v0 {
                    US10::US10_SpiprojProject => {
                        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let mut v11: bool = v8.starts_with(&*v10);
                        let v14: bool = if v11 {
                            true
                        } else {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let v13: bool = v8.starts_with(&*v12);
                            v13
                        };
                        let v15: bool = v14 == false;
                        v15
                    }
                    US10::US10_TomlProject => {
                        let v16: i32 = (v8.clone().len() as i32);
                        let mut v17: i32 = 0i32;
                        let v18: i32 = method20(v8.clone(), v16, v17);
                        let v19: i32 = v16.wrapping_sub(1i32);
                        let v20: Rc<str> = string_slice(&v8.clone(), v18 as i64, v19 as i64);
                        let v21: i32 = (v20.clone().len() as i32);
                        let v22: i32 = method21(v20.clone(), v21);
                        let v23: Rc<str> = string_slice(&v20, 0i32 as i64, v22 as i64);
                        let v24: bool = &*v23 == "'''";
                        v24
                    }
                };
                if v26 {
                    let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v28: Rc<UH0> = method9(v3, v27);
                    return US11::US11_SplitProject(v2, v28, v1);
                } else {
                    let v30: i32 = (v8.clone().len() as i32);
                    let mut v31: i32 = 0i32;
                    let v32: i32 = method20(v8.clone(), v30, v31);
                    let v33: i32 = v30.wrapping_sub(1i32);
                    let mut v34: Rc<str> = string_slice(&v8, v32 as i64, v33 as i64);
                    let mut v35: i32 = (v34.clone().len() as i32);
                    let v36: i32 = method21(v34.clone(), v35);
                    let v37: Rc<str> = string_slice(&v34, 0i32 as i64, v36 as i64);
                    let v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v37, v3));
                    (v1, v3) = (v9, v38);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH0> = method9(v3, v4);
                let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return US11::US11_SplitProject(v2, v5, v6);
            }
        }
    }
}
fn split_lines_33(mut v0: US10, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> US11 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: i32 = (v4.clone().len() as i32);
                let mut v7: i32 = 0i32;
                let mut v8: i32 = method20(v4.clone(), v6, v7);
                let mut v9: i32 = v6.wrapping_sub(1i32);
                let mut v10: Rc<str> = string_slice(&v4.clone(), v8 as i64, v9 as i64);
                let mut v11: i32 = (v10.clone().len() as i32);
                let v12: i32 = method21(v10.clone(), v11);
                let v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
                let mut v17: Rc<str> = match &v0 {
                    US10::US10_SpiprojProject => {
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                        v14
                    }
                    US10::US10_TomlProject => {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                        v15
                    }
                };
                let v18: bool = &*v13 == &*v17;
                if v18 {
                    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v19));
                    let v21: Rc<UH0> = method9(v2, v20);
                    let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    return section_lines_34(v0, v5, v21, v22);
                } else {
                    let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v2));
                    (v1, v2) = (v5, v24);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US11::US11_NoModules;
            }
        }
    }
}
fn closure3(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US13> {
    Rc::new(move |mut v2: Rc<UH0>| -> US13 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US13::US13_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US13> = method35(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US13::US13_None
            }
        }
    })
}
fn method35(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US13> {
    closure3(v0, v1)
}
fn method36(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method36(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method37(mut v0: US10, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method37(v0.clone(), v4, v2);
            let v13: Rc<str> = match v0 {
                US10::US10_SpiprojProject => {
                    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                    v6.borrow_mut().push(v7);
                    v6.borrow_mut().push(v3);
                    let mut v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
                    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
                    v11
                }
                US10::US10_TomlProject => {
                    v3
                }
            };
            Rc::new(UH0::UH0_Cons(v13, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn modules_with_sprints_31(mut v0: US10, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sprints"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v1.replace(&*v4, &*v5));
    let mut v7: Rc<UH0> = split_list_8(v3.clone(), v6);
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method32(v7, v8);
    let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v11: US11 = split_lines_33(v0.clone(), v9, v10);
    let v42: US12 = match v11 {
        US11::US11_NoModules => {
            US12::US12_NoModulesSection
        }
        US11::US11_SplitProject(v13, v14, v15) => {
            let v13: Rc<UH0> = v13;
            let v14: Rc<UH0> = v14;
            let v15: Rc<UH0> = v15;
            let v16: i32 = 0i32;
            let mut v17: Rc<dyn Fn(Rc<UH0>) -> US13> = method35(v2.clone(), v16);
            let v18: US13 = v17(v14.clone());
            let v19: bool = match v18 {
                US13::US13_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v20: bool = v19 != true;
            if v20 {
                US12::US12_ModuleListed
            } else {
                let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v22));
                let v24: Rc<UH0> = method36(v14, v23);
                let v25: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v26: Rc<UH0> = method37(v0, v24, v25);
                let v27: Rc<UH0> = method36(v26, v15);
                let v28: Rc<UH0> = method36(v13, v27);
                let v29: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v30: i32 = 0i32;
                let mut v31: i32 = method12(v29.clone(), v28, v30);
                let v32: Rc<Vec<Rc<str>>> = Rc::new(v29.borrow().clone());
                let v33: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v32).as_ref().clone()));
                let mut v34: Rc<str> = Rc::<str>::from((*v33.borrow()).join(&*v3));
                let mut v35: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v35.borrow_mut().push(v34);
                v35.borrow_mut().push(v3);
                let v36: Rc<Vec<Rc<str>>> = Rc::new(v35.borrow().clone());
                let v37: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v36).as_ref().clone()));
                let v38: Rc<str> = Rc::<str>::from((*v37.borrow()).join(&*v5));
                US12::US12_ModuleAdded(v38)
            }
        }
    };
    match v42 {
        US12::US12_ModuleAdded(v43) => {
            let v43: Rc<str> = v43;
            v43
        }
        _ => {
            v1
        }
    }
}
fn method39(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: US7 = v4;
                let v5: Rc<UH2> = v5;
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn format_real_40(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn outcome_text_38(mut v0: US5) -> (i32, Rc<str>) {
    match v0 {
        US5::US5_AnotherSprintOpen(v15) => {
            let v15: Rc<str> = v15;
            let mut v17: Rc<str> = string_concat(&["rig error: agile sprint start refused: sprint ", &*v15, " is still open"]);
            (1i32, v17)
        }
        US5::US5_ItemNotOpen(v18) => {
            let v18: Rc<str> = v18;
            let v20: Rc<str> = string_concat(&["rig error: agile sprint start refused: item ", &*v18, " is unknown or Done"]);
            (1i32, v20)
        }
        US5::US5_ItemUnestimated(v21) => {
            let v21: Rc<str> = v21;
            let v23: Rc<str> = string_concat(&["rig error: agile sprint start refused: item ", &*v21, " has no estimate"]);
            (1i32, v23)
        }
        US5::US5_Started(v1, v2, v3, v4, v5, v6) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: US6 = v5;
            let mut v6: Rc<UH2> = v6;
            let mut v7: i32 = 0i32;
            let mut v8: i32 = method39(v6, v7);
            let mut v11: Rc<str> = format_real_40(v8);
            let v14: Rc<str> = string_concat(&["rig agile sprint start ok id=", &*v1, " items=", &*v11, " finish=", &*v4]);
            (0i32, v14)
        }
    }
}
fn sprint_started_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<UH0>) -> i32 {
    let v5: Option<i64> = (v2).trim().parse::<i64>().ok();
    let mut v6: Option<i64> = method2(v5);
    let mut v7: Rc<dyn Fn((i64)) -> US0> = closure1();
    let mut v8: Option<US0> = v6.map(|x| v7(x));
    let mut v10: US0 = US0::US0_None;
    let mut v11: US0 = v10.clone();
    let v12: US0 = v8.unwrap_or(v11);
    let v15: i64 = match v12 {
        US0::US0_None => {
            0i64
        }
        US0::US0_Some(v13) => {
            let v13: i64 = v13;
            v13
        }
    };
    let v16: bool = v15 <= 0i64;
    let v18: bool = if v16 {
        true
    } else {
        let mut v17: bool = &*v1 == "";
        v17
    };
    if v18 {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: agile usage: sprint start <root> <id> <days> <goal> [--items A,B]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v19);
        2i32
    } else {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = Rc::<str>::from(v0.replace(&*v20, &*v21));
        let v23: Rc<str> = string_concat(&[&*v22, "/state"]);
        let v24: Rc<str> = string_concat(&[&*v23, "/agile.spi"]);
        let v25: bool = std::path::Path::new(&*v24).is_file();
        let v77: Rc<str> = if v25 {
            let (mut v26, mut v27): (bool, Rc<str>) = match std::fs::read(&*v24) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v26 {
                let v28: Rc<str> = string_concat(&["rig state read failed: ", &*v24]);
                std::panic::panic_any::<std::string::String>(format!("{}", v28))
            } else {
                let v30: Rc<str> = Rc::<str>::from(v24.replace(&*v20, &*v21));
                let mut v31: Rc<str> = loop_3(v30);
                let v32: Option<usize> = v31.rfind(&*v21);
                let v33: Option<usize> = method4(v32);
                let mut v34: Rc<dyn Fn((usize)) -> i32> = closure2();
                let mut v35: Option<i32> = v33.map(|x| v34(x));
                let v37: i32 = v35.unwrap_or(-1i32);
                let v38: bool = v37 < 0i32;
                let v42: bool = if v38 {
                    true
                } else {
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v40: bool = v31.ends_with(&*v39);
                    let v41: bool = v40 == false;
                    v41
                };
                let v63: US1 = if v42 {
                    US1::US1_OtherFile
                } else {
                    let v44: usize = ((0i32) as usize);
                    let v45: usize = ((v37) as usize);
                    let v46: Rc<str> = Rc::<str>::from(&(v31)[v44..v45]);
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v48: bool = v46.ends_with(&*v47);
                    if v48 {
                        let v49: Rc<str> = loop_5(v46);
                        US1::US1_StateFile(v49)
                    } else {
                        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v52: bool = v46.ends_with(&*v51);
                        if v52 {
                            let v53: Rc<str> = loop_6(v46);
                            US1::US1_HistoryFile(v53)
                        } else {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v56: bool = v46.ends_with(&*v55);
                            if v56 {
                                let v57: Rc<str> = loop_7(v46);
                                US1::US1_ArchiveFile(v57)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                match v63 {
                    US1::US1_OtherFile => {
                        v27
                    }
                    _ => {
                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v65: Rc<UH0> = split_list_8(v64.clone(), v27);
                        let v66: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v67: Rc<UH0> = method11(v65, v66);
                        let v68: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v69: i32 = 0i32;
                        let v70: i32 = method12(v68.clone(), v67, v69);
                        let v71: Rc<Vec<Rc<str>>> = Rc::new(v68.borrow().clone());
                        let v72: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v71).as_ref().clone()));
                        let v73: Rc<str> = Rc::<str>::from((*v72.borrow()).join(&*v64));
                        v73
                    }
                }
            }
        } else {
            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v76
        };
        let v78: Rc<str> = string_concat(&[&*v23, "/sprints.spi"]);
        let v79: bool = std::path::Path::new(&*v78).is_file();
        let v131: Rc<str> = if v79 {
            let (mut v80, mut v81): (bool, Rc<str>) = match std::fs::read(&*v78) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v80 {
                let v82: Rc<str> = string_concat(&["rig state read failed: ", &*v78]);
                std::panic::panic_any::<std::string::String>(format!("{}", v82))
            } else {
                let v84: Rc<str> = Rc::<str>::from(v78.replace(&*v20, &*v21));
                let mut v85: Rc<str> = loop_3(v84);
                let v86: Option<usize> = v85.rfind(&*v21);
                let v87: Option<usize> = method4(v86);
                let mut v88: Rc<dyn Fn((usize)) -> i32> = closure2();
                let mut v89: Option<i32> = v87.map(|x| v88(x));
                let v91: i32 = v89.unwrap_or(-1i32);
                let v92: bool = v91 < 0i32;
                let v96: bool = if v92 {
                    true
                } else {
                    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v94: bool = v85.ends_with(&*v93);
                    let v95: bool = v94 == false;
                    v95
                };
                let v117: US1 = if v96 {
                    US1::US1_OtherFile
                } else {
                    let v98: usize = ((0i32) as usize);
                    let v99: usize = ((v91) as usize);
                    let v100: Rc<str> = Rc::<str>::from(&(v85)[v98..v99]);
                    let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v102: bool = v100.ends_with(&*v101);
                    if v102 {
                        let v103: Rc<str> = loop_5(v100);
                        US1::US1_StateFile(v103)
                    } else {
                        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v106: bool = v100.ends_with(&*v105);
                        if v106 {
                            let v107: Rc<str> = loop_6(v100);
                            US1::US1_HistoryFile(v107)
                        } else {
                            let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v110: bool = v100.ends_with(&*v109);
                            if v110 {
                                let v111: Rc<str> = loop_7(v100);
                                US1::US1_ArchiveFile(v111)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                match v117 {
                    US1::US1_OtherFile => {
                        v81
                    }
                    _ => {
                        let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v119: Rc<UH0> = split_list_8(v118.clone(), v81);
                        let v120: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v121: Rc<UH0> = method11(v119, v120);
                        let v122: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v123: i32 = 0i32;
                        let v124: i32 = method12(v122.clone(), v121, v123);
                        let v125: Rc<Vec<Rc<str>>> = Rc::new(v122.borrow().clone());
                        let v126: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v125).as_ref().clone()));
                        let v127: Rc<str> = Rc::<str>::from((*v126.borrow()).join(&*v118));
                        v127
                    }
                }
            }
        } else {
            let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v130
        };
        let v132: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let v133: i64 = (v132 as i64);
        let v134: i64 = v133.wrapping_div(86400000i64);
        let v135: Rc<str> = civil_date_13(v134);
        let v136: i64 = v134.wrapping_add(v15);
        let v137: Rc<str> = civil_date_13(v136);
        let v138: Rc<UH0> = items_given_17(v4);
        let v139: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let v140: Rc<UH1> = method22(v77, v138, v139);
        let v141: Rc<str> = rig_agile_sprint_state::rig_agile_open_sprint(&v131);
        let v142: bool = &*v141 != "";
        let v152: US5 = if v142 {
            US5::US5_AnotherSprintOpen(v141)
        } else {
            let v144: US8 = first_refusal_25(v140.clone());
            match v144 {
                US8::US8_None => {
                    let v146: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let v147: Rc<UH2> = method27(v140.clone(), v146);
                    let v148: US6 = US6::US6_SprintOpen;
                    US5::US5_Started(v1.clone(), v3.clone(), v135.clone(), v137.clone(), v148, v147)
                }
                US8::US8_Some(v145) => {
                    let v145: US5 = v145;
                    v145
                }
            }
        };
        match &v152 {
            US5::US5_Started(v153, v154, v155, v156, v157, v158) => {
                let v153: Rc<str> = v153.clone();
                let v154: Rc<str> = v154.clone();
                let v155: Rc<str> = v155.clone();
                let v156: Rc<str> = v156.clone();
                let v157: US6 = v157.clone();
                let v158: Rc<UH2> = v158.clone();
                let v159: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new(vec![]));
                let v160: Rc<RefCell<Vec<(Rc<str>, i32)>>> = method28(v140, v159);
                let v161: Rc<str> = rig_agile_sprint_append::rig_agile_sprint_appended(&v131, &v1, &v3, &v135, &v137, v160);
                let v162: Rc<str> = Rc::<str>::from(v78.replace(&*v20, &*v21));
                let mut v163: Rc<str> = loop_3(v162);
                let v164: Option<usize> = v163.rfind(&*v21);
                let v165: Option<usize> = method4(v164);
                let mut v166: Rc<dyn Fn((usize)) -> i32> = closure2();
                let mut v167: Option<i32> = v165.map(|x| v166(x));
                let v169: i32 = v167.unwrap_or(-1i32);
                let v170: bool = v169 < 0i32;
                let v174: bool = if v170 {
                    true
                } else {
                    let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v172: bool = v163.ends_with(&*v171);
                    let v173: bool = v172 == false;
                    v173
                };
                let v195: US1 = if v174 {
                    US1::US1_OtherFile
                } else {
                    let v176: usize = ((0i32) as usize);
                    let v177: usize = ((v169) as usize);
                    let v178: Rc<str> = Rc::<str>::from(&(v163)[v176..v177]);
                    let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v180: bool = v178.ends_with(&*v179);
                    if v180 {
                        let v181: Rc<str> = loop_5(v178);
                        US1::US1_StateFile(v181)
                    } else {
                        let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v184: bool = v178.ends_with(&*v183);
                        if v184 {
                            let v185: Rc<str> = loop_6(v178);
                            US1::US1_HistoryFile(v185)
                        } else {
                            let v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v188: bool = v178.ends_with(&*v187);
                            if v188 {
                                let v189: Rc<str> = loop_7(v178);
                                US1::US1_ArchiveFile(v189)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                let v218: US9 = match v195 {
                    US1::US1_ArchiveFile(v208) => {
                        let v208: Rc<str> = v208;
                        let v209: Rc<str> = string_concat(&[&*v208, "/state/spiral.toml"]);
                        let v210: bool = std::path::Path::new(&*v209).is_file();
                        if v210 {
                            US9::US9_SymbolsRoot
                        } else {
                            US9::US9_WordsRoot
                        }
                    }
                    US1::US1_HistoryFile(v202) => {
                        let v202: Rc<str> = v202;
                        let v203: Rc<str> = string_concat(&[&*v202, "/state/spiral.toml"]);
                        let v204: bool = std::path::Path::new(&*v203).is_file();
                        if v204 {
                            US9::US9_SymbolsRoot
                        } else {
                            US9::US9_WordsRoot
                        }
                    }
                    US1::US1_OtherFile => {
                        US9::US9_WordsRoot
                    }
                    US1::US1_StateFile(v196) => {
                        let v196: Rc<str> = v196;
                        let v197: Rc<str> = string_concat(&[&*v196, "/state/spiral.toml"]);
                        let v198: bool = std::path::Path::new(&*v197).is_file();
                        if v198 {
                            US9::US9_SymbolsRoot
                        } else {
                            US9::US9_WordsRoot
                        }
                    }
                };
                let v230: Rc<str> = match v218 {
                    US9::US9_SymbolsRoot => {
                        let v219: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v220: Rc<UH0> = split_list_8(v219.clone(), v161);
                        let v221: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v222: Rc<UH0> = method30(v220, v221);
                        let v223: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v224: i32 = 0i32;
                        let v225: i32 = method12(v223.clone(), v222, v224);
                        let v226: Rc<Vec<Rc<str>>> = Rc::new(v223.borrow().clone());
                        let v227: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v226).as_ref().clone()));
                        let v228: Rc<str> = Rc::<str>::from((*v227.borrow()).join(&*v219));
                        v228
                    }
                    US9::US9_WordsRoot => {
                        v161
                    }
                };
                let v231: bool = std::fs::write(&*v78, &*v230).is_ok();
                if v231 {
                    ()
                } else {
                    let v232: Rc<str> = string_concat(&["rig state write failed: ", &*v78]);
                    std::panic::panic_any::<std::string::String>(format!("{}", v232))
                };
                let mut v233: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v233.borrow_mut().push(v23.clone());
                let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.toml"); } LIT.with(|lit| lit.clone()) };
                v233.borrow_mut().push(v234.clone());
                let v235: Rc<Vec<Rc<str>>> = Rc::new(v233.borrow().clone());
                let v236: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v235).as_ref().clone()));
                let v237: Rc<str> = Rc::<str>::from((*v236.borrow()).join(&*v21));
                let v238: bool = std::path::Path::new(&*v237).is_file();
                let v241: US10 = if v238 {
                    US10::US10_TomlProject
                } else {
                    US10::US10_SpiprojProject
                };
                let v244: Rc<str> = match v241 {
                    US10::US10_SpiprojProject => {
                        let v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
                        v242
                    }
                    US10::US10_TomlProject => {
                        v234.clone()
                    }
                };
                let mut v245: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v245.borrow_mut().push(v23.clone());
                v245.borrow_mut().push(v244);
                let v246: Rc<Vec<Rc<str>>> = Rc::new(v245.borrow().clone());
                let v247: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v246).as_ref().clone()));
                let v248: Rc<str> = Rc::<str>::from((*v247.borrow()).join(&*v21));
                let v249: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v248).unwrap_or_default().as_str());
                let mut v250: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v250.borrow_mut().push(v23);
                v250.borrow_mut().push(v234);
                let v251: Rc<Vec<Rc<str>>> = Rc::new(v250.borrow().clone());
                let v252: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v251).as_ref().clone()));
                let v253: Rc<str> = Rc::<str>::from((*v252.borrow()).join(&*v21));
                let v254: bool = std::path::Path::new(&*v253).is_file();
                let v257: US10 = if v254 {
                    US10::US10_TomlProject
                } else {
                    US10::US10_SpiprojProject
                };
                let v258: Rc<str> = modules_with_sprints_31(v257, v249.clone());
                let v259: bool = &*v258 != &*v249;
                if v259 {
                    std::fs::write(&*v248, &*v258).unwrap();
                    ()
                }
            }
            _ => {
                ()
            }
        };
        let (mut v261, mut v262): (i32, Rc<str>) = outcome_text_38(v152);
        let v263: bool = v261 == 0i32;
        if v263 {
            println!("{}", v262);
            ()
        } else {
            eprintln!("{}", v262);
            ()
        };
        v261
    }
}
fn method41(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
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
fn started_0(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> i32 {
    let v5: Rc<Vec<Rc<str>>> = Rc::new((v0).borrow().clone());
    let mut v6: usize = (v5).len();
    let mut v7: i32 = ((v6) as i32);
    let mut v8: i32 = v7.wrapping_sub(1i32);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v10: Rc<UH0> = method41(v5, v8, v9);
    sprint_started_1(v4, v3, v2, v1, v10)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<RefCell<Vec<Rc<str>>>>| -> i32 {
        started_0(v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_sprint_started(v0: &str, v1: &str, v2: &str, v3: &str, v4: Rc<RefCell<Vec<Rc<str>>>>) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4)
}
use agile_task_census::SpiralShape_97df63ebabc3 as Mut1;
use rig_agile_sprint_state::SpiralShape_2617b4e49837 as Mut2;
pub type SpiralShape_652accfc6a8d = Mut0;
