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
    US0_Forward,
    US0_Rename,
    US0_Reorder,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, US0, Rc<str>, i32, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, US0, Rc<str>, i32),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(US0, Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(i32),
    US3_None,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<UH2>),
}
fn method3(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method4(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method4(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH1> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method3(v16, v17)
    }
}
fn cut_start_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: i32 = (v2.clone().len() as i32);
    let v4: bool = &*v1 == "";
    let v6: bool = if v4 {
        let mut v5: bool = v0.starts_with(&*v2);
        v5
    } else {
        false
    };
    if v6 {
        let v7: i32 = (v0.clone().len() as i32);
        let v8: i32 = v7.wrapping_sub(v3);
        let mut v9: usize = ((v3) as usize);
        let v10: i32 = v3.wrapping_add(v8);
        let mut v11: usize = ((v10) as usize);
        let mut v12: Rc<str> = Rc::<str>::from(&(v0)[v9..v11]);
        v12
    } else {
        v1
    }
}
fn definition_rest_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":~ ^ "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = cut_start_8(v0.clone(), v1, v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= ^ "); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = cut_start_8(v0.clone(), v3, v4);
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":~ "); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = cut_start_8(v0.clone(), v5, v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = cut_start_8(v0.clone(), v7, v8);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    cut_start_8(v0, v9, v10)
}
fn head_word_9(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v3: Rc<UH1> = split_list_2(v1, v2);
    match Rc::unwrap_or_clone(v3) {
        UH1::UH1_Cons(v4, v5) => {
            let v4: Rc<str> = v4;
            let mut v5: Rc<UH1> = v5;
            v4
        }
        UH1::UH1_Nil => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v6
        }
    }
}
fn method11(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn params_from_10(mut v0: Rc<str>, mut v1: Rc<UH1>) -> (Rc<UH1>, Rc<str>) {
    loop {
        let v2: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v2.starts_with(&*v3);
        let mut v5: bool = v4 == false;
        if v5 {
            let v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v7: Rc<UH1> = method3(v1, v6);
            return (v7, v2);
        } else {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Option<usize> = v2.find(&*v8);
            let v10: Option<usize> = method11(v9);
            let mut v11: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v12: Option<i32> = v10.map(|x| v11(x));
            let v14: i32 = v12.unwrap_or(-1i32);
            let v15: bool = v14 < 0i32;
            let mut v22: Rc<str> = if v15 {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v16
            } else {
                let v17: i32 = v14.wrapping_sub(1i32);
                let v18: usize = ((1i32) as usize);
                let v19: i32 = 1i32.wrapping_add(v17);
                let v20: usize = ((v19) as usize);
                let mut v21: Rc<str> = Rc::<str>::from(&(v2)[v18..v20]);
                v21
            };
            let v23: Rc<str> = head_word_9(v22.clone());
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = Rc::<str>::from(v23.replace(&*v24, &*v25));
            let v28: bool = if v15 {
                true
            } else {
                let v27: bool = v22.contains(&*v3);
                v27
            };
            let v30: bool = if v28 {
                true
            } else {
                let v29: bool = &*v26 == "";
                v29
            };
            if v30 {
                let v31: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                return (v31, v25);
            } else {
                let v32: i32 = v14.wrapping_add(1i32);
                let v33: i32 = (v2.clone().len() as i32);
                let v34: i32 = v33.wrapping_sub(v14);
                let v35: i32 = v34.wrapping_sub(1i32);
                let v36: usize = ((v32) as usize);
                let v37: i32 = v32.wrapping_add(v35);
                let v38: usize = ((v37) as usize);
                let v39: Rc<str> = Rc::<str>::from(&(v2)[v36..v38]);
                let v40: Rc<UH1> = Rc::new(UH1::UH1_Cons(v26, v1));
                (v0, v1) = (v39, v40);
                continue;
            }
        }
    }
}
fn method12(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method12(v3, v1);
            let mut v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH1::UH1_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method15(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn identifier_16(mut v0: Rc<str>) -> bool {
    let v1: bool = &*v0 != "";
    let mut v5: bool = if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
        let v3: bool = v0.contains(&*v2);
        let v4: bool = v3 == false;
        v4
    } else {
        false
    };
    let mut v9: bool = if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v7: bool = v0.contains(&*v6);
        let v8: bool = v7 == false;
        v8
    } else {
        false
    };
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v11: bool = v0.contains(&*v10);
        let mut v12: bool = v11 == false;
        v12
    } else {
        false
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US3> {
    Rc::new(move |mut v2: Rc<UH1>| -> US3 {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH1> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US3::US3_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US3> = method17(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH1::UH1_Nil => {
                US3::US3_None
            }
        }
    })
}
fn method17(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US3> {
    closure2(v0, v1)
}
fn method18(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> bool {
    loop {
        match (Rc::unwrap_or_clone(v1), Rc::unwrap_or_clone(v0)) {
            (UH1::UH1_Cons(v2, v3), UH1::UH1_Cons(v4, v5)) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH1> = v5;
                let v6: bool = &*v2 == &*v4;
                if v6 {
                    (v0, v1) = (v5, v3);
                    continue;
                } else {
                    return false;
                }
            }
            (UH1::UH1_Nil, UH1::UH1_Nil) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}
fn passed_shape_19(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.contains(&*v1);
    if v2 {
        US0::US0_Forward
    } else {
        US0::US0_Rename
    }
}
fn all_within_20(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: i32 = 0i32;
                let mut v5: Rc<dyn Fn(Rc<UH1>) -> US3> = method17(v2, v4);
                let v6: US3 = v5(v1.clone());
                let v7: bool = match v6 {
                    US3::US3_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v8: bool = v7 != true;
                if v8 {
                    v0 = v3;
                    continue;
                } else {
                    return false;
                }
            }
            UH1::UH1_Nil => {
                return true;
            }
        }
    }
}
fn called_shape_14(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<UH1>) -> US2 {
    let v3: i32 = 0i32;
    let v4: i32 = method15(v2.clone(), v3);
    let mut v5: i32 = 0i32;
    let v6: i32 = method15(v0.clone(), v5);
    let v7: bool = v4 == v6;
    let v8: bool = identifier_16(v1.clone());
    let mut v9: bool = v8 == false;
    let mut v11: bool = if v9 {
        true
    } else {
        match &*v2 {
            UH1::UH1_Nil => {
                true
            }
            _ => {
                false
            }
        }
    };
    let v17: bool = if v11 {
        true
    } else {
        let mut v12: i32 = 0i32;
        let mut v13: Rc<dyn Fn(Rc<UH1>) -> US3> = method17(v1.clone(), v12);
        let v14: US3 = v13(v0.clone());
        let v15: bool = match v14 {
            US3::US3_None => {
                true
            }
            _ => {
                false
            }
        };
        let v16: bool = v15 != true;
        v16
    };
    if v17 {
        US2::US2_None
    } else {
        let v19: bool = method18(v0.clone(), v2.clone());
        if v19 {
            let v20: US0 = passed_shape_19(v1.clone());
            US2::US2_Some(v20, v1)
        } else {
            let v23: bool = if v7 {
                all_within_20(v2.clone(), v0.clone())
            } else {
                false
            };
            let v25: bool = if v23 {
                all_within_20(v0, v2)
            } else {
                false
            };
            if v25 {
                let v26: US0 = US0::US0_Reorder;
                US2::US2_Some(v26, v1)
            } else {
                US2::US2_None
            }
        }
    }
}
fn shape_of_13(mut v0: Rc<UH1>, mut v1: &Rc<UH1>) -> US2 {
    match &**v1 {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH1> = v3.clone();
            match &*v3 {
                UH1::UH1_Cons(v4, v5) => {
                    let v4: Rc<str> = v4.clone();
                    let mut v5: Rc<UH1> = v5.clone();
                    match Rc::unwrap_or_clone(v5) {
                        UH1::UH1_Cons(v6, v7) => {
                            let v6: Rc<str> = v6;
                            let v7: Rc<UH1> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH1::UH1_Nil => {
                                    let v8: bool = &*v4 == "|>";
                                    let v15: bool = if v8 {
                                        match &*v0 {
                                            UH1::UH1_Cons(v9, v10) => {
                                                let mut v9: Rc<str> = v9.clone();
                                                let v10: Rc<UH1> = v10.clone();
                                                let mut v11: bool = &*v9 == &*v2;
                                                if v11 {
                                                    match Rc::unwrap_or_clone(v10) {
                                                        UH1::UH1_Nil => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    }
                                                } else {
                                                    false
                                                }
                                            }
                                            _ => {
                                                false
                                            }
                                        }
                                    } else {
                                        false
                                    };
                                    let v17: bool = if v15 {
                                        identifier_16(v6.clone())
                                    } else {
                                        false
                                    };
                                    if v17 {
                                        let v18: US0 = passed_shape_19(v6.clone());
                                        US2::US2_Some(v18, v6)
                                    } else {
                                        called_shape_14(v0, v2, v3)
                                    }
                                }
                                _ => {
                                    called_shape_14(v0, v2, v3)
                                }
                            }
                        }
                        _ => {
                            called_shape_14(v0, v2, v3)
                        }
                    }
                }
                _ => {
                    called_shape_14(v0, v2, v3)
                }
            }
        }
        UH1::UH1_Nil => {
            US2::US2_None
        }
    }
}
fn wrapper_of_6(mut v0: Rc<str>, mut v1: i32) -> US1 {
    let v2: Rc<str> = definition_rest_7(v0);
    let v3: Rc<str> = head_word_9(v2.clone());
    let v4: bool = &*v3 == "";
    let mut v13: Rc<str> = if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let v6: i32 = (v3.clone().len() as i32);
        let v7: i32 = (v2.clone().len() as i32);
        let v8: i32 = v7.wrapping_sub(v6);
        let mut v9: usize = ((v6) as usize);
        let v10: i32 = v6.wrapping_add(v8);
        let mut v11: usize = ((v10) as usize);
        let mut v12: Rc<str> = Rc::<str>::from(&(v2)[v9..v11]);
        v12
    };
    let v14: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let (mut v15, mut v16): (Rc<UH1>, Rc<str>) = params_from_10(v13, v14);
    let v17: Rc<str> = string_concat(&[" ", &*v16]);
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
    let v19: Option<usize> = v17.find(&*v18);
    let v20: Option<usize> = method11(v19);
    let mut v21: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v22: Option<i32> = v20.map(|x| v21(x));
    let v24: i32 = v22.unwrap_or(-1i32);
    let v26: bool = if v4 {
        true
    } else {
        match &*v15 {
            UH1::UH1_Nil => {
                true
            }
            _ => {
                false
            }
        }
    };
    let v28: bool = if v26 {
        true
    } else {
        let v27: bool = v24 < 0i32;
        v27
    };
    if v28 {
        US1::US1_None
    } else {
        let v30: i32 = v24.wrapping_add(3i32);
        let v31: i32 = (v17.clone().len() as i32);
        let v32: i32 = v31.wrapping_sub(v24);
        let v33: i32 = v32.wrapping_sub(3i32);
        let v34: usize = ((v30) as usize);
        let v35: i32 = v30.wrapping_add(v33);
        let v36: usize = ((v35) as usize);
        let v37: Rc<str> = Rc::<str>::from(&(v17)[v34..v36]);
        let v38: Rc<str> = Rc::<str>::from((v37).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v40: Rc<UH1> = split_list_2(v39, v38);
        let v41: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let v42: Rc<UH1> = method12(v40, v41);
        let v43: US2 = shape_of_13(v15, &v42);
        match v43 {
            US2::US2_None => {
                US1::US1_None
            }
            US2::US2_Some(v44, v45) => {
                let v44: US0 = v44;
                let v45: Rc<str> = v45;
                US1::US1_Some(v3, v44, v45, v1)
            }
        }
    }
}
fn method21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
            let v2: Rc<str> = v2;
            let v3: US0 = v3;
            let v4: Rc<str> = v4;
            let mut v5: i32 = v5;
            let v6: Rc<UH0> = v6;
            let v7: Rc<UH0> = method21(v6, v1);
            Rc::new(UH0::UH0_Cons(v2, v3, v4, v5, v7))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method5(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: i32) -> (Rc<UH0>, i32) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                let mut v5: US1 = wrapper_of_6(v3, v2);
                let (mut v17, mut v18): (Rc<UH0>, i32) = match v5 {
                    US1::US1_None => {
                        let v14: i32 = v2.wrapping_add(1i32);
                        (v1, v14)
                    }
                    US1::US1_Some(v6, v7, v8, v9) => {
                        let v6: Rc<str> = v6;
                        let v7: US0 = v7;
                        let v8: Rc<str> = v8;
                        let mut v9: i32 = v9;
                        let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v7, v8, v9, v10));
                        let mut v12: Rc<UH0> = method21(v1, v11);
                        let mut v13: i32 = v2.wrapping_add(1i32);
                        (v12, v13)
                    }
                };
                (v0, v1, v2) = (v4, v17, v18);
                continue;
            }
            UH1::UH1_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn wrappers_in_1(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH1> = split_list_2(v1, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: i32 = 1i32;
    let (mut v8, mut v9): (Rc<UH0>, i32) = method5(v5, v6, v7);
    v8
}
fn wrapper_score_23(mut v0: &Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: i32, mut v4: i32) -> i32 {
    let v7: i32 = match v1 {
        US0::US0_Forward => {
            3i32
        }
        US0::US0_Rename => {
            3i32
        }
        US0::US0_Reorder => {
            2i32
        }
    };
    let v8: bool = v4 <= 1i32;
    let mut v11: i32 = if v8 {
        2i32
    } else {
        let mut v9: bool = v4 >= 3i32;
        if v9 {
            -1i32
        } else {
            0i32
        }
    };
    let mut v12: i32 = v7.wrapping_add(v11);
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v14: bool = v2.contains(&*v13);
    let v15: i32 = if v14 {
        1i32
    } else {
        0i32
    };
    let v16: i32 = v12.wrapping_add(v15);
    v16
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
            let v2: Rc<str> = v2;
            let v3: US0 = v3;
            let v4: Rc<str> = v4;
            let mut v5: i32 = v5;
            let v6: Rc<UH0> = v6;
            let v7: Rc<UH2> = method22(v6, v1);
            let mut v13: Rc<str> = match &v3 {
                US0::US0_Forward => {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("forward"); } LIT.with(|lit| lit.clone()) };
                    v8
                }
                US0::US0_Rename => {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rename"); } LIT.with(|lit| lit.clone()) };
                    v9
                }
                US0::US0_Reorder => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reorder"); } LIT.with(|lit| lit.clone()) };
                    v10
                }
            };
            let v14: i32 = 2i32;
            let v15: i32 = wrapper_score_23(&v2, v3, v4.clone(), v5, v14);
            Rc::new(UH2::UH2_Cons(v2, v13, v4, v5, v15, v7))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method24(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, i32)>>>, mut v1: Rc<UH2>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4, v5, v6, v7, v8) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
                let v6: i32 = v6;
                let v7: i32 = v7;
                let v8: Rc<UH2> = v8;
                v0.borrow_mut().push((v3, v4, v5, v6, v7));
                let mut v9: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v8, v9);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn wrapper_rows_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, i32)>>> {
    let v1: Rc<UH0> = wrappers_in_1(v0);
    let v2: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH2> = method22(v1, v2);
    let v4: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, i32)>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v5: i32 = 0i32;
    let v6: i32 = method24(v4.clone(), v3, v5);
    let v7: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, i32)>> = Rc::new(v4.borrow().clone());
    let v8: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, i32)>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    v8
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, i32)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, i32)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, i32)>>> {
        wrapper_rows_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_wrapper_rows(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, i32)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub type SpiralShape_accbf6de6842 = Mut0;
