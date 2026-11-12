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
    UH0_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_PlanningFields,
    US0_DoneDefinition,
    US0_NextStep,
    US0_TypeFeatures,
    US0_DoneEvidence,
    US0_BlockerNamed,
    US0_PauseResume,
    US0_ProseWait,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US0, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<UH2>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH2> }
fn closure1(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let v2: bool = v0.contains(&*v1);
        v2
    })
}
fn method7(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure1(v0)
}
fn method6(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: Rc<UH2>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH2> = v4;
                let mut v5: bool = v1(v3);
                if v5 {
                    return true;
                } else {
                    let v6: Rc<dyn Fn(Rc<str>) -> bool> = method7(v0.clone());
                    (v1, v2) = (v6, v4);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return false;
            }
        }
    }
}
fn says_5(mut v0: Rc<UH2>, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = Rc::<str>::from(v1.to_lowercase());
    let v3: Rc<dyn Fn(Rc<str>) -> bool> = method7(v2.clone());
    method6(v2, v3, v0)
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team:"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.starts_with(&*v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9() -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure2()
}
fn method8(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<UH2>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let mut v4: bool = v0(v2);
                if v4 {
                    return true;
                } else {
                    let mut v5: Rc<dyn Fn(Rc<str>) -> bool> = method9();
                    (v0, v1) = (v5, v3);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return false;
            }
        }
    }
}
fn method11(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH2> = v3;
            let mut v4: Rc<UH2> = method11(v3, v1);
            Rc::new(UH2::UH2_Cons(v2, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn markers_of_12(mut v0: US0) -> Rc<UH2> {
    match v0 {
        US0::US0_BlockerNamed => {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked on"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocker:"); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waits on"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v31: Rc<UH2> = Rc::new(UH2::UH2_Cons(v29, v30));
            let v32: Rc<UH2> = Rc::new(UH2::UH2_Cons(v28, v31));
            Rc::new(UH2::UH2_Cons(v27, v32))
        }
        US0::US0_DoneDefinition => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dod:"); } LIT.with(|lit| lit.clone()) };
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("definition of done"); } LIT.with(|lit| lit.clone()) };
            let v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let mut v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v3));
            Rc::new(UH2::UH2_Cons(v1, v4))
        }
        US0::US0_DoneEvidence => {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit 0"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" pass"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("green"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("proof:"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("proven"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v22: Rc<UH2> = Rc::new(UH2::UH2_Cons(v20, v21));
            let v23: Rc<UH2> = Rc::new(UH2::UH2_Cons(v19, v22));
            let v24: Rc<UH2> = Rc::new(UH2::UH2_Cons(v18, v23));
            let v25: Rc<UH2> = Rc::new(UH2::UH2_Cons(v17, v24));
            Rc::new(UH2::UH2_Cons(v16, v25))
        }
        US0::US0_NextStep => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next:"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next step"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let mut v9: Rc<UH2> = Rc::new(UH2::UH2_Cons(v7, v8));
            Rc::new(UH2::UH2_Cons(v6, v9))
        }
        US0::US0_PauseResume => {
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resume when"); } LIT.with(|lit| lit.clone()) };
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resume:"); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resumes on"); } LIT.with(|lit| lit.clone()) };
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resume from"); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v39: Rc<UH2> = Rc::new(UH2::UH2_Cons(v37, v38));
            let v40: Rc<UH2> = Rc::new(UH2::UH2_Cons(v36, v39));
            let v41: Rc<UH2> = Rc::new(UH2::UH2_Cons(v35, v40));
            Rc::new(UH2::UH2_Cons(v34, v41))
        }
        US0::US0_PlanningFields => {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_ProseWait => {
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waits on"); } LIT.with(|lit| lit.clone()) };
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resume when"); } LIT.with(|lit| lit.clone()) };
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked on"); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v47: Rc<UH2> = Rc::new(UH2::UH2_Cons(v45, v46));
            let v48: Rc<UH2> = Rc::new(UH2::UH2_Cons(v44, v47));
            Rc::new(UH2::UH2_Cons(v43, v48))
        }
        US0::US0_TypeFeatures => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("typefeatures:"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type feature plan"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v14: Rc<UH2> = Rc::new(UH2::UH2_Cons(v12, v13));
            Rc::new(UH2::UH2_Cons(v11, v14))
        }
    }
}
fn method14(mut v0: Rc<str>, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v1) {
        UH2::UH2_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH2> = v4;
            let mut v5: Rc<UH2> = method14(v0.clone(), v4, v2);
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(v3.replace(&*v0, &*v6));
            let v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
            Rc::new(UH2::UH2_Cons(v8, v5))
        }
        UH2::UH2_Nil => {
            v2
        }
    }
}
fn method16(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let mut v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method17(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH2> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH2::UH2_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method17(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH2> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH2> = Rc::new(UH2::UH2_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH2> = v9.borrow().l1.clone();
        let v17: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        method16(v16, v17)
    }
}
fn method18(mut v0: Rc<str>, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v1) {
        UH2::UH2_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH2> = v4;
            let mut v5: Rc<UH2> = method18(v0.clone(), v4, v2);
            let v6: bool = v3.starts_with(&*v0);
            if v6 {
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
fn declared_13(mut v0: Rc<str>, mut v1: US0) -> Rc<UH2> {
    ;
    ;
    ;
    ;
    ;
    ;
    ;
    ;
    let v2: bool = match &v1 {
        US0::US0_PlanningFields => {
            true
        }
        _ => {
            false
        }
    };
    let v6: US1 = if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PlanningFields"); } LIT.with(|lit| lit.clone()) };
        US1::US1_Some(v3)
    } else {
        US1::US1_None
    };
    let v74: US1 = match v6 {
        US1::US1_None => {
            let mut v9: bool = match &v1 {
                US0::US0_DoneDefinition => {
                    true
                }
                _ => {
                    false
                }
            };
            let v13: US1 = if v9 {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DoneDefinition"); } LIT.with(|lit| lit.clone()) };
                US1::US1_Some(v10)
            } else {
                US1::US1_None
            };
            match v13 {
                US1::US1_None => {
                    let v16: bool = match &v1 {
                        US0::US0_NextStep => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v20: US1 = if v16 {
                        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("NextStep"); } LIT.with(|lit| lit.clone()) };
                        US1::US1_Some(v17)
                    } else {
                        US1::US1_None
                    };
                    match v20 {
                        US1::US1_None => {
                            let v23: bool = match &v1 {
                                US0::US0_TypeFeatures => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            };
                            let v27: US1 = if v23 {
                                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TypeFeatures"); } LIT.with(|lit| lit.clone()) };
                                US1::US1_Some(v24)
                            } else {
                                US1::US1_None
                            };
                            match v27 {
                                US1::US1_None => {
                                    let v30: bool = match &v1 {
                                        US0::US0_DoneEvidence => {
                                            true
                                        }
                                        _ => {
                                            false
                                        }
                                    };
                                    let v34: US1 = if v30 {
                                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DoneEvidence"); } LIT.with(|lit| lit.clone()) };
                                        US1::US1_Some(v31)
                                    } else {
                                        US1::US1_None
                                    };
                                    match v34 {
                                        US1::US1_None => {
                                            let v37: bool = match &v1 {
                                                US0::US0_BlockerNamed => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            };
                                            let v41: US1 = if v37 {
                                                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BlockerNamed"); } LIT.with(|lit| lit.clone()) };
                                                US1::US1_Some(v38)
                                            } else {
                                                US1::US1_None
                                            };
                                            match v41 {
                                                US1::US1_None => {
                                                    let v44: bool = match &v1 {
                                                        US0::US0_PauseResume => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    };
                                                    let v48: US1 = if v44 {
                                                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PauseResume"); } LIT.with(|lit| lit.clone()) };
                                                        US1::US1_Some(v45)
                                                    } else {
                                                        US1::US1_None
                                                    };
                                                    match v48 {
                                                        US1::US1_None => {
                                                            let v51: bool = match v1 {
                                                                US0::US0_ProseWait => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            };
                                                            let v55: US1 = if v51 {
                                                                let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ProseWait"); } LIT.with(|lit| lit.clone()) };
                                                                US1::US1_Some(v52)
                                                            } else {
                                                                US1::US1_None
                                                            };
                                                            match v55 {
                                                                US1::US1_None => {
                                                                    US1::US1_None
                                                                }
                                                                US1::US1_Some(v56) => {
                                                                    let v56: Rc<str> = v56;
                                                                    US1::US1_Some(v56)
                                                                }
                                                            }
                                                        }
                                                        US1::US1_Some(v49) => {
                                                            let v49: Rc<str> = v49;
                                                            US1::US1_Some(v49)
                                                        }
                                                    }
                                                }
                                                US1::US1_Some(v42) => {
                                                    let v42: Rc<str> = v42;
                                                    US1::US1_Some(v42)
                                                }
                                            }
                                        }
                                        US1::US1_Some(v35) => {
                                            let v35: Rc<str> = v35;
                                            US1::US1_Some(v35)
                                        }
                                    }
                                }
                                US1::US1_Some(v28) => {
                                    let v28: Rc<str> = v28;
                                    US1::US1_Some(v28)
                                }
                            }
                        }
                        US1::US1_Some(v21) => {
                            let v21: Rc<str> = v21;
                            US1::US1_Some(v21)
                        }
                    }
                }
                US1::US1_Some(v14) => {
                    let v14: Rc<str> = v14;
                    US1::US1_Some(v14)
                }
            }
        }
        US1::US1_Some(v7) => {
            let v7: Rc<str> = v7;
            US1::US1_Some(v7)
        }
    };
    let v78: Rc<str> = match v74 {
        US1::US1_None => {
            std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Option does not have a value."); } LIT.with(|lit| lit.clone()) }))
        }
        US1::US1_Some(v75) => {
            let v75: Rc<str> = v75;
            v75
        }
    };
    let v79: Rc<str> = string_concat(&[&*v78, "="]);
    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v81: Rc<UH2> = split_list_15(v80, v0);
    let v82: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v83: Rc<UH2> = method18(v79.clone(), v81, v82);
    let v84: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method14(v79, v83, v84)
}
fn markers_for_10(mut v0: &Rc<str>, mut v1: &Rc<str>, mut v2: i32, mut v3: &Rc<str>, mut v4: &Rc<str>, mut v5: &Rc<str>, mut v6: &Rc<str>, mut v7: Rc<str>, mut v8: US0) -> Rc<UH2> {
    let mut v9: Rc<UH2> = markers_of_12(v8.clone());
    let v10: Rc<UH2> = declared_13(v7, v8);
    method11(v9, v10)
}
fn method19(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH2> = v3;
            let mut v4: Rc<UH2> = method19(v3, v1);
            let mut v5: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            Rc::new(UH2::UH2_Cons(v5, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method21(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure3() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn last_note_20(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.rfind(&*v1);
    let v3: Option<usize> = method21(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure3();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        v0
    } else {
        let mut v9: i32 = v7.wrapping_add(1i32);
        let v10: i32 = (v0.clone().len() as i32);
        let v11: i32 = v10.wrapping_sub(v7);
        let v12: i32 = v11.wrapping_sub(1i32);
        let v13: usize = ((v9) as usize);
        let v14: i32 = v9.wrapping_add(v12);
        let v15: usize = ((v14) as usize);
        let v16: Rc<str> = Rc::<str>::from(&(v0)[v13..v15]);
        v16
    }
}
fn passes_4(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: US0) -> bool {
    match &v8 {
        US0::US0_BlockerNamed => {
            let v27: bool = &*v6 != "";
            if v27 {
                true
            } else {
                let v28: Rc<UH2> = markers_for_10(v0, &v1, v2, &v3, &v4, &v5, &v6, v7, v8);
                says_5(v28, v5)
            }
        }
        US0::US0_PauseResume => {
            let v31: bool = &*v6 != "";
            if v31 {
                true
            } else {
                let v32: Rc<UH2> = markers_for_10(v0, &v1, v2, &v3, &v4, &v5, &v6, v7, v8);
                says_5(v32, v5)
            }
        }
        US0::US0_PlanningFields => {
            let mut v9: bool = v2 > 0i32;
            let v11: bool = if v9 {
                let v10: bool = &*v3 != "";
                v10
            } else {
                false
            };
            let v13: bool = if v11 {
                let v12: bool = &*v4 != "";
                v12
            } else {
                false
            };
            if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<UH2> = split_list_15(v14, v1);
                let v16: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                let v17: Rc<UH2> = method19(v15, v16);
                let v18: Rc<dyn Fn(Rc<str>) -> bool> = method9();
                method8(v18, v17)
            } else {
                false
            }
        }
        US0::US0_ProseWait => {
            let v21: bool = &*v6 != "";
            if v21 {
                true
            } else {
                let v22: Rc<UH2> = markers_for_10(v0, &v1, v2, &v3, &v4, &v5, &v6, v7, v8);
                let v23: Rc<str> = last_note_20(v5);
                let v24: bool = says_5(v22, v23);
                let v25: bool = v24 == false;
                v25
            }
        }
        _ => {
            let v35: Rc<UH2> = markers_for_10(v0, &v1, v2, &v3, &v4, &v5, &v6, v7, v8);
            says_5(v35, v5)
        }
    }
}
fn method3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<UH1>, mut v9: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v8) {
        UH1::UH1_Cons(v10, v11) => {
            let v10: US0 = v10;
            let v11: Rc<UH1> = v11;
            let v12: Rc<UH1> = method3(v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v11, v9);
            let v13: bool = passes_4(&v0, v1, v2, v3, v4, v5, v6, v7, v10.clone());
            let v14: bool = v13 == false;
            if v14 {
                Rc::new(UH1::UH1_Cons(v10, v12))
            } else {
                v12
            }
        }
        UH1::UH1_Nil => {
            v9
        }
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        let mut v1: bool = &*v0 == "kit";
        let v3: bool = if v1 {
            true
        } else {
            let v2: bool = &*v0 == "lib";
            v2
        };
        if v3 {
            true
        } else {
            let mut v4: bool = &*v0 == "compiler";
            v4
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method23() -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure4()
}
fn method24(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<UH2>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let mut v4: bool = v0(v2);
                if v4 {
                    return true;
                } else {
                    let mut v5: Rc<dyn Fn(Rc<str>) -> bool> = method23();
                    (v0, v1) = (v5, v3);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return false;
            }
        }
    }
}
fn checks_for_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: bool = &*v0 == "Done";
    if v2 {
        let v3: US0 = US0::US0_DoneEvidence;
        let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v3, v4))
    } else {
        let v6: bool = &*v0 == "Blocked";
        if v6 {
            let v7: US0 = US0::US0_BlockerNamed;
            let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_Cons(v7, v8))
        } else {
            let v10: bool = &*v0 == "Paused";
            if v10 {
                let v11: US0 = US0::US0_PauseResume;
                let v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                Rc::new(UH1::UH1_Cons(v11, v12))
            } else {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<UH2> = split_list_15(v14, v1);
                let v16: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                let v17: Rc<UH2> = method19(v15, v16);
                let v18: Rc<dyn Fn(Rc<str>) -> bool> = method23();
                let v19: bool = method24(v18, v17);
                if v19 {
                    let v20: US0 = US0::US0_PlanningFields;
                    let v21: US0 = US0::US0_DoneDefinition;
                    let v22: US0 = US0::US0_NextStep;
                    let v23: US0 = US0::US0_TypeFeatures;
                    let v24: US0 = US0::US0_ProseWait;
                    let v25: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v26: Rc<UH1> = Rc::new(UH1::UH1_Cons(v24, v25));
                    let v27: Rc<UH1> = Rc::new(UH1::UH1_Cons(v23, v26));
                    let v28: Rc<UH1> = Rc::new(UH1::UH1_Cons(v22, v27));
                    let v29: Rc<UH1> = Rc::new(UH1::UH1_Cons(v21, v28));
                    Rc::new(UH1::UH1_Cons(v20, v29))
                } else {
                    let v31: US0 = US0::US0_PlanningFields;
                    let v32: US0 = US0::US0_DoneDefinition;
                    let v33: US0 = US0::US0_NextStep;
                    let v34: US0 = US0::US0_ProseWait;
                    let v35: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v36: Rc<UH1> = Rc::new(UH1::UH1_Cons(v34, v35));
                    let v37: Rc<UH1> = Rc::new(UH1::UH1_Cons(v33, v36));
                    let v38: Rc<UH1> = Rc::new(UH1::UH1_Cons(v32, v37));
                    Rc::new(UH1::UH1_Cons(v31, v38))
                }
            }
        }
    }
}
fn failing_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> Rc<UH1> {
    let v8: Rc<UH1> = checks_for_22(v0.clone(), v1.clone());
    let mut v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method3(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
}
fn method25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: &Rc<str>, mut v3: i32, mut v4: &Rc<str>, mut v5: &Rc<str>, mut v6: &Rc<str>, mut v7: &Rc<str>, mut v8: &Rc<str>, mut v9: Rc<UH1>, mut v10: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v9) {
        UH1::UH1_Cons(v11, v12) => {
            let v11: US0 = v11;
            let v12: Rc<UH1> = v12;
            let v13: Rc<UH0> = method25(v0.clone(), v1.clone(), v2, v3, v4, v5, v6, v7, v8, v12, v10);
            ;
            ;
            ;
            ;
            ;
            ;
            ;
            ;
            let v14: bool = match &v11 {
                US0::US0_PlanningFields => {
                    true
                }
                _ => {
                    false
                }
            };
            let v18: US1 = if v14 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PlanningFields"); } LIT.with(|lit| lit.clone()) };
                US1::US1_Some(v15)
            } else {
                US1::US1_None
            };
            let v86: US1 = match v18 {
                US1::US1_None => {
                    let v21: bool = match &v11 {
                        US0::US0_DoneDefinition => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v25: US1 = if v21 {
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DoneDefinition"); } LIT.with(|lit| lit.clone()) };
                        US1::US1_Some(v22)
                    } else {
                        US1::US1_None
                    };
                    match v25 {
                        US1::US1_None => {
                            let v28: bool = match &v11 {
                                US0::US0_NextStep => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            };
                            let v32: US1 = if v28 {
                                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("NextStep"); } LIT.with(|lit| lit.clone()) };
                                US1::US1_Some(v29)
                            } else {
                                US1::US1_None
                            };
                            match v32 {
                                US1::US1_None => {
                                    let v35: bool = match &v11 {
                                        US0::US0_TypeFeatures => {
                                            true
                                        }
                                        _ => {
                                            false
                                        }
                                    };
                                    let v39: US1 = if v35 {
                                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TypeFeatures"); } LIT.with(|lit| lit.clone()) };
                                        US1::US1_Some(v36)
                                    } else {
                                        US1::US1_None
                                    };
                                    match v39 {
                                        US1::US1_None => {
                                            let v42: bool = match &v11 {
                                                US0::US0_DoneEvidence => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            };
                                            let v46: US1 = if v42 {
                                                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DoneEvidence"); } LIT.with(|lit| lit.clone()) };
                                                US1::US1_Some(v43)
                                            } else {
                                                US1::US1_None
                                            };
                                            match v46 {
                                                US1::US1_None => {
                                                    let v49: bool = match &v11 {
                                                        US0::US0_BlockerNamed => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    };
                                                    let v53: US1 = if v49 {
                                                        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BlockerNamed"); } LIT.with(|lit| lit.clone()) };
                                                        US1::US1_Some(v50)
                                                    } else {
                                                        US1::US1_None
                                                    };
                                                    match v53 {
                                                        US1::US1_None => {
                                                            let v56: bool = match &v11 {
                                                                US0::US0_PauseResume => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            };
                                                            let v60: US1 = if v56 {
                                                                let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PauseResume"); } LIT.with(|lit| lit.clone()) };
                                                                US1::US1_Some(v57)
                                                            } else {
                                                                US1::US1_None
                                                            };
                                                            match v60 {
                                                                US1::US1_None => {
                                                                    let v63: bool = match v11 {
                                                                        US0::US0_ProseWait => {
                                                                            true
                                                                        }
                                                                        _ => {
                                                                            false
                                                                        }
                                                                    };
                                                                    let v67: US1 = if v63 {
                                                                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ProseWait"); } LIT.with(|lit| lit.clone()) };
                                                                        US1::US1_Some(v64)
                                                                    } else {
                                                                        US1::US1_None
                                                                    };
                                                                    match v67 {
                                                                        US1::US1_None => {
                                                                            US1::US1_None
                                                                        }
                                                                        US1::US1_Some(v68) => {
                                                                            let v68: Rc<str> = v68;
                                                                            US1::US1_Some(v68)
                                                                        }
                                                                    }
                                                                }
                                                                US1::US1_Some(v61) => {
                                                                    let v61: Rc<str> = v61;
                                                                    US1::US1_Some(v61)
                                                                }
                                                            }
                                                        }
                                                        US1::US1_Some(v54) => {
                                                            let v54: Rc<str> = v54;
                                                            US1::US1_Some(v54)
                                                        }
                                                    }
                                                }
                                                US1::US1_Some(v47) => {
                                                    let v47: Rc<str> = v47;
                                                    US1::US1_Some(v47)
                                                }
                                            }
                                        }
                                        US1::US1_Some(v40) => {
                                            let v40: Rc<str> = v40;
                                            US1::US1_Some(v40)
                                        }
                                    }
                                }
                                US1::US1_Some(v33) => {
                                    let v33: Rc<str> = v33;
                                    US1::US1_Some(v33)
                                }
                            }
                        }
                        US1::US1_Some(v26) => {
                            let v26: Rc<str> = v26;
                            US1::US1_Some(v26)
                        }
                    }
                }
                US1::US1_Some(v19) => {
                    let v19: Rc<str> = v19;
                    US1::US1_Some(v19)
                }
            };
            let v90: Rc<str> = match v86 {
                US1::US1_None => {
                    std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Option does not have a value."); } LIT.with(|lit| lit.clone()) }))
                }
                US1::US1_Some(v87) => {
                    let v87: Rc<str> = v87;
                    v87
                }
            };
            Rc::new(UH0::UH0_Cons(v0, v1, v90, v13))
        }
        UH1::UH1_Nil => {
            v10
        }
    }
}
fn method26(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let mut v5: Rc<UH0> = v5;
            let v6: Rc<UH0> = method26(v5, v1);
            Rc::new(UH0::UH0_Cons(v2, v3, v4, v6))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn rows_from_1(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = v0.borrow()[v1 as usize].clone();
            let v13: Rc<UH1> = failing_2(v5.clone(), v6.clone(), v7, v8.clone(), v9.clone(), v10.clone(), v11.clone(), v12.clone());
            let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v15: Rc<UH0> = method25(v4, v5, &v6, v7, &v8, &v9, &v10, &v11, &v12, v13, v14);
            let v16: i32 = v1.wrapping_sub(1i32);
            let v17: Rc<UH0> = method26(v15, v2);
            (v1, v2) = (v16, v17);
            continue;
        }
    }
}
fn method27(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
                let v6: Rc<UH0> = v6;
                v0.borrow_mut().push((v3, v4, v5));
                let v7: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v6, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn failing_rows_0(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = rows_from_1(&v0, v2, v3);
    let mut v5: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v6: i32 = 0i32;
    let v7: i32 = method27(v5.clone(), v4, v6);
    let v8: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>)>> = Rc::new(v5.borrow().clone());
    let mut v9: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
    v9
}
fn closure0() -> Rc<dyn Fn(Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
        failing_rows_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_lint_failing_rows(v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    closure0()(v0)
}
pub type SpiralShape_0408665020a9 = Mut0;
