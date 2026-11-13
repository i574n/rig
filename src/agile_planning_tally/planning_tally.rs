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
    US0_EveryTask,
    US0_StatusBucket(Rc<str>),
    US0_LoadBucket(i32),
    US0_AssigneeBucket(i32),
    US0_EpicBucket(Rc<str>),
    US0_UnknownBucket,
}
#[derive(Clone)]
enum US1 {
    US1_Unestimated,
    US1_EstimateMinutes(i32),
}
#[derive(Clone)]
enum US3 {
    US3_LoadLight,
    US3_LoadMedium,
    US3_LoadHeavy,
}
#[derive(Clone)]
enum US2 {
    US2_LoadUnset,
    US2_PlannedLoad(US3),
}
#[derive(Clone)]
enum US5 {
    US5_AssigneeAgent,
    US5_AssigneePair,
    US5_AssigneeUser,
}
#[derive(Clone)]
enum US4 {
    US4_Unassigned,
    US4_PlannedAssignee(US5),
}
#[derive(Clone)]
enum US6 {
    US6_ScanPlain,
    US6_ScanQuoted,
    US6_ScanEscaped,
}
#[derive(Clone)]
enum US7 {
    US7_Valid(i32),
    US7_Invalid,
}
#[derive(Clone)]
enum US8 {
    US8_Valid(US2),
    US8_Invalid,
}
#[derive(Clone)]
enum US9 {
    US9_Valid(US3),
    US9_Invalid,
}
#[derive(Clone)]
enum US10 {
    US10_Valid(US4),
    US10_Invalid,
}
#[derive(Clone)]
enum US11 {
    US11_Valid(US5),
    US11_Invalid,
}
fn field_end_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US6) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = match &v3 {
                US6::US6_ScanPlain => {
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
                let v25: US6 = match v3 {
                    US6::US6_ScanEscaped => {
                        US6::US6_ScanQuoted
                    }
                    US6::US6_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US6::US6_ScanQuoted
                        } else {
                            US6::US6_ScanPlain
                        }
                    }
                    US6::US6_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US6::US6_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US6::US6_ScanPlain
                            } else {
                                US6::US6_ScanQuoted
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
fn skip_spaces_5(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US6 = US6::US6_ScanPlain;
            let v6: i32 = field_end_4(v0.clone(), v1, v2, v5);
            let mut v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_5(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn decimal_digits_value_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v6: bool = 48i32 <= v5;
            let v8: bool = if v6 {
                let mut v7: bool = v5 <= 57i32;
                v7
            } else {
                false
            };
            if v8 {
                let v9: i32 = v1.wrapping_add(1i32);
                let v10: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v11: i32 = v3 * 10 + v10 - 48;
                (v1, v3) = (v9, v11);
                continue;
            } else {
                return -1i32;
            }
        }
    }
}
fn decimal_value_8(mut v0: Rc<str>) -> US7 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = v1 <= 0i32;
    if v2 {
        US7::US7_Invalid
    } else {
        let v4: bool = v1 <= 6i32;
        if v4 {
            let v5: i32 = 0i32;
            let v6: i32 = 0i32;
            let mut v7: i32 = decimal_digits_value_9(v0, v5, v1, v6);
            let v8: bool = v7 < 0i32;
            if v8 {
                US7::US7_Invalid
            } else {
                US7::US7_Valid(v7)
            }
        } else {
            US7::US7_Invalid
        }
    }
}
fn parse_planned_load_10(mut v0: Rc<str>) -> US8 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: bool = &*v1 == "none";
    let v4: bool = if v2 {
        true
    } else {
        let mut v3: bool = &*v1 == "-";
        v3
    };
    if v4 {
        let v5: US2 = US2::US2_LoadUnset;
        US8::US8_Valid(v5)
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
        let v9: bool = &*v1 == &*v8;
        let v13: US9 = if v9 {
            let v10: US3 = US3::US3_LoadLight;
            US9::US9_Valid(v10)
        } else {
            US9::US9_Invalid
        };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = std::rc::Rc::<str>::from(v14.to_ascii_lowercase());
        let v16: bool = &*v1 == &*v15;
        let v19: US9 = if v16 {
            let v17: US3 = US3::US3_LoadMedium;
            US9::US9_Valid(v17)
        } else {
            v13
        };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = std::rc::Rc::<str>::from(v20.to_ascii_lowercase());
        let v22: bool = &*v1 == &*v21;
        let v25: US9 = if v22 {
            let v23: US3 = US3::US3_LoadHeavy;
            US9::US9_Valid(v23)
        } else {
            v19
        };
        match v25 {
            US9::US9_Invalid => {
                US8::US8_Invalid
            }
            US9::US9_Valid(v26) => {
                let v26: US3 = v26;
                let v27: US2 = US2::US2_PlannedLoad(v26);
                US8::US8_Valid(v27)
            }
        }
    }
}
fn parse_planned_assignee_11(mut v0: Rc<str>) -> US10 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: bool = &*v1 == "none";
    let v4: bool = if v2 {
        true
    } else {
        let mut v3: bool = &*v1 == "-";
        v3
    };
    if v4 {
        let v5: US4 = US4::US4_Unassigned;
        US10::US10_Valid(v5)
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
        let v9: bool = &*v1 == &*v8;
        let v13: US11 = if v9 {
            let v10: US5 = US5::US5_AssigneeAgent;
            US11::US11_Valid(v10)
        } else {
            US11::US11_Invalid
        };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = std::rc::Rc::<str>::from(v14.to_ascii_lowercase());
        let v16: bool = &*v1 == &*v15;
        let v19: US11 = if v16 {
            let v17: US5 = US5::US5_AssigneePair;
            US11::US11_Valid(v17)
        } else {
            v13
        };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = std::rc::Rc::<str>::from(v20.to_ascii_lowercase());
        let v22: bool = &*v1 == &*v21;
        let v25: US11 = if v22 {
            let v23: US5 = US5::US5_AssigneeUser;
            US11::US11_Valid(v23)
        } else {
            v19
        };
        match v25 {
            US11::US11_Invalid => {
                US10::US10_Invalid
            }
            US11::US11_Valid(v26) => {
                let v26: US5 = v26;
                let v27: US4 = US4::US4_PlannedAssignee(v26);
                US10::US10_Valid(v27)
            }
        }
    }
}
fn planning_at_7(mut v0: Rc<str>, mut v1: i32) -> (US1, US2, US4) {
    let mut v2: i32 = v1.wrapping_add(10i32);
    let mut v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = 1i32;
    let v5: i32 = field_start_3(v0.clone(), v2, v3, v4);
    let v6: US6 = US6::US6_ScanPlain;
    let mut v7: i32 = field_end_4(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Minutes "); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v8.starts_with(&*v9);
    let v23: US1 = if v10 {
        let v11: i32 = (v8.clone().len() as i32);
        let v12: Rc<str> = std::rc::Rc::<str>::from(v8.get(8i32 as usize..v11 as usize).unwrap_or(""));
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = Rc::<str>::from(v12.replace(&*v13, &*v14));
        let v16: US7 = decimal_value_8(v15);
        match v16 {
            US7::US7_Invalid => {
                US1::US1_Unestimated
            }
            US7::US7_Valid(v17) => {
                let v17: i32 = v17;
                US1::US1_EstimateMinutes(v17)
            }
        }
    } else {
        US1::US1_Unestimated
    };
    let v24: i32 = 2i32;
    let v25: i32 = field_start_3(v0.clone(), v2, v3, v24);
    let v26: US6 = US6::US6_ScanPlain;
    let v27: i32 = field_end_4(v0.clone(), v25, v3, v26);
    let v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load "); } LIT.with(|lit| lit.clone()) };
    let v30: bool = v28.starts_with(&*v29);
    let v39: US2 = if v30 {
        let v31: i32 = (v28.clone().len() as i32);
        let v32: Rc<str> = std::rc::Rc::<str>::from(v28.get(5i32 as usize..v31 as usize).unwrap_or(""));
        let v33: US8 = parse_planned_load_10(v32);
        match v33 {
            US8::US8_Invalid => {
                US2::US2_LoadUnset
            }
            US8::US8_Valid(v34) => {
                let v34: US2 = v34;
                v34
            }
        }
    } else {
        US2::US2_LoadUnset
    };
    let v40: i32 = 3i32;
    let v41: i32 = field_start_3(v0.clone(), v2, v3, v40);
    let v42: US6 = US6::US6_ScanPlain;
    let v43: i32 = field_end_4(v0.clone(), v41, v3, v42);
    let v44: Rc<str> = std::rc::Rc::<str>::from(v0.get(v41 as usize..v43 as usize).unwrap_or(""));
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) };
    let v46: bool = v44.starts_with(&*v45);
    let v55: US4 = if v46 {
        let v47: i32 = (v44.clone().len() as i32);
        let v48: Rc<str> = std::rc::Rc::<str>::from(v44.get(9i32 as usize..v47 as usize).unwrap_or(""));
        let v49: US10 = parse_planned_assignee_11(v48);
        match v49 {
            US10::US10_Invalid => {
                US4::US4_Unassigned
            }
            US10::US10_Valid(v50) => {
                let v50: US4 = v50;
                v50
            }
        }
    } else {
        US4::US4_Unassigned
    };
    (v23, v39, v55)
}
fn declared_from_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
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
fn planning_of_12(mut v0: Rc<str>, mut v1: Rc<str>) -> (US1, US2, US4) {
    let mut v3: Rc<str> = string_concat(&["Planning (\"", &*v1, "\","]);
    let v4: i32 = 0i32;
    let v5: i32 = declared_from_13(v0.clone(), v3, v4);
    let v6: bool = v5 < 0i32;
    let (mut v13, mut v14, mut v15): (US1, US2, US4) = if v6 {
        let mut v7: US1 = US1::US1_Unestimated;
        let v8: US2 = US2::US2_LoadUnset;
        let v9: US4 = US4::US4_Unassigned;
        (v7, v8, v9)
    } else {
        let (mut v10, mut v11, mut v12): (US1, US2, US4) = planning_at_7(v0, v5);
        (v10, v11, v12)
    };
    (v13, v14, v15)
}
fn byte_find_between_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    let v4: i32 = (v0).get(v2 as usize..v3 as usize).and_then(|slice| slice.find(&*v1)).map(|at| at as i32 + v2).unwrap_or(-1);
    v4
}
fn planning_offset_near_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    let mut v3: i32 = v2.wrapping_add(1i32);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = v0.get(v3 as usize..).and_then(|rest| rest.find(&*v4)).map(|found| (found + v3 as usize) as i32).unwrap_or(-1);
    let mut v7: Rc<str> = string_concat(&["Planning (\"", &*v1, "\","]);
    let v8: bool = v5 < 0i32;
    let v10: i32 = if v8 {
        let v9: i32 = (v0.clone().len() as i32);
        v9
    } else {
        v5
    };
    let v11: i32 = byte_find_between_15(v0.clone(), v7, v2, v10);
    let v12: bool = v11 >= 0i32;
    let v17: bool = if v12 {
        let v13: i32 = v0.get(..v11 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v13 as usize..v11 as usize).unwrap_or(""));
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
        let v16: bool = v14.starts_with(&*v15);
        v16
    } else {
        false
    };
    if v17 {
        v11
    } else {
        -1i32
    }
}
fn planning_beside_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> (US1, US2, US4) {
    let mut v3: i32 = planning_offset_near_14(v0.clone(), v1.clone(), v2);
    let v4: bool = v3 >= 0i32;
    let (mut v11, mut v12, mut v13): (US1, US2, US4) = if v4 {
        let (mut v5, mut v6, mut v7): (US1, US2, US4) = planning_at_7(v0, v3);
        (v5, v6, v7)
    } else {
        let (mut v8, mut v9, mut v10): (US1, US2, US4) = planning_of_12(v0, v1);
        (v8, v9, v10)
    };
    (v11, v12, v13)
}
fn rollup_task_at_2(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, US1, US2, US4) {
    let mut v3: i32 = 0i32;
    let v4: i32 = field_start_3(v0.clone(), v1, v2, v3);
    let v5: US6 = US6::US6_ScanPlain;
    let v6: i32 = field_end_4(v0.clone(), v4, v2, v5);
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let v8: i32 = (v7.clone().len() as i32);
    let v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let v11: i32 = 6i32;
    let v12: i32 = field_start_3(v0.clone(), v1, v2, v11);
    let v13: US6 = US6::US6_ScanPlain;
    let mut v14: i32 = field_end_4(v0.clone(), v12, v2, v13);
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: i32 = 7i32;
    let v17: i32 = field_start_3(v0.clone(), v1, v2, v16);
    let v18: US6 = US6::US6_ScanPlain;
    let v19: i32 = field_end_4(v0.clone(), v17, v2, v18);
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let v21: i32 = (v20.clone().len() as i32);
    let v22: i32 = v21.wrapping_sub(1i32);
    let v23: Rc<str> = std::rc::Rc::<str>::from(v20.get(1i32 as usize..v22 as usize).unwrap_or(""));
    let (mut v24, mut v25, mut v26): (US1, US2, US4) = planning_beside_6(v0, v10.clone(), v1);
    (v10, v15, v23, v24, v25, v26)
}
fn loop_17(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let v4: bool = v3 >= v2;
        if v4 {
            return false;
        } else {
            let v5: Rc<str> = v1.borrow()[v3 as usize].clone();
            let v6: Rc<str> = Rc::<str>::from((v5).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let mut v7: bool = &*v6 == &*v0;
            if v7 {
                return true;
            } else {
                let v8: i32 = v3.wrapping_add(1i32);
                v3 = v8;
                continue;
            }
        }
    }
}
fn bucket_holds_16(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1, mut v5: US2, mut v6: US4) -> bool {
    match v0 {
        US0::US0_AssigneeBucket(v17) => {
            let v17: i32 = v17;
            let v23: i32 = match v6 {
                US4::US4_PlannedAssignee(v18) => {
                    let v18: US5 = v18;
                    match v18 {
                        US5::US5_AssigneeAgent => {
                            1i32
                        }
                        US5::US5_AssigneePair => {
                            2i32
                        }
                        US5::US5_AssigneeUser => {
                            3i32
                        }
                    }
                }
                US4::US4_Unassigned => {
                    0i32
                }
            };
            let v24: bool = v17 == v23;
            v24
        }
        US0::US0_EpicBucket(v25) => {
            let v25: Rc<str> = v25;
            let v26: bool = &*v25 == &*v1;
            if v26 {
                true
            } else {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v27).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let v29: i32 = (v28.clone().borrow().len() as i32);
                let v30: i32 = 0i32;
                loop_17(v25, v28, v29, v30)
            }
        }
        US0::US0_EveryTask => {
            true
        }
        US0::US0_LoadBucket(v9) => {
            let v9: i32 = v9;
            let v15: i32 = match v5 {
                US2::US2_LoadUnset => {
                    0i32
                }
                US2::US2_PlannedLoad(v10) => {
                    let v10: US3 = v10;
                    match v10 {
                        US3::US3_LoadHeavy => {
                            3i32
                        }
                        US3::US3_LoadLight => {
                            1i32
                        }
                        US3::US3_LoadMedium => {
                            2i32
                        }
                    }
                }
            };
            let v16: bool = v9 == v15;
            v16
        }
        US0::US0_StatusBucket(v7) => {
            let mut v7: Rc<str> = v7;
            let v8: bool = &*v7 == &*v2;
            v8
        }
        US0::US0_UnknownBucket => {
            false
        }
    }
}
fn tally_with_18(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: &Rc<str>, mut v6: Rc<str>, mut v7: &Rc<str>, mut v8: US1, mut v9: US2, mut v10: US4) -> (i32, i32, i32, i32, i32) {
    let v13: i32 = match &v8 {
        US1::US1_EstimateMinutes(v11) => {
            let v11: i32 = *v11;
            v11
        }
        US1::US1_Unestimated => {
            0i32
        }
    };
    let mut v14: bool = &*v6 == "Done";
    let v16: bool = if v14 {
        true
    } else {
        let v15: bool = &*v6 == "Dropped";
        v15
    };
    let v17: bool = v16 == false;
    let v18: i32 = v0.wrapping_add(1i32);
    let v19: i32 = if v17 {
        1i32
    } else {
        0i32
    };
    let mut v20: i32 = v1.wrapping_add(v19);
    let v23: bool = match v8 {
        US1::US1_EstimateMinutes(v21) => {
            let v21: i32 = v21;
            true
        }
        US1::US1_Unestimated => {
            false
        }
    };
    let v24: i32 = if v23 {
        1i32
    } else {
        0i32
    };
    let v25: i32 = v2.wrapping_add(v24);
    let v26: i32 = v3.wrapping_add(v13);
    let v27: i32 = if v17 {
        v13
    } else {
        0i32
    };
    let v28: i32 = v4.wrapping_add(v27);
    (v18, v20, v25, v26, v28)
}
fn tally_from_1(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32) -> (i32, i32, i32, i32, i32) {
    loop {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v10: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v11: bool = v10 < 0i32;
        if v11 {
            return (v4, v5, v6, v7, v8);
        } else {
            let v12: i32 = v10.wrapping_add(6i32);
            let v13: i32 = v0.get(..v10 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v13 as usize..v10 as usize).unwrap_or(""));
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v16: bool = v14.starts_with(&*v15);
            if v16 {
                let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22): (Rc<str>, Rc<str>, Rc<str>, US1, US2, US4) = rollup_task_at_2(v0.clone(), v12, v2);
                let v23: bool = bucket_holds_16(v3.clone(), v17.clone(), v18.clone(), v19.clone(), v20.clone(), v21.clone(), v22.clone());
                if v23 {
                    let (mut v24, mut v25, mut v26, mut v27, mut v28): (i32, i32, i32, i32, i32) = tally_with_18(v4, v5, v6, v7, v8, &v17, v18, &v19, v20, v21, v22);
                    (v1, v4, v5, v6, v7, v8) = (v12, v24, v25, v26, v27, v28);
                    continue;
                } else {
                    v1 = v12;
                    continue;
                }
            } else {
                v1 = v12;
                continue;
            }
        }
    }
}
fn bucket_of_19(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let mut v2: bool = &*v0 == "all";
    let v5: US0 = if v2 {
        US0::US0_EveryTask
    } else {
        US0::US0_UnknownBucket
    };
    let v6: bool = &*v0 == "status";
    let v8: US0 = if v6 {
        US0::US0_StatusBucket(v1.clone())
    } else {
        v5
    };
    let v9: bool = &*v0 == "load";
    let v22: US0 = if v9 {
        let v10: US8 = parse_planned_load_10(v1.clone());
        match v10 {
            US8::US8_Invalid => {
                US0::US0_UnknownBucket
            }
            US8::US8_Valid(v11) => {
                let v11: US2 = v11;
                let v17: i32 = match v11 {
                    US2::US2_LoadUnset => {
                        0i32
                    }
                    US2::US2_PlannedLoad(v12) => {
                        let v12: US3 = v12;
                        match v12 {
                            US3::US3_LoadHeavy => {
                                3i32
                            }
                            US3::US3_LoadLight => {
                                1i32
                            }
                            US3::US3_LoadMedium => {
                                2i32
                            }
                        }
                    }
                };
                US0::US0_LoadBucket(v17)
            }
        }
    } else {
        v8
    };
    let v23: bool = &*v0 == "assignee";
    let v36: US0 = if v23 {
        let v24: US10 = parse_planned_assignee_11(v1.clone());
        match v24 {
            US10::US10_Invalid => {
                US0::US0_UnknownBucket
            }
            US10::US10_Valid(v25) => {
                let v25: US4 = v25;
                let v31: i32 = match v25 {
                    US4::US4_PlannedAssignee(v26) => {
                        let v26: US5 = v26;
                        match v26 {
                            US5::US5_AssigneeAgent => {
                                1i32
                            }
                            US5::US5_AssigneePair => {
                                2i32
                            }
                            US5::US5_AssigneeUser => {
                                3i32
                            }
                        }
                    }
                    US4::US4_Unassigned => {
                        0i32
                    }
                };
                US0::US0_AssigneeBucket(v31)
            }
        }
    } else {
        v22
    };
    let v37: bool = &*v0 == "epic";
    if v37 {
        US0::US0_EpicBucket(v1)
    } else {
        v36
    }
}
fn planning_tally_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (i32, i32, i32, i32, i32) {
    let mut v3: i32 = 0i32;
    let v4: i32 = (v0.clone().len() as i32);
    let v5: US0 = bucket_of_19(v1, v2);
    let v6: i32 = 0i32;
    let mut v7: i32 = 0i32;
    let v8: i32 = 0i32;
    let v9: i32 = 0i32;
    let v10: i32 = 0i32;
    let (mut v11, mut v12, mut v13, mut v14, mut v15): (i32, i32, i32, i32, i32) = tally_from_1(v0, v3, v4, v5, v6, v7, v8, v9, v10);
    (v11, v12, v13, v14, v15)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (i32, i32, i32, i32, i32)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (i32, i32, i32, i32, i32)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> (i32, i32, i32, i32, i32) {
        planning_tally_0(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_planning_tally(v0: &str, v1: &str, v2: &str) -> (i32, i32, i32, i32, i32) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
