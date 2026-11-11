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
    US0_RecordCatalogComplete,
    US0_RecordCatalogPartial(Rc<str>),
}
fn agile_record_catalog_text_0(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    let v6: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v9: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let v10: Rc<str> = string_concat(&["rig agile records catalog=", &*v0, " present=", &*v6, "/", &*v9]);
    let v11: bool = v1 == v2;
    let v14: US0 = if v11 {
        US0::US0_RecordCatalogComplete
    } else {
        US0::US0_RecordCatalogPartial(v3)
    };
    match v14 {
        US0::US0_RecordCatalogComplete => {
            let v15: Rc<str> = string_concat(&[&*v10, " missing=none"]);
            v15
        }
        US0::US0_RecordCatalogPartial(v16) => {
            let v16: Rc<str> = v16;
            let v19: Rc<str> = string_concat(&[&*v10, " missing=", &*v16, " (agile check does not need them)"]);
            v19
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, i32, i32, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, i32, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>| -> Rc<str> {
        agile_record_catalog_text_0(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_record_catalog_text(v0: &str, v1: i32, v2: i32, v3: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), v1, v2, Rc::<str>::from(v3))
}
