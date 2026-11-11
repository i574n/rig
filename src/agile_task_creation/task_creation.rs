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
    US0_Valid(Rc<str>),
    US0_Invalid,
}
#[derive(Clone)]
enum US2 {
    US2_AgileEpic,
    US2_AgileFeature,
    US2_AgileStory,
    US2_AgileTaskKind,
}
#[derive(Clone)]
enum US1 {
    US1_Valid(US2),
    US1_Invalid,
}
#[derive(Clone)]
enum US4 {
    US4_AgileP0,
    US4_AgileP1,
    US4_AgileP2,
    US4_AgileP3,
    US4_AgileP4,
    US4_AgileP5,
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US4),
    US3_Invalid,
}
#[derive(Clone)]
enum US6 {
    US6_AgileD0,
    US6_AgileD1,
    US6_AgileD2,
    US6_AgileD3,
    US6_AgileD4,
    US6_AgileD5,
    US6_AgileD6,
    US6_AgileD7,
    US6_AgileD8,
}
#[derive(Clone)]
enum US5 {
    US5_Valid(US6),
    US5_Invalid,
}
#[derive(Clone)]
enum US7 {
    US7_ExtrasRead(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US7_ExtrasRefused(Rc<str>),
}
#[derive(Clone)]
enum US9 {
    US9_FlagEstimate,
    US9_FlagLoad,
    US9_FlagAssignee,
    US9_FlagDeps,
    US9_FlagNote,
    US9_FlagAs,
}
#[derive(Clone)]
enum US8 {
    US8_FlagKnown(US9),
    US8_FlagUnknown,
}
#[derive(Clone)]
enum US10 {
    US10_FileReadable(Rc<str>),
    US10_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US11 {
    US11_StateFile(Rc<str>),
    US11_HistoryFile(Rc<str>),
    US11_ArchiveFile(Rc<str>),
    US11_OtherFile,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US13 {
    US13_HeadInl,
    US13_HeadUnion,
    US13_HeadOpen,
    US13_HeadType,
}
#[derive(Clone)]
enum US12 {
    US12_KeywordHead(US13),
    US12_SymbolHead(US13),
    US12_OtherLine,
}
#[derive(Clone)]
enum US14 {
    US14_AddedText(Rc<str>),
    US14_AddedRefused(Rc<str>),
}
#[derive(Clone)]
enum US15 {
    US15_Replaced,
    US15_TemporaryWriteFailed,
    US15_RenameFailed,
    US15_RenameFailedTemporaryKept,
    US15_ReadbackMismatch,
    US15_ChangedSinceRead,
}
#[derive(Clone)]
enum US16 {
    US16_WordsRoot,
    US16_SymbolsRoot,
}
fn concat5_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn task_id_bytes_allowed_2(mut v0: i32, mut v1: i32) -> bool {
    loop {
        let mut v2: bool = v0 == v1;
        if v2 {
            return true;
        } else {
            let v3: i32 = 2i32;
            let v4: i32 = usize::try_from(v3).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v0).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
            let v5: bool = 65i32 <= v4;
            let v7: bool = if v5 {
                let v6: bool = v4 <= 90i32;
                v6
            } else {
                false
            };
            let v13: bool = if v7 {
                true
            } else {
                let mut v8: bool = 48i32 <= v4;
                let v10: bool = if v8 {
                    let mut v9: bool = v4 <= 57i32;
                    v9
                } else {
                    false
                };
                if v10 {
                    true
                } else {
                    let mut v11: bool = v4 == 45i32;
                    v11
                }
            };
            if v13 {
                let v14: i32 = v0.wrapping_add(1i32);
                v0 = v14;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn flag_of_word_4(mut v0: Rc<str>) -> US8 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = std::rc::Rc::<str>::from(v2.to_ascii_lowercase());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_1(v1.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    let v6: bool = &*v0 == &*v5;
    if v6 {
        let v7: US9 = US9::US9_FlagEstimate;
        US8::US8_FlagKnown(v7)
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
        let mut v11: Rc<str> = concat5_1(v1.clone(), v10, v4.clone(), v4.clone(), v4.clone());
        let mut v12: bool = &*v0 == &*v11;
        if v12 {
            let v13: US9 = US9::US9_FlagLoad;
            US8::US8_FlagKnown(v13)
        } else {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = std::rc::Rc::<str>::from(v15.to_ascii_lowercase());
            let v17: Rc<str> = concat5_1(v1.clone(), v16, v4.clone(), v4.clone(), v4.clone());
            let v18: bool = &*v0 == &*v17;
            if v18 {
                let v19: US9 = US9::US9_FlagAssignee;
                US8::US8_FlagKnown(v19)
            } else {
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Deps"); } LIT.with(|lit| lit.clone()) };
                let v22: Rc<str> = std::rc::Rc::<str>::from(v21.to_ascii_lowercase());
                let v23: Rc<str> = concat5_1(v1.clone(), v22, v4.clone(), v4.clone(), v4.clone());
                let v24: bool = &*v0 == &*v23;
                if v24 {
                    let v25: US9 = US9::US9_FlagDeps;
                    US8::US8_FlagKnown(v25)
                } else {
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note"); } LIT.with(|lit| lit.clone()) };
                    let v28: Rc<str> = std::rc::Rc::<str>::from(v27.to_ascii_lowercase());
                    let v29: Rc<str> = concat5_1(v1.clone(), v28, v4.clone(), v4.clone(), v4.clone());
                    let v30: bool = &*v0 == &*v29;
                    if v30 {
                        let v31: US9 = US9::US9_FlagNote;
                        US8::US8_FlagKnown(v31)
                    } else {
                        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                        let v34: Rc<str> = std::rc::Rc::<str>::from(v33.to_ascii_lowercase());
                        let v35: Rc<str> = concat5_1(v1, v34, v4.clone(), v4.clone(), v4);
                        let v36: bool = &*v0 == &*v35;
                        if v36 {
                            let v37: US9 = US9::US9_FlagAs;
                            US8::US8_FlagKnown(v37)
                        } else {
                            US8::US8_FlagUnknown
                        }
                    }
                }
            }
        }
    }
}
fn extras_from_3(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> US7 {
    loop {
        let mut v8: bool = v1 <= v0;
        if v8 {
            return US7::US7_ExtrasRead(v2, v3, v4, v5, v6, v7);
        } else {
            let v10: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v11: US8 = flag_of_word_4(v10.clone());
            match v11 {
                US8::US8_FlagKnown(v12) => {
                    let mut v12: US9 = v12;
                    let v13: i32 = v0.wrapping_add(1i32);
                    let v14: bool = v1 <= v13;
                    if v14 {
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs a value"); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v17: Rc<str> = concat5_1(v10, v15, v16.clone(), v16.clone(), v16.clone());
                        return US7::US7_ExtrasRefused(v17);
                    } else {
                        let v19: i32 = v0.wrapping_add(2i32);
                        let v20: Rc<str> = usize::try_from(v13).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                            US9::US9_FlagEstimate => {
                                (v20, v3, v4, v5, v6, v7)
                            }
                            _ => {
                                let (mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                    US9::US9_FlagLoad => {
                                        (v2, v20, v4, v5, v6, v7)
                                    }
                                    _ => {
                                        let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                            US9::US9_FlagAssignee => {
                                                (v2, v3, v20, v5, v6, v7)
                                            }
                                            _ => {
                                                let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                                    US9::US9_FlagDeps => {
                                                        (v2, v3, v4, v20, v6, v7)
                                                    }
                                                    _ => {
                                                        let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                                            US9::US9_FlagNote => {
                                                                let mut v21: Rc<str> = std::rc::Rc::<str>::from(v20.trim());
                                                                (v2, v3, v4, v5, v21, v7)
                                                            }
                                                            _ => {
                                                                let (mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v12 {
                                                                    US9::US9_FlagAs => {
                                                                        let v22: Rc<str> = std::rc::Rc::<str>::from(v20.trim());
                                                                        (v2, v3, v4, v5, v6, v22)
                                                                    }
                                                                    _ => unreachable!(),
                                                                };
                                                                (v23, v24, v25, v26, v27, v28)
                                                            }
                                                        };
                                                        (v29, v30, v31, v32, v33, v34)
                                                    }
                                                };
                                                (v35, v36, v37, v38, v39, v40)
                                            }
                                        };
                                        (v41, v42, v43, v44, v45, v46)
                                    }
                                };
                                (v47, v48, v49, v50, v51, v52)
                            }
                        };
                        (v0, v2, v3, v4, v5, v6, v7) = (v19, v53, v54, v55, v56, v57, v58);
                        continue;
                    }
                }
                US8::US8_FlagUnknown => {
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
                    let v65: Rc<str> = std::rc::Rc::<str>::from(v64.to_ascii_lowercase());
                    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v67: Rc<str> = concat5_1(v63.clone(), v65, v66.clone(), v66.clone(), v66.clone());
                    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
                    let v71: Rc<str> = std::rc::Rc::<str>::from(v70.to_ascii_lowercase());
                    let v72: Rc<str> = concat5_1(v63.clone(), v71, v66.clone(), v66.clone(), v66.clone());
                    let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
                    let v76: Rc<str> = std::rc::Rc::<str>::from(v75.to_ascii_lowercase());
                    let v77: Rc<str> = concat5_1(v63.clone(), v76, v66.clone(), v66.clone(), v66.clone());
                    let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Deps"); } LIT.with(|lit| lit.clone()) };
                    let v81: Rc<str> = std::rc::Rc::<str>::from(v80.to_ascii_lowercase());
                    let v82: Rc<str> = concat5_1(v63.clone(), v81, v66.clone(), v66.clone(), v66.clone());
                    let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note"); } LIT.with(|lit| lit.clone()) };
                    let v86: Rc<str> = std::rc::Rc::<str>::from(v85.to_ascii_lowercase());
                    let v87: Rc<str> = concat5_1(v63.clone(), v86, v66.clone(), v66.clone(), v66.clone());
                    let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                    let v91: Rc<str> = std::rc::Rc::<str>::from(v90.to_ascii_lowercase());
                    let v92: Rc<str> = concat5_1(v63, v91, v66.clone(), v66.clone(), v66.clone());
                    let v94: Rc<str> = string_concat(&["", &*v67, ", ", &*v72, ", ", &*v77, ", ", &*v82, ", ", &*v87, ", ", &*v92]);
                    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                    let v96: Rc<str> = concat5_1(v61, v10, v62, v94, v95);
                    return US7::US7_ExtrasRefused(v96);
                }
            }
        }
    }
}
fn extras_checked_5(mut v0: US7) -> US7 {
    match &v0 {
        US7::US7_ExtrasRead(v2, v3, v4, v5, v6, v7) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<str> = v3.clone();
            let v4: Rc<str> = v4.clone();
            let v5: Rc<str> = v5.clone();
            let v6: Rc<str> = v6.clone();
            let v7: Rc<str> = v7.clone();
            let mut v8: bool = &*v7 == "";
            let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v8 {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AUTHOR"); } LIT.with(|lit| lit.clone()) };
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_"); } LIT.with(|lit| lit.clone()) };
                let mut v11: bool = v9.starts_with(&*v10);
                let v14: bool = if v11 {
                    true
                } else {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RIG_"); } LIT.with(|lit| lit.clone()) };
                    let v13: bool = v9.starts_with(&*v12);
                    v13
                };
                let v28: Rc<str> = if v14 {
                    let mut v15: i32 = (v9.clone().len() as i32);
                    let v16: i32 = v15.wrapping_sub(4i32);
                    let v17: usize = ((4i32) as usize);
                    let v18: i32 = 4i32.wrapping_add(v16);
                    let v19: usize = ((v18) as usize);
                    let v20: Rc<str> = Rc::<str>::from(&(v9)[v17..v19]);
                    let mut v21: Rc<str> = string_concat(&["RIG_", &*v20]);
                    let v22: Rc<str> = std::env::var(&*v21).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                    let v23: bool = &*v22 != "";
                    if v23 {
                        v22
                    } else {
                        let v24: Rc<str> = string_concat(&["KIT_", &*v20]);
                        let v25: Rc<str> = std::env::var(&*v24).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                        v25
                    }
                } else {
                    let mut v27: Rc<str> = std::env::var(&*v9).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                    v27
                };
                let v29: Rc<str> = std::rc::Rc::<str>::from(v28.trim());
                (v2, v3, v4, v5, v6, v29)
            } else {
                (v2, v3, v4, v5, v6, v7)
            };
            let v36: bool = &*v34 == "";
            if v36 {
                if v8 {
                    US7::US7_ExtrasRead(v30, v31, v32, v33, v34, v35)
                } else {
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as names the author of --note; give --note too"); } LIT.with(|lit| lit.clone()) };
                    US7::US7_ExtrasRefused(v38)
                }
            } else {
                let v41: bool = &*v35 == "";
                if v41 {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a note needs its author: --as <author> (or set KIT_AUTHOR)"); } LIT.with(|lit| lit.clone()) };
                    US7::US7_ExtrasRefused(v42)
                } else {
                    US7::US7_ExtrasRead(v30, v31, v32, v33, v34, v35)
                }
            }
        }
        US7::US7_ExtrasRefused(v1) => {
            let v1: Rc<str> = v1.clone();
            v0
        }
    }
}
fn loop_6(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method7(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_8(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(6i32);
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
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_10(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method13(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method13(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method12(v16, v17)
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method14(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let mut v15: US12 = if v6 {
                let v7: US13 = US13::US13_HeadType;
                US12::US12_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US13 = US13::US13_HeadType;
                    US12::US12_SymbolHead(v11)
                } else {
                    US12::US12_OtherLine
                }
            };
            let mut v27: US12 = match &v15 {
                US12::US12_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US13 = US13::US13_HeadInl;
                        US12::US12_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let mut v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US13 = US13::US13_HeadInl;
                            US12::US12_SymbolHead(v22)
                        } else {
                            US12::US12_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US12 = match &v27 {
                US12::US12_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US13 = US13::US13_HeadUnion;
                        US12::US12_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let mut v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US13 = US13::US13_HeadUnion;
                            US12::US12_SymbolHead(v34)
                        } else {
                            US12::US12_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US12 = match &v39 {
                US12::US12_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US13 = US13::US13_HeadOpen;
                        US12::US12_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US13 = US13::US13_HeadOpen;
                            US12::US12_SymbolHead(v46)
                        } else {
                            US12::US12_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US12::US12_SymbolHead(v52) => {
                    let v52: US13 = v52;
                    let v60: Rc<str> = match &v52 {
                        US13::US13_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US13::US13_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US13::US13_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US13::US13_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US13::US13_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US13::US13_HeadOpen => {
                            let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US13::US13_HeadType => {
                            v5
                        }
                        US13::US13_HeadUnion => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                    };
                    let v69: i32 = (v2.clone().len() as i32);
                    let mut v70: i32 = v69.wrapping_sub(v61);
                    let v71: usize = ((v61) as usize);
                    let v72: i32 = v61.wrapping_add(v70);
                    let v73: usize = ((v72) as usize);
                    let v74: Rc<str> = Rc::<str>::from(&(v2)[v71..v73]);
                    let mut v75: Rc<str> = string_concat(&[&*v68, &*v74]);
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
fn method15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
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
fn extras_applied_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: &Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> US14 {
    let (mut v8, mut v9): (i32, Rc<str>) = agile_task_notes::rig_agile_groomed(&*v0, &*v1, &*v2, &*v3, &*v4, &*v6, &*v7);
    let v10: bool = v8 == 0i32;
    if v10 {
        US14::US14_AddedText(v9)
    } else {
        US14::US14_AddedRefused(v9)
    }
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method18(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let mut v15: US12 = if v6 {
                let v7: US13 = US13::US13_HeadType;
                US12::US12_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let mut v11: US13 = US13::US13_HeadType;
                    US12::US12_SymbolHead(v11)
                } else {
                    US12::US12_OtherLine
                }
            };
            let mut v27: US12 = match &v15 {
                US12::US12_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US13 = US13::US13_HeadInl;
                        US12::US12_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let mut v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US13 = US13::US13_HeadInl;
                            US12::US12_SymbolHead(v22)
                        } else {
                            US12::US12_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US12 = match &v27 {
                US12::US12_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US13 = US13::US13_HeadUnion;
                        US12::US12_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let mut v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US13 = US13::US13_HeadUnion;
                            US12::US12_SymbolHead(v34)
                        } else {
                            US12::US12_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US12 = match &v39 {
                US12::US12_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US13 = US13::US13_HeadOpen;
                        US12::US12_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US13 = US13::US13_HeadOpen;
                            US12::US12_SymbolHead(v46)
                        } else {
                            US12::US12_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US12::US12_KeywordHead(v52) => {
                    let v52: US13 = v52;
                    let v59: Rc<str> = match &v52 {
                        US13::US13_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US13::US13_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US13::US13_HeadType => {
                            v5
                        }
                        US13::US13_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v60: i32 = (v59.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US13::US13_HeadInl => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v61
                        }
                        US13::US13_HeadOpen => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                        US13::US13_HeadType => {
                            let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US13::US13_HeadUnion => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                    };
                    let v69: i32 = (v2.clone().len() as i32);
                    let mut v70: i32 = v69.wrapping_sub(v60);
                    let v71: usize = ((v60) as usize);
                    let v72: i32 = v60.wrapping_add(v70);
                    let v73: usize = ((v72) as usize);
                    let v74: Rc<str> = Rc::<str>::from(&(v2)[v71..v73]);
                    let mut v75: Rc<str> = string_concat(&[&*v68, &*v74]);
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
fn renamed_within_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: bool = std::fs::rename(&*v1, &*v0).is_ok();
        if v3 {
            return true;
        } else {
            let v4: bool = v2 <= 1i32;
            if v4 {
                return false;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(50u64));
                let v5: i32 = v2.wrapping_sub(1i32);
                v2 = v5;
                continue;
            }
        }
    }
}
fn replace_records_unchanged_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US15 {
    let v3: bool = rig_agile_records::rig_agile_records_unchanged(&*v0, &*v1);
    if v3 {
        let v4: Rc<str> = string_concat(&[&*v0, ".pending"]);
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v6));
        let mut v8: Rc<str> = loop_6(v7);
        let mut v9: Option<usize> = v8.rfind(&*v6);
        let v10: Option<usize> = method7(v9);
        let mut v11: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v12: Option<i32> = v10.map(|x| v11(x));
        let v14: i32 = v12.unwrap_or(-1i32);
        let mut v15: bool = v14 < 0i32;
        let v19: bool = if v15 {
            true
        } else {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v17: bool = v8.ends_with(&*v16);
            let v18: bool = v17 == false;
            v18
        };
        let v40: US11 = if v19 {
            US11::US11_OtherFile
        } else {
            let mut v21: usize = ((0i32) as usize);
            let v22: usize = ((v14) as usize);
            let v23: Rc<str> = Rc::<str>::from(&(v8)[v21..v22]);
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v25: bool = v23.ends_with(&*v24);
            if v25 {
                let v26: Rc<str> = loop_8(v23);
                US11::US11_StateFile(v26)
            } else {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v29: bool = v23.ends_with(&*v28);
                if v29 {
                    let v30: Rc<str> = loop_9(v23);
                    US11::US11_HistoryFile(v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let mut v33: bool = v23.ends_with(&*v32);
                    if v33 {
                        let v34: Rc<str> = loop_10(v23);
                        US11::US11_ArchiveFile(v34)
                    } else {
                        US11::US11_OtherFile
                    }
                }
            }
        };
        let v63: US16 = match v40 {
            US11::US11_ArchiveFile(v53) => {
                let v53: Rc<str> = v53;
                let v54: Rc<str> = string_concat(&[&*v53, "/state/spiral.toml"]);
                let v55: bool = std::path::Path::new(&*v54).is_file();
                if v55 {
                    US16::US16_SymbolsRoot
                } else {
                    US16::US16_WordsRoot
                }
            }
            US11::US11_HistoryFile(v47) => {
                let v47: Rc<str> = v47;
                let v48: Rc<str> = string_concat(&[&*v47, "/state/spiral.toml"]);
                let v49: bool = std::path::Path::new(&*v48).is_file();
                if v49 {
                    US16::US16_SymbolsRoot
                } else {
                    US16::US16_WordsRoot
                }
            }
            US11::US11_OtherFile => {
                US16::US16_WordsRoot
            }
            US11::US11_StateFile(v41) => {
                let v41: Rc<str> = v41;
                let v42: Rc<str> = string_concat(&[&*v41, "/state/spiral.toml"]);
                let v43: bool = std::path::Path::new(&*v42).is_file();
                if v43 {
                    US16::US16_SymbolsRoot
                } else {
                    US16::US16_WordsRoot
                }
            }
        };
        let mut v75: Rc<str> = match v63 {
            US16::US16_SymbolsRoot => {
                let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v65: Rc<UH0> = split_list_11(v64.clone(), v2.clone());
                let v66: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v67: Rc<UH0> = method18(v65, v66);
                let v68: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v69: i32 = 0i32;
                let mut v70: i32 = method15(v68.clone(), v67, v69);
                let v71: Rc<Vec<Rc<str>>> = Rc::new(v68.borrow().clone());
                let v72: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v71).as_ref().clone()));
                let v73: Rc<str> = Rc::<str>::from((*v72.borrow()).join(&*v64));
                v73
            }
            US16::US16_WordsRoot => {
                v2.clone()
            }
        };
        let v76: bool = std::fs::write(&*v4, &*v75).is_ok();
        if v76 {
            let v77: i32 = 40i32;
            let v78: bool = renamed_within_19(v0.clone(), v4.clone(), v77);
            if v78 {
                let v79: Rc<str> = Rc::<str>::from(v0.replace(&*v5, &*v6));
                let mut v80: Rc<str> = loop_6(v79);
                let v81: Option<usize> = v80.rfind(&*v6);
                let v82: Option<usize> = method7(v81);
                let mut v83: Option<i32> = v82.map(|x| v11(x));
                let mut v85: i32 = v83.unwrap_or(-1i32);
                let v86: bool = v85 < 0i32;
                let mut v90: bool = if v86 {
                    true
                } else {
                    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v88: bool = v80.ends_with(&*v87);
                    let v89: bool = v88 == false;
                    v89
                };
                let v111: US11 = if v90 {
                    US11::US11_OtherFile
                } else {
                    let v92: usize = ((0i32) as usize);
                    let v93: usize = ((v85) as usize);
                    let v94: Rc<str> = Rc::<str>::from(&(v80)[v92..v93]);
                    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v96: bool = v94.ends_with(&*v95);
                    if v96 {
                        let v97: Rc<str> = loop_8(v94);
                        US11::US11_StateFile(v97)
                    } else {
                        let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v100: bool = v94.ends_with(&*v99);
                        if v100 {
                            let v101: Rc<str> = loop_9(v94);
                            US11::US11_HistoryFile(v101)
                        } else {
                            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v104: bool = v94.ends_with(&*v103);
                            if v104 {
                                let v105: Rc<str> = loop_10(v94);
                                US11::US11_ArchiveFile(v105)
                            } else {
                                US11::US11_OtherFile
                            }
                        }
                    }
                };
                let v134: US16 = match v111 {
                    US11::US11_ArchiveFile(v124) => {
                        let v124: Rc<str> = v124;
                        let v125: Rc<str> = string_concat(&[&*v124, "/state/spiral.toml"]);
                        let v126: bool = std::path::Path::new(&*v125).is_file();
                        if v126 {
                            US16::US16_SymbolsRoot
                        } else {
                            US16::US16_WordsRoot
                        }
                    }
                    US11::US11_HistoryFile(v118) => {
                        let v118: Rc<str> = v118;
                        let v119: Rc<str> = string_concat(&[&*v118, "/state/spiral.toml"]);
                        let v120: bool = std::path::Path::new(&*v119).is_file();
                        if v120 {
                            US16::US16_SymbolsRoot
                        } else {
                            US16::US16_WordsRoot
                        }
                    }
                    US11::US11_OtherFile => {
                        US16::US16_WordsRoot
                    }
                    US11::US11_StateFile(v112) => {
                        let v112: Rc<str> = v112;
                        let v113: Rc<str> = string_concat(&[&*v112, "/state/spiral.toml"]);
                        let v114: bool = std::path::Path::new(&*v113).is_file();
                        if v114 {
                            US16::US16_SymbolsRoot
                        } else {
                            US16::US16_WordsRoot
                        }
                    }
                };
                let v146: Rc<str> = match v134 {
                    US16::US16_SymbolsRoot => {
                        let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v136: Rc<UH0> = split_list_11(v135.clone(), v2);
                        let v137: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v138: Rc<UH0> = method18(v136, v137);
                        let v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v140: i32 = 0i32;
                        let v141: i32 = method15(v139.clone(), v138, v140);
                        let v142: Rc<Vec<Rc<str>>> = Rc::new(v139.borrow().clone());
                        let v143: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v142).as_ref().clone()));
                        let v144: Rc<str> = Rc::<str>::from((*v143.borrow()).join(&*v135));
                        v144
                    }
                    US16::US16_WordsRoot => {
                        v2
                    }
                };
                let v147: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v146);
                if v147 {
                    US15::US15_Replaced
                } else {
                    US15::US15_ReadbackMismatch
                }
            } else {
                let v151: bool = std::fs::remove_file(&*v4).is_ok() || !std::path::Path::new(&*v4).exists();
                if v151 {
                    US15::US15_RenameFailed
                } else {
                    US15::US15_RenameFailedTemporaryKept
                }
            }
        } else {
            US15::US15_TemporaryWriteFailed
        }
    } else {
        US15::US15_ChangedSinceRead
    }
}
fn agile_add_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let v1: i32 = 0i32;
    let mut v2: i32 = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v1).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
    let v3: bool = 48i32 <= v2;
    let v5: bool = if v3 {
        let v4: bool = v2 <= 57i32;
        v4
    } else {
        false
    };
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id must start with a letter, so its inl name is a valid identifier"); } LIT.with(|lit| lit.clone()) };
        let v7: i32 = 2i32;
        let mut v8: Rc<str> = usize::try_from(v7).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = concat5_1(v9, v8, v10, v6, v11);
        let v13: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v12 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v13
    } else {
        let v14: i32 = 2i32;
        let mut v15: i32 = usize::try_from(v14).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
        let v16: bool = 0 < v15;
        let v25: US0 = if v16 {
            let v17: i32 = 0i32;
            let v18: bool = task_id_bytes_allowed_2(v17, v15);
            if v18 {
                let v19: i32 = 2i32;
                let v20: Rc<str> = usize::try_from(v19).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                US0::US0_Valid(v20)
            } else {
                US0::US0_Invalid
            }
        } else {
            US0::US0_Invalid
        };
        let v26: i32 = 3i32;
        let mut v27: Rc<str> = usize::try_from(v26).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v28: bool = &*v27 == "Epic";
        let v32: US1 = if v28 {
            let v29: US2 = US2::US2_AgileEpic;
            US1::US1_Valid(v29)
        } else {
            US1::US1_Invalid
        };
        let mut v33: bool = &*v27 == "Feature";
        let v36: US1 = if v33 {
            let v34: US2 = US2::US2_AgileFeature;
            US1::US1_Valid(v34)
        } else {
            v32
        };
        let v37: bool = &*v27 == "Story";
        let v40: US1 = if v37 {
            let v38: US2 = US2::US2_AgileStory;
            US1::US1_Valid(v38)
        } else {
            v36
        };
        let v41: bool = &*v27 == "TaskKind";
        let v44: US1 = if v41 {
            let v42: US2 = US2::US2_AgileTaskKind;
            US1::US1_Valid(v42)
        } else {
            v40
        };
        let v45: i32 = 4i32;
        let v46: Rc<str> = usize::try_from(v45).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v47: bool = &*v46 == "P0";
        let v51: US3 = if v47 {
            let v48: US4 = US4::US4_AgileP0;
            US3::US3_Valid(v48)
        } else {
            US3::US3_Invalid
        };
        let v52: bool = &*v46 == "P1";
        let v55: US3 = if v52 {
            let v53: US4 = US4::US4_AgileP1;
            US3::US3_Valid(v53)
        } else {
            v51
        };
        let v56: bool = &*v46 == "P2";
        let v59: US3 = if v56 {
            let v57: US4 = US4::US4_AgileP2;
            US3::US3_Valid(v57)
        } else {
            v55
        };
        let v60: bool = &*v46 == "P3";
        let v63: US3 = if v60 {
            let v61: US4 = US4::US4_AgileP3;
            US3::US3_Valid(v61)
        } else {
            v59
        };
        let mut v64: bool = &*v46 == "P4";
        let v67: US3 = if v64 {
            let v65: US4 = US4::US4_AgileP4;
            US3::US3_Valid(v65)
        } else {
            v63
        };
        let v68: bool = &*v46 == "P5";
        let v71: US3 = if v68 {
            let v69: US4 = US4::US4_AgileP5;
            US3::US3_Valid(v69)
        } else {
            v67
        };
        let v72: i32 = 5i32;
        let v73: Rc<str> = usize::try_from(v72).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v74: bool = &*v73 == "D0";
        let v78: US5 = if v74 {
            let mut v75: US6 = US6::US6_AgileD0;
            US5::US5_Valid(v75)
        } else {
            US5::US5_Invalid
        };
        let v79: bool = &*v73 == "D1";
        let v82: US5 = if v79 {
            let mut v80: US6 = US6::US6_AgileD1;
            US5::US5_Valid(v80)
        } else {
            v78
        };
        let mut v83: bool = &*v73 == "D2";
        let v86: US5 = if v83 {
            let v84: US6 = US6::US6_AgileD2;
            US5::US5_Valid(v84)
        } else {
            v82
        };
        let v87: bool = &*v73 == "D3";
        let mut v90: US5 = if v87 {
            let v88: US6 = US6::US6_AgileD3;
            US5::US5_Valid(v88)
        } else {
            v86
        };
        let v91: bool = &*v73 == "D4";
        let v94: US5 = if v91 {
            let v92: US6 = US6::US6_AgileD4;
            US5::US5_Valid(v92)
        } else {
            v90
        };
        let v95: bool = &*v73 == "D5";
        let v98: US5 = if v95 {
            let v96: US6 = US6::US6_AgileD5;
            US5::US5_Valid(v96)
        } else {
            v94
        };
        let v99: bool = &*v73 == "D6";
        let v102: US5 = if v99 {
            let v100: US6 = US6::US6_AgileD6;
            US5::US5_Valid(v100)
        } else {
            v98
        };
        let v103: bool = &*v73 == "D7";
        let v106: US5 = if v103 {
            let v104: US6 = US6::US6_AgileD7;
            US5::US5_Valid(v104)
        } else {
            v102
        };
        let v107: bool = &*v73 == "D8";
        let v110: US5 = if v107 {
            let v108: US6 = US6::US6_AgileD8;
            US5::US5_Valid(v108)
        } else {
            v106
        };
        match v25 {
            US0::US0_Invalid => {
                let v386: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id must be uppercase letters, digits and dashes"); } LIT.with(|lit| lit.clone()) };
                let v387: i32 = 2i32;
                let v388: Rc<str> = usize::try_from(v387).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v389: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                let v390: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v391: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v392: Rc<str> = concat5_1(v389, v388, v390, v386, v391);
                let v393: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v392 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v393
            }
            US0::US0_Valid(v111) => {
                let v111: Rc<str> = v111;
                match v44 {
                    US1::US1_Invalid => {
                        let v376: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind must be Epic, Feature, Story or TaskKind"); } LIT.with(|lit| lit.clone()) };
                        let v377: i32 = 2i32;
                        let v378: Rc<str> = usize::try_from(v377).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let v379: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                        let v380: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let v381: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v382: Rc<str> = concat5_1(v379, v378, v380, v376, v381);
                        let v383: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v382 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v383
                    }
                    US1::US1_Valid(v112) => {
                        let v112: US2 = v112;
                        match v71 {
                            US3::US3_Invalid => {
                                let v366: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority must be P0..P5"); } LIT.with(|lit| lit.clone()) };
                                let v367: i32 = 2i32;
                                let v368: Rc<str> = usize::try_from(v367).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let v369: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                let v370: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                let v371: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v372: Rc<str> = concat5_1(v369, v368, v370, v366, v371);
                                let v373: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v372 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v373
                            }
                            US3::US3_Valid(v113) => {
                                let v113: US4 = v113;
                                match v110 {
                                    US5::US5_Invalid => {
                                        let v356: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty must be D0..D8"); } LIT.with(|lit| lit.clone()) };
                                        let v357: i32 = 2i32;
                                        let v358: Rc<str> = usize::try_from(v357).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let v359: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                        let v360: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                        let v361: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v362: Rc<str> = concat5_1(v359, v358, v360, v356, v361);
                                        let v363: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v362 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                        v363
                                    }
                                    US5::US5_Valid(v114) => {
                                        let v114: US6 = v114;
                                        let v115: i32 = 8i32;
                                        let v116: i32 = std::env::args().skip(2i32 as usize).count() as i32;
                                        let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v118: US7 = extras_from_3(v115, v116, v117.clone(), v117.clone(), v117.clone(), v117.clone(), v117.clone(), v117.clone());
                                        let v119: US7 = extras_checked_5(v118);
                                        match v119 {
                                            US7::US7_ExtrasRead(v120, v121, v122, v123, v124, v125) => {
                                                let v120: Rc<str> = v120;
                                                let v121: Rc<str> = v121;
                                                let v122: Rc<str> = v122;
                                                let v123: Rc<str> = v123;
                                                let v124: Rc<str> = v124;
                                                let v125: Rc<str> = v125;
                                                let v126: i32 = 1i32;
                                                let v127: Rc<str> = usize::try_from(v126).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v128: i32 = 6i32;
                                                let v129: Rc<str> = usize::try_from(v128).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v130: i32 = 7i32;
                                                let v131: Rc<str> = usize::try_from(v130).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v132: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                                                let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                                                let v134: Rc<str> = concat5_1(v127, v133, v117.clone(), v117.clone(), v117.clone());
                                                let v135: bool = std::path::Path::new(&*v134).is_file();
                                                let v138: US10 = if v135 {
                                                    US10::US10_FileReadable(v134.clone())
                                                } else {
                                                    US10::US10_FileMissing(v134.clone())
                                                };
                                                match v138 {
                                                    US10::US10_FileMissing(v139) => {
                                                        let v139: Rc<str> = v139;
                                                        let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                                                        let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                        let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                        let v143: Rc<str> = concat5_1(v141, v111, v142, v140, v117);
                                                        let v144: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v143 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                        v144
                                                    }
                                                    US10::US10_FileReadable(v145) => {
                                                        let v145: Rc<str> = v145;
                                                        let v146: Rc<str> = std::fs::read_to_string(&*v145).unwrap_or_default().into();
                                                        let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                                        let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                                                        let v149: Rc<str> = Rc::<str>::from(v145.replace(&*v147, &*v148));
                                                        let mut v150: Rc<str> = loop_6(v149);
                                                        let v151: Option<usize> = v150.rfind(&*v148);
                                                        let v152: Option<usize> = method7(v151);
                                                        let mut v153: Rc<dyn Fn((usize)) -> i32> = closure1();
                                                        let mut v154: Option<i32> = v152.map(|x| v153(x));
                                                        let v156: i32 = v154.unwrap_or(-1i32);
                                                        let v157: bool = v156 < 0i32;
                                                        let v161: bool = if v157 {
                                                            true
                                                        } else {
                                                            let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                            let v159: bool = v150.ends_with(&*v158);
                                                            let v160: bool = v159 == false;
                                                            v160
                                                        };
                                                        let v182: US11 = if v161 {
                                                            US11::US11_OtherFile
                                                        } else {
                                                            let v163: usize = ((0i32) as usize);
                                                            let v164: usize = ((v156) as usize);
                                                            let v165: Rc<str> = Rc::<str>::from(&(v150)[v163..v164]);
                                                            let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                            let v167: bool = v165.ends_with(&*v166);
                                                            if v167 {
                                                                let v168: Rc<str> = loop_8(v165);
                                                                US11::US11_StateFile(v168)
                                                            } else {
                                                                let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                                let v171: bool = v165.ends_with(&*v170);
                                                                if v171 {
                                                                    let v172: Rc<str> = loop_9(v165);
                                                                    US11::US11_HistoryFile(v172)
                                                                } else {
                                                                    let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                                    let v175: bool = v165.ends_with(&*v174);
                                                                    if v175 {
                                                                        let v176: Rc<str> = loop_10(v165);
                                                                        US11::US11_ArchiveFile(v176)
                                                                    } else {
                                                                        US11::US11_OtherFile
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        let v193: Rc<str> = match v182 {
                                                            US11::US11_OtherFile => {
                                                                v146
                                                            }
                                                            _ => {
                                                                let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                                let v184: Rc<UH0> = split_list_11(v183.clone(), v146);
                                                                let v185: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                                let v186: Rc<UH0> = method14(v184, v185);
                                                                let v187: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                                let v188: i32 = 0i32;
                                                                let v189: i32 = method15(v187.clone(), v186, v188);
                                                                let v190: Rc<Vec<Rc<str>>> = Rc::new(v187.borrow().clone());
                                                                let v191: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v190).as_ref().clone()));
                                                                let v192: Rc<str> = Rc::<str>::from((*v191.borrow()).join(&*v183));
                                                                v192
                                                            }
                                                        };
                                                        let mut v194: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v193);
                                                        let v195: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                                                        let v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                                                        let v197: Rc<str> = concat5_1(v195, v111.clone(), v196, v117.clone(), v117.clone());
                                                        let v198: bool = v194.contains(&*v197);
                                                        if v198 {
                                                            let v199: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task with this id already exists"); } LIT.with(|lit| lit.clone()) };
                                                            let v200: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                            let v201: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                            let v202: Rc<str> = concat5_1(v200, v111, v201, v199, v117);
                                                            let v203: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v202 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                            v203
                                                        } else {
                                                            let v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                            let v205: Rc<str> = std::rc::Rc::<str>::from(v194.trim_end_matches(&*v204));
                                                            let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                                                            let v207: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                                            let v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v209: Rc<str> = Rc::<str>::from(v111.replace(&*v207, &*v208));
                                                            let v210: Rc<str> = std::rc::Rc::<str>::from(v209.to_ascii_lowercase());
                                                            let v211: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Task ("); } LIT.with(|lit| lit.clone()) };
                                                            let v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                            let v213: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                                            let v214: Rc<str> = Rc::<str>::from(v111.replace(&*v147, &*v213));
                                                            let v215: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                                            let v216: Rc<str> = Rc::<str>::from(v214.replace(&*v212, &*v215));
                                                            let v217: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                                            let v218: Rc<str> = Rc::<str>::from(v216.replace(&*v204, &*v217));
                                                            let v219: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                                            let v220: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                                            let v221: Rc<str> = Rc::<str>::from(v218.replace(&*v219, &*v220));
                                                            let v222: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                                            let v223: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                                            let v224: Rc<str> = Rc::<str>::from(v221.replace(&*v222, &*v223));
                                                            let v225: Rc<str> = concat5_1(v212.clone(), v224, v212.clone(), v117.clone(), v117.clone());
                                                            let v226: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                                                            let v227: Rc<str> = concat5_1(v206, v210, v211, v225, v226.clone());
                                                            let v228: Rc<str> = Rc::<str>::from(v129.replace(&*v147, &*v213));
                                                            let v229: Rc<str> = Rc::<str>::from(v228.replace(&*v212, &*v215));
                                                            let v230: Rc<str> = Rc::<str>::from(v229.replace(&*v204, &*v217));
                                                            let v231: Rc<str> = Rc::<str>::from(v230.replace(&*v219, &*v220));
                                                            let v232: Rc<str> = Rc::<str>::from(v231.replace(&*v222, &*v223));
                                                            let v233: Rc<str> = concat5_1(v212.clone(), v232, v212.clone(), v117.clone(), v117.clone());
                                                            let v238: Rc<str> = match v112 {
                                                                US2::US2_AgileEpic => {
                                                                    let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
                                                                    v234
                                                                }
                                                                US2::US2_AgileFeature => {
                                                                    let v235: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
                                                                    v235
                                                                }
                                                                US2::US2_AgileStory => {
                                                                    let v236: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
                                                                    v236
                                                                }
                                                                US2::US2_AgileTaskKind => {
                                                                    let v237: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                                                                    v237
                                                                }
                                                            };
                                                            let v245: Rc<str> = match v113 {
                                                                US4::US4_AgileP0 => {
                                                                    let v239: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
                                                                    v239
                                                                }
                                                                US4::US4_AgileP1 => {
                                                                    let v240: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                                                                    v240
                                                                }
                                                                US4::US4_AgileP2 => {
                                                                    let v241: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
                                                                    v241
                                                                }
                                                                US4::US4_AgileP3 => {
                                                                    let v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
                                                                    v242
                                                                }
                                                                US4::US4_AgileP4 => {
                                                                    let v243: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
                                                                    v243
                                                                }
                                                                US4::US4_AgileP5 => {
                                                                    let v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
                                                                    v244
                                                                }
                                                            };
                                                            let v246: Rc<str> = concat5_1(v233, v226.clone(), v238, v226.clone(), v245);
                                                            let v256: Rc<str> = match v114 {
                                                                US6::US6_AgileD0 => {
                                                                    let v247: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
                                                                    v247
                                                                }
                                                                US6::US6_AgileD1 => {
                                                                    let v248: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
                                                                    v248
                                                                }
                                                                US6::US6_AgileD2 => {
                                                                    let v249: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                                                                    v249
                                                                }
                                                                US6::US6_AgileD3 => {
                                                                    let v250: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
                                                                    v250
                                                                }
                                                                US6::US6_AgileD4 => {
                                                                    let v251: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
                                                                    v251
                                                                }
                                                                US6::US6_AgileD5 => {
                                                                    let v252: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
                                                                    v252
                                                                }
                                                                US6::US6_AgileD6 => {
                                                                    let v253: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
                                                                    v253
                                                                }
                                                                US6::US6_AgileD7 => {
                                                                    let v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
                                                                    v254
                                                                }
                                                                US6::US6_AgileD8 => {
                                                                    let v255: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
                                                                    v255
                                                                }
                                                            };
                                                            let v257: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", 0u32, Planned, "); } LIT.with(|lit| lit.clone()) };
                                                            let v258: Rc<str> = Rc::<str>::from(v123.replace(&*v147, &*v213));
                                                            let v259: Rc<str> = Rc::<str>::from(v258.replace(&*v212, &*v215));
                                                            let v260: Rc<str> = Rc::<str>::from(v259.replace(&*v204, &*v217));
                                                            let v261: Rc<str> = Rc::<str>::from(v260.replace(&*v219, &*v220));
                                                            let v262: Rc<str> = Rc::<str>::from(v261.replace(&*v222, &*v223));
                                                            let v263: Rc<str> = concat5_1(v212.clone(), v262, v212.clone(), v117.clone(), v117.clone());
                                                            let v264: Rc<str> = Rc::<str>::from(v131.replace(&*v147, &*v213));
                                                            let v265: Rc<str> = Rc::<str>::from(v264.replace(&*v212, &*v215));
                                                            let v266: Rc<str> = Rc::<str>::from(v265.replace(&*v204, &*v217));
                                                            let v267: Rc<str> = Rc::<str>::from(v266.replace(&*v219, &*v220));
                                                            let v268: Rc<str> = Rc::<str>::from(v267.replace(&*v222, &*v223));
                                                            let v269: Rc<str> = concat5_1(v212.clone(), v268, v212.clone(), v117.clone(), v117.clone());
                                                            let v270: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", \"current\", \"current\")"); } LIT.with(|lit| lit.clone()) };
                                                            let v271: Rc<str> = concat5_1(v226.clone(), v269, v270, v117.clone(), v117.clone());
                                                            let v272: Rc<str> = concat5_1(v226, v256, v257, v263, v271);
                                                            let v273: Rc<str> = concat5_1(v227, v246, v272, v117.clone(), v117.clone());
                                                            let v274: Rc<str> = concat5_1(v205, v204.clone(), v273, v204.clone(), v117.clone());
                                                            let v275: US14 = extras_applied_16(v274, v111.clone(), v120.clone(), v121.clone(), v122.clone(), &v123, v124.clone(), v125);
                                                            match v275 {
                                                                US14::US14_AddedRefused(v276) => {
                                                                    let v276: Rc<str> = v276;
                                                                    let v277: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                                    let v278: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                                    let v279: Rc<str> = concat5_1(v277, v111, v278, v276, v117);
                                                                    let v280: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v279 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                    v280
                                                                }
                                                                US14::US14_AddedText(v281) => {
                                                                    let v281: Rc<str> = v281;
                                                                    let v282: US15 = replace_records_unchanged_17(v134, v194.clone(), v281.clone());
                                                                    match &v282 {
                                                                        US15::US15_Replaced => {
                                                                            let mut v283: Rc<str> = Rc::<str>::from(v111.replace(&*v207, &*v208));
                                                                            let v284: Rc<str> = std::rc::Rc::<str>::from(v283.to_ascii_lowercase());
                                                                            let v285: u64 = v194.len() as u64;
                                                                            let v286: u64 = v281.len() as u64;
                                                                            let v287: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v132);
                                                                            let v288: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig agile add ok id="); } LIT.with(|lit| lit.clone()) };
                                                                            let v289: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" binding="); } LIT.with(|lit| lit.clone()) };
                                                                            let v290: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" status=Planned progress=0"); } LIT.with(|lit| lit.clone()) };
                                                                            let v291: bool = &*v123 == "";
                                                                            let v294: Rc<str> = if v291 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v292: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deps="); } LIT.with(|lit| lit.clone()) };
                                                                                concat5_1(v292, v123, v117.clone(), v117.clone(), v117.clone())
                                                                            };
                                                                            let v295: bool = &*v120 == "";
                                                                            let v297: Rc<str> = if v295 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v296: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" estimate"); } LIT.with(|lit| lit.clone()) };
                                                                                v296
                                                                            };
                                                                            let v298: bool = &*v121 == "";
                                                                            let v300: Rc<str> = if v298 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v299: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" load"); } LIT.with(|lit| lit.clone()) };
                                                                                v299
                                                                            };
                                                                            let v301: bool = &*v122 == "";
                                                                            let v303: Rc<str> = if v301 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v302: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee"); } LIT.with(|lit| lit.clone()) };
                                                                                v302
                                                                            };
                                                                            let v304: Rc<str> = concat5_1(v297, v300, v303, v117.clone(), v117.clone());
                                                                            let v305: bool = &*v304 == "";
                                                                            let v312: Rc<str> = if v305 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v306: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" planned="); } LIT.with(|lit| lit.clone()) };
                                                                                let v307: Rc<str> = std::rc::Rc::<str>::from(v304.trim());
                                                                                let v308: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                                                                let v309: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                                                                let v310: Rc<str> = Rc::<str>::from(v307.replace(&*v308, &*v309));
                                                                                concat5_1(v306, v310, v117.clone(), v117.clone(), v117.clone())
                                                                            };
                                                                            let v313: bool = &*v124 == "";
                                                                            let v315: Rc<str> = if v313 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v314: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" notes=1"); } LIT.with(|lit| lit.clone()) };
                                                                                v314
                                                                            };
                                                                            let v316: Rc<str> = concat5_1(v294, v312, v315, v117.clone(), v117.clone());
                                                                            let v317: Rc<str> = concat5_1(v290, v316, v117.clone(), v117.clone(), v117.clone());
                                                                            let v318: Rc<str> = concat5_1(v288, v111, v289, v284, v317);
                                                                            let v319: Rc<str> = std::rc::Rc::<str>::from((v285).to_string());
                                                                            let v322: Rc<str> = std::rc::Rc::<str>::from((v286).to_string());
                                                                            let v325: Rc<str> = std::rc::Rc::<str>::from((v287).to_string());
                                                                            let v326: Rc<str> = string_concat(&[" bytes_before=", &*v319, " bytes_after=", &*v322, " elapsed_ms=", &*v325]);
                                                                            let v327: Rc<str> = concat5_1(v318, v326, v117.clone(), v117.clone(), v117.clone());
                                                                            let v328: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v327.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                                                            v328
                                                                        }
                                                                        _ => {
                                                                            let v338: Rc<str> = match v282 {
                                                                                US15::US15_ChangedSinceRead => {
                                                                                    let v333: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                                                                    v333
                                                                                }
                                                                                US15::US15_ReadbackMismatch => {
                                                                                    let v332: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                                    v332
                                                                                }
                                                                                US15::US15_RenameFailed => {
                                                                                    let v330: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                                    v330
                                                                                }
                                                                                US15::US15_RenameFailedTemporaryKept => {
                                                                                    let v331: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                                                                    v331
                                                                                }
                                                                                US15::US15_TemporaryWriteFailed => {
                                                                                    let v329: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                                    v329
                                                                                }
                                                                                _ => unreachable!(),
                                                                            };
                                                                            let v339: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                                            let v340: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                                            let v341: Rc<str> = concat5_1(v339, v111, v340, v338, v117);
                                                                            let v342: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v341 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                            v342
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            US7::US7_ExtrasRefused(v349) => {
                                                let v349: Rc<str> = v349;
                                                let v350: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                let v351: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                let v352: Rc<str> = concat5_1(v350, v111, v351, v349, v117);
                                                let v353: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v352 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                v353
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
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_add_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_add_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
