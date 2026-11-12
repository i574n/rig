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
    US3_Some(i32),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_StateFile(Rc<str>),
    US5_HistoryFile(Rc<str>),
    US5_ArchiveFile(Rc<str>),
    US5_OtherFile,
}
#[derive(Clone)]
enum US7 {
    US7_HeadInl,
    US7_HeadUnion,
    US7_HeadOpen,
    US7_HeadType,
}
#[derive(Clone)]
enum US6 {
    US6_KeywordHead(US7),
    US6_SymbolHead(US7),
    US6_OtherLine,
}
#[derive(Clone)]
enum US8 {
    US8_TaskLine(Rc<str>, Rc<str>, Rc<str>),
    US8_PlanningLine(Rc<str>, i32, Rc<str>, Rc<str>),
    US8_NoteLine(Rc<str>, Rc<str>),
    US8_WaitLine(Rc<str>, Rc<str>),
    US8_MarkerLine(Rc<str>),
    US8_OtherLine,
}
#[derive(Clone)]
enum US9 {
    US9_Some(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US9_None,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH2>),
}
#[derive(Clone)]
enum US10 {
    US10_Some(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US10_None,
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
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
fn printed_3(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH0> = method4(v0, v2);
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v5: i32 = 0i32;
    let mut v6: i32 = method5(v4.clone(), v3, v5);
    let mut v7: Rc<Vec<Rc<str>>> = Rc::new(v4.borrow().clone());
    let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = Rc::<str>::from((*v8.borrow()).join(&*v9));
    let mut v11: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v10.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v12: bool = v11 != 0i32;
    if v12 {
        v11
    } else {
        v1
    }
}
fn method6(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let mut v6: i32 = v1.wrapping_sub(1i32);
            let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method7(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method9(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_8(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method9(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method10(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
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
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
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
    let mut v7: Rc<RefCell<Vec<Rc<str>>>> = agile_claim_listing::rig_claim_listed_ids(&v5, &v1, &v2, &v3, v6);
    let mut v8: Rc<Vec<Rc<str>>> = Rc::new((v7).borrow().clone());
    let mut v9: usize = (v8).len();
    let mut v10: i32 = ((v9) as i32);
    let mut v11: i32 = v10.wrapping_sub(1i32);
    let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH0> = method6(v8, v11, v12);
    let mut v14: i32 = 0i32;
    let mut v15: i32 = method7(v13.clone(), v14);
    let mut v16: Rc<str> = format_real_8(v15);
    let v20: Rc<str> = string_concat(&["rig agile next ok listed=", &*v16, " lane=", &*v1, " claimed=none"]);
    let v24: Rc<str> = string_concat(&["take one: rig agile claim ", &*v0, " <ID> --as ", &*v1, " (or next --claim takes the first)"]);
    let mut v25: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v26: Rc<UH0> = method10(v0, v13, v25);
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
    let mut v7: Rc<str> = match v0 {
        US0::US0_OutcomeDropped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Dropped"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_OutcomeGranted => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Granted"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_OutcomeNone => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_OutcomeRefused => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Refused"); } LIT.with(|lit| lit.clone()) };
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
    let mut v1: US0 = US0::US0_OutcomeGranted;
    let mut v2: Rc<str> = outcome_word_14(v1);
    let mut v3: bool = &*v2 == &*v0;
    if v3 {
        US0::US0_OutcomeGranted
    } else {
        let v5: US0 = US0::US0_OutcomeRefused;
        let mut v6: Rc<str> = outcome_word_14(v5);
        let mut v7: bool = &*v6 == &*v0;
        if v7 {
            US0::US0_OutcomeRefused
        } else {
            let mut v9: US0 = US0::US0_OutcomeNone;
            let mut v10: Rc<str> = outcome_word_14(v9);
            let mut v11: bool = &*v10 == &*v0;
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
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let mut v4: u64 = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<UH1> = v6;
                let mut v7: i32 = v1.wrapping_add(1i32);
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
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let mut v4: u64 = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<UH1> = v6;
                let mut v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v1));
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
fn method21(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method21(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method20(v16, v17)
    }
}
fn method24(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method25(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US2> = Rc::new(move |mut v0: (u64)| -> US2 {
        let mut v1: u64 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn globs_from_26(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().borrow().len() as i32);
    let mut v3: bool = v2 <= v1;
    if v3 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
        let mut v6: i32 = v1.wrapping_add(2i32);
        let mut v7: Rc<UH0> = globs_from_26(v0, v6);
        Rc::new(UH0::UH0_Cons(v5, v7))
    }
}
fn claim_of_row_23(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: bool = v3 < 5i32;
    let mut v8: bool = if v4 {
        true
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl claim_"); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = v0.starts_with(&*v5);
        let mut v7: bool = v6 == false;
        v7
    };
    if v8 {
        US1::US1_None
    } else {
        let mut v10: Rc<str> = v2.borrow()[4i32 as usize].clone();
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Option<usize> = v10.find(&*v11);
        let v13: Option<usize> = method24(v12);
        let mut v14: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v15: Option<i32> = v13.map(|x| v14(x));
        let v17: i32 = v15.unwrap_or(-1i32);
        let v18: bool = v17 > 2i32;
        let mut v25: Rc<str> = if v18 {
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
        let mut v26: Option<u64> = (v25).trim().parse::<u64>().ok();
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
                let mut v38: Rc<UH0> = globs_from_26(&v2, v37);
                US1::US1_Some(v35, v36, v34, v38)
            }
        }
    }
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: US1 = claim_of_row_23(v2);
                let mut v11: Rc<UH1> = match v4 {
                    US1::US1_None => {
                        v1
                    }
                    US1::US1_Some(v5, v6, v7, v8) => {
                        let v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        let mut v7: u64 = v7;
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
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_19(v1, v4);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH1> = method22(v5, v6);
    let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method18(v7, v8)
}
fn decliner_29(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: &Rc<UH0>) -> Rc<str> {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
    let v5: bool = v1.starts_with(&*v4);
    if v5 {
        let mut v6: i32 = (v1.clone().len() as i32);
        let mut v7: i32 = v6.wrapping_sub(1i32);
        let mut v8: usize = ((1i32) as usize);
        let mut v9: i32 = 1i32.wrapping_add(v7);
        let mut v10: usize = ((v9) as usize);
        let mut v11: Rc<str> = Rc::<str>::from(&(v1)[v8..v10]);
        v11
    } else {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    }
}
fn method28(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<UH1>, mut v3: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v4, v5, v6, v7, v8) => {
            let mut v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let mut v6: u64 = v6;
            let mut v7: Rc<UH0> = v7;
            let mut v8: Rc<UH1> = v8;
            let mut v9: Rc<UH1> = method28(v0.clone(), v1, v8, v3);
            let mut v10: bool = v1 < v6;
            let v13: bool = if v10 {
                let mut v11: Rc<str> = decliner_29(&v4, v5.clone(), v6, &v7);
                let mut v12: bool = &*v11 == "";
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
    let mut v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method28(v0, v1, v2, v3)
}
fn lane_wip_15(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let (mut v2, mut v3): (bool, Rc<str>) = agile_claim_store::rig_claims_loaded(&*v0);
    let mut v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v5: Rc<UH1> = claims_of_text_17(v3);
    let mut v6: Rc<UH1> = live_claims_27(v1, v4, v5);
    let mut v7: i32 = 0i32;
    method16(v6, v7)
}
fn method30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH1>, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v4, v5, v6, v7, v8) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let mut v6: u64 = v6;
                let mut v7: Rc<UH0> = v7;
                let mut v8: Rc<UH1> = v8;
                let mut v9: bool = &*v4 == &*v1;
                let mut v11: bool = if v9 {
                    let mut v10: bool = &*v5 == &*v0;
                    v10
                } else {
                    false
                };
                let mut v12: Rc<UH0> = if v11 {
                    v7
                } else {
                    v3
                };
                (v2, v3) = (v8, v12);
                continue;
            }
            UH1::UH1_Nil => {
                return v3;
            }
        }
    }
}
fn method31(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method31(v3, v1);
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
fn closure3(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US3> {
    Rc::new(move |mut v2: Rc<UH0>| -> US3 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US3::US3_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US3> = method33(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US3::US3_None
            }
        }
    })
}
fn method33(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US3> {
    closure3(v0, v1)
}
fn method32(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method32(v0.clone(), v4, v2);
            let mut v6: i32 = 0i32;
            let mut v7: Rc<dyn Fn(Rc<UH0>) -> US3> = method33(v3.clone(), v6);
            let mut v8: US3 = v7(v0);
            let mut v9: bool = match v8 {
                US3::US3_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let mut v10: bool = v9 != true;
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
fn concat5_36(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn civil_of_days_37(mut v0: u64) -> (u64, u64, u64) {
    let mut v1: u64 = v0.wrapping_add(719468u64);
    let mut v2: u64 = v1.wrapping_div(146097u64);
    let mut v3: u64 = v2.wrapping_mul(146097u64);
    let mut v4: u64 = v1.wrapping_sub(v3);
    let v5: u64 = v4.wrapping_div(1460u64);
    let mut v6: u64 = v4.wrapping_sub(v5);
    let mut v7: u64 = v4.wrapping_div(36524u64);
    let mut v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let mut v10: u64 = v8.wrapping_sub(v9);
    let mut v11: u64 = v10.wrapping_div(365u64);
    let mut v12: u64 = 365u64.wrapping_mul(v11);
    let v13: u64 = v11.wrapping_div(4u64);
    let mut v14: u64 = v12.wrapping_add(v13);
    let mut v15: u64 = v11.wrapping_div(100u64);
    let mut v16: u64 = v14.wrapping_sub(v15);
    let v17: u64 = v4.wrapping_sub(v16);
    let v18: u64 = 5u64.wrapping_mul(v17);
    let v19: u64 = v18.wrapping_add(2u64);
    let v20: u64 = v19.wrapping_div(153u64);
    let v21: u64 = 153u64.wrapping_mul(v20);
    let v22: u64 = v21.wrapping_add(2u64);
    let v23: u64 = v22.wrapping_div(5u64);
    let v24: u64 = v17.wrapping_sub(v23);
    let mut v25: u64 = v24.wrapping_add(1u64);
    let mut v26: bool = v20 < 10u64;
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
fn utc_stamp_35(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(1000u64);
    let mut v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_37(v2);
    let mut v6: u64 = v1.wrapping_rem(86400u64);
    let mut v7: u64 = v6.wrapping_div(60u64);
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v10: bool = v4 < 10u64;
    let mut v16: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_36(v11, v12, v13.clone(), v13.clone(), v13)
    } else {
        let mut v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15
    };
    let v17: bool = v5 < 10u64;
    let v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_36(v18, v19, v20.clone(), v20.clone(), v20)
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22
    };
    let v24: Rc<str> = concat5_36(v8, v9.clone(), v16, v9, v23);
    let mut v25: u64 = v7.wrapping_div(60u64);
    let mut v26: bool = v25 < 10u64;
    let v32: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_36(v27, v28, v29.clone(), v29.clone(), v29)
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
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_36(v36, v37, v38.clone(), v38.clone(), v38)
    } else {
        let v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = concat5_36(v32, v33, v41, v42, v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    concat5_36(v24, v45, v44, v43.clone(), v43)
}
fn local_offset_minutes_40() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn local_stamp_39(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i64 = local_offset_minutes_40();
    let mut v2: Rc<str> = { let at = &*v0; let offset = v1; let parsed = (|| -> Option<i64> { let hours: i64 = at.get(11..13)?.parse().ok()?; let minutes: i64 = at.get(14..16)?.parse().ok()?; Some(hours * 60 + minutes) })(); std::rc::Rc::<str>::from(match parsed { Some(total) => { let local = (total + offset).rem_euclid(1440); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; format!(" ({:02}:{:02} {}{:02}:{:02})", local / 60, local % 60, sign, offset.abs() / 60, offset.abs() % 60) } None => String::new() }) };
    v2
}
fn local_clock_38(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = utc_stamp_35(v0);
    let mut v2: Rc<str> = local_stamp_39(v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v4));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v4));
    v9
}
fn receipt_34(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: i32, mut v5: i32, mut v6: i32) -> Rc<str> {
    let mut v11: Rc<str> = string_concat(&["rig agile ", &*v0, " ok claimed=", &*v1, " owner=", &*v2]);
    let mut v12: Rc<str> = utc_stamp_35(v3);
    let mut v15: Rc<str> = local_clock_38(v3);
    let mut v16: Rc<str> = string_concat(&[" expires=", &*v12, " expires_local=", &*v15]);
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
            let mut v38: Rc<str> = Rc::<str>::from((*v36.borrow()).join(&*v37));
            let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) = agile_claim_lease::rig_claim_lease(&*v4, &*v5, v6, &*v38, &*v11, &*v13, &*v14, &*v10, v15);
            (v39, v40, v41, v42, v43, v44, v45, v46)
        }
    };
    let v63: Rc<RefCell<Vec<(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(vec![(v8.clone(), v9, v10.clone(), v62, v56.clone(), v57.clone(), v58, v61.clone(), v13)]));
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
                let v127: Rc<str> = string_concat(&["rig agile release ok id=", &*v4, " owner=", &*v5]);
                let v128: i32 = lane_wip_15(v8, v5);
                let v129: Rc<str> = format_real_8(v128);
                let mut v134: Rc<str> = format_real_8(v60);
                let v136: Rc<str> = string_concat(&[&*v127, " released=", &*v61, " wip=", &*v134, " lane_wip=", &*v129]);
                let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v138: Rc<UH0> = Rc::new(UH0::UH0_Cons(v136, v137));
                let v139: i32 = 0i32;
                printed_3(v138, v139)
            }
            US0::US0_OutcomeGranted => {
                let v71: Rc<UH1> = claims_of_text_17(v10);
                let v72: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v73: Rc<UH0> = method30(v57.clone(), v56.clone(), v71, v72);
                let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
                let v75: Rc<UH0> = split_list_19(v74, v61.clone());
                let v76: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v77: Rc<UH0> = method31(v75, v76);
                let v78: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v79: Rc<UH0> = method32(v77, v73.clone(), v78);
                let v80: i32 = 0i32;
                let mut v81: i32 = method7(v79, v80);
                let v82: i32 = 0i32;
                let v83: i32 = method7(v73.clone(), v82);
                let v84: i32 = v83.wrapping_sub(v81);
                let v85: bool = match Rc::unwrap_or_clone(v73) {
                    UH0::UH0_Nil => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v92: Rc<str> = if v85 {
                    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v86
                } else {
                    let v87: Rc<str> = format_real_8(v81);
                    let v90: Rc<str> = format_real_8(v84);
                    let mut v91: Rc<str> = string_concat(&[" kept=", &*v87, " removed=", &*v90]);
                    v91
                };
                let v93: bool = &*v61 == "";
                let v96: Rc<str> = if v93 {
                    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("paths none"); } LIT.with(|lit| lit.clone()) };
                    v94
                } else {
                    let v95: Rc<str> = string_concat(&["paths ", &*v61]);
                    v95
                };
                let v97: Rc<str> = string_concat(&[&*v96, &*v92]);
                let v98: Rc<str> = receipt_34(v3, v56.clone(), v57.clone(), v58, v6, v60, v59);
                let v100: i32 = lane_wip_15(v8.clone(), v57);
                let v101: Rc<str> = format_real_8(v100);
                let v102: Rc<str> = string_concat(&[&*v98, " lane_wip=", &*v101]);
                let v103: Rc<str> = agile_claim_item::rig_claim_item_lines(&*v8, &*v56);
                let v104: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v105: Rc<UH0> = Rc::new(UH0::UH0_Cons(v97, v104));
                let v106: Rc<UH0> = Rc::new(UH0::UH0_Cons(v103, v105));
                let v107: Rc<UH0> = Rc::new(UH0::UH0_Cons(v102, v106));
                let v108: i32 = 0i32;
                printed_3(v107, v108)
            }
            US0::US0_OutcomeNone => {
                let v119: Rc<str> = format_real_8(v60);
                let v120: Rc<str> = string_concat(&["rig agile next ok claimed=none reason=", &*v61, " wip=", &*v119]);
                let v121: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v122: Rc<UH0> = Rc::new(UH0::UH0_Cons(v120, v121));
                let v123: i32 = 3i32;
                printed_3(v122, v123)
            }
            US0::US0_OutcomeRefused => {
                let v112: Rc<str> = string_concat(&["rig agile claim refused id=", &*v4, " reason=", &*v61]);
                let v113: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v114: Rc<UH0> = Rc::new(UH0::UH0_Cons(v112, v113));
                let v115: i32 = 5i32;
                printed_3(v114, v115)
            }
            US0::US0_OutcomeReleaseRefused => {
                let v143: Rc<str> = string_concat(&["rig agile release refused id=", &*v4, " reason=", &*v61]);
                let v144: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v145: Rc<UH0> = Rc::new(UH0::UH0_Cons(v143, v144));
                let v146: i32 = 5i32;
                printed_3(v145, v146)
            }
            US0::US0_OutcomeUnknown => {
                let v151: Rc<str> = string_concat(&["agile ", &*v3, " rejected: no open task with id ", &*v4]);
                let v152: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v151 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v152
            }
        }
    }
}
fn method42(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn get_current_directory_41() -> Rc<str> {
    let mut v1: Result<std::path::PathBuf, std::io::Error> = std::env::current_dir();
    let mut v3: std::path::PathBuf = v1.unwrap();
    let mut v4: std::path::PathBuf = method42(v3);
    let mut v6: std::path::Display = v4.display();
    let mut v8: std::string::String = format!("{}", v6);
    let mut v10: Rc<str> = Rc::<str>::from(String::as_str(&v8));
    v10
}
fn anchored_44(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: bool = v4.starts_with(&*v3);
    let mut v14: bool = if v5 {
        true
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":/"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Option<usize> = v4.find(&*v6);
        let mut v8: Option<usize> = method24(v7);
        let mut v9: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v10: Option<i32> = v8.map(|x| v9(x));
        let mut v12: i32 = v10.unwrap_or(-1i32);
        let v13: bool = v12 == 1i32;
        v13
    };
    let mut v15: i32 = (v4.clone().len() as i32);
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("./"); } LIT.with(|lit| lit.clone()) };
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
        let mut v26: Rc<str> = string_concat(&[&*v24, "/", &*v23]);
        v26
    }
}
fn loop_45(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn directory_exists_47(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn file_exists_48(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn closure4() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn repository_of_46(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = string_concat(&[&*v0, "/.git"]);
        let mut v2: bool = directory_exists_47(v1.clone());
        let mut v4: bool = if v2 {
            true
        } else {
            file_exists_48(v1)
        };
        if v4 {
            return v0;
        } else {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Option<usize> = v0.rfind(&*v5);
            let mut v7: Option<usize> = method24(v6);
            let mut v8: Rc<dyn Fn((usize)) -> i32> = closure4();
            let mut v9: Option<i32> = v7.map(|x| v8(x));
            let mut v11: i32 = v9.unwrap_or(-1i32);
            let mut v12: bool = v11 <= 0i32;
            if v12 {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v13;
            } else {
                let mut v14: usize = ((0i32) as usize);
                let mut v15: usize = ((v11) as usize);
                let mut v16: Rc<str> = Rc::<str>::from(&(v0)[v14..v15]);
                v0 = v16;
                continue;
            }
        }
    }
}
fn claims_base_43(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = anchored_44(v0.clone(), v1);
    let mut v3: Rc<str> = loop_45(v2);
    let mut v4: Rc<str> = repository_of_46(v3);
    let v5: bool = &*v4 == "";
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
        v8
    } else {
        v4
    }
}
fn method49(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method49(v0.clone(), v4, v2);
            let mut v6: Rc<str> = anchored_44(v0, v3);
            Rc::new(UH0::UH0_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn claim_requested_1(mut v0: Rc<str>, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: i32, mut v8: Rc<UH0>) -> i32 {
    let mut v9: Rc<str> = get_current_directory_41();
    let mut v10: Rc<str> = claims_base_43(v9, v0.clone());
    let mut v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v12: Rc<UH0> = method49(v10, v8, v11);
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
    let mut v25: bool = v13 == false;
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
fn command_line_argument_count_51() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_52(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_50(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_51();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US4 = if v4 {
        let v5: Rc<str> = command_line_argument_52(v0);
        US4::US4_Some(v5)
    } else {
        US4::US4_None
    };
    match v8 {
        US4::US4_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US4::US4_Some(v10) => {
            let mut v10: Rc<str> = v10;
            let mut v11: i32 = v0.wrapping_add(1i32);
            let mut v12: Rc<UH0> = arguments_from_50(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn method54(mut v0: i32, mut v1: &Rc<RefCell<Mut2>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn method53(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2]));
                let v5: i32 = (v1.clone().borrow().len() as i32);
                let mut v6: i32 = (v4.clone().borrow().len() as i32);
                let mut v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: 0i32 }));
                while method54(v7, &v9) {
                    let mut v11: i32 = v9.borrow().l0;
                    let mut v12: bool = v11 < v5;
                    let mut v16: Rc<str> = if v12 {
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
fn listed_paths_56(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: bool = v1 < 0i32;
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
fn order_of_55(mut v0: &Rc<str>, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: i32, mut v8: &Rc<RefCell<Vec<Rc<str>>>>) -> (bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<UH0>) {
    let mut v9: i32 = (v8.clone().borrow().len() as i32);
    let mut v10: i32 = v9.wrapping_sub(1i32);
    let mut v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v12: Rc<UH0> = listed_paths_56(v8, v10, v11);
    (v1, v2, v3, v4, v5, v6, v7, v12)
}
fn claim_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_50(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
                            let mut v9: Rc<RefCell<Vec<Rc<str>>>> = method53(v7, v8);
                            let mut v10: Rc<RefCell<Vec<(Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>)>>> = agile_claim_request::agile_claim_request_parsed(&v4, v9);
                            let (mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19): (Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>) = v10.borrow()[0i32 as usize].clone();
                            let v20: bool = &*v11 != "";
                            if v20 {
                                let v23: Rc<str> = string_concat(&["agile ", &*v4, ": ", &*v11]);
                                let v24: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v23 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v24
                            } else {
                                let (mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32): (bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<UH0>) = order_of_55(&v11, v12, v13, v14, v15, v16, v17, v18, &v19);
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
                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile: usage: next|claim|release <root> ..."); } LIT.with(|lit| lit.clone()) };
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
fn loop_61(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_62(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(6i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_63(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_64(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method65(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method65(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let mut v15: US6 = if v6 {
                let mut v7: US7 = US7::US7_HeadType;
                US6::US6_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US7 = US7::US7_HeadType;
                    US6::US6_SymbolHead(v11)
                } else {
                    US6::US6_OtherLine
                }
            };
            let v27: US6 = match &v15 {
                US6::US6_OtherLine => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US7 = US7::US7_HeadInl;
                        US6::US6_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US7 = US7::US7_HeadInl;
                            US6::US6_SymbolHead(v22)
                        } else {
                            US6::US6_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US6 = match &v27 {
                US6::US6_OtherLine => {
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let mut v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US7 = US7::US7_HeadUnion;
                        US6::US6_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US7 = US7::US7_HeadUnion;
                            US6::US6_SymbolHead(v34)
                        } else {
                            US6::US6_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let mut v51: US6 = match &v39 {
                US6::US6_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US7 = US7::US7_HeadOpen;
                        US6::US6_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US7 = US7::US7_HeadOpen;
                            US6::US6_SymbolHead(v46)
                        } else {
                            US6::US6_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US6::US6_SymbolHead(v52) => {
                    let v52: US7 = v52;
                    let v60: Rc<str> = match &v52 {
                        US7::US7_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US7::US7_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US7::US7_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US7::US7_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US7::US7_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US7::US7_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US7::US7_HeadType => {
                            v5
                        }
                        US7::US7_HeadUnion => {
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
fn method71(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method71(v3, v1);
            let v5: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method72(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure6() -> Rc<dyn Fn((i32)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US3> = Rc::new(move |mut v0: (i32)| -> US3 {
        let mut v1: i32 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_line_70(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> US8 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = v0.borrow()[2i32 as usize].clone();
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let mut v6: Rc<UH0> = split_list_19(v1, v5);
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = method71(v6, v7);
    match Rc::unwrap_or_clone(v8) {
        UH0::UH0_Cons(v9, v10) => {
            let mut v9: Rc<str> = v9;
            let mut v10: Rc<UH0> = v10;
            match Rc::unwrap_or_clone(v10) {
                UH0::UH0_Cons(v11, v12) => {
                    let mut v11: Rc<str> = v11;
                    let mut v12: Rc<UH0> = v12;
                    match Rc::unwrap_or_clone(v12) {
                        UH0::UH0_Cons(v13, v14) => {
                            let v13: Rc<str> = v13;
                            let mut v14: Rc<UH0> = v14;
                            match Rc::unwrap_or_clone(v14) {
                                UH0::UH0_Cons(v15, v16) => {
                                    let mut v15: Rc<str> = v15;
                                    let mut v16: Rc<UH0> = v16;
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Minutes "); } LIT.with(|lit| lit.clone()) };
                                    let v18: bool = v11.starts_with(&*v17);
                                    let v20: Rc<str> = if v18 {
                                        let v19: Rc<str> = Rc::<str>::from(v11.replace(&*v17, &*v4));
                                        v19
                                    } else {
                                        v4.clone()
                                    };
                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                                    let v22: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v4));
                                    let v23: Option<i32> = (v22).trim().parse::<i32>().ok();
                                    let v24: Option<i32> = method72(v23);
                                    let mut v25: Rc<dyn Fn((i32)) -> US3> = closure6();
                                    let mut v26: Option<US3> = v24.map(|x| v25(x));
                                    let mut v28: US3 = US3::US3_None;
                                    let mut v29: US3 = v28.clone();
                                    let v30: US3 = v26.unwrap_or(v29);
                                    let v33: i32 = match v30 {
                                        US3::US3_None => {
                                            0i32
                                        }
                                        US3::US3_Some(v31) => {
                                            let v31: i32 = v31;
                                            v31
                                        }
                                    };
                                    let v34: Rc<str> = v0.borrow()[1i32 as usize].clone();
                                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load "); } LIT.with(|lit| lit.clone()) };
                                    let v36: bool = v13.starts_with(&*v35);
                                    let mut v38: Rc<str> = if v36 {
                                        let v37: Rc<str> = Rc::<str>::from(v13.replace(&*v35, &*v4));
                                        v37
                                    } else {
                                        v4.clone()
                                    };
                                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) };
                                    let v40: bool = v15.starts_with(&*v39);
                                    let v42: Rc<str> = if v40 {
                                        let v41: Rc<str> = Rc::<str>::from(v15.replace(&*v39, &*v4));
                                        v41
                                    } else {
                                        v4
                                    };
                                    US8::US8_PlanningLine(v34, v33, v38, v42)
                                }
                                _ => {
                                    US8::US8_OtherLine
                                }
                            }
                        }
                        _ => {
                            US8::US8_OtherLine
                        }
                    }
                }
                _ => {
                    US8::US8_OtherLine
                }
            }
        }
        _ => {
            US8::US8_OtherLine
        }
    }
}
fn method74(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method74(v3, v1);
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
fn method75(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                (v0, v1) = (v3, v2);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn task_line_73(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> US8 {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = v1.wrapping_sub(9i32);
    let mut v4: Rc<str> = v0.borrow()[v3 as usize].clone();
    let v5: Rc<UH0> = split_list_19(v2, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method71(v5, v6);
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method74(v7, v8);
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = method75(v9, v10);
    let mut v12: Rc<str> = v0.borrow()[1i32 as usize].clone();
    let v13: i32 = v1.wrapping_sub(6i32);
    let mut v14: Rc<str> = v0.borrow()[v13 as usize].clone();
    US8::US8_TaskLine(v12, v11, v14)
}
fn state_line_of_69(mut v0: Rc<str>) -> US8 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: bool = v3 < 3i32;
    if v4 {
        US8::US8_OtherLine
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl note_of_"); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v0.starts_with(&*v6);
        if v7 {
            let mut v8: Rc<str> = v2.borrow()[1i32 as usize].clone();
            US8::US8_NoteLine(v8, v0)
        } else {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl wait_of_"); } LIT.with(|lit| lit.clone()) };
            let mut v11: bool = v0.starts_with(&*v10);
            let mut v14: bool = if v11 {
                true
            } else {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= wait_of_"); } LIT.with(|lit| lit.clone()) };
                let v13: bool = v0.starts_with(&*v12);
                v13
            };
            let mut v16: bool = if v14 {
                let mut v15: bool = v3 >= 7i32;
                v15
            } else {
                false
            };
            if v16 {
                let v17: Rc<str> = v2.borrow()[1i32 as usize].clone();
                let v18: Rc<str> = v2.borrow()[3i32 as usize].clone();
                let v20: Rc<str> = v2.borrow()[5i32 as usize].clone();
                let v21: Rc<str> = string_concat(&[&*v18, ":", &*v20]);
                US8::US8_WaitLine(v17, v21)
            } else {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Marker (\""); } LIT.with(|lit| lit.clone()) };
                let v24: bool = v0.contains(&*v23);
                let mut v26: bool = if v24 {
                    let mut v25: bool = v3 >= 5i32;
                    v25
                } else {
                    false
                };
                if v26 {
                    let v27: Rc<str> = v2.borrow()[1i32 as usize].clone();
                    let mut v29: Rc<str> = v2.borrow()[3i32 as usize].clone();
                    let v30: Rc<str> = string_concat(&[&*v27, "=", &*v29]);
                    US8::US8_MarkerLine(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl planning_of_"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v0.starts_with(&*v32);
                    if v33 {
                        planning_line_70(v2)
                    } else {
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                        let v36: bool = v0.starts_with(&*v35);
                        let v39: bool = if v36 {
                            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() = Task ("); } LIT.with(|lit| lit.clone()) };
                            let mut v38: bool = v0.contains(&*v37);
                            v38
                        } else {
                            false
                        };
                        let v41: bool = if v39 {
                            let v40: bool = v3 >= 10i32;
                            v40
                        } else {
                            false
                        };
                        if v41 {
                            task_line_73(v2)
                        } else {
                            US8::US8_OtherLine
                        }
                    }
                }
            }
        }
    }
}
fn method68(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: US8 = state_line_of_69(v2);
                let mut v10: Rc<str> = match v4 {
                    US8::US8_MarkerLine(v5) => {
                        let v5: Rc<str> = v5;
                        let mut v6: bool = &*v1 == "";
                        if v6 {
                            v5
                        } else {
                            let mut v8: Rc<str> = string_concat(&[&*v1, "\n", &*v5]);
                            v8
                        }
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v3, v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn declared_markers_67(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_19(v1, v4);
    method68(v5, v3)
}
fn method79(mut v0: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>) -> Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    v0
}
fn closure7() -> Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US9> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US9> = Rc::new(move |mut v0: ((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))| -> US9 {
        let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = (v0);
        US9::US9_Some(v1, v2, v3, v4, v5, v6, v7, v8)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn folded_78(mut v0: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>, mut v1: US8) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> {
    match v1 {
        US8::US8_MarkerLine(v175) => {
            let v175: Rc<str> = v175;
            v0
        }
        US8::US8_NoteLine(v87, v88) => {
            let v87: Rc<str> = v87;
            let v88: Rc<str> = v88;
            let v89: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v87).cloned();
            let v90: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method79(v89);
            let mut v91: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US9> = closure7();
            let mut v92: Option<US9> = v90.map(|x| v91(x));
            let v94: US9 = US9::US9_None;
            let v95: US9 = v94.clone();
            let v96: US9 = v92.unwrap_or(v95);
            let (mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v96 {
                US9::US9_Some(v97, v98, v99, v100, v101, v102, v103, v104) => {
                    let v97: Rc<str> = v97.clone();
                    let v98: Rc<str> = v98.clone();
                    let v99: i32 = *v99;
                    let v100: Rc<str> = v100.clone();
                    let v101: Rc<str> = v101.clone();
                    let v102: Rc<str> = v102.clone();
                    let v103: Rc<str> = v103.clone();
                    let v104: Rc<str> = v104.clone();
                    (v97, v98, v99, v100, v101, v102, v103, v104)
                }
                _ => {
                    let (mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v96 {
                        US9::US9_None => {
                            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v105.clone(), v105.clone(), 0i32, v105.clone(), v105.clone(), v105.clone(), v105.clone(), v105.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v106, v107, v108, v109, v110, v111, v112, v113)
                }
            };
            let v123: Rc<str> = string_concat(&[&*v119, "\n", &*v88]);
            let mut v124: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v126: bool = true; // 1;
            let mut v124 = v124;
            let v128: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v124.insert(v87, (v114, v115, v116, v117, v118, v123, v120, v121));
            let v129: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v124);
            v129
        }
        US8::US8_OtherLine => {
            v0
        }
        US8::US8_PlanningLine(v44, v45, v46, v47) => {
            let v44: Rc<str> = v44;
            let v45: i32 = v45;
            let v46: Rc<str> = v46;
            let v47: Rc<str> = v47;
            let v48: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v44).cloned();
            let v49: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method79(v48);
            let mut v50: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US9> = closure7();
            let mut v51: Option<US9> = v49.map(|x| v50(x));
            let v53: US9 = US9::US9_None;
            let v54: US9 = v53.clone();
            let v55: US9 = v51.unwrap_or(v54);
            let (mut v73, mut v74, mut v78, mut v79, mut v80): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v55 {
                US9::US9_Some(v56, v57, v58, v59, v60, v61, v62, v63) => {
                    let v56: Rc<str> = v56.clone();
                    let v57: Rc<str> = v57.clone();
                    let v58: i32 = *v58;
                    let v59: Rc<str> = v59.clone();
                    let v60: Rc<str> = v60.clone();
                    let v61: Rc<str> = v61.clone();
                    let v62: Rc<str> = v62.clone();
                    let v63: Rc<str> = v63.clone();
                    (v56, v57, v61, v62, v63)
                }
                _ => {
                    let (mut v65, mut v66, mut v70, mut v71, mut v72): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v55 {
                        US9::US9_None => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v64.clone(), v64.clone(), v64.clone(), v64.clone(), v64.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v65, v66, v70, v71, v72)
                }
            };
            let mut v81: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v83: bool = true; // 1;
            let mut v81 = v81;
            let v85: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v81.insert(v44, (v73, v74, v45, v46, v47, v78, v79, v80));
            let v86: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v81);
            v86
        }
        US8::US8_TaskLine(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let v5: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v2).cloned();
            let mut v6: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method79(v5);
            let mut v7: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US9> = closure7();
            let mut v8: Option<US9> = v6.map(|x| v7(x));
            let mut v10: US9 = US9::US9_None;
            let mut v11: US9 = v10.clone();
            let mut v12: US9 = v8.unwrap_or(v11);
            let (mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                US9::US9_Some(v13, v14, v15, v16, v17, v18, v19, v20) => {
                    let v13: Rc<str> = v13.clone();
                    let mut v14: Rc<str> = v14.clone();
                    let mut v15: i32 = *v15;
                    let mut v16: Rc<str> = v16.clone();
                    let v17: Rc<str> = v17.clone();
                    let v18: Rc<str> = v18.clone();
                    let v19: Rc<str> = v19.clone();
                    let v20: Rc<str> = v20.clone();
                    (v15, v16, v17, v18, v19, v20)
                }
                _ => {
                    let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v12 {
                        US9::US9_None => {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (0i32, v21.clone(), v21.clone(), v21.clone(), v21.clone(), v21.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v24, v25, v26, v27, v28, v29)
                }
            };
            let mut v38: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v40: bool = true; // 1;
            let mut v38 = v38;
            let v42: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v38.insert(v2, (v3, v4, v32, v33, v34, v35, v36, v37));
            let v43: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v38);
            v43
        }
        US8::US8_WaitLine(v130, v131) => {
            let v130: Rc<str> = v130;
            let v131: Rc<str> = v131;
            let v132: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v130).cloned();
            let v133: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method79(v132);
            let mut v134: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US9> = closure7();
            let mut v135: Option<US9> = v133.map(|x| v134(x));
            let v137: US9 = US9::US9_None;
            let v138: US9 = v137.clone();
            let v139: US9 = v135.unwrap_or(v138);
            let (mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v139 {
                US9::US9_Some(v140, v141, v142, v143, v144, v145, v146, v147) => {
                    let v140: Rc<str> = v140.clone();
                    let v141: Rc<str> = v141.clone();
                    let v142: i32 = *v142;
                    let v143: Rc<str> = v143.clone();
                    let v144: Rc<str> = v144.clone();
                    let v145: Rc<str> = v145.clone();
                    let v146: Rc<str> = v146.clone();
                    let v147: Rc<str> = v147.clone();
                    (v140, v141, v142, v143, v144, v145, v146, v147)
                }
                _ => {
                    let (mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v139 {
                        US9::US9_None => {
                            let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v148.clone(), v148.clone(), 0i32, v148.clone(), v148.clone(), v148.clone(), v148.clone(), v148.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v149, v150, v151, v152, v153, v154, v155, v156)
                }
            };
            let v165: bool = &*v163 == "";
            let v168: Rc<str> = if v165 {
                v131
            } else {
                let v167: Rc<str> = string_concat(&[&*v163, ",", &*v131]);
                v167
            };
            let mut v169: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v171: bool = true; // 1;
            let mut v169 = v169;
            let v173: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v169.insert(v130, (v157, v158, v159, v160, v161, v162, v168, v164));
            let v174: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v169);
            v174
        }
    }
}
fn method77(mut v0: Rc<UH0>, mut v1: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: US8 = state_line_of_69(v2);
                let v5: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = folded_78(v1, v4);
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn facts_of_76(mut v0: Rc<str>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_19(v1, v4);
    let mut v7: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::collections::BTreeMap::new();
    let mut v8: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v7);
    method77(v5, v8)
}
fn method80(mut v0: Rc<str>, mut v1: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>, mut v2: Rc<UH0>, mut v3: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v1.get(&v4).cloned();
                let mut v7: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method79(v6);
                let mut v8: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US9> = closure7();
                let mut v9: Option<US9> = v7.map(|x| v8(x));
                let mut v11: US9 = US9::US9_None;
                let mut v12: US9 = v11.clone();
                let v13: US9 = v9.unwrap_or(v12);
                let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v13 {
                    US9::US9_Some(v14, v15, v16, v17, v18, v19, v20, v21) => {
                        let mut v14: Rc<str> = v14.clone();
                        let mut v15: Rc<str> = v15.clone();
                        let mut v16: i32 = *v16;
                        let v17: Rc<str> = v17.clone();
                        let v18: Rc<str> = v18.clone();
                        let v19: Rc<str> = v19.clone();
                        let v20: Rc<str> = v20.clone();
                        let v21: Rc<str> = v21.clone();
                        (v14, v15, v16, v17, v18, v19, v20, v21)
                    }
                    _ => {
                        let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v13 {
                            US9::US9_None => {
                                std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dict.present: the key is not in the dict"); } LIT.with(|lit| lit.clone()) }))
                            }
                            _ => unreachable!(),
                        };
                        (v30, v31, v32, v33, v34, v35, v36, v37)
                    }
                };
                let v46: bool = &*v38 == "";
                let v48: Rc<UH2> = if v46 {
                    v3
                } else {
                    Rc::new(UH2::UH2_Cons(v4, v38, v39, v40, v41, v42, v43, v44, v0.clone(), v3))
                };
                (v2, v3) = (v5, v48);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn method81(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let v5: i32 = v5;
                let mut v6: Rc<str> = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let mut v10: Rc<str> = v10;
                let mut v11: Rc<UH2> = v11;
                let mut v12: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10, v1));
                (v0, v1) = (v11, v12);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method82(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: Rc<UH2>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let mut v6: i32 = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let mut v10: Rc<str> = v10;
                let mut v11: Rc<str> = v11;
                let mut v12: Rc<UH2> = v12;
                v0.borrow_mut().push((v3, v4, v5, v6, v7, v8, v9, v10, v11));
                let v13: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v12, v13);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn fact_rows_of_66(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    let mut v1: Rc<str> = declared_markers_67(v0.clone());
    let mut v2: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = facts_of_76(v0);
    let mut v3: Vec<Rc<str>> = v2.keys().cloned().collect::<Vec<_>>();
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: bool = true; let _vec_fold_ = v3.into_iter().fold(v4, |acc, x| { //;
    let mut v8: Rc<UH0> = acc;
    let mut v10: Rc<str> = x;
    let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v8));
    let v13: bool = true; v12 });
    let mut v15: Rc<UH0> = _vec_fold_;
    let mut v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH0> = method20(v15, v16);
    let v18: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v19: Rc<UH2> = method80(v1, v2, v17, v18);
    let v20: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v21: Rc<UH2> = method81(v19, v20);
    let v22: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v23: i32 = 0i32;
    let v24: i32 = method82(v22.clone(), v21, v23);
    let mut v25: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new(v22.borrow().clone());
    let mut v26: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v25).as_ref().clone()));
    v26
}
fn method83(mut v0: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>, mut v1: i32, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = (v0)[v4].clone();
            let mut v14: i32 = v1.wrapping_sub(1i32);
            let mut v15: Rc<UH2> = Rc::new(UH2::UH2_Cons(v5, v6, v7, v8, v9, v10, v11, v12, v13, v2));
            (v1, v2) = (v14, v15);
            continue;
        }
    }
}
fn method84(mut v0: Rc<str>, mut v1: Rc<UH2>) -> US10 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let mut v6: i32 = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let mut v10: Rc<str> = v10;
                let mut v11: Rc<str> = v11;
                let mut v12: Rc<UH2> = v12;
                let v13: bool = &*v3 == &*v0;
                if v13 {
                    return US10::US10_Some(v3, v4, v5, v6, v7, v8, v9, v10, v11);
                } else {
                    v1 = v12;
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return US10::US10_None;
            }
        }
    }
}
fn tags_of_item_60(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = string_concat(&[&*v4, "/state/agile.spi"]);
    let mut v6: bool = std::path::Path::new(&*v5).is_file();
    let v58: Rc<str> = if v6 {
        let (mut v7, mut v8): (bool, Rc<str>) = match std::fs::read(&*v5) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v7 {
            let mut v9: Rc<str> = string_concat(&["rig state read failed: ", &*v5]);
            std::panic::panic_any::<std::string::String>(format!("{}", v9))
        } else {
            let mut v11: Rc<str> = Rc::<str>::from(v5.replace(&*v2, &*v3));
            let mut v12: Rc<str> = loop_61(v11);
            let v13: Option<usize> = v12.rfind(&*v3);
            let mut v14: Option<usize> = method24(v13);
            let mut v15: Rc<dyn Fn((usize)) -> i32> = closure4();
            let mut v16: Option<i32> = v14.map(|x| v15(x));
            let v18: i32 = v16.unwrap_or(-1i32);
            let v19: bool = v18 < 0i32;
            let v23: bool = if v19 {
                true
            } else {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v21: bool = v12.ends_with(&*v20);
                let v22: bool = v21 == false;
                v22
            };
            let v44: US5 = if v23 {
                US5::US5_OtherFile
            } else {
                let mut v25: usize = ((0i32) as usize);
                let mut v26: usize = ((v18) as usize);
                let v27: Rc<str> = Rc::<str>::from(&(v12)[v25..v26]);
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let mut v29: bool = v27.ends_with(&*v28);
                if v29 {
                    let v30: Rc<str> = loop_62(v27);
                    US5::US5_StateFile(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v27.ends_with(&*v32);
                    if v33 {
                        let v34: Rc<str> = loop_63(v27);
                        US5::US5_HistoryFile(v34)
                    } else {
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v37: bool = v27.ends_with(&*v36);
                        if v37 {
                            let mut v38: Rc<str> = loop_64(v27);
                            US5::US5_ArchiveFile(v38)
                        } else {
                            US5::US5_OtherFile
                        }
                    }
                }
            };
            match v44 {
                US5::US5_OtherFile => {
                    v8
                }
                _ => {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v46: Rc<UH0> = split_list_19(v45.clone(), v8);
                    let v47: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v48: Rc<UH0> = method65(v46, v47);
                    let v49: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let mut v50: i32 = 0i32;
                    let mut v51: i32 = method5(v49.clone(), v48, v50);
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
    let v59: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = fact_rows_of_66(v58);
    let v60: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new((v59).borrow().clone());
    let v61: usize = (v60).len();
    let v62: i32 = ((v61) as i32);
    let v63: i32 = v62.wrapping_sub(1i32);
    let v64: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v65: Rc<UH2> = method83(v60, v63, v64);
    let v66: US10 = method84(v1, v65);
    match v66 {
        US10::US10_None => {
            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v76
        }
        US10::US10_Some(v67, v68, v69, v70, v71, v72, v73, v74, v75) => {
            let v67: Rc<str> = v67;
            let v68: Rc<str> = v68;
            let v69: Rc<str> = v69;
            let v70: i32 = v70;
            let v71: Rc<str> = v71;
            let v72: Rc<str> = v72;
            let v73: Rc<str> = v73;
            let v74: Rc<str> = v74;
            let v75: Rc<str> = v75;
            v69
        }
    }
}
fn tag_listed_86(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v3: Rc<str> = string_concat(&[",", &*v1, ","]);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
    let mut v8: Rc<str> = string_concat(&[",", &*v6, ","]);
    let mut v9: Option<usize> = v8.find(&*v3);
    let mut v10: Option<usize> = method24(v9);
    let mut v11: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v12: Option<i32> = v10.map(|x| v11(x));
    let mut v14: i32 = v12.unwrap_or(-1i32);
    let mut v15: bool = v14 >= 0i32;
    v15
}
fn team_tagged_87(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = string_concat(&[",", &*v3]);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",team:"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Option<usize> = v4.find(&*v5);
    let mut v7: Option<usize> = method24(v6);
    let mut v8: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v9: Option<i32> = v7.map(|x| v8(x));
    let mut v11: i32 = v9.unwrap_or(-1i32);
    let mut v12: bool = v11 >= 0i32;
    v12
}
fn on_team_85(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: bool = team_tagged_87(v2.clone());
    if v3 {
        let mut v4: Rc<str> = string_concat(&["team:", &*v0]);
        tag_listed_86(v2, v4)
    } else {
        let mut v6: bool = tag_listed_86(v2.clone(), v0.clone());
        let mut v9: bool = if v6 {
            true
        } else {
            let mut v7: Rc<str> = string_concat(&[&*v0, "-team"]);
            tag_listed_86(v2, v7)
        };
        if v9 {
            true
        } else {
            let mut v10: Rc<str> = Rc::<str>::from(v0.to_uppercase());
            let mut v11: Rc<str> = string_concat(&[&*v10, "-"]);
            let mut v12: bool = v1.starts_with(&*v11);
            v12
        }
    }
}
fn method59(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH1>, mut v3: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v4, v5, v6, v7, v8) => {
            let mut v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let mut v6: u64 = v6;
            let mut v7: Rc<UH0> = v7;
            let mut v8: Rc<UH1> = v8;
            let mut v9: Rc<UH1> = method59(v0.clone(), v1.clone(), v8, v3);
            let mut v10: bool = &*v1 == "";
            let v13: bool = if v10 {
                true
            } else {
                let mut v11: Rc<str> = tags_of_item_60(v0, v4.clone());
                on_team_85(v1, v4.clone(), v11)
            };
            if v13 {
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
fn format_real_90(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method9(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn claim_line_89(mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: Rc<UH0>) -> Rc<str> {
    let v5: u64 = v3.wrapping_sub(v0);
    let mut v6: u64 = v5.wrapping_div(60000u64);
    let mut v7: bool = match &*v4 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    let mut v16: Rc<str> = if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
        v8
    } else {
        let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let mut v10: i32 = 0i32;
        let mut v11: i32 = method5(v9.clone(), v4, v10);
        let mut v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
        let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v14));
        v15
    };
    let v21: Rc<str> = utc_stamp_35(v3);
    let v24: Rc<str> = local_clock_38(v3);
    let v27: Rc<str> = format_real_90(v6);
    let v30: Rc<str> = string_concat(&["  ", &*v1, "  ", &*v2, "  expires=", &*v21, " local=", &*v24, " left_min=", &*v27, "  paths=", &*v16]);
    v30
}
fn method88(mut v0: u64, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let v5: u64 = v5;
            let mut v6: Rc<UH0> = v6;
            let mut v7: Rc<UH1> = v7;
            let mut v8: Rc<UH0> = method88(v0, v7, v2);
            let mut v9: Rc<str> = claim_line_89(v0, v3, v4, v5, v6);
            Rc::new(UH0::UH0_Cons(v9, v8))
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn claims_listed_58(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let (mut v3, mut v4): (bool, Rc<str>) = agile_claim_store::rig_claims_loaded(&*v0);
    let v5: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v6: Rc<UH1> = claims_of_text_17(v4);
    let mut v7: Rc<UH1> = live_claims_27(v1.clone(), v5, v6);
    let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH1> = method59(v0, v2.clone(), v7, v8);
    let mut v10: i32 = 0i32;
    let mut v11: i32 = method16(v9.clone(), v10);
    let mut v12: bool = &*v1 == "";
    let mut v15: Rc<str> = if v12 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    } else {
        let mut v14: Rc<str> = string_concat(&[" lane=", &*v1]);
        v14
    };
    let mut v16: bool = &*v2 == "";
    let v19: Rc<str> = if v16 {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v17
    } else {
        let v18: Rc<str> = string_concat(&[" team=", &*v2]);
        v18
    };
    let v21: Rc<str> = format_real_8(v11);
    let v23: Rc<str> = string_concat(&["rig agile claims ok live=", &*v21, &*v15, &*v19]);
    let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v25: Rc<UH0> = method88(v5, v9, v24);
    let mut v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v27: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v26));
    let mut v28: Rc<UH0> = method11(v25, v27);
    let mut v29: i32 = 0i32;
    printed_3(v28, v29)
}
fn claims_run_57() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_50(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH0::UH0_Cons(v10, v11) => {
                                    let mut v10: Rc<str> = v10;
                                    let mut v11: Rc<UH0> = v11;
                                    match Rc::unwrap_or_clone(v11) {
                                        UH0::UH0_Cons(v12, v13) => {
                                            let mut v12: Rc<str> = v12;
                                            let v13: Rc<UH0> = v13;
                                            match Rc::unwrap_or_clone(v13) {
                                                UH0::UH0_Cons(v24, v25) => {
                                                    let v24: Rc<str> = v24;
                                                    let mut v25: Rc<UH0> = v25;
                                                    match Rc::unwrap_or_clone(v25) {
                                                        UH0::UH0_Cons(v26, v27) => {
                                                            let mut v26: Rc<str> = v26;
                                                            let v27: Rc<UH0> = v27;
                                                            match Rc::unwrap_or_clone(v27) {
                                                                UH0::UH0_Nil => {
                                                                    let mut v28: bool = &*v10 == "--as";
                                                                    let v30: bool = if v28 {
                                                                        let mut v29: bool = &*v24 == "--team";
                                                                        v29
                                                                    } else {
                                                                        false
                                                                    };
                                                                    if v30 {
                                                                        claims_listed_58(v6, v12, v26)
                                                                    } else {
                                                                        let v32: bool = &*v10 == "--team";
                                                                        let v34: bool = if v32 {
                                                                            let v33: bool = &*v24 == "--as";
                                                                            v33
                                                                        } else {
                                                                            false
                                                                        };
                                                                        if v34 {
                                                                            claims_listed_58(v6, v26, v12)
                                                                        } else {
                                                                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>] [--team <team>]"); } LIT.with(|lit| lit.clone()) };
                                                                            let v37: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v36 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                            v37
                                                                        }
                                                                    }
                                                                }
                                                                _ => {
                                                                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>] [--team <team>]"); } LIT.with(|lit| lit.clone()) };
                                                                    let v41: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v40 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                    v41
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>] [--team <team>]"); } LIT.with(|lit| lit.clone()) };
                                                            let v44: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v43 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                            v44
                                                        }
                                                    }
                                                }
                                                UH0::UH0_Nil => {
                                                    let mut v14: bool = &*v10 == "--as";
                                                    if v14 {
                                                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        claims_listed_58(v6, v12, v15)
                                                    } else {
                                                        let v17: bool = &*v10 == "--team";
                                                        if v17 {
                                                            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                            claims_listed_58(v6, v18, v12)
                                                        } else {
                                                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>] [--team <team>]"); } LIT.with(|lit| lit.clone()) };
                                                            let v21: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v20 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                            v21
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>] [--team <team>]"); } LIT.with(|lit| lit.clone()) };
                                            let v49: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v48 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                            v49
                                        }
                                    }
                                }
                                UH0::UH0_Nil => {
                                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    claims_listed_58(v6, v8.clone(), v8.clone())
                                }
                            }
                        }
                        _ => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>] [--team <team>]"); } LIT.with(|lit| lit.clone()) };
                            let v54: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v53 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v54
                        }
                    }
                }
                _ => {
                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>] [--team <team>]"); } LIT.with(|lit| lit.clone()) };
                    let v57: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v56 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v57
                }
            }
        }
        _ => {
            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile claims: usage: claims <root> [--as <lane>] [--team <team>]"); } LIT.with(|lit| lit.clone()) };
            let v60: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v59 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v60
        }
    }
}
fn closure5() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        claims_run_57()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_claim_run() -> i32 {
    closure0()()
}
pub fn rig_agile_claims_run() -> i32 {
    closure5()()
}
use agile_claim_store::SpiralShape_97df63ebabc3 as Mut0;
use agile_claim_request::SpiralShape_2617b4e49837 as Mut2;
pub type SpiralShape_652accfc6a8d = Mut1;
