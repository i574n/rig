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
    US1_StateWritten,
    US1_StateChangedSinceRead,
    US1_StateWriteFailed(Rc<str>),
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
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn declared_from_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
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
                let v9: i32 = v3.wrapping_add(1i32);
                v2 = v9;
                continue;
            }
        }
    }
}
fn binding_declared_4(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v3: Rc<str> = string_concat(&["\ninl ", &*v1, " ("]);
    let v5: Rc<str> = string_concat(&["inl ", &*v1, " ("]);
    let v6: bool = v0.starts_with(&*v5);
    if v6 {
        true
    } else {
        let v7: i32 = v0.find(&*v3).map(|offset| offset as i32).unwrap_or(-1);
        let v8: bool = v7 < 0i32;
        let v9: bool = v8 == false;
        v9
    }
}
fn renamed_within_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn state_written_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US1 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_2(v0.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    std::env::var("KIT_AGILE_TEST_PAUSE_MS").ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
    if v6 {
        let v7: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let v8: bool = std::fs::write(&*v7, &*v2).is_ok();
        let v21: US2 = if v8 {
            let v9: i32 = 40i32;
            let v10: bool = renamed_within_6(v0.clone(), v7.clone(), v9);
            if v10 {
                let v11: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v11 {
                    US2::US2_Replaced
                } else {
                    US2::US2_ReadbackMismatch
                }
            } else {
                let v15: bool = std::fs::remove_file(&*v7).is_ok() || !std::path::Path::new(&*v7).exists();
                if v15 {
                    US2::US2_RenameFailed
                } else {
                    US2::US2_RenameFailedTemporaryKept
                }
            }
        } else {
            US2::US2_TemporaryWriteFailed
        };
        match &v21 {
            US2::US2_Replaced => {
                US1::US1_StateWritten
            }
            _ => {
                let v32: Rc<str> = match v21 {
                    US2::US2_ChangedSinceRead => {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v27
                    }
                    US2::US2_ReadbackMismatch => {
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v26
                    }
                    US2::US2_RenameFailed => {
                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v24
                    }
                    US2::US2_RenameFailedTemporaryKept => {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v25
                    }
                    US2::US2_TemporaryWriteFailed => {
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v23
                    }
                    _ => unreachable!(),
                };
                US1::US1_StateWriteFailed(v32)
            }
        }
    } else {
        US1::US1_StateChangedSinceRead
    }
}
fn renamed_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_2(v0, v4, v5.clone(), v5.clone(), v5.clone());
    let v7: bool = std::path::Path::new(&*v6).is_file();
    let v10: US0 = if v7 {
        US0::US0_FileReadable(v6.clone())
    } else {
        US0::US0_FileMissing(v6.clone())
    };
    match v10 {
        US0::US0_FileMissing(v11) => {
            let v11: Rc<str> = v11;
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v12]);
            let v16: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v15 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v16
        }
        US0::US0_FileReadable(v17) => {
            let v17: Rc<str> = v17;
            let v18: Rc<str> = std::fs::read_to_string(&*v17).unwrap_or_default().into();
            let v19: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v18);
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = concat5_2(v20.clone(), v21.clone(), v1.clone(), v22.clone(), v5.clone());
            let v24: i32 = 0i32;
            let v25: i32 = declared_from_3(v19.clone(), v23, v24);
            let v26: bool = v25 < 0i32;
            let v27: bool = v26 == false;
            let v28: bool = v27 == false;
            if v28 {
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v29]);
                let v33: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v32 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v33
            } else {
                let v34: Rc<str> = concat5_2(v20, v21, v2.clone(), v22, v5.clone());
                let v35: i32 = 0i32;
                let v36: i32 = declared_from_3(v19.clone(), v34, v35);
                let v37: bool = v36 < 0i32;
                let v38: bool = v37 == false;
                if v38 {
                    let v43: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: the id ", &*v2, " is already taken"]);
                    let v44: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v43 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v44
                } else {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v47: Rc<str> = Rc::<str>::from(v2.replace(&*v45, &*v46));
                    let v48: Rc<str> = std::rc::Rc::<str>::from(v47.to_ascii_lowercase());
                    let v49: bool = binding_declared_4(v19.clone(), v48);
                    if v49 {
                        let mut v50: Rc<str> = Rc::<str>::from(v2.replace(&*v45, &*v46));
                        let v51: Rc<str> = std::rc::Rc::<str>::from(v50.to_ascii_lowercase());
                        let v56: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: the name inl ", &*v51, " is already declared"]);
                        let v57: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v56 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v57
                    } else {
                        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning_of_"); } LIT.with(|lit| lit.clone()) };
                        let mut v59: Rc<str> = Rc::<str>::from(v2.replace(&*v45, &*v46));
                        let v60: Rc<str> = std::rc::Rc::<str>::from(v59.to_ascii_lowercase());
                        let v61: Rc<str> = concat5_2(v58.clone(), v60, v5.clone(), v5.clone(), v5.clone());
                        let v62: bool = binding_declared_4(v19.clone(), v61);
                        if v62 {
                            let mut v63: Rc<str> = Rc::<str>::from(v2.replace(&*v45, &*v46));
                            let v64: Rc<str> = std::rc::Rc::<str>::from(v63.to_ascii_lowercase());
                            let v65: Rc<str> = concat5_2(v58, v64, v5.clone(), v5.clone(), v5.clone());
                            let v70: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: the name inl ", &*v65, " is already declared"]);
                            let v71: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v70 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v71
                        } else {
                            let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77): (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) = agile_rename_rewrite::rig_agile_renamed_text(&*v19, &*v1, &*v2);
                            let v78: US1 = state_written_5(v6, v18, v72.clone());
                            match &v78 {
                                US1::US1_StateWritten => {
                                    let v79: u64 = v19.len() as u64;
                                    let v80: u64 = v72.len() as u64;
                                    let v81: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
                                    let v85: Rc<str> = string_concat(&["kit agile rename ok old=", &*v1, " new=", &*v2, " items="]);
                                    let v86: Rc<str> = std::rc::Rc::<str>::from((v73).to_string());
                                    let v88: Rc<str> = std::rc::Rc::<str>::from((v74).to_string());
                                    let v90: Rc<str> = string_concat(&[&*v86, " deps_rewritten=", &*v88, " deps_in="]);
                                    let v91: bool = &*v75 == "";
                                    let v92: Rc<str> = if v91 {
                                        v45.clone()
                                    } else {
                                        v75
                                    };
                                    let v93: Rc<str> = string_concat(&[&*v90, &*v92]);
                                    let v94: Rc<str> = std::rc::Rc::<str>::from((v76).to_string());
                                    let v96: Rc<str> = string_concat(&[" titles_mentioning_old=", &*v94, " titles_in="]);
                                    let v97: bool = &*v77 == "";
                                    let v98: Rc<str> = if v97 {
                                        v45
                                    } else {
                                        v77
                                    };
                                    let v102: Rc<str> = std::rc::Rc::<str>::from((v79).to_string());
                                    let v105: Rc<str> = std::rc::Rc::<str>::from((v80).to_string());
                                    let v108: Rc<str> = std::rc::Rc::<str>::from((v81).to_string());
                                    let v110: Rc<str> = string_concat(&[&*v85, &*v93, &*v96, &*v98, " bytes_before=", &*v102, " bytes_after=", &*v105, " elapsed_ms=", &*v108]);
                                    let v111: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v110.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                    v111
                                }
                                _ => {
                                    let v115: Rc<str> = match v78 {
                                        US1::US1_StateChangedSinceRead => {
                                            let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                            v112
                                        }
                                        US1::US1_StateWriteFailed(v113) => {
                                            let v113: Rc<str> = v113;
                                            v113
                                        }
                                        _ => unreachable!(),
                                    };
                                    let v118: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v115]);
                                    let v119: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v118 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v119
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn command_line_argument_count_8() -> i32 {
    let v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_7() -> i32 {
    let v0: i32 = command_line_argument_count_8();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn id_bytes_valid_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let v3: bool = v2 <= v1;
        if v3 {
            return true;
        } else {
            let v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v5: bool = 65i32 <= v4;
            let v11: bool = if v5 {
                let v6: bool = v4 <= 90i32;
                v6
            } else {
                let v7: bool = 48i32 <= v4;
                if v7 {
                    let v8: bool = v4 <= 57i32;
                    v8
                } else {
                    let v9: bool = v4 == 45i32;
                    v9
                }
            };
            if v11 {
                let v12: i32 = v1.wrapping_add(1i32);
                v1 = v12;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn rename_entry_0() -> i32 {
    let v0: i32 = 2i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 3i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v4: i32 = argument_count_7();
    let v5: bool = v4 == 4i32;
    let v6: bool = v5 == false;
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile rename <root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v7]);
        let v11: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v10 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v11
    } else {
        let v12: i32 = (v3.clone().len() as i32);
        let v13: bool = v12 <= 0i32;
        let v16: bool = if v13 {
            false
        } else {
            let v14: i32 = 0i32;
            id_bytes_valid_9(v3.clone(), v14, v12)
        };
        let v17: bool = v16 == false;
        if v17 {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task id is upper-case letters, digits and -"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v18]);
            let v22: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v21 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v22
        } else {
            let v23: i32 = v3.as_bytes().get(0i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v24: bool = 48i32 <= v23;
            let v26: bool = if v24 {
                let v25: bool = v23 <= 57i32;
                v25
            } else {
                false
            };
            if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the new id must start with a letter, so its inl name is a valid identifier"); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v27]);
                let v31: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v30 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v31
            } else {
                let v32: bool = &*v1 == &*v3;
                if v32 {
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the new id equals the old one"); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = string_concat(&["agile rename ", &*v1, " rejected: ", &*v33]);
                    let v37: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v36 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v37
                } else {
                    let v38: i32 = 1i32;
                    let v39: Rc<str> = usize::try_from(v38).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    renamed_1(v39, v1, v3)
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        rename_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_rename_run() -> i32 {
    closure0()()
}
