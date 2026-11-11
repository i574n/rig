#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;

pub fn spiral_json_quote(s: &str) -> std::rc::Rc<str> {
    let (q, b) = (char::from(34u8), char::from(92u8));
    let mut out = String::from(q);
    for c in s.chars() {
        match c as u32 {
            34 | 92 => { out.push(b); out.push(c); }
            n @ (8 | 9 | 10 | 12 | 13) => { out.push(b); out.push(char::from(b"btn_fr"[(n - 8) as usize])); }
            n if n < 32 => out.push_str(&format!("{b}u{n:04x}")),
            _ => out.push(c),
        }
    }
    out.push(q);
    std::rc::Rc::from(out.as_str())
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
    US0_AllNotes,
    US0_NewestNotes(i32),
    US0_NotesMatching(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_ScanPlain,
    US1_ScanQuoted,
    US1_ScanEscaped,
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_SpiprojProject,
    US3_TomlProject,
}
#[derive(Clone)]
enum US4 {
    US4_StateFile(Rc<str>),
    US4_HistoryFile(Rc<str>),
    US4_ArchiveFile(Rc<str>),
    US4_OtherFile,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US6 {
    US6_HeadInl,
    US6_HeadUnion,
    US6_HeadOpen,
    US6_HeadType,
}
#[derive(Clone)]
enum US5 {
    US5_KeywordHead(US6),
    US5_SymbolHead(US6),
    US5_OtherLine,
}
#[derive(Clone)]
enum US7 {
    US7_SplitProject(Rc<UH0>, Rc<UH0>, Rc<UH0>),
    US7_NoModules,
}
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn declared_from_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v6.starts_with(&*v7);
            if v8 {
                return v3;
            } else {
                let mut v9: i32 = v3.wrapping_add(1i32);
                v2 = v9;
                continue;
            }
        }
    }
}
fn declared_count_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_4(v0.clone(), v1.clone(), v2);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            let v7: i32 = v3.wrapping_add(1i32);
            (v2, v3) = (v6, v7);
            continue;
        }
    }
}
fn field_end_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US1) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US1::US1_ScanPlain => {
                    let mut v6: bool = v5 == 44i32;
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
                let mut v25: US1 = match v3 {
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
                            let mut v14: bool = v5 == 34i32;
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
fn skip_spaces_9(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
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
fn field_start_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US1 = US1::US1_ScanPlain;
            let mut v6: i32 = field_end_8(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_9(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn unescaped_from_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return concat5_2(v4, v6, v7.clone(), v7.clone(), v7.clone());
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v10: bool = v9 == 92i32;
            if v10 {
                let v11: i32 = v1.wrapping_add(1i32);
                let v12: i32 = v11.wrapping_add(1i32);
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v15: bool = v14 == 110i32;
                let v24: Rc<str> = if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16
                } else {
                    let mut v17: bool = v14 == 114i32;
                    if v17 {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18
                    } else {
                        let v19: bool = v14 == 116i32;
                        if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20
                        } else {
                            let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21
                        }
                    }
                };
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = concat5_2(v4, v13, v24, v25.clone(), v25.clone());
                (v1, v2, v4) = (v12, v12, v26);
                continue;
            } else {
                let mut v28: i32 = v1.wrapping_add(1i32);
                v1 = v28;
                continue;
            }
        }
    }
}
fn unquoted_10(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_11(v3, v4, v5, v6, v7)
}
fn note_at_6(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1.wrapping_add(6i32);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = 2i32;
    let mut v5: i32 = field_start_7(v0.clone(), v2, v3, v4);
    let mut v6: US1 = US1::US1_ScanPlain;
    let v7: i32 = field_end_8(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_10(v8);
    let v10: i32 = 3i32;
    let v11: i32 = field_start_7(v0.clone(), v2, v3, v10);
    let v12: US1 = US1::US1_ScanPlain;
    let mut v13: i32 = field_end_8(v0.clone(), v11, v3, v12);
    let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v13 as usize).unwrap_or(""));
    let v15: Rc<str> = unquoted_10(v14);
    let v16: i32 = 4i32;
    let mut v17: i32 = field_start_7(v0.clone(), v2, v3, v16);
    let mut v18: US1 = US1::US1_ScanPlain;
    let v19: i32 = field_end_8(v0.clone(), v17, v3, v18);
    let v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let v21: Rc<str> = unquoted_10(v20);
    (v9, v15, v21)
}
fn local_offset_minutes_13() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn local_stamp_12(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i64 = local_offset_minutes_13();
    let mut v2: Rc<str> = { let at = &*v0; let offset = v1; let parsed = (|| -> Option<i64> { let hours: i64 = at.get(11..13)?.parse().ok()?; let minutes: i64 = at.get(14..16)?.parse().ok()?; Some(hours * 60 + minutes) })(); std::rc::Rc::<str>::from(match parsed { Some(total) => { let local = (total + offset).rem_euclid(1440); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; format!(" ({:02}:{:02} {}{:02}:{:02})", local / 60, local % 60, sign, offset.abs() / 60, offset.abs() % 60) } None => String::new() }) };
    v2
}
fn kept_lines_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: US0, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: i32) -> (Rc<str>, i32) {
    loop {
        let v8: i32 = declared_from_4(v0.clone(), v1.clone(), v2);
        let mut v9: bool = v8 < 0i32;
        if v9 {
            return (v6, v7);
        } else {
            let (mut v10, mut v11, mut v12): (Rc<str>, Rc<str>, Rc<str>) = note_at_6(v0.clone(), v8);
            let mut v13: i32 = v4.wrapping_add(1i32);
            let mut v25: bool = match &v3 {
                US0::US0_AllNotes => {
                    true
                }
                US0::US0_NewestNotes(v14) => {
                    let mut v14: i32 = *v14;
                    let v15: i32 = v5.wrapping_sub(v14);
                    let v16: bool = v13 > v15;
                    v16
                }
                US0::US0_NotesMatching(v17) => {
                    let mut v17: Rc<str> = v17.clone();
                    let mut v18: Rc<str> = Rc::<str>::from(v12.to_lowercase());
                    let v19: bool = v18.contains(&*v17);
                    if v19 {
                        true
                    } else {
                        let v20: Rc<str> = Rc::<str>::from(v11.to_lowercase());
                        let v21: bool = v20.contains(&*v17);
                        v21
                    }
                }
            };
            if v25 {
                let v26: i32 = v8.wrapping_add(6i32);
                let v27: i32 = (v0.clone().len() as i32);
                let mut v28: i32 = 1i32;
                let mut v29: i32 = field_start_7(v0.clone(), v26, v27, v28);
                let v30: US1 = US1::US1_ScanPlain;
                let v31: i32 = field_end_8(v0.clone(), v29, v27, v30);
                let v32: Rc<str> = std::rc::Rc::<str>::from(v0.get(v29 as usize..v31 as usize).unwrap_or(""));
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = Rc::<str>::from(v32.replace(&*v33, &*v34));
                let v36: i32 = v8.wrapping_add(1i32);
                let v40: Rc<str> = local_stamp_12(v10.clone());
                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v42: Rc<str> = concat5_2(v41.clone(), v12, v34.clone(), v34.clone(), v34.clone());
                let v43: Rc<str> = concat5_2(v10, v40, v41, v11, v42);
                let v44: Rc<str> = string_concat(&[&*v6, "\n#", &*v35, " ", &*v43]);
                let v45: i32 = v7.wrapping_add(1i32);
                (v2, v4, v6, v7) = (v36, v13, v44, v45);
                continue;
            } else {
                let v48: i32 = v8.wrapping_add(1i32);
                (v2, v4) = (v48, v13);
                continue;
            }
        }
    }
}
fn notes_filtered_text_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US0) -> Rc<str> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = concat5_2(v3.clone(), v4.clone(), v1.clone(), v5.clone(), v6.clone());
    let v8: i32 = 0i32;
    let mut v9: i32 = 0i32;
    let v10: i32 = declared_count_3(v0.clone(), v7, v8, v9);
    let v11: bool = v10 == 0i32;
    if v11 {
        v6
    } else {
        let v12: Rc<str> = concat5_2(v3, v4, v1, v5, v6.clone());
        let mut v13: i32 = 0i32;
        let mut v14: i32 = 0i32;
        let v15: i32 = 0i32;
        let (mut v16, mut v17): (Rc<str>, i32) = kept_lines_5(v0, v12, v13, v2, v14, v10, v6, v15);
        let mut v18: Rc<str> = std::rc::Rc::<str>::from((v17).to_string());
        let v21: Rc<str> = std::rc::Rc::<str>::from((v10).to_string());
        let v24: Rc<str> = string_concat(&["notes (", &*v18, "/", &*v21, ")", &*v16]);
        v24
    }
}
fn method15(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure1() -> Rc<dyn Fn((i32)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US2> = Rc::new(move |mut v0: (i32)| -> US2 {
        let mut v1: i32 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_filter_of_14(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let mut v2: bool = &*v0 == "last";
    if v2 {
        let v3: Option<i32> = (v1).trim().parse::<i32>().ok();
        let v4: Option<i32> = method15(v3);
        let mut v5: Rc<dyn Fn((i32)) -> US2> = closure1();
        let mut v6: Option<US2> = v4.map(|x| v5(x));
        let v8: US2 = US2::US2_None;
        let mut v9: US2 = v8.clone();
        let v10: US2 = v6.unwrap_or(v9);
        match v10 {
            US2::US2_None => {
                US0::US0_AllNotes
            }
            US2::US2_Some(v11) => {
                let v11: i32 = v11;
                US0::US0_NewestNotes(v11)
            }
        }
    } else {
        let v16: bool = &*v0 == "grep";
        if v16 {
            let mut v17: Rc<str> = Rc::<str>::from(v1.to_lowercase());
            US0::US0_NotesMatching(v17)
        } else {
            US0::US0_AllNotes
        }
    }
}
fn last_note_of_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("last"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
    let v4: US0 = note_filter_of_14(v2, v3);
    notes_filtered_text_1(v1, v0, v4)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        last_note_of_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_18(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method19(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure3() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_20(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(6i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_21(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_22(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method25(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method25(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method24(v16, v17)
    }
}
fn method26(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method26(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US5 = if v6 {
                let v7: US6 = US6::US6_HeadType;
                US5::US5_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US6 = US6::US6_HeadType;
                    US5::US5_SymbolHead(v11)
                } else {
                    US5::US5_OtherLine
                }
            };
            let v27: US5 = match &v15 {
                US5::US5_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let mut v18: US6 = US6::US6_HeadInl;
                        US5::US5_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US6 = US6::US6_HeadInl;
                            US5::US5_SymbolHead(v22)
                        } else {
                            US5::US5_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US5 = match &v27 {
                US5::US5_OtherLine => {
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let mut v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US6 = US6::US6_HeadUnion;
                        US5::US5_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US6 = US6::US6_HeadUnion;
                            US5::US5_SymbolHead(v34)
                        } else {
                            US5::US5_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US5 = match &v39 {
                US5::US5_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US6 = US6::US6_HeadOpen;
                        US5::US5_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US6 = US6::US6_HeadOpen;
                            US5::US5_SymbolHead(v46)
                        } else {
                            US5::US5_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US5::US5_SymbolHead(v52) => {
                    let v52: US6 = v52;
                    let v60: Rc<str> = match &v52 {
                        US6::US6_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US6::US6_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US6::US6_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US6::US6_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US6::US6_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US6::US6_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US6::US6_HeadType => {
                            v5
                        }
                        US6::US6_HeadUnion => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                    };
                    let v69: i32 = (v2.clone().len() as i32);
                    let v70: i32 = v69.wrapping_sub(v61);
                    let v71: usize = ((v61) as usize);
                    let mut v72: i32 = v61.wrapping_add(v70);
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
fn method27(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
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
fn method28(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method28(v3, v1);
            let mut v5: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let mut v6: bool = &*v5 != "";
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
fn section_lines_30(mut v0: US3, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> US7 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v8, v9) => {
                let v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH0> = v9.clone();
                let v19: bool = match &v0 {
                    US3::US3_SpiprojProject => {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v11: bool = v8.starts_with(&*v10);
                        let mut v14: bool = if v11 {
                            true
                        } else {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v13: bool = v8.starts_with(&*v12);
                            v13
                        };
                        let v15: bool = v14 == false;
                        v15
                    }
                    US3::US3_TomlProject => {
                        let v16: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                        let mut v17: bool = &*v16 == "'''";
                        v17
                    }
                };
                if v19 {
                    let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v21: Rc<UH0> = method24(v3, v20);
                    return US7::US7_SplitProject(v2, v21, v1);
                } else {
                    let v23: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                    let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v3));
                    (v1, v3) = (v9, v24);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v5: Rc<UH0> = method24(v3, v4);
                let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return US7::US7_SplitProject(v2, v5, v6);
            }
        }
    }
}
fn split_lines_29(mut v0: US3, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> US7 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: Rc<str> = Rc::<str>::from((v4).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v10: Rc<str> = match &v0 {
                    US3::US3_SpiprojProject => {
                        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                        v7
                    }
                    US3::US3_TomlProject => {
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                        v8
                    }
                };
                let v11: bool = &*v6 == &*v10;
                if v11 {
                    let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v12));
                    let mut v14: Rc<UH0> = method24(v2, v13);
                    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    return section_lines_30(v0, v5, v14, v15);
                } else {
                    let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v2));
                    (v1, v2) = (v5, v17);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US7::US7_NoModules;
            }
        }
    }
}
fn method31(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method31(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes_"); } LIT.with(|lit| lit.clone()) };
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
fn state_file_text_33(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v2.borrow_mut().push(v0);
    v2.borrow_mut().push(v1);
    let v3: Rc<Vec<Rc<str>>> = Rc::new(v2.borrow().clone());
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v3).as_ref().clone()));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from((*v4.borrow()).join(&*v5));
    let v7: bool = std::path::Path::new(&*v6).is_file();
    if v7 {
        let (mut v8, mut v9): (bool, Rc<str>) = match std::fs::read(&*v6) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v8 {
            let v10: Rc<str> = string_concat(&["rig state read failed: ", &*v6]);
            std::panic::panic_any::<std::string::String>(format!("{}", v10))
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = Rc::<str>::from(v6.replace(&*v12, &*v5));
            let mut v14: Rc<str> = loop_18(v13);
            let v15: Option<usize> = v14.rfind(&*v5);
            let v16: Option<usize> = method19(v15);
            let mut v17: Rc<dyn Fn((usize)) -> i32> = closure3();
            let mut v18: Option<i32> = v16.map(|x| v17(x));
            let v20: i32 = v18.unwrap_or(-1i32);
            let v21: bool = v20 < 0i32;
            let mut v25: bool = if v21 {
                true
            } else {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v23: bool = v14.ends_with(&*v22);
                let v24: bool = v23 == false;
                v24
            };
            let v46: US4 = if v25 {
                US4::US4_OtherFile
            } else {
                let v27: usize = ((0i32) as usize);
                let mut v28: usize = ((v20) as usize);
                let mut v29: Rc<str> = Rc::<str>::from(&(v14)[v27..v28]);
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v31: bool = v29.ends_with(&*v30);
                if v31 {
                    let v32: Rc<str> = loop_20(v29);
                    US4::US4_StateFile(v32)
                } else {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v35: bool = v29.ends_with(&*v34);
                    if v35 {
                        let v36: Rc<str> = loop_21(v29);
                        US4::US4_HistoryFile(v36)
                    } else {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v39: bool = v29.ends_with(&*v38);
                        if v39 {
                            let v40: Rc<str> = loop_22(v29);
                            US4::US4_ArchiveFile(v40)
                        } else {
                            US4::US4_OtherFile
                        }
                    }
                }
            };
            match v46 {
                US4::US4_OtherFile => {
                    v9
                }
                _ => {
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<UH0> = split_list_23(v47.clone(), v9);
                    let v49: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v50: Rc<UH0> = method26(v48, v49);
                    let v51: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v52: i32 = 0i32;
                    let v53: i32 = method27(v51.clone(), v50, v52);
                    let v54: Rc<Vec<Rc<str>>> = Rc::new(v51.borrow().clone());
                    let v55: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v54).as_ref().clone()));
                    let v56: Rc<str> = Rc::<str>::from((*v55.borrow()).join(&*v47));
                    v56
                }
            }
        }
    } else {
        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v59
    }
}
fn method32(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let mut v5: Rc<UH0> = method32(v0.clone(), v4, v2);
            let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v6.borrow_mut().push(v3);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            v6.borrow_mut().push(v7);
            let v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
            let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
            let v12: Rc<str> = state_file_text_33(v0, v11);
            let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            v13.borrow_mut().push(v14);
            v13.borrow_mut().push(v12);
            let v15: Rc<Vec<Rc<str>>> = Rc::new(v13.borrow().clone());
            let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
            let mut v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v10));
            Rc::new(UH0::UH0_Cons(v17, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn state_segments_text_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v1.borrow_mut().push(v0.clone());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.toml"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v2.clone());
    let v3: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v3).as_ref().clone()));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from((*v4.borrow()).join(&*v5));
    let v7: bool = std::path::Path::new(&*v6).is_file();
    let v10: US3 = if v7 {
        US3::US3_TomlProject
    } else {
        US3::US3_SpiprojProject
    };
    let mut v13: Rc<str> = match v10 {
        US3::US3_SpiprojProject => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US3::US3_TomlProject => {
            v2.clone()
        }
    };
    let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v14.borrow_mut().push(v0.clone());
    v14.borrow_mut().push(v13);
    let v15: Rc<Vec<Rc<str>>> = Rc::new(v14.borrow().clone());
    let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
    let mut v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v5));
    let mut v18: bool = std::path::Path::new(&*v17).is_file();
    let v71: Rc<str> = if v18 {
        let (mut v19, mut v20): (bool, Rc<str>) = match std::fs::read(&*v17) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v19 {
            let v21: Rc<str> = string_concat(&["rig state read failed: ", &*v17]);
            std::panic::panic_any::<std::string::String>(format!("{}", v21))
        } else {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = Rc::<str>::from(v17.replace(&*v23, &*v5));
            let mut v25: Rc<str> = loop_18(v24);
            let v26: Option<usize> = v25.rfind(&*v5);
            let v27: Option<usize> = method19(v26);
            let mut v28: Rc<dyn Fn((usize)) -> i32> = closure3();
            let mut v29: Option<i32> = v27.map(|x| v28(x));
            let v31: i32 = v29.unwrap_or(-1i32);
            let v32: bool = v31 < 0i32;
            let v36: bool = if v32 {
                true
            } else {
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v34: bool = v25.ends_with(&*v33);
                let v35: bool = v34 == false;
                v35
            };
            let v57: US4 = if v36 {
                US4::US4_OtherFile
            } else {
                let v38: usize = ((0i32) as usize);
                let v39: usize = ((v31) as usize);
                let v40: Rc<str> = Rc::<str>::from(&(v25)[v38..v39]);
                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v42: bool = v40.ends_with(&*v41);
                if v42 {
                    let v43: Rc<str> = loop_20(v40);
                    US4::US4_StateFile(v43)
                } else {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v46: bool = v40.ends_with(&*v45);
                    if v46 {
                        let v47: Rc<str> = loop_21(v40);
                        US4::US4_HistoryFile(v47)
                    } else {
                        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v50: bool = v40.ends_with(&*v49);
                        if v50 {
                            let v51: Rc<str> = loop_22(v40);
                            US4::US4_ArchiveFile(v51)
                        } else {
                            US4::US4_OtherFile
                        }
                    }
                }
            };
            match v57 {
                US4::US4_OtherFile => {
                    v20
                }
                _ => {
                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v59: Rc<UH0> = split_list_23(v58.clone(), v20);
                    let v60: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v61: Rc<UH0> = method26(v59, v60);
                    let v62: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v63: i32 = 0i32;
                    let v64: i32 = method27(v62.clone(), v61, v63);
                    let v65: Rc<Vec<Rc<str>>> = Rc::new(v62.borrow().clone());
                    let v66: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v65).as_ref().clone()));
                    let v67: Rc<str> = Rc::<str>::from((*v66.borrow()).join(&*v58));
                    v67
                }
            }
        }
    } else {
        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v70
    };
    let mut v72: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v72.borrow_mut().push(v0.clone());
    v72.borrow_mut().push(v2);
    let v73: Rc<Vec<Rc<str>>> = Rc::new(v72.borrow().clone());
    let v74: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v73).as_ref().clone()));
    let v75: Rc<str> = Rc::<str>::from((*v74.borrow()).join(&*v5));
    let v76: bool = std::path::Path::new(&*v75).is_file();
    let v79: US3 = if v76 {
        US3::US3_TomlProject
    } else {
        US3::US3_SpiprojProject
    };
    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v83: Rc<str> = Rc::<str>::from(v71.replace(&*v81, &*v82));
    let v84: Rc<UH0> = split_list_23(v80, v83);
    let v85: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v86: Rc<UH0> = method28(v84, v85);
    let v87: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v88: US7 = split_lines_29(v79, v86, v87);
    let v94: Rc<UH0> = match v88 {
        US7::US7_NoModules => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US7::US7_SplitProject(v89, v90, v91) => {
            let v89: Rc<UH0> = v89;
            let v90: Rc<UH0> = v90;
            let v91: Rc<UH0> = v91;
            v90
        }
    };
    let v95: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v96: Rc<UH0> = method31(v94, v95);
    let v97: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v98: Rc<UH0> = method32(v0, v96, v97);
    let v99: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v100: i32 = 0i32;
    let v101: i32 = method27(v99.clone(), v98, v100);
    let v102: Rc<Vec<Rc<str>>> = Rc::new(v99.borrow().clone());
    let v103: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v102).as_ref().clone()));
    let v104: Rc<str> = Rc::<str>::from((*v103.borrow()).join(&*v82));
    v104
}
fn segments_of_16(mut v0: Rc<str>) -> Rc<str> {
    state_segments_text_17(v0)
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        segments_of_16(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_lines_from_35(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: i32 = declared_from_4(v0.clone(), v1.clone(), v2);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let (mut v8, mut v9, mut v10): (Rc<str>, Rc<str>, Rc<str>) = note_at_6(v0.clone(), v4);
            let v11: Rc<str> = local_stamp_12(v8.clone());
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = concat5_2(v12.clone(), v10, v13.clone(), v13.clone(), v13.clone());
            let v15: Rc<str> = concat5_2(v8, v11, v12, v9, v14);
            let v16: Rc<str> = concat5_2(v3, v7, v15, v13.clone(), v13.clone());
            (v2, v3) = (v6, v16);
            continue;
        }
    }
}
fn notes_text_34(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = concat5_2(v2.clone(), v3.clone(), v1.clone(), v4.clone(), v5.clone());
    let v7: i32 = 0i32;
    let v8: i32 = 0i32;
    let mut v9: i32 = declared_count_3(v0.clone(), v6, v7, v8);
    let v10: bool = v9 == 0i32;
    if v10 {
        v5
    } else {
        let v11: Rc<str> = concat5_2(v2, v3, v1, v4, v5.clone());
        let v12: i32 = 0i32;
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes ("); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = concat5_2(v13, v14, v15, v5.clone(), v5);
        note_lines_from_35(v0, v11, v12, v16)
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        notes_text_34(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_json_at_38(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: i32 = v1.wrapping_add(6i32);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = 2i32;
    let mut v5: i32 = field_start_7(v0.clone(), v2, v3, v4);
    let mut v6: US1 = US1::US1_ScanPlain;
    let v7: i32 = field_end_8(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_10(v8);
    let v10: Rc<str> = spiral_json_quote(&*v9);
    let v11: i32 = 3i32;
    let v12: i32 = field_start_7(v0.clone(), v2, v3, v11);
    let mut v13: US1 = US1::US1_ScanPlain;
    let mut v14: i32 = field_end_8(v0.clone(), v12, v3, v13);
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: Rc<str> = unquoted_10(v15);
    let mut v17: Rc<str> = spiral_json_quote(&*v16);
    let mut v18: i32 = 4i32;
    let v19: i32 = field_start_7(v0.clone(), v2, v3, v18);
    let v20: US1 = US1::US1_ScanPlain;
    let v21: i32 = field_end_8(v0.clone(), v19, v3, v20);
    let v22: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v21 as usize).unwrap_or(""));
    let v23: Rc<str> = unquoted_10(v22);
    let v24: Rc<str> = spiral_json_quote(&*v23);
    let mut v25: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"at\":"); } LIT.with(|lit| lit.clone()) };
    v25.borrow_mut().push(v26);
    v25.borrow_mut().push(v10);
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"author\":"); } LIT.with(|lit| lit.clone()) };
    v25.borrow_mut().push(v27);
    v25.borrow_mut().push(v17);
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"text\":"); } LIT.with(|lit| lit.clone()) };
    v25.borrow_mut().push(v28);
    v25.borrow_mut().push(v24);
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    v25.borrow_mut().push(v29);
    let v30: Rc<Vec<Rc<str>>> = Rc::new(v25.borrow().clone());
    let v31: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v30).as_ref().clone()));
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = Rc::<str>::from((*v31.borrow()).join(&*v32));
    v33
}
fn note_json_from_37(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: i32 = declared_from_4(v0.clone(), v1.clone(), v2);
        let mut v6: bool = v5 < 0i32;
        if v6 {
            return v3;
        } else {
            let v7: i32 = v5.wrapping_add(1i32);
            let v8: Rc<str> = note_json_at_38(v0.clone(), v5);
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = concat5_2(v3, v4, v8, v9.clone(), v9.clone());
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            (v2, v3, v4) = (v7, v10, v11);
            continue;
        }
    }
}
fn notes_json_36(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"notes\":["); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = concat5_2(v3, v4, v1, v5, v6.clone());
    let v8: i32 = 0i32;
    let mut v9: Rc<str> = note_json_from_37(v0, v7, v8, v6.clone(), v6.clone());
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
    concat5_2(v2, v9, v10, v6.clone(), v6)
}
fn closure5() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        notes_json_36(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        state_segments_text_17(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        let v4: US0 = note_filter_of_14(v2, v3);
        notes_filtered_text_1(v0, v1, v4)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_last_note(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_agile_segments_of(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
pub fn rig_agile_notes_text(v0: &str, v1: &str) -> Rc<str> {
    closure4()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_agile_notes_json(v0: &str, v1: &str) -> Rc<str> {
    closure5()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_agile_state_segments(v0: &str) -> Rc<str> {
    closure6()(Rc::<str>::from(v0))
}
pub fn rig_agile_notes_filtered_text(v0: &str, v1: &str, v2: &str, v3: &str) -> Rc<str> {
    closure7()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub type SpiralShape_652accfc6a8d = Mut0;
