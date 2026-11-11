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
    US0_RemoteUnreachable(Rc<str>),
    US0_RemoteReported(u64, u64, u64),
}
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_DiskAbove(u64),
    US3_DiskBelow(u64),
    US3_DiskUnknown(Rc<str>),
}
fn summary_marker_3() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" window list ok "); } LIT.with(|lit| lit.clone()) };
    v0
}
fn method5(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let mut v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn field_value_4(mut v0: Rc<str>, mut v1: Rc<str>) -> u64 {
    let v3: Rc<str> = string_concat(&[" ", &*v1, "="]);
    let mut v4: i32 = v0.get(0i32 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + 0i32 as usize) as i32).unwrap_or(-1);
    let v5: bool = v4 < 0i32;
    if v5 {
        0u64
    } else {
        let mut v6: i32 = (v1.len() as i32);
        let v7: i32 = v4.wrapping_add(v6);
        let v8: i32 = v7.wrapping_add(2i32);
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v10: i32 = v0.get(v8 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v8 as usize) as i32).unwrap_or(-1);
        let mut v11: bool = v10 < 0i32;
        let v13: i32 = if v11 {
            let v12: i32 = (v0.clone().len() as i32);
            v12
        } else {
            v10
        };
        let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v13 as usize).unwrap_or(""));
        let mut v15: Option<u64> = (v14).trim().parse::<u64>().ok();
        let mut v16: Option<u64> = method5(v15);
        let mut v17: Rc<dyn Fn((u64)) -> US1> = closure1();
        let mut v18: Option<US1> = v16.map(|x| v17(x));
        let v20: US1 = US1::US1_None;
        let v21: US1 = v20.clone();
        let v22: US1 = v18.unwrap_or(v21);
        match v22 {
            US1::US1_None => {
                0u64
            }
            US1::US1_Some(v23) => {
                let v23: u64 = v23;
                v23
            }
        }
    }
}
fn status_of_2(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = summary_marker_3();
    let v2: i32 = v0.get(0i32 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + 0i32 as usize) as i32).unwrap_or(-1);
    let v3: bool = v2 < 0i32;
    if v3 {
        let mut v4: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
        US0::US0_RemoteUnreachable(v4)
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v7: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v6)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v8: bool = v7 < 0i32;
        let v10: i32 = if v8 {
            let mut v9: i32 = (v0.clone().len() as i32);
            v9
        } else {
            v7
        };
        let mut v11: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v10 as usize).unwrap_or(""));
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("windows"); } LIT.with(|lit| lit.clone()) };
        let v13: u64 = field_value_4(v11.clone(), v12);
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("free_mb"); } LIT.with(|lit| lit.clone()) };
        let mut v15: u64 = field_value_4(v11.clone(), v14);
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy_slots"); } LIT.with(|lit| lit.clone()) };
        let mut v17: u64 = field_value_4(v11, v16);
        US0::US0_RemoteReported(v13, v15, v17)
    }
}
fn ssh_arguments_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: Rc<str> = string_concat(&[&*v1, " window list"]);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-o"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BatchMode=yes"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ConnectTimeout=10"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v6));
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v7));
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v8));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v9));
    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v10));
    Rc::new(UH0::UH0_Cons(v3, v11))
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method9(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
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
fn remote_status_of_1(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let v2: Rc<UH0> = ssh_arguments_6(v0, v1);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v3)));
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = method7(v2, v5);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = method8(v6, v7);
    method9(v4.clone(), v8);
    let mut v9: Rc<std::io::Result<std::process::Output>> = Rc::new(v4.borrow_mut().output());
    let v10: bool = v9.is_ok();
    let (mut v16, mut v17, mut v18): (bool, i32, Rc<str>) = if v10 {
        let mut v11: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v9).as_ref().unwrap().stderr).as_ref());
        let v12: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v9).as_ref().unwrap().stdout).as_ref());
        let v13: Rc<str> = string_concat(&[&*v12, &*v11]);
        let v14: i32 = (&*v9).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v14, v13)
    } else {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v15)
    };
    let v20: bool = if v16 {
        let v19: bool = v17 == 0i32;
        v19
    } else {
        false
    };
    let v23: US2 = if v20 {
        US2::US2_Some(v18)
    } else {
        US2::US2_None
    };
    match v23 {
        US2::US2_None => {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh did not run"); } LIT.with(|lit| lit.clone()) };
            US0::US0_RemoteUnreachable(v26)
        }
        US2::US2_Some(v24) => {
            let v24: Rc<str> = v24;
            status_of_2(v24)
        }
    }
}
fn method12(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_11(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method12(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn status_line_10(mut v0: Rc<str>, mut v1: US0) -> Rc<str> {
    match v1 {
        US0::US0_RemoteReported(v2, v3, v4) => {
            let v2: u64 = v2;
            let v3: u64 = v3;
            let mut v4: u64 = v4;
            let v7: Rc<str> = format_real_11(v2);
            let v10: Rc<str> = format_real_11(v3);
            let v13: Rc<str> = format_real_11(v4);
            let v14: Rc<str> = string_concat(&["rig remote status ok machine=", &*v0, " windows=", &*v7, " free_mb=", &*v10, " heavy_slots=", &*v13]);
            v14
        }
        US0::US0_RemoteUnreachable(v15) => {
            let mut v15: Rc<str> = v15;
            let mut v18: Rc<str> = string_concat(&["rig remote status unreachable machine=", &*v0, " reason=", &*v15]);
            v18
        }
    }
}
fn jobs_arguments_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: Rc<str> = string_concat(&[&*v1, " job list"]);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-o"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BatchMode=yes"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ConnectTimeout=10"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v6));
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v7));
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v8));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v9));
    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v10));
    Rc::new(UH0::UH0_Cons(v3, v11))
}
fn disk_arguments_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: Rc<str> = string_concat(&[&*v1, " machine health --gate --min-disk-gb 0 --max-busy-pct 1000"]);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-o"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BatchMode=yes"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ConnectTimeout=10"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v6));
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v7));
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v8));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v9));
    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v10));
    Rc::new(UH0::UH0_Cons(v3, v11))
}
fn disk_line_15(mut v0: Rc<str>, mut v1: US3) -> Rc<str> {
    let v2: u64 = 10u64;
    let v3: Rc<str> = format_real_11(v2);
    let mut v4: Rc<str> = string_concat(&[" floor_gb=", &*v3]);
    match v1 {
        US3::US3_DiskAbove(v5) => {
            let v5: u64 = v5;
            let v8: Rc<str> = format_real_11(v5);
            let v10: Rc<str> = string_concat(&["rig remote status disk machine=", &*v0, " free_gb=", &*v8, &*v4]);
            v10
        }
        US3::US3_DiskBelow(v11) => {
            let mut v11: u64 = v11;
            let v14: Rc<str> = format_real_11(v11);
            let mut v17: Rc<str> = string_concat(&["rig remote status DISK-LOW machine=", &*v0, " free_gb=", &*v14, &*v4, " (prune finished staged workspaces before a heavy job)"]);
            v17
        }
        US3::US3_DiskUnknown(v18) => {
            let mut v18: Rc<str> = v18;
            let v21: Rc<str> = string_concat(&["rig remote status disk unknown machine=", &*v0, " reason=", &*v18]);
            v21
        }
    }
}
fn disk_of_16(mut v0: Rc<str>) -> US3 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig machine health gate="); } LIT.with(|lit| lit.clone()) };
    let v2: i32 = v0.get(0i32 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + 0i32 as usize) as i32).unwrap_or(-1);
    let v3: bool = v2 < 0i32;
    let v7: Rc<str> = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let v5: i32 = (v0.clone().len() as i32);
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v5 as usize).unwrap_or(""));
        v6
    };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disk"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = rig_remote_activity::rig_remote_field_text(&v7, &v8);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gb"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v11));
    let v13: Option<u64> = (v12).trim().parse::<u64>().ok();
    let v14: Option<u64> = method5(v13);
    let mut v15: Rc<dyn Fn((u64)) -> US1> = closure1();
    let mut v16: Option<US1> = v14.map(|x| v15(x));
    let mut v18: US1 = US1::US1_None;
    let v19: US1 = v18.clone();
    let v20: US1 = v16.unwrap_or(v19);
    match v20 {
        US1::US1_None => {
            let v26: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            US3::US3_DiskUnknown(v26)
        }
        US1::US1_Some(v21) => {
            let v21: u64 = v21;
            let v22: bool = v21 < 10u64;
            if v22 {
                US3::US3_DiskBelow(v21)
            } else {
                US3::US3_DiskAbove(v21)
            }
        }
    }
}
fn remote_status_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: US0 = remote_status_of_1(v0.clone(), v1.clone());
    let v3: Rc<str> = status_line_10(v0.clone(), v2.clone());
    println!("{}", v3);
    let mut v4: Rc<UH0> = jobs_arguments_13(v0.clone(), v1.clone());
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v5)));
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = method7(v4, v7);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH0> = method8(v8, v9);
    method9(v6.clone(), v10);
    let mut v11: Rc<std::io::Result<std::process::Output>> = Rc::new(v6.borrow_mut().output());
    let v12: bool = v11.is_ok();
    let (mut v18, mut v19, mut v20): (bool, i32, Rc<str>) = if v12 {
        let v13: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v11).as_ref().unwrap().stderr).as_ref());
        let v14: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v11).as_ref().unwrap().stdout).as_ref());
        let mut v15: Rc<str> = string_concat(&[&*v14, &*v13]);
        let mut v16: i32 = (&*v11).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v16, v15)
    } else {
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v17)
    };
    let v22: bool = if v18 {
        let v21: bool = v19 == 0i32;
        v21
    } else {
        false
    };
    let v25: US2 = if v22 {
        US2::US2_Some(v20)
    } else {
        US2::US2_None
    };
    match v25 {
        US2::US2_None => {
            let v29: Rc<str> = string_concat(&["rig remote status jobs unknown machine=", &*v0, " reason=ssh did not run"]);
            println!("{}", v29);
            ()
        }
        US2::US2_Some(v26) => {
            let v26: Rc<str> = v26;
            let v27: Rc<str> = rig_remote_activity::rig_remote_activity_text(&v0, &v26);
            println!("{}", v27);
            ()
        }
    };
    let v30: Rc<UH0> = disk_arguments_14(v0.clone(), v1);
    let mut v31: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v5)));
    let v32: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v33: Rc<UH0> = method7(v30, v32);
    let v34: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v35: Rc<UH0> = method8(v33, v34);
    method9(v31.clone(), v35);
    let mut v36: Rc<std::io::Result<std::process::Output>> = Rc::new(v31.borrow_mut().output());
    let v37: bool = v36.is_ok();
    let (mut v43, mut v44, mut v45): (bool, i32, Rc<str>) = if v37 {
        let v38: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v36).as_ref().unwrap().stderr).as_ref());
        let v39: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v36).as_ref().unwrap().stdout).as_ref());
        let v40: Rc<str> = string_concat(&[&*v39, &*v38]);
        let v41: i32 = (&*v36).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v41, v40)
    } else {
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v42)
    };
    let v47: bool = if v43 {
        let v46: bool = v44 == 0i32;
        v46
    } else {
        false
    };
    let v50: US2 = if v47 {
        US2::US2_Some(v45)
    } else {
        US2::US2_None
    };
    match v50 {
        US2::US2_None => {
            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh did not run"); } LIT.with(|lit| lit.clone()) };
            let v55: US3 = US3::US3_DiskUnknown(v54);
            let v56: Rc<str> = disk_line_15(v0, v55);
            println!("{}", v56);
            ()
        }
        US2::US2_Some(v51) => {
            let v51: Rc<str> = v51;
            let v52: US3 = disk_of_16(v51);
            let v53: Rc<str> = disk_line_15(v0, v52);
            println!("{}", v53);
            ()
        }
    };
    match v2 {
        US0::US0_RemoteReported(v57, v58, v59) => {
            let v57: u64 = v57;
            let v58: u64 = v58;
            let v59: u64 = v59;
            0i32
        }
        US0::US0_RemoteUnreachable(v60) => {
            let v60: Rc<str> = v60;
            1i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        remote_status_run_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_remote_status_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
use rig_remote_activity::SpiralShape_97df63ebabc3 as Mut0;
