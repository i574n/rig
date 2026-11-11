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
    US0_Valid(i32),
    US0_Invalid,
}
#[derive(Clone)]
enum US2 {
    US2_AgilePlanned,
    US2_AgileActive,
    US2_AgileBlocked,
    US2_AgilePaused,
    US2_AgileDone,
}
#[derive(Clone)]
enum US1 {
    US1_Valid(US2),
    US1_Invalid,
}
#[derive(Clone)]
enum US3 {
    US3_StateFile(Rc<str>),
    US3_HistoryFile(Rc<str>),
    US3_ArchiveFile(Rc<str>),
    US3_OtherFile,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut1 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US5 {
    US5_HeadInl,
    US5_HeadUnion,
    US5_HeadOpen,
    US5_HeadType,
}
#[derive(Clone)]
enum US4 {
    US4_KeywordHead(US5),
    US4_SymbolHead(US5),
    US4_OtherLine,
}
fn agile_subcommand_shape_1(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = &*v0 == "init";
    let mut v4: Rc<str> = if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--dialect symbols|words]"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3
    };
    let v5: bool = &*v4 == "";
    let mut v7: bool = if v5 {
        let v6: bool = &*v0 == "begin";
        v6
    } else {
        false
    };
    let mut v9: Rc<str> = if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <title>"); } LIT.with(|lit| lit.clone()) };
        v8
    } else {
        v4
    };
    let v10: bool = &*v9 == "";
    let v12: bool = if v10 {
        let v11: bool = &*v0 == "lease";
        v11
    } else {
        false
    };
    let v14: Rc<str> = if v12 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v13
    } else {
        v9
    };
    let v15: bool = &*v14 == "";
    let mut v17: bool = if v15 {
        let mut v16: bool = &*v0 == "status";
        v16
    } else {
        false
    };
    let v19: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
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
    let v25: bool = &*v24 == "";
    let v27: bool = if v25 {
        let v26: bool = &*v0 == "list";
        v26
    } else {
        false
    };
    let v29: Rc<str> = if v27 {
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json]"); } LIT.with(|lit| lit.clone()) };
        v28
    } else {
        v24
    };
    let v30: bool = &*v29 == "";
    let v32: bool = if v30 {
        let v31: bool = &*v0 == "check";
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
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
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
        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"); } LIT.with(|lit| lit.clone()) };
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
    let v50: bool = &*v49 == "";
    let v52: bool = if v50 {
        let v51: bool = &*v0 == "edit";
        v51
    } else {
        false
    };
    let v54: Rc<str> = if v52 {
        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>"); } LIT.with(|lit| lit.clone()) };
        v53
    } else {
        v49
    };
    let v55: bool = &*v54 == "";
    let v57: bool = if v55 {
        let v56: bool = &*v0 == "show";
        v56
    } else {
        false
    };
    let v59: Rc<str> = if v57 {
        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
        v58
    } else {
        v54
    };
    let v60: bool = &*v59 == "";
    let v62: bool = if v60 {
        let v61: bool = &*v0 == "plan";
        v61
    } else {
        false
    };
    let v64: Rc<str> = if v62 {
        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--json]"); } LIT.with(|lit| lit.clone()) };
        v63
    } else {
        v59
    };
    let v65: bool = &*v64 == "";
    let v67: bool = if v65 {
        let v66: bool = &*v0 == "migrate-tags";
        v66
    } else {
        false
    };
    let v69: Rc<str> = if v67 {
        let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview|apply <root>"); } LIT.with(|lit| lit.clone()) };
        v68
    } else {
        v64
    };
    let v70: bool = &*v69 == "";
    let v72: bool = if v70 {
        let v71: bool = &*v0 == "migrate-activity";
        v71
    } else {
        false
    };
    let v74: Rc<str> = if v72 {
        let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview|apply <root>"); } LIT.with(|lit| lit.clone()) };
        v73
    } else {
        v69
    };
    let v75: bool = &*v74 == "";
    let v77: bool = if v75 {
        let v76: bool = &*v0 == "sprint";
        v76
    } else {
        false
    };
    let v79: Rc<str> = if v77 {
        let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start <root> <id> <days> <goal> [--items A,B] | show <root> [<id>] | add|drop <root> <id> <item> | close <root> <id> | velocity <root> [--last n]"); } LIT.with(|lit| lit.clone()) };
        v78
    } else {
        v74
    };
    let v80: bool = &*v79 == "";
    let v82: bool = if v80 {
        let v81: bool = &*v0 == "lint";
        v81
    } else {
        false
    };
    let v84: Rc<str> = if v82 {
        let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--census | --team <team>]"); } LIT.with(|lit| lit.clone()) };
        v83
    } else {
        v79
    };
    let v85: bool = &*v84 == "";
    let v87: bool = if v85 {
        let v86: bool = &*v0 == "gate";
        v86
    } else {
        false
    };
    let v89: Rc<str> = if v87 {
        let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
        v88
    } else {
        v84
    };
    let v90: bool = &*v89 == "";
    let v92: bool = if v90 {
        let v91: bool = &*v0 == "cadence";
        v91
    } else {
        false
    };
    let v94: Rc<str> = if v92 {
        let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--hours n] [--days n | --since yyyy-mm-dd] [--team t] [--round h]"); } LIT.with(|lit| lit.clone()) };
        v93
    } else {
        v89
    };
    let v95: bool = &*v94 == "";
    let v97: bool = if v95 {
        let v96: bool = &*v0 == "note";
        v96
    } else {
        false
    };
    let v99: Rc<str> = if v97 {
        let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <text> [--as <author>]"); } LIT.with(|lit| lit.clone()) };
        v98
    } else {
        v94
    };
    let v100: bool = &*v99 == "";
    let v102: bool = if v100 {
        let v101: bool = &*v0 == "note-edit";
        v101
    } else {
        false
    };
    let v104: Rc<str> = if v102 {
        let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>]"); } LIT.with(|lit| lit.clone()) };
        v103
    } else {
        v99
    };
    let v105: bool = &*v104 == "";
    let v107: bool = if v105 {
        let v106: bool = &*v0 == "notes";
        v106
    } else {
        false
    };
    let v109: Rc<str> = if v107 {
        let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("from-title <root> <id>"); } LIT.with(|lit| lit.clone()) };
        v108
    } else {
        v104
    };
    let v110: bool = &*v109 == "";
    let v112: bool = if v110 {
        let v111: bool = &*v0 == "rename";
        v111
    } else {
        false
    };
    let v114: Rc<str> = if v112 {
        let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
        v113
    } else {
        v109
    };
    let v115: bool = &*v114 == "";
    let v117: bool = if v115 {
        let v116: bool = &*v0 == "seal";
        v116
    } else {
        false
    };
    let v119: Rc<str> = if v117 {
        let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--dry-run]"); } LIT.with(|lit| lit.clone()) };
        v118
    } else {
        v114
    };
    let v120: bool = &*v119 == "";
    let v122: bool = if v120 {
        let v121: bool = &*v0 == "todos";
        v121
    } else {
        false
    };
    let v124: Rc<str> = if v122 {
        let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record"); } LIT.with(|lit| lit.clone()) };
        v123
    } else {
        v119
    };
    let v125: bool = &*v124 == "";
    let v127: bool = if v125 {
        let v126: bool = &*v0 == "next";
        v126
    } else {
        false
    };
    let v129: Rc<str> = if v127 {
        let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | <root> --any --as <lane> [--ttl <minutes>] (claims the top free item)"); } LIT.with(|lit| lit.clone()) };
        v128
    } else {
        v124
    };
    let v130: bool = &*v129 == "";
    let v132: bool = if v130 {
        let v131: bool = &*v0 == "claim";
        v131
    } else {
        false
    };
    let v134: Rc<str> = if v132 {
        let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> --as <lane> [--paths <glob,glob>] [--ttl <minutes>]"); } LIT.with(|lit| lit.clone()) };
        v133
    } else {
        v129
    };
    let v135: bool = &*v134 == "";
    let v137: bool = if v135 {
        let v136: bool = &*v0 == "release";
        v136
    } else {
        false
    };
    let v139: Rc<str> = if v137 {
        let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> --as <lane>"); } LIT.with(|lit| lit.clone()) };
        v138
    } else {
        v134
    };
    let v140: bool = &*v139 == "";
    let v142: bool = if v140 {
        let v141: bool = &*v0 == "claims";
        v141
    } else {
        false
    };
    let v144: Rc<str> = if v142 {
        let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--as <lane>]"); } LIT.with(|lit| lit.clone()) };
        v143
    } else {
        v139
    };
    let v145: bool = &*v144 == "";
    let v147: bool = if v145 {
        let v146: bool = &*v0 == "roots";
        v146
    } else {
        false
    };
    let v149: Rc<str> = if v147 {
        let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [add|drop <path>...]"); } LIT.with(|lit| lit.clone()) };
        v148
    } else {
        v144
    };
    let v150: bool = &*v149 == "";
    let v152: bool = if v150 {
        let v151: bool = &*v0 == "links";
        v151
    } else {
        false
    };
    let v154: Rc<str> = if v152 {
        let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --memory <dir> [--json]"); } LIT.with(|lit| lit.clone()) };
        v153
    } else {
        v149
    };
    let v155: bool = &*v154 == "";
    let v157: bool = if v155 {
        let v156: bool = &*v0 == "wip";
        v156
    } else {
        false
    };
    let v159: Rc<str> = if v157 {
        let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v158
    } else {
        v154
    };
    let v160: bool = &*v159 == "";
    let v162: bool = if v160 {
        let v161: bool = &*v0 == "views";
        v161
    } else {
        false
    };
    let v164: Rc<str> = if v162 {
        let v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v163
    } else {
        v159
    };
    let v165: bool = &*v164 == "";
    let v167: bool = if v165 {
        let v166: bool = &*v0 == "view";
        v166
    } else {
        false
    };
    let v169: Rc<str> = if v167 {
        let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> [extra flags...]"); } LIT.with(|lit| lit.clone()) };
        v168
    } else {
        v164
    };
    let v170: bool = &*v169 == "";
    let v172: bool = if v170 {
        let v171: bool = &*v0 == "view-add";
        v171
    } else {
        false
    };
    let v174: Rc<str> = if v172 {
        let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> <list|next> [flags...]"); } LIT.with(|lit| lit.clone()) };
        v173
    } else {
        v169
    };
    let v175: bool = &*v174 == "";
    let v177: bool = if v175 {
        let v176: bool = &*v0 == "dialect";
        v176
    } else {
        false
    };
    if v177 {
        let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <symbols|words>"); } LIT.with(|lit| lit.clone()) };
        v178
    } else {
        v174
    }
}
fn method3(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_2(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method3(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn agile_usage_error_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v2: Rc<str> = agile_subcommand_shape_1(v1.clone());
    let v3: bool = &*v2 == "";
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile supports init <root> [--dialect symbols|words], begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root> [--compiler <absolute>], record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>], edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>, migrate-activity preview|apply <root>, sprint start <root> <id> <days> <goal> [--items A,B] | show <root> [<id>] | add|drop <root> <id> <item> | close <root> <id> | velocity <root> [--last n], lint <root> [--census | --team <team>], gate <root> --team <team> --as <lane>, cadence <root> [--hours n] [--days n | --since yyyy-mm-dd] [--team t] [--round h], note <root> <id> <text> [--as <author>], note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>], notes from-title <root> <id>, rename <root> <old-id> <new-id>, seal <root> [--dry-run], todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record, next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | <root> --any --as <lane> [--ttl <minutes>] (claims the top free item), claim <root> <ID> --as <lane> [--paths <glob,glob>] [--ttl <minutes>], release <root> <ID> --as <lane>, claims <root> [--as <lane>], roots <root> [add|drop <path>...], links <root> --memory <dir> [--json], wip <root>, views <root>, view <root> <name> [extra flags...], view-add <root> <name> <list|next> [flags...], dialect <root> <symbols|words>"); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v4 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v5
    } else {
        let v10: i32 = std::env::args().skip(3i32 as usize).count() as i32;
        let v11: Rc<str> = format_real_2(v10);
        let mut v13: Rc<str> = string_concat(&["agile ", &*v1, " expects ", &*v2, ", received ", &*v11, " operands; see kit agile --help"]);
        let v14: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v13 + "
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
        let v2: i32 = 2i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v4: i32 = rig_agile_lease::rig_lease_title_words();
        let v5: bool = 0 < v4;
        if v5 {
            let v6: i32 = rig_agile_state::agile_begin_owned(v1.as_ref(), v3.as_ref());
            v6
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin title must contain 3..7 visible words"); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v7 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v8
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
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
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(6i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
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
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
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
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
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
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
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
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method12(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method11(v16, v17)
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method13(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let v15: US4 = if v6 {
                let mut v7: US5 = US5::US5_HeadType;
                US4::US4_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US5 = US5::US5_HeadType;
                    US4::US4_SymbolHead(v11)
                } else {
                    US4::US4_OtherLine
                }
            };
            let v27: US4 = match &v15 {
                US4::US4_OtherLine => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US5 = US5::US5_HeadInl;
                        US4::US4_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US5 = US5::US5_HeadInl;
                            US4::US4_SymbolHead(v22)
                        } else {
                            US4::US4_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US4 = match &v27 {
                US4::US4_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US5 = US5::US5_HeadUnion;
                        US4::US4_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US5 = US5::US5_HeadUnion;
                            US4::US4_SymbolHead(v34)
                        } else {
                            US4::US4_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US4 = match &v39 {
                US4::US4_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US5 = US5::US5_HeadOpen;
                        US4::US4_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US5 = US5::US5_HeadOpen;
                            US4::US4_SymbolHead(v46)
                        } else {
                            US4::US4_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US4::US4_SymbolHead(v52) => {
                    let v52: US5 = v52;
                    let v60: Rc<str> = match &v52 {
                        US5::US5_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US5::US5_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US5::US5_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US5::US5_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US5::US5_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US5::US5_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US5::US5_HeadType => {
                            v5
                        }
                        US5::US5_HeadUnion => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                    };
                    let v69: i32 = (v2.clone().len() as i32);
                    let v70: i32 = v69.wrapping_sub(v61);
                    let v71: usize = ((v61) as usize);
                    let v72: i32 = v61.wrapping_add(v70);
                    let v73: usize = ((v72) as usize);
                    let v74: Rc<str> = Rc::<str>::from(&(v2)[v71..v73]);
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
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn checks_joined_15(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let (mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>) = v0.borrow()[v1 as usize].clone();
            let v8: i32 = v1.wrapping_add(1i32);
            let mut v9: bool = &*v2 == "";
            let v12: Rc<str> = if v9 {
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
fn lint_warned_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> () {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v0.replace(&*v3, &*v4));
    let v6: Rc<str> = string_concat(&[&*v5, "/state/agile.spi"]);
    let mut v7: bool = std::path::Path::new(&*v6).is_file();
    let v59: Rc<str> = if v7 {
        let (mut v8, mut v9): (bool, Rc<str>) = match std::fs::read(&*v6) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v8 {
            let v10: Rc<str> = string_concat(&["rig state read failed: ", &*v6]);
            std::panic::panic_any::<std::string::String>(format!("{}", v10))
        } else {
            let v12: Rc<str> = Rc::<str>::from(v6.replace(&*v3, &*v4));
            let mut v13: Rc<str> = loop_5(v12);
            let v14: Option<usize> = v13.rfind(&*v4);
            let v15: Option<usize> = method6(v14);
            let mut v16: Rc<dyn Fn((usize)) -> i32> = closure3();
            let mut v17: Option<i32> = v15.map(|x| v16(x));
            let v19: i32 = v17.unwrap_or(-1i32);
            let v20: bool = v19 < 0i32;
            let v24: bool = if v20 {
                true
            } else {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v22: bool = v13.ends_with(&*v21);
                let v23: bool = v22 == false;
                v23
            };
            let v45: US3 = if v24 {
                US3::US3_OtherFile
            } else {
                let v26: usize = ((0i32) as usize);
                let v27: usize = ((v19) as usize);
                let v28: Rc<str> = Rc::<str>::from(&(v13)[v26..v27]);
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v30: bool = v28.ends_with(&*v29);
                if v30 {
                    let v31: Rc<str> = loop_7(v28);
                    US3::US3_StateFile(v31)
                } else {
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v34: bool = v28.ends_with(&*v33);
                    if v34 {
                        let v35: Rc<str> = loop_8(v28);
                        US3::US3_HistoryFile(v35)
                    } else {
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v38: bool = v28.ends_with(&*v37);
                        if v38 {
                            let v39: Rc<str> = loop_9(v28);
                            US3::US3_ArchiveFile(v39)
                        } else {
                            US3::US3_OtherFile
                        }
                    }
                }
            };
            match v45 {
                US3::US3_OtherFile => {
                    v9
                }
                _ => {
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v47: Rc<UH0> = split_list_10(v46.clone(), v9);
                    let v48: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v49: Rc<UH0> = method13(v47, v48);
                    let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v51: i32 = 0i32;
                    let v52: i32 = method14(v50.clone(), v49, v51);
                    let v53: Rc<Vec<Rc<str>>> = Rc::new(v50.borrow().clone());
                    let v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v53).as_ref().clone()));
                    let v55: Rc<str> = Rc::<str>::from((*v54.borrow()).join(&*v46));
                    v55
                }
            }
        }
    } else {
        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v58
    };
    let v60: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = agile_quality_state::rig_agile_lint_item_failures(&v59, &v1, &v2);
    let v61: i32 = 0i32;
    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v63: Rc<str> = checks_joined_15(&v60, v61, v62);
    let v64: bool = &*v63 == "";
    if v64 {
        ()
    } else {
        let v71: Rc<str> = string_concat(&["rig agile lint warning id=", &*v1, " status=", &*v2, " failing=", &*v63, " (add a note with the marker lines, e.g. 'dod:' or 'pronto quando:', 'next:' or 'próximo:'; a later train refuses the change)"]);
        eprintln!("{}", v71);
        ()
    }
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 1i32;
        let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v2: i32 = 2i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v4: i32 = 3i32;
        let v5: i32 = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
        let v6: i32 = 3i32;
        let mut v7: i32 = 0i32;
        let v8: i32 = usize::try_from(v6).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v7).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
        let mut v9: bool = 48i32 <= v8;
        let v11: bool = if v9 {
            let v10: bool = v8 <= 57i32;
            v10
        } else {
            false
        };
        let v14: i32 = if v11 {
            let v12: i32 = v8 - 48;
            v12
        } else {
            let mut v13: i32 = -1;
            v13
        };
        let v15: i32 = 3i32;
        let mut v16: i32 = 1i32;
        let mut v17: i32 = usize::try_from(v15).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v16).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
        let v18: bool = 48i32 <= v17;
        let v20: bool = if v18 {
            let v19: bool = v17 <= 57i32;
            v19
        } else {
            false
        };
        let v23: i32 = if v20 {
            let v21: i32 = v17 - 48;
            v21
        } else {
            let v22: i32 = -1;
            v22
        };
        let v24: i32 = 3i32;
        let v25: i32 = 2i32;
        let v26: i32 = usize::try_from(v24).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v25).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
        let v27: bool = 48i32 <= v26;
        let v29: bool = if v27 {
            let v28: bool = v26 <= 57i32;
            v28
        } else {
            false
        };
        let v32: i32 = if v29 {
            let v30: i32 = v26 - 48;
            v30
        } else {
            let v31: i32 = -1;
            v31
        };
        let v33: bool = v5 == 1i32;
        let v34: bool = 0 <= v14;
        let v35: bool = v33 && v34;
        let v59: US0 = if v35 {
            US0::US0_Valid(v14)
        } else {
            let v37: bool = v5 == 2i32;
            let v38: bool = 0 <= v14;
            let v39: bool = 0 <= v23;
            let v41: bool = if v37 {
                let v40: bool = v38 && v39;
                v40
            } else {
                false
            };
            if v41 {
                let v42: i32 = v14 * 10 + v23;
                US0::US0_Valid(v42)
            } else {
                let v44: bool = v5 == 3i32;
                let v45: bool = 0 <= v14;
                let v46: bool = 0 <= v23;
                let v47: bool = 0 <= v32;
                let v50: bool = if v44 {
                    if v45 {
                        let v48: bool = v46 && v47;
                        v48
                    } else {
                        false
                    }
                } else {
                    false
                };
                if v50 {
                    let v51: i32 = v14 * 100 + v23 * 10 + v32;
                    let v52: bool = v51 <= 100i32;
                    if v52 {
                        US0::US0_Valid(v51)
                    } else {
                        US0::US0_Invalid
                    }
                } else {
                    US0::US0_Invalid
                }
            }
        };
        let v60: i32 = 4i32;
        let v61: Rc<str> = usize::try_from(v60).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v62: bool = &*v61 == "Planned";
        let v66: US1 = if v62 {
            let v63: US2 = US2::US2_AgilePlanned;
            US1::US1_Valid(v63)
        } else {
            US1::US1_Invalid
        };
        let v67: bool = &*v61 == "Active";
        let v70: US1 = if v67 {
            let v68: US2 = US2::US2_AgileActive;
            US1::US1_Valid(v68)
        } else {
            v66
        };
        let v71: bool = &*v61 == "Blocked";
        let v74: US1 = if v71 {
            let v72: US2 = US2::US2_AgileBlocked;
            US1::US1_Valid(v72)
        } else {
            v70
        };
        let v75: bool = &*v61 == "Paused";
        let v78: US1 = if v75 {
            let v76: US2 = US2::US2_AgilePaused;
            US1::US1_Valid(v76)
        } else {
            v74
        };
        let v79: bool = &*v61 == "Done";
        let v82: US1 = if v79 {
            let v80: US2 = US2::US2_AgileDone;
            US1::US1_Valid(v80)
        } else {
            v78
        };
        match v59 {
            US0::US0_Invalid => {
                let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
                let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a whole number 0..100"); } LIT.with(|lit| lit.clone()) };
                let v125: i32 = 3i32;
                let v126: Rc<str> = usize::try_from(v125).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v130: Rc<str> = string_concat(&["agile set: ", &*v120, " must be ", &*v121, ", received '", &*v126, "'; usage: agile set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"]);
                let v131: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v130 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v131
            }
            US0::US0_Valid(v83) => {
                let v83: i32 = v83;
                match v82 {
                    US1::US1_Invalid => {
                        let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
                        let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned, Active, Blocked, Paused or Done"); } LIT.with(|lit| lit.clone()) };
                        let v111: i32 = 4i32;
                        let v112: Rc<str> = usize::try_from(v111).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let v116: Rc<str> = string_concat(&["agile set: ", &*v106, " must be ", &*v107, ", received '", &*v112, "'; usage: agile set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"]);
                        let v117: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v116 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v117
                    }
                    US1::US1_Valid(v84) => {
                        let v84: US2 = v84;
                        let v98: i32 = match &v84 {
                            US2::US2_AgileDone => {
                                let v85: Rc<str> = std::rc::Rc::<str>::from(agile_todo_scan::rig_agile_todo_sites(&*v1, &*v3).trim_end());
                                let v86: bool = &*v85 == "";
                                if v86 {
                                    let v87: i32 = rig_agile_state::agile_set_validated_owned(v1.as_ref(), v3.as_ref(), v83, 4i32);
                                    v87
                                } else {
                                    let v90: Rc<str> = string_concat(&["agile set ", &*v3, " rejected: TODO blocks still name it; cut them or keep the item open:\n", &*v85]);
                                    let v91: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v90 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v91
                                }
                            }
                            _ => {
                                let v96: i32 = match v84 {
                                    US2::US2_AgileActive => {
                                        1i32
                                    }
                                    US2::US2_AgileBlocked => {
                                        2i32
                                    }
                                    US2::US2_AgilePaused => {
                                        3i32
                                    }
                                    US2::US2_AgilePlanned => {
                                        0i32
                                    }
                                    _ => unreachable!(),
                                };
                                let v97: i32 = rig_agile_state::agile_set_validated_owned(v1.as_ref(), v3.as_ref(), v83, v96);
                                v97
                            }
                        };
                        let v99: bool = v98 == 0i32;
                        if v99 {
                            let v100: i32 = 4i32;
                            let v101: Rc<str> = usize::try_from(v100).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            lint_warned_4(v1.clone(), v3.clone(), v101)
                        };
                        let v105: i32 = if v99 {
                            let v102: i32 = 4i32;
                            let v103: Rc<str> = usize::try_from(v102).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let v104: i32 = agile_status_changes::rig_agile_status_change_recorded(&v1, &v3, &v103);
                            v104
                        } else {
                            0i32
                        };
                        v98
                    }
                }
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_field_named_16(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = std::rc::Rc::<str>::from(v1.to_ascii_lowercase());
    let v3: bool = &*v0 == &*v2;
    if v3 {
        true
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = std::rc::Rc::<str>::from(v4.to_ascii_lowercase());
        let v6: bool = &*v0 == &*v5;
        if v6 {
            true
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
            let mut v9: bool = &*v0 == &*v8;
            v9
        }
    }
}
fn closure4() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 3i32;
        let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v2: bool = &*v1 == "--parent";
        if v2 {
            let v3: i32 = agile_task_parents::rig_agile_parent_edit_run();
            v3
        } else {
            let mut v4: i32 = 3i32;
            let v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v6: bool = &*v5 == "--asked";
            if v6 {
                let mut v7: i32 = agile_task_asks::rig_agile_ask_edit_run();
                v7
            } else {
                let v8: i32 = 3i32;
                let mut v9: Rc<str> = usize::try_from(v8).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v10: bool = &*v9 == "--split";
                if v10 {
                    let v11: i32 = agile_task_split::rig_agile_split_run();
                    v11
                } else {
                    let v12: i32 = 3i32;
                    let mut v13: Rc<str> = usize::try_from(v12).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let v14: bool = &*v13 == "--wait";
                    if v14 {
                        let v15: i32 = agile_task_waits::rig_agile_wait_run();
                        v15
                    } else {
                        let mut v16: i32 = 3i32;
                        let mut v17: Rc<str> = usize::try_from(v16).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let v18: bool = planning_field_named_16(v17);
                        if v18 {
                            let v19: i32 = agile_planning_revision::rig_agile_planning_edit_run();
                            v19
                        } else {
                            let v20: i32 = agile_task_revision::rig_agile_edit_run();
                            v20
                        }
                    }
                }
            }
        }
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
    closure4()()
}
use agile_task_parents::SpiralShape_97df63ebabc3 as Mut0;
pub type SpiralShape_652accfc6a8d = Mut1;
