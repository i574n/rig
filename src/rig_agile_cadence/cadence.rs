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
enum US1 {
    US1_Some(i64),
    US1_None,
}
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_1(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_2();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
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
fn text_after_5(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let v4: Rc<str> = v4.clone();
                        let v5: Rc<UH0> = v5.clone();
                        let mut v6: bool = &*v2 == &*v1;
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
fn method6(mut v0: Option<i64>) -> Option<i64> {
    v0
}
fn closure1() -> Rc<dyn Fn((i64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i64)) -> US1> = Rc::new(move |mut v0: (i64)| -> US1 {
        let mut v1: i64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn number_after_4(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i64) -> i64 {
    let v3: Rc<str> = text_after_5(v0, v1);
    let v4: Option<i64> = (v3).trim().parse::<i64>().ok();
    let v5: Option<i64> = method6(v4);
    let mut v6: Rc<dyn Fn((i64)) -> US1> = closure1();
    let mut v7: Option<US1> = v5.map(|x| v6(x));
    let v9: US1 = US1::US1_None;
    let v10: US1 = v9.clone();
    let v11: US1 = v7.unwrap_or(v10);
    match v11 {
        US1::US1_None => {
            v2
        }
        US1::US1_Some(v12) => {
            let v12: i64 = v12;
            v12
        }
    }
}
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
fn method9(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
fn day_of_date_7(mut v0: Rc<str>) -> US1 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method8(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0, v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method9(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let v9: Rc<str> = string_concat(&[&*v8, "T00:00Z"]);
    let v10: i64 = agile_quality_state::rig_agile_stamp_hour(&v9);
    let v11: bool = v10 > 0i64;
    if v11 {
        let v12: i64 = v10.wrapping_div(24i64);
        US1::US1_Some(v12)
    } else {
        US1::US1_None
    }
}
fn method11(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
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
fn listed_10(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<UH0> {
    let mut v1: Rc<Vec<Rc<str>>> = Rc::new((v0).borrow().clone());
    let mut v2: usize = (v1).len();
    let v3: i32 = ((v2) as i32);
    let v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method11(v1, v4, v5)
}
fn method12(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method14(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_13(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method14(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method16(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig agile cadence done "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            if v6 {
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
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method17(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig agile cadence silent "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            if v6 {
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
fn format_real_18(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method14(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn dating_text_19(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = StatusChanged (\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.contains(&*v1);
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (done dated by status change, else last note)"); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (done dated by last note)"); } LIT.with(|lit| lit.clone()) };
        v4
    }
}
fn summary_of_15(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i64, mut v3: Rc<str>) -> Rc<str> {
    let v4: i32 = 0i32;
    let v5: i32 = method12(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method16(v0.clone(), v6);
    let v8: i32 = 0i32;
    let v9: i32 = method12(v7, v8);
    let v10: Rc<str> = format_real_13(v9);
    let v13: Rc<str> = format_real_13(v5);
    let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH0> = method17(v0, v16);
    let v18: i32 = 0i32;
    let v19: i32 = method12(v17, v18);
    let v20: Rc<str> = format_real_13(v19);
    let v23: Rc<str> = format_real_18(v2);
    let v25: Rc<str> = dating_text_19(v3);
    let v26: Rc<str> = string_concat(&["rig agile cadence ok done_rows=", &*v10, " activity_rows=", &*v13, " silent_active=", &*v20, " window_hours=", &*v23, &*v25]);
    v26
}
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method20(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method21(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<UH0> = v2;
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
fn cadence_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_1(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                            let v9: bool = v6.starts_with(&*v8);
                            let v10: bool = v9 == false;
                            if v10 {
                                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                                let v13: Rc<str> = Rc::<str>::from(v6.replace(&*v11, &*v12));
                                let v14: Rc<str> = string_concat(&[&*v13, "/state"]);
                                let v15: Rc<str> = string_concat(&[&*v14, "/agile.spi"]);
                                let v16: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v15).unwrap_or_default().as_str());
                                let v17: Rc<str> = Rc::<str>::from(v6.replace(&*v11, &*v12));
                                let v18: Rc<str> = string_concat(&[&*v17, "/history/status_changes.spi"]);
                                let v19: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v18).unwrap_or_default().as_str());
                                let v20: Rc<str> = string_concat(&[&*v14, "/asks.spi"]);
                                let v21: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v20).unwrap_or_default().as_str());
                                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ask-days"); } LIT.with(|lit| lit.clone()) };
                                let v23: i64 = 3i64;
                                let v24: i64 = number_after_4(v7.clone(), v22, v23);
                                let v25: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                                let v26: i64 = (v25 as i64);
                                let v27: i64 = v26.wrapping_div(3600000i64);
                                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--hours"); } LIT.with(|lit| lit.clone()) };
                                let v29: i64 = 48i64;
                                let v30: i64 = number_after_4(v7.clone(), v28, v29);
                                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--since"); } LIT.with(|lit| lit.clone()) };
                                let v32: Rc<str> = text_after_5(v7.clone(), v31);
                                let v33: US1 = day_of_date_7(v32);
                                let v41: i64 = match v33 {
                                    US1::US1_None => {
                                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--days"); } LIT.with(|lit| lit.clone()) };
                                        let v38: i64 = 7i64;
                                        number_after_4(v7.clone(), v37, v38)
                                    }
                                    US1::US1_Some(v34) => {
                                        let v34: i64 = v34;
                                        let v35: i64 = v27.wrapping_div(24i64);
                                        let v36: i64 = v35.wrapping_sub(v34);
                                        v36
                                    }
                                };
                                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--round"); } LIT.with(|lit| lit.clone()) };
                                let v43: i64 = 0i64;
                                let v44: i64 = number_after_4(v7.clone(), v42, v43);
                                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team"); } LIT.with(|lit| lit.clone()) };
                                let v46: Rc<str> = text_after_5(v7, v45);
                                let v47: bool = v44 > 0i64;
                                let v68: Rc<UH0> = if v47 {
                                    let v48: Rc<RefCell<Vec<Rc<str>>>> = rig_cadence_activity::rig_cadence_round_report(&v16, v27, v44);
                                    listed_10(v48)
                                } else {
                                    let v50: Rc<RefCell<Vec<Rc<str>>>> = rig_cadence_done::rig_cadence_done_report(&v16, &v19, v27, v30, v41, &v46);
                                    let v51: Rc<UH0> = listed_10(v50);
                                    let v52: i64 = v27.wrapping_div(24i64);
                                    let v53: Rc<RefCell<Vec<Rc<str>>>> = rig_cadence_activity::rig_cadence_activity_report(&v16, v52, v41, &v46);
                                    let v54: Rc<UH0> = listed_10(v53);
                                    let v55: Rc<RefCell<Vec<Rc<str>>>> = rig_cadence_asks::rig_cadence_asks_report(&v16, &v21, v27, v24, &v46);
                                    let v56: Rc<UH0> = listed_10(v55);
                                    let v57: i32 = 0i32;
                                    let v58: i32 = method12(v56.clone(), v57);
                                    let v59: Rc<str> = format_real_13(v58);
                                    let v61: Rc<str> = summary_of_15(v51.clone(), v54.clone(), v30, v19);
                                    let v62: Rc<str> = string_concat(&[&*v61, " neglected_asks=", &*v59]);
                                    let v63: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v64: Rc<UH0> = Rc::new(UH0::UH0_Cons(v62, v63));
                                    let v65: Rc<UH0> = method20(v56, v64);
                                    let v66: Rc<UH0> = method20(v54, v65);
                                    method20(v51, v66)
                                };
                                method21(v68);
                                0i32
                            } else {
                                let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile cadence <root> [--hours n] [--days n | --since yyyy-mm-dd] [--team t] [--round h] [--ask-days n]"); } LIT.with(|lit| lit.clone()) };
                                eprintln!("{}", v69);
                                2i32
                            }
                        }
                        _ => {
                            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile cadence <root> [--hours n] [--days n | --since yyyy-mm-dd] [--team t] [--round h] [--ask-days n]"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v71);
                            2i32
                        }
                    }
                }
                _ => {
                    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile cadence <root> [--hours n] [--days n | --since yyyy-mm-dd] [--team t] [--round h] [--ask-days n]"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v73);
                    2i32
                }
            }
        }
        _ => {
            let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile cadence <root> [--hours n] [--days n | --since yyyy-mm-dd] [--team t] [--round h] [--ask-days n]"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v75);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        cadence_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_cadence_run() -> i32 {
    closure0()()
}
use rig_cadence_activity::SpiralShape_97df63ebabc3 as Mut0;
