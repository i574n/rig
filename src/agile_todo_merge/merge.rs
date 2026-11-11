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
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
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
enum US3 {
    US3_CFamily,
    US3_HashFamily,
    US3_DashFamily,
    US3_HaskellFamily,
    US3_LeanFamily,
    US3_LispFamily,
    US3_ErlangFamily,
    US3_MarkupFamily,
    US3_MlFamily,
    US3_PascalFamily,
    US3_BatchFamily,
    US3_FsharpFamily,
}
#[derive(Clone)]
enum US5 {
    US5_SkipNestedRepository,
    US5_SkipVendorDirectory,
    US5_SkipUnknownSyntax,
    US5_SkipGeneratedOutput,
    US5_SkipLinkedEntry,
}
#[derive(Clone)]
enum US4 {
    US4_ScanPath(US3),
    US4_SkipPath(US5),
}
#[derive(Clone)]
enum US6 {
    US6_ContentScan,
    US6_ContentBinary,
    US6_ContentLarge,
    US6_ContentUnreadable,
}
#[derive(Clone)]
enum US7 {
    US7_RowMiss,
    US7_RowSkip(i32),
    US7_RowHit(u64, u64, Rc<str>, i32, i32),
}
#[derive(Clone)]
enum US8 {
    US8_WordsRoot,
    US8_SymbolsRoot,
}
#[derive(Clone)]
enum US9 {
    US9_Replaced,
    US9_TemporaryWriteFailed,
    US9_RenameFailed,
    US9_RenameFailedTemporaryKept,
    US9_ReadbackMismatch,
    US9_ChangedSinceRead,
}
pub struct Mut1 { pub l0: Rc<str> }
fn concat5_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn file_exists_2(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn loop_3(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method4(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_5(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(6i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_6(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method9(v16, v17)
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method11(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US1 = if v6 {
                let v7: US2 = US2::US2_HeadType;
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
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US2 = US2::US2_HeadInl;
                        US1::US1_KeywordHead(v18)
                    } else {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let mut v21: bool = v2.starts_with(&*v20);
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
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US2 = US2::US2_HeadUnion;
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
            let v51: US1 = match &v39 {
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
                    let v60: Rc<str> = match &v52 {
                        US2::US2_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US2::US2_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US2::US2_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
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
fn method12(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn get_environment_variable_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn cache_path_13(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v2));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v2));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = get_environment_variable_14(v8);
    let v12: Rc<str> = string_concat(&[&*v9, "/kit/todo-cache/", &*v7, ".spi"]);
    v12
}
fn method15(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn harvest_roots_from_16(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v2;
        } else {
            let mut v6: i32 = v4.wrapping_add(19i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v0.get(v6 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v6 as usize) as i32).unwrap_or(-1);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v6 as usize..v8 as usize).unwrap_or(""));
            let v10: bool = &*v2 == "";
            let v14: Rc<str> = if v10 {
                v9
            } else {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                concat5_1(v2, v11, v9, v12.clone(), v12.clone())
            };
            let v15: i32 = v8.wrapping_add(1i32);
            (v1, v2) = (v15, v14);
            continue;
        }
    }
}
fn cached_head_of_19(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    let (mut v2, mut v3): (Rc<str>, Rc<str>) = agile_todo_cache::rig_agile_todo_head_of(&*v0, &*v1);
    (v2, v3)
}
fn cache_row_24(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: Rc<str>, mut v4: i32) -> Rc<str> {
    let v5: Rc<str> = agile_todo_cache::rig_agile_todo_cache_row(&*v0, v1, v2, &*v3, v4);
    v5
}
fn recorded_23(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: u64, mut v12: u64, mut v13: Rc<str>, mut v14: i32) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v15: Rc<str> = cache_row_24(v10, v11, v12, v13.clone(), v14);
    let v16: i32 = v2.wrapping_add(v14);
    let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v0));
    let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v1));
    (v17, v18, v16, v3, v4, v5, v6, v7, v8, v9)
}
fn hit_recorded_22(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: u64, mut v12: u64, mut v13: Rc<str>, mut v14: i32, mut v15: i32) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = recorded_23(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14);
    let v26: i32 = v4.wrapping_add(1i32);
    (v16, v17, v18, v19, v26, v15, v22, v23, v24, v25)
}
fn skipped_row_27(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = agile_todo_cache::rig_agile_todo_skipped_row(&*v0);
    v1
}
fn skipped_26(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v11: Rc<str> = skipped_row_27(v10);
    let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v0));
    (v12, v1, v2, v3, v4, v5, v6, v7, v8, v9)
}
fn findings_of_29(mut v0: US3, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, i32) {
    let v4: i32 = match v0 {
        US3::US3_BatchFamily => {
            11i32
        }
        US3::US3_CFamily => {
            1i32
        }
        US3::US3_DashFamily => {
            3i32
        }
        US3::US3_ErlangFamily => {
            7i32
        }
        US3::US3_FsharpFamily => {
            12i32
        }
        US3::US3_HashFamily => {
            2i32
        }
        US3::US3_HaskellFamily => {
            4i32
        }
        US3::US3_LeanFamily => {
            5i32
        }
        US3::US3_LispFamily => {
            6i32
        }
        US3::US3_MarkupFamily => {
            8i32
        }
        US3::US3_MlFamily => {
            9i32
        }
        US3::US3_PascalFamily => {
            10i32
        }
    };
    let (mut v5, mut v6, mut v7): (Rc<str>, i32, Rc<str>) = agile_todo_lines::rig_agile_todo_findings(v4, "", &*v1, &*v2, &*v3, true);
    (v5, v6)
}
fn ids_in_30(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = agile_todo_cache::rig_agile_todo_ids_in(&*v0);
    v1
}
fn occurrences_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = (v1.clone().len() as i32);
            let v7: i32 = v4.wrapping_add(v6);
            let v8: i32 = v3.wrapping_add(1i32);
            (v2, v3) = (v7, v8);
            continue;
        }
    }
}
fn words_in_31(mut v0: Rc<str>) -> i32 {
    let v1: bool = &*v0 == "";
    if v1 {
        0i32
    } else {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v3: i32 = 0i32;
        let v4: i32 = 0i32;
        let v5: i32 = occurrences_32(v0, v2, v3, v4);
        let mut v6: i32 = v5.wrapping_add(1i32);
        v6
    }
}
fn file_read_28(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: US3, mut v11: Rc<str>, mut v12: u64, mut v13: u64) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let (mut v14, mut v15): (bool, Rc<str>) = match std::fs::read(&*v11) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
    let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v14 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v17: i32 = 0i32;
        let (mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = recorded_23(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v11, v12, v13, v16, v17);
        let v28: i32 = v3.wrapping_add(1i32);
        (v18, v19, v20, v28, v22, v23, v24, v25, v26, v27)
    } else {
        let (mut v29, mut v30): (Rc<str>, i32) = findings_of_29(v10, v7.clone(), v15, v11.clone());
        let v31: Rc<str> = ids_in_30(v29);
        let v32: i32 = words_in_31(v31.clone());
        let v33: i32 = v30.wrapping_sub(v32);
        let (mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = recorded_23(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v11, v12, v13, v31, v33);
        let v44: i32 = v3.wrapping_add(1i32);
        (v34, v35, v36, v44, v38, v39, v40, v41, v42, v43)
    };
    (v45, v46, v47, v48, v49, v50, v51, v52, v53, v54)
}
fn path_verdict_of_code_33(mut v0: i32) -> US4 {
    let v1: bool = 0i32 < v0;
    if v1 {
        let mut v2: bool = v0 == 1i32;
        let v38: US3 = if v2 {
            US3::US3_CFamily
        } else {
            let v4: bool = v0 == 2i32;
            if v4 {
                US3::US3_HashFamily
            } else {
                let mut v6: bool = v0 == 3i32;
                if v6 {
                    US3::US3_DashFamily
                } else {
                    let v8: bool = v0 == 4i32;
                    if v8 {
                        US3::US3_HaskellFamily
                    } else {
                        let v10: bool = v0 == 5i32;
                        if v10 {
                            US3::US3_LeanFamily
                        } else {
                            let v12: bool = v0 == 6i32;
                            if v12 {
                                US3::US3_LispFamily
                            } else {
                                let v14: bool = v0 == 7i32;
                                if v14 {
                                    US3::US3_ErlangFamily
                                } else {
                                    let v16: bool = v0 == 8i32;
                                    if v16 {
                                        US3::US3_MarkupFamily
                                    } else {
                                        let v18: bool = v0 == 9i32;
                                        if v18 {
                                            US3::US3_MlFamily
                                        } else {
                                            let mut v20: bool = v0 == 10i32;
                                            if v20 {
                                                US3::US3_PascalFamily
                                            } else {
                                                let v22: bool = v0 == 11i32;
                                                if v22 {
                                                    US3::US3_BatchFamily
                                                } else {
                                                    let v24: bool = v0 == 12i32;
                                                    if v24 {
                                                        US3::US3_FsharpFamily
                                                    } else {
                                                        US3::US3_CFamily
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
        };
        US4::US4_ScanPath(v38)
    } else {
        let v40: i32 = v0.wrapping_neg();
        let v41: bool = v40 == 1i32;
        let v56: US5 = if v41 {
            US5::US5_SkipNestedRepository
        } else {
            let v43: bool = v40 == 2i32;
            if v43 {
                US5::US5_SkipVendorDirectory
            } else {
                let v45: bool = v40 == 3i32;
                if v45 {
                    US5::US5_SkipUnknownSyntax
                } else {
                    let v47: bool = v40 == 4i32;
                    if v47 {
                        US5::US5_SkipGeneratedOutput
                    } else {
                        let v49: bool = v40 == 5i32;
                        if v49 {
                            US5::US5_SkipLinkedEntry
                        } else {
                            US5::US5_SkipUnknownSyntax
                        }
                    }
                }
            }
        };
        US4::US4_SkipPath(v56)
    }
}
fn cached_row_of_34(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US7 {
    let (mut v3, mut v4, mut v5, mut v6, mut v7, mut v8): (i32, u64, u64, Rc<str>, i32, i32) = agile_todo_cache::rig_agile_todo_row_of(&*v0, v1, &*v2);
    let mut v9: bool = v3 == 2i32;
    if v9 {
        US7::US7_RowHit(v4, v5, v6, v7, v8)
    } else {
        let v11: bool = v3 == 1i32;
        if v11 {
            US7::US7_RowSkip(v8)
        } else {
            US7::US7_RowMiss
        }
    }
}
fn file_scanned_25(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v12: bool = std::path::Path::new(&*v10).with_extension("spi").is_file();
    let v13: i32 = agile_todo_paths::rig_agile_todo_path_verdict(&*v11, v12);
    let v14: US4 = path_verdict_of_code_33(v13);
    let (mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match v14 {
        US4::US4_ScanPath(v15) => {
            let v15: US3 = v15;
            let v16: u64 = std::fs::metadata(&*v10).map(|metadata| metadata.len()).unwrap_or(0);
            let mut v17: bool = v16 > 33554432u64;
            let mut v20: US6 = if v17 {
                US6::US6_ContentLarge
            } else {
                US6::US6_ContentScan
            };
            match v20 {
                US6::US6_ContentLarge => {
                    skipped_26(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
                }
                _ => {
                    let v31: u64 = std::fs::metadata(&*v10).and_then(|metadata| metadata.modified()).ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
                    let v32: US7 = cached_row_of_34(v6.clone(), v5, v10.clone());
                    let (mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match v32 {
                        US7::US7_RowHit(v33, v34, v35, v36, v37) => {
                            let v33: u64 = v33;
                            let v34: u64 = v34;
                            let v35: Rc<str> = v35;
                            let v36: i32 = v36;
                            let v37: i32 = v37;
                            let v38: bool = v33 == v16;
                            let v40: bool = if v38 {
                                let v39: bool = v34 == v31;
                                v39
                            } else {
                                false
                            };
                            if v40 {
                                let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = hit_recorded_22(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v16, v31, v35, v36, v37);
                                (v41, v42, v43, v44, v45, v46, v47, v48, v49, v50)
                            } else {
                                let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = file_read_28(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v15, v10, v16, v31);
                                (v51, v52, v53, v54, v55, v56, v57, v58, v59, v60)
                            }
                        }
                        _ => {
                            let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = file_read_28(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v15, v10, v16, v31);
                            (v71, v72, v73, v74, v75, v76, v77, v78, v79, v80)
                        }
                    };
                    (v81, v82, v83, v84, v85, v86, v87, v88, v89, v90)
                }
            }
        }
        _ => {
            let (mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = skipped_26(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10);
            (v101, v102, v103, v104, v105, v106, v107, v108, v109, v110)
        }
    };
    (v111, v112, v113, v114, v115, v116, v117, v118, v119, v120)
}
fn changed_in_36(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Option<char> = char::from_u32(0u32);
    let v4: char = v2.unwrap();
    let v5: Rc<str> = Rc::<str>::from(format!("{}", v4));
    let v7: Option<char> = char::from_u32(0u32);
    let mut v9: char = v7.unwrap();
    let v10: Rc<str> = Rc::<str>::from(format!("{}", v9));
    let v11: Rc<str> = string_concat(&[&*v5, &*v1, &*v10]);
    let v12: bool = v0.contains(&*v11);
    v12
}
fn last_byte_of_38(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            (v2, v3) = (v6, v4);
            continue;
        }
    }
}
fn spiral_sibling_relative_37(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = 0i32;
    let v3: i32 = -1i32;
    let v4: i32 = last_byte_of_38(v0.clone(), v1, v2, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = v4.wrapping_add(1i32);
    let v7: i32 = -1i32;
    let v8: i32 = last_byte_of_38(v0.clone(), v5, v6, v7);
    let mut v9: bool = v8 > v6;
    let v11: Rc<str> = if v9 {
        let v10: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v8 as usize).unwrap_or(""));
        v10
    } else {
        v0
    };
    let v12: Rc<str> = string_concat(&[&*v11, ".spi"]);
    v12
}
fn unchanged_35(mut v0: &Rc<UH0>, mut v1: &Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: &Rc<str>, mut v7: &Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>) -> bool {
    let v13: bool = if v8 {
        let v11: bool = changed_in_36(v9.clone(), v10.clone());
        let v12: bool = v11 == false;
        v12
    } else {
        false
    };
    if v13 {
        let v14: Rc<str> = spiral_sibling_relative_37(v10);
        let v15: bool = changed_in_36(v9, v14);
        let v16: bool = v15 == false;
        v16
    } else {
        false
    }
}
fn file_walked_21(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v13: Rc<str> = string_concat(&[&*v10, "/", &*v11]);
    let v14: bool = unchanged_35(&v0, &v1, v2, v3, v4, v5, &v6, &v7, v8, v9.clone(), v11.clone());
    let mut v17: US7 = if v14 {
        cached_row_of_34(v6.clone(), v5, v13.clone())
    } else {
        US7::US7_RowMiss
    };
    let (mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match &v17 {
        US7::US7_RowHit(v18, v19, v20, v21, v22) => {
            let v18: u64 = *v18;
            let v19: u64 = *v19;
            let mut v20: Rc<str> = v20.clone();
            let mut v21: i32 = *v21;
            let v22: i32 = *v22;
            hit_recorded_22(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v13, v18, v19, v20, v21, v22)
        }
        _ => {
            let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match &v17 {
                US7::US7_RowSkip(v33) => {
                    let v33: i32 = *v33;
                    let (mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = skipped_26(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v13);
                    (v34, v35, v36, v37, v38, v33, v40, v41, v42, v43)
                }
                _ => {
                    let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match v17 {
                        US7::US7_RowMiss => {
                            file_scanned_25(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v13, v11)
                        }
                        _ => unreachable!(),
                    };
                    (v54, v55, v56, v57, v58, v59, v60, v61, v62, v63)
                }
            };
            (v64, v65, v66, v67, v68, v69, v70, v71, v72, v73)
        }
    };
    (v74, v75, v76, v77, v78, v79, v80, v81, v82, v83)
}
fn listed_20(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: i32) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    loop {
        let v13: i32 = (v11.clone().len() as i32);
        let v14: bool = v13 <= v12;
        if v14 {
            return (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9);
        } else {
            let v15: Option<char> = char::from_u32(0u32);
            let mut v17: char = v15.unwrap();
            let v18: Rc<str> = Rc::<str>::from(format!("{}", v17));
            let v19: i32 = v11.get(v12 as usize..).and_then(|rest| rest.find(&*v18)).map(|found| (found + v12 as usize) as i32).unwrap_or(-1);
            let mut v20: bool = v19 < 0i32;
            let mut v21: i32 = if v20 {
                v13
            } else {
                v19
            };
            let v22: Rc<str> = std::rc::Rc::<str>::from(v11.get(v12 as usize..v21 as usize).unwrap_or(""));
            let v23: bool = &*v22 == "";
            let (mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v23 {
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
            } else {
                let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = file_walked_21(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10.clone(), v22);
                (v24, v25, v26, v27, v28, v29, v30, v31, v32, v33)
            };
            let v44: i32 = v21.wrapping_add(1i32);
            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v12) = (v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44);
            continue;
        }
    }
}
fn head_row_39(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = agile_todo_cache::rig_agile_todo_head_row(&*v0, &*v1, &*v2);
    v3
}
fn root_walked_18(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: &Rc<str>, mut v10: Rc<str>) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let (mut v11, mut v12): (bool, Rc<str>) = agile_todo_sources::rig_agile_todo_sources(&*v10);
    let v13: Rc<str> = agile_todo_sources::rig_agile_todo_git_head(&*v10);
    let (mut v14, mut v15): (Rc<str>, Rc<str>) = cached_head_of_19(v6.clone(), v10.clone());
    let v16: bool = &*v13 == "";
    let v19: Rc<str> = if v16 {
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v17
    } else {
        let v18: Rc<str> = agile_todo_sources::rig_agile_todo_git_changed(&*v10, &*v13);
        v18
    };
    let mut v21: bool = if v16 {
        true
    } else {
        let mut v20: bool = &*v14 == "";
        v20
    };
    let v26: Rc<str> = if v21 {
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v22
    } else {
        let v23: bool = &*v14 == &*v13;
        if v23 {
            v19.clone()
        } else {
            let v24: Rc<str> = agile_todo_sources::rig_agile_todo_git_changed(&*v10, &*v14);
            v24
        }
    };
    let v27: bool = &*v26 == "";
    let v29: bool = if v27 {
        true
    } else {
        let v28: bool = &*v19 == "";
        v28
    };
    let v43: Rc<str> = if v29 {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v30
    } else {
        let v32: Option<char> = char::from_u32(0u32);
        let v34: char = v32.unwrap();
        let v35: Rc<str> = Rc::<str>::from(format!("{}", v34));
        let v38: Option<char> = char::from_u32(0u32);
        let v40: char = v38.unwrap();
        let v41: Rc<str> = Rc::<str>::from(format!("{}", v40));
        let v42: Rc<str> = string_concat(&[&*v26, &*v19, &*v35, &*v15, &*v41]);
        v42
    };
    let v44: bool = &*v43 != "";
    let (mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v11 {
        let v45: i32 = 0i32;
        let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = listed_20(v0, v1, v2, v3, v4, v5, v6, v7, v44, v43, v10.clone(), v12, v45);
        (v46, v47, v48, v49, v50, v51, v52, v53, v54, v55)
    } else {
        (v0, v1, v2, v3, v4, v5, v6, v7, v44, v43)
    };
    let v67: bool = if v16 {
        true
    } else {
        let v66: bool = &*v19 == "";
        v66
    };
    let v70: Rc<UH0> = if v67 {
        v56
    } else {
        let v68: Rc<str> = head_row_39(v10, v13, v19);
        Rc::new(UH0::UH0_Cons(v68, v56))
    };
    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v70, v57, v58, v59, v60, v61, v62, v63, false, v71)
}
fn roots_walked_17(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: i32) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    loop {
        let v12: i32 = (v10.clone().len() as i32);
        let v13: bool = v12 <= v11;
        if v13 {
            return (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9);
        } else {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v15: i32 = v10.get(v11 as usize..).and_then(|rest| rest.find(&*v14)).map(|found| (found + v11 as usize) as i32).unwrap_or(-1);
            let v16: bool = v15 < 0i32;
            let mut v17: i32 = if v16 {
                v12
            } else {
                v15
            };
            let v18: Rc<str> = std::rc::Rc::<str>::from(v10.get(v11 as usize..v17 as usize).unwrap_or(""));
            let v19: bool = &*v18 == "";
            let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v19 {
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
            } else {
                let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = root_walked_18(v0, v1, v2, v3, v4, v5, v6, v7, v8, &v9, v18);
                (v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)
            };
            let v40: i32 = v17.wrapping_add(1i32);
            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v11) = (v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40);
            continue;
        }
    }
}
fn method40(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method40(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US1 = if v6 {
                let v7: US2 = US2::US2_HeadType;
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
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US2 = US2::US2_HeadInl;
                        US1::US1_KeywordHead(v18)
                    } else {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let mut v21: bool = v2.starts_with(&*v20);
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
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US2 = US2::US2_HeadUnion;
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
            let v51: US1 = match &v39 {
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
                US1::US1_KeywordHead(v52) => {
                    let v52: US2 = v52;
                    let v59: Rc<str> = match &v52 {
                        US2::US2_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US2::US2_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US2::US2_HeadType => {
                            v5
                        }
                        US2::US2_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v60: i32 = (v59.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US2::US2_HeadInl => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v61
                        }
                        US2::US2_HeadOpen => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                        US2::US2_HeadType => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US2::US2_HeadUnion => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                    };
                    let v69: i32 = (v2.clone().len() as i32);
                    let v70: i32 = v69.wrapping_sub(v60);
                    let v71: usize = ((v60) as usize);
                    let v72: i32 = v60.wrapping_add(v70);
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
fn renamed_within_41(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn method44(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_43(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method44(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn item_lines_42(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> (i32, i32) {
    loop {
        let v5: i32 = (v0.clone().len() as i32);
        let mut v6: bool = v5 <= v2;
        if v6 {
            return (v3, v4);
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let mut v9: bool = v8 < 0i32;
            let v10: i32 = if v9 {
                v5
            } else {
                v8
            };
            let v11: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v10 as usize).unwrap_or(""));
            let v12: bool = &*v11 == "";
            let v18: i32 = if v12 {
                0i32
            } else {
                let v14: Rc<str> = string_concat(&[" ", &*v11, " "]);
                let v15: i32 = 0i32;
                let v16: i32 = 0i32;
                occurrences_32(v1.clone(), v14, v15, v16)
            };
            let v19: bool = v18 > 0i32;
            let v24: i32 = if v19 {
                let mut v21: Rc<str> = format_real_43(v18);
                let v22: Rc<str> = string_concat(&[&*v11, "  wip=", &*v21]);
                let v23: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v22.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                v23
            } else {
                0i32
            };
            let v25: i32 = v10.wrapping_add(1i32);
            let v27: i32 = if v19 {
                let v26: i32 = v3.wrapping_add(1i32);
                v26
            } else {
                v3
            };
            let v28: i32 = v4.wrapping_add(v18);
            (v2, v3, v4) = (v25, v27, v28);
            continue;
        }
    }
}
fn method45(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: i32 = words_in_31(v2);
                let v5: i32 = v1.wrapping_add(v4);
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn format_real_46(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method44(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn wip_run_0(mut v0: Rc<str>) -> i32 {
    let v1: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = concat5_1(v0.clone(), v2, v3.clone(), v3.clone(), v3.clone());
    let v5: bool = file_exists_2(v4.clone());
    let mut v6: bool = v5 == false;
    if v6 {
        let v7: Rc<str> = string_concat(&["agile wip rejected: no backlog state at ", &*v4]);
        let v8: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v7 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v8
    } else {
        let mut v9: bool = std::path::Path::new(&*v4).is_file();
        let v62: Rc<str> = if v9 {
            let (mut v10, mut v11): (bool, Rc<str>) = match std::fs::read(&*v4) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v10 {
                let v12: Rc<str> = string_concat(&["rig state read failed: ", &*v4]);
                std::panic::panic_any::<std::string::String>(format!("{}", v12))
            } else {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = Rc::<str>::from(v4.replace(&*v14, &*v15));
                let mut v17: Rc<str> = loop_3(v16);
                let v18: Option<usize> = v17.rfind(&*v15);
                let v19: Option<usize> = method4(v18);
                let mut v20: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v21: Option<i32> = v19.map(|x| v20(x));
                let v23: i32 = v21.unwrap_or(-1i32);
                let v24: bool = v23 < 0i32;
                let v28: bool = if v24 {
                    true
                } else {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v26: bool = v17.ends_with(&*v25);
                    let v27: bool = v26 == false;
                    v27
                };
                let v49: US0 = if v28 {
                    US0::US0_OtherFile
                } else {
                    let v30: usize = ((0i32) as usize);
                    let v31: usize = ((v23) as usize);
                    let v32: Rc<str> = Rc::<str>::from(&(v17)[v30..v31]);
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v34: bool = v32.ends_with(&*v33);
                    if v34 {
                        let v35: Rc<str> = loop_5(v32);
                        US0::US0_StateFile(v35)
                    } else {
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v38: bool = v32.ends_with(&*v37);
                        if v38 {
                            let v39: Rc<str> = loop_6(v32);
                            US0::US0_HistoryFile(v39)
                        } else {
                            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v42: bool = v32.ends_with(&*v41);
                            if v42 {
                                let v43: Rc<str> = loop_7(v32);
                                US0::US0_ArchiveFile(v43)
                            } else {
                                US0::US0_OtherFile
                            }
                        }
                    }
                };
                match v49 {
                    US0::US0_OtherFile => {
                        v11
                    }
                    _ => {
                        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v51: Rc<UH0> = split_list_8(v50.clone(), v11);
                        let v52: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v53: Rc<UH0> = method11(v51, v52);
                        let v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v55: i32 = 0i32;
                        let v56: i32 = method12(v54.clone(), v53, v55);
                        let v57: Rc<Vec<Rc<str>>> = Rc::new(v54.borrow().clone());
                        let v58: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v57).as_ref().clone()));
                        let v59: Rc<str> = Rc::<str>::from((*v58.borrow()).join(&*v50));
                        v59
                    }
                }
            }
        } else {
            v3.clone()
        };
        let v63: Rc<str> = cache_path_13(v0);
        let v64: bool = file_exists_2(v63.clone());
        let v76: Rc<str> = if v64 {
            let v66: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v63);
            let v68: Vec<u8> = v66.unwrap();
            let v69: Vec<u8> = method15(v68);
            let v71: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v69);
            let v73: std::string::String = v71.unwrap();
            let v75: Rc<str> = Rc::<str>::from(String::as_str(&v73));
            v75
        } else {
            v3.clone()
        };
        let v77: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v78: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v79: i32 = 0i32;
        let v80: i32 = 0i32;
        let v81: i32 = 0i32;
        let v82: i32 = 0i32;
        let v83: bool = false;
        let v84: i32 = 0i32;
        let v85: Rc<str> = harvest_roots_from_16(v62.clone(), v84, v3.clone());
        let v86: i32 = 0i32;
        let (mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = roots_walked_17(v77, v78, v79, v80, v81, v82, v76, v62.clone(), v83, v3, v85, v86);
        let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
        let v98: Rc<str> = get_environment_variable_14(v97);
        let v99: Rc<str> = string_concat(&[&*v98, "/kit/todo-cache"]);
        let v101: Result<(), std::io::Error> = std::fs::create_dir_all(&*v99);
        let v102: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v103: Rc<UH0> = method9(v87, v102);
        let v104: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v105: i32 = 0i32;
        let v106: i32 = method12(v104.clone(), v103, v105);
        let v107: Rc<Vec<Rc<str>>> = Rc::new(v104.borrow().clone());
        let v108: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v107).as_ref().clone()));
        let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v110: Rc<str> = Rc::<str>::from((*v108.borrow()).join(&*v109));
        let v111: Rc<str> = string_concat(&[&*v63, ".pending"]);
        let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v114: Rc<str> = Rc::<str>::from(v111.replace(&*v112, &*v113));
        let mut v115: Rc<str> = loop_3(v114);
        let v116: Option<usize> = v115.rfind(&*v113);
        let v117: Option<usize> = method4(v116);
        let mut v118: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v119: Option<i32> = v117.map(|x| v118(x));
        let v121: i32 = v119.unwrap_or(-1i32);
        let v122: bool = v121 < 0i32;
        let v126: bool = if v122 {
            true
        } else {
            let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v124: bool = v115.ends_with(&*v123);
            let v125: bool = v124 == false;
            v125
        };
        let v147: US0 = if v126 {
            US0::US0_OtherFile
        } else {
            let v128: usize = ((0i32) as usize);
            let v129: usize = ((v121) as usize);
            let v130: Rc<str> = Rc::<str>::from(&(v115)[v128..v129]);
            let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let v132: bool = v130.ends_with(&*v131);
            if v132 {
                let v133: Rc<str> = loop_5(v130);
                US0::US0_StateFile(v133)
            } else {
                let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                let v136: bool = v130.ends_with(&*v135);
                if v136 {
                    let v137: Rc<str> = loop_6(v130);
                    US0::US0_HistoryFile(v137)
                } else {
                    let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                    let v140: bool = v130.ends_with(&*v139);
                    if v140 {
                        let v141: Rc<str> = loop_7(v130);
                        US0::US0_ArchiveFile(v141)
                    } else {
                        US0::US0_OtherFile
                    }
                }
            }
        };
        let v170: US8 = match v147 {
            US0::US0_ArchiveFile(v160) => {
                let v160: Rc<str> = v160;
                let v161: Rc<str> = string_concat(&[&*v160, "/state/spiral.toml"]);
                let v162: bool = std::path::Path::new(&*v161).is_file();
                if v162 {
                    US8::US8_SymbolsRoot
                } else {
                    US8::US8_WordsRoot
                }
            }
            US0::US0_HistoryFile(v154) => {
                let v154: Rc<str> = v154;
                let v155: Rc<str> = string_concat(&[&*v154, "/state/spiral.toml"]);
                let v156: bool = std::path::Path::new(&*v155).is_file();
                if v156 {
                    US8::US8_SymbolsRoot
                } else {
                    US8::US8_WordsRoot
                }
            }
            US0::US0_OtherFile => {
                US8::US8_WordsRoot
            }
            US0::US0_StateFile(v148) => {
                let v148: Rc<str> = v148;
                let v149: Rc<str> = string_concat(&[&*v148, "/state/spiral.toml"]);
                let v150: bool = std::path::Path::new(&*v149).is_file();
                if v150 {
                    US8::US8_SymbolsRoot
                } else {
                    US8::US8_WordsRoot
                }
            }
        };
        let v181: Rc<str> = match v170 {
            US8::US8_SymbolsRoot => {
                let v171: Rc<UH0> = split_list_8(v109.clone(), v110.clone());
                let v172: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v173: Rc<UH0> = method40(v171, v172);
                let v174: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v175: i32 = 0i32;
                let v176: i32 = method12(v174.clone(), v173, v175);
                let v177: Rc<Vec<Rc<str>>> = Rc::new(v174.borrow().clone());
                let v178: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v177).as_ref().clone()));
                let v179: Rc<str> = Rc::<str>::from((*v178.borrow()).join(&*v109));
                v179
            }
            US8::US8_WordsRoot => {
                v110.clone()
            }
        };
        let v182: bool = std::fs::write(&*v111, &*v181).is_ok();
        let v262: US9 = if v182 {
            let v183: i32 = 40i32;
            let v184: bool = renamed_within_41(v63.clone(), v111.clone(), v183);
            if v184 {
                let v185: Rc<str> = Rc::<str>::from(v63.replace(&*v112, &*v113));
                let mut v186: Rc<str> = loop_3(v185);
                let v187: Option<usize> = v186.rfind(&*v113);
                let v188: Option<usize> = method4(v187);
                let mut v189: Option<i32> = v188.map(|x| v118(x));
                let v191: i32 = v189.unwrap_or(-1i32);
                let v192: bool = v191 < 0i32;
                let v196: bool = if v192 {
                    true
                } else {
                    let v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v194: bool = v186.ends_with(&*v193);
                    let v195: bool = v194 == false;
                    v195
                };
                let v217: US0 = if v196 {
                    US0::US0_OtherFile
                } else {
                    let v198: usize = ((0i32) as usize);
                    let v199: usize = ((v191) as usize);
                    let v200: Rc<str> = Rc::<str>::from(&(v186)[v198..v199]);
                    let v201: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                    let v202: bool = v200.ends_with(&*v201);
                    if v202 {
                        let v203: Rc<str> = loop_5(v200);
                        US0::US0_StateFile(v203)
                    } else {
                        let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                        let v206: bool = v200.ends_with(&*v205);
                        if v206 {
                            let v207: Rc<str> = loop_6(v200);
                            US0::US0_HistoryFile(v207)
                        } else {
                            let v209: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                            let v210: bool = v200.ends_with(&*v209);
                            if v210 {
                                let v211: Rc<str> = loop_7(v200);
                                US0::US0_ArchiveFile(v211)
                            } else {
                                US0::US0_OtherFile
                            }
                        }
                    }
                };
                let v240: US8 = match v217 {
                    US0::US0_ArchiveFile(v230) => {
                        let v230: Rc<str> = v230;
                        let v231: Rc<str> = string_concat(&[&*v230, "/state/spiral.toml"]);
                        let v232: bool = std::path::Path::new(&*v231).is_file();
                        if v232 {
                            US8::US8_SymbolsRoot
                        } else {
                            US8::US8_WordsRoot
                        }
                    }
                    US0::US0_HistoryFile(v224) => {
                        let v224: Rc<str> = v224;
                        let v225: Rc<str> = string_concat(&[&*v224, "/state/spiral.toml"]);
                        let v226: bool = std::path::Path::new(&*v225).is_file();
                        if v226 {
                            US8::US8_SymbolsRoot
                        } else {
                            US8::US8_WordsRoot
                        }
                    }
                    US0::US0_OtherFile => {
                        US8::US8_WordsRoot
                    }
                    US0::US0_StateFile(v218) => {
                        let v218: Rc<str> = v218;
                        let v219: Rc<str> = string_concat(&[&*v218, "/state/spiral.toml"]);
                        let v220: bool = std::path::Path::new(&*v219).is_file();
                        if v220 {
                            US8::US8_SymbolsRoot
                        } else {
                            US8::US8_WordsRoot
                        }
                    }
                };
                let v251: Rc<str> = match v240 {
                    US8::US8_SymbolsRoot => {
                        let v241: Rc<UH0> = split_list_8(v109.clone(), v110);
                        let v242: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v243: Rc<UH0> = method40(v241, v242);
                        let v244: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v245: i32 = 0i32;
                        let v246: i32 = method12(v244.clone(), v243, v245);
                        let v247: Rc<Vec<Rc<str>>> = Rc::new(v244.borrow().clone());
                        let v248: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v247).as_ref().clone()));
                        let v249: Rc<str> = Rc::<str>::from((*v248.borrow()).join(&*v109));
                        v249
                    }
                    US8::US8_WordsRoot => {
                        v110
                    }
                };
                let v252: bool = std::fs::read_to_string(&*v63).is_ok_and(|value| value == *v251);
                if v252 {
                    US9::US9_Replaced
                } else {
                    US9::US9_ReadbackMismatch
                }
            } else {
                let v256: bool = std::fs::remove_file(&*v111).is_ok() || !std::path::Path::new(&*v111).exists();
                if v256 {
                    US9::US9_RenameFailed
                } else {
                    US9::US9_RenameFailedTemporaryKept
                }
            }
        } else {
            US9::US9_TemporaryWriteFailed
        };
        let v263: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v264: i32 = 0i32;
        let v265: i32 = method12(v263.clone(), v88.clone(), v264);
        let v266: Rc<Vec<Rc<str>>> = Rc::new(v263.borrow().clone());
        let v267: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v266).as_ref().clone()));
        let v268: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v269: Rc<str> = Rc::<str>::from((*v267.borrow()).join(&*v268));
        let v271: Rc<str> = string_concat(&[" ", &*v269, " "]);
        let v272: Rc<str> = agile_task_census::rig_agile_unsettled_ids(&*v62);
        let v273: i32 = 0i32;
        let v274: i32 = 0i32;
        let v275: i32 = 0i32;
        let (mut v276, mut v277): (i32, i32) = item_lines_42(v272, v271, v273, v274, v275);
        let v278: i32 = 0i32;
        let v279: i32 = method45(v88, v278);
        let v280: i32 = v279.wrapping_sub(v277);
        let v281: Rc<str> = format_real_43(v280);
        let v283: Rc<str> = string_concat(&["stale wip: ", &*v281, " markers naming a Done or unknown item"]);
        let v284: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v283.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        let v285: Rc<str> = format_real_43(v89);
        let v287: Rc<str> = string_concat(&["untracked wip: ", &*v285, " markers without an ID"]);
        let v288: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v287.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        let v289: Rc<str> = format_real_43(v276);
        let v292: Rc<str> = format_real_43(v277);
        let v295: Rc<str> = format_real_43(v89);
        let v298: Rc<str> = format_real_43(v90);
        let v301: Rc<str> = format_real_43(v91);
        let v304: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v1);
        let v305: Rc<str> = format_real_46(v304);
        let v306: Rc<str> = string_concat(&["rig agile wip ok items=", &*v289, " markers=", &*v292, " untracked=", &*v295, " scanned=", &*v298, " cached=", &*v301, " elapsed_ms=", &*v305]);
        let v307: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v306.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        v307
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 1i32;
        let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        wip_run_0(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_wip_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
