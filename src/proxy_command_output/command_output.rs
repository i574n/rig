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
    US0_ChildSucceeded,
    US0_ChildFailed(i32),
}
fn command_output_receipt_text_0(mut v0: i32, mut v1: u64, mut v2: u64, mut v3: u64) -> Rc<str> {
    let v4: bool = v0 == 0i32;
    let v7: US0 = if v4 {
        US0::US0_ChildSucceeded
    } else {
        US0::US0_ChildFailed(v0)
    };
    let v12: Rc<str> = match v7 {
        US0::US0_ChildFailed(v9) => {
            let v9: i32 = v9;
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("child-failed"); } LIT.with(|lit| lit.clone()) };
            v10
        }
        US0::US0_ChildSucceeded => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v8
        }
    };
    let v15: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v18: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let v21: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v23: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v25: Rc<str> = string_concat(&["kit proxy command-output ", &*v12, " status=", &*v15, " elapsed_ms=", &*v23, " stdout_bytes=", &*v18, " stderr_bytes=", &*v21]);
    v25
}
fn closure0() -> Rc<dyn Fn(i32, u64, u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, u64, u64, u64) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: u64, mut v2: u64, mut v3: u64| -> Rc<str> {
        command_output_receipt_text_0(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_command_output_receipt(v0: i32, v1: u64, v2: u64, v3: u64) -> Rc<str> {
    closure0()(v0, v1, v2, v3)
}
