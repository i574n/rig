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
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, Rc<str>, Rc<str>),
    US0_None,
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn method3(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method3(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method2(v16, v17)
    }
}
fn baseline_row_of_spi_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl lint_known_"); } LIT.with(|lit| lit.clone()) };
    let v4: bool = v0.starts_with(&*v3);
    let mut v5: bool = v4 == false;
    let v8: bool = if v5 {
        true
    } else {
        let v6: i32 = (v2.clone().borrow().len() as i32);
        let mut v7: bool = v6 < 7i32;
        v7
    };
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    } else {
        let v10: Rc<str> = v2.borrow()[1i32 as usize].clone();
        let v11: Rc<str> = v2.borrow()[3i32 as usize].clone();
        let mut v12: Rc<str> = v2.borrow()[5i32 as usize].clone();
        let v13: bool = &*v11 == "";
        if v13 {
            let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v14.borrow_mut().push(v10);
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":0  "); } LIT.with(|lit| lit.clone()) };
            v14.borrow_mut().push(v15);
            v14.borrow_mut().push(v12);
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  -"); } LIT.with(|lit| lit.clone()) };
            v14.borrow_mut().push(v16);
            let mut v17: Rc<Vec<Rc<str>>> = Rc::new(v14.borrow().clone());
            let v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v17).as_ref().clone()));
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = Rc::<str>::from((*v18.borrow()).join(&*v19));
            v20
        } else {
            let mut v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v21.borrow_mut().push(v10);
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":0  "); } LIT.with(|lit| lit.clone()) };
            v21.borrow_mut().push(v22);
            v21.borrow_mut().push(v11);
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            v21.borrow_mut().push(v23);
            v21.borrow_mut().push(v12);
            let v24: Rc<Vec<Rc<str>>> = Rc::new(v21.borrow().clone());
            let v25: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v24).as_ref().clone()));
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = Rc::<str>::from((*v25.borrow()).join(&*v26));
            v27
        }
    }
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method4(v3, v1);
            let mut v5: Rc<str> = string_concat(&["inl lint_known_", &*v2]);
            let v6: Rc<str> = baseline_row_of_spi_5(v5);
            Rc::new(UH0::UH0_Cons(v6, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method6(v3, v1);
            let mut v5: bool = &*v2 != "";
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
fn method7(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn baseline_rows_of_spi_0(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl lint_known_"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v6));
    let v8: Rc<UH0> = split_list_1(v1, v7);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH0> = method4(v8, v9);
    let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v12: Rc<UH0> = method6(v10, v11);
    let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v14: i32 = 0i32;
    let v15: i32 = method7(v13.clone(), v12, v14);
    let mut v16: Rc<Vec<Rc<str>>> = Rc::new(v13.borrow().clone());
    let mut v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v16).as_ref().clone()));
    let v18: Rc<str> = Rc::<str>::from((*v17.borrow()).join(&*v5));
    v18
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        baseline_rows_of_spi_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_9(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method10(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn words_in_15(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method2(v2, v5);
        } else {
            let mut v7: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v8: i32 = v1.wrapping_add(1i32);
            let mut v9: bool = &*v7 == "";
            let v11: Rc<UH0> = if v9 {
                v2
            } else {
                Rc::new(UH0::UH0_Cons(v7, v2))
            };
            (v1, v2) = (v8, v11);
            continue;
        }
    }
}
fn method16(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn known_of_14(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v4: i32 = 0i32;
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = words_in_15(&v3, v4, v5);
    match Rc::unwrap_or_clone(v6) {
        UH0::UH0_Cons(v7, v8) => {
            let mut v7: Rc<str> = v7;
            let v8: Rc<UH0> = v8;
            match Rc::unwrap_or_clone(v8) {
                UH0::UH0_Cons(v9, v10) => {
                    let mut v9: Rc<str> = v9;
                    let v10: Rc<UH0> = v10;
                    match Rc::unwrap_or_clone(v10) {
                        UH0::UH0_Cons(v11, v12) => {
                            let v11: Rc<str> = v11;
                            let mut v12: Rc<UH0> = v12;
                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                            let mut v14: Option<usize> = v7.rfind(&*v13);
                            let v15: Option<usize> = method16(v14);
                            let mut v16: Rc<dyn Fn((usize)) -> i32> = closure2();
                            let mut v17: Option<i32> = v15.map(|x| v16(x));
                            let v19: i32 = v17.unwrap_or(-1i32);
                            let v20: bool = &*v9 == "long";
                            let v28: bool = if v20 {
                                true
                            } else {
                                let mut v21: bool = &*v9 == "block";
                                if v21 {
                                    true
                                } else {
                                    let v22: bool = &*v9 == "flow";
                                    if v22 {
                                        true
                                    } else {
                                        let v23: bool = &*v9 == "cast";
                                        if v23 {
                                            true
                                        } else {
                                            let v24: bool = &*v9 == "arith";
                                            v24
                                        }
                                    }
                                }
                            };
                            let v39: bool = if v28 {
                                let v29: bool = &*v11 == "long";
                                let v37: bool = if v29 {
                                    true
                                } else {
                                    let v30: bool = &*v11 == "block";
                                    if v30 {
                                        true
                                    } else {
                                        let v31: bool = &*v11 == "flow";
                                        if v31 {
                                            true
                                        } else {
                                            let v32: bool = &*v11 == "cast";
                                            if v32 {
                                                true
                                            } else {
                                                let v33: bool = &*v11 == "arith";
                                                v33
                                            }
                                        }
                                    }
                                };
                                let v38: bool = v37 == false;
                                v38
                            } else {
                                false
                            };
                            let v40: bool = v19 <= 0i32;
                            if v40 {
                                US0::US0_None
                            } else {
                                let v42: usize = ((0i32) as usize);
                                let v43: usize = ((v19) as usize);
                                let v44: Rc<str> = Rc::<str>::from(&(v7)[v42..v43]);
                                if v39 {
                                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    US0::US0_Some(v44, v45, v9)
                                } else {
                                    US0::US0_Some(v44, v9, v11)
                                }
                            }
                        }
                        _ => {
                            US0::US0_None
                        }
                    }
                }
                _ => {
                    US0::US0_None
                }
            }
        }
        _ => {
            US0::US0_None
        }
    }
}
fn baseline_from_13(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: US0 = known_of_14(v5);
            match v6 {
                US0::US0_None => {
                    let v13: i32 = v1.wrapping_add(1i32);
                    v1 = v13;
                    continue;
                }
                US0::US0_Some(v7, v8, v9) => {
                    let mut v7: Rc<str> = v7;
                    let v8: Rc<str> = v8;
                    let mut v9: Rc<str> = v9;
                    let v10: i32 = v1.wrapping_add(1i32);
                    let v11: Rc<UH1> = Rc::new(UH1::UH1_Cons(v7, v8, v9, v2));
                    (v1, v2) = (v10, v11);
                    continue;
                }
            }
        }
    }
}
fn baseline_of_12(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v6: i32 = 0i32;
    let mut v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    baseline_from_13(v5, v6, v7)
}
fn method17(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH1> = v5;
                let v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn baseline_row_spi_19(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = format_real_9(v0);
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl lint_known_"); } LIT.with(|lit| lit.clone()) };
    v5.borrow_mut().push(v6);
    v5.borrow_mut().push(v4);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = LintKnown (\""); } LIT.with(|lit| lit.clone()) };
    v5.borrow_mut().push(v7);
    let v8: Rc<Vec<Rc<str>>> = Rc::new(v5.borrow().clone());
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
    let mut v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v12.borrow_mut().push(v11);
    v12.borrow_mut().push(v1);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    v12.borrow_mut().push(v13.clone());
    v12.borrow_mut().push(v2);
    v12.borrow_mut().push(v13);
    v12.borrow_mut().push(v3);
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
    v12.borrow_mut().push(v14);
    let v15: Rc<Vec<Rc<str>>> = Rc::new(v12.borrow().clone());
    let mut v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
    let mut v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v10));
    v17
}
fn method18(mut v0: Rc<UH1>, mut v1: i32, mut v2: Rc<UH0>) -> (i32, Rc<UH0>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
                let v6: Rc<UH1> = v6;
                let mut v7: i32 = v1.wrapping_add(1i32);
                let v8: Rc<str> = baseline_row_spi_19(v1, v3, v4, v5);
                let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v2));
                (v0, v1, v2) = (v6, v7, v9);
                continue;
            }
            UH1::UH1_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn baseline_spi_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<UH1> = baseline_of_12(v0);
    let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH1> = method17(v1, v2);
    let v4: i32 = 1i32;
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v6, mut v7): (i32, Rc<UH0>) = method18(v3, v4, v5);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method2(v7, v8);
    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v11: i32 = 0i32;
    let mut v12: i32 = method7(v10.clone(), v9, v11);
    let v13: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
    let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v13).as_ref().clone()));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = Rc::<str>::from((*v14.borrow()).join(&*v15));
    let mut v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v17.borrow_mut().push(v16);
    v17.borrow_mut().push(v15);
    let v18: Rc<Vec<Rc<str>>> = Rc::new(v17.borrow().clone());
    let v19: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v18).as_ref().clone()));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = Rc::<str>::from((*v19.borrow()).join(&*v20));
    v21
}
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method20(v3, v1);
            let mut v5: bool = &*v2 != "";
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
fn method21(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn baseline_written_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = baseline_spi_11(v1.clone());
    std::fs::write(&*v0, &*v2).unwrap();
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<UH0> = split_list_1(v4, v1);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method20(v5, v6);
    let v8: i32 = 0i32;
    let mut v9: i32 = method21(v7, v8);
    format_real_9(v9)
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        baseline_written_8(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_lint_baseline_rows(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_lint_baseline_write(v0: &str, v1: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
