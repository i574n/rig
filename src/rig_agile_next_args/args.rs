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
    US0_ArgsParsed(Rc<str>, i32, Rc<str>, Rc<str>, bool),
    US0_ArgsRejected(Rc<str>),
}
fn load_words_2(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    v2
}
fn loads_valid_3(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> bool {
    loop {
        let v2: i32 = (v0.clone().borrow().len() as i32);
        let v3: bool = v2 <= v1;
        if v3 {
            return true;
        } else {
            let v4: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v5: bool = agile_planning_filters::rig_agile_planning_filters_valid(&*v4, "all");
            if v5 {
                let v6: i32 = v1.wrapping_add(1i32);
                v1 = v6;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn next_request_from_1(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: bool) -> US0 {
    loop {
        let v7: bool = v1 <= v0;
        if v7 {
            return US0::US0_ArgsParsed(v2, v3, v4, v5, v6);
        } else {
            let v9: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v10: i32 = v0.wrapping_add(1i32);
            let v11: Rc<str> = usize::try_from(v10).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v12: bool = &*v9 == "--as";
            if v12 {
                let v13: bool = &*v11 == "main";
                let v15: bool = if v13 {
                    true
                } else {
                    let v14: bool = &*v11 == "subagent";
                    v14
                };
                if v15 {
                    let v16: i32 = v0.wrapping_add(2i32);
                    (v0, v2) = (v16, v11);
                    continue;
                } else {
                    let v18: Rc<str> = string_concat(&["--as expects main or subagent, got ", &*v11]);
                    return US0::US0_ArgsRejected(v18);
                }
            } else {
                let v21: bool = &*v9 == "--json";
                if v21 {
                    let v22: bool = true;
                    (v0, v6) = (v10, v22);
                    continue;
                } else {
                    let v24: bool = &*v9 == "--fits";
                    if v24 {
                        let v25: Rc<str> = rig_window_query::rig_fitting_loads();
                        (v0, v5) = (v10, v25);
                        continue;
                    } else {
                        let v27: bool = &*v9 == "--load";
                        if v27 {
                            let v28: bool = &*v11 != "";
                            let v32: bool = if v28 {
                                let v29: Rc<RefCell<Vec<Rc<str>>>> = load_words_2(v11.clone());
                                let v30: i32 = 0i32;
                                loads_valid_3(v29, v30)
                            } else {
                                false
                            };
                            if v32 {
                                let v33: i32 = v0.wrapping_add(2i32);
                                (v0, v5) = (v33, v11);
                                continue;
                            } else {
                                let v35: Rc<str> = string_concat(&["--load expects light, medium, heavy or none, or a comma list of them, got ", &*v11]);
                                return US0::US0_ArgsRejected(v35);
                            }
                        } else {
                            let v38: i32 = agile_task_query::rig_query_flag_width(&*v9, &*v11);
                            let v39: bool = v38 > 0i32;
                            if v39 {
                                let v40: i32 = v0.wrapping_add(v38);
                                let v41: bool = v38 == 2i32;
                                let v43: Rc<str> = if v41 {
                                    v11
                                } else {
                                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v42
                                };
                                let v44: bool = &*v4 == "";
                                let v47: Rc<str> = if v44 {
                                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v45
                                } else {
                                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    v46
                                };
                                let v49: Rc<str> = string_concat(&[&*v4, &*v47, &*v9]);
                                let v50: bool = &*v43 == "";
                                let v53: Rc<str> = if v50 {
                                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v51
                                } else {
                                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    v52
                                };
                                let v55: Rc<str> = string_concat(&[&*v49, &*v53, &*v43]);
                                (v0, v4) = (v40, v55);
                                continue;
                            } else {
                                let v57: bool = v38 < 0i32;
                                if v57 {
                                    let v58: Rc<str> = agile_task_query::rig_query_flag_refusal(&*v9, &*v11);
                                    return US0::US0_ArgsRejected(v58);
                                } else {
                                    let v60: i32 = (v9).parse::<i32>().ok().filter(|value| *value > 0).unwrap_or(0);
                                    let v61: bool = 0i32 < v60;
                                    if v61 {
                                        (v0, v3) = (v10, v60);
                                        continue;
                                    } else {
                                        let v65: Rc<str> = agile_task_query::rig_query_usage();
                                        let v69: Rc<str> = string_concat(&["unexpected argument ", &*v9, " (expected --as main|subagent, ", &*v65, ", --load <light|medium|heavy|none,...>, --fits, --json or a positive count)"]);
                                        return US0::US0_ArgsRejected(v69);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn next_request_0() -> (bool, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, bool) {
    let v0: i32 = 2i32;
    let mut v1: usize = std::env::args().count();
    let v2: i32 = ((v1) as i32);
    let v3: bool = v2 > 2i32;
    let v5: i32 = if v3 {
        let v4: i32 = v2.wrapping_sub(2i32);
        v4
    } else {
        0i32
    };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("main"); } LIT.with(|lit| lit.clone()) };
    let v7: i32 = 10i32;
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let v10: bool = false;
    let v11: US0 = next_request_from_1(v0, v5, v6, v7, v8.clone(), v9, v10);
    match v11 {
        US0::US0_ArgsParsed(v12, v13, v14, v15, v16) => {
            let v12: Rc<str> = v12;
            let v13: i32 = v13;
            let v14: Rc<str> = v14;
            let v15: Rc<str> = v15;
            let v16: bool = v16;
            (true, v8, v12, v13, v14, v15, v16)
        }
        US0::US0_ArgsRejected(v17) => {
            let v17: Rc<str> = v17;
            (false, v17, v8.clone(), 0i32, v8.clone(), v8, false)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (bool, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, bool)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (bool, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, bool)> = Rc::new(move || -> (bool, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, bool) {
        next_request_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_next_request() -> (bool, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, bool) {
    closure0()()
}
