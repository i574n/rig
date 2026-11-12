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
                    let v19: Rc<str> = string_concat(&["--as expects main or subagent, got ", &*v11, "; a lane is not a role here: --team <team> filters the list, claim <ID> --as <lane> takes an item"]);
                    return US0::US0_ArgsRejected(v19);
                }
            } else {
                let v22: bool = &*v9 == "--json";
                if v22 {
                    let v23: bool = true;
                    (v0, v6) = (v10, v23);
                    continue;
                } else {
                    let v25: bool = &*v9 == "--fits";
                    if v25 {
                        let v26: Rc<str> = rig_window_query::rig_fitting_loads();
                        (v0, v5) = (v10, v26);
                        continue;
                    } else {
                        let v28: bool = &*v9 == "--load";
                        if v28 {
                            let v29: bool = &*v11 != "";
                            let v33: bool = if v29 {
                                let v30: Rc<RefCell<Vec<Rc<str>>>> = load_words_2(v11.clone());
                                let v31: i32 = 0i32;
                                loads_valid_3(v30, v31)
                            } else {
                                false
                            };
                            if v33 {
                                let v34: i32 = v0.wrapping_add(2i32);
                                (v0, v5) = (v34, v11);
                                continue;
                            } else {
                                let v36: Rc<str> = string_concat(&["--load expects light, medium, heavy or none, or a comma list of them, got ", &*v11]);
                                return US0::US0_ArgsRejected(v36);
                            }
                        } else {
                            let v39: i32 = agile_task_query::rig_query_flag_width(&*v9, &*v11);
                            let v40: bool = v39 > 0i32;
                            if v40 {
                                let v41: i32 = v0.wrapping_add(v39);
                                let v42: bool = v39 == 2i32;
                                let v44: Rc<str> = if v42 {
                                    v11
                                } else {
                                    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v43
                                };
                                let v45: bool = &*v4 == "";
                                let v48: Rc<str> = if v45 {
                                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v46
                                } else {
                                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    v47
                                };
                                let v50: Rc<str> = string_concat(&[&*v4, &*v48, &*v9]);
                                let v51: bool = &*v44 == "";
                                let v54: Rc<str> = if v51 {
                                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v52
                                } else {
                                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    v53
                                };
                                let v56: Rc<str> = string_concat(&[&*v50, &*v54, &*v44]);
                                (v0, v4) = (v41, v56);
                                continue;
                            } else {
                                let v58: bool = v39 < 0i32;
                                if v58 {
                                    let v59: Rc<str> = agile_task_query::rig_query_flag_refusal(&*v9, &*v11);
                                    return US0::US0_ArgsRejected(v59);
                                } else {
                                    let v61: i32 = (v9).parse::<i32>().ok().filter(|value| *value > 0).unwrap_or(0);
                                    let v62: bool = 0i32 < v61;
                                    if v62 {
                                        (v0, v3) = (v10, v61);
                                        continue;
                                    } else {
                                        let v66: Rc<str> = agile_task_query::rig_query_usage();
                                        let v70: Rc<str> = string_concat(&["unexpected argument ", &*v9, " (expected --as main|subagent, ", &*v66, ", --load <light|medium|heavy|none,...>, --fits, --json or a positive count)"]);
                                        return US0::US0_ArgsRejected(v70);
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
