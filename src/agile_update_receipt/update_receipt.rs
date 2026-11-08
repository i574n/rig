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
fn agile_clock_ms_0() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    v0
}
fn closure0() -> Rc<dyn Fn() -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> u64> = Rc::new(move || -> u64 {
        agile_clock_ms_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_set_receipt_text_1(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64) -> Rc<str> {
    let v8: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v13: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v5);
    let v14: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v17: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let v20: Rc<str> = std::rc::Rc::<str>::from((v13).to_string());
    let v22: Rc<str> = string_concat(&["kit agile set ok id=", &*v0, " progress=", &*v8, " status=", &*v2, " validation=runtime-attested compiler=not-required bytes_before=", &*v14, " bytes_after=", &*v17, " elapsed_ms=", &*v20]);
    v22
}
fn closure1() -> Rc<dyn Fn(Rc<str>, i32, Rc<str>, u64, u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, Rc<str>, u64, u64, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64| -> Rc<str> {
        agile_set_receipt_text_1(v0, v1, v2, v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_clock_ms() -> u64 {
    closure0()()
}
pub fn rig_agile_set_receipt_text(v0: &str, v1: i32, v2: &str, v3: u64, v4: u64, v5: u64) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2), v3, v4, v5)
}
