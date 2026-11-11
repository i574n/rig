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
#[derive(Clone)]
enum US0 {
    US0_Ungated,
    US0_Gated(i32, i32, i32),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32, i32),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Admitted,
    US3_TooLoaded(i32, i32),
    US3_TooLittleMemory(i32, i32),
    US3_SampleUnread(Rc<str>),
}
pub struct Mut1 { pub l0: Rc<str> }
fn gate_path_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v7: Rc<str> = string_concat(&[&*v4, "/machines/", &*v1, ".gate"]);
    v7
}
fn file_exists_2(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method3(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method6(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method9(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
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
fn method11(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure2() -> Rc<dyn Fn((i32)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US1> = Rc::new(move |mut v0: (i32)| -> US1 {
        let mut v1: i32 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn number_of_10(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: Option<i32> = (v4).trim().parse::<i32>().ok();
    let mut v6: Option<i32> = method11(v5);
    let mut v7: Rc<dyn Fn((i32)) -> US1> = closure2();
    let mut v8: Option<US1> = v6.map(|x| v7(x));
    let v10: US1 = US1::US1_None;
    let v11: US1 = v10.clone();
    let v12: US1 = v8.unwrap_or(v11);
    v12
}
fn method5(mut v0: Rc<UH0>, mut v1: US0) -> US0 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = MachineGate ("); } LIT.with(|lit| lit.clone()) };
                let mut v6: Option<usize> = v4.find(&*v5);
                let mut v7: Option<usize> = method6(v6);
                let mut v8: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v9: Option<i32> = v7.map(|x| v8(x));
                let v11: i32 = v9.unwrap_or(-1i32);
                let v12: bool = v11 < 0i32;
                let v16: bool = if v12 {
                    true
                } else {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                    let v14: bool = v4.ends_with(&*v13);
                    let v15: bool = v14 == false;
                    v15
                };
                let v43: US0 = if v16 {
                    v1
                } else {
                    let v17: i32 = v11.wrapping_add(16i32);
                    let v18: i32 = (v4.clone().len() as i32);
                    let v19: i32 = v18.wrapping_sub(2i32);
                    let v20: Rc<str> = string_slice(&v4, v17 as i64, v19 as i64);
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<UH0> = split_list_7(v21, v20);
                    match Rc::unwrap_or_clone(v22) {
                        UH0::UH0_Cons(v23, v24) => {
                            let v23: Rc<str> = v23;
                            let v24: Rc<UH0> = v24;
                            match Rc::unwrap_or_clone(v24) {
                                UH0::UH0_Cons(v25, v26) => {
                                    let v25: Rc<str> = v25;
                                    let v26: Rc<UH0> = v26;
                                    match Rc::unwrap_or_clone(v26) {
                                        UH0::UH0_Cons(v27, v28) => {
                                            let v27: Rc<str> = v27;
                                            let mut v28: Rc<UH0> = v28;
                                            match Rc::unwrap_or_clone(v28) {
                                                UH0::UH0_Nil => {
                                                    let v29: US1 = number_of_10(v23);
                                                    let v30: US1 = number_of_10(v25);
                                                    let v31: US1 = number_of_10(v27);
                                                    match v29 {
                                                        US1::US1_Some(v32) => {
                                                            let v32: i32 = v32;
                                                            match v30 {
                                                                US1::US1_Some(v33) => {
                                                                    let v33: i32 = v33;
                                                                    match v31 {
                                                                        US1::US1_Some(v34) => {
                                                                            let v34: i32 = v34;
                                                                            US0::US0_Gated(v32, v33, v34)
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
                (v0, v1) = (v3, v43);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn gate_of_text_4(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_7(v1, v4);
    let mut v6: US0 = US0::US0_Ungated;
    method5(v5, v6)
}
fn load_x100_of_16(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v3: Rc<UH0> = split_list_7(v1, v2);
    match Rc::unwrap_or_clone(v3) {
        UH0::UH0_Cons(v4, v5) => {
            let v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            match Rc::unwrap_or_clone(v5) {
                UH0::UH0_Cons(v6, v7) => {
                    let mut v6: Rc<str> = v6;
                    let mut v7: Rc<UH0> = v7;
                    match Rc::unwrap_or_clone(v7) {
                        UH0::UH0_Nil => {
                            let mut v8: i32 = (v6.clone().len() as i32);
                            let mut v9: bool = v8 >= 2i32;
                            let v10: i32 = if v9 {
                                2i32
                            } else {
                                v8
                            };
                            let v11: usize = ((0i32) as usize);
                            let v12: usize = ((v10) as usize);
                            let v13: Rc<str> = Rc::<str>::from(&(v6)[v11..v12]);
                            let v14: i32 = (v13.clone().len() as i32);
                            let v15: bool = v14 == 1i32;
                            let v16: i32 = if v15 {
                                10i32
                            } else {
                                1i32
                            };
                            let v17: US1 = number_of_10(v4);
                            let v18: US1 = number_of_10(v13);
                            match v17 {
                                US1::US1_Some(v19) => {
                                    let v19: i32 = v19;
                                    match v18 {
                                        US1::US1_Some(v20) => {
                                            let v20: i32 = v20;
                                            let v21: i32 = v19.wrapping_mul(100i32);
                                            let v22: i32 = v20.wrapping_mul(v16);
                                            let v23: i32 = v21.wrapping_add(v22);
                                            US1::US1_Some(v23)
                                        }
                                        _ => {
                                            US1::US1_None
                                        }
                                    }
                                }
                                _ => {
                                    US1::US1_None
                                }
                            }
                        }
                        _ => {
                            US1::US1_None
                        }
                    }
                }
                UH0::UH0_Nil => {
                    let v31: US1 = number_of_10(v4);
                    match v31 {
                        US1::US1_None => {
                            US1::US1_None
                        }
                        US1::US1_Some(v32) => {
                            let v32: i32 = v32;
                            let v33: i32 = v32.wrapping_mul(100i32);
                            US1::US1_Some(v33)
                        }
                    }
                }
            }
        }
        _ => {
            US1::US1_None
        }
    }
}
fn load_line_15(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v3: Rc<UH0> = split_list_7(v1, v2);
    match Rc::unwrap_or_clone(v3) {
        UH0::UH0_Cons(v4, v5) => {
            let v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            match Rc::unwrap_or_clone(v5) {
                UH0::UH0_Cons(v6, v7) => {
                    let mut v6: Rc<str> = v6;
                    let mut v7: Rc<UH0> = v7;
                    match Rc::unwrap_or_clone(v7) {
                        UH0::UH0_Cons(v8, v9) => {
                            let mut v8: Rc<str> = v8;
                            let mut v9: Rc<UH0> = v9;
                            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                            let v11: bool = v0.contains(&*v10);
                            if v11 {
                                load_x100_of_16(v4)
                            } else {
                                US1::US1_None
                            }
                        }
                        _ => {
                            US1::US1_None
                        }
                    }
                }
                _ => {
                    US1::US1_None
                }
            }
        }
        _ => {
            US1::US1_None
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: US1) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v7: US1 = match &v1 {
                    US1::US1_None => {
                        load_line_15(v2)
                    }
                    US1::US1_Some(v4) => {
                        let v4: i32 = *v4;
                        v1
                    }
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
fn memory_line_18(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MemAvailable:"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    let v3: bool = v2 == false;
    if v3 {
        US1::US1_None
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v5));
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kB"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = Rc::<str>::from(v6.replace(&*v7, &*v5));
        let mut v9: US1 = number_of_10(v8);
        match v9 {
            US1::US1_None => {
                US1::US1_None
            }
            US1::US1_Some(v10) => {
                let v10: i32 = v10;
                let v11: i32 = v10.wrapping_div(1024i32);
                US1::US1_Some(v11)
            }
        }
    }
}
fn method17(mut v0: Rc<UH0>, mut v1: US1) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v7: US1 = match &v1 {
                    US1::US1_None => {
                        memory_line_18(v2)
                    }
                    US1::US1_Some(v4) => {
                        let v4: i32 = *v4;
                        v1
                    }
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
fn sample_of_13(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_7(v1, v4);
    let mut v6: US1 = US1::US1_None;
    let mut v7: US1 = method14(v5.clone(), v6);
    let mut v8: US1 = US1::US1_None;
    let mut v9: US1 = method17(v5, v8);
    match v7 {
        US1::US1_Some(v10) => {
            let v10: i32 = v10;
            match v9 {
                US1::US1_Some(v11) => {
                    let v11: i32 = v11;
                    US2::US2_Some(v10, v11)
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
fn admitted_by_19(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> US3 {
    let v5: bool = v3 >= v0;
    if v5 {
        US3::US3_TooLoaded(v3, v0)
    } else {
        let mut v7: i32 = v2.wrapping_add(v1);
        let mut v8: bool = v4 < v7;
        if v8 {
            US3::US3_TooLittleMemory(v4, v7)
        } else {
            US3::US3_Admitted
        }
    }
}
fn method22(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_21(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method22(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn scoped_line_20(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = format_real_21(v2);
    let mut v7: Rc<str> = string_concat(&["systemd-run --scope --quiet -p MemoryMax=", &*v4, "M -p MemorySwapMax=0 ", &*v3]);
    v7
}
fn hundredths_text_24(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_rem(100i32);
    let v2: i32 = v0.wrapping_div(100i32);
    let v3: Rc<str> = format_real_21(v2);
    let v4: Rc<str> = string_concat(&[&*v3, "."]);
    let v5: bool = v1 < 10i32;
    let mut v8: Rc<str> = if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v7
    };
    let v10: Rc<str> = format_real_21(v1);
    let v11: Rc<str> = string_concat(&[&*v4, &*v8, &*v10]);
    v11
}
fn reason_of_23(mut v0: US3) -> Rc<str> {
    match v0 {
        US3::US3_Admitted => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US3::US3_SampleUnread(v17) => {
            let v17: Rc<str> = v17;
            let v18: Rc<str> = Rc::<str>::from((v17).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let v20: Rc<str> = string_concat(&["admission: no load or MemAvailable in the sample (", &*v18, ")"]);
            v20
        }
        US3::US3_TooLittleMemory(v9, v10) => {
            let mut v9: i32 = v9;
            let v10: i32 = v10;
            let v11: Rc<str> = format_real_21(v9);
            let v14: Rc<str> = format_real_21(v10);
            let v16: Rc<str> = string_concat(&["admission: MemAvailable ", &*v11, " MB < cap + floor ", &*v14, " MB"]);
            v16
        }
        US3::US3_TooLoaded(v2, v3) => {
            let v2: i32 = v2;
            let v3: i32 = v3;
            let v4: Rc<str> = hundredths_text_24(v2);
            let mut v7: Rc<str> = hundredths_text_24(v3);
            let mut v8: Rc<str> = string_concat(&["admission: load ", &*v4, " >= ", &*v7]);
            v8
        }
    }
}
fn gated_of_12(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>) -> (bool, Rc<str>, Rc<str>) {
    let (mut v23, mut v24, mut v25): (bool, Rc<str>, Rc<str>) = match &v0 {
        US0::US0_Ungated => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (true, v2, v3)
        }
        _ => {
            let (mut v20, mut v21, mut v22): (bool, Rc<str>, Rc<str>) = match v0 {
                US0::US0_Gated(v4, v5, v6) => {
                    let v4: i32 = v4;
                    let v5: i32 = v5;
                    let mut v6: i32 = v6;
                    let mut v7: US2 = sample_of_13(v1.clone());
                    let v13: US3 = match v7 {
                        US2::US2_None => {
                            US3::US3_SampleUnread(v1)
                        }
                        US2::US2_Some(v8, v9) => {
                            let mut v8: i32 = v8;
                            let mut v9: i32 = v9;
                            admitted_by_19(v4, v5, v6, v8, v9)
                        }
                    };
                    match &v13 {
                        US3::US3_Admitted => {
                            let v14: Rc<str> = scoped_line_20(v4, v5, v6, v2);
                            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (true, v14, v15)
                        }
                        _ => {
                            let v16: Rc<str> = reason_of_23(v13);
                            (false, v2, v16)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v20, v21, v22)
        }
    };
    (v23, v24, v25)
}
fn sample_command_25() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cat /proc/loadavg; grep MemAvailable /proc/meminfo"); } LIT.with(|lit| lit.clone()) };
    v0
}
fn method26(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
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
fn gated_line_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<RefCell<Vec<(bool, Rc<str>, Rc<str>)>>> {
    let v3: Rc<str> = gate_path_1(v2, v1.clone());
    let v4: bool = file_exists_2(v3.clone());
    let v17: Rc<str> = if v4 {
        let mut v6: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v3);
        let mut v8: Vec<u8> = v6.unwrap();
        let mut v9: Vec<u8> = method3(v8);
        let v11: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v9);
        let v13: std::string::String = v11.unwrap();
        let v15: Rc<str> = Rc::<str>::from(String::as_str(&v13));
        v15
    } else {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v16
    };
    let v18: US0 = gate_of_text_4(v17);
    let (mut v69, mut v70, mut v71): (bool, Rc<str>, Rc<str>) = match &v18 {
        US0::US0_Ungated => {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            gated_of_12(v18, v19, v0)
        }
        _ => {
            let (mut v66, mut v67, mut v68): (bool, Rc<str>, Rc<str>) = match &v18 {
                US0::US0_Gated(v23, v24, v25) => {
                    let v23: i32 = *v23;
                    let v24: i32 = *v24;
                    let v25: i32 = *v25;
                    let v26: Rc<str> = sample_command_25();
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh"); } LIT.with(|lit| lit.clone()) };
                    let mut v28: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v27)));
                    let v29: bool = &*v1 == "";
                    let v47: Rc<UH0> = if v29 {
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ConnectTimeout=10"); } LIT.with(|lit| lit.clone()) };
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-o"); } LIT.with(|lit| lit.clone()) };
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BatchMode=yes"); } LIT.with(|lit| lit.clone()) };
                        let v33: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v34: Rc<UH0> = Rc::new(UH0::UH0_Cons(v31.clone(), v33));
                        let v35: Rc<UH0> = Rc::new(UH0::UH0_Cons(v32, v34));
                        let v36: Rc<UH0> = Rc::new(UH0::UH0_Cons(v31, v35));
                        Rc::new(UH0::UH0_Cons(v30, v36))
                    } else {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ConnectTimeout=10"); } LIT.with(|lit| lit.clone()) };
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-o"); } LIT.with(|lit| lit.clone()) };
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BatchMode=yes"); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v42: Rc<UH0> = Rc::new(UH0::UH0_Cons(v39.clone(), v41));
                        let v43: Rc<UH0> = Rc::new(UH0::UH0_Cons(v40, v42));
                        let v44: Rc<UH0> = Rc::new(UH0::UH0_Cons(v39, v43));
                        let v45: Rc<UH0> = Rc::new(UH0::UH0_Cons(v38, v44));
                        Rc::new(UH0::UH0_Cons(v1, v45))
                    };
                    let v48: bool = &*v26 == "";
                    let v50: Rc<UH0> = if v48 {
                        v47
                    } else {
                        Rc::new(UH0::UH0_Cons(v26, v47))
                    };
                    let v51: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v52: Rc<UH0> = method8(v50, v51);
                    method26(v28.clone(), v52);
                    let mut v53: Rc<std::io::Result<std::process::Output>> = Rc::new(v28.borrow_mut().output());
                    let v54: bool = v53.is_ok();
                    let (mut v60, mut v61, mut v62): (bool, i32, Rc<str>) = if v54 {
                        let v55: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v53).as_ref().unwrap().stderr).as_ref());
                        let v56: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v53).as_ref().unwrap().stdout).as_ref());
                        let v57: Rc<str> = string_concat(&[&*v56, &*v55]);
                        let v58: i32 = (&*v53).as_ref().unwrap().status.code().unwrap_or(-2);
                        (true, v58, v57)
                    } else {
                        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        (false, -1i32, v59)
                    };
                    gated_of_12(v18, v62, v0)
                }
                _ => unreachable!(),
            };
            (v66, v67, v68)
        }
    };
    Rc::new(RefCell::new(vec![(v69, v70, v71)]))
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(bool, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(bool, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> Rc<RefCell<Vec<(bool, Rc<str>, Rc<str>)>>> {
        gated_line_0(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_job_remote_gated_line(v0: &str, v1: &str, v2: &str) -> Rc<RefCell<Vec<(bool, Rc<str>, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
