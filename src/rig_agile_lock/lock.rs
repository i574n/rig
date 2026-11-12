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
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Ok,
    US1_Error(Rc<str>),
}
fn get_environment_variable_2(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn state_lock_path_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = string_concat(&[&*v0, "/state/agile.spi.lock"]);
    v1
}
fn lock_holder_path_3(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = state_lock_path_4(v0);
    let mut v2: Rc<str> = string_concat(&[&*v1, "/holder"]);
    v2
}
fn method6(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_5(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method6(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn held_by_ancestor_1(mut v0: Rc<str>) -> bool {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AGILE_LOCK_HOLDER"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = get_environment_variable_2(v1);
    let v3: u32 = std::process::id();
    let mut v4: u64 = ((v3) as u64);
    let mut v5: Rc<str> = lock_holder_path_3(v0);
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v5).unwrap_or_default().as_str());
    let mut v7: bool = &*v2 != "";
    let mut v10: bool = if v7 {
        let v8: Rc<str> = format_real_5(v4);
        let v9: bool = &*v2 != &*v8;
        v9
    } else {
        false
    };
    if v10 {
        let mut v11: Rc<str> = string_concat(&[&*v2, " "]);
        let v12: bool = v6.starts_with(&*v11);
        v12
    } else {
        false
    }
}
fn os_lock_path_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from({ let state = std::path::Path::new(&*v0).join("state"); let canonical = std::fs::canonicalize(&state).unwrap_or(state); let mut hash: u64 = 0xcbf29ce484222325; for byte in canonical.to_string_lossy().to_lowercase().bytes() { hash ^= byte as u64; hash = hash.wrapping_mul(0x100000001b3); } let dir = std::env::temp_dir().join("rig-agile-locks"); let _ = std::fs::create_dir_all(&dir); dir.join(format!("{:016x}.lock", hash)).to_string_lossy().into_owned() });
    v1
}
fn os_lock_taken_8(mut v0: Rc<str>, mut v1: u64) -> bool {
    let mut v2: bool = { let deadline = std::time::Instant::now() + std::time::Duration::from_millis(v1); let mark = format!("{} {}", std::process::id(), &*v0); if std::env::var("RIG_AGILE_OSLOCK").ok().as_deref() == Some(mark.as_str()) { true } else { match std::fs::OpenOptions::new().create(true).read(true).write(true).open(&*v0) { Err(_) => true, Ok(file) => loop { match file.try_lock() { Ok(()) => { std::mem::forget(file); unsafe { std::env::set_var("RIG_AGILE_OSLOCK", &mark) }; break true } Err(std::fs::TryLockError::WouldBlock) if std::time::Instant::now() < deadline => std::thread::sleep(std::time::Duration::from_millis(10)), Err(_) => break false } } } } };
    v2
}
fn lock_directory_claimed_10(mut v0: Rc<str>) -> bool {
    let v1: bool = std::fs::create_dir(&*v0).is_ok();
    v1
}
fn lock_holder_text_11(mut v0: u64, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = format_real_5(v0);
    let mut v4: Rc<str> = format_real_5(v1);
    let mut v5: Rc<str> = string_concat(&[&*v2, " ", &*v4]);
    v5
}
fn directory_exists_12(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method13(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn lock_number_16(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v2: Option<u64> = (v1).trim().parse::<u64>().ok();
    let v3: Option<u64> = method17(v2);
    let mut v4: Rc<dyn Fn((u64)) -> US0> = closure2();
    let mut v5: Option<US0> = v3.map(|x| v4(x));
    let mut v7: US0 = US0::US0_None;
    let v8: US0 = v7.clone();
    let v9: US0 = v5.unwrap_or(v8);
    match v9 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v10) => {
            let mut v10: u64 = v10;
            v10
        }
    }
}
fn holder_stale_by_15(mut v0: Rc<dyn Fn(u64) -> bool>, mut v1: Rc<str>, mut v2: u64) -> bool {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Option<usize> = v1.find(&*v3);
    let mut v5: Option<usize> = method13(v4);
    let mut v6: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v7: Option<i32> = v5.map(|x| v6(x));
    let v9: i32 = v7.unwrap_or(-1i32);
    let mut v10: bool = v9 < 0i32;
    if v10 {
        true
    } else {
        let mut v11: usize = ((0i32) as usize);
        let v12: usize = ((v9) as usize);
        let v13: Rc<str> = Rc::<str>::from(&(v1)[v11..v12]);
        let v14: u64 = lock_number_16(v13);
        let v15: i32 = v9.wrapping_add(1i32);
        let v16: i32 = (v1.clone().len() as i32);
        let v17: i32 = v16.wrapping_sub(v9);
        let v18: i32 = v17.wrapping_sub(1i32);
        let v19: usize = ((v15) as usize);
        let mut v20: i32 = v15.wrapping_add(v18);
        let mut v21: usize = ((v20) as usize);
        let v22: Rc<str> = Rc::<str>::from(&(v1)[v19..v21]);
        let v23: u64 = lock_number_16(v22);
        let v24: bool = v14 == 0u64;
        let v27: bool = if v24 {
            true
        } else {
            let v25: u64 = v23.wrapping_add(600000u64);
            let v26: bool = v2 > v25;
            v26
        };
        if v27 {
            true
        } else {
            let v28: bool = v0(v14);
            let v29: bool = v28 == false;
            v29
        }
    }
}
fn closure3() -> Rc<dyn Fn(u64) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64) -> bool> = Rc::new(move |mut v0: u64| -> bool {
        let v1: bool = rig_process_alive::rig_process_alive(v0);
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn lock_holder_stale_14(mut v0: Rc<str>, mut v1: u64) -> bool {
    let mut v2: Rc<dyn Fn(u64) -> bool> = closure3();
    holder_stale_by_15(v2, v0, v1)
}
fn lock_directory_removed_19(mut v0: Rc<str>) -> () {
    { let _ = std::fs::remove_dir(&*v0); };
    ()
}
fn file_exists_20(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn closure4() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> Rc<str>> = Rc::new(move |mut v0: std::io::Error| -> Rc<str> {
        let v1: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v0)).s() });
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method21() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    closure4()
}
fn ok_c5() -> Rc<dyn Fn() -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US1> = Rc::new(move || -> US1 {
        US1::US1_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method22() -> Rc<dyn Fn() -> US1> {
    ok_c5()
}
fn error_c6() -> Rc<dyn Fn(Rc<str>) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US1> = Rc::new(move |mut v0: Rc<str>| -> US1 {
        US1::US1_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method23() -> Rc<dyn Fn(Rc<str>) -> US1> {
    error_c6()
}
fn state_lock_cleared_18(mut v0: Rc<str>) -> () {
    let v1: Rc<str> = lock_holder_path_3(v0.clone());
    let mut v2: bool = file_exists_20(v1.clone());
    let mut v6: Result<(), std::io::Error> = if v2 {
        let mut v4: Result<(), std::io::Error> = std::fs::remove_file(&*v1);
        v4
    } else {
        let mut v5: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
        v5
    };
    let mut v7: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method21();
    let v9: Result<(), Rc<str>> = v6.map_err(|x| v7(x));
    let mut v10: Rc<dyn Fn() -> US1> = method22();
    let mut v11: Rc<dyn Fn(Rc<str>) -> US1> = method23();
    let v12: US1 = match v9 { Ok(()) => v10(), Err(e) => v11(e) };
    match v12 {
        US1::US1_Error(v13) => {
            let v13: Rc<str> = v13;
            let v14: Rc<str> = string_concat(&["file_system.file_delete: ", &*v13]);
            eprintln!("{}", v14);
            ()
        }
        US1::US1_Ok => {
            ()
        }
    };
    let v15: Rc<str> = state_lock_path_4(v0);
    lock_directory_removed_19(v15)
}
fn state_lock_waited_9(mut v0: Rc<str>, mut v1: u64, mut v2: i32) -> bool {
    loop {
        let v3: Rc<str> = state_lock_path_4(v0.clone());
        let mut v4: bool = lock_directory_claimed_10(v3);
        if v4 {
            let mut v5: u32 = std::process::id();
            let mut v6: u64 = ((v5) as u64);
            let mut v7: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
            let v8: Rc<str> = lock_holder_path_3(v0);
            let v9: Rc<str> = lock_holder_text_11(v6, v7);
            std::fs::write(&*v8, &*v9).unwrap();
            return true;
        } else {
            let mut v11: Rc<str> = string_concat(&[&*v0, "/state"]);
            let v12: bool = directory_exists_12(v11);
            let v13: bool = v12 == false;
            if v13 {
                return true;
            } else {
                let v14: Rc<str> = lock_holder_path_3(v0.clone());
                let v15: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v14).unwrap_or_default().as_str());
                let v16: bool = &*v15 == "";
                let v25: bool = if v16 {
                    true
                } else {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v18: Option<usize> = v15.find(&*v17);
                    let v19: Option<usize> = method13(v18);
                    let mut v20: Rc<dyn Fn((usize)) -> i32> = closure1();
                    let mut v21: Option<i32> = v19.map(|x| v20(x));
                    let v23: i32 = v21.unwrap_or(-1i32);
                    let v24: bool = v23 < 0i32;
                    v24
                };
                let v27: bool = if v25 {
                    let v26: bool = v2 < 400i32;
                    v26
                } else {
                    false
                };
                if v27 {
                    std::thread::sleep(std::time::Duration::from_millis(25u64));
                    let v28: i32 = v2.wrapping_add(1i32);
                    v2 = v28;
                    continue;
                } else {
                    let v30: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                    let v31: bool = lock_holder_stale_14(v15, v30);
                    if v31 {
                        state_lock_cleared_18(v0.clone());
                        let v32: i32 = 0i32;
                        v2 = v32;
                        continue;
                    } else {
                        let v34: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                        let v35: bool = v34 >= v1;
                        if v35 {
                            return false;
                        } else {
                            std::thread::sleep(std::time::Duration::from_millis(25u64));
                            let v36: i32 = 0i32;
                            v2 = v36;
                            continue;
                        }
                    }
                }
            }
        }
    }
}
fn set_environment_variable_24(mut v0: Rc<str>, mut v1: Rc<str>) -> () {
    unsafe { std::env::set_var(&*v0, &*v1) };
    ()
}
fn lock_taken_0(mut v0: Rc<str>) -> bool {
    let v1: bool = held_by_ancestor_1(v0.clone());
    if v1 {
        true
    } else {
        let mut v2: Rc<str> = os_lock_path_7(v0.clone());
        let v3: u64 = 300000u64;
        let mut v4: bool = os_lock_taken_8(v2, v3);
        let mut v5: bool = v4 == false;
        if v5 {
            false
        } else {
            let mut v6: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
            let mut v7: u64 = v6.wrapping_add(300000u64);
            let v8: i32 = 0i32;
            let v9: bool = state_lock_waited_9(v0, v7, v8);
            if v9 {
                let mut v10: u32 = std::process::id();
                let mut v11: u64 = ((v10) as u64);
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AGILE_LOCK_HOLDER"); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = format_real_5(v11);
                set_environment_variable_24(v12, v13);
                true
            } else {
                false
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        lock_taken_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn owns_holder_26(mut v0: u64, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = format_real_5(v0);
    let v3: Rc<str> = string_concat(&[&*v2, " "]);
    let mut v4: bool = v1.starts_with(&*v3);
    v4
}
fn lock_released_25(mut v0: Rc<str>) -> bool {
    let v1: bool = held_by_ancestor_1(v0.clone());
    if v1 {
        true
    } else {
        let mut v2: u32 = std::process::id();
        let v3: u64 = ((v2) as u64);
        let mut v4: Rc<str> = lock_holder_path_3(v0.clone());
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v4).unwrap_or_default().as_str());
        let mut v6: bool = owns_holder_26(v3, v5);
        if v6 {
            state_lock_cleared_18(v0)
        };
        true
    }
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        lock_released_25(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_lock_taken(v0: &str) -> bool {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_agile_lock_released(v0: &str) -> bool {
    closure7()(Rc::<str>::from(v0))
}
use rig_process_alive::SpiralShape_97df63ebabc3 as Mut0;
