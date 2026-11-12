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
#[derive(Clone)]
enum US2 {
    US2_CaseAdded(Rc<str>),
    US2_OutcomeRose(Rc<str>, Rc<str>, Rc<str>),
    US2_MeasureRose(Rc<str>, Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(US2, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_NonEmpty(US2, Rc<UH0>),
}
#[derive(Clone)]
enum US3 {
    US3_OutcomeChanged(Rc<str>, Rc<str>, Rc<str>),
    US3_MeasureMoved(Rc<str>, Rc<str>, Rc<str>),
    US3_OrderChanged,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US3, Rc<UH1>),
}
#[derive(Clone)]
enum US4 {
    US4_NonEmpty(US3, Rc<UH1>),
}
#[derive(Clone)]
enum US6 {
    US6_CaseLost(Rc<str>),
    US6_OutcomeFell(Rc<str>, Rc<str>, Rc<str>),
    US6_MeasureFell(Rc<str>, Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US6, Rc<UH2>),
}
#[derive(Clone)]
enum US5 {
    US5_NonEmpty(US6, Rc<UH2>),
}
#[derive(Clone)]
enum US0 {
    US0_Equal,
    US0_Dominates(US1, Rc<UH1>),
    US0_Changed(US4),
    US0_Regressed(US5, Rc<UH0>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, i32, Rc<UH3>),
}
#[derive(Clone)]
enum US7 {
    US7_Higher,
    US7_Lower,
}
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(Rc<str>, US7, u64, Rc<UH4>),
}
#[derive(Clone)]
enum US9 {
    US9_Some(u64, u64),
    US9_None,
}
#[derive(Clone)]
enum UH5 {
    UH5_Nil,
    UH5_Cons(Rc<str>, Rc<str>, US9, Rc<UH5>),
}
#[derive(Clone)]
enum US8 {
    US8_Side(Rc<UH5>),
    US8_SideMalformed(Rc<str>),
}
#[derive(Clone)]
enum UH6 {
    UH6_Nil,
    UH6_Cons(Rc<str>, Rc<UH6>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH6> }
#[derive(Clone)]
enum US10 {
    US10_Some(u64),
    US10_None,
}
#[derive(Clone)]
enum US11 {
    US11_Lost(US6),
    US11_Won(US2),
    US11_Moved(US3),
}
#[derive(Clone)]
enum UH7 {
    UH7_Nil,
    UH7_Cons(US11, Rc<UH7>),
}
#[derive(Clone)]
enum US12 {
    US12_Some(Rc<str>, Rc<str>, US9),
    US12_None,
}
#[derive(Clone)]
enum US13 {
    US13_Some(i32),
    US13_None,
}
#[derive(Clone)]
enum US14 {
    US14_Some(Rc<str>, i32),
    US14_None,
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US15 {
    US15_Some(Rc<str>, US7, u64),
    US15_None,
}
#[derive(Clone)]
enum US16 {
    US16_Worse,
    US16_Better,
    US16_Within,
}
#[derive(Clone)]
enum UH8 {
    UH8_Nil,
    UH8_Cons(Rc<UH2>, Rc<UH8>),
}
#[derive(Clone)]
enum UH9 {
    UH9_Nil,
    UH9_Cons(Rc<UH0>, Rc<UH9>),
}
#[derive(Clone)]
enum UH10 {
    UH10_Nil,
    UH10_Cons(Rc<UH1>, Rc<UH10>),
}
#[derive(Clone)]
enum US17 {
    US17_Some(Rc<str>),
    US17_None,
}
#[derive(Clone)]
enum US18 {
    US18_Rule(Rc<UH3>, Rc<UH4>),
    US18_RuleMalformed(Rc<str>),
}
#[derive(Clone)]
enum US19 {
    US19_Some(US7),
    US19_None,
}
fn refused_2(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = string_concat(&["rig error: verdict: ", &*v0]);
    eprintln!("{}", v1);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: verdict <before> <after> [--rule <file>] (each side: case<TAB>result[<TAB>measure] per line; rule: rank <result> <n> and measure <case prefix|*> <higher|lower> <tolerance %>; exit Equal 0, Regressed 1, Changed 2, Dominates 3)"); } LIT.with(|lit| lit.clone()) };
    eprintln!("{}", v2);
    64i32
}
fn verdict_exit_3(mut v0: US0) -> i32 {
    match v0 {
        US0::US0_Changed(v3) => {
            let mut v3: US4 = v3;
            2i32
        }
        US0::US0_Dominates(v4, v5) => {
            let mut v4: US1 = v4;
            let v5: Rc<UH1> = v5;
            3i32
        }
        US0::US0_Equal => {
            0i32
        }
        US0::US0_Regressed(v1, v2) => {
            let mut v1: US5 = v1;
            let v2: Rc<UH0> = v2;
            1i32
        }
    }
}
fn method6(mut v0: Rc<UH6>, mut v1: Rc<UH6>) -> Rc<UH6> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH6::UH6_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH6> = v3;
                let mut v4: Rc<UH6> = Rc::new(UH6::UH6_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH6::UH6_Nil => {
                return v1;
            }
        }
    }
}
fn method7(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH6> {
    let v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH6::UH6_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method7(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH6> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH6> = Rc::new(UH6::UH6_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH6> = v9.borrow().l1.clone();
        let v17: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
        method6(v16, v17)
    }
}
fn closure1(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>, Rc<str>, US9) -> bool> {
    Rc::new(move |mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US9| -> bool {
        let mut v4: bool = &*v1 == &*v0;
        v4
    })
}
fn method12(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>, Rc<str>, US9) -> bool> {
    closure1(v0)
}
fn method11(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>, Rc<str>, US9) -> bool>, mut v2: Rc<UH5>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH5::UH5_Cons(v3, v4, v5, v6) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let v5: US9 = v5;
                let v6: Rc<UH5> = v6;
                let v7: bool = v1(v3, v4, v5);
                if v7 {
                    return true;
                } else {
                    let v8: Rc<dyn Fn(Rc<str>, Rc<str>, US9) -> bool> = method12(v0.clone());
                    (v1, v2) = (v8, v6);
                    continue;
                }
            }
            UH5::UH5_Nil => {
                return false;
            }
        }
    }
}
fn present_10(mut v0: Rc<UH5>, mut v1: Rc<str>) -> bool {
    let v2: Rc<dyn Fn(Rc<str>, Rc<str>, US9) -> bool> = method12(v1.clone());
    method11(v1, v2, v0)
}
fn method15(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US10> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US10> = Rc::new(move |mut v0: (u64)| -> US10 {
        let mut v1: u64 = (v0);
        US10::US10_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn whole_of_14(mut v0: Rc<str>) -> US10 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method15(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US10> = closure2();
    let mut v4: Option<US10> = v2.map(|x| v3(x));
    let v6: US10 = US10::US10_None;
    let v7: US10 = v6.clone();
    let v8: US10 = v4.unwrap_or(v7);
    v8
}
fn ten_to_16(mut v0: i32) -> u64 {
    let mut v1: bool = v0 <= 0i32;
    if v1 {
        1u64
    } else {
        let v2: i32 = v0.wrapping_sub(1i32);
        let mut v3: u64 = ten_to_16(v2);
        let mut v4: u64 = 10u64.wrapping_mul(v3);
        v4
    }
}
fn ratio_of_13(mut v0: Rc<str>) -> US9 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH6> = split_list_5(v1, v0);
    match Rc::unwrap_or_clone(v2) {
        UH6::UH6_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<UH6> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH6::UH6_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let v6: Rc<UH6> = v6;
                    match Rc::unwrap_or_clone(v6) {
                        UH6::UH6_Nil => {
                            let v7: US10 = whole_of_14(v3);
                            let v8: US10 = whole_of_14(v5);
                            match v7 {
                                US10::US10_Some(v9) => {
                                    let mut v9: u64 = v9;
                                    match v8 {
                                        US10::US10_Some(v10) => {
                                            let v10: u64 = v10;
                                            let v11: bool = v10 > 0u64;
                                            if v11 {
                                                US9::US9_Some(v9, v10)
                                            } else {
                                                US9::US9_None
                                            }
                                        }
                                        _ => {
                                            US9::US9_None
                                        }
                                    }
                                }
                                _ => {
                                    US9::US9_None
                                }
                            }
                        }
                        _ => {
                            US9::US9_None
                        }
                    }
                }
                UH6::UH6_Nil => {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                    let mut v22: Rc<UH6> = split_list_5(v21, v3);
                    match Rc::unwrap_or_clone(v22) {
                        UH6::UH6_Cons(v23, v24) => {
                            let mut v23: Rc<str> = v23;
                            let v24: Rc<UH6> = v24;
                            match Rc::unwrap_or_clone(v24) {
                                UH6::UH6_Cons(v31, v32) => {
                                    let v31: Rc<str> = v31;
                                    let v32: Rc<UH6> = v32;
                                    match Rc::unwrap_or_clone(v32) {
                                        UH6::UH6_Nil => {
                                            let v33: i32 = (v31.clone().len() as i32);
                                            let v34: u64 = ten_to_16(v33);
                                            let v35: US10 = whole_of_14(v23);
                                            let v36: US10 = whole_of_14(v31);
                                            match v35 {
                                                US10::US10_Some(v37) => {
                                                    let v37: u64 = v37;
                                                    match v36 {
                                                        US10::US10_Some(v38) => {
                                                            let v38: u64 = v38;
                                                            let v39: u64 = v37.wrapping_mul(v34);
                                                            let v40: u64 = v39.wrapping_add(v38);
                                                            US9::US9_Some(v40, v34)
                                                        }
                                                        _ => {
                                                            US9::US9_None
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    US9::US9_None
                                                }
                                            }
                                        }
                                        _ => {
                                            US9::US9_None
                                        }
                                    }
                                }
                                UH6::UH6_Nil => {
                                    let v25: US10 = whole_of_14(v23);
                                    match v25 {
                                        US10::US10_None => {
                                            US9::US9_None
                                        }
                                        US10::US10_Some(v26) => {
                                            let v26: u64 = v26;
                                            US9::US9_Some(v26, 1u64)
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            US9::US9_None
                        }
                    }
                }
            }
        }
        _ => {
            US9::US9_None
        }
    }
}
fn row_added_9(mut v0: US8, mut v1: Rc<str>) -> US8 {
    let v2: Rc<str> = Rc::<str>::from((v1).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    match &v0 {
        US8::US8_Side(v5) => {
            let v5: Rc<UH5> = v5.clone();
            let v6: bool = &*v2 == "";
            let mut v9: bool = if v6 {
                true
            } else {
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
                let v8: bool = v2.starts_with(&*v7);
                v8
            };
            if v9 {
                v0
            } else {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v11: Rc<UH6> = split_list_5(v10, v2.clone());
                match Rc::unwrap_or_clone(v11) {
                    UH6::UH6_Cons(v12, v13) => {
                        let v12: Rc<str> = v12;
                        let v13: Rc<UH6> = v13;
                        match Rc::unwrap_or_clone(v13) {
                            UH6::UH6_Cons(v14, v15) => {
                                let v14: Rc<str> = v14;
                                let v15: Rc<UH6> = v15;
                                let v16: bool = present_10(v5.clone(), v12.clone());
                                if v16 {
                                    let v17: Rc<str> = string_concat(&["case listed twice: ", &*v12]);
                                    US8::US8_SideMalformed(v17)
                                } else {
                                    match Rc::unwrap_or_clone(v15) {
                                        UH6::UH6_Cons(v22, v23) => {
                                            let mut v22: Rc<str> = v22;
                                            let mut v23: Rc<UH6> = v23;
                                            match Rc::unwrap_or_clone(v23) {
                                                UH6::UH6_Nil => {
                                                    let v24: US9 = ratio_of_13(v22.clone());
                                                    match v24 {
                                                        US9::US9_None => {
                                                            let v30: Rc<str> = string_concat(&["not a measure (n, n.n or n/m): ", &*v22]);
                                                            US8::US8_SideMalformed(v30)
                                                        }
                                                        US9::US9_Some(v25, v26) => {
                                                            let v25: u64 = v25;
                                                            let v26: u64 = v26;
                                                            let v27: US9 = US9::US9_Some(v25, v26);
                                                            let v28: Rc<UH5> = Rc::new(UH5::UH5_Cons(v12, v14, v27, v5));
                                                            US8::US8_Side(v28)
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    let v34: Rc<str> = string_concat(&["expected case<TAB>result[<TAB>measure], got: ", &*v2]);
                                                    US8::US8_SideMalformed(v34)
                                                }
                                            }
                                        }
                                        UH6::UH6_Nil => {
                                            let v19: US9 = US9::US9_None;
                                            let v20: Rc<UH5> = Rc::new(UH5::UH5_Cons(v12, v14, v19, v5));
                                            US8::US8_Side(v20)
                                        }
                                    }
                                }
                            }
                            _ => {
                                let v40: Rc<str> = string_concat(&["expected case<TAB>result[<TAB>measure], got: ", &*v2]);
                                US8::US8_SideMalformed(v40)
                            }
                        }
                    }
                    _ => {
                        let v43: Rc<str> = string_concat(&["expected case<TAB>result[<TAB>measure], got: ", &*v2]);
                        US8::US8_SideMalformed(v43)
                    }
                }
            }
        }
        US8::US8_SideMalformed(v3) => {
            let mut v3: Rc<str> = v3.clone();
            US8::US8_SideMalformed(v3)
        }
    }
}
fn method8(mut v0: Rc<UH6>, mut v1: US8) -> US8 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH6::UH6_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH6> = v3;
                let mut v4: US8 = row_added_9(v1, v2);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH6::UH6_Nil => {
                return v1;
            }
        }
    }
}
fn method17(mut v0: Rc<UH5>, mut v1: Rc<UH5>) -> Rc<UH5> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH5::UH5_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let mut v4: US9 = v4;
                let v5: Rc<UH5> = v5;
                let v6: Rc<UH5> = Rc::new(UH5::UH5_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH5::UH5_Nil => {
                return v1;
            }
        }
    }
}
fn side_of_4(mut v0: Rc<str>) -> US8 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH6> = split_list_5(v1, v0);
    let mut v3: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: US8 = US8::US8_Side(v3);
    let v5: US8 = method8(v2, v4);
    match v5 {
        US8::US8_Side(v6) => {
            let v6: Rc<UH5> = v6;
            let v7: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
            let v8: Rc<UH5> = method17(v6, v7);
            US8::US8_Side(v8)
        }
        US8::US8_SideMalformed(v10) => {
            let v10: Rc<str> = v10;
            US8::US8_SideMalformed(v10)
        }
    }
}
fn method18(mut v0: Rc<UH5>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH5::UH5_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let mut v4: US9 = v4;
                let v5: Rc<UH5> = v5;
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH5::UH5_Nil => {
                return v1;
            }
        }
    }
}
fn method20(mut v0: Rc<UH7>, mut v1: Rc<UH7>) -> Rc<UH7> {
    match Rc::unwrap_or_clone(v0) {
        UH7::UH7_Cons(v2, v3) => {
            let v2: US11 = v2;
            let mut v3: Rc<UH7> = v3;
            let mut v4: Rc<UH7> = method20(v3, v1);
            Rc::new(UH7::UH7_Cons(v2, v4))
        }
        UH7::UH7_Nil => {
            v1
        }
    }
}
fn method21(mut v0: Rc<UH5>, mut v1: Rc<UH5>, mut v2: Rc<UH5>) -> Rc<UH5> {
    match Rc::unwrap_or_clone(v1) {
        UH5::UH5_Cons(v3, v4, v5, v6) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let v5: US9 = v5;
            let v6: Rc<UH5> = v6;
            let v7: Rc<UH5> = method21(v0.clone(), v6, v2);
            let v8: bool = present_10(v0, v3.clone());
            let mut v9: bool = v8 == false;
            if v9 {
                Rc::new(UH5::UH5_Cons(v3, v4, v5, v7))
            } else {
                v7
            }
        }
        UH5::UH5_Nil => {
            v2
        }
    }
}
fn method22(mut v0: Rc<UH5>, mut v1: Rc<UH7>) -> Rc<UH7> {
    match Rc::unwrap_or_clone(v0) {
        UH5::UH5_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let mut v4: US9 = v4;
            let v5: Rc<UH5> = v5;
            let v6: Rc<UH7> = method22(v5, v1);
            let v7: US6 = US6::US6_CaseLost(v2);
            let v8: US11 = US11::US11_Lost(v7);
            Rc::new(UH7::UH7_Cons(v8, v6))
        }
        UH5::UH5_Nil => {
            v1
        }
    }
}
fn method23(mut v0: Rc<UH5>, mut v1: Rc<UH5>, mut v2: Rc<UH5>) -> Rc<UH5> {
    match Rc::unwrap_or_clone(v1) {
        UH5::UH5_Cons(v3, v4, v5, v6) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let v5: US9 = v5;
            let v6: Rc<UH5> = v6;
            let v7: Rc<UH5> = method23(v0.clone(), v6, v2);
            let v8: bool = present_10(v0, v3.clone());
            let mut v9: bool = v8 == false;
            if v9 {
                Rc::new(UH5::UH5_Cons(v3, v4, v5, v7))
            } else {
                v7
            }
        }
        UH5::UH5_Nil => {
            v2
        }
    }
}
fn method24(mut v0: Rc<UH5>, mut v1: Rc<UH7>) -> Rc<UH7> {
    match Rc::unwrap_or_clone(v0) {
        UH5::UH5_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let mut v4: US9 = v4;
            let v5: Rc<UH5> = v5;
            let v6: Rc<UH7> = method24(v5, v1);
            let v7: US2 = US2::US2_CaseAdded(v2);
            let v8: US11 = US11::US11_Won(v7);
            Rc::new(UH7::UH7_Cons(v8, v6))
        }
        UH5::UH5_Nil => {
            v1
        }
    }
}
fn method27(mut v0: Rc<str>, mut v1: Rc<UH5>) -> US12 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH5::UH5_Cons(v3, v4, v5, v6) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let v5: US9 = v5;
                let v6: Rc<UH5> = v6;
                let v7: bool = &*v3 == &*v0;
                if v7 {
                    return US12::US12_Some(v3, v4, v5);
                } else {
                    v1 = v6;
                    continue;
                }
            }
            UH5::UH5_Nil => {
                return US12::US12_None;
            }
        }
    }
}
fn found_26(mut v0: Rc<UH5>, mut v1: Rc<str>) -> US12 {
    method27(v1, v0)
}
fn method30(mut v0: Rc<str>, mut v1: Rc<UH3>) -> US14 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH3::UH3_Cons(v3, v4, v5) => {
                let mut v3: Rc<str> = v3;
                let mut v4: i32 = v4;
                let v5: Rc<UH3> = v5;
                let v6: bool = &*v3 == &*v0;
                if v6 {
                    return US14::US14_Some(v3, v4);
                } else {
                    v1 = v5;
                    continue;
                }
            }
            UH3::UH3_Nil => {
                return US14::US14_None;
            }
        }
    }
}
fn rank_of_29(mut v0: Rc<UH3>, mut v1: &Rc<UH4>, mut v2: Rc<str>) -> US13 {
    let mut v3: US14 = method30(v2, v0);
    match v3 {
        US14::US14_None => {
            US13::US13_None
        }
        US14::US14_Some(v4, v5) => {
            let mut v4: Rc<str> = v4;
            let v5: i32 = v5;
            US13::US13_Some(v5)
        }
    }
}
fn outcome_findings_28(mut v0: Rc<UH3>, mut v1: &Rc<UH4>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US9, mut v5: &Rc<str>, mut v6: Rc<str>, mut v7: US9) -> Rc<UH7> {
    let v8: bool = &*v3 == &*v6;
    if v8 {
        { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v10: US13 = rank_of_29(v0.clone(), v1, v3.clone());
        let v11: US13 = rank_of_29(v0, v1, v6.clone());
        match v10 {
            US13::US13_Some(v12) => {
                let v12: i32 = v12;
                match v11 {
                    US13::US13_Some(v13) => {
                        let v13: i32 = v13;
                        let v14: bool = v13 < v12;
                        if v14 {
                            let v15: US6 = US6::US6_OutcomeFell(v2, v3, v6);
                            let v16: US11 = US11::US11_Lost(v15);
                            let v17: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
                            Rc::new(UH7::UH7_Cons(v16, v17))
                        } else {
                            let v19: bool = v13 > v12;
                            if v19 {
                                let v20: US2 = US2::US2_OutcomeRose(v2, v3, v6);
                                let v21: US11 = US11::US11_Won(v20);
                                let mut v22: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
                                Rc::new(UH7::UH7_Cons(v21, v22))
                            } else {
                                let v24: US3 = US3::US3_OutcomeChanged(v2, v3, v6);
                                let v25: US11 = US11::US11_Moved(v24);
                                let v26: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
                                Rc::new(UH7::UH7_Cons(v25, v26))
                            }
                        }
                    }
                    _ => {
                        let v30: US3 = US3::US3_OutcomeChanged(v2, v3, v6);
                        let v31: US11 = US11::US11_Moved(v30);
                        let v32: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH7::UH7_Cons(v31, v32))
                    }
                }
            }
            _ => {
                let v35: US3 = US3::US3_OutcomeChanged(v2, v3, v6);
                let v36: US11 = US11::US11_Moved(v35);
                let v37: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
                Rc::new(UH7::UH7_Cons(v36, v37))
            }
        }
    }
}
fn method35(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_34(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method35(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn ratio_text_33(mut v0: u64, mut v1: u64) -> Rc<str> {
    let v2: bool = v1 == 1u64;
    if v2 {
        format_real_34(v0)
    } else {
        let mut v4: Rc<str> = format_real_34(v0);
        let v6: Rc<str> = format_real_34(v1);
        let v7: Rc<str> = string_concat(&[&*v4, "/", &*v6]);
        v7
    }
}
fn measure_text_32(mut v0: US9) -> Rc<str> {
    match v0 {
        US9::US9_None => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US9::US9_Some(v1, v2) => {
            let mut v1: u64 = v1;
            let v2: u64 = v2;
            ratio_text_33(v1, v2)
        }
    }
}
fn specificity_38(mut v0: Rc<str>, mut v1: US7, mut v2: u64) -> i32 {
    let mut v3: bool = &*v0 == "*";
    if v3 {
        0i32
    } else {
        let mut v4: i32 = (v0.len() as i32);
        v4
    }
}
fn method37(mut v0: Rc<str>, mut v1: Rc<UH4>, mut v2: US15) -> US15 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH4::UH4_Cons(v3, v4, v5, v6) => {
                let mut v3: Rc<str> = v3;
                let mut v4: US7 = v4;
                let v5: u64 = v5;
                let v6: Rc<UH4> = v6;
                let v7: bool = &*v3 == "*";
                let mut v9: bool = if v7 {
                    true
                } else {
                    let v8: bool = v0.starts_with(&*v3);
                    v8
                };
                let v10: bool = v9 == false;
                let v21: US15 = if v10 {
                    v2
                } else {
                    match &v2 {
                        US15::US15_Some(v11, v12, v13) => {
                            let v11: Rc<str> = v11.clone();
                            let v12: US7 = v12.clone();
                            let v13: u64 = *v13;
                            let v14: i32 = specificity_38(v11, v12, v13);
                            let v15: i32 = specificity_38(v3.clone(), v4.clone(), v5);
                            let v16: bool = v14 >= v15;
                            if v16 {
                                v2
                            } else {
                                US15::US15_Some(v3, v4, v5)
                            }
                        }
                        _ => {
                            US15::US15_Some(v3, v4, v5)
                        }
                    }
                };
                (v1, v2) = (v6, v21);
                continue;
            }
            UH4::UH4_Nil => {
                return v2;
            }
        }
    }
}
fn measure_rule_for_36(mut v0: &Rc<UH3>, mut v1: Rc<UH4>, mut v2: Rc<str>) -> US15 {
    let mut v3: US15 = US15::US15_None;
    method37(v2, v1, v3)
}
fn movement_of_39(mut v0: &Rc<str>, mut v1: US7, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64) -> US16 {
    let v7: u64 = v5.wrapping_mul(v4);
    let v8: u64 = v7.wrapping_mul(100u64);
    let mut v9: u64 = v3.wrapping_mul(v6);
    let v10: u64 = 100u64.wrapping_add(v2);
    let v11: u64 = v9.wrapping_mul(v10);
    let v12: bool = v8 > v11;
    let v13: bool = v2 < 100u64;
    let v16: u64 = if v13 {
        let v14: u64 = 100u64.wrapping_sub(v2);
        let v15: u64 = v9.wrapping_mul(v14);
        v15
    } else {
        0u64
    };
    let v17: bool = v8 < v16;
    let v18: bool = v2 >= 100u64;
    if v18 {
        US16::US16_Within
    } else {
        match v1 {
            US7::US7_Higher => {
                if v17 {
                    US16::US16_Worse
                } else {
                    if v12 {
                        US16::US16_Better
                    } else {
                        US16::US16_Within
                    }
                }
            }
            US7::US7_Lower => {
                if v12 {
                    US16::US16_Worse
                } else {
                    if v17 {
                        US16::US16_Better
                    } else {
                        US16::US16_Within
                    }
                }
            }
        }
    }
}
fn measure_findings_31(mut v0: &Rc<UH3>, mut v1: Rc<UH4>, mut v2: Rc<str>, mut v3: &Rc<str>, mut v4: US9, mut v5: &Rc<str>, mut v6: &Rc<str>, mut v7: US9) -> Rc<UH7> {
    let v8: Rc<str> = measure_text_32(v4.clone());
    let mut v9: Rc<str> = measure_text_32(v7.clone());
    match v4 {
        US9::US9_None => {
            match v7 {
                US9::US9_None => {
                    { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) }
                }
                _ => {
                    let v48: US3 = US3::US3_MeasureMoved(v2, v8, v9);
                    let v49: US11 = US11::US11_Moved(v48);
                    let v50: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH7::UH7_Cons(v49, v50))
                }
            }
        }
        US9::US9_Some(v10, v11) => {
            let v10: u64 = v10;
            let v11: u64 = v11;
            match v7 {
                US9::US9_Some(v12, v13) => {
                    let v12: u64 = v12;
                    let v13: u64 = v13;
                    let v14: u64 = v10.wrapping_mul(v13);
                    let v15: u64 = v12.wrapping_mul(v11);
                    let v16: bool = v14 == v15;
                    if v16 {
                        { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) }
                    } else {
                        let v18: US15 = measure_rule_for_36(v0, v1, v2.clone());
                        match v18 {
                            US15::US15_None => {
                                let v19: US3 = US3::US3_MeasureMoved(v2, v8, v9);
                                let v20: US11 = US11::US11_Moved(v19);
                                let v21: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
                                Rc::new(UH7::UH7_Cons(v20, v21))
                            }
                            US15::US15_Some(v23, v24, v25) => {
                                let mut v23: Rc<str> = v23;
                                let v24: US7 = v24;
                                let v25: u64 = v25;
                                let v26: US16 = movement_of_39(&v23, v24, v25, v10, v11, v12, v13);
                                match v26 {
                                    US16::US16_Better => {
                                        let v31: US2 = US2::US2_MeasureRose(v2, v8, v9);
                                        let v32: US11 = US11::US11_Won(v31);
                                        let v33: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
                                        Rc::new(UH7::UH7_Cons(v32, v33))
                                    }
                                    US16::US16_Within => {
                                        { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) }
                                    }
                                    US16::US16_Worse => {
                                        let v27: US6 = US6::US6_MeasureFell(v2, v8, v9);
                                        let v28: US11 = US11::US11_Lost(v27);
                                        let v29: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
                                        Rc::new(UH7::UH7_Cons(v28, v29))
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    let v42: US3 = US3::US3_MeasureMoved(v2, v8, v9);
                    let v43: US11 = US11::US11_Moved(v42);
                    let v44: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH7::UH7_Cons(v43, v44))
                }
            }
        }
    }
}
fn method25(mut v0: Rc<UH3>, mut v1: Rc<UH4>, mut v2: Rc<UH5>, mut v3: Rc<UH5>, mut v4: Rc<UH7>) -> Rc<UH7> {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH5::UH5_Cons(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5;
                let v6: Rc<str> = v6;
                let v7: US9 = v7;
                let v8: Rc<UH5> = v8;
                let mut v9: US12 = found_26(v2.clone(), v5.clone());
                let v18: Rc<UH7> = match v9 {
                    US12::US12_None => {
                        v4
                    }
                    US12::US12_Some(v10, v11, v12) => {
                        let v10: Rc<str> = v10;
                        let v11: Rc<str> = v11;
                        let v12: US9 = v12;
                        let v13: Rc<UH7> = outcome_findings_28(v0.clone(), &v1, v5.clone(), v6.clone(), v7.clone(), &v10, v11.clone(), v12.clone());
                        let v14: Rc<UH7> = measure_findings_31(&v0, v1.clone(), v5, &v6, v7, &v10, &v11, v12);
                        let v15: Rc<UH7> = method20(v13, v14);
                        method20(v4, v15)
                    }
                };
                (v3, v4) = (v8, v18);
                continue;
            }
            UH5::UH5_Nil => {
                return v4;
            }
        }
    }
}
fn method41(mut v0: Rc<UH5>, mut v1: Rc<UH6>) -> Rc<UH6> {
    match Rc::unwrap_or_clone(v0) {
        UH5::UH5_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let mut v4: US9 = v4;
            let v5: Rc<UH5> = v5;
            let v6: Rc<UH6> = method41(v5, v1);
            Rc::new(UH6::UH6_Cons(v2, v6))
        }
        UH5::UH5_Nil => {
            v1
        }
    }
}
fn method42(mut v0: Rc<UH5>, mut v1: Rc<UH5>, mut v2: Rc<UH5>) -> Rc<UH5> {
    match Rc::unwrap_or_clone(v1) {
        UH5::UH5_Cons(v3, v4, v5, v6) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let v5: US9 = v5;
            let v6: Rc<UH5> = v6;
            let v7: Rc<UH5> = method42(v0.clone(), v6, v2);
            let v8: bool = present_10(v0, v3.clone());
            if v8 {
                Rc::new(UH5::UH5_Cons(v3, v4, v5, v7))
            } else {
                v7
            }
        }
        UH5::UH5_Nil => {
            v2
        }
    }
}
fn shared_order_40(mut v0: Rc<UH5>, mut v1: Rc<UH5>) -> Rc<UH6> {
    let v2: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH5> = method42(v1, v0, v2);
    let mut v4: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
    method41(v3, v4)
}
fn method43(mut v0: Rc<UH6>, mut v1: Rc<UH6>) -> bool {
    loop {
        match (Rc::unwrap_or_clone(v1), Rc::unwrap_or_clone(v0)) {
            (UH6::UH6_Cons(v2, v3), UH6::UH6_Cons(v4, v5)) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH6> = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH6> = v5;
                let v6: bool = &*v2 == &*v4;
                if v6 {
                    (v0, v1) = (v5, v3);
                    continue;
                } else {
                    return false;
                }
            }
            (UH6::UH6_Nil, UH6::UH6_Nil) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}
fn findings_of_19(mut v0: Rc<UH3>, mut v1: Rc<UH4>, mut v2: Rc<UH5>, mut v3: Rc<UH5>) -> Rc<UH7> {
    let mut v4: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH5> = method21(v3.clone(), v2.clone(), v4);
    let v6: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH7> = method22(v5, v6);
    let v8: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH5> = method23(v2.clone(), v3.clone(), v8);
    let v10: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH7> = method24(v9, v10);
    let v12: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH7> = method25(v0, v1, v3.clone(), v2.clone(), v12);
    let v14: Rc<UH6> = shared_order_40(v2.clone(), v3.clone());
    let v15: Rc<UH6> = shared_order_40(v3, v2);
    let v16: bool = method43(v15, v14);
    let mut v22: Rc<UH7> = if v16 {
        { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v18: US3 = US3::US3_OrderChanged;
        let v19: US11 = US11::US11_Moved(v18);
        let v20: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH7::UH7_Cons(v19, v20))
    };
    let mut v23: Rc<UH7> = method20(v11, v22);
    let v24: Rc<UH7> = method20(v13, v23);
    method20(v7, v24)
}
fn moved_text_46(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v9: Rc<str> = string_concat(&[&*v0, " ", &*v1, " ", &*v2, " -> ", &*v3]);
    v9
}
fn finding_text_45(mut v0: US11) -> Rc<str> {
    match v0 {
        US11::US11_Lost(v1) => {
            let mut v1: US6 = v1;
            match v1 {
                US6::US6_CaseLost(v2) => {
                    let v2: Rc<str> = v2;
                    let mut v3: Rc<str> = string_concat(&["loss case-lost ", &*v2]);
                    v3
                }
                US6::US6_MeasureFell(v10, v11, v12) => {
                    let v10: Rc<str> = v10;
                    let v11: Rc<str> = v11;
                    let v12: Rc<str> = v12;
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("measure-fell"); } LIT.with(|lit| lit.clone()) };
                    let v14: Rc<str> = moved_text_46(v13, v10, v11, v12);
                    let v15: Rc<str> = string_concat(&["loss ", &*v14]);
                    v15
                }
                US6::US6_OutcomeFell(v4, v5, v6) => {
                    let mut v4: Rc<str> = v4;
                    let v5: Rc<str> = v5;
                    let v6: Rc<str> = v6;
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("result-fell"); } LIT.with(|lit| lit.clone()) };
                    let v8: Rc<str> = moved_text_46(v7, v4, v5, v6);
                    let mut v9: Rc<str> = string_concat(&["loss ", &*v8]);
                    v9
                }
            }
        }
        US11::US11_Moved(v37) => {
            let v37: US3 = v37;
            match v37 {
                US3::US3_MeasureMoved(v44, v45, v46) => {
                    let v44: Rc<str> = v44;
                    let v45: Rc<str> = v45;
                    let v46: Rc<str> = v46;
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("measure-moved"); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<str> = moved_text_46(v47, v44, v45, v46);
                    let v49: Rc<str> = string_concat(&["change ", &*v48]);
                    v49
                }
                US3::US3_OrderChanged => {
                    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("change order"); } LIT.with(|lit| lit.clone()) };
                    v50
                }
                US3::US3_OutcomeChanged(v38, v39, v40) => {
                    let v38: Rc<str> = v38;
                    let v39: Rc<str> = v39;
                    let v40: Rc<str> = v40;
                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("result-changed"); } LIT.with(|lit| lit.clone()) };
                    let v42: Rc<str> = moved_text_46(v41, v38, v39, v40);
                    let v43: Rc<str> = string_concat(&["change ", &*v42]);
                    v43
                }
            }
        }
        US11::US11_Won(v19) => {
            let v19: US2 = v19;
            match v19 {
                US2::US2_CaseAdded(v20) => {
                    let v20: Rc<str> = v20;
                    let v21: Rc<str> = string_concat(&["gain case-added ", &*v20]);
                    v21
                }
                US2::US2_MeasureRose(v28, v29, v30) => {
                    let v28: Rc<str> = v28;
                    let v29: Rc<str> = v29;
                    let v30: Rc<str> = v30;
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("measure-rose"); } LIT.with(|lit| lit.clone()) };
                    let v32: Rc<str> = moved_text_46(v31, v28, v29, v30);
                    let v33: Rc<str> = string_concat(&["gain ", &*v32]);
                    v33
                }
                US2::US2_OutcomeRose(v22, v23, v24) => {
                    let mut v22: Rc<str> = v22;
                    let mut v23: Rc<str> = v23;
                    let v24: Rc<str> = v24;
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("result-rose"); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = moved_text_46(v25, v22, v23, v24);
                    let v27: Rc<str> = string_concat(&["gain ", &*v26]);
                    v27
                }
            }
        }
    }
}
fn method44(mut v0: Rc<UH7>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH7::UH7_Cons(v1, v2) => {
                let mut v1: US11 = v1;
                let v2: Rc<UH7> = v2;
                let mut v3: Rc<str> = finding_text_45(v1);
                println!("{}", v3);
                v0 = v2;
                continue;
            }
            UH7::UH7_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn loss_in_49(mut v0: US11) -> Rc<UH2> {
    match v0 {
        US11::US11_Lost(v1) => {
            let mut v1: US6 = v1;
            let v2: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_Cons(v1, v2))
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method48(mut v0: Rc<UH7>, mut v1: Rc<UH8>) -> Rc<UH8> {
    match Rc::unwrap_or_clone(v0) {
        UH7::UH7_Cons(v2, v3) => {
            let v2: US11 = v2;
            let mut v3: Rc<UH7> = v3;
            let mut v4: Rc<UH8> = method48(v3, v1);
            let v5: Rc<UH2> = loss_in_49(v2);
            Rc::new(UH8::UH8_Cons(v5, v4))
        }
        UH7::UH7_Nil => {
            v1
        }
    }
}
fn method51(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let v2: US6 = v2;
            let mut v3: Rc<UH2> = v3;
            let mut v4: Rc<UH2> = method51(v3, v1);
            Rc::new(UH2::UH2_Cons(v2, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method50(mut v0: Rc<UH8>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH8::UH8_Cons(v2, v3) => {
                let v2: Rc<UH2> = v2;
                let mut v3: Rc<UH8> = v3;
                let mut v4: Rc<UH2> = method51(v1, v2);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH8::UH8_Nil => {
                return v1;
            }
        }
    }
}
fn gain_in_53(mut v0: US11) -> Rc<UH0> {
    match v0 {
        US11::US11_Won(v1) => {
            let mut v1: US2 = v1;
            let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v1, v2))
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method52(mut v0: Rc<UH7>, mut v1: Rc<UH9>) -> Rc<UH9> {
    match Rc::unwrap_or_clone(v0) {
        UH7::UH7_Cons(v2, v3) => {
            let v2: US11 = v2;
            let mut v3: Rc<UH7> = v3;
            let mut v4: Rc<UH9> = method52(v3, v1);
            let v5: Rc<UH0> = gain_in_53(v2);
            Rc::new(UH9::UH9_Cons(v5, v4))
        }
        UH7::UH7_Nil => {
            v1
        }
    }
}
fn method55(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: US2 = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method55(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method54(mut v0: Rc<UH9>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH9::UH9_Cons(v2, v3) => {
                let v2: Rc<UH0> = v2;
                let mut v3: Rc<UH9> = v3;
                let mut v4: Rc<UH0> = method55(v1, v2);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH9::UH9_Nil => {
                return v1;
            }
        }
    }
}
fn change_in_57(mut v0: US11) -> Rc<UH1> {
    match v0 {
        US11::US11_Moved(v1) => {
            let mut v1: US3 = v1;
            let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_Cons(v1, v2))
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method56(mut v0: Rc<UH7>, mut v1: Rc<UH10>) -> Rc<UH10> {
    match Rc::unwrap_or_clone(v0) {
        UH7::UH7_Cons(v2, v3) => {
            let v2: US11 = v2;
            let mut v3: Rc<UH7> = v3;
            let mut v4: Rc<UH10> = method56(v3, v1);
            let v5: Rc<UH1> = change_in_57(v2);
            Rc::new(UH10::UH10_Cons(v5, v4))
        }
        UH7::UH7_Nil => {
            v1
        }
    }
}
fn method59(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: US3 = v2;
            let mut v3: Rc<UH1> = v3;
            let mut v4: Rc<UH1> = method59(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method58(mut v0: Rc<UH10>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH10::UH10_Cons(v2, v3) => {
                let v2: Rc<UH1> = v2;
                let mut v3: Rc<UH10> = v3;
                let mut v4: Rc<UH1> = method59(v1, v2);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH10::UH10_Nil => {
                return v1;
            }
        }
    }
}
fn verdict_of_47(mut v0: Rc<UH7>) -> US0 {
    let mut v1: Rc<UH8> = { thread_local!{ static CASE: Rc<UH8> = Rc::new(UH8::UH8_Nil); } CASE.with(|case| case.clone()) };
    let v2: Rc<UH8> = method48(v0.clone(), v1);
    let mut v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH2> = method50(v2, v3);
    let v5: Rc<UH9> = { thread_local!{ static CASE: Rc<UH9> = Rc::new(UH9::UH9_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH9> = method52(v0.clone(), v5);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = method54(v6, v7);
    let mut v9: Rc<UH10> = { thread_local!{ static CASE: Rc<UH10> = Rc::new(UH10::UH10_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH10> = method56(v0, v9);
    let v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH1> = method58(v10, v11);
    match Rc::unwrap_or_clone(v4) {
        UH2::UH2_Cons(v13, v14) => {
            let v13: US6 = v13;
            let v14: Rc<UH2> = v14;
            let v15: US5 = US5::US5_NonEmpty(v13, v14);
            US0::US0_Regressed(v15, v8)
        }
        UH2::UH2_Nil => {
            match Rc::unwrap_or_clone(v8) {
                UH0::UH0_Cons(v17, v18) => {
                    let v17: US2 = v17;
                    let v18: Rc<UH0> = v18;
                    let v19: US1 = US1::US1_NonEmpty(v17, v18);
                    US0::US0_Dominates(v19, v12)
                }
                UH0::UH0_Nil => {
                    match Rc::unwrap_or_clone(v12) {
                        UH1::UH1_Cons(v21, v22) => {
                            let v21: US3 = v21;
                            let mut v22: Rc<UH1> = v22;
                            let mut v23: US4 = US4::US4_NonEmpty(v21, v22);
                            US0::US0_Changed(v23)
                        }
                        _ => {
                            US0::US0_Equal
                        }
                    }
                }
            }
        }
    }
}
fn verdict_name_60(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_Changed(v5) => {
            let v5: US4 = v5;
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Changed"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US0::US0_Dominates(v2, v3) => {
            let v2: US1 = v2;
            let mut v3: Rc<UH1> = v3;
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Dominates"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_Equal => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Equal"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_Regressed(v7, v8) => {
            let v7: US5 = v7;
            let v8: Rc<UH0> = v8;
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Regressed"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    }
}
fn method62(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: US3 = v2;
                let mut v3: Rc<UH1> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn format_real_63(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method35(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method64(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: US2 = v2;
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
fn method65(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: US6 = v2;
                let mut v3: Rc<UH2> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn counted_61(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_Changed(v32) => {
            let v32: US4 = v32;
            match v32 {
                US4::US4_NonEmpty(v33, v34) => {
                    let v33: US3 = v33;
                    let v34: Rc<UH1> = v34;
                    let v35: i32 = 0i32;
                    let v36: i32 = method62(v34, v35);
                    let v37: i32 = v36.wrapping_add(1i32);
                    let v38: Rc<str> = format_real_63(v37);
                    let v39: Rc<str> = string_concat(&[" losses=0 gains=0 changes=", &*v38]);
                    v39
                }
            }
        }
        US0::US0_Dominates(v17, v18) => {
            let v17: US1 = v17;
            let v18: Rc<UH1> = v18;
            match v17 {
                US1::US1_NonEmpty(v19, v20) => {
                    let v19: US2 = v19;
                    let v20: Rc<UH0> = v20;
                    let v21: i32 = 0i32;
                    let mut v22: i32 = method62(v18, v21);
                    let mut v23: Rc<str> = format_real_63(v22);
                    let v24: i32 = 0i32;
                    let v25: i32 = method64(v20, v24);
                    let v26: i32 = v25.wrapping_add(1i32);
                    let v27: Rc<str> = format_real_63(v26);
                    let v30: Rc<str> = string_concat(&[" losses=0 gains=", &*v27, " changes=", &*v23]);
                    v30
                }
            }
        }
        US0::US0_Equal => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" losses=0 gains=0 changes=0"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_Regressed(v2, v3) => {
            let v2: US5 = v2;
            let mut v3: Rc<UH0> = v3;
            match v2 {
                US5::US5_NonEmpty(v4, v5) => {
                    let mut v4: US6 = v4;
                    let v5: Rc<UH2> = v5;
                    let v6: i32 = 0i32;
                    let v7: i32 = method65(v5, v6);
                    let v8: i32 = v7.wrapping_add(1i32);
                    let mut v9: Rc<str> = format_real_63(v8);
                    let v12: i32 = 0i32;
                    let v13: i32 = method64(v3, v12);
                    let v14: Rc<str> = format_real_63(v13);
                    let v15: Rc<str> = string_concat(&[" losses=", &*v9, " gains=", &*v14]);
                    v15
                }
            }
        }
    }
}
fn judged_run_1(mut v0: Rc<UH3>, mut v1: Rc<UH4>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v2).unwrap_or_default().as_str());
    let v5: US8 = side_of_4(v4);
    let v6: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v3).unwrap_or_default().as_str());
    let v7: US8 = side_of_4(v6);
    match &v5 {
        US8::US8_SideMalformed(v8) => {
            let v8: Rc<str> = v8.clone();
            let v10: Rc<str> = string_concat(&[&*v2, ": ", &*v8]);
            refused_2(v10)
        }
        _ => {
            match &v7 {
                US8::US8_SideMalformed(v12) => {
                    let v12: Rc<str> = v12.clone();
                    let v14: Rc<str> = string_concat(&[&*v3, ": ", &*v12]);
                    refused_2(v14)
                }
                _ => {
                    match v5 {
                        US8::US8_Side(v16) => {
                            let v16: Rc<UH5> = v16;
                            match v7 {
                                US8::US8_Side(v17) => {
                                    let v17: Rc<UH5> = v17;
                                    let v18: i32 = 0i32;
                                    let v19: i32 = method18(v16.clone(), v18);
                                    let v20: bool = v19 == 0i32;
                                    let v24: bool = if v20 {
                                        let v21: i32 = 0i32;
                                        let mut v22: i32 = method18(v17.clone(), v21);
                                        let mut v23: bool = v22 == 0i32;
                                        v23
                                    } else {
                                        false
                                    };
                                    if v24 {
                                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("both sides are empty or unreadable"); } LIT.with(|lit| lit.clone()) };
                                        refused_2(v25)
                                    } else {
                                        let v27: Rc<UH7> = findings_of_19(v0, v1, v16, v17);
                                        method44(v27.clone());
                                        let v28: US0 = verdict_of_47(v27);
                                        let v29: Rc<str> = verdict_name_60(v28.clone());
                                        let v31: Rc<str> = counted_61(v28.clone());
                                        let v32: Rc<str> = string_concat(&["rig verdict ok verdict=", &*v29, &*v31]);
                                        println!("{}", v32);
                                        verdict_exit_3(v28)
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
fn command_line_argument_count_67() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_68(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_66(mut v0: i32) -> Rc<UH6> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_67();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US17 = if v4 {
        let v5: Rc<str> = command_line_argument_68(v0);
        US17::US17_Some(v5)
    } else {
        US17::US17_None
    };
    match v8 {
        US17::US17_None => {
            { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) }
        }
        US17::US17_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH6> = arguments_from_66(v11);
            Rc::new(UH6::UH6_Cons(v10, v12))
        }
    }
}
fn method72(mut v0: Rc<UH6>, mut v1: Rc<UH6>) -> Rc<UH6> {
    match Rc::unwrap_or_clone(v0) {
        UH6::UH6_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH6> = v3;
            let mut v4: Rc<UH6> = method72(v3, v1);
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH6::UH6_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH6::UH6_Nil => {
            v1
        }
    }
}
fn direction_of_73(mut v0: Rc<str>) -> US19 {
    let mut v1: bool = &*v0 == "higher";
    if v1 {
        let v2: US7 = US7::US7_Higher;
        US19::US19_Some(v2)
    } else {
        let mut v4: bool = &*v0 == "lower";
        if v4 {
            let v5: US7 = US7::US7_Lower;
            US19::US19_Some(v5)
        } else {
            US19::US19_None
        }
    }
}
fn method74(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> Rc<UH4> {
    match Rc::unwrap_or_clone(v0) {
        UH4::UH4_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2;
            let mut v3: US7 = v3;
            let mut v4: u64 = v4;
            let v5: Rc<UH4> = v5;
            let v6: Rc<UH4> = method74(v5, v1);
            Rc::new(UH4::UH4_Cons(v2, v3, v4, v6))
        }
        UH4::UH4_Nil => {
            v1
        }
    }
}
fn method75(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure3() -> Rc<dyn Fn((i32)) -> US13> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US13> = Rc::new(move |mut v0: (i32)| -> US13 {
        let mut v1: i32 = (v0);
        US13::US13_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method76(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_Cons(v2, v3, v4) => {
            let v2: Rc<str> = v2;
            let mut v3: i32 = v3;
            let mut v4: Rc<UH3> = v4;
            let v5: Rc<UH3> = method76(v4, v1);
            Rc::new(UH3::UH3_Cons(v2, v3, v5))
        }
        UH3::UH3_Nil => {
            v1
        }
    }
}
fn rule_added_71(mut v0: US18, mut v1: Rc<str>) -> US18 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from((v1).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v4: Rc<UH6> = split_list_5(v2, v3);
    let v5: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH6> = method72(v4, v5);
    match &v0 {
        US18::US18_Rule(v9, v10) => {
            let mut v9: Rc<UH3> = v9.clone();
            let v10: Rc<UH4> = v10.clone();
            match Rc::unwrap_or_clone(v6) {
                UH6::UH6_Cons(v11, v12) => {
                    let v11: Rc<str> = v11;
                    let v12: Rc<UH6> = v12;
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
                    let v14: bool = v11.starts_with(&*v13);
                    if v14 {
                        v0
                    } else {
                        match Rc::unwrap_or_clone(v12) {
                            UH6::UH6_Cons(v15, v16) => {
                                let v15: Rc<str> = v15;
                                let v16: Rc<UH6> = v16;
                                match Rc::unwrap_or_clone(v16) {
                                    UH6::UH6_Cons(v17, v18) => {
                                        let v17: Rc<str> = v17;
                                        let v18: Rc<UH6> = v18;
                                        match Rc::unwrap_or_clone(v18) {
                                            UH6::UH6_Cons(v40, v41) => {
                                                let v40: Rc<str> = v40;
                                                let v41: Rc<UH6> = v41;
                                                match Rc::unwrap_or_clone(v41) {
                                                    UH6::UH6_Nil => {
                                                        let v42: bool = &*v11 == "measure";
                                                        if v42 {
                                                            let v43: US19 = direction_of_73(v17);
                                                            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%"); } LIT.with(|lit| lit.clone()) };
                                                            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                            let v46: Rc<str> = Rc::<str>::from(v40.replace(&*v44, &*v45));
                                                            let v47: US10 = whole_of_14(v46);
                                                            match v43 {
                                                                US19::US19_Some(v48) => {
                                                                    let v48: US7 = v48;
                                                                    match v47 {
                                                                        US10::US10_Some(v49) => {
                                                                            let v49: u64 = v49;
                                                                            let v50: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
                                                                            let v51: Rc<UH4> = Rc::new(UH4::UH4_Cons(v15, v48, v49, v50));
                                                                            let v52: Rc<UH4> = method74(v10, v51);
                                                                            US18::US18_Rule(v9, v52)
                                                                        }
                                                                        _ => {
                                                                            let v54: Rc<str> = Rc::<str>::from((v1).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                                                            let v55: Rc<str> = string_concat(&["measure <case prefix|*> <higher|lower> <tolerance %>, got: ", &*v54]);
                                                                            US18::US18_RuleMalformed(v55)
                                                                        }
                                                                    }
                                                                }
                                                                _ => {
                                                                    let v58: Rc<str> = Rc::<str>::from((v1).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                                                    let v59: Rc<str> = string_concat(&["measure <case prefix|*> <higher|lower> <tolerance %>, got: ", &*v58]);
                                                                    US18::US18_RuleMalformed(v59)
                                                                }
                                                            }
                                                        } else {
                                                            let v62: Rc<str> = string_concat(&["expected rank <result> <n> or measure <prefix|*> <higher|lower> <tol>, got: ", &*v1]);
                                                            US18::US18_RuleMalformed(v62)
                                                        }
                                                    }
                                                    _ => {
                                                        let v65: Rc<str> = string_concat(&["expected rank <result> <n> or measure <prefix|*> <higher|lower> <tol>, got: ", &*v1]);
                                                        US18::US18_RuleMalformed(v65)
                                                    }
                                                }
                                            }
                                            UH6::UH6_Nil => {
                                                let v19: bool = &*v11 == "rank";
                                                if v19 {
                                                    let v20: Option<i32> = (v17).trim().parse::<i32>().ok();
                                                    let v21: Option<i32> = method75(v20);
                                                    let mut v22: Rc<dyn Fn((i32)) -> US13> = closure3();
                                                    let mut v23: Option<US13> = v21.map(|x| v22(x));
                                                    let v25: US13 = US13::US13_None;
                                                    let v26: US13 = v25.clone();
                                                    let v27: US13 = v23.unwrap_or(v26);
                                                    match v27 {
                                                        US13::US13_None => {
                                                            let v33: Rc<str> = string_concat(&["rank needs a whole number: ", &*v17]);
                                                            US18::US18_RuleMalformed(v33)
                                                        }
                                                        US13::US13_Some(v28) => {
                                                            let v28: i32 = v28;
                                                            let v29: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                                                            let v30: Rc<UH3> = Rc::new(UH3::UH3_Cons(v15, v28, v29));
                                                            let v31: Rc<UH3> = method76(v9, v30);
                                                            US18::US18_Rule(v31, v10)
                                                        }
                                                    }
                                                } else {
                                                    let v37: Rc<str> = string_concat(&["expected rank <result> <n> or measure <prefix|*> <higher|lower> <tol>, got: ", &*v1]);
                                                    US18::US18_RuleMalformed(v37)
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        let v70: Rc<str> = string_concat(&["expected rank <result> <n> or measure <prefix|*> <higher|lower> <tol>, got: ", &*v1]);
                                        US18::US18_RuleMalformed(v70)
                                    }
                                }
                            }
                            _ => {
                                let v73: Rc<str> = string_concat(&["expected rank <result> <n> or measure <prefix|*> <higher|lower> <tol>, got: ", &*v1]);
                                US18::US18_RuleMalformed(v73)
                            }
                        }
                    }
                }
                UH6::UH6_Nil => {
                    v0
                }
            }
        }
        US18::US18_RuleMalformed(v7) => {
            let v7: Rc<str> = v7.clone();
            US18::US18_RuleMalformed(v7)
        }
    }
}
fn method70(mut v0: Rc<UH6>, mut v1: US18) -> US18 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH6::UH6_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH6> = v3;
                let mut v4: US18 = rule_added_71(v1, v2);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH6::UH6_Nil => {
                return v1;
            }
        }
    }
}
fn rule_of_69(mut v0: Rc<str>) -> US18 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH6> = split_list_5(v1, v0);
    let mut v3: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let v5: US18 = US18::US18_Rule(v3, v4);
    method70(v2, v5)
}
fn verdict_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH6> = arguments_from_66(v0);
    match Rc::unwrap_or_clone(v1) {
        UH6::UH6_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH6> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH6::UH6_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let v5: Rc<UH6> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH6::UH6_Cons(v6, v7) => {
                            let v6: Rc<str> = v6;
                            let v7: Rc<UH6> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH6::UH6_Cons(v11, v12) => {
                                    let v11: Rc<str> = v11;
                                    let v12: Rc<UH6> = v12;
                                    match Rc::unwrap_or_clone(v12) {
                                        UH6::UH6_Cons(v13, v14) => {
                                            let v13: Rc<str> = v13;
                                            let v14: Rc<UH6> = v14;
                                            match Rc::unwrap_or_clone(v14) {
                                                UH6::UH6_Nil => {
                                                    let v15: bool = &*v11 == "--rule";
                                                    if v15 {
                                                        let v16: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v13).unwrap_or_default().as_str());
                                                        let v17: US18 = rule_of_69(v16);
                                                        match v17 {
                                                            US18::US18_Rule(v18, v19) => {
                                                                let v18: Rc<UH3> = v18;
                                                                let v19: Rc<UH4> = v19;
                                                                judged_run_1(v18, v19, v4, v6)
                                                            }
                                                            US18::US18_RuleMalformed(v21) => {
                                                                let v21: Rc<str> = v21;
                                                                let mut v23: Rc<str> = string_concat(&[&*v13, ": ", &*v21]);
                                                                refused_2(v23)
                                                            }
                                                        }
                                                    } else {
                                                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected <before> <after> [--rule <file>]"); } LIT.with(|lit| lit.clone()) };
                                                        refused_2(v27)
                                                    }
                                                }
                                                _ => {
                                                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected <before> <after> [--rule <file>]"); } LIT.with(|lit| lit.clone()) };
                                                    refused_2(v30)
                                                }
                                            }
                                        }
                                        _ => {
                                            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected <before> <after> [--rule <file>]"); } LIT.with(|lit| lit.clone()) };
                                            refused_2(v33)
                                        }
                                    }
                                }
                                UH6::UH6_Nil => {
                                    let v8: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                                    let mut v9: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
                                    judged_run_1(v8, v9, v4, v6)
                                }
                            }
                        }
                        _ => {
                            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected <before> <after> [--rule <file>]"); } LIT.with(|lit| lit.clone()) };
                            refused_2(v38)
                        }
                    }
                }
                _ => {
                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected <before> <after> [--rule <file>]"); } LIT.with(|lit| lit.clone()) };
                    refused_2(v41)
                }
            }
        }
        _ => {
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected <before> <after> [--rule <file>]"); } LIT.with(|lit| lit.clone()) };
            refused_2(v44)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        verdict_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_verdict_run() -> i32 {
    closure0()()
}
pub type SpiralShape_8ca9aee563d6 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
