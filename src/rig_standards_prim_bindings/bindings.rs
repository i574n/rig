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
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_RawMacros(i32),
    US0_MissingArms(i32, Rc<UH1>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(US0, Rc<UH0>),
}
pub struct Mut0 { pub l0: Rc<str> }
pub struct Mut1 { pub l0: i32 }
pub struct Mut2 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<UH1>, Rc<UH2>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
fn method3(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH1> = v4;
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn method5(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_4(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn binding_text_2(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_MissingArms(v5, v6) => {
            let mut v5: i32 = v5;
            let v6: Rc<UH1> = v6;
            let mut v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v8: i32 = 0i32;
            let mut v9: i32 = method3(v7.clone(), v6, v8);
            let mut v10: Rc<Vec<Rc<str>>> = Rc::new(v7.borrow().clone());
            let v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v10).as_ref().clone()));
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = Rc::<str>::from((*v11.borrow()).join(&*v12));
            let v14: Rc<str> = format_real_4(v5);
            let v17: Rc<str> = string_concat(&["partial backend binding: backend_switch #", &*v14, " has no arm for ", &*v13]);
            v17
        }
        US0::US0_RawMacros(v1) => {
            let v1: i32 = v1;
            let mut v2: Rc<str> = format_real_4(v1);
            let mut v4: Rc<str> = string_concat(&["partial backend binding: ", &*v2, " host macro(s) outside a backend_switch"]);
            v4
        }
    }
}
fn method6(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method1(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: US0 = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<str> = binding_text_2(v2);
                let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v4]));
                let v6: i32 = (v1.clone().borrow().len() as i32);
                let mut v7: i32 = (v5.clone().borrow().len() as i32);
                let v8: i32 = v6.wrapping_add(v7);
                let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v8 as usize]));
                let mut v10: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                while method6(v8, &v10) {
                    let v12: i32 = v10.borrow().l0;
                    let v13: bool = v12 < v6;
                    let v17: Rc<str> = if v13 {
                        let v14: Rc<str> = v1.borrow()[v12 as usize].clone();
                        v14
                    } else {
                        let v15: i32 = v12.wrapping_sub(v6);
                        let v16: Rc<str> = v5.borrow()[v15 as usize].clone();
                        v16
                    };
                    v9.borrow_mut()[v12 as usize] = v17;
                    let v18: i32 = v12.wrapping_add(1i32);
                    v10.borrow_mut().l0 = v18;
                    ()
                };
                (v0, v1) = (v3, v9);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method9(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut2>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH1> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method9(v16, v17)
    }
}
fn method14(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: Rc<UH1> = v2;
            let v3: Rc<UH2> = v3;
            let mut v4: Rc<UH2> = method14(v3, v1);
            Rc::new(UH2::UH2_Cons(v2, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn switch_closed_13(mut v0: i32, mut v1: i32, mut v2: Rc<UH1>, mut v3: Rc<UH2>) -> (i32, i32, Rc<UH1>, Rc<UH2>) {
    let mut v4: bool = v1 < 0i32;
    let (mut v9, mut v10, mut v11, mut v12): (i32, i32, Rc<UH1>, Rc<UH2>) = if v4 {
        (v0, v1, v2, v3)
    } else {
        let mut v5: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let v6: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v5));
        let mut v7: Rc<UH2> = method14(v3, v6);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        (v0, -1i32, v8, v7)
    };
    (v9, v10, v11, v12)
}
fn method15(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
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
                        let v8: bool = v4 == b'\n';
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
fn method18(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn occurrences_17(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<UH1> = split_list_8(v0, v1);
    let v3: i32 = 0i32;
    let mut v4: i32 = method18(v2, v3);
    let mut v5: i32 = v4.wrapping_sub(1i32);
    v5
}
fn own_macros_16(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = occurrences_17(v1, v0.clone());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$\""); } LIT.with(|lit| lit.clone()) };
    let mut v4: i32 = occurrences_17(v3, v0.clone());
    let mut v5: i32 = v2.wrapping_add(v4);
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!!!!"); } LIT.with(|lit| lit.clone()) };
    let mut v7: i32 = occurrences_17(v6, v0);
    let v8: i32 = v5.wrapping_add(v7);
    v8
}
fn indent_of_19(mut v0: Rc<str>) -> i32 {
    let v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method15(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0, v3 as i64, v4 as i64);
    let v6: i32 = (v5.len() as i32);
    let mut v7: i32 = v1.wrapping_sub(v6);
    v7
}
fn method21(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn first_char_22(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = &*v0 == "";
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: usize = ((0i32) as usize);
        let mut v4: usize = ((1i32) as usize);
        let mut v5: Rc<str> = Rc::<str>::from(&(v0)[v3..v4]);
        v5
    }
}
fn arm_word_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method21(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 > 0i32;
    let v13: Rc<str> = if v8 {
        let mut v9: usize = ((0i32) as usize);
        let mut v10: usize = ((v7) as usize);
        let v11: Rc<str> = Rc::<str>::from(&(v0)[v9..v10]);
        v11
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    };
    let v14: Rc<str> = first_char_22(v13.clone());
    let v15: bool = &*v14 != "";
    let v18: bool = if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); } LIT.with(|lit| lit.clone()) };
        let v17: bool = v16.contains(&*v14);
        v17
    } else {
        false
    };
    let v22: bool = if v18 {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v20: bool = v13.contains(&*v19);
        let v21: bool = v20 == false;
        v21
    } else {
        false
    };
    if v22 {
        v13
    } else {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v23
    }
}
fn method23(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let mut v4: Rc<UH1> = method23(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn scanned_line_12(mut v0: i32, mut v1: i32, mut v2: Rc<UH1>, mut v3: Rc<UH2>, mut v4: Rc<str>) -> (i32, i32, Rc<UH1>, Rc<UH2>) {
    let mut v5: i32 = (v4.clone().len() as i32);
    let v6: i32 = 0i32;
    let mut v7: i32 = method15(v4.clone(), v5, v6);
    let v8: i32 = v5.wrapping_sub(1i32);
    let mut v9: Rc<str> = string_slice(&v4.clone(), v7 as i64, v8 as i64);
    let mut v10: i32 = own_macros_16(v4.clone());
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backend_switch {"); } LIT.with(|lit| lit.clone()) };
    let v12: bool = v4.contains(&*v11);
    let (mut v46, mut v47, mut v48, mut v49): (i32, i32, Rc<UH1>, Rc<UH2>) = if v12 {
        let (mut v13, mut v14, mut v15, mut v16): (i32, i32, Rc<UH1>, Rc<UH2>) = switch_closed_13(v0, v1, v2, v3);
        let v17: i32 = indent_of_19(v4);
        (v13, v17, v15, v16)
    } else {
        let v18: bool = v1 >= 0i32;
        let v20: bool = if v18 {
            let v19: bool = &*v9 != "";
            v19
        } else {
            false
        };
        let v23: bool = if v20 {
            let v21: i32 = indent_of_19(v4);
            let v22: bool = v21 <= v1;
            v22
        } else {
            false
        };
        let (mut v42, mut v43, mut v44, mut v45): (i32, i32, Rc<UH1>, Rc<UH2>) = if v23 {
            let (mut v24, mut v25, mut v26, mut v27): (i32, i32, Rc<UH1>, Rc<UH2>) = switch_closed_13(v0, v1, v2, v3);
            (v24, v25, v26, v27)
        } else {
            let (mut v38, mut v39, mut v40, mut v41): (i32, i32, Rc<UH1>, Rc<UH2>) = if v18 {
                let v28: Rc<str> = arm_word_20(v9);
                let v29: bool = &*v28 == "";
                let (mut v33, mut v34, mut v35, mut v36): (i32, i32, Rc<UH1>, Rc<UH2>) = if v29 {
                    (v0, v1, v2, v3)
                } else {
                    let v30: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v31: Rc<UH1> = Rc::new(UH1::UH1_Cons(v28, v30));
                    let v32: Rc<UH1> = method23(v2, v31);
                    (v0, v1, v32, v3)
                };
                (v33, v34, v35, v36)
            } else {
                let v37: i32 = v0.wrapping_add(v10);
                (v37, v1, v2, v3)
            };
            (v38, v39, v40, v41)
        };
        (v42, v43, v44, v45)
    };
    (v46, v47, v48, v49)
}
fn method11(mut v0: Rc<UH1>, mut v1: i32, mut v2: i32, mut v3: Rc<UH1>, mut v4: Rc<UH2>) -> (i32, i32, Rc<UH1>, Rc<UH2>) {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v5, v6) => {
                let mut v5: Rc<str> = v5.clone();
                let v6: Rc<UH1> = v6.clone();
                let (mut v7, mut v8, mut v9, mut v10): (i32, i32, Rc<UH1>, Rc<UH2>) = scanned_line_12(v1, v2, v3, v4, v5);
                (v0, v1, v2, v3, v4) = (v6, v7, v8, v9, v10);
                continue;
            }
            _ => {
                let (mut v15, mut v16, mut v17, mut v18): (i32, i32, Rc<UH1>, Rc<UH2>) = match Rc::unwrap_or_clone(v0) {
                    UH1::UH1_Nil => {
                        (v1, v2, v3, v4)
                    }
                    _ => unreachable!(),
                };
                return (v15, v16, v17, v18);
            }
        }
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US1> {
    Rc::new(move |mut v2: Rc<UH1>| -> US1 {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH1> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US1::US1_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US1> = method26(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH1::UH1_Nil => {
                US1::US1_None
            }
        }
    })
}
fn method26(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US1> {
    closure2(v0, v1)
}
fn method25(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let mut v4: i32 = 0i32;
                let mut v5: Rc<dyn Fn(Rc<UH1>) -> US1> = method26(v2.clone(), v4);
                let v6: US1 = v5(v1.clone());
                let mut v7: bool = match v6 {
                    US1::US1_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v8: bool = v7 != true;
                let v12: Rc<UH1> = if v8 {
                    v1
                } else {
                    let mut v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let mut v10: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v9));
                    method23(v1, v10)
                };
                (v0, v1) = (v3, v12);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method24(mut v0: Rc<UH2>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<UH1> = v2;
                let v3: Rc<UH2> = v3;
                let mut v4: Rc<UH1> = method25(v2, v1);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method28(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH1> = v4;
            let mut v5: Rc<UH1> = method28(v0.clone(), v4, v2);
            let v6: i32 = 0i32;
            let mut v7: Rc<dyn Fn(Rc<UH1>) -> US1> = method26(v3.clone(), v6);
            let v8: US1 = v7(v0);
            let mut v9: bool = match v8 {
                US1::US1_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let mut v10: bool = v9 != true;
            let v11: bool = v10 == false;
            if v11 {
                Rc::new(UH1::UH1_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn method29(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: US0 = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method29(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method27(mut v0: Rc<UH1>, mut v1: Rc<UH2>, mut v2: Rc<UH0>, mut v3: i32) -> (Rc<UH0>, i32) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v4, v5) => {
                let mut v4: Rc<UH1> = v4;
                let mut v5: Rc<UH2> = v5;
                let v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let mut v7: Rc<UH1> = method28(v4, v0.clone(), v6);
                let v8: bool = match &*v7 {
                    UH1::UH1_Nil => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v13: Rc<UH0> = if v8 {
                    v2
                } else {
                    let mut v9: US0 = US0::US0_MissingArms(v3, v7);
                    let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v10));
                    method29(v2, v11)
                };
                let v14: i32 = v3.wrapping_add(1i32);
                (v1, v2, v3) = (v5, v13, v14);
                continue;
            }
            UH2::UH2_Nil => {
                return (v2, v3);
            }
        }
    }
}
fn prim_bindings_7(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH1> = split_list_8(v1, v0);
    let v3: i32 = 0i32;
    let mut v4: i32 = -1i32;
    let mut v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let (mut v7, mut v8, mut v9, mut v10): (i32, i32, Rc<UH1>, Rc<UH2>) = method11(v2, v3, v4, v5, v6);
    let (mut v11, mut v12, mut v13, mut v14): (i32, i32, Rc<UH1>, Rc<UH2>) = switch_closed_13(v7, v8, v9, v10);
    let v15: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH1> = method24(v14.clone(), v15);
    let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v18: i32 = 1i32;
    let (mut v19, mut v20): (Rc<UH0>, i32) = method27(v16, v14, v17, v18);
    let v21: bool = v11 > 0i32;
    if v21 {
        let v22: US0 = US0::US0_RawMacros(v11);
        Rc::new(UH0::UH0_Cons(v22, v19))
    } else {
        v19
    }
}
fn binding_texts_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v1: Rc<UH0> = prim_bindings_7(v0);
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v3: Rc<RefCell<Vec<Rc<str>>>> = method1(v1, v2);
    v3
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<Rc<str>>>> {
        binding_texts_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method31(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: bool) -> (Rc<str>, bool) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH1> = v4;
                let mut v5: i32 = (v3.clone().len() as i32);
                let v6: i32 = 0i32;
                let mut v7: i32 = method15(v3.clone(), v5, v6);
                let v8: i32 = v5.wrapping_sub(1i32);
                let mut v9: Rc<str> = string_slice(&v3.clone(), v7 as i64, v8 as i64);
                let v14: bool = if v2 {
                    let mut v10: bool = &*v9 == "";
                    if v10 {
                        true
                    } else {
                        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v12: bool = v3.starts_with(&*v11);
                        v12
                    }
                } else {
                    false
                };
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":#"); } LIT.with(|lit| lit.clone()) };
                let v16: bool = v3.starts_with(&*v15);
                let (mut v24, mut v25): (Rc<str>, bool) = if v16 {
                    (v1, true)
                } else {
                    let v19: bool = if v14 {
                        true
                    } else {
                        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//"); } LIT.with(|lit| lit.clone()) };
                        let v18: bool = v9.starts_with(&*v17);
                        v18
                    };
                    if v19 {
                        (v1, v2)
                    } else {
                        let v21: Rc<str> = string_concat(&[&*v1, &*v3, "\n"]);
                        (v21, false)
                    }
                };
                (v0, v1, v2) = (v4, v24, v25);
                continue;
            }
            UH1::UH1_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn code_of_30(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH1> = split_list_8(v1, v4);
    let v6: bool = false;
    let (mut v7, mut v8): (Rc<str>, bool) = method31(v5, v3, v6);
    v7
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        code_of_30(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn last_char_35(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = &*v0 == "";
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: i32 = (v0.clone().len() as i32);
        let mut v4: i32 = v3.wrapping_sub(1i32);
        let mut v5: usize = ((v4) as usize);
        let v6: i32 = v4.wrapping_add(1i32);
        let mut v7: usize = ((v6) as usize);
        let v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
        v8
    }
}
fn word_char_36(mut v0: Rc<str>) -> bool {
    let v1: bool = &*v0 != "";
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_'"); } LIT.with(|lit| lit.clone()) };
        let v3: bool = v2.contains(&*v0);
        v3
    } else {
        false
    }
}
fn member_start_37(mut v0: Rc<str>) -> bool {
    let v1: bool = &*v0 != "";
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyz_"); } LIT.with(|lit| lit.clone()) };
        let v3: bool = v2.contains(&*v0);
        v3
    } else {
        false
    }
}
fn method34(mut v0: Rc<UH1>, mut v1: bool, mut v2: Rc<str>) -> (bool, Rc<str>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH1> = v4;
                let v11: bool = if v1 {
                    true
                } else {
                    let mut v5: Rc<str> = last_char_35(v2);
                    let v6: bool = word_char_36(v5);
                    let mut v7: bool = v6 == false;
                    if v7 {
                        let v8: Rc<str> = first_char_22(v3.clone());
                        member_start_37(v8)
                    } else {
                        false
                    }
                };
                (v0, v1, v2) = (v4, v11, v3);
                continue;
            }
            UH1::UH1_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn referenced_by_33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let v3: Rc<str> = string_concat(&[&*v1, &*v2]);
    let mut v4: bool = v0.contains(&*v3);
    let mut v5: bool = v4 == false;
    if v5 {
        false
    } else {
        let v6: Rc<UH1> = split_list_8(v3, v0);
        match Rc::unwrap_or_clone(v6) {
            UH1::UH1_Cons(v7, v8) => {
                let mut v7: Rc<str> = v7;
                let v8: Rc<UH1> = v8;
                let mut v9: bool = false;
                let (mut v10, mut v11): (bool, Rc<str>) = method34(v8, v9, v7);
                v10
            }
            UH1::UH1_Nil => {
                false
            }
        }
    }
}
fn module_opened_32(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v3: Rc<str> = string_concat(&["\n:< ", &*v0, "\n"]);
    let mut v4: Rc<str> = string_concat(&["\n", &*v1]);
    let mut v5: bool = v4.contains(&*v3);
    let v8: bool = if v5 {
        true
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        referenced_by_33(v1.clone(), v0.clone(), v6)
    };
    if v8 {
        true
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("::"); } LIT.with(|lit| lit.clone()) };
        referenced_by_33(v1, v0, v9)
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> bool {
        module_opened_32(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        own_macros_16(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_standards_prim_binding_texts(v0: &str) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_standards_code_of(v0: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0))
}
pub fn rig_standards_module_opened(v0: &str, v1: &str) -> bool {
    closure4()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_standards_own_macros(v0: &str) -> i32 {
    closure5()(Rc::<str>::from(v0))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
pub type SpiralShape_2617b4e49837 = Mut1;
pub type SpiralShape_accbf6de6842 = Mut2;
