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
    US0_PathFirst(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US0, Rc<UH1>),
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method4(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
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
fn method5(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method5(v3, v1);
            let v5: bool = &*v2 != "";
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
fn same_dir_8(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let mut v6: Rc<str> = Rc::<str>::from(v5.to_lowercase());
    let mut v7: bool = v6.ends_with(&*v4);
    let v13: Rc<str> = if v7 {
        let mut v8: i32 = (v6.clone().len() as i32);
        let mut v9: i32 = v8.wrapping_sub(1i32);
        let v10: usize = ((0i32) as usize);
        let v11: usize = ((v9) as usize);
        let v12: Rc<str> = Rc::<str>::from(&(v6)[v10..v11]);
        v12
    } else {
        v6
    };
    let v14: Rc<str> = Rc::<str>::from((v1).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v15: Rc<str> = Rc::<str>::from(v14.replace(&*v3, &*v4));
    let v16: Rc<str> = Rc::<str>::from(v15.to_lowercase());
    let v17: bool = v16.ends_with(&*v4);
    let v23: Rc<str> = if v17 {
        let v18: i32 = (v16.clone().len() as i32);
        let v19: i32 = v18.wrapping_sub(1i32);
        let v20: usize = ((0i32) as usize);
        let v21: usize = ((v19) as usize);
        let v22: Rc<str> = Rc::<str>::from(&(v16)[v20..v21]);
        v22
    } else {
        v16
    };
    let v24: bool = &*v13 == &*v23;
    v24
}
fn method7(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: bool) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let mut v6: bool = if v2 {
                    true
                } else {
                    same_dir_8(v3, v0.clone())
                };
                (v1, v2) = (v4, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method6(v0.clone(), v4, v2);
            let mut v6: bool = false;
            let mut v7: bool = method7(v3.clone(), v0, v6);
            let mut v8: bool = v7 == false;
            if v8 {
                Rc::new(UH0::UH0_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method9(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method10(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn path_with_first_1(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<UH0> = split_list_2(v2.clone(), v0);
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH0> = method5(v3, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method6(v1.clone(), v5, v6);
    let mut v8: Rc<UH0> = method9(v1, v7);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v10: i32 = 0i32;
    let v11: i32 = method10(v9.clone(), v8, v10);
    let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
    let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
    let v14: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v2));
    v14
}
fn method13(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method14(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: US0 = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method14(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = PathFirst ("); } LIT.with(|lit| lit.clone()) };
                let v5: Option<usize> = v2.find(&*v4);
                let mut v6: Option<usize> = method13(v5);
                let mut v7: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v8: Option<i32> = v6.map(|x| v7(x));
                let v10: i32 = v8.unwrap_or(-1i32);
                let v11: bool = v10 < 0i32;
                let v42: Rc<UH1> = if v11 {
                    v1
                } else {
                    let v12: i32 = v10.wrapping_add(14i32);
                    let v13: i32 = (v2.clone().len() as i32);
                    let v14: i32 = v13.wrapping_sub(v10);
                    let v15: i32 = v14.wrapping_sub(14i32);
                    let v16: usize = ((v12) as usize);
                    let v17: i32 = v12.wrapping_add(v15);
                    let v18: usize = ((v17) as usize);
                    let v19: Rc<str> = Rc::<str>::from(&(v2)[v16..v18]);
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<UH0> = split_list_2(v20, v19);
                    match Rc::unwrap_or_clone(v21) {
                        UH0::UH0_Cons(v22, v23) => {
                            let v22: Rc<str> = v22;
                            let v23: Rc<UH0> = v23;
                            match Rc::unwrap_or_clone(v23) {
                                UH0::UH0_Cons(v24, v25) => {
                                    let v24: Rc<str> = v24;
                                    let v25: Rc<UH0> = v25;
                                    match Rc::unwrap_or_clone(v25) {
                                        UH0::UH0_Cons(v26, v27) => {
                                            let v26: Rc<str> = v26;
                                            let v27: Rc<UH0> = v27;
                                            match Rc::unwrap_or_clone(v27) {
                                                UH0::UH0_Cons(v28, v29) => {
                                                    let v28: Rc<str> = v28;
                                                    let v29: Rc<UH0> = v29;
                                                    let v30: bool = &*v24 != "";
                                                    let v32: bool = if v30 {
                                                        let v31: bool = &*v28 != "";
                                                        v31
                                                    } else {
                                                        false
                                                    };
                                                    if v32 {
                                                        let v33: US0 = US0::US0_PathFirst(v24, v28);
                                                        let v34: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                                                        let v35: Rc<UH1> = Rc::new(UH1::UH1_Cons(v33, v34));
                                                        method14(v1, v35)
                                                    } else {
                                                        v1
                                                    }
                                                }
                                                _ => {
                                                    v1
                                                }
                                            }
                                        }
                                        _ => {
                                            v1
                                        }
                                    }
                                }
                                _ => {
                                    v1
                                }
                            }
                        }
                        _ => {
                            v1
                        }
                    }
                };
                (v0, v1) = (v3, v42);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn path_first_rows_11(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_2(v1, v4);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method12(v5, v6)
}
fn row_dir_16(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_PathFirst(v1, v2) => {
            let v1: Rc<str> = v1;
            let v2: Rc<str> = v2;
            v2
        }
    }
}
fn method15(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: US0 = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH0> = method15(v3, v1);
            let v5: Rc<str> = row_dir_16(v2);
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn directory_exists_18(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method17(v3, v1);
            let v5: bool = directory_exists_18(v2.clone());
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
fn path_first_value_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = string_concat(&[&*v1, "/path_first.spi"]);
    let v3: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v2).unwrap_or_default().as_str());
    let v4: Rc<UH1> = path_first_rows_11(v3);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = method15(v4, v5);
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = method17(v6, v7);
    let mut v9: bool = match &*v8 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v10
    } else {
        let v11: bool = cfg!(windows);
        let v14: Rc<str> = if v11 {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
            v12
        } else {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            v13
        };
        path_with_first_1(v0, v8, v14)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        path_first_value_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_path_first_value(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_652accfc6a8d = Mut0;
