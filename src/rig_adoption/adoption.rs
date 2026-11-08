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
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_ByDay,
    US1_ByHabit,
}
#[derive(Clone)]
enum US2 {
    US2_ClaudeCode,
    US2_Codex,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoDays,
    UH0_DayThen(Rc<str>, u64, u64, u64, Rc<UH0>),
}
#[derive(Clone)]
enum US3 {
    US3_KitCall,
    US3_HabitCall,
    US3_OtherCall,
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US5 {
    US5_SwitchHabits,
}
#[derive(Clone)]
enum US4 {
    US4_Some(US5),
    US4_None,
}
#[derive(Clone)]
enum US7 {
    US7_ValuedDaysBack,
    US7_ValuedDays,
    US7_ValuedOut,
}
#[derive(Clone)]
enum US6 {
    US6_Some(US7),
    US6_None,
}
#[derive(Clone)]
enum US8 {
    US8_Some(Rc<str>),
    US8_None,
}
fn method3(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn days_back_of_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: &Rc<str>, mut v3: bool, mut v4: &Rc<str>) -> u64 {
    let v5: bool = &*v0 != "";
    let v6: Rc<str> = if v5 {
        v0
    } else {
        v1
    };
    let v7: Option<u64> = (v6).trim().parse::<u64>().ok();
    let v8: Option<u64> = method3(v7);
    let mut v9: Rc<dyn Fn((u64)) -> US0> = closure1();
    let mut v10: Option<US0> = v8.map(|x| v9(x));
    let v12: US0 = US0::US0_None;
    let v13: US0 = v12.clone();
    let v14: US0 = v10.unwrap_or(v13);
    let v17: u64 = match v14 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v15) => {
            let v15: u64 = v15;
            v15
        }
    };
    let v18: bool = v17 == 0u64;
    if v18 {
        7u64
    } else {
        v17
    }
}
fn session_files_4(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = { let since = std::time::SystemTime::now() - std::time::Duration::from_secs(v1 * 86400); let mut found = Vec::new(); let mut stack = vec![std::path::PathBuf::from(&*v0)]; while let Some(dir) = stack.pop() { let Ok(entries) = std::fs::read_dir(&dir) else { continue }; for entry in entries.flatten() { let path = entry.path(); if path.is_dir() { stack.push(path); } else if path.extension().map(|extension| extension == "jsonl").unwrap_or(false) && entry.metadata().and_then(|meta| meta.modified()).map(|modified| modified >= since).unwrap_or(false) { found.push(path.to_string_lossy().into_owned()); } } } found.sort(); std::rc::Rc::<str>::from(found.join("
").as_str()) };
    v2
}
fn cutoff_day_5(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0).saturating_sub(v0 * 86400) as i64; let z = secs.div_euclid(86400) + 719468; let era = z.div_euclid(146097); let doe = z - era * 146097; let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); let mp = (5 * doy + 2) / 153; let day = doy - (153 * mp + 2) / 5 + 1; let month = if mp < 10 { mp + 3 } else { mp - 9 }; let year = yoe + era * 400 + if month <= 2 { 1 } else { 0 }; std::rc::Rc::<str>::from(format!("{year:04}-{month:02}-{day:02}").as_str()) };
    v1
}
fn file_calls_7(mut v0: Rc<str>, mut v1: US2, mut v2: Rc<str>) -> Rc<str> {
    let v6: Rc<str> = match &v1 {
        US2::US2_ClaudeCode => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tool_use"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US2::US2_Codex => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("function_call"); } LIT.with(|lit| lit.clone()) };
            v4
        }
    };
    let mut v10: Rc<str> = match v1 {
        US2::US2_ClaudeCode => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\x22type\\x22:\\x22tool_use\\x22,\\x22id\\x22:\\x22[^\\x22]*\\x22,\\x22name\\x22:\\x22(?:PowerShell|Bash)\\x22,\\x22input\\x22:\\{\\x22command\\x22:\\x22((?:[^\\x22\\\\]|\\\\.)*)\\x22"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US2::US2_Codex => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\x22type\\x22:\\x22function_call\\x22,\\x22name\\x22:\\x22(?:shell|exec_command|local_shell)\\x22,\\x22arguments\\x22:\\x22((?:[^\\x22\\\\]|\\\\.)*)\\x22"); } LIT.with(|lit| lit.clone()) };
            v8
        }
    };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\x22timestamp\\x22:\\x22(\\d{4}-\\d\\d-\\d\\d)"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { let quote = char::from(34u8); let backslash = char::from(92u8); let stamp = regex::Regex::new(&*v11).unwrap(); let command = regex::Regex::new(&*v10).unwrap(); let text = std::fs::read_to_string(&*v0).unwrap_or_default(); let mut out = String::new(); for line in text.lines().filter(|line| line.contains(&*v6)) { let Some(day) = stamp.captures(line).map(|found| found[1].to_owned()) else { continue }; if day.as_str() < &*v2 { continue; } for found in command.captures_iter(line) { let decoded = found[1].replace(&format!("{backslash}{quote}"), &quote.to_string()).replace(&format!("{backslash}{backslash}"), &backslash.to_string()).replace(char::from(10u8), " ").replace(char::from(9u8), " "); out.push_str(&format!("{day}	{decoded}
