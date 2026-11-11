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
    US0_Ok(regex::Regex),
    US0_Error(regex::Error),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
enum US1 {
    US1_Ok(Vec<std::string::String>),
    US1_Error(std::io::Error),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, Rc<UH1>),
}
pub struct Mut2 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
fn matches_from_3(mut v0: regex::Regex, mut v1: &Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v2.wrapping_sub(1i32);
            let mut v6: Rc<str> = v1.borrow()[v2 as usize].clone();
            let v7: bool = v0.is_match(&*v6);
            let mut v9: i32 = if v7 {
                let mut v8: i32 = v3.wrapping_add(1i32);
                v8
            } else {
                v3
            };
            (v2, v3) = (v5, v9);
            continue;
        }
    }
}
fn ok_c1() -> Rc<dyn Fn(regex::Regex) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(regex::Regex) -> US0> = Rc::new(move |mut v0: regex::Regex| -> US0 {
        US0::US0_Ok(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method4() -> Rc<dyn Fn(regex::Regex) -> US0> {
    ok_c1()
}
fn error_c2() -> Rc<dyn Fn(regex::Error) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(regex::Error) -> US0> = Rc::new(move |mut v0: regex::Error| -> US0 {
        US0::US0_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method5() -> Rc<dyn Fn(regex::Error) -> US0> {
    error_c2()
}
fn matching_lines_2(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>) -> i32 {
    let mut v3: Result<regex::Regex, regex::Error> = regex::Regex::new(&v1);
    let mut v4: Rc<dyn Fn(regex::Regex) -> US0> = method4();
    let mut v5: Rc<dyn Fn(regex::Error) -> US0> = method5();
    let mut v6: US0 = match v3 { Ok(x) => v4(x), Err(e) => v5(e) };
    match v6 {
        US0::US0_Error(v12) => {
            let v12: regex::Error = v12;
            0i32
        }
        US0::US0_Ok(v7) => {
            let v7: regex::Regex = v7;
            let mut v8: i32 = (v0.clone().borrow().len() as i32);
            let mut v9: i32 = v8.wrapping_sub(1i32);
            let mut v10: i32 = 0i32;
            matches_from_3(v7, v0, v9, v10)
        }
    }
}
fn method6(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn rows_from_1(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>>, mut v1: Rc<str>, mut v2: &Rc<RefCell<Vec<Rc<str>>>>, mut v3: i32, mut v4: i32, mut v5: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> {
    loop {
        let mut v6: i32 = (v0.clone().borrow().len() as i32);
        let v7: bool = v4 >= v6;
        if v7 {
            return v5;
        } else {
            let (mut v8, mut v9): (Rc<str>, Rc<str>) = v0.borrow()[v4 as usize].clone();
            let mut v10: i32 = matching_lines_2(v2, v9);
            let v11: i32 = v4.wrapping_add(1i32);
            let v12: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = Rc::new(RefCell::new(vec![(v1.clone(), v8, v10, v3)]));
            let mut v13: i32 = (v5.clone().borrow().len() as i32);
            let mut v14: i32 = (v12.clone().borrow().len() as i32);
            let v15: i32 = v13.wrapping_add(v14);
            let mut v16: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, Rc<str>, i32, i32)>::default(); v15 as usize]));
            let mut v17: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
            while method6(v15, &v17) {
                let v19: i32 = v17.borrow().l0;
                let v20: bool = v19 < v13;
                let (mut v30, mut v31, mut v32, mut v33): (Rc<str>, Rc<str>, i32, i32) = if v20 {
                    let (mut v21, mut v22, mut v23, mut v24): (Rc<str>, Rc<str>, i32, i32) = v5.borrow()[v19 as usize].clone();
                    (v21, v22, v23, v24)
                } else {
                    let v25: i32 = v19.wrapping_sub(v13);
                    let (mut v26, mut v27, mut v28, mut v29): (Rc<str>, Rc<str>, i32, i32) = v12.borrow()[v25 as usize].clone();
                    (v26, v27, v28, v29)
                };
                v16.borrow_mut()[v19 as usize] = (v30, v31, v32, v33);
                let mut v34: i32 = v19.wrapping_add(1i32);
                v17.borrow_mut().l0 = v34;
                ()
            };
            (v4, v5) = (v11, v16);
            continue;
        }
    }
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v2).unwrap_or_default().as_str());
                let mut v5: Rc<str> = string_concat(&[&*v1, &*v4]);
                let mut v6: bool = &*v4 == "";
                let mut v9: bool = if v6 {
                    true
                } else {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v8: bool = v4.ends_with(&*v7);
                    v8
                };
                let v12: Rc<str> = if v9 {
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v10
                } else {
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v11
                };
                let mut v13: Rc<str> = string_concat(&[&*v5, &*v12]);
                (v0, v1) = (v3, v13);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn paths_from_12(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v2.wrapping_sub(1i32);
            let v7: Rc<str> = v1.borrow()[v2 as usize].clone();
            let mut v8: Rc<str> = string_concat(&[&*v0, "/", &*v7]);
            let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v3));
            (v2, v3) = (v5, v9);
            continue;
        }
    }
}
fn ok_c3() -> Rc<dyn Fn(Vec<std::string::String>) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<std::string::String>) -> US1> = Rc::new(move |mut v0: Vec<std::string::String>| -> US1 {
        US1::US1_Ok(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method13() -> Rc<dyn Fn(Vec<std::string::String>) -> US1> {
    ok_c3()
}
fn error_c4() -> Rc<dyn Fn(std::io::Error) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US1> = Rc::new(move |mut v0: std::io::Error| -> US1 {
        US1::US1_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method14() -> Rc<dyn Fn(std::io::Error) -> US1> {
    error_c4()
}
fn method15(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2]));
                let mut v5: i32 = (v1.clone().borrow().len() as i32);
                let mut v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method6(v7, &v9) {
                    let v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let mut v16: Rc<str> = if v12 {
                        let mut v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13
                    } else {
                        let mut v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15
                    };
                    v8.borrow_mut()[v11 as usize] = v16;
                    let mut v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn closure5() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let v1: Rc<str> = (v0);
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method16() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure5()
}
fn method17(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: &Rc<RefCell<Mut1>>) -> bool {
    let mut v3: i32 = v2.borrow().l0;
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let mut v5: i32 = v2.borrow().l0;
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let v7: Rc<str> = v0.borrow()[v6 as usize].clone();
        let mut v8: bool = &*v7 < &*v1;
        let v11: i32 = if v8 {
            -1i32
        } else {
            let mut v9: bool = &*v7 > &*v1;
            if v9 {
                1i32
            } else {
                0i32
            }
        };
        let v12: bool = v11 == 1i32;
        v12
    } else {
        false
    }
}
fn child_paths_11(mut v0: Rc<str>) -> Rc<UH0> {
    let v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v3: Rc<dyn Fn(Vec<std::string::String>) -> US1> = method13();
    let mut v4: Rc<dyn Fn(std::io::Error) -> US1> = method14();
    let mut v5: US1 = match v2 { Ok(x) => v3(x), Err(e) => v4(e) };
    let v26: Rc<UH0> = match v5 {
        US1::US1_Error(v23) => {
            let v23: std::io::Error = v23;
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US1::US1_Ok(v6) => {
            let mut v6: Vec<std::string::String> = v6;
            let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v9: bool = true; let _vec_fold_ = v6.into_iter().fold(v7, |acc, x| { //;
            let v11: Rc<UH0> = acc;
            let mut v13: std::string::String = x;
            let v15: Rc<str> = Rc::<str>::from(String::as_str(&v13));
            let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v11));
            let v18: bool = true; v17 });
            let v20: Rc<UH0> = _vec_fold_;
            let v21: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            method10(v20, v21)
        }
    };
    let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v28: Rc<RefCell<Vec<Rc<str>>>> = method15(v26, v27);
    let v29: Rc<RefCell<Vec<Rc<str>>>> = v28 ;
    let mut v30: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method16();
    let mut v31: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v29.borrow().clone()));
    let mut v32: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v29.borrow().iter().map(|x| v30(x.clone())).collect::<Vec<_>>()));
    let v33: i32 = (v29.borrow().len() as i32);
    let mut v34: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 1i32 }));
    while method6(v33, &v34) {
        let v36: i32 = v34.borrow().l0;
        let v37: Rc<str> = v31.borrow()[v36 as usize].clone();
        let v38: Rc<str> = v32.borrow()[v36 as usize].clone();
        let mut v39: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v36 }));
        while method17(&v32, v38.clone(), &v39) {
            let v41: i32 = v39.borrow().l0;
            let v42: i32 = v41.wrapping_sub(1i32);
            let v43: Rc<str> = v31.borrow()[v42 as usize].clone();
            v31.borrow_mut()[v41 as usize] = v43;
            let v44: Rc<str> = v32.borrow()[v42 as usize].clone();
            v32.borrow_mut()[v41 as usize] = v44;
            v39.borrow_mut().l0 = v42;
            ()
        };
        let v45: i32 = v39.borrow().l0;
        v31.borrow_mut()[v45 as usize] = v37;
        let v46: i32 = v39.borrow().l0;
        v32.borrow_mut()[v46 as usize] = v38;
        let v47: i32 = v36.wrapping_add(1i32);
        v34.borrow_mut().l0 = v47;
        ()
    };
    let v48: Rc<RefCell<Vec<Rc<str>>>> = v31 ;
    let v49: i32 = (v48.clone().borrow().len() as i32);
    let v50: i32 = v49.wrapping_sub(1i32);
    let v51: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    paths_from_12(v0, v48, v50, v51)
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let mut v5: bool = v2.ends_with(&*v4);
                let v7: Rc<UH0> = if v5 {
                    Rc::new(UH0::UH0_Cons(v2, v1))
                } else {
                    v1
                };
                (v0, v1) = (v3, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn spi_paths_9(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<UH0> = child_paths_11(v0);
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH0> = method18(v1, v2);
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method10(v3, v4)
}
fn package_text_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<UH0> = spi_paths_9(v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method8(v1, v2)
}
fn lines_of_19(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: bool = v3.ends_with(&*v2);
    let mut v10: Rc<str> = if v5 {
        let mut v6: i32 = v4.wrapping_sub(1i32);
        let v7: usize = ((0i32) as usize);
        let mut v8: usize = ((v6) as usize);
        let mut v9: Rc<str> = Rc::<str>::from(&(v3)[v7..v8]);
        v9
    } else {
        v3.clone()
    };
    let v11: bool = &*v3 == "";
    if v11 {
        let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
        v12
    } else {
        let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v10.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        v13
    }
}
fn method21(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure6() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn package_name_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: bool = v3.ends_with(&*v2);
    let mut v10: Rc<str> = if v4 {
        let mut v5: i32 = (v3.clone().len() as i32);
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let v7: usize = ((0i32) as usize);
        let mut v8: usize = ((v6) as usize);
        let mut v9: Rc<str> = Rc::<str>::from(&(v3)[v7..v8]);
        v9
    } else {
        v3
    };
    let v11: Option<usize> = v10.rfind(&*v2);
    let v12: Option<usize> = method21(v11);
    let mut v13: Rc<dyn Fn((usize)) -> i32> = closure6();
    let mut v14: Option<i32> = v12.map(|x| v13(x));
    let mut v16: i32 = v14.unwrap_or(-1i32);
    let mut v17: bool = v16 < 0i32;
    if v17 {
        v10
    } else {
        let v18: i32 = v16.wrapping_add(1i32);
        let v19: i32 = (v10.clone().len() as i32);
        let v20: i32 = v19.wrapping_sub(v16);
        let v21: i32 = v20.wrapping_sub(1i32);
        let v22: usize = ((v18) as usize);
        let v23: i32 = v18.wrapping_add(v21);
        let v24: usize = ((v23) as usize);
        let v25: Rc<str> = Rc::<str>::from(&(v10)[v22..v24]);
        v25
    }
}
fn ends_test_block_23(mut v0: Rc<str>) -> bool {
    let v1: bool = &*v0 == "";
    if v1 {
        false
    } else {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":#"); } LIT.with(|lit| lit.clone()) };
        let mut v3: bool = v0.starts_with(&*v2);
        if v3 {
            false
        } else {
            let mut v4: usize = ((0i32) as usize);
            let mut v5: usize = ((1i32) as usize);
            let mut v6: Rc<str> = Rc::<str>::from(&(v0)[v4..v5]);
            let v7: Rc<str> = Rc::<str>::from((v6).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let mut v8: bool = &*v7 != "";
            v8
        }
    }
}
fn tests_from_22(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: bool, mut v3: i32) -> i32 {
    loop {
        let mut v4: i32 = (v0.clone().borrow().len() as i32);
        let mut v5: bool = v1 >= v4;
        if v5 {
            return v3;
        } else {
            let mut v6: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> "); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v6.starts_with(&*v7);
            let v11: bool = if v8 {
                true
            } else {
                let mut v9: bool = ends_test_block_23(v6);
                if v9 {
                    false
                } else {
                    v2
                }
            };
            let v12: i32 = v1.wrapping_add(1i32);
            let mut v14: i32 = if v11 {
                let mut v13: i32 = v3.wrapping_add(1i32);
                v13
            } else {
                v3
            };
            (v1, v2, v3) = (v12, v11, v14);
            continue;
        }
    }
}
fn measured_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> {
    let v1: Rc<str> = package_text_7(v0.clone());
    let v2: Rc<RefCell<Vec<Rc<str>>>> = lines_of_19(v1);
    let mut v3: Rc<str> = package_name_20(v0);
    let mut v4: i32 = (v2.clone().borrow().len() as i32);
    let mut v5: i32 = 0i32;
    let mut v6: bool = false;
    let v7: i32 = 0i32;
    let mut v8: i32 = tests_from_22(&v2, v5, v6, v7);
    let mut v9: i32 = v4.wrapping_sub(v8);
    let mut v10: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> = rig_style_probes::rig_style_probe_specs();
    let v11: i32 = 0i32;
    let v12: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = Rc::new(RefCell::new(vec![]));
    let mut v13: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = rows_from_1(&v10, v3, &v2, v9, v11, v12);
    v13
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> {
        measured_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method25(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2]));
                let mut v5: i32 = (v1.clone().borrow().len() as i32);
                let mut v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method6(v7, &v9) {
                    let v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let mut v16: Rc<str> = if v12 {
                        let mut v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13
                    } else {
                        let mut v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15
                    };
                    v8.borrow_mut()[v11 as usize] = v16;
                    let mut v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn list_of_base_26(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: i32 = v1.wrapping_sub(1i32);
            let mut v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v4, v6);
            continue;
        }
    }
}
fn method27(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH1> = method27(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, 0i32, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn directory_exists_30(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method33(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                (v0, v1) = (v3, v2);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method35(mut v0: i32, mut v1: &Rc<RefCell<Mut2>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_34(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: 0i32, l1: v8 }));
        while method35(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method10(v16, v17)
    }
}
fn last_segment_32(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v1));
    let mut v4: Rc<UH0> = split_list_34(v1, v3);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method33(v4, v5)
}
fn closure8(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    Rc::new(move |mut v2: Rc<UH0>| -> US2 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US2> = method36(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method36(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    closure8(v0, v1)
}
fn skipped_dir_31(mut v0: Rc<str>) -> bool {
    let v1: Rc<str> = last_segment_32(v0);
    let v2: i32 = 0i32;
    let mut v3: Rc<dyn Fn(Rc<UH0>) -> US2> = method36(v1, v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".git"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".cache"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bin"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("obj"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_build"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("vendor"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v13));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v14));
    let mut v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v15));
    let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v16));
    let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v17));
    let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v18));
    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v19));
    let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v20));
    let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v21));
    let v23: US2 = v3(v22);
    let v24: bool = match v23 {
        US2::US2_None => {
            true
        }
        _ => {
            false
        }
    };
    let v25: bool = v24 != true;
    v25
}
fn method29(mut v0: i32, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let mut v5: Rc<UH0> = method29(v0, v4, v2);
            let mut v6: bool = v0 < 8i32;
            let mut v8: bool = if v6 {
                directory_exists_30(v3.clone())
            } else {
                false
            };
            let v11: bool = if v8 {
                let mut v9: bool = skipped_dir_31(v3.clone());
                let mut v10: bool = v9 == false;
                v10
            } else {
                false
            };
            if v11 {
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
fn method37(mut v0: i32, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let mut v5: Rc<UH1> = method37(v0, v4, v2);
            let mut v6: i32 = v0.wrapping_add(1i32);
            Rc::new(UH1::UH1_Cons(v3, v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method38(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let v2: Rc<str> = v2;
            let mut v3: i32 = v3;
            let mut v4: Rc<UH1> = v4;
            let mut v5: Rc<UH1> = method38(v4, v1);
            Rc::new(UH1::UH1_Cons(v2, v3, v5))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn packages_from_28(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v4, v5, v6) => {
                let mut v4: Rc<str> = v4;
                let mut v5: i32 = v5;
                let mut v6: Rc<UH1> = v6;
                let v7: Rc<UH0> = spi_paths_9(v4.clone());
                let mut v8: bool = match Rc::unwrap_or_clone(v7) {
                    UH0::UH0_Nil => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v9: bool = v8 != true;
                let v11: Rc<UH0> = if v9 {
                    Rc::new(UH0::UH0_Cons(v4.clone(), v1))
                } else {
                    v1
                };
                let v12: Rc<UH0> = child_paths_11(v4);
                let mut v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v14: Rc<UH0> = method29(v5, v12, v13);
                let v15: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let mut v16: Rc<UH1> = method37(v5, v14, v15);
                let mut v17: Rc<UH1> = method38(v16, v6);
                (v0, v1) = (v17, v11);
                continue;
            }
            UH1::UH1_Nil => {
                let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method10(v1, v2);
            }
        }
    }
}
fn packages_under_24(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v1: i32 = (v0.clone().borrow().len() as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = list_of_base_26(&v0, v2, v3);
    let mut v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH1> = method27(v4, v5);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = packages_from_28(v6, v7);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let mut v10: Rc<RefCell<Vec<Rc<str>>>> = method25(v8, v9);
    v10
}
fn closure7() -> Rc<dyn Fn(Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<RefCell<Vec<Rc<str>>>>| -> Rc<RefCell<Vec<Rc<str>>>> {
        packages_under_24(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_style_measured(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_style_packages_under(v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure7()(v0)
}
use rig_style_probes::SpiralShape_2617b4e49837 as Mut0;
use rig_style_probes::SpiralShape_2617b4e49837 as Mut1;
pub type SpiralShape_652accfc6a8d = Mut2;
