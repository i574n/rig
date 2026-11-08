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
enum US0 {
    US0_RemovalReceipt(Rc<str>),
    US0_RemovalRejected(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_RemovedFile,
    US1_RemovedSymlink,
    US1_MissingEntry,
    US1_DirectoryEntry,
}
fn command_line_argument_count_3() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn proxy_argument_count_2() -> i32 {
    let v0: i32 = command_line_argument_count_3();
    let v1: bool = v0 > 0i32;
    if v1 {
        let v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn file_removal_outcome_1() -> US0 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v1: i32 = proxy_argument_count_2();
    let v2: bool = v1 == 3;
    if v2 {
        let v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
        let v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
        let v5: Rc<str> = rig_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v4).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        let v6: bool = &*v5 == "";
        if v6 {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-remove target escapes the root"); } LIT.with(|lit| lit.clone()) };
            US0::US0_RemovalRejected(v7)
        } else {
            let v9: i32 = std::fs::symlink_metadata(&*v5).map(|meta| if meta.file_type().is_symlink() { 1 } else if meta.is_dir() { 3 } else { 0 }).unwrap_or(2);
            let v10: bool = 0i32 == v9;
            let v19: US1 = if v10 {
                US1::US1_RemovedFile
            } else {
                let v12: bool = 1i32 == v9;
                if v12 {
                    US1::US1_RemovedSymlink
                } else {
                    let v14: bool = 3i32 == v9;
                    if v14 {
                        US1::US1_DirectoryEntry
                    } else {
                        US1::US1_MissingEntry
                    }
                }
            };
            match &v19 {
                US1::US1_DirectoryEntry => {
                    let v22: Rc<str> = string_concat(&["fs-remove target is a directory (use fs-remove-tree): ", &*v5]);
                    US0::US0_RemovalRejected(v22)
                }
                US1::US1_MissingEntry => {
                    let v20: Rc<str> = string_concat(&["fs-remove target is missing: ", &*v5]);
                    US0::US0_RemovalRejected(v20)
                }
                _ => {
                    let v24: u64 = std::fs::symlink_metadata(&*v5).map(|meta| meta.len()).unwrap_or(0);
                    let v25: Rc<str> = std::rc::Rc::<str>::from(rig_rust_std_fs::rooted_unlink_nondirectory(std::path::Path::new(&*v5)).err().unwrap_or_default().as_str());
                    let v26: bool = &*v25 == "";
                    if v26 {
                        let v28: Rc<str> = string_concat(&["kit proxy fs-remove ok path=", &*v5, " kind="]);
                        let v32: Rc<str> = match v19 {
                            US1::US1_RemovedFile => {
                                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file"); } LIT.with(|lit| lit.clone()) };
                                v29
                            }
                            US1::US1_RemovedSymlink => {
                                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("symlink"); } LIT.with(|lit| lit.clone()) };
                                v30
                            }
                            _ => unreachable!(),
                        };
                        let v35: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
                        let v38: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                        let v39: Rc<str> = std::rc::Rc::<str>::from((v38).to_string());
                        let v40: Rc<str> = string_concat(&[&*v28, &*v32, " bytes=", &*v35, " elapsed_ms=", &*v39]);
                        US0::US0_RemovalReceipt(v40)
                    } else {
                        let v42: Rc<str> = string_concat(&["fs-remove failed: ", &*v25]);
                        US0::US0_RemovalRejected(v42)
                    }
                }
            }
        }
    } else {
        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-remove expects <root> <relative>"); } LIT.with(|lit| lit.clone()) };
        US0::US0_RemovalRejected(v48)
    }
}
fn file_removal_report_0() -> (i32, Rc<str>) {
    let v0: US0 = file_removal_outcome_1();
    match v0 {
        US0::US0_RemovalReceipt(v1) => {
            let v1: Rc<str> = v1;
            (0i32, v1)
        }
        US0::US0_RemovalRejected(v2) => {
            let v2: Rc<str> = v2;
            (2i32, v2)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (i32, Rc<str>)> = Rc::new(move || -> (i32, Rc<str>) {
        file_removal_report_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_file_removal_outcome() -> (i32, Rc<str>) {
    closure0()()
}
