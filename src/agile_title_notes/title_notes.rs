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
    US0_ScanPlain,
    US0_ScanQuoted,
    US0_ScanEscaped,
}
#[derive(Clone)]
enum US1 {
    US1_StateWritten,
    US1_StateChangedSinceRead,
    US1_StateWriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_StateFile(Rc<str>),
    US2_HistoryFile(Rc<str>),
    US2_ArchiveFile(Rc<str>),
    US2_OtherFile,
}
#[derive(Clone)]
enum US3 {
    US3_WordsRoot,
    US3_SymbolsRoot,
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
    US6_Replaced,
    US6_TemporaryWriteFailed,
    US6_RenameFailed,
    US6_RenameFailedTemporaryKept,
    US6_ReadbackMismatch,
    US6_ChangedSinceRead,
}
#[derive(Clone)]
enum US7 {
    US7_FileReadable(Rc<str>),
    US7_FileMissing(Rc<str>),
}
fn concat5_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn note_schema_ensured_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note ="); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = v0.find(&*v1).map(|offset| offset as i32).unwrap_or(-1);
    let v3: bool = v2 < 0i32;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-> task_planning"); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v0.find(&*v4).map(|offset| offset as i32).unwrap_or(-1);
        let v6: bool = v5 < 0i32;
        let mut v9: i32 = if v6 {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-> agile_item"); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v0.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
            v8
        } else {
            v5
        };
        let v10: bool = v9 < 0i32;
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note =\n    | Note :: string * u32 * string * string * string -> task_note"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_3(v11, v12, v0, v13.clone(), v13.clone())
        } else {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v16: i32 = v0.get(v9 as usize..).and_then(|rest| rest.find(&*v15)).map(|found| (found + v9 as usize) as i32).unwrap_or(-1);
            let v17: bool = v16 < 0i32;
            let v19: i32 = if v17 {
                let v18: i32 = (v0.clone().len() as i32);
                v18
            } else {
                v16
            };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note =\n    | Note :: string * u32 * string * string * string -> task_note"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = concat5_3(v20, v21, v22.clone(), v22.clone(), v22.clone());
            let v24: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v19 as usize).unwrap_or(""));
            let v25: i32 = (v0.clone().len() as i32);
            let v26: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v25 as usize).unwrap_or(""));
            concat5_3(v24, v23, v26, v22.clone(), v22.clone())
        }
    } else {
        v0
    }
}
fn declared_from_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
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
fn field_end_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US0::US0_ScanPlain => {
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
                let v25: US0 = match v3 {
                    US0::US0_ScanEscaped => {
                        US0::US0_ScanQuoted
                    }
                    US0::US0_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US0::US0_ScanQuoted
                        } else {
                            US0::US0_ScanPlain
                        }
                    }
                    US0::US0_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US0::US0_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US0::US0_ScanPlain
                            } else {
                                US0::US0_ScanQuoted
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
fn skip_spaces_7(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US0 = US0::US0_ScanPlain;
            let v6: i32 = field_end_6(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_7(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn unescaped_from_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 <= v1;
        if v5 {
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return concat5_3(v4, v6, v7.clone(), v7.clone(), v7.clone());
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v10: bool = v9 == 92i32;
            if v10 {
                let v11: i32 = v1.wrapping_add(1i32);
                let mut v12: i32 = v11.wrapping_add(1i32);
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v15: bool = v14 == 110i32;
                let v24: Rc<str> = if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16
                } else {
                    let v17: bool = v14 == 114i32;
                    if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
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
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = concat5_3(v4, v13, v24, v25.clone(), v25.clone());
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
fn unquoted_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_9(v3, v4, v5, v6, v7)
}
fn declared_count_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_4(v0.clone(), v1.clone(), v2);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let v6: i32 = v4.wrapping_add(1i32);
            let v7: i32 = v3.wrapping_add(1i32);
            (v2, v3) = (v6, v7);
            continue;
        }
    }
}
fn civil_of_days_12(mut v0: u64) -> (u64, u64, u64) {
    let mut v1: u64 = v0.wrapping_add(719468u64);
    let mut v2: u64 = v1.wrapping_div(146097u64);
    let v3: u64 = v2.wrapping_mul(146097u64);
    let v4: u64 = v1.wrapping_sub(v3);
    let v5: u64 = v4.wrapping_div(1460u64);
    let v6: u64 = v4.wrapping_sub(v5);
    let v7: u64 = v4.wrapping_div(36524u64);
    let v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let v10: u64 = v8.wrapping_sub(v9);
    let v11: u64 = v10.wrapping_div(365u64);
    let mut v12: u64 = 365u64.wrapping_mul(v11);
    let mut v13: u64 = v11.wrapping_div(4u64);
    let v14: u64 = v12.wrapping_add(v13);
    let v15: u64 = v11.wrapping_div(100u64);
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
    let v29: u64 = if v26 {
        let v27: u64 = v20.wrapping_add(3u64);
        v27
    } else {
        let mut v28: u64 = v20.wrapping_sub(9u64);
        v28
    };
    let v30: u64 = v2.wrapping_mul(400u64);
    let mut v31: u64 = v11.wrapping_add(v30);
    let mut v32: bool = v29 < 3u64;
    let v33: u64 = if v32 {
        1u64
    } else {
        0u64
    };
    let v34: u64 = v31.wrapping_add(v33);
    (v34, v29, v25)
}
fn utc_stamp_11(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(1000u64);
    let mut v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_12(v2);
    let v6: u64 = v1.wrapping_rem(86400u64);
    let v7: u64 = v6.wrapping_div(60u64);
    let v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v4 < 10u64;
    let v16: Rc<str> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_3(v11, v12, v13.clone(), v13.clone(), v13)
    } else {
        let v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15
    };
    let v17: bool = v5 < 10u64;
    let v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_3(v18, v19, v20.clone(), v20.clone(), v20)
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22
    };
    let v24: Rc<str> = concat5_3(v8, v9.clone(), v16, v9, v23);
    let v25: u64 = v7.wrapping_div(60u64);
    let v26: bool = v25 < 10u64;
    let mut v32: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_3(v27, v28, v29.clone(), v29.clone(), v29)
    } else {
        let mut v31: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        v31
    };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v34: u64 = v7.wrapping_rem(60u64);
    let v35: bool = v34 < 10u64;
    let v41: Rc<str> = if v35 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_3(v36, v37, v38.clone(), v38.clone(), v38)
    } else {
        let v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = concat5_3(v32, v33, v41, v42, v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    concat5_3(v24, v45, v44, v43.clone(), v43)
}
fn last_declared_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_4(v0.clone(), v1.clone(), v2);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let v6: i32 = v4.wrapping_add(1i32);
            (v2, v3) = (v6, v4);
            continue;
        }
    }
}
fn notes_insertion_14(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_3(v2, v3.clone(), v1.clone(), v4.clone(), v5.clone());
    let v7: i32 = 0i32;
    let v8: i32 = declared_from_4(v0.clone(), v6, v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = concat5_3(v9, v3.clone(), v1.clone(), v4.clone(), v5.clone());
    let v11: i32 = 0i32;
    let mut v12: i32 = declared_from_4(v0.clone(), v10, v11);
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = concat5_3(v13, v3, v1, v4, v5);
    let v15: i32 = 0i32;
    let v16: i32 = -1i32;
    let v17: i32 = last_declared_15(v0.clone(), v14, v15, v16);
    let v18: bool = v12 <= v17;
    let v19: i32 = if v18 {
        v17
    } else {
        v12
    };
    let v20: bool = v8 <= v19;
    let v21: i32 = if v20 {
        v19
    } else {
        v8
    };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v23: i32 = v0.get(v21 as usize..).and_then(|rest| rest.find(&*v22)).map(|found| (found + v21 as usize) as i32).unwrap_or(-1);
    let v24: bool = v23 < 0i32;
    if v24 {
        let v25: i32 = (v0.len() as i32);
        v25
    } else {
        v23
    }
}
fn notes_appended_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: i32 = notes_insertion_14(v0.clone(), v1);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_3(v4, v2, v5.clone(), v5.clone(), v5.clone());
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v3 as usize).unwrap_or(""));
    let v8: i32 = (v0.clone().len() as i32);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v3 as usize..v8 as usize).unwrap_or(""));
    concat5_3(v7, v6, v9, v5.clone(), v5)
}
fn loop_17(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
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
fn method18(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_19(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
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
fn loop_20(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
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
fn loop_21(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
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
fn method23(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method24(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method24(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method23(v16, v17)
    }
}
fn method25(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method25(v3, v1);
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
                            let v22: US5 = US5::US5_HeadInl;
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
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US5 = US5::US5_HeadUnion;
                        US4::US4_KeywordHead(v30)
                    } else {
                        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
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
                US4::US4_KeywordHead(v52) => {
                    let v52: US5 = v52;
                    let v59: Rc<str> = match &v52 {
                        US5::US5_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US5::US5_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US5::US5_HeadType => {
                            v5
                        }
                        US5::US5_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v60: i32 = (v59.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US5::US5_HeadInl => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v61
                        }
                        US5::US5_HeadOpen => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                        US5::US5_HeadType => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US5::US5_HeadUnion => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                    };
                    let v69: i32 = (v2.clone().len() as i32);
                    let v70: i32 = v69.wrapping_sub(v60);
                    let v71: usize = ((v60) as usize);
                    let v72: i32 = v60.wrapping_add(v70);
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
fn method26(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn renamed_within_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn state_written_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US1 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_3(v0.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    std::env::var("RIG_AGILE_TEST_PAUSE_MS").or_else(|_| std::env::var("KIT_AGILE_TEST_PAUSE_MS")).ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
    let mut v9: Rc<str> = loop_17(v8);
    let v10: Option<usize> = v9.rfind(&*v7);
    let v11: Option<usize> = method18(v10);
    let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v13: Option<i32> = v11.map(|x| v12(x));
    let v15: i32 = v13.unwrap_or(-1i32);
    let v16: bool = v15 < 0i32;
    let v20: bool = if v16 {
        true
    } else {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v18: bool = v9.ends_with(&*v17);
        let v19: bool = v18 == false;
        v19
    };
    let v41: US2 = if v20 {
        US2::US2_OtherFile
    } else {
        let v22: usize = ((0i32) as usize);
        let v23: usize = ((v15) as usize);
        let v24: Rc<str> = Rc::<str>::from(&(v9)[v22..v23]);
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v26: bool = v24.ends_with(&*v25);
        if v26 {
            let v27: Rc<str> = loop_19(v24);
            US2::US2_StateFile(v27)
        } else {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
            let v30: bool = v24.ends_with(&*v29);
            if v30 {
                let mut v31: Rc<str> = loop_20(v24);
                US2::US2_HistoryFile(v31)
            } else {
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                let v34: bool = v24.ends_with(&*v33);
                if v34 {
                    let v35: Rc<str> = loop_21(v24);
                    US2::US2_ArchiveFile(v35)
                } else {
                    US2::US2_OtherFile
                }
            }
        }
    };
    let v64: US3 = match v41 {
        US2::US2_ArchiveFile(v54) => {
            let v54: Rc<str> = v54;
            let v55: Rc<str> = string_concat(&[&*v54, "/state/spiral.toml"]);
            let v56: bool = std::path::Path::new(&*v55).is_file();
            if v56 {
                US3::US3_SymbolsRoot
            } else {
                US3::US3_WordsRoot
            }
        }
        US2::US2_HistoryFile(v48) => {
            let v48: Rc<str> = v48;
            let v49: Rc<str> = string_concat(&[&*v48, "/state/spiral.toml"]);
            let v50: bool = std::path::Path::new(&*v49).is_file();
            if v50 {
                US3::US3_SymbolsRoot
            } else {
                US3::US3_WordsRoot
            }
        }
        US2::US2_OtherFile => {
            US3::US3_WordsRoot
        }
        US2::US2_StateFile(v42) => {
            let v42: Rc<str> = v42;
            let v43: Rc<str> = string_concat(&[&*v42, "/state/spiral.toml"]);
            let v44: bool = std::path::Path::new(&*v43).is_file();
            if v44 {
                US3::US3_SymbolsRoot
            } else {
                US3::US3_WordsRoot
            }
        }
    };
    let v76: Rc<str> = match v64 {
        US3::US3_SymbolsRoot => {
            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v66: Rc<UH0> = split_list_22(v65.clone(), v1);
            let v67: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v68: Rc<UH0> = method25(v66, v67);
            let v69: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v70: i32 = 0i32;
            let v71: i32 = method26(v69.clone(), v68, v70);
            let v72: Rc<Vec<Rc<str>>> = Rc::new(v69.borrow().clone());
            let v73: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v72).as_ref().clone()));
            let v74: Rc<str> = Rc::<str>::from((*v73.borrow()).join(&*v65));
            v74
        }
        US3::US3_WordsRoot => {
            v1
        }
    };
    let v77: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v76);
    if v77 {
        let v78: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let v79: Rc<str> = Rc::<str>::from(v78.replace(&*v6, &*v7));
        let mut v80: Rc<str> = loop_17(v79);
        let v81: Option<usize> = v80.rfind(&*v7);
        let v82: Option<usize> = method18(v81);
        let mut v83: Option<i32> = v82.map(|x| v12(x));
        let v85: i32 = v83.unwrap_or(-1i32);
        let v86: bool = v85 < 0i32;
        let v90: bool = if v86 {
            true
        } else {
            let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v88: bool = v80.ends_with(&*v87);
            let v89: bool = v88 == false;
            v89
        };
        let v111: US2 = if v90 {
            US2::US2_OtherFile
        } else {
            let v92: usize = ((0i32) as usize);
            let v93: usize = ((v85) as usize);
            let v94: Rc<str> = Rc::<str>::from(&(v80)[v92..v93]);
            let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v96: bool = v94.ends_with(&*v95);
            if v96 {
                let v97: Rc<str> = loop_19(v94);
                US2::US2_StateFile(v97)
            } else {
                let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v100: bool = v94.ends_with(&*v99);
                if v100 {
                    let v101: Rc<str> = loop_20(v94);
                    US2::US2_HistoryFile(v101)
                } else {
                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v104: bool = v94.ends_with(&*v103);
                    if v104 {
                        let v105: Rc<str> = loop_21(v94);
                        US2::US2_ArchiveFile(v105)
                    } else {
                        US2::US2_OtherFile
                    }
                }
            }
        };
        let v134: US3 = match v111 {
            US2::US2_ArchiveFile(v124) => {
                let v124: Rc<str> = v124;
                let v125: Rc<str> = string_concat(&[&*v124, "/state/spiral.toml"]);
                let v126: bool = std::path::Path::new(&*v125).is_file();
                if v126 {
                    US3::US3_SymbolsRoot
                } else {
                    US3::US3_WordsRoot
                }
            }
            US2::US2_HistoryFile(v118) => {
                let v118: Rc<str> = v118;
                let v119: Rc<str> = string_concat(&[&*v118, "/state/spiral.toml"]);
                let v120: bool = std::path::Path::new(&*v119).is_file();
                if v120 {
                    US3::US3_SymbolsRoot
                } else {
                    US3::US3_WordsRoot
                }
            }
            US2::US2_OtherFile => {
                US3::US3_WordsRoot
            }
            US2::US2_StateFile(v112) => {
                let v112: Rc<str> = v112;
                let v113: Rc<str> = string_concat(&[&*v112, "/state/spiral.toml"]);
                let v114: bool = std::path::Path::new(&*v113).is_file();
                if v114 {
                    US3::US3_SymbolsRoot
                } else {
                    US3::US3_WordsRoot
                }
            }
        };
        let v146: Rc<str> = match v134 {
            US3::US3_SymbolsRoot => {
                let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v136: Rc<UH0> = split_list_22(v135.clone(), v2.clone());
                let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v138: Rc<UH0> = method25(v136, v137);
                let v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v140: i32 = 0i32;
                let v141: i32 = method26(v139.clone(), v138, v140);
                let v142: Rc<Vec<Rc<str>>> = Rc::new(v139.borrow().clone());
                let v143: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v142).as_ref().clone()));
                let v144: Rc<str> = Rc::<str>::from((*v143.borrow()).join(&*v135));
                v144
            }
            US3::US3_WordsRoot => {
                v2.clone()
            }
        };
        let v147: bool = std::fs::write(&*v78, &*v146).is_ok();
        let v228: US6 = if v147 {
            let v148: i32 = 40i32;
            let v149: bool = renamed_within_27(v0.clone(), v78.clone(), v148);
            if v149 {
                let v150: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
                let mut v151: Rc<str> = loop_17(v150);
                let v152: Option<usize> = v151.rfind(&*v7);
                let v153: Option<usize> = method18(v152);
                let mut v154: Option<i32> = v153.map(|x| v12(x));
                let v156: i32 = v154.unwrap_or(-1i32);
                let v157: bool = v156 < 0i32;
                let v161: bool = if v157 {
                    true
                } else {
                    let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v159: bool = v151.ends_with(&*v158);
                    let v160: bool = v159 == false;
                    v160
                };
                let v182: US2 = if v161 {
                    US2::US2_OtherFile
                } else {
                    let v163: usize = ((0i32) as usize);
                    let v164: usize = ((v156) as usize);
                    let v165: Rc<str> = Rc::<str>::from(&(v151)[v163..v164]);
                    let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v167: bool = v165.ends_with(&*v166);
                    if v167 {
                        let v168: Rc<str> = loop_19(v165);
                        US2::US2_StateFile(v168)
                    } else {
                        let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v171: bool = v165.ends_with(&*v170);
                        if v171 {
                            let v172: Rc<str> = loop_20(v165);
                            US2::US2_HistoryFile(v172)
                        } else {
                            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v175: bool = v165.ends_with(&*v174);
                            if v175 {
                                let v176: Rc<str> = loop_21(v165);
                                US2::US2_ArchiveFile(v176)
                            } else {
                                US2::US2_OtherFile
                            }
                        }
                    }
                };
                let v205: US3 = match v182 {
                    US2::US2_ArchiveFile(v195) => {
                        let v195: Rc<str> = v195;
                        let v196: Rc<str> = string_concat(&[&*v195, "/state/spiral.toml"]);
                        let v197: bool = std::path::Path::new(&*v196).is_file();
                        if v197 {
                            US3::US3_SymbolsRoot
                        } else {
                            US3::US3_WordsRoot
                        }
                    }
                    US2::US2_HistoryFile(v189) => {
                        let v189: Rc<str> = v189;
                        let v190: Rc<str> = string_concat(&[&*v189, "/state/spiral.toml"]);
                        let v191: bool = std::path::Path::new(&*v190).is_file();
                        if v191 {
                            US3::US3_SymbolsRoot
                        } else {
                            US3::US3_WordsRoot
                        }
                    }
                    US2::US2_OtherFile => {
                        US3::US3_WordsRoot
                    }
                    US2::US2_StateFile(v183) => {
                        let v183: Rc<str> = v183;
                        let v184: Rc<str> = string_concat(&[&*v183, "/state/spiral.toml"]);
                        let v185: bool = std::path::Path::new(&*v184).is_file();
                        if v185 {
                            US3::US3_SymbolsRoot
                        } else {
                            US3::US3_WordsRoot
                        }
                    }
                };
                let v217: Rc<str> = match v205 {
                    US3::US3_SymbolsRoot => {
                        let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v207: Rc<UH0> = split_list_22(v206.clone(), v2);
                        let v208: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v209: Rc<UH0> = method25(v207, v208);
                        let v210: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v211: i32 = 0i32;
                        let v212: i32 = method26(v210.clone(), v209, v211);
                        let v213: Rc<Vec<Rc<str>>> = Rc::new(v210.borrow().clone());
                        let v214: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v213).as_ref().clone()));
                        let v215: Rc<str> = Rc::<str>::from((*v214.borrow()).join(&*v206));
                        v215
                    }
                    US3::US3_WordsRoot => {
                        v2
                    }
                };
                let v218: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v217);
                if v218 {
                    US6::US6_Replaced
                } else {
                    US6::US6_ReadbackMismatch
                }
            } else {
                let v222: bool = std::fs::remove_file(&*v78).is_ok() || !std::path::Path::new(&*v78).exists();
                if v222 {
                    US6::US6_RenameFailed
                } else {
                    US6::US6_RenameFailedTemporaryKept
                }
            }
        } else {
            US6::US6_TemporaryWriteFailed
        };
        match &v228 {
            US6::US6_Replaced => {
                US1::US1_StateWritten
            }
            _ => {
                let v239: Rc<str> = match v228 {
                    US6::US6_ChangedSinceRead => {
                        let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v234
                    }
                    US6::US6_ReadbackMismatch => {
                        let v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v233
                    }
                    US6::US6_RenameFailed => {
                        let v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v231
                    }
                    US6::US6_RenameFailedTemporaryKept => {
                        let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v232
                    }
                    US6::US6_TemporaryWriteFailed => {
                        let v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v230
                    }
                    _ => unreachable!(),
                };
                US1::US1_StateWriteFailed(v239)
            }
        }
    } else {
        US1::US1_StateChangedSinceRead
    }
}
fn title_moved_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> i32 {
    let v5: Rc<str> = note_schema_ensured_2(v3.clone());
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = concat5_3(v6, v7.clone(), v1.clone(), v8.clone(), v9.clone());
    let v11: i32 = 0i32;
    let mut v12: i32 = declared_from_4(v5.clone(), v10, v11);
    let mut v13: i32 = (v5.clone().len() as i32);
    let v14: i32 = v12.wrapping_add(6i32);
    let v15: i32 = 1i32;
    let v16: i32 = field_start_5(v5.clone(), v14, v13, v15);
    let v17: US0 = US0::US0_ScanPlain;
    let v18: i32 = field_end_6(v5.clone(), v16, v13, v17);
    let v19: Rc<str> = std::rc::Rc::<str>::from(v5.get(v16 as usize..v18 as usize).unwrap_or(""));
    let v20: Rc<str> = unquoted_8(v19);
    let v21: i32 = agile_title_tails::rig_agile_title_kept_end(&*v20);
    let v22: bool = v21 < 0i32;
    if v22 {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the title has no \"; \" tail starting with a date or time to move"); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = string_concat(&["agile notes from-title ", &*v1, " rejected: ", &*v23]);
        let v27: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v26 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v27
    } else {
        let mut v28: Rc<str> = std::rc::Rc::<str>::from(v20.get(0i32 as usize..v21 as usize).unwrap_or(""));
        let v29: Rc<str> = Rc::<str>::from((v28).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = concat5_3(v30, v7.clone(), v1.clone(), v8, v9.clone());
        let mut v32: i32 = 0i32;
        let v33: i32 = 0i32;
        let v34: i32 = declared_count_10(v5.clone(), v31, v32, v33);
        let v35: i32 = v34.wrapping_add(1i32);
        let v36: Rc<str> = utc_stamp_11(v4);
        let (mut v37, mut v38): (i32, Rc<str>) = agile_title_tails::rig_agile_title_tail_rows(&*v20, &*v1, v35, &*v36);
        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
        let v41: Rc<str> = Rc::<str>::from(v29.replace(&*v39, &*v40));
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
        let v43: Rc<str> = Rc::<str>::from(v41.replace(&*v7, &*v42));
        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
        let v46: Rc<str> = Rc::<str>::from(v43.replace(&*v44, &*v45));
        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
        let v49: Rc<str> = Rc::<str>::from(v46.replace(&*v47, &*v48));
        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
        let v52: Rc<str> = Rc::<str>::from(v49.replace(&*v50, &*v51));
        let v53: Rc<str> = concat5_3(v7.clone(), v52, v7.clone(), v9.clone(), v9.clone());
        let v54: Rc<str> = std::rc::Rc::<str>::from(v5.get(0i32 as usize..v16 as usize).unwrap_or(""));
        let v55: Rc<str> = std::rc::Rc::<str>::from(v5.get(v18 as usize..v13 as usize).unwrap_or(""));
        let v56: Rc<str> = concat5_3(v54, v53, v55, v9.clone(), v9.clone());
        let v57: Rc<str> = notes_appended_13(v56, v1.clone(), v38);
        let v58: US1 = state_written_16(v0, v2, v57.clone());
        match &v58 {
            US1::US1_StateWritten => {
                let v59: i32 = (v20.len() as i32);
                let v60: i32 = (v29.len() as i32);
                let v61: u64 = v3.len() as u64;
                let v62: u64 = v57.len() as u64;
                let v63: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
                let v66: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
                let v69: Rc<str> = std::rc::Rc::<str>::from((v59).to_string());
                let v71: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                let v74: Rc<str> = std::rc::Rc::<str>::from((v61).to_string());
                let v77: Rc<str> = std::rc::Rc::<str>::from((v62).to_string());
                let mut v80: Rc<str> = std::rc::Rc::<str>::from((v63).to_string());
                let v82: Rc<str> = string_concat(&["rig agile notes ok id=", &*v1, " moved=", &*v66, " title_bytes_before=", &*v69, " title_bytes_after=", &*v71, " bytes_before=", &*v74, " bytes_after=", &*v77, " elapsed_ms=", &*v80]);
                let mut v83: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v82.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                v83
            }
            _ => {
                let v87: Rc<str> = match v58 {
                    US1::US1_StateChangedSinceRead => {
                        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v84
                    }
                    US1::US1_StateWriteFailed(v85) => {
                        let v85: Rc<str> = v85;
                        v85
                    }
                    _ => unreachable!(),
                };
                let v90: Rc<str> = string_concat(&["agile notes from-title ", &*v1, " rejected: ", &*v87]);
                let v91: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v90 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v91
            }
        }
    }
}
fn command_line_argument_count_29() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_28() -> i32 {
    let mut v0: i32 = command_line_argument_count_29();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn method30(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method30(v3, v1);
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
                            let v22: US5 = US5::US5_HeadInl;
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
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US5 = US5::US5_HeadUnion;
                        US4::US4_KeywordHead(v30)
                    } else {
                        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
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
fn from_title_entry_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = argument_count_28();
    let v3: bool = v2 == 4i32;
    if v3 {
        let v4: i32 = 1i32;
        let v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v6: bool = &*v5 == "from-title";
        if v6 {
            let v7: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
            let v8: i32 = 2i32;
            let mut v9: Rc<str> = usize::try_from(v8).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = concat5_3(v9, v10, v11.clone(), v11.clone(), v11.clone());
            let mut v13: bool = std::path::Path::new(&*v12).is_file();
            let v16: US7 = if v13 {
                US7::US7_FileReadable(v12.clone())
            } else {
                US7::US7_FileMissing(v12.clone())
            };
            match v16 {
                US7::US7_FileMissing(v17) => {
                    let v17: Rc<str> = v17;
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<str> = string_concat(&["agile notes from-title ", &*v1, " rejected: ", &*v18]);
                    let v22: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v21 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v22
                }
                US7::US7_FileReadable(v23) => {
                    let v23: Rc<str> = v23;
                    let v24: Rc<str> = std::fs::read_to_string(&*v23).unwrap_or_default().into();
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = Rc::<str>::from(v23.replace(&*v25, &*v26));
                    let mut v28: Rc<str> = loop_17(v27);
                    let v29: Option<usize> = v28.rfind(&*v26);
                    let v30: Option<usize> = method18(v29);
                    let mut v31: Rc<dyn Fn((usize)) -> i32> = closure1();
                    let mut v32: Option<i32> = v30.map(|x| v31(x));
                    let v34: i32 = v32.unwrap_or(-1i32);
                    let v35: bool = v34 < 0i32;
                    let v39: bool = if v35 {
                        true
                    } else {
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                        let v37: bool = v28.ends_with(&*v36);
                        let v38: bool = v37 == false;
                        v38
                    };
                    let v60: US2 = if v39 {
                        US2::US2_OtherFile
                    } else {
                        let v41: usize = ((0i32) as usize);
                        let v42: usize = ((v34) as usize);
                        let v43: Rc<str> = Rc::<str>::from(&(v28)[v41..v42]);
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v43.ends_with(&*v44);
                        if v45 {
                            let v46: Rc<str> = loop_19(v43);
                            US2::US2_StateFile(v46)
                        } else {
                            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                            let v49: bool = v43.ends_with(&*v48);
                            if v49 {
                                let v50: Rc<str> = loop_20(v43);
                                US2::US2_HistoryFile(v50)
                            } else {
                                let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                let v53: bool = v43.ends_with(&*v52);
                                if v53 {
                                    let v54: Rc<str> = loop_21(v43);
                                    US2::US2_ArchiveFile(v54)
                                } else {
                                    US2::US2_OtherFile
                                }
                            }
                        }
                    };
                    let v71: Rc<str> = match v60 {
                        US2::US2_OtherFile => {
                            v24
                        }
                        _ => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let v62: Rc<UH0> = split_list_22(v61.clone(), v24);
                            let v63: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v64: Rc<UH0> = method30(v62, v63);
                            let v65: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v66: i32 = 0i32;
                            let v67: i32 = method26(v65.clone(), v64, v66);
                            let v68: Rc<Vec<Rc<str>>> = Rc::new(v65.borrow().clone());
                            let v69: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v68).as_ref().clone()));
                            let v70: Rc<str> = Rc::<str>::from((*v69.borrow()).join(&*v61));
                            v70
                        }
                    };
                    let v72: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v71);
                    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
                    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                    let v76: Rc<str> = concat5_3(v73, v74, v1.clone(), v75, v11);
                    let v77: i32 = 0i32;
                    let v78: i32 = declared_from_4(v72.clone(), v76, v77);
                    let v79: bool = v78 < 0i32;
                    let mut v80: bool = v79 == false;
                    if v80 {
                        title_moved_1(v12, v1, v71, v72, v7)
                    } else {
                        let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                        let v85: Rc<str> = string_concat(&["agile notes from-title ", &*v1, " rejected: ", &*v82]);
                        let v86: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v85 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v86
                    }
                }
            }
        } else {
            let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile notes from-title <root> <id>"); } LIT.with(|lit| lit.clone()) };
            let v93: Rc<str> = string_concat(&["agile notes from-title ", &*v1, " rejected: ", &*v90]);
            let v94: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v93 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v94
        }
    } else {
        let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile notes from-title <root> <id>"); } LIT.with(|lit| lit.clone()) };
        let v99: Rc<str> = string_concat(&["agile notes from-title ", &*v1, " rejected: ", &*v96]);
        let v100: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v99 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v100
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        from_title_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_notes_from_title_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
