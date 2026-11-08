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
fn file_clock_ms_0() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    v0
}
fn closure0() -> Rc<dyn Fn() -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> u64> = Rc::new(move || -> u64 {
        file_clock_ms_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn elapsed_field_1(mut v0: u64) -> Rc<str> {
    let v1: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
    let v2: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v3: Rc<str> = string_concat(&[" elapsed_ms=", &*v2]);
    v3
}
fn closure1() -> Rc<dyn Fn(u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64) -> Rc<str>> = Rc::new(move |mut v0: u64| -> Rc<str> {
        elapsed_field_1(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_file_clock_ms() -> u64 {
    closure0()()
}
pub fn rig_elapsed_field(v0: u64) -> Rc<str> {
    closure1()(v0)
}
