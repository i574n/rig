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
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_Some(i32),
    US0_None,
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method4(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method4(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method3(v16, v17)
    }
}
fn method6(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn name_end_7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v2 <= v1;
        if v3 {
            return v1;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: i32 = v1.wrapping_add(1i32);
            let mut v6: usize = ((v5) as usize);
            let v7: Rc<str> = Rc::<str>::from(&(v0)[v4..v6]);
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_"); } LIT.with(|lit| lit.clone()) };
            let mut v9: bool = v8.contains(&*v7);
            if v9 {
                v1 = v5;
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn references_from_5(mut v0: Rc<str>, mut v1: i32) -> Rc<UH0> {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let v3: i32 = v1.wrapping_add(1i32);
        let v4: bool = v2 <= v3;
        if v4 {
            return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        } else {
            let mut v6: usize = ((v1) as usize);
            let v7: usize = ((v3) as usize);
            let mut v8: Rc<str> = Rc::<str>::from(&(v0)[v6..v7]);
            let mut v9: bool = &*v8 != "$";
            if v9 {
                v1 = v3;
                continue;
            } else {
                let v11: usize = ((v3) as usize);
                let v12: i32 = v3.wrapping_add(1i32);
                let v13: usize = ((v12) as usize);
                let v14: Rc<str> = Rc::<str>::from(&(v0)[v11..v13]);
                let v15: i32 = v1.wrapping_add(2i32);
                let v16: i32 = v2.wrapping_sub(v1);
                let v17: i32 = v16.wrapping_sub(2i32);
                let v18: usize = ((v15) as usize);
                let v19: i32 = v15.wrapping_add(v17);
                let v20: usize = ((v19) as usize);
                let v21: Rc<str> = Rc::<str>::from(&(v0)[v18..v20]);
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
                let v23: Option<usize> = v21.find(&*v22);
                let v24: Option<usize> = method6(v23);
                let mut v25: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v26: Option<i32> = v24.map(|x| v25(x));
                let v28: i32 = v26.unwrap_or(-1i32);
                let v29: bool = &*v14 == "$";
                if v29 {
                    v1 = v15;
                    continue;
                } else {
                    let v31: bool = &*v14 == "{";
                    let v33: bool = if v31 {
                        let v32: bool = v28 > 0i32;
                        v32
                    } else {
                        false
                    };
                    if v33 {
                        let v34: usize = ((0i32) as usize);
                        let v35: usize = ((v28) as usize);
                        let v36: Rc<str> = Rc::<str>::from(&(v21)[v34..v35]);
                        let v37: i32 = v1.wrapping_add(3i32);
                        let v38: i32 = v37.wrapping_add(v28);
                        let v39: Rc<UH0> = references_from_5(v0, v38);
                        return Rc::new(UH0::UH0_Cons(v36, v39));
                    } else {
                        let v41: i32 = name_end_7(v0.clone(), v3);
                        let v42: bool = v41 == v3;
                        if v42 {
                            v1 = v3;
                            continue;
                        } else {
                            let v44: i32 = v41.wrapping_sub(v1);
                            let v45: i32 = v44.wrapping_sub(1i32);
                            let v46: usize = ((v3) as usize);
                            let v47: i32 = v3.wrapping_add(v45);
                            let v48: usize = ((v47) as usize);
                            let v49: Rc<str> = Rc::<str>::from(&(v0)[v46..v48]);
                            let v50: Rc<UH0> = references_from_5(v0, v41);
                            return Rc::new(UH0::UH0_Cons(v49, v50));
                        }
                    }
                }
            }
        }
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    Rc::new(move |mut v2: Rc<UH0>| -> US0 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US0::US0_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US0> = method9(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US0::US0_None
            }
        }
    })
}
fn method9(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    closure2(v0, v1)
}
fn unknown_reference_8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: i32 = 0i32;
                let mut v6: Rc<dyn Fn(Rc<UH0>) -> US0> = method9(v3.clone(), v5);
                let v7: US0 = v6(v0.clone());
                let mut v8: bool = match v7 {
                    US0::US0_None => {
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
                    return v3;
                }
            }
            UH0::UH0_Nil => {
                let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v2;
            }
        }
    }
}
fn digits_end_11(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v2 <= v1;
        if v3 {
            return v1;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: i32 = v1.wrapping_add(1i32);
            let mut v6: usize = ((v5) as usize);
            let v7: Rc<str> = Rc::<str>::from(&(v0)[v4..v6]);
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) };
            let mut v9: bool = v8.contains(&*v7);
            if v9 {
                v1 = v5;
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn known_prefix_12(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 <= 0i32;
        if v3 {
            return 0i32;
        } else {
            let v4: usize = ((0i32) as usize);
            let v5: usize = ((v2) as usize);
            let mut v6: Rc<str> = Rc::<str>::from(&(v1)[v4..v5]);
            let v7: i32 = 0i32;
            let mut v8: Rc<dyn Fn(Rc<UH0>) -> US0> = method9(v6, v7);
            let mut v9: US0 = v8(v0.clone());
            let v10: bool = match v9 {
                US0::US0_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v11: bool = v10 != true;
            if v11 {
                return v2;
            } else {
                let v12: i32 = v2.wrapping_sub(1i32);
                v2 = v12;
                continue;
            }
        }
    }
}
fn braced_hint_10(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: i32 = 0i32;
    let v3: i32 = digits_end_11(v1.clone(), v2);
    let v4: i32 = known_prefix_12(v0, v1.clone(), v3);
    let v5: usize = ((0i32) as usize);
    let mut v6: usize = ((v4) as usize);
    let v7: Rc<str> = Rc::<str>::from(&(v1)[v5..v6]);
    let mut v8: i32 = (v1.clone().len() as i32);
    let mut v9: i32 = v8.wrapping_sub(v4);
    let v10: usize = ((v4) as usize);
    let v11: i32 = v4.wrapping_add(v9);
    let v12: usize = ((v11) as usize);
    let v13: Rc<str> = Rc::<str>::from(&(v1)[v10..v12]);
    let v14: bool = v4 > 0i32;
    if v14 {
        let v18: Rc<str> = string_concat(&["; did you mean ${", &*v7, "}", &*v13, "?"]);
        v18
    } else {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v19
    }
}
fn replacement_problem_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH0> = split_list_2(v2.clone(), v0.clone());
    let v4: i32 = 0i32;
    let v5: Rc<UH0> = references_from_5(v1, v4);
    let mut v6: Rc<str> = unknown_reference_8(v3.clone(), v5);
    let v7: bool = &*v6 == "";
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v8
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v12));
        let v15: Rc<str> = string_concat(&["the replacement names group ", &*v6, ", which the pattern does not define (groups ", &*v13, ")"]);
        let v16: Rc<str> = braced_hint_10(v3, v6);
        let v17: Rc<str> = string_concat(&[&*v15, &*v16]);
        v17
    }
}
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method15(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn char_at_16(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v1 < v2;
    if v3 {
        let v4: usize = ((v1) as usize);
        let v5: i32 = v1.wrapping_add(1i32);
        let mut v6: usize = ((v5) as usize);
        let v7: Rc<str> = Rc::<str>::from(&(v0)[v4..v6]);
        v7
    } else {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v8
    }
}
fn numbers_upto_17(mut v0: i32, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v2: bool = v0 < 0i32;
        if v2 {
            return v1;
        } else {
            let v3: i32 = v0.wrapping_sub(1i32);
            let v4: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
            let v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v1));
            (v0, v1) = (v3, v5);
            continue;
        }
    }
}
fn class_body_18(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: Rc<str> = char_at_16(v0.clone(), v1);
    let v3: bool = &*v2 == "^";
    let v5: i32 = if v3 {
        let v4: i32 = v1.wrapping_add(1i32);
        v4
    } else {
        v1
    };
    let mut v6: Rc<str> = char_at_16(v0, v5);
    let v7: bool = &*v6 == "]";
    if v7 {
        let mut v8: i32 = v5.wrapping_add(1i32);
        v8
    } else {
        v5
    }
}
fn class_end_19(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: Rc<str> = char_at_16(v0.clone(), v1);
        let v3: bool = &*v2 == "";
        if v3 {
            return v1;
        } else {
            let v4: bool = &*v2 == "\\";
            if v4 {
                let v5: i32 = v1.wrapping_add(2i32);
                v1 = v5;
                continue;
            } else {
                let v7: bool = &*v2 == "]";
                if v7 {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    return v8;
                } else {
                    let mut v9: i32 = v1.wrapping_add(1i32);
                    v1 = v9;
                    continue;
                }
            }
        }
    }
}
fn group_name_end_20(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: Rc<str> = char_at_16(v0.clone(), v1);
        let v3: bool = &*v2 == "";
        let v5: bool = if v3 {
            true
        } else {
            let v4: bool = &*v2 == ">";
            v4
        };
        if v5 {
            return v1;
        } else {
            let mut v6: i32 = v1.wrapping_add(1i32);
            v1 = v6;
            continue;
        }
    }
}
fn groups_from_14(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: Rc<str> = char_at_16(v0.clone(), v1);
        let v5: bool = &*v4 == "";
        if v5 {
            let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v7: Rc<UH0> = numbers_upto_17(v2, v6);
            return method15(v7, v3);
        } else {
            let mut v9: bool = &*v4 == "\\";
            if v9 {
                let v10: i32 = v1.wrapping_add(2i32);
                v1 = v10;
                continue;
            } else {
                let v12: bool = &*v4 == "[";
                if v12 {
                    let v13: i32 = v1.wrapping_add(1i32);
                    let v14: i32 = class_body_18(v0.clone(), v13);
                    let v15: i32 = class_end_19(v0.clone(), v14);
                    v1 = v15;
                    continue;
                } else {
                    let v17: bool = &*v4 != "(";
                    if v17 {
                        let v18: i32 = v1.wrapping_add(1i32);
                        v1 = v18;
                        continue;
                    } else {
                        let v20: i32 = v1.wrapping_add(1i32);
                        let v21: Rc<str> = char_at_16(v0.clone(), v20);
                        let v22: bool = &*v21 != "?";
                        if v22 {
                            let v23: i32 = v2.wrapping_add(1i32);
                            (v1, v2) = (v20, v23);
                            continue;
                        } else {
                            let mut v25: i32 = v1.wrapping_add(2i32);
                            let mut v26: Rc<str> = char_at_16(v0.clone(), v25);
                            let v27: bool = &*v26 == "P";
                            let v29: i32 = if v27 {
                                let v28: i32 = v1.wrapping_add(3i32);
                                v28
                            } else {
                                v25
                            };
                            let v30: Rc<str> = char_at_16(v0.clone(), v29);
                            let v31: i32 = v29.wrapping_add(1i32);
                            let v32: Rc<str> = char_at_16(v0.clone(), v31);
                            let v33: bool = &*v30 == "<";
                            let v35: bool = if v33 {
                                let v34: bool = &*v32 != "=";
                                v34
                            } else {
                                false
                            };
                            let v37: bool = if v35 {
                                let v36: bool = &*v32 != "!";
                                v36
                            } else {
                                false
                            };
                            if v37 {
                                let v38: i32 = group_name_end_20(v0.clone(), v31);
                                let v39: i32 = v38.wrapping_sub(v29);
                                let v40: i32 = v39.wrapping_sub(1i32);
                                let v41: usize = ((v31) as usize);
                                let v42: i32 = v31.wrapping_add(v40);
                                let v43: usize = ((v42) as usize);
                                let v44: Rc<str> = Rc::<str>::from(&(v0)[v41..v43]);
                                let v45: i32 = v38.wrapping_add(1i32);
                                let v46: i32 = v2.wrapping_add(1i32);
                                let v47: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v44, v47));
                                let v49: Rc<UH0> = method15(v3, v48);
                                (v1, v2, v3) = (v45, v46, v49);
                                continue;
                            } else {
                                v1 = v20;
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method21(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn pattern_groups_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 0i32;
    let mut v2: i32 = 0i32;
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH0> = groups_from_14(v0, v1, v2, v3);
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v6: i32 = 0i32;
    let v7: i32 = method21(v5.clone(), v4, v6);
    let mut v8: Rc<Vec<Rc<str>>> = Rc::new(v5.borrow().clone());
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
    v11
}
fn group_problem_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = pattern_groups_13(v1);
    replacement_problem_1(v2, v0)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        group_problem_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_do_replacement_problem(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_652accfc6a8d = Mut0;
