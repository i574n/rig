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
    US0_MainAgent,
    US0_Subagent,
}
#[derive(Clone)]
enum US1 {
    US1_FileReadable(Rc<str>),
    US1_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_ScanPlain,
    US2_ScanQuoted,
    US2_ScanEscaped,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoTasks,
    UH0_TaskThen(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_NoRanked,
    UH1_RankedThen(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32 }
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn field_end_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US2) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = match &v3 {
                US2::US2_ScanPlain => {
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
                let v25: US2 = match v3 {
                    US2::US2_ScanEscaped => {
                        US2::US2_ScanQuoted
                    }
                    US2::US2_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US2::US2_ScanQuoted
                        } else {
                            US2::US2_ScanPlain
                        }
                    }
                    US2::US2_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US2::US2_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US2::US2_ScanPlain
                            } else {
                                US2::US2_ScanQuoted
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
fn skip_spaces_6(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let mut v3: bool = v2 == 32i32;
        if v3 {
            let v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
            continue;
        } else {
            return v1;
        }
    }
}
fn top_level_field_count_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: US2 = US2::US2_ScanPlain;
        let v5: i32 = field_end_5(v0.clone(), v1, v2, v4);
        let v6: bool = v2 <= v5;
        if v6 {
            return -1i32;
        } else {
            let v7: i32 = v0.as_bytes().get(v5 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v8: bool = v7 == 41i32;
            if v8 {
                let v9: i32 = v3.wrapping_add(1i32);
                return v9;
            } else {
                let v10: i32 = v5.wrapping_add(1i32);
                let v11: i32 = skip_spaces_6(v0.clone(), v10);
                let v12: i32 = v3.wrapping_add(1i32);
                (v1, v3) = (v11, v12);
                continue;
            }
        }
    }
}
fn first_malformed_task_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return -1i32;
        } else {
            let v6: i32 = v4.wrapping_add(6i32);
            let v7: i32 = v0.get(..v4 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v7 as usize..v4 as usize).unwrap_or(""));
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v10: bool = v8.starts_with(&*v9);
            if v10 {
                let v11: i32 = 0i32;
                let v12: i32 = top_level_field_count_4(v0.clone(), v6, v2, v11);
                let v13: bool = v12 == 11i32;
                if v13 {
                    v1 = v6;
                    continue;
                } else {
                    return v4;
                }
            } else {
                v1 = v6;
                continue;
            }
        }
    }
}
fn field_start_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US2 = US2::US2_ScanPlain;
            let v6: i32 = field_end_5(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_6(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn backlog_task_at_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>) {
    let mut v3: i32 = 0i32;
    let v4: i32 = field_start_9(v0.clone(), v1, v2, v3);
    let v5: US2 = US2::US2_ScanPlain;
    let v6: i32 = field_end_5(v0.clone(), v4, v2, v5);
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let v8: i32 = (v7.clone().len() as i32);
    let v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let v11: i32 = 1i32;
    let v12: i32 = field_start_9(v0.clone(), v1, v2, v11);
    let v13: US2 = US2::US2_ScanPlain;
    let v14: i32 = field_end_5(v0.clone(), v12, v2, v13);
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: i32 = (v15.clone().len() as i32);
    let v17: i32 = v16.wrapping_sub(1i32);
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v15.get(1i32 as usize..v17 as usize).unwrap_or(""));
    let mut v19: i32 = 2i32;
    let v20: i32 = field_start_9(v0.clone(), v1, v2, v19);
    let v21: US2 = US2::US2_ScanPlain;
    let v22: i32 = field_end_5(v0.clone(), v20, v2, v21);
    let v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let v24: i32 = 3i32;
    let v25: i32 = field_start_9(v0.clone(), v1, v2, v24);
    let v26: US2 = US2::US2_ScanPlain;
    let v27: i32 = field_end_5(v0.clone(), v25, v2, v26);
    let v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let v29: i32 = (v28).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
    let v30: i32 = 4i32;
    let v31: i32 = field_start_9(v0.clone(), v1, v2, v30);
    let v32: US2 = US2::US2_ScanPlain;
    let v33: i32 = field_end_5(v0.clone(), v31, v2, v32);
    let v34: Rc<str> = std::rc::Rc::<str>::from(v0.get(v31 as usize..v33 as usize).unwrap_or(""));
    let v35: i32 = (v34).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
    let v36: i32 = 6i32;
    let v37: i32 = field_start_9(v0.clone(), v1, v2, v36);
    let v38: US2 = US2::US2_ScanPlain;
    let v39: i32 = field_end_5(v0.clone(), v37, v2, v38);
    let v40: Rc<str> = std::rc::Rc::<str>::from(v0.get(v37 as usize..v39 as usize).unwrap_or(""));
    let v41: i32 = 7i32;
    let v42: i32 = field_start_9(v0.clone(), v1, v2, v41);
    let v43: US2 = US2::US2_ScanPlain;
    let v44: i32 = field_end_5(v0.clone(), v42, v2, v43);
    let v45: Rc<str> = std::rc::Rc::<str>::from(v0.get(v42 as usize..v44 as usize).unwrap_or(""));
    let v46: i32 = (v45.clone().len() as i32);
    let v47: i32 = v46.wrapping_sub(1i32);
    let v48: Rc<str> = std::rc::Rc::<str>::from(v45.get(1i32 as usize..v47 as usize).unwrap_or(""));
    let v49: i32 = 8i32;
    let v50: i32 = field_start_9(v0.clone(), v1, v2, v49);
    let v51: US2 = US2::US2_ScanPlain;
    let v52: i32 = field_end_5(v0.clone(), v50, v2, v51);
    let v53: Rc<str> = std::rc::Rc::<str>::from(v0.get(v50 as usize..v52 as usize).unwrap_or(""));
    let v54: i32 = (v53.clone().len() as i32);
    let v55: i32 = v54.wrapping_sub(1i32);
    let v56: Rc<str> = std::rc::Rc::<str>::from(v53.get(1i32 as usize..v55 as usize).unwrap_or(""));
    let v57: i32 = agile_planning_fields::rig_agile_planning_minutes_near(&v0, &v10, v1);
    let v58: i32 = 5i32;
    let v59: i32 = field_start_9(v0.clone(), v1, v2, v58);
    let v60: US2 = US2::US2_ScanPlain;
    let v61: i32 = field_end_5(v0.clone(), v59, v2, v60);
    let v62: Rc<str> = std::rc::Rc::<str>::from(v0.get(v59 as usize..v61 as usize).unwrap_or(""));
    let v63: i32 = (v62).trim().trim_end_matches("u32").parse::<i32>().unwrap_or(0);
    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v10, v18, v23, v29, v35, v40, v48, v56, v57, v63, 0i32, false, v64)
}
fn tasks_from_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v4)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v6: bool = v5 < 0i32;
        if v6 {
            return v3;
        } else {
            let v7: i32 = v5.wrapping_add(6i32);
            let v8: i32 = v0.get(..v5 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v5 as usize).unwrap_or(""));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v11: bool = v9.starts_with(&*v10);
            if v11 {
                let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24): (Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>) = backlog_task_at_8(v0.clone(), v7, v2);
                let v25: Rc<UH0> = Rc::new(UH0::UH0_TaskThen(v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v3));
                (v1, v3) = (v7, v25);
                continue;
            } else {
                v1 = v7;
                continue;
            }
        }
    }
}
fn annotated_from_11(mut v0: Rc<RefCell<Vec<(Rc<str>, i32, Rc<str>, bool)>>>, mut v1: Rc<UH0>, mut v2: i32) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_NoTasks => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoTasks); } CASE.with(|case| case.clone()) }
        }
        UH0::UH0_TaskThen(v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17) => {
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: i32 = v7;
            let v8: i32 = v8;
            let v9: Rc<str> = v9;
            let v10: Rc<str> = v10;
            let v11: Rc<str> = v11;
            let v12: i32 = v12;
            let v13: i32 = v13;
            let v14: i32 = v14;
            let v15: bool = v15;
            let v16: Rc<str> = v16;
            let v17: Rc<UH0> = v17;
            let (mut v18, mut v19, mut v20, mut v21): (Rc<str>, i32, Rc<str>, bool) = v0.borrow()[v2 as usize].clone();
            let v22: i32 = v2.wrapping_add(1i32);
            let v23: Rc<UH0> = annotated_from_11(v0, v17, v22);
            Rc::new(UH0::UH0_TaskThen(v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v19, v21, v20, v23))
        }
    }
}
fn ids_text_12(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_NoTasks => {
                return v1;
            }
            UH0::UH0_TaskThen(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: i32 = v5;
                let v6: i32 = v6;
                let v7: Rc<str> = v7;
                let v8: Rc<str> = v8;
                let v9: Rc<str> = v9;
                let v10: i32 = v10;
                let v11: i32 = v11;
                let v12: i32 = v12;
                let v13: bool = v13;
                let v14: Rc<str> = v14;
                let v15: Rc<UH0> = v15;
                let v16: bool = &*v1 == "";
                let mut v19: Rc<str> = if v16 {
                    v2
                } else {
                    let mut v18: Rc<str> = string_concat(&[&*v1, "\n", &*v2]);
                    v18
                };
                (v0, v1) = (v15, v19);
                continue;
            }
        }
    }
}
fn annotated_tasks_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i64, mut v3: Rc<UH0>) -> Rc<UH0> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = ids_text_12(v3.clone(), v4);
    let v6: Rc<RefCell<Vec<(Rc<str>, i32, Rc<str>, bool)>>> = agile_task_annotation::rig_agile_task_annotations(&v0, &v1, &v5, v2);
    let v7: i32 = 0i32;
    annotated_from_11(v6, v3, v7)
}
fn load_words_13(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    v2
}
fn loads_pass_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<RefCell<Vec<Rc<str>>>>, mut v3: i32) -> bool {
    loop {
        let v4: i32 = (v2.clone().borrow().len() as i32);
        let v5: bool = v4 <= v3;
        if v5 {
            return false;
        } else {
            let v6: Rc<str> = v2.borrow()[v3 as usize].clone();
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            let v8: bool = agile_planning_filters::rig_agile_planning_selected(&v0, &v1, &v6, &v7);
            if v8 {
                return true;
            } else {
                let v9: i32 = v3.wrapping_add(1i32);
                v3 = v9;
                continue;
            }
        }
    }
}
fn listed_at_15(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v2 >= v3;
        if v4 {
            return false;
        } else {
            let v5: Rc<str> = v0.borrow()[v2 as usize].clone();
            let v6: bool = &*v5 == &*v1;
            if v6 {
                return true;
            } else {
                let v7: i32 = v2.wrapping_add(1i32);
                v2 = v7;
                continue;
            }
        }
    }
}
fn closure1(mut v0: US0, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: Rc<str>, mut v6: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>) -> bool> {
    Rc::new(move |mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: i32, mut v11: i32, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: i32, mut v16: i32, mut v17: i32, mut v18: bool, mut v19: Rc<str>| -> bool {
        let v20: bool = &*v2 == "";
        let v22: bool = if v20 {
            true
        } else {
            let v21: bool = agile_task_query::rig_query_passes(&*v2, &*v7, &*v8, &*v14, &*v12, true);
            v21
        };
        let v26: bool = if v22 {
            let v23: Rc<RefCell<Vec<Rc<str>>>> = load_words_13(v3.clone());
            let v24: i32 = 0i32;
            loads_pass_14(v5.clone(), v7.clone(), v23, v24)
        } else {
            false
        };
        if v26 {
            let v27: i32 = 0i32;
            let v28: bool = listed_at_15(&v6, v7, v27);
            let v29: bool = v28 == false;
            v29
        } else {
            false
        }
    })
}
fn task_open_17(mut v0: &Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: &Rc<str>, mut v7: &Rc<str>, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: &Rc<str>) -> bool {
    let v13: bool = &*v5 == "Planned";
    if v13 {
        true
    } else {
        let v14: bool = &*v5 == "Active";
        if v14 {
            true
        } else {
            let v15: bool = &*v5 == "Blocked";
            if v15 {
                false
            } else {
                let v16: bool = &*v5 == "Paused";
                if v16 {
                    false
                } else {
                    false
                }
            }
        }
    }
}
fn status_of_19(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_NoTasks => {
                let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v2;
            }
            UH0::UH0_TaskThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: i32 = v6;
                let v7: i32 = v7;
                let v8: Rc<str> = v8;
                let v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: i32 = v11;
                let v12: i32 = v12;
                let v13: i32 = v13;
                let v14: bool = v14;
                let v15: Rc<str> = v15;
                let v16: Rc<UH0> = v16;
                let v17: bool = &*v3 == &*v1;
                if v17 {
                    return v8;
                } else {
                    v0 = v16;
                    continue;
                }
            }
        }
    }
}
fn deps_settled_18(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: i32 = (v1.clone().len() as i32);
        let v4: bool = v3 <= v2;
        if v4 {
            return true;
        } else {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v6: i32 = v1.get(v2 as usize..).and_then(|rest| rest.find(&*v5)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let v7: bool = v6 < 0i32;
            let v8: i32 = if v7 {
                v3
            } else {
                v6
            };
            let v9: Rc<str> = std::rc::Rc::<str>::from(v1.get(v2 as usize..v8 as usize).unwrap_or(""));
            let v10: Rc<str> = std::rc::Rc::<str>::from(v9.trim());
            let v11: Rc<str> = status_of_19(v0.clone(), v10);
            let v12: bool = &*v11 == "";
            let v14: bool = if v12 {
                true
            } else {
                let v13: bool = &*v11 == "Done";
                v13
            };
            if v14 {
                let v15: i32 = v8.wrapping_add(1i32);
                v2 = v15;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn open_deps_22(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = (v1.clone().len() as i32);
        let v5: bool = v4 <= v2;
        if v5 {
            return v3;
        } else {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v7: i32 = v1.get(v2 as usize..).and_then(|rest| rest.find(&*v6)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let v8: bool = v7 < 0i32;
            let v9: i32 = if v8 {
                v4
            } else {
                v7
            };
            let v10: Rc<str> = std::rc::Rc::<str>::from(v1.get(v2 as usize..v9 as usize).unwrap_or(""));
            let v11: Rc<str> = std::rc::Rc::<str>::from(v10.trim());
            let v12: Rc<str> = status_of_19(v0.clone(), v11);
            let v13: i32 = v9.wrapping_add(1i32);
            let v14: bool = &*v12 == "";
            let v16: bool = if v14 {
                true
            } else {
                let v15: bool = &*v12 == "Done";
                v15
            };
            let mut v18: i32 = if v16 {
                v3
            } else {
                let v17: i32 = v3.wrapping_add(1i32);
                v17
            };
            (v2, v3) = (v13, v18);
            continue;
        }
    }
}
fn unblocked_weight_21(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_NoTasks => {
                return v2;
            }
            UH0::UH0_TaskThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: Rc<str> = v5.clone();
                let v6: i32 = *v6;
                let v7: i32 = *v7;
                let v8: Rc<str> = v8.clone();
                let v9: Rc<str> = v9.clone();
                let v10: Rc<str> = v10.clone();
                let v11: i32 = *v11;
                let v12: i32 = *v12;
                let v13: i32 = *v13;
                let v14: bool = *v14;
                let v15: Rc<str> = v15.clone();
                let v16: Rc<UH0> = v16.clone();
                let v17: bool = task_open_17(&v3, &v4, &v5, v6, v7, v8, &v9, &v10, v11, v12, v13, v14, &v15);
                let v24: bool = if v17 {
                    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                    let v20: Rc<str> = Rc::<str>::from(v9.replace(&*v18, &*v19));
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v19));
                    let v23: bool = v22.split("|").any(|item| item == &*v1);
                    v23
                } else {
                    false
                };
                let v32: i32 = if v24 {
                    let v25: i32 = 0i32;
                    let v26: i32 = 0i32;
                    let v27: i32 = open_deps_22(v0, v9, v25, v26);
                    let v28: i32 = 1i32 << (2 * (5 - (v6).clamp(0, 5)));
                    let v29: bool = v27 < 1i32;
                    let v30: i32 = if v29 {
                        1i32
                    } else {
                        v27
                    };
                    let v31: i32 = v28.wrapping_div(v30);
                    v31
                } else {
                    0i32
                };
                let v33: i32 = v2.wrapping_add(v32);
                (v0, v2) = (v16, v33);
                continue;
            }
        }
    }
}
fn ranked_of_20(mut v0: US0, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: &Rc<str>, mut v5: i32, mut v6: i32, mut v7: &Rc<str>, mut v8: &Rc<str>, mut v9: &Rc<str>, mut v10: i32, mut v11: i32, mut v12: i32, mut v13: bool, mut v14: Rc<str>) -> (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>) {
    let v15: bool = 0i32 < v10;
    let v33: i32 = if v15 {
        v10
    } else {
        let v16: bool = 0i32 == v6;
        let v31: i32 = if v16 {
            15i32
        } else {
            let v17: bool = 1i32 == v6;
            if v17 {
                30i32
            } else {
                let mut v18: bool = 2i32 == v6;
                if v18 {
                    60i32
                } else {
                    let mut v19: bool = 3i32 == v6;
                    if v19 {
                        180i32
                    } else {
                        let v20: bool = 4i32 == v6;
                        if v20 {
                            300i32
                        } else {
                            let v21: bool = 5i32 == v6;
                            if v21 {
                                600i32
                            } else {
                                let v22: bool = 6i32 == v6;
                                if v22 {
                                    1200i32
                                } else {
                                    let v23: bool = 7i32 == v6;
                                    if v23 {
                                        2400i32
                                    } else {
                                        3600i32
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        let v32: i32 = ((v31 as i64) * (100 - (v11).clamp(0, 100)) as i64 / 100).max(15) as i32;
        v32
    };
    let v34: i32 = 0i32;
    let v35: i32 = unblocked_weight_21(v1, v2.clone(), v34);
    let v36: i32 = 1i32 << (2 * (5 - (v5).clamp(0, 5)));
    let v37: i32 = v36.wrapping_add(v35);
    let v43: i32 = match &v0 {
        US0::US0_MainAgent => {
            let v38: i32 = ((v37 as i64) * 6000 / (v33).max(1) as i64).min(i32::MAX as i64) as i32;
            v38
        }
        US0::US0_Subagent => {
            let v39: i32 = v33.wrapping_mul(8i32);
            let v40: i32 = v39.wrapping_add(5i32);
            let v41: i32 = v40.wrapping_sub(v5);
            v41
        }
    };
    let v55: i32 = match v0 {
        US0::US0_MainAgent => {
            let v44: bool = v12 < 14i32;
            let v45: i32 = if v44 {
                v12
            } else {
                14i32
            };
            let v46: i32 = 7i32.wrapping_add(v45);
            let v47: i32 = if v13 {
                2i32
            } else {
                1i32
            };
            let v48: i32 = v46.wrapping_mul(v47);
            let v49: i32 = 2147483647i32.wrapping_div(v48);
            let v50: bool = v43 > v49;
            if v50 {
                2147483647i32
            } else {
                let v51: i32 = v43.wrapping_mul(v48);
                let v52: i32 = v51.wrapping_div(7i32);
                v52
            }
        }
        US0::US0_Subagent => {
            v43
        }
    };
    (v2, v3, v5, v6, v33, v15, v35, v55, v14)
}
fn inserted_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: bool, mut v6: i32, mut v7: i32, mut v8: Rc<str>, mut v9: Rc<UH1>) -> Rc<UH1> {
    match &*v9 {
        UH1::UH1_NoRanked => {
            let v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRanked); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_RankedThen(v0, v1, v2, v3, v4, v5, v6, v7, v8, v10))
        }
        UH1::UH1_RankedThen(v12, v13, v14, v15, v16, v17, v18, v19, v20, v21) => {
            let v12: Rc<str> = v12.clone();
            let v13: Rc<str> = v13.clone();
            let v14: i32 = *v14;
            let v15: i32 = *v15;
            let v16: i32 = *v16;
            let v17: bool = *v17;
            let mut v18: i32 = *v18;
            let mut v19: i32 = *v19;
            let v20: Rc<str> = v20.clone();
            let v21: Rc<UH1> = v21.clone();
            let v22: bool = v19 < v7;
            if v22 {
                Rc::new(UH1::UH1_RankedThen(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9))
            } else {
                let v24: Rc<UH1> = inserted_23(v0, v1, v2, v3, v4, v5, v6, v7, v8, v21);
                Rc::new(UH1::UH1_RankedThen(v12, v13, v14, v15, v16, v17, v18, v19, v20, v24))
            }
        }
    }
}
fn ranked_from_16(mut v0: US0, mut v1: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>) -> bool>, mut v2: Rc<UH0>, mut v3: Rc<UH0>, mut v4: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH0::UH0_NoTasks => {
                return v4;
            }
            UH0::UH0_TaskThen(v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18) => {
                let v5: Rc<str> = v5;
                let v6: Rc<str> = v6;
                let v7: Rc<str> = v7;
                let v8: i32 = v8;
                let v9: i32 = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<str> = v11;
                let v12: Rc<str> = v12;
                let v13: i32 = v13;
                let v14: i32 = v14;
                let v15: i32 = v15;
                let v16: bool = v16;
                let v17: Rc<str> = v17;
                let mut v18: Rc<UH0> = v18;
                let mut v19: bool = task_open_17(&v5, &v6, &v7, v8, v9, v10.clone(), &v11, &v12, v13, v14, v15, v16, &v17);
                let v23: bool = if v19 {
                    match &v0 {
                        US0::US0_MainAgent => {
                            true
                        }
                        US0::US0_Subagent => {
                            let v20: bool = &*v7 != "Epic";
                            v20
                        }
                    }
                } else {
                    false
                };
                let v32: bool = if v23 {
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user-batch"); } LIT.with(|lit| lit.clone()) };
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = Rc::<str>::from(v12.replace(&*v25, &*v26));
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v29: Rc<str> = Rc::<str>::from(v27.replace(&*v28, &*v26));
                    let v30: bool = v29.split("|").any(|item| item == &*v24);
                    let v31: bool = v30 == false;
                    v31
                } else {
                    false
                };
                let v35: bool = if v32 {
                    let v33: i32 = 0i32;
                    deps_settled_18(v2.clone(), v11.clone(), v33)
                } else {
                    false
                };
                let v37: bool = if v35 {
                    v1(v5.clone(), v6.clone(), v7.clone(), v8, v9, v10.clone(), v11.clone(), v12.clone(), v13, v14, v15, v16, v17.clone())
                } else {
                    false
                };
                if v37 {
                    let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>) = ranked_of_20(v0.clone(), v2.clone(), v5, v6, &v7, v8, v9, &v10, &v11, &v12, v13, v14, v15, v16, v17);
                    let v47: Rc<UH1> = inserted_23(v38, v39, v40, v41, v42, v43, v44, v45, v46, v4);
                    (v3, v4) = (v18, v47);
                    continue;
                } else {
                    v3 = v18;
                    continue;
                }
            }
        }
    }
}
fn method25(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn rows_of_24(mut v0: Rc<UH1>, mut v1: i32, mut v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_NoRanked => {
                return v2;
            }
            UH1::UH1_RankedThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: i32 = v5;
                let v6: i32 = v6;
                let v7: i32 = v7;
                let v8: bool = v8;
                let v9: i32 = v9;
                let v10: i32 = v10;
                let v11: Rc<str> = v11;
                let v12: Rc<UH1> = v12;
                let v13: i32 = (v2.clone().borrow().len() as i32);
                let v14: bool = v13 >= v1;
                if v14 {
                    return v2;
                } else {
                    let v15: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>>> = Rc::new(RefCell::new(vec![(v3, v4, v5, v6, v7, v8, v9, v10, v11)]));
                    let v16: i32 = (v15.clone().borrow().len() as i32);
                    let v17: i32 = v13.wrapping_add(v16);
                    let mut v18: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>::default(); v17 as usize]));
                    let mut v19: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                    while method25(v17, &v19) {
                        let v21: i32 = v19.borrow().l0;
                        let v22: bool = v21 < v13;
                        let (mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>) = if v22 {
                            let (mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>) = v2.borrow()[v21 as usize].clone();
                            (v23, v24, v25, v26, v27, v28, v29, v30, v31)
                        } else {
                            let v32: i32 = v21.wrapping_sub(v13);
                            let (mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>) = v15.borrow()[v32 as usize].clone();
                            (v33, v34, v35, v36, v37, v38, v39, v40, v41)
                        };
                        v18.borrow_mut()[v21 as usize] = (v42, v43, v44, v45, v46, v47, v48, v49, v50);
                        let v51: i32 = v21.wrapping_add(1i32);
                        v19.borrow_mut().l0 = v51;
                        ()
                    };
                    (v0, v2) = (v12, v18);
                    continue;
                }
            }
        }
    }
}
fn ranked_count_26(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_NoRanked => {
                return v1;
            }
            UH1::UH1_RankedThen(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: i32 = v4;
                let v5: i32 = v5;
                let v6: i32 = v6;
                let v7: bool = v7;
                let v8: i32 = v8;
                let v9: i32 = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<UH1> = v11;
                let v12: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v11, v12);
                continue;
            }
        }
    }
}
fn agile_next_1(mut v0: Rc<str>, mut v1: US0, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool) -> i32 {
    let v6: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = concat5_2(v0.clone(), v7, v8.clone(), v8.clone(), v8.clone());
    let v10: bool = std::path::Path::new(&*v9).is_file();
    let v13: US1 = if v10 {
        US1::US1_FileReadable(v9)
    } else {
        US1::US1_FileMissing(v9)
    };
    match v13 {
        US1::US1_FileMissing(v14) => {
            let v14: Rc<str> = v14;
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile next rejected: "); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = concat5_2(v16, v15, v8.clone(), v8.clone(), v8.clone());
            let mut v18: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v17 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v18
        }
        US1::US1_FileReadable(v19) => {
            let mut v19: Rc<str> = v19;
            let v20: Rc<str> = std::fs::read_to_string(&*v19).unwrap_or_default().into();
            let v21: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v20);
            let v22: i32 = (v21.clone().len() as i32);
            let v23: i32 = 0i32;
            let v24: i32 = first_malformed_task_3(v21.clone(), v23, v22);
            let v25: bool = v24 < 0i32;
            if v25 {
                let v26: Rc<RefCell<Vec<Rc<str>>>> = agile_waits_hold::rig_agile_waits_holding(&v0, &v21);
                let v27: u64 = v6.wrapping_div(3600000u64);
                let v28: i64 = (v27 as i64);
                let v29: i32 = 0i32;
                let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoTasks); } CASE.with(|case| case.clone()) };
                let v31: Rc<UH0> = tasks_from_7(v21.clone(), v29, v22, v30);
                let v32: Rc<UH0> = annotated_tasks_10(v0.clone(), v21.clone(), v28, v31);
                let v33: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, bool, Rc<str>) -> bool> = closure1(v1.clone(), v2, v3.clone(), v4.clone(), v5, v21, v26);
                let v34: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRanked); } CASE.with(|case| case.clone()) };
                let v35: Rc<UH1> = ranked_from_16(v1.clone(), v33, v32.clone(), v32.clone(), v34);
                let v36: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>>> = Rc::new(RefCell::new(vec![]));
                let v37: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>>> = rows_of_24(v35.clone(), v2, v36);
                let v38: bool = match &v1 {
                    US0::US0_MainAgent => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v39: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v6);
                let v43: Rc<str> = match v1 {
                    US0::US0_MainAgent => {
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("main"); } LIT.with(|lit| lit.clone()) };
                        v40
                    }
                    US0::US0_Subagent => {
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("subagent"); } LIT.with(|lit| lit.clone()) };
                        v41
                    }
                };
                let v44: i32 = 0i32;
                let v45: i32 = ranked_count_26(v35, v44);
                let v46: i32 = agile_next_print::rig_agile_next_printed(v37, &v43, v38, v5, &v4, v39, v45);
                let v48: i32 = if v5 {
                    0i32
                } else {
                    let v47: i32 = agile_waits_hold::rig_agile_lane_waits_printed(&v0, &v3);
                    v47
                };
                v46
            } else {
                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed Task fields at byte "); } LIT.with(|lit| lit.clone()) };
                let v50: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
                let v51: Rc<str> = concat5_2(v49, v50, v8.clone(), v8.clone(), v8.clone());
                let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile next rejected: "); } LIT.with(|lit| lit.clone()) };
                let v53: Rc<str> = concat5_2(v52, v51, v8.clone(), v8.clone(), v8.clone());
                let v54: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v53 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v54
            }
        }
    }
}
fn agile_next_entry_0() -> i32 {
    let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (bool, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, bool) = rig_agile_next_args::rig_agile_next_request();
    if v0 {
        let v7: bool = &*v2 == "subagent";
        let v10: US0 = if v7 {
            US0::US0_Subagent
        } else {
            US0::US0_MainAgent
        };
        let v11: i32 = 1i32;
        let v12: Rc<str> = usize::try_from(v11).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        agile_next_1(v12, v10, v3, v4, v5, v6)
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile next rejected: "); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = concat5_2(v14, v1, v15.clone(), v15.clone(), v15.clone());
        let v17: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v16 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v17
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_next_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_next_run() -> i32 {
    closure0()()
}
pub type SpiralShape_2617b4e49837 = Mut0;
