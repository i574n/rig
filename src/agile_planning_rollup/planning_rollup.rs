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
    US0_RollupText,
    US0_RollupJson,
}
#[derive(Clone)]
enum US1 {
    US1_ScanPlain,
    US1_ScanQuoted,
    US1_ScanEscaped,
}
#[derive(Clone)]
enum US2 {
    US2_FileReadable(Rc<str>),
    US2_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_StateFile(Rc<str>),
    US3_HistoryFile(Rc<str>),
    US3_ArchiveFile(Rc<str>),
    US3_OtherFile,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US5 {
    US5_HeadInl,
    US5_HeadUnion,
    US5_HeadOpen,
    US5_HeadType,
}
#[derive(Clone)]
enum US4 {
    US4_KeywordHead(US5),
    US4_SymbolHead(US5),
    US4_OtherLine,
}
#[derive(Clone)]
enum US6 {
    US6_Valid(US0),
    US6_Invalid,
}
fn tally_of_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (i32, i32, i32, i32, i32) {
    let (mut v3, mut v4, mut v5, mut v6, mut v7): (i32, i32, i32, i32, i32) = agile_planning_tally::rig_agile_planning_tally(&*v0, &*v1, &*v2);
    (v3, v4, v5, v6, v7)
}
fn tally_json_6(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tasks"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open_tasks"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimated"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("minutes"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open_minutes"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let v33: Rc<str> = string_concat(&[",\"", &*v5, "\":", &*v8, ",\"", &*v10, "\":", &*v13, ",\"", &*v16, "\":", &*v19, ",\"", &*v22, "\":", &*v25, ",\"", &*v28, "\":", &*v31]);
    v33
}
fn tally_columns_7(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> Rc<str> {
    let v5: Rc<str> = { let text = (v0).to_string(); let filler = (5i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let v6: Rc<str> = { let text = (v1).to_string(); let filler = (7i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let v8: Rc<str> = { let text = (v2).to_string(); let filler = (11i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let v10: Rc<str> = { let text = (v3).to_string(); let filler = (9i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let v12: Rc<str> = { let text = (v4).to_string(); let filler = (14i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let v13: Rc<str> = string_concat(&[&*v5, &*v6, &*v8, &*v10, &*v12]);
    v13
}
fn rollup_line_5(mut v0: US0, mut v1: &Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: bool) -> Rc<str> {
    match v0 {
        US0::US0_RollupJson => {
            let v17: Rc<str> = if v9 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v15
            } else {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                v16
            };
            let mut v23: Rc<str> = tally_json_6(v4, v5, v6, v7, v8);
            let v25: Rc<str> = string_concat(&[&*v17, "{\"group\":\"", &*v2, "\",\"value\":\"", &*v3, "\"", &*v23, "}"]);
            v25
        }
        US0::US0_RollupText => {
            let v10: Rc<str> = { let text = v2; let filler = (11i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let v11: Rc<str> = { let text = v3; let filler = (28i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let v13: Rc<str> = tally_columns_7(v4, v5, v6, v7, v8);
            let v14: Rc<str> = string_concat(&[&*v10, &*v11, &*v13]);
            v14
        }
    }
}
fn print_group_rows_3(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: i32, mut v6: bool) -> i32 {
    loop {
        let v7: i32 = (v4.clone().len() as i32);
        let v8: bool = v7 <= v5;
        if v8 {
            return 0i32;
        } else {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v10: i32 = v4.get(v5 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v5 as usize) as i32).unwrap_or(-1);
            let v11: bool = v10 < 0i32;
            let v12: i32 = if v11 {
                v7
            } else {
                v10
            };
            let v13: Rc<str> = std::rc::Rc::<str>::from(v4.get(v5 as usize..v12 as usize).unwrap_or(""));
            let (mut v14, mut v15, mut v16, mut v17, mut v18): (i32, i32, i32, i32, i32) = tally_of_4(v0.clone(), v2.clone(), v13.clone());
            let mut v19: Rc<str> = rollup_line_5(v1.clone(), &v2, v3.clone(), v13, v14, v15, v16, v17, v18, v6);
            let v20: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v19.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            let v21: i32 = v12.wrapping_add(1i32);
            let mut v22: bool = false;
            (v5, v6) = (v21, v22);
            continue;
        }
    }
}
fn field_end_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US1) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US1::US1_ScanPlain => {
                    let v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let v7: bool = v5 == 41i32;
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
                        let mut v19: bool = v5 == 34i32;
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
fn skip_spaces_12(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US1 = US1::US1_ScanPlain;
            let v6: i32 = field_end_11(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_12(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn epic_ids_from_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v4)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v6: bool = v5 < 0i32;
        if v6 {
            return v3;
        } else {
            let v7: i32 = v5.wrapping_add(6i32);
            let v8: i32 = v0.get(..v5 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v5 as usize).unwrap_or(""));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v11: bool = v9.starts_with(&*v10);
            if v11 {
                let v12: i32 = 2i32;
                let v13: i32 = field_start_10(v0.clone(), v7, v2, v12);
                let v14: US1 = US1::US1_ScanPlain;
                let v15: i32 = field_end_11(v0.clone(), v13, v2, v14);
                let v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v13 as usize..v15 as usize).unwrap_or(""));
                let v17: bool = &*v16 == "Epic";
                if v17 {
                    let v18: i32 = 0i32;
                    let mut v19: i32 = field_start_10(v0.clone(), v7, v2, v18);
                    let v20: US1 = US1::US1_ScanPlain;
                    let v21: i32 = field_end_11(v0.clone(), v19, v2, v20);
                    let mut v22: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v21 as usize).unwrap_or(""));
                    let mut v23: i32 = (v22.clone().len() as i32);
                    let v24: i32 = v23.wrapping_sub(1i32);
                    let v25: Rc<str> = std::rc::Rc::<str>::from(v22.get(1i32 as usize..v24 as usize).unwrap_or(""));
                    let v26: bool = &*v3 == "";
                    let v29: Rc<str> = if v26 {
                        v25
                    } else {
                        let v28: Rc<str> = string_concat(&[&*v3, ",", &*v25]);
                        v28
                    };
                    (v1, v3) = (v7, v29);
                    continue;
                } else {
                    v1 = v7;
                    continue;
                }
            } else {
                v1 = v7;
                continue;
            }
        }
    }
}
fn planning_epic_ids_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 0i32;
    let v2: i32 = (v0.clone().len() as i32);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    epic_ids_from_9(v0, v1, v2, v3)
}
fn print_rollup_2(mut v0: Rc<str>, mut v1: US0, mut v2: u64) -> i32 {
    let v11: Rc<str> = match &v1 {
        US0::US0_RollupJson => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"groups\":["); } LIT.with(|lit| lit.clone()) };
            v9
        }
        US0::US0_RollupText => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("GROUP"); } LIT.with(|lit| lit.clone()) };
            let v4: Rc<str> = { let text = v3; let filler = (11i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("VALUE"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = { let text = v5; let filler = (28i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let v8: Rc<str> = string_concat(&[&*v4, &*v6, "TASKS   OPEN  ESTIMATED  MINUTES  OPEN_MINUTES"]);
            v8
        }
    };
    let v12: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v11.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned,Active,Blocked,Paused,Done"); } LIT.with(|lit| lit.clone()) };
    let v15: i32 = 0i32;
    let v16: bool = true;
    let v17: i32 = print_group_rows_3(v0.clone(), v1.clone(), v13.clone(), v13.clone(), v14, v15, v16);
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("light,medium,heavy,none"); } LIT.with(|lit| lit.clone()) };
    let v20: i32 = 0i32;
    let v21: bool = false;
    let mut v22: i32 = print_group_rows_3(v0.clone(), v1.clone(), v18.clone(), v18.clone(), v19, v20, v21);
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("assignee"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agent,pair,user,none"); } LIT.with(|lit| lit.clone()) };
    let v25: i32 = 0i32;
    let v26: bool = false;
    let v27: i32 = print_group_rows_3(v0.clone(), v1.clone(), v23.clone(), v23.clone(), v24, v25, v26);
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("epic"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("epic+deps"); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = planning_epic_ids_8(v0.clone());
    let v31: i32 = 0i32;
    let v32: bool = false;
    let v33: i32 = print_group_rows_3(v0.clone(), v1.clone(), v28, v29, v30, v31, v32);
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v36, mut v37, mut v38, mut v39, mut v40): (i32, i32, i32, i32, i32) = tally_of_4(v0, v34, v35);
    let v67: Rc<str> = match v1 {
        US0::US0_RollupJson => {
            let v59: Rc<str> = tally_json_6(v36, v37, v38, v39, v40);
            let v62: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
            let v63: Rc<str> = std::rc::Rc::<str>::from((v62).to_string());
            let v65: Rc<str> = string_concat(&["]", &*v59, ",\"elapsed_ms\":", &*v63, "}"]);
            v65
        }
        US0::US0_RollupText => {
            let v41: Rc<str> = std::rc::Rc::<str>::from((v36).to_string());
            let v44: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
            let v47: Rc<str> = std::rc::Rc::<str>::from((v38).to_string());
            let v49: Rc<str> = std::rc::Rc::<str>::from((v39).to_string());
            let v52: Rc<str> = std::rc::Rc::<str>::from((v40).to_string());
            let v56: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
            let v57: Rc<str> = std::rc::Rc::<str>::from((v56).to_string());
            let v58: Rc<str> = string_concat(&["rig agile plan ok tasks=", &*v41, " open=", &*v44, " estimated=", &*v47, " minutes=", &*v49, " open_minutes=", &*v52, " elapsed_ms=", &*v57]);
            v58
        }
    };
    let v68: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v67.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v68
}
fn concat5_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn loop_14(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
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
fn method15(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_16(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
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
fn loop_17(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
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
fn loop_18(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
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
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method21(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method21(v7, &v9) {
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
        method20(v16, v17)
    }
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method22(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let v15: US4 = if v6 {
                let v7: US5 = US5::US5_HeadType;
                US4::US4_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US5 = US5::US5_HeadType;
                    US4::US4_SymbolHead(v11)
                } else {
                    US4::US4_OtherLine
                }
            };
            let v27: US4 = match &v15 {
                US4::US4_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US5 = US5::US5_HeadInl;
                        US4::US4_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let mut v22: US5 = US5::US5_HeadInl;
                            US4::US4_SymbolHead(v22)
                        } else {
                            US4::US4_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US4 = match &v27 {
                US4::US4_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US5 = US5::US5_HeadUnion;
                        US4::US4_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US5 = US5::US5_HeadUnion;
                            US4::US4_SymbolHead(v34)
                        } else {
                            US4::US4_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US4 = match &v39 {
                US4::US4_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US5 = US5::US5_HeadOpen;
                        US4::US4_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US5 = US5::US5_HeadOpen;
                            US4::US4_SymbolHead(v46)
                        } else {
                            US4::US4_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US4::US4_SymbolHead(v52) => {
                    let v52: US5 = v52;
                    let v60: Rc<str> = match &v52 {
                        US5::US5_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US5::US5_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US5::US5_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US5::US5_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US5::US5_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US5::US5_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US5::US5_HeadType => {
                            v5
                        }
                        US5::US5_HeadUnion => {
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
fn method23(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn planning_rollup_1(mut v0: Rc<str>, mut v1: US0) -> i32 {
    let v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_13(v0, v3, v4.clone(), v4.clone(), v4.clone());
    let v6: bool = std::path::Path::new(&*v5).is_file();
    let mut v9: US2 = if v6 {
        US2::US2_FileReadable(v5)
    } else {
        US2::US2_FileMissing(v5)
    };
    match v9 {
        US2::US2_FileMissing(v10) => {
            let v10: Rc<str> = v10;
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = string_concat(&["agile plan rejected: ", &*v11]);
            let v13: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v12 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v13
        }
        US2::US2_FileReadable(v14) => {
            let v14: Rc<str> = v14;
            let v15: Rc<str> = std::fs::read_to_string(&*v14).unwrap_or_default().into();
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(v14.replace(&*v16, &*v17));
            let mut v19: Rc<str> = loop_14(v18);
            let v20: Option<usize> = v19.rfind(&*v17);
            let v21: Option<usize> = method15(v20);
            let mut v22: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v23: Option<i32> = v21.map(|x| v22(x));
            let v25: i32 = v23.unwrap_or(-1i32);
            let v26: bool = v25 < 0i32;
            let v30: bool = if v26 {
                true
            } else {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v28: bool = v19.ends_with(&*v27);
                let v29: bool = v28 == false;
                v29
            };
            let v51: US3 = if v30 {
                US3::US3_OtherFile
            } else {
                let v32: usize = ((0i32) as usize);
                let v33: usize = ((v25) as usize);
                let v34: Rc<str> = Rc::<str>::from(&(v19)[v32..v33]);
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v36: bool = v34.ends_with(&*v35);
                if v36 {
                    let v37: Rc<str> = loop_16(v34);
                    US3::US3_StateFile(v37)
                } else {
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v40: bool = v34.ends_with(&*v39);
                    if v40 {
                        let v41: Rc<str> = loop_17(v34);
                        US3::US3_HistoryFile(v41)
                    } else {
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v44: bool = v34.ends_with(&*v43);
                        if v44 {
                            let v45: Rc<str> = loop_18(v34);
                            US3::US3_ArchiveFile(v45)
                        } else {
                            US3::US3_OtherFile
                        }
                    }
                }
            };
            let v62: Rc<str> = match v51 {
                US3::US3_OtherFile => {
                    v15
                }
                _ => {
                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v53: Rc<UH0> = split_list_19(v52.clone(), v15);
                    let v54: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v55: Rc<UH0> = method22(v53, v54);
                    let v56: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v57: i32 = 0i32;
                    let v58: i32 = method23(v56.clone(), v55, v57);
                    let v59: Rc<Vec<Rc<str>>> = Rc::new(v56.borrow().clone());
                    let v60: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v59).as_ref().clone()));
                    let v61: Rc<str> = Rc::<str>::from((*v60.borrow()).join(&*v52));
                    v61
                }
            };
            let v63: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v62);
            print_rollup_2(v63, v1, v2)
        }
    }
}
fn command_line_argument_count_25() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_24() -> i32 {
    let mut v0: i32 = command_line_argument_count_25();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn planning_rollup_entry_0() -> i32 {
    let mut v0: i32 = argument_count_24();
    let mut v1: bool = v0 == 2i32;
    if v1 {
        let v2: i32 = 1i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v4: US0 = US0::US0_RollupText;
        planning_rollup_1(v3, v4)
    } else {
        let v6: bool = v0 == 3i32;
        if v6 {
            let v7: i32 = 2i32;
            let v8: Rc<str> = usize::try_from(v7).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v9: bool = &*v8 == "--json";
            let v13: US6 = if v9 {
                let v10: US0 = US0::US0_RollupJson;
                US6::US6_Valid(v10)
            } else {
                US6::US6_Invalid
            };
            match v13 {
                US6::US6_Invalid => {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan expects <root> [--json]"); } LIT.with(|lit| lit.clone()) };
                    let mut v19: Rc<str> = string_concat(&["agile plan rejected: ", &*v18]);
                    let v20: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v19 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v20
                }
                US6::US6_Valid(v14) => {
                    let v14: US0 = v14;
                    let v15: i32 = 1i32;
                    let v16: Rc<str> = usize::try_from(v15).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    planning_rollup_1(v16, v14)
                }
            }
        } else {
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan expects <root> [--json]"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = string_concat(&["agile plan rejected: ", &*v23]);
            let v25: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v24 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v25
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        planning_rollup_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_planning_rollup_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
