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
    US0_MigratePreview,
    US0_MigrateApply,
}
#[derive(Clone)]
enum US1 {
    US1_SpiprojProject,
    US1_TomlProject,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_SplitProject(Rc<UH0>, Rc<UH0>, Rc<UH0>),
    US2_NoModules,
}
#[derive(Clone)]
enum US3 {
    US3_ModuleListed,
    US3_ModuleAdded(Rc<str>),
    US3_NoModulesSection,
}
#[derive(Clone)]
enum US4 {
    US4_Some(i32),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_StateFile(Rc<str>),
    US5_HistoryFile(Rc<str>),
    US5_ArchiveFile(Rc<str>),
    US5_OtherFile,
}
#[derive(Clone)]
enum US6 {
    US6_WordsRoot,
    US6_SymbolsRoot,
}
#[derive(Clone)]
enum US8 {
    US8_HeadInl,
    US8_HeadUnion,
    US8_HeadOpen,
    US8_HeadType,
}
#[derive(Clone)]
enum US7 {
    US7_KeywordHead(US8),
    US7_SymbolHead(US8),
    US7_OtherLine,
}
#[derive(Clone)]
enum US9 {
    US9_Some(Rc<str>),
    US9_None,
}
#[derive(Clone)]
enum US10 {
    US10_Some(US0),
    US10_None,
}
fn migrate_usage_1() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: agile usage: migrate-activity preview|apply <root>"); } LIT.with(|lit| lit.clone()) };
    eprintln!("{}", v0);
    2i32
}
fn file_exists_3(mut v0: Rc<str>) -> bool {
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
fn method4(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn schema_module_6() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("activity_schema"); } LIT.with(|lit| lit.clone()) };
    v0
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method9(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
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
        while method9(v7, &v9) {
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
        method8(v16, v17)
    }
}
fn method11(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method12(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method10(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let mut v6: i32 = 0i32;
            let v7: i32 = method11(v2.clone(), v5, v6);
            let v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2.clone(), v7 as i64, v8 as i64);
            let v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method12(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            let v13: bool = &*v12 != "";
            if v13 {
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
fn section_lines_14(mut v0: US1, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> US2 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v8, v9) => {
                let v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH0> = v9.clone();
                let v26: bool = match &v0 {
                    US1::US1_SpiprojProject => {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v11: bool = v8.starts_with(&*v10);
                        let v14: bool = if v11 {
                            true
                        } else {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let v13: bool = v8.starts_with(&*v12);
                            v13
                        };
                        let v15: bool = v14 == false;
                        v15
                    }
                    US1::US1_TomlProject => {
                        let v16: i32 = (v8.clone().len() as i32);
                        let mut v17: i32 = 0i32;
                        let mut v18: i32 = method11(v8.clone(), v16, v17);
                        let v19: i32 = v16.wrapping_sub(1i32);
                        let v20: Rc<str> = string_slice(&v8.clone(), v18 as i64, v19 as i64);
                        let v21: i32 = (v20.clone().len() as i32);
                        let v22: i32 = method12(v20.clone(), v21);
                        let v23: Rc<str> = string_slice(&v20, 0i32 as i64, v22 as i64);
                        let mut v24: bool = &*v23 == "'''";
                        v24
                    }
                };
                if v26 {
                    let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v28: Rc<UH0> = method8(v3, v27);
                    return US2::US2_SplitProject(v2, v28, v1);
                } else {
                    let v30: i32 = (v8.clone().len() as i32);
                    let v31: i32 = 0i32;
                    let v32: i32 = method11(v8.clone(), v30, v31);
                    let v33: i32 = v30.wrapping_sub(1i32);
                    let v34: Rc<str> = string_slice(&v8, v32 as i64, v33 as i64);
                    let v35: i32 = (v34.clone().len() as i32);
                    let v36: i32 = method12(v34.clone(), v35);
                    let v37: Rc<str> = string_slice(&v34, 0i32 as i64, v36 as i64);
                    let v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v37, v3));
                    (v1, v3) = (v9, v38);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH0> = method8(v3, v4);
                let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return US2::US2_SplitProject(v2, v5, v6);
            }
        }
    }
}
fn split_lines_13(mut v0: US1, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> US2 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: i32 = (v4.clone().len() as i32);
                let v7: i32 = 0i32;
                let v8: i32 = method11(v4.clone(), v6, v7);
                let mut v9: i32 = v6.wrapping_sub(1i32);
                let v10: Rc<str> = string_slice(&v4.clone(), v8 as i64, v9 as i64);
                let v11: i32 = (v10.clone().len() as i32);
                let v12: i32 = method12(v10.clone(), v11);
                let v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
                let mut v17: Rc<str> = match &v0 {
                    US1::US1_SpiprojProject => {
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                        v14
                    }
                    US1::US1_TomlProject => {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                        v15
                    }
                };
                let mut v18: bool = &*v13 == &*v17;
                if v18 {
                    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v19));
                    let v21: Rc<UH0> = method8(v2, v20);
                    let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    return section_lines_14(v0, v5, v21, v22);
                } else {
                    let mut v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v2));
                    (v1, v2) = (v5, v24);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US2::US2_NoModules;
            }
        }
    }
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US4> {
    Rc::new(move |mut v2: Rc<UH0>| -> US4 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US4::US4_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US4> = method15(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US4::US4_None
            }
        }
    })
}
fn method15(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US4> {
    closure1(v0, v1)
}
fn method16(mut v0: US1, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method16(v0.clone(), v4, v2);
            let v13: Rc<str> = match v0 {
                US1::US1_SpiprojProject => {
                    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                    v6.borrow_mut().push(v7);
                    v6.borrow_mut().push(v3);
                    let v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
                    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
                    v11
                }
                US1::US1_TomlProject => {
                    v3
                }
            };
            Rc::new(UH0::UH0_Cons(v13, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method17(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method18(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn modules_with_schema_5(mut v0: US1, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = schema_module_6();
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v1.replace(&*v4, &*v5));
    let v7: Rc<UH0> = split_list_7(v3.clone(), v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method10(v7, v8);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v11: US2 = split_lines_13(v0.clone(), v9, v10);
    let v47: US3 = match v11 {
        US2::US2_NoModules => {
            US3::US3_NoModulesSection
        }
        US2::US2_SplitProject(v13, v14, v15) => {
            let v13: Rc<UH0> = v13;
            let v14: Rc<UH0> = v14;
            let v15: Rc<UH0> = v15;
            let v16: i32 = 0i32;
            let mut v17: Rc<dyn Fn(Rc<UH0>) -> US4> = method15(v2.clone(), v16);
            let mut v18: US4 = v17(v14.clone());
            let v19: bool = match v18 {
                US4::US4_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v20: bool = v19 != true;
            if v20 {
                US3::US3_ModuleListed
            } else {
                let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v23: Rc<UH0> = method16(v0.clone(), v14, v22);
                let v30: Rc<str> = match &v0 {
                    US1::US1_SpiprojProject => {
                        let mut v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                        v24.borrow_mut().push(v25);
                        v24.borrow_mut().push(v2.clone());
                        let v26: Rc<Vec<Rc<str>>> = Rc::new(v24.borrow().clone());
                        let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v26).as_ref().clone()));
                        let v28: Rc<str> = Rc::<str>::from((*v27.borrow()).join(&*v5));
                        v28
                    }
                    US1::US1_TomlProject => {
                        v2.clone()
                    }
                };
                let v31: Rc<UH0> = method17(v23, v15);
                let v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v30, v31));
                let v33: Rc<UH0> = method17(v13, v32);
                let v34: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v35: i32 = 0i32;
                let v36: i32 = method18(v34.clone(), v33, v35);
                let v37: Rc<Vec<Rc<str>>> = Rc::new(v34.borrow().clone());
                let v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v37).as_ref().clone()));
                let v39: Rc<str> = Rc::<str>::from((*v38.borrow()).join(&*v3));
                let mut v40: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v40.borrow_mut().push(v39);
                v40.borrow_mut().push(v3.clone());
                let v41: Rc<Vec<Rc<str>>> = Rc::new(v40.borrow().clone());
                let v42: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v41).as_ref().clone()));
                let v43: Rc<str> = Rc::<str>::from((*v42.borrow()).join(&*v5));
                US3::US3_ModuleAdded(v43)
            }
        }
    };
    match v47 {
        US3::US3_ModuleAdded(v48) => {
            let v48: Rc<str> = v48;
            v48
        }
        US3::US3_ModuleListed => {
            v1
        }
        US3::US3_NoModulesSection => {
            let v49: Rc<str> = Rc::<str>::from(v1.replace(&*v4, &*v5));
            let mut v50: Rc<UH0> = split_list_7(v3.clone(), v49);
            let v51: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v52: Rc<UH0> = method10(v50, v51);
            let v56: Rc<str> = match &v0 {
                US1::US1_SpiprojProject => {
                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                    v53
                }
                US1::US1_TomlProject => {
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                    v54
                }
            };
            let mut v63: Rc<str> = match &v0 {
                US1::US1_SpiprojProject => {
                    let mut v57: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                    v57.borrow_mut().push(v58);
                    v57.borrow_mut().push(v2);
                    let v59: Rc<Vec<Rc<str>>> = Rc::new(v57.borrow().clone());
                    let mut v60: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v59).as_ref().clone()));
                    let v61: Rc<str> = Rc::<str>::from((*v60.borrow()).join(&*v5));
                    v61
                }
                US1::US1_TomlProject => {
                    v2
                }
            };
            let v69: Rc<UH0> = match v0 {
                US1::US1_SpiprojProject => {
                    { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                }
                US1::US1_TomlProject => {
                    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'''"); } LIT.with(|lit| lit.clone()) };
                    let v66: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH0::UH0_Cons(v65, v66))
                }
            };
            let v70: Rc<UH0> = Rc::new(UH0::UH0_Cons(v63, v69));
            let v71: Rc<UH0> = Rc::new(UH0::UH0_Cons(v56, v70));
            let v72: Rc<UH0> = method17(v52, v71);
            let v73: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v74: i32 = 0i32;
            let v75: i32 = method18(v73.clone(), v72, v74);
            let v76: Rc<Vec<Rc<str>>> = Rc::new(v73.borrow().clone());
            let v77: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v76).as_ref().clone()));
            let v78: Rc<str> = Rc::<str>::from((*v77.borrow()).join(&*v3));
            let mut v79: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v79.borrow_mut().push(v78);
            v79.borrow_mut().push(v3);
            let v80: Rc<Vec<Rc<str>>> = Rc::new(v79.borrow().clone());
            let v81: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v80).as_ref().clone()));
            let v82: Rc<str> = Rc::<str>::from((*v81.borrow()).join(&*v5));
            v82
        }
    }
}
fn loop_19(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
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
fn method20(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_21(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
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
fn loop_22(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
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
fn loop_23(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
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
fn method24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method24(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US7 = if v6 {
                let v7: US8 = US8::US8_HeadType;
                US7::US7_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US8 = US8::US8_HeadType;
                    US7::US7_SymbolHead(v11)
                } else {
                    US7::US7_OtherLine
                }
            };
            let v27: US7 = match &v15 {
                US7::US7_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let mut v18: US8 = US8::US8_HeadInl;
                        US7::US7_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let v22: US8 = US8::US8_HeadInl;
                            US7::US7_SymbolHead(v22)
                        } else {
                            US7::US7_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US7 = match &v27 {
                US7::US7_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US8 = US8::US8_HeadUnion;
                        US7::US7_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US8 = US8::US8_HeadUnion;
                            US7::US7_SymbolHead(v34)
                        } else {
                            US7::US7_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US7 = match &v39 {
                US7::US7_OtherLine => {
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US8 = US8::US8_HeadOpen;
                        US7::US7_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US8 = US8::US8_HeadOpen;
                            US7::US7_SymbolHead(v46)
                        } else {
                            US7::US7_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US7::US7_KeywordHead(v52) => {
                    let v52: US8 = v52;
                    let v59: Rc<str> = match &v52 {
                        US8::US8_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US8::US8_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US8::US8_HeadType => {
                            v5
                        }
                        US8::US8_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let mut v60: i32 = (v59.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US8::US8_HeadInl => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v61
                        }
                        US8::US8_HeadOpen => {
                            let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                        US8::US8_HeadType => {
                            let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US8::US8_HeadUnion => {
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
fn step_word_25(mut v0: bool, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: bool = v1 == false;
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kept"); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        if v0 {
            v2
        } else {
            v3
        }
    }
}
fn migrated_2(mut v0: US0, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: Rc<str> = string_concat(&[&*v4, "/state"]);
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v6.borrow_mut().push(v5.clone());
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.toml"); } LIT.with(|lit| lit.clone()) };
    v6.borrow_mut().push(v7.clone());
    let v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
    let v10: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v3));
    let v11: bool = std::path::Path::new(&*v10).is_file();
    let v14: US1 = if v11 {
        US1::US1_TomlProject
    } else {
        US1::US1_SpiprojProject
    };
    let mut v17: Rc<str> = match v14 {
        US1::US1_SpiprojProject => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
            v15
        }
        US1::US1_TomlProject => {
            v7.clone()
        }
    };
    let mut v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v18.borrow_mut().push(v5.clone());
    v18.borrow_mut().push(v17);
    let v19: Rc<Vec<Rc<str>>> = Rc::new(v18.borrow().clone());
    let v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v19).as_ref().clone()));
    let v21: Rc<str> = Rc::<str>::from((*v20.borrow()).join(&*v3));
    let v22: Rc<str> = string_concat(&[&*v5, "/activity_schema.spi"]);
    let v23: bool = file_exists_3(v21.clone());
    let mut v24: bool = v23 == false;
    if v24 {
        let v25: Rc<str> = string_concat(&["rig error: agile migrate-activity: no state project (package.spiproj or spiral.toml) under ", &*v1]);
        eprintln!("{}", v25);
        2i32
    } else {
        let v26: bool = match &v0 {
            US0::US0_MigrateApply => {
                true
            }
            _ => {
                false
            }
        };
        let v27: bool = file_exists_3(v22.clone());
        let v28: bool = v27 == false;
        let v30: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v21);
        let v32: Vec<u8> = v30.unwrap();
        let v33: Vec<u8> = method4(v32);
        let v35: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v33);
        let v37: std::string::String = v35.unwrap();
        let v39: Rc<str> = Rc::<str>::from(String::as_str(&v37));
        let mut v40: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        v40.borrow_mut().push(v5);
        v40.borrow_mut().push(v7);
        let v41: Rc<Vec<Rc<str>>> = Rc::new(v40.borrow().clone());
        let v42: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v41).as_ref().clone()));
        let v43: Rc<str> = Rc::<str>::from((*v42.borrow()).join(&*v3));
        let v44: bool = std::path::Path::new(&*v43).is_file();
        let v47: US1 = if v44 {
            US1::US1_TomlProject
        } else {
            US1::US1_SpiprojProject
        };
        let v48: Rc<str> = modules_with_schema_5(v47, v39.clone());
        let v49: bool = v26 && v28;
        if v49 {
            let mut v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\n"); } LIT.with(|lit| lit.clone()) };
            v50.borrow_mut().push(v51);
            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union activity_row =\n"); } LIT.with(|lit| lit.clone()) };
            v50.borrow_mut().push(v52);
            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    | StateChange :: string * string * string * string * string * string -> activity_row\n"); } LIT.with(|lit| lit.clone()) };
            v50.borrow_mut().push(v53);
            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    | ActualMinutes :: string * string * i32 * string -> activity_row\n"); } LIT.with(|lit| lit.clone()) };
            v50.borrow_mut().push(v54);
            let v55: Rc<Vec<Rc<str>>> = Rc::new(v50.borrow().clone());
            let v56: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v55).as_ref().clone()));
            let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v58: Rc<str> = Rc::<str>::from((*v56.borrow()).join(&*v57));
            let v59: Rc<str> = Rc::<str>::from(v22.replace(&*v2, &*v3));
            let mut v60: Rc<str> = loop_19(v59);
            let v61: Option<usize> = v60.rfind(&*v3);
            let v62: Option<usize> = method20(v61);
            let mut v63: Rc<dyn Fn((usize)) -> i32> = closure2();
            let mut v64: Option<i32> = v62.map(|x| v63(x));
            let v66: i32 = v64.unwrap_or(-1i32);
            let v67: bool = v66 < 0i32;
            let v71: bool = if v67 {
                true
            } else {
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v69: bool = v60.ends_with(&*v68);
                let v70: bool = v69 == false;
                v70
            };
            let v92: US5 = if v71 {
                US5::US5_OtherFile
            } else {
                let v73: usize = ((0i32) as usize);
                let v74: usize = ((v66) as usize);
                let v75: Rc<str> = Rc::<str>::from(&(v60)[v73..v74]);
                let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v77: bool = v75.ends_with(&*v76);
                if v77 {
                    let v78: Rc<str> = loop_21(v75);
                    US5::US5_StateFile(v78)
                } else {
                    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v81: bool = v75.ends_with(&*v80);
                    if v81 {
                        let v82: Rc<str> = loop_22(v75);
                        US5::US5_HistoryFile(v82)
                    } else {
                        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v85: bool = v75.ends_with(&*v84);
                        if v85 {
                            let v86: Rc<str> = loop_23(v75);
                            US5::US5_ArchiveFile(v86)
                        } else {
                            US5::US5_OtherFile
                        }
                    }
                }
            };
            let v115: US6 = match v92 {
                US5::US5_ArchiveFile(v105) => {
                    let v105: Rc<str> = v105;
                    let v106: Rc<str> = string_concat(&[&*v105, "/state/spiral.toml"]);
                    let v107: bool = std::path::Path::new(&*v106).is_file();
                    if v107 {
                        US6::US6_SymbolsRoot
                    } else {
                        US6::US6_WordsRoot
                    }
                }
                US5::US5_HistoryFile(v99) => {
                    let v99: Rc<str> = v99;
                    let v100: Rc<str> = string_concat(&[&*v99, "/state/spiral.toml"]);
                    let v101: bool = std::path::Path::new(&*v100).is_file();
                    if v101 {
                        US6::US6_SymbolsRoot
                    } else {
                        US6::US6_WordsRoot
                    }
                }
                US5::US5_OtherFile => {
                    US6::US6_WordsRoot
                }
                US5::US5_StateFile(v93) => {
                    let v93: Rc<str> = v93;
                    let v94: Rc<str> = string_concat(&[&*v93, "/state/spiral.toml"]);
                    let v95: bool = std::path::Path::new(&*v94).is_file();
                    if v95 {
                        US6::US6_SymbolsRoot
                    } else {
                        US6::US6_WordsRoot
                    }
                }
            };
            let v127: Rc<str> = match v115 {
                US6::US6_SymbolsRoot => {
                    let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v117: Rc<UH0> = split_list_7(v116.clone(), v58);
                    let v118: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v119: Rc<UH0> = method24(v117, v118);
                    let v120: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v121: i32 = 0i32;
                    let v122: i32 = method18(v120.clone(), v119, v121);
                    let v123: Rc<Vec<Rc<str>>> = Rc::new(v120.borrow().clone());
                    let v124: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v123).as_ref().clone()));
                    let v125: Rc<str> = Rc::<str>::from((*v124.borrow()).join(&*v116));
                    v125
                }
                US6::US6_WordsRoot => {
                    v58
                }
            };
            let v128: bool = std::fs::write(&*v22, &*v127).is_ok();
            if v128 {
                ()
            } else {
                let v129: Rc<str> = string_concat(&["rig state write failed: ", &*v22]);
                std::panic::panic_any::<std::string::String>(format!("{}", v129))
            }
        };
        let v131: bool = if v26 {
            let v130: bool = &*v48 != &*v39;
            v130
        } else {
            false
        };
        if v131 {
            std::fs::write(&*v21, &*v48).unwrap();
            ()
        };
        let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("written"); } LIT.with(|lit| lit.clone()) };
        let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("would-write"); } LIT.with(|lit| lit.clone()) };
        let v135: Rc<str> = step_word_25(v26, v28, v133, v134);
        let v136: bool = &*v48 != &*v39;
        let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("added"); } LIT.with(|lit| lit.clone()) };
        let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("would-add"); } LIT.with(|lit| lit.clone()) };
        let v139: Rc<str> = step_word_25(v26, v136, v137, v138);
        let v142: Rc<str> = match v0 {
            US0::US0_MigrateApply => {
                let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("apply"); } LIT.with(|lit| lit.clone()) };
                v141
            }
            US0::US0_MigratePreview => {
                let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview"); } LIT.with(|lit| lit.clone()) };
                v140
            }
        };
        let v147: Rc<str> = string_concat(&["rig agile migrate-activity ok mode=", &*v142, " schema=", &*v135, " module=", &*v139]);
        println!("{}", v147);
        0i32
    }
}
fn command_line_argument_count_27() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_28(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_26(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_27();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US9 = if v4 {
        let v5: Rc<str> = command_line_argument_28(v0);
        US9::US9_Some(v5)
    } else {
        US9::US9_None
    };
    match v8 {
        US9::US9_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US9::US9_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_26(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn mode_named_29(mut v0: Rc<str>) -> US10 {
    let mut v1: bool = &*v0 == "preview";
    if v1 {
        let mut v2: US0 = US0::US0_MigratePreview;
        US10::US10_Some(v2)
    } else {
        let v4: bool = &*v0 == "apply";
        if v4 {
            let v5: US0 = US0::US0_MigrateApply;
            US10::US10_Some(v5)
        } else {
            US10::US10_None
        }
    }
}
fn migrate_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_26(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH0::UH0_Cons(v8, v9) => {
                                    let v8: Rc<str> = v8;
                                    let mut v9: Rc<UH0> = v9;
                                    let v10: US10 = mode_named_29(v6);
                                    match v10 {
                                        US10::US10_None => {
                                            migrate_usage_1()
                                        }
                                        US10::US10_Some(v11) => {
                                            let v11: US0 = v11;
                                            migrated_2(v11, v8)
                                        }
                                    }
                                }
                                _ => {
                                    migrate_usage_1()
                                }
                            }
                        }
                        _ => {
                            migrate_usage_1()
                        }
                    }
                }
                _ => {
                    migrate_usage_1()
                }
            }
        }
        _ => {
            migrate_usage_1()
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        migrate_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_migrate_activity() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
