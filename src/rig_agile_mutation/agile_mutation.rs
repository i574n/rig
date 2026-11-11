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
    if v146 {
        let v147: US1 = US1::US1_AgileDialect;
        US0::US0_AgileKnown(v147)
    } else {
        v145
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
fn agile_argument_count_5() -> i32 {
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
fn method6() -> i32 {
    let v0: i32 = agile_argument_count_5();
    let v1: bool = 2i32 <= v0;
    if v1 {
        let v2: i32 = agile_claim_cli::rig_agile_claim_run();
        v2
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
    let v0: i32 = agile_argument_count_5();
    let v1: bool = 2i32 <= v0;
    if v1 {
        let v2: i32 = agile_task_views::rig_agile_views_run();
        v2
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
                        let v268: i32 = match v3 {
                            US1::US1_AgileAdd => {
                                let v103: i32 = agile_argument_count_5();
                                let v104: bool = 8i32 <= v103;
                                if v104 {
                                    let v105: i32 = agile_task_creation::rig_agile_add_run();
                                    v105
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileBegin => {
                                let v50: i32 = 3i32;
                                let v51: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v50 ;
                                if v51 {
                                    let v52: i32 = rig_agile_routes::rig_agile_begin_route();
                                    v52
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileCadence => {
                                let v156: i32 = agile_argument_count_5();
                                let v157: bool = 2i32 <= v156;
                                if v157 {
                                    let v158: i32 = rig_agile_cadence::rig_agile_cadence_run();
                                    v158
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileCheck => {
                                let v69: i32 = 2i32;
                                let v70: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v69 ;
                                if v70 {
                                    let v71: i32 = 1i32;
                                    let v72: Rc<str> = usize::try_from(v71).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v73: i32 = rig_agile_state::agile_check_owned(v72.as_ref());
                                    v73
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileClaim => {
                                method6()
                            }
                            US1::US1_AgileClaims => {
                                let v209: i32 = agile_argument_count_5();
                                let v210: bool = 2i32 <= v209;
                                if v210 {
                                    let v211: i32 = agile_claim_cli::rig_agile_claims_run();
                                    v211
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileDialect => {
                                let v45: i32 = 3i32;
                                let v46: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v45 ;
                                if v46 {
                                    let v47: i32 = agile_backlog_scaffold::rig_agile_dialect_run();
                                    v47
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileEdit => {
                                let v113: i32 = 5i32;
                                let v114: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v113 ;
                                let v117: bool = if v114 {
                                    true
                                } else {
                                    let v115: i32 = 7i32;
                                    let v116: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v115 ;
                                    v116
                                };
                                if v117 {
                                    let v118: i32 = rig_agile_routes::rig_agile_edit_route();
                                    v118
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileGap => {
                                let v108: i32 = agile_argument_count_5();
                                let v109: bool = 2i32 <= v108;
                                if v109 {
                                    let v110: i32 = rig_agile_gap::rig_agile_gap_run();
                                    v110
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileGate => {
                                let v151: i32 = agile_argument_count_5();
                                let v152: bool = 2i32 <= v151;
                                if v152 {
                                    let v153: i32 = rig_agile_gate::rig_agile_gate_run();
                                    v153
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileHandoff => {
                                let v57: i32 = 2i32;
                                let v58: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v57 ;
                                if v58 {
                                    let v59: i32 = 1i32;
                                    let v60: Rc<str> = usize::try_from(v59).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v61: i32 = rig_handoff_render::handoff_render_owned(v60.as_ref());
                                    v61
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileInit => {
                                let v37: i32 = 2i32;
                                let v38: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v37 ;
                                let v41: bool = if v38 {
                                    true
                                } else {
                                    let v39: i32 = 4i32;
                                    let v40: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v39 ;
                                    v40
                                };
                                if v41 {
                                    let v42: i32 = agile_backlog_scaffold::rig_agile_init_run();
                                    v42
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileLease => {
                                method7()
                            }
                            US1::US1_AgileLinks => {
                                let v219: i32 = agile_argument_count_5();
                                let v220: bool = 2i32 <= v219;
                                if v220 {
                                    let v221: i32 = agile_memory_links::rig_agile_links_run();
                                    v221
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileLint => {
                                let v146: i32 = agile_argument_count_5();
                                let v147: bool = 2i32 <= v146;
                                if v147 {
                                    let v148: i32 = rig_agile_lint::rig_agile_lint_run();
                                    v148
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileList => {
                                let v64: i32 = agile_argument_count_5();
                                let v65: bool = 2i32 <= v64;
                                if v65 {
                                    let v66: i32 = agile_task_tree::rig_agile_list_routed();
                                    v66
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileMigrateActivity => {
                                let v136: i32 = agile_argument_count_5();
                                let v137: bool = 2i32 <= v136;
                                if v137 {
                                    let v138: i32 = rig_agile_activity_migration::rig_agile_migrate_activity();
                                    v138
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileMigrateTags => {
                                let v131: i32 = agile_argument_count_5();
                                let v132: bool = 2i32 <= v131;
                                if v132 {
                                    let v133: i32 = agile_planning_migration::rig_agile_migrate_tags_run();
                                    v133
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNext => {
                                let v198: i32 = agile_argument_count_5();
                                let v199: bool = 2i32 <= v198;
                                if v199 {
                                    let v200: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--any"); } LIT.with(|lit| lit.clone()) };
                                    let v201: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v200);
                                    if v201 {
                                        let v202: i32 = agile_claim_cli::rig_agile_claim_run();
                                        v202
                                    } else {
                                        let v203: i32 = agile_task_next::rig_agile_next_run();
                                        v203
                                    }
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNote => {
                                let v161: i32 = 4i32;
                                let v162: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v161 ;
                                let v165: bool = if v162 {
                                    true
                                } else {
                                    let v163: i32 = 6i32;
                                    let v164: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v163 ;
                                    v164
                                };
                                if v165 {
                                    let v166: i32 = agile_task_notes::rig_agile_note_run();
                                    v166
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNoteEdit => {
                                let v169: i32 = agile_argument_count_5();
                                let v170: bool = 2i32 <= v169;
                                if v170 {
                                    let v171: i32 = agile_note_edit::rig_agile_note_edit_run();
                                    v171
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNotes => {
                                let v174: i32 = 4i32;
                                let v175: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v174 ;
                                if v175 {
                                    let v176: i32 = agile_title_notes::rig_agile_notes_from_title_run();
                                    v176
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgilePlan => {
                                let v126: i32 = agile_argument_count_5();
                                let v127: bool = 2i32 <= v126;
                                if v127 {
                                    let v128: i32 = agile_planning_rollup::rig_agile_planning_rollup_run();
                                    v128
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRecord => {
                                let v76: i32 = 2i32;
                                let v77: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v76 ;
                                if v77 {
                                    let v78: i32 = 1i32;
                                    let v79: Rc<str> = usize::try_from(v78).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v80: i32 = rig_agile_state::agile_record_owned(v79.as_ref());
                                    v80
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRelease => {
                                method6()
                            }
                            US1::US1_AgileRename => {
                                let v179: i32 = 4i32;
                                let v180: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v179 ;
                                if v180 {
                                    let v181: i32 = agile_task_rename::rig_agile_rename_run();
                                    v181
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRoots => {
                                let v214: i32 = agile_argument_count_5();
                                let v215: bool = 2i32 <= v214;
                                if v215 {
                                    let v216: i32 = agile_todo_roots::rig_agile_roots_run();
                                    v216
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSeal => {
                                let v184: i32 = agile_argument_count_5();
                                let v185: bool = 2i32 <= v184;
                                if v185 {
                                    let v186: i32 = agile_note_seal::rig_agile_seal_run();
                                    v186
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSet => {
                                let v83: i32 = 7i32;
                                let v84: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v83 ;
                                let v88: bool = if v84 {
                                    let v85: i32 = 5i32;
                                    let v86: Rc<str> = usize::try_from(v85).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v87: bool = &*v86 == "--as";
                                    v87
                                } else {
                                    false
                                };
                                if v88 {
                                    let v89: i32 = 6i32;
                                    let v90: Rc<str> = usize::try_from(v89).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v93: i32 = 2i32;
                                    let v94: Rc<str> = usize::try_from(v93).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v95: Rc<str> = string_concat(&["agile set records no author (only agile note does); drop --as ", &*v90, ", or add a note with --as to say who moved ", &*v94]);
                                    let v96: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v95 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v96
                                } else {
                                    let v97: i32 = 5i32;
                                    let v98: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v97 ;
                                    if v98 {
                                        let v99: i32 = rig_agile_routes::rig_agile_set_route();
                                        v99
                                    } else {
                                        agile_usage_error_2()
                                    }
                                }
                            }
                            US1::US1_AgileShow => {
                                let v121: i32 = agile_argument_count_5();
                                let v122: bool = 2i32 <= v121;
                                if v122 {
                                    let v123: i32 = agile_task_inspection::rig_agile_show_run();
                                    v123
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSprint => {
                                let v141: i32 = agile_argument_count_5();
                                let v142: bool = 2i32 <= v141;
                                if v142 {
                                    let v143: i32 = rig_agile_sprint::rig_agile_sprint_run();
                                    v143
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileStatus => {
                                method7()
                            }
                            US1::US1_AgileTodos => {
                                let v189: i32 = agile_argument_count_5();
                                let v190: bool = 2i32 <= v189;
                                if v190 {
                                    let v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--record"); } LIT.with(|lit| lit.clone()) };
                                    let v192: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v191);
                                    if v192 {
                                        let v193: i32 = agile_todo_record::rig_agile_todos_record_run();
                                        v193
                                    } else {
                                        let v194: i32 = agile_todo_scan::rig_agile_todos_run();
                                        v194
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
                                let v224: i32 = agile_argument_count_5();
                                let v225: bool = 2i32 <= v224;
                                if v225 {
                                    let v226: i32 = agile_todo_merge::rig_agile_wip_run();
                                    v226
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                        };
                        state_lock_done_9(v35, v26);
                        v268
                    } else {
                        let v273: bool = match v3 {
                            US1::US1_AgileClaim => {
                                true
                            }
                            US1::US1_AgileNext => {
                                let v269: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--any"); } LIT.with(|lit| lit.clone()) };
                                let v270: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v269);
                                v270
                            }
                            US1::US1_AgileRelease => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        if v273 {
                            let v274: i32 = 0i32;
                            let v275: Rc<str> = usize::try_from(v274).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let v277: Rc<str> = string_concat(&["rig agile ", &*v275, " refused reason=contention"]);
                            let v278: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v277.to_string() + "
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
