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
fn task_line_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> Rc<str> {
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(v7.replace(&*v10, &*v11));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(v12.replace(&*v13, &*v14));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(v15.replace(&*v16, &*v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(v18.replace(&*v19, &*v20));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = Rc::<str>::from(v21.replace(&*v22, &*v23));
    let v29: Rc<str> = Rc::<str>::from(v2.replace(&*v10, &*v11));
    let v30: Rc<str> = Rc::<str>::from(v29.replace(&*v13, &*v14));
    let v31: Rc<str> = Rc::<str>::from(v30.replace(&*v16, &*v17));
    let v32: Rc<str> = Rc::<str>::from(v31.replace(&*v19, &*v20));
    let v33: Rc<str> = Rc::<str>::from(v32.replace(&*v22, &*v23));
    let v44: Rc<str> = Rc::<str>::from(v0.replace(&*v10, &*v11));
    let v45: Rc<str> = Rc::<str>::from(v44.replace(&*v13, &*v14));
    let v46: Rc<str> = Rc::<str>::from(v45.replace(&*v16, &*v17));
    let v47: Rc<str> = Rc::<str>::from(v46.replace(&*v19, &*v20));
    let v48: Rc<str> = Rc::<str>::from(v47.replace(&*v22, &*v23));
    let v53: Rc<str> = Rc::<str>::from(v1.replace(&*v10, &*v11));
    let v54: Rc<str> = Rc::<str>::from(v53.replace(&*v13, &*v14));
    let v55: Rc<str> = Rc::<str>::from(v54.replace(&*v16, &*v17));
    let v56: Rc<str> = Rc::<str>::from(v55.replace(&*v19, &*v20));
    let v57: Rc<str> = Rc::<str>::from(v56.replace(&*v22, &*v23));
    let v61: Rc<str> = string_concat(&["inl ", &*v6, " () = Task (\"", &*v24, "\", \"", &*v33, "\", ", &*v5, ", ", &*v4, ", ", &*v3, ", 0u32, Planned, \"", &*v48, "\", \"", &*v57, "\", \"current\", \"current\")"]);
    v61
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>| -> Rc<str> {
        task_line_0(v7, v6, v5, v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn add_receipt_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> Rc<str> {
    let v10: Rc<str> = string_concat(&["rig agile add ok id=", &*v6, " binding=", &*v5, " status=Planned progress=0"]);
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" estimate"); } LIT.with(|lit| lit.clone()) };
    let v12: bool = &*v4 == "";
    let v14: Rc<str> = if v12 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    } else {
        v11
    };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" load"); } LIT.with(|lit| lit.clone()) };
    let v16: bool = &*v3 == "";
    let v18: Rc<str> = if v16 {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v17
    } else {
        v15
    };
    let v19: Rc<str> = string_concat(&[&*v14, &*v18]);
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee"); } LIT.with(|lit| lit.clone()) };
    let v21: bool = &*v2 == "";
    let v23: Rc<str> = if v21 {
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v22
    } else {
        v20
    };
    let v24: Rc<str> = string_concat(&[&*v19, &*v23]);
    let v25: Rc<str> = string_concat(&[" deps=", &*v1]);
    let v26: bool = &*v1 == "";
    let v28: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v27
    } else {
        v25
    };
    let v29: bool = &*v24 == "";
    let v36: Rc<str> = if v29 {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v30
    } else {
        let v31: Rc<str> = Rc::<str>::from((v24).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v34: Rc<str> = Rc::<str>::from(v31.replace(&*v32, &*v33));
        let v35: Rc<str> = string_concat(&[" planned=", &*v34]);
        v35
    };
    let v37: Rc<str> = string_concat(&[&*v28, &*v36]);
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" notes=1"); } LIT.with(|lit| lit.clone()) };
    let v39: bool = &*v0 == "";
    let v41: Rc<str> = if v39 {
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v40
    } else {
        v38
    };
    let v43: Rc<str> = string_concat(&[&*v10, &*v37, &*v41]);
    v43
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>| -> Rc<str> {
        add_receipt_1(v6, v5, v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_task_line(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str, v5: &str, v6: &str, v7: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), Rc::<str>::from(v5), Rc::<str>::from(v6), Rc::<str>::from(v7))
}
pub fn rig_agile_add_receipt(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str, v5: &str, v6: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), Rc::<str>::from(v5), Rc::<str>::from(v6))
}
