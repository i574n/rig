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
enum US0 {
    US0_Some(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_NetworkLine,
    US1_MachineLine,
}
pub struct Mut0 { pub l0: i32 }
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut1 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
pub struct Mut2 { pub l0: Rc<str> }
fn method4(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(2i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn fields_of_3(mut v0: Rc<str>, mut v1: US1) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v2: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v6: Rc<str> = match v1 {
        US1::US1_MachineLine => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Machine (\""); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US1::US1_NetworkLine => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Network (\""); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let v7: Option<usize> = v2.find(&*v6);
    let v8: Option<usize> = method4(v7);
    let mut v9: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v10: Option<i32> = v8.map(|x| v9(x));
    let v12: i32 = v10.unwrap_or(-1i32);
    let v13: bool = v12 < 0i32;
    let mut v17: bool = if v13 {
        true
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
        let mut v15: bool = v2.ends_with(&*v14);
        let mut v16: bool = v15 == false;
        v16
    };
    if v17 {
        Rc::new(RefCell::new(vec![]))
    } else {
        let v19: i32 = (v6.len() as i32);
        let v20: i32 = v12.wrapping_add(v19);
        let v21: i32 = (v2.clone().len() as i32);
        let v22: i32 = v21.wrapping_sub(1i32);
        let v23: Rc<str> = string_slice(&v2, v20 as i64, v22 as i64);
        let v24: Rc<str> = loop_5(v23);
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v24.split(&*v25).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        v26
    }
}
fn line_of_2(mut v0: Rc<str>) -> US0 {
    let mut v1: US1 = US1::US1_NetworkLine;
    let v2: Rc<RefCell<Vec<Rc<str>>>> = fields_of_3(v0.clone(), v1);
    let v3: US1 = US1::US1_MachineLine;
    let v4: Rc<RefCell<Vec<Rc<str>>>> = fields_of_3(v0.clone(), v3);
    let v5: i32 = (v2.clone().borrow().len() as i32);
    let v6: bool = v5 == 3i32;
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("network"); } LIT.with(|lit| lit.clone()) };
        US0::US0_Some(v7, v2, v0)
    } else {
        let mut v9: i32 = (v4.clone().borrow().len() as i32);
        let mut v10: bool = v9 == 7i32;
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("machine"); } LIT.with(|lit| lit.clone()) };
            US0::US0_Some(v11, v4, v0)
        } else {
            let v13: bool = v5 > 0i32;
            let mut v15: bool = if v13 {
                true
            } else {
                let v14: bool = v9 > 0i32;
                v14
            };
            if v15 {
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unread"); } LIT.with(|lit| lit.clone()) };
                US0::US0_Some(v16, v2, v0)
            } else {
                US0::US0_None
            }
        }
    }
}
fn method6(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn lines_from_1(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: US0 = line_of_2(v5);
            match v6 {
                US0::US0_None => {
                    let v32: i32 = v1.wrapping_add(1i32);
                    v1 = v32;
                    continue;
                }
                US0::US0_Some(v7, v8, v9) => {
                    let v7: Rc<str> = v7;
                    let v8: Rc<RefCell<Vec<Rc<str>>>> = v8;
                    let mut v9: Rc<str> = v9;
                    let mut v10: i32 = v1.wrapping_add(1i32);
                    let v11: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> = Rc::new(RefCell::new(vec![(v7, v8, v9)]));
                    let v12: i32 = (v2.clone().borrow().len() as i32);
                    let v13: i32 = (v11.clone().borrow().len() as i32);
                    let v14: i32 = v12.wrapping_add(v13);
                    let mut v15: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>::default(); v14 as usize]));
                    let mut v16: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                    while method6(v14, &v16) {
                        let v18: i32 = v16.borrow().l0;
                        let v19: bool = v18 < v12;
                        let (mut v27, mut v28, mut v29): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = if v19 {
                            let (mut v20, mut v21, mut v22): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = v2.borrow()[v18 as usize].clone();
                            (v20, v21, v22)
                        } else {
                            let v23: i32 = v18.wrapping_sub(v12);
                            let (mut v24, mut v25, mut v26): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = v11.borrow()[v23 as usize].clone();
                            (v24, v25, v26)
                        };
                        v15.borrow_mut()[v18 as usize] = (v27, v28, v29);
                        let v30: i32 = v18.wrapping_add(1i32);
                        v16.borrow_mut().l0 = v30;
                        ()
                    };
                    (v1, v2) = (v10, v15);
                    continue;
                }
            }
        }
    }
}
fn fleet_lines_of_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v6: i32 = 0i32;
    let v7: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> = Rc::new(RefCell::new(vec![]));
    let v8: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> = lines_from_1(&v5, v6, v7);
    v8
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> {
        fleet_lines_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method9(v16, v17)
    }
}
fn method11(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure3() -> Rc<dyn Fn((i32)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US2> = Rc::new(move |mut v0: (i32)| -> US2 {
        let mut v1: i32 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn clock_minutes_7(mut v0: Rc<str>) -> Rc<RefCell<Vec<i32>>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH0> = split_list_8(v1, v0);
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH0::UH0_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let v6: Rc<UH0> = v6;
                    match Rc::unwrap_or_clone(v6) {
                        UH0::UH0_Nil => {
                            let v7: Option<i32> = (v3).trim().parse::<i32>().ok();
                            let v8: Option<i32> = method11(v7);
                            let mut v9: Rc<dyn Fn((i32)) -> US2> = closure3();
                            let mut v10: Option<US2> = v8.map(|x| v9(x));
                            let v12: US2 = US2::US2_None;
                            let v13: US2 = v12.clone();
                            let v14: US2 = v10.unwrap_or(v13);
                            let mut v15: Option<i32> = (v5).trim().parse::<i32>().ok();
                            let mut v16: Option<i32> = method11(v15);
                            let mut v17: Option<US2> = v16.map(|x| v9(x));
                            let v19: US2 = US2::US2_None;
                            let v20: US2 = v19.clone();
                            let v21: US2 = v17.unwrap_or(v20);
                            match v14 {
                                US2::US2_Some(v22) => {
                                    let v22: i32 = v22;
                                    match v21 {
                                        US2::US2_Some(v23) => {
                                            let v23: i32 = v23;
                                            let v24: bool = v22 >= 0i32;
                                            let v26: bool = if v24 {
                                                let v25: bool = v22 < 24i32;
                                                v25
                                            } else {
                                                false
                                            };
                                            let v28: bool = if v26 {
                                                let v27: bool = v23 >= 0i32;
                                                v27
                                            } else {
                                                false
                                            };
                                            let v30: bool = if v28 {
                                                let v29: bool = v23 < 60i32;
                                                v29
                                            } else {
                                                false
                                            };
                                            if v30 {
                                                let v31: i32 = v22.wrapping_mul(60i32);
                                                let v32: i32 = v31.wrapping_add(v23);
                                                Rc::new(RefCell::new(vec![v32]))
                                            } else {
                                                Rc::new(RefCell::new(vec![]))
                                            }
                                        }
                                        _ => {
                                            Rc::new(RefCell::new(vec![]))
                                        }
                                    }
                                }
                                _ => {
                                    Rc::new(RefCell::new(vec![]))
                                }
                            }
                        }
                        _ => {
                            Rc::new(RefCell::new(vec![]))
                        }
                    }
                }
                _ => {
                    Rc::new(RefCell::new(vec![]))
                }
            }
        }
        _ => {
            Rc::new(RefCell::new(vec![]))
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<i32>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<i32>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<i32>>> {
        clock_minutes_7(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method14(mut v0: &Rc<RefCell<Mut2>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_13(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method14(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn clock_text_12(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_div(60i32);
    let v2: bool = v1 < 10i32;
    let v6: Rc<str> = if v2 {
        let v3: Rc<str> = format_real_13(v1);
        let v4: Rc<str> = string_concat(&["0", &*v3]);
        v4
    } else {
        format_real_13(v1)
    };
    let v7: Rc<str> = string_concat(&[&*v6, ":"]);
    let v8: i32 = v0.wrapping_rem(60i32);
    let mut v9: bool = v8 < 10i32;
    let v13: Rc<str> = if v9 {
        let mut v10: Rc<str> = format_real_13(v8);
        let v11: Rc<str> = string_concat(&["0", &*v10]);
        v11
    } else {
        format_real_13(v8)
    };
    let v14: Rc<str> = string_concat(&[&*v7, &*v13]);
    v14
}
fn closure4() -> Rc<dyn Fn(i32) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> Rc<str>> = Rc::new(move |mut v0: i32| -> Rc<str> {
        clock_text_12(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_net_fleet_lines(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_net_clock_minutes(v0: &str) -> Rc<RefCell<Vec<i32>>> {
    closure2()(Rc::<str>::from(v0))
}
pub fn rig_net_clock_text(v0: i32) -> Rc<str> {
    closure4()(v0)
}
pub type SpiralShape_2617b4e49837 = Mut0;
pub type SpiralShape_652accfc6a8d = Mut1;
pub type SpiralShape_97df63ebabc3 = Mut2;
