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
    US0_FileReadable(Rc<str>),
    US0_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_SitesKept,
    US1_SitesAppeared(Rc<str>),
    US1_SitesMoved(Rc<str>, Rc<str>),
    US1_SitesVanished(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_StateWritten,
    US2_StateChangedSinceRead,
    US2_StateWriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_Replaced,
    US3_TemporaryWriteFailed,
    US3_RenameFailed,
    US3_RenameFailedTemporaryKept,
    US3_ReadbackMismatch,
    US3_ChangedSinceRead,
}
fn concat5_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn harvested_ledger_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: bool = &*v1 == "";
        if v3 {
            let v4: Rc<str> = std::rc::Rc::<str>::from({ let mut lines = (v2).lines().filter(|line| !line.is_empty()).collect::<Vec<_>>(); lines.sort(); lines.dedup(); lines.join("
") }.as_str());
            return v4;
        } else {
            let v5: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(10u8)).next().unwrap_or(""));
            let v6: Rc<str> = agile_todo_lines::rig_agile_todo_id(&*v0, &*v5);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v5.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
            let mut v9: bool = &*v6 == "";
            let v11: bool = if v9 {
                true
            } else {
                let v10: bool = v8 < 0i32;
                v10
            };
            let v15: bool = if v11 {
                true
            } else {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TODO"); } LIT.with(|lit| lit.clone()) };
                let v13: i32 = v5.find(&*v12).map(|offset| offset as i32).unwrap_or(-1);
                let v14: bool = v13 < 0i32;
                v14
            };
            let v21: Rc<str> = if v15 {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v16
            } else {
                let v18: Rc<str> = std::rc::Rc::<str>::from(v5.get(0i32 as usize..v8 as usize).unwrap_or(""));
                let v20: Rc<str> = string_concat(&[&*v6, "\t", &*v18, "\n"]);
                v20
            };
            let v22: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let v23: Rc<str> = string_concat(&[&*v2, &*v21]);
            (v1, v2) = (v22, v23);
            continue;
        }
    }
}
fn ledger_from_rows_3(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = TodoSite "); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            let v6: Rc<str> = std::rc::Rc::<str>::from({ let mut lines = (v2).lines().filter(|line| !line.is_empty()).collect::<Vec<_>>(); lines.sort(); lines.dedup(); lines.join("
") }.as_str());
            return v6;
        } else {
            let v7: i32 = v4.wrapping_add(15i32);
            let v8: i32 = v7.wrapping_add(2i32);
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v10: i32 = v0.get(v8 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v8 as usize) as i32).unwrap_or(-1);
            let v11: i32 = v10.wrapping_add(1i32);
            let v12: i32 = v0.get(v11 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v11 as usize) as i32).unwrap_or(-1);
            let v13: i32 = v12.wrapping_add(1i32);
            let v14: i32 = v0.get(v13 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v13 as usize) as i32).unwrap_or(-1);
            let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v10 as usize).unwrap_or(""));
            let v17: Rc<str> = std::rc::Rc::<str>::from(v0.get(v13 as usize..v14 as usize).unwrap_or(""));
            let v20: i32 = v14.wrapping_add(1i32);
            let v21: Rc<str> = string_concat(&[&*v2, &*v15, "\t", &*v17, "\n"]);
            (v1, v2) = (v20, v21);
            continue;
        }
    }
}
fn record_start_4(mut v0: Rc<str>) -> (Rc<str>, i32, i32, i32, i32) {
    (v0, 0i32, 0i32, 0i32, 0i32)
}
fn civil_of_days_6(mut v0: u64) -> (u64, u64, u64) {
    let v1: u64 = v0.wrapping_add(719468u64);
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
    let v12: u64 = 365u64.wrapping_mul(v11);
    let v13: u64 = v11.wrapping_div(4u64);
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
        let v28: u64 = v20.wrapping_sub(9u64);
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
fn utc_stamp_5(mut v0: u64) -> Rc<str> {
    let v1: u64 = v0.wrapping_div(1000u64);
    let mut v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_6(v2);
    let v6: u64 = v1.wrapping_rem(86400u64);
    let v7: u64 = v6.wrapping_div(60u64);
    let v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v4 < 10u64;
    let v16: Rc<str> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_1(v11, v12, v13.clone(), v13.clone(), v13)
    } else {
        let v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15
    };
    let v17: bool = v5 < 10u64;
    let v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_1(v18, v19, v20.clone(), v20.clone(), v20)
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22
    };
    let v24: Rc<str> = concat5_1(v8, v9.clone(), v16, v9, v23);
    let v25: u64 = v7.wrapping_div(60u64);
    let v26: bool = v25 < 10u64;
    let v32: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_1(v27, v28, v29.clone(), v29.clone(), v29)
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
        concat5_1(v36, v37, v38.clone(), v38.clone(), v38)
    } else {
        let v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = concat5_1(v32, v33, v41, v42, v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    concat5_1(v24, v45, v44, v43.clone(), v43)
}
fn sites_of_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: bool = &*v0 == "";
        if v3 {
            return v2;
        } else {
            let v4: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let v5: Rc<str> = string_concat(&[&*v1, "\t"]);
            let v6: bool = v4.starts_with(&*v5);
            let v17: Rc<str> = if v6 {
                let v7: bool = &*v2 == "";
                if v7 {
                    let v8: i32 = (v5.len() as i32);
                    let mut v9: i32 = (v4.clone().len() as i32);
                    let v10: Rc<str> = std::rc::Rc::<str>::from(v4.get(v8 as usize..v9 as usize).unwrap_or(""));
                    v10
                } else {
                    let v12: i32 = (v5.len() as i32);
                    let v13: i32 = (v4.clone().len() as i32);
                    let v14: Rc<str> = std::rc::Rc::<str>::from(v4.get(v12 as usize..v13 as usize).unwrap_or(""));
                    let v15: Rc<str> = string_concat(&[&*v2, ", ", &*v14]);
                    v15
                }
            } else {
                v2
            };
            let v18: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            (v0, v2) = (v18, v17);
            continue;
        }
    }
}
fn declared_from_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
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
fn note_schema_ensured_11(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note ="); } LIT.with(|lit| lit.clone()) };
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
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_1(v11, v12, v0, v13.clone(), v13.clone())
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
            let v23: Rc<str> = concat5_1(v20, v21, v22.clone(), v22.clone(), v22.clone());
            let v24: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v19 as usize).unwrap_or(""));
            let v25: i32 = (v0.clone().len() as i32);
            let v26: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v25 as usize).unwrap_or(""));
            concat5_1(v24, v23, v26, v22.clone(), v22.clone())
        }
    } else {
        v0
    }
}
fn declared_count_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_10(v0.clone(), v1.clone(), v2);
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
fn note_row_13(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v8));
    let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = concat5_1(v6, v10, v8, v11, v12.clone());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note ("); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(v0.replace(&*v16, &*v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(v18.replace(&*v15, &*v19));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v22));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(v23.replace(&*v24, &*v25));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = Rc::<str>::from(v26.replace(&*v27, &*v28));
    let v30: Rc<str> = concat5_1(v15.clone(), v29, v15.clone(), v12.clone(), v12.clone());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = concat5_1(v5, v13, v14, v30, v31.clone());
    let v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = Rc::<str>::from(v2.replace(&*v16, &*v17));
    let v36: Rc<str> = Rc::<str>::from(v35.replace(&*v15, &*v19));
    let v37: Rc<str> = Rc::<str>::from(v36.replace(&*v21, &*v22));
    let v38: Rc<str> = Rc::<str>::from(v37.replace(&*v24, &*v25));
    let v39: Rc<str> = Rc::<str>::from(v38.replace(&*v27, &*v28));
    let v40: Rc<str> = concat5_1(v15.clone(), v39, v15.clone(), v12.clone(), v12.clone());
    let v41: Rc<str> = Rc::<str>::from(v3.replace(&*v16, &*v17));
    let v42: Rc<str> = Rc::<str>::from(v41.replace(&*v15, &*v19));
    let v43: Rc<str> = Rc::<str>::from(v42.replace(&*v21, &*v22));
    let v44: Rc<str> = Rc::<str>::from(v43.replace(&*v24, &*v25));
    let v45: Rc<str> = Rc::<str>::from(v44.replace(&*v27, &*v28));
    let v46: Rc<str> = concat5_1(v15.clone(), v45, v15.clone(), v12.clone(), v12.clone());
    let v47: Rc<str> = concat5_1(v33, v34, v40, v31.clone(), v46);
    let v48: Rc<str> = Rc::<str>::from(v4.replace(&*v16, &*v17));
    let v49: Rc<str> = Rc::<str>::from(v48.replace(&*v15, &*v19));
    let v50: Rc<str> = Rc::<str>::from(v49.replace(&*v21, &*v22));
    let v51: Rc<str> = Rc::<str>::from(v50.replace(&*v24, &*v25));
    let v52: Rc<str> = Rc::<str>::from(v51.replace(&*v27, &*v28));
    let v53: Rc<str> = concat5_1(v15.clone(), v52, v15, v12.clone(), v12);
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    concat5_1(v32, v47, v31, v53, v54)
}
fn last_declared_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_10(v0.clone(), v1.clone(), v2);
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
fn notes_insertion_15(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_1(v2, v3.clone(), v1.clone(), v4.clone(), v5.clone());
    let v7: i32 = 0i32;
    let v8: i32 = declared_from_10(v0.clone(), v6, v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = concat5_1(v9, v3.clone(), v1.clone(), v4.clone(), v5.clone());
    let v11: i32 = 0i32;
    let v12: i32 = declared_from_10(v0.clone(), v10, v11);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = concat5_1(v13, v3, v1, v4, v5);
    let v15: i32 = 0i32;
    let v16: i32 = -1i32;
    let v17: i32 = last_declared_16(v0.clone(), v14, v15, v16);
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
fn notes_appended_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: i32 = notes_insertion_15(v0.clone(), v1);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_1(v4, v2, v5.clone(), v5.clone(), v5.clone());
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v3 as usize).unwrap_or(""));
    let v8: i32 = (v0.clone().len() as i32);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v3 as usize..v8 as usize).unwrap_or(""));
    concat5_1(v7, v6, v9, v5.clone(), v5)
}
fn noted_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> (Rc<str>, i32, i32, i32, i32) {
    let v8: bool = &*v6 == "";
    let v19: bool = if v8 {
        true
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = concat5_1(v9, v10, v5.clone(), v11, v12);
        let v14: i32 = 0i32;
        let v15: i32 = declared_from_10(v0.clone(), v13, v14);
        let v16: bool = v15 < 0i32;
        let v17: bool = v16 == false;
        let v18: bool = v17 == false;
        v18
    };
    let (mut v34, mut v35, mut v36, mut v37, mut v38): (Rc<str>, i32, i32, i32, i32) = if v19 {
        (v0, v1, v2, v3, v4)
    } else {
        let v20: Rc<str> = note_schema_ensured_11(v0);
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = concat5_1(v21, v22, v5.clone(), v23, v24);
        let v26: i32 = 0i32;
        let v27: i32 = 0i32;
        let v28: i32 = declared_count_12(v20.clone(), v25, v26, v27);
        let v29: i32 = v28.wrapping_add(1i32);
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos"); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<str> = note_row_13(v5.clone(), v29, v7, v30, v6);
        let v32: Rc<str> = notes_appended_14(v20, v5, v31);
        let v33: i32 = v4.wrapping_add(1i32);
        (v32, v1, v2, v3, v33)
    };
    (v34, v35, v36, v37, v38)
}
fn changes_recorded_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32) -> (Rc<str>, i32, i32, i32, i32) {
    loop {
        let mut v9: bool = &*v0 == "";
        if v9 {
            return (v4, v5, v6, v7, v8);
        } else {
            let v10: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = sites_of_8(v1.clone(), v10.clone(), v11.clone());
            let v13: Rc<str> = sites_of_8(v2.clone(), v10.clone(), v11.clone());
            let v14: bool = &*v12 == &*v13;
            let v23: US1 = if v14 {
                US1::US1_SitesKept
            } else {
                let v16: bool = &*v12 == "";
                if v16 {
                    US1::US1_SitesAppeared(v13)
                } else {
                    let v18: bool = &*v13 == "";
                    if v18 {
                        US1::US1_SitesVanished(v12)
                    } else {
                        US1::US1_SitesMoved(v12, v13)
                    }
                }
            };
            let (mut v46, mut v47, mut v48, mut v49, mut v50): (Rc<str>, i32, i32, i32, i32) = match &v23 {
                US1::US1_SitesKept => {
                    (v4, v5, v6, v7, v8)
                }
                _ => {
                    let (mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, i32, i32, i32, i32) = match &v23 {
                        US1::US1_SitesAppeared(v24) => {
                            let v24: Rc<str> = v24.clone();
                            let v25: i32 = v5.wrapping_add(1i32);
                            (v4, v25, v6, v7, v8)
                        }
                        _ => {
                            let (mut v36, mut v37, mut v38, mut v39, mut v40): (Rc<str>, i32, i32, i32, i32) = match &v23 {
                                US1::US1_SitesMoved(v26, v27) => {
                                    let v26: Rc<str> = v26.clone();
                                    let v27: Rc<str> = v27.clone();
                                    let v28: i32 = v6.wrapping_add(1i32);
                                    (v4, v5, v28, v7, v8)
                                }
                                _ => {
                                    let (mut v31, mut v32, mut v33, mut v34, mut v35): (Rc<str>, i32, i32, i32, i32) = match &v23 {
                                        US1::US1_SitesVanished(v29) => {
                                            let v29: Rc<str> = v29.clone();
                                            let v30: i32 = v7.wrapping_add(1i32);
                                            (v4, v5, v6, v30, v8)
                                        }
                                        _ => unreachable!(),
                                    };
                                    (v31, v32, v33, v34, v35)
                                }
                            };
                            (v36, v37, v38, v39, v40)
                        }
                    };
                    (v41, v42, v43, v44, v45)
                }
            };
            let v64: Rc<str> = match v23 {
                US1::US1_SitesAppeared(v51) => {
                    let v51: Rc<str> = v51;
                    let v52: Rc<str> = string_concat(&["todos: block appeared at ", &*v51]);
                    v52
                }
                US1::US1_SitesKept => {
                    v11
                }
                US1::US1_SitesMoved(v53, v54) => {
                    let v53: Rc<str> = v53;
                    let v54: Rc<str> = v54;
                    let v57: Rc<str> = string_concat(&["todos: block moved from ", &*v53, " to ", &*v54]);
                    v57
                }
                US1::US1_SitesVanished(v58) => {
                    let v58: Rc<str> = v58;
                    let v60: Rc<str> = string_concat(&["todos: block vanished from ", &*v58, " (cut on purpose, or lost)"]);
                    v60
                }
            };
            let (mut v65, mut v66, mut v67, mut v68, mut v69): (Rc<str>, i32, i32, i32, i32) = noted_9(v46, v47, v48, v49, v50, v10, v64, v3.clone());
            let v70: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            (v0, v4, v5, v6, v7, v8) = (v70, v65, v66, v67, v68, v69);
            continue;
        }
    }
}
fn rows_removed_18(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = TodoSite "); } LIT.with(|lit| lit.clone()) };
        let mut v2: i32 = v0.find(&*v1).map(|offset| offset as i32).unwrap_or(-1);
        let v3: bool = v2 < 0i32;
        if v3 {
            return v0;
        } else {
            let v4: i32 = v0.get(..v2 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v6: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v5)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let v7: bool = v6 < 0i32;
            let v10: i32 = if v7 {
                let v8: i32 = (v0.clone().len() as i32);
                v8
            } else {
                let mut v9: i32 = v6.wrapping_add(1i32);
                v9
            };
            let v11: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v4 as usize).unwrap_or(""));
            let v12: i32 = (v0.clone().len() as i32);
            let v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v12 as usize).unwrap_or(""));
            let v14: Rc<str> = string_concat(&[&*v11, &*v13]);
            v0 = v14;
            continue;
        }
    }
}
fn site_rows_19(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: bool = &*v0 == "";
        if v3 {
            return v2;
        } else {
            let v4: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let v5: Rc<str> = std::rc::Rc::<str>::from((v4).split(char::from(9u8)).next().unwrap_or(""));
            let v6: Rc<str> = std::rc::Rc::<str>::from((v4).split_once(char::from(9u8)).map(|(_, site)| site).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = TodoSite "); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = std::rc::Rc::<str>::from(format!("inl todo_site_{:04}{}({}{}{}, {}{}{})
", v1, &*(v7), char::from(34u8), &*v5, char::from(34u8), char::from(34u8), &*v6, char::from(34u8)).as_str());
            let mut v9: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let v10: i32 = v1.wrapping_add(1i32);
            let v11: Rc<str> = string_concat(&[&*v2, &*v8]);
            (v0, v1, v2) = (v9, v10, v11);
            continue;
        }
    }
}
fn ledger_written_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = rows_removed_18(v0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union todo_site ="); } LIT.with(|lit| lit.clone()) };
    let v4: bool = v2.contains(&*v3);
    let v27: Rc<str> = if v4 {
        v2
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
        let v6: bool = v2.starts_with(&*v5);
        let v13: i32 = if v6 {
            0i32
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\ninl "); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v2.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
            let mut v9: bool = v8 < 0i32;
            if v9 {
                let v10: i32 = (v2.clone().len() as i32);
                v10
            } else {
                let v11: i32 = v8.wrapping_add(1i32);
                v11
            }
        };
        let v14: Rc<str> = std::rc::Rc::<str>::from(v2.get(0i32 as usize..v13 as usize).unwrap_or(""));
        let v15: Rc<str> = string_concat(&[&*v14, "union todo_site =\n    | TodoSite :: string * string -> todo_site\n\n"]);
        let v16: bool = v2.starts_with(&*v5);
        let v23: i32 = if v16 {
            0i32
        } else {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\ninl "); } LIT.with(|lit| lit.clone()) };
            let v18: i32 = v2.find(&*v17).map(|offset| offset as i32).unwrap_or(-1);
            let v19: bool = v18 < 0i32;
            if v19 {
                let v20: i32 = (v2.clone().len() as i32);
                v20
            } else {
                let v21: i32 = v18.wrapping_add(1i32);
                v21
            }
        };
        let v24: i32 = (v2.clone().len() as i32);
        let v25: Rc<str> = std::rc::Rc::<str>::from(v2.get(v23 as usize..v24 as usize).unwrap_or(""));
        let v26: Rc<str> = string_concat(&[&*v15, &*v25]);
        v26
    };
    let v28: i32 = v27.find(&*v3).map(|offset| offset as i32).unwrap_or(-1);
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
    let v30: i32 = v27.get(v28 as usize..).and_then(|rest| rest.find(&*v29)).map(|found| (found + v28 as usize) as i32).unwrap_or(-1);
    let v31: i32 = v30.wrapping_add(2i32);
    let v32: Rc<str> = std::rc::Rc::<str>::from(v27.get(0i32 as usize..v31 as usize).unwrap_or(""));
    let v33: i32 = 0i32;
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = site_rows_19(v1, v33, v34);
    let v37: i32 = (v27.clone().len() as i32);
    let v38: Rc<str> = std::rc::Rc::<str>::from(v27.get(v31 as usize..v37 as usize).unwrap_or(""));
    let v39: Rc<str> = string_concat(&[&*v32, &*v35, &*v38]);
    v39
}
fn renamed_within_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn state_written_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US2 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_1(v0.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    std::env::var("KIT_AGILE_TEST_PAUSE_MS").ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
    if v6 {
        let v7: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let v8: bool = std::fs::write(&*v7, &*v2).is_ok();
        let v21: US3 = if v8 {
            let mut v9: i32 = 40i32;
            let v10: bool = renamed_within_21(v0.clone(), v7.clone(), v9);
            if v10 {
                let v11: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v11 {
                    US3::US3_Replaced
                } else {
                    US3::US3_ReadbackMismatch
                }
            } else {
                let v15: bool = std::fs::remove_file(&*v7).is_ok() || !std::path::Path::new(&*v7).exists();
                if v15 {
                    US3::US3_RenameFailed
                } else {
                    US3::US3_RenameFailedTemporaryKept
                }
            }
        } else {
            US3::US3_TemporaryWriteFailed
        };
        match &v21 {
            US3::US3_Replaced => {
                US2::US2_StateWritten
            }
            _ => {
                let v32: Rc<str> = match v21 {
                    US3::US3_ChangedSinceRead => {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v27
                    }
                    US3::US3_ReadbackMismatch => {
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v26
                    }
                    US3::US3_RenameFailed => {
                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v24
                    }
                    US3::US3_RenameFailedTemporaryKept => {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v25
                    }
                    US3::US3_TemporaryWriteFailed => {
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v23
                    }
                    _ => unreachable!(),
                };
                US2::US2_StateWriteFailed(v32)
            }
        }
    } else {
        US2::US2_StateChangedSinceRead
    }
}
fn record_entry_0() -> i32 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v1: i32 = 1i32;
    let mut v2: Rc<str> = usize::try_from(v1).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_1(v2.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    let v6: bool = std::path::Path::new(&*v5).is_file();
    let mut v9: US0 = if v6 {
        US0::US0_FileReadable(v5.clone())
    } else {
        US0::US0_FileMissing(v5.clone())
    };
    match v9 {
        US0::US0_FileMissing(v10) => {
            let v10: Rc<str> = v10;
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = string_concat(&["agile todos --record rejected: ", &*v11]);
            let v13: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v12 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v13
        }
        US0::US0_FileReadable(v14) => {
            let v14: Rc<str> = v14;
            let v15: Rc<str> = std::fs::read_to_string(&*v14).unwrap_or_default().into();
            let v16: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v15);
            let v17: Rc<str> = agile_todo_scan::rig_agile_todo_sites(&*v2, "*");
            let v18: Rc<str> = harvested_ledger_2(v16.clone(), v17, v4.clone());
            let v19: i32 = 0i32;
            let v20: Rc<str> = ledger_from_rows_3(v16.clone(), v19, v4);
            let (mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, i32, i32, i32, i32) = record_start_4(v16.clone());
            let v27: Rc<str> = string_concat(&[&*v20, "\n", &*v18]);
            let v28: Rc<str> = std::rc::Rc::<str>::from({ let mut ids = (v27).lines().filter_map(|line| line.split(char::from(9u8)).next()).filter(|id| !id.is_empty()).collect::<Vec<_>>(); ids.sort(); ids.dedup(); ids.join("
") }.as_str());
            let v29: Rc<str> = utc_stamp_5(v0);
            let (mut v30, mut v31, mut v32, mut v33, mut v34): (Rc<str>, i32, i32, i32, i32) = changes_recorded_7(v28, v20, v18.clone(), v29, v21, v22, v23, v24, v25);
            let v35: Rc<str> = ledger_written_17(v30, v18.clone());
            let v36: Rc<str> = std::rc::Rc::<str>::from((v31).to_string());
            let v39: Rc<str> = std::rc::Rc::<str>::from((v32).to_string());
            let v41: Rc<str> = std::rc::Rc::<str>::from((v33).to_string());
            let v44: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
            let v46: Rc<str> = string_concat(&[" appeared=", &*v36, " moved=", &*v39, " vanished=", &*v41, " notes=", &*v44]);
            let v47: bool = &*v35 == &*v16;
            if v47 {
                let v48: i32 = (v18).lines().count() as i32;
                let v49: Rc<str> = std::rc::Rc::<str>::from((v48).to_string());
                let v53: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                let v54: Rc<str> = std::rc::Rc::<str>::from((v53).to_string());
                let v55: Rc<str> = string_concat(&["kit agile todos record ok sites=", &*v49, &*v46, " unchanged=true elapsed_ms=", &*v54]);
                let v56: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v55.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                v56
            } else {
                let v57: US2 = state_written_20(v5, v15, v35.clone());
                match &v57 {
                    US2::US2_StateWritten => {
                        let v58: i32 = (v18).lines().count() as i32;
                        let v59: Rc<str> = std::rc::Rc::<str>::from((v58).to_string());
                        let v62: u64 = v16.len() as u64;
                        let v63: u64 = v35.len() as u64;
                        let v64: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                        let v65: Rc<str> = std::rc::Rc::<str>::from((v62).to_string());
                        let v68: Rc<str> = std::rc::Rc::<str>::from((v63).to_string());
                        let v71: Rc<str> = std::rc::Rc::<str>::from((v64).to_string());
                        let v73: Rc<str> = string_concat(&["kit agile todos record ok sites=", &*v59, &*v46, " bytes_before=", &*v65, " bytes_after=", &*v68, " elapsed_ms=", &*v71]);
                        let v74: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v73.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v74
                    }
                    _ => {
                        let v78: Rc<str> = match v57 {
                            US2::US2_StateChangedSinceRead => {
                                let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                v75
                            }
                            US2::US2_StateWriteFailed(v76) => {
                                let v76: Rc<str> = v76;
                                v76
                            }
                            _ => unreachable!(),
                        };
                        let v79: Rc<str> = string_concat(&["agile todos --record rejected: ", &*v78]);
                        let v80: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v79 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v80
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        record_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_todos_record_run() -> i32 {
    closure0()()
}
