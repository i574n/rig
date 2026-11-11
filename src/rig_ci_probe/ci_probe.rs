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
enum US1 {
    US1_ToolRust,
    US1_ToolZig,
    US1_ToolOtp,
    US1_ToolElixir,
    US1_ToolGleam,
    US1_ToolRebar,
    US1_ToolPython,
}
#[derive(Clone)]
enum US0 {
    US0_Some(US1),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
fn tool_of_name_1(mut v0: Rc<str>) -> US0 {
    let mut v1: bool = &*v0 == "rust";
    if v1 {
        let v2: US1 = US1::US1_ToolRust;
        US0::US0_Some(v2)
    } else {
        let mut v4: bool = &*v0 == "zig";
        if v4 {
            let mut v5: US1 = US1::US1_ToolZig;
            US0::US0_Some(v5)
        } else {
            let mut v7: bool = &*v0 == "otp";
            if v7 {
                let v8: US1 = US1::US1_ToolOtp;
                US0::US0_Some(v8)
            } else {
                let v10: bool = &*v0 == "elixir";
                if v10 {
                    let v11: US1 = US1::US1_ToolElixir;
                    US0::US0_Some(v11)
                } else {
                    let v13: bool = &*v0 == "gleam";
                    if v13 {
                        let mut v14: US1 = US1::US1_ToolGleam;
                        US0::US0_Some(v14)
                    } else {
                        let v16: bool = &*v0 == "rebar3";
                        if v16 {
                            let v17: US1 = US1::US1_ToolRebar;
                            US0::US0_Some(v17)
                        } else {
                            let v19: bool = &*v0 == "python";
                            if v19 {
                                let v20: US1 = US1::US1_ToolPython;
                                US0::US0_Some(v20)
                            } else {
                                US0::US0_None
                            }
                        }
                    }
                }
            }
        }
    }
}
fn unquoted_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v3));
    v6
}
fn channel_line_4(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> US2 {
    loop {
        let v2: i32 = (v0.clone().borrow().len() as i32);
        let v3: bool = v1 >= v2;
        if v3 {
            return US2::US2_None;
        } else {
            let mut v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: Rc<str> = Rc::<str>::from((v5).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("channel"); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v6.starts_with(&*v7);
            let v11: bool = if v8 {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v6.contains(&*v9);
                v10
            } else {
                false
            };
            if v11 {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v6.split(&*v12).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let mut v14: Rc<str> = v13.borrow()[1i32 as usize].clone();
                let v15: Rc<str> = unquoted_5(v14);
                let v16: bool = &*v15 == "";
                if v16 {
                    return US2::US2_None;
                } else {
                    return US2::US2_Some(v15);
                }
            } else {
                let v20: i32 = v1.wrapping_add(1i32);
                v1 = v20;
                continue;
            }
        }
    }
}
fn toolchain_file_channel_3(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[toolchain]"); } LIT.with(|lit| lit.clone()) };
    let mut v6: bool = v4.contains(&*v5);
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v4.split(&*v7).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v9: i32 = 0i32;
        channel_line_4(v8, v9)
    } else {
        let v11: bool = &*v4 == "";
        let mut v14: bool = if v11 {
            true
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v13: bool = v4.contains(&*v12);
            v13
        };
        if v14 {
            US2::US2_None
        } else {
            US2::US2_Some(v4)
        }
    }
}
fn method7(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: &str = &*v0;
    let mut v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let mut v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method7(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn file_exists_9(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method10(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn text_at_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = file_exists_9(v0.clone());
    if v1 {
        let v3: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let mut v5: Vec<u8> = v3.unwrap();
        let mut v6: Vec<u8> = method10(v5);
        let v8: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v6);
        let v10: std::string::String = v8.unwrap();
        let v12: Rc<str> = Rc::<str>::from(String::as_str(&v10));
        v12
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    }
}
fn repo_toolchain_2(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-toolchain.toml"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = method6(v0.clone(), v1);
    let v3: Rc<str> = text_at_8(v2);
    let mut v4: US2 = toolchain_file_channel_3(v3);
    match v4 {
        US2::US2_None => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-toolchain"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = method6(v0, v7);
            let mut v9: Rc<str> = text_at_8(v8);
            toolchain_file_channel_3(v9)
        }
        US2::US2_Some(v5) => {
            let mut v5: Rc<str> = v5;
            US2::US2_Some(v5)
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn word_at_15(mut v0: i32, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v4: i32 = (v3.clone().borrow().len() as i32);
    let mut v5: bool = v0 < v4;
    if v5 {
        let mut v6: Rc<str> = v3.borrow()[v0 as usize].clone();
        v6
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v7
    }
}
fn method18(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn earlier_cut_17(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let v3: Option<usize> = v0.find(&*v2);
    let mut v4: Option<usize> = method18(v3);
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let v8: i32 = v6.unwrap_or(-1i32);
    let mut v9: bool = v8 > 0i32;
    let v13: bool = if v9 {
        let v10: bool = v1 < 0i32;
        if v10 {
            true
        } else {
            let v11: bool = v8 < v1;
            v11
        }
    } else {
        false
    };
    if v13 {
        v8
    } else {
        v1
    }
}
fn without_host_triple_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = -1i32;
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-x86_64-"); } LIT.with(|lit| lit.clone()) };
    let v3: i32 = earlier_cut_17(v0.clone(), v1, v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-aarch64-"); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = earlier_cut_17(v0.clone(), v3, v4);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-i686-"); } LIT.with(|lit| lit.clone()) };
    let mut v7: i32 = earlier_cut_17(v0.clone(), v5, v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-arm"); } LIT.with(|lit| lit.clone()) };
    let mut v9: i32 = earlier_cut_17(v0.clone(), v7, v8);
    let v10: bool = v9 < 0i32;
    if v10 {
        v0
    } else {
        let v11: usize = ((0i32) as usize);
        let v12: usize = ((v9) as usize);
        let v13: Rc<str> = Rc::<str>::from(&(v0)[v11..v12]);
        v13
    }
}
fn channels_from_13(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method14(v2, v5);
        } else {
            let mut v7: i32 = 0i32;
            let v8: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v9: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let v10: Rc<str> = word_at_15(v7, v9);
            let v11: Rc<str> = without_host_triple_16(v10);
            let v12: i32 = v1.wrapping_add(1i32);
            let v13: bool = &*v11 == "";
            let v15: Rc<UH0> = if v13 {
                v2
            } else {
                Rc::new(UH0::UH0_Cons(v11, v2))
            };
            (v1, v2) = (v12, v15);
            continue;
        }
    }
}
fn installed_channels_12(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = 0i32;
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    channels_from_13(v5, v6, v7)
}
fn method21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method22(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
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
fn capture_tried_20(mut v0: Rc<UH0>, mut v1: US2, mut v2: Rc<str>) -> US2 {
    match &v1 {
        US2::US2_None => {
            let mut v4: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v2)));
            let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v6: Rc<UH0> = method21(v0, v5);
            let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v8: Rc<UH0> = method14(v6, v7);
            method22(v4.clone(), v8);
            let mut v9: Rc<std::io::Result<std::process::Output>> = Rc::new(v4.borrow_mut().output());
            let v10: bool = v9.is_ok();
            let (mut v16, mut v17, mut v18): (bool, i32, Rc<str>) = if v10 {
                let v11: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v9).as_ref().unwrap().stderr).as_ref());
                let v12: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v9).as_ref().unwrap().stdout).as_ref());
                let v13: Rc<str> = string_concat(&[&*v12, &*v11]);
                let mut v14: i32 = (&*v9).as_ref().unwrap().status.code().unwrap_or(-2);
                (true, v14, v13)
            } else {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (false, -1i32, v15)
            };
            let v20: bool = if v16 {
                let v19: bool = v17 == 0i32;
                v19
            } else {
                false
            };
            if v20 {
                US2::US2_Some(v18)
            } else {
                US2::US2_None
            }
        }
        US2::US2_Some(v3) => {
            let v3: Rc<str> = v3.clone();
            v1
        }
    }
}
fn captured_19(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US2 {
    let v2: Rc<str> = string_concat(&[&*v0, ".cmd"]);
    let v3: Rc<str> = string_concat(&[&*v0, ".bat"]);
    let mut v4: US2 = US2::US2_None;
    let mut v5: US2 = capture_tried_20(v1.clone(), v4, v0);
    let mut v6: US2 = capture_tried_20(v1.clone(), v5, v2);
    capture_tried_20(v1, v6, v3)
}
fn rust_installed_11() -> Rc<UH0> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustup"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3));
    let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v4));
    let mut v6: US2 = captured_19(v0, v5);
    match v6 {
        US2::US2_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US2::US2_Some(v7) => {
            let mut v7: Rc<str> = v7;
            installed_channels_12(v7)
        }
    }
}
fn first_line_25(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v4.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v7: Rc<str> = v6.borrow()[0i32 as usize].clone();
    let v8: Rc<str> = Rc::<str>::from((v7).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    v8
}
fn method26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 1i32;
    word_at_15(v1, v0)
}
fn version_from_output_24(mut v0: US1, mut v1: Rc<str>) -> US2 {
    let v2: Rc<str> = first_line_25(v1);
    let v15: Rc<str> = match v0 {
        US1::US1_ToolElixir => {
            v2
        }
        US1::US1_ToolGleam => {
            method26(v2)
        }
        US1::US1_ToolOtp => {
            v2
        }
        US1::US1_ToolPython => {
            method26(v2)
        }
        US1::US1_ToolRebar => {
            method26(v2)
        }
        US1::US1_ToolRust => {
            let v3: i32 = 0i32;
            let mut v4: Rc<str> = word_at_15(v3, v2);
            without_host_triple_16(v4)
        }
        US1::US1_ToolZig => {
            v2
        }
    };
    let v16: bool = &*v15 == "";
    if v16 {
        US2::US2_None
    } else {
        US2::US2_Some(v15)
    }
}
fn probe_of_27(mut v0: US1) -> (Rc<str>, Rc<UH0>, US2) {
    let (mut v61, mut v62, mut v63): (Rc<str>, Rc<UH0>, US2) = match &v0 {
        US1::US1_ToolRust => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustup"); } LIT.with(|lit| lit.clone()) };
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("active-toolchain"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v4));
            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v5));
            let mut v7: US2 = US2::US2_None;
            (v1, v6, v7)
        }
        _ => {
            let (mut v58, mut v59, mut v60): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                US1::US1_ToolZig => {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("version"); } LIT.with(|lit| lit.clone()) };
                    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v10));
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_ZIG"); } LIT.with(|lit| lit.clone()) };
                    let v13: US2 = US2::US2_Some(v12);
                    (v8, v11, v13)
                }
                _ => {
                    let (mut v55, mut v56, mut v57): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                        US1::US1_ToolOtp => {
                            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("erl"); } LIT.with(|lit| lit.clone()) };
                            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-noshell"); } LIT.with(|lit| lit.clone()) };
                            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-eval"); } LIT.with(|lit| lit.clone()) };
                            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("io:format(\"~s\",[element(2,file:read_file(filename:join([code:root_dir(),\"releases\",erlang:system_info(otp_release),\"OTP_VERSION\"])))]),halt()."); } LIT.with(|lit| lit.clone()) };
                            let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17, v18));
                            let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v19));
                            let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v20));
                            let v22: US2 = US2::US2_None;
                            (v14, v21, v22)
                        }
                        _ => {
                            let (mut v52, mut v53, mut v54): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                                US1::US1_ToolElixir => {
                                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elixir"); } LIT.with(|lit| lit.clone()) };
                                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--short-version"); } LIT.with(|lit| lit.clone()) };
                                    let v25: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v26: Rc<UH0> = Rc::new(UH0::UH0_Cons(v24, v25));
                                    let v27: US2 = US2::US2_None;
                                    (v23, v26, v27)
                                }
                                _ => {
                                    let (mut v49, mut v50, mut v51): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                                        US1::US1_ToolGleam => {
                                            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
                                            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--version"); } LIT.with(|lit| lit.clone()) };
                                            let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                            let v31: Rc<UH0> = Rc::new(UH0::UH0_Cons(v29, v30));
                                            let v32: US2 = US2::US2_None;
                                            (v28, v31, v32)
                                        }
                                        _ => {
                                            let (mut v46, mut v47, mut v48): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                                                US1::US1_ToolRebar => {
                                                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rebar3"); } LIT.with(|lit| lit.clone()) };
                                                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("version"); } LIT.with(|lit| lit.clone()) };
                                                    let v35: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                    let v36: Rc<UH0> = Rc::new(UH0::UH0_Cons(v34, v35));
                                                    let v37: US2 = US2::US2_None;
                                                    (v33, v36, v37)
                                                }
                                                _ => {
                                                    let (mut v43, mut v44, mut v45): (Rc<str>, Rc<UH0>, US2) = match v0 {
                                                        US1::US1_ToolPython => {
                                                            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("python"); } LIT.with(|lit| lit.clone()) };
                                                            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--version"); } LIT.with(|lit| lit.clone()) };
                                                            let v40: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                            let v41: Rc<UH0> = Rc::new(UH0::UH0_Cons(v39, v40));
                                                            let v42: US2 = US2::US2_None;
                                                            (v38, v41, v42)
                                                        }
                                                        _ => unreachable!(),
                                                    };
                                                    (v43, v44, v45)
                                                }
                                            };
                                            (v46, v47, v48)
                                        }
                                    };
                                    (v49, v50, v51)
                                }
                            };
                            (v52, v53, v54)
                        }
                    };
                    (v55, v56, v57)
                }
            };
            (v58, v59, v60)
        }
    };
    (v61, v62, v63)
}
fn get_environment_variable_29(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn probe_program_28(mut v0: Rc<str>, mut v1: &Rc<UH0>, mut v2: US2) -> Rc<str> {
    let v15: Rc<str> = match v2 {
        US2::US2_None => {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v13
        }
        US2::US2_Some(v3) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<str> = get_environment_variable_29(v3);
            let mut v5: bool = &*v4 == "";
            let v8: US2 = if v5 {
                US2::US2_None
            } else {
                US2::US2_Some(v4)
            };
            match v8 {
                US2::US2_None => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v10
                }
                US2::US2_Some(v9) => {
                    let mut v9: Rc<str> = v9;
                    v9
                }
            }
        }
    };
    let v16: Rc<str> = Rc::<str>::from((v15).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v17: bool = &*v16 == "";
    if v17 {
        v0
    } else {
        v15
    }
}
fn probed_version_23(mut v0: US1) -> US2 {
    let (mut v1, mut v2, mut v3): (Rc<str>, Rc<UH0>, US2) = probe_of_27(v0.clone());
    let mut v4: Rc<str> = probe_program_28(v1, &v2, v3);
    let mut v5: US2 = captured_19(v4, v2);
    match v5 {
        US2::US2_None => {
            US2::US2_None
        }
        US2::US2_Some(v6) => {
            let mut v6: Rc<str> = v6;
            version_from_output_24(v0, v6)
        }
    }
}
fn method31(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US2 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = &*v3 == &*v0;
                if v5 {
                    return US2::US2_Some(v3);
                } else {
                    v1 = v4;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US2::US2_None;
            }
        }
    }
}
fn rust_local_30(mut v0: US2, mut v1: Rc<UH0>, mut v2: US2) -> US2 {
    match v0 {
        US2::US2_None => {
            v2
        }
        US2::US2_Some(v3) => {
            let v3: Rc<str> = v3;
            let mut v4: US2 = method31(v3.clone(), v1);
            let mut v5: bool = match v4 {
                US2::US2_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let mut v6: bool = v5 != true;
            if v6 {
                US2::US2_Some(v3)
            } else {
                US2::US2_None
            }
        }
    }
}
fn local_version_0(mut v0: Rc<str>, mut v1: Rc<str>) -> (bool, Rc<str>) {
    let v2: US0 = tool_of_name_1(v0);
    let v13: US2 = match v2 {
        US0::US0_None => {
            US2::US2_None
        }
        US0::US0_Some(v3) => {
            let v3: US1 = v3;
            match &v3 {
                US1::US1_ToolRust => {
                    let mut v4: US2 = repo_toolchain_2(v1);
                    let mut v5: Rc<UH0> = rust_installed_11();
                    let mut v6: US1 = US1::US1_ToolRust;
                    let mut v7: US2 = probed_version_23(v6);
                    rust_local_30(v4, v5, v7)
                }
                _ => {
                    probed_version_23(v3)
                }
            }
        }
    };
    match v13 {
        US2::US2_None => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, v15)
        }
        US2::US2_Some(v14) => {
            let mut v14: Rc<str> = v14;
            (true, v14)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (bool, Rc<str>) {
        local_version_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method33(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US2 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = &*v3 == &*v0;
                if v5 {
                    return US2::US2_Some(v3);
                } else {
                    v1 = v4;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US2::US2_None;
            }
        }
    }
}
fn rust_channel_installed_32(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<UH0> = rust_installed_11();
    let v2: US2 = method33(v0, v1);
    let v3: bool = match v2 {
        US2::US2_None => {
            true
        }
        _ => {
            false
        }
    };
    let mut v4: bool = v3 != true;
    v4
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        rust_channel_installed_32(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_ci_local_version(v0: &str, v1: &str) -> (bool, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_ci_rust_installed(v0: &str) -> bool {
    closure2()(Rc::<str>::from(v0))
}
