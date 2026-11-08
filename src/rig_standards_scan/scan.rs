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
    US0_NoCommentLines,
    US0_LineWidth(i32),
    US0_LfOnly,
    US0_IntegerOnly(Rc<str>),
    US0_TestTargets,
    US0_HostFree,
    US0_LibReferenced,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoStandards,
    UH0_StandardThen(US0, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_DeclaredRule(US0),
    US1_UnknownRule(Rc<str>),
    US1_NotARule,
}
#[derive(Clone)]
enum US2 {
    US2_BlankLine,
    US2_CodeLine,
    US2_DirectiveLine,
    US2_TodoLine,
    US2_CommentLine,
}
pub struct Mut0 { pub l0: Rc<str> }
fn declared_row_of_3(mut v0: Rc<str>) -> US1 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    let v5: bool = if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("let "); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.starts_with(&*v3);
        v4
    };
    let v8: bool = if v5 {
        true
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
        let v7: bool = v0.starts_with(&*v6);
        v7
    };
    let v11: bool = if v8 {
        true
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":~ "); } LIT.with(|lit| lit.clone()) };
        let v10: bool = v0.starts_with(&*v9);
        v10
    };
    let v12: bool = v11 == false;
    if v12 {
        US1::US1_NotARule
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= NoCommentLines"); } LIT.with(|lit| lit.clone()) };
        let v15: bool = v0.contains(&*v14);
        if v15 {
            let v16: US0 = US0::US0_NoCommentLines;
            US1::US1_DeclaredRule(v16)
        } else {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= LfOnly"); } LIT.with(|lit| lit.clone()) };
            let v19: bool = v0.contains(&*v18);
            if v19 {
                let v20: US0 = US0::US0_LfOnly;
                US1::US1_DeclaredRule(v20)
            } else {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= TestTargets"); } LIT.with(|lit| lit.clone()) };
                let v23: bool = v0.contains(&*v22);
                if v23 {
                    let v24: US0 = US0::US0_TestTargets;
                    US1::US1_DeclaredRule(v24)
                } else {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= HostFree"); } LIT.with(|lit| lit.clone()) };
                    let v27: bool = v0.contains(&*v26);
                    if v27 {
                        let v28: US0 = US0::US0_HostFree;
                        US1::US1_DeclaredRule(v28)
                    } else {
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= LibReferenced"); } LIT.with(|lit| lit.clone()) };
                        let v31: bool = v0.contains(&*v30);
                        if v31 {
                            let v32: US0 = US0::US0_LibReferenced;
                            US1::US1_DeclaredRule(v32)
                        } else {
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= LineWidth "); } LIT.with(|lit| lit.clone()) };
                            let v35: bool = v0.contains(&*v34);
                            if v35 {
                                let v36: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(&*v34).map(|(_, rest)| rest.trim().trim_end_matches("u32").trim_end_matches("i32").trim_matches(char::from(34u8))).unwrap_or(""));
                                let v37: i32 = (v36).parse::<i32>().unwrap_or(120);
                                let v38: US0 = US0::US0_LineWidth(v37);
                                US1::US1_DeclaredRule(v38)
                            } else {
                                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= IntegerOnly "); } LIT.with(|lit| lit.clone()) };
                                let v41: bool = v0.contains(&*v40);
                                if v41 {
                                    let v42: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(&*v40).map(|(_, rest)| rest.trim().trim_end_matches("u32").trim_end_matches("i32").trim_matches(char::from(34u8))).unwrap_or(""));
                                    let v43: US0 = US0::US0_IntegerOnly(v42);
                                    US1::US1_DeclaredRule(v43)
                                } else {
                                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() = "); } LIT.with(|lit| lit.clone()) };
                                    let v46: bool = v0.contains(&*v45);
                                    let v50: bool = if v46 {
                                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(") : "); } LIT.with(|lit| lit.clone()) };
                                        let v48: bool = v0.contains(&*v47);
                                        let v49: bool = v48 == false;
                                        v49
                                    } else {
                                        false
                                    };
                                    if v50 {
                                        let v51: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(&*v45).map(|(_, rest)| rest.trim().trim_end_matches("u32").trim_end_matches("i32").trim_matches(char::from(34u8))).unwrap_or(""));
                                        US1::US1_UnknownRule(v51)
                                    } else {
                                        US1::US1_NotARule
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
fn declared_from_2(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v2: bool = &*v0 == "";
        if v2 {
            return v1;
        } else {
            let v3: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let v4: US1 = declared_row_of_3(v3);
            match v4 {
                US1::US1_DeclaredRule(v5) => {
                    let v5: US0 = v5;
                    let v6: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
                    let v7: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v5, v1));
                    (v0, v1) = (v6, v7);
                    continue;
                }
                _ => {
                    let v9: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
                    v0 = v9;
                    continue;
                }
            }
        }
    }
}
fn declared_or_default_1(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoStandards); } CASE.with(|case| case.clone()) };
    let v2: Rc<UH0> = declared_from_2(v0, v1);
    match &*v2 {
        UH0::UH0_NoStandards => {
            let v3: US0 = US0::US0_NoCommentLines;
            let v4: US0 = US0::US0_LineWidth(120i32);
            let v5: US0 = US0::US0_LfOnly;
            let v6: US0 = US0::US0_TestTargets;
            let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoStandards); } CASE.with(|case| case.clone()) };
            let v8: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v6, v7));
            let v9: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v5, v8));
            let v10: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v4, v9));
            Rc::new(UH0::UH0_StandardThen(v3, v10))
        }
        _ => {
            v2
        }
    }
}
fn file_scan_start_4() -> (Rc<str>, i32, bool, Rc<str>, bool, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), 0i32, false, v0, false, false)
}
fn outside_quotes_9(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v1.wrapping_add(2i32);
            let v7: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v8: Rc<str> = string_concat(&[&*v2, " ", &*v7]);
            (v1, v2) = (v5, v8);
            continue;
        }
    }
}
fn unquoted_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = 0i32;
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    outside_quotes_9(v2, v3, v4)
}
fn method11(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_10(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method11(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn stray_target_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US2) -> bool {
    let v3: Rc<str> = std::rc::Rc::<str>::from((v1).trim_start());
    let v4: bool = &*v0 == "///> _";
    let v7: bool = if v4 {
        true
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> _ "); } LIT.with(|lit| lit.clone()) };
        let v6: bool = v0.starts_with(&*v5);
        v6
    };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> "); } LIT.with(|lit| lit.clone()) };
    let v9: bool = v0.starts_with(&*v8);
    let v11: bool = if v9 {
        let v10: bool = v7 == false;
        v10
    } else {
        false
    };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":# "); } LIT.with(|lit| lit.clone()) };
    let v13: bool = v1.starts_with(&*v12);
    let v16: bool = if v13 {
        true
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test "); } LIT.with(|lit| lit.clone()) };
        let v15: bool = v1.starts_with(&*v14);
        v15
    };
    let v20: bool = if v16 {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" static ="); } LIT.with(|lit| lit.clone()) };
        let v18: bool = v1.contains(&*v17);
        let v19: bool = v18 == false;
        v19
    } else {
        false
    };
    match v2 {
        US2::US2_BlankLine => {
            false
        }
        _ => {
            let v22: bool = if v11 {
                let v21: bool = v20 == false;
                v21
            } else {
                false
            };
            if v22 {
                let v23: bool = v3.starts_with(&*v8);
                let v24: bool = v23 == false;
                v24
            } else {
                false
            }
        }
    }
}
fn line_findings_7(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: US2, mut v6: &Rc<str>, mut v7: i32, mut v8: bool, mut v9: Rc<str>, mut v10: bool, mut v11: bool) -> Rc<str> {
    match v0 {
        US0::US0_HostFree => {
            match v5 {
                US2::US2_CodeLine => {
                    let v157: Rc<str> = unquoted_8(v4);
                    let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
                    let v159: bool = v157.contains(&*v158);
                    if v159 {
                        let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a host macro in a host-free package; reach for lib/spiral (sm', am', listm', file_system) or the OS boundary package"); } LIT.with(|lit| lit.clone()) };
                        let v162: Rc<str> = format_real_10(v3);
                        let v167: Rc<str> = string_concat(&[&*v1, ":", &*v162, "  host-free  ", &*v160, "\n"]);
                        v167
                    } else {
                        let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v168
                    }
                }
                _ => {
                    let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v170
                }
            }
        }
        US0::US0_IntegerOnly(v52) => {
            let v52: Rc<str> = v52;
            match v5 {
                US2::US2_CodeLine => {
                    let v53: bool = &*v52 == &*v2;
                    let v55: bool = if v53 {
                        let v54: bool = { let line = &*v4; line.contains("f64") || line.contains("f32") };
                        v54
                    } else {
                        false
                    };
                    if v55 {
                        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("floating point in an integer-only module"); } LIT.with(|lit| lit.clone()) };
                        let v58: Rc<str> = format_real_10(v3);
                        let v63: Rc<str> = string_concat(&[&*v1, ":", &*v58, "  integer-only  ", &*v56, "\n"]);
                        v63
                    } else {
                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v64
                    }
                }
                _ => {
                    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v66
                }
            }
        }
        US0::US0_LfOnly => {
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v41: bool = v4.ends_with(&*v40);
            if v41 {
                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CRLF line ending"); } LIT.with(|lit| lit.clone()) };
                let v44: Rc<str> = format_real_10(v3);
                let v49: Rc<str> = string_concat(&[&*v1, ":", &*v44, "  lf-only  ", &*v42, "\n"]);
                v49
            } else {
                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v50
            }
        }
        US0::US0_LibReferenced => {
            let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v172
        }
        US0::US0_LineWidth(v24) => {
            let v24: i32 = v24;
            let v25: i32 = (v4).trim_end_matches(char::from(13u8)).chars().count() as i32;
            let v26: bool = v24 < v25;
            if v26 {
                let v27: Rc<str> = format_real_10(v25);
                let v29: Rc<str> = format_real_10(v24);
                let v32: Rc<str> = format_real_10(v3);
                let v37: Rc<str> = string_concat(&[&*v1, ":", &*v32, "  line-width  ", &*v27, " columns, limit ", &*v29, "\n"]);
                v37
            } else {
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v38
            }
        }
        US0::US0_NoCommentLines => {
            match v5 {
                US2::US2_CommentLine => {
                    if v8 {
                        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v12
                    } else {
                        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a comment line; encode it in names or drop it"); } LIT.with(|lit| lit.clone()) };
                        let v15: Rc<str> = format_real_10(v3);
                        let v20: Rc<str> = string_concat(&[&*v1, ":", &*v15, "  no-comment-lines  ", &*v13, "\n"]);
                        v20
                    }
                }
                _ => {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v22
                }
            }
        }
        US0::US0_TestTargets => {
            let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":# "); } LIT.with(|lit| lit.clone()) };
            let v69: bool = v4.starts_with(&*v68);
            let v72: bool = if v69 {
                true
            } else {
                let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test "); } LIT.with(|lit| lit.clone()) };
                let v71: bool = v4.starts_with(&*v70);
                v71
            };
            let v76: bool = if v72 {
                let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" static ="); } LIT.with(|lit| lit.clone()) };
                let v74: bool = v4.contains(&*v73);
                let v75: bool = v74 == false;
                v75
            } else {
                false
            };
            let v77: Rc<str> = std::rc::Rc::<str>::from((v4).trim_start());
            let v78: bool = &*v77 == "///> _";
            let v81: bool = if v78 {
                true
            } else {
                let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> _ "); } LIT.with(|lit| lit.clone()) };
                let v80: bool = v77.starts_with(&*v79);
                v80
            };
            if v81 {
                let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> _ is retired: Cells infers the cells that never run (a file with tests never runs its main, nor does an export-only or backend-dispatching main); delete the line"); } LIT.with(|lit| lit.clone()) };
                let v84: Rc<str> = format_real_10(v3);
                let v89: Rc<str> = string_concat(&[&*v1, ":", &*v84, "  test-targets  ", &*v82, "\n"]);
                v89
            } else {
                let v91: bool = if v76 {
                    let v90: bool = v10 == false;
                    v90
                } else {
                    false
                };
                let v93: bool = if v91 {
                    let v92: bool = v11 == false;
                    v92
                } else {
                    false
                };
                if v93 {
                    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a test block in a module without a notebook header never runs; start the file with ///- --package . and a :# <module> static = prelude"); } LIT.with(|lit| lit.clone()) };
                    let v96: Rc<str> = format_real_10(v3);
                    let v101: Rc<str> = string_concat(&[&*v1, ":", &*v96, "  test-targets  ", &*v94, "\n"]);
                    v101
                } else {
                    let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
                    let v103: bool = v77.starts_with(&*v102);
                    let v107: bool = if v103 {
                        let v104: Rc<str> = string_concat(&[&*v77, " "]);
                        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -d "); } LIT.with(|lit| lit.clone()) };
                        let v106: bool = v104.contains(&*v105);
                        v106
                    } else {
                        false
                    };
                    if v107 {
                        let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("crates on a ///- package line; put -d on each test's own ///> line"); } LIT.with(|lit| lit.clone()) };
                        let v110: Rc<str> = format_real_10(v3);
                        let v115: Rc<str> = string_concat(&[&*v1, ":", &*v110, "  test-targets  ", &*v108, "\n"]);
                        v115
                    } else {
                        let v119: bool = if v76 {
                            let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> "); } LIT.with(|lit| lit.clone()) };
                            let v117: bool = v9.starts_with(&*v116);
                            let v118: bool = v117 == false;
                            v118
                        } else {
                            false
                        };
                        if v119 {
                            let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test block without its own ///> target line directly above it"); } LIT.with(|lit| lit.clone()) };
                            let v122: Rc<str> = format_real_10(v3);
                            let v127: Rc<str> = string_concat(&[&*v1, ":", &*v122, "  test-targets  ", &*v120, "\n"]);
                            v127
                        } else {
                            let v128: bool = stray_target_12(v9.clone(), v4.clone(), v5.clone());
                            let v129: bool = v128 && v72;
                            let v132: bool = if v129 {
                                let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" static ="); } LIT.with(|lit| lit.clone()) };
                                let v131: bool = v4.contains(&*v130);
                                v131
                            } else {
                                false
                            };
                            if v132 {
                                let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a ///> target above a static block; static blocks carry no target"); } LIT.with(|lit| lit.clone()) };
                                let v135: Rc<str> = format_real_10(v3);
                                let v140: Rc<str> = string_concat(&[&*v1, ":", &*v135, "  test-targets  ", &*v133, "\n"]);
                                v140
                            } else {
                                let v141: bool = stray_target_12(v9, v4, v5);
                                if v141 {
                                    let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a ///> target not directly above a test block; targets sit on each test"); } LIT.with(|lit| lit.clone()) };
                                    let v144: Rc<str> = format_real_10(v3);
                                    let v149: Rc<str> = string_concat(&[&*v1, ":", &*v144, "  test-targets  ", &*v142, "\n"]);
                                    v149
                                } else {
                                    let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v150
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn rules_applied_6(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: US2, mut v6: &Rc<str>, mut v7: i32, mut v8: bool, mut v9: Rc<str>, mut v10: bool, mut v11: bool, mut v12: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_NoStandards => {
                return v12;
            }
            UH0::UH0_StandardThen(v13, v14) => {
                let v13: US0 = v13;
                let v14: Rc<UH0> = v14;
                let v15: Rc<str> = line_findings_7(v13, v1.clone(), v2.clone(), v3, v4.clone(), v5.clone(), v6, v7, v8, v9.clone(), v10, v11);
                let v16: Rc<str> = string_concat(&[&*v12, &*v15]);
                (v0, v12) = (v14, v16);
                continue;
            }
        }
    }
}
fn lines_scanned_5(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: bool, mut v8: Rc<str>, mut v9: bool, mut v10: bool) -> (Rc<str>, i32, bool, Rc<str>, bool, bool) {
    loop {
        let v11: bool = &*v3 == "";
        if v11 {
            return (v5, v6, v7, v8, v9, v10);
        } else {
            let v12: Rc<str> = std::rc::Rc::<str>::from((v3).split(char::from(10u8)).next().unwrap_or(""));
            let v13: Rc<str> = std::rc::Rc::<str>::from((v12).trim_start());
            let v14: bool = &*v13 == "";
            let v16: bool = if v14 {
                true
            } else {
                let v15: bool = &*v13 == "\r";
                v15
            };
            let v31: US2 = if v16 {
                US2::US2_BlankLine
            } else {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///"); } LIT.with(|lit| lit.clone()) };
                let v19: bool = v13.starts_with(&*v18);
                if v19 {
                    US2::US2_DirectiveLine
                } else {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//"); } LIT.with(|lit| lit.clone()) };
                    let v22: bool = v13.starts_with(&*v21);
                    if v22 {
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TODO"); } LIT.with(|lit| lit.clone()) };
                        let v24: bool = v13.contains(&*v23);
                        if v24 {
                            US2::US2_TodoLine
                        } else {
                            US2::US2_CommentLine
                        }
                    } else {
                        US2::US2_CodeLine
                    }
                }
            };
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = rules_applied_6(v0.clone(), v1.clone(), v2.clone(), v4, v12.clone(), v31.clone(), &v5, v6, v7, v8.clone(), v9, v10, v32);
            let v35: bool = match &v31 {
                US2::US2_CommentLine => {
                    v7
                }
                US2::US2_TodoLine => {
                    true
                }
                _ => {
                    false
                }
            };
            let v37: Rc<str> = match v31 {
                US2::US2_BlankLine => {
                    v8.clone()
                }
                _ => {
                    let v36: Rc<str> = std::rc::Rc::<str>::from((v12).trim_start());
                    v36
                }
            };
            let v44: bool = if v9 {
                true
            } else {
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":# "); } LIT.with(|lit| lit.clone()) };
                let v39: bool = v12.starts_with(&*v38);
                if v39 {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" static ="); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v12.contains(&*v40);
                    let v42: bool = v41 == false;
                    v42
                } else {
                    false
                }
            };
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":# "); } LIT.with(|lit| lit.clone()) };
            let v46: bool = v12.starts_with(&*v45);
            let v49: bool = if v46 {
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" static ="); } LIT.with(|lit| lit.clone()) };
                let v48: bool = v12.contains(&*v47);
                v48
            } else {
                false
            };
            let v50: Rc<str> = std::rc::Rc::<str>::from((v12).trim_start());
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
            let v52: bool = v50.starts_with(&*v51);
            let v53: bool = v4 == 2i32;
            let v56: bool = if v53 {
                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> "); } LIT.with(|lit| lit.clone()) };
                let v55: bool = v8.starts_with(&*v54);
                v55
            } else {
                false
            };
            let v60: bool = if v10 {
                true
            } else {
                if v52 {
                    let v57: bool = v4 == 1i32;
                    let v58: bool = v57 || v56;
                    v58
                } else {
                    false
                }
            };
            let v61: bool = v60 || v49;
            let v62: Rc<str> = string_concat(&[&*v5, &*v33]);
            let v63: i32 = (v33).lines().count() as i32;
            let v64: i32 = v6.wrapping_add(v63);
            let v65: Rc<str> = std::rc::Rc::<str>::from((v3).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let v66: i32 = v4.wrapping_add(1i32);
            (v3, v4, v5, v6, v7, v8, v9, v10) = (v65, v66, v62, v64, v35, v37, v44, v61);
            continue;
        }
    }
}
fn file_findings_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<UH0> = declared_or_default_1(v0);
    let v5: i32 = 1i32;
    let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11): (Rc<str>, i32, bool, Rc<str>, bool, bool) = file_scan_start_4();
    let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17): (Rc<str>, i32, bool, Rc<str>, bool, bool) = lines_scanned_5(v4, v1.clone(), v2, v3.clone(), v5, v6, v7, v8, v9, v10, v11);
    let v18: Rc<str> = rig_rec_cycle::rig_rec_cycle_findings(&*v1, &*v3);
    let v19: Rc<str> = string_concat(&[&*v12, &*v18]);
    v19
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        file_findings_0(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_standards_file_findings(v0: &str, v1: &str, v2: &str, v3: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
