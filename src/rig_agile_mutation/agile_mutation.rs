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
    US1_AgileRoots,
    US1_AgileLinks,
    US1_AgileWip,
    US1_AgileViews,
    US1_AgileView,
    US1_AgileViewAdd,
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
    let v42: bool = &*v0 == "edit";
    let v45: US0 = if v42 {
        let v43: US1 = US1::US1_AgileEdit;
        US0::US0_AgileKnown(v43)
    } else {
        v41
    };
    let v46: bool = &*v0 == "show";
    let v49: US0 = if v46 {
        let v47: US1 = US1::US1_AgileShow;
        US0::US0_AgileKnown(v47)
    } else {
        v45
    };
    let v50: bool = &*v0 == "plan";
    let v53: US0 = if v50 {
        let v51: US1 = US1::US1_AgilePlan;
        US0::US0_AgileKnown(v51)
    } else {
        v49
    };
    let v54: bool = &*v0 == "migrate-tags";
    let v57: US0 = if v54 {
        let v55: US1 = US1::US1_AgileMigrateTags;
        US0::US0_AgileKnown(v55)
    } else {
        v53
    };
    let v58: bool = &*v0 == "migrate-activity";
    let v61: US0 = if v58 {
        let v59: US1 = US1::US1_AgileMigrateActivity;
        US0::US0_AgileKnown(v59)
    } else {
        v57
    };
    let v62: bool = &*v0 == "sprint";
    let v65: US0 = if v62 {
        let v63: US1 = US1::US1_AgileSprint;
        US0::US0_AgileKnown(v63)
    } else {
        v61
    };
    let v66: bool = &*v0 == "lint";
    let v69: US0 = if v66 {
        let v67: US1 = US1::US1_AgileLint;
        US0::US0_AgileKnown(v67)
    } else {
        v65
    };
    let v70: bool = &*v0 == "gate";
    let v73: US0 = if v70 {
        let v71: US1 = US1::US1_AgileGate;
        US0::US0_AgileKnown(v71)
    } else {
        v69
    };
    let v74: bool = &*v0 == "cadence";
    let v77: US0 = if v74 {
        let v75: US1 = US1::US1_AgileCadence;
        US0::US0_AgileKnown(v75)
    } else {
        v73
    };
    let v78: bool = &*v0 == "note";
    let v81: US0 = if v78 {
        let v79: US1 = US1::US1_AgileNote;
        US0::US0_AgileKnown(v79)
    } else {
        v77
    };
    let v82: bool = &*v0 == "note-edit";
    let v85: US0 = if v82 {
        let v83: US1 = US1::US1_AgileNoteEdit;
        US0::US0_AgileKnown(v83)
    } else {
        v81
    };
    let v86: bool = &*v0 == "notes";
    let v89: US0 = if v86 {
        let v87: US1 = US1::US1_AgileNotes;
        US0::US0_AgileKnown(v87)
    } else {
        v85
    };
    let v90: bool = &*v0 == "rename";
    let v93: US0 = if v90 {
        let v91: US1 = US1::US1_AgileRename;
        US0::US0_AgileKnown(v91)
    } else {
        v89
    };
    let v94: bool = &*v0 == "seal";
    let v97: US0 = if v94 {
        let v95: US1 = US1::US1_AgileSeal;
        US0::US0_AgileKnown(v95)
    } else {
        v93
    };
    let v98: bool = &*v0 == "todos";
    let v101: US0 = if v98 {
        let v99: US1 = US1::US1_AgileTodos;
        US0::US0_AgileKnown(v99)
    } else {
        v97
    };
    let v102: bool = &*v0 == "next";
    let v105: US0 = if v102 {
        let v103: US1 = US1::US1_AgileNext;
        US0::US0_AgileKnown(v103)
    } else {
        v101
    };
    let v106: bool = &*v0 == "claim";
    let v109: US0 = if v106 {
        let v107: US1 = US1::US1_AgileClaim;
        US0::US0_AgileKnown(v107)
    } else {
        v105
    };
    let v110: bool = &*v0 == "release";
    let v113: US0 = if v110 {
        let v111: US1 = US1::US1_AgileRelease;
        US0::US0_AgileKnown(v111)
    } else {
        v109
    };
    let v114: bool = &*v0 == "roots";
    let v117: US0 = if v114 {
        let v115: US1 = US1::US1_AgileRoots;
        US0::US0_AgileKnown(v115)
    } else {
        v113
    };
    let v118: bool = &*v0 == "links";
    let v121: US0 = if v118 {
        let v119: US1 = US1::US1_AgileLinks;
        US0::US0_AgileKnown(v119)
    } else {
        v117
    };
    let v122: bool = &*v0 == "wip";
    let v125: US0 = if v122 {
        let v123: US1 = US1::US1_AgileWip;
        US0::US0_AgileKnown(v123)
    } else {
        v121
    };
    let v126: bool = &*v0 == "views";
    let v129: US0 = if v126 {
        let v127: US1 = US1::US1_AgileViews;
        US0::US0_AgileKnown(v127)
    } else {
        v125
    };
    let v130: bool = &*v0 == "view";
    let v133: US0 = if v130 {
        let v131: US1 = US1::US1_AgileView;
        US0::US0_AgileKnown(v131)
    } else {
        v129
    };
    let v134: bool = &*v0 == "view-add";
    if v134 {
        let v135: US1 = US1::US1_AgileViewAdd;
        US0::US0_AgileKnown(v135)
    } else {
        v133
    }
}
fn state_lock_taken_5(mut v0: Rc<str>) -> bool {
    let v1: bool = rig_agile_lock::rig_agile_lock_taken(&v0);
    v1
}
fn state_lock_held_4(mut v0: bool, mut v1: Rc<str>) -> bool {
    if v0 {
        state_lock_taken_5(v1)
    } else {
        true
    }
}
fn method6() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: bool = v1 > 2i32;
    let v4: i32 = if v2 {
        let v3: i32 = v1.wrapping_sub(2i32);
        v3
    } else {
        0i32
    };
    let v5: bool = 2i32 <= v4;
    if v5 {
        let v6: i32 = agile_claim_cli::rig_agile_claim_run();
        v6
    } else {
        agile_usage_error_2()
    }
}
fn method7() -> i32 {
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
fn method8() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: bool = v1 > 2i32;
    let v4: i32 = if v2 {
        let v3: i32 = v1.wrapping_sub(2i32);
        v3
    } else {
        0i32
    };
    let v5: bool = 2i32 <= v4;
    if v5 {
        let v6: i32 = agile_task_views::rig_agile_views_run();
        v6
    } else {
        agile_usage_error_2()
    }
}
fn state_lock_released_10(mut v0: Rc<str>) -> () {
    let v1: bool = rig_agile_lock::rig_agile_lock_released(&v0);
    ()
}
fn state_lock_done_9(mut v0: bool, mut v1: Rc<str>) -> () {
    if v0 {
        state_lock_released_10(v1)
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
                        let v317: i32 = match v3 {
                            US1::US1_AgileAdd => {
                                let v101: usize = std::env::args().count();
                                let v102: i32 = ((v101) as i32);
                                let v103: bool = v102 > 2i32;
                                let v105: i32 = if v103 {
                                    let v104: i32 = v102.wrapping_sub(2i32);
                                    v104
                                } else {
                                    0i32
                                };
                                let v106: bool = 8i32 <= v105;
                                if v106 {
                                    let v107: i32 = agile_task_creation::rig_agile_add_run();
                                    v107
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileBegin => {
                                let v44: i32 = 3i32;
                                let v45: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v44 ;
                                if v45 {
                                    let v46: i32 = rig_agile_routes::rig_agile_begin_route();
                                    v46
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileCadence => {
                                let v181: usize = std::env::args().count();
                                let v182: i32 = ((v181) as i32);
                                let v183: bool = v182 > 2i32;
                                let v185: i32 = if v183 {
                                    let v184: i32 = v182.wrapping_sub(2i32);
                                    v184
                                } else {
                                    0i32
                                };
                                let v186: bool = 2i32 <= v185;
                                if v186 {
                                    let v187: i32 = rig_agile_cadence::rig_agile_cadence_run();
                                    v187
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileCheck => {
                                let v67: i32 = 2i32;
                                let v68: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v67 ;
                                if v68 {
                                    let v69: i32 = 1i32;
                                    let v70: Rc<str> = usize::try_from(v69).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v71: i32 = rig_agile_state::agile_check_owned(v70.as_ref());
                                    v71
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileClaim => {
                                method6()
                            }
                            US1::US1_AgileEdit => {
                                let v110: i32 = 5i32;
                                let v111: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v110 ;
                                let v114: bool = if v111 {
                                    true
                                } else {
                                    let v112: i32 = 7i32;
                                    let v113: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v112 ;
                                    v113
                                };
                                if v114 {
                                    let v115: i32 = rig_agile_routes::rig_agile_edit_route();
                                    v115
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileGate => {
                                let v172: usize = std::env::args().count();
                                let v173: i32 = ((v172) as i32);
                                let v174: bool = v173 > 2i32;
                                let v176: i32 = if v174 {
                                    let v175: i32 = v173.wrapping_sub(2i32);
                                    v175
                                } else {
                                    0i32
                                };
                                let v177: bool = 2i32 <= v176;
                                if v177 {
                                    let v178: i32 = rig_agile_gate::rig_agile_gate_run();
                                    v178
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileHandoff => {
                                let v51: i32 = 2i32;
                                let v52: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v51 ;
                                if v52 {
                                    let v53: i32 = 1i32;
                                    let v54: Rc<str> = usize::try_from(v53).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v55: i32 = rig_handoff_render::handoff_render_owned(v54.as_ref());
                                    v55
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileInit => {
                                let v37: i32 = 2i32;
                                let v38: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v37 ;
                                if v38 {
                                    let v39: i32 = 1i32;
                                    let v40: Rc<str> = usize::try_from(v39).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v41: i32 = agile_backlog_scaffold::rig_agile_init_run();
                                    v41
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileLease => {
                                method7()
                            }
                            US1::US1_AgileLinks => {
                                let v263: usize = std::env::args().count();
                                let v264: i32 = ((v263) as i32);
                                let v265: bool = v264 > 2i32;
                                let v267: i32 = if v265 {
                                    let v266: i32 = v264.wrapping_sub(2i32);
                                    v266
                                } else {
                                    0i32
                                };
                                let v268: bool = 2i32 <= v267;
                                if v268 {
                                    let v269: i32 = agile_memory_links::rig_agile_links_run();
                                    v269
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileLint => {
                                let v163: usize = std::env::args().count();
                                let v164: i32 = ((v163) as i32);
                                let v165: bool = v164 > 2i32;
                                let v167: i32 = if v165 {
                                    let v166: i32 = v164.wrapping_sub(2i32);
                                    v166
                                } else {
                                    0i32
                                };
                                let v168: bool = 2i32 <= v167;
                                if v168 {
                                    let v169: i32 = rig_agile_lint::rig_agile_lint_run();
                                    v169
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileList => {
                                let v58: usize = std::env::args().count();
                                let v59: i32 = ((v58) as i32);
                                let v60: bool = v59 > 2i32;
                                let v62: i32 = if v60 {
                                    let v61: i32 = v59.wrapping_sub(2i32);
                                    v61
                                } else {
                                    0i32
                                };
                                let v63: bool = 2i32 <= v62;
                                if v63 {
                                    let v64: i32 = agile_task_tree::rig_agile_list_routed();
                                    v64
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileMigrateActivity => {
                                let v145: usize = std::env::args().count();
                                let v146: i32 = ((v145) as i32);
                                let v147: bool = v146 > 2i32;
                                let v149: i32 = if v147 {
                                    let v148: i32 = v146.wrapping_sub(2i32);
                                    v148
                                } else {
                                    0i32
                                };
                                let v150: bool = 2i32 <= v149;
                                if v150 {
                                    let v151: i32 = rig_agile_activity_migration::rig_agile_migrate_activity();
                                    v151
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileMigrateTags => {
                                let v136: usize = std::env::args().count();
                                let v137: i32 = ((v136) as i32);
                                let v138: bool = v137 > 2i32;
                                let v140: i32 = if v138 {
                                    let v139: i32 = v137.wrapping_sub(2i32);
                                    v139
                                } else {
                                    0i32
                                };
                                let v141: bool = 2i32 <= v140;
                                if v141 {
                                    let v142: i32 = agile_planning_migration::rig_agile_migrate_tags_run();
                                    v142
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNext => {
                                let v239: usize = std::env::args().count();
                                let v240: i32 = ((v239) as i32);
                                let v241: bool = v240 > 2i32;
                                let v243: i32 = if v241 {
                                    let v242: i32 = v240.wrapping_sub(2i32);
                                    v242
                                } else {
                                    0i32
                                };
                                let v244: bool = 2i32 <= v243;
                                if v244 {
                                    let v245: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--any"); } LIT.with(|lit| lit.clone()) };
                                    let v246: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v245);
                                    if v246 {
                                        let v247: i32 = agile_claim_cli::rig_agile_claim_run();
                                        v247
                                    } else {
                                        let v248: i32 = agile_task_next::rig_agile_next_run();
                                        v248
                                    }
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNote => {
                                let v190: i32 = 4i32;
                                let v191: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v190 ;
                                let v194: bool = if v191 {
                                    true
                                } else {
                                    let v192: i32 = 6i32;
                                    let v193: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v192 ;
                                    v193
                                };
                                if v194 {
                                    let v195: i32 = agile_task_notes::rig_agile_note_run();
                                    v195
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNoteEdit => {
                                let v198: usize = std::env::args().count();
                                let v199: i32 = ((v198) as i32);
                                let v200: bool = v199 > 2i32;
                                let v202: i32 = if v200 {
                                    let v201: i32 = v199.wrapping_sub(2i32);
                                    v201
                                } else {
                                    0i32
                                };
                                let v203: bool = 2i32 <= v202;
                                if v203 {
                                    let v204: i32 = agile_note_edit::rig_agile_note_edit_run();
                                    v204
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNotes => {
                                let v207: i32 = 4i32;
                                let v208: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v207 ;
                                if v208 {
                                    let v209: i32 = agile_title_notes::rig_agile_notes_from_title_run();
                                    v209
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgilePlan => {
                                let v127: usize = std::env::args().count();
                                let v128: i32 = ((v127) as i32);
                                let v129: bool = v128 > 2i32;
                                let v131: i32 = if v129 {
                                    let v130: i32 = v128.wrapping_sub(2i32);
                                    v130
                                } else {
                                    0i32
                                };
                                let v132: bool = 2i32 <= v131;
                                if v132 {
                                    let v133: i32 = agile_planning_rollup::rig_agile_planning_rollup_run();
                                    v133
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRecord => {
                                let v74: i32 = 2i32;
                                let v75: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v74 ;
                                if v75 {
                                    let v76: i32 = 1i32;
                                    let v77: Rc<str> = usize::try_from(v76).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v78: i32 = rig_agile_state::agile_record_owned(v77.as_ref());
                                    v78
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRelease => {
                                method6()
                            }
                            US1::US1_AgileRename => {
                                let v212: i32 = 4i32;
                                let v213: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v212 ;
                                if v213 {
                                    let v214: i32 = agile_task_rename::rig_agile_rename_run();
                                    v214
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRoots => {
                                let v254: usize = std::env::args().count();
                                let v255: i32 = ((v254) as i32);
                                let v256: bool = v255 > 2i32;
                                let v258: i32 = if v256 {
                                    let v257: i32 = v255.wrapping_sub(2i32);
                                    v257
                                } else {
                                    0i32
                                };
                                let v259: bool = 2i32 <= v258;
                                if v259 {
                                    let v260: i32 = agile_todo_roots::rig_agile_roots_run();
                                    v260
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSeal => {
                                let v217: usize = std::env::args().count();
                                let v218: i32 = ((v217) as i32);
                                let v219: bool = v218 > 2i32;
                                let v221: i32 = if v219 {
                                    let v220: i32 = v218.wrapping_sub(2i32);
                                    v220
                                } else {
                                    0i32
                                };
                                let v222: bool = 2i32 <= v221;
                                if v222 {
                                    let v223: i32 = agile_note_seal::rig_agile_seal_run();
                                    v223
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSet => {
                                let v81: i32 = 7i32;
                                let v82: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v81 ;
                                let v86: bool = if v82 {
                                    let v83: i32 = 5i32;
                                    let v84: Rc<str> = usize::try_from(v83).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v85: bool = &*v84 == "--as";
                                    v85
                                } else {
                                    false
                                };
                                if v86 {
                                    let v87: i32 = 6i32;
                                    let v88: Rc<str> = usize::try_from(v87).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v91: i32 = 2i32;
                                    let v92: Rc<str> = usize::try_from(v91).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v93: Rc<str> = string_concat(&["agile set records no author (only agile note does); drop --as ", &*v88, ", or add a note with --as to say who moved ", &*v92]);
                                    let v94: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v93 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v94
                                } else {
                                    let v95: i32 = 5i32;
                                    let v96: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v95 ;
                                    if v96 {
                                        let v97: i32 = rig_agile_routes::rig_agile_set_route();
                                        v97
                                    } else {
                                        agile_usage_error_2()
                                    }
                                }
                            }
                            US1::US1_AgileShow => {
                                let v118: usize = std::env::args().count();
                                let v119: i32 = ((v118) as i32);
                                let v120: bool = v119 > 2i32;
                                let v122: i32 = if v120 {
                                    let v121: i32 = v119.wrapping_sub(2i32);
                                    v121
                                } else {
                                    0i32
                                };
                                let v123: bool = 2i32 <= v122;
                                if v123 {
                                    let v124: i32 = agile_task_inspection::rig_agile_show_run();
                                    v124
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSprint => {
                                let v154: usize = std::env::args().count();
                                let v155: i32 = ((v154) as i32);
                                let v156: bool = v155 > 2i32;
                                let v158: i32 = if v156 {
                                    let v157: i32 = v155.wrapping_sub(2i32);
                                    v157
                                } else {
                                    0i32
                                };
                                let v159: bool = 2i32 <= v158;
                                if v159 {
                                    let v160: i32 = rig_agile_sprint::rig_agile_sprint_run();
                                    v160
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileStatus => {
                                method7()
                            }
                            US1::US1_AgileTodos => {
                                let v226: usize = std::env::args().count();
                                let v227: i32 = ((v226) as i32);
                                let v228: bool = v227 > 2i32;
                                let v230: i32 = if v228 {
                                    let v229: i32 = v227.wrapping_sub(2i32);
                                    v229
                                } else {
                                    0i32
                                };
                                let v231: bool = 2i32 <= v230;
                                if v231 {
                                    let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--record"); } LIT.with(|lit| lit.clone()) };
                                    let v233: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v232);
                                    if v233 {
                                        let v234: i32 = agile_todo_record::rig_agile_todos_record_run();
                                        v234
                                    } else {
                                        let v235: i32 = agile_todo_scan::rig_agile_todos_run();
                                        v235
                                    }
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileView => {
                                method8()
                            }
                            US1::US1_AgileViewAdd => {
                                method8()
                            }
                            US1::US1_AgileViews => {
                                method8()
                            }
                            US1::US1_AgileWip => {
                                let v272: usize = std::env::args().count();
                                let v273: i32 = ((v272) as i32);
                                let v274: bool = v273 > 2i32;
                                let v276: i32 = if v274 {
                                    let v275: i32 = v273.wrapping_sub(2i32);
                                    v275
                                } else {
                                    0i32
                                };
                                let v277: bool = 2i32 <= v276;
                                if v277 {
                                    let v278: i32 = agile_todo_merge::rig_agile_wip_run();
                                    v278
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                        };
                        state_lock_done_9(v35, v26);
                        v317
                    } else {
                        let v322: bool = match v3 {
                            US1::US1_AgileClaim => {
                                true
                            }
                            US1::US1_AgileNext => {
                                let v318: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--any"); } LIT.with(|lit| lit.clone()) };
                                let v319: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v318);
                                v319
                            }
                            US1::US1_AgileRelease => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        if v322 {
                            let v323: i32 = 0i32;
                            let v324: Rc<str> = usize::try_from(v323).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let v326: Rc<str> = string_concat(&["rig agile ", &*v324, " refused reason=contention"]);
                            let v327: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v326.to_string() + "
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
