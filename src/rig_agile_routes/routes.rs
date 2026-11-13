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
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US0 {
    US0_StateFile(Rc<str>),
    US0_HistoryFile(Rc<str>),
    US0_ArchiveFile(Rc<str>),
    US0_OtherFile,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut1 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_HeadInl,
    US2_HeadUnion,
    US2_HeadOpen,
    US2_HeadType,
}
#[derive(Clone)]
enum US1 {
    US1_KeywordHead(US2),
    US1_SymbolHead(US2),
    US1_OtherLine,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US3 {
    US3_TaskLine(Rc<str>, Rc<str>, Rc<str>),
    US3_PlanningLine(Rc<str>, i32, Rc<str>, Rc<str>),
    US3_NoteLine(Rc<str>, Rc<str>),
    US3_WaitLine(Rc<str>, Rc<str>),
    US3_MarkerLine(Rc<str>),
    US3_OtherLine,
}
#[derive(Clone)]
enum US4 {
    US4_Some(i32),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_Some(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US5_None,
}
#[derive(Clone)]
enum US6 {
    US6_Some(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US6_None,
}
#[derive(Clone)]
enum US7 {
    US7_Some(Rc<str>, Rc<str>, u64, Rc<str>),
    US7_None,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<str>, u64, Rc<UH2>),
}
#[derive(Clone)]
enum US8 {
    US8_Some(Rc<str>, Rc<str>, u64),
    US8_None,
}
#[derive(Clone)]
enum US9 {
    US9_Some(u64),
    US9_None,
}
#[derive(Clone)]
enum US10 {
    US10_Some(Rc<str>),
    US10_None,
}
#[derive(Clone)]
enum US11 {
    US11_Valid(i32),
    US11_Invalid,
}
#[derive(Clone)]
enum US13 {
    US13_AgilePlanned,
    US13_AgileActive,
    US13_AgileBlocked,
    US13_AgilePaused,
    US13_AgileDone,
    US13_AgileDropped,
}
#[derive(Clone)]
enum US12 {
    US12_Valid(US13),
    US12_Invalid,
}
fn agile_subcommand_shape_1(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = &*v0 == "init";
    let mut v4: Rc<str> = if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--dialect symbols|words]"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3
    };
    let mut v5: bool = &*v4 == "";
    let mut v7: bool = if v5 {
        let mut v6: bool = &*v0 == "begin";
        v6
    } else {
        false
    };
    let mut v9: Rc<str> = if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <title>"); } LIT.with(|lit| lit.clone()) };
        v8
    } else {
        v4
    };
    let v10: bool = &*v9 == "";
    let mut v12: bool = if v10 {
        let v11: bool = &*v0 == "lease";
        v11
    } else {
        false
    };
    let mut v14: Rc<str> = if v12 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v13
    } else {
        v9
    };
    let mut v15: bool = &*v14 == "";
    let v17: bool = if v15 {
        let mut v16: bool = &*v0 == "status";
        v16
    } else {
        false
    };
    let mut v19: Rc<str> = if v17 {
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v18
    } else {
        v14
    };
    let v20: bool = &*v19 == "";
    let v22: bool = if v20 {
        let v21: bool = &*v0 == "handoff";
        v21
    } else {
        false
    };
    let v24: Rc<str> = if v22 {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v23
    } else {
        v19
    };
    let mut v25: bool = &*v24 == "";
    let v27: bool = if v25 {
        let mut v26: bool = &*v0 == "list";
        v26
    } else {
        false
    };
    let mut v29: Rc<str> = if v27 {
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [all|Planned|Active|Blocked|Paused|Done|Dropped] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json]"); } LIT.with(|lit| lit.clone()) };
        v28
    } else {
        v24
    };
    let mut v30: bool = &*v29 == "";
    let v32: bool = if v30 {
        let mut v31: bool = &*v0 == "check";
        v31
    } else {
        false
    };
    let v34: Rc<str> = if v32 {
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--compiler <absolute>]"); } LIT.with(|lit| lit.clone()) };
        v33
    } else {
        v29
    };
    let v35: bool = &*v34 == "";
    let v37: bool = if v35 {
        let v36: bool = &*v0 == "record";
        v36
    } else {
        false
    };
    let v39: Rc<str> = if v37 {
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v38
    } else {
        v34
    };
    let v40: bool = &*v39 == "";
    let v42: bool = if v40 {
        let v41: bool = &*v0 == "set";
        v41
    } else {
        false
    };
    let v44: Rc<str> = if v42 {
        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <progress|-> <Planned|Active|Blocked|Paused|Done|Dropped> [--expect <status>] [--as <lane>]"); } LIT.with(|lit| lit.clone()) };
        v43
    } else {
        v39
    };
    let v45: bool = &*v44 == "";
    let v47: bool = if v45 {
        let v46: bool = &*v0 == "add";
        v46
    } else {
        false
    };
    let v49: Rc<str> = if v47 {
        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>]"); } LIT.with(|lit| lit.clone()) };
        v48
    } else {
        v44
    };
    let mut v50: bool = &*v49 == "";
    let v52: bool = if v50 {
        let mut v51: bool = &*v0 == "gap";
        v51
    } else {
        false
    };
    let v54: Rc<str> = if v52 {
        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <title> --verb <verb> --as <lane> [--exit <n>] [--argv <text>] [--cwd <dir>]"); } LIT.with(|lit| lit.clone()) };
        v53
    } else {
        v49
    };
    let v55: bool = &*v54 == "";
    let v57: bool = if v55 {
        let mut v56: bool = &*v0 == "edit";
        v56
    } else {
        false
    };
    let mut v59: Rc<str> = if v57 {
        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | <root> <id> --wait <train|item|lane|user|job|when>:<value> | --wait-remove <kind>:<value> | --wait --clear | --parent <EPIC> | --after <ID> | --after --clear | --split milestones | --asked <yyyy-mm-dd> --ask \"<scope>; done when <criterion>\" | --asked --clear | <field> <value> --expect <current value>"); } LIT.with(|lit| lit.clone()) };
        v58
    } else {
        v54
    };
    let mut v60: bool = &*v59 == "";
    let v62: bool = if v60 {
        let v61: bool = &*v0 == "show";
        v61
    } else {
        false
    };
    let v64: Rc<str> = if v62 {
        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
        v63
    } else {
        v59
    };
    let v65: bool = &*v64 == "";
    let v67: bool = if v65 {
        let v66: bool = &*v0 == "plan";
        v66
    } else {
        false
    };
    let v69: Rc<str> = if v67 {
        let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--json]"); } LIT.with(|lit| lit.clone()) };
        v68
    } else {
        v64
    };
    let mut v70: bool = &*v69 == "";
    let v72: bool = if v70 {
        let v71: bool = &*v0 == "migrate-tags";
        v71
    } else {
        false
    };
    let mut v74: Rc<str> = if v72 {
        let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview|apply <root>"); } LIT.with(|lit| lit.clone()) };
        v73
    } else {
        v69
    };
    let v75: bool = &*v74 == "";
    let v77: bool = if v75 {
        let v76: bool = &*v0 == "migrate-activity";
        v76
    } else {
        false
    };
    let v79: Rc<str> = if v77 {
        let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview|apply <root>"); } LIT.with(|lit| lit.clone()) };
        v78
    } else {
        v74
    };
    let v80: bool = &*v79 == "";
    let v82: bool = if v80 {
        let mut v81: bool = &*v0 == "sprint";
        v81
    } else {
        false
    };
    let v84: Rc<str> = if v82 {
        let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start <root> <id> <days> <goal> [--items A,B] | show <root> [<id>] | add|drop <root> <id> <item> | close <root> <id> | velocity <root> [--last n]"); } LIT.with(|lit| lit.clone()) };
        v83
    } else {
        v79
    };
    let v85: bool = &*v84 == "";
    let v87: bool = if v85 {
        let v86: bool = &*v0 == "lint";
        v86
    } else {
        false
    };
    let v89: Rc<str> = if v87 {
        let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--census | --team <team>]"); } LIT.with(|lit| lit.clone()) };
        v88
    } else {
        v84
    };
    let v90: bool = &*v89 == "";
    let mut v92: bool = if v90 {
        let mut v91: bool = &*v0 == "gate";
        v91
    } else {
        false
    };
    let v94: Rc<str> = if v92 {
        let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
        v93
    } else {
        v89
    };
    let v95: bool = &*v94 == "";
    let v97: bool = if v95 {
        let v96: bool = &*v0 == "cadence";
        v96
    } else {
        false
    };
    let v99: Rc<str> = if v97 {
        let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--hours n] [--days n | --since yyyy-mm-dd] [--team t] [--round h]"); } LIT.with(|lit| lit.clone()) };
        v98
    } else {
        v94
    };
    let v100: bool = &*v99 == "";
    let v102: bool = if v100 {
        let v101: bool = &*v0 == "note";
        v101
    } else {
        false
    };
    let v104: Rc<str> = if v102 {
        let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <text> [--as <author>]"); } LIT.with(|lit| lit.clone()) };
        v103
    } else {
        v99
    };
    let v105: bool = &*v104 == "";
    let v107: bool = if v105 {
        let v106: bool = &*v0 == "note-edit";
        v106
    } else {
        false
    };
    let v109: Rc<str> = if v107 {
        let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>]"); } LIT.with(|lit| lit.clone()) };
        v108
    } else {
        v104
    };
    let v110: bool = &*v109 == "";
    let v112: bool = if v110 {
        let v111: bool = &*v0 == "redact";
        v111
    } else {
        false
    };
    let v114: Rc<str> = if v112 {
        let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --terms <file outside the backlog> --as <lane> [--apply]"); } LIT.with(|lit| lit.clone()) };
        v113
    } else {
        v109
    };
    let v115: bool = &*v114 == "";
    let v117: bool = if v115 {
        let v116: bool = &*v0 == "notes";
        v116
    } else {
        false
    };
    let v119: Rc<str> = if v117 {
        let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("from-title <root> <id>"); } LIT.with(|lit| lit.clone()) };
        v118
    } else {
        v114
    };
    let v120: bool = &*v119 == "";
    let v122: bool = if v120 {
        let v121: bool = &*v0 == "rename";
        v121
    } else {
        false
    };
    let mut v124: Rc<str> = if v122 {
        let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
        v123
    } else {
        v119
    };
    let v125: bool = &*v124 == "";
    let v127: bool = if v125 {
        let mut v126: bool = &*v0 == "seal";
        v126
    } else {
        false
    };
    let mut v129: Rc<str> = if v127 {
        let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--dry-run]"); } LIT.with(|lit| lit.clone()) };
        v128
    } else {
        v124
    };
    let mut v130: bool = &*v129 == "";
    let v132: bool = if v130 {
        let v131: bool = &*v0 == "todos";
        v131
    } else {
        false
    };
    let mut v134: Rc<str> = if v132 {
        let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record"); } LIT.with(|lit| lit.clone()) };
        v133
    } else {
        v129
    };
    let mut v135: bool = &*v134 == "";
    let v137: bool = if v135 {
        let v136: bool = &*v0 == "next";
        v136
    } else {
        false
    };
    let v139: Rc<str> = if v137 {
        let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | <root> --any --as <lane> [--ttl <minutes>] (claims the top free item)"); } LIT.with(|lit| lit.clone()) };
        v138
    } else {
        v134
    };
    let v140: bool = &*v139 == "";
    let v142: bool = if v140 {
        let v141: bool = &*v0 == "claim";
        v141
    } else {
        false
    };
    let v144: Rc<str> = if v142 {
        let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> --as <lane> [--paths <glob,glob>] [--ttl <minutes>]"); } LIT.with(|lit| lit.clone()) };
        v143
    } else {
        v139
    };
    let v145: bool = &*v144 == "";
    let v147: bool = if v145 {
        let v146: bool = &*v0 == "release";
        v146
    } else {
        false
    };
    let v149: Rc<str> = if v147 {
        let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> --as <lane>"); } LIT.with(|lit| lit.clone()) };
        v148
    } else {
        v144
    };
    let v150: bool = &*v149 == "";
    let v152: bool = if v150 {
        let v151: bool = &*v0 == "claims";
        v151
    } else {
        false
    };
    let v154: Rc<str> = if v152 {
        let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--as <lane>] [--team <team>]"); } LIT.with(|lit| lit.clone()) };
        v153
    } else {
        v149
    };
    let v155: bool = &*v154 == "";
    let v157: bool = if v155 {
        let v156: bool = &*v0 == "roots";
        v156
    } else {
        false
    };
    let v159: Rc<str> = if v157 {
        let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [add|drop <path>...]"); } LIT.with(|lit| lit.clone()) };
        v158
    } else {
        v154
    };
    let v160: bool = &*v159 == "";
    let v162: bool = if v160 {
        let v161: bool = &*v0 == "links";
        v161
    } else {
        false
    };
    let v164: Rc<str> = if v162 {
        let v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --memory <dir> [--json]"); } LIT.with(|lit| lit.clone()) };
        v163
    } else {
        v159
    };
    let v165: bool = &*v164 == "";
    let v167: bool = if v165 {
        let v166: bool = &*v0 == "wip";
        v166
    } else {
        false
    };
    let mut v169: Rc<str> = if v167 {
        let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v168
    } else {
        v164
    };
    let v170: bool = &*v169 == "";
    let v172: bool = if v170 {
        let v171: bool = &*v0 == "views";
        v171
    } else {
        false
    };
    let v174: Rc<str> = if v172 {
        let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v173
    } else {
        v169
    };
    let v175: bool = &*v174 == "";
    let v177: bool = if v175 {
        let v176: bool = &*v0 == "view";
        v176
    } else {
        false
    };
    let v179: Rc<str> = if v177 {
        let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> [extra flags...]"); } LIT.with(|lit| lit.clone()) };
        v178
    } else {
        v174
    };
    let v180: bool = &*v179 == "";
    let v182: bool = if v180 {
        let v181: bool = &*v0 == "view-add";
        v181
    } else {
        false
    };
    let v184: Rc<str> = if v182 {
        let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> <list|next> [flags...]"); } LIT.with(|lit| lit.clone()) };
        v183
    } else {
        v179
    };
    let v185: bool = &*v184 == "";
    let v187: bool = if v185 {
        let v186: bool = &*v0 == "dialect";
        v186
    } else {
        false
    };
    let v189: Rc<str> = if v187 {
        let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <symbols|words>"); } LIT.with(|lit| lit.clone()) };
        v188
    } else {
        v184
    };
    let v190: bool = &*v189 == "";
    let v192: bool = if v190 {
        let v191: bool = &*v0 == "receipts";
        v191
    } else {
        false
    };
    let v194: Rc<str> = if v192 {
        let v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> verify <file>"); } LIT.with(|lit| lit.clone()) };
        v193
    } else {
        v189
    };
    let v195: bool = &*v194 == "";
    let v197: bool = if v195 {
        let v196: bool = &*v0 == "terms";
        v196
    } else {
        false
    };
    let v199: Rc<str> = if v197 {
        let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v198
    } else {
        v194
    };
    let v200: bool = &*v199 == "";
    let v202: bool = if v200 {
        let v201: bool = &*v0 == "changes";
        v201
    } else {
        false
    };
    let v204: Rc<str> = if v202 {
        let v203: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --since <8h|30m|iso> [--team <team>]"); } LIT.with(|lit| lit.clone()) };
        v203
    } else {
        v199
    };
    let v205: bool = &*v204 == "";
    let v207: bool = if v205 {
        let v206: bool = &*v0 == "ask";
        v206
    } else {
        false
    };
    let v209: Rc<str> = if v207 {
        let v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> \"<scope>; done when <criterion>\""); } LIT.with(|lit| lit.clone()) };
        v208
    } else {
        v204
    };
    let v210: bool = &*v209 == "";
    let v212: bool = if v210 {
        let v211: bool = &*v0 == "batch";
        v211
    } else {
        false
    };
    if v212 {
        let v213: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --file <plan.tsv> [--as <lane>] [--dry-run]"); } LIT.with(|lit| lit.clone()) };
        v213
    } else {
        v209
    }
}
fn method3(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_2(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method3(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn agile_usage_error_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: Rc<str> = agile_subcommand_shape_1(v1.clone());
    let mut v3: bool = &*v2 == "";
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile supports init <root> [--dialect symbols|words], begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done|Dropped] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root> [--compiler <absolute>], record <root>, set <root> <id> <progress|-> <Planned|Active|Blocked|Paused|Done|Dropped> [--expect <status>] [--as <lane>], add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>], gap <root> <title> --verb <verb> --as <lane> [--exit <n>] [--argv <text>] [--cwd <dir>], edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | <root> <id> --wait <train|item|lane|user|job|when>:<value> | --wait-remove <kind>:<value> | --wait --clear | --parent <EPIC> | --after <ID> | --after --clear | --split milestones | --asked <yyyy-mm-dd> --ask \"<scope>; done when <criterion>\" | --asked --clear | <field> <value> --expect <current value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>, migrate-activity preview|apply <root>, sprint start <root> <id> <days> <goal> [--items A,B] | show <root> [<id>] | add|drop <root> <id> <item> | close <root> <id> | velocity <root> [--last n], lint <root> [--census | --team <team>], gate <root> --team <team> --as <lane>, cadence <root> [--hours n] [--days n | --since yyyy-mm-dd] [--team t] [--round h], note <root> <id> <text> [--as <author>], note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>], redact <root> --terms <file outside the backlog> --as <lane> [--apply], notes from-title <root> <id>, rename <root> <old-id> <new-id>, seal <root> [--dry-run], todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record, next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | <root> --any --as <lane> [--ttl <minutes>] (claims the top free item), claim <root> <ID> --as <lane> [--paths <glob,glob>] [--ttl <minutes>], release <root> <ID> --as <lane>, claims <root> [--as <lane>] [--team <team>], roots <root> [add|drop <path>...], links <root> --memory <dir> [--json], wip <root>, views <root>, view <root> <name> [extra flags...], view-add <root> <name> <list|next> [flags...], dialect <root> <symbols|words>, receipts <root> verify <file>, terms <root>, changes <root> --since <8h|30m|iso> [--team <team>], ask <root> <id> \"<scope>; done when <criterion>\", batch <root> --file <plan.tsv> [--as <lane>] [--dry-run]"); } LIT.with(|lit| lit.clone()) };
        let mut v5: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v4 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v5
    } else {
        let v10: i32 = std::env::args().skip(3i32 as usize).count() as i32;
        let v11: Rc<str> = format_real_2(v10);
        let mut v13: Rc<str> = string_concat(&["agile ", &*v1, " expects ", &*v2, ", received ", &*v11, " operands; see kit agile --help"]);
        let mut v14: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v13 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v14
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_usage_error_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 1i32;
        let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v2: i32 = 2i32;
        let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v4: i32 = rig_agile_lease::rig_lease_title_words();
        let mut v5: bool = 0 < v4;
        if v5 {
            let mut v6: i32 = rig_agile_state::agile_begin_owned(v1.as_ref(), v3.as_ref());
            v6
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin title must contain 3..7 visible words"); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v7 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v8
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method6(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure3() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(6i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_8(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_9(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method12(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method12(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method11(v16, v17)
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method13(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let mut v15: US1 = if v6 {
                let mut v7: US2 = US2::US2_HeadType;
                US1::US1_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US2 = US2::US2_HeadType;
                    US1::US1_SymbolHead(v11)
                } else {
                    US1::US1_OtherLine
                }
            };
            let v27: US1 = match &v15 {
                US1::US1_OtherLine => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let mut v18: US2 = US2::US2_HeadInl;
                        US1::US1_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US2 = US2::US2_HeadInl;
                            US1::US1_SymbolHead(v22)
                        } else {
                            US1::US1_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US1 = match &v27 {
                US1::US1_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let mut v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let mut v30: US2 = US2::US2_HeadUnion;
                        US1::US1_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US2 = US2::US2_HeadUnion;
                            US1::US1_SymbolHead(v34)
                        } else {
                            US1::US1_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let mut v51: US1 = match &v39 {
                US1::US1_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US2 = US2::US2_HeadOpen;
                        US1::US1_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US2 = US2::US2_HeadOpen;
                            US1::US1_SymbolHead(v46)
                        } else {
                            US1::US1_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US1::US1_SymbolHead(v52) => {
                    let v52: US2 = v52;
                    let mut v60: Rc<str> = match &v52 {
                        US2::US2_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US2::US2_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US2::US2_HeadType => {
                            let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US2::US2_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US2::US2_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US2::US2_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US2::US2_HeadType => {
                            v5
                        }
                        US2::US2_HeadUnion => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                    };
                    let v69: i32 = (v2.clone().len() as i32);
                    let mut v70: i32 = v69.wrapping_sub(v61);
                    let v71: usize = ((v61) as usize);
                    let v72: i32 = v61.wrapping_add(v70);
                    let mut v73: usize = ((v72) as usize);
                    let mut v74: Rc<str> = Rc::<str>::from(&(v2)[v71..v73]);
                    let v75: Rc<str> = string_concat(&[&*v68, &*v74]);
                    v75
                }
                _ => {
                    v2
                }
            };
            Rc::new(UH0::UH0_Cons(v76, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method14(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method16(mut v0: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = (v0)[v4].clone();
            let mut v14: i32 = v1.wrapping_sub(1i32);
            let mut v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v6, v7, v8, v9, v10, v11, v12, v13, v2));
            (v1, v2) = (v14, v15);
            continue;
        }
    }
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method22(v3, v1);
            let mut v5: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method23(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure4() -> Rc<dyn Fn((i32)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US4> = Rc::new(move |mut v0: (i32)| -> US4 {
        let mut v1: i32 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_line_21(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> US3 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = v0.borrow()[2i32 as usize].clone();
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let mut v6: Rc<UH0> = split_list_10(v1, v5);
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = method22(v6, v7);
    match Rc::unwrap_or_clone(v8) {
        UH0::UH0_Cons(v9, v10) => {
            let mut v9: Rc<str> = v9;
            let v10: Rc<UH0> = v10;
            match Rc::unwrap_or_clone(v10) {
                UH0::UH0_Cons(v11, v12) => {
                    let v11: Rc<str> = v11;
                    let mut v12: Rc<UH0> = v12;
                    match Rc::unwrap_or_clone(v12) {
                        UH0::UH0_Cons(v13, v14) => {
                            let mut v13: Rc<str> = v13;
                            let mut v14: Rc<UH0> = v14;
                            match Rc::unwrap_or_clone(v14) {
                                UH0::UH0_Cons(v15, v16) => {
                                    let mut v15: Rc<str> = v15;
                                    let mut v16: Rc<UH0> = v16;
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Minutes "); } LIT.with(|lit| lit.clone()) };
                                    let mut v18: bool = v11.starts_with(&*v17);
                                    let v20: Rc<str> = if v18 {
                                        let mut v19: Rc<str> = Rc::<str>::from(v11.replace(&*v17, &*v4));
                                        v19
                                    } else {
                                        v4.clone()
                                    };
                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                                    let v22: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v4));
                                    let v23: Option<i32> = (v22).trim().parse::<i32>().ok();
                                    let v24: Option<i32> = method23(v23);
                                    let mut v25: Rc<dyn Fn((i32)) -> US4> = closure4();
                                    let mut v26: Option<US4> = v24.map(|x| v25(x));
                                    let v28: US4 = US4::US4_None;
                                    let mut v29: US4 = v28.clone();
                                    let mut v30: US4 = v26.unwrap_or(v29);
                                    let v33: i32 = match v30 {
                                        US4::US4_None => {
                                            0i32
                                        }
                                        US4::US4_Some(v31) => {
                                            let mut v31: i32 = v31;
                                            v31
                                        }
                                    };
                                    let v34: Rc<str> = v0.borrow()[1i32 as usize].clone();
                                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load "); } LIT.with(|lit| lit.clone()) };
                                    let v36: bool = v13.starts_with(&*v35);
                                    let mut v38: Rc<str> = if v36 {
                                        let v37: Rc<str> = Rc::<str>::from(v13.replace(&*v35, &*v4));
                                        v37
                                    } else {
                                        v4.clone()
                                    };
                                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) };
                                    let v40: bool = v15.starts_with(&*v39);
                                    let v42: Rc<str> = if v40 {
                                        let v41: Rc<str> = Rc::<str>::from(v15.replace(&*v39, &*v4));
                                        v41
                                    } else {
                                        v4
                                    };
                                    US3::US3_PlanningLine(v34, v33, v38, v42)
                                }
                                _ => {
                                    US3::US3_OtherLine
                                }
                            }
                        }
                        _ => {
                            US3::US3_OtherLine
                        }
                    }
                }
                _ => {
                    US3::US3_OtherLine
                }
            }
        }
        _ => {
            US3::US3_OtherLine
        }
    }
}
fn method25(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method25(v3, v1);
            let mut v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method26(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                (v0, v1) = (v3, v2);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn task_line_24(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> US3 {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = v1.wrapping_sub(9i32);
    let mut v4: Rc<str> = v0.borrow()[v3 as usize].clone();
    let mut v5: Rc<UH0> = split_list_10(v2, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method22(v5, v6);
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method25(v7, v8);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = method26(v9, v10);
    let mut v12: Rc<str> = v0.borrow()[1i32 as usize].clone();
    let mut v13: i32 = v1.wrapping_sub(6i32);
    let mut v14: Rc<str> = v0.borrow()[v13 as usize].clone();
    US3::US3_TaskLine(v12, v11, v14)
}
fn state_line_of_20(mut v0: Rc<str>) -> US3 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: bool = v3 < 3i32;
    if v4 {
        US3::US3_OtherLine
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl note_of_"); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v0.starts_with(&*v6);
        if v7 {
            let mut v8: Rc<str> = v2.borrow()[1i32 as usize].clone();
            US3::US3_NoteLine(v8, v0)
        } else {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl wait_of_"); } LIT.with(|lit| lit.clone()) };
            let v11: bool = v0.starts_with(&*v10);
            let mut v14: bool = if v11 {
                true
            } else {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= wait_of_"); } LIT.with(|lit| lit.clone()) };
                let mut v13: bool = v0.starts_with(&*v12);
                v13
            };
            let mut v16: bool = if v14 {
                let mut v15: bool = v3 >= 7i32;
                v15
            } else {
                false
            };
            if v16 {
                let v17: Rc<str> = v2.borrow()[1i32 as usize].clone();
                let mut v18: Rc<str> = v2.borrow()[3i32 as usize].clone();
                let v20: Rc<str> = v2.borrow()[5i32 as usize].clone();
                let v21: Rc<str> = string_concat(&[&*v18, ":", &*v20]);
                US3::US3_WaitLine(v17, v21)
            } else {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Marker (\""); } LIT.with(|lit| lit.clone()) };
                let v24: bool = v0.contains(&*v23);
                let mut v26: bool = if v24 {
                    let mut v25: bool = v3 >= 5i32;
                    v25
                } else {
                    false
                };
                if v26 {
                    let v27: Rc<str> = v2.borrow()[1i32 as usize].clone();
                    let mut v29: Rc<str> = v2.borrow()[3i32 as usize].clone();
                    let mut v30: Rc<str> = string_concat(&[&*v27, "=", &*v29]);
                    US3::US3_MarkerLine(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl planning_of_"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v0.starts_with(&*v32);
                    if v33 {
                        planning_line_21(v2)
                    } else {
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                        let v36: bool = v0.starts_with(&*v35);
                        let v39: bool = if v36 {
                            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() = Task ("); } LIT.with(|lit| lit.clone()) };
                            let mut v38: bool = v0.contains(&*v37);
                            v38
                        } else {
                            false
                        };
                        let v41: bool = if v39 {
                            let v40: bool = v3 >= 10i32;
                            v40
                        } else {
                            false
                        };
                        if v41 {
                            task_line_24(v2)
                        } else {
                            US3::US3_OtherLine
                        }
                    }
                }
            }
        }
    }
}
fn method19(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: US3 = state_line_of_20(v2);
                let v10: Rc<str> = match v4 {
                    US3::US3_MarkerLine(v5) => {
                        let mut v5: Rc<str> = v5;
                        let mut v6: bool = &*v1 == "";
                        if v6 {
                            v5
                        } else {
                            let mut v8: Rc<str> = string_concat(&[&*v1, "\n", &*v5]);
                            v8
                        }
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v3, v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn declared_markers_18(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_10(v1, v4);
    method19(v5, v3)
}
fn method30(mut v0: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>) -> Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    v0
}
fn closure5() -> Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US5> = Rc::new(move |mut v0: ((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))| -> US5 {
        let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = (v0);
        US5::US5_Some(v1, v2, v3, v4, v5, v6, v7, v8)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn folded_29(mut v0: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>, mut v1: US3) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> {
    match v1 {
        US3::US3_MarkerLine(v175) => {
            let v175: Rc<str> = v175;
            v0
        }
        US3::US3_NoteLine(v87, v88) => {
            let v87: Rc<str> = v87;
            let v88: Rc<str> = v88;
            let v89: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v87).cloned();
            let v90: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method30(v89);
            let mut v91: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US5> = closure5();
            let mut v92: Option<US5> = v90.map(|x| v91(x));
            let v94: US5 = US5::US5_None;
            let v95: US5 = v94.clone();
            let v96: US5 = v92.unwrap_or(v95);
            let (mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v96 {
                US5::US5_Some(v97, v98, v99, v100, v101, v102, v103, v104) => {
                    let v97: Rc<str> = v97.clone();
                    let v98: Rc<str> = v98.clone();
                    let v99: i32 = *v99;
                    let v100: Rc<str> = v100.clone();
                    let v101: Rc<str> = v101.clone();
                    let v102: Rc<str> = v102.clone();
                    let v103: Rc<str> = v103.clone();
                    let v104: Rc<str> = v104.clone();
                    (v97, v98, v99, v100, v101, v102, v103, v104)
                }
                _ => {
                    let (mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v96 {
                        US5::US5_None => {
                            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v105.clone(), v105.clone(), 0i32, v105.clone(), v105.clone(), v105.clone(), v105.clone(), v105.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v106, v107, v108, v109, v110, v111, v112, v113)
                }
            };
            let v123: Rc<str> = string_concat(&[&*v119, "\n", &*v88]);
            let mut v124: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let mut v126: bool = true; // 1;
            let mut v124 = v124;
            let v128: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v124.insert(v87, (v114, v115, v116, v117, v118, v123, v120, v121));
            let mut v129: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v124);
            v129
        }
        US3::US3_OtherLine => {
            v0
        }
        US3::US3_PlanningLine(v44, v45, v46, v47) => {
            let v44: Rc<str> = v44;
            let v45: i32 = v45;
            let v46: Rc<str> = v46;
            let v47: Rc<str> = v47;
            let v48: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v44).cloned();
            let v49: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method30(v48);
            let mut v50: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US5> = closure5();
            let mut v51: Option<US5> = v49.map(|x| v50(x));
            let v53: US5 = US5::US5_None;
            let v54: US5 = v53.clone();
            let v55: US5 = v51.unwrap_or(v54);
            let (mut v73, mut v74, mut v78, mut v79, mut v80): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v55 {
                US5::US5_Some(v56, v57, v58, v59, v60, v61, v62, v63) => {
                    let mut v56: Rc<str> = v56.clone();
                    let v57: Rc<str> = v57.clone();
                    let v58: i32 = *v58;
                    let mut v59: Rc<str> = v59.clone();
                    let mut v60: Rc<str> = v60.clone();
                    let v61: Rc<str> = v61.clone();
                    let v62: Rc<str> = v62.clone();
                    let v63: Rc<str> = v63.clone();
                    (v56, v57, v61, v62, v63)
                }
                _ => {
                    let (mut v65, mut v66, mut v70, mut v71, mut v72): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v55 {
                        US5::US5_None => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v64.clone(), v64.clone(), v64.clone(), v64.clone(), v64.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v65, v66, v70, v71, v72)
                }
            };
            let mut v81: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v83: bool = true; // 1;
            let mut v81 = v81;
            let v85: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v81.insert(v44, (v73, v74, v45, v46, v47, v78, v79, v80));
            let v86: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v81);
            v86
        }
        US3::US3_TaskLine(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let mut v5: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v2).cloned();
            let mut v6: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method30(v5);
            let mut v7: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US5> = closure5();
            let mut v8: Option<US5> = v6.map(|x| v7(x));
            let v10: US5 = US5::US5_None;
            let v11: US5 = v10.clone();
            let mut v12: US5 = v8.unwrap_or(v11);
            let (mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                US5::US5_Some(v13, v14, v15, v16, v17, v18, v19, v20) => {
                    let mut v13: Rc<str> = v13.clone();
                    let mut v14: Rc<str> = v14.clone();
                    let mut v15: i32 = *v15;
                    let mut v16: Rc<str> = v16.clone();
                    let v17: Rc<str> = v17.clone();
                    let mut v18: Rc<str> = v18.clone();
                    let mut v19: Rc<str> = v19.clone();
                    let v20: Rc<str> = v20.clone();
                    (v15, v16, v17, v18, v19, v20)
                }
                _ => {
                    let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v12 {
                        US5::US5_None => {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (0i32, v21.clone(), v21.clone(), v21.clone(), v21.clone(), v21.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v24, v25, v26, v27, v28, v29)
                }
            };
            let mut v38: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v40: bool = true; // 1;
            let mut v38 = v38;
            let v42: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v38.insert(v2, (v3, v4, v32, v33, v34, v35, v36, v37));
            let v43: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v38);
            v43
        }
        US3::US3_WaitLine(v130, v131) => {
            let mut v130: Rc<str> = v130;
            let v131: Rc<str> = v131;
            let v132: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v130).cloned();
            let v133: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method30(v132);
            let mut v134: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US5> = closure5();
            let mut v135: Option<US5> = v133.map(|x| v134(x));
            let v137: US5 = US5::US5_None;
            let v138: US5 = v137.clone();
            let v139: US5 = v135.unwrap_or(v138);
            let (mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v139 {
                US5::US5_Some(v140, v141, v142, v143, v144, v145, v146, v147) => {
                    let v140: Rc<str> = v140.clone();
                    let v141: Rc<str> = v141.clone();
                    let v142: i32 = *v142;
                    let v143: Rc<str> = v143.clone();
                    let v144: Rc<str> = v144.clone();
                    let v145: Rc<str> = v145.clone();
                    let v146: Rc<str> = v146.clone();
                    let v147: Rc<str> = v147.clone();
                    (v140, v141, v142, v143, v144, v145, v146, v147)
                }
                _ => {
                    let (mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v139 {
                        US5::US5_None => {
                            let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v148.clone(), v148.clone(), 0i32, v148.clone(), v148.clone(), v148.clone(), v148.clone(), v148.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v149, v150, v151, v152, v153, v154, v155, v156)
                }
            };
            let v165: bool = &*v163 == "";
            let v168: Rc<str> = if v165 {
                v131
            } else {
                let v167: Rc<str> = string_concat(&[&*v163, ",", &*v131]);
                v167
            };
            let mut v169: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v171: bool = true; // 1;
            let mut v169 = v169;
            let v173: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v169.insert(v130, (v157, v158, v159, v160, v161, v162, v168, v164));
            let v174: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v169);
            v174
        }
    }
}
fn method28(mut v0: Rc<UH0>, mut v1: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: US3 = state_line_of_20(v2);
                let mut v5: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = folded_29(v1, v4);
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn facts_of_27(mut v0: Rc<str>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_10(v1, v4);
    let mut v7: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = std::collections::BTreeMap::new();
    let mut v8: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v7);
    method28(v5, v8)
}
fn method31(mut v0: Rc<str>, mut v1: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>, mut v2: Rc<UH0>, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = v1.get(&v4).cloned();
                let mut v7: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = method30(v6);
                let mut v8: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>))) -> US5> = closure5();
                let mut v9: Option<US5> = v7.map(|x| v8(x));
                let v11: US5 = US5::US5_None;
                let mut v12: US5 = v11.clone();
                let mut v13: US5 = v9.unwrap_or(v12);
                let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v13 {
                    US5::US5_Some(v14, v15, v16, v17, v18, v19, v20, v21) => {
                        let mut v14: Rc<str> = v14.clone();
                        let mut v15: Rc<str> = v15.clone();
                        let mut v16: i32 = *v16;
                        let v17: Rc<str> = v17.clone();
                        let mut v18: Rc<str> = v18.clone();
                        let mut v19: Rc<str> = v19.clone();
                        let v20: Rc<str> = v20.clone();
                        let v21: Rc<str> = v21.clone();
                        (v14, v15, v16, v17, v18, v19, v20, v21)
                    }
                    _ => {
                        let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v13 {
                            US5::US5_None => {
                                std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dict.present: the key is not in the dict"); } LIT.with(|lit| lit.clone()) }))
                            }
                            _ => unreachable!(),
                        };
                        (v30, v31, v32, v33, v34, v35, v36, v37)
                    }
                };
                let v46: bool = &*v38 == "";
                let v48: Rc<UH1> = if v46 {
                    v3
                } else {
                    Rc::new(UH1::UH1_Cons(v4, v38, v39, v40, v41, v42, v43, v44, v0.clone(), v3))
                };
                (v2, v3) = (v5, v48);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn method32(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let mut v5: i32 = v5;
                let mut v6: Rc<str> = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<UH1> = v11;
                let mut v12: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10, v1));
                (v0, v1) = (v11, v12);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method33(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
                let mut v6: i32 = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<str> = v11;
                let mut v12: Rc<UH1> = v12;
                v0.borrow_mut().push((v3, v4, v5, v6, v7, v8, v9, v10, v11));
                let mut v13: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v12, v13);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn fact_rows_of_17(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    let mut v1: Rc<str> = declared_markers_18(v0.clone());
    let mut v2: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = facts_of_27(v0);
    let mut v3: Vec<Rc<str>> = v2.keys().cloned().collect::<Vec<_>>();
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: bool = true; let _vec_fold_ = v3.into_iter().fold(v4, |acc, x| { //;
    let mut v8: Rc<UH0> = acc;
    let v10: Rc<str> = x;
    let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v8));
    let mut v13: bool = true; v12 });
    let mut v15: Rc<UH0> = _vec_fold_;
    let mut v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH0> = method11(v15, v16);
    let mut v18: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v19: Rc<UH1> = method31(v1, v2, v17, v18);
    let v20: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v21: Rc<UH1> = method32(v19, v20);
    let v22: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v23: i32 = 0i32;
    let v24: i32 = method33(v22.clone(), v21, v23);
    let mut v25: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new(v22.borrow().clone());
    let mut v26: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v25).as_ref().clone()));
    v26
}
fn facts_list_15(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = fact_rows_of_17(v0);
    let mut v2: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new((v1).borrow().clone());
    let mut v3: usize = (v2).len();
    let mut v4: i32 = ((v3) as i32);
    let mut v5: i32 = v4.wrapping_sub(1i32);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method16(v2, v5, v6)
}
fn method34(mut v0: Rc<str>, mut v1: Rc<UH1>) -> US6 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
                let mut v6: i32 = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<str> = v11;
                let mut v12: Rc<UH1> = v12;
                let mut v13: bool = &*v3 == &*v0;
                if v13 {
                    return US6::US6_Some(v3, v4, v5, v6, v7, v8, v9, v10, v11);
                } else {
                    v1 = v12;
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US6::US6_None;
            }
        }
    }
}
fn status_now_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<str> = string_concat(&[&*v4, "/state/agile.spi"]);
    let mut v6: bool = std::path::Path::new(&*v5).is_file();
    let v58: Rc<str> = if v6 {
        let (mut v7, mut v8): (bool, Rc<str>) = match std::fs::read(&*v5) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v7 {
            let mut v9: Rc<str> = string_concat(&["rig state read failed: ", &*v5]);
            std::panic::panic_any::<std::string::String>(format!("{}", v9))
        } else {
            let v11: Rc<str> = Rc::<str>::from(v5.replace(&*v2, &*v3));
            let mut v12: Rc<str> = loop_5(v11);
            let mut v13: Option<usize> = v12.rfind(&*v3);
            let mut v14: Option<usize> = method6(v13);
            let mut v15: Rc<dyn Fn((usize)) -> i32> = closure3();
            let mut v16: Option<i32> = v14.map(|x| v15(x));
            let mut v18: i32 = v16.unwrap_or(-1i32);
            let mut v19: bool = v18 < 0i32;
            let v23: bool = if v19 {
                true
            } else {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v21: bool = v12.ends_with(&*v20);
                let v22: bool = v21 == false;
                v22
            };
            let v44: US0 = if v23 {
                US0::US0_OtherFile
            } else {
                let mut v25: usize = ((0i32) as usize);
                let mut v26: usize = ((v18) as usize);
                let v27: Rc<str> = Rc::<str>::from(&(v12)[v25..v26]);
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let mut v29: bool = v27.ends_with(&*v28);
                if v29 {
                    let mut v30: Rc<str> = loop_7(v27);
                    US0::US0_StateFile(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v33: bool = v27.ends_with(&*v32);
                    if v33 {
                        let v34: Rc<str> = loop_8(v27);
                        US0::US0_HistoryFile(v34)
                    } else {
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v37: bool = v27.ends_with(&*v36);
                        if v37 {
                            let mut v38: Rc<str> = loop_9(v27);
                            US0::US0_ArchiveFile(v38)
                        } else {
                            US0::US0_OtherFile
                        }
                    }
                }
            };
            match v44 {
                US0::US0_OtherFile => {
                    v8
                }
                _ => {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v46: Rc<UH0> = split_list_10(v45.clone(), v8);
                    let v47: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v48: Rc<UH0> = method13(v46, v47);
                    let v49: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let mut v50: i32 = 0i32;
                    let mut v51: i32 = method14(v49.clone(), v48, v50);
                    let v52: Rc<Vec<Rc<str>>> = Rc::new(v49.borrow().clone());
                    let v53: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v52).as_ref().clone()));
                    let v54: Rc<str> = Rc::<str>::from((*v53.borrow()).join(&*v45));
                    v54
                }
            }
        }
    } else {
        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v57
    };
    let mut v59: Rc<UH1> = facts_list_15(v58);
    let mut v60: US6 = method34(v1, v59);
    match v60 {
        US6::US6_None => {
            let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v70
        }
        US6::US6_Some(v61, v62, v63, v64, v65, v66, v67, v68, v69) => {
            let v61: Rc<str> = v61;
            let v62: Rc<str> = v62;
            let v63: Rc<str> = v63;
            let v64: i32 = v64;
            let v65: Rc<str> = v65;
            let v66: Rc<str> = v66;
            let v67: Rc<str> = v67;
            let v68: Rc<str> = v68;
            let v69: Rc<str> = v69;
            v62
        }
    }
}
fn method39(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let mut v4: u64 = v4;
                let mut v5: Rc<UH2> = v5;
                let mut v6: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn closure6() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method42(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure7() -> Rc<dyn Fn((u64)) -> US9> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US9> = Rc::new(move |mut v0: (u64)| -> US9 {
        let mut v1: u64 = (v0);
        US9::US9_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn change_of_line_41(mut v0: Rc<str>) -> US8 {
    let mut v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = StatusChanged (\""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Option<usize> = v1.find(&*v2);
    let mut v4: Option<usize> = method6(v3);
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure6();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let mut v8: i32 = v6.unwrap_or(-1i32);
    let mut v9: bool = v8 < 0i32;
    let mut v13: bool = if v9 {
        true
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64)"); } LIT.with(|lit| lit.clone()) };
        let v11: bool = v1.ends_with(&*v10);
        let mut v12: bool = v11 == false;
        v12
    };
    if v13 {
        US8::US8_None
    } else {
        let mut v15: i32 = v8.wrapping_add(19i32);
        let mut v16: i32 = (v1.clone().len() as i32);
        let v17: i32 = v16.wrapping_sub(5i32);
        let mut v18: Rc<str> = string_slice(&v1, v15 as i64, v17 as i64);
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<UH0> = split_list_10(v19, v18);
        match Rc::unwrap_or_clone(v20) {
            UH0::UH0_Cons(v21, v22) => {
                let v21: Rc<str> = v21;
                let v22: Rc<UH0> = v22;
                match Rc::unwrap_or_clone(v22) {
                    UH0::UH0_Cons(v23, v24) => {
                        let v23: Rc<str> = v23;
                        let v24: Rc<UH0> = v24;
                        match Rc::unwrap_or_clone(v24) {
                            UH0::UH0_Cons(v25, v26) => {
                                let mut v25: Rc<str> = v25;
                                let mut v26: Rc<UH0> = v26;
                                match Rc::unwrap_or_clone(v26) {
                                    UH0::UH0_Nil => {
                                        let v27: bool = &*v21 != "";
                                        if v27 {
                                            let v28: Option<u64> = (v25).trim().parse::<u64>().ok();
                                            let mut v29: Option<u64> = method42(v28);
                                            let mut v30: Rc<dyn Fn((u64)) -> US9> = closure7();
                                            let mut v31: Option<US9> = v29.map(|x| v30(x));
                                            let v33: US9 = US9::US9_None;
                                            let v34: US9 = v33.clone();
                                            let v35: US9 = v31.unwrap_or(v34);
                                            match v35 {
                                                US9::US9_None => {
                                                    US8::US8_None
                                                }
                                                US9::US9_Some(v36) => {
                                                    let v36: u64 = v36;
                                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    let v39: Rc<str> = Rc::<str>::from(v23.replace(&*v37, &*v38));
                                                    US8::US8_Some(v21, v39, v36)
                                                }
                                            }
                                        } else {
                                            US8::US8_None
                                        }
                                    }
                                    _ => {
                                        US8::US8_None
                                    }
                                }
                            }
                            _ => {
                                US8::US8_None
                            }
                        }
                    }
                    _ => {
                        US8::US8_None
                    }
                }
            }
            _ => {
                US8::US8_None
            }
        }
    }
}
fn method40(mut v0: Rc<UH0>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: US8 = change_of_line_41(v2);
                let v10: Rc<UH2> = match v4 {
                    US8::US8_None => {
                        v1
                    }
                    US8::US8_Some(v5, v6, v7) => {
                        let mut v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        let mut v7: u64 = v7;
                        Rc::new(UH2::UH2_Cons(v5, v6, v7, v1))
                    }
                };
                (v0, v1) = (v3, v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn changes_of_text_38(mut v0: Rc<str>) -> Rc<UH2> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_10(v1, v4);
    let mut v6: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH2> = method40(v5, v6);
    let mut v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method39(v7, v8)
}
fn method43(mut v0: Rc<str>, mut v1: Rc<UH2>, mut v2: US8) -> US8 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4, v5, v6) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let mut v5: u64 = v5;
                let mut v6: Rc<UH2> = v6;
                let mut v7: bool = &*v3 == &*v0;
                let mut v9: US8 = if v7 {
                    US8::US8_Some(v3, v4, v5)
                } else {
                    v2
                };
                (v1, v2) = (v6, v9);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn format_real_45(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method3(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method46(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US10 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = v3.contains(&*v0);
                if v5 {
                    return US10::US10_Some(v3);
                } else {
                    v1 = v4;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US10::US10_None;
            }
        }
    }
}
fn author_at_44(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> Rc<str> {
    let mut v5: Rc<str> = format_real_45(v2);
    let mut v7: Rc<str> = string_concat(&["StatusAuthor (\"", &*v1, "\", ", &*v5, "u64, \""]);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<UH0> = split_list_10(v8, v0);
    let v10: US10 = method46(v7.clone(), v9);
    match v10 {
        US10::US10_None => {
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v35
        }
        US10::US10_Some(v11) => {
            let v11: Rc<str> = v11;
            let mut v12: Option<usize> = v11.find(&*v7);
            let mut v13: Option<usize> = method6(v12);
            let mut v14: Rc<dyn Fn((usize)) -> i32> = closure6();
            let mut v15: Option<i32> = v13.map(|x| v14(x));
            let v17: i32 = v15.unwrap_or(-1i32);
            let mut v18: i32 = (v7.len() as i32);
            let mut v19: i32 = v17.wrapping_add(v18);
            let v20: i32 = (v11.clone().len() as i32);
            let v21: i32 = v20.wrapping_sub(v19);
            let v22: usize = ((v19) as usize);
            let v23: i32 = v19.wrapping_add(v21);
            let v24: usize = ((v23) as usize);
            let mut v25: Rc<str> = Rc::<str>::from(&(v11)[v22..v24]);
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v27: Option<usize> = v25.find(&*v26);
            let v28: Option<usize> = method6(v27);
            let mut v29: Option<i32> = v28.map(|x| v14(x));
            let mut v31: i32 = v29.unwrap_or(-1i32);
            let v32: usize = ((0i32) as usize);
            let v33: usize = ((v31) as usize);
            let v34: Rc<str> = Rc::<str>::from(&(v25)[v32..v33]);
            v34
        }
    }
}
fn recent_by_other_37(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> US7 {
    let mut v4: Rc<UH2> = changes_of_text_38(v0.clone());
    let mut v5: US8 = US8::US8_None;
    let mut v6: US8 = method43(v1.clone(), v4, v5);
    match v6 {
        US8::US8_Some(v7, v8, v9) => {
            let mut v7: Rc<str> = v7;
            let mut v8: Rc<str> = v8;
            let mut v9: u64 = v9;
            let v10: u64 = v9.wrapping_add(1800000u64);
            let v11: bool = v10 >= v3;
            if v11 {
                let mut v12: Rc<str> = author_at_44(v0, v1, v9);
                let mut v13: bool = &*v12 != "";
                let mut v15: bool = if v13 {
                    let mut v14: bool = &*v12 != &*v2;
                    v14
                } else {
                    false
                };
                if v15 {
                    US7::US7_Some(v7, v8, v9, v12)
                } else {
                    US7::US7_None
                }
            } else {
                US7::US7_None
            }
        }
        _ => {
            US7::US7_None
        }
    }
}
fn stale_warning_text_36(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> Rc<str> {
    let mut v4: US7 = recent_by_other_37(v0, v1.clone(), v2, v3);
    match v4 {
        US7::US7_None => {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v22
        }
        US7::US7_Some(v5, v6, v7, v8) => {
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<str> = v6;
            let mut v7: u64 = v7;
            let mut v8: Rc<str> = v8;
            let mut v9: u64 = v3.wrapping_sub(v7);
            let v10: u64 = v9.wrapping_div(60000u64);
            let mut v19: Rc<str> = format_real_45(v10);
            let v21: Rc<str> = string_concat(&["rig warning: ", &*v1, " status was set to ", &*v6, " by ", &*v8, " ", &*v19, " min ago; pass --expect ", &*v6, " to refuse a stale write"]);
            v21
        }
    }
}
fn history_path_47(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = string_concat(&[&*v3, "/history/status_changes.spi"]);
    v4
}
fn text_of_48(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = std::path::Path::new(&*v0).is_file();
    if v1 {
        let (mut v2, mut v3): (bool, Rc<str>) = match std::fs::read(&*v0) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v2 {
            let mut v4: Rc<str> = string_concat(&["rig state read failed: ", &*v0]);
            std::panic::panic_any::<std::string::String>(format!("{}", v4))
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
            let mut v9: Rc<str> = loop_5(v8);
            let v10: Option<usize> = v9.rfind(&*v7);
            let v11: Option<usize> = method6(v10);
            let mut v12: Rc<dyn Fn((usize)) -> i32> = closure3();
            let mut v13: Option<i32> = v11.map(|x| v12(x));
            let mut v15: i32 = v13.unwrap_or(-1i32);
            let mut v16: bool = v15 < 0i32;
            let v20: bool = if v16 {
                true
            } else {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let mut v18: bool = v9.ends_with(&*v17);
                let mut v19: bool = v18 == false;
                v19
            };
            let v41: US0 = if v20 {
                US0::US0_OtherFile
            } else {
                let v22: usize = ((0i32) as usize);
                let v23: usize = ((v15) as usize);
                let v24: Rc<str> = Rc::<str>::from(&(v9)[v22..v23]);
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let mut v26: bool = v24.ends_with(&*v25);
                if v26 {
                    let v27: Rc<str> = loop_7(v24);
                    US0::US0_StateFile(v27)
                } else {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let mut v30: bool = v24.ends_with(&*v29);
                    if v30 {
                        let mut v31: Rc<str> = loop_8(v24);
                        US0::US0_HistoryFile(v31)
                    } else {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v34: bool = v24.ends_with(&*v33);
                        if v34 {
                            let v35: Rc<str> = loop_9(v24);
                            US0::US0_ArchiveFile(v35)
                        } else {
                            US0::US0_OtherFile
                        }
                    }
                }
            };
            match v41 {
                US0::US0_OtherFile => {
                    v3
                }
                _ => {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<UH0> = split_list_10(v42.clone(), v3);
                    let v44: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v45: Rc<UH0> = method13(v43, v44);
                    let v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v47: i32 = 0i32;
                    let v48: i32 = method14(v46.clone(), v45, v47);
                    let v49: Rc<Vec<Rc<str>>> = Rc::new(v46.borrow().clone());
                    let mut v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v49).as_ref().clone()));
                    let mut v51: Rc<str> = Rc::<str>::from((*v50.borrow()).join(&*v42));
                    v51
                }
            }
        }
    } else {
        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v54
    }
}
fn stale_warning_35(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = history_path_47(v0);
    let mut v4: Rc<str> = text_of_48(v3);
    let mut v5: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    stale_warning_text_36(v4, v1, v2, v5)
}
fn expect_refusal_49(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let mut v8: Rc<str> = string_concat(&["agile set ", &*v0, " rejected: the status changed since it was read; it is ", &*v1, ", --expect said ", &*v2, "; nothing written"]);
    let mut v9: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v8 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
    v9
}
fn progress_in_50(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: i32 = { let text = &*v0; let key = format!("Task ({q}{}{q}, {q}", &*v1, q = char::from(34u8)); match text.find(&key) { None => -1i32, Some(at) => { let rest = &text[at + key.len()..]; let mut escaped = false; let mut end = None; for (i, ch) in rest.char_indices() { if escaped { escaped = false; continue; } if ch == char::from(92u8) { escaped = true; continue; } if ch == char::from(34u8) { end = Some(i); break; } } end.and_then(|e| rest[e + 1..].split(char::from(44u8)).map(str::trim).nth(4) .and_then(|field| field.strip_suffix("u32")).and_then(|n| n.parse::<i32>().ok())).unwrap_or(-1) } } };
    v2
}
fn checks_joined_52(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let (mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>) = v0.borrow()[v1 as usize].clone();
            let mut v8: i32 = v1.wrapping_add(1i32);
            let mut v9: bool = &*v2 == "";
            let mut v12: Rc<str> = if v9 {
                v7
            } else {
                let v11: Rc<str> = string_concat(&[&*v2, ",", &*v7]);
                v11
            };
            (v1, v2) = (v8, v12);
            continue;
        }
    }
}
fn lint_warned_51(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> () {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(v0.replace(&*v3, &*v4));
    let mut v7: Rc<str> = string_concat(&[&*v5, "/state/"]);
    let mut v8: Rc<str> = string_concat(&[&*v7, "agile.spi"]);
    let mut v9: bool = std::path::Path::new(&*v8).is_file();
    let v61: Rc<str> = if v9 {
        let (mut v10, mut v11): (bool, Rc<str>) = match std::fs::read(&*v8) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v10 {
            let mut v12: Rc<str> = string_concat(&["rig state read failed: ", &*v8]);
            std::panic::panic_any::<std::string::String>(format!("{}", v12))
        } else {
            let mut v14: Rc<str> = Rc::<str>::from(v8.replace(&*v3, &*v4));
            let mut v15: Rc<str> = loop_5(v14);
            let mut v16: Option<usize> = v15.rfind(&*v4);
            let v17: Option<usize> = method6(v16);
            let mut v18: Rc<dyn Fn((usize)) -> i32> = closure3();
            let mut v19: Option<i32> = v17.map(|x| v18(x));
            let v21: i32 = v19.unwrap_or(-1i32);
            let v22: bool = v21 < 0i32;
            let mut v26: bool = if v22 {
                true
            } else {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v24: bool = v15.ends_with(&*v23);
                let mut v25: bool = v24 == false;
                v25
            };
            let v47: US0 = if v26 {
                US0::US0_OtherFile
            } else {
                let v28: usize = ((0i32) as usize);
                let mut v29: usize = ((v21) as usize);
                let mut v30: Rc<str> = Rc::<str>::from(&(v15)[v28..v29]);
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v32: bool = v30.ends_with(&*v31);
                if v32 {
                    let v33: Rc<str> = loop_7(v30);
                    US0::US0_StateFile(v33)
                } else {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v36: bool = v30.ends_with(&*v35);
                    if v36 {
                        let v37: Rc<str> = loop_8(v30);
                        US0::US0_HistoryFile(v37)
                    } else {
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v40: bool = v30.ends_with(&*v39);
                        if v40 {
                            let v41: Rc<str> = loop_9(v30);
                            US0::US0_ArchiveFile(v41)
                        } else {
                            US0::US0_OtherFile
                        }
                    }
                }
            };
            match v47 {
                US0::US0_OtherFile => {
                    v11
                }
                _ => {
                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v49: Rc<UH0> = split_list_10(v48.clone(), v11);
                    let mut v50: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v51: Rc<UH0> = method13(v49, v50);
                    let v52: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v53: i32 = 0i32;
                    let v54: i32 = method14(v52.clone(), v51, v53);
                    let v55: Rc<Vec<Rc<str>>> = Rc::new(v52.borrow().clone());
                    let mut v56: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v55).as_ref().clone()));
                    let v57: Rc<str> = Rc::<str>::from((*v56.borrow()).join(&*v48));
                    v57
                }
            }
        }
    } else {
        let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v60
    };
    let v62: Rc<str> = string_concat(&[&*v61, "\n"]);
    let v63: Rc<str> = string_concat(&[&*v7, "waits.spi"]);
    let v64: bool = std::path::Path::new(&*v63).is_file();
    let v116: Rc<str> = if v64 {
        let (mut v65, mut v66): (bool, Rc<str>) = match std::fs::read(&*v63) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v65 {
            let v67: Rc<str> = string_concat(&["rig state read failed: ", &*v63]);
            std::panic::panic_any::<std::string::String>(format!("{}", v67))
        } else {
            let v69: Rc<str> = Rc::<str>::from(v63.replace(&*v3, &*v4));
            let mut v70: Rc<str> = loop_5(v69);
            let v71: Option<usize> = v70.rfind(&*v4);
            let v72: Option<usize> = method6(v71);
            let mut v73: Rc<dyn Fn((usize)) -> i32> = closure3();
            let mut v74: Option<i32> = v72.map(|x| v73(x));
            let v76: i32 = v74.unwrap_or(-1i32);
            let v77: bool = v76 < 0i32;
            let mut v81: bool = if v77 {
                true
            } else {
                let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v79: bool = v70.ends_with(&*v78);
                let v80: bool = v79 == false;
                v80
            };
            let v102: US0 = if v81 {
                US0::US0_OtherFile
            } else {
                let v83: usize = ((0i32) as usize);
                let v84: usize = ((v76) as usize);
                let v85: Rc<str> = Rc::<str>::from(&(v70)[v83..v84]);
                let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v87: bool = v85.ends_with(&*v86);
                if v87 {
                    let v88: Rc<str> = loop_7(v85);
                    US0::US0_StateFile(v88)
                } else {
                    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let mut v91: bool = v85.ends_with(&*v90);
                    if v91 {
                        let mut v92: Rc<str> = loop_8(v85);
                        US0::US0_HistoryFile(v92)
                    } else {
                        let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v95: bool = v85.ends_with(&*v94);
                        if v95 {
                            let v96: Rc<str> = loop_9(v85);
                            US0::US0_ArchiveFile(v96)
                        } else {
                            US0::US0_OtherFile
                        }
                    }
                }
            };
            match v102 {
                US0::US0_OtherFile => {
                    v66
                }
                _ => {
                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v104: Rc<UH0> = split_list_10(v103.clone(), v66);
                    let v105: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v106: Rc<UH0> = method13(v104, v105);
                    let v107: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v108: i32 = 0i32;
                    let v109: i32 = method14(v107.clone(), v106, v108);
                    let v110: Rc<Vec<Rc<str>>> = Rc::new(v107.borrow().clone());
                    let v111: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v110).as_ref().clone()));
                    let v112: Rc<str> = Rc::<str>::from((*v111.borrow()).join(&*v103));
                    v112
                }
            }
        }
    } else {
        let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v115
    };
    let v118: Rc<str> = string_concat(&[&*v62, &*v116, "\n"]);
    let v119: Rc<str> = string_concat(&[&*v7, "markers.spi"]);
    let v120: bool = std::path::Path::new(&*v119).is_file();
    let v172: Rc<str> = if v120 {
        let (mut v121, mut v122): (bool, Rc<str>) = match std::fs::read(&*v119) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v121 {
            let v123: Rc<str> = string_concat(&["rig state read failed: ", &*v119]);
            std::panic::panic_any::<std::string::String>(format!("{}", v123))
        } else {
            let v125: Rc<str> = Rc::<str>::from(v119.replace(&*v3, &*v4));
            let mut v126: Rc<str> = loop_5(v125);
            let v127: Option<usize> = v126.rfind(&*v4);
            let v128: Option<usize> = method6(v127);
            let mut v129: Rc<dyn Fn((usize)) -> i32> = closure3();
            let mut v130: Option<i32> = v128.map(|x| v129(x));
            let v132: i32 = v130.unwrap_or(-1i32);
            let v133: bool = v132 < 0i32;
            let v137: bool = if v133 {
                true
            } else {
                let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let mut v135: bool = v126.ends_with(&*v134);
                let v136: bool = v135 == false;
                v136
            };
            let v158: US0 = if v137 {
                US0::US0_OtherFile
            } else {
                let v139: usize = ((0i32) as usize);
                let v140: usize = ((v132) as usize);
                let v141: Rc<str> = Rc::<str>::from(&(v126)[v139..v140]);
                let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v143: bool = v141.ends_with(&*v142);
                if v143 {
                    let v144: Rc<str> = loop_7(v141);
                    US0::US0_StateFile(v144)
                } else {
                    let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v147: bool = v141.ends_with(&*v146);
                    if v147 {
                        let v148: Rc<str> = loop_8(v141);
                        US0::US0_HistoryFile(v148)
                    } else {
                        let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v151: bool = v141.ends_with(&*v150);
                        if v151 {
                            let v152: Rc<str> = loop_9(v141);
                            US0::US0_ArchiveFile(v152)
                        } else {
                            US0::US0_OtherFile
                        }
                    }
                }
            };
            match v158 {
                US0::US0_OtherFile => {
                    v122
                }
                _ => {
                    let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v160: Rc<UH0> = split_list_10(v159.clone(), v122);
                    let v161: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v162: Rc<UH0> = method13(v160, v161);
                    let v163: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v164: i32 = 0i32;
                    let v165: i32 = method14(v163.clone(), v162, v164);
                    let v166: Rc<Vec<Rc<str>>> = Rc::new(v163.borrow().clone());
                    let v167: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v166).as_ref().clone()));
                    let v168: Rc<str> = Rc::<str>::from((*v167.borrow()).join(&*v159));
                    v168
                }
            }
        }
    } else {
        let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v171
    };
    let v173: Rc<str> = string_concat(&[&*v118, &*v172]);
    let v174: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = agile_quality_state::rig_agile_lint_item_failures(&v173, &v1, &v2);
    let v175: i32 = 0i32;
    let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v177: Rc<str> = checks_joined_52(&v174, v175, v176);
    let v178: bool = &*v177 == "";
    if v178 {
        ()
    } else {
        let v185: Rc<str> = string_concat(&["rig agile lint warning id=", &*v1, " status=", &*v2, " failing=", &*v177, " (add a note with the marker lines, e.g. 'dod:' and 'next:', or ones state/markers.spi declares; a later train refuses the change)"]);
        eprintln!("{}", v185);
        let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ProseWait"); } LIT.with(|lit| lit.clone()) };
        let v187: bool = v177.contains(&*v186);
        if v187 {
            let v192: Rc<str> = string_concat(&["  ProseWait: a wait in prose keeps the item in agile next; record it: rig agile edit ", &*v0, " ", &*v1, " --wait <train|item|lane|user|job|when>:<value>"]);
            eprintln!("{}", v192);
            ()
        }
    }
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 1i32;
        let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v2: i32 = 2i32;
        let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v4: i32 = 5i32;
        let mut v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v6: bool = &*v5 == "--expect";
        let mut v16: Rc<str> = if v6 {
            let mut v7: i32 = 6i32;
            let mut v8: Rc<str> = usize::try_from(v7).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            v8
        } else {
            let mut v9: i32 = 7i32;
            let v10: Rc<str> = usize::try_from(v9).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v11: bool = &*v10 == "--expect";
            if v11 {
                let mut v12: i32 = 8i32;
                let mut v13: Rc<str> = usize::try_from(v12).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                v13
            } else {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v14
            }
        };
        let v17: i32 = 5i32;
        let mut v18: Rc<str> = usize::try_from(v17).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v19: bool = &*v18 == "--as";
        let mut v29: Rc<str> = if v19 {
            let v20: i32 = 6i32;
            let v21: Rc<str> = usize::try_from(v20).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            v21
        } else {
            let v22: i32 = 7i32;
            let v23: Rc<str> = usize::try_from(v22).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v24: bool = &*v23 == "--as";
            if v24 {
                let mut v25: i32 = 8i32;
                let mut v26: Rc<str> = usize::try_from(v25).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                v26
            } else {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v27
            }
        };
        let mut v30: bool = &*v16 == "";
        let v33: Rc<str> = if v30 {
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v31
        } else {
            status_now_4(v1.clone(), v3.clone())
        };
        let v36: Rc<str> = if v30 {
            stale_warning_35(v1.clone(), v3.clone(), v29.clone())
        } else {
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v35
        };
        let v37: bool = &*v16 != "";
        let v39: bool = if v37 {
            let mut v38: bool = &*v33 != &*v16;
            v38
        } else {
            false
        };
        if v39 {
            expect_refusal_49(v3, v33, v16)
        } else {
            let v41: i32 = 3i32;
            let v42: Rc<str> = usize::try_from(v41).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v43: bool = &*v42 == "-";
            let v168: US11 = if v43 {
                let v44: i32 = 1i32;
                let v45: Rc<str> = usize::try_from(v44).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let v48: Rc<str> = Rc::<str>::from(v45.replace(&*v46, &*v47));
                let v49: Rc<str> = string_concat(&[&*v48, "/state/agile.spi"]);
                let mut v50: bool = std::path::Path::new(&*v49).is_file();
                let v102: Rc<str> = if v50 {
                    let (mut v51, mut v52): (bool, Rc<str>) = match std::fs::read(&*v49) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                    if v51 {
                        let v53: Rc<str> = string_concat(&["rig state read failed: ", &*v49]);
                        std::panic::panic_any::<std::string::String>(format!("{}", v53))
                    } else {
                        let v55: Rc<str> = Rc::<str>::from(v49.replace(&*v46, &*v47));
                        let mut v56: Rc<str> = loop_5(v55);
                        let v57: Option<usize> = v56.rfind(&*v47);
                        let v58: Option<usize> = method6(v57);
                        let mut v59: Rc<dyn Fn((usize)) -> i32> = closure3();
                        let mut v60: Option<i32> = v58.map(|x| v59(x));
                        let v62: i32 = v60.unwrap_or(-1i32);
                        let v63: bool = v62 < 0i32;
                        let v67: bool = if v63 {
                            true
                        } else {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                            let v65: bool = v56.ends_with(&*v64);
                            let v66: bool = v65 == false;
                            v66
                        };
                        let v88: US0 = if v67 {
                            US0::US0_OtherFile
                        } else {
                            let v69: usize = ((0i32) as usize);
                            let mut v70: usize = ((v62) as usize);
                            let v71: Rc<str> = Rc::<str>::from(&(v56)[v69..v70]);
                            let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                            let mut v73: bool = v71.ends_with(&*v72);
                            if v73 {
                                let mut v74: Rc<str> = loop_7(v71);
                                US0::US0_StateFile(v74)
                            } else {
                                let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                let v77: bool = v71.ends_with(&*v76);
                                if v77 {
                                    let v78: Rc<str> = loop_8(v71);
                                    US0::US0_HistoryFile(v78)
                                } else {
                                    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                    let mut v81: bool = v71.ends_with(&*v80);
                                    if v81 {
                                        let v82: Rc<str> = loop_9(v71);
                                        US0::US0_ArchiveFile(v82)
                                    } else {
                                        US0::US0_OtherFile
                                    }
                                }
                            }
                        };
                        match v88 {
                            US0::US0_OtherFile => {
                                v52
                            }
                            _ => {
                                let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let v90: Rc<UH0> = split_list_10(v89.clone(), v52);
                                let mut v91: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                let mut v92: Rc<UH0> = method13(v90, v91);
                                let v93: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                let v94: i32 = 0i32;
                                let v95: i32 = method14(v93.clone(), v92, v94);
                                let v96: Rc<Vec<Rc<str>>> = Rc::new(v93.borrow().clone());
                                let v97: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v96).as_ref().clone()));
                                let v98: Rc<str> = Rc::<str>::from((*v97.borrow()).join(&*v89));
                                v98
                            }
                        }
                    }
                } else {
                    let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v101
                };
                let v103: i32 = 2i32;
                let v104: Rc<str> = usize::try_from(v103).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v105: i32 = progress_in_50(v102, v104);
                let v106: bool = v105 >= 0i32;
                let v108: bool = if v106 {
                    let v107: bool = v105 <= 100i32;
                    v107
                } else {
                    false
                };
                if v108 {
                    US11::US11_Valid(v105)
                } else {
                    US11::US11_Invalid
                }
            } else {
                let v112: i32 = 3i32;
                let v113: i32 = usize::try_from(v112).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
                let v114: i32 = 3i32;
                let v115: i32 = 0i32;
                let v116: i32 = usize::try_from(v114).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v115).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                let v117: bool = 48i32 <= v116;
                let v119: bool = if v117 {
                    let v118: bool = v116 <= 57i32;
                    v118
                } else {
                    false
                };
                let v122: i32 = if v119 {
                    let v120: i32 = v116 - 48;
                    v120
                } else {
                    let v121: i32 = -1;
                    v121
                };
                let v123: i32 = 3i32;
                let mut v124: i32 = 1i32;
                let v125: i32 = usize::try_from(v123).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v124).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                let mut v126: bool = 48i32 <= v125;
                let v128: bool = if v126 {
                    let v127: bool = v125 <= 57i32;
                    v127
                } else {
                    false
                };
                let v131: i32 = if v128 {
                    let mut v129: i32 = v125 - 48;
                    v129
                } else {
                    let mut v130: i32 = -1;
                    v130
                };
                let v132: i32 = 3i32;
                let v133: i32 = 2i32;
                let mut v134: i32 = usize::try_from(v132).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v133).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                let mut v135: bool = 48i32 <= v134;
                let v137: bool = if v135 {
                    let v136: bool = v134 <= 57i32;
                    v136
                } else {
                    false
                };
                let v140: i32 = if v137 {
                    let v138: i32 = v134 - 48;
                    v138
                } else {
                    let v139: i32 = -1;
                    v139
                };
                let v141: bool = v113 == 1i32;
                let v142: bool = 0 <= v122;
                let v143: bool = v141 && v142;
                if v143 {
                    US11::US11_Valid(v122)
                } else {
                    let v145: bool = v113 == 2i32;
                    let v146: bool = 0 <= v122;
                    let v147: bool = 0 <= v131;
                    let v149: bool = if v145 {
                        let v148: bool = v146 && v147;
                        v148
                    } else {
                        false
                    };
                    if v149 {
                        let v150: i32 = v122 * 10 + v131;
                        US11::US11_Valid(v150)
                    } else {
                        let v152: bool = v113 == 3i32;
                        let v153: bool = 0 <= v122;
                        let v154: bool = 0 <= v131;
                        let v155: bool = 0 <= v140;
                        let v158: bool = if v152 {
                            if v153 {
                                let v156: bool = v154 && v155;
                                v156
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        if v158 {
                            let v159: i32 = v122 * 100 + v131 * 10 + v140;
                            let v160: bool = v159 <= 100i32;
                            if v160 {
                                US11::US11_Valid(v159)
                            } else {
                                US11::US11_Invalid
                            }
                        } else {
                            US11::US11_Invalid
                        }
                    }
                }
            };
            let mut v169: i32 = 4i32;
            let v170: Rc<str> = usize::try_from(v169).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v171: bool = &*v170 == "Planned";
            let v175: US12 = if v171 {
                let v172: US13 = US13::US13_AgilePlanned;
                US12::US12_Valid(v172)
            } else {
                US12::US12_Invalid
            };
            let v176: bool = &*v170 == "Active";
            let v179: US12 = if v176 {
                let v177: US13 = US13::US13_AgileActive;
                US12::US12_Valid(v177)
            } else {
                v175
            };
            let v180: bool = &*v170 == "Blocked";
            let v183: US12 = if v180 {
                let v181: US13 = US13::US13_AgileBlocked;
                US12::US12_Valid(v181)
            } else {
                v179
            };
            let v184: bool = &*v170 == "Paused";
            let v187: US12 = if v184 {
                let v185: US13 = US13::US13_AgilePaused;
                US12::US12_Valid(v185)
            } else {
                v183
            };
            let v188: bool = &*v170 == "Done";
            let v191: US12 = if v188 {
                let v189: US13 = US13::US13_AgileDone;
                US12::US12_Valid(v189)
            } else {
                v187
            };
            let v192: bool = &*v170 == "Dropped";
            let v195: US12 = if v192 {
                let v193: US13 = US13::US13_AgileDropped;
                US12::US12_Valid(v193)
            } else {
                v191
            };
            match v168 {
                US11::US11_Invalid => {
                    let v237: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
                    let v238: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a whole number 0..100"); } LIT.with(|lit| lit.clone()) };
                    let v242: i32 = 3i32;
                    let v243: Rc<str> = usize::try_from(v242).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let v248: Rc<str> = string_concat(&["agile set: ", &*v237, " must be ", &*v238, ", received '", &*v243, "'; usage: agile set <root> <id> <progress|-> <Planned|Active|Blocked|Paused|Done|Dropped> [--expect <status>] [--as <lane>]"]);
                    let v249: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v248 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v249
                }
                US11::US11_Valid(v196) => {
                    let v196: i32 = v196;
                    match v195 {
                        US12::US12_Invalid => {
                            let v222: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
                            let v223: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned, Active, Blocked, Paused, Done or Dropped"); } LIT.with(|lit| lit.clone()) };
                            let v227: i32 = 4i32;
                            let v228: Rc<str> = usize::try_from(v227).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let v233: Rc<str> = string_concat(&["agile set: ", &*v222, " must be ", &*v223, ", received '", &*v228, "'; usage: agile set <root> <id> <progress|-> <Planned|Active|Blocked|Paused|Done|Dropped> [--expect <status>] [--as <lane>]"]);
                            let v234: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v233 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v234
                        }
                        US12::US12_Valid(v197) => {
                            let v197: US13 = v197;
                            let v212: i32 = match &v197 {
                                US13::US13_AgileDone => {
                                    let v198: Rc<str> = std::rc::Rc::<str>::from(agile_todo_scan::rig_agile_todo_sites(&*v1, &*v3).trim_end());
                                    let v199: bool = &*v198 == "";
                                    if v199 {
                                        let v200: i32 = rig_agile_state::agile_set_validated_owned(v1.as_ref(), v3.as_ref(), v196, 4i32);
                                        v200
                                    } else {
                                        let v203: Rc<str> = string_concat(&["agile set ", &*v3, " rejected: TODO blocks still name it; cut them or keep the item open:\n", &*v198]);
                                        let v204: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v203 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                        v204
                                    }
                                }
                                _ => {
                                    let v210: i32 = match v197 {
                                        US13::US13_AgileActive => {
                                            1i32
                                        }
                                        US13::US13_AgileBlocked => {
                                            2i32
                                        }
                                        US13::US13_AgileDropped => {
                                            5i32
                                        }
                                        US13::US13_AgilePaused => {
                                            3i32
                                        }
                                        US13::US13_AgilePlanned => {
                                            0i32
                                        }
                                        _ => unreachable!(),
                                    };
                                    let v211: i32 = rig_agile_state::agile_set_validated_owned(v1.as_ref(), v3.as_ref(), v196, v210);
                                    v211
                                }
                            };
                            let v213: bool = v212 == 0i32;
                            if v213 {
                                let v214: i32 = 4i32;
                                let v215: Rc<str> = usize::try_from(v214).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                lint_warned_51(v1.clone(), v3.clone(), v215)
                            };
                            let v217: bool = if v213 {
                                let v216: bool = &*v36 != "";
                                v216
                            } else {
                                false
                            };
                            if v217 {
                                eprintln!("{}", v36);
                                ()
                            };
                            let v221: i32 = if v213 {
                                let v218: i32 = 4i32;
                                let v219: Rc<str> = usize::try_from(v218).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let v220: i32 = agile_status_changes::rig_agile_status_change_recorded(&v1, &v3, &v219, &v29);
                                v220
                            } else {
                                0i32
                            };
                            v212
                        }
                    }
                }
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_field_named_53(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v1.to_ascii_lowercase());
    let mut v3: bool = &*v0 == &*v2;
    if v3 {
        true
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v4.to_ascii_lowercase());
        let mut v6: bool = &*v0 == &*v5;
        if v6 {
            true
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
            let mut v9: bool = &*v0 == &*v8;
            v9
        }
    }
}
fn closure8() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 3i32;
        let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v2: bool = &*v1 == "--parent";
        if v2 {
            let mut v3: i32 = agile_task_parents::rig_agile_parent_edit_run();
            v3
        } else {
            let mut v4: i32 = 3i32;
            let mut v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v6: bool = &*v5 == "--after";
            if v6 {
                let mut v7: i32 = agile_task_afters::rig_agile_after_edit_run();
                v7
            } else {
                let mut v8: i32 = 3i32;
                let mut v9: Rc<str> = usize::try_from(v8).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v10: bool = &*v9 == "--asked";
                if v10 {
                    let v11: i32 = agile_task_asks::rig_agile_ask_edit_run();
                    v11
                } else {
                    let mut v12: i32 = 3i32;
                    let mut v13: Rc<str> = usize::try_from(v12).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v14: bool = &*v13 == "--split";
                    if v14 {
                        let mut v15: i32 = agile_task_split::rig_agile_split_run();
                        v15
                    } else {
                        let mut v16: i32 = 3i32;
                        let v17: Rc<str> = usize::try_from(v16).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v18: bool = &*v17 == "--wait";
                        let v22: bool = if v18 {
                            true
                        } else {
                            let mut v19: i32 = 3i32;
                            let v20: Rc<str> = usize::try_from(v19).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let v21: bool = &*v20 == "--wait-remove";
                            v21
                        };
                        if v22 {
                            let v23: i32 = agile_task_waits::rig_agile_wait_run();
                            v23
                        } else {
                            let v24: i32 = 3i32;
                            let mut v25: Rc<str> = usize::try_from(v24).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v26: bool = planning_field_named_53(v25);
                            if v26 {
                                let v27: i32 = agile_planning_revision::rig_agile_planning_edit_run();
                                v27
                            } else {
                                let v28: i32 = agile_task_revision::rig_agile_edit_run();
                                v28
                            }
                        }
                    }
                }
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = agile_task_asks::rig_agile_ask_now_run();
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_usage_error() -> i32 {
    closure0()()
}
pub fn rig_agile_begin_route() -> i32 {
    closure1()()
}
pub fn rig_agile_set_route() -> i32 {
    closure2()()
}
pub fn rig_agile_edit_route() -> i32 {
    closure8()()
}
pub fn rig_agile_ask_route() -> i32 {
    closure9()()
}
use agile_task_parents::SpiralShape_97df63ebabc3 as Mut0;
pub type SpiralShape_652accfc6a8d = Mut1;
