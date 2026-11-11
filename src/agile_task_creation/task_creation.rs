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
                                                                let mut v21: Rc<str> = Rc::<str>::from((v20).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                                                                (v2, v3, v4, v5, v21, v7)
                                                            }
                                                            _ => {
                                                                let (mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v12 {
                                                                    US9::US9_FlagAs => {
                                                                        let v22: Rc<str> = Rc::<str>::from((v20).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
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
            let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v8 {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RIG_AUTHOR"); } LIT.with(|lit| lit.clone()) };
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AUTHOR"); } LIT.with(|lit| lit.clone()) };
                let mut v11: Rc<str> = std::env::var(&*v9).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                let mut v12: bool = &*v11 != "";
                let v14: Rc<str> = if v12 {
                    v11
                } else {
                    let v13: Rc<str> = std::env::var(&*v10).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                    v13
                };
                let mut v15: Rc<str> = Rc::<str>::from((v14).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                (v2, v3, v4, v5, v6, v15)
            } else {
                (v2, v3, v4, v5, v6, v7)
            };
            let v22: bool = &*v20 == "";
            if v22 {
                if v8 {
                    US7::US7_ExtrasRead(v16, v17, v18, v19, v20, v21)
                } else {
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as names the author of --note; give --note too"); } LIT.with(|lit| lit.clone()) };
                    US7::US7_ExtrasRefused(v24)
                }
            } else {
                let mut v27: bool = &*v21 == "";
                if v27 {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a note needs its author: --as <author> (or set KIT_AUTHOR)"); } LIT.with(|lit| lit.clone()) };
                    US7::US7_ExtrasRefused(v28)
                } else {
                    US7::US7_ExtrasRead(v16, v17, v18, v19, v20, v21)
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
    let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: Rc<str> = rig_agile_id_prefix::rig_agile_new_id_refusal(&v1);
    let v3: i32 = 2i32;
    let v4: i32 = 0i32;
    let v5: i32 = usize::try_from(v3).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v4).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
    let v6: bool = 48i32 <= v5;
    let mut v8: bool = if v6 {
        let v7: bool = v5 <= 57i32;
        v7
    } else {
        false
    };
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id must start with a letter, so its inl name is a valid identifier"); } LIT.with(|lit| lit.clone()) };
        let v10: i32 = 2i32;
        let mut v11: Rc<str> = usize::try_from(v10).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = concat5_1(v12, v11, v13, v9, v14);
        let v16: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v15 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v16
    } else {
        let v17: bool = &*v2 != "";
        if v17 {
            let v18: i32 = 2i32;
            let v19: Rc<str> = usize::try_from(v18).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = concat5_1(v20, v19, v21, v2, v22);
            let v24: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v23 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v24
        } else {
            let v25: i32 = 2i32;
            let v26: i32 = usize::try_from(v25).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
            let mut v27: bool = 0 < v26;
            let v36: US0 = if v27 {
                let v28: i32 = 0i32;
                let v29: bool = task_id_bytes_allowed_2(v28, v26);
                if v29 {
                    let v30: i32 = 2i32;
                    let v31: Rc<str> = usize::try_from(v30).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    US0::US0_Valid(v31)
                } else {
                    US0::US0_Invalid
                }
            } else {
                US0::US0_Invalid
            };
            let v37: i32 = 3i32;
            let v38: Rc<str> = usize::try_from(v37).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v39: bool = &*v38 == "Epic";
            let v43: US1 = if v39 {
                let v40: US2 = US2::US2_AgileEpic;
                US1::US1_Valid(v40)
            } else {
                US1::US1_Invalid
            };
            let v44: bool = &*v38 == "Feature";
            let v47: US1 = if v44 {
                let v45: US2 = US2::US2_AgileFeature;
                US1::US1_Valid(v45)
            } else {
                v43
            };
            let v48: bool = &*v38 == "Story";
            let v51: US1 = if v48 {
                let v49: US2 = US2::US2_AgileStory;
                US1::US1_Valid(v49)
            } else {
                v47
            };
            let v52: bool = &*v38 == "TaskKind";
            let v55: US1 = if v52 {
                let v53: US2 = US2::US2_AgileTaskKind;
                US1::US1_Valid(v53)
            } else {
                v51
            };
            let v56: i32 = 4i32;
            let v57: Rc<str> = usize::try_from(v56).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v58: bool = &*v57 == "P0";
            let v62: US3 = if v58 {
                let v59: US4 = US4::US4_AgileP0;
                US3::US3_Valid(v59)
            } else {
                US3::US3_Invalid
            };
            let v63: bool = &*v57 == "P1";
            let v66: US3 = if v63 {
                let mut v64: US4 = US4::US4_AgileP1;
                US3::US3_Valid(v64)
            } else {
                v62
            };
            let v67: bool = &*v57 == "P2";
            let mut v70: US3 = if v67 {
                let v68: US4 = US4::US4_AgileP2;
                US3::US3_Valid(v68)
            } else {
                v66
            };
            let v71: bool = &*v57 == "P3";
            let v74: US3 = if v71 {
                let v72: US4 = US4::US4_AgileP3;
                US3::US3_Valid(v72)
            } else {
                v70
            };
            let mut v75: bool = &*v57 == "P4";
            let v78: US3 = if v75 {
                let v76: US4 = US4::US4_AgileP4;
                US3::US3_Valid(v76)
            } else {
                v74
            };
            let v79: bool = &*v57 == "P5";
            let v82: US3 = if v79 {
                let mut v80: US4 = US4::US4_AgileP5;
                US3::US3_Valid(v80)
            } else {
                v78
            };
            let mut v83: i32 = 5i32;
            let v84: Rc<str> = usize::try_from(v83).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v85: bool = &*v84 == "D0";
            let v89: US5 = if v85 {
                let v86: US6 = US6::US6_AgileD0;
                US5::US5_Valid(v86)
            } else {
                US5::US5_Invalid
            };
            let mut v90: bool = &*v84 == "D1";
            let v93: US5 = if v90 {
                let v91: US6 = US6::US6_AgileD1;
                US5::US5_Valid(v91)
            } else {
                v89
            };
            let v94: bool = &*v84 == "D2";
            let v97: US5 = if v94 {
                let v95: US6 = US6::US6_AgileD2;
                US5::US5_Valid(v95)
            } else {
                v93
            };
            let v98: bool = &*v84 == "D3";
            let v101: US5 = if v98 {
                let v99: US6 = US6::US6_AgileD3;
                US5::US5_Valid(v99)
            } else {
                v97
            };
            let v102: bool = &*v84 == "D4";
            let v105: US5 = if v102 {
                let v103: US6 = US6::US6_AgileD4;
                US5::US5_Valid(v103)
            } else {
                v101
            };
            let v106: bool = &*v84 == "D5";
            let v109: US5 = if v106 {
                let v107: US6 = US6::US6_AgileD5;
                US5::US5_Valid(v107)
            } else {
                v105
            };
            let v110: bool = &*v84 == "D6";
            let v113: US5 = if v110 {
                let v111: US6 = US6::US6_AgileD6;
                US5::US5_Valid(v111)
            } else {
                v109
            };
            let v114: bool = &*v84 == "D7";
            let v117: US5 = if v114 {
                let v115: US6 = US6::US6_AgileD7;
                US5::US5_Valid(v115)
            } else {
                v113
            };
            let v118: bool = &*v84 == "D8";
            let v121: US5 = if v118 {
                let v119: US6 = US6::US6_AgileD8;
                US5::US5_Valid(v119)
            } else {
                v117
            };
            match v36 {
                US0::US0_Invalid => {
                    let v328: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id must be uppercase letters, digits and dashes"); } LIT.with(|lit| lit.clone()) };
                    let v329: i32 = 2i32;
                    let v330: Rc<str> = usize::try_from(v329).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let v331: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                    let v332: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let v333: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v334: Rc<str> = concat5_1(v331, v330, v332, v328, v333);
                    let v335: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v334 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v335
                }
                US0::US0_Valid(v122) => {
                    let v122: Rc<str> = v122;
                    match v55 {
                        US1::US1_Invalid => {
                            let v318: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind must be Epic, Feature, Story or TaskKind"); } LIT.with(|lit| lit.clone()) };
                            let v319: i32 = 2i32;
                            let v320: Rc<str> = usize::try_from(v319).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let v321: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                            let v322: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                            let v323: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v324: Rc<str> = concat5_1(v321, v320, v322, v318, v323);
                            let v325: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v324 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v325
                        }
                        US1::US1_Valid(v123) => {
                            let v123: US2 = v123;
                            match v82 {
                                US3::US3_Invalid => {
                                    let v308: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority must be P0..P5"); } LIT.with(|lit| lit.clone()) };
                                    let v309: i32 = 2i32;
                                    let v310: Rc<str> = usize::try_from(v309).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v311: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                    let v312: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                    let v313: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v314: Rc<str> = concat5_1(v311, v310, v312, v308, v313);
                                    let v315: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v314 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v315
                                }
                                US3::US3_Valid(v124) => {
                                    let v124: US4 = v124;
                                    match v121 {
                                        US5::US5_Invalid => {
                                            let v298: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty must be D0..D8"); } LIT.with(|lit| lit.clone()) };
                                            let v299: i32 = 2i32;
                                            let v300: Rc<str> = usize::try_from(v299).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                            let v301: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                            let v302: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                            let v303: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let v304: Rc<str> = concat5_1(v301, v300, v302, v298, v303);
                                            let v305: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v304 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                            v305
                                        }
                                        US5::US5_Valid(v125) => {
                                            let v125: US6 = v125;
                                            let v126: i32 = 8i32;
                                            let v127: i32 = std::env::args().skip(2i32 as usize).count() as i32;
                                            let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let v129: US7 = extras_from_3(v126, v127, v128.clone(), v128.clone(), v128.clone(), v128.clone(), v128.clone(), v128.clone());
                                            let v130: US7 = extras_checked_5(v129);
                                            match v130 {
                                                US7::US7_ExtrasRead(v131, v132, v133, v134, v135, v136) => {
                                                    let v131: Rc<str> = v131;
                                                    let v132: Rc<str> = v132;
                                                    let v133: Rc<str> = v133;
                                                    let v134: Rc<str> = v134;
                                                    let v135: Rc<str> = v135;
                                                    let v136: Rc<str> = v136;
                                                    let v137: i32 = 1i32;
                                                    let v138: Rc<str> = usize::try_from(v137).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                    let v139: i32 = 6i32;
                                                    let v140: Rc<str> = usize::try_from(v139).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                    let v141: i32 = 7i32;
                                                    let v142: Rc<str> = usize::try_from(v141).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                    let v143: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                                                    let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                                                    let v145: Rc<str> = concat5_1(v138, v144, v128.clone(), v128.clone(), v128.clone());
                                                    let v146: bool = std::path::Path::new(&*v145).is_file();
                                                    let v149: US10 = if v146 {
                                                        US10::US10_FileReadable(v145.clone())
                                                    } else {
                                                        US10::US10_FileMissing(v145.clone())
                                                    };
                                                    match v149 {
                                                        US10::US10_FileMissing(v150) => {
                                                            let v150: Rc<str> = v150;
                                                            let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                                                            let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                            let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                            let v154: Rc<str> = concat5_1(v152, v122, v153, v151, v128);
                                                            let v155: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v154 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                            v155
                                                        }
                                                        US10::US10_FileReadable(v156) => {
                                                            let v156: Rc<str> = v156;
                                                            let v157: Rc<str> = std::fs::read_to_string(&*v156).unwrap_or_default().into();
                                                            let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                                            let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                                                            let v160: Rc<str> = Rc::<str>::from(v156.replace(&*v158, &*v159));
                                                            let mut v161: Rc<str> = loop_6(v160);
                                                            let v162: Option<usize> = v161.rfind(&*v159);
                                                            let v163: Option<usize> = method7(v162);
                                                            let mut v164: Rc<dyn Fn((usize)) -> i32> = closure1();
                                                            let mut v165: Option<i32> = v163.map(|x| v164(x));
                                                            let v167: i32 = v165.unwrap_or(-1i32);
                                                            let v168: bool = v167 < 0i32;
                                                            let v172: bool = if v168 {
                                                                true
                                                            } else {
                                                                let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                                                                let v170: bool = v161.ends_with(&*v169);
                                                                let v171: bool = v170 == false;
                                                                v171
                                                            };
                                                            let v193: US11 = if v172 {
                                                                US11::US11_OtherFile
                                                            } else {
                                                                let v174: usize = ((0i32) as usize);
                                                                let v175: usize = ((v167) as usize);
                                                                let v176: Rc<str> = Rc::<str>::from(&(v161)[v174..v175]);
                                                                let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                                                                let v178: bool = v176.ends_with(&*v177);
                                                                if v178 {
                                                                    let v179: Rc<str> = loop_8(v176);
                                                                    US11::US11_StateFile(v179)
                                                                } else {
                                                                    let v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                                                                    let v182: bool = v176.ends_with(&*v181);
                                                                    if v182 {
                                                                        let v183: Rc<str> = loop_9(v176);
                                                                        US11::US11_HistoryFile(v183)
                                                                    } else {
                                                                        let v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                                                                        let v186: bool = v176.ends_with(&*v185);
                                                                        if v186 {
                                                                            let v187: Rc<str> = loop_10(v176);
                                                                            US11::US11_ArchiveFile(v187)
                                                                        } else {
                                                                            US11::US11_OtherFile
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            let v204: Rc<str> = match v193 {
                                                                US11::US11_OtherFile => {
                                                                    v157
                                                                }
                                                                _ => {
                                                                    let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                                    let v195: Rc<UH0> = split_list_11(v194.clone(), v157);
                                                                    let v196: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                                    let v197: Rc<UH0> = method14(v195, v196);
                                                                    let v198: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                                                    let v199: i32 = 0i32;
                                                                    let v200: i32 = method15(v198.clone(), v197, v199);
                                                                    let v201: Rc<Vec<Rc<str>>> = Rc::new(v198.borrow().clone());
                                                                    let v202: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v201).as_ref().clone()));
                                                                    let v203: Rc<str> = Rc::<str>::from((*v202.borrow()).join(&*v194));
                                                                    v203
                                                                }
                                                            };
                                                            let mut v205: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v204);
                                                            let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                                                            let v207: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                                                            let v208: Rc<str> = concat5_1(v206, v122.clone(), v207, v128.clone(), v128.clone());
                                                            let v209: bool = v205.contains(&*v208);
                                                            if v209 {
                                                                let v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task with this id already exists"); } LIT.with(|lit| lit.clone()) };
                                                                let v211: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                                let v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                                let v213: Rc<str> = concat5_1(v211, v122, v212, v210, v128);
                                                                let v214: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v213 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                v214
                                                            } else {
                                                                let v224: Rc<str> = match v125 {
                                                                    US6::US6_AgileD0 => {
                                                                        let v215: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
                                                                        v215
                                                                    }
                                                                    US6::US6_AgileD1 => {
                                                                        let v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
                                                                        v216
                                                                    }
                                                                    US6::US6_AgileD2 => {
                                                                        let v217: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                                                                        v217
                                                                    }
                                                                    US6::US6_AgileD3 => {
                                                                        let v218: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
                                                                        v218
                                                                    }
                                                                    US6::US6_AgileD4 => {
                                                                        let v219: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
                                                                        v219
                                                                    }
                                                                    US6::US6_AgileD5 => {
                                                                        let v220: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
                                                                        v220
                                                                    }
                                                                    US6::US6_AgileD6 => {
                                                                        let v221: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
                                                                        v221
                                                                    }
                                                                    US6::US6_AgileD7 => {
                                                                        let v222: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
                                                                        v222
                                                                    }
                                                                    US6::US6_AgileD8 => {
                                                                        let v223: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
                                                                        v223
                                                                    }
                                                                };
                                                                let v225: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                                                let v226: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                                                                let mut v227: Rc<str> = Rc::<str>::from(v122.replace(&*v225, &*v226));
                                                                let v228: Rc<str> = std::rc::Rc::<str>::from(v227.to_ascii_lowercase());
                                                                let v233: Rc<str> = match v123 {
                                                                    US2::US2_AgileEpic => {
                                                                        let v229: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
                                                                        v229
                                                                    }
                                                                    US2::US2_AgileFeature => {
                                                                        let v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
                                                                        v230
                                                                    }
                                                                    US2::US2_AgileStory => {
                                                                        let v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
                                                                        v231
                                                                    }
                                                                    US2::US2_AgileTaskKind => {
                                                                        let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                                                                        v232
                                                                    }
                                                                };
                                                                let v240: Rc<str> = match v124 {
                                                                    US4::US4_AgileP0 => {
                                                                        let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
                                                                        v234
                                                                    }
                                                                    US4::US4_AgileP1 => {
                                                                        let v235: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                                                                        v235
                                                                    }
                                                                    US4::US4_AgileP2 => {
                                                                        let v236: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
                                                                        v236
                                                                    }
                                                                    US4::US4_AgileP3 => {
                                                                        let v237: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
                                                                        v237
                                                                    }
                                                                    US4::US4_AgileP4 => {
                                                                        let v238: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
                                                                        v238
                                                                    }
                                                                    US4::US4_AgileP5 => {
                                                                        let v239: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
                                                                        v239
                                                                    }
                                                                };
                                                                let v241: Rc<str> = agile_task_creation_text::rig_agile_task_line(&v122, &v228, &v233, &v240, &v224, &v140, &v142, &v134);
                                                                let v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                                let v243: Rc<str> = std::rc::Rc::<str>::from(v205.trim_end_matches(&*v242));
                                                                let v246: Rc<str> = string_concat(&[&*v243, "\n", &*v241, "\n"]);
                                                                let v247: US14 = extras_applied_16(v246, v122.clone(), v131.clone(), v132.clone(), v133.clone(), &v134, v135.clone(), v136);
                                                                match v247 {
                                                                    US14::US14_AddedRefused(v248) => {
                                                                        let v248: Rc<str> = v248;
                                                                        let v249: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                                        let v250: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                                        let v251: Rc<str> = concat5_1(v249, v122, v250, v248, v128);
                                                                        let v252: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v251 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                        v252
                                                                    }
                                                                    US14::US14_AddedText(v253) => {
                                                                        let v253: Rc<str> = v253;
                                                                        let v254: US15 = replace_records_unchanged_17(v145, v205.clone(), v253.clone());
                                                                        match &v254 {
                                                                            US15::US15_Replaced => {
                                                                                let mut v255: Rc<str> = Rc::<str>::from(v122.replace(&*v225, &*v226));
                                                                                let v256: Rc<str> = std::rc::Rc::<str>::from(v255.to_ascii_lowercase());
                                                                                let v257: u64 = v205.len() as u64;
                                                                                let v258: u64 = v253.len() as u64;
                                                                                let v259: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v143);
                                                                                let v260: Rc<str> = agile_task_creation_text::rig_agile_add_receipt(&v122, &v256, &v131, &v132, &v133, &v134, &v135);
                                                                                let v261: Rc<str> = std::rc::Rc::<str>::from((v257).to_string());
                                                                                let v264: Rc<str> = std::rc::Rc::<str>::from((v258).to_string());
                                                                                let v267: Rc<str> = std::rc::Rc::<str>::from((v259).to_string());
                                                                                let v269: Rc<str> = string_concat(&[&*v260, " bytes_before=", &*v261, " bytes_after=", &*v264, " elapsed_ms=", &*v267]);
                                                                                let v270: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v269.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                                                                v270
                                                                            }
                                                                            _ => {
                                                                                let v280: Rc<str> = match v254 {
                                                                                    US15::US15_ChangedSinceRead => {
                                                                                        let v275: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                                                                        v275
                                                                                    }
                                                                                    US15::US15_ReadbackMismatch => {
                                                                                        let v274: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                                        v274
                                                                                    }
                                                                                    US15::US15_RenameFailed => {
                                                                                        let v272: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                                        v272
                                                                                    }
                                                                                    US15::US15_RenameFailedTemporaryKept => {
                                                                                        let v273: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                                                                        v273
                                                                                    }
                                                                                    US15::US15_TemporaryWriteFailed => {
                                                                                        let v271: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                                        v271
                                                                                    }
                                                                                    _ => unreachable!(),
                                                                                };
                                                                                let v281: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                                                let v282: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                                                let v283: Rc<str> = concat5_1(v281, v122, v282, v280, v128);
                                                                                let v284: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v283 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                                v284
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                US7::US7_ExtrasRefused(v291) => {
                                                    let v291: Rc<str> = v291;
                                                    let v292: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                    let v293: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                    let v294: Rc<str> = concat5_1(v292, v122, v293, v291, v128);
                                                    let v295: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v294 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                    v295
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
