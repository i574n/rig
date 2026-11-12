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
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_LiveGui,
    US2_TestSuite,
    US2_HeavyBuild,
    US2_Compile,
    US2_CompilerInstall,
    US2_TreeRead,
    US2_Bench,
    US2_CompileHold,
    US2_TreeWrite,
}
#[derive(Clone)]
enum US1 {
    US1_ClassKnown(US2),
    US1_ClassUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_KeepWaiting,
    US3_ClearNow,
    US3_GaveUp,
}
#[derive(Clone)]
enum US4 {
    US4_ProcessHolder,
    US4_LeaseHolder,
}
#[derive(Clone)]
enum US5 {
    US5_WindowQueued,
    US5_WindowGranted,
    US5_WindowReleased,
    US5_WindowReleasedWaiting,
    US5_WindowGaveUp,
    US5_WindowPruned,
    US5_WindowDropped,
}
#[derive(Clone)]
enum US6 {
    US6_Quiet,
    US6_Contended(u64),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US7 {
    US7_Some(Rc<str>),
    US7_None,
}
fn concat5_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn stderr_line_4(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn refused_2(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: window "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = concat5_3(v1, v0, v2.clone(), v2.clone(), v2);
    stderr_line_4(v3);
    2i32
}
fn option_value_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1
}
fn method7(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_6(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method7(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US0> = closure1();
    let mut v4: Option<US0> = v2.map(|x| v3(x));
    let v6: US0 = US0::US0_None;
    let v7: US0 = v6.clone();
    let v8: US0 = v4.unwrap_or(v7);
    match v8 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn class_word_10(mut v0: US2) -> Rc<str> {
    match v0 {
        US2::US2_Bench => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US2::US2_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US2::US2_CompileHold => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US2::US2_CompilerInstall => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US2::US2_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US2::US2_LiveGui => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US2::US2_TestSuite => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US2::US2_TreeRead => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US2::US2_TreeWrite => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-write"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    }
}
fn class_of_9(mut v0: Rc<str>) -> US1 {
    let mut v1: US2 = US2::US2_LiveGui;
    let mut v2: Rc<str> = class_word_10(v1);
    let mut v3: bool = &*v0 == &*v2;
    if v3 {
        let mut v4: US2 = US2::US2_LiveGui;
        US1::US1_ClassKnown(v4)
    } else {
        let v6: US2 = US2::US2_TestSuite;
        let v7: Rc<str> = class_word_10(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let v9: US2 = US2::US2_TestSuite;
            US1::US1_ClassKnown(v9)
        } else {
            let v11: US2 = US2::US2_HeavyBuild;
            let mut v12: Rc<str> = class_word_10(v11);
            let mut v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US2 = US2::US2_HeavyBuild;
                US1::US1_ClassKnown(v14)
            } else {
                let mut v16: US2 = US2::US2_Compile;
                let mut v17: Rc<str> = class_word_10(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US2 = US2::US2_Compile;
                    US1::US1_ClassKnown(v19)
                } else {
                    let v21: US2 = US2::US2_CompilerInstall;
                    let v22: Rc<str> = class_word_10(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: US2 = US2::US2_CompilerInstall;
                        US1::US1_ClassKnown(v24)
                    } else {
                        let v26: US2 = US2::US2_TreeRead;
                        let v27: Rc<str> = class_word_10(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let v29: US2 = US2::US2_TreeRead;
                            US1::US1_ClassKnown(v29)
                        } else {
                            let v31: US2 = US2::US2_Bench;
                            let v32: Rc<str> = class_word_10(v31);
                            let v33: bool = &*v0 == &*v32;
                            if v33 {
                                let v34: US2 = US2::US2_Bench;
                                US1::US1_ClassKnown(v34)
                            } else {
                                let v36: US2 = US2::US2_CompileHold;
                                let v37: Rc<str> = class_word_10(v36);
                                let v38: bool = &*v0 == &*v37;
                                if v38 {
                                    let v39: US2 = US2::US2_CompileHold;
                                    US1::US1_ClassKnown(v39)
                                } else {
                                    let v41: US2 = US2::US2_TreeWrite;
                                    let v42: Rc<str> = class_word_10(v41);
                                    let v43: bool = &*v0 == &*v42;
                                    if v43 {
                                        let v44: US2 = US2::US2_TreeWrite;
                                        US1::US1_ClassKnown(v44)
                                    } else {
                                        US1::US1_ClassUnknown
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn loop_8(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return false;
        } else {
            let mut v4: Rc<str> = v0.borrow()[v2 as usize].clone();
            let v5: US1 = class_of_9(v4);
            let v8: bool = match v5 {
                US1::US1_ClassKnown(v6) => {
                    let v6: US2 = v6;
                    false
                }
                US1::US1_ClassUnknown => {
                    true
                }
            };
            if v8 {
                return true;
            } else {
                let v9: i32 = v2.wrapping_add(1i32);
                v2 = v9;
                continue;
            }
        }
    }
}
fn windows_root_12() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_WINDOWS_HOME").or_else(|| std::env::var_os("KIT_WINDOWS_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-windows")).to_string_lossy().as_ref());
    v0
}
fn stored_rows_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1
}
fn now_ms_14() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn first_row_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1
}
fn tab_field_19(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn row_expired_18(mut v0: Rc<str>, mut v1: u64) -> bool {
    let mut v2: i32 = 5i32;
    let mut v3: Rc<str> = tab_field_19(v0, v2);
    let mut v4: u64 = decimal_of_6(v3);
    let v5: bool = v4 < v1;
    v5
}
fn holder_of_21(mut v0: Rc<str>) -> US4 {
    let mut v1: i32 = 3i32;
    let mut v2: Rc<str> = tab_field_19(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(kit job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let v9: bool = if v4 {
        true
    } else {
        let v5: i32 = 0i32;
        let v6: Rc<str> = tab_field_19(v0, v5);
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v6.ends_with(&*v7);
        v8
    };
    if v9 {
        US4::US4_ProcessHolder
    } else {
        US4::US4_LeaseHolder
    }
}
fn window_pid_22(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).split(char::from(45u8)).nth(2).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_23(mut v0: u64) -> bool {
    let mut v1: bool = rig_process_alive::rig_process_alive(v0);
    v1
}
fn orphaned_window_20(mut v0: Rc<str>) -> bool {
    let mut v1: US4 = holder_of_21(v0.clone());
    match v1 {
        US4::US4_LeaseHolder => {
            false
        }
        US4::US4_ProcessHolder => {
            let mut v2: i32 = 0i32;
            let mut v3: Rc<str> = tab_field_19(v0, v2);
            let mut v4: u64 = window_pid_22(v3);
            let v5: bool = process_alive_23(v4);
            let v6: bool = v5 == false;
            v6
        }
    }
}
fn window_ended_17(mut v0: Rc<str>, mut v1: u64) -> bool {
    let mut v2: US4 = holder_of_21(v0.clone());
    match v2 {
        US4::US4_LeaseHolder => {
            row_expired_18(v0, v1)
        }
        US4::US4_ProcessHolder => {
            orphaned_window_20(v0)
        }
    }
}
fn joined_25(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn window_file_24(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = concat5_3(v1, v2, v3.clone(), v3.clone(), v3);
    joined_25(v0, v4)
}
fn file_removed_26(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn method30(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_29(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method30(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn number_text_28(mut v0: u64) -> Rc<str> {
    format_real_29(v0)
}
fn window_event_word_31(mut v0: US5) -> Rc<str> {
    match v0 {
        US5::US5_WindowDropped => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US5::US5_WindowGaveUp => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gave-up"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US5::US5_WindowGranted => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US5::US5_WindowPruned => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pruned"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US5::US5_WindowQueued => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US5::US5_WindowReleased => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US5::US5_WindowReleasedWaiting => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released-waiting"); } LIT.with(|lit| lit.clone()) };
            v4
        }
    }
}
fn event_logged_27(mut v0: Rc<str>, mut v1: US5, mut v2: Rc<str>) -> () {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("events.log"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = joined_25(v0, v3);
    let v5: u64 = now_ms_14();
    let v6: Rc<str> = number_text_28(v5);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = window_event_word_31(v1);
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = concat5_3(v2, v9, v10.clone(), v10.clone(), v10);
    let mut v12: Rc<str> = concat5_3(v6, v7.clone(), v8, v7, v11);
    let mut v13: std::io::Result<std::fs::File> = std::fs::OpenOptions::new().create(true).append(true).open(&*v4);
    let v14: bool = v13.is_ok();
    let v15: bool = v14 == false;
    if v15 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the file could not be opened"); } LIT.with(|lit| lit.clone()) }))
    };
    let mut v16: std::fs::File = v13.unwrap();
    let mut v17: std::io::Result<()> = std::io::Write::write_all(&mut v16, (v12).as_bytes());
    let v18: bool = v17.is_ok();
    let v19: bool = v18 == false;
    if v19 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the write failed"); } LIT.with(|lit| lit.clone()) }))
    }
}
fn other_rows_32(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1
}
fn live_rows_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = &*v1 == "";
        if v4 {
            return v3;
        } else {
            let v5: Rc<str> = first_row_16(v1.clone());
            let v6: bool = window_ended_17(v5.clone(), v2);
            if v6 {
                let v7: i32 = 0i32;
                let v8: Rc<str> = tab_field_19(v5.clone(), v7);
                let v9: Rc<str> = window_file_24(v0.clone(), v8);
                let v10: bool = file_removed_26(v9);
                let v11: US5 = US5::US5_WindowPruned;
                event_logged_27(v0.clone(), v11, v5);
                let mut v12: Rc<str> = other_rows_32(v1);
                v1 = v12;
                continue;
            } else {
                let v14: Rc<str> = other_rows_32(v1);
                let v15: bool = &*v3 == "";
                let v19: Rc<str> = if v15 {
                    v5
                } else {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_3(v3, v16, v5, v17.clone(), v17.clone())
                };
                (v1, v3) = (v14, v19);
                continue;
            }
        }
    }
}
fn method34(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn rows_busy_33(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    loop {
        let mut v2: bool = &*v1 == "";
        if v2 {
            return false;
        } else {
            let mut v3: Rc<str> = first_row_16(v1.clone());
            let mut v4: i32 = 1i32;
            let v5: Rc<str> = tab_field_19(v3, v4);
            let v7: Rc<str> = string_concat(&[",", &*v5, ","]);
            let v9: Rc<str> = string_concat(&[",", &*v0, ","]);
            let v10: Option<usize> = v9.find(&*v7);
            let v11: Option<usize> = method34(v10);
            let mut v12: Rc<dyn Fn((usize)) -> i32> = closure2();
            let mut v13: Option<i32> = v11.map(|x| v12(x));
            let v15: i32 = v13.unwrap_or(-1i32);
            let mut v16: bool = v15 >= 0i32;
            if v16 {
                return true;
            } else {
                let mut v17: Rc<str> = other_rows_32(v1);
                v1 = v17;
                continue;
            }
        }
    }
}
fn waited_11(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64) -> (US3, u64, u64, u64) {
    loop {
        let v7: Rc<str> = windows_root_12();
        let v8: Rc<str> = stored_rows_13(v7.clone());
        let v9: u64 = now_ms_14();
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = live_rows_15(v7, v8, v9, v10);
        let mut v12: bool = rows_busy_33(v0.clone(), v11);
        let mut v13: u64 = v4.wrapping_add(1u64);
        let (mut v16, mut v17, mut v18): (u64, u64, u64) = if v12 {
            let v14: u64 = v5.wrapping_add(1u64);
            (0u64, v13, v14)
        } else {
            let v15: u64 = v3.wrapping_add(1u64);
            (v15, v13, v5)
        };
        let v19: bool = v16 >= v1;
        let v25: US3 = if v19 {
            US3::US3_ClearNow
        } else {
            let v21: bool = v17 >= v2;
            if v21 {
                US3::US3_GaveUp
            } else {
                US3::US3_KeepWaiting
            }
        };
        match &v25 {
            US3::US3_KeepWaiting => {
                std::thread::sleep(std::time::Duration::from_millis(v6));
                (v3, v4, v5) = (v16, v17, v18);
                continue;
            }
            _ => {
                return (v25, v16, v17, v18);
            }
        }
    }
}
fn stdout_line_35(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn window_wait_cleared_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--every"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = option_value_5(v1.clone());
    let mut v3: bool = &*v2 == "";
    let v6: u64 = if v3 {
        30u64
    } else {
        let mut v4: Rc<str> = option_value_5(v1);
        decimal_of_6(v4)
    };
    let v7: u64 = v6.wrapping_mul(1000u64);
    let v8: bool = &*v0 == "";
    let v11: bool = if v8 {
        true
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
        let v10: bool = v0.starts_with(&*v9);
        v10
    };
    let mut v17: bool = if v11 {
        true
    } else {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v12).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v14: i32 = (v13.clone().borrow().len() as i32);
        let v15: i32 = 0i32;
        loop_8(v13, v14, v15)
    };
    if v17 {
        let v18: US2 = US2::US2_LiveGui;
        let v19: Rc<str> = class_word_10(v18);
        let v22: US2 = US2::US2_TestSuite;
        let v23: Rc<str> = class_word_10(v22);
        let v26: US2 = US2::US2_HeavyBuild;
        let v27: Rc<str> = class_word_10(v26);
        let v30: US2 = US2::US2_Compile;
        let v31: Rc<str> = class_word_10(v30);
        let v34: US2 = US2::US2_CompilerInstall;
        let v35: Rc<str> = class_word_10(v34);
        let v38: US2 = US2::US2_TreeRead;
        let v39: Rc<str> = class_word_10(v38);
        let v42: US2 = US2::US2_Bench;
        let v43: Rc<str> = class_word_10(v42);
        let v46: US2 = US2::US2_CompileHold;
        let v47: Rc<str> = class_word_10(v46);
        let v50: US2 = US2::US2_TreeWrite;
        let v51: Rc<str> = class_word_10(v50);
        let v54: Rc<str> = string_concat(&["wait-clear: give known window classes separated by commas, e.g. window wait-clear live-gui,compiler-install --settle 2 --max 120; classes ", &*v19, "|", &*v23, "|", &*v27, "|", &*v31, "|", &*v35, "|", &*v39, "|", &*v43, "|", &*v47, "|", &*v51]);
        refused_2(v54)
    } else {
        let v56: bool = v7 == 0u64;
        if v56 {
            let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-clear: --every needs a positive number of seconds"); } LIT.with(|lit| lit.clone()) };
            refused_2(v57)
        } else {
            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--settle"); } LIT.with(|lit| lit.clone()) };
            let v60: Rc<str> = option_value_5(v59.clone());
            let v61: bool = &*v60 == "";
            let v64: u64 = if v61 {
                2u64
            } else {
                let v62: Rc<str> = option_value_5(v59);
                decimal_of_6(v62)
            };
            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max"); } LIT.with(|lit| lit.clone()) };
            let v66: Rc<str> = option_value_5(v65.clone());
            let v67: bool = &*v66 == "";
            let v70: u64 = if v67 {
                60u64
            } else {
                let v68: Rc<str> = option_value_5(v65);
                decimal_of_6(v68)
            };
            let v71: u64 = v70.wrapping_mul(60000u64);
            let v72: u64 = v71.wrapping_div(v7);
            let v73: u64 = v72.wrapping_add(1u64);
            let v74: u64 = 0u64;
            let v75: u64 = 0u64;
            let v76: u64 = 0u64;
            let (mut v77, mut v78, mut v79, mut v80): (US3, u64, u64, u64) = waited_11(v0.clone(), v64, v73, v74, v75, v76, v7);
            let v83: Rc<str> = number_text_28(v79);
            let v85: Rc<str> = string_concat(&[" classes=", &*v0, " samples=", &*v83, " "]);
            let v86: bool = v80 == 0u64;
            let v89: US6 = if v86 {
                US6::US6_Quiet
            } else {
                US6::US6_Contended(v80)
            };
            let v95: Rc<str> = match v89 {
                US6::US6_Contended(v91) => {
                    let v91: u64 = v91;
                    let v92: Rc<str> = number_text_28(v91);
                    let v93: Rc<str> = string_concat(&["contended busy_samples=", &*v92]);
                    v93
                }
                US6::US6_Quiet => {
                    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet"); } LIT.with(|lit| lit.clone()) };
                    v90
                }
            };
            let v96: Rc<str> = string_concat(&[&*v85, &*v95]);
            match v77 {
                US3::US3_ClearNow => {
                    let v97: Rc<str> = string_concat(&["rig window wait-clear ok", &*v96]);
                    stdout_line_35(v97);
                    0i32
                }
                _ => {
                    let v98: Rc<str> = string_concat(&["rig window wait-clear gave-up", &*v96]);
                    stdout_line_35(v98);
                    1i32
                }
            }
        }
    }
}
fn command_line_argument_count_37() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_38(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_36(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_37();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US7 = if v4 {
        let v5: Rc<str> = command_line_argument_38(v0);
        US7::US7_Some(v5)
    } else {
        US7::US7_None
    };
    match v8 {
        US7::US7_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US7::US7_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let mut v12: Rc<UH0> = arguments_from_36(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn wait_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_36(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            window_wait_cleared_1(v6)
                        }
                        _ => {
                            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            window_wait_cleared_1(v9)
                        }
                    }
                }
                _ => {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    window_wait_cleared_1(v12)
                }
            }
        }
        _ => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            window_wait_cleared_1(v15)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        wait_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_window_wait_run() -> i32 {
    closure0()()
}
use rig_process_alive::SpiralShape_97df63ebabc3 as Mut0;
