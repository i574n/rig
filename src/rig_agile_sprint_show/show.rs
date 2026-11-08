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
enum US0 {
    US0_ShowItems,
    US0_ShowNotes,
}
#[derive(Clone)]
enum US1 {
    US1_Some(i64),
    US1_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
fn depth_word_2(mut v0: US0) -> Rc<str> {
    let mut v1: bool = match v0 {
        US0::US0_ShowNotes => {
            true
        }
        _ => {
            false
        }
    };
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("retro"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
        v3
    }
}
fn committed_from_3(mut v0: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let (mut v5, mut v6, mut v7): (Rc<str>, i32, bool) = v0.borrow()[v1 as usize].clone();
            let mut v8: i32 = v1.wrapping_add(1i32);
            let v10: i32 = if v7 {
                let mut v9: i32 = v2.wrapping_add(v6);
                v9
            } else {
                v2
            };
            (v1, v2) = (v8, v10);
            continue;
        }
    }
}
fn method5(mut v0: Option<i64>) -> Option<i64> {
    v0
}
fn closure1() -> Rc<dyn Fn((i64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i64)) -> US1> = Rc::new(move |mut v0: (i64)| -> US1 {
        let mut v1: i64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn days_of_4(mut v0: Rc<str>) -> i64 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = v1 < 10i32;
    if v2 {
        0i64
    } else {
        let v3: usize = ((0i32) as usize);
        let v4: usize = ((4i32) as usize);
        let v5: Rc<str> = Rc::<str>::from(&(v0)[v3..v4]);
        let v6: Option<i64> = (v5).trim().parse::<i64>().ok();
        let v7: Option<i64> = method5(v6);
        let mut v8: Rc<dyn Fn((i64)) -> US1> = closure1();
        let mut v9: Option<US1> = v7.map(|x| v8(x));
        let v11: US1 = US1::US1_None;
        let v12: US1 = v11.clone();
        let mut v13: US1 = v9.unwrap_or(v12);
        let v16: i64 = match v13 {
            US1::US1_None => {
                0i64
            }
            US1::US1_Some(v14) => {
                let v14: i64 = v14;
                v14
            }
        };
        let v17: usize = ((5i32) as usize);
        let v18: usize = ((7i32) as usize);
        let v19: Rc<str> = Rc::<str>::from(&(v0)[v17..v18]);
        let v20: Option<i64> = (v19).trim().parse::<i64>().ok();
        let v21: Option<i64> = method5(v20);
        let mut v22: Option<US1> = v21.map(|x| v8(x));
        let v24: US1 = US1::US1_None;
        let v25: US1 = v24.clone();
        let v26: US1 = v22.unwrap_or(v25);
        let v29: i64 = match v26 {
            US1::US1_None => {
                0i64
            }
            US1::US1_Some(v27) => {
                let v27: i64 = v27;
                v27
            }
        };
        let v30: usize = ((8i32) as usize);
        let v31: usize = ((10i32) as usize);
        let v32: Rc<str> = Rc::<str>::from(&(v0)[v30..v31]);
        let v33: Option<i64> = (v32).trim().parse::<i64>().ok();
        let v34: Option<i64> = method5(v33);
        let mut v35: Option<US1> = v34.map(|x| v8(x));
        let v37: US1 = US1::US1_None;
        let v38: US1 = v37.clone();
        let v39: US1 = v35.unwrap_or(v38);
        let v42: i64 = match v39 {
            US1::US1_None => {
                0i64
            }
            US1::US1_Some(v40) => {
                let v40: i64 = v40;
                v40
            }
        };
        let v43: bool = v29 <= 2i64;
        let v45: i64 = if v43 {
            let v44: i64 = v16.wrapping_sub(1i64);
            v44
        } else {
            v16
        };
        let v46: i64 = v45.wrapping_div(400i64);
        let v47: i64 = v45.wrapping_rem(400i64);
        let v48: bool = v47 != 0i64;
        let v50: bool = if v48 {
            let v49: bool = v45 < 0i64;
            v49
        } else {
            false
        };
        let v52: i64 = if v50 {
            let v51: i64 = v46.wrapping_sub(1i64);
            v51
        } else {
            v46
        };
        let v53: i64 = v52.wrapping_mul(400i64);
        let v54: i64 = v45.wrapping_sub(v53);
        let v55: i64 = v54.wrapping_mul(365i64);
        let v56: i64 = v54.wrapping_div(4i64);
        let v57: i64 = v55.wrapping_add(v56);
        let v58: i64 = v54.wrapping_div(100i64);
        let v59: i64 = v57.wrapping_sub(v58);
        let v60: bool = v29 > 2i64;
        let v63: i64 = if v60 {
            let v61: i64 = v29.wrapping_sub(3i64);
            v61
        } else {
            let v62: i64 = v29.wrapping_add(9i64);
            v62
        };
        let v64: i64 = 153i64.wrapping_mul(v63);
        let v65: i64 = v64.wrapping_add(2i64);
        let v66: i64 = v65.wrapping_div(5i64);
        let v67: i64 = v66.wrapping_add(v42);
        let v68: i64 = v67.wrapping_sub(1i64);
        let v69: i64 = v59.wrapping_add(v68);
        let v70: i64 = v52.wrapping_mul(146097i64);
        let v71: i64 = v70.wrapping_add(v69);
        let v72: i64 = v71.wrapping_sub(719468i64);
        v72
    }
}
fn method7(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_6(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method7(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_9(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method7(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn clipped_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = v1 > 160i32;
    if v2 {
        let v3: usize = ((0i32) as usize);
        let v4: usize = ((160i32) as usize);
        let v5: Rc<str> = Rc::<str>::from(&(v0)[v3..v4]);
        let v6: Rc<str> = string_concat(&[&*v5, "..."]);
        v6
    } else {
        v0
    }
}
fn method12(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method13(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn last_note_line_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = agile_note_listing::rig_agile_last_note(&v0, &v1);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v2.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v5: i32 = (v4.clone().borrow().len() as i32);
    let v6: bool = v5 < 2i32;
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v7
    } else {
        let mut v8: Rc<str> = v4.borrow()[1i32 as usize].clone();
        let mut v9: i32 = (v8.clone().len() as i32);
        let v10: i32 = 0i32;
        let v11: i32 = method12(v8.clone(), v9, v10);
        let v12: i32 = v9.wrapping_sub(1i32);
        let mut v13: Rc<str> = string_slice(&v8, v11 as i64, v12 as i64);
        let v14: i32 = (v13.clone().len() as i32);
        let v15: i32 = method13(v13.clone(), v14);
        let v16: Rc<str> = string_slice(&v13, 0i32 as i64, v15 as i64);
        clipped_11(v16)
    }
}
fn items_shown_8(mut v0: US0, mut v1: Rc<str>, mut v2: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v3: i32, mut v4: i32) -> i32 {
    loop {
        let v5: i32 = (v2.clone().borrow().len() as i32);
        let v6: bool = v3 >= v5;
        if v6 {
            return v4;
        } else {
            let (mut v7, mut v8, mut v9): (Rc<str>, i32, bool) = v2.borrow()[v3 as usize].clone();
            let v10: Rc<str> = agile_task_census::rig_agile_task_status_of(&v1, &v7);
            let mut v13: Rc<str> = if v9 {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("committed"); } LIT.with(|lit| lit.clone()) };
                v11
            } else {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
                v12
            };
            let v14: bool = &*v10 == "";
            let v16: Rc<str> = if v14 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown"); } LIT.with(|lit| lit.clone()) };
                v15
            } else {
                v10.clone()
            };
            let v19: Rc<str> = format_real_9(v8);
            let v24: Rc<str> = string_concat(&["  ", &*v7, " ", &*v19, "m ", &*v16, " ", &*v13]);
            println!("{}", v24);
            let v25: bool = match &v0 {
                US0::US0_ShowNotes => {
                    true
                }
                _ => {
                    false
                }
            };
            let v28: Rc<str> = if v25 {
                last_note_line_10(v1.clone(), v7)
            } else {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v27
            };
            let v29: bool = &*v28 != "";
            if v29 {
                let v30: Rc<str> = string_concat(&["    ", &*v28]);
                println!("{}", v30);
                ()
            };
            let v31: bool = &*v10 == "Done";
            let v32: bool = v31 && v9;
            let v34: i32 = if v32 {
                let v33: i32 = v4.wrapping_add(v8);
                v33
            } else {
                v4
            };
            let mut v35: i32 = v3.wrapping_add(1i32);
            (v3, v4) = (v35, v34);
            continue;
        }
    }
}
fn sprint_shown_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US0) -> i32 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v0.replace(&*v3, &*v4));
    let v6: Rc<str> = string_concat(&[&*v5, "/state"]);
    let v7: Rc<str> = string_concat(&[&*v6, "/sprints.spi"]);
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v7).unwrap_or_default().as_str());
    let mut v9: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>> = rig_agile_sprint_state::rig_agile_sprint_head(&v8, &v1);
    let v10: i32 = (v9.clone().borrow().len() as i32);
    let v11: bool = v10 == 0i32;
    if v11 {
        let v12: bool = &*v1 == "";
        let v15: Rc<str> = if v12 {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open sprint"); } LIT.with(|lit| lit.clone()) };
            v13
        } else {
            let v14: Rc<str> = string_concat(&["sprint ", &*v1]);
            v14
        };
        let v16: Rc<str> = depth_word_2(v2);
        let v19: Rc<str> = string_concat(&["kit error: agile sprint ", &*v16, ": no ", &*v15]);
        eprintln!("{}", v19);
        1i32
    } else {
        let (mut v20, mut v21, mut v22, mut v23, mut v24): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) = v9.borrow()[0i32 as usize].clone();
        let v25: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> = rig_agile_sprint_state::rig_agile_sprint_items(&v8, &v20);
        let v26: Rc<str> = string_concat(&[&*v6, "/agile.spi"]);
        let v27: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v26).unwrap_or_default().as_str());
        let v28: i32 = 0i32;
        let v29: i32 = 0i32;
        let v30: i32 = committed_from_3(&v25, v28, v29);
        let v31: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let v32: i64 = (v31 as i64);
        let v33: i64 = v32.wrapping_div(86400000i64);
        let v34: i64 = days_of_4(v23.clone());
        let mut v35: i64 = v34.wrapping_sub(v33);
        let v38: Rc<str> = if v24 {
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open"); } LIT.with(|lit| lit.clone()) };
            v36
        } else {
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("closed"); } LIT.with(|lit| lit.clone()) };
            v37
        };
        let v39: Rc<str> = depth_word_2(v2.clone());
        let v50: Rc<str> = format_real_6(v35);
        let v53: Rc<str> = string_concat(&["kit agile sprint ", &*v39, " ok id=", &*v20, " status=", &*v38, " start=", &*v22, " finish=", &*v23, " days_left=", &*v50, " goal=", &*v21]);
        println!("{}", v53);
        let v54: i32 = 0i32;
        let v55: i32 = 0i32;
        let v56: i32 = items_shown_8(v2, v27, &v25, v54, v55);
        let v57: Rc<str> = format_real_9(v30);
        let v60: Rc<str> = format_real_9(v56);
        let v62: Rc<str> = string_concat(&["  committed=", &*v57, "m done=", &*v60, "m"]);
        println!("{}", v62);
        0i32
    }
}
fn shown_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: US0 = US0::US0_ShowItems;
    sprint_shown_1(v1, v0, v2)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        shown_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn retro_14(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: Rc<str> = string_concat(&[&*v4, "/state/sprints.spi"]);
    let v6: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v5).unwrap_or_default().as_str());
    let v7: bool = &*v0 == "";
    let mut v9: Rc<str> = if v7 {
        let mut v8: Rc<str> = rig_agile_sprint_state::rig_agile_latest_sprint(&v6);
        v8
    } else {
        v0
    };
    let v10: US0 = US0::US0_ShowNotes;
    sprint_shown_1(v1, v9, v10)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        retro_14(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    Rc::new(move |mut v2: Rc<UH0>| -> US2 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US2> = method17(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method17(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    closure4(v0, v1)
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method18(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn done_since_16(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>>, mut v1: i32, mut v2: u64, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            return v3;
        } else {
            let (mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, u64) = v0.borrow()[v1 as usize].clone();
            let mut v9: bool = &*v7 == "Done";
            let v11: bool = if v9 {
                let v10: bool = v8 >= v2;
                v10
            } else {
                false
            };
            let v18: bool = if v11 {
                let v12: i32 = 0i32;
                let mut v13: Rc<dyn Fn(Rc<UH0>) -> US2> = method17(v6.clone(), v12);
                let v14: US2 = v13(v3.clone());
                let v15: bool = match v14 {
                    US2::US2_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v16: bool = v15 != true;
                let v17: bool = v16 == false;
                v17
            } else {
                false
            };
            let v19: i32 = v1.wrapping_add(1i32);
            let v23: Rc<UH0> = if v18 {
                let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v20));
                method18(v3, v21)
            } else {
                v3
            };
            (v1, v3) = (v19, v23);
            continue;
        }
    }
}
fn method19(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn method20(mut v0: Rc<str>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<str> = agile_task_census::rig_agile_task_status_of(&v0, &v2);
                let v6: Rc<str> = string_concat(&["  ", &*v2, " "]);
                let v7: bool = &*v4 == "";
                let mut v9: Rc<str> = if v7 {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown"); } LIT.with(|lit| lit.clone()) };
                    v8
                } else {
                    v4
                };
                let v10: Rc<str> = string_concat(&[&*v6, &*v9]);
                println!("{}", v10);
                let v11: Rc<str> = last_note_line_10(v0.clone(), v2);
                let v12: bool = &*v11 != "";
                if v12 {
                    let mut v13: Rc<str> = string_concat(&["    ", &*v11]);
                    println!("{}", v13);
                    ()
                };
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
fn retro_since_15(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: i64 = days_of_4(v0.clone());
    let v6: bool = v5 <= 0i64;
    if v6 {
        let v7: Rc<str> = string_concat(&["kit error: agile sprint retro --since expects a YYYY-MM-DD date, got ", &*v0]);
        eprintln!("{}", v7);
        2i32
    } else {
        let mut v8: Rc<str> = string_concat(&[&*v4, "/history/status_changes.spi"]);
        let mut v9: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v8).unwrap_or_default().as_str());
        let v10: Rc<str> = string_concat(&[&*v4, "/state/agile.spi"]);
        let v11: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v10).unwrap_or_default().as_str());
        let v12: i64 = v5.wrapping_mul(86400000i64);
        let mut v13: u64 = (v12 as u64);
        let v14: Rc<RefCell<Vec<(Rc<str>, Rc<str>, u64)>>> = agile_status_changes::rig_agile_status_changes(&v9);
        let v15: i32 = 0i32;
        let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v17: Rc<UH0> = done_since_16(&v14, v15, v13, v16);
        let v18: i32 = 0i32;
        let v19: i32 = method19(v17.clone(), v18);
        let mut v22: Rc<str> = format_real_9(v19);
        let v23: Rc<str> = string_concat(&["kit agile sprint retro ok since=", &*v0, " done=", &*v22]);
        println!("{}", v23);
        method20(v11, v17);
        0i32
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        retro_since_15(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_sprint_shown(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_agile_sprint_retro(v0: &str, v1: &str) -> i32 {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn rig_agile_sprint_retro_since(v0: &str, v1: &str) -> i32 {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
use agile_task_census::SpiralShape_97df63ebabc3 as Mut0;
