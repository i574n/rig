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
fn method1(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn drift_named_2(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("receipt"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v1.contains(&*v2);
    let v6: bool = if v3 {
        true
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ledger"); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v1.contains(&*v4);
        v5
    };
    let v9: bool = if v6 {
        true
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gauge"); } LIT.with(|lit| lit.clone()) };
        let mut v8: bool = v1.contains(&*v7);
        v8
    };
    let v12: bool = if v9 {
        true
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rating"); } LIT.with(|lit| lit.clone()) };
        let v11: bool = v1.contains(&*v10);
        v11
    };
    let v15: bool = if v12 {
        true
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("authority"); } LIT.with(|lit| lit.clone()) };
        let v14: bool = v1.contains(&*v13);
        v14
    };
    let v18: bool = if v15 {
        true
    } else {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settlement"); } LIT.with(|lit| lit.clone()) };
        let v17: bool = v1.contains(&*v16);
        v17
    };
    if v18 {
        true
    } else {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("burnin"); } LIT.with(|lit| lit.clone()) };
        let v20: bool = v1.contains(&*v19);
        v20
    }
}
fn has_test_block_4(mut v0: Rc<str>) -> bool {
    loop {
        let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":# "); } LIT.with(|lit| lit.clone()) };
        let v3: bool = v1.starts_with(&*v2);
        let v6: bool = if v3 {
            true
        } else {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test "); } LIT.with(|lit| lit.clone()) };
            let v5: bool = v1.starts_with(&*v4);
            v5
        };
        let v10: bool = if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" static ="); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v1.contains(&*v7);
            let v9: bool = v8 == false;
            v9
        } else {
            false
        };
        let v11: bool = &*v0 == "";
        if v11 {
            return false;
        } else {
            if v10 {
                return true;
            } else {
                let v12: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
                v0 = v12;
                continue;
            }
        }
    }
}
fn any_tested_3(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    loop {
        let v3: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(10u8)).next().unwrap_or(""));
        let mut v4: Rc<str> = string_concat(&[&*v0, "/", &*v3]);
        let v5: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v4).unwrap_or_default().as_str());
        let v6: bool = &*v1 == "";
        if v6 {
            return false;
        } else {
            let mut v7: bool = has_test_block_4(v5);
            if v7 {
                return true;
            } else {
                let mut v8: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
                v1 = v8;
                continue;
            }
        }
    }
}
fn method6(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
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
fn method7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
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
fn manifest_crates_5(mut v0: Rc<str>) -> bool {
    loop {
        let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
        let mut v2: i32 = (v1.clone().len() as i32);
        let v3: i32 = 0i32;
        let mut v4: i32 = method6(v1.clone(), v2, v3);
        let v5: i32 = v2.wrapping_sub(1i32);
        let v6: Rc<str> = string_slice(&v1, v4 as i64, v5 as i64);
        let mut v7: i32 = (v6.clone().len() as i32);
        let mut v8: i32 = method7(v6.clone(), v7);
        let v9: Rc<str> = string_slice(&v6, 0i32 as i64, v8 as i64);
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("crates:"); } LIT.with(|lit| lit.clone()) };
        let v11: bool = v9.starts_with(&*v10);
        let v14: bool = if v11 {
            true
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("crates ="); } LIT.with(|lit| lit.clone()) };
            let v13: bool = v9.starts_with(&*v12);
            v13
        };
        let v17: bool = if v14 {
            true
        } else {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("crates="); } LIT.with(|lit| lit.clone()) };
            let v16: bool = v9.starts_with(&*v15);
            v16
        };
        let v18: bool = &*v0 == "";
        if v18 {
            return false;
        } else {
            if v17 {
                return true;
            } else {
                let v19: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
                v0 = v19;
                continue;
            }
        }
    }
}
fn package_target_findings_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: Option<usize> = v4.rfind(&*v3);
    let v6: Option<usize> = method1(v5);
    let mut v7: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v8: Option<i32> = v6.map(|x| v7(x));
    let v10: i32 = v8.unwrap_or(-1i32);
    let v11: bool = v10 < 0i32;
    let v20: Rc<str> = if v11 {
        v4.clone()
    } else {
        let v12: i32 = v10.wrapping_add(1i32);
        let v13: i32 = (v4.clone().len() as i32);
        let v14: i32 = v13.wrapping_sub(v10);
        let v15: i32 = v14.wrapping_sub(1i32);
        let v16: usize = ((v12) as usize);
        let v17: i32 = v12.wrapping_add(v15);
        let v18: usize = ((v17) as usize);
        let v19: Rc<str> = Rc::<str>::from(&(v4)[v16..v18]);
        v19
    };
    let v21: bool = drift_named_2(v20);
    let v24: bool = if v21 {
        let v22: bool = any_tested_3(v4.clone(), v0);
        let v23: bool = v22 == false;
        v23
    } else {
        false
    };
    let v27: Rc<str> = if v24 {
        let v25: Rc<str> = string_concat(&[&*v4, "/package.spiproj:1  test-targets  a receipt-like package with no test that checks it against a fact\n"]);
        v25
    } else {
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v26
    };
    let v28: Rc<str> = string_concat(&[&*v4, "/"]);
    let v29: Rc<str> = string_concat(&[&*v28, "package.spiproj"]);
    let v30: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v29).unwrap_or_default().as_str());
    let v31: bool = manifest_crates_5(v30);
    let v34: Rc<str> = if v31 {
        let v33: Rc<str> = string_concat(&[&*v27, &*v29, ":1  test-targets  crates in a manifest; put -d on each test's own ///> line\n"]);
        v33
    } else {
        v27
    };
    let v35: Rc<str> = string_concat(&[&*v28, "spiral.toml"]);
    let v36: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v35).unwrap_or_default().as_str());
    let v37: bool = manifest_crates_5(v36);
    if v37 {
        let v39: Rc<str> = string_concat(&[&*v34, &*v35, ":1  test-targets  crates in a manifest; put -d on each test's own ///> line\n"]);
        v39
    } else {
        v34
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        package_target_findings_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_standards_package_target_findings(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
