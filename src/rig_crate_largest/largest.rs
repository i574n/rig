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
pub struct Mut0 { pub l0: Rc<str> }
fn longer_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: i32) -> (Rc<str>, i32) {
    let v5: bool = &*v0 != "";
    let v8: bool = if v5 {
        let v6: i32 = v2.wrapping_sub(v1);
        let v7: bool = v6 > v4;
        v7
    } else {
        false
    };
    let (mut v10, mut v11): (Rc<str>, i32) = if v8 {
        let v9: i32 = v2.wrapping_sub(v1);
        (v0, v9)
    } else {
        (v3, v4)
    };
    (v10, v11)
}
fn method5(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn function_name_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v2: i32 = (v1.clone().len() as i32);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pub fn "); } LIT.with(|lit| lit.clone()) };
    let v4: bool = v1.starts_with(&*v3);
    let v19: Rc<str> = if v4 {
        let v5: i32 = v2.wrapping_sub(7i32);
        let v6: usize = ((7i32) as usize);
        let v7: i32 = 7i32.wrapping_add(v5);
        let v8: usize = ((v7) as usize);
        let v9: Rc<str> = Rc::<str>::from(&(v1)[v6..v8]);
        v9
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn "); } LIT.with(|lit| lit.clone()) };
        let v11: bool = v1.starts_with(&*v10);
        if v11 {
            let v12: i32 = v2.wrapping_sub(3i32);
            let v13: usize = ((3i32) as usize);
            let v14: i32 = 3i32.wrapping_add(v12);
            let v15: usize = ((v14) as usize);
            let v16: Rc<str> = Rc::<str>::from(&(v1)[v13..v15]);
            v16
        } else {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v17
        }
    };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
    let v21: Option<usize> = v19.find(&*v20);
    let v22: Option<usize> = method5(v21);
    let mut v23: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v24: Option<i32> = v22.map(|x| v23(x));
    let v26: i32 = v24.unwrap_or(-1i32);
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
    let v28: Option<usize> = v19.find(&*v27);
    let v29: Option<usize> = method5(v28);
    let mut v30: Option<i32> = v29.map(|x| v23(x));
    let v32: i32 = v30.unwrap_or(-1i32);
    let v33: bool = v32 >= 0i32;
    let v37: bool = if v33 {
        let v34: bool = v26 < 0i32;
        if v34 {
            true
        } else {
            let v35: bool = v32 < v26;
            v35
        }
    } else {
        false
    };
    let v38: i32 = if v37 {
        v32
    } else {
        v26
    };
    let v39: bool = v38 <= 0i32;
    if v39 {
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v40
    } else {
        let v41: usize = ((0i32) as usize);
        let v42: usize = ((v38) as usize);
        let v43: Rc<str> = Rc::<str>::from(&(v19)[v41..v42]);
        v43
    }
}
fn largest_from_2(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: i32) -> (Rc<str>, i32) {
    loop {
        let v6: i32 = (v0.clone().borrow().len() as i32);
        let v7: bool = v6 <= v1;
        if v7 {
            let (mut v8, mut v9): (Rc<str>, i32) = longer_3(v2, v3, v1, v4, v5);
            return (v8, v9);
        } else {
            let v10: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v11: Rc<str> = function_name_4(v10);
            let v12: bool = &*v11 == "";
            if v12 {
                let v13: i32 = v1.wrapping_add(1i32);
                v1 = v13;
                continue;
            } else {
                let v16: i32 = v1.wrapping_add(1i32);
                let (mut v17, mut v18): (Rc<str>, i32) = longer_3(v2, v3, v1, v4, v5);
                (v1, v2, v3, v4, v5) = (v16, v11, v1, v17, v18);
                continue;
            }
        }
    }
}
fn largest_function_of_1(mut v0: Rc<str>) -> (Rc<str>, i32) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v6: i32 = 0i32;
    let v7: i32 = 0i32;
    let v8: i32 = 0i32;
    let (mut v9, mut v10): (Rc<str>, i32) = largest_from_2(&v5, v6, v2.clone(), v7, v2.clone(), v8);
    (v9, v10)
}
fn method7(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_6(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method7(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn largest_text_0(mut v0: Rc<str>) -> Rc<str> {
    let (mut v1, mut v2): (Rc<str>, i32) = largest_function_of_1(v0);
    let v3: bool = &*v1 == "";
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let v7: Rc<str> = format_real_6(v2);
        let v8: Rc<str> = string_concat(&[" largest ", &*v1, " ", &*v7]);
        v8
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        largest_text_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_crate_largest(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
