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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_FileReadable(Rc<str>),
    US0_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_AddedText(Rc<str>),
    US1_AddedRefused(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Replaced,
    US2_TemporaryWriteFailed,
    US2_RenameFailed,
    US2_RenameFailedTemporaryKept,
    US2_ReadbackMismatch,
    US2_ChangedSinceRead,
}
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_RequestRead(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US4_RequestRefused(Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_FlagVerb,
    US6_FlagExit,
    US6_FlagArgv,
    US6_FlagCwd,
    US6_FlagAs,
}
#[derive(Clone)]
enum US5 {
    US5_FlagKnown(US6),
    US5_FlagUnknown,
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method4(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
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
        while method4(v7, &v9) {
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
        method3(v16, v17)
    }
}
fn trim_dashes_6(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.starts_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(1i32);
            let v5: usize = ((1i32) as usize);
            let v6: i32 = 1i32.wrapping_add(v4);
            let v7: usize = ((v6) as usize);
            let v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
            v0 = v8;
            continue;
        } else {
            return v0;
        }
    }
}
fn upper_words_5(mut v0: Rc<UH0>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<str> = trim_dashes_6(v2);
            let v5: Rc<str> = Rc::<str>::from(v4.to_uppercase());
            let v6: Rc<str> = upper_words_5(v3);
            let v7: bool = &*v5 == "";
            if v7 {
                v6
            } else {
                let v8: bool = &*v6 == "";
                if v8 {
                    v5
                } else {
                    let v10: Rc<str> = string_concat(&[&*v5, "-", &*v6]);
                    v10
                }
            }
        }
        UH0::UH0_Nil => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
    }
}
fn free_id_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    loop {
        let v3: bool = v2 == 1i32;
        let v7: Rc<str> = if v3 {
            v1.clone()
        } else {
            let v5: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let v6: Rc<str> = string_concat(&[&*v1, "-", &*v5]);
            v6
        };
        let mut v9: Rc<str> = string_concat(&["Task (\"", &*v7, "\","]);
        let v10: bool = v0.contains(&*v9);
        if v10 {
            let v11: i32 = v2.wrapping_add(1i32);
            v2 = v11;
            continue;
        } else {
            return v7;
        }
    }
}
fn concat5_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn extras_applied_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: &Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> US1 {
    let (mut v8, mut v9): (i32, Rc<str>) = agile_task_notes::rig_agile_groomed(&*v0, &*v1, &*v2, &*v3, &*v4, &*v6, &*v7);
    let v10: bool = v8 == 0i32;
    if v10 {
        US1::US1_AddedText(v9)
    } else {
        US1::US1_AddedRefused(v9)
    }
}
fn renamed_within_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn replace_records_unchanged_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US2 {
    let v3: bool = rig_agile_records::rig_agile_records_unchanged(&*v0, &*v1);
    if v3 {
        let v4: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let v5: bool = std::fs::write(&*v4, &*v2).is_ok();
        if v5 {
            let v6: i32 = 40i32;
            let v7: bool = renamed_within_11(v0.clone(), v4.clone(), v6);
            if v7 {
                let v8: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v8 {
                    US2::US2_Replaced
                } else {
                    US2::US2_ReadbackMismatch
                }
            } else {
                let v12: bool = std::fs::remove_file(&*v4).is_ok() || !std::path::Path::new(&*v4).exists();
                if v12 {
                    US2::US2_RenameFailed
                } else {
                    US2::US2_RenameFailedTemporaryKept
                }
            }
        } else {
            US2::US2_TemporaryWriteFailed
        }
    } else {
        US2::US2_ChangedSinceRead
    }
}
fn gap_filed_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> i32 {
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v8));
    let v10: Rc<str> = string_concat(&[&*v9, "/state/agile.spi"]);
    let v11: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v10).unwrap_or_default().as_str());
    let v12: bool = &*v11 == "";
    if v12 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing or empty"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = string_concat(&["agile gap rejected: ", &*v13]);
        let v15: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v14 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v15
    } else {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<UH0> = split_list_2(v16.clone(), v2.clone());
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<UH0> = split_list_2(v18.clone(), v6.clone());
        let v24: Rc<str> = match Rc::unwrap_or_clone(v19) {
            UH0::UH0_Cons(v20, v21) => {
                let v20: Rc<str> = v20;
                let v21: Rc<UH0> = v21;
                v20
            }
            UH0::UH0_Nil => {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v22
            }
        };
        let v25: Rc<str> = Rc::<str>::from(v24.to_uppercase());
        let v27: Rc<str> = upper_words_5(v17);
        let v28: Rc<str> = string_concat(&[&*v25, "-GAP-", &*v27]);
        let v29: i32 = 1i32;
        let v30: Rc<str> = free_id_7(v11, v28, v29);
        let v31: Rc<UH0> = split_list_2(v16, v2.clone());
        let v37: Rc<str> = match Rc::unwrap_or_clone(v31) {
            UH0::UH0_Cons(v32, v33) => {
                let v32: Rc<str> = v32;
                let v33: Rc<UH0> = v33;
                trim_dashes_6(v32)
            }
            UH0::UH0_Nil => {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v35
            }
        };
        let v39: Rc<str> = string_concat(&["rig,gap,", &*v37, ",team:"]);
        let v40: Rc<UH0> = split_list_2(v18.clone(), v6.clone());
        let v45: Rc<str> = match Rc::unwrap_or_clone(v40) {
            UH0::UH0_Cons(v41, v42) => {
                let v41: Rc<str> = v41;
                let v42: Rc<UH0> = v42;
                v41
            }
            UH0::UH0_Nil => {
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v43
            }
        };
        let v46: Rc<str> = string_concat(&[&*v39, &*v45]);
        let v48: Rc<str> = string_concat(&["FAILED CALL: verb=", &*v2, " exit="]);
        let v49: bool = &*v3 == "";
        let v50: Rc<str> = if v49 {
            v18.clone()
        } else {
            v3
        };
        let v52: Rc<str> = string_concat(&[&*v48, &*v50, " cwd="]);
        let v53: bool = &*v5 == "";
        let v54: Rc<str> = if v53 {
            v18.clone()
        } else {
            v5
        };
        let v56: Rc<str> = string_concat(&[&*v52, &*v54, " argv="]);
        let v57: bool = &*v4 == "";
        let v58: Rc<str> = if v57 {
            v18.clone()
        } else {
            v4
        };
        let v62: Rc<str> = string_concat(&[&*v56, &*v58, ". DoD: the call works in one step, or refuses with intent, usage, valid values and a corrected example. Next step: reproduce the call, fix it in the lane that hit it, add a spec/one-call case. TypeFeatures: none yet."]);
        let v63: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v66: Rc<str> = concat5_8(v0, v64, v65.clone(), v65.clone(), v65.clone());
        let v67: bool = std::path::Path::new(&*v66).is_file();
        let v70: US0 = if v67 {
            US0::US0_FileReadable(v66.clone())
        } else {
            US0::US0_FileMissing(v66.clone())
        };
        match v70 {
            US0::US0_FileMissing(v71) => {
                let v71: Rc<str> = v71;
                let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v75: Rc<str> = concat5_8(v73, v30, v74, v72, v65);
                let v76: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v75 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v76
            }
            US0::US0_FileReadable(v77) => {
                let v77: Rc<str> = v77;
                let v78: Rc<str> = std::fs::read_to_string(&*v77).unwrap_or_default().into();
                let mut v79: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v78);
                let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                let v82: Rc<str> = concat5_8(v80, v30.clone(), v81, v65.clone(), v65.clone());
                let v83: bool = v79.contains(&*v82);
                if v83 {
                    let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task with this id already exists"); } LIT.with(|lit| lit.clone()) };
                    let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let v87: Rc<str> = concat5_8(v85, v30, v86, v84, v65);
                    let v88: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v87 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v88
                } else {
                    let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v90: Rc<str> = Rc::<str>::from(v30.replace(&*v18, &*v89));
                    let v91: Rc<str> = std::rc::Rc::<str>::from(v90.to_ascii_lowercase());
                    let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                    let v95: Rc<str> = agile_task_creation_text::rig_agile_task_line(&v30, &v91, &v92, &v93, &v94, &v1, &v46, &v65);
                    let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v97: Rc<str> = std::rc::Rc::<str>::from(v79.trim_end_matches(&*v96));
                    let v100: Rc<str> = string_concat(&[&*v97, "\n", &*v95, "\n"]);
                    let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("30"); } LIT.with(|lit| lit.clone()) };
                    let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("light"); } LIT.with(|lit| lit.clone()) };
                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agent"); } LIT.with(|lit| lit.clone()) };
                    let v104: US1 = extras_applied_9(v100, v30.clone(), v101.clone(), v102.clone(), v103.clone(), &v65, v62.clone(), v6);
                    match v104 {
                        US1::US1_AddedRefused(v105) => {
                            let v105: Rc<str> = v105;
                            let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                            let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                            let v108: Rc<str> = concat5_8(v106, v30, v107, v105, v65);
                            let v109: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v108 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v109
                        }
                        US1::US1_AddedText(v110) => {
                            let v110: Rc<str> = v110;
                            let v111: US2 = replace_records_unchanged_10(v66, v79.clone(), v110.clone());
                            match &v111 {
                                US2::US2_Replaced => {
                                    let mut v112: Rc<str> = Rc::<str>::from(v30.replace(&*v18, &*v89));
                                    let v113: Rc<str> = std::rc::Rc::<str>::from(v112.to_ascii_lowercase());
                                    let v114: u64 = v79.len() as u64;
                                    let v115: u64 = v110.len() as u64;
                                    let v116: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v63);
                                    let v117: Rc<str> = agile_task_creation_text::rig_agile_add_receipt(&v30, &v113, &v101, &v102, &v103, &v65, &v62);
                                    let v118: Rc<str> = std::rc::Rc::<str>::from((v114).to_string());
                                    let v121: Rc<str> = std::rc::Rc::<str>::from((v115).to_string());
                                    let v124: Rc<str> = std::rc::Rc::<str>::from((v116).to_string());
                                    let v126: Rc<str> = string_concat(&[&*v117, " bytes_before=", &*v118, " bytes_after=", &*v121, " elapsed_ms=", &*v124]);
                                    let v127: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v126.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                    v127
                                }
                                _ => {
                                    let v137: Rc<str> = match v111 {
                                        US2::US2_ChangedSinceRead => {
                                            let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                            v132
                                        }
                                        US2::US2_ReadbackMismatch => {
                                            let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                            v131
                                        }
                                        US2::US2_RenameFailed => {
                                            let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                            v129
                                        }
                                        US2::US2_RenameFailedTemporaryKept => {
                                            let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                            v130
                                        }
                                        US2::US2_TemporaryWriteFailed => {
                                            let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                            v128
                                        }
                                        _ => unreachable!(),
                                    };
                                    let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                    let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                    let v140: Rc<str> = concat5_8(v138, v30, v139, v137, v65);
                                    let v141: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v140 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v141
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn command_line_argument_count_13() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_14(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_12(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_13();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US3 = if v4 {
        let v5: Rc<str> = command_line_argument_14(v0);
        US3::US3_Some(v5)
    } else {
        US3::US3_None
    };
    match v8 {
        US3::US3_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US3::US3_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_12(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn flag_of_word_16(mut v0: Rc<str>) -> US5 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Verb"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(v1.to_lowercase());
    let v3: Rc<str> = string_concat(&["--", &*v2]);
    let v4: bool = &*v0 == &*v3;
    if v4 {
        let v5: US6 = US6::US6_FlagVerb;
        US5::US5_FlagKnown(v5)
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Exit"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
        let mut v9: Rc<str> = string_concat(&["--", &*v8]);
        let v10: bool = &*v0 == &*v9;
        if v10 {
            let v11: US6 = US6::US6_FlagExit;
            US5::US5_FlagKnown(v11)
        } else {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Argv"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = Rc::<str>::from(v13.to_lowercase());
            let v15: Rc<str> = string_concat(&["--", &*v14]);
            let v16: bool = &*v0 == &*v15;
            if v16 {
                let v17: US6 = US6::US6_FlagArgv;
                US5::US5_FlagKnown(v17)
            } else {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Cwd"); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = Rc::<str>::from(v19.to_lowercase());
                let v21: Rc<str> = string_concat(&["--", &*v20]);
                let v22: bool = &*v0 == &*v21;
                if v22 {
                    let v23: US6 = US6::US6_FlagCwd;
                    US5::US5_FlagKnown(v23)
                } else {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = Rc::<str>::from(v25.to_lowercase());
                    let v27: Rc<str> = string_concat(&["--", &*v26]);
                    let v28: bool = &*v0 == &*v27;
                    if v28 {
                        let v29: US6 = US6::US6_FlagAs;
                        US5::US5_FlagKnown(v29)
                    } else {
                        US5::US5_FlagUnknown
                    }
                }
            }
        }
    }
}
fn request_from_15(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> US4 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v8, v9) => {
                let v8: Rc<str> = v8;
                let mut v9: Rc<UH0> = v9;
                let v10: US5 = flag_of_word_16(v8.clone());
                match v10 {
                    US5::US5_FlagKnown(v41) => {
                        let v41: US6 = v41;
                        match Rc::unwrap_or_clone(v9) {
                            UH0::UH0_Cons(v44, v45) => {
                                let v44: Rc<str> = v44;
                                let v45: Rc<UH0> = v45;
                                let v46: Rc<str> = Rc::<str>::from((v44).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v41 {
                                    US6::US6_FlagVerb => {
                                        (v1, v46, v3, v4, v5, v6)
                                    }
                                    _ => {
                                        let (mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v41 {
                                            US6::US6_FlagExit => {
                                                (v1, v2, v46, v4, v5, v6)
                                            }
                                            _ => {
                                                let (mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v41 {
                                                    US6::US6_FlagArgv => {
                                                        (v1, v2, v3, v46, v5, v6)
                                                    }
                                                    _ => {
                                                        let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v41 {
                                                            US6::US6_FlagCwd => {
                                                                (v1, v2, v3, v4, v46, v6)
                                                            }
                                                            _ => {
                                                                let (mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v41 {
                                                                    US6::US6_FlagAs => {
                                                                        (v1, v2, v3, v4, v5, v46)
                                                                    }
                                                                    _ => unreachable!(),
                                                                };
                                                                (v47, v48, v49, v50, v51, v52)
                                                            }
                                                        };
                                                        (v53, v54, v55, v56, v57, v58)
                                                    }
                                                };
                                                (v59, v60, v61, v62, v63, v64)
                                            }
                                        };
                                        (v65, v66, v67, v68, v69, v70)
                                    }
                                };
                                (v0, v1, v2, v3, v4, v5, v6) = (v45, v71, v72, v73, v74, v75, v76);
                                continue;
                            }
                            UH0::UH0_Nil => {
                                let v42: Rc<str> = string_concat(&[&*v8, " needs a value"]);
                                return US4::US4_RequestRefused(v42);
                            }
                        }
                    }
                    US5::US5_FlagUnknown => {
                        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Verb"); } LIT.with(|lit| lit.clone()) };
                        let v14: Rc<str> = Rc::<str>::from(v13.to_lowercase());
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Exit"); } LIT.with(|lit| lit.clone()) };
                        let v19: Rc<str> = Rc::<str>::from(v18.to_lowercase());
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Argv"); } LIT.with(|lit| lit.clone()) };
                        let v24: Rc<str> = Rc::<str>::from(v23.to_lowercase());
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Cwd"); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = Rc::<str>::from(v28.to_lowercase());
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                        let v34: Rc<str> = Rc::<str>::from(v33.to_lowercase());
                        let v39: Rc<str> = string_concat(&["unknown option ", &*v8, " (expected --", &*v14, ", --", &*v19, ", --", &*v24, ", --", &*v29, ", --", &*v34, ")"]);
                        return US4::US4_RequestRefused(v39);
                    }
                }
            }
            UH0::UH0_Nil => {
                return US4::US4_RequestRead(v1, v2, v3, v4, v5, v6);
            }
        }
    }
}
fn bytes_within_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v1.clone().len() as i32);
        let v4: bool = v3 <= v2;
        if v4 {
            return true;
        } else {
            let v5: usize = ((v2) as usize);
            let v6: i32 = v2.wrapping_add(1i32);
            let v7: usize = ((v6) as usize);
            let v8: Rc<str> = Rc::<str>::from(&(v1)[v5..v7]);
            let mut v9: bool = v0.contains(&*v8);
            if v9 {
                v2 = v6;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn request_checked_17(mut v0: US4) -> US4 {
    match &v0 {
        US4::US4_RequestRead(v2, v3, v4, v5, v6, v7) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<str> = v3.clone();
            let v4: Rc<str> = v4.clone();
            let v5: Rc<str> = v5.clone();
            let v6: Rc<str> = v6.clone();
            let v7: Rc<str> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<UH0> = split_list_2(v8, v7);
            let v14: Rc<str> = match Rc::unwrap_or_clone(v9) {
                UH0::UH0_Cons(v10, v11) => {
                    let v10: Rc<str> = v10;
                    let v11: Rc<UH0> = v11;
                    v10
                }
                UH0::UH0_Nil => {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v12
                }
            };
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyz0123456789- "); } LIT.with(|lit| lit.clone()) };
            let v16: i32 = 0i32;
            let v17: bool = bytes_within_18(v15, v3.clone(), v16);
            let v18: bool = &*v14 != "";
            let v22: bool = if v18 {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyz0123456789"); } LIT.with(|lit| lit.clone()) };
                let v20: i32 = 0i32;
                bytes_within_18(v19, v14, v20)
            } else {
                false
            };
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) };
            let v24: i32 = 0i32;
            let v25: bool = bytes_within_18(v23, v4, v24);
            let v26: bool = &*v2 == "";
            if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a gap needs its title: what the call needed and did not get"); } LIT.with(|lit| lit.clone()) };
                US4::US4_RequestRefused(v27)
            } else {
                let v29: bool = &*v3 == "";
                if v29 {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--verb names the rig verb that failed or was missing, e.g. --verb 'job chain'"); } LIT.with(|lit| lit.clone()) };
                    US4::US4_RequestRefused(v30)
                } else {
                    let v32: bool = v17 == false;
                    if v32 {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--verb takes lower-case words, digits and dashes, e.g. --verb 'help agile add'"); } LIT.with(|lit| lit.clone()) };
                        US4::US4_RequestRefused(v33)
                    } else {
                        let v35: bool = v22 == false;
                        if v35 {
                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as <lane> names who hit the gap; its first word is the team, e.g. --as rig-fc1568a6"); } LIT.with(|lit| lit.clone()) };
                            US4::US4_RequestRefused(v36)
                        } else {
                            let v38: bool = v25 == false;
                            if v38 {
                                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--exit takes the failed call's exit code as digits, e.g. --exit 2"); } LIT.with(|lit| lit.clone()) };
                                US4::US4_RequestRefused(v39)
                            } else {
                                v0
                            }
                        }
                    }
                }
            }
        }
        US4::US4_RequestRefused(v1) => {
            let v1: Rc<str> = v1.clone();
            v0
        }
    }
}
fn gap_entry_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_12(v0);
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
                            let v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH0::UH0_Cons(v8, v9) => {
                                    let v8: Rc<str> = v8;
                                    let mut v9: Rc<UH0> = v9;
                                    let v10: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v12: US4 = request_from_15(v9, v10, v11.clone(), v11.clone(), v11.clone(), v11.clone(), v11.clone());
                                    let v13: US4 = request_checked_17(v12);
                                    match v13 {
                                        US4::US4_RequestRead(v17, v18, v19, v20, v21, v22) => {
                                            let v17: Rc<str> = v17;
                                            let v18: Rc<str> = v18;
                                            let v19: Rc<str> = v19;
                                            let v20: Rc<str> = v20;
                                            let v21: Rc<str> = v21;
                                            let v22: Rc<str> = v22;
                                            gap_filed_1(v6, v17, v18, v19, v20, v21, v22)
                                        }
                                        US4::US4_RequestRefused(v14) => {
                                            let v14: Rc<str> = v14;
                                            let v15: Rc<str> = string_concat(&["agile gap rejected: ", &*v14]);
                                            let v16: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v15 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                            v16
                                        }
                                    }
                                }
                                _ => {
                                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile gap <root> <title> --verb <verb> --as <lane> [--exit <n>] [--argv <text>] [--cwd <dir>]"); } LIT.with(|lit| lit.clone()) };
                                    let v27: Rc<str> = string_concat(&["agile gap rejected: ", &*v26]);
                                    let v28: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v27 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v28
                                }
                            }
                        }
                        _ => {
                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile gap <root> <title> --verb <verb> --as <lane> [--exit <n>] [--argv <text>] [--cwd <dir>]"); } LIT.with(|lit| lit.clone()) };
                            let v31: Rc<str> = string_concat(&["agile gap rejected: ", &*v30]);
                            let v32: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v31 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v32
                        }
                    }
                }
                _ => {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile gap <root> <title> --verb <verb> --as <lane> [--exit <n>] [--argv <text>] [--cwd <dir>]"); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = string_concat(&["agile gap rejected: ", &*v34]);
                    let v36: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v35 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v36
                }
            }
        }
        _ => {
            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile gap <root> <title> --verb <verb> --as <lane> [--exit <n>] [--argv <text>] [--cwd <dir>]"); } LIT.with(|lit| lit.clone()) };
            let v39: Rc<str> = string_concat(&["agile gap rejected: ", &*v38]);
            let v40: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v39 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v40
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        gap_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_gap_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
