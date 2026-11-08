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
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, bool, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_MachineInTwoNetworks(Rc<str>),
    US0_HubCount(Rc<str>, i32),
    US0_UnknownNetwork(Rc<str>, Rc<str>),
    US0_AnchorCount(Rc<str>, i32),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US0, Rc<UH2>),
}
#[derive(Clone)]
enum US1 {
    US1_DirectReach,
    US1_JumpReach,
    US1_ReverseReach,
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
pub struct Mut0 { pub l0: Rc<str> }
fn method1(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method2(mut v0: Rc<Vec<(Rc<str>, Rc<str>, bool, Rc<str>)>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, bool, Rc<str>) = (v0)[v4].clone();
            let mut v9: i32 = v1.wrapping_sub(1i32);
            let mut v10: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v6, v7, v8, v2));
            (v1, v2) = (v9, v10);
            continue;
        }
    }
}
fn method4(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let v2: US0 = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH2> = method4(v3, v1);
            Rc::new(UH2::UH2_Cons(v2, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method7(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: bool = v5;
                let v6: Rc<str> = v6;
                let v7: Rc<UH1> = v7;
                let v8: bool = &*v4 == &*v0;
                let mut v9: bool = v8 && v5;
                let mut v11: i32 = if v9 {
                    let mut v10: i32 = v2.wrapping_add(1i32);
                    v10
                } else {
                    v2
                };
                (v1, v2) = (v7, v11);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn method6(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: i32 = 0i32;
                let v6: i32 = method7(v3.clone(), v0.clone(), v5);
                let v7: bool = v6 == 1i32;
                let mut v10: Rc<UH2> = if v7 {
                    v2
                } else {
                    let v8: US0 = US0::US0_HubCount(v3, v6);
                    Rc::new(UH2::UH2_Cons(v8, v2))
                };
                (v1, v2) = (v4, v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn hub_problems_5(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH2> {
    let v2: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method6(v1, v0, v2)
}
fn method10(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: bool = v5;
            let v6: Rc<str> = v6;
            let v7: Rc<UH1> = v7;
            let v8: Rc<UH1> = method10(v0.clone(), v7, v2);
            let mut v9: bool = &*v4 == &*v0;
            if v9 {
                Rc::new(UH1::UH1_Cons(v3, v4, v5, v6, v8))
            } else {
                v8
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn reach_of_12(mut v0: Rc<str>) -> US1 {
    let v1: bool = &*v0 == "direct";
    if v1 {
        US1::US1_DirectReach
    } else {
        let v3: bool = &*v0 == "reverse";
        if v3 {
            US1::US1_ReverseReach
        } else {
            US1::US1_JumpReach
        }
    }
}
fn method11(mut v0: Rc<UH1>, mut v1: bool) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: bool = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<UH1> = v6;
                let mut v10: bool = if v1 {
                    true
                } else {
                    let v7: US1 = reach_of_12(v5);
                    let v8: bool = match v7 {
                        US1::US1_DirectReach => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let mut v9: bool = v8 == false;
                    v9
                };
                (v0, v1) = (v6, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method13(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: bool = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<UH1> = v6;
                let v7: US1 = reach_of_12(v5);
                let v8: bool = match v7 {
                    US1::US1_ReverseReach => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v10: i32 = if v8 {
                    let mut v9: i32 = v1.wrapping_add(1i32);
                    v9
                } else {
                    v1
                };
                (v0, v1) = (v6, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method9(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v6: Rc<UH1> = method10(v3.clone(), v0.clone(), v5);
                let v7: bool = false;
                let v8: bool = method11(v6.clone(), v7);
                let mut v9: i32 = 0i32;
                let mut v10: i32 = method13(v6, v9);
                let v12: bool = if v8 {
                    let mut v11: bool = v10 != 1i32;
                    v11
                } else {
                    false
                };
                let mut v15: Rc<UH2> = if v12 {
                    let v13: US0 = US0::US0_AnchorCount(v3, v10);
                    Rc::new(UH2::UH2_Cons(v13, v2))
                } else {
                    v2
                };
                (v1, v2) = (v4, v15);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn anchor_problems_8(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH2> {
    let v2: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method9(v1, v0, v2)
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
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
    closure1(v0, v1)
}
fn method16(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: bool = v5;
                let v6: Rc<str> = v6;
                let v7: Rc<UH1> = v7;
                let v8: bool = &*v3 == &*v0;
                let mut v15: bool = if v8 {
                    let mut v9: i32 = 0i32;
                    let mut v10: Rc<dyn Fn(Rc<UH0>) -> US2> = method17(v4.clone(), v9);
                    let mut v11: US2 = v10(v2.clone());
                    let v12: bool = match v11 {
                        US2::US2_None => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v13: bool = v12 != true;
                    let v14: bool = v13 == false;
                    v14
                } else {
                    false
                };
                let v17: Rc<UH0> = if v15 {
                    Rc::new(UH0::UH0_Cons(v4, v2))
                } else {
                    v2
                };
                (v1, v2) = (v7, v17);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
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
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH2>) -> US2> {
    Rc::new(move |mut v2: Rc<UH2>| -> US2 {
        match Rc::unwrap_or_clone(v2) {
            UH2::UH2_Cons(v4, v5) => {
                let v4: US0 = v4;
                let v5: Rc<UH2> = v5;
                let v8: bool = match v4 {
                    US0::US0_MachineInTwoNetworks(v6) => {
                        let v6: Rc<str> = v6;
                        let v7: bool = &*v6 == &*v0;
                        v7
                    }
                    _ => {
                        false
                    }
                };
                if v8 {
                    US2::US2_Some(v1)
                } else {
                    let mut v10: i32 = v1.wrapping_add(1i32);
                    let mut v11: Rc<dyn Fn(Rc<UH2>) -> US2> = method19(v0.clone(), v10);
                    v11(v5)
                }
            }
            UH2::UH2_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method19(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH2>) -> US2> {
    closure2(v0, v1)
}
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: Rc<UH1>, mut v3: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v4, v5, v6, v7, v8) => {
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: bool = v6;
                let v7: Rc<str> = v7;
                let v8: Rc<UH1> = v8;
                let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v10: Rc<UH0> = method16(v4.clone(), v1.clone(), v9);
                let mut v11: i32 = 0i32;
                let v12: i32 = method18(v10, v11);
                let v13: bool = v12 > 1i32;
                let v20: bool = if v13 {
                    let v14: i32 = 0i32;
                    let mut v15: Rc<dyn Fn(Rc<UH2>) -> US2> = method19(v4.clone(), v14);
                    let v16: US2 = v15(v3.clone());
                    let v17: bool = match v16 {
                        US2::US2_None => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v18: bool = v17 != true;
                    let v19: bool = v18 == false;
                    v19
                } else {
                    false
                };
                let v23: Rc<UH2> = if v20 {
                    let v21: US0 = US0::US0_MachineInTwoNetworks(v4.clone());
                    Rc::new(UH2::UH2_Cons(v21, v3))
                } else {
                    v3
                };
                let v24: i32 = 0i32;
                let mut v25: Rc<dyn Fn(Rc<UH0>) -> US2> = method17(v5.clone(), v24);
                let v26: US2 = v25(v0.clone());
                let v27: bool = match v26 {
                    US2::US2_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v28: bool = v27 != true;
                let v31: Rc<UH2> = if v28 {
                    v23
                } else {
                    let v29: US0 = US0::US0_UnknownNetwork(v4, v5);
                    Rc::new(UH2::UH2_Cons(v29, v23))
                };
                (v2, v3) = (v8, v31);
                continue;
            }
            UH1::UH1_Nil => {
                return v3;
            }
        }
    }
}
fn membership_problems_14(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH2> {
    let v2: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method15(v0, v1.clone(), v1, v2)
}
fn fleet_problems_3(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH2> {
    let v2: Rc<UH2> = hub_problems_5(v0.clone(), v1.clone());
    let v3: Rc<UH2> = anchor_problems_8(v0.clone(), v1.clone());
    let v4: Rc<UH2> = method4(v2, v3);
    let v5: Rc<UH2> = membership_problems_14(v0, v1);
    method4(v5, v4)
}
fn method23(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_22(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method23(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn problem_text_21(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_AnchorCount(v16, v17) => {
            let v16: Rc<str> = v16;
            let v17: i32 = v17;
            let v20: Rc<str> = format_real_22(v17);
            let v22: Rc<str> = string_concat(&["network ", &*v16, " has ", &*v20, " reverse anchors; a network with indirect machines needs one"]);
            v22
        }
        US0::US0_HubCount(v4, v5) => {
            let v4: Rc<str> = v4;
            let v5: i32 = v5;
            let v8: Rc<str> = format_real_22(v5);
            let mut v10: Rc<str> = string_concat(&["network ", &*v4, " has ", &*v8, " hubs; it needs exactly one"]);
            v10
        }
        US0::US0_MachineInTwoNetworks(v1) => {
            let v1: Rc<str> = v1;
            let v3: Rc<str> = string_concat(&["machine ", &*v1, " is listed in two networks"]);
            v3
        }
        US0::US0_UnknownNetwork(v11, v12) => {
            let mut v11: Rc<str> = v11;
            let v12: Rc<str> = v12;
            let mut v15: Rc<str> = string_concat(&["machine ", &*v11, " names unknown network ", &*v12]);
            v15
        }
    }
}
fn method20(mut v0: Rc<UH2>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let v2: US0 = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH0> = method20(v3, v1);
            let v5: Rc<str> = problem_text_21(v2);
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method24(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn problem_lines_0(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, bool, Rc<str>)>>>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v2: Rc<Vec<Rc<str>>> = Rc::new((v1).borrow().clone());
    let v3: usize = (v2).len();
    let v4: i32 = ((v3) as i32);
    let v5: i32 = v4.wrapping_sub(1i32);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method1(v2, v5, v6);
    let v8: Rc<Vec<(Rc<str>, Rc<str>, bool, Rc<str>)>> = Rc::new((v0).borrow().clone());
    let mut v9: usize = (v8).len();
    let mut v10: i32 = ((v9) as i32);
    let mut v11: i32 = v10.wrapping_sub(1i32);
    let v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH1> = method2(v8, v11, v12);
    let v14: Rc<UH2> = fleet_problems_3(v7, v13);
    let mut v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH0> = method20(v14, v15);
    let v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v18: i32 = 0i32;
    let v19: i32 = method24(v17.clone(), v16, v18);
    let v20: Rc<Vec<Rc<str>>> = Rc::new(v17.borrow().clone());
    let v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v20).as_ref().clone()));
    v21
}
fn closure0() -> Rc<dyn Fn(Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<(Rc<str>, Rc<str>, bool, Rc<str>)>>>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<(Rc<str>, Rc<str>, bool, Rc<str>)>>>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, bool, Rc<str>)>>>| -> Rc<RefCell<Vec<Rc<str>>>> {
        problem_lines_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_net_fleet_problem_lines(v0: Rc<RefCell<Vec<Rc<str>>>>, v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, bool, Rc<str>)>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure0()(v0, v1)
}
pub type SpiralShape_97df63ebabc3 = Mut0;
