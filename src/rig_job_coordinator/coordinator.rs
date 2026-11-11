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
pub struct Mut0 { pub l0: i32 }
#[derive(Clone)]
enum US0 {
    US0_ReceiptRead(Rc<str>, Rc<UH0>, Rc<str>),
    US0_ReceiptRefused(Rc<str>),
}
pub struct Mut1 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
pub struct Mut2 { pub l0: Rc<str> }
fn method2(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method1(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2]));
                let v5: i32 = (v1.clone().borrow().len() as i32);
                let mut v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method2(v7, &v9) {
                    let mut v11: i32 = v9.borrow().l0;
                    let mut v12: bool = v11 < v5;
                    let v16: Rc<str> = if v12 {
                        let mut v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13
                    } else {
                        let v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15
                    };
                    v8.borrow_mut()[v11 as usize] = v16;
                    let v17: i32 = v11.wrapping_add(1i32);
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
fn method5(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
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
fn method6(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method6(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method5(v16, v17)
    }
}
fn value_after_9(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let v3: Rc<str> = string_concat(&["cells-build-only ", &*v0, "="]);
    let mut v4: i32 = (v3.clone().len() as i32);
    let v5: bool = v1.starts_with(&*v3);
    if v5 {
        let mut v6: i32 = (v1.clone().len() as i32);
        let v7: i32 = v6.wrapping_sub(v4);
        let mut v8: usize = ((v4) as usize);
        let mut v9: i32 = v4.wrapping_add(v7);
        let v10: usize = ((v9) as usize);
        let mut v11: Rc<str> = Rc::<str>::from(&(v1)[v8..v10]);
        let mut v12: Rc<str> = Rc::<str>::from((v11).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        US1::US1_Some(v12)
    } else {
        US1::US1_None
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method10(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn receipt_folded_8(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, Rc<UH0>, Rc<str>) {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package"); } LIT.with(|lit| lit.clone()) };
    let v5: US1 = value_after_9(v4, v3.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ebin"); } LIT.with(|lit| lit.clone()) };
    let v7: US1 = value_after_9(v6, v3.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("entry"); } LIT.with(|lit| lit.clone()) };
    let mut v9: US1 = value_after_9(v8, v3);
    let (mut v22, mut v23, mut v24): (Rc<str>, Rc<UH0>, Rc<str>) = match v5 {
        US1::US1_Some(v10) => {
            let v10: Rc<str> = v10;
            (v10, v1, v2)
        }
        _ => {
            let (mut v19, mut v20, mut v21): (Rc<str>, Rc<UH0>, Rc<str>) = match v7 {
                US1::US1_Some(v11) => {
                    let mut v11: Rc<str> = v11;
                    let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v12));
                    let v14: Rc<UH0> = method10(v1, v13);
                    (v0, v14, v2)
                }
                _ => {
                    let (mut v16, mut v17, mut v18): (Rc<str>, Rc<UH0>, Rc<str>) = match v9 {
                        US1::US1_Some(v15) => {
                            let v15: Rc<str> = v15;
                            (v0, v1, v15)
                        }
                        _ => {
                            (v0, v1, v2)
                        }
                    };
                    (v16, v17, v18)
                }
            };
            (v19, v20, v21)
        }
    };
    (v22, v23, v24)
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<str>) -> (Rc<str>, Rc<UH0>, Rc<str>) {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let (mut v6, mut v7, mut v8): (Rc<str>, Rc<UH0>, Rc<str>) = receipt_folded_8(v1, v2, v3, v4);
                (v0, v1, v2, v3) = (v5, v6, v7, v8);
                continue;
            }
            _ => {
                let (mut v12, mut v13, mut v14): (Rc<str>, Rc<UH0>, Rc<str>) = match Rc::unwrap_or_clone(v0) {
                    UH0::UH0_Nil => {
                        (v1, v2, v3)
                    }
                    _ => unreachable!(),
                };
                return (v12, v13, v14);
            }
        }
    }
}
fn receipt_of_3(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH0> = split_list_4(v1, v0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v5, mut v6, mut v7): (Rc<str>, Rc<UH0>, Rc<str>) = method7(v2, v3.clone(), v4, v3);
    let mut v8: bool = match &*v6 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the receipt names no ebin; run the build again"); } LIT.with(|lit| lit.clone()) };
        US0::US0_ReceiptRefused(v9)
    } else {
        let mut v11: bool = &*v7 == "";
        if v11 {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the receipt names no entry module"); } LIT.with(|lit| lit.clone()) };
            US0::US0_ReceiptRefused(v12)
        } else {
            US0::US0_ReceiptRead(v5, v6, v7)
        }
    }
}
fn erl_args_11(mut v0: &Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<UH0> {
    let v3: Rc<str> = string_concat(&[&*v2, ":main()"]);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-eval"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v5));
    let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v6));
    let mut v8: Rc<UH0> = method10(v1, v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-noshell"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-pa"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v8));
    Rc::new(UH0::UH0_Cons(v9, v11))
}
fn erl_args_of_text_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: US0 = receipt_of_3(v0);
    match v1 {
        US0::US0_ReceiptRead(v2, v3, v4) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<str> = v4;
            let v5: Rc<UH0> = erl_args_11(&v2, v3, v4);
            let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
            let v7: Rc<RefCell<Vec<Rc<str>>>> = method1(v5, v6);
            v7
        }
        US0::US0_ReceiptRefused(v8) => {
            let mut v8: Rc<str> = v8;
            Rc::new(RefCell::new(vec![]))
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<Rc<str>>>> {
        erl_args_of_text_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn get_environment_variable_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn cells_root_of_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v0 != "";
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = Rc::<str>::from(v0.replace(&*v3, &*v4));
        v5
    } else {
        let mut v6: Rc<str> = string_concat(&[&*v1, "/apps/cells"]);
        v6
    }
}
fn build_args_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<UH0> {
    let v3: Rc<str> = string_concat(&[&*v0, "/spi/run_notebook.ps1"]);
    let mut v4: Rc<str> = string_concat(&[&*v1, "/src/rig_coordinator_actor/actor.spi"]);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-NoProfile"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-File"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--path"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--build-only"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the_coordinator_serves_the_spool_its_environment_names"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--no-html"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--output-path"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v13));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v14));
    let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v15));
    let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v16));
    let mut v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v17));
    let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v18));
    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v19));
    let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v20));
    let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v21));
    let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v22));
    Rc::new(UH0::UH0_Cons(v5, v23))
}
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v2 == "";
                let mut v6: Rc<UH0> = if v4 {
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
fn method18(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
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
fn method19(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
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
fn method21(mut v0: &Rc<RefCell<Mut2>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_20(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method21(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn built_13(mut v0: Rc<str>, mut v1: bool, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> i32 {
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_CELLS_ROOT"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = get_environment_variable_14(v7);
    let mut v9: Rc<str> = cells_root_of_15(v8, v3);
    let v10: Rc<str> = string_concat(&[&*v0, "/coordinator-build.ipynb"]);
    let mut v11: Rc<UH0> = build_args_16(v9, v4, v10);
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pwsh"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v12)));
    let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v15: Rc<UH0> = method17(v11, v14);
    let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH0> = method5(v15, v16);
    method18(v13.clone(), v17);
    let mut v18: Rc<std::io::Result<std::process::Output>> = Rc::new(v13.borrow_mut().output());
    let v19: bool = v18.is_ok();
    let (mut v25, mut v26, mut v27): (bool, i32, Rc<str>) = if v19 {
        let v20: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v18).as_ref().unwrap().stderr).as_ref());
        let v21: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v18).as_ref().unwrap().stdout).as_ref());
        let v22: Rc<str> = string_concat(&[&*v21, &*v20]);
        let v23: i32 = (&*v18).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v23, v22)
    } else {
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v24)
    };
    let v28: US0 = receipt_of_3(v27.clone());
    match v28 {
        US0::US0_ReceiptRead(v35, v36, v37) => {
            let v35: Rc<str> = v35;
            let v36: Rc<UH0> = v36;
            let mut v37: Rc<str> = v37;
            let v38: Rc<str> = string_concat(&[&*v0, "/coordinator.receipt"]);
            std::fs::write(&*v38, &*v27).unwrap();
            let v40: i32 = 0i32;
            let v41: i32 = method19(v36, v40);
            let v42: Rc<str> = format_real_20(v41);
            let v45: Rc<str> = string_concat(&["rig job coordinator build ok ebins=", &*v42, " package=", &*v35]);
            println!("{}", v45);
            0i32
        }
        US0::US0_ReceiptRefused(v29) => {
            let v29: Rc<str> = v29;
            let v30: Rc<str> = format_real_20(v26);
            let v34: Rc<str> = string_concat(&["rig error: job coordinator build: ", &*v29, " (exit ", &*v30, ")"]);
            eprintln!("{}", v34);
            1i32
        }
    }
}
fn file_exists_23(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method24(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn directory_exists_27(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method26(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method26(v3, v1);
            let v5: bool = directory_exists_27(v2.clone());
            let mut v6: bool = v5 == false;
            if v6 {
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
fn missing_ebins_25(mut v0: &Rc<str>, mut v1: Rc<UH0>, mut v2: &Rc<str>) -> Rc<UH0> {
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method26(v1, v3)
}
fn method29(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure2() -> Rc<dyn Fn((Rc<str>)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US1> = Rc::new(move |mut v0: (Rc<str>)| -> US1 {
        let mut v1: Rc<str> = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn current_executable_28() -> Rc<str> {
    let mut v0: Option<std::path::PathBuf> = std::env::current_exe().ok();
    let v2: bool = true; let _optionm_map_ = v0.map(|x| { //;
    let mut v4: std::path::PathBuf = x;
    let v5: Rc<str> = Rc::<str>::from(v4.to_string_lossy().as_ref());
    let v7: bool = true; v5 });
    let mut v9: Option<Rc<str>> = _optionm_map_;
    let v10: Option<Rc<str>> = method29(v9);
    let mut v11: Rc<dyn Fn((Rc<str>)) -> US1> = closure2();
    let mut v12: Option<US1> = v10.map(|x| v11(x));
    let v14: US1 = US1::US1_None;
    let v15: US1 = v14.clone();
    let v16: US1 = v12.unwrap_or(v15);
    match v16 {
        US1::US1_None => {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v18
        }
        US1::US1_Some(v17) => {
            let v17: Rc<str> = v17;
            v17
        }
    }
}
fn quoted_31(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("''"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v5: Rc<str> = string_concat(&["'", &*v3, "'"]);
    v5
}
fn method32(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method32(v3, v1);
            let v5: Rc<str> = quoted_31(v2);
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method33(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
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
fn start_script_30(mut v0: &Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = quoted_31(v3);
    let mut v8: Rc<str> = quoted_31(v4);
    let mut v11: Rc<UH0> = erl_args_11(v0, v1, v2);
    let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v13: Rc<UH0> = method32(v11, v12);
    let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v15: i32 = 0i32;
    let v16: i32 = method33(v14.clone(), v13, v15);
    let v17: Rc<Vec<Rc<str>>> = Rc::new(v14.borrow().clone());
    let mut v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v17).as_ref().clone()));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from((*v18.borrow()).join(&*v19));
    let v21: Rc<str> = string_concat(&["$env:KIT_COORDINATOR_SPOOL = ", &*v5, "; $env:KIT_COORDINATOR_KIT = ", &*v8, "; & erl ", &*v20]);
    v21
}
fn started_22(mut v0: Rc<str>, mut v1: bool, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> i32 {
    let v7: Rc<str> = string_concat(&[&*v0, "/coordinator.receipt"]);
    let mut v8: bool = file_exists_23(v7.clone());
    let v21: Rc<str> = if v8 {
        let v10: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v7);
        let mut v12: Vec<u8> = v10.unwrap();
        let mut v13: Vec<u8> = method24(v12);
        let v15: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v13);
        let v17: std::string::String = v15.unwrap();
        let v19: Rc<str> = Rc::<str>::from(String::as_str(&v17));
        v19
    } else {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v20
    };
    let v22: US0 = receipt_of_3(v21);
    match v22 {
        US0::US0_ReceiptRead(v26, v27, v28) => {
            let v26: Rc<str> = v26;
            let v27: Rc<UH0> = v27;
            let v28: Rc<str> = v28;
            let v29: Rc<UH0> = missing_ebins_25(&v26, v27.clone(), &v28);
            let v30: bool = match Rc::unwrap_or_clone(v29) {
                UH0::UH0_Nil => {
                    true
                }
                _ => {
                    false
                }
            };
            let v31: bool = v30 != true;
            if v31 {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: job coordinator start: a built ebin is gone; run kit job coordinator build"); } LIT.with(|lit| lit.clone()) };
                eprintln!("{}", v32);
                2i32
            } else {
                let v33: bool = &*v5 == "";
                if v33 {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: job coordinator start: --spool <dir> is required"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v34);
                    2i32
                } else {
                    let v35: Rc<str> = current_executable_28();
                    let v36: Rc<str> = start_script_30(&v26, v27, v28, v5, v35.clone());
                    let mut v37: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v35)));
                    let v38: bool = &*v36 == "";
                    let v70: Rc<UH0> = if v38 {
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-Command"); } LIT.with(|lit| lit.clone()) };
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-NoProfile"); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pwsh"); } LIT.with(|lit| lit.clone()) };
                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("604800000"); } LIT.with(|lit| lit.clone()) };
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-coordinator"); } LIT.with(|lit| lit.clone()) };
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                        let v46: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v47: Rc<UH0> = Rc::new(UH0::UH0_Cons(v45, v46));
                        let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v44, v47));
                        let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v43, v48));
                        let v50: Rc<UH0> = Rc::new(UH0::UH0_Cons(v42, v49));
                        let v51: Rc<UH0> = Rc::new(UH0::UH0_Cons(v41, v50));
                        let v52: Rc<UH0> = Rc::new(UH0::UH0_Cons(v40, v51));
                        Rc::new(UH0::UH0_Cons(v39, v52))
                    } else {
                        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-Command"); } LIT.with(|lit| lit.clone()) };
                        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-NoProfile"); } LIT.with(|lit| lit.clone()) };
                        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pwsh"); } LIT.with(|lit| lit.clone()) };
                        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("604800000"); } LIT.with(|lit| lit.clone()) };
                        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-coordinator"); } LIT.with(|lit| lit.clone()) };
                        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
                        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                        let v61: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v62: Rc<UH0> = Rc::new(UH0::UH0_Cons(v60, v61));
                        let v63: Rc<UH0> = Rc::new(UH0::UH0_Cons(v59, v62));
                        let v64: Rc<UH0> = Rc::new(UH0::UH0_Cons(v58, v63));
                        let v65: Rc<UH0> = Rc::new(UH0::UH0_Cons(v57, v64));
                        let v66: Rc<UH0> = Rc::new(UH0::UH0_Cons(v56, v65));
                        let v67: Rc<UH0> = Rc::new(UH0::UH0_Cons(v55, v66));
                        let v68: Rc<UH0> = Rc::new(UH0::UH0_Cons(v54, v67));
                        Rc::new(UH0::UH0_Cons(v36, v68))
                    };
                    let v71: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v72: Rc<UH0> = method5(v70, v71);
                    method18(v37.clone(), v72);
                    let mut v73: Rc<std::io::Result<std::process::Output>> = Rc::new(v37.borrow_mut().output());
                    let v74: bool = v73.is_ok();
                    let (mut v80, mut v81, mut v82): (bool, i32, Rc<str>) = if v74 {
                        let v75: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v73).as_ref().unwrap().stderr).as_ref());
                        let v76: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v73).as_ref().unwrap().stdout).as_ref());
                        let v77: Rc<str> = string_concat(&[&*v76, &*v75]);
                        let v78: i32 = (&*v73).as_ref().unwrap().status.code().unwrap_or(-2);
                        (true, v78, v77)
                    } else {
                        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        (false, -1i32, v79)
                    };
                    print!("{}", v82);
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    let v84: bool = if v80 {
                        let v83: bool = v81 == 0i32;
                        v83
                    } else {
                        false
                    };
                    if v84 {
                        0i32
                    } else {
                        1i32
                    }
                }
            }
        }
        US0::US0_ReceiptRefused(v23) => {
            let v23: Rc<str> = v23;
            let v25: Rc<str> = string_concat(&["rig error: job coordinator start: ", &*v23, "; run kit job coordinator build"]);
            eprintln!("{}", v25);
            2i32
        }
    }
}
fn coordinator_run_12(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<RefCell<Vec<(bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = rig_job_coordinator_args::rig_job_coordinator_args_read(2i32);
    let (mut v2, mut v3, mut v4, mut v5, mut v6, mut v7): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = v1.borrow()[0i32 as usize].clone();
    let mut v8: bool = &*v7 != "";
    if v8 {
        let mut v9: Rc<str> = string_concat(&["rig error: job coordinator: ", &*v7]);
        eprintln!("{}", v9);
        2i32
    } else {
        if v2 {
            built_13(v0, v2, v3, v4, v5, v6, v7)
        } else {
            started_22(v0, v2, v3, v4, v5, v6, v7)
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        coordinator_run_12(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_job_coordinator_erl_args(v0: &str) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_job_coordinator_run(v0: &str) -> i32 {
    closure1()(Rc::<str>::from(v0))
}
pub type SpiralShape_2617b4e49837 = Mut0;
pub type SpiralShape_652accfc6a8d = Mut1;
pub type SpiralShape_97df63ebabc3 = Mut2;
