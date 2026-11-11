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
fn digits_in_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let v3: bool = v2 <= v1;
        if v3 {
            return true;
        } else {
            let v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v5: bool = 48i32 <= v4;
            let v7: bool = if v5 {
                let v6: bool = v4 <= 57i32;
                v6
            } else {
                false
            };
            if v7 {
                let v8: i32 = v1.wrapping_add(1i32);
                v1 = v8;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn dated_clock_2(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = (v0.clone().len() as i32);
    let v2: bool = 16i32 <= v1;
    if v2 {
        let v3: i32 = 0i32;
        let v4: i32 = 4i32;
        let v5: bool = digits_in_3(v0.clone(), v3, v4);
        if v5 {
            let v6: i32 = v0.as_bytes().get(4i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v7: bool = v6 == 45i32;
            if v7 {
                let v8: i32 = 5i32;
                let mut v9: i32 = 7i32;
                let v10: bool = digits_in_3(v0.clone(), v8, v9);
                if v10 {
                    let v11: i32 = v0.as_bytes().get(7i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                    let v12: bool = v11 == 45i32;
                    if v12 {
                        let v13: i32 = 8i32;
                        let v14: i32 = 10i32;
                        let v15: bool = digits_in_3(v0.clone(), v13, v14);
                        if v15 {
                            let v16: i32 = v0.as_bytes().get(10i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                            let v17: bool = v16 == 32i32;
                            if v17 {
                                let v18: i32 = 11i32;
                                let v19: i32 = 13i32;
                                let v20: bool = digits_in_3(v0.clone(), v18, v19);
                                if v20 {
                                    let v21: i32 = v0.as_bytes().get(13i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                                    let v22: bool = v21 == 58i32;
                                    if v22 {
                                        let v23: i32 = 14i32;
                                        let v24: i32 = 16i32;
                                        let v25: bool = digits_in_3(v0.clone(), v23, v24);
                                        if v25 {
                                            let v26: bool = v1 == 16i32;
                                            if v26 {
                                                true
                                            } else {
                                                let v27: i32 = v0.as_bytes().get(16i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                                                let v28: bool = v27 == 32i32;
                                                v28
                                            }
                                        } else {
                                            false
                                        }
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
fn dated_day_4(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = (v0.clone().len() as i32);
    let v2: bool = 10i32 <= v1;
    if v2 {
        let v3: i32 = 0i32;
        let v4: i32 = 4i32;
        let v5: bool = digits_in_3(v0.clone(), v3, v4);
        if v5 {
            let v6: i32 = v0.as_bytes().get(4i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v7: bool = v6 == 45i32;
            if v7 {
                let v8: i32 = 5i32;
                let mut v9: i32 = 7i32;
                let v10: bool = digits_in_3(v0.clone(), v8, v9);
                if v10 {
                    let v11: i32 = v0.as_bytes().get(7i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                    let v12: bool = v11 == 45i32;
                    if v12 {
                        let v13: i32 = 8i32;
                        let v14: i32 = 10i32;
                        let v15: bool = digits_in_3(v0.clone(), v13, v14);
                        if v15 {
                            let v16: bool = v1 == 10i32;
                            if v16 {
                                true
                            } else {
                                let v17: i32 = v0.as_bytes().get(10i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                                let v18: bool = v17 == 32i32;
                                v18
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
fn bare_clock_5(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = (v0.clone().len() as i32);
    let v2: bool = 5i32 <= v1;
    if v2 {
        let v3: i32 = 0i32;
        let v4: i32 = 2i32;
        let v5: bool = digits_in_3(v0.clone(), v3, v4);
        if v5 {
            let v6: i32 = v0.as_bytes().get(2i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v7: bool = v6 == 58i32;
            if v7 {
                let v8: i32 = 3i32;
                let mut v9: i32 = 5i32;
                let v10: bool = digits_in_3(v0.clone(), v8, v9);
                if v10 {
                    let v11: bool = v1 == 5i32;
                    if v11 {
                        true
                    } else {
                        let v12: i32 = v0.as_bytes().get(5i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                        let v13: bool = v12 == 32i32;
                        v13
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
fn stamped_separator_from_1(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v3: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v2)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v3.wrapping_add(2i32);
            let v6: i32 = (v0.clone().len() as i32);
            let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v6 as usize).unwrap_or(""));
            let v8: bool = dated_clock_2(v7.clone());
            let v12: bool = if v8 {
                true
            } else {
                let mut v9: bool = dated_day_4(v7.clone());
                if v9 {
                    true
                } else {
                    bare_clock_5(v7)
                }
            };
            if v12 {
                return v3;
            } else {
                v1 = v5;
                continue;
            }
        }
    }
}
fn stamped_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let v4: bool = dated_clock_2(v0.clone());
    let (mut v33, mut v34, mut v35): (Rc<str>, Rc<str>, Rc<str>) = if v4 {
        let v5: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..16i32 as usize).unwrap_or(""));
        let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..10i32 as usize).unwrap_or(""));
        let v7: i32 = (v0.clone().len() as i32);
        let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(16i32 as usize..v7 as usize).unwrap_or(""));
        let mut v9: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        (v5, v6, v9)
    } else {
        let v10: bool = dated_day_4(v0.clone());
        let (mut v30, mut v31, mut v32): (Rc<str>, Rc<str>, Rc<str>) = if v10 {
            let v11: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..10i32 as usize).unwrap_or(""));
            let v12: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..10i32 as usize).unwrap_or(""));
            let v13: i32 = (v0.clone().len() as i32);
            let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(10i32 as usize..v13 as usize).unwrap_or(""));
            let v15: Rc<str> = Rc::<str>::from((v14).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            (v11, v12, v15)
        } else {
            let v16: bool = bare_clock_5(v0.clone());
            let (mut v27, mut v28, mut v29): (Rc<str>, Rc<str>, Rc<str>) = if v16 {
                let v17: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..5i32 as usize).unwrap_or(""));
                let v18: bool = &*v1 == "";
                let v21: Rc<str> = if v18 {
                    v17
                } else {
                    let v20: Rc<str> = string_concat(&[&*v1, " ", &*v17]);
                    v20
                };
                let v22: i32 = (v0.clone().len() as i32);
                let v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(5i32 as usize..v22 as usize).unwrap_or(""));
                let v24: Rc<str> = Rc::<str>::from((v23).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                (v21, v1, v24)
            } else {
                let v25: bool = &*v2 == "";
                let v26: Rc<str> = if v25 {
                    v3
                } else {
                    v2
                };
                (v26, v1, v0)
            };
            (v27, v28, v29)
        };
        (v30, v31, v32)
    };
    (v33, v34, v35)
}
fn concat5_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn note_row_8(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v8));
    let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = concat5_9(v6, v10, v8, v11, v12.clone());
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
    let v30: Rc<str> = concat5_9(v15.clone(), v29, v15.clone(), v12.clone(), v12.clone());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = concat5_9(v5, v13, v14, v30, v31.clone());
    let v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = Rc::<str>::from(v2.replace(&*v16, &*v17));
    let v36: Rc<str> = Rc::<str>::from(v35.replace(&*v15, &*v19));
    let v37: Rc<str> = Rc::<str>::from(v36.replace(&*v21, &*v22));
    let v38: Rc<str> = Rc::<str>::from(v37.replace(&*v24, &*v25));
    let v39: Rc<str> = Rc::<str>::from(v38.replace(&*v27, &*v28));
    let v40: Rc<str> = concat5_9(v15.clone(), v39, v15.clone(), v12.clone(), v12.clone());
    let v41: Rc<str> = Rc::<str>::from(v3.replace(&*v16, &*v17));
    let v42: Rc<str> = Rc::<str>::from(v41.replace(&*v15, &*v19));
    let v43: Rc<str> = Rc::<str>::from(v42.replace(&*v21, &*v22));
    let v44: Rc<str> = Rc::<str>::from(v43.replace(&*v24, &*v25));
    let v45: Rc<str> = Rc::<str>::from(v44.replace(&*v27, &*v28));
    let v46: Rc<str> = concat5_9(v15.clone(), v45, v15.clone(), v12.clone(), v12.clone());
    let v47: Rc<str> = concat5_9(v33, v34, v40, v31.clone(), v46);
    let v48: Rc<str> = Rc::<str>::from(v4.replace(&*v16, &*v17));
    let v49: Rc<str> = Rc::<str>::from(v48.replace(&*v15, &*v19));
    let v50: Rc<str> = Rc::<str>::from(v49.replace(&*v21, &*v22));
    let v51: Rc<str> = Rc::<str>::from(v50.replace(&*v24, &*v25));
    let v52: Rc<str> = Rc::<str>::from(v51.replace(&*v27, &*v28));
    let v53: Rc<str> = concat5_9(v15.clone(), v52, v15, v12.clone(), v12);
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    concat5_9(v32, v47, v31, v53, v54)
}
fn moved_from_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32) -> (Rc<str>, i32) {
    loop {
        let v10: bool = v2 <= v1;
        if v10 {
            return (v8, v9);
        } else {
            let v11: i32 = stamped_separator_from_1(v0.clone(), v1);
            let v12: bool = v11 < 0i32;
            let v13: i32 = if v12 {
                v2
            } else {
                v11
            };
            let v15: i32 = if v12 {
                v2
            } else {
                let v14: i32 = v11.wrapping_add(2i32);
                v14
            };
            let v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v13 as usize).unwrap_or(""));
            let v17: Rc<str> = Rc::<str>::from((v16).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let v18: bool = &*v17 == "";
            if v18 {
                v1 = v15;
                continue;
            } else {
                let (mut v21, mut v22, mut v23): (Rc<str>, Rc<str>, Rc<str>) = stamped_7(v17.clone(), v5, v6, v7.clone());
                let v24: bool = &*v23 == "";
                let v25: Rc<str> = if v24 {
                    v17
                } else {
                    v23
                };
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrated"); } LIT.with(|lit| lit.clone()) };
                let v27: Rc<str> = note_row_8(v3.clone(), v4, v21.clone(), v26, v25);
                let v28: bool = &*v8 == "";
                let v31: Rc<str> = if v28 {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v29
                } else {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v30
                };
                let v32: i32 = v4.wrapping_add(1i32);
                let v34: Rc<str> = string_concat(&[&*v8, &*v31, &*v27]);
                let v35: i32 = v9.wrapping_add(1i32);
                (v1, v4, v5, v6, v8, v9) = (v15, v32, v22, v21, v34, v35);
                continue;
            }
        }
    }
}
fn tail_rows_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let v4: i32 = 0i32;
    let v5: i32 = stamped_separator_from_1(v0.clone(), v4);
    let v6: bool = v5 < 0i32;
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (0i32, v7)
    } else {
        let v8: i32 = v5.wrapping_add(2i32);
        let mut v9: i32 = (v0.clone().len() as i32);
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v11: i32 = 0i32;
        let (mut v12, mut v13): (Rc<str>, i32) = moved_from_6(v0, v8, v9, v1, v2, v10.clone(), v10.clone(), v3, v10, v11);
        (v13, v12)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, i32, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, i32, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>| -> (i32, Rc<str>) {
        tail_rows_0(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        let mut v1: i32 = 0i32;
        stamped_separator_from_1(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_title_tail_rows(v0: &str, v1: &str, v2: i32, v3: &str) -> (i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2, Rc::<str>::from(v3))
}
pub fn rig_agile_title_kept_end(v0: &str) -> i32 {
    closure1()(Rc::<str>::from(v0))
}
