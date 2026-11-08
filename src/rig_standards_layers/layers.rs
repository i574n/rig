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
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
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
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, Rc<str>, Rc<str>),
    US2_None,
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method4(v3, v1);
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
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method7(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method7(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method6(v16, v17)
    }
}
fn method9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method10(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method8(v3, v1);
            let mut v5: i32 = (v2.clone().len() as i32);
            let v6: i32 = 0i32;
            let mut v7: i32 = method9(v2.clone(), v5, v6);
            let mut v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
            let v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method10(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            Rc::new(UH0::UH0_Cons(v12, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn listed_3(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_5(v1, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method8(v5, v6);
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method4(v7, v8)
}
fn method11(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let mut v5: Rc<UH0> = method11(v0.clone(), v4, v2);
            let v6: Rc<str> = string_concat(&["/", &*v3]);
            let mut v7: bool = v0.ends_with(&*v6);
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
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    Rc::new(move |mut v2: Rc<UH0>| -> US0 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US0::US0_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US0> = method13(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US0::US0_None
            }
        }
    })
}
fn method13(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    closure1(v0, v1)
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let mut v5: Rc<UH0> = method12(v0.clone(), v4, v2);
            let v6: i32 = 0i32;
            let mut v7: Rc<dyn Fn(Rc<UH0>) -> US0> = method13(v3.clone(), v6);
            let mut v8: US0 = v7(v0);
            let mut v9: bool = match v8 {
                US0::US0_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v10: bool = v9 != true;
            let v11: bool = v10 == false;
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
fn method15(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let mut v5: Rc<str> = (v0)[v4].clone();
            let v6: i32 = v1.wrapping_sub(1i32);
            let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn lines_of_14(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<UH0> {
    let mut v1: Rc<Vec<Rc<str>>> = Rc::new((v0).borrow().clone());
    let mut v2: usize = (v1).len();
    let mut v3: i32 = ((v2) as i32);
    let mut v4: i32 = v3.wrapping_sub(1i32);
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method15(v1, v4, v5)
}
fn method16(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<UH0> = v2;
                println!("{}", v1);
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
fn method19(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method20(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn stem_of_18(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = split_list_5(v1, v0);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = method19(v2, v3);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v6: Option<usize> = v4.rfind(&*v5);
    let mut v7: Option<usize> = method20(v6);
    let mut v8: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v9: Option<i32> = v7.map(|x| v8(x));
    let v11: i32 = v9.unwrap_or(-1i32);
    let v12: bool = v11 <= 0i32;
    if v12 {
        v4
    } else {
        let v13: usize = ((0i32) as usize);
        let v14: usize = ((v11) as usize);
        let v15: Rc<str> = Rc::<str>::from(&(v4)[v13..v14]);
        v15
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unassigned macros="); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.contains(&*v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method21() -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure3()
}
fn method22(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<UH0>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: bool = v0(v2);
                if v4 {
                    return true;
                } else {
                    let mut v5: Rc<dyn Fn(Rc<str>) -> bool> = method21();
                    (v0, v1) = (v5, v3);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn method17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let v6: Rc<str> = stem_of_18(v4.clone());
                let mut v8: Rc<str> = string_concat(&[&*v1, "/", &*v4]);
                let mut v9: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v8).unwrap_or_default().as_str());
                let v10: Rc<RefCell<Vec<Rc<str>>>> = rig_standards_layers_model::rig_layers_module_lines(&v0, &v6, &v9);
                let v11: Rc<UH0> = lines_of_14(v10);
                method16(v11.clone());
                let v12: Rc<dyn Fn(Rc<str>) -> bool> = method21();
                let v13: bool = method22(v12, v11);
                let v15: i32 = if v13 {
                    let v14: i32 = v3.wrapping_add(1i32);
                    v14
                } else {
                    v3
                };
                (v2, v3) = (v5, v15);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn now_c4(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    Rc::new(move |mut v1: Rc<str>| -> Rc<str> {
        let mut v3: Rc<str> = string_concat(&[&*v0, "/", &*v1]);
        let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v3).unwrap_or_default().as_str());
        v4
    })
}
fn method24(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> Rc<str>>, mut v2: Rc<UH0>, mut v3: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let v6: Rc<str> = stem_of_18(v4.clone());
                let mut v7: Rc<str> = v1(v4);
                let mut v8: i32 = rig_standards_layers_model::rig_layers_module_violations(&v0, &v6, &v7);
                let mut v9: i32 = v3.wrapping_add(v8);
                (v2, v3) = (v5, v9);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn violations_in_23(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<dyn Fn(Rc<str>) -> Rc<str>>) -> i32 {
    let mut v3: i32 = 0i32;
    method24(v0, v2, v1, v3)
}
fn method26(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: bool = &*v2 == "";
                let v6: Rc<UH0> = if v4 {
                    v1
                } else {
                    Rc::new(UH0::UH0_Cons(v2, v1))
                };
                (v0, v1) = (v3, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method27(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                drop(v0.borrow_mut().arg(&*v2));
                v1 = v3;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn git_run_25(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US1 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v2)));
    let mut v4: bool = &*v0 == "";
    let v12: Rc<UH0> = if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v5, v6))
    } else {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v9));
        Rc::new(UH0::UH0_Cons(v0, v10))
    };
    let v13: Rc<UH0> = method26(v1, v12);
    let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v15: Rc<UH0> = method6(v13, v14);
    method27(v3.clone(), v15);
    let mut v16: Rc<std::io::Result<std::process::Output>> = Rc::new(v3.borrow_mut().output());
    let v17: bool = v16.is_ok();
    let (mut v22, mut v23, mut v24): (bool, i32, Rc<str>) = if v17 {
        let v18: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v16).as_ref().unwrap().stdout).as_ref());
        let v19: Rc<str> = string_concat(&[&*v18, ""]);
        let v20: i32 = (&*v16).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v20, v19)
    } else {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v21)
    };
    let v26: bool = if v22 {
        let v25: bool = v23 == 0i32;
        v25
    } else {
        false
    };
    if v26 {
        US1::US1_Some(v24)
    } else {
        US1::US1_None
    }
}
fn before_c5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    Rc::new(move |mut v3: Rc<str>| -> Rc<str> {
        let mut v4: i32 = 0i32;
        let mut v5: Rc<dyn Fn(Rc<UH0>) -> US0> = method13(v3.clone(), v4);
        let v6: US0 = v5(v2.clone());
        let mut v7: bool = match v6 {
            US0::US0_None => {
                true
            }
            _ => {
                false
            }
        };
        let mut v8: bool = v7 != true;
        if v8 {
            let mut v9: Rc<str> = string_concat(&["HEAD:./", &*v3]);
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v11));
            let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v12));
            let v14: US1 = git_run_25(v0.clone(), v13);
            match v14 {
                US1::US1_None => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v16
                }
                US1::US1_Some(v15) => {
                    let v15: Rc<str> = v15;
                    v15
                }
            }
        } else {
            let v20: Rc<str> = string_concat(&[&*v1, "/", &*v3]);
            let v21: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v20).unwrap_or_default().as_str());
            v21
        }
    })
}
fn method28(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method30(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_29(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method30(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn layers_measured_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>) -> i32 {
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(v1.replace(&*v6, &*v7));
    let mut v9: Rc<str> = string_concat(&["/", &*v8]);
    let v10: Rc<UH0> = listed_3(v3);
    let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH0> = method11(v9, v10, v11);
    let v13: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
    let v14: Rc<UH0> = listed_3(v4);
    let v15: Rc<UH0> = listed_3(v5);
    let mut v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH0> = method12(v15, v12.clone(), v16);
    let v18: Rc<RefCell<Vec<Rc<str>>>> = rig_standards_layers_model::rig_layers_table_problems(&v2);
    let v19: Rc<UH0> = lines_of_14(v18);
    method16(v19);
    let v20: i32 = 0i32;
    let v21: i32 = method17(v2.clone(), v13.clone(), v12.clone(), v20);
    let v22: Rc<dyn Fn(Rc<str>) -> Rc<str>> = now_c4(v13.clone());
    let v23: i32 = violations_in_23(v2.clone(), v12.clone(), v22);
    let v24: Rc<dyn Fn(Rc<str>) -> Rc<str>> = before_c5(v0, v13, v14);
    let v25: i32 = violations_in_23(v2, v17, v24);
    let v26: i32 = 0i32;
    let v27: i32 = method28(v12, v26);
    let v28: Rc<str> = format_real_29(v27);
    let v31: Rc<str> = format_real_29(v21);
    let v34: Rc<str> = format_real_29(v23);
    let v37: Rc<str> = format_real_29(v25);
    let v38: Rc<str> = string_concat(&[" modules=", &*v28, " unassigned=", &*v31, " violations=", &*v34, " baseline=", &*v37]);
    let v39: bool = v23 > v25;
    if v39 {
        let v40: Rc<str> = string_concat(&["kit standards layers refused: violations grew past the HEAD baseline;", &*v38]);
        println!("{}", v40);
        1i32
    } else {
        let v41: Rc<str> = string_concat(&["kit standards layers ok", &*v38]);
        println!("{}", v41);
        0i32
    }
}
fn tree_of_31(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ls-files"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--cached"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--others"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--exclude-standard"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*.spir"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7.clone(), v8));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6.clone(), v9));
    let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v10));
    let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v11));
    let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v12));
    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v13));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1.clone(), v14));
    let mut v16: US1 = git_run_25(v0.clone(), v15);
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--name-only"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--relative"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HEAD"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7.clone(), v21));
    let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6.clone(), v22));
    let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v23));
    let v25: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20, v24));
    let v26: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19, v25));
    let v27: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v26));
    let v28: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17, v27));
    let v29: US1 = git_run_25(v0.clone(), v28);
    let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v31: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v30));
    let v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v31));
    let v33: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v32));
    let v34: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v33));
    let v35: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v34));
    let v36: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v35));
    let v37: US1 = git_run_25(v0, v36);
    match v16 {
        US1::US1_Some(v38) => {
            let v38: Rc<str> = v38;
            match v29 {
                US1::US1_Some(v39) => {
                    let v39: Rc<str> = v39;
                    match v37 {
                        US1::US1_Some(v40) => {
                            let v40: Rc<str> = v40;
                            US2::US2_Some(v38, v39, v40)
                        }
                        _ => {
                            US2::US2_None
                        }
                    }
                }
                _ => {
                    US2::US2_None
                }
            }
        }
        _ => {
            US2::US2_None
        }
    }
}
fn layers_checked_1(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v1).unwrap_or_default().as_str());
    let mut v3: i32 = rig_standards_layers_model::rig_layers_table_count(&v2);
    let mut v4: bool = v3 == 0i32;
    if v4 {
        let mut v5: Rc<str> = string_concat(&["kit error: standards --layers: no LayerRow rows in ", &*v1]);
        eprintln!("{}", v5);
        2i32
    } else {
        let v6: US2 = tree_of_31(v0.clone());
        match v6 {
            US2::US2_None => {
                let v12: Rc<str> = string_concat(&["kit error: standards --layers: git could not list ", &*v0, "; the baseline needs HEAD"]);
                eprintln!("{}", v12);
                2i32
            }
            US2::US2_Some(v7, v8, v9) => {
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                layers_measured_2(v0, v1, v2, v7, v8, v9)
            }
        }
    }
}
fn command_line_argument_count_33() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_34(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_32(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_33();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US1 = if v4 {
        let mut v5: Rc<str> = command_line_argument_34(v0);
        US1::US1_Some(v5)
    } else {
        US1::US1_None
    };
    match v8 {
        US1::US1_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US1::US1_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_32(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn table_flag_35(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let mut v4: Rc<str> = v4.clone();
                        let mut v5: Rc<UH0> = v5.clone();
                        let v6: bool = &*v2 == "--table";
                        if v6 {
                            (v0, v1) = (v5, v4);
                            continue;
                        } else {
                            v0 = v3;
                            continue;
                        }
                    }
                    _ => {
                        v0 = v3;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn layers_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_32(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let mut v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v9: Rc<str> = table_flag_35(v7, v8);
                            let v10: bool = &*v9 == "";
                            let v15: Rc<str> = if v10 {
                                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                                let v13: Rc<str> = Rc::<str>::from(v6.replace(&*v11, &*v12));
                                let v14: Rc<str> = string_concat(&[&*v13, "/layers.spi"]);
                                v14
                            } else {
                                v9
                            };
                            layers_checked_1(v6, v15)
                        }
                        _ => {
                            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: standards --layers <lib root> [--table <layers.spi>]"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v17);
                            2i32
                        }
                    }
                }
                _ => {
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: standards --layers <lib root> [--table <layers.spi>]"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v19);
                    2i32
                }
            }
        }
        _ => {
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: standards --layers <lib root> [--table <layers.spi>]"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v21);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        layers_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_standards_layers_run() -> i32 {
    closure0()()
}
use rig_standards_layers_model::SpiralShape_97df63ebabc3 as Mut1;
pub type SpiralShape_652accfc6a8d = Mut0;
