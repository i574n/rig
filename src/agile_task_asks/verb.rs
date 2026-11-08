#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
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
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, Rc<str>, Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_AsksWritten(i32),
    US1_AsksRefused(Rc<str>),
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US3 {
    US3_AskSet(Rc<str>, Rc<str>, Rc<str>),
    US3_AskCleared(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, US3, Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US4 {
    US4_Replaced,
    US4_TemporaryWriteFailed,
    US4_RenameFailed,
    US4_RenameFailedTemporaryKept,
    US4_ReadbackMismatch,
    US4_ChangedSinceRead,
}
#[derive(Clone)]
enum US5 {
    US5_ModuleListed,
    US5_ModuleAdded(Rc<str>),
    US5_NoModulesSection,
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method4(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method5(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method5(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH1> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method4(v16, v17)
    }
}
fn loop_9(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(2i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn quoted_triple_8(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH1> = split_list_3(v1, v0);
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH1> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH1::UH1_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let mut v6: Rc<UH1> = v6;
                    match Rc::unwrap_or_clone(v6) {
                        UH1::UH1_Cons(v7, v8) => {
                            let v7: Rc<str> = v7;
                            let v8: Rc<UH1> = v8;
                            match Rc::unwrap_or_clone(v8) {
                                UH1::UH1_Nil => {
                                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
                                    let v10: bool = v7.ends_with(&*v9);
                                    if v10 {
                                        let mut v11: Rc<str> = loop_9(v7);
                                        let mut v12: bool = &*v3 == "";
                                        let v14: bool = if v12 {
                                            true
                                        } else {
                                            let mut v13: bool = &*v5 == "";
                                            v13
                                        };
                                        if v14 {
                                            US0::US0_None
                                        } else {
                                            US0::US0_Some(v3, v5, v11)
                                        }
                                    } else {
                                        US0::US0_None
                                    }
                                }
                                _ => {
                                    US0::US0_None
                                }
                            }
                        }
                        _ => {
                            US0::US0_None
                        }
                    }
                }
                _ => {
                    US0::US0_None
                }
            }
        }
        _ => {
            US0::US0_None
        }
    }
}
fn method10(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                let mut v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method11(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn method12(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn ask_of_line_7(mut v0: Rc<str>) -> US0 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method10(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0, v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method11(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Asked (\""); } LIT.with(|lit| lit.clone()) };
    let v10: Option<usize> = v8.find(&*v9);
    let mut v11: Option<usize> = method12(v10);
    let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v13: Option<i32> = v11.map(|x| v12(x));
    let v15: i32 = v13.unwrap_or(-1i32);
    let v16: bool = v15 < 0i32;
    if v16 {
        US0::US0_None
    } else {
        let v18: i32 = v15.wrapping_add(11i32);
        let v19: i32 = (v8.clone().len() as i32);
        let v20: i32 = v19.wrapping_sub(1i32);
        let v21: Rc<str> = string_slice(&v8, v18 as i64, v20 as i64);
        quoted_triple_8(v21)
    }
}
fn method6(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: US0 = ask_of_line_7(v2);
                let v10: Rc<UH0> = match v4 {
                    US0::US0_None => {
                        v1
                    }
                    US0::US0_Some(v5, v6, v7) => {
                        let v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        let v7: Rc<str> = v7;
                        Rc::new(UH0::UH0_Cons(v5, v6, v7, v1))
                    }
                };
                (v0, v1) = (v3, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn asks_of_text_1(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH1> = split_list_3(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method6(v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method2(v7, v8)
}
fn method13(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let mut v6: Rc<UH0> = v6;
                v0.borrow_mut().push((v3, v4, v5));
                let v7: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v6, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn asks_of_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    let mut v1: Rc<UH0> = asks_of_text_1(v0);
    let mut v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v3: i32 = 0i32;
    let v4: i32 = method13(v2.clone(), v1, v3);
    let v5: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>)>> = Rc::new(v2.borrow().clone());
    let mut v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    v6
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
        asks_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_16(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method17(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn reported_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US1) -> i32 {
    match v2 {
        US1::US1_AsksRefused(v10) => {
            let v10: Rc<str> = v10;
            let mut v11: Rc<str> = string_concat(&["kit error: agile edit --asked: ", &*v10]);
            eprintln!("{}", v11);
            2i32
        }
        US1::US1_AsksWritten(v3) => {
            let v3: i32 = v3;
            let mut v6: Rc<str> = format_real_16(v3);
            let mut v9: Rc<str> = string_concat(&["kit agile edit ok id=", &*v0, " asked=", &*v1, " asks=", &*v6]);
            println!("{}", v9);
            0i32
        }
    }
}
fn digits_from_20(mut v0: Rc<str>, mut v1: i32) -> bool {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 >= v2;
        if v3 {
            return true;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: i32 = v1.wrapping_add(1i32);
            let mut v6: usize = ((v5) as usize);
            let v7: Rc<str> = Rc::<str>::from(&(v0)[v4..v6]);
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Option<usize> = v8.find(&*v7);
            let v10: Option<usize> = method12(v9);
            let mut v11: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v12: Option<i32> = v10.map(|x| v11(x));
            let v14: i32 = v12.unwrap_or(-1i32);
            let v15: bool = v14 >= 0i32;
            if v15 {
                v1 = v5;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn day_shaped_19(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: i32 = (v0.clone().len() as i32);
    let v5: bool = v4 == 10i32;
    let v10: bool = if v5 {
        let mut v6: usize = ((4i32) as usize);
        let v7: usize = ((5i32) as usize);
        let v8: Rc<str> = Rc::<str>::from(&(v0)[v6..v7]);
        let mut v9: bool = &*v8 == "-";
        v9
    } else {
        false
    };
    let v15: bool = if v10 {
        let mut v11: usize = ((7i32) as usize);
        let mut v12: usize = ((8i32) as usize);
        let mut v13: Rc<str> = Rc::<str>::from(&(v0)[v11..v12]);
        let v14: bool = &*v13 == "-";
        v14
    } else {
        false
    };
    let v18: bool = if v15 {
        let v16: i32 = (v3.clone().len() as i32);
        let v17: bool = v16 == 8i32;
        v17
    } else {
        false
    };
    if v18 {
        let v19: i32 = 0i32;
        digits_from_20(v3, v19)
    } else {
        false
    }
}
fn ask_request_18() -> US2 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v4: i32 = 4i32;
    let v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v6: i32 = 6i32;
    let v7: Rc<str> = usize::try_from(v6).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v8: i32 = 3i32;
    let mut v9: Rc<str> = usize::try_from(v8).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v10: bool = &*v9 != "--asked";
    if v10 {
        US2::US2_None
    } else {
        let mut v12: bool = &*v5 == "--clear";
        let v16: bool = if v12 {
            let mut v13: i32 = 5i32;
            let v14: Rc<str> = usize::try_from(v13).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v15: bool = &*v14 == "";
            v15
        } else {
            false
        };
        if v16 {
            let v17: US3 = US3::US3_AskCleared(v3);
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
            US2::US2_Some(v1, v17, v18)
        } else {
            let v20: i32 = 5i32;
            let v21: Rc<str> = usize::try_from(v20).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v22: bool = &*v21 == "--quote";
            let v26: bool = if v22 {
                let v23: i32 = 7i32;
                let v24: Rc<str> = usize::try_from(v23).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v25: bool = &*v24 == "";
                v25
            } else {
                false
            };
            let v28: bool = if v26 {
                day_shaped_19(v5.clone())
            } else {
                false
            };
            let v38: bool = if v28 {
                let v29: i32 = (v7.clone().len() as i32);
                let v30: i32 = 0i32;
                let v31: i32 = method10(v7.clone(), v29, v30);
                let v32: i32 = v29.wrapping_sub(1i32);
                let v33: Rc<str> = string_slice(&v7.clone(), v31 as i64, v32 as i64);
                let v34: i32 = (v33.clone().len() as i32);
                let v35: i32 = method11(v33.clone(), v34);
                let v36: Rc<str> = string_slice(&v33, 0i32 as i64, v35 as i64);
                let v37: bool = &*v36 != "";
                v37
            } else {
                false
            };
            if v38 {
                let v39: US3 = US3::US3_AskSet(v3, v5.clone(), v7);
                US2::US2_Some(v1, v39, v5)
            } else {
                US2::US2_None
            }
        }
    }
}
fn file_exists_23(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method24(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn text_of_22(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = file_exists_23(v0.clone());
    if v1 {
        let v3: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let v5: Vec<u8> = v3.unwrap();
        let mut v6: Vec<u8> = method24(v5);
        let v8: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v6);
        let v10: std::string::String = v8.unwrap();
        let mut v12: Rc<str> = Rc::<str>::from(String::as_str(&v10));
        v12
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    }
}
fn method26(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4, v5) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            let mut v6: Rc<UH0> = method26(v5, v1);
            Rc::new(UH0::UH0_Cons(v2, v3, v4, v6))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method27(mut v0: US3, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4, v5, v6) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let mut v6: Rc<UH0> = v6;
            let v7: Rc<UH0> = method27(v0.clone(), v6, v2);
            let mut v13: Rc<str> = match v0 {
                US3::US3_AskCleared(v11) => {
                    let mut v11: Rc<str> = v11;
                    v11
                }
                US3::US3_AskSet(v8, v9, v10) => {
                    let v8: Rc<str> = v8;
                    let mut v9: Rc<str> = v9;
                    let v10: Rc<str> = v10;
                    v8
                }
            };
            let v14: bool = &*v3 != &*v13;
            if v14 {
                Rc::new(UH0::UH0_Cons(v3, v4, v5, v7))
            } else {
                v7
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn asks_changed_25(mut v0: Rc<UH0>, mut v1: US3) -> Rc<UH0> {
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH0> = method27(v1.clone(), v0, v2);
    match v1 {
        US3::US3_AskCleared(v10) => {
            let v10: Rc<str> = v10;
            v3
        }
        US3::US3_AskSet(v4, v5, v6) => {
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let mut v6: Rc<str> = v6;
            let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5, v6, v7));
            method26(v3, v8)
        }
    }
}
fn task_known_28(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v3: Rc<str> = string_concat(&["Task (\"", &*v1, "\""]);
    let v4: bool = v0.contains(&*v3);
    v4
}
fn slug_of_32(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v3));
    v6
}
fn quote_text_33(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v6.replace(&*v7, &*v8));
    v9
}
fn row_of_ask_31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = slug_of_32(v0.clone());
    let v10: Rc<str> = quote_text_33(v2);
    let mut v12: Rc<str> = string_concat(&["inl asked_of_", &*v3, " () = Asked (\"", &*v0, "\", \"", &*v1, "\", \"", &*v10, "\")"]);
    v12
}
fn method30(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4, v5) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            let mut v6: Rc<UH1> = method30(v5, v1);
            let v7: Rc<str> = row_of_ask_31(v2, v3, v4);
            let v8: Rc<str> = string_concat(&[&*v7, "\n"]);
            Rc::new(UH1::UH1_Cons(v8, v6))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method34(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn asks_text_29(mut v0: Rc<UH0>) -> Rc<str> {
    let mut v1: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH1> = method30(v0, v1);
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v4: i32 = 0i32;
    let v5: i32 = method34(v3.clone(), v2, v4);
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from((*v7.borrow()).join(&*v8));
    let v10: Rc<str> = string_concat(&["///- --no-respell\nunion task_ask =\n    | Asked :: string * string * string -> task_ask\n", &*v9]);
    v10
}
fn renamed_within_36(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: bool = std::fs::rename(&*v1, &*v0).is_ok();
        if v3 {
            return true;
        } else {
            let v4: bool = v2 <= 1i32;
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
fn replaced_35(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = string_concat(&[&*v0, ".pending"]);
    let v3: bool = std::fs::write(&*v2, &*v1).is_ok();
    let v16: US4 = if v3 {
        let v4: i32 = 40i32;
        let v5: bool = renamed_within_36(v0.clone(), v2.clone(), v4);
        if v5 {
            let mut v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
            if v6 {
                US4::US4_Replaced
            } else {
                US4::US4_ReadbackMismatch
            }
        } else {
            let v10: bool = std::fs::remove_file(&*v2).is_ok() || !std::path::Path::new(&*v2).exists();
            if v10 {
                US4::US4_RenameFailed
            } else {
                US4::US4_RenameFailedTemporaryKept
            }
        }
    } else {
        US4::US4_TemporaryWriteFailed
    };
    match v16 {
        US4::US4_Replaced => {
            true
        }
        _ => {
            false
        }
    }
}
fn method37(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn module_listed_38(mut v0: Rc<str>) -> US5 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n    asks\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.contains(&*v1);
    let v5: bool = if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n    asks"); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.ends_with(&*v3);
        v4
    };
    if v5 {
        US5::US5_ModuleListed
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v0.contains(&*v7);
        if v8 {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n    asks\n"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v9));
            US5::US5_ModuleAdded(v10)
        } else {
            US5::US5_NoModulesSection
        }
    }
}
fn asks_written_21(mut v0: Rc<str>, mut v1: US3) -> US1 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = string_concat(&[&*v4, "/state"]);
    let mut v6: Rc<str> = string_concat(&[&*v5, "/agile.spi"]);
    let v7: Rc<str> = text_of_22(v6);
    let v8: Rc<str> = string_concat(&[&*v5, "/asks.spi"]);
    let mut v9: Rc<str> = text_of_22(v8.clone());
    let v10: Rc<UH0> = asks_of_text_1(v9.clone());
    let mut v11: Rc<UH0> = asks_changed_25(v10, v1.clone());
    let mut v12: bool = &*v7 == "";
    if v12 {
        let mut v13: Rc<str> = string_concat(&["no backlog state at ", &*v5]);
        US1::US1_AsksRefused(v13)
    } else {
        let v20: Rc<str> = match &v1 {
            US3::US3_AskCleared(v18) => {
                let v18: Rc<str> = v18.clone();
                v18
            }
            US3::US3_AskSet(v15, v16, v17) => {
                let v15: Rc<str> = v15.clone();
                let v16: Rc<str> = v16.clone();
                let v17: Rc<str> = v17.clone();
                v15
            }
        };
        let v21: bool = task_known_28(v7, v20);
        let v22: bool = v21 == false;
        if v22 {
            let v28: Rc<str> = match v1 {
                US3::US3_AskCleared(v26) => {
                    let v26: Rc<str> = v26;
                    v26
                }
                US3::US3_AskSet(v23, v24, v25) => {
                    let v23: Rc<str> = v23;
                    let v24: Rc<str> = v24;
                    let v25: Rc<str> = v25;
                    v23
                }
            };
            let v30: Rc<str> = string_concat(&["no task ", &*v28, " in state/agile.spi"]);
            US1::US1_AsksRefused(v30)
        } else {
            let v32: Rc<str> = text_of_22(v8.clone());
            let v33: bool = &*v32 != &*v9;
            if v33 {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/asks.spi changed since it was read; run again"); } LIT.with(|lit| lit.clone()) };
                US1::US1_AsksRefused(v34)
            } else {
                let v36: Rc<str> = asks_text_29(v11.clone());
                let v37: bool = replaced_35(v8, v36);
                let v38: bool = v37 == false;
                if v38 {
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/asks.spi could not be written"); } LIT.with(|lit| lit.clone()) };
                    US1::US1_AsksRefused(v39)
                } else {
                    let v41: i32 = 0i32;
                    let v42: i32 = method37(v11, v41);
                    let v43: Rc<str> = string_concat(&[&*v5, "/package.spiproj"]);
                    let v44: Rc<str> = text_of_22(v43.clone());
                    let v45: US5 = module_listed_38(v44);
                    match v45 {
                        US5::US5_ModuleAdded(v49) => {
                            let v49: Rc<str> = v49;
                            let v50: bool = replaced_35(v43, v49);
                            if v50 {
                                US1::US1_AsksWritten(v42)
                            } else {
                                let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj not updated"); } LIT.with(|lit| lit.clone()) };
                                US1::US1_AsksRefused(v52)
                            }
                        }
                        US5::US5_ModuleListed => {
                            US1::US1_AsksWritten(v42)
                        }
                        US5::US5_NoModulesSection => {
                            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj has no modules section to list asks in"); } LIT.with(|lit| lit.clone()) };
                            US1::US1_AsksRefused(v47)
                        }
                    }
                }
            }
        }
    }
}
fn ask_edit_run_14() -> i32 {
    let mut v0: US2 = ask_request_18();
    match v0 {
        US2::US2_None => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile edit <root> <id> --asked <yyyy-mm-dd> --quote <the user's words> | --asked --clear"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v12);
            2i32
        }
        US2::US2_Some(v1, v2, v3) => {
            let mut v1: Rc<str> = v1;
            let mut v2: US3 = v2;
            let v3: Rc<str> = v3;
            let mut v9: Rc<str> = match &v2 {
                US3::US3_AskCleared(v7) => {
                    let v7: Rc<str> = v7.clone();
                    v7
                }
                US3::US3_AskSet(v4, v5, v6) => {
                    let v4: Rc<str> = v4.clone();
                    let v5: Rc<str> = v5.clone();
                    let mut v6: Rc<str> = v6.clone();
                    v4
                }
            };
            let v10: US1 = asks_written_21(v1, v2);
            reported_15(v9, v3, v10)
        }
    }
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        ask_edit_run_14()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_asks(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_agile_ask_edit_run() -> i32 {
    closure2()()
}
pub type SpiralShape_accbf6de6842 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
