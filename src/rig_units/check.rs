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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
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
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, US1, Rc<UH0>, Rc<UH0>, Rc<UH0>, Rc<str>, Rc<UH0>, bool, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
pub struct Mut0 { pub l0: Rc<str> }
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_1(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_2();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_3(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_1(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn kind_of_word_7(mut v0: Rc<str>) -> US1 {
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
fn list_from_base_9(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v4, v6);
            continue;
        }
    }
}
fn list_of_base_8(mut v0: &Rc<RefCell<Vec<Rc<str>>>>) -> Rc<UH0> {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    list_from_base_9(v0, v2, v3)
}
fn unit_of_row_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: &Rc<RefCell<Vec<Rc<str>>>>, mut v3: &Rc<RefCell<Vec<Rc<str>>>>, mut v4: &Rc<RefCell<Vec<Rc<str>>>>, mut v5: Rc<str>, mut v6: &Rc<RefCell<Vec<Rc<str>>>>, mut v7: bool) -> (Rc<str>, US1, Rc<UH0>, Rc<UH0>, Rc<UH0>, Rc<str>, Rc<UH0>, bool) {
    let v8: US1 = kind_of_word_7(v1);
    let mut v9: Rc<UH0> = list_of_base_8(v2);
    let v10: Rc<UH0> = list_of_base_8(v3);
    let v11: Rc<UH0> = list_of_base_8(v4);
    let v12: Rc<UH0> = list_of_base_8(v6);
    (v0, v8, v9, v10, v11, v5, v12, v7)
}
fn units_from_5(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool) = v0.borrow()[v1 as usize].clone();
            let (mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20): (Rc<str>, US1, Rc<UH0>, Rc<UH0>, Rc<UH0>, Rc<str>, Rc<UH0>, bool) = unit_of_row_6(v5, v6, &v7, &v8, &v9, v10, &v11, v12);
            let v21: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v14, v15, v16, v17, v18, v19, v20, v2));
            (v1, v2) = (v4, v21);
            continue;
        }
    }
}
fn units_of_4(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>>) -> Rc<UH1> {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    units_from_5(v0, v2, v3)
}
fn method10(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
                let v2: Rc<str> = v2;
                let v3: US1 = v3;
                let v4: Rc<UH0> = v4;
                let v5: Rc<UH0> = v5;
                let v6: Rc<UH0> = v6;
                let v7: Rc<str> = v7;
                let v8: Rc<UH0> = v8;
                let mut v9: bool = v9;
                let v10: Rc<UH1> = v10;
                let v11: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v10, v11);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn closure1(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>, US1, Rc<UH0>, Rc<UH0>, Rc<UH0>, Rc<str>, Rc<UH0>, bool) -> bool> {
    Rc::new(move |mut v1: Rc<str>, mut v2: US1, mut v3: Rc<UH0>, mut v4: Rc<UH0>, mut v5: Rc<UH0>, mut v6: Rc<str>, mut v7: Rc<UH0>, mut v8: bool| -> bool {
        let mut v9: bool = &*v1 == &*v0;
        v9
    })
}
fn method16(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>, US1, Rc<UH0>, Rc<UH0>, Rc<UH0>, Rc<str>, Rc<UH0>, bool) -> bool> {
    closure1(v0)
}
fn method15(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>, US1, Rc<UH0>, Rc<UH0>, Rc<UH0>, Rc<str>, Rc<UH0>, bool) -> bool>, mut v2: Rc<UH1>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let v3: Rc<str> = v3;
                let v4: US1 = v4;
                let v5: Rc<UH0> = v5;
                let v6: Rc<UH0> = v6;
                let v7: Rc<UH0> = v7;
                let v8: Rc<str> = v8;
                let mut v9: Rc<UH0> = v9;
                let v10: bool = v10;
                let v11: Rc<UH1> = v11;
                let v12: bool = v1(v3, v4, v5, v6, v7, v8, v9, v10);
                if v12 {
                    return true;
                } else {
                    let v13: Rc<dyn Fn(Rc<str>, US1, Rc<UH0>, Rc<UH0>, Rc<UH0>, Rc<str>, Rc<UH0>, bool) -> bool> = method16(v0.clone());
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
fn is_unit_14(mut v0: Rc<UH1>, mut v1: Rc<str>) -> bool {
    let v2: Rc<dyn Fn(Rc<str>, US1, Rc<UH0>, Rc<UH0>, Rc<UH0>, Rc<str>, Rc<UH0>, bool) -> bool> = method16(v1.clone());
    method15(v1, v2, v0)
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    Rc::new(move |mut v2: Rc<UH0>| -> US2 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US2> = method17(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method17(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    closure2(v0, v1)
}
fn method13(mut v0: Rc<UH1>, mut v1: &Rc<str>, mut v2: US1, mut v3: &Rc<UH0>, mut v4: Rc<UH0>, mut v5: &Rc<UH0>, mut v6: &Rc<str>, mut v7: &Rc<UH0>, mut v8: bool, mut v9: Rc<UH0>, mut v10: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v9) {
        UH0::UH0_Cons(v11, v12) => {
            let v11: Rc<str> = v11;
            let v12: Rc<UH0> = v12;
            let v13: Rc<UH0> = method13(v0.clone(), v1, v2, v3, v4.clone(), v5, v6, v7, v8, v12, v10);
            let v14: bool = is_unit_14(v0, v11.clone());
            let v21: bool = if v14 {
                let v15: i32 = 0i32;
                let mut v16: Rc<dyn Fn(Rc<UH0>) -> US2> = method17(v11.clone(), v15);
                let v17: US2 = v16(v4);
                let v18: bool = match v17 {
                    US2::US2_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v19: bool = v18 != true;
                let v20: bool = v19 == false;
                v20
            } else {
                false
            };
            if v21 {
                Rc::new(UH0::UH0_Cons(v11, v13))
            } else {
                v13
            }
        }
        UH0::UH0_Nil => {
            v10
        }
    }
}
fn method18(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn method12(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let v3: Rc<str> = v3;
                let v4: US1 = v4;
                let v5: Rc<UH0> = v5;
                let v6: Rc<UH0> = v6;
                let v7: Rc<UH0> = v7;
                let v8: Rc<str> = v8;
                let mut v9: Rc<UH0> = v9;
                let v10: bool = v10;
                let v11: Rc<UH1> = v11;
                let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v13: Rc<UH0> = method13(v0.clone(), &v3, v4, &v5, v6, &v7, &v8, &v9, v10, v7.clone(), v12);
                let v14: i32 = 0i32;
                let v15: i32 = method18(v13, v14);
                let mut v16: i32 = v2.wrapping_add(v15);
                (v1, v2) = (v11, v16);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn host_call_deps_11(mut v0: Rc<UH1>) -> i32 {
    let mut v1: i32 = 0i32;
    method12(v0.clone(), v0, v1)
}
fn method21(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_20(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method21(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn receipt_19(mut v0: i32, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: bool = v1 == 0i32;
    let v6: Rc<str> = if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
        v5
    };
    let v7: Rc<str> = format_real_20(v0);
    let v10: Rc<str> = format_real_20(v1);
    let v15: Rc<str> = format_real_20(v2);
    let mut v16: Rc<str> = string_concat(&["rig units check ", &*v6, " units=", &*v7, " hard=", &*v10, " host_call_deps=", &*v15]);
    v16
}
fn check_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_1(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            let v8: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>> = rig_units_manifest::rig_units_rows_under(&v6);
                            let mut v9: Rc<RefCell<Vec<Rc<str>>>> = rig_units_manifest::rig_units_members_under(&v6);
                            let v10: Rc<UH1> = units_of_4(&v8);
                            let v11: i32 = rig_units_edges::rig_units_edges_hard(v8.clone());
                            let v12: i32 = rig_units_layout::rig_units_layout_hard(v8, v9);
                            let v13: i32 = v11.wrapping_add(v12);
                            let v14: i32 = 0i32;
                            let v15: i32 = method10(v10.clone(), v14);
                            let mut v16: i32 = host_call_deps_11(v10);
                            let v17: Rc<str> = receipt_19(v15, v13, v16);
                            println!("{}", v17);
                            let v18: bool = v13 == 0i32;
                            if v18 {
                                0i32
                            } else {
                                1i32
                            }
                        }
                        _ => {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: units check: expected <workspace root>"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v20);
                            2i32
                        }
                    }
                }
                _ => {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: units check: expected <workspace root>"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v22);
                    2i32
                }
            }
        }
        _ => {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: units check: expected <workspace root>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v24);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        check_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_units_check_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
