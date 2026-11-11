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
    US0_LinkNotInPackages(Rc<str>, Rc<str>),
    US0_LinkWithoutCargoDep(Rc<str>, Rc<str>),
    US0_HostCallDep(Rc<str>, Rc<str>),
    US0_MemberMissing(Rc<str>),
    US0_MemberWithoutUnit(Rc<str>),
    US0_KindEdge(Rc<str>, Rc<str>),
    US0_EntryNotNamedAfterUnit(Rc<str>, Rc<str>),
    US0_DuplicateModule(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(US0, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Plain,
    US1_Model,
    US1_Boundary,
    US1_Feature,
    US1_Bin,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<UH2>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, US1, Rc<UH2>, Rc<UH2>, Rc<UH2>, Rc<str>, Rc<UH2>, bool, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>, US1, Rc<UH2>, Rc<UH2>, Rc<UH2>, Rc<str>, Rc<UH2>, bool),
    US3_None,
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<str>, Rc<UH3>),
}
fn method2(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: US0 = v2;
                let v3: Rc<UH0> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn finding_line_4(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_DuplicateModule(v61, v62) => {
            let v61: Rc<str> = v61;
            let v62: Rc<str> = v62;
            let v65: Rc<str> = string_concat(&["units / ", &*v61, " duplicate-module"]);
            let v66: bool = &*v62 == "";
            let v69: Rc<str> = if v66 {
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v67
            } else {
                let v68: Rc<str> = string_concat(&[" ", &*v62]);
                v68
            };
            let v70: Rc<str> = string_concat(&[&*v65, &*v69]);
            v70
        }
        US0::US0_EntryNotNamedAfterUnit(v51, v52) => {
            let v51: Rc<str> = v51;
            let v52: Rc<str> = v52;
            let v55: Rc<str> = string_concat(&["units / ", &*v51, " entry-not-named-after-unit"]);
            let v56: bool = &*v52 == "";
            let v59: Rc<str> = if v56 {
                let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v57
            } else {
                let v58: Rc<str> = string_concat(&[" ", &*v52]);
                v58
            };
            let v60: Rc<str> = string_concat(&[&*v55, &*v59]);
            v60
        }
        US0::US0_HostCallDep(v21, v22) => {
            let v21: Rc<str> = v21;
            let v22: Rc<str> = v22;
            let v25: Rc<str> = string_concat(&["units / ", &*v21, " host-call-dep"]);
            let v26: bool = &*v22 == "";
            let v29: Rc<str> = if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v27
            } else {
                let v28: Rc<str> = string_concat(&[" ", &*v22]);
                v28
            };
            let v30: Rc<str> = string_concat(&[&*v25, &*v29]);
            v30
        }
        US0::US0_KindEdge(v41, v42) => {
            let v41: Rc<str> = v41;
            let v42: Rc<str> = v42;
            let v45: Rc<str> = string_concat(&["units / ", &*v41, " kind-edge"]);
            let v46: bool = &*v42 == "";
            let v49: Rc<str> = if v46 {
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v47
            } else {
                let v48: Rc<str> = string_concat(&[" ", &*v42]);
                v48
            };
            let v50: Rc<str> = string_concat(&[&*v45, &*v49]);
            v50
        }
        US0::US0_LinkNotInPackages(v1, v2) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            let v5: Rc<str> = string_concat(&["units / ", &*v1, " link-not-in-packages"]);
            let mut v6: bool = &*v2 == "";
            let mut v9: Rc<str> = if v6 {
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v7
            } else {
                let v8: Rc<str> = string_concat(&[" ", &*v2]);
                v8
            };
            let v10: Rc<str> = string_concat(&[&*v5, &*v9]);
            v10
        }
        US0::US0_LinkWithoutCargoDep(v11, v12) => {
            let v11: Rc<str> = v11;
            let v12: Rc<str> = v12;
            let v15: Rc<str> = string_concat(&["units / ", &*v11, " link-without-cargo-dep"]);
            let v16: bool = &*v12 == "";
            let v19: Rc<str> = if v16 {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v17
            } else {
                let v18: Rc<str> = string_concat(&[" ", &*v12]);
                v18
            };
            let v20: Rc<str> = string_concat(&[&*v15, &*v19]);
            v20
        }
        US0::US0_MemberMissing(v31) => {
            let v31: Rc<str> = v31;
            let v35: Rc<str> = string_concat(&["units / ", &*v31, " member-missing"]);
            v35
        }
        US0::US0_MemberWithoutUnit(v36) => {
            let v36: Rc<str> = v36;
            let v40: Rc<str> = string_concat(&["units / ", &*v36, " member-without-unit"]);
            v40
        }
    }
}
fn method3(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: US0 = v1;
                let mut v2: Rc<UH0> = v2;
                let v3: Rc<str> = finding_line_4(v1);
                println!("{}", v3);
                v0 = v2;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn hard_6(mut v0: US0) -> bool {
    match v0 {
        US0::US0_HostCallDep(v1, v2) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            false
        }
        _ => {
            true
        }
    }
}
fn method5(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: US0 = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method5(v3, v1);
            let v5: bool = hard_6(v2.clone());
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn hard_printed_1(mut v0: Rc<UH0>) -> i32 {
    method3(v0.clone());
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH0> = method5(v0, v1);
    let v3: i32 = 0i32;
    method2(v2, v3)
}
fn kind_of_word_10(mut v0: Rc<str>) -> US1 {
    let mut v1: bool = &*v0 == "model";
    if v1 {
        US1::US1_Model
    } else {
        let v3: bool = &*v0 == "boundary";
        if v3 {
            US1::US1_Boundary
        } else {
            let v5: bool = &*v0 == "feature";
            if v5 {
                US1::US1_Feature
            } else {
                let v7: bool = &*v0 == "bin";
                if v7 {
                    US1::US1_Bin
                } else {
                    US1::US1_Plain
                }
            }
        }
    }
}
fn list_from_base_12(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: Rc<UH2> = Rc::new(UH2::UH2_Cons(v5, v2));
            (v1, v2) = (v4, v6);
            continue;
        }
    }
}
fn list_of_base_11(mut v0: &Rc<RefCell<Vec<Rc<str>>>>) -> Rc<UH2> {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    list_from_base_12(v0, v2, v3)
}
fn unit_of_row_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: &Rc<RefCell<Vec<Rc<str>>>>, mut v3: &Rc<RefCell<Vec<Rc<str>>>>, mut v4: &Rc<RefCell<Vec<Rc<str>>>>, mut v5: Rc<str>, mut v6: &Rc<RefCell<Vec<Rc<str>>>>, mut v7: bool) -> (Rc<str>, US1, Rc<UH2>, Rc<UH2>, Rc<UH2>, Rc<str>, Rc<UH2>, bool) {
    let v8: US1 = kind_of_word_10(v1);
    let mut v9: Rc<UH2> = list_of_base_11(v2);
    let v10: Rc<UH2> = list_of_base_11(v3);
    let v11: Rc<UH2> = list_of_base_11(v4);
    let v12: Rc<UH2> = list_of_base_11(v6);
    (v0, v8, v9, v10, v11, v5, v12, v7)
}
fn units_from_8(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool) = v0.borrow()[v1 as usize].clone();
            let (mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20): (Rc<str>, US1, Rc<UH2>, Rc<UH2>, Rc<UH2>, Rc<str>, Rc<UH2>, bool) = unit_of_row_9(v5, v6, &v7, &v8, &v9, v10, &v11, v12);
            let v21: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v14, v15, v16, v17, v18, v19, v20, v2));
            (v1, v2) = (v4, v21);
            continue;
        }
    }
}
fn units_of_7(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>>) -> Rc<UH1> {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    units_from_8(v0, v2, v3)
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: US0 = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method14(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method17(mut v0: Rc<str>, mut v1: US1, mut v2: &Rc<UH2>, mut v3: &Rc<UH2>, mut v4: &Rc<UH2>, mut v5: &Rc<str>, mut v6: &Rc<UH2>, mut v7: bool, mut v8: Rc<UH2>, mut v9: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v8) {
        UH2::UH2_Cons(v10, v11) => {
            let v10: Rc<str> = v10;
            let v11: Rc<UH2> = v11;
            let v12: Rc<UH0> = method17(v0.clone(), v1, v2, v3, v4, v5, v6, v7, v11, v9);
            let v13: US0 = US0::US0_DuplicateModule(v0, v10);
            Rc::new(UH0::UH0_Cons(v13, v12))
        }
        UH2::UH2_Nil => {
            v9
        }
    }
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH2>) -> US2> {
    Rc::new(move |mut v2: Rc<UH2>| -> US2 {
        match Rc::unwrap_or_clone(v2) {
            UH2::UH2_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH2> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH2>) -> US2> = method19(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH2::UH2_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method19(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH2>) -> US2> {
    closure1(v0, v1)
}
fn unit_named_20(mut v0: Rc<UH1>, mut v1: Rc<str>) -> US3 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let v3: Rc<str> = v3;
                let v4: US1 = v4;
                let v5: Rc<UH2> = v5;
                let mut v6: Rc<UH2> = v6;
                let v7: Rc<UH2> = v7;
                let v8: Rc<str> = v8;
                let mut v9: Rc<UH2> = v9;
                let v10: bool = v10;
                let v11: Rc<UH1> = v11;
                let v12: bool = &*v3 == &*v1;
                if v12 {
                    return US3::US3_Some(v3, v4, v5, v6, v7, v8, v9, v10);
                } else {
                    v0 = v11;
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US3::US3_None;
            }
        }
    }
}
fn method21(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH2> = method21(v3, v1);
            Rc::new(UH2::UH2_Cons(v2, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn closure_of_18(mut v0: Rc<UH1>, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH2> = v4;
                let v5: i32 = 0i32;
                let mut v6: Rc<dyn Fn(Rc<UH2>) -> US2> = method19(v3.clone(), v5);
                let v7: US2 = v6(v2.clone());
                let v8: bool = match v7 {
                    US2::US2_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v9: bool = v8 != true;
                if v9 {
                    v1 = v4;
                    continue;
                } else {
                    let v11: US3 = unit_named_20(v0.clone(), v3.clone());
                    let v22: Rc<UH2> = match v11 {
                        US3::US3_None => {
                            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
                        }
                        US3::US3_Some(v12, v13, v14, v15, v16, v17, v18, v19) => {
                            let v12: Rc<str> = v12;
                            let v13: US1 = v13;
                            let mut v14: Rc<UH2> = v14;
                            let v15: Rc<UH2> = v15;
                            let v16: Rc<UH2> = v16;
                            let v17: Rc<str> = v17;
                            let v18: Rc<UH2> = v18;
                            let v19: bool = v19;
                            v15
                        }
                    };
                    let v23: Rc<UH2> = method21(v4, v22);
                    let v24: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let v25: Rc<UH2> = Rc::new(UH2::UH2_Cons(v3, v24));
                    let v26: Rc<UH2> = method21(v2, v25);
                    (v1, v2) = (v23, v26);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn method24(mut v0: Rc<str>, mut v1: US1, mut v2: Rc<UH2>, mut v3: Rc<UH2>, mut v4: Rc<UH2>, mut v5: Rc<str>, mut v6: Rc<UH2>, mut v7: bool, mut v8: Rc<UH2>, mut v9: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v8) {
        UH2::UH2_Cons(v10, v11) => {
            let v10: Rc<str> = v10;
            let v11: Rc<UH2> = v11;
            let v12: Rc<UH3> = method24(v0.clone(), v1, v2, v3, v4, v5, v6, v7, v11, v9);
            Rc::new(UH3::UH3_Cons(v10, v0, v12))
        }
        UH2::UH2_Nil => {
            v9
        }
    }
}
fn module_pairs_23(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<UH3> {
    let mut v2: US3 = unit_named_20(v0, v1);
    match v2 {
        US3::US3_None => {
            { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) }
        }
        US3::US3_Some(v3, v4, v5, v6, v7, v8, v9, v10) => {
            let v3: Rc<str> = v3;
            let v4: US1 = v4;
            let v5: Rc<UH2> = v5;
            let mut v6: Rc<UH2> = v6;
            let v7: Rc<UH2> = v7;
            let v8: Rc<str> = v8;
            let mut v9: Rc<UH2> = v9;
            let v10: bool = v10;
            let v11: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
            method24(v3, v4, v5, v6, v7, v8, v9.clone(), v10, v9, v11)
        }
    }
}
fn method25(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_Cons(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<UH3> = v4;
            let v5: Rc<UH3> = method25(v4, v1);
            Rc::new(UH3::UH3_Cons(v2, v3, v5))
        }
        UH3::UH3_Nil => {
            v1
        }
    }
}
fn method22(mut v0: Rc<UH1>, mut v1: Rc<UH2>, mut v2: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH2> = v4;
                let v5: Rc<UH3> = module_pairs_23(v0.clone(), v3);
                let mut v6: Rc<UH3> = method25(v2, v5);
                (v1, v2) = (v4, v6);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn closure2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> {
    Rc::new(move |mut v2: Rc<str>, mut v3: Rc<str>| -> bool {
        let v4: bool = &*v2 == &*v1;
        if v4 {
            let v5: bool = &*v3 != &*v0;
            v5
        } else {
            false
        }
    })
}
fn method27(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> {
    closure2(v0, v1)
}
fn method28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<dyn Fn(Rc<str>, Rc<str>) -> bool>, mut v3: Rc<UH3>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH3::UH3_Cons(v4, v5, v6) => {
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let mut v6: Rc<UH3> = v6;
                let v7: bool = v2(v4, v5);
                if v7 {
                    return true;
                } else {
                    let v8: Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> = method27(v0.clone(), v1.clone());
                    (v2, v3) = (v8, v6);
                    continue;
                }
            }
            UH3::UH3_Nil => {
                return false;
            }
        }
    }
}
fn method26(mut v0: Rc<UH3>, mut v1: Rc<UH3>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH3::UH3_Cons(v3, v4, v5) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH3> = v5;
                let mut v6: Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> = method27(v4.clone(), v3.clone());
                let v7: bool = method28(v4, v3.clone(), v6, v0.clone());
                let mut v14: bool = if v7 {
                    let v8: i32 = 0i32;
                    let mut v9: Rc<dyn Fn(Rc<UH2>) -> US2> = method19(v3.clone(), v8);
                    let v10: US2 = v9(v2.clone());
                    let v11: bool = match v10 {
                        US2::US2_None => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v12: bool = v11 != true;
                    let v13: bool = v12 == false;
                    v13
                } else {
                    false
                };
                let v18: Rc<UH2> = if v14 {
                    let v15: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let v16: Rc<UH2> = Rc::new(UH2::UH2_Cons(v3, v15));
                    method21(v2, v16)
                } else {
                    v2
                };
                (v1, v2) = (v5, v18);
                continue;
            }
            UH3::UH3_Nil => {
                return v2;
            }
        }
    }
}
fn duplicate_findings_16(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: US1, mut v3: &Rc<UH2>, mut v4: &Rc<UH2>, mut v5: &Rc<UH2>, mut v6: &Rc<str>, mut v7: &Rc<UH2>, mut v8: bool) -> Rc<UH0> {
    let mut v9: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH2> = Rc::new(UH2::UH2_Cons(v1.clone(), v9));
    let v11: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH2> = closure_of_18(v0.clone(), v10, v11);
    let v13: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let mut v14: Rc<UH3> = method22(v0, v12, v13);
    let v15: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH2> = method26(v14.clone(), v14, v15);
    let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method17(v1, v2, v3, v4, v5, v6, v7, v8, v16, v17)
}
fn method15(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let v3: Rc<str> = v3;
                let v4: US1 = v4;
                let v5: Rc<UH2> = v5;
                let mut v6: Rc<UH2> = v6;
                let v7: Rc<UH2> = v7;
                let v8: Rc<str> = v8;
                let mut v9: Rc<UH2> = v9;
                let v10: bool = v10;
                let v11: Rc<UH1> = v11;
                let v12: Rc<UH0> = duplicate_findings_16(v0.clone(), v3, v4, &v5, &v6, &v7, &v8, &v9, v10);
                let v13: Rc<UH0> = method14(v2, v12);
                (v1, v2) = (v11, v13);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn method30(mut v0: Rc<UH2>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
            let v3: Rc<str> = v3;
            let v4: US1 = v4;
            let v5: Rc<UH2> = v5;
            let mut v6: Rc<UH2> = v6;
            let v7: Rc<UH2> = v7;
            let v8: Rc<str> = v8;
            let mut v9: Rc<UH2> = v9;
            let v10: bool = v10;
            let v11: Rc<UH1> = v11;
            let v12: Rc<UH1> = method30(v0.clone(), v11, v2);
            let v19: bool = if v10 {
                let v13: i32 = 0i32;
                let mut v14: Rc<dyn Fn(Rc<UH2>) -> US2> = method19(v3.clone(), v13);
                let v15: US2 = v14(v0);
                let v16: bool = match v15 {
                    US2::US2_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v17: bool = v16 != true;
                let v18: bool = v17 == false;
                v18
            } else {
                false
            };
            if v19 {
                Rc::new(UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v12))
            } else {
                v12
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn method31(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
            let mut v2: Rc<str> = v2;
            let v3: US1 = v3;
            let v4: Rc<UH2> = v4;
            let v5: Rc<UH2> = v5;
            let mut v6: Rc<UH2> = v6;
            let v7: Rc<str> = v7;
            let v8: Rc<UH2> = v8;
            let mut v9: bool = v9;
            let v10: Rc<UH1> = v10;
            let v11: Rc<UH0> = method31(v10, v1);
            let v12: US0 = US0::US0_MemberMissing(v2);
            Rc::new(UH0::UH0_Cons(v12, v11))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn closure3(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>, US1, Rc<UH2>, Rc<UH2>, Rc<UH2>, Rc<str>, Rc<UH2>, bool) -> bool> {
    Rc::new(move |mut v1: Rc<str>, mut v2: US1, mut v3: Rc<UH2>, mut v4: Rc<UH2>, mut v5: Rc<UH2>, mut v6: Rc<str>, mut v7: Rc<UH2>, mut v8: bool| -> bool {
        let mut v9: bool = &*v1 == &*v0;
        v9
    })
}
fn method35(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>, US1, Rc<UH2>, Rc<UH2>, Rc<UH2>, Rc<str>, Rc<UH2>, bool) -> bool> {
    closure3(v0)
}
fn method34(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>, US1, Rc<UH2>, Rc<UH2>, Rc<UH2>, Rc<str>, Rc<UH2>, bool) -> bool>, mut v2: Rc<UH1>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let v3: Rc<str> = v3;
                let v4: US1 = v4;
                let v5: Rc<UH2> = v5;
                let mut v6: Rc<UH2> = v6;
                let v7: Rc<UH2> = v7;
                let v8: Rc<str> = v8;
                let mut v9: Rc<UH2> = v9;
                let v10: bool = v10;
                let v11: Rc<UH1> = v11;
                let v12: bool = v1(v3, v4, v5, v6, v7, v8, v9, v10);
                if v12 {
                    return true;
                } else {
                    let v13: Rc<dyn Fn(Rc<str>, US1, Rc<UH2>, Rc<UH2>, Rc<UH2>, Rc<str>, Rc<UH2>, bool) -> bool> = method35(v0.clone());
                    (v1, v2) = (v13, v11);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return false;
            }
        }
    }
}
fn is_unit_33(mut v0: Rc<UH1>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<dyn Fn(Rc<str>, US1, Rc<UH2>, Rc<UH2>, Rc<UH2>, Rc<str>, Rc<UH2>, bool) -> bool> = method35(v1.clone());
    method34(v1, v2, v0)
}
fn method32(mut v0: Rc<UH1>, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v1) {
        UH2::UH2_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH2> = v4;
            let v5: Rc<UH2> = method32(v0.clone(), v4, v2);
            let mut v6: bool = is_unit_33(v0, v3.clone());
            let v7: bool = v6 == false;
            if v7 {
                Rc::new(UH2::UH2_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH2::UH2_Nil => {
            v2
        }
    }
}
fn method36(mut v0: Rc<UH2>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH0> = method36(v3, v1);
            let v5: US0 = US0::US0_MemberWithoutUnit(v2);
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn member_findings_29(mut v0: Rc<UH1>, mut v1: Rc<UH2>) -> Rc<UH0> {
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH1> = method30(v1.clone(), v0.clone(), v2);
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH0> = method31(v3, v4);
    let mut v6: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH2> = method32(v0, v1, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method36(v7, v8);
    method14(v5, v9)
}
fn layout_findings_13(mut v0: Rc<UH1>, mut v1: Rc<UH2>) -> Rc<UH0> {
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH0> = method15(v0.clone(), v0.clone(), v2);
    let v4: Rc<UH0> = member_findings_29(v0, v1);
    method14(v3, v4)
}
fn layout_hard_0(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>>) -> i32 {
    let mut v2: Rc<UH1> = units_of_7(&v1);
    let v3: Rc<UH2> = list_of_base_11(&v0);
    let v4: Rc<UH0> = layout_findings_13(v2, v3);
    hard_printed_1(v4)
}
fn closure0() -> Rc<dyn Fn(Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>>, Rc<RefCell<Vec<Rc<str>>>>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>>, Rc<RefCell<Vec<Rc<str>>>>) -> i32> = Rc::new(move |mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>>, mut v1: Rc<RefCell<Vec<Rc<str>>>>| -> i32 {
        layout_hard_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_units_layout_hard(v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>>, v1: Rc<RefCell<Vec<Rc<str>>>>) -> i32 {
    closure0()(v0, v1)
}
