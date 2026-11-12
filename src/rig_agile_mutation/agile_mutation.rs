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
enum US1 {
    US1_AgileInit,
    US1_AgileBegin,
    US1_AgileLease,
    US1_AgileStatus,
    US1_AgileHandoff,
    US1_AgileList,
    US1_AgileCheck,
    US1_AgileRecord,
    US1_AgileSet,
    US1_AgileAdd,
    US1_AgileGap,
    US1_AgileEdit,
    US1_AgileShow,
    US1_AgilePlan,
    US1_AgileMigrateTags,
    US1_AgileMigrateActivity,
    US1_AgileSprint,
    US1_AgileLint,
    US1_AgileGate,
    US1_AgileCadence,
    US1_AgileNote,
    US1_AgileNoteEdit,
    US1_AgileNotes,
    US1_AgileRename,
    US1_AgileSeal,
    US1_AgileTodos,
    US1_AgileNext,
    US1_AgileClaim,
    US1_AgileRelease,
    US1_AgileClaims,
    US1_AgileRoots,
    US1_AgileLinks,
    US1_AgileWip,
    US1_AgileViews,
    US1_AgileView,
    US1_AgileViewAdd,
    US1_AgileDialect,
    US1_AgileReceipts,
    US1_AgileTerms,
    US1_AgileChanges,
    US1_AgileAsk,
}
#[derive(Clone)]
enum US0 {
    US0_AgileKnown(US1),
    US0_AgileUnknownName,
}
#[derive(Clone)]
enum US2 {
    US2_RootLeads,
    US2_RootAfterMode,
}
#[derive(Clone)]
enum US3 {
    US3_RootNamed(Rc<str>),
    US3_RootFlagged(Rc<str>),
}
fn state_lock_refused_1() -> i32 {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile state/agile.spi stayed locked by a live writer for 5 min; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
    let v1: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v0 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
    v1
}
fn agile_usage_error_2() -> i32 {
    let v0: i32 = rig_agile_routes::rig_agile_usage_error();
    v0
}
fn agile_subcommand_of_3(mut v0: Rc<str>) -> US0 {
    let v1: bool = &*v0 == "init";
    let v5: US0 = if v1 {
        let v2: US1 = US1::US1_AgileInit;
        US0::US0_AgileKnown(v2)
    } else {
        US0::US0_AgileUnknownName
    };
    let v6: bool = &*v0 == "begin";
    let v9: US0 = if v6 {
        let v7: US1 = US1::US1_AgileBegin;
        US0::US0_AgileKnown(v7)
    } else {
        v5
    };
    let v10: bool = &*v0 == "lease";
    let v13: US0 = if v10 {
        let v11: US1 = US1::US1_AgileLease;
        US0::US0_AgileKnown(v11)
    } else {
        v9
    };
    let v14: bool = &*v0 == "status";
    let v17: US0 = if v14 {
        let v15: US1 = US1::US1_AgileStatus;
        US0::US0_AgileKnown(v15)
    } else {
        v13
    };
    let v18: bool = &*v0 == "handoff";
    let v21: US0 = if v18 {
        let v19: US1 = US1::US1_AgileHandoff;
        US0::US0_AgileKnown(v19)
    } else {
        v17
    };
    let v22: bool = &*v0 == "list";
    let v25: US0 = if v22 {
        let v23: US1 = US1::US1_AgileList;
        US0::US0_AgileKnown(v23)
    } else {
        v21
    };
    let v26: bool = &*v0 == "check";
    let v29: US0 = if v26 {
        let v27: US1 = US1::US1_AgileCheck;
        US0::US0_AgileKnown(v27)
    } else {
        v25
    };
    let v30: bool = &*v0 == "record";
    let v33: US0 = if v30 {
        let v31: US1 = US1::US1_AgileRecord;
        US0::US0_AgileKnown(v31)
    } else {
        v29
    };
    let v34: bool = &*v0 == "set";
    let v37: US0 = if v34 {
        let v35: US1 = US1::US1_AgileSet;
        US0::US0_AgileKnown(v35)
    } else {
        v33
    };
    let v38: bool = &*v0 == "add";
    let v41: US0 = if v38 {
        let v39: US1 = US1::US1_AgileAdd;
        US0::US0_AgileKnown(v39)
    } else {
        v37
    };
    let v42: bool = &*v0 == "gap";
    let v45: US0 = if v42 {
        let v43: US1 = US1::US1_AgileGap;
        US0::US0_AgileKnown(v43)
    } else {
        v41
    };
    let v46: bool = &*v0 == "edit";
    let v49: US0 = if v46 {
        let v47: US1 = US1::US1_AgileEdit;
        US0::US0_AgileKnown(v47)
    } else {
        v45
    };
    let v50: bool = &*v0 == "show";
    let v53: US0 = if v50 {
        let v51: US1 = US1::US1_AgileShow;
        US0::US0_AgileKnown(v51)
    } else {
        v49
    };
    let v54: bool = &*v0 == "plan";
    let v57: US0 = if v54 {
        let v55: US1 = US1::US1_AgilePlan;
        US0::US0_AgileKnown(v55)
    } else {
        v53
    };
    let v58: bool = &*v0 == "migrate-tags";
    let v61: US0 = if v58 {
        let v59: US1 = US1::US1_AgileMigrateTags;
        US0::US0_AgileKnown(v59)
    } else {
        v57
    };
    let v62: bool = &*v0 == "migrate-activity";
    let v65: US0 = if v62 {
        let v63: US1 = US1::US1_AgileMigrateActivity;
        US0::US0_AgileKnown(v63)
    } else {
        v61
    };
    let v66: bool = &*v0 == "sprint";
    let v69: US0 = if v66 {
        let v67: US1 = US1::US1_AgileSprint;
        US0::US0_AgileKnown(v67)
    } else {
        v65
    };
    let v70: bool = &*v0 == "lint";
    let v73: US0 = if v70 {
        let v71: US1 = US1::US1_AgileLint;
        US0::US0_AgileKnown(v71)
    } else {
        v69
    };
    let v74: bool = &*v0 == "gate";
    let v77: US0 = if v74 {
        let v75: US1 = US1::US1_AgileGate;
        US0::US0_AgileKnown(v75)
    } else {
        v73
    };
    let v78: bool = &*v0 == "cadence";
    let v81: US0 = if v78 {
        let v79: US1 = US1::US1_AgileCadence;
        US0::US0_AgileKnown(v79)
    } else {
        v77
    };
    let v82: bool = &*v0 == "note";
    let v85: US0 = if v82 {
        let v83: US1 = US1::US1_AgileNote;
        US0::US0_AgileKnown(v83)
    } else {
        v81
    };
    let v86: bool = &*v0 == "note-edit";
    let v89: US0 = if v86 {
        let v87: US1 = US1::US1_AgileNoteEdit;
        US0::US0_AgileKnown(v87)
    } else {
        v85
    };
    let v90: bool = &*v0 == "notes";
    let v93: US0 = if v90 {
        let v91: US1 = US1::US1_AgileNotes;
        US0::US0_AgileKnown(v91)
    } else {
        v89
    };
    let v94: bool = &*v0 == "rename";
    let v97: US0 = if v94 {
        let v95: US1 = US1::US1_AgileRename;
        US0::US0_AgileKnown(v95)
    } else {
        v93
    };
    let v98: bool = &*v0 == "seal";
    let v101: US0 = if v98 {
        let v99: US1 = US1::US1_AgileSeal;
        US0::US0_AgileKnown(v99)
    } else {
        v97
    };
    let v102: bool = &*v0 == "todos";
    let v105: US0 = if v102 {
        let v103: US1 = US1::US1_AgileTodos;
        US0::US0_AgileKnown(v103)
    } else {
        v101
    };
    let v106: bool = &*v0 == "next";
    let v109: US0 = if v106 {
        let v107: US1 = US1::US1_AgileNext;
        US0::US0_AgileKnown(v107)
    } else {
        v105
    };
    let v110: bool = &*v0 == "claim";
    let v113: US0 = if v110 {
        let v111: US1 = US1::US1_AgileClaim;
        US0::US0_AgileKnown(v111)
    } else {
        v109
    };
    let v114: bool = &*v0 == "release";
    let v117: US0 = if v114 {
        let v115: US1 = US1::US1_AgileRelease;
        US0::US0_AgileKnown(v115)
    } else {
        v113
    };
    let v118: bool = &*v0 == "claims";
    let v121: US0 = if v118 {
        let v119: US1 = US1::US1_AgileClaims;
        US0::US0_AgileKnown(v119)
    } else {
        v117
    };
    let v122: bool = &*v0 == "roots";
    let v125: US0 = if v122 {
        let v123: US1 = US1::US1_AgileRoots;
        US0::US0_AgileKnown(v123)
    } else {
        v121
    };
    let v126: bool = &*v0 == "links";
    let v129: US0 = if v126 {
        let v127: US1 = US1::US1_AgileLinks;
        US0::US0_AgileKnown(v127)
    } else {
        v125
    };
    let v130: bool = &*v0 == "wip";
    let v133: US0 = if v130 {
        let v131: US1 = US1::US1_AgileWip;
        US0::US0_AgileKnown(v131)
    } else {
        v129
    };
    let v134: bool = &*v0 == "views";
    let v137: US0 = if v134 {
        let v135: US1 = US1::US1_AgileViews;
        US0::US0_AgileKnown(v135)
    } else {
        v133
    };
    let v138: bool = &*v0 == "view";
    let v141: US0 = if v138 {
        let v139: US1 = US1::US1_AgileView;
        US0::US0_AgileKnown(v139)
    } else {
        v137
    };
    let v142: bool = &*v0 == "view-add";
    let v145: US0 = if v142 {
        let v143: US1 = US1::US1_AgileViewAdd;
        US0::US0_AgileKnown(v143)
    } else {
        v141
    };
    let v146: bool = &*v0 == "dialect";
    let v149: US0 = if v146 {
        let v147: US1 = US1::US1_AgileDialect;
        US0::US0_AgileKnown(v147)
    } else {
        v145
    };
    let v150: bool = &*v0 == "receipts";
    let v153: US0 = if v150 {
        let v151: US1 = US1::US1_AgileReceipts;
        US0::US0_AgileKnown(v151)
    } else {
        v149
    };
    let v154: bool = &*v0 == "terms";
    let v157: US0 = if v154 {
        let v155: US1 = US1::US1_AgileTerms;
        US0::US0_AgileKnown(v155)
    } else {
        v153
    };
    let v158: bool = &*v0 == "changes";
    let v161: US0 = if v158 {
        let v159: US1 = US1::US1_AgileChanges;
        US0::US0_AgileKnown(v159)
    } else {
        v157
    };
    let v162: bool = &*v0 == "ask";
    if v162 {
        let v163: US1 = US1::US1_AgileAsk;
        US0::US0_AgileKnown(v163)
    } else {
        v161
    }
}
fn state_lock_held_4(mut v0: bool, mut v1: Rc<str>) -> bool {
    if v0 {
        let v2: bool = rig_agile_lock::rig_agile_lock_taken(&v1);
        v2
    } else {
        true
    }
}
fn term_warning_6() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from({ let path = std::env::var("RIG_AGILE_TERMS").ok().filter(|p| !p.is_empty()) .or_else(|| std::env::var("LOCALAPPDATA").ok().map(|base| format!("{base}/rig/agile-terms.txt"))) .unwrap_or_default(); let rules = std::fs::read_to_string(&path).unwrap_or_default(); let written = std::env::args().skip(4).collect::<Vec<_>>().join(" "); let mut found = Vec::new(); for line in rules.lines() { if let Some((kind, pattern)) = line.split_once(char::from(9u8)) { if let Ok(rule) = regex::RegexBuilder::new(pattern).case_insensitive(true).build() { if let Some(hit) = rule.find(&written) { found.push(format!("{kind} {:?} at {}", hit.as_str(), hit.start())); } } } } if found.is_empty() { String::new() } else { format!("rig agile warning: operational terms in the text ({}); the committed backlog holds project facts only, rules from {path}", found.join("; ")) } });
    v0
}
fn terms_warned_5() -> i32 {
    let v0: Rc<str> = term_warning_6();
    let v1: bool = &*v0 == "";
    if v1 {
        0i32
    } else {
        let v2: i32 = std::io::Write::write_all(&mut std::io::stderr(), (v0.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        v2
    }
}
fn agile_argument_count_7() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: bool = v1 > 2i32;
    if v2 {
        let v3: i32 = v1.wrapping_sub(2i32);
        v3
    } else {
        0i32
    }
}
fn method8() -> i32 {
    let v0: i32 = agile_argument_count_7();
    let v1: bool = 2i32 <= v0;
    if v1 {
        let v2: i32 = agile_claim_cli::rig_agile_claim_run();
        v2
    } else {
        agile_usage_error_2()
    }
}
fn method9() -> i32 {
    let v0: i32 = 2i32;
    let v1: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v0 ;
    if v1 {
        let v2: i32 = 1i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v4: i32 = rig_agile_state::agile_lease_status_owned(v3.as_ref());
        v4
    } else {
        agile_usage_error_2()
    }
}
fn terms_report_10(mut v0: Rc<str>) -> i32 {
    let v1: i32 = { let path = std::env::var("RIG_AGILE_TERMS").ok().filter(|p| !p.is_empty()) .or_else(|| std::env::var("LOCALAPPDATA").ok().map(|base| format!("{base}/rig/agile-terms.txt"))) .unwrap_or_default(); let rules = std::fs::read_to_string(&path).unwrap_or_default(); let state = std::path::Path::new(&*v0).join("state"); let mut lines = Vec::new(); for entry in std::fs::read_dir(&state).into_iter().flatten().flatten() { if entry.path().extension().and_then(|e| e.to_str()) == Some("spi") { let text = std::fs::read_to_string(entry.path()).unwrap_or_default(); lines.extend(text.lines().map(str::to_owned)); } } let mut count = 0; for line in rules.lines() { if let Some((kind, pattern)) = line.split_once(char::from(9u8)) { count += 1; let hits = regex::RegexBuilder::new(pattern).case_insensitive(true).build() .map(|rule| lines.iter().filter(|row| rule.is_match(row)).count().to_string()) .unwrap_or_else(|_| "invalid".to_owned()); println!("rig agile terms rule kind={kind} state_lines={hits} pattern={pattern}"); } } println!("rig agile terms ok rules={count} source={path}"); 0i32 };
    v1
}
fn method11() -> i32 {
    let v0: i32 = agile_argument_count_7();
    let v1: bool = 2i32 <= v0;
    if v1 {
        let v2: i32 = agile_task_views::rig_agile_views_run();
        v2
    } else {
        agile_usage_error_2()
    }
}
fn state_lock_released_13(mut v0: Rc<str>) -> () {
    let v1: bool = rig_agile_lock::rig_agile_lock_released(&v0);
    ()
}
fn state_lock_done_12(mut v0: bool, mut v1: Rc<str>) -> () {
    if v0 {
        state_lock_released_13(v1)
    }
}
fn agile_entry_0() -> i32 {
    let v0: i32 = 0i32;
    let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v2: US0 = agile_subcommand_of_3(v1);
    match v2 {
        US0::US0_AgileKnown(v3) => {
            let v3: US1 = v3;
            let v12: US2 = match &v3 {
                US1::US1_AgileMigrateActivity => {
                    US2::US2_RootAfterMode
                }
                US1::US1_AgileMigrateTags => {
                    US2::US2_RootAfterMode
                }
                US1::US1_AgileNotes => {
                    US2::US2_RootAfterMode
                }
                US1::US1_AgileSprint => {
                    US2::US2_RootAfterMode
                }
                _ => {
                    US2::US2_RootLeads
                }
            };
            let v14: i32 = match v12 {
                US2::US2_RootAfterMode => {
                    2i32
                }
                US2::US2_RootLeads => {
                    1i32
                }
            };
            let v15: Rc<str> = usize::try_from(v14).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
            let v17: bool = v15.starts_with(&*v16);
            let v20: US3 = if v17 {
                US3::US3_RootFlagged(v15)
            } else {
                US3::US3_RootNamed(v15)
            };
            match v20 {
                US3::US3_RootFlagged(v21) => {
                    let v21: Rc<str> = v21;
                    let v22: i32 = 0i32;
                    let v23: Rc<str> = usize::try_from(v22).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let v24: Rc<str> = rig_command_help::rig_agile_root_flag_refusal(&*v23, &*v21);
                    let v25: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v24 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v25
                }
                US3::US3_RootNamed(v26) => {
                    let v26: Rc<str> = v26;
                    let v27: i32 = 0i32;
                    let v28: Rc<str> = usize::try_from(v27).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let v29: bool = agile_state_access::rig_agile_writes_named(&v28);
                    let v35: bool = if v29 {
                        true
                    } else {
                        match &v3 {
                            US1::US1_AgileClaim => {
                                true
                            }
                            US1::US1_AgileNext => {
                                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--any"); } LIT.with(|lit| lit.clone()) };
                                let v31: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v30);
                                v31
                            }
                            US1::US1_AgileRelease => {
                                true
                            }
                            _ => {
                                false
                            }
                        }
                    };
                    let v36: bool = state_lock_held_4(v35, v26.clone());
                    if v36 {
                        let v38: i32 = if v35 {
                            terms_warned_5()
                        } else {
                            0i32
                        };
                        let v316: i32 = match v3 {
                            US1::US1_AgileAdd => {
                                let v125: i32 = agile_argument_count_7();
                                let v126: bool = 8i32 <= v125;
                                if v126 {
                                    let v127: i32 = agile_task_creation::rig_agile_add_run();
                                    v127
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileAsk => {
                                let v140: i32 = agile_argument_count_7();
                                let v141: bool = 2i32 <= v140;
                                if v141 {
                                    let v142: i32 = rig_agile_routes::rig_agile_ask_route();
                                    v142
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileBegin => {
                                let v52: i32 = 3i32;
                                let v53: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v52 ;
                                if v53 {
                                    let v54: i32 = rig_agile_routes::rig_agile_begin_route();
                                    v54
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileCadence => {
                                let v200: i32 = agile_argument_count_7();
                                let v201: bool = 2i32 <= v200;
                                if v201 {
                                    let v202: i32 = rig_agile_cadence::rig_agile_cadence_run();
                                    v202
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileChanges => {
                                let v135: i32 = agile_argument_count_7();
                                let v136: bool = 2i32 <= v135;
                                if v136 {
                                    let v137: i32 = rig_agile_changes::rig_agile_changes_run();
                                    v137
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileCheck => {
                                let v71: i32 = 2i32;
                                let v72: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v71 ;
                                if v72 {
                                    let v73: i32 = 1i32;
                                    let v74: Rc<str> = usize::try_from(v73).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v75: i32 = rig_agile_state::agile_check_owned(v74.as_ref());
                                    v75
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileClaim => {
                                method8()
                            }
                            US1::US1_AgileClaims => {
                                let v253: i32 = agile_argument_count_7();
                                let v254: bool = 2i32 <= v253;
                                if v254 {
                                    let v255: i32 = agile_claim_cli::rig_agile_claims_run();
                                    v255
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileDialect => {
                                let v47: i32 = 3i32;
                                let v48: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v47 ;
                                if v48 {
                                    let v49: i32 = agile_backlog_scaffold::rig_agile_dialect_run();
                                    v49
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileEdit => {
                                let v157: i32 = 5i32;
                                let v158: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v157 ;
                                let v161: bool = if v158 {
                                    true
                                } else {
                                    let v159: i32 = 7i32;
                                    let v160: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v159 ;
                                    v160
                                };
                                if v161 {
                                    let v162: i32 = rig_agile_routes::rig_agile_edit_route();
                                    v162
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileGap => {
                                let v130: i32 = agile_argument_count_7();
                                let v131: bool = 2i32 <= v130;
                                if v131 {
                                    let v132: i32 = rig_agile_gap::rig_agile_gap_run();
                                    v132
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileGate => {
                                let v195: i32 = agile_argument_count_7();
                                let v196: bool = 2i32 <= v195;
                                if v196 {
                                    let v197: i32 = rig_agile_gate::rig_agile_gate_run();
                                    v197
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileHandoff => {
                                let v59: i32 = 2i32;
                                let v60: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v59 ;
                                if v60 {
                                    let v61: i32 = 1i32;
                                    let v62: Rc<str> = usize::try_from(v61).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v63: i32 = rig_handoff_render::handoff_render_owned(v62.as_ref());
                                    v63
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileInit => {
                                let v39: i32 = 2i32;
                                let v40: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v39 ;
                                let v43: bool = if v40 {
                                    true
                                } else {
                                    let v41: i32 = 4i32;
                                    let v42: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v41 ;
                                    v42
                                };
                                if v43 {
                                    let v44: i32 = agile_backlog_scaffold::rig_agile_init_run();
                                    v44
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileLease => {
                                method9()
                            }
                            US1::US1_AgileLinks => {
                                let v263: i32 = agile_argument_count_7();
                                let v264: bool = 2i32 <= v263;
                                if v264 {
                                    let v265: i32 = agile_memory_links::rig_agile_links_run();
                                    v265
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileLint => {
                                let v190: i32 = agile_argument_count_7();
                                let v191: bool = 2i32 <= v190;
                                if v191 {
                                    let v192: i32 = rig_agile_lint::rig_agile_lint_run();
                                    v192
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileList => {
                                let v66: i32 = agile_argument_count_7();
                                let v67: bool = 2i32 <= v66;
                                if v67 {
                                    let v68: i32 = agile_task_tree::rig_agile_list_routed();
                                    v68
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileMigrateActivity => {
                                let v180: i32 = agile_argument_count_7();
                                let v181: bool = 2i32 <= v180;
                                if v181 {
                                    let v182: i32 = rig_agile_activity_migration::rig_agile_migrate_activity();
                                    v182
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileMigrateTags => {
                                let v175: i32 = agile_argument_count_7();
                                let v176: bool = 2i32 <= v175;
                                if v176 {
                                    let v177: i32 = agile_planning_migration::rig_agile_migrate_tags_run();
                                    v177
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNext => {
                                let v242: i32 = agile_argument_count_7();
                                let v243: bool = 2i32 <= v242;
                                if v243 {
                                    let v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--any"); } LIT.with(|lit| lit.clone()) };
                                    let v245: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v244);
                                    if v245 {
                                        let v246: i32 = agile_claim_cli::rig_agile_claim_run();
                                        v246
                                    } else {
                                        let v247: i32 = agile_task_next::rig_agile_next_run();
                                        v247
                                    }
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNote => {
                                let v205: i32 = 4i32;
                                let v206: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v205 ;
                                let v209: bool = if v206 {
                                    true
                                } else {
                                    let v207: i32 = 6i32;
                                    let v208: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v207 ;
                                    v208
                                };
                                if v209 {
                                    let v210: i32 = agile_task_notes::rig_agile_note_run();
                                    v210
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNoteEdit => {
                                let v213: i32 = agile_argument_count_7();
                                let v214: bool = 2i32 <= v213;
                                if v214 {
                                    let v215: i32 = agile_note_edit::rig_agile_note_edit_run();
                                    v215
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNotes => {
                                let v218: i32 = 4i32;
                                let v219: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v218 ;
                                if v219 {
                                    let v220: i32 = agile_title_notes::rig_agile_notes_from_title_run();
                                    v220
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgilePlan => {
                                let v170: i32 = agile_argument_count_7();
                                let v171: bool = 2i32 <= v170;
                                if v171 {
                                    let v172: i32 = agile_planning_rollup::rig_agile_planning_rollup_run();
                                    v172
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileReceipts => {
                                let v145: i32 = agile_argument_count_7();
                                let v146: bool = 2i32 <= v145;
                                if v146 {
                                    let v147: i32 = rig_agile_receipts::rig_agile_receipts_verify_run();
                                    v147
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRecord => {
                                let v78: i32 = 2i32;
                                let v79: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v78 ;
                                if v79 {
                                    let v80: i32 = 1i32;
                                    let v81: Rc<str> = usize::try_from(v80).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v82: i32 = rig_agile_state::agile_record_owned(v81.as_ref());
                                    v82
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRelease => {
                                method8()
                            }
                            US1::US1_AgileRename => {
                                let v223: i32 = 4i32;
                                let v224: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v223 ;
                                if v224 {
                                    let v225: i32 = agile_task_rename::rig_agile_rename_run();
                                    v225
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRoots => {
                                let v258: i32 = agile_argument_count_7();
                                let v259: bool = 2i32 <= v258;
                                if v259 {
                                    let v260: i32 = agile_todo_roots::rig_agile_roots_run();
                                    v260
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSeal => {
                                let v228: i32 = agile_argument_count_7();
                                let v229: bool = 2i32 <= v228;
                                if v229 {
                                    let v230: i32 = agile_note_seal::rig_agile_seal_run();
                                    v230
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSet => {
                                let v85: i32 = 5i32;
                                let v86: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v85 ;
                                let v121: bool = if v86 {
                                    true
                                } else {
                                    let v87: i32 = 7i32;
                                    let v88: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v87 ;
                                    if v88 {
                                        let v89: i32 = 5i32;
                                        let v90: Rc<str> = usize::try_from(v89).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let v91: bool = &*v90 == "--expect";
                                        if v91 {
                                            true
                                        } else {
                                            let v92: i32 = 5i32;
                                            let v93: Rc<str> = usize::try_from(v92).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                            let v94: bool = &*v93 == "--as";
                                            v94
                                        }
                                    } else {
                                        let v96: i32 = 9i32;
                                        let v97: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v96 ;
                                        if v97 {
                                            let v98: i32 = 5i32;
                                            let v99: Rc<str> = usize::try_from(v98).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                            let v100: bool = &*v99 == "--expect";
                                            let v104: bool = if v100 {
                                                true
                                            } else {
                                                let v101: i32 = 5i32;
                                                let v102: Rc<str> = usize::try_from(v101).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v103: bool = &*v102 == "--as";
                                                v103
                                            };
                                            let v112: bool = if v104 {
                                                let v105: i32 = 7i32;
                                                let v106: Rc<str> = usize::try_from(v105).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v107: bool = &*v106 == "--expect";
                                                if v107 {
                                                    true
                                                } else {
                                                    let v108: i32 = 7i32;
                                                    let v109: Rc<str> = usize::try_from(v108).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                    let v110: bool = &*v109 == "--as";
                                                    v110
                                                }
                                            } else {
                                                false
                                            };
                                            if v112 {
                                                let v113: i32 = 5i32;
                                                let v114: Rc<str> = usize::try_from(v113).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v115: i32 = 7i32;
                                                let v116: Rc<str> = usize::try_from(v115).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v117: bool = &*v114 != &*v116;
                                                v117
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        }
                                    }
                                };
                                if v121 {
                                    let v122: i32 = rig_agile_routes::rig_agile_set_route();
                                    v122
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileShow => {
                                let v165: i32 = agile_argument_count_7();
                                let v166: bool = 2i32 <= v165;
                                if v166 {
                                    let v167: i32 = agile_task_inspection::rig_agile_show_run();
                                    v167
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSprint => {
                                let v185: i32 = agile_argument_count_7();
                                let v186: bool = 2i32 <= v185;
                                if v186 {
                                    let v187: i32 = rig_agile_sprint::rig_agile_sprint_run();
                                    v187
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileStatus => {
                                method9()
                            }
                            US1::US1_AgileTerms => {
                                let v150: i32 = 2i32;
                                let v151: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v150 ;
                                if v151 {
                                    let v152: i32 = 1i32;
                                    let v153: Rc<str> = usize::try_from(v152).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    terms_report_10(v153)
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileTodos => {
                                let v233: i32 = agile_argument_count_7();
                                let v234: bool = 2i32 <= v233;
                                if v234 {
                                    let v235: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--record"); } LIT.with(|lit| lit.clone()) };
                                    let v236: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v235);
                                    if v236 {
                                        let v237: i32 = agile_todo_record::rig_agile_todos_record_run();
                                        v237
                                    } else {
                                        let v238: i32 = agile_todo_scan::rig_agile_todos_run();
                                        v238
                                    }
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileView => {
                                method11()
                            }
                            US1::US1_AgileViewAdd => {
                                method11()
                            }
                            US1::US1_AgileViews => {
                                method11()
                            }
                            US1::US1_AgileWip => {
                                let v268: i32 = agile_argument_count_7();
                                let v269: bool = 2i32 <= v268;
                                if v269 {
                                    let v270: i32 = agile_todo_merge::rig_agile_wip_run();
                                    v270
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                        };
                        state_lock_done_12(v35, v26);
                        v316
                    } else {
                        let v321: bool = match v3 {
                            US1::US1_AgileClaim => {
                                true
                            }
                            US1::US1_AgileNext => {
                                let v317: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--any"); } LIT.with(|lit| lit.clone()) };
                                let v318: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v317);
                                v318
                            }
                            US1::US1_AgileRelease => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        if v321 {
                            let v322: i32 = 0i32;
                            let v323: Rc<str> = usize::try_from(v322).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let v325: Rc<str> = string_concat(&["rig agile ", &*v323, " refused reason=contention"]);
                            let v326: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v325.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                            4i32
                        } else {
                            state_lock_refused_1()
                        }
                    }
                }
            }
        }
        US0::US0_AgileUnknownName => {
            agile_usage_error_2()
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_run() -> i32 {
    closure0()()
}