")); } } std::rc::Rc::<str>::from(out.as_str()) };
    v12
}
fn tally_added_10(mut v0: Rc<str>, mut v1: US3, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_DayThen(v14, v15, v16, v17, v18) => {
            let v14: Rc<str> = v14;
            let v15: u64 = v15;
            let v16: u64 = v16;
            let v17: u64 = v17;
            let v18: Rc<UH0> = v18;
            let v19: bool = &*v14 == &*v0;
            if v19 {
                let (mut v31, mut v32, mut v33): (u64, u64, u64) = match &v1 {
                    US3::US3_KitCall => {
                        let v20: u64 = v15.wrapping_add(1u64);
                        let v21: u64 = v16.wrapping_add(1u64);
                        (v20, v21, v17)
                    }
                    _ => {
                        let (mut v28, mut v29, mut v30): (u64, u64, u64) = match &v1 {
                            US3::US3_HabitCall => {
                                let v22: u64 = v15.wrapping_add(1u64);
                                let v23: u64 = v17.wrapping_add(1u64);
                                (v22, v16, v23)
                            }
                            _ => {
                                let (mut v25, mut v26, mut v27): (u64, u64, u64) = match v1 {
                                    US3::US3_OtherCall => {
                                        let v24: u64 = v15.wrapping_add(1u64);
                                        (v24, v16, v17)
                                    }
                                    _ => unreachable!(),
                                };
                                (v25, v26, v27)
                            }
                        };
                        (v28, v29, v30)
                    }
                };
                Rc::new(UH0::UH0_DayThen(v14, v31, v32, v33, v18))
            } else {
                let v35: Rc<UH0> = tally_added_10(v0, v1, v18);
                Rc::new(UH0::UH0_DayThen(v14, v15, v16, v17, v35))
            }
        }
        UH0::UH0_NoDays => {
            let (mut v9, mut v10, mut v11): (u64, u64, u64) = match &v1 {
                US3::US3_KitCall => {
                    (1u64, 1u64, 0u64)
                }
                _ => {
                    let (mut v6, mut v7, mut v8): (u64, u64, u64) = match &v1 {
                        US3::US3_HabitCall => {
                            (1u64, 0u64, 1u64)
                        }
                        _ => {
                            let (mut v3, mut v4, mut v5): (u64, u64, u64) = match v1 {
                                US3::US3_OtherCall => {
                                    (1u64, 0u64, 0u64)
                                }
                                _ => unreachable!(),
                            };
                            (v3, v4, v5)
                        }
                    };
                    (v6, v7, v8)
                }
            };
            let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoDays); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_DayThen(v0, v9, v10, v11, v12))
        }
    }
}
fn words_added_11(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: u64 = (&*v0).len() as u64;
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let v5: u64 = (&*v0)[v1 as usize..].find(char::from(10u8)).map(|found| v1 + found as u64).unwrap_or((&*v0).len() as u64);
            let v6: Rc<str> = std::rc::Rc::<str>::from(&(&*v0)[v1 as usize..v5 as usize]);
            let v7: u64 = v5.wrapping_add(1u64);
            let v8: bool = &*v6 == "";
            let v11: Rc<UH0> = if v8 {
                v2
            } else {
                let mut v9: US3 = US3::US3_HabitCall;
                tally_added_10(v6, v9, v2)
            };
            (v1, v2) = (v7, v11);
            continue;
        }
    }
}
fn habit_any_12(mut v0: Rc<str>) -> bool {
    let v1: bool = rig_adoption_habits::rig_adoption_habit_any(&*v0);
    v1
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        habit_any_12(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn rig_named_14(mut v0: Rc<str>) -> bool {
    let v1: bool = { let text = &*v0; let bytes = text.as_bytes(); let separators = [32u8, 9, 13, 10, 38, 39, 34, 92, 47, 40, 59, 124]; text.match_indices("kit").any(|(at, found)| { let before = at == 0 || separators.contains(&bytes[at - 1]); let mut rest = &text[at + found.len()..]; if let Some(stripped) = rest.strip_prefix(".exe") { rest = stripped; } if let Some(stripped) = rest.strip_prefix(|c: char| c == char::from(39u8) || c == char::from(34u8)) { rest = stripped; } let trimmed = rest.trim_start(); before && trimmed.len() < rest.len() && trimmed.chars().next().is_some_and(|c| c.is_ascii_lowercase()) }) };
    v1
}
fn call_of_command_13(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<str>) -> US3 {
    let v2: bool = rig_named_14(v1.clone());
    if v2 {
        US3::US3_KitCall
    } else {
        let v4: bool = v0(v1);
        if v4 {
            US3::US3_HabitCall
        } else {
            US3::US3_OtherCall
        }
    }
}
fn habit_words_15(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = rig_adoption_habits::rig_adoption_habit_words(&*v0);
    v1
}
fn row_tallied_9(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match v0 {
        US1::US1_ByDay => {
            let v4: Rc<dyn Fn(Rc<str>) -> bool> = closure2();
            let v5: US3 = call_of_command_13(v4, v2);
            tally_added_10(v1, v5, v3)
        }
        US1::US1_ByHabit => {
            let v7: Rc<str> = habit_words_15(v2);
            let v8: u64 = 0u64;
            words_added_11(v7, v8, v3)
        }
    }
}
fn calls_tallied_8(mut v0: US1, mut v1: US2, mut v2: Rc<str>, mut v3: u64, mut v4: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v5: u64 = (&*v2).len() as u64;
        let v6: bool = v3 >= v5;
        if v6 {
            return v4;
        } else {
            let v7: u64 = (&*v2)[v3 as usize..].find(char::from(10u8)).map(|found| v3 + found as u64).unwrap_or((&*v2).len() as u64);
            let v8: Rc<str> = std::rc::Rc::<str>::from(&(&*v2)[v3 as usize..v7 as usize]);
            let v12: Rc<str> = match &v1 {
                US2::US2_ClaudeCode => {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude"); } LIT.with(|lit| lit.clone()) };
                    v9
                }
                US2::US2_Codex => {
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("codex"); } LIT.with(|lit| lit.clone()) };
                    v10
                }
            };
            let v14: Rc<str> = std::rc::Rc::<str>::from((&*v8).split(char::from(9u8)).nth(0i32 as usize).unwrap_or(""));
            let v15: Rc<str> = string_concat(&[&*v12, "\t", &*v14]);
            let v16: u64 = v7.wrapping_add(1u64);
            let v17: bool = &*v8 == "";
            let v20: Rc<UH0> = if v17 {
                v4
            } else {
                let v18: Rc<str> = std::rc::Rc::<str>::from((&*v8).split(char::from(9u8)).nth(1i32 as usize).unwrap_or(""));
                row_tallied_9(v0.clone(), v15, v18, v4)
            };
            (v3, v4) = (v16, v20);
            continue;
        }
    }
}
fn files_tallied_6(mut v0: US1, mut v1: US2, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v6: u64 = (&*v2).len() as u64;
        let v7: bool = v4 >= v6;
        if v7 {
            return v5;
        } else {
            let v8: u64 = (&*v2)[v4 as usize..].find(char::from(10u8)).map(|found| v4 + found as u64).unwrap_or((&*v2).len() as u64);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(&(&*v2)[v4 as usize..v8 as usize]);
            let mut v10: u64 = v8.wrapping_add(1u64);
            let v11: bool = &*v9 == "";
            let v15: Rc<UH0> = if v11 {
                v5
            } else {
                let v12: Rc<str> = file_calls_7(v9, v1.clone(), v3.clone());
                let v13: u64 = 0u64;
                calls_tallied_8(v0.clone(), v1.clone(), v12, v13, v5)
            };
            (v4, v5) = (v10, v15);
            continue;
        }
    }
}
fn method18(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_17(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method18(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn rig_share_19(mut v0: u64, mut v1: u64, mut v2: u64) -> f64 {
    let v3: bool = v0 == 0u64;
    if v3 {
        0.0f64
    } else {
        let v4: f64 = (v1 as f64) / (v0 as f64);
        v4
    }
}
fn rows_of_16(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: u64) -> (Rc<str>, u64, u64, u64) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_DayThen(v5, v6, v7, v8, v9) => {
                let v5: Rc<str> = v5;
                let v6: u64 = v6;
                let v7: u64 = v7;
                let v8: u64 = v8;
                let mut v9: Rc<UH0> = v9;
                let mut v10: bool = &*v1 == "";
                let v39: Rc<str> = if v10 {
                    let v12: Rc<str> = format_real_17(v6);
                    let v15: Rc<str> = format_real_17(v7);
                    let v17: Rc<str> = format_real_17(v8);
                    let v20: f64 = rig_share_19(v6, v7, v8);
                    let v21: Rc<str> = std::rc::Rc::<str>::from(format!("{:.3}", v20));
                    let v23: Rc<str> = string_concat(&[&*v5, "\t", &*v12, "\t", &*v15, "\t", &*v17, "\t", &*v21]);
                    v23
                } else {
                    let v26: Rc<str> = format_real_17(v6);
                    let v29: Rc<str> = format_real_17(v7);
                    let v31: Rc<str> = format_real_17(v8);
                    let v34: f64 = rig_share_19(v6, v7, v8);
                    let v35: Rc<str> = std::rc::Rc::<str>::from(format!("{:.3}", v34));
                    let v38: Rc<str> = string_concat(&[&*v1, "\n", &*v5, "\t", &*v26, "\t", &*v29, "\t", &*v31, "\t", &*v35]);
                    v38
                };
                let v40: u64 = v2.wrapping_add(v6);
                let v41: u64 = v3.wrapping_add(v7);
                let v42: u64 = v4.wrapping_add(v8);
                (v0, v1, v2, v3, v4) = (v9, v39, v40, v41, v42);
                continue;
            }
            UH0::UH0_NoDays => {
                return (v1, v2, v3, v4);
            }
        }
    }
}
fn max_habit_21(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64) -> (Rc<str>, u64) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_DayThen(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let v4: u64 = v4;
                let v5: u64 = v5;
                let v6: u64 = v6;
                let v7: Rc<UH0> = v7;
                let v8: bool = v6 > v2;
                if v8 {
                    (v0, v1, v2) = (v7, v3, v6);
                    continue;
                } else {
                    v0 = v7;
                    continue;
                }
            }
            UH0::UH0_NoDays => {
                return (v1, v2);
            }
        }
    }
}
fn without_key_22(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_DayThen(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3;
            let v4: u64 = v4;
            let v5: u64 = v5;
            let v6: u64 = v6;
            let v7: Rc<UH0> = v7;
            let v8: bool = &*v3 == &*v0;
            if v8 {
                v7
            } else {
                let mut v9: Rc<UH0> = without_key_22(v0, v7);
                Rc::new(UH0::UH0_DayThen(v3, v4, v5, v6, v9))
            }
        }
        UH0::UH0_NoDays => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoDays); } CASE.with(|case| case.clone()) }
        }
    }
}
fn habit_lines_20(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64) -> (Rc<str>, u64) {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v4: u64 = 0u64;
        let (mut v5, mut v6): (Rc<str>, u64) = max_habit_21(v0.clone(), v3, v4);
        let v7: bool = &*v5 == "";
        if v7 {
            return (v1, v2);
        } else {
            let v8: Rc<UH0> = without_key_22(v5.clone(), v0);
            let v11: Rc<str> = format_real_17(v6);
            let v13: Rc<str> = string_concat(&[&*v1, &*v5, "\t", &*v11, "\n"]);
            let v14: u64 = v2.wrapping_add(1u64);
            (v0, v1, v2) = (v8, v13, v14);
            continue;
        }
    }
}
fn adoption_reported_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: Rc<str>) -> i32 {
    let v5: u64 = days_back_of_2(v0, v1, &v2, v3, &v4);
    let v8: US1 = if v3 {
        US1::US1_ByHabit
    } else {
        US1::US1_ByDay
    };
    let mut v9: US2 = US2::US2_ClaudeCode;
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".claude/projects"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(std::path::PathBuf::from).unwrap_or_default().join(&*v10).to_string_lossy().as_ref());
    let v12: Rc<str> = session_files_4(v11, v5);
    let v13: Rc<str> = cutoff_day_5(v5);
    let v14: u64 = 0u64;
    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoDays); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH0> = files_tallied_6(v8.clone(), v9, v12, v13, v14, v15);
    let v17: US2 = US2::US2_Codex;
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".codex/sessions"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(std::path::PathBuf::from).unwrap_or_default().join(&*v18).to_string_lossy().as_ref());
    let v20: Rc<str> = session_files_4(v19, v5);
    let v21: Rc<str> = cutoff_day_5(v5);
    let v22: u64 = 0u64;
    let v23: Rc<UH0> = files_tallied_6(v8.clone(), v17, v20, v21, v22, v16);
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v25: u64 = 0u64;
    let v26: u64 = 0u64;
    let v27: u64 = 0u64;
    let (mut v28, mut v29, mut v30, mut v31): (Rc<str>, u64, u64, u64) = rows_of_16(v23.clone(), v24.clone(), v25, v26, v27);
    let v32: u64 = 0u64;
    let (mut v33, mut v34): (Rc<str>, u64) = habit_lines_20(v23, v24.clone(), v32);
    let v40: Rc<str> = match &v8 {
        US1::US1_ByDay => {
            let v35: Rc<str> = { let mut lines = (&*v28).lines().filter(|line| !line.is_empty()).collect::<Vec<_>>(); lines.sort(); std::rc::Rc::<str>::from(lines.join("
")) };
            let v37: Rc<str> = string_concat(&["client\tday\tshell_calls\tkit_calls\thabit_calls\tkit_share\n", &*v35, "\n"]);
            v37
        }
        US1::US1_ByHabit => {
            let v38: Rc<str> = string_concat(&["habit\tcalls\n", &*v33]);
            v38
        }
    };
    let v41: bool = &*v2 == "";
    let v43: Rc<str> = if v41 {
        v24.clone()
    } else {
        let v42: Rc<str> = std::rc::Rc::<str>::from(std::fs::write(&*v2, (&*v40).as_bytes()).err().map(|error| format!("write {}: {error}", &*v2)).unwrap_or_default().as_str());
        v42
    };
    let v44: bool = &*v43 != "";
    if v44 {
        let v45: Rc<str> = string_concat(&["kit error: adoption: ", &*v43]);
        eprintln!("{}", v45);
        2i32
    } else {
        if v41 {
            print!("{}", v40);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            ()
        };
        let v75: Rc<str> = match v8 {
            US1::US1_ByDay => {
                let v46: Rc<str> = format_real_17(v29);
                let v49: Rc<str> = format_real_17(v30);
                let v51: Rc<str> = format_real_17(v31);
                let v54: f64 = rig_share_19(v29, v30, v31);
                let v55: Rc<str> = std::rc::Rc::<str>::from(format!("{:.3}", v54));
                let v58: Rc<str> = format_real_17(v5);
                let v60: Rc<str> = string_concat(&["kit adoption ok days_back=", &*v58, " shell_calls=", &*v46, " rig_calls=", &*v49, " habit_calls=", &*v51, " rig_share=", &*v55]);
                let v62: Rc<str> = if v41 {
                    v24
                } else {
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
                    v61
                };
                let v64: Rc<str> = string_concat(&[&*v60, &*v62, &*v2]);
                v64
            }
            US1::US1_ByHabit => {
                let v65: Rc<str> = format_real_17(v5);
                let v68: Rc<str> = format_real_17(v34);
                let v69: Rc<str> = string_concat(&["kit adoption ok mode=habits days=", &*v65, " habits=", &*v68]);
                let v71: Rc<str> = if v41 {
                    v24
                } else {
                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
                    v70
                };
                let v73: Rc<str> = string_concat(&[&*v69, &*v71, &*v2]);
                v73
            }
        };
        println!("{}", v75);
        0i32
    }
}
fn valued_applied_26(mut v0: US7, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>) {
    let (mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>) = match &v0 {
        US7::US7_ValuedDaysBack => {
            (v1, v3, v4, v5, v6)
        }
        _ => {
            let (mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>) = match &v0 {
                US7::US7_ValuedDays => {
                    (v2, v1, v4, v5, v6)
                }
                _ => {
                    let (mut v7, mut v8, mut v9, mut v10, mut v11): (Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>) = match v0 {
                        US7::US7_ValuedOut => {
                            (v2, v3, v1, v5, v6)
                        }
                        _ => unreachable!(),
                    };
                    (v7, v8, v9, v10, v11)
                }
            };
            (v12, v13, v14, v15, v16)
        }
    };
    (v17, v18, v19, v20, v21)
}
fn words_read_25(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>) {
    loop {
        match &*v0 {
            UH1::UH1_Nil => {
                return (v1, v2, v3, v4, v5);
            }
            _ => {
                match Rc::unwrap_or_clone(v0) {
                    UH1::UH1_Cons(v6, v7) => {
                        let v6: Rc<str> = v6;
                        let v7: Rc<UH1> = v7;
                        let v8: bool = &*v6 == "--habits";
                        let v12: US4 = if v8 {
                            let mut v9: US5 = US5::US5_SwitchHabits;
                            US4::US4_Some(v9)
                        } else {
                            US4::US4_None
                        };
                        let v13: bool = &*v6 == "--days-back";
                        let v25: US6 = if v13 {
                            let v14: US7 = US7::US7_ValuedDaysBack;
                            US6::US6_Some(v14)
                        } else {
                            let v16: bool = &*v6 == "--days";
                            if v16 {
                                let v17: US7 = US7::US7_ValuedDays;
                                US6::US6_Some(v17)
                            } else {
                                let v19: bool = &*v6 == "--out";
                                if v19 {
                                    let v20: US7 = US7::US7_ValuedOut;
                                    US6::US6_Some(v20)
                                } else {
                                    US6::US6_None
                                }
                            }
                        };
                        match v12 {
                            US4::US4_Some(v26) => {
                                let v26: US5 = v26;
                                let v27: bool = true;
                                (v0, v4) = (v7, v27);
                                continue;
                            }
                            _ => {
                                match v25 {
                                    US6::US6_Some(v33) => {
                                        let v33: US7 = v33;
                                        match &*v7 {
                                            UH1::UH1_Nil => {
                                                let v35: Rc<str> = string_concat(&["option ", &*v6, " has no value"]);
                                                return (v1, v2, v3, v4, v35);
                                            }
                                            _ => {
                                                match Rc::unwrap_or_clone(v7) {
                                                    UH1::UH1_Cons(v36, v37) => {
                                                        let v36: Rc<str> = v36;
                                                        let v37: Rc<UH1> = v37;
                                                        let (mut v38, mut v39, mut v40, mut v41, mut v42): (Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>) = valued_applied_26(v33, v36, v1, v2, v3, v4, v5);
                                                        (v0, v1, v2, v3, v4, v5) = (v37, v38, v39, v40, v41, v42);
                                                        continue;
                                                    }
                                                    _ => unreachable!(),
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        let v58: Rc<str> = string_concat(&["unknown option ", &*v6]);
                                        return (v1, v2, v3, v4, v58);
                                    }
                                }
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
fn args_of_24(mut v0: Rc<UH1>) -> (Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: bool = false;
    let (mut v3, mut v4, mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>) = words_read_25(v0, v1.clone(), v1.clone(), v1.clone(), v2, v1);
    (v3, v4, v5, v6, v7)
}
fn command_line_argument_count_28() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_29(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_27(mut v0: i32) -> Rc<UH1> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_28();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US8 = if v4 {
        let v5: Rc<str> = command_line_argument_29(v0);
        US8::US8_Some(v5)
    } else {
        US8::US8_None
    };
    match v8 {
        US8::US8_None => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
        }
        US8::US8_Some(v10) => {
            let mut v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH1> = arguments_from_27(v11);
            Rc::new(UH1::UH1_Cons(v10, v12))
        }
    }
}
fn dropped_30(mut v0: i32, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: bool = v0 > 0i32;
                if v4 {
                    let v5: i32 = v0.wrapping_sub(1i32);
                    (v0, v1) = (v5, v3);
                    continue;
                } else {
                    return v1;
                }
            }
            _ => {
                return v1;
            }
        }
    }
}
fn adoption_args_read_23() -> (Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>) {
    let mut v0: i32 = 1i32;
    let v1: i32 = 0i32;
    let v2: Rc<UH1> = arguments_from_27(v1);
    let v3: Rc<UH1> = dropped_30(v0, v2);
    let (mut v4, mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>) = args_of_24(v3);
    (v4, v5, v6, v7, v8)
}
fn adoption_run_0() -> i32 {
    let (mut v0, mut v1, mut v2, mut v3, mut v4): (Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>) = adoption_args_read_23();
    let v5: bool = &*v4 != "";
    if v5 {
        let v6: Rc<str> = string_concat(&["kit error: adoption: ", &*v4]);
        eprintln!("{}", v6);
        2i32
    } else {
        adoption_reported_1(v0, v1, v2, v3, v4)
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        adoption_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_adoption_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
