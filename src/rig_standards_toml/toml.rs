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
#[derive(Clone)]
enum US0 {
    US0_Formatted(i32),
    US0_Unformatted(Rc<UH0>),
    US0_NoConfig(Rc<str>),
    US0_NoTaplo,
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn verdict_code_1(mut v0: US0) -> i32 {
    match v0 {
        US0::US0_Formatted(v1) => {
            let mut v1: i32 = v1;
            0i32
        }
        US0::US0_Unformatted(v2) => {
            let v2: Rc<UH0> = v2;
            1i32
        }
        _ => {
            2i32
        }
    }
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_2(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_3();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US1 = if v4 {
        let v5: Rc<str> = command_line_argument_4(v0);
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
            let v12: Rc<UH0> = arguments_from_2(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn get_environment_variable_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn file_exists_7(mut v0: Rc<str>) -> bool {
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
fn config_choice_10(mut v0: Rc<str>, mut v1: bool, mut v2: bool) -> Rc<str> {
    let v4: bool = if v1 {
        true
    } else {
        let v3: bool = v2 == false;
        v3
    };
    if v4 {
        let v5: Rc<str> = string_concat(&[&*v0, "/rig"]);
        v5
    } else {
        let mut v6: Rc<str> = string_concat(&[&*v0, "/kit"]);
        v6
    }
}
fn directory_exists_11(mut v0: Rc<str>) -> bool {
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
fn config_dir_9() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RIG_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = get_environment_variable_6(v0);
    let v2: bool = &*v1 != "";
    let v5: Rc<str> = if v2 {
        v1
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
        get_environment_variable_6(v3)
    };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("APPDATA"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = get_environment_variable_6(v6);
    let mut v8: bool = &*v7 != "";
    let v12: Rc<str> = if v8 {
        v7
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HOME"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = get_environment_variable_6(v9);
        let v11: Rc<str> = string_concat(&[&*v10, "/.config"]);
        v11
    };
    let v13: Rc<str> = string_concat(&[&*v12, "/rig"]);
    let v14: bool = directory_exists_11(v13);
    let v15: bool = &*v5 != "";
    if v15 {
        v5
    } else {
        let v16: Rc<str> = string_concat(&[&*v12, "/kit"]);
        let mut v17: bool = directory_exists_11(v16);
        config_choice_10(v12, v14, v17)
    }
}
fn bundled_config_8() -> Rc<str> {
    let mut v0: Rc<str> = config_dir_9();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = string_concat(&[&*v3, "/.taplo.toml"]);
    let v5: bool = file_exists_7(v4.clone());
    if v5 {
        v4
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v6
    }
}
fn taplo_config_12(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: bool = &*v2 != "";
    if v4 {
        v2
    } else {
        if v1 {
            v0
        } else {
            v3
        }
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method14(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
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
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method17(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<str> = Rc::<str>::from((v2).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                let v5: bool = &*v4 == "";
                let mut v8: bool = if v5 {
                    true
                } else {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("manifest.toml"); } LIT.with(|lit| lit.clone()) };
                    let mut v7: bool = v4.ends_with(&*v6);
                    v7
                };
                let v12: Rc<UH0> = if v8 {
                    v1
                } else {
                    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v9));
                    method17(v1, v10)
                };
                (v0, v1) = (v3, v12);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method19(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
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
        while method19(v7, &v9) {
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
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method13(v16, v17)
    }
}
fn toml_files_15(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_18(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method16(v5, v6)
}
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: bool = &*v2 == "";
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
fn method23(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method24(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: bool = &*v1 == "";
                let v5: Rc<str> = if v4 {
                    v2
                } else {
                    v1
                };
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    Rc::new(move |mut v2: Rc<UH0>| -> US2 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US2> = method25(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method25(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    closure2(v0, v1)
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("not properly formatted path=\""); } LIT.with(|lit| lit.clone()) };
                let v5: Option<usize> = v2.find(&*v4);
                let mut v6: Option<usize> = method23(v5);
                let mut v7: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v8: Option<i32> = v6.map(|x| v7(x));
                let v10: i32 = v8.unwrap_or(-1i32);
                let v11: bool = v10 < 0i32;
                let v35: Rc<UH0> = if v11 {
                    v1
                } else {
                    let v12: i32 = v10.wrapping_add(29i32);
                    let v13: i32 = (v2.clone().len() as i32);
                    let v14: i32 = v13.wrapping_sub(v10);
                    let v15: i32 = v14.wrapping_sub(29i32);
                    let v16: usize = ((v12) as usize);
                    let mut v17: i32 = v12.wrapping_add(v15);
                    let v18: usize = ((v17) as usize);
                    let v19: Rc<str> = Rc::<str>::from(&(v2)[v16..v18]);
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<UH0> = split_list_18(v20, v19);
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = method24(v21, v22);
                    let v24: bool = &*v23 == "";
                    let v30: bool = if v24 {
                        true
                    } else {
                        let v25: i32 = 0i32;
                        let mut v26: Rc<dyn Fn(Rc<UH0>) -> US2> = method25(v23.clone(), v25);
                        let v27: US2 = v26(v1.clone());
                        let v28: bool = match v27 {
                            US2::US2_None => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v29: bool = v28 != true;
                        v29
                    };
                    if v30 {
                        v1
                    } else {
                        let v31: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v31));
                        method17(v1, v32)
                    }
                };
                (v0, v1) = (v3, v35);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn unformatted_paths_21(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_18(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method22(v5, v6)
}
fn method26(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn toml_verdict_of_5(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = string_concat(&[&*v3, "/.taplo.toml"]);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TAPLO_CONFIG"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = get_environment_variable_6(v5);
    let mut v7: Rc<str> = Rc::<str>::from(v6.replace(&*v1, &*v2));
    let mut v8: bool = file_exists_7(v4.clone());
    let mut v9: Rc<str> = bundled_config_8();
    let v10: Rc<str> = taplo_config_12(v4, v8, v7, v9);
    let v11: bool = &*v10 == "";
    let v14: bool = if v11 {
        true
    } else {
        let v12: bool = file_exists_7(v10.clone());
        let v13: bool = v12 == false;
        v13
    };
    if v14 {
        US0::US0_NoConfig(v0)
    } else {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git"); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v16)));
        let v18: bool = &*v0 == "";
        let mut v26: Rc<UH0> = if v18 {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v19, v20))
        } else {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v22, v23));
            Rc::new(UH0::UH0_Cons(v0, v24))
        };
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ls-files"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*.toml"); } LIT.with(|lit| lit.clone()) };
        let v29: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v30: Rc<UH0> = Rc::new(UH0::UH0_Cons(v28, v29));
        let v31: Rc<UH0> = Rc::new(UH0::UH0_Cons(v27, v30));
        let v32: Rc<UH0> = method13(v26, v31);
        method14(v17.clone(), v32);
        let mut v33: Rc<std::io::Result<std::process::Output>> = Rc::new(v17.borrow_mut().output());
        let v34: bool = v33.is_ok();
        let (mut v39, mut v40, mut v41): (bool, i32, Rc<str>) = if v34 {
            let v35: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v33).as_ref().unwrap().stdout).as_ref());
            let v36: Rc<str> = string_concat(&[&*v35, ""]);
            let v37: i32 = (&*v33).as_ref().unwrap().status.code().unwrap_or(-2);
            (true, v37, v36)
        } else {
            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, -1i32, v38)
        };
        let v42: Rc<UH0> = toml_files_15(v41);
        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("taplo"); } LIT.with(|lit| lit.clone()) };
        let mut v44: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v43)));
        let v60: Rc<UH0> = if v11 {
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--config"); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--check"); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fmt"); } LIT.with(|lit| lit.clone()) };
            let v48: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v47, v48));
            let v50: Rc<UH0> = Rc::new(UH0::UH0_Cons(v46, v49));
            Rc::new(UH0::UH0_Cons(v45, v50))
        } else {
            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--config"); } LIT.with(|lit| lit.clone()) };
            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--check"); } LIT.with(|lit| lit.clone()) };
            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fmt"); } LIT.with(|lit| lit.clone()) };
            let v55: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v56: Rc<UH0> = Rc::new(UH0::UH0_Cons(v54, v55));
            let v57: Rc<UH0> = Rc::new(UH0::UH0_Cons(v53, v56));
            let v58: Rc<UH0> = Rc::new(UH0::UH0_Cons(v52, v57));
            Rc::new(UH0::UH0_Cons(v10, v58))
        };
        let v61: Rc<UH0> = method20(v42.clone(), v60);
        let v62: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v63: Rc<UH0> = method13(v61, v62);
        method14(v44.clone(), v63);
        let mut v64: Rc<std::io::Result<std::process::Output>> = Rc::new(v44.borrow_mut().output());
        let v65: bool = v64.is_ok();
        let (mut v71, mut v72, mut v73): (bool, i32, Rc<str>) = if v65 {
            let v66: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v64).as_ref().unwrap().stderr).as_ref());
            let v67: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v64).as_ref().unwrap().stdout).as_ref());
            let v68: Rc<str> = string_concat(&[&*v67, &*v66]);
            let v69: i32 = (&*v64).as_ref().unwrap().status.code().unwrap_or(-2);
            (true, v69, v68)
        } else {
            let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, -1i32, v70)
        };
        let v74: bool = v71 == false;
        if v74 {
            US0::US0_NoTaplo
        } else {
            let v76: Rc<UH0> = unformatted_paths_21(v73);
            match &*v76 {
                UH0::UH0_Nil => {
                    let v77: i32 = 0i32;
                    let v78: i32 = method26(v42, v77);
                    US0::US0_Formatted(v78)
                }
                _ => {
                    US0::US0_Unformatted(v76)
                }
            }
        }
    }
}
fn method29(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_28(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method29(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method30(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method30(v3, v1);
            let v5: Rc<str> = string_concat(&[&*v2, ":1  toml-format  not formatted by the taplo config"]);
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn verdict_lines_27(mut v0: US0) -> Rc<UH0> {
    match v0 {
        US0::US0_Formatted(v1) => {
            let mut v1: i32 = v1;
            let v2: Rc<str> = format_real_28(v1);
            let v4: Rc<str> = string_concat(&["rig standards toml ok files=", &*v2, " unformatted=0"]);
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v4, v5))
        }
        US0::US0_NoConfig(v17) => {
            let mut v17: Rc<str> = v17;
            let v19: Rc<str> = string_concat(&["rig standards toml: no .taplo.toml at ", &*v17, " and TAPLO_CONFIG names no file"]);
            let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v19, v20))
        }
        US0::US0_NoTaplo => {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig standards toml: taplo is not on PATH"); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v22, v23))
        }
        US0::US0_Unformatted(v7) => {
            let mut v7: Rc<UH0> = v7;
            let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v9: Rc<UH0> = method30(v7.clone(), v8);
            let v10: i32 = 0i32;
            let v11: i32 = method26(v7, v10);
            let v12: Rc<str> = format_real_28(v11);
            let v13: Rc<str> = string_concat(&["rig standards toml refused unformatted=", &*v12]);
            let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v14));
            method17(v9, v15)
        }
    }
}
fn method31(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
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
fn toml_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_2(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            let mut v8: US0 = toml_verdict_of_5(v6);
                            let mut v9: Rc<UH0> = verdict_lines_27(v8.clone());
                            method31(v9);
                            verdict_code_1(v8)
                        }
                        _ => {
                            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: standards --toml <repo>"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v11);
                            2i32
                        }
                    }
                }
                _ => {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: standards --toml <repo>"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v13);
                    2i32
                }
            }
        }
        _ => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: standards --toml <repo>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v15);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        toml_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_standards_toml_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
