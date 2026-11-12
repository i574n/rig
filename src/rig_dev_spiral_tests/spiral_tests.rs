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
    UH0_NoFailures,
    UH0_FailedThen(Rc<str>, i32, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<str>, Rc<UH2>),
}
fn announced_1(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v4: i32 = (v3.clone().borrow().len() as i32);
        let v5: bool = v4 > 0i32;
        let mut v9: bool = if v5 {
            let v6: i32 = v4.wrapping_sub(1i32);
            let v7: Rc<str> = v3.borrow()[v6 as usize].clone();
            let v8: bool = &*v7 == "";
            v8
        } else {
            false
        };
        let v11: i32 = if v9 {
            let v10: i32 = v4.wrapping_sub(1i32);
            v10
        } else {
            v4
        };
        let v12: bool = v1 < v11;
        if v12 {
            let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v14: bool = v1 >= 0i32;
            let v24: bool = if v14 {
                let v15: i32 = (v13.clone().borrow().len() as i32);
                let v16: bool = v15 > 0i32;
                let v20: bool = if v16 {
                    let v17: i32 = v15.wrapping_sub(1i32);
                    let v18: Rc<str> = v13.borrow()[v17 as usize].clone();
                    let v19: bool = &*v18 == "";
                    v19
                } else {
                    false
                };
                let v22: i32 = if v20 {
                    let v21: i32 = v15.wrapping_sub(1i32);
                    v21
                } else {
                    v15
                };
                let v23: bool = v1 < v22;
                v23
            } else {
                false
            };
            let v37: Rc<str> = if v24 {
                let v25: Rc<str> = v13.borrow()[v1 as usize].clone();
                let v26: i32 = (v25.clone().len() as i32);
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v28: bool = v25.ends_with(&*v27);
                let v29: bool = v28 == false;
                if v29 {
                    v25
                } else {
                    let v30: bool = v26 == 1i32;
                    if v30 {
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v31
                    } else {
                        let v32: i32 = v26.wrapping_sub(2i32);
                        let v33: Rc<str> = string_slice(&v25, 0i32 as i64, v32 as i64);
                        v33
                    }
                }
            } else {
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v36
            };
            let v38: i32 = { std::io::Write::write_all(&mut std::io::stdout(),format!("spiral test blocks {}
",&*v37).as_bytes()).unwrap(); 0i32 };
            let v39: i32 = v1.wrapping_add(1i32);
            v1 = v39;
            continue;
        } else {
            return 0i32;
        }
    }
}
fn first_code_4(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_FailedThen(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: Rc<UH0> = v4;
                (v0, v1) = (v4, v3);
                continue;
            }
            UH0::UH0_NoFailures => {
                return v1;
            }
        }
    }
}
fn failure_list_5(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_FailedThen(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: Rc<UH0> = v4;
                let v5: bool = &*v1 == "";
                let v8: Rc<str> = if v5 {
                    v2
                } else {
                    let v7: Rc<str> = string_concat(&[&*v2, ", ", &*v1]);
                    v7
                };
                (v0, v1) = (v4, v8);
                continue;
            }
            UH0::UH0_NoFailures => {
                return v1;
            }
        }
    }
}
fn failure_count_6(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_FailedThen(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: Rc<UH0> = v4;
                let v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4, v5);
                continue;
            }
            UH0::UH0_NoFailures => {
                return v1;
            }
        }
    }
}
fn reported_3(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    match &*v0 {
        UH0::UH0_NoFailures => {
            let mut v2: bool = v1 == 0i32;
            if v2 {
                ()
            } else {
                eprintln!("spiral notebooks: every notebook reported passing, but the batch exited {}", v1);
                ()
            };
            v1
        }
        _ => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v4: Rc<str> = failure_list_5(v0.clone(), v3);
            let v5: i32 = 0i32;
            let v6: i32 = failure_count_6(v0.clone(), v5);
            eprintln!("spiral notebooks failed ({}): {}", v6, &*v4);
            let v7: i32 = 1i32;
            first_code_4(v0, v7)
        }
    }
}
fn run_batch_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> (i32, std::rc::Rc<str>, bool, std::rc::Rc<str>) {
    let v4: (i32, std::rc::Rc<str>, bool, std::rc::Rc<str>) = { let root=std::path::Path::new(&*v0); let target=std::path::Path::new(&*v1); let cells_root=std::env::var_os("RIG_SPIRAL_CELLS").or_else(|| std::env::var_os("KIT_SPIRAL_CELLS")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("RIG_SPIRAL_BUNDLE").or_else(|| std::env::var_os("KIT_SPIRAL_BUNDLE")).map(|bundle| std::path::PathBuf::from(bundle).join("..").join("..").join("cells"))).unwrap_or_else(|| root.join("..").join("spiral").join("apps").join("cells")); let relatives=(&*v2).lines().map(str::to_owned).collect::<Vec<_>>(); let reports_dir=target.join("spiral-tests"); let _=std::fs::create_dir_all(&reports_dir); let kit=target.join("debug").join(format!("kit{}",std::env::consts::EXE_SUFFIX)); let drives_cli=relatives.iter().any(|relative| std::fs::read_to_string(root.join(relative)).map(|text| text.lines().next().unwrap_or("").contains("rig_cli_check")).unwrap_or(false)); if drives_cli { let _=std::process::Command::new("cargo").current_dir(root.join("src")) .args(["build","--locked","-p","rig-cli","--target-dir"]).arg(target).status(); } let port=std::net::TcpListener::bind("127.0.0.1:0").ok().and_then(|listener| listener.local_addr().ok()) .map(|address| address.port() as u32).unwrap_or(20000+std::process::id()%10000); let mut command=std::process::Command::new("pwsh"); command.current_dir(root).arg("-NoProfile").arg("-File").arg(cells_root.join("spi").join("run_notebook.ps1")); for relative in &relatives { command.arg("--path").arg(root.join(relative)); } command.arg("--output-dir").arg(&reports_dir).arg("--no-html").arg("--gate").env("RIG_BIN_UNDER_TEST", &&kit).env("KIT_BIN_UNDER_TEST", &kit).env("SPIRAL_CELLS_WARM","0").env("SPIRAL_CELLS_MEMO","0").env("SPIRAL_CELLS_PER_TEST","0").env_remove("SPIRAL_CELLS_COMPILER").env("SPIRAL_CELLS_COMPILER_PORT",port.to_string()); let bounded=(v3).saturating_mul(relatives.len().max(1) as u64); let stdout_handle=std::io::stdout(); let stderr_handle=std::io::stderr(); let streamed=rig_process_streaming::run_bounded_streaming(&mut command,bounded,None,|kind,bytes| match kind { rig_process_streaming::ProcessStreamKind::Stdout => { let mut out=stdout_handle.lock(); std::io::Write::write_all(&mut out,bytes).and_then(|_| std::io::Write::flush(&mut out)).map_err(|error| error.to_string()) } rig_process_streaming::ProcessStreamKind::Stderr => { let mut err=stderr_handle.lock(); std::io::Write::write_all(&mut err,bytes).and_then(|_| std::io::Write::flush(&mut err)).map_err(|error| error.to_string()) } }); match streamed { Ok(capture) => { let code=if matches!(capture.termination,rig_process_streaming::ProcessTermination::TimedOut) { 124 } else { capture.status.code().unwrap_or(1) }; let quote=char::from(34u8); let failed_key=format!("{quote}failed{quote}:"); let file_key=format!("{quote}file{quote}:{quote}"); let text=String::from_utf8_lossy(&capture.stdout).replace(char::from(92u8),"/").to_lowercase(); let reports=text.lines().filter_map(|line| line.strip_prefix("spiral-tests file ")).map(|json| { let failed=json.split(failed_key.as_str()).nth(1).map(|rest| rest.chars().take_while(|c| c.is_ascii_digit()).collect::<String>()).unwrap_or_default(); let file=json.split(file_key.as_str()).nth(1).and_then(|rest| rest.split(quote).next()).unwrap_or("").to_owned(); (file,failed) }).collect::<Vec<_>>(); let failing=relatives.iter().filter(|relative| { let wanted=relative.replace(char::from(92u8),"/").to_lowercase(); match reports.iter().find(|(file,_)| file.ends_with(&wanted)) { Some((_,failed)) => failed.as_str() != "0", None => true } }).cloned().collect::<Vec<_>>(); let daemon=["never answers the handshake","is already used by another program","compiler connection failed"]; let infra=code != 0 && daemon.iter().any(|marker| text.contains(marker)); (code,std::rc::Rc::<str>::from(failing.join("
")),infra,std::rc::Rc::<str>::from(text.as_str())) } Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); (2,std::rc::Rc::<str>::from(relatives.join("
")),false,{ thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) } } };
    v4
}
fn failure_rows_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from({ let quote = char::from(34u8); let text = &*v1; let mut rows = Vec::new(); let file_key = format!("{quote}file{quote}:{quote}"); let daemon = ["never answers the handshake", "is already used by another program", "compiler connection failed"]; for relative in (&*v0).lines() { let wanted = relative.replace(char::from(92u8), "/").to_lowercase(); let report = text.lines().filter_map(|line| line.strip_prefix("spiral-tests file ")).filter(|json| json.split(file_key.as_str()).nth(1).and_then(|rest| rest.split(quote).next()) .map(|file| file.ends_with(&wanted)).unwrap_or(false)).last(); let Some(json) = report else { rows.push(format!("{wanted}		partial")); continue; }; let number = |key: &str| json.split(format!("{quote}{key}{quote}:").as_str()).nth(1) .map(|rest| rest.chars().take_while(|c| c.is_ascii_digit()).collect::<String>()) .and_then(|digits| digits.parse::<i64>().ok()).unwrap_or(0); if number("executed") < number("discovered") { rows.push(format!("{wanted}		partial")); } let messages = json.split(format!("{quote}failed_messages{quote}:").as_str()).nth(1) .and_then(|rest| rest.split(char::from(125u8)).next()).unwrap_or(""); let names = json.split(format!("{quote}failed_tests{quote}:").as_str()).nth(1) .and_then(|rest| rest.split(char::from(93u8)).next()).unwrap_or(""); let bracket = char::from(91u8); for name in names.split(char::from(44u8)).map(|n| n.trim_matches(|c: char| c == quote || c == bracket)) { if name.is_empty() { continue; } let message = messages.split(format!("{quote}{name}{quote}:{quote}").as_str()).nth(1) .and_then(|rest| rest.split(quote).next()).unwrap_or(""); let kind = if daemon.iter().any(|marker| message.contains(marker)) { "daemon" } else { "real" }; rows.push(format!("{wanted}	{name}	{kind}")); } } rows.join("
") });
    v2
}
fn method10(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method10(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method14(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method14(v3, v1);
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH1::UH1_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method16(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method17(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method17(v7, &v9) {
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
        method16(v16, v17)
    }
}
fn attempt_rows_13(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH1> = split_list_15(v1, v0);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method14(v2, v3)
}
fn method12(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<UH1> = attempt_rows_13(v2);
                let v5: Rc<UH1> = method10(v1, v4);
                (v0, v1) = (v3, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\treal"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18() -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure1()
}
fn method19(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<UH1>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: bool = v0(v2);
                if v4 {
                    return true;
                } else {
                    let v5: Rc<dyn Fn(Rc<str>) -> bool> = method18();
                    (v0, v1) = (v5, v3);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return false;
            }
        }
    }
}
fn closure4(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: Rc<str> = string_concat(&[&*v0, "\t\tpartial"]);
        let v3: bool = &*v1 == &*v2;
        v3
    })
}
fn method25(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure4(v0)
}
fn method26(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: Rc<UH1>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                let v5: bool = v1(v3);
                if v5 {
                    return true;
                } else {
                    let v6: Rc<dyn Fn(Rc<str>) -> bool> = method25(v0.clone());
                    (v1, v2) = (v6, v4);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return false;
            }
        }
    }
}
fn ran_fully_24(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<UH1> = attempt_rows_13(v0);
    let v3: Rc<dyn Fn(Rc<str>) -> bool> = method25(v1.clone());
    let v4: bool = method26(v1, v3, v2);
    let v5: bool = v4 == false;
    v5
}
fn closure5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v2: Rc<str>| -> bool {
        let v5: Rc<str> = string_concat(&[&*v0, "\t", &*v1, "\t"]);
        let v6: bool = v2.starts_with(&*v5);
        v6
    })
}
fn method29(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure5(v0, v1)
}
fn method28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<dyn Fn(Rc<str>) -> bool>, mut v3: Rc<UH1>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let v6: bool = v2(v4);
                if v6 {
                    return true;
                } else {
                    let v7: Rc<dyn Fn(Rc<str>) -> bool> = method29(v0.clone(), v1.clone());
                    (v2, v3) = (v7, v5);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return false;
            }
        }
    }
}
fn failed_in_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let v3: Rc<UH1> = attempt_rows_13(v0);
    let v4: Rc<dyn Fn(Rc<str>) -> bool> = method29(v1.clone(), v2.clone());
    method28(v1, v2, v4, v3)
}
fn closure3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v2: Rc<str>| -> bool {
        let v3: bool = ran_fully_24(v2.clone(), v1.clone());
        if v3 {
            let v4: bool = &*v0 == "";
            if v4 {
                true
            } else {
                let v5: bool = failed_in_27(v2, v1.clone(), v0.clone());
                let v6: bool = v5 == false;
                v6
            }
        } else {
            false
        }
    })
}
fn method23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure3(v0, v1)
}
fn method22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<dyn Fn(Rc<str>) -> bool>, mut v3: Rc<UH1>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let v6: bool = v2(v4);
                if v6 {
                    return true;
                } else {
                    let v7: Rc<dyn Fn(Rc<str>) -> bool> = method23(v0.clone(), v1.clone());
                    (v2, v3) = (v7, v5);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return false;
            }
        }
    }
}
fn cleared_21(mut v0: Rc<UH1>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH1> = split_list_15(v2, v1);
    match Rc::unwrap_or_clone(v3) {
        UH1::UH1_Cons(v4, v5) => {
            let v4: Rc<str> = v4;
            let v5: Rc<UH1> = v5;
            match Rc::unwrap_or_clone(v5) {
                UH1::UH1_Cons(v6, v7) => {
                    let v6: Rc<str> = v6;
                    let v7: Rc<UH1> = v7;
                    let v8: Rc<dyn Fn(Rc<str>) -> bool> = method23(v6.clone(), v4.clone());
                    method22(v6, v4, v8, v0)
                }
                _ => {
                    false
                }
            }
        }
        _ => {
            false
        }
    }
}
fn closure2(mut v0: Rc<UH1>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: bool = cleared_21(v0.clone(), v1);
        let v3: bool = v2 == false;
        v3
    })
}
fn method20(mut v0: Rc<UH1>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure2(v0)
}
fn method30(mut v0: Rc<UH1>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: Rc<UH1>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                let v5: bool = v1(v3);
                if v5 {
                    return true;
                } else {
                    let v6: Rc<dyn Fn(Rc<str>) -> bool> = method20(v0.clone());
                    (v1, v2) = (v6, v4);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return false;
            }
        }
    }
}
fn daemon_only_and_cleared_11(mut v0: Rc<UH1>) -> bool {
    let mut v1: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH1> = method12(v0.clone(), v1);
    let v3: Rc<dyn Fn(Rc<str>) -> bool> = method18();
    let v4: bool = method19(v3, v2.clone());
    let v5: bool = v4 == false;
    let v8: bool = if v5 {
        let v6: bool = match &*v2 {
            UH1::UH1_Nil => {
                true
            }
            _ => {
                false
            }
        };
        let v7: bool = v6 != true;
        v7
    } else {
        false
    };
    if v8 {
        let mut v9: Rc<dyn Fn(Rc<str>) -> bool> = method20(v0.clone());
        let v10: bool = method30(v0, v9, v2);
        let v11: bool = v10 == false;
        v11
    } else {
        false
    }
}
fn chunk_outcome_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: i32, mut v5: Rc<UH1>) -> (i32, std::rc::Rc<str>, bool, std::rc::Rc<str>) {
    loop {
        let v6: (i32, std::rc::Rc<str>, bool, std::rc::Rc<str>) = run_batch_8(v0.clone(), v1.clone(), v2.clone(), v3);
        let v7: Rc<str> = v6.3.clone();
        let v8: Rc<str> = failure_rows_9(v2.clone(), v7);
        let mut v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let v10: Rc<UH1> = Rc::new(UH1::UH1_Cons(v8, v9));
        let v11: Rc<UH1> = method10(v5, v10);
        let v12: i32 = v6.clone().0;
        let v13: bool = v12 == 0i32;
        let v16: bool = if v13 {
            true
        } else {
            let v14: bool = v6.clone().2;
            let v15: bool = v14 == false;
            v15
        };
        if v16 {
            return v6;
        } else {
            let v17: bool = daemon_only_and_cleared_11(v11.clone());
            if v17 {
                eprintln!("spiral notebooks: every daemon-failed test passed in another full attempt");
                let v18: (i32, std::rc::Rc<str>, bool, std::rc::Rc<str>) = (0i32, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, false, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                return v18;
            } else {
                let v19: bool = v4 < 4i32;
                if v19 {
                    eprintln!("spiral notebooks: the compiler daemon did not answer; attempt {}", v4 + 1);
                    let v20: i32 = v4.wrapping_add(1i32);
                    (v4, v5) = (v20, v11);
                    continue;
                } else {
                    return v6;
                }
            }
        }
    }
}
fn failures_of_31(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v6: i32 = (v5.clone().borrow().len() as i32);
        let v7: bool = v6 > 0i32;
        let v11: bool = if v7 {
            let v8: i32 = v6.wrapping_sub(1i32);
            let mut v9: Rc<str> = v5.borrow()[v8 as usize].clone();
            let v10: bool = &*v9 == "";
            v10
        } else {
            false
        };
        let v13: i32 = if v11 {
            let v12: i32 = v6.wrapping_sub(1i32);
            v12
        } else {
            v6
        };
        let v14: bool = v1 < v13;
        if v14 {
            let v15: i32 = v1.wrapping_add(1i32);
            let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v17: bool = v1 >= 0i32;
            let v27: bool = if v17 {
                let v18: i32 = (v16.clone().borrow().len() as i32);
                let v19: bool = v18 > 0i32;
                let v23: bool = if v19 {
                    let v20: i32 = v18.wrapping_sub(1i32);
                    let v21: Rc<str> = v16.borrow()[v20 as usize].clone();
                    let v22: bool = &*v21 == "";
                    v22
                } else {
                    false
                };
                let v25: i32 = if v23 {
                    let v24: i32 = v18.wrapping_sub(1i32);
                    v24
                } else {
                    v18
                };
                let v26: bool = v1 < v25;
                v26
            } else {
                false
            };
            let v40: Rc<str> = if v27 {
                let v28: Rc<str> = v16.borrow()[v1 as usize].clone();
                let v29: i32 = (v28.clone().len() as i32);
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v31: bool = v28.ends_with(&*v30);
                let v32: bool = v31 == false;
                if v32 {
                    v28
                } else {
                    let v33: bool = v29 == 1i32;
                    if v33 {
                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v34
                    } else {
                        let v35: i32 = v29.wrapping_sub(2i32);
                        let v36: Rc<str> = string_slice(&v28, 0i32 as i64, v35 as i64);
                        v36
                    }
                }
            } else {
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v39
            };
            let v41: Rc<UH0> = Rc::new(UH0::UH0_FailedThen(v40, v2, v3));
            (v1, v3) = (v15, v41);
            continue;
        } else {
            return v3;
        }
    }
}
fn batches_run_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH1>, mut v4: Rc<UH0>, mut v5: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH1::UH1_Cons(v7, v8) => {
                let v7: Rc<str> = v7;
                let v8: Rc<UH1> = v8;
                let mut v9: i32 = 1i32;
                let v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v11: (i32, std::rc::Rc<str>, bool, std::rc::Rc<str>) = chunk_outcome_7(v0.clone(), v1.clone(), v7, v2, v9, v10);
                let v12: i32 = v11.clone().0;
                let v13: Rc<str> = v11.1.clone();
                let v14: i32 = 0i32;
                let v15: bool = v12 == 0i32;
                let v16: i32 = if v15 {
                    1i32
                } else {
                    v12
                };
                let v17: Rc<UH0> = failures_of_31(v13, v14, v16, v4);
                let v18: bool = v5 == 0i32;
                let v19: i32 = if v18 {
                    v12
                } else {
                    v5
                };
                (v3, v4, v5) = (v8, v17, v19);
                continue;
            }
            UH1::UH1_Nil => {
                return reported_3(v4, v5);
            }
        }
    }
}
fn selected_notebooks_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH2>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH2::UH2_Cons(v5, v6, v7) => {
                let v5: Rc<str> = v5;
                let v6: Rc<str> = v6;
                let v7: Rc<UH2> = v7;
                let v8: bool = &*v1 == "";
                let v14: bool = if v8 {
                    true
                } else {
                    let v10: Rc<str> = string_concat(&[&*v2, &*v5, &*v2]);
                    let v12: Rc<str> = string_concat(&[&*v2, &*v1, &*v2]);
                    let v13: bool = v12.contains(&*v10);
                    v13
                };
                let v16: bool = if v14 {
                    let v15: bool = std::path::Path::new(&*v0).join(&*v6).is_file();
                    v15
                } else {
                    false
                };
                let v21: Rc<str> = if v16 {
                    let v17: bool = &*v4 == "";
                    if v17 {
                        v6
                    } else {
                        let v19: Rc<str> = string_concat(&[&*v4, "\n", &*v6]);
                        v19
                    }
                } else {
                    v4
                };
                (v3, v4) = (v7, v21);
                continue;
            }
            UH2::UH2_Nil => {
                return v4;
            }
        }
    }
}
fn chunks_from_33(mut v0: i32, mut v1: Rc<UH1>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v8, v9) => {
                let v8: Rc<str> = v8;
                let mut v9: Rc<UH1> = v9;
                let v10: bool = v3 >= v0;
                if v10 {
                    let v11: i32 = 1i32;
                    let v12: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v4));
                    (v1, v2, v3, v4) = (v9, v8, v11, v12);
                    continue;
                } else {
                    let v14: bool = &*v2 == "";
                    let v17: Rc<str> = if v14 {
                        v8
                    } else {
                        let v16: Rc<str> = string_concat(&[&*v2, "\n", &*v8]);
                        v16
                    };
                    let v18: i32 = v3.wrapping_add(1i32);
                    (v1, v2, v3) = (v9, v17, v18);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                let v5: bool = &*v2 == "";
                if v5 {
                    return v4;
                } else {
                    return Rc::new(UH1::UH1_Cons(v2, v4));
                }
            }
        }
    }
}
fn method34(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn run_registered_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64) -> i32 {
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-ci-model"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_ci_model/tools.spi"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-ci-probe"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_ci_probe/probe.spi"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-ci-verdict"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_ci_verdict/verdict.spi"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-ci-parity"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_ci_parity/parity.spi"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-rechain"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_rechain/rechain.spi"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-chain"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_chain/chain.spi"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-jobs-doctor-env"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_jobs_doctor_env/environment.spi"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-adoption-habits"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_adoption_habits/habits.spi"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-log-sections"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_log_sections/section_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_log_sections/section_scan.spi"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-planning-rollup"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_planning_rollup/planning_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-scan"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_model/todo_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-archive-tar-runtime"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_archive_tar_runtime/format.spi"); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-bundle-retention"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_bundle_retention/series.spi"); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_bundle_retention/receipt.spi"); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-host"); } LIT.with(|lit| lit.clone()) };
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_host/host_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-bundle-manifest"); } LIT.with(|lit| lit.clone()) };
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_bundle_manifest/bundle_manifest.spi"); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-verbs"); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_verbs/verbs.spi"); } LIT.with(|lit| lit.clone()) };
    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-machine-crowding"); } LIT.with(|lit| lit.clone()) };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_machine_crowding/crowding.spi"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-repo-affected"); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_repo_affected/graph.spi"); } LIT.with(|lit| lit.clone()) };
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-log-sequences-model"); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_log_sequences/sequence_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-log-sequences"); } LIT.with(|lit| lit.clone()) };
    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_log_sequences/sequences.spi"); } LIT.with(|lit| lit.clone()) };
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-process-ownership"); } LIT.with(|lit| lit.clone()) };
    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_process_table/ownership.spi"); } LIT.with(|lit| lit.clone()) };
    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-table-sequences-cli"); } LIT.with(|lit| lit.clone()) };
    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_table_sequences/sequences_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-mirror-model"); } LIT.with(|lit| lit.clone()) };
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_mirror/mirror.spi"); } LIT.with(|lit| lit.clone()) };
    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-mirror-cli"); } LIT.with(|lit| lit.clone()) };
    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_mirror/mirror_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-env-pins-model"); } LIT.with(|lit| lit.clone()) };
    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_env_pins/pins.spi"); } LIT.with(|lit| lit.clone()) };
    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-env-pins-cli"); } LIT.with(|lit| lit.clone()) };
    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_env_pins/env_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-repo-accept-review"); } LIT.with(|lit| lit.clone()) };
    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_repo_accept_review/review.spi"); } LIT.with(|lit| lit.clone()) };
    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("command-spec-domain"); } LIT.with(|lit| lit.clone()) };
    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/command_spec_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("canonical-plan-ir-domain"); } LIT.with(|lit| lit.clone()) };
    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/canonical_plan_ir_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-views"); } LIT.with(|lit| lit.clone()) };
    let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_views/views_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-edit"); } LIT.with(|lit| lit.clone()) };
    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_edit/edit_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-dev-spiral-tests"); } LIT.with(|lit| lit.clone()) };
    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_dev_spiral_tests/failures.spi"); } LIT.with(|lit| lit.clone()) };
    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_dev_spiral_tests/batches.spi"); } LIT.with(|lit| lit.clone()) };
    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-handoff-render"); } LIT.with(|lit| lit.clone()) };
    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_handoff_render/render.spi"); } LIT.with(|lit| lit.clone()) };
    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("authority-state-domain"); } LIT.with(|lit| lit.clone()) };
    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/authority_state_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-note-model"); } LIT.with(|lit| lit.clone()) };
    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_note_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-machine-watch"); } LIT.with(|lit| lit.clone()) };
    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_machine_watch_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-window-claim"); } LIT.with(|lit| lit.clone()) };
    let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_tree_lease/lease.spi"); } LIT.with(|lit| lit.clone()) };
    let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-adoption"); } LIT.with(|lit| lit.clone()) };
    let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_adoption/classify.spi"); } LIT.with(|lit| lit.clone()) };
    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-process-table"); } LIT.with(|lit| lit.clone()) };
    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_process_table/ps_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-cli"); } LIT.with(|lit| lit.clone()) };
    let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_cli_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_cli_model/argument_file.spi"); } LIT.with(|lit| lit.clone()) };
    let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-entry"); } LIT.with(|lit| lit.clone()) };
    let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_jobs/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_os_process/process.spi"); } LIT.with(|lit| lit.clone()) };
    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-process-alive"); } LIT.with(|lit| lit.clone()) };
    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_process_alive/alive.spi"); } LIT.with(|lit| lit.clone()) };
    let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-repo-state"); } LIT.with(|lit| lit.clone()) };
    let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_repo_state/state.spi"); } LIT.with(|lit| lit.clone()) };
    let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-table-query"); } LIT.with(|lit| lit.clone()) };
    let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_table_query/query.spi"); } LIT.with(|lit| lit.clone()) };
    let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-lcov"); } LIT.with(|lit| lit.clone()) };
    let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lcov/lcov.spi"); } LIT.with(|lit| lit.clone()) };
    let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_bundle_retention/receipt_read.spi"); } LIT.with(|lit| lit.clone()) };
    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-read-query"); } LIT.with(|lit| lit.clone()) };
    let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_read_query/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_receipt/receipt.spi"); } LIT.with(|lit| lit.clone()) };
    let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-program"); } LIT.with(|lit| lit.clone()) };
    let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_program/program.spi"); } LIT.with(|lit| lit.clone()) };
    let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-read"); } LIT.with(|lit| lit.clone()) };
    let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_read/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_process_table/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-facts"); } LIT.with(|lit| lit.clone()) };
    let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_facts/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-command-line"); } LIT.with(|lit| lit.clone()) };
    let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_command_line/launch.spi"); } LIT.with(|lit| lit.clone()) };
    let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-process"); } LIT.with(|lit| lit.clone()) };
    let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_process/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-tables"); } LIT.with(|lit| lit.clone()) };
    let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_tables/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-write"); } LIT.with(|lit| lit.clone()) };
    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_write/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-grep-query"); } LIT.with(|lit| lit.clone()) };
    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_grep_query/grep.spi"); } LIT.with(|lit| lit.clone()) };
    let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-lint-args"); } LIT.with(|lit| lit.clone()) };
    let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lint_args/args.spi"); } LIT.with(|lit| lit.clone()) };
    let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-experiments"); } LIT.with(|lit| lit.clone()) };
    let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_experiments/experiments.spi"); } LIT.with(|lit| lit.clone()) };
    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-file-steps"); } LIT.with(|lit| lit.clone()) };
    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_file_steps/steps.spi"); } LIT.with(|lit| lit.clone()) };
    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-claim-lease"); } LIT.with(|lit| lit.clone()) };
    let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_claim_lease/lease.spi"); } LIT.with(|lit| lit.clone()) };
    let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-git"); } LIT.with(|lit| lit.clone()) };
    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_git/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-jobs"); } LIT.with(|lit| lit.clone()) };
    let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_jobs/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-lab"); } LIT.with(|lit| lit.clone()) };
    let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_lab/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/workspace_primitives/toolchain.spi"); } LIT.with(|lit| lit.clone()) };
    let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-contracts"); } LIT.with(|lit| lit.clone()) };
    let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_notes/notes_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_contracts/receipts_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-contracts"); } LIT.with(|lit| lit.clone()) };
    let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_contracts/agile_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-archive-contracts"); } LIT.with(|lit| lit.clone()) };
    let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_archive_contracts/archive_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-release-contracts"); } LIT.with(|lit| lit.clone()) };
    let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_release_contracts/upgrade_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_cli/windows_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-coverage-prune"); } LIT.with(|lit| lit.clone()) };
    let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/coverage_prune_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_coverage_prune/coverage_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_coverage_prune/evidence_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_release_contracts/hygiene_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-proxy-search"); } LIT.with(|lit| lit.clone()) };
    let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_proxy_search/hash_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_jobs/stop_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-query"); } LIT.with(|lit| lit.clone()) };
    let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_query/query.spi"); } LIT.with(|lit| lit.clone()) };
    let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-next"); } LIT.with(|lit| lit.clone()) };
    let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_next/ranking.spi"); } LIT.with(|lit| lit.clone()) };
    let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_next/next_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_model/roots.spi"); } LIT.with(|lit| lit.clone()) };
    let v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-roots"); } LIT.with(|lit| lit.clone()) };
    let v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_roots/roots_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_table_query/stats.spi"); } LIT.with(|lit| lit.clone()) };
    let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_table_query/table_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-record"); } LIT.with(|lit| lit.clone()) };
    let v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_record/ledger.spi"); } LIT.with(|lit| lit.clone()) };
    let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_record/record_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-standards"); } LIT.with(|lit| lit.clone()) };
    let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_standards/rules.spi"); } LIT.with(|lit| lit.clone()) };
    let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-style-probes"); } LIT.with(|lit| lit.clone()) };
    let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_style_probes/probes.spi"); } LIT.with(|lit| lit.clone()) };
    let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-style-baseline"); } LIT.with(|lit| lit.clone()) };
    let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_style_baseline/baseline.spi"); } LIT.with(|lit| lit.clone()) };
    let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-style-measure"); } LIT.with(|lit| lit.clone()) };
    let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_style_measure/measure.spi"); } LIT.with(|lit| lit.clone()) };
    let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-grep-found"); } LIT.with(|lit| lit.clone()) };
    let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_grep_found/found.spi"); } LIT.with(|lit| lit.clone()) };
    let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-gauge-xsd"); } LIT.with(|lit| lit.clone()) };
    let v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_gauge_xsd/gauge.spi"); } LIT.with(|lit| lit.clone()) };
    let v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-lint-growth"); } LIT.with(|lit| lit.clone()) };
    let v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lint_growth/growth.spi"); } LIT.with(|lit| lit.clone()) };
    let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-lint-cli"); } LIT.with(|lit| lit.clone()) };
    let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lint_cli/cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-claim-request"); } LIT.with(|lit| lit.clone()) };
    let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_claim_request/request.spi"); } LIT.with(|lit| lit.clone()) };
    let v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-claim-cli"); } LIT.with(|lit| lit.clone()) };
    let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_claim_cli/cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_claim_model/claim.spi"); } LIT.with(|lit| lit.clone()) };
    let v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-quality-lint"); } LIT.with(|lit| lit.clone()) };
    let v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_quality_lint/lint.spi"); } LIT.with(|lit| lit.clone()) };
    let v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-quality-facts"); } LIT.with(|lit| lit.clone()) };
    let v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_quality_facts/facts.spi"); } LIT.with(|lit| lit.clone()) };
    let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-quality-state"); } LIT.with(|lit| lit.clone()) };
    let v195: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_quality_state/state_rows.spi"); } LIT.with(|lit| lit.clone()) };
    let v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-title-tails"); } LIT.with(|lit| lit.clone()) };
    let v197: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_title_tails/title_tails.spi"); } LIT.with(|lit| lit.clone()) };
    let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-asks"); } LIT.with(|lit| lit.clone()) };
    let v199: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_asks/asks_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v200: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-lock"); } LIT.with(|lit| lit.clone()) };
    let v201: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_lock/lock.spi"); } LIT.with(|lit| lit.clone()) };
    let v202: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-changes"); } LIT.with(|lit| lit.clone()) };
    let v203: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_changes/changes.spi"); } LIT.with(|lit| lit.clone()) };
    let v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-verdict"); } LIT.with(|lit| lit.clone()) };
    let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_verdict/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-waits"); } LIT.with(|lit| lit.clone()) };
    let v207: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_waits/waits.spi"); } LIT.with(|lit| lit.clone()) };
    let v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_waits/verb.spi"); } LIT.with(|lit| lit.clone()) };
    let v209: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-gap"); } LIT.with(|lit| lit.clone()) };
    let v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_gap/gap.spi"); } LIT.with(|lit| lit.clone()) };
    let v211: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_dev_spiral_tests/coverage.spi"); } LIT.with(|lit| lit.clone()) };
    let v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_degrade/degrade.spi"); } LIT.with(|lit| lit.clone()) };
    let v213: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_delegate/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-delete-guard"); } LIT.with(|lit| lit.clone()) };
    let v215: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_delete_guard/delete_guard.spi"); } LIT.with(|lit| lit.clone()) };
    let v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_generated_profile/profile.spi"); } LIT.with(|lit| lit.clone()) };
    let v217: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_experiments/experiment_gates.spi"); } LIT.with(|lit| lit.clone()) };
    let v218: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-lint-opens"); } LIT.with(|lit| lit.clone()) };
    let v219: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lint_opens/opens.spi"); } LIT.with(|lit| lit.clone()) };
    let v220: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_memory_adapters/adapter.spi"); } LIT.with(|lit| lit.clone()) };
    let v221: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_quiet_forecast/forecast.spi"); } LIT.with(|lit| lit.clone()) };
    let v222: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-sprint-append"); } LIT.with(|lit| lit.clone()) };
    let v223: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_sprint_rows/rows.spi"); } LIT.with(|lit| lit.clone()) };
    let v224: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-sprint"); } LIT.with(|lit| lit.clone()) };
    let v225: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_sprint/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v226: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-state-access"); } LIT.with(|lit| lit.clone()) };
    let v227: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_state_access/access.spi"); } LIT.with(|lit| lit.clone()) };
    let v228: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-backlog-scaffold"); } LIT.with(|lit| lit.clone()) };
    let v229: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_state_fs/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_state_fs/project.spi"); } LIT.with(|lit| lit.clone()) };
    let v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_state_spelling/spelling.spi"); } LIT.with(|lit| lit.clone()) };
    let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_subcommand_domain/subcommand.spi"); } LIT.with(|lit| lit.clone()) };
    let v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-annotation"); } LIT.with(|lit| lit.clone()) };
    let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_annotation/annotation.spi"); } LIT.with(|lit| lit.clone()) };
    let v235: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_asks/asks.spi"); } LIT.with(|lit| lit.clone()) };
    let v236: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/k3s_manifest/manifest.spi"); } LIT.with(|lit| lit.clone()) };
    let v237: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_shards/shards.spi"); } LIT.with(|lit| lit.clone()) };
    let v238: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-sprint-show"); } LIT.with(|lit| lit.clone()) };
    let v239: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_sprint_show/show.spi"); } LIT.with(|lit| lit.clone()) };
    let v240: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_todo_rows/rows.spi"); } LIT.with(|lit| lit.clone()) };
    let v241: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_authority_audit/audit.spi"); } LIT.with(|lit| lit.clone()) };
    let v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_coordinator_table/table.spi"); } LIT.with(|lit| lit.clone()) };
    let v243: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-crate-largest"); } LIT.with(|lit| lit.clone()) };
    let v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_crate_largest/largest.spi"); } LIT.with(|lit| lit.clone()) };
    let v245: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-crate-walk"); } LIT.with(|lit| lit.clone()) };
    let v246: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_crate_walk/walk.spi"); } LIT.with(|lit| lit.clone()) };
    let v247: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_delete_plan/plan.spi"); } LIT.with(|lit| lit.clone()) };
    let v248: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-mirror-local"); } LIT.with(|lit| lit.clone()) };
    let v249: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_mirror_local/local.spi"); } LIT.with(|lit| lit.clone()) };
    let v250: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-proof-scope"); } LIT.with(|lit| lit.clone()) };
    let v251: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_proof_scope/scope_run.spi"); } LIT.with(|lit| lit.clone()) };
    let v252: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_verify/verify.spi"); } LIT.with(|lit| lit.clone()) };
    let v253: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_decide/decide.spi"); } LIT.with(|lit| lit.clone()) };
    let v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_duties/duties.spi"); } LIT.with(|lit| lit.clone()) };
    let v255: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_lease_time/lease.spi"); } LIT.with(|lit| lit.clone()) };
    let v256: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_requests/requests.spi"); } LIT.with(|lit| lit.clone()) };
    let v257: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_backlog_slice/slice.spi"); } LIT.with(|lit| lit.clone()) };
    let v258: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_bindings_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v259: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_boundary_census/census.spi"); } LIT.with(|lit| lit.clone()) };
    let v260: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_code_dup/dup.spi"); } LIT.with(|lit| lit.clone()) };
    let v261: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_code_index/index.spi"); } LIT.with(|lit| lit.clone()) };
    let v262: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_config_baseline/baseline.spi"); } LIT.with(|lit| lit.clone()) };
    let v263: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_convo_replay/replay.spi"); } LIT.with(|lit| lit.clone()) };
    let v264: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_coordinator_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v265: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_coordinator_placement/placement.spi"); } LIT.with(|lit| lit.clone()) };
    let v266: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_coordinator_table/decision.spi"); } LIT.with(|lit| lit.clone()) };
    let v267: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_coordinator_table/dispatch.spi"); } LIT.with(|lit| lit.clone()) };
    let v268: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_dir_mirror/mirror.spi"); } LIT.with(|lit| lit.clone()) };
    let v269: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-flag-words"); } LIT.with(|lit| lit.clone()) };
    let v270: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_flag_words/words.spi"); } LIT.with(|lit| lit.clone()) };
    let v271: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_gauge_model/gauge.spi"); } LIT.with(|lit| lit.clone()) };
    let v272: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_heal_reap/reap.spi"); } LIT.with(|lit| lit.clone()) };
    let v273: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_heal_tunnel/tunnel.spi"); } LIT.with(|lit| lit.clone()) };
    let v274: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-hook-rules"); } LIT.with(|lit| lit.clone()) };
    let v275: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_hook_rules/rules.spi"); } LIT.with(|lit| lit.clone()) };
    let v276: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-hook"); } LIT.with(|lit| lit.clone()) };
    let v277: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_hook/hook.spi"); } LIT.with(|lit| lit.clone()) };
    let v278: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-bin"); } LIT.with(|lit| lit.clone()) };
    let v279: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_bin/bin.spi"); } LIT.with(|lit| lit.clone()) };
    let v280: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-control"); } LIT.with(|lit| lit.clone()) };
    let v281: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_control/control.spi"); } LIT.with(|lit| lit.clone()) };
    let v282: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-coordinator-args"); } LIT.with(|lit| lit.clone()) };
    let v283: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_coordinator_args/args.spi"); } LIT.with(|lit| lit.clone()) };
    let v284: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-coordinator"); } LIT.with(|lit| lit.clone()) };
    let v285: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_coordinator/coordinator.spi"); } LIT.with(|lit| lit.clone()) };
    let v286: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-dispatch"); } LIT.with(|lit| lit.clone()) };
    let v287: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_dispatch/dispatch.spi"); } LIT.with(|lit| lit.clone()) };
    let v288: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_escaped/escaped.spi"); } LIT.with(|lit| lit.clone()) };
    let v289: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_fan/fan.spi"); } LIT.with(|lit| lit.clone()) };
    let v290: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_host/queue.spi"); } LIT.with(|lit| lit.clone()) };
    let v291: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-create"); } LIT.with(|lit| lit.clone()) };
    let v292: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_launch_model/shapes.spi"); } LIT.with(|lit| lit.clone()) };
    let v293: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_liveness/liveness.spi"); } LIT.with(|lit| lit.clone()) };
    let v294: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-names"); } LIT.with(|lit| lit.clone()) };
    let v295: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_names/names.spi"); } LIT.with(|lit| lit.clone()) };
    let v296: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-peers"); } LIT.with(|lit| lit.clone()) };
    let v297: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_peers/peers.spi"); } LIT.with(|lit| lit.clone()) };
    let v298: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_placement/placement.spi"); } LIT.with(|lit| lit.clone()) };
    let v299: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-remote-gate"); } LIT.with(|lit| lit.clone()) };
    let v300: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_remote_gate/gate.spi"); } LIT.with(|lit| lit.clone()) };
    let v301: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_settle/settle.spi"); } LIT.with(|lit| lit.clone()) };
    let v302: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-start-args"); } LIT.with(|lit| lit.clone()) };
    let v303: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_start_args/start_args.spi"); } LIT.with(|lit| lit.clone()) };
    let v304: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lane_work/rows.spi"); } LIT.with(|lit| lit.clone()) };
    let v305: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lane_work/work.spi"); } LIT.with(|lit| lit.clone()) };
    let v306: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lint_catalogue/catalogue.spi"); } LIT.with(|lit| lit.clone()) };
    let v307: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-log-counts"); } LIT.with(|lit| lit.clone()) };
    let v308: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_log_counts/count_scan.spi"); } LIT.with(|lit| lit.clone()) };
    let v309: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_memory_check/check.spi"); } LIT.with(|lit| lit.clone()) };
    let v310: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-mirror-remote"); } LIT.with(|lit| lit.clone()) };
    let v311: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_mirror_remote/remote_run.spi"); } LIT.with(|lit| lit.clone()) };
    let v312: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-mirror-sync"); } LIT.with(|lit| lit.clone()) };
    let v313: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_mirror_sync/sync.spi"); } LIT.with(|lit| lit.clone()) };
    let v314: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-mirror-unpack"); } LIT.with(|lit| lit.clone()) };
    let v315: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_mirror_unpack/unpack.spi"); } LIT.with(|lit| lit.clone()) };
    let v316: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_modeling_topology/topology.spi"); } LIT.with(|lit| lit.clone()) };
    let v317: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-nameof-findings"); } LIT.with(|lit| lit.clone()) };
    let v318: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_nameof_findings/findings.spi"); } LIT.with(|lit| lit.clone()) };
    let v319: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_net_crossing/crossing.spi"); } LIT.with(|lit| lit.clone()) };
    let v320: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-net-fleet-file"); } LIT.with(|lit| lit.clone()) };
    let v321: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_net_fleet_file/fleet_file.spi"); } LIT.with(|lit| lit.clone()) };
    let v322: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-net-fleet-rules"); } LIT.with(|lit| lit.clone()) };
    let v323: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_net_fleet_rules/rules.spi"); } LIT.with(|lit| lit.clone()) };
    let v324: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-net-topology"); } LIT.with(|lit| lit.clone()) };
    let v325: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_net_topology/topology.spi"); } LIT.with(|lit| lit.clone()) };
    let v326: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_new_harvest/patterns.spi"); } LIT.with(|lit| lit.clone()) };
    let v327: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-notebook-progress"); } LIT.with(|lit| lit.clone()) };
    let v328: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_notebook_progress/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v329: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-os-launch"); } LIT.with(|lit| lit.clone()) };
    let v330: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_os_launch/owner.spi"); } LIT.with(|lit| lit.clone()) };
    let v331: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_pack_ship_model/ship.spi"); } LIT.with(|lit| lit.clone()) };
    let v332: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-path-first"); } LIT.with(|lit| lit.clone()) };
    let v333: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_path_first/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v334: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-proof-scope-manifests"); } LIT.with(|lit| lit.clone()) };
    let v335: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_proof_scope_manifests/manifest_walk.spi"); } LIT.with(|lit| lit.clone()) };
    let v336: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_proof_scope_manifests/manifest.spi"); } LIT.with(|lit| lit.clone()) };
    let v337: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-proof-scope-toml"); } LIT.with(|lit| lit.clone()) };
    let v338: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_proof_scope_toml/toml.spi"); } LIT.with(|lit| lit.clone()) };
    let v339: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-rec-cycle"); } LIT.with(|lit| lit.clone()) };
    let v340: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_rec_cycle/rec_cycle.spi"); } LIT.with(|lit| lit.clone()) };
    let v341: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-remote-activity"); } LIT.with(|lit| lit.clone()) };
    let v342: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_remote_activity/activity.spi"); } LIT.with(|lit| lit.clone()) };
    let v343: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-remote-status"); } LIT.with(|lit| lit.clone()) };
    let v344: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_remote_status/status.spi"); } LIT.with(|lit| lit.clone()) };
    let v345: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-run-build"); } LIT.with(|lit| lit.clone()) };
    let v346: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_run_build/builder.spi"); } LIT.with(|lit| lit.clone()) };
    let v347: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-run-cli"); } LIT.with(|lit| lit.clone()) };
    let v348: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_run_cli/run.spi"); } LIT.with(|lit| lit.clone()) };
    let v349: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_run_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v350: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-schedule"); } LIT.with(|lit| lit.clone()) };
    let v351: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_schedule/schedule.spi"); } LIT.with(|lit| lit.clone()) };
    let v352: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-scratch"); } LIT.with(|lit| lit.clone()) };
    let v353: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_scratch_cited/cited.spi"); } LIT.with(|lit| lit.clone()) };
    let v354: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_scratch_place/place.spi"); } LIT.with(|lit| lit.clone()) };
    let v355: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_self_cost/cost.spi"); } LIT.with(|lit| lit.clone()) };
    let v356: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_spec_cases/cells.spi"); } LIT.with(|lit| lit.clone()) };
    let v357: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_spec_cases/extract.spi"); } LIT.with(|lit| lit.clone()) };
    let v358: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-standards-layers-model"); } LIT.with(|lit| lit.clone()) };
    let v359: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_standards_layers_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v360: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_standards_md/width.spi"); } LIT.with(|lit| lit.clone()) };
    let v361: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-standards-prim-bindings"); } LIT.with(|lit| lit.clone()) };
    let v362: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_standards_prim_bindings/bindings.spi"); } LIT.with(|lit| lit.clone()) };
    let v363: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_standards_repo/repo_entry.spi"); } LIT.with(|lit| lit.clone()) };
    let v364: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_standards_repo/repo_io.spi"); } LIT.with(|lit| lit.clone()) };
    let v365: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-standards-targets"); } LIT.with(|lit| lit.clone()) };
    let v366: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_standards_targets/targets.spi"); } LIT.with(|lit| lit.clone()) };
    let v367: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-standards-toml"); } LIT.with(|lit| lit.clone()) };
    let v368: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_standards_toml/toml.spi"); } LIT.with(|lit| lit.clone()) };
    let v369: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-status-facts"); } LIT.with(|lit| lit.clone()) };
    let v370: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_status_facts/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v371: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_store_key/key.spi"); } LIT.with(|lit| lit.clone()) };
    let v372: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-style-census"); } LIT.with(|lit| lit.clone()) };
    let v373: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_style_census/census.spi"); } LIT.with(|lit| lit.clone()) };
    let v374: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-style-scoreboard"); } LIT.with(|lit| lit.clone()) };
    let v375: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_style_scoreboard/scoreboard.spi"); } LIT.with(|lit| lit.clone()) };
    let v376: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-table-append"); } LIT.with(|lit| lit.clone()) };
    let v377: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_table_append/append.spi"); } LIT.with(|lit| lit.clone()) };
    let v378: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_table_jsonl/framing.spi"); } LIT.with(|lit| lit.clone()) };
    let v379: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-table-logs-args"); } LIT.with(|lit| lit.clone()) };
    let v380: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_table_logs_args/args.spi"); } LIT.with(|lit| lit.clone()) };
    let v381: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_table_query/wheres.spi"); } LIT.with(|lit| lit.clone()) };
    let v382: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_table_rows/rows.spi"); } LIT.with(|lit| lit.clone()) };
    let v383: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_test_placement/placement.spi"); } LIT.with(|lit| lit.clone()) };
    let v384: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_tfs_queue/census.spi"); } LIT.with(|lit| lit.clone()) };
    let v385: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_tfs_queue/queue.spi"); } LIT.with(|lit| lit.clone()) };
    let v386: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_timeline/timeline.spi"); } LIT.with(|lit| lit.clone()) };
    let v387: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-tracked-text"); } LIT.with(|lit| lit.clone()) };
    let v388: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_tracked_text/text.spi"); } LIT.with(|lit| lit.clone()) };
    let v389: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_txn_saga/saga.spi"); } LIT.with(|lit| lit.clone()) };
    let v390: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-units-edges"); } LIT.with(|lit| lit.clone()) };
    let v391: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_units_edges/edges.spi"); } LIT.with(|lit| lit.clone()) };
    let v392: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-units-layout"); } LIT.with(|lit| lit.clone()) };
    let v393: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_units_layout/layout.spi"); } LIT.with(|lit| lit.clone()) };
    let v394: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-units-manifest"); } LIT.with(|lit| lit.clone()) };
    let v395: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_units_manifest/manifest.spi"); } LIT.with(|lit| lit.clone()) };
    let v396: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-units"); } LIT.with(|lit| lit.clone()) };
    let v397: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_units_model/shape.spi"); } LIT.with(|lit| lit.clone()) };
    let v398: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_units/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v399: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-window-announce-check"); } LIT.with(|lit| lit.clone()) };
    let v400: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_window_announce_check/check.spi"); } LIT.with(|lit| lit.clone()) };
    let v401: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_window_announce_rows/rows.spi"); } LIT.with(|lit| lit.clone()) };
    let v402: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-window-announce-verb"); } LIT.with(|lit| lit.clone()) };
    let v403: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_window_announce_verb/verb.spi"); } LIT.with(|lit| lit.clone()) };
    let v404: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_window_announce/announce.spi"); } LIT.with(|lit| lit.clone()) };
    let v405: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-window-wait"); } LIT.with(|lit| lit.clone()) };
    let v406: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_window_wait/settle.spi"); } LIT.with(|lit| lit.clone()) };
    let v407: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-wrapper-score-model"); } LIT.with(|lit| lit.clone()) };
    let v408: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_wrapper_score_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v409: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("semantic-evidence-domain"); } LIT.with(|lit| lit.clone()) };
    let v410: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/semantic_evidence_domain/main.spi"); } LIT.with(|lit| lit.clone()) };
    let v411: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state-receipt-parser-domain"); } LIT.with(|lit| lit.clone()) };
    let v412: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/state_receipt_parser_domain/main.spi"); } LIT.with(|lit| lit.clone()) };
    let v413: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state-receipt-serializer-domain"); } LIT.with(|lit| lit.clone()) };
    let v414: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/state_receipt_serializer_domain/main.spi"); } LIT.with(|lit| lit.clone()) };
    let v415: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state-receipt-tuple-domain"); } LIT.with(|lit| lit.clone()) };
    let v416: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/state_receipt_tuple_domain/main.spi"); } LIT.with(|lit| lit.clone()) };
    let v417: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/workspace_primitives/files.spi"); } LIT.with(|lit| lit.clone()) };
    let v418: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-activity-migration"); } LIT.with(|lit| lit.clone()) };
    let v419: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_activity_rows/rows.spi"); } LIT.with(|lit| lit.clone()) };
    let v420: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_activity/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v421: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_backlog_scaffold/dialect.spi"); } LIT.with(|lit| lit.clone()) };
    let v422: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-cadence-activity"); } LIT.with(|lit| lit.clone()) };
    let v423: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_cadence/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v424: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-claim-item"); } LIT.with(|lit| lit.clone()) };
    let v425: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_claim_item/item.spi"); } LIT.with(|lit| lit.clone()) };
    let v426: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-claim-listing"); } LIT.with(|lit| lit.clone()) };
    let v427: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_claim_listing/listing.spi"); } LIT.with(|lit| lit.clone()) };
    let v428: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-claim-queue"); } LIT.with(|lit| lit.clone()) };
    let v429: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_claim_queue/queue.spi"); } LIT.with(|lit| lit.clone()) };
    let v430: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-claim-rank"); } LIT.with(|lit| lit.clone()) };
    let v431: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_claim_rank/rank.spi"); } LIT.with(|lit| lit.clone()) };
    let v432: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-claim-release"); } LIT.with(|lit| lit.clone()) };
    let v433: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_claim_release/release.spi"); } LIT.with(|lit| lit.clone()) };
    let v434: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_epic_tree/tree.spi"); } LIT.with(|lit| lit.clone()) };
    let v435: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-next-print"); } LIT.with(|lit| lit.clone()) };
    let v436: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_next_print/next_print.spi"); } LIT.with(|lit| lit.clone()) };
    let v437: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-note-edit-args"); } LIT.with(|lit| lit.clone()) };
    let v438: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_note_edit_args/args.spi"); } LIT.with(|lit| lit.clone()) };
    let v439: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-note-events"); } LIT.with(|lit| lit.clone()) };
    let v440: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_note_events/events.spi"); } LIT.with(|lit| lit.clone()) };
    let v441: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-note-listing"); } LIT.with(|lit| lit.clone()) };
    let v442: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_note_listing/note_filter.spi"); } LIT.with(|lit| lit.clone()) };
    let v443: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-note-seal"); } LIT.with(|lit| lit.clone()) };
    let v444: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_note_seal/seal.spi"); } LIT.with(|lit| lit.clone()) };
    let v445: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-note-segments"); } LIT.with(|lit| lit.clone()) };
    let v446: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_note_segments/segments.spi"); } LIT.with(|lit| lit.clone()) };
    let v447: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-planning-gaps"); } LIT.with(|lit| lit.clone()) };
    let v448: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_planning_gaps/gaps.spi"); } LIT.with(|lit| lit.clone()) };
    let v449: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-planning-tally"); } LIT.with(|lit| lit.clone()) };
    let v450: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_planning_tally/planning_tally.spi"); } LIT.with(|lit| lit.clone()) };
    let v451: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-rename-rewrite"); } LIT.with(|lit| lit.clone()) };
    let v452: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_rename_rewrite/rename_rewrite.spi"); } LIT.with(|lit| lit.clone()) };
    let v453: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_report_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v454: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-census"); } LIT.with(|lit| lit.clone()) };
    let v455: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_census/task_census.spi"); } LIT.with(|lit| lit.clone()) };
    let v456: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-update-receipt"); } LIT.with(|lit| lit.clone()) };
    let v457: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_update_receipt/update_receipt.spi"); } LIT.with(|lit| lit.clone()) };
    let v458: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-waits-hold"); } LIT.with(|lit| lit.clone()) };
    let v459: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_waits_hold/hold.spi"); } LIT.with(|lit| lit.clone()) };
    let v460: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-rename"); } LIT.with(|lit| lit.clone()) };
    let v461: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_side_rename/side.spi"); } LIT.with(|lit| lit.clone()) };
    let v462: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-sprint-state"); } LIT.with(|lit| lit.clone()) };
    let v463: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_sprint_state/state.spi"); } LIT.with(|lit| lit.clone()) };
    let v464: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_sprint/sprint.spi"); } LIT.with(|lit| lit.clone()) };
    let v465: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_bench_model/bench.spi"); } LIT.with(|lit| lit.clone()) };
    let v466: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_algebra/external.spi"); } LIT.with(|lit| lit.clone()) };
    let v467: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_algebra/read.spi"); } LIT.with(|lit| lit.clone()) };
    let v468: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_algebra/write.spi"); } LIT.with(|lit| lit.clone()) };
    let v469: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-commit"); } LIT.with(|lit| lit.clone()) };
    let v470: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_commit/commit.spi"); } LIT.with(|lit| lit.clone()) };
    let v471: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-facts-receipt"); } LIT.with(|lit| lit.clone()) };
    let v472: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_facts_receipt/receipt.spi"); } LIT.with(|lit| lit.clone()) };
    let v473: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_move_model/move.spi"); } LIT.with(|lit| lit.clone()) };
    let v474: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-replacement"); } LIT.with(|lit| lit.clone()) };
    let v475: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_replacement/replacement.spi"); } LIT.with(|lit| lit.clone()) };
    let v476: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-slop"); } LIT.with(|lit| lit.clone()) };
    let v477: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_slop/slop.spi"); } LIT.with(|lit| lit.clone()) };
    let v478: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_drafts_drift/drift.spi"); } LIT.with(|lit| lit.clone()) };
    let v479: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-drafts-list"); } LIT.with(|lit| lit.clone()) };
    let v480: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_drafts_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v481: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-evidence"); } LIT.with(|lit| lit.clone()) };
    let v482: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_evidence/evidence.spi"); } LIT.with(|lit| lit.clone()) };
    let v483: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-exec-cells"); } LIT.with(|lit| lit.clone()) };
    let v484: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_exec_cells/cells.spi"); } LIT.with(|lit| lit.clone()) };
    let v485: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-exec-export"); } LIT.with(|lit| lit.clone()) };
    let v486: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_exec_export/export.spi"); } LIT.with(|lit| lit.clone()) };
    let v487: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-exec-jobs"); } LIT.with(|lit| lit.clone()) };
    let v488: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_exec_jobs/jobs.spi"); } LIT.with(|lit| lit.clone()) };
    let v489: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_exec_metrics/metrics.spi"); } LIT.with(|lit| lit.clone()) };
    let v490: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-experiment-args"); } LIT.with(|lit| lit.clone()) };
    let v491: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_experiment_args/args.spi"); } LIT.with(|lit| lit.clone()) };
    let v492: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_experiment_ledger/ledger.spi"); } LIT.with(|lit| lit.clone()) };
    let v493: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_experiment_ledger/rows.spi"); } LIT.with(|lit| lit.clone()) };
    let v494: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_experiment_registration/registration.spi"); } LIT.with(|lit| lit.clone()) };
    let v495: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-experiment-verb-args"); } LIT.with(|lit| lit.clone()) };
    let v496: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_experiment_verb_args/verbs.spi"); } LIT.with(|lit| lit.clone()) };
    let v497: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_gauge_ledger/ledger.spi"); } LIT.with(|lit| lit.clone()) };
    let v498: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-hook-verbs"); } LIT.with(|lit| lit.clone()) };
    let v499: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_hook_verbs/lease_verbs.spi"); } LIT.with(|lit| lit.clone()) };
    let v500: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-job-ticket"); } LIT.with(|lit| lit.clone()) };
    let v501: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_ticket/ticket.spi"); } LIT.with(|lit| lit.clone()) };
    let v502: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-lint-shadows-core"); } LIT.with(|lit| lit.clone()) };
    let v503: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lint_shadows_core/shadows.spi"); } LIT.with(|lit| lit.clone()) };
    let v504: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_memory_dupes/dupes.spi"); } LIT.with(|lit| lit.clone()) };
    let v505: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_memory_typed/memory.spi"); } LIT.with(|lit| lit.clone()) };
    let v506: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_proof_scope/import_scope.spi"); } LIT.with(|lit| lit.clone()) };
    let v507: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-status-changes"); } LIT.with(|lit| lit.clone()) };
    let v508: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_status_changes/changes.spi"); } LIT.with(|lit| lit.clone()) };
    let v509: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-split-plan"); } LIT.with(|lit| lit.clone()) };
    let v510: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_split_plan/plan.spi"); } LIT.with(|lit| lit.clone()) };
    let v511: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_asks/verb.spi"); } LIT.with(|lit| lit.clone()) };
    let v512: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-creation-text"); } LIT.with(|lit| lit.clone()) };
    let v513: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_creation_text/text.spi"); } LIT.with(|lit| lit.clone()) };
    let v514: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-notes"); } LIT.with(|lit| lit.clone()) };
    let v515: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_notes/task_notes.spi"); } LIT.with(|lit| lit.clone()) };
    let v516: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-parents"); } LIT.with(|lit| lit.clone()) };
    let v517: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_parents/parents.spi"); } LIT.with(|lit| lit.clone()) };
    let v518: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_parents/verb.spi"); } LIT.with(|lit| lit.clone()) };
    let v519: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-split"); } LIT.with(|lit| lit.clone()) };
    let v520: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_split/verb.spi"); } LIT.with(|lit| lit.clone()) };
    let v521: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-tree"); } LIT.with(|lit| lit.clone()) };
    let v522: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_tree/epic_tree.spi"); } LIT.with(|lit| lit.clone()) };
    let v523: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-team-lane"); } LIT.with(|lit| lit.clone()) };
    let v524: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_team_lane/lane.spi"); } LIT.with(|lit| lit.clone()) };
    let v525: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-team-work"); } LIT.with(|lit| lit.clone()) };
    let v526: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_team_work/work.spi"); } LIT.with(|lit| lit.clone()) };
    let v527: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-cache"); } LIT.with(|lit| lit.clone()) };
    let v528: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_cache/cache.spi"); } LIT.with(|lit| lit.clone()) };
    let v529: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-merge"); } LIT.with(|lit| lit.clone()) };
    let v530: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_merge/merge.spi"); } LIT.with(|lit| lit.clone()) };
    let v531: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/command_spec_domain/direct_verbs.spi"); } LIT.with(|lit| lit.clone()) };
    let v532: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("evidence-currentness-domain"); } LIT.with(|lit| lit.clone()) };
    let v533: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/evidence_currentness_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v534: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("proxy-file-receipt"); } LIT.with(|lit| lit.clone()) };
    let v535: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/proxy_file_receipt/file_receipt.spi"); } LIT.with(|lit| lit.clone()) };
    let v536: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("proxy-search-receipt"); } LIT.with(|lit| lit.clone()) };
    let v537: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/proxy_search_receipt/search_receipt.spi"); } LIT.with(|lit| lit.clone()) };
    let v538: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rating-guard-runtime"); } LIT.with(|lit| lit.clone()) };
    let v539: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rating_guard_runtime/guard.spi"); } LIT.with(|lit| lit.clone()) };
    let v540: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_adoption/args.spi"); } LIT.with(|lit| lit.clone()) };
    let v541: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_activity_migration/migration.spi"); } LIT.with(|lit| lit.clone()) };
    let v542: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-cadence"); } LIT.with(|lit| lit.clone()) };
    let v543: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_cadence/cadence.spi"); } LIT.with(|lit| lit.clone()) };
    let v544: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-gate"); } LIT.with(|lit| lit.clone()) };
    let v545: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_gate/gate.spi"); } LIT.with(|lit| lit.clone()) };
    let v546: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-id-prefix"); } LIT.with(|lit| lit.clone()) };
    let v547: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_id_prefix/prefix.spi"); } LIT.with(|lit| lit.clone()) };
    let v548: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-lint"); } LIT.with(|lit| lit.clone()) };
    let v549: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_lint/lint_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v550: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_sprint_append/append.spi"); } LIT.with(|lit| lit.clone()) };
    let v551: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-sprint-change"); } LIT.with(|lit| lit.clone()) };
    let v552: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_sprint_change/change.spi"); } LIT.with(|lit| lit.clone()) };
    let v553: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-sprint-read"); } LIT.with(|lit| lit.clone()) };
    let v554: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_sprint_read/read.spi"); } LIT.with(|lit| lit.clone()) };
    let v555: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-sprint-review"); } LIT.with(|lit| lit.clone()) };
    let v556: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_sprint_review/review.spi"); } LIT.with(|lit| lit.clone()) };
    let v557: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-sprint-start"); } LIT.with(|lit| lit.clone()) };
    let v558: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_sprint_start/start.spi"); } LIT.with(|lit| lit.clone()) };
    let v559: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_cadence_activity/activity.spi"); } LIT.with(|lit| lit.clone()) };
    let v560: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-cadence-asks"); } LIT.with(|lit| lit.clone()) };
    let v561: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_cadence_asks/asks.spi"); } LIT.with(|lit| lit.clone()) };
    let v562: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-cadence-done"); } LIT.with(|lit| lit.clone()) };
    let v563: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_cadence_done/done.spi"); } LIT.with(|lit| lit.clone()) };
    let v564: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-ceremonies"); } LIT.with(|lit| lit.clone()) };
    let v565: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_ceremonies/ceremonies.spi"); } LIT.with(|lit| lit.clone()) };
    let v566: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-cli-routes"); } LIT.with(|lit| lit.clone()) };
    let v567: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_cli_routes/routes.spi"); } LIT.with(|lit| lit.clone()) };
    let v568: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-receipts"); } LIT.with(|lit| lit.clone()) };
    let v569: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_receipts/verify.spi"); } LIT.with(|lit| lit.clone()) };
    let v570: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_style_census/census_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v571: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-window-run"); } LIT.with(|lit| lit.clone()) };
    let v572: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_window_run/run_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v573: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-table-test"); } LIT.with(|lit| lit.clone()) };
    let v574: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_table_test/table_test_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v575: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-resume"); } LIT.with(|lit| lit.clone()) };
    let v576: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_resume/resume_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v577: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-memory"); } LIT.with(|lit| lit.clone()) };
    let v578: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_memory/memory_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v579: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-guide"); } LIT.with(|lit| lit.clone()) };
    let v580: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_guide/guide_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v581: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-table-logs"); } LIT.with(|lit| lit.clone()) };
    let v582: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_table_logs/logs_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v583: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_table_logs/logs_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v584: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_experiment_stats/experiment_stats.spi"); } LIT.with(|lit| lit.clone()) };
    let v585: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_experiments/experiment_plan.spi"); } LIT.with(|lit| lit.clone()) };
    let v586: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_experiments/experiment_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v587: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_experiments/experiments_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v588: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-machine-windows"); } LIT.with(|lit| lit.clone()) };
    let v589: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_machine_windows/window_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v590: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_machine_windows/windows_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v591: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_standards/standards_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v592: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-memory-links"); } LIT.with(|lit| lit.clone()) };
    let v593: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_memory_links/links_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v594: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_memory_links/links_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v595: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_read_query/grep_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v596: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-crate-gate"); } LIT.with(|lit| lit.clone()) };
    let v597: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_crate_gate/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v598: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-table-text"); } LIT.with(|lit| lit.clone()) };
    let v599: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_table_text/text.spi"); } LIT.with(|lit| lit.clone()) };
    let v600: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_ci_parity/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v601: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_job_after_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v602: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-jobs-doctor"); } LIT.with(|lit| lit.clone()) };
    let v603: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_jobs_doctor_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v604: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-jobs-doctor-check"); } LIT.with(|lit| lit.clone()) };
    let v605: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_jobs_doctor/check.spi"); } LIT.with(|lit| lit.clone()) };
    let v606: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-agile-records"); } LIT.with(|lit| lit.clone()) };
    let v607: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_agile_records/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v608: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-lint-macros"); } LIT.with(|lit| lit.clone()) };
    let v609: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lint_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v610: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-lint-flow"); } LIT.with(|lit| lit.clone()) };
    let v611: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lint_pattern/pattern.spi"); } LIT.with(|lit| lit.clone()) };
    let v612: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lint_flow/flow.spi"); } LIT.with(|lit| lit.clone()) };
    let v613: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-lint-flow-constructs"); } LIT.with(|lit| lit.clone()) };
    let v614: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lint_flow_constructs/constructs.spi"); } LIT.with(|lit| lit.clone()) };
    let v615: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-lint-flow-idioms"); } LIT.with(|lit| lit.clone()) };
    let v616: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_lint_flow_idioms/idioms.spi"); } LIT.with(|lit| lit.clone()) };
    let v617: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_git/git_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v618: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-machine-sample"); } LIT.with(|lit| lit.clone()) };
    let v619: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_machine_sample/sample.spi"); } LIT.with(|lit| lit.clone()) };
    let v620: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig-do-walk"); } LIT.with(|lit| lit.clone()) };
    let v621: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/rig_do_walk/walk.spi"); } LIT.with(|lit| lit.clone()) };
    let v622: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v623: Rc<UH2> = Rc::new(UH2::UH2_Cons(v620, v621, v622));
    let v624: Rc<UH2> = Rc::new(UH2::UH2_Cons(v618, v619, v623));
    let v625: Rc<UH2> = Rc::new(UH2::UH2_Cons(v131.clone(), v617, v624));
    let v626: Rc<UH2> = Rc::new(UH2::UH2_Cons(v615, v616, v625));
    let v627: Rc<UH2> = Rc::new(UH2::UH2_Cons(v613, v614, v626));
    let v628: Rc<UH2> = Rc::new(UH2::UH2_Cons(v610.clone(), v612, v627));
    let v629: Rc<UH2> = Rc::new(UH2::UH2_Cons(v610, v611, v628));
    let v630: Rc<UH2> = Rc::new(UH2::UH2_Cons(v608, v609, v629));
    let v631: Rc<UH2> = Rc::new(UH2::UH2_Cons(v606, v607, v630));
    let v632: Rc<UH2> = Rc::new(UH2::UH2_Cons(v604, v605, v631));
    let v633: Rc<UH2> = Rc::new(UH2::UH2_Cons(v602, v603, v632));
    let v634: Rc<UH2> = Rc::new(UH2::UH2_Cons(v14.clone(), v601, v633));
    let v635: Rc<UH2> = Rc::new(UH2::UH2_Cons(v12.clone(), v600, v634));
    let v636: Rc<UH2> = Rc::new(UH2::UH2_Cons(v598, v599, v635));
    let v637: Rc<UH2> = Rc::new(UH2::UH2_Cons(v596, v597, v636));
    let v638: Rc<UH2> = Rc::new(UH2::UH2_Cons(v103.clone(), v595, v637));
    let v639: Rc<UH2> = Rc::new(UH2::UH2_Cons(v592.clone(), v594, v638));
    let v640: Rc<UH2> = Rc::new(UH2::UH2_Cons(v592, v593, v639));
    let v641: Rc<UH2> = Rc::new(UH2::UH2_Cons(v169.clone(), v591, v640));
    let v642: Rc<UH2> = Rc::new(UH2::UH2_Cons(v588.clone(), v590, v641));
    let v643: Rc<UH2> = Rc::new(UH2::UH2_Cons(v588, v589, v642));
    let v644: Rc<UH2> = Rc::new(UH2::UH2_Cons(v125.clone(), v587, v643));
    let v645: Rc<UH2> = Rc::new(UH2::UH2_Cons(v125.clone(), v586, v644));
    let v646: Rc<UH2> = Rc::new(UH2::UH2_Cons(v125.clone(), v585, v645));
    let v647: Rc<UH2> = Rc::new(UH2::UH2_Cons(v125.clone(), v584, v646));
    let v648: Rc<UH2> = Rc::new(UH2::UH2_Cons(v581.clone(), v583, v647));
    let v649: Rc<UH2> = Rc::new(UH2::UH2_Cons(v581, v582, v648));
    let v650: Rc<UH2> = Rc::new(UH2::UH2_Cons(v579, v580, v649));
    let v651: Rc<UH2> = Rc::new(UH2::UH2_Cons(v577, v578, v650));
    let v652: Rc<UH2> = Rc::new(UH2::UH2_Cons(v575, v576, v651));
    let v653: Rc<UH2> = Rc::new(UH2::UH2_Cons(v573, v574, v652));
    let v654: Rc<UH2> = Rc::new(UH2::UH2_Cons(v571, v572, v653));
    let v655: Rc<UH2> = Rc::new(UH2::UH2_Cons(v372.clone(), v570, v654));
    let v656: Rc<UH2> = Rc::new(UH2::UH2_Cons(v568, v569, v655));
    let v657: Rc<UH2> = Rc::new(UH2::UH2_Cons(v566, v567, v656));
    let v658: Rc<UH2> = Rc::new(UH2::UH2_Cons(v564, v565, v657));
    let v659: Rc<UH2> = Rc::new(UH2::UH2_Cons(v562, v563, v658));
    let v660: Rc<UH2> = Rc::new(UH2::UH2_Cons(v560, v561, v659));
    let v661: Rc<UH2> = Rc::new(UH2::UH2_Cons(v422.clone(), v559, v660));
    let v662: Rc<UH2> = Rc::new(UH2::UH2_Cons(v557, v558, v661));
    let v663: Rc<UH2> = Rc::new(UH2::UH2_Cons(v555, v556, v662));
    let v664: Rc<UH2> = Rc::new(UH2::UH2_Cons(v553, v554, v663));
    let v665: Rc<UH2> = Rc::new(UH2::UH2_Cons(v551, v552, v664));
    let v666: Rc<UH2> = Rc::new(UH2::UH2_Cons(v222.clone(), v550, v665));
    let v667: Rc<UH2> = Rc::new(UH2::UH2_Cons(v548, v549, v666));
    let v668: Rc<UH2> = Rc::new(UH2::UH2_Cons(v546, v547, v667));
    let v669: Rc<UH2> = Rc::new(UH2::UH2_Cons(v544, v545, v668));
    let v670: Rc<UH2> = Rc::new(UH2::UH2_Cons(v542, v543, v669));
    let v671: Rc<UH2> = Rc::new(UH2::UH2_Cons(v418.clone(), v541, v670));
    let v672: Rc<UH2> = Rc::new(UH2::UH2_Cons(v84.clone(), v540, v671));
    let v673: Rc<UH2> = Rc::new(UH2::UH2_Cons(v538, v539, v672));
    let v674: Rc<UH2> = Rc::new(UH2::UH2_Cons(v536, v537, v673));
    let v675: Rc<UH2> = Rc::new(UH2::UH2_Cons(v534, v535, v674));
    let v676: Rc<UH2> = Rc::new(UH2::UH2_Cons(v532, v533, v675));
    let v677: Rc<UH2> = Rc::new(UH2::UH2_Cons(v63.clone(), v531, v676));
    let v678: Rc<UH2> = Rc::new(UH2::UH2_Cons(v529, v530, v677));
    let v679: Rc<UH2> = Rc::new(UH2::UH2_Cons(v527, v528, v678));
    let v680: Rc<UH2> = Rc::new(UH2::UH2_Cons(v525, v526, v679));
    let v681: Rc<UH2> = Rc::new(UH2::UH2_Cons(v523, v524, v680));
    let v682: Rc<UH2> = Rc::new(UH2::UH2_Cons(v521, v522, v681));
    let v683: Rc<UH2> = Rc::new(UH2::UH2_Cons(v519, v520, v682));
    let v684: Rc<UH2> = Rc::new(UH2::UH2_Cons(v516.clone(), v518, v683));
    let v685: Rc<UH2> = Rc::new(UH2::UH2_Cons(v516, v517, v684));
    let v686: Rc<UH2> = Rc::new(UH2::UH2_Cons(v514, v515, v685));
    let v687: Rc<UH2> = Rc::new(UH2::UH2_Cons(v512, v513, v686));
    let v688: Rc<UH2> = Rc::new(UH2::UH2_Cons(v198.clone(), v511, v687));
    let v689: Rc<UH2> = Rc::new(UH2::UH2_Cons(v509, v510, v688));
    let v690: Rc<UH2> = Rc::new(UH2::UH2_Cons(v507, v508, v689));
    let v691: Rc<UH2> = Rc::new(UH2::UH2_Cons(v250.clone(), v506, v690));
    let v692: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v505, v691));
    let v693: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v504, v692));
    let v694: Rc<UH2> = Rc::new(UH2::UH2_Cons(v502, v503, v693));
    let v695: Rc<UH2> = Rc::new(UH2::UH2_Cons(v500, v501, v694));
    let v696: Rc<UH2> = Rc::new(UH2::UH2_Cons(v498, v499, v695));
    let v697: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v497, v696));
    let v698: Rc<UH2> = Rc::new(UH2::UH2_Cons(v495, v496, v697));
    let v699: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v494, v698));
    let v700: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v493, v699));
    let v701: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v492, v700));
    let v702: Rc<UH2> = Rc::new(UH2::UH2_Cons(v490, v491, v701));
    let v703: Rc<UH2> = Rc::new(UH2::UH2_Cons(v483.clone(), v489, v702));
    let v704: Rc<UH2> = Rc::new(UH2::UH2_Cons(v487, v488, v703));
    let v705: Rc<UH2> = Rc::new(UH2::UH2_Cons(v485, v486, v704));
    let v706: Rc<UH2> = Rc::new(UH2::UH2_Cons(v483, v484, v705));
    let v707: Rc<UH2> = Rc::new(UH2::UH2_Cons(v481, v482, v706));
    let v708: Rc<UH2> = Rc::new(UH2::UH2_Cons(v479, v480, v707));
    let v709: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v478, v708));
    let v710: Rc<UH2> = Rc::new(UH2::UH2_Cons(v476, v477, v709));
    let v711: Rc<UH2> = Rc::new(UH2::UH2_Cons(v474, v475, v710));
    let v712: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v473, v711));
    let v713: Rc<UH2> = Rc::new(UH2::UH2_Cons(v471, v472, v712));
    let v714: Rc<UH2> = Rc::new(UH2::UH2_Cons(v469, v470, v713));
    let v715: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v468, v714));
    let v716: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v467, v715));
    let v717: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v466, v716));
    let v718: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v465, v717));
    let v719: Rc<UH2> = Rc::new(UH2::UH2_Cons(v224.clone(), v464, v718));
    let v720: Rc<UH2> = Rc::new(UH2::UH2_Cons(v462, v463, v719));
    let v721: Rc<UH2> = Rc::new(UH2::UH2_Cons(v460, v461, v720));
    let v722: Rc<UH2> = Rc::new(UH2::UH2_Cons(v458, v459, v721));
    let v723: Rc<UH2> = Rc::new(UH2::UH2_Cons(v456, v457, v722));
    let v724: Rc<UH2> = Rc::new(UH2::UH2_Cons(v454, v455, v723));
    let v725: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v453, v724));
    let v726: Rc<UH2> = Rc::new(UH2::UH2_Cons(v451, v452, v725));
    let v727: Rc<UH2> = Rc::new(UH2::UH2_Cons(v449, v450, v726));
    let v728: Rc<UH2> = Rc::new(UH2::UH2_Cons(v447, v448, v727));
    let v729: Rc<UH2> = Rc::new(UH2::UH2_Cons(v445, v446, v728));
    let v730: Rc<UH2> = Rc::new(UH2::UH2_Cons(v443, v444, v729));
    let v731: Rc<UH2> = Rc::new(UH2::UH2_Cons(v441, v442, v730));
    let v732: Rc<UH2> = Rc::new(UH2::UH2_Cons(v439, v440, v731));
    let v733: Rc<UH2> = Rc::new(UH2::UH2_Cons(v437, v438, v732));
    let v734: Rc<UH2> = Rc::new(UH2::UH2_Cons(v435, v436, v733));
    let v735: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v434, v734));
    let v736: Rc<UH2> = Rc::new(UH2::UH2_Cons(v432, v433, v735));
    let v737: Rc<UH2> = Rc::new(UH2::UH2_Cons(v430, v431, v736));
    let v738: Rc<UH2> = Rc::new(UH2::UH2_Cons(v428, v429, v737));
    let v739: Rc<UH2> = Rc::new(UH2::UH2_Cons(v426, v427, v738));
    let v740: Rc<UH2> = Rc::new(UH2::UH2_Cons(v424, v425, v739));
    let v741: Rc<UH2> = Rc::new(UH2::UH2_Cons(v422, v423, v740));
    let v742: Rc<UH2> = Rc::new(UH2::UH2_Cons(v228.clone(), v421, v741));
    let v743: Rc<UH2> = Rc::new(UH2::UH2_Cons(v418.clone(), v420, v742));
    let v744: Rc<UH2> = Rc::new(UH2::UH2_Cons(v418, v419, v743));
    let v745: Rc<UH2> = Rc::new(UH2::UH2_Cons(v228.clone(), v417, v744));
    let v746: Rc<UH2> = Rc::new(UH2::UH2_Cons(v415, v416, v745));
    let v747: Rc<UH2> = Rc::new(UH2::UH2_Cons(v413, v414, v746));
    let v748: Rc<UH2> = Rc::new(UH2::UH2_Cons(v411, v412, v747));
    let v749: Rc<UH2> = Rc::new(UH2::UH2_Cons(v409, v410, v748));
    let v750: Rc<UH2> = Rc::new(UH2::UH2_Cons(v407, v408, v749));
    let v751: Rc<UH2> = Rc::new(UH2::UH2_Cons(v405, v406, v750));
    let v752: Rc<UH2> = Rc::new(UH2::UH2_Cons(v399.clone(), v404, v751));
    let v753: Rc<UH2> = Rc::new(UH2::UH2_Cons(v402, v403, v752));
    let v754: Rc<UH2> = Rc::new(UH2::UH2_Cons(v399.clone(), v401, v753));
    let v755: Rc<UH2> = Rc::new(UH2::UH2_Cons(v399, v400, v754));
    let v756: Rc<UH2> = Rc::new(UH2::UH2_Cons(v396.clone(), v398, v755));
    let v757: Rc<UH2> = Rc::new(UH2::UH2_Cons(v396, v397, v756));
    let v758: Rc<UH2> = Rc::new(UH2::UH2_Cons(v394, v395, v757));
    let v759: Rc<UH2> = Rc::new(UH2::UH2_Cons(v392, v393, v758));
    let v760: Rc<UH2> = Rc::new(UH2::UH2_Cons(v390, v391, v759));
    let v761: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v389, v760));
    let v762: Rc<UH2> = Rc::new(UH2::UH2_Cons(v387, v388, v761));
    let v763: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v386, v762));
    let v764: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v385, v763));
    let v765: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v384, v764));
    let v766: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v383, v765));
    let v767: Rc<UH2> = Rc::new(UH2::UH2_Cons(v376.clone(), v382, v766));
    let v768: Rc<UH2> = Rc::new(UH2::UH2_Cons(v98.clone(), v381, v767));
    let v769: Rc<UH2> = Rc::new(UH2::UH2_Cons(v379, v380, v768));
    let v770: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v378, v769));
    let v771: Rc<UH2> = Rc::new(UH2::UH2_Cons(v376, v377, v770));
    let v772: Rc<UH2> = Rc::new(UH2::UH2_Cons(v374, v375, v771));
    let v773: Rc<UH2> = Rc::new(UH2::UH2_Cons(v372, v373, v772));
    let v774: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v371, v773));
    let v775: Rc<UH2> = Rc::new(UH2::UH2_Cons(v369, v370, v774));
    let v776: Rc<UH2> = Rc::new(UH2::UH2_Cons(v367, v368, v775));
    let v777: Rc<UH2> = Rc::new(UH2::UH2_Cons(v365, v366, v776));
    let v778: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v364, v777));
    let v779: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v363, v778));
    let v780: Rc<UH2> = Rc::new(UH2::UH2_Cons(v361, v362, v779));
    let v781: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v360, v780));
    let v782: Rc<UH2> = Rc::new(UH2::UH2_Cons(v358, v359, v781));
    let v783: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v357, v782));
    let v784: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v356, v783));
    let v785: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v355, v784));
    let v786: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v354, v785));
    let v787: Rc<UH2> = Rc::new(UH2::UH2_Cons(v352, v353, v786));
    let v788: Rc<UH2> = Rc::new(UH2::UH2_Cons(v350, v351, v787));
    let v789: Rc<UH2> = Rc::new(UH2::UH2_Cons(v345.clone(), v349, v788));
    let v790: Rc<UH2> = Rc::new(UH2::UH2_Cons(v347, v348, v789));
    let v791: Rc<UH2> = Rc::new(UH2::UH2_Cons(v345, v346, v790));
    let v792: Rc<UH2> = Rc::new(UH2::UH2_Cons(v343, v344, v791));
    let v793: Rc<UH2> = Rc::new(UH2::UH2_Cons(v341, v342, v792));
    let v794: Rc<UH2> = Rc::new(UH2::UH2_Cons(v339, v340, v793));
    let v795: Rc<UH2> = Rc::new(UH2::UH2_Cons(v337, v338, v794));
    let v796: Rc<UH2> = Rc::new(UH2::UH2_Cons(v334.clone(), v336, v795));
    let v797: Rc<UH2> = Rc::new(UH2::UH2_Cons(v334, v335, v796));
    let v798: Rc<UH2> = Rc::new(UH2::UH2_Cons(v332, v333, v797));
    let v799: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v331, v798));
    let v800: Rc<UH2> = Rc::new(UH2::UH2_Cons(v329, v330, v799));
    let v801: Rc<UH2> = Rc::new(UH2::UH2_Cons(v327, v328, v800));
    let v802: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v326, v801));
    let v803: Rc<UH2> = Rc::new(UH2::UH2_Cons(v324, v325, v802));
    let v804: Rc<UH2> = Rc::new(UH2::UH2_Cons(v322, v323, v803));
    let v805: Rc<UH2> = Rc::new(UH2::UH2_Cons(v320, v321, v804));
    let v806: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v319, v805));
    let v807: Rc<UH2> = Rc::new(UH2::UH2_Cons(v317, v318, v806));
    let v808: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v316, v807));
    let v809: Rc<UH2> = Rc::new(UH2::UH2_Cons(v314, v315, v808));
    let v810: Rc<UH2> = Rc::new(UH2::UH2_Cons(v312, v313, v809));
    let v811: Rc<UH2> = Rc::new(UH2::UH2_Cons(v310, v311, v810));
    let v812: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v309, v811));
    let v813: Rc<UH2> = Rc::new(UH2::UH2_Cons(v307, v308, v812));
    let v814: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v306, v813));
    let v815: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v305, v814));
    let v816: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v304, v815));
    let v817: Rc<UH2> = Rc::new(UH2::UH2_Cons(v302, v303, v816));
    let v818: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v301, v817));
    let v819: Rc<UH2> = Rc::new(UH2::UH2_Cons(v299, v300, v818));
    let v820: Rc<UH2> = Rc::new(UH2::UH2_Cons(v91.clone(), v298, v819));
    let v821: Rc<UH2> = Rc::new(UH2::UH2_Cons(v296, v297, v820));
    let v822: Rc<UH2> = Rc::new(UH2::UH2_Cons(v294, v295, v821));
    let v823: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v293, v822));
    let v824: Rc<UH2> = Rc::new(UH2::UH2_Cons(v291, v292, v823));
    let v825: Rc<UH2> = Rc::new(UH2::UH2_Cons(v35.clone(), v290, v824));
    let v826: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v289, v825));
    let v827: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v288, v826));
    let v828: Rc<UH2> = Rc::new(UH2::UH2_Cons(v286, v287, v827));
    let v829: Rc<UH2> = Rc::new(UH2::UH2_Cons(v284, v285, v828));
    let v830: Rc<UH2> = Rc::new(UH2::UH2_Cons(v282, v283, v829));
    let v831: Rc<UH2> = Rc::new(UH2::UH2_Cons(v280, v281, v830));
    let v832: Rc<UH2> = Rc::new(UH2::UH2_Cons(v278, v279, v831));
    let v833: Rc<UH2> = Rc::new(UH2::UH2_Cons(v276, v277, v832));
    let v834: Rc<UH2> = Rc::new(UH2::UH2_Cons(v274, v275, v833));
    let v835: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v273, v834));
    let v836: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v272, v835));
    let v837: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v271, v836));
    let v838: Rc<UH2> = Rc::new(UH2::UH2_Cons(v269, v270, v837));
    let v839: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v268, v838));
    let v840: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v267, v839));
    let v841: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v266, v840));
    let v842: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v265, v841));
    let v843: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v264, v842));
    let v844: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v263, v843));
    let v845: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v262, v844));
    let v846: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v261, v845));
    let v847: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v260, v846));
    let v848: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v259, v847));
    let v849: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v258, v848));
    let v850: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v257, v849));
    let v851: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v256, v850));
    let v852: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v255, v851));
    let v853: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v254, v852));
    let v854: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v253, v853));
    let v855: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v252, v854));
    let v856: Rc<UH2> = Rc::new(UH2::UH2_Cons(v250, v251, v855));
    let v857: Rc<UH2> = Rc::new(UH2::UH2_Cons(v248, v249, v856));
    let v858: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v247, v857));
    let v859: Rc<UH2> = Rc::new(UH2::UH2_Cons(v245, v246, v858));
    let v860: Rc<UH2> = Rc::new(UH2::UH2_Cons(v243, v244, v859));
    let v861: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v242, v860));
    let v862: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v241, v861));
    let v863: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v240, v862));
    let v864: Rc<UH2> = Rc::new(UH2::UH2_Cons(v238, v239, v863));
    let v865: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v237, v864));
    let v866: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v236, v865));
    let v867: Rc<UH2> = Rc::new(UH2::UH2_Cons(v198.clone(), v235, v866));
    let v868: Rc<UH2> = Rc::new(UH2::UH2_Cons(v233, v234, v867));
    let v869: Rc<UH2> = Rc::new(UH2::UH2_Cons(v187.clone(), v232, v868));
    let v870: Rc<UH2> = Rc::new(UH2::UH2_Cons(v228.clone(), v231, v869));
    let v871: Rc<UH2> = Rc::new(UH2::UH2_Cons(v228.clone(), v230, v870));
    let v872: Rc<UH2> = Rc::new(UH2::UH2_Cons(v228, v229, v871));
    let v873: Rc<UH2> = Rc::new(UH2::UH2_Cons(v226, v227, v872));
    let v874: Rc<UH2> = Rc::new(UH2::UH2_Cons(v224, v225, v873));
    let v875: Rc<UH2> = Rc::new(UH2::UH2_Cons(v222, v223, v874));
    let v876: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v221, v875));
    let v877: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v220, v876));
    let v878: Rc<UH2> = Rc::new(UH2::UH2_Cons(v218, v219, v877));
    let v879: Rc<UH2> = Rc::new(UH2::UH2_Cons(v125.clone(), v217, v878));
    let v880: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v216, v879));
    let v881: Rc<UH2> = Rc::new(UH2::UH2_Cons(v214, v215, v880));
    let v882: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v213, v881));
    let v883: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v212, v882));
    let v884: Rc<UH2> = Rc::new(UH2::UH2_Cons(v71.clone(), v211, v883));
    let v885: Rc<UH2> = Rc::new(UH2::UH2_Cons(v209, v210, v884));
    let v886: Rc<UH2> = Rc::new(UH2::UH2_Cons(v206.clone(), v208, v885));
    let v887: Rc<UH2> = Rc::new(UH2::UH2_Cons(v206, v207, v886));
    let v888: Rc<UH2> = Rc::new(UH2::UH2_Cons(v204, v205, v887));
    let v889: Rc<UH2> = Rc::new(UH2::UH2_Cons(v202, v203, v888));
    let v890: Rc<UH2> = Rc::new(UH2::UH2_Cons(v200, v201, v889));
    let v891: Rc<UH2> = Rc::new(UH2::UH2_Cons(v198, v199, v890));
    let v892: Rc<UH2> = Rc::new(UH2::UH2_Cons(v196, v197, v891));
    let v893: Rc<UH2> = Rc::new(UH2::UH2_Cons(v194, v195, v892));
    let v894: Rc<UH2> = Rc::new(UH2::UH2_Cons(v192, v193, v893));
    let v895: Rc<UH2> = Rc::new(UH2::UH2_Cons(v190, v191, v894));
    let v896: Rc<UH2> = Rc::new(UH2::UH2_Cons(v129.clone(), v189, v895));
    let v897: Rc<UH2> = Rc::new(UH2::UH2_Cons(v187, v188, v896));
    let v898: Rc<UH2> = Rc::new(UH2::UH2_Cons(v185, v186, v897));
    let v899: Rc<UH2> = Rc::new(UH2::UH2_Cons(v183, v184, v898));
    let v900: Rc<UH2> = Rc::new(UH2::UH2_Cons(v181, v182, v899));
    let v901: Rc<UH2> = Rc::new(UH2::UH2_Cons(v179, v180, v900));
    let v902: Rc<UH2> = Rc::new(UH2::UH2_Cons(v177, v178, v901));
    let v903: Rc<UH2> = Rc::new(UH2::UH2_Cons(v175, v176, v902));
    let v904: Rc<UH2> = Rc::new(UH2::UH2_Cons(v173, v174, v903));
    let v905: Rc<UH2> = Rc::new(UH2::UH2_Cons(v171, v172, v904));
    let v906: Rc<UH2> = Rc::new(UH2::UH2_Cons(v169, v170, v905));
    let v907: Rc<UH2> = Rc::new(UH2::UH2_Cons(v166.clone(), v168, v906));
    let v908: Rc<UH2> = Rc::new(UH2::UH2_Cons(v166, v167, v907));
    let v909: Rc<UH2> = Rc::new(UH2::UH2_Cons(v98.clone(), v165, v908));
    let v910: Rc<UH2> = Rc::new(UH2::UH2_Cons(v98.clone(), v164, v909));
    let v911: Rc<UH2> = Rc::new(UH2::UH2_Cons(v162, v163, v910));
    let v912: Rc<UH2> = Rc::new(UH2::UH2_Cons(v27.clone(), v161, v911));
    let v913: Rc<UH2> = Rc::new(UH2::UH2_Cons(v158.clone(), v160, v912));
    let v914: Rc<UH2> = Rc::new(UH2::UH2_Cons(v158, v159, v913));
    let v915: Rc<UH2> = Rc::new(UH2::UH2_Cons(v156, v157, v914));
    let v916: Rc<UH2> = Rc::new(UH2::UH2_Cons(v91.clone(), v155, v915));
    let v917: Rc<UH2> = Rc::new(UH2::UH2_Cons(v153, v154, v916));
    let v918: Rc<UH2> = Rc::new(UH2::UH2_Cons(v145.clone(), v152, v917));
    let v919: Rc<UH2> = Rc::new(UH2::UH2_Cons(v148.clone(), v151, v918));
    let v920: Rc<UH2> = Rc::new(UH2::UH2_Cons(v148.clone(), v150, v919));
    let v921: Rc<UH2> = Rc::new(UH2::UH2_Cons(v148, v149, v920));
    let v922: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v147, v921));
    let v923: Rc<UH2> = Rc::new(UH2::UH2_Cons(v145, v146, v922));
    let v924: Rc<UH2> = Rc::new(UH2::UH2_Cons(v143, v144, v923));
    let v925: Rc<UH2> = Rc::new(UH2::UH2_Cons(v141, v142, v924));
    let v926: Rc<UH2> = Rc::new(UH2::UH2_Cons(v138.clone(), v140, v925));
    let v927: Rc<UH2> = Rc::new(UH2::UH2_Cons(v138, v139, v926));
    let v928: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v137, v927));
    let v929: Rc<UH2> = Rc::new(UH2::UH2_Cons(v135, v136, v928));
    let v930: Rc<UH2> = Rc::new(UH2::UH2_Cons(v133, v134, v929));
    let v931: Rc<UH2> = Rc::new(UH2::UH2_Cons(v131, v132, v930));
    let v932: Rc<UH2> = Rc::new(UH2::UH2_Cons(v129, v130, v931));
    let v933: Rc<UH2> = Rc::new(UH2::UH2_Cons(v127, v128, v932));
    let v934: Rc<UH2> = Rc::new(UH2::UH2_Cons(v125, v126, v933));
    let v935: Rc<UH2> = Rc::new(UH2::UH2_Cons(v123, v124, v934));
    let v936: Rc<UH2> = Rc::new(UH2::UH2_Cons(v121, v122, v935));
    let v937: Rc<UH2> = Rc::new(UH2::UH2_Cons(v119, v120, v936));
    let v938: Rc<UH2> = Rc::new(UH2::UH2_Cons(v117, v118, v937));
    let v939: Rc<UH2> = Rc::new(UH2::UH2_Cons(v115, v116, v938));
    let v940: Rc<UH2> = Rc::new(UH2::UH2_Cons(v113, v114, v939));
    let v941: Rc<UH2> = Rc::new(UH2::UH2_Cons(v111, v112, v940));
    let v942: Rc<UH2> = Rc::new(UH2::UH2_Cons(v86.clone(), v110, v941));
    let v943: Rc<UH2> = Rc::new(UH2::UH2_Cons(v108, v109, v942));
    let v944: Rc<UH2> = Rc::new(UH2::UH2_Cons(v106, v107, v943));
    let v945: Rc<UH2> = Rc::new(UH2::UH2_Cons(v103.clone(), v105, v944));
    let v946: Rc<UH2> = Rc::new(UH2::UH2_Cons(v103, v104, v945));
    let v947: Rc<UH2> = Rc::new(UH2::UH2_Cons(v32.clone(), v102, v946));
    let v948: Rc<UH2> = Rc::new(UH2::UH2_Cons(v100, v101, v947));
    let v949: Rc<UH2> = Rc::new(UH2::UH2_Cons(v98, v99, v948));
    let v950: Rc<UH2> = Rc::new(UH2::UH2_Cons(v96, v97, v949));
    let v951: Rc<UH2> = Rc::new(UH2::UH2_Cons(v94, v95, v950));
    let v952: Rc<UH2> = Rc::new(UH2::UH2_Cons(v91.clone(), v93, v951));
    let v953: Rc<UH2> = Rc::new(UH2::UH2_Cons(v91, v92, v952));
    let v954: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v90, v953));
    let v955: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88, v89, v954));
    let v956: Rc<UH2> = Rc::new(UH2::UH2_Cons(v86, v87, v955));
    let v957: Rc<UH2> = Rc::new(UH2::UH2_Cons(v84, v85, v956));
    let v958: Rc<UH2> = Rc::new(UH2::UH2_Cons(v82, v83, v957));
    let v959: Rc<UH2> = Rc::new(UH2::UH2_Cons(v80, v81, v958));
    let v960: Rc<UH2> = Rc::new(UH2::UH2_Cons(v78, v79, v959));
    let v961: Rc<UH2> = Rc::new(UH2::UH2_Cons(v76, v77, v960));
    let v962: Rc<UH2> = Rc::new(UH2::UH2_Cons(v74, v75, v961));
    let v963: Rc<UH2> = Rc::new(UH2::UH2_Cons(v71.clone(), v73, v962));
    let v964: Rc<UH2> = Rc::new(UH2::UH2_Cons(v71, v72, v963));
    let v965: Rc<UH2> = Rc::new(UH2::UH2_Cons(v69, v70, v964));
    let v966: Rc<UH2> = Rc::new(UH2::UH2_Cons(v67, v68, v965));
    let v967: Rc<UH2> = Rc::new(UH2::UH2_Cons(v65, v66, v966));
    let v968: Rc<UH2> = Rc::new(UH2::UH2_Cons(v63, v64, v967));
    let v969: Rc<UH2> = Rc::new(UH2::UH2_Cons(v61, v62, v968));
    let v970: Rc<UH2> = Rc::new(UH2::UH2_Cons(v59, v60, v969));
    let v971: Rc<UH2> = Rc::new(UH2::UH2_Cons(v57, v58, v970));
    let v972: Rc<UH2> = Rc::new(UH2::UH2_Cons(v55, v56, v971));
    let v973: Rc<UH2> = Rc::new(UH2::UH2_Cons(v53, v54, v972));
    let v974: Rc<UH2> = Rc::new(UH2::UH2_Cons(v51, v52, v973));
    let v975: Rc<UH2> = Rc::new(UH2::UH2_Cons(v49, v50, v974));
    let v976: Rc<UH2> = Rc::new(UH2::UH2_Cons(v47, v48, v975));
    let v977: Rc<UH2> = Rc::new(UH2::UH2_Cons(v45, v46, v976));
    let v978: Rc<UH2> = Rc::new(UH2::UH2_Cons(v43, v44, v977));
    let v979: Rc<UH2> = Rc::new(UH2::UH2_Cons(v41, v42, v978));
    let v980: Rc<UH2> = Rc::new(UH2::UH2_Cons(v39, v40, v979));
    let v981: Rc<UH2> = Rc::new(UH2::UH2_Cons(v37, v38, v980));
    let v982: Rc<UH2> = Rc::new(UH2::UH2_Cons(v35, v36, v981));
    let v983: Rc<UH2> = Rc::new(UH2::UH2_Cons(v32.clone(), v34, v982));
    let v984: Rc<UH2> = Rc::new(UH2::UH2_Cons(v32, v33, v983));
    let v985: Rc<UH2> = Rc::new(UH2::UH2_Cons(v30, v31, v984));
    let v986: Rc<UH2> = Rc::new(UH2::UH2_Cons(v27.clone(), v29, v985));
    let v987: Rc<UH2> = Rc::new(UH2::UH2_Cons(v27, v28, v986));
    let v988: Rc<UH2> = Rc::new(UH2::UH2_Cons(v25, v26, v987));
    let v989: Rc<UH2> = Rc::new(UH2::UH2_Cons(v22.clone(), v24, v988));
    let v990: Rc<UH2> = Rc::new(UH2::UH2_Cons(v22, v23, v989));
    let v991: Rc<UH2> = Rc::new(UH2::UH2_Cons(v20, v21, v990));
    let v992: Rc<UH2> = Rc::new(UH2::UH2_Cons(v18, v19, v991));
    let v993: Rc<UH2> = Rc::new(UH2::UH2_Cons(v16, v17, v992));
    let v994: Rc<UH2> = Rc::new(UH2::UH2_Cons(v14, v15, v993));
    let v995: Rc<UH2> = Rc::new(UH2::UH2_Cons(v12, v13, v994));
    let v996: Rc<UH2> = Rc::new(UH2::UH2_Cons(v10, v11, v995));
    let v997: Rc<UH2> = Rc::new(UH2::UH2_Cons(v8, v9, v996));
    let v998: Rc<UH2> = Rc::new(UH2::UH2_Cons(v6, v7, v997));
    let v999: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v1000: Rc<str> = selected_notebooks_32(v0.clone(), v2, v3, v998, v999.clone());
    if v4 {
        let v1001: i32 = 0i32;
        announced_1(v1000, v1001)
    } else {
        let v1003: bool = &*v1000 == "";
        if v1003 {
            0i32
        } else {
            let v1004: i32 = 32i32;
            let v1005: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v1006: Rc<UH1> = split_list_15(v1005, v1000);
            let v1007: i32 = 0i32;
            let v1008: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v1009: Rc<UH1> = chunks_from_33(v1004, v1006, v999, v1007, v1008);
            let v1010: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v1011: Rc<UH1> = method34(v1009, v1010);
            let v1012: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFailures); } CASE.with(|case| case.clone()) };
            let v1013: i32 = 0i32;
            batches_run_2(v0, v1, v5, v1011, v1012, v1013)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, u64) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64| -> i32 {
        run_registered_0(v0, v1, v2, v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_dev_spiral_tests_run(v0: &str, v1: &str, v2: &str, v3: &str, v4: bool, v5: u64) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4, v5)
}
pub type SpiralShape_accbf6de6842 = Mut0;
