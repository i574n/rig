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
    US0_Box,
    US0_Node,
}
#[derive(Clone)]
enum US1 {
    US1_Compile,
    US1_Rustc,
    US1_Run,
    US1_SlotWait,
    US1_DotnetStartup,
    US1_Mount,
    US1_LockWait,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US1, u64, Rc<UH1>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<UH1>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US4 {
    US4_Rust,
    US4_Gleam,
    US4_Zig,
    US4_CBackend,
    US4_Lua,
    US4_Wasm,
    US4_Fsharp,
}
#[derive(Clone)]
enum US3 {
    US3_Duration(US4, US1, u64),
    US3_JobDuration(Rc<str>, Rc<str>, u64),
}
#[derive(Clone)]
enum US5 {
    US5_PeakBytes(Rc<str>, u64),
    US5_JobPeak(Rc<str>, Rc<str>, u64),
}
#[derive(Clone)]
enum US6 {
    US6_Count(Rc<str>, u64),
}
#[derive(Clone)]
enum US2 {
    US2_TimedSample(US0, US3),
    US2_SizedSample(US0, US5),
    US2_CountedSample(US0, US6),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US2, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<UH3>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH3> }
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(US4, Rc<UH4>),
}
#[derive(Clone)]
enum US7 {
    US7_Some(US4),
    US7_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn method4(mut v0: US0, mut v1: US4, mut v2: Rc<UH1>, mut v3: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v4, v5, v6) => {
            let v4: US1 = v4;
            let v5: u64 = v5;
            let v6: Rc<UH1> = v6;
            let v7: Rc<UH2> = method4(v0.clone(), v1.clone(), v6, v3);
            let v8: US3 = US3::US3_Duration(v1, v4, v5);
            let mut v9: US2 = US2::US2_TimedSample(v0, v8);
            Rc::new(UH2::UH2_Cons(v9, v7))
        }
        UH1::UH1_Nil => {
            v3
        }
    }
}
fn method6(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH3> = v3;
                let v4: Rc<UH3> = Rc::new(UH3::UH3_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn method7(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH3> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH3::UH3_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method7(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH3> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH3> = Rc::new(UH3::UH3_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH3> = v9.borrow().l1.clone();
        let v17: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        method6(v16, v17)
    }
}
fn method8(mut v0: Rc<str>, mut v1: Rc<UH4>) -> US7 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH4::UH4_Cons(v3, v4) => {
                let v3: US4 = v3;
                let v4: Rc<UH4> = v4;
                let v18: Rc<str> = match &v3 {
                    US4::US4_CBackend => {
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c"); } LIT.with(|lit| lit.clone()) };
                        v8
                    }
                    US4::US4_Fsharp => {
                        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fsharp"); } LIT.with(|lit| lit.clone()) };
                        v11
                    }
                    US4::US4_Gleam => {
                        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
                        v6
                    }
                    US4::US4_Lua => {
                        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lua"); } LIT.with(|lit| lit.clone()) };
                        v9
                    }
                    US4::US4_Rust => {
                        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust"); } LIT.with(|lit| lit.clone()) };
                        v5
                    }
                    US4::US4_Wasm => {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wasm"); } LIT.with(|lit| lit.clone()) };
                        v10
                    }
                    US4::US4_Zig => {
                        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
                        v7
                    }
                };
                let v19: bool = &*v18 == &*v0;
                if v19 {
                    return US7::US7_Some(v3);
                } else {
                    v1 = v4;
                    continue;
                }
            }
            UH4::UH4_Nil => {
                return US7::US7_None;
            }
        }
    }
}
fn cell_samples_3(mut v0: US0, mut v1: Rc<UH1>, mut v2: Rc<str>) -> Rc<UH2> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v5: Rc<UH3> = split_list_5(v3, v4);
    let v10: Rc<str> = match Rc::unwrap_or_clone(v5) {
        UH3::UH3_Cons(v6, v7) => {
            let v6: Rc<str> = v6;
            let v7: Rc<UH3> = v7;
            v6
        }
        UH3::UH3_Nil => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v8
        }
    };
    let v11: US4 = US4::US4_Rust;
    let v12: US4 = US4::US4_Gleam;
    let v13: US4 = US4::US4_Zig;
    let v14: US4 = US4::US4_CBackend;
    let v15: US4 = US4::US4_Lua;
    let v16: US4 = US4::US4_Wasm;
    let v17: US4 = US4::US4_Fsharp;
    let v18: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let v19: Rc<UH4> = Rc::new(UH4::UH4_Cons(v17, v18));
    let v20: Rc<UH4> = Rc::new(UH4::UH4_Cons(v16, v19));
    let v21: Rc<UH4> = Rc::new(UH4::UH4_Cons(v15, v20));
    let v22: Rc<UH4> = Rc::new(UH4::UH4_Cons(v14, v21));
    let v23: Rc<UH4> = Rc::new(UH4::UH4_Cons(v13, v22));
    let v24: Rc<UH4> = Rc::new(UH4::UH4_Cons(v12, v23));
    let v25: Rc<UH4> = Rc::new(UH4::UH4_Cons(v11, v24));
    let v26: US7 = method8(v10, v25);
    match v26 {
        US7::US7_None => {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
        }
        US7::US7_Some(v28) => {
            let v28: US4 = v28;
            let v29: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            method4(v0, v28, v1, v29)
        }
    }
}
fn method9(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: US2 = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH2> = method9(v3, v1);
            Rc::new(UH2::UH2_Cons(v2, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method2(mut v0: US0, mut v1: Rc<UH0>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5) => {
                let v3: Rc<UH1> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: Rc<UH2> = cell_samples_3(v0.clone(), v3, v4);
                let v7: Rc<UH2> = method9(v2, v6);
                (v1, v2) = (v5, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4) => {
                let mut v2: Rc<UH1> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v1));
                (v0, v1) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method14(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_13(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method14(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn records_from_11(mut v0: std::rc::Rc<serde_json::Value>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: bool = v1 >= v2;
        if v4 {
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method12(v3, v5);
        } else {
            let v7: Rc<str> = format_real_13(v1);
            let mut v9: Rc<str> = string_concat(&["/cells/", &*v7, "/metadata"]);
            let v10: Rc<str> = string_concat(&[&*v9, "/spiral_test/targets/0/target"]);
            let v11: Rc<str> = std::rc::Rc::<str>::from(v0.pointer(&*v10).and_then(serde_json::Value::as_str).unwrap_or(""));
            let v12: i32 = v1.wrapping_add(1i32);
            let v13: Rc<str> = string_concat(&[&*v9, "/phases_ms/"]);
            let v14: Rc<str> = string_concat(&[&*v13, "target"]);
            let v15: u64 = v0.pointer(&*v14).and_then(serde_json::Value::as_u64).unwrap_or(0);
            let v16: Rc<str> = string_concat(&[&*v13, "rust_lock_wait"]);
            let v17: u64 = v0.pointer(&*v16).and_then(serde_json::Value::as_u64).unwrap_or(0);
            let v18: Rc<str> = string_concat(&[&*v13, "slot_wait"]);
            let v19: u64 = v0.pointer(&*v18).and_then(serde_json::Value::as_u64).unwrap_or(0);
            let v20: Rc<str> = string_concat(&[&*v13, "mount"]);
            let v21: u64 = v0.pointer(&*v20).and_then(serde_json::Value::as_u64).unwrap_or(0);
            let v22: Rc<str> = string_concat(&[&*v13, "compile"]);
            let v23: u64 = v0.pointer(&*v22).and_then(serde_json::Value::as_u64).unwrap_or(0);
            let v24: US1 = US1::US1_Compile;
            let v25: US1 = US1::US1_Mount;
            let v26: US1 = US1::US1_SlotWait;
            let v27: US1 = US1::US1_LockWait;
            let v28: US1 = US1::US1_Run;
            let v29: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v30: Rc<UH1> = Rc::new(UH1::UH1_Cons(v28, v15, v29));
            let v31: Rc<UH1> = Rc::new(UH1::UH1_Cons(v27, v17, v30));
            let v32: Rc<UH1> = Rc::new(UH1::UH1_Cons(v26, v19, v31));
            let v33: Rc<UH1> = Rc::new(UH1::UH1_Cons(v25, v21, v32));
            let v34: Rc<UH1> = Rc::new(UH1::UH1_Cons(v24, v23, v33));
            let v35: Rc<UH0> = Rc::new(UH0::UH0_Cons(v34, v11, v3));
            (v1, v3) = (v12, v35);
            continue;
        }
    }
}
fn cell_records_of_10(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Option<serde_json::Value> = serde_json::from_str::<serde_json::Value>(&*v0).ok();
    let mut v2: std::rc::Rc<serde_json::Value> = std::rc::Rc::new(v1.unwrap_or_default());
    let v3: i32 = 0i32;
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/cells"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = v2.pointer(&*v4).and_then(serde_json::Value::as_array).map_or(0, |items| items.len() as i32);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    records_from_11(v2, v3, v5, v6)
}
fn notebook_samples_1(mut v0: US0, mut v1: Rc<str>) -> Rc<UH2> {
    let mut v2: Rc<UH0> = cell_records_of_10(v1);
    let v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method2(v0, v2, v3)
}
fn format_real_16(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method14(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method15(mut v0: Rc<UH2>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: US2 = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH3> = method15(v3, v1);
            let v150: Rc<str> = match v2 {
                US2::US2_CountedSample(v126, v127) => {
                    let v126: US0 = v126;
                    let v127: US6 = v127;
                    let v131: Rc<str> = match v126 {
                        US0::US0_Box => {
                            let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("box"); } LIT.with(|lit| lit.clone()) };
                            v128
                        }
                        US0::US0_Node => {
                            let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node"); } LIT.with(|lit| lit.clone()) };
                            v129
                        }
                    };
                    let v133: Rc<str> = string_concat(&["machine=\"", &*v131, "\""]);
                    match v127 {
                        US6::US6_Count(v134, v135) => {
                            let v134: Rc<str> = v134;
                            let v135: u64 = v135;
                            let v137: Rc<str> = string_concat(&["what=\"", &*v134, "\""]);
                            let v138: Rc<str> = format_real_16(v135);
                            let mut v139: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig_exec_count{"); } LIT.with(|lit| lit.clone()) };
                            v139.borrow_mut().push(v140);
                            v139.borrow_mut().push(v133);
                            let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            v139.borrow_mut().push(v141);
                            v139.borrow_mut().push(v137);
                            let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v139.borrow_mut().push(v142);
                            v139.borrow_mut().push(v138);
                            let v143: Rc<Vec<Rc<str>>> = Rc::new(v139.borrow().clone());
                            let v144: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v143).as_ref().clone()));
                            let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v146: Rc<str> = Rc::<str>::from((*v144.borrow()).join(&*v145));
                            v146
                        }
                    }
                }
                US2::US2_SizedSample(v83, v84) => {
                    let v83: US0 = v83;
                    let v84: US5 = v84;
                    let v88: Rc<str> = match v83 {
                        US0::US0_Box => {
                            let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("box"); } LIT.with(|lit| lit.clone()) };
                            v85
                        }
                        US0::US0_Node => {
                            let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node"); } LIT.with(|lit| lit.clone()) };
                            v86
                        }
                    };
                    let v90: Rc<str> = string_concat(&["machine=\"", &*v88, "\""]);
                    match v84 {
                        US5::US5_JobPeak(v104, v105, v106) => {
                            let v104: Rc<str> = v104;
                            let v105: Rc<str> = v105;
                            let v106: u64 = v106;
                            let v108: Rc<str> = string_concat(&["lane=\"", &*v104, "\""]);
                            let v110: Rc<str> = string_concat(&["job=\"", &*v105, "\""]);
                            let mut v111: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            v111.borrow_mut().push(v90);
                            v111.borrow_mut().push(v108);
                            v111.borrow_mut().push(v110);
                            let v112: Rc<Vec<Rc<str>>> = Rc::new(v111.borrow().clone());
                            let v113: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v112).as_ref().clone()));
                            let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            let v115: Rc<str> = Rc::<str>::from((*v113.borrow()).join(&*v114));
                            let v116: Rc<str> = format_real_16(v106);
                            let mut v117: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig_job_peak_bytes{"); } LIT.with(|lit| lit.clone()) };
                            v117.borrow_mut().push(v118);
                            v117.borrow_mut().push(v115);
                            let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v117.borrow_mut().push(v119);
                            v117.borrow_mut().push(v116);
                            let v120: Rc<Vec<Rc<str>>> = Rc::new(v117.borrow().clone());
                            let v121: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v120).as_ref().clone()));
                            let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v123: Rc<str> = Rc::<str>::from((*v121.borrow()).join(&*v122));
                            v123
                        }
                        US5::US5_PeakBytes(v91, v92) => {
                            let v91: Rc<str> = v91;
                            let v92: u64 = v92;
                            let v94: Rc<str> = string_concat(&["what=\"", &*v91, "\""]);
                            let v95: Rc<str> = format_real_16(v92);
                            let mut v96: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig_exec_peak_bytes{"); } LIT.with(|lit| lit.clone()) };
                            v96.borrow_mut().push(v97);
                            v96.borrow_mut().push(v90);
                            let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            v96.borrow_mut().push(v98);
                            v96.borrow_mut().push(v94);
                            let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v96.borrow_mut().push(v99);
                            v96.borrow_mut().push(v95);
                            let v100: Rc<Vec<Rc<str>>> = Rc::new(v96.borrow().clone());
                            let v101: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v100).as_ref().clone()));
                            let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v103: Rc<str> = Rc::<str>::from((*v101.borrow()).join(&*v102));
                            v103
                        }
                    }
                }
                US2::US2_TimedSample(v5, v6) => {
                    let v5: US0 = v5;
                    let v6: US3 = v6;
                    let v10: Rc<str> = match v5 {
                        US0::US0_Box => {
                            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("box"); } LIT.with(|lit| lit.clone()) };
                            v7
                        }
                        US0::US0_Node => {
                            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node"); } LIT.with(|lit| lit.clone()) };
                            v8
                        }
                    };
                    let v12: Rc<str> = string_concat(&["machine=\"", &*v10, "\""]);
                    match v6 {
                        US3::US3_Duration(v13, v14, v15) => {
                            let v13: US4 = v13;
                            let v14: US1 = v14;
                            let v15: u64 = v15;
                            let v29: Rc<str> = match v13 {
                                US4::US4_CBackend => {
                                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c"); } LIT.with(|lit| lit.clone()) };
                                    v19
                                }
                                US4::US4_Fsharp => {
                                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fsharp"); } LIT.with(|lit| lit.clone()) };
                                    v22
                                }
                                US4::US4_Gleam => {
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
                                    v17
                                }
                                US4::US4_Lua => {
                                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lua"); } LIT.with(|lit| lit.clone()) };
                                    v20
                                }
                                US4::US4_Rust => {
                                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust"); } LIT.with(|lit| lit.clone()) };
                                    v16
                                }
                                US4::US4_Wasm => {
                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wasm"); } LIT.with(|lit| lit.clone()) };
                                    v21
                                }
                                US4::US4_Zig => {
                                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
                                    v18
                                }
                            };
                            let v31: Rc<str> = string_concat(&["backend=\"", &*v29, "\""]);
                            let v45: Rc<str> = match v14 {
                                US1::US1_Compile => {
                                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
                                    v32
                                }
                                US1::US1_DotnetStartup => {
                                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dotnet_startup"); } LIT.with(|lit| lit.clone()) };
                                    v36
                                }
                                US1::US1_LockWait => {
                                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lock_wait"); } LIT.with(|lit| lit.clone()) };
                                    v38
                                }
                                US1::US1_Mount => {
                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mount"); } LIT.with(|lit| lit.clone()) };
                                    v37
                                }
                                US1::US1_Run => {
                                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
                                    v34
                                }
                                US1::US1_Rustc => {
                                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustc"); } LIT.with(|lit| lit.clone()) };
                                    v33
                                }
                                US1::US1_SlotWait => {
                                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_wait"); } LIT.with(|lit| lit.clone()) };
                                    v35
                                }
                            };
                            let v47: Rc<str> = string_concat(&["phase=\"", &*v45, "\""]);
                            let mut v48: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            v48.borrow_mut().push(v12);
                            v48.borrow_mut().push(v31);
                            v48.borrow_mut().push(v47);
                            let v49: Rc<Vec<Rc<str>>> = Rc::new(v48.borrow().clone());
                            let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v49).as_ref().clone()));
                            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            let v52: Rc<str> = Rc::<str>::from((*v50.borrow()).join(&*v51));
                            let v53: Rc<str> = format_real_16(v15);
                            let mut v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig_exec_duration_ms{"); } LIT.with(|lit| lit.clone()) };
                            v54.borrow_mut().push(v55);
                            v54.borrow_mut().push(v52);
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v54.borrow_mut().push(v56);
                            v54.borrow_mut().push(v53);
                            let v57: Rc<Vec<Rc<str>>> = Rc::new(v54.borrow().clone());
                            let v58: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v57).as_ref().clone()));
                            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v60: Rc<str> = Rc::<str>::from((*v58.borrow()).join(&*v59));
                            v60
                        }
                        US3::US3_JobDuration(v61, v62, v63) => {
                            let v61: Rc<str> = v61;
                            let v62: Rc<str> = v62;
                            let v63: u64 = v63;
                            let v65: Rc<str> = string_concat(&["lane=\"", &*v61, "\""]);
                            let v67: Rc<str> = string_concat(&["job=\"", &*v62, "\""]);
                            let mut v68: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            v68.borrow_mut().push(v12);
                            v68.borrow_mut().push(v65);
                            v68.borrow_mut().push(v67);
                            let v69: Rc<Vec<Rc<str>>> = Rc::new(v68.borrow().clone());
                            let v70: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v69).as_ref().clone()));
                            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            let v72: Rc<str> = Rc::<str>::from((*v70.borrow()).join(&*v71));
                            let v73: Rc<str> = format_real_16(v63);
                            let mut v74: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig_job_duration_ms{"); } LIT.with(|lit| lit.clone()) };
                            v74.borrow_mut().push(v75);
                            v74.borrow_mut().push(v72);
                            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v74.borrow_mut().push(v76);
                            v74.borrow_mut().push(v73);
                            let v77: Rc<Vec<Rc<str>>> = Rc::new(v74.borrow().clone());
                            let v78: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v77).as_ref().clone()));
                            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v80: Rc<str> = Rc::<str>::from((*v78.borrow()).join(&*v79));
                            v80
                        }
                    }
                }
            };
            Rc::new(UH3::UH3_Cons(v150, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method17(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH3>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH3::UH3_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH3> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH3::UH3_Nil => {
                return v2;
            }
        }
    }
}
fn notebook_prom_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = &*v1 == "node";
    let v5: US0 = if v2 {
        US0::US0_Node
    } else {
        US0::US0_Box
    };
    let v6: Rc<UH2> = notebook_samples_1(v5, v0);
    let v7: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH3> = method15(v6, v7);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v10: i32 = 0i32;
    let v11: i32 = method17(v9.clone(), v8, v10);
    let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
    let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v14));
    v15
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        notebook_prom_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_exec_notebook_prom(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_4fa472513c6d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
