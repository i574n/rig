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
    US0_Plain,
    US0_Model,
    US0_Boundary,
    US0_Feature,
    US0_Bin,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, US0, Rc<UH1>, Rc<UH1>, Rc<UH1>, Rc<str>, Rc<UH1>, bool, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32 }
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, US0, Rc<UH1>, Rc<UH1>, Rc<UH1>, Rc<str>, Rc<UH1>, bool),
    US1_None,
}
pub struct Mut1 { pub l0: i32, pub l1: Rc<UH1> }
fn kind_word_4(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_Bin => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bin"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US0::US0_Boundary => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("boundary"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_Feature => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("feature"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_Model => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("model"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_Plain => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plain"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    }
}
fn method7(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method6(mut v0: Rc<UH1>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2]));
                let mut v5: i32 = (v1.clone().borrow().len() as i32);
                let v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method7(v7, &v9) {
                    let v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let mut v16: Rc<str> = if v12 {
                        let v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13
                    } else {
                        let v14: i32 = v11.wrapping_sub(v5);
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
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn base_of_list_5(mut v0: Rc<UH1>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v2: Rc<RefCell<Vec<Rc<str>>>> = method6(v0, v1);
    v2
}
fn row_of_3(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<UH1>, mut v3: Rc<UH1>, mut v4: Rc<UH1>, mut v5: Rc<str>, mut v6: Rc<UH1>, mut v7: bool) -> (Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool) {
    let mut v8: Rc<str> = kind_word_4(v1);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = base_of_list_5(v2);
    let mut v10: Rc<RefCell<Vec<Rc<str>>>> = base_of_list_5(v3);
    let v11: Rc<RefCell<Vec<Rc<str>>>> = base_of_list_5(v4);
    let v12: Rc<RefCell<Vec<Rc<str>>>> = base_of_list_5(v6);
    (v0, v8, v9, v10, v11, v5, v12, v7)
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
                let v2: Rc<str> = v2;
                let v3: US0 = v3;
                let mut v4: Rc<UH1> = v4;
                let mut v5: Rc<UH1> = v5;
                let v6: Rc<UH1> = v6;
                let v7: Rc<str> = v7;
                let mut v8: Rc<UH1> = v8;
                let mut v9: bool = v9;
                let mut v10: Rc<UH0> = v10;
                let (mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18): (Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool) = row_of_3(v2, v3, v4, v5, v6, v7, v8, v9);
                let mut v19: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>> = Rc::new(RefCell::new(vec![(v11, v12, v13, v14, v15, v16, v17, v18)]));
                let mut v20: i32 = (v1.clone().borrow().len() as i32);
                let v21: i32 = (v19.clone().borrow().len() as i32);
                let mut v22: i32 = v20.wrapping_add(v21);
                let mut v23: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>::default(); v22 as usize]));
                let mut v24: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method7(v22, &v24) {
                    let v26: i32 = v24.borrow().l0;
                    let v27: bool = v26 < v20;
                    let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool) = if v27 {
                        let (mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35): (Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool) = v1.borrow()[v26 as usize].clone();
                        (v28, v29, v30, v31, v32, v33, v34, v35)
                    } else {
                        let v36: i32 = v26.wrapping_sub(v20);
                        let (mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44): (Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool) = v19.borrow()[v36 as usize].clone();
                        (v37, v38, v39, v40, v41, v42, v43, v44)
                    };
                    v23.borrow_mut()[v26 as usize] = (v45, v46, v47, v48, v49, v50, v51, v52);
                    let v53: i32 = v26.wrapping_add(1i32);
                    v24.borrow_mut().l0 = v53;
                    ()
                };
                (v0, v1) = (v10, v23);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn rows_of_1(mut v0: Rc<UH0>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>> {
    let v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>> = Rc::new(RefCell::new(vec![]));
    let v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>> = method2(v0, v1);
    v2
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
                let v2: Rc<str> = v2;
                let v3: US0 = v3;
                let mut v4: Rc<UH1> = v4;
                let mut v5: Rc<UH1> = v5;
                let v6: Rc<UH1> = v6;
                let v7: Rc<str> = v7;
                let mut v8: Rc<UH1> = v8;
                let mut v9: bool = v9;
                let mut v10: Rc<UH0> = v10;
                let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v1));
                (v0, v1) = (v10, v11);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method11(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let mut v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn names_in_10(mut v0: Rc<str>) -> Rc<UH1> {
    let v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Vec<std::string::String> = v2.unwrap();
    let mut v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v7: bool = true; let _vec_fold_ = v4.into_iter().fold(v5, |acc, x| { //;
    let mut v9: Rc<UH1> = acc;
    let v11: std::string::String = x;
    let v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
    let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v9));
    let mut v16: bool = true; v15 });
    let v18: Rc<UH1> = _vec_fold_;
    let mut v19: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method11(v18, v19)
}
fn method18(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method18(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH1> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH1> = v9.borrow().l1.clone();
        let mut v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method11(v16, v17)
    }
}
fn lines_of_16(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    split_list_17(v1, v4)
}
fn method20(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let mut v4: Rc<UH1> = method20(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method19(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>, mut v3: bool) -> (Rc<UH1>, bool) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH1> = v5;
                let v6: Rc<str> = Rc::<str>::from((v4).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v7: Rc<str> = string_concat(&[&*v0, " = '''"]);
                let mut v8: bool = v6.starts_with(&*v7);
                let (mut v20, mut v21): (Rc<UH1>, bool) = if v8 {
                    (v2, true)
                } else {
                    let mut v10: bool = if v3 {
                        let mut v9: bool = &*v6 == "'''";
                        v9
                    } else {
                        false
                    };
                    if v10 {
                        (v2, false)
                    } else {
                        let v12: bool = if v3 {
                            let v11: bool = &*v6 != "";
                            v11
                        } else {
                            false
                        };
                        if v12 {
                            let v13: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                            let v14: Rc<UH1> = Rc::new(UH1::UH1_Cons(v6, v13));
                            let v15: Rc<UH1> = method20(v2, v14);
                            (v15, true)
                        } else {
                            (v2, v3)
                        }
                    }
                };
                (v1, v2, v3) = (v5, v20, v21);
                continue;
            }
            UH1::UH1_Nil => {
                return (v2, v3);
            }
        }
    }
}
fn toml_block_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: Rc<UH1> = lines_of_16(v0);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: bool = false;
    let (mut v5, mut v6): (Rc<UH1>, bool) = method19(v1, v2, v3, v4);
    v5
}
fn method24(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn entry_name_23(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
    let v6: bool = v4.starts_with(&*v5);
    let v13: Rc<str> = if v6 {
        let v7: i32 = (v4.clone().len() as i32);
        let mut v8: i32 = v7.wrapping_sub(1i32);
        let mut v9: usize = ((1i32) as usize);
        let mut v10: i32 = 1i32.wrapping_add(v8);
        let v11: usize = ((v10) as usize);
        let v12: Rc<str> = Rc::<str>::from(&(v4)[v9..v11]);
        v12
    } else {
        v4
    };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v15: bool = v13.ends_with(&*v14);
    let v21: Rc<str> = if v15 {
        let mut v16: i32 = (v13.clone().len() as i32);
        let mut v17: i32 = v16.wrapping_sub(1i32);
        let v18: usize = ((0i32) as usize);
        let mut v19: usize = ((v17) as usize);
        let mut v20: Rc<str> = Rc::<str>::from(&(v13)[v18..v19]);
        v20
    } else {
        v13
    };
    let mut v22: bool = v21.ends_with(&*v3);
    let mut v28: Rc<str> = if v22 {
        let mut v23: i32 = (v21.clone().len() as i32);
        let mut v24: i32 = v23.wrapping_sub(1i32);
        let v25: usize = ((0i32) as usize);
        let v26: usize = ((v24) as usize);
        let v27: Rc<str> = Rc::<str>::from(&(v21)[v25..v26]);
        v27
    } else {
        v21
    };
    let v29: Option<usize> = v28.rfind(&*v3);
    let v30: Option<usize> = method24(v29);
    let mut v31: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v32: Option<i32> = v30.map(|x| v31(x));
    let v34: i32 = v32.unwrap_or(-1i32);
    let v35: bool = v34 < 0i32;
    if v35 {
        v28
    } else {
        let v36: i32 = v34.wrapping_add(1i32);
        let v37: i32 = (v28.clone().len() as i32);
        let v38: i32 = v37.wrapping_sub(v34);
        let v39: i32 = v38.wrapping_sub(1i32);
        let v40: usize = ((v36) as usize);
        let v41: i32 = v36.wrapping_add(v39);
        let v42: usize = ((v41) as usize);
        let v43: Rc<str> = Rc::<str>::from(&(v28)[v40..v42]);
        v43
    }
}
fn method22(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>, mut v3: bool) -> (Rc<UH1>, bool) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH1> = v5;
                let v6: Rc<str> = string_concat(&[&*v0, ":"]);
                let v7: bool = v4.starts_with(&*v6);
                let (mut v24, mut v25): (Rc<UH1>, bool) = if v7 {
                    (v2, true)
                } else {
                    let mut v10: bool = if v3 {
                        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                        let mut v9: bool = v4.starts_with(&*v8);
                        v9
                    } else {
                        false
                    };
                    let v13: bool = if v10 {
                        let v11: Rc<str> = Rc::<str>::from((v4).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                        let v12: bool = &*v11 != "";
                        v12
                    } else {
                        false
                    };
                    if v13 {
                        let v14: Rc<str> = entry_name_23(v4);
                        let v15: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let mut v16: Rc<UH1> = Rc::new(UH1::UH1_Cons(v14, v15));
                        let mut v17: Rc<UH1> = method20(v2, v16);
                        (v17, true)
                    } else {
                        let v18: Rc<str> = Rc::<str>::from((v4).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                        let mut v19: bool = &*v18 == "";
                        if v19 {
                            (v2, v3)
                        } else {
                            (v2, false)
                        }
                    }
                };
                (v1, v2, v3) = (v5, v24, v25);
                continue;
            }
            UH1::UH1_Nil => {
                return (v2, v3);
            }
        }
    }
}
fn spiproj_section_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: Rc<UH1> = lines_of_16(v0);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: bool = false;
    let (mut v5, mut v6): (Rc<UH1>, bool) = method22(v1, v2, v3, v4);
    v5
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn quoted_after_27(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = Rc::<str>::from((v1).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v3: Rc<str> = string_concat(&[&*v0, " = \""]);
    let mut v4: bool = v2.starts_with(&*v3);
    let mut v5: bool = v4 == false;
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        let v7: i32 = (v0.len() as i32);
        let mut v8: i32 = v7.wrapping_add(4i32);
        let mut v9: i32 = (v2.clone().len() as i32);
        let mut v10: i32 = v9.wrapping_sub(v7);
        let v11: i32 = v10.wrapping_sub(4i32);
        let v12: usize = ((v8) as usize);
        let v13: i32 = v8.wrapping_add(v11);
        let v14: usize = ((v13) as usize);
        let v15: Rc<str> = Rc::<str>::from(&(v2)[v12..v14]);
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v17: Option<usize> = v15.find(&*v16);
        let v18: Option<usize> = method24(v17);
        let mut v19: Rc<dyn Fn((usize)) -> i32> = closure2();
        let mut v20: Option<i32> = v18.map(|x| v19(x));
        let mut v22: i32 = v20.unwrap_or(-1i32);
        let mut v23: bool = v22 < 0i32;
        if v23 {
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24
        } else {
            let v25: usize = ((0i32) as usize);
            let v26: usize = ((v22) as usize);
            let v27: Rc<str> = Rc::<str>::from(&(v15)[v25..v26]);
            v27
        }
    }
}
fn method26(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH1> = v4;
                let mut v5: bool = &*v2 != "";
                let v7: Rc<str> = if v5 {
                    v2
                } else {
                    quoted_after_27(v0.clone(), v3)
                };
                (v1, v2) = (v4, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn cargo_value_25(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<UH1> = lines_of_16(v0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method26(v1, v2, v3)
}
fn kind_of_word_28(mut v0: Rc<str>) -> US0 {
    let v1: bool = &*v0 == "model";
    if v1 {
        US0::US0_Model
    } else {
        let v3: bool = &*v0 == "boundary";
        if v3 {
            US0::US0_Boundary
        } else {
            let mut v5: bool = &*v0 == "feature";
            if v5 {
                US0::US0_Feature
            } else {
                let v7: bool = &*v0 == "bin";
                if v7 {
                    US0::US0_Bin
                } else {
                    US0::US0_Plain
                }
            }
        }
    }
}
fn method32(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let mut v4: Rc<UH1> = method32(v3, v1);
            let mut v5: Rc<str> = entry_name_23(v2);
            Rc::new(UH1::UH1_Cons(v5, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method33(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let mut v4: Rc<UH1> = method33(v3, v1);
            let mut v5: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v7));
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = Rc::<str>::from(v8.replace(&*v9, &*v7));
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = Rc::<str>::from(v10.replace(&*v11, &*v7));
            Rc::new(UH1::UH1_Cons(v12, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method34(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let mut v4: Rc<UH1> = method34(v3, v1);
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
fn quoted_items_31(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH1> = split_list_17(v1, v0);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH1> = method33(v2, v3);
    let mut v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH1> = method34(v4, v5);
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method32(v6, v7)
}
fn method30(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH1> = v4;
                let mut v5: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
                let v7: Option<usize> = v5.find(&*v6);
                let mut v8: Option<usize> = method24(v7);
                let mut v9: Rc<dyn Fn((usize)) -> i32> = closure2();
                let mut v10: Option<i32> = v8.map(|x| v9(x));
                let v12: i32 = v10.unwrap_or(-1i32);
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
                let v14: Option<usize> = v5.rfind(&*v13);
                let v15: Option<usize> = method24(v14);
                let mut v16: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v17: Option<i32> = v15.map(|x| v16(x));
                let mut v19: i32 = v17.unwrap_or(-1i32);
                let mut v20: Rc<str> = string_concat(&[&*v0, " = ["]);
                let v21: bool = v5.starts_with(&*v20);
                let mut v23: bool = if v21 {
                    let mut v22: bool = v19 > v12;
                    v22
                } else {
                    false
                };
                let v33: Rc<UH1> = if v23 {
                    let mut v24: i32 = v12.wrapping_add(1i32);
                    let v25: i32 = v19.wrapping_sub(v12);
                    let v26: i32 = v25.wrapping_sub(1i32);
                    let v27: usize = ((v24) as usize);
                    let mut v28: i32 = v24.wrapping_add(v26);
                    let v29: usize = ((v28) as usize);
                    let v30: Rc<str> = Rc::<str>::from(&(v5)[v27..v29]);
                    let mut v31: Rc<UH1> = quoted_items_31(v30);
                    method20(v2, v31)
                } else {
                    v2
                };
                (v1, v2) = (v4, v33);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn toml_list_29(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: Rc<UH1> = lines_of_16(v0);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method30(v1, v2, v3)
}
fn method36(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let mut v4: Rc<UH1> = method36(v3, v1);
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
fn path_dep_38(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("path = \""); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method24(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let mut v8: bool = v7 < 0i32;
    let v18: Rc<str> = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    } else {
        let mut v10: i32 = v7.wrapping_add(8i32);
        let v11: i32 = (v0.clone().len() as i32);
        let v12: i32 = v11.wrapping_sub(v7);
        let v13: i32 = v12.wrapping_sub(8i32);
        let v14: usize = ((v10) as usize);
        let v15: i32 = v10.wrapping_add(v13);
        let mut v16: usize = ((v15) as usize);
        let mut v17: Rc<str> = Rc::<str>::from(&(v0)[v14..v16]);
        v17
    };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Option<usize> = v18.find(&*v19);
    let v21: Option<usize> = method24(v20);
    let mut v22: Option<i32> = v21.map(|x| v4(x));
    let mut v24: i32 = v22.unwrap_or(-1i32);
    let v26: bool = if v8 {
        true
    } else {
        let v25: bool = v24 < 0i32;
        v25
    };
    if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v27
    } else {
        let mut v28: usize = ((0i32) as usize);
        let v29: usize = ((v24) as usize);
        let v30: Rc<str> = Rc::<str>::from(&(v18)[v28..v29]);
        entry_name_23(v30)
    }
}
fn method37(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let mut v4: Rc<UH1> = method37(v3, v1);
            let mut v5: Rc<str> = path_dep_38(v2);
            Rc::new(UH1::UH1_Cons(v5, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn cargo_path_deps_35(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<UH1> = lines_of_16(v0);
    let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH1> = method37(v1, v2);
    let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method36(v3, v4)
}
fn unit_of_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool, mut v3: Rc<str>) -> (Rc<str>, US0, Rc<UH1>, Rc<UH1>, Rc<UH1>, Rc<str>, Rc<UH1>, bool) {
    let mut v8: Rc<UH1> = if v2 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules"); } LIT.with(|lit| lit.clone()) };
        toml_block_15(v1.clone(), v4)
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules"); } LIT.with(|lit| lit.clone()) };
        spiproj_section_21(v1.clone(), v6)
    };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = cargo_value_25(v3.clone(), v9);
    let v11: US0 = kind_of_word_28(v10);
    let mut v16: Rc<UH1> = if v2 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packages"); } LIT.with(|lit| lit.clone()) };
        toml_list_29(v1.clone(), v12)
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packages"); } LIT.with(|lit| lit.clone()) };
        spiproj_section_21(v1.clone(), v14)
    };
    let v21: Rc<UH1> = if v2 {
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("link"); } LIT.with(|lit| lit.clone()) };
        toml_list_29(v1, v17)
    } else {
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("link"); } LIT.with(|lit| lit.clone()) };
        spiproj_section_21(v1, v19)
    };
    let mut v22: Rc<UH1> = cargo_path_deps_35(v3.clone());
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("entry"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = cargo_value_25(v3.clone(), v23);
    let v25: bool = &*v3 != "";
    (v0, v11, v16, v21, v22, v24, v8, v25)
}
fn unit_at_13(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let v3: Rc<str> = string_concat(&[&*v0, "/", &*v1]);
    let mut v4: Rc<str> = string_concat(&[&*v3, "/package.spiproj"]);
    let mut v5: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v4).unwrap_or_default().as_str());
    let v6: Rc<str> = string_concat(&[&*v3, "/spiral.toml"]);
    let v7: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v6).unwrap_or_default().as_str());
    let mut v8: Rc<str> = string_concat(&[&*v3, "/Cargo.toml"]);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v8).unwrap_or_default().as_str());
    let mut v10: bool = &*v5 != "";
    if v10 {
        let v11: bool = false;
        let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19): (Rc<str>, US0, Rc<UH1>, Rc<UH1>, Rc<UH1>, Rc<str>, Rc<UH1>, bool) = unit_of_14(v1, v5, v11, v9);
        US1::US1_Some(v12, v13, v14, v15, v16, v17, v18, v19)
    } else {
        let v21: bool = &*v7 != "";
        if v21 {
            let mut v22: bool = true;
            let (mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (Rc<str>, US0, Rc<UH1>, Rc<UH1>, Rc<UH1>, Rc<str>, Rc<UH1>, bool) = unit_of_14(v1, v7, v22, v9);
            US1::US1_Some(v23, v24, v25, v26, v27, v28, v29, v30)
        } else {
            US1::US1_None
        }
    }
}
fn method12(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH1> = v4;
                let mut v5: US1 = unit_at_13(v0.clone(), v3);
                let mut v16: Rc<UH0> = match v5 {
                    US1::US1_None => {
                        v2
                    }
                    US1::US1_Some(v6, v7, v8, v9, v10, v11, v12, v13) => {
                        let v6: Rc<str> = v6;
                        let v7: US0 = v7;
                        let mut v8: Rc<UH1> = v8;
                        let mut v9: Rc<UH1> = v9;
                        let mut v10: Rc<UH1> = v10;
                        let v11: Rc<str> = v11;
                        let v12: Rc<UH1> = v12;
                        let v13: bool = v13;
                        Rc::new(UH0::UH0_Cons(v6, v7, v8, v9, v10, v11, v12, v13, v2))
                    }
                };
                (v1, v2) = (v4, v16);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn units_under_8(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<UH1> = names_in_10(v0.clone());
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH0> = method12(v0, v1, v2);
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method9(v3, v4)
}
fn rows_under_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<UH0> = units_under_8(v3);
    rows_of_1(v4)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>> {
        rows_under_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method41(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: bool) -> (Rc<UH1>, bool) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH1> = v4;
                let mut v5: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
                let v7: bool = v5.ends_with(&*v6);
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("members = ["); } LIT.with(|lit| lit.clone()) };
                let mut v9: bool = v5.starts_with(&*v8);
                let (mut v23, mut v24): (Rc<UH1>, bool) = if v9 {
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v11: Rc<str> = Rc::<str>::from(v5.replace(&*v8, &*v10));
                    let v12: Rc<UH1> = quoted_items_31(v11);
                    let v13: Rc<UH1> = method20(v1, v12);
                    let v14: bool = v7 == false;
                    (v13, v14)
                } else {
                    let mut v16: bool = if v2 {
                        let v15: bool = v5.starts_with(&*v6);
                        v15
                    } else {
                        false
                    };
                    if v16 {
                        (v1, false)
                    } else {
                        if v2 {
                            let mut v17: Rc<UH1> = quoted_items_31(v5);
                            let v18: Rc<UH1> = method20(v1, v17);
                            (v18, true)
                        } else {
                            (v1, false)
                        }
                    }
                };
                (v0, v1, v2) = (v4, v23, v24);
                continue;
            }
            UH1::UH1_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn members_of_40(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<UH1> = lines_of_16(v0);
    let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: bool = false;
    let (mut v4, mut v5): (Rc<UH1>, bool) = method41(v1, v2, v3);
    v4
}
fn members_under_39(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = string_concat(&[&*v3, "/Cargo.toml"]);
    let mut v5: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v4).unwrap_or_default().as_str());
    let v6: Rc<UH1> = members_of_40(v5);
    base_of_list_5(v6)
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<Rc<str>>>> {
        members_under_39(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_units_rows_under(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, bool)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_units_members_under(v0: &str) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure3()(Rc::<str>::from(v0))
}
pub type SpiralShape_2617b4e49837 = Mut0;
pub type SpiralShape_accbf6de6842 = Mut1;
