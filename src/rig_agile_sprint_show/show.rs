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
    US0_ShowItems,
    US0_ShowNotes,
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
    US4_Some(i64),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_Some(i32),
    US5_None,
}
fn loop_2(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn method3(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_4(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn loop_6(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn method9(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method9(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method8(v16, v17)
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method10(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
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
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let mut v18: US3 = US3::US3_HeadInl;
                        US2::US2_KeywordHead(v18)
                    } else {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
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
                    let mut v70: i32 = v69.wrapping_sub(v61);
                    let v71: usize = ((v61) as usize);
                    let v72: i32 = v61.wrapping_add(v70);
                    let mut v73: usize = ((v72) as usize);
                    let mut v74: Rc<str> = Rc::<str>::from(&(v2)[v71..v73]);
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
                let mut v3: Rc<str> = v3;
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
fn depth_word_12(mut v0: US0) -> Rc<str> {
    let mut v1: bool = match v0 {
        US0::US0_ShowNotes => {
            true
        }
        _ => {
            false
        }
    };
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("retro"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
        v3
    }
}
fn method15(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_14(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn elapsed_field_13(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
    let v2: Rc<str> = format_real_14(v1);
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    v3.borrow_mut().push(v4);
    v3.borrow_mut().push(v2);
    let v5: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from((*v6.borrow()).join(&*v7));
    v8
}
fn committed_from_16(mut v0: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let (mut v5, mut v6, mut v7): (Rc<str>, i32, bool) = v0.borrow()[v1 as usize].clone();
            let mut v8: i32 = v1.wrapping_add(1i32);
            let v10: i32 = if v7 {
                let mut v9: i32 = v2.wrapping_add(v6);
                v9
            } else {
                v2
            };
            (v1, v2) = (v8, v10);
            continue;
        }
    }
}
fn method18(mut v0: Option<i64>) -> Option<i64> {
    v0
}
fn closure2() -> Rc<dyn Fn((i64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i64)) -> US4> = Rc::new(move |mut v0: (i64)| -> US4 {
        let mut v1: i64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn days_of_17(mut v0: Rc<str>) -> i64 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let v2: bool = v1 < 10i32;
    if v2 {
        0i64
    } else {
        let mut v3: usize = ((0i32) as usize);
        let v4: usize = ((4i32) as usize);
        let v5: Rc<str> = Rc::<str>::from(&(v0)[v3..v4]);
        let v6: Option<i64> = (v5).trim().parse::<i64>().ok();
        let v7: Option<i64> = method18(v6);
        let mut v8: Rc<dyn Fn((i64)) -> US4> = closure2();
        let mut v9: Option<US4> = v7.map(|x| v8(x));
        let v11: US4 = US4::US4_None;
        let mut v12: US4 = v11.clone();
        let mut v13: US4 = v9.unwrap_or(v12);
        let mut v16: i64 = match v13 {
            US4::US4_None => {
                0i64
            }
            US4::US4_Some(v14) => {
                let v14: i64 = v14;
                v14
            }
        };
        let v17: usize = ((5i32) as usize);
        let mut v18: usize = ((7i32) as usize);
        let mut v19: Rc<str> = Rc::<str>::from(&(v0)[v17..v18]);
        let mut v20: Option<i64> = (v19).trim().parse::<i64>().ok();
        let v21: Option<i64> = method18(v20);
        let mut v22: Option<US4> = v21.map(|x| v8(x));
        let v24: US4 = US4::US4_None;
        let v25: US4 = v24.clone();
        let v26: US4 = v22.unwrap_or(v25);
        let v29: i64 = match v26 {
            US4::US4_None => {
                0i64
            }
            US4::US4_Some(v27) => {
                let v27: i64 = v27;
                v27
            }
        };
        let v30: usize = ((8i32) as usize);
        let v31: usize = ((10i32) as usize);
        let v32: Rc<str> = Rc::<str>::from(&(v0)[v30..v31]);
        let v33: Option<i64> = (v32).trim().parse::<i64>().ok();
        let v34: Option<i64> = method18(v33);
        let mut v35: Option<US4> = v34.map(|x| v8(x));
        let v37: US4 = US4::US4_None;
        let v38: US4 = v37.clone();
        let v39: US4 = v35.unwrap_or(v38);
        let v42: i64 = match v39 {
            US4::US4_None => {
                0i64
            }
            US4::US4_Some(v40) => {
                let v40: i64 = v40;
                v40
            }
        };
        let v43: bool = v29 <= 2i64;
        let v45: i64 = if v43 {
            let v44: i64 = v16.wrapping_sub(1i64);
            v44
        } else {
            v16
        };
        let v46: i64 = v45.wrapping_div(400i64);
        let v47: i64 = v45.wrapping_rem(400i64);
        let v48: bool = v47 != 0i64;
        let v50: bool = if v48 {
            let v49: bool = v45 < 0i64;
            v49
        } else {
            false
        };
        let v52: i64 = if v50 {
            let v51: i64 = v46.wrapping_sub(1i64);
            v51
        } else {
            v46
        };
        let v53: i64 = v52.wrapping_mul(400i64);
        let v54: i64 = v45.wrapping_sub(v53);
        let v55: i64 = v54.wrapping_mul(365i64);
        let v56: i64 = v54.wrapping_div(4i64);
        let v57: i64 = v55.wrapping_add(v56);
        let v58: i64 = v54.wrapping_div(100i64);
        let v59: i64 = v57.wrapping_sub(v58);
        let v60: bool = v29 > 2i64;
        let v63: i64 = if v60 {
            let v61: i64 = v29.wrapping_sub(3i64);
            v61
        } else {
            let v62: i64 = v29.wrapping_add(9i64);
            v62
        };
        let v64: i64 = 153i64.wrapping_mul(v63);
        let v65: i64 = v64.wrapping_add(2i64);
        let v66: i64 = v65.wrapping_div(5i64);
        let v67: i64 = v66.wrapping_add(v42);
        let v68: i64 = v67.wrapping_sub(1i64);
        let v69: i64 = v59.wrapping_add(v68);
        let mut v70: i64 = v52.wrapping_mul(146097i64);
        let v71: i64 = v70.wrapping_add(v69);
        let v72: i64 = v71.wrapping_sub(719468i64);
        v72
    }
}
fn format_real_19(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_21(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn clipped_23(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let v2: bool = v1 > 160i32;
    if v2 {
        let mut v3: usize = ((0i32) as usize);
        let v4: usize = ((160i32) as usize);
        let v5: Rc<str> = Rc::<str>::from(&(v0)[v3..v4]);
        let v6: Rc<str> = string_concat(&[&*v5, "..."]);
        v6
    } else {
        v0
    }
}
fn last_note_line_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = agile_note_listing::rig_agile_last_note(&v0, &v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v2.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v5: i32 = (v4.clone().borrow().len() as i32);
    let v6: bool = v5 < 2i32;
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v7
    } else {
        let mut v8: Rc<str> = v4.borrow()[1i32 as usize].clone();
        let mut v9: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        clipped_23(v9)
    }
}
fn items_shown_20(mut v0: US0, mut v1: Rc<str>, mut v2: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v3: i32, mut v4: i32) -> i32 {
    loop {
        let v5: i32 = (v2.clone().borrow().len() as i32);
        let v6: bool = v3 >= v5;
        if v6 {
            return v4;
        } else {
            let (mut v7, mut v8, mut v9): (Rc<str>, i32, bool) = v2.borrow()[v3 as usize].clone();
            let v10: Rc<str> = agile_task_census::rig_agile_task_status_of(&v1, &v7);
            let mut v13: Rc<str> = if v9 {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("committed"); } LIT.with(|lit| lit.clone()) };
                v11
            } else {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
                v12
            };
            let v14: bool = &*v10 == "";
            let mut v16: Rc<str> = if v14 {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown"); } LIT.with(|lit| lit.clone()) };
                v15
            } else {
                v10.clone()
            };
            let mut v19: Rc<str> = format_real_21(v8);
            let v24: Rc<str> = string_concat(&["  ", &*v7, " ", &*v19, "m ", &*v16, " ", &*v13]);
            println!("{}", v24);
            let v25: bool = match &v0 {
                US0::US0_ShowNotes => {
                    true
                }
                _ => {
                    false
                }
            };
            let v28: Rc<str> = if v25 {
                last_note_line_22(v1.clone(), v7)
            } else {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v27
            };
            let v29: bool = &*v28 != "";
            if v29 {
                let v30: Rc<str> = string_concat(&["    ", &*v28]);
                println!("{}", v30);
                ()
            };
            let v31: bool = &*v10 == "Done";
            let v32: bool = v31 && v9;
            let v34: i32 = if v32 {
                let v33: i32 = v4.wrapping_add(v8);
                v33
            } else {
                v4
            };
            let mut v35: i32 = v3.wrapping_add(1i32);
            (v3, v4) = (v35, v34);
            continue;
        }
    }
}
fn sprint_shown_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US0) -> i32 {
    let mut v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
    let v7: Rc<str> = string_concat(&[&*v6, "/state"]);
    let mut v8: Rc<str> = string_concat(&[&*v7, "/sprints.spi"]);
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
            let v17: Option<usize> = method3(v16);
            let mut v18: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v19: Option<i32> = v17.map(|x| v18(x));
            let v21: i32 = v19.unwrap_or(-1i32);
            let mut v22: bool = v21 < 0i32;
            let v26: bool = if v22 {
                true
            } else {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v24: bool = v15.ends_with(&*v23);
                let v25: bool = v24 == false;
                v25
            };
            let v47: US1 = if v26 {
                US1::US1_OtherFile
            } else {
                let v28: usize = ((0i32) as usize);
                let v29: usize = ((v21) as usize);
                let v30: Rc<str> = Rc::<str>::from(&(v15)[v28..v29]);
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v32: bool = v30.ends_with(&*v31);
                if v32 {
                    let v33: Rc<str> = loop_4(v30);
                    US1::US1_StateFile(v33)
                } else {
                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
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
            match v47 {
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
            }
        }
    } else {
        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v60
    };
    let v62: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>> = rig_agile_sprint_state::rig_agile_sprint_head(&v61, &v1);
    let v63: i32 = (v62.clone().borrow().len() as i32);
    let v64: bool = v63 == 0i32;
    if v64 {
        let v65: bool = &*v1 == "";
        let v68: Rc<str> = if v65 {
            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open sprint"); } LIT.with(|lit| lit.clone()) };
            v66
        } else {
            let v67: Rc<str> = string_concat(&["sprint ", &*v1]);
            v67
        };
        let v69: Rc<str> = depth_word_12(v2);
        let mut v73: Rc<str> = elapsed_field_13(v3);
        let mut v74: Rc<str> = string_concat(&["rig error: agile sprint ", &*v69, ": no ", &*v68, &*v73]);
        eprintln!("{}", v74);
        1i32
    } else {
        let (mut v75, mut v76, mut v77, mut v78, mut v79): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) = v62.borrow()[0i32 as usize].clone();
        let v80: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> = rig_agile_sprint_state::rig_agile_sprint_items(&v61, &v75);
        let v81: Rc<str> = string_concat(&[&*v7, "/agile.spi"]);
        let v82: bool = std::path::Path::new(&*v81).is_file();
        let v134: Rc<str> = if v82 {
            let (mut v83, mut v84): (bool, Rc<str>) = match std::fs::read(&*v81) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v83 {
                let v85: Rc<str> = string_concat(&["rig state read failed: ", &*v81]);
                std::panic::panic_any::<std::string::String>(format!("{}", v85))
            } else {
                let v87: Rc<str> = Rc::<str>::from(v81.replace(&*v4, &*v5));
                let mut v88: Rc<str> = loop_2(v87);
                let v89: Option<usize> = v88.rfind(&*v5);
                let v90: Option<usize> = method3(v89);
                let mut v91: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v92: Option<i32> = v90.map(|x| v91(x));
                let v94: i32 = v92.unwrap_or(-1i32);
                let v95: bool = v94 < 0i32;
                let v99: bool = if v95 {
                    true
                } else {
                    let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v97: bool = v88.ends_with(&*v96);
                    let v98: bool = v97 == false;
                    v98
                };
                let v120: US1 = if v99 {
                    US1::US1_OtherFile
                } else {
                    let v101: usize = ((0i32) as usize);
                    let v102: usize = ((v94) as usize);
                    let v103: Rc<str> = Rc::<str>::from(&(v88)[v101..v102]);
                    let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v105: bool = v103.ends_with(&*v104);
                    if v105 {
                        let v106: Rc<str> = loop_4(v103);
                        US1::US1_StateFile(v106)
                    } else {
                        let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v109: bool = v103.ends_with(&*v108);
                        if v109 {
                            let v110: Rc<str> = loop_5(v103);
                            US1::US1_HistoryFile(v110)
                        } else {
                            let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v113: bool = v103.ends_with(&*v112);
                            if v113 {
                                let v114: Rc<str> = loop_6(v103);
                                US1::US1_ArchiveFile(v114)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                match v120 {
                    US1::US1_OtherFile => {
                        v84
                    }
                    _ => {
                        let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v122: Rc<UH0> = split_list_7(v121.clone(), v84);
                        let v123: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v124: Rc<UH0> = method10(v122, v123);
                        let v125: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v126: i32 = 0i32;
                        let v127: i32 = method11(v125.clone(), v124, v126);
                        let v128: Rc<Vec<Rc<str>>> = Rc::new(v125.borrow().clone());
                        let v129: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v128).as_ref().clone()));
                        let v130: Rc<str> = Rc::<str>::from((*v129.borrow()).join(&*v121));
                        v130
                    }
                }
            }
        } else {
            let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v133
        };
        let v135: i32 = 0i32;
        let v136: i32 = 0i32;
        let v137: i32 = committed_from_16(&v80, v135, v136);
        let v138: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let v139: i64 = (v138 as i64);
        let v140: i64 = v139.wrapping_div(86400000i64);
        let v141: i64 = days_of_17(v78.clone());
        let v142: i64 = v141.wrapping_sub(v140);
        let v145: Rc<str> = if v79 {
            let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open"); } LIT.with(|lit| lit.clone()) };
            v143
        } else {
            let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("closed"); } LIT.with(|lit| lit.clone()) };
            v144
        };
        let v146: Rc<str> = depth_word_12(v2.clone());
        let v157: Rc<str> = format_real_19(v142);
        let v160: Rc<str> = string_concat(&["rig agile sprint ", &*v146, " ok id=", &*v75, " status=", &*v145, " start=", &*v77, " finish=", &*v78, " days_left=", &*v157, " goal=", &*v76]);
        println!("{}", v160);
        let v161: i32 = 0i32;
        let v162: i32 = 0i32;
        let v163: i32 = items_shown_20(v2, v134, &v80, v161, v162);
        let v164: Rc<str> = format_real_21(v137);
        let v167: Rc<str> = format_real_21(v163);
        let v170: Rc<str> = elapsed_field_13(v3);
        let v171: Rc<str> = string_concat(&["  committed=", &*v164, "m done=", &*v167, "m", &*v170]);
        println!("{}", v171);
        0i32
    }
}
fn shown_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: US0 = US0::US0_ShowItems;
    sprint_shown_1(v1, v0, v2)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        shown_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn retro_24(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: Rc<str> = string_concat(&[&*v4, "/state/sprints.spi"]);
    let v6: bool = std::path::Path::new(&*v5).is_file();
    let v58: Rc<str> = if v6 {
        let (mut v7, mut v8): (bool, Rc<str>) = match std::fs::read(&*v5) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v7 {
            let mut v9: Rc<str> = string_concat(&["rig state read failed: ", &*v5]);
            std::panic::panic_any::<std::string::String>(format!("{}", v9))
        } else {
            let v11: Rc<str> = Rc::<str>::from(v5.replace(&*v2, &*v3));
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
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v21: bool = v12.ends_with(&*v20);
                let mut v22: bool = v21 == false;
                v22
            };
            let v44: US1 = if v23 {
                US1::US1_OtherFile
            } else {
                let v25: usize = ((0i32) as usize);
                let v26: usize = ((v18) as usize);
                let v27: Rc<str> = Rc::<str>::from(&(v12)[v25..v26]);
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v29: bool = v27.ends_with(&*v28);
                if v29 {
                    let v30: Rc<str> = loop_4(v27);
                    US1::US1_StateFile(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v27.ends_with(&*v32);
                    if v33 {
                        let v34: Rc<str> = loop_5(v27);
                        US1::US1_HistoryFile(v34)
                    } else {
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v37: bool = v27.ends_with(&*v36);
                        if v37 {
                            let v38: Rc<str> = loop_6(v27);
                            US1::US1_ArchiveFile(v38)
                        } else {
                            US1::US1_OtherFile
                        }
                    }
                }
            };
            match v44 {
                US1::US1_OtherFile => {
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
    let v59: bool = &*v0 == "";
    let v61: Rc<str> = if v59 {
        let v60: Rc<str> = rig_agile_sprint_state::rig_agile_latest_sprint(&v58);
        v60
    } else {
        v0
    };
    let v62: US0 = US0::US0_ShowNotes;
    sprint_shown_1(v1, v61, v62)
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        retro_24(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US5> {
    Rc::new(move |mut v2: Rc<UH0>| -> US5 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US5::US5_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US5> = method27(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US5::US5_None
            }
        }
    })
}
fn method27(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US5> {
    closure5(v0, v1)
}
fn method28(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method28(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn done_since_26(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>>, mut v1: i32, mut v2: u64, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            return v3;
        } else {
            let (mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, u64) = v0.borrow()[v1 as usize].clone();
            let mut v9: bool = &*v7 == "Done";
            let v11: bool = if v9 {
                let v10: bool = v8 >= v2;
                v10
            } else {
                false
            };
            let mut v18: bool = if v11 {
                let mut v12: i32 = 0i32;
                let mut v13: Rc<dyn Fn(Rc<UH0>) -> US5> = method27(v6.clone(), v12);
                let v14: US5 = v13(v3.clone());
                let mut v15: bool = match v14 {
                    US5::US5_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v16: bool = v15 != true;
                let v17: bool = v16 == false;
                v17
            } else {
                false
            };
            let mut v19: i32 = v1.wrapping_add(1i32);
            let v23: Rc<UH0> = if v18 {
                let mut v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v20));
                method28(v3, v21)
            } else {
                v3
            };
            (v1, v3) = (v19, v23);
            continue;
        }
    }
}
fn method29(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn method30(mut v0: Rc<str>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: Rc<str> = agile_task_census::rig_agile_task_status_of(&v0, &v2);
                let v6: Rc<str> = string_concat(&["  ", &*v2, " "]);
                let v7: bool = &*v4 == "";
                let mut v9: Rc<str> = if v7 {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown"); } LIT.with(|lit| lit.clone()) };
                    v8
                } else {
                    v4
                };
                let v10: Rc<str> = string_concat(&[&*v6, &*v9]);
                println!("{}", v10);
                let v11: Rc<str> = last_note_line_22(v0.clone(), v2);
                let mut v12: bool = &*v11 != "";
                if v12 {
                    let mut v13: Rc<str> = string_concat(&["    ", &*v11]);
                    println!("{}", v13);
                    ()
                };
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
fn retro_since_25(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v1.replace(&*v3, &*v4));
    let v6: i64 = days_of_17(v0.clone());
    let v7: bool = v6 <= 0i64;
    if v7 {
        let mut v8: Rc<str> = string_concat(&["rig error: agile sprint retro --since expects a YYYY-MM-DD date, got ", &*v0]);
        eprintln!("{}", v8);
        2i32
    } else {
        let mut v9: Rc<str> = string_concat(&[&*v5, "/history/status_changes.spi"]);
        let v10: bool = std::path::Path::new(&*v9).is_file();
        let v62: Rc<str> = if v10 {
            let (mut v11, mut v12): (bool, Rc<str>) = match std::fs::read(&*v9) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v11 {
                let mut v13: Rc<str> = string_concat(&["rig state read failed: ", &*v9]);
                std::panic::panic_any::<std::string::String>(format!("{}", v13))
            } else {
                let mut v15: Rc<str> = Rc::<str>::from(v9.replace(&*v3, &*v4));
                let mut v16: Rc<str> = loop_2(v15);
                let v17: Option<usize> = v16.rfind(&*v4);
                let mut v18: Option<usize> = method3(v17);
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
                let v48: US1 = if v27 {
                    US1::US1_OtherFile
                } else {
                    let v29: usize = ((0i32) as usize);
                    let v30: usize = ((v22) as usize);
                    let v31: Rc<str> = Rc::<str>::from(&(v16)[v29..v30]);
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v31.ends_with(&*v32);
                    if v33 {
                        let v34: Rc<str> = loop_4(v31);
                        US1::US1_StateFile(v34)
                    } else {
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v37: bool = v31.ends_with(&*v36);
                        if v37 {
                            let v38: Rc<str> = loop_5(v31);
                            US1::US1_HistoryFile(v38)
                        } else {
                            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v41: bool = v31.ends_with(&*v40);
                            if v41 {
                                let v42: Rc<str> = loop_6(v31);
                                US1::US1_ArchiveFile(v42)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                match v48 {
                    US1::US1_OtherFile => {
                        v12
                    }
                    _ => {
                        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v50: Rc<UH0> = split_list_7(v49.clone(), v12);
                        let v51: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v52: Rc<UH0> = method10(v50, v51);
                        let v53: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v54: i32 = 0i32;
                        let v55: i32 = method11(v53.clone(), v52, v54);
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
        let v63: Rc<str> = string_concat(&[&*v5, "/state/agile.spi"]);
        let v64: bool = std::path::Path::new(&*v63).is_file();
        let v116: Rc<str> = if v64 {
            let (mut v65, mut v66): (bool, Rc<str>) = match std::fs::read(&*v63) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v65 {
                let v67: Rc<str> = string_concat(&["rig state read failed: ", &*v63]);
                std::panic::panic_any::<std::string::String>(format!("{}", v67))
            } else {
                let v69: Rc<str> = Rc::<str>::from(v63.replace(&*v3, &*v4));
                let mut v70: Rc<str> = loop_2(v69);
                let v71: Option<usize> = v70.rfind(&*v4);
                let v72: Option<usize> = method3(v71);
                let mut v73: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v74: Option<i32> = v72.map(|x| v73(x));
                let v76: i32 = v74.unwrap_or(-1i32);
                let v77: bool = v76 < 0i32;
                let v81: bool = if v77 {
                    true
                } else {
                    let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v79: bool = v70.ends_with(&*v78);
                    let v80: bool = v79 == false;
                    v80
                };
                let v102: US1 = if v81 {
                    US1::US1_OtherFile
                } else {
                    let v83: usize = ((0i32) as usize);
                    let v84: usize = ((v76) as usize);
                    let v85: Rc<str> = Rc::<str>::from(&(v70)[v83..v84]);
                    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v87: bool = v85.ends_with(&*v86);
                    if v87 {
                        let mut v88: Rc<str> = loop_4(v85);
                        US1::US1_StateFile(v88)
                    } else {
                        let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let mut v91: bool = v85.ends_with(&*v90);
                        if v91 {
                            let mut v92: Rc<str> = loop_5(v85);
                            US1::US1_HistoryFile(v92)
                        } else {
                            let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v95: bool = v85.ends_with(&*v94);
                            if v95 {
                                let v96: Rc<str> = loop_6(v85);
                                US1::US1_ArchiveFile(v96)
                            } else {
                                US1::US1_OtherFile
                            }
                        }
                    }
                };
                match v102 {
                    US1::US1_OtherFile => {
                        v66
                    }
                    _ => {
                        let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v104: Rc<UH0> = split_list_7(v103.clone(), v66);
                        let v105: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v106: Rc<UH0> = method10(v104, v105);
                        let v107: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v108: i32 = 0i32;
                        let v109: i32 = method11(v107.clone(), v106, v108);
                        let v110: Rc<Vec<Rc<str>>> = Rc::new(v107.borrow().clone());
                        let v111: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v110).as_ref().clone()));
                        let v112: Rc<str> = Rc::<str>::from((*v111.borrow()).join(&*v103));
                        v112
                    }
                }
            }
        } else {
            let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v115
        };
        let v117: i64 = v6.wrapping_mul(86400000i64);
        let v118: u64 = (v117 as u64);
        let v119: Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>> = agile_status_changes::rig_agile_status_changes(&v62);
        let v120: i32 = 0i32;
        let v121: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v122: Rc<UH0> = done_since_26(&v119, v120, v118, v121);
        let v123: i32 = 0i32;
        let v124: i32 = method29(v122.clone(), v123);
        let v127: Rc<str> = format_real_21(v124);
        let v129: Rc<str> = elapsed_field_13(v2);
        let v130: Rc<str> = string_concat(&["rig agile sprint retro ok since=", &*v0, " done=", &*v127, &*v129]);
        println!("{}", v130);
        method30(v116, v122);
        0i32
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        retro_since_25(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_sprint_shown(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_agile_sprint_retro(v0: &str, v1: &str) -> i32 {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_agile_sprint_retro_since(v0: &str, v1: &str) -> i32 {
    closure4()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
use agile_task_census::SpiralShape_97df63ebabc3 as Mut1;
pub type SpiralShape_652accfc6a8d = Mut0;
