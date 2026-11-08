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
pub struct Mut0 { pub l0: Rc<str> }
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
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn state_lock_path_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = string_concat(&[&*v0, "/state/agile.spi.lock"]);
    v1
}
fn lock_holder_path_3(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = state_lock_path_4(v0);
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
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method6(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn held_by_ancestor_1(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AGILE_LOCK_HOLDER"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = get_environment_variable_2(v1);
    let v3: u32 = std::process::id();
    let v4: u64 = ((v3) as u64);
    let v5: Rc<str> = lock_holder_path_3(v0);
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
        let mut v12: bool = v6.starts_with(&*v11);
        v12
    } else {
        false
    }
}
fn lock_directory_claimed_8(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::create_dir(&*v0).is_ok();
    v1
}
fn lock_holder_text_9(mut v0: u64, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = format_real_5(v0);
    let v4: Rc<str> = format_real_5(v1);
    let v5: Rc<str> = string_concat(&[&*v2, " ", &*v4]);
    v5
}
fn directory_exists_10(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
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
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method16(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
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
fn method17(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn lock_number_14(mut v0: Rc<str>) -> u64 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method15(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0, v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method16(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let v9: Option<u64> = (v8).trim().parse::<u64>().ok();
    let mut v10: Option<u64> = method17(v9);
    let mut v11: Rc<dyn Fn((u64)) -> US0> = closure2();
    let mut v12: Option<US0> = v10.map(|x| v11(x));
    let v14: US0 = US0::US0_None;
    let v15: US0 = v14.clone();
    let v16: US0 = v12.unwrap_or(v15);
    match v16 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v17) => {
            let v17: u64 = v17;
            v17
        }
    }
}
fn holder_stale_by_12(mut v0: Rc<dyn Fn(u64) -> bool>, mut v1: Rc<str>, mut v2: u64) -> bool {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v4: Option<usize> = v1.find(&*v3);
    let v5: Option<usize> = method13(v4);
    let mut v6: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v7: Option<i32> = v5.map(|x| v6(x));
    let v9: i32 = v7.unwrap_or(-1i32);
    let mut v10: bool = v9 < 0i32;
    if v10 {
        true
    } else {
        let mut v11: usize = ((0i32) as usize);
        let mut v12: usize = ((v9) as usize);
        let v13: Rc<str> = Rc::<str>::from(&(v1)[v11..v12]);
        let v14: u64 = lock_number_14(v13);
        let v15: i32 = v9.wrapping_add(1i32);
        let v16: i32 = (v1.clone().len() as i32);
        let v17: i32 = v16.wrapping_sub(v9);
        let v18: i32 = v17.wrapping_sub(1i32);
        let v19: usize = ((v15) as usize);
        let v20: i32 = v15.wrapping_add(v18);
        let v21: usize = ((v20) as usize);
        let v22: Rc<str> = Rc::<str>::from(&(v1)[v19..v21]);
        let v23: u64 = lock_number_14(v22);
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
        let mut v1: bool = rig_process_alive::rig_process_alive(v0);
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn lock_holder_stale_11(mut v0: Rc<str>, mut v1: u64) -> bool {
    let mut v2: Rc<dyn Fn(u64) -> bool> = closure3();
    holder_stale_by_12(v2, v0, v1)
}
fn lock_directory_removed_19(mut v0: Rc<str>) -> () {
    { let _ = std::fs::remove_dir(&*v0); };
    ()
}
fn file_exists_20(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
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
        let mut v1: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v0)).s() });
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
    let mut v1: Rc<str> = lock_holder_path_3(v0.clone());
    let mut v2: bool = file_exists_20(v1.clone());
    let mut v6: Result<(), std::io::Error> = if v2 {
        let v4: Result<(), std::io::Error> = std::fs::remove_file(&*v1);
        v4
    } else {
        let v5: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
        v5
    };
    let mut v7: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method21();
    let v9: Result<(), Rc<str>> = v6.map_err(|x| v7(x));
    let mut v10: Rc<dyn Fn() -> US1> = method22();
    let mut v11: Rc<dyn Fn(Rc<str>) -> US1> = method23();
    let mut v12: US1 = match v9 { Ok(()) => v10(), Err(e) => v11(e) };
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
fn state_lock_waited_7(mut v0: Rc<str>, mut v1: u64, mut v2: i32) -> bool {
    loop {
        let v3: Rc<str> = state_lock_path_4(v0.clone());
        let v4: bool = lock_directory_claimed_8(v3);
        if v4 {
            let v5: u32 = std::process::id();
            let mut v6: u64 = ((v5) as u64);
            let mut v7: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
            let v8: Rc<str> = lock_holder_path_3(v0);
            let v9: Rc<str> = lock_holder_text_9(v6, v7);
            std::fs::write(&*v8, &*v9).unwrap();
            return true;
        } else {
            let mut v11: Rc<str> = string_concat(&[&*v0, "/state"]);
            let mut v12: bool = directory_exists_10(v11);
            let v13: bool = v12 == false;
            if v13 {
                return true;
            } else {
                let v14: Rc<str> = lock_holder_path_3(v0.clone());
                let v15: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v14).unwrap_or_default().as_str());
                let v16: bool = &*v15 == "";
                let v18: bool = if v16 {
                    let v17: bool = v2 < 400i32;
                    v17
                } else {
                    false
                };
                if v18 {
                    std::thread::sleep(std::time::Duration::from_millis(25u64));
                    let v19: i32 = v2.wrapping_add(1i32);
                    v2 = v19;
                    continue;
                } else {
                    let v21: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                    let v22: bool = lock_holder_stale_11(v15, v21);
                    if v22 {
                        state_lock_cleared_18(v0.clone());
                        let v23: i32 = 0i32;
                        v2 = v23;
                        continue;
                    } else {
                        let v25: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                        let v26: bool = v25 >= v1;
                        if v26 {
                            return false;
                        } else {
                            std::thread::sleep(std::time::Duration::from_millis(25u64));
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
    let mut v1: bool = held_by_ancestor_1(v0.clone());
    if v1 {
        true
    } else {
        let mut v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let v3: u64 = v2.wrapping_add(300000u64);
        let v4: i32 = 0i32;
        let v5: bool = state_lock_waited_7(v0, v3, v4);
        if v5 {
            let mut v6: u32 = std::process::id();
            let mut v7: u64 = ((v6) as u64);
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AGILE_LOCK_HOLDER"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = format_real_5(v7);
            set_environment_variable_24(v8, v9);
            true
        } else {
            false
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        lock_taken_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn lock_released_25(mut v0: Rc<str>) -> bool {
    let mut v1: bool = held_by_ancestor_1(v0.clone());
    if v1 {
        true
    } else {
        state_lock_cleared_18(v0);
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
pub type SpiralShape_97df63ebabc3 = Mut0;
