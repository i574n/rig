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
    US1_AgileRedact,
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
    US1_AgileBatch,
}
#[derive(Clone)]
enum US0 {
    US0_AgileKnown(US1),
    US0_AgileUnknownName,
}
fn agile_subcommand_of_1(mut v0: Rc<str>) -> US0 {
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
    let v90: bool = &*v0 == "redact";
    let v93: US0 = if v90 {
        let v91: US1 = US1::US1_AgileRedact;
        US0::US0_AgileKnown(v91)
    } else {
        v89
    };
    let v94: bool = &*v0 == "notes";
    let v97: US0 = if v94 {
        let v95: US1 = US1::US1_AgileNotes;
        US0::US0_AgileKnown(v95)
    } else {
        v93
    };
    let v98: bool = &*v0 == "rename";
    let v101: US0 = if v98 {
        let v99: US1 = US1::US1_AgileRename;
        US0::US0_AgileKnown(v99)
    } else {
        v97
    };
    let v102: bool = &*v0 == "seal";
    let v105: US0 = if v102 {
        let v103: US1 = US1::US1_AgileSeal;
        US0::US0_AgileKnown(v103)
    } else {
        v101
    };
    let v106: bool = &*v0 == "todos";
    let v109: US0 = if v106 {
        let v107: US1 = US1::US1_AgileTodos;
        US0::US0_AgileKnown(v107)
    } else {
        v105
    };
    let v110: bool = &*v0 == "next";
    let v113: US0 = if v110 {
        let v111: US1 = US1::US1_AgileNext;
        US0::US0_AgileKnown(v111)
    } else {
        v109
    };
    let v114: bool = &*v0 == "claim";
    let v117: US0 = if v114 {
        let v115: US1 = US1::US1_AgileClaim;
        US0::US0_AgileKnown(v115)
    } else {
        v113
    };
    let v118: bool = &*v0 == "release";
    let v121: US0 = if v118 {
        let v119: US1 = US1::US1_AgileRelease;
        US0::US0_AgileKnown(v119)
    } else {
        v117
    };
    let v122: bool = &*v0 == "claims";
    let v125: US0 = if v122 {
        let v123: US1 = US1::US1_AgileClaims;
        US0::US0_AgileKnown(v123)
    } else {
        v121
    };
    let v126: bool = &*v0 == "roots";
    let v129: US0 = if v126 {
        let v127: US1 = US1::US1_AgileRoots;
        US0::US0_AgileKnown(v127)
    } else {
        v125
    };
    let v130: bool = &*v0 == "links";
    let v133: US0 = if v130 {
        let v131: US1 = US1::US1_AgileLinks;
        US0::US0_AgileKnown(v131)
    } else {
        v129
    };
    let v134: bool = &*v0 == "wip";
    let v137: US0 = if v134 {
        let v135: US1 = US1::US1_AgileWip;
        US0::US0_AgileKnown(v135)
    } else {
        v133
    };
    let v138: bool = &*v0 == "views";
    let v141: US0 = if v138 {
        let v139: US1 = US1::US1_AgileViews;
        US0::US0_AgileKnown(v139)
    } else {
        v137
    };
    let v142: bool = &*v0 == "view";
    let v145: US0 = if v142 {
        let v143: US1 = US1::US1_AgileView;
        US0::US0_AgileKnown(v143)
    } else {
        v141
    };
    let v146: bool = &*v0 == "view-add";
    let v149: US0 = if v146 {
        let v147: US1 = US1::US1_AgileViewAdd;
        US0::US0_AgileKnown(v147)
    } else {
        v145
    };
    let v150: bool = &*v0 == "dialect";
    let v153: US0 = if v150 {
        let v151: US1 = US1::US1_AgileDialect;
        US0::US0_AgileKnown(v151)
    } else {
        v149
    };
    let v154: bool = &*v0 == "receipts";
    let v157: US0 = if v154 {
        let v155: US1 = US1::US1_AgileReceipts;
        US0::US0_AgileKnown(v155)
    } else {
        v153
    };
    let v158: bool = &*v0 == "terms";
    let v161: US0 = if v158 {
        let v159: US1 = US1::US1_AgileTerms;
        US0::US0_AgileKnown(v159)
    } else {
        v157
    };
    let v162: bool = &*v0 == "changes";
    let v165: US0 = if v162 {
        let v163: US1 = US1::US1_AgileChanges;
        US0::US0_AgileKnown(v163)
    } else {
        v161
    };
    let v166: bool = &*v0 == "ask";
    let v169: US0 = if v166 {
        let v167: US1 = US1::US1_AgileAsk;
        US0::US0_AgileKnown(v167)
    } else {
        v165
    };
    let v170: bool = &*v0 == "batch";
    if v170 {
        let v171: US1 = US1::US1_AgileBatch;
        US0::US0_AgileKnown(v171)
    } else {
        v169
    }
}
fn root_flag_refusal_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: US0 = agile_subcommand_of_1(v0);
    match v2 {
        US0::US0_AgileKnown(v3) => {
            let v3: US1 = v3;
            let (mut v174, mut v175): (Rc<str>, Rc<str>) = match &v3 {
                US1::US1_AgileInit => {
                    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init"); } LIT.with(|lit| lit.clone()) };
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--dialect symbols|words]"); } LIT.with(|lit| lit.clone()) };
                    (v4, v5)
                }
                _ => {
                    let (mut v172, mut v173): (Rc<str>, Rc<str>) = match &v3 {
                        US1::US1_AgileBegin => {
                            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin"); } LIT.with(|lit| lit.clone()) };
                            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <title>"); } LIT.with(|lit| lit.clone()) };
                            (v6, v7)
                        }
                        _ => {
                            let (mut v170, mut v171): (Rc<str>, Rc<str>) = match &v3 {
                                US1::US1_AgileLease => {
                                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lease"); } LIT.with(|lit| lit.clone()) };
                                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                    (v8, v9)
                                }
                                _ => {
                                    let (mut v168, mut v169): (Rc<str>, Rc<str>) = match &v3 {
                                        US1::US1_AgileStatus => {
                                            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
                                            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                            (v10, v11)
                                        }
                                        _ => {
                                            let (mut v166, mut v167): (Rc<str>, Rc<str>) = match &v3 {
                                                US1::US1_AgileHandoff => {
                                                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff"); } LIT.with(|lit| lit.clone()) };
                                                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                    (v12, v13)
                                                }
                                                _ => {
                                                    let (mut v164, mut v165): (Rc<str>, Rc<str>) = match &v3 {
                                                        US1::US1_AgileList => {
                                                            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                                                            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [all|Planned|Active|Blocked|Paused|Done|Dropped] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json]"); } LIT.with(|lit| lit.clone()) };
                                                            (v14, v15)
                                                        }
                                                        _ => {
                                                            let (mut v162, mut v163): (Rc<str>, Rc<str>) = match &v3 {
                                                                US1::US1_AgileCheck => {
                                                                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
                                                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--compiler <absolute>]"); } LIT.with(|lit| lit.clone()) };
                                                                    (v16, v17)
                                                                }
                                                                _ => {
                                                                    let (mut v160, mut v161): (Rc<str>, Rc<str>) = match &v3 {
                                                                        US1::US1_AgileRecord => {
                                                                            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record"); } LIT.with(|lit| lit.clone()) };
                                                                            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                                            (v18, v19)
                                                                        }
                                                                        _ => {
                                                                            let (mut v158, mut v159): (Rc<str>, Rc<str>) = match &v3 {
                                                                                US1::US1_AgileSet => {
                                                                                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
                                                                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <progress|-> <Planned|Active|Blocked|Paused|Done|Dropped> [--expect <status>] [--as <lane>]"); } LIT.with(|lit| lit.clone()) };
                                                                                    (v20, v21)
                                                                                }
                                                                                _ => {
                                                                                    let (mut v156, mut v157): (Rc<str>, Rc<str>) = match &v3 {
                                                                                        US1::US1_AgileAdd => {
                                                                                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
                                                                                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>]"); } LIT.with(|lit| lit.clone()) };
                                                                                            (v22, v23)
                                                                                        }
                                                                                        _ => {
                                                                                            let (mut v154, mut v155): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                US1::US1_AgileGap => {
                                                                                                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gap"); } LIT.with(|lit| lit.clone()) };
                                                                                                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <title> --verb <verb> --as <lane> [--exit <n>] [--argv <text>] [--cwd <dir>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                    (v24, v25)
                                                                                                }
                                                                                                _ => {
                                                                                                    let (mut v152, mut v153): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                        US1::US1_AgileEdit => {
                                                                                                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
                                                                                                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | <root> <id> --wait <train|item|lane|user|job|when>:<value> | --wait-remove <kind>:<value> | --wait --clear | --parent <EPIC> | --after <ID> | --after --clear | --split milestones | --asked <yyyy-mm-dd> --ask \"<scope>; done when <criterion>\" | --asked --clear | <field> <value> --expect <current value>"); } LIT.with(|lit| lit.clone()) };
                                                                                                            (v26, v27)
                                                                                                        }
                                                                                                        _ => {
                                                                                                            let (mut v150, mut v151): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                US1::US1_AgileShow => {
                                                                                                                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
                                                                                                                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                    (v28, v29)
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    let (mut v148, mut v149): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                        US1::US1_AgilePlan => {
                                                                                                                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan"); } LIT.with(|lit| lit.clone()) };
                                                                                                                            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--json]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                            (v30, v31)
                                                                                                                        }
                                                                                                                        _ => {
                                                                                                                            let (mut v146, mut v147): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                US1::US1_AgileMigrateTags => {
                                                                                                                                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-tags"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview|apply <root>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                    (v32, v33)
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                    let (mut v144, mut v145): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                        US1::US1_AgileMigrateActivity => {
                                                                                                                                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-activity"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview|apply <root>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                            (v34, v35)
                                                                                                                                        }
                                                                                                                                        _ => {
                                                                                                                                            let (mut v142, mut v143): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                US1::US1_AgileSprint => {
                                                                                                                                                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sprint"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start <root> <id> <days> <goal> [--items A,B] | show <root> [<id>] | add|drop <root> <id> <item> | close <root> <id> | velocity <root> [--last n]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                    (v36, v37)
                                                                                                                                                }
                                                                                                                                                _ => {
                                                                                                                                                    let (mut v140, mut v141): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                        US1::US1_AgileLint => {
                                                                                                                                                            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lint"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--census | --team <team>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                            (v38, v39)
                                                                                                                                                        }
                                                                                                                                                        _ => {
                                                                                                                                                            let (mut v138, mut v139): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                US1::US1_AgileGate => {
                                                                                                                                                                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gate"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                    (v40, v41)
                                                                                                                                                                }
                                                                                                                                                                _ => {
                                                                                                                                                                    let (mut v136, mut v137): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                        US1::US1_AgileCadence => {
                                                                                                                                                                            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cadence"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--hours n] [--days n | --since yyyy-mm-dd] [--team t] [--round h]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                            (v42, v43)
                                                                                                                                                                        }
                                                                                                                                                                        _ => {
                                                                                                                                                                            let (mut v134, mut v135): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                US1::US1_AgileNote => {
                                                                                                                                                                                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <text> [--as <author>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                    (v44, v45)
                                                                                                                                                                                }
                                                                                                                                                                                _ => {
                                                                                                                                                                                    let (mut v132, mut v133): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                        US1::US1_AgileNoteEdit => {
                                                                                                                                                                                            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note-edit"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                            (v46, v47)
                                                                                                                                                                                        }
                                                                                                                                                                                        _ => {
                                                                                                                                                                                            let (mut v130, mut v131): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                US1::US1_AgileRedact => {
                                                                                                                                                                                                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("redact"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --terms <file outside the backlog> --as <lane> [--apply]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                    (v48, v49)
                                                                                                                                                                                                }
                                                                                                                                                                                                _ => {
                                                                                                                                                                                                    let (mut v128, mut v129): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                        US1::US1_AgileNotes => {
                                                                                                                                                                                                            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("from-title <root> <id>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                            (v50, v51)
                                                                                                                                                                                                        }
                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                            let (mut v126, mut v127): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                US1::US1_AgileRename => {
                                                                                                                                                                                                                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rename"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                    (v52, v53)
                                                                                                                                                                                                                }
                                                                                                                                                                                                                _ => {
                                                                                                                                                                                                                    let (mut v124, mut v125): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                        US1::US1_AgileSeal => {
                                                                                                                                                                                                                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("seal"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--dry-run]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                            (v54, v55)
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                                            let (mut v122, mut v123): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                US1::US1_AgileTodos => {
                                                                                                                                                                                                                                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                    (v56, v57)
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                _ => {
                                                                                                                                                                                                                                    let (mut v120, mut v121): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                        US1::US1_AgileNext => {
                                                                                                                                                                                                                                            let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | <root> --any --as <lane> [--ttl <minutes>] (claims the top free item)"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                            (v58, v59)
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                                                            let (mut v118, mut v119): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                US1::US1_AgileClaim => {
                                                                                                                                                                                                                                                    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claim"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> --as <lane> [--paths <glob,glob>] [--ttl <minutes>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                    (v60, v61)
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                _ => {
                                                                                                                                                                                                                                                    let (mut v116, mut v117): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                        US1::US1_AgileRelease => {
                                                                                                                                                                                                                                                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                            (v62, v63)
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                                                                            let (mut v114, mut v115): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                                US1::US1_AgileClaims => {
                                                                                                                                                                                                                                                                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claims"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--as <lane>] [--team <team>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                    (v64, v65)
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                _ => {
                                                                                                                                                                                                                                                                    let (mut v112, mut v113): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                                        US1::US1_AgileRoots => {
                                                                                                                                                                                                                                                                            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("roots"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [add|drop <path>...]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                            (v66, v67)
                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                                                                                            let (mut v110, mut v111): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                                                US1::US1_AgileLinks => {
                                                                                                                                                                                                                                                                                    let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("links"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --memory <dir> [--json]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                    (v68, v69)
                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                _ => {
                                                                                                                                                                                                                                                                                    let (mut v108, mut v109): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                                                        US1::US1_AgileWip => {
                                                                                                                                                                                                                                                                                            let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wip"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                            (v70, v71)
                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                                                                                                            let (mut v106, mut v107): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                                                                US1::US1_AgileViews => {
                                                                                                                                                                                                                                                                                                    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("views"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                    (v72, v73)
                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                _ => {
                                                                                                                                                                                                                                                                                                    let (mut v104, mut v105): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                                                                        US1::US1_AgileView => {
                                                                                                                                                                                                                                                                                                            let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                            let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> [extra flags...]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                            (v74, v75)
                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                                                                                                                            let (mut v102, mut v103): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                                                                                US1::US1_AgileViewAdd => {
                                                                                                                                                                                                                                                                                                                    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view-add"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> <list|next> [flags...]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                    (v76, v77)
                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                _ => {
                                                                                                                                                                                                                                                                                                                    let (mut v100, mut v101): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                                                                                        US1::US1_AgileDialect => {
                                                                                                                                                                                                                                                                                                                            let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dialect"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <symbols|words>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                            (v78, v79)
                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                                                                                                                                            let (mut v98, mut v99): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                                                                                                US1::US1_AgileChanges => {
                                                                                                                                                                                                                                                                                                                                    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("changes"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                                    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --since <8h|30m|iso> [--team <team>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                                    (v80, v81)
                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                _ => {
                                                                                                                                                                                                                                                                                                                                    let (mut v96, mut v97): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                                                                                                        US1::US1_AgileReceipts => {
                                                                                                                                                                                                                                                                                                                                            let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("receipts"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                                            let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> verify <file>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                                            (v82, v83)
                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                                                                                                                                                            let (mut v94, mut v95): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                                                                                                                US1::US1_AgileTerms => {
                                                                                                                                                                                                                                                                                                                                                    let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("terms"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                                                    let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                                                    (v84, v85)
                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                _ => {
                                                                                                                                                                                                                                                                                                                                                    let (mut v92, mut v93): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                                                                                                                        US1::US1_AgileAsk => {
                                                                                                                                                                                                                                                                                                                                                            let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ask"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                                                            let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> \"<scope>; done when <criterion>\""); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                                                            (v86, v87)
                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                                                                                                                                                                            let (mut v90, mut v91): (Rc<str>, Rc<str>) = match v3 {
                                                                                                                                                                                                                                                                                                                                                                US1::US1_AgileBatch => {
                                                                                                                                                                                                                                                                                                                                                                    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("batch"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                                                                    let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --file <plan.tsv> [--as <lane>] [--dry-run]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                                                                                                                                    (v88, v89)
                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                _ => unreachable!(),
                                                                                                                                                                                                                                                                                                                                                            };
                                                                                                                                                                                                                                                                                                                                                            (v90, v91)
                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                    };
                                                                                                                                                                                                                                                                                                                                                    (v92, v93)
                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                            };
                                                                                                                                                                                                                                                                                                                                            (v94, v95)
                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                    };
                                                                                                                                                                                                                                                                                                                                    (v96, v97)
                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                            };
                                                                                                                                                                                                                                                                                                                            (v98, v99)
                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                    };
                                                                                                                                                                                                                                                                                                                    (v100, v101)
                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                            };
                                                                                                                                                                                                                                                                                                            (v102, v103)
                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                    };
                                                                                                                                                                                                                                                                                                    (v104, v105)
                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                            };
                                                                                                                                                                                                                                                                                            (v106, v107)
                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                    };
                                                                                                                                                                                                                                                                                    (v108, v109)
                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                            };
                                                                                                                                                                                                                                                                            (v110, v111)
                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                    };
                                                                                                                                                                                                                                                                    (v112, v113)
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                            };
                                                                                                                                                                                                                                                            (v114, v115)
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                    };
                                                                                                                                                                                                                                                    (v116, v117)
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                            };
                                                                                                                                                                                                                                            (v118, v119)
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                    };
                                                                                                                                                                                                                                    (v120, v121)
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                            };
                                                                                                                                                                                                                            (v122, v123)
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    };
                                                                                                                                                                                                                    (v124, v125)
                                                                                                                                                                                                                }
                                                                                                                                                                                                            };
                                                                                                                                                                                                            (v126, v127)
                                                                                                                                                                                                        }
                                                                                                                                                                                                    };
                                                                                                                                                                                                    (v128, v129)
                                                                                                                                                                                                }
                                                                                                                                                                                            };
                                                                                                                                                                                            (v130, v131)
                                                                                                                                                                                        }
                                                                                                                                                                                    };
                                                                                                                                                                                    (v132, v133)
                                                                                                                                                                                }
                                                                                                                                                                            };
                                                                                                                                                                            (v134, v135)
                                                                                                                                                                        }
                                                                                                                                                                    };
                                                                                                                                                                    (v136, v137)
                                                                                                                                                                }
                                                                                                                                                            };
                                                                                                                                                            (v138, v139)
                                                                                                                                                        }
                                                                                                                                                    };
                                                                                                                                                    (v140, v141)
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            (v142, v143)
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    (v144, v145)
                                                                                                                                }
                                                                                                                            };
                                                                                                                            (v146, v147)
                                                                                                                        }
                                                                                                                    };
                                                                                                                    (v148, v149)
                                                                                                                }
                                                                                                            };
                                                                                                            (v150, v151)
                                                                                                        }
                                                                                                    };
                                                                                                    (v152, v153)
                                                                                                }
                                                                                            };
                                                                                            (v154, v155)
                                                                                        }
                                                                                    };
                                                                                    (v156, v157)
                                                                                }
                                                                            };
                                                                            (v158, v159)
                                                                        }
                                                                    };
                                                                    (v160, v161)
                                                                }
                                                            };
                                                            (v162, v163)
                                                        }
                                                    };
                                                    (v164, v165)
                                                }
                                            };
                                            (v166, v167)
                                        }
                                    };
                                    (v168, v169)
                                }
                            };
                            (v170, v171)
                        }
                    };
                    (v172, v173)
                }
            };
            let v182: Rc<str> = string_concat(&["agile ", &*v174, " takes <root> before any flag, got ", &*v1, "; usage: agile ", &*v174, " ", &*v175]);
            v182
        }
        US0::US0_AgileUnknownName => {
            let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v183
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        root_flag_refusal_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_subcommand_known_2(mut v0: Rc<str>) -> i32 {
    let v1: US0 = agile_subcommand_of_1(v0);
    match v1 {
        US0::US0_AgileKnown(v2) => {
            let v2: US1 = v2;
            1i32
        }
        US0::US0_AgileUnknownName => {
            0i32
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        agile_subcommand_known_2(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_root_flag_refusal(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_agile_subcommand_known(v0: &str) -> i32 {
    closure1()(Rc::<str>::from(v0))
}
