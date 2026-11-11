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
    US0_WindowFine,
    US0_WindowStale(Rc<str>),
}
fn window_health_of_1(mut v0: bool, mut v1: Rc<str>) -> US0 {
    let v3: bool = if v0 {
        let v2: bool = &*v1 != "";
        v2
    } else {
        false
    };
    let v5: bool = if v3 {
        let v4: bool = &*v1 != "running";
        v4
    } else {
        false
    };
    let v7: bool = if v5 {
        let v6: bool = &*v1 != "queued";
        v6
    } else {
        false
    };
    if v7 {
        US0::US0_WindowStale(v1)
    } else {
        US0::US0_WindowFine
    }
}
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn windows_watched_0(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (&*v0).lines().filter(|line| !line.is_empty()).count() as i32;
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let v5: Rc<str> = std::rc::Rc::<str>::from((&*v0).lines().filter(|line| !line.is_empty()).nth(v1 as usize).unwrap_or(""));
            let v6: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(3i32 as usize).unwrap_or(""));
            let v7: Rc<str> = std::rc::Rc::<str>::from((&*v6).split("(kit job ").nth(1).and_then(|rest| rest.split(char::from(41u8)).next()).unwrap_or(""));
            let v8: bool = &*v7 == "";
            let v11: Rc<str> = if v8 {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v9
            } else {
                let v10: Rc<str> = rig_job_control::rig_job_state_word(&*v7);
                v10
            };
            let v12: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(0i32 as usize).unwrap_or(""));
            let v15: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(1i32 as usize).unwrap_or(""));
            let v18: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(2i32 as usize).unwrap_or(""));
            let v21: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(3i32 as usize).unwrap_or(""));
            let v22: Rc<str> = string_concat(&["  ", &*v12, "  ", &*v15, "  ", &*v18, "  ", &*v21]);
            let v23: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(2i32 as usize).unwrap_or(""));
            let v24: bool = &*v23 == "held";
            let v25: US0 = window_health_of_1(v24, v11);
            let v32: Rc<str> = match v25 {
                US0::US0_WindowFine => {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v26
                }
                US0::US0_WindowStale(v27) => {
                    let v27: Rc<str> = v27;
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  <- STALE: job is "); } LIT.with(|lit| lit.clone()) };
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_2(v28, v27, v29.clone(), v29.clone(), v29.clone())
                }
            };
            let v33: Rc<str> = string_concat(&[&*v22, &*v32]);
            let v34: i32 = v1.wrapping_add(1i32);
            let v35: bool = &*v2 == "";
            let v38: Rc<str> = if v35 {
                v33
            } else {
                let v37: Rc<str> = string_concat(&[&*v2, "\n", &*v33]);
                v37
            };
            (v1, v2) = (v34, v38);
            continue;
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: i32 = 0i32;
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        windows_watched_0(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_machine_windows_lines(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
