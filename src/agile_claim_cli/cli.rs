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
    US0_OutcomeGranted,
    US0_OutcomeRefused,
    US0_OutcomeNone,
    US0_OutcomeDropped,
    US0_OutcomeReleaseRefused,
    US0_OutcomeUnknown,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, u64, Rc<UH0>, Rc<UH1>),
}
pub struct Mut1 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<str>, u64, Rc<UH0>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(u64),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>),
    US3_None,
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method4(v3, v1);
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
fn method5(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
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
fn printed_3(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH0> = method4(v0, v2);
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v5: i32 = 0i32;
    let mut v6: i32 = method5(v4.clone(), v3, v5);
    let v7: Rc<Vec<Rc<str>>> = Rc::new(v4.borrow().clone());
    let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = Rc::<str>::from((*v8.borrow()).join(&*v9));
    let v11: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v10.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v12: bool = v11 != 0i32;
    if v12 {
        v11
    } else {
        v1
    }
}
fn method6(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let mut v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method7(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
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
fn method9(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_8(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method9(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method10(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method10(v0.clone(), v4, v2);
            let mut v6: Rc<str> = agile_claim_item::rig_claim_item_lines(&*v0, &*v3);
            Rc::new(UH0::UH0_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method11(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn listed_next_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let (mut v4, mut v5): (bool, Rc<str>) = agile_claim_store::rig_claims_loaded(&*v0);
    let mut v6: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v7: Rc<RefCell<Vec<Rc<str>>>> = agile_claim_listing::rig_claim_listed_ids(&v5, &v1, &v2, &v3, v6);
    let mut v8: Rc<Vec<Rc<str>>> = Rc::new((v7).borrow().clone());
    let mut v9: usize = (v8).len();
    let mut v10: i32 = ((v9) as i32);
    let v11: i32 = v10.wrapping_sub(1i32);
    let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH0> = method6(v8, v11, v12);
    let mut v14: i32 = 0i32;
    let mut v15: i32 = method7(v13.clone(), v14);
    let v16: Rc<str> = format_real_8(v15);
    let v20: Rc<str> = string_concat(&["rig agile next ok listed=", &*v16, " lane=", &*v1, " claimed=none"]);
    let v24: Rc<str> = string_concat(&["take one: rig agile claim ", &*v0, " <ID> --as ", &*v1, " (or next --claim takes the first)"]);
    let v25: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v26: Rc<UH0> = method10(v0, v13, v25);
    let v27: bool = v15 > 0i32;
    let mut v29: Rc<str> = if v27 {
        v24
    } else {
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v28
    };
    let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v31: Rc<UH0> = Rc::new(UH0::UH0_Cons(v29, v30));
    let v32: Rc<UH0> = method11(v26, v31);
    let v33: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20, v32));
    let v34: i32 = 0i32;
    printed_3(v33, v34)
}
fn outcome_word_14(mut v0: US0) -> Rc<str> {
    let v7: Rc<str> = match v0 {
        US0::US0_OutcomeDropped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Dropped"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_OutcomeGranted => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Granted"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_OutcomeNone => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_OutcomeRefused => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Refused"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_OutcomeReleaseRefused => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ReleaseRefused"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US0::US0_OutcomeUnknown => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unknown"); } LIT.with(|lit| lit.clone()) };
            v6
        }
    };
    let mut v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
    v8
}
fn outcome_of_13(mut v0: Rc<str>) -> US0 {
    let v1: US0 = US0::US0_OutcomeGranted;
    let v2: Rc<str> = outcome_word_14(v1);
    let v3: bool = &*v2 == &*v0;
    if v3 {
        US0::US0_OutcomeGranted
    } else {
        let v5: US0 = US0::US0_OutcomeRefused;
        let mut v6: Rc<str> = outcome_word_14(v5);
        let v7: bool = &*v6 == &*v0;
        if v7 {
            US0::US0_OutcomeRefused
        } else {
            let mut v9: US0 = US0::US0_OutcomeNone;
            let mut v10: Rc<str> = outcome_word_14(v9);
            let v11: bool = &*v10 == &*v0;
            if v11 {
                US0::US0_OutcomeNone
            } else {
                let v13: US0 = US0::US0_OutcomeDropped;
                let mut v14: Rc<str> = outcome_word_14(v13);
                let mut v15: bool = &*v14 == &*v0;
                if v15 {
                    US0::US0_OutcomeDropped
                } else {
                    let v17: US0 = US0::US0_OutcomeReleaseRefused;
                    let v18: Rc<str> = outcome_word_14(v17);
                    let v19: bool = &*v18 == &*v0;
                    if v19 {
                        US0::US0_OutcomeReleaseRefused
                    } else {
                        let v21: US0 = US0::US0_OutcomeUnknown;
                        let v22: Rc<str> = outcome_word_14(v21);
                        US0::US0_OutcomeUnknown
                    }
                }
            }
        }
    }
}
fn method16(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let mut v4: u64 = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<UH1> = v6;
                let v7: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method18(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let mut v4: u64 = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<UH1> = v6;
                let v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v1));
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
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
fn method21(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method21(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method20(v16, v17)
    }
}
fn method24(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method25(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US2> = Rc::new(move |mut v0: (u64)| -> US2 {
        let v1: u64 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn globs_from_26(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> Rc<UH0> {
    let v2: i32 = (v0.clone().borrow().len() as i32);
    let v3: bool = v2 <= v1;
    if v3 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
        let mut v6: i32 = v1.wrapping_add(2i32);
        let v7: Rc<UH0> = globs_from_26(v0, v6);
        Rc::new(UH0::UH0_Cons(v5, v7))
    }
}
fn claim_of_row_23(mut v0: Rc<str>) -> US1 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: bool = v3 < 5i32;
    let mut v8: bool = if v4 {
        true
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl claim_"); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = v0.starts_with(&*v5);
        let v7: bool = v6 == false;
        v7
    };
    if v8 {
        US1::US1_None
    } else {
        let mut v10: Rc<str> = v2.borrow()[4i32 as usize].clone();
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
        let v12: Option<usize> = v10.find(&*v11);
        let v13: Option<usize> = method24(v12);
        let mut v14: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v15: Option<i32> = v13.map(|x| v14(x));
        let v17: i32 = v15.unwrap_or(-1i32);
        let v18: bool = v17 > 2i32;
        let v25: Rc<str> = if v18 {
            let v19: i32 = v17.wrapping_sub(2i32);
            let v20: usize = ((2i32) as usize);
            let v21: i32 = 2i32.wrapping_add(v19);
            let v22: usize = ((v21) as usize);
            let v23: Rc<str> = Rc::<str>::from(&(v10)[v20..v22]);
            v23
        } else {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24
        };
        let v26: Option<u64> = (v25).trim().parse::<u64>().ok();
        let v27: Option<u64> = method25(v26);
        let mut v28: Rc<dyn Fn((u64)) -> US2> = closure2();
        let mut v29: Option<US2> = v27.map(|x| v28(x));
        let v31: US2 = US2::US2_None;
        let v32: US2 = v31.clone();
        let v33: US2 = v29.unwrap_or(v32);
        match v33 {
            US2::US2_None => {
                US1::US1_None
            }
            US2::US2_Some(v34) => {
                let v34: u64 = v34;
                let v35: Rc<str> = v2.borrow()[1i32 as usize].clone();
                let v36: Rc<str> = v2.borrow()[3i32 as usize].clone();
                let v37: i32 = 5i32;
                let v38: Rc<UH0> = globs_from_26(&v2, v37);
                US1::US1_Some(v35, v36, v34, v38)
            }
        }
    }
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: US1 = claim_of_row_23(v2);
                let v11: Rc<UH1> = match v4 {
                    US1::US1_None => {
                        v1
                    }
                    US1::US1_Some(v5, v6, v7, v8) => {
                        let v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        let v7: u64 = v7;
                        let mut v8: Rc<UH0> = v8;
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
fn claims_of_text_17(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_19(v1, v4);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH1> = method22(v5, v6);
    let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method18(v7, v8)
}
fn decliner_29(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: &Rc<UH0>) -> Rc<str> {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
    let v5: bool = v1.starts_with(&*v4);
    if v5 {
        let mut v6: i32 = (v1.clone().len() as i32);
        let v7: i32 = v6.wrapping_sub(1i32);
        let mut v8: usize = ((1i32) as usize);
        let mut v9: i32 = 1i32.wrapping_add(v7);
        let mut v10: usize = ((v9) as usize);
        let v11: Rc<str> = Rc::<str>::from(&(v1)[v8..v10]);
        v11
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    }
}
fn method28(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<UH1>, mut v3: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v4, v5, v6, v7, v8) => {
            let mut v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let mut v6: u64 = v6;
            let v7: Rc<UH0> = v7;
            let mut v8: Rc<UH1> = v8;
            let mut v9: Rc<UH1> = method28(v0.clone(), v1, v8, v3);
            let mut v10: bool = v1 < v6;
            let v13: bool = if v10 {
                let v11: Rc<str> = decliner_29(&v4, v5.clone(), v6, &v7);
                let v12: bool = &*v11 == "";
                v12
            } else {
                false
            };
            let v17: bool = if v13 {
                let mut v14: bool = &*v0 == "";
                if v14 {
                    true
                } else {
                    let mut v15: bool = &*v5 == &*v0;
                    v15
                }
            } else {
                false
            };
            if v17 {
                Rc::new(UH1::UH1_Cons(v4, v5, v6, v7, v9))
            } else {
                v9
            }
        }
        UH1::UH1_Nil => {
            v3
        }
    }
}
fn live_claims_27(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<UH1>) -> Rc<UH1> {
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method28(v0, v1, v2, v3)
}
fn lane_wip_15(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let (mut v2, mut v3): (bool, Rc<str>) = agile_claim_store::rig_claims_loaded(&*v0);
    let mut v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v5: Rc<UH1> = claims_of_text_17(v3);
    let mut v6: Rc<UH1> = live_claims_27(v1, v4, v5);
    let v7: i32 = 0i32;
    method16(v6, v7)
}
fn concat5_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn civil_of_days_33(mut v0: u64) -> (u64, u64, u64) {
    let v1: u64 = v0.wrapping_add(719468u64);
    let v2: u64 = v1.wrapping_div(146097u64);
    let v3: u64 = v2.wrapping_mul(146097u64);
    let mut v4: u64 = v1.wrapping_sub(v3);
    let v5: u64 = v4.wrapping_div(1460u64);
    let mut v6: u64 = v4.wrapping_sub(v5);
    let v7: u64 = v4.wrapping_div(36524u64);
    let mut v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let mut v10: u64 = v8.wrapping_sub(v9);
    let v11: u64 = v10.wrapping_div(365u64);
    let v12: u64 = 365u64.wrapping_mul(v11);
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
    let v25: u64 = v24.wrapping_add(1u64);
    let v26: bool = v20 < 10u64;
    let mut v29: u64 = if v26 {
        let v27: u64 = v20.wrapping_add(3u64);
        v27
    } else {
        let mut v28: u64 = v20.wrapping_sub(9u64);
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
fn utc_stamp_31(mut v0: u64) -> Rc<str> {
    let v1: u64 = v0.wrapping_div(1000u64);
    let v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_33(v2);
    let mut v6: u64 = v1.wrapping_rem(86400u64);
    let v7: u64 = v6.wrapping_div(60u64);
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v10: bool = v4 < 10u64;
    let v16: Rc<str> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_32(v11, v12, v13.clone(), v13.clone(), v13)
    } else {
        let mut v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15
    };
    let v17: bool = v5 < 10u64;
    let v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_32(v18, v19, v20.clone(), v20.clone(), v20)
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22
    };
    let v24: Rc<str> = concat5_32(v8, v9.clone(), v16, v9, v23);
    let v25: u64 = v7.wrapping_div(60u64);
    let v26: bool = v25 < 10u64;
    let v32: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_32(v27, v28, v29.clone(), v29.clone(), v29)
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
        concat5_32(v36, v37, v38.clone(), v38.clone(), v38)
    } else {
        let v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = concat5_32(v32, v33, v41, v42, v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    concat5_32(v24, v45, v44, v43.clone(), v43)
}
fn local_offset_minutes_36() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn local_stamp_35(mut v0: Rc<str>) -> Rc<str> {
    let v1: i64 = local_offset_minutes_36();
    let v2: Rc<str> = { let at = &*v0; let offset = v1; let parsed = (|| -> Option<i64> { let hours: i64 = at.get(11..13)?.parse().ok()?; let minutes: i64 = at.get(14..16)?.parse().ok()?; Some(hours * 60 + minutes) })(); std::rc::Rc::<str>::from(match parsed { Some(total) => { let local = (total + offset).rem_euclid(1440); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; format!(" ({:02}:{:02} {}{:02}:{:02})", local / 60, local % 60, sign, offset.abs() / 60, offset.abs() % 60) } None => String::new() }) };
    v2
}
fn local_clock_34(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = utc_stamp_31(v0);
    let v2: Rc<str> = local_stamp_35(v1);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v4));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v4));
    v9
}
fn receipt_30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: i32, mut v5: i32, mut v6: i32) -> Rc<str> {
    let v11: Rc<str> = string_concat(&["rig agile ", &*v0, " ok claimed=", &*v1, " owner=", &*v2]);
    let v12: Rc<str> = utc_stamp_31(v3);
    let mut v15: Rc<str> = local_clock_34(v3);
    let v16: Rc<str> = string_concat(&[" expires=", &*v12, " expires_local=", &*v15]);
    let v17: bool = v6 > 0i32;
    let v21: Rc<str> = if v17 {
        let v18: Rc<str> = format_real_8(v6);
        let v19: Rc<str> = string_concat(&[" rank=", &*v18]);
        v19
    } else {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v20
    };
    let v24: Rc<str> = format_real_8(v4);
    let v27: Rc<str> = format_real_8(v5);
    let mut v29: Rc<str> = string_concat(&[&*v11, &*v16, " ttl_min=", &*v24, " wip=", &*v27, &*v21]);
    v29
}
fn claim_answered_12(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<UH0>, mut v8: Rc<str>, mut v9: bool, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>) -> i32 {
    let mut v15: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62): (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) = if v0 {
        let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23): (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) = agile_claim_queue::rig_claim_next(&*v5, v6, &*v11, &*v12, &*v14, &*v10, v15);
        (v16, v17, v18, v19, v20, v21, v22, v23)
    } else {
        if v2 {
            let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31): (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) = agile_claim_release::rig_claim_release(&*v4, &*v5, v6, &*v10, v15);
            (v24, v25, v26, v27, v28, v29, v30, v31)
        } else {
            let v32: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v33: i32 = 0i32;
            let v34: i32 = method5(v32.clone(), v7, v33);
            let v35: Rc<Vec<Rc<str>>> = Rc::new(v32.borrow().clone());
            let v36: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v35).as_ref().clone()));
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<str> = Rc::<str>::from((*v36.borrow()).join(&*v37));
            let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) = agile_claim_lease::rig_claim_lease(&*v4, &*v5, v6, &*v38, &*v11, &*v13, &*v14, &*v10, v15);
            (v39, v40, v41, v42, v43, v44, v45, v46)
        }
    };
    let v63: Rc<RefCell<Vec<(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(vec![(v8.clone(), v9, v10, v62, v56.clone(), v57.clone(), v58, v61.clone(), v13)]));
    let v64: Rc<str> = agile_claim_store::rig_claims_applied(v63);
    let v65: bool = &*v64 != "";
    if v65 {
        let v68: Rc<str> = string_concat(&["agile ", &*v3, " rejected: ", &*v64]);
        let v69: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v68 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v69
    } else {
        let v70: US0 = outcome_of_13(v55);
        match v70 {
            US0::US0_OutcomeDropped => {
                let v104: Rc<str> = string_concat(&["rig agile release ok id=", &*v4, " owner=", &*v5]);
                let v105: i32 = lane_wip_15(v8, v5);
                let v106: Rc<str> = format_real_8(v105);
                let v111: Rc<str> = format_real_8(v60);
                let v113: Rc<str> = string_concat(&[&*v104, " released=", &*v61, " wip=", &*v111, " lane_wip=", &*v106]);
                let v114: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v115: Rc<UH0> = Rc::new(UH0::UH0_Cons(v113, v114));
                let v116: i32 = 0i32;
                printed_3(v115, v116)
            }
            US0::US0_OutcomeGranted => {
                let v71: bool = &*v61 == "";
                let v74: Rc<str> = if v71 {
                    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("paths none"); } LIT.with(|lit| lit.clone()) };
                    v72
                } else {
                    let v73: Rc<str> = string_concat(&["paths ", &*v61]);
                    v73
                };
                let v75: Rc<str> = receipt_30(v3, v56.clone(), v57.clone(), v58, v6, v60, v59);
                let v77: i32 = lane_wip_15(v8.clone(), v57);
                let v78: Rc<str> = format_real_8(v77);
                let v79: Rc<str> = string_concat(&[&*v75, " lane_wip=", &*v78]);
                let v80: Rc<str> = agile_claim_item::rig_claim_item_lines(&*v8, &*v56);
                let v81: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v82: Rc<UH0> = Rc::new(UH0::UH0_Cons(v74, v81));
                let v83: Rc<UH0> = Rc::new(UH0::UH0_Cons(v80, v82));
                let v84: Rc<UH0> = Rc::new(UH0::UH0_Cons(v79, v83));
                let v85: i32 = 0i32;
                printed_3(v84, v85)
            }
            US0::US0_OutcomeNone => {
                let v96: Rc<str> = format_real_8(v60);
                let v97: Rc<str> = string_concat(&["rig agile next ok claimed=none reason=", &*v61, " wip=", &*v96]);
                let v98: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v99: Rc<UH0> = Rc::new(UH0::UH0_Cons(v97, v98));
                let v100: i32 = 3i32;
                printed_3(v99, v100)
            }
            US0::US0_OutcomeRefused => {
                let v89: Rc<str> = string_concat(&["rig agile claim refused id=", &*v4, " reason=", &*v61]);
                let v90: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v91: Rc<UH0> = Rc::new(UH0::UH0_Cons(v89, v90));
                let v92: i32 = 5i32;
                printed_3(v91, v92)
            }
            US0::US0_OutcomeReleaseRefused => {
                let v120: Rc<str> = string_concat(&["rig agile release refused id=", &*v4, " reason=", &*v61]);
                let v121: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v122: Rc<UH0> = Rc::new(UH0::UH0_Cons(v120, v121));
                let v123: i32 = 5i32;
                printed_3(v122, v123)
            }
            US0::US0_OutcomeUnknown => {
                let v128: Rc<str> = string_concat(&["agile ", &*v3, " rejected: no open task with id ", &*v4]);
                let v129: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v128 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v129
            }
        }
    }
}
fn method38(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn get_current_directory_37() -> Rc<str> {
    let v1: Result<std::path::PathBuf, std::io::Error> = std::env::current_dir();
    let v3: std::path::PathBuf = v1.unwrap();
    let mut v4: std::path::PathBuf = method38(v3);
    let mut v6: std::path::Display = v4.display();
    let mut v8: std::string::String = format!("{}", v6);
    let mut v10: Rc<str> = Rc::<str>::from(String::as_str(&v8));
    v10
}
fn anchored_40(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: bool = v4.starts_with(&*v3);
    let mut v14: bool = if v5 {
        true
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":/"); } LIT.with(|lit| lit.clone()) };
        let v7: Option<usize> = v4.find(&*v6);
        let mut v8: Option<usize> = method24(v7);
        let mut v9: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v10: Option<i32> = v8.map(|x| v9(x));
        let v12: i32 = v10.unwrap_or(-1i32);
        let v13: bool = v12 == 1i32;
        v13
    };
    let mut v15: i32 = (v4.clone().len() as i32);
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("./"); } LIT.with(|lit| lit.clone()) };
    let v17: bool = v4.starts_with(&*v16);
    let v23: Rc<str> = if v17 {
        let v18: i32 = v15.wrapping_sub(2i32);
        let v19: usize = ((2i32) as usize);
        let v20: i32 = 2i32.wrapping_add(v18);
        let v21: usize = ((v20) as usize);
        let v22: Rc<str> = Rc::<str>::from(&(v4)[v19..v21]);
        v22
    } else {
        v4.clone()
    };
    if v14 {
        v4
    } else {
        let v24: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
        let v26: Rc<str> = string_concat(&[&*v24, "/", &*v23]);
        v26
    }
}
fn loop_41(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(1i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn directory_exists_43(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn file_exists_44(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn closure3() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn repository_of_42(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = string_concat(&[&*v0, "/.git"]);
        let v2: bool = directory_exists_43(v1.clone());
        let mut v4: bool = if v2 {
            true
        } else {
            file_exists_44(v1)
        };
        if v4 {
            return v0;
        } else {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Option<usize> = v0.rfind(&*v5);
            let v7: Option<usize> = method24(v6);
            let mut v8: Rc<dyn Fn((usize)) -> i32> = closure3();
            let mut v9: Option<i32> = v7.map(|x| v8(x));
            let v11: i32 = v9.unwrap_or(-1i32);
            let v12: bool = v11 <= 0i32;
            if v12 {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v13;
            } else {
                let mut v14: usize = ((0i32) as usize);
                let mut v15: usize = ((v11) as usize);
                let v16: Rc<str> = Rc::<str>::from(&(v0)[v14..v15]);
                v0 = v16;
                continue;
            }
        }
    }
}
fn claims_base_39(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = anchored_40(v0.clone(), v1);
    let v3: Rc<str> = loop_41(v2);
    let mut v4: Rc<str> = repository_of_42(v3);
    let v5: bool = &*v4 == "";
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
        v8
    } else {
        v4
    }
}
fn method45(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method45(v0.clone(), v4, v2);
            let mut v6: Rc<str> = anchored_40(v0, v3);
            Rc::new(UH0::UH0_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn claim_requested_1(mut v0: Rc<str>, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: i32, mut v8: Rc<UH0>) -> i32 {
    let mut v9: Rc<str> = get_current_directory_37();
    let mut v10: Rc<str> = claims_base_39(v9, v0.clone());
    let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH0> = method45(v10, v8, v11);
    let (mut v13, mut v14, mut v15, mut v16, mut v17): (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = agile_claim_rank::rig_claim_ranking(&*v0);
    let v20: Rc<str> = if v1 {
        let v18: Rc<str> = agile_team_lane::rig_claim_lane_items(&v0, &v6);
        v18
    } else {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v19
    };
    let (mut v23, mut v24): (Rc<str>, Rc<str>) = if v1 {
        let (mut v21, mut v22): (Rc<str>, Rc<str>) = agile_claim_listing::rig_claim_lane_order(&v14, &v15, &v20);
        (v21, v22)
    } else {
        (v14, v15)
    };
    let v25: bool = v13 == false;
    if v25 {
        let v27: Rc<str> = string_concat(&["agile ", &*v4, ": state/agile.spi is missing"]);
        let mut v28: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v27 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v28
    } else {
        let v30: bool = if v1 {
            let mut v29: bool = v2 == false;
            v29
        } else {
            false
        };
        if v30 {
            listed_next_2(v0, v6, v23, v24)
        } else {
            let (mut v32, mut v33): (bool, Rc<str>) = agile_claim_store::rig_claims_loaded(&*v0);
            claim_answered_12(v1, v2, v3, v4, v5, v6, v7, v12, v0, v32, v33, v23, v24, v16, v17)
        }
    }
}
fn command_line_argument_count_47() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_48(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_46(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_47();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US3 = if v4 {
        let v5: Rc<str> = command_line_argument_48(v0);
        US3::US3_Some(v5)
    } else {
        US3::US3_None
    };
    match v8 {
        US3::US3_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US3::US3_Some(v10) => {
            let mut v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_46(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn method50(mut v0: i32, mut v1: &Rc<RefCell<Mut2>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method49(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2]));
                let v5: i32 = (v1.clone().borrow().len() as i32);
                let mut v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: 0i32 }));
                while method50(v7, &v9) {
                    let v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let v16: Rc<str> = if v12 {
                        let v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13
                    } else {
                        let mut v14: i32 = v11.wrapping_sub(v5);
                        let mut v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15
                    };
                    v8.borrow_mut()[v11 as usize] = v16;
                    let v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn listed_paths_52(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v4, v6);
            continue;
        }
    }
}
fn order_of_51(mut v0: &Rc<str>, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: i32, mut v8: &Rc<RefCell<Vec<Rc<str>>>>) -> (bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<UH0>) {
    let mut v9: i32 = (v8.clone().borrow().len() as i32);
    let mut v10: i32 = v9.wrapping_sub(1i32);
    let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH0> = listed_paths_52(v8, v10, v11);
    (v1, v2, v3, v4, v5, v6, v7, v12)
}
fn claim_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_46(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
                            let mut v9: Rc<RefCell<Vec<Rc<str>>>> = method49(v7, v8);
                            let mut v10: Rc<RefCell<Vec<(Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>)>>> = agile_claim_request::agile_claim_request_parsed(&v4, v9);
                            let (mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19): (Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>) = v10.borrow()[0i32 as usize].clone();
                            let v20: bool = &*v11 != "";
                            if v20 {
                                let v23: Rc<str> = string_concat(&["agile ", &*v4, ": ", &*v11]);
                                let v24: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v23 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v24
                            } else {
                                let (mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32): (bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<UH0>) = order_of_51(&v11, v12, v13, v14, v15, v16, v17, v18, &v19);
                                claim_requested_1(v6, v25, v26, v27, v28, v29, v30, v31, v32)
                            }
                        }
                        _ => {
                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile: usage: next|claim|release <root> ..."); } LIT.with(|lit| lit.clone()) };
                            let v36: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v35 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v36
                        }
                    }
                }
                _ => {
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile: usage: next|claim|release <root> ..."); } LIT.with(|lit| lit.clone()) };
                    let v39: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v38 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v39
                }
            }
        }
        _ => {
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile: usage: next|claim|release <root> ..."); } LIT.with(|lit| lit.clone()) };
            let v42: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v41 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v42
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        claim_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn format_real_57(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method9(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn claim_line_56(mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: Rc<UH0>) -> Rc<str> {
    let v5: u64 = v3.wrapping_sub(v0);
    let mut v6: u64 = v5.wrapping_div(60000u64);
    let v7: bool = match &*v4 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    let v16: Rc<str> = if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
        v8
    } else {
        let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let mut v10: i32 = 0i32;
        let v11: i32 = method5(v9.clone(), v4, v10);
        let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
        let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v14));
        v15
    };
    let v21: Rc<str> = utc_stamp_31(v3);
    let v24: Rc<str> = local_clock_34(v3);
    let v27: Rc<str> = format_real_57(v6);
    let v30: Rc<str> = string_concat(&["  ", &*v1, "  ", &*v2, "  expires=", &*v21, " local=", &*v24, " left_min=", &*v27, "  paths=", &*v16]);
    v30
}
fn method55(mut v0: u64, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let v5: u64 = v5;
            let mut v6: Rc<UH0> = v6;
            let v7: Rc<UH1> = v7;
            let mut v8: Rc<UH0> = method55(v0, v7, v2);
            let mut v9: Rc<str> = claim_line_56(v0, v3, v4, v5, v6);
            Rc::new(UH0::UH0_Cons(v9, v8))
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn claims_listed_54(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let (mut v2, mut v3): (bool, Rc<str>) = agile_claim_store::rig_claims_loaded(&*v0);
    let mut v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v5: Rc<UH1> = claims_of_text_17(v3);
    let mut v6: Rc<UH1> = live_claims_27(v1.clone(), v4, v5);
    let v7: i32 = 0i32;
    let mut v8: i32 = method16(v6.clone(), v7);
    let mut v9: Rc<str> = format_real_8(v8);
    let mut v10: Rc<str> = string_concat(&["rig agile claims ok live=", &*v9]);
    let v11: bool = &*v1 == "";
    let mut v14: Rc<str> = if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    } else {
        let v13: Rc<str> = string_concat(&[" lane=", &*v1]);
        v13
    };
    let mut v15: Rc<str> = string_concat(&[&*v10, &*v14]);
    let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH0> = method55(v4, v6, v16);
    let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v18));
    let v20: Rc<UH0> = method11(v17, v19);
    let v21: i32 = 0i32;
    printed_3(v20, v21)
}
fn claims_run_53() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_46(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH0::UH0_Cons(v10, v11) => {
                                    let mut v10: Rc<str> = v10;
                                    let v11: Rc<UH0> = v11;
                                    match Rc::unwrap_or_clone(v11) {
                                        UH0::UH0_Cons(v12, v13) => {
                                            let v12: Rc<str> = v12;
                                            let v13: Rc<UH0> = v13;
                                            match Rc::unwrap_or_clone(v13) {
                                                UH0::UH0_Nil => {
                                                    let mut v14: bool = &*v10 == "--as";
                                                    if v14 {
                                                        claims_listed_54(v6, v12)
                                                    } else {
                                                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>]"); } LIT.with(|lit| lit.clone()) };
                                                        let v17: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v16 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                        v17
                                                    }
                                                }
                                                _ => {
                                                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>]"); } LIT.with(|lit| lit.clone()) };
                                                    let v20: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v19 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                    v20
                                                }
                                            }
                                        }
                                        _ => {
                                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>]"); } LIT.with(|lit| lit.clone()) };
                                            let v23: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v22 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                            v23
                                        }
                                    }
                                }
                                UH0::UH0_Nil => {
                                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    claims_listed_54(v6, v8)
                                }
                            }
                        }
                        _ => {
                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>]"); } LIT.with(|lit| lit.clone()) };
                            let mut v28: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v27 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v28
                        }
                    }
                }
                _ => {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>]"); } LIT.with(|lit| lit.clone()) };
                    let v31: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v30 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v31
                }
            }
        }
        _ => {
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>]"); } LIT.with(|lit| lit.clone()) };
            let v34: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v33 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v34
        }
    }
}
fn closure4() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        claims_run_53()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_claim_run() -> i32 {
    closure0()()
}
pub fn rig_agile_claims_run() -> i32 {
    closure4()()
}
use agile_claim_store::SpiralShape_97df63ebabc3 as Mut0;
use agile_claim_request::SpiralShape_2617b4e49837 as Mut2;
pub type SpiralShape_652accfc6a8d = Mut1;
