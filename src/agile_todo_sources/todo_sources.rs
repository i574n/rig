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
fn walked_names_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: i32 = (v2.clone().len() as i32);
        let v6: bool = v5 <= v3;
        if v6 {
            return v4;
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v2.get(v3 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v3 as usize) as i32).unwrap_or(-1);
            let v9: bool = v8 < 0i32;
            let v10: i32 = if v9 {
                v5
            } else {
                v8
            };
            let v11: Rc<str> = std::rc::Rc::<str>::from(v2.get(v3 as usize..v10 as usize).unwrap_or(""));
            let v12: i32 = v10.wrapping_add(1i32);
            let v13: bool = &*v0 == "";
            let v16: Rc<str> = if v13 {
                v11.clone()
            } else {
                let v15: Rc<str> = string_concat(&[&*v0, "/", &*v11]);
                v15
            };
            let v17: bool = &*v1 == "";
            let v20: Rc<str> = if v17 {
                v11.clone()
            } else {
                let v19: Rc<str> = string_concat(&[&*v1, "/", &*v11]);
                v19
            };
            let v21: bool = &*v11 == "";
            let v43: Rc<str> = if v21 {
                v4
            } else {
                let v22: i32 = std::fs::symlink_metadata(&*v16).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
                let v23: bool = v22 == 2i32;
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules|vendor|third_party|target|dist|_build|.zig-cache|zig-out|.lake|__pycache__|.venv|venv|.gradle|.next|bin|obj|deps|.git|.cache|elm-stuff|bower_components|.dart_tool|.mypy_cache|.pytest_cache|.tox"); } LIT.with(|lit| lit.clone()) };
                let v25: bool = v24.split("|").any(|item| item == &*v11);
                let v26: bool = v23 && v25;
                if v26 {
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                    let v28: bool = &*v20 == "";
                    let v31: Rc<str> = if v28 {
                        v27
                    } else {
                        let v30: Rc<str> = string_concat(&[&*v20, "/", &*v27]);
                        v30
                    };
                    let v33: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                    let v34: Rc<str> = string_concat(&[&*v4, &*v31, &*v33]);
                    v34
                } else {
                    let v35: i32 = std::fs::symlink_metadata(&*v16).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
                    let v36: bool = v35 == 2i32;
                    if v36 {
                        walked_directory_1(v16, v20, v4)
                    } else {
                        let v39: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                        let v40: Rc<str> = string_concat(&[&*v4, &*v20, &*v39]);
                        v40
                    }
                }
            };
            (v3, v4) = (v12, v43);
            continue;
        }
    }
}
fn walked_directory_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = std::rc::Rc::<str>::from(rig_rust_std_fs::rooted_list_names(std::path::Path::new(&*v0)).unwrap_or_default().join(&char::from(10u8).to_string()).as_str());
    let v4: i32 = 0i32;
    walked_names_2(v0, v1, v3, v4, v2)
}
fn todo_sources_0(mut v0: Rc<str>) -> (bool, Rc<str>) {
    let v1: i32 = std::fs::symlink_metadata(&*v0).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
    let v2: bool = v1 == 2i32;
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rev-parse\t--is-inside-work-tree"); } LIT.with(|lit| lit.clone()) };
        let (mut v4, mut v5): (bool, Rc<str>) = { let mut command = std::process::Command::new("git"); command.arg("-C").arg(&*v0).args(v3.split(char::from(9u8))); match rig_process::run_bounded_capture_with_input(&mut command, 60000u64, None) { Ok(capture) => (capture.termination == rig_process::ProcessTermination::Exited(0), std::rc::Rc::<str>::from(String::from_utf8_lossy(&capture.stdout).as_ref())), Err(error) => (false, std::rc::Rc::<str>::from(error)) } };
        if v4 {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ls-files\t-co\t--exclude-standard\t-z"); } LIT.with(|lit| lit.clone()) };
            let (mut v7, mut v8): (bool, Rc<str>) = { let mut command = std::process::Command::new("git"); command.arg("-C").arg(&*v0).args(v6.split(char::from(9u8))); match rig_process::run_bounded_capture_with_input(&mut command, 60000u64, None) { Ok(capture) => (capture.termination == rig_process::ProcessTermination::Exited(0), std::rc::Rc::<str>::from(String::from_utf8_lossy(&capture.stdout).as_ref())), Err(error) => (false, std::rc::Rc::<str>::from(error)) } };
            (v7, v8)
        } else {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = walked_directory_1(v0, v9.clone(), v9.clone());
            (true, v10)
        }
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, v13)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (bool, Rc<str>) {
        todo_sources_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn git_head_3(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rev-parse\tHEAD"); } LIT.with(|lit| lit.clone()) };
    let (mut v2, mut v3): (bool, Rc<str>) = { let mut command = std::process::Command::new("git"); command.arg("-C").arg(&*v0).args(v1.split(char::from(9u8))); match rig_process::run_bounded_capture_with_input(&mut command, 60000u64, None) { Ok(capture) => (capture.termination == rig_process::ProcessTermination::Exited(0), std::rc::Rc::<str>::from(String::from_utf8_lossy(&capture.stdout).as_ref())), Err(error) => (false, std::rc::Rc::<str>::from(error)) } };
    if v2 {
        let v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        v4
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        git_head_3(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn git_changed_since_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = string_concat(&["diff\t--name-only\t--relative\t-z\t", &*v1]);
    let (mut v3, mut v4): (bool, Rc<str>) = { let mut command = std::process::Command::new("git"); command.arg("-C").arg(&*v0).args(v2.split(char::from(9u8))); match rig_process::run_bounded_capture_with_input(&mut command, 60000u64, None) { Ok(capture) => (capture.termination == rig_process::ProcessTermination::Exited(0), std::rc::Rc::<str>::from(String::from_utf8_lossy(&capture.stdout).as_ref())), Err(error) => (false, std::rc::Rc::<str>::from(error)) } };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ls-files\t-o\t--exclude-standard\t-z"); } LIT.with(|lit| lit.clone()) };
    let (mut v6, mut v7): (bool, Rc<str>) = { let mut command = std::process::Command::new("git"); command.arg("-C").arg(&*v0).args(v5.split(char::from(9u8))); match rig_process::run_bounded_capture_with_input(&mut command, 60000u64, None) { Ok(capture) => (capture.termination == rig_process::ProcessTermination::Exited(0), std::rc::Rc::<str>::from(String::from_utf8_lossy(&capture.stdout).as_ref())), Err(error) => (false, std::rc::Rc::<str>::from(error)) } };
    let v8: bool = v3 && v6;
    if v8 {
        let v9: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
        let v11: Rc<str> = string_concat(&[&*v9, &*v4, &*v7]);
        v11
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        git_changed_since_4(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_todo_sources(v0: &str) -> (bool, Rc<str>) {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_agile_todo_git_head(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
pub fn rig_agile_todo_git_changed(v0: &str, v1: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
