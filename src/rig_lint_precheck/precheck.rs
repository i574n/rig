#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
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
#[derive(Clone)]
enum US0 {
    US0_WidthOver(Rc<str>, i32, i32),
    US0_OpenString(Rc<str>, i32),
    US0_ReservedOutput(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US0, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(u8, Rc<UH2>),
}
pub struct Mut1 { pub l0: Rc<str> }
fn method1(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method5(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH1> = method5(v0.clone(), v4, v2);
            let v6: US0 = US0::US0_ReservedOutput(v0, v3);
            Rc::new(UH1::UH1_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn manifest_findings_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("output = \"lib.rs\""); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v1.contains(&*v2);
    let v8: Rc<UH0> = if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lib.rs"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v4, v5))
    } else {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("output = \"main.rs\""); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v1.contains(&*v9);
    let v13: Rc<UH0> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("main.rs"); } LIT.with(|lit| lit.clone()) };
        Rc::new(UH0::UH0_Cons(v11, v8))
    } else {
        v8
    };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("output = \"build.rs\""); } LIT.with(|lit| lit.clone()) };
    let v15: bool = v1.contains(&*v14);
    let v18: Rc<UH0> = if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("build.rs"); } LIT.with(|lit| lit.clone()) };
        Rc::new(UH0::UH0_Cons(v16, v13))
    } else {
        v13
    };
    let v19: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method5(v0, v18, v19)
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method9(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method9(v7, &v9) {
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
        method8(v16, v17)
    }
}
fn raw_marks_11(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\"\""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: i32 = (v4.len() as i32);
    let v6: i32 = v1.wrapping_sub(v5);
    let v7: i32 = v6.wrapping_div(3i32);
    v7
}
fn method13(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
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
fn method14(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
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
fn host_line_12(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.contains(&*v1);
    let v5: bool = if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$\""); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.contains(&*v3);
        v4
    };
    if v5 {
        true
    } else {
        let v6: i32 = (v0.clone().len() as i32);
        let v7: i32 = 0i32;
        let v8: i32 = method13(v0.clone(), v6, v7);
        let mut v9: i32 = v6.wrapping_sub(1i32);
        let v10: Rc<str> = string_slice(&v0, v8 as i64, v9 as i64);
        let v11: i32 = (v10.clone().len() as i32);
        let v12: i32 = method14(v10.clone(), v11);
        let v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v15: bool = v13.ends_with(&*v14);
        v15
    }
}
fn method16(mut v0: i64, mut v1: Rc<str>, mut v2: i64) -> Rc<UH2> {
    let v3: bool = v2 < v0;
    if v3 {
        let v4: u8 = v1.clone().as_bytes()[v2 as usize];
        let v5: i64 = v2.wrapping_add(1i64);
        let v6: Rc<UH2> = method16(v0, v1, v5);
        Rc::new(UH2::UH2_Cons(v4, v6))
    } else {
        { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
    }
}
fn quotes_from_17(mut v0: Rc<UH2>, mut v1: bool, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: u8 = v3;
                let v4: Rc<UH2> = v4;
                if v1 {
                    let v5: bool = false;
                    (v0, v1) = (v4, v5);
                    continue;
                } else {
                    let v7: bool = v3 == b'\\';
                    if v7 {
                        let v8: bool = true;
                        (v0, v1) = (v4, v8);
                        continue;
                    } else {
                        let v10: bool = v3 == b'"';
                        if v10 {
                            let v11: bool = false;
                            let v12: i32 = v2.wrapping_add(1i32);
                            (v0, v1, v2) = (v4, v11, v12);
                            continue;
                        } else {
                            let v14: bool = false;
                            (v0, v1) = (v4, v14);
                            continue;
                        }
                    }
                }
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn open_string_15(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'\"'"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: i64 = (v3.clone().len() as i64);
    let v5: i64 = 0i64;
    let v6: Rc<UH2> = method16(v4, v3, v5);
    let v7: bool = false;
    let v8: i32 = 0i32;
    let mut v9: i32 = quotes_from_17(v6, v7, v8);
    let v10: i32 = v9.wrapping_rem(2i32);
    let v11: bool = v10 == 1i32;
    v11
}
fn method18(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: US0 = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method18(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method10(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>, mut v3: i32, mut v4: bool, mut v5: bool) -> (Rc<UH1>, i32, bool, bool) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v6, v7) => {
                let v6: Rc<str> = v6;
                let v7: Rc<UH0> = v7;
                let v8: i32 = (v6.clone().len() as i32);
                let mut v9: bool = v8 > 120i32;
                let v14: Rc<UH1> = if v9 {
                    let v10: US0 = US0::US0_WidthOver(v0.clone(), v3, v8);
                    let v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH1::UH1_Cons(v10, v11))
                } else {
                    { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
                };
                let v15: i32 = raw_marks_11(v6.clone());
                let v17: bool = if v5 {
                    true
                } else {
                    let v16: bool = v15 > 0i32;
                    v16
                };
                let v18: bool = v17 || v4;
                let v20: bool = if v18 {
                    true
                } else {
                    host_line_12(v6.clone())
                };
                let v30: bool = if v20 {
                    true
                } else {
                    let v21: i32 = 0i32;
                    let v22: i32 = method13(v6.clone(), v8, v21);
                    let v23: i32 = v8.wrapping_sub(1i32);
                    let v24: Rc<str> = string_slice(&v6.clone(), v22 as i64, v23 as i64);
                    let v25: i32 = (v24.clone().len() as i32);
                    let v26: i32 = method14(v24.clone(), v25);
                    let v27: Rc<str> = string_slice(&v24, 0i32 as i64, v26 as i64);
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///"); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v27.starts_with(&*v28);
                    v29
                };
                let v31: bool = v30 == false;
                let v33: bool = if v31 {
                    open_string_15(v6.clone())
                } else {
                    false
                };
                let v38: Rc<UH1> = if v33 {
                    let v34: US0 = US0::US0_OpenString(v0.clone(), v3);
                    let v35: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH1::UH1_Cons(v34, v35))
                } else {
                    { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
                };
                let v39: i32 = v15.wrapping_rem(2i32);
                let v40: bool = v39 == 1i32;
                let v42: bool = if v40 {
                    let v41: bool = v5 == false;
                    v41
                } else {
                    v5
                };
                let v43: Rc<UH1> = method18(v14, v38);
                let v44: Rc<UH1> = method18(v2, v43);
                let v45: i32 = v3.wrapping_add(1i32);
                let v46: i32 = 0i32;
                let v47: i32 = method13(v6.clone(), v8, v46);
                let v48: i32 = v8.wrapping_sub(1i32);
                let v49: Rc<str> = string_slice(&v6, v47 as i64, v48 as i64);
                let v50: i32 = (v49.clone().len() as i32);
                let v51: i32 = method14(v49.clone(), v50);
                let v52: Rc<str> = string_slice(&v49, 0i32 as i64, v51 as i64);
                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                let v54: bool = v52.ends_with(&*v53);
                (v1, v2, v3, v4, v5) = (v7, v44, v45, v54, v42);
                continue;
            }
            UH0::UH0_Nil => {
                return (v2, v3, v4, v5);
            }
        }
    }
}
fn file_findings_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v1.replace(&*v3, &*v4));
    let v6: Rc<UH0> = split_list_7(v2, v5);
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v8: i32 = 1i32;
    let mut v9: bool = false;
    let v10: bool = false;
    let (mut v11, mut v12, mut v13, mut v14): (Rc<UH1>, i32, bool, bool) = method10(v0, v6, v7, v8, v9, v10);
    v11
}
fn path_findings_3(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default().as_str());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Cargo.toml"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v0.ends_with(&*v2);
    if v3 {
        manifest_findings_4(v0, v1)
    } else {
        file_findings_6(v0, v1)
    }
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH1> = path_findings_3(v2);
                let v5: Rc<UH1> = method18(v1, v4);
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method22(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_21(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method22(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn finding_line_20(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_OpenString(v16, v17) => {
            let v16: Rc<str> = v16;
            let v17: i32 = v17;
            let v19: Rc<str> = format_real_21(v17);
            let v21: Rc<str> = string_concat(&[&*v16, ":", &*v19, ": a string literal is left open at the line end (a broken \\n or \\\"?)"]);
            v21
        }
        US0::US0_ReservedOutput(v1, v2) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            let v5: Rc<str> = string_concat(&[&*v1, ": output ", &*v2, " is a name Cargo reserves in a crate root (build.rs = build script); rename the module"]);
            v5
        }
        US0::US0_WidthOver(v6, v7, v8) => {
            let v6: Rc<str> = v6;
            let v7: i32 = v7;
            let v8: i32 = v8;
            let v10: Rc<str> = format_real_21(v7);
            let v13: Rc<str> = format_real_21(v8);
            let v15: Rc<str> = string_concat(&[&*v6, ":", &*v10, ": width ", &*v13, " > 120"]);
            v15
        }
    }
}
fn method19(mut v0: Rc<UH1>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v1, v2) => {
                let mut v1: US0 = v1;
                let mut v2: Rc<UH1> = v2;
                let v3: Rc<str> = finding_line_20(v1);
                println!("{}", v3);
                v0 = v2;
                continue;
            }
            UH1::UH1_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method23(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: US0 = v2;
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
fn method24(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn precheck_run_0(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> i32 {
    let mut v1: Rc<Vec<Rc<str>>> = Rc::new((v0).borrow().clone());
    let mut v2: usize = (v1).len();
    let v3: i32 = ((v2) as i32);
    let v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = method1(v1, v4, v5);
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH1> = method2(v6.clone(), v7);
    method19(v8.clone());
    let mut v9: i32 = 0i32;
    let v10: i32 = method23(v8, v9);
    let v11: i32 = 0i32;
    let v12: i32 = method24(v6, v11);
    let v13: bool = v10 == 0i32;
    if v13 {
        let v14: Rc<str> = format_real_21(v12);
        let v16: Rc<str> = string_concat(&["rig lint precheck ok files=", &*v14, " findings=0"]);
        println!("{}", v16);
        0i32
    } else {
        let v17: Rc<str> = format_real_21(v12);
        let v20: Rc<str> = format_real_21(v10);
        let v22: Rc<str> = string_concat(&["rig lint precheck refused: files=", &*v17, " findings=", &*v20]);
        println!("{}", v22);
        1i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<RefCell<Vec<Rc<str>>>>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<RefCell<Vec<Rc<str>>>>) -> i32> = Rc::new(move |mut v0: Rc<RefCell<Vec<Rc<str>>>>| -> i32 {
        precheck_run_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_lint_precheck_run(v0: Rc<RefCell<Vec<Rc<str>>>>) -> i32 {
    closure0()(v0)
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
