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
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US0 {
    US0_Some(i32),
    US0_None,
}
fn binary_of_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v0 != "kit";
    let mut v5: bool = if v2 {
        true
    } else {
        let v3: Rc<str> = Rc::<str>::from((v1).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        let mut v4: bool = &*v3 == "";
        v4
    };
    if v5 {
        v0
    } else {
        let mut v6: Rc<str> = Rc::<str>::from((v1).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        v6
    }
}
fn machines_folder_2(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = string_concat(&[&*v3, "/machines"]);
    v4
}
fn file_exists_3(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
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
fn machine_binary_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = machines_folder_2(v2.clone());
    let mut v6: Rc<str> = string_concat(&[&*v3, "/", &*v1, ".bin"]);
    let v7: bool = &*v0 != "kit";
    if v7 {
        let v8: Rc<str> = machines_folder_2(v2);
        let v10: Result<(), std::io::Error> = std::fs::create_dir_all(&*v8);
        std::fs::write(&*v6, &*v0).unwrap();
        v0
    } else {
        let v12: bool = file_exists_3(v6.clone());
        let v25: Rc<str> = if v12 {
            let v14: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v6);
            let v16: Vec<u8> = v14.unwrap();
            let v17: Vec<u8> = method4(v16);
            let v19: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v17);
            let v21: std::string::String = v19.unwrap();
            let v23: Rc<str> = Rc::<str>::from(String::as_str(&v21));
            v23
        } else {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24
        };
        binary_of_1(v0, v25)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> Rc<str> {
        machine_binary_0(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_7(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method8(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method9(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure2() -> Rc<dyn Fn((i32)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US0> = Rc::new(move |mut v0: (i32)| -> US0 {
        let v1: i32 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn connect_seconds_of_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v2: Option<i32> = (v1).trim().parse::<i32>().ok();
    let v3: Option<i32> = method9(v2);
    let mut v4: Rc<dyn Fn((i32)) -> US0> = closure2();
    let mut v5: Option<US0> = v3.map(|x| v4(x));
    let v7: US0 = US0::US0_None;
    let v8: US0 = v7.clone();
    let v9: US0 = v5.unwrap_or(v8);
    match v9 {
        US0::US0_Some(v10) => {
            let v10: i32 = v10;
            let v11: bool = v10 > 0i32;
            if v11 {
                format_real_7(v10)
            } else {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("10"); } LIT.with(|lit| lit.clone()) };
                v13
            }
        }
        _ => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("10"); } LIT.with(|lit| lit.clone()) };
            v15
        }
    }
}
fn machine_connect_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = machines_folder_2(v1);
    let mut v5: Rc<str> = string_concat(&[&*v2, "/", &*v0, ".connect"]);
    let mut v6: bool = file_exists_3(v5.clone());
    let v19: Rc<str> = if v6 {
        let v8: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v5);
        let v10: Vec<u8> = v8.unwrap();
        let v11: Vec<u8> = method4(v10);
        let v13: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v11);
        let v15: std::string::String = v13.unwrap();
        let v17: Rc<str> = Rc::<str>::from(String::as_str(&v15));
        v17
    } else {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v18
    };
    connect_seconds_of_6(v19)
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        machine_connect_5(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_job_machine_binary(v0: &str, v1: &str, v2: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub fn rig_job_machine_connect(v0: &str, v1: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
