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
    US0_InstalledTrain(Rc<str>, Rc<str>, i64),
    US0_GateVerdict(Rc<str>, Rc<str>, i64),
    US0_BacklogCounts(Rc<str>, Rc<str>),
    US0_FactMissing(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US0, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_ScanPlain,
    US1_ScanQuoted,
    US1_ScanEscaped,
}
pub struct Mut1 { pub l0: Rc<str> }
fn get_environment_variable_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn release_home_1(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_RELEASE_HOME"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = get_environment_variable_2(v1);
    let v3: bool = &*v2 != "";
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = Rc::<str>::from(v2.replace(&*v4, &*v5));
        v6
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v8));
        let v10: Rc<str> = string_concat(&[&*v9, "/.cache/release-train"]);
        v10
    }
}
fn last_write_ms_4(mut v0: Rc<str>) -> u64 {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v0);
    let mut v2: bool = v1.is_ok();
    let v3: bool = v2 == false;
    if v3 {
        0u64
    } else {
        let mut v4: std::fs::Metadata = v1.unwrap();
        let mut v5: std::io::Result<std::time::SystemTime> = v4.modified();
        let v6: bool = v5.is_ok();
        if v6 {
            let mut v7: std::time::SystemTime = v5.unwrap();
            let mut v8: std::time::Duration = v7.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
            let mut v9: u64 = v8.as_secs();
            let v10: u64 = v9.wrapping_mul(1000u64);
            let v11: u32 = v8.subsec_millis();
            let v12: u64 = (v11 as u64);
            let v13: u64 = v10.wrapping_add(v12);
            v13
        } else {
            0u64
        }
    }
}
fn age_hours_3(mut v0: Rc<str>) -> i64 {
    let mut v1: u64 = last_write_ms_4(v0);
    let mut v2: i64 = (v1 as i64);
    let v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v4: i64 = (v3 as i64);
    let mut v5: bool = v2 <= 0i64;
    if v5 {
        -1i64
    } else {
        let v6: i64 = v4.wrapping_sub(v2);
        let mut v7: i64 = v6.wrapping_div(3600000i64);
        v7
    }
}
fn method6(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
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
fn method7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
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
fn train_of_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i64) -> US0 {
    let v3: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: i32 = 0i32;
    let v6: i32 = method6(v3.clone(), v4, v5);
    let mut v7: i32 = v4.wrapping_sub(1i32);
    let mut v8: Rc<str> = string_slice(&v3, v6 as i64, v7 as i64);
    let mut v9: i32 = (v8.clone().len() as i32);
    let v10: i32 = method7(v8.clone(), v9);
    let v11: Rc<str> = string_slice(&v8, 0i32 as i64, v10 as i64);
    let v12: bool = &*v11 == "";
    if v12 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("train"); } LIT.with(|lit| lit.clone()) };
        US0::US0_FactMissing(v13, v1)
    } else {
        US0::US0_InstalledTrain(v11, v1, v2)
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method11(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method11(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
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
        method10(v16, v17)
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit crate gate "); } LIT.with(|lit| lit.clone()) };
                let mut v5: bool = v2.starts_with(&*v4);
                let v6: Rc<str> = if v5 {
                    v2
                } else {
                    v1
                };
                (v0, v1) = (v3, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn verdict_of_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i64) -> US0 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
    let mut v7: Rc<UH0> = split_list_9(v3, v6);
    let mut v8: Rc<str> = method12(v7, v5.clone());
    let mut v9: bool = &*v8 == "";
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gate"); } LIT.with(|lit| lit.clone()) };
        US0::US0_FactMissing(v10, v1)
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit crate gate "); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = Rc::<str>::from(v8.replace(&*v12, &*v5));
        US0::US0_GateVerdict(v13, v1, v2)
    }
}
fn field_end_18(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US1) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US1::US1_ScanPlain => {
                    let v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let mut v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let v10: i32 = v1.wrapping_add(1i32);
                let v25: US1 = match v3 {
                    US1::US1_ScanEscaped => {
                        US1::US1_ScanQuoted
                    }
                    US1::US1_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US1::US1_ScanQuoted
                        } else {
                            US1::US1_ScanPlain
                        }
                    }
                    US1::US1_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US1::US1_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US1::US1_ScanPlain
                            } else {
                                US1::US1_ScanQuoted
                            }
                        }
                    }
                };
                (v1, v3) = (v10, v25);
                continue;
            }
        }
    }
}
fn skip_spaces_19(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let mut v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_17(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US1 = US1::US1_ScanPlain;
            let v6: i32 = field_end_18(v0.clone(), v1, v2, v5);
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_19(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn count_status_16(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: i32) -> i32 {
    loop {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v6: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v5)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v7: bool = v6 < 0i32;
        if v7 {
            return v4;
        } else {
            let mut v8: i32 = v6.wrapping_add(6i32);
            let mut v9: i32 = v0.get(..v6 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v10: Rc<str> = std::rc::Rc::<str>::from(v0.get(v9 as usize..v6 as usize).unwrap_or(""));
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v12: bool = v10.starts_with(&*v11);
            let v19: bool = if v12 {
                let v13: i32 = 6i32;
                let v14: i32 = field_start_17(v0.clone(), v8, v2, v13);
                let v15: US1 = US1::US1_ScanPlain;
                let v16: i32 = field_end_18(v0.clone(), v14, v2, v15);
                let v17: Rc<str> = std::rc::Rc::<str>::from(v0.get(v14 as usize..v16 as usize).unwrap_or(""));
                let v18: bool = &*v17 == &*v3;
                v18
            } else {
                false
            };
            let v21: i32 = if v19 {
                let v20: i32 = v4.wrapping_add(1i32);
                v20
            } else {
                v4
            };
            (v1, v4) = (v8, v21);
            continue;
        }
    }
}
fn method21(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_20(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method21(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn status_count_text_15(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = std::rc::Rc::<str>::from(v2.to_ascii_lowercase());
    let v6: i32 = 0i32;
    let mut v7: i32 = 0i32;
    let mut v8: i32 = count_status_16(v0, v6, v1, v2, v7);
    let mut v9: Rc<str> = format_real_20(v8);
    let v10: Rc<str> = string_concat(&[" ", &*v3, "=", &*v9]);
    v10
}
fn status_counts_14(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = status_count_text_15(v0.clone(), v1, v2);
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = status_count_text_15(v0.clone(), v1, v6);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = status_count_text_15(v0.clone(), v1, v10);
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Paused"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = status_count_text_15(v0.clone(), v1, v14);
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = status_count_text_15(v0, v1, v18);
    let v21: Rc<str> = string_concat(&["", &*v3, "", &*v7, "", &*v11, "", &*v15, "", &*v19]);
    v21
}
fn census_text_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    status_counts_14(v0, v1)
}
fn format_real_25(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method21(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn age_text_24(mut v0: i64) -> Rc<str> {
    let mut v1: bool = v0 < 0i64;
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: Rc<str> = format_real_25(v0);
        let mut v4: Rc<str> = string_concat(&[" age_hours=", &*v3]);
        v4
    }
}
fn fact_line_23(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_BacklogCounts(v17, v18) => {
            let v17: Rc<str> = v17;
            let v18: Rc<str> = v18;
            let v21: Rc<str> = string_concat(&["kit status backlog", &*v17, " source=", &*v18]);
            v21
        }
        US0::US0_FactMissing(v22, v23) => {
            let v22: Rc<str> = v22;
            let v23: Rc<str> = v23;
            let v26: Rc<str> = string_concat(&["kit status missing ", &*v22, " source=", &*v23]);
            v26
        }
        US0::US0_GateVerdict(v9, v10, v11) => {
            let mut v9: Rc<str> = v9;
            let v10: Rc<str> = v10;
            let v11: i64 = v11;
            let v13: Rc<str> = age_text_24(v11);
            let v16: Rc<str> = string_concat(&["kit status gate ", &*v9, &*v13, " source=", &*v10]);
            v16
        }
        US0::US0_InstalledTrain(v1, v2, v3) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            let v3: i64 = v3;
            let mut v5: Rc<str> = age_text_24(v3);
            let mut v8: Rc<str> = string_concat(&["kit status train=", &*v1, &*v5, " source=", &*v2]);
            v8
        }
    }
}
fn method22(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: US0 = v2;
            let v3: Rc<UH1> = v3;
            let mut v4: Rc<UH0> = method22(v3, v1);
            let mut v5: Rc<str> = fact_line_23(v2);
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn fact_missing_28(mut v0: US0) -> bool {
    match v0 {
        US0::US0_FactMissing(v1, v2) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            true
        }
        _ => {
            false
        }
    }
}
fn method27(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: US0 = v2;
            let v3: Rc<UH1> = v3;
            let mut v4: Rc<UH1> = method27(v3, v1);
            let mut v5: bool = fact_missing_28(v2.clone());
            if v5 {
                Rc::new(UH1::UH1_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method29(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: US0 = v2;
                let v3: Rc<UH1> = v3;
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
fn receipt_line_26(mut v0: Rc<UH1>) -> Rc<str> {
    let mut v1: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH1> = method27(v0.clone(), v1);
    let v3: i32 = 0i32;
    let mut v4: i32 = method29(v2, v3);
    let mut v5: i32 = 0i32;
    let v6: i32 = method29(v0, v5);
    let mut v7: i32 = v6.wrapping_sub(v4);
    let mut v8: Rc<str> = format_real_20(v7);
    let v11: Rc<str> = format_real_20(v4);
    let v12: Rc<str> = string_concat(&["kit status ok facts=", &*v8, " missing=", &*v11]);
    v12
}
fn method30(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method30(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method31(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn facts_of_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v2: Rc<str> = release_home_1(v1);
    let v3: Rc<str> = string_concat(&[&*v2, "/last-installed.txt"]);
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v3).unwrap_or_default().as_str());
    let mut v5: i64 = age_hours_3(v3.clone());
    let v6: US0 = train_of_5(v4, v3, v5);
    let v19: US0 = match &v6 {
        US0::US0_InstalledTrain(v7, v8, v9) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<str> = v8.clone();
            let mut v9: i64 = *v9;
            let v12: Rc<str> = string_concat(&[&*v2, "/", &*v7, "/crate-gate.log"]);
            let v13: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v12).unwrap_or_default().as_str());
            let v14: i64 = age_hours_3(v12.clone());
            verdict_of_8(v13, v12, v14)
        }
        _ => {
            let v16: Rc<str> = string_concat(&[&*v2, "/<train>/crate-gate.log"]);
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gate"); } LIT.with(|lit| lit.clone()) };
            US0::US0_FactMissing(v17, v16)
        }
    };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = Rc::<str>::from(v0.replace(&*v20, &*v21));
    let v23: Rc<str> = string_concat(&[&*v22, "/state/agile.spi"]);
    let v24: bool = &*v0 == "";
    let v37: Rc<UH1> = if v24 {
        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v26: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v23).unwrap_or_default().as_str());
        let v27: bool = &*v26 == "";
        if v27 {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backlog"); } LIT.with(|lit| lit.clone()) };
            let v29: US0 = US0::US0_FactMissing(v28, v23);
            let v30: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_Cons(v29, v30))
        } else {
            let v32: Rc<str> = census_text_13(v26);
            let v33: US0 = US0::US0_BacklogCounts(v32, v23);
            let v34: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_Cons(v33, v34))
        }
    };
    let v38: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v39: Rc<UH0> = method22(v37.clone(), v38);
    let v40: Rc<str> = fact_line_23(v19.clone());
    let v41: Rc<str> = fact_line_23(v6.clone());
    let v42: Rc<UH1> = Rc::new(UH1::UH1_Cons(v19, v37));
    let v43: Rc<UH1> = Rc::new(UH1::UH1_Cons(v6, v42));
    let v44: Rc<str> = receipt_line_26(v43);
    let v45: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v46: Rc<UH0> = Rc::new(UH0::UH0_Cons(v44, v45));
    let v47: Rc<UH0> = method30(v39, v46);
    let mut v48: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v48.borrow_mut().push(v41);
    v48.borrow_mut().push(v40);
    let v49: i32 = 2i32;
    let v50: i32 = method31(v48.clone(), v47, v49);
    let v51: Rc<Vec<Rc<str>>> = Rc::new(v48.borrow().clone());
    let v52: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v51).as_ref().clone()));
    v52
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<RefCell<Vec<Rc<str>>>> {
        facts_of_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method33(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let mut v5: Rc<str> = (v0)[v4].clone();
            let v6: i32 = v1.wrapping_sub(1i32);
            let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method34(mut v0: Rc<UH0>) -> () {
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
fn status_printed_32(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = facts_of_0(v0, v1);
    let v3: Rc<Vec<Rc<str>>> = Rc::new((v2).borrow().clone());
    let mut v4: usize = (v3).len();
    let mut v5: i32 = ((v4) as i32);
    let v6: i32 = v5.wrapping_sub(1i32);
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = method33(v3, v6, v7);
    method34(v8);
    0i32
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        status_printed_32(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_status_facts(v0: &str, v1: &str) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_status_printed(v0: &str, v1: &str) -> i32 {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
