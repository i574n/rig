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
    UH0_Cons(bool, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<UH2>),
}
#[derive(Clone)]
enum US1 {
    US1_Direct(Rc<str>),
    US1_Jump(Rc<UH2>, Rc<str>),
    US1_Reverse(Rc<str>, i32),
}
#[derive(Clone)]
enum US2 {
    US2_WindowsHost,
    US2_LinuxHost,
}
#[derive(Clone)]
enum US0 {
    US0_NetworkRow(Rc<str>, i32, i32),
    US0_MachineRow(Rc<str>, Rc<str>, bool, US1, US2),
    US0_UnreadRow(Rc<str>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US0, Rc<UH1>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(i32),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Some(US1),
    US4_None,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH2> }
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, i32, i32, Rc<UH3>),
}
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(Rc<str>, Rc<str>, bool, US1, US2, Rc<UH4>),
}
#[derive(Clone)]
enum UH5 {
    UH5_Nil,
    UH5_Cons(Rc<str>, Rc<str>, bool, Rc<str>, Rc<UH5>),
}
#[derive(Clone)]
enum UH6 {
    UH6_Nil,
    UH6_Cons(Rc<UH2>, Rc<UH6>),
}
fn get_environment_variable_2(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn config_dir_1() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RIG_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
    let v1: Rc<str> = get_environment_variable_2(v0);
    let v2: bool = &*v1 != "";
    let v5: Rc<str> = if v2 {
        v1
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
        get_environment_variable_2(v3)
    };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("APPDATA"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = get_environment_variable_2(v6);
    let v8: bool = &*v5 != "";
    if v8 {
        v5
    } else {
        let mut v9: bool = &*v7 != "";
        if v9 {
            let v10: Rc<str> = string_concat(&[&*v7, "/kit"]);
            v10
        } else {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HOME"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = get_environment_variable_2(v11);
            let v13: Rc<str> = string_concat(&[&*v12, "/.config/kit"]);
            v13
        }
    }
}
fn file_exists_3(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method4(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method7(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: US0 = v2;
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
fn minute_of_10(mut v0: Rc<str>) -> US3 {
    let v1: Rc<RefCell<Vec<i32>>> = rig_net_fleet_file::rig_net_clock_minutes(&v0);
    let v2: i32 = (v1.clone().borrow().len() as i32);
    let v3: bool = v2 == 1i32;
    if v3 {
        let v4: i32 = v1.borrow()[0i32 as usize];
        US3::US3_Some(v4)
    } else {
        US3::US3_None
    }
}
fn network_row_9(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<Rc<str>>>>) -> US0 {
    let v2: Rc<str> = v1.borrow()[1i32 as usize].clone();
    let v3: US3 = minute_of_10(v2);
    let v4: Rc<str> = v1.borrow()[2i32 as usize].clone();
    let v5: US3 = minute_of_10(v4);
    match v3 {
        US3::US3_Some(v6) => {
            let mut v6: i32 = v6;
            match v5 {
                US3::US3_Some(v7) => {
                    let v7: i32 = v7;
                    let v8: Rc<str> = v1.borrow()[0i32 as usize].clone();
                    US0::US0_NetworkRow(v8, v6, v7)
                }
                _ => {
                    US0::US0_UnreadRow(v0)
                }
            }
        }
        _ => {
            US0::US0_UnreadRow(v0)
        }
    }
}
fn method14(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method15(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH2> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH2::UH2_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method15(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH2> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH2> = Rc::new(UH2::UH2_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH2> = v9.borrow().l1.clone();
        let mut v17: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        method14(v16, v17)
    }
}
fn method16(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH2> = method16(v3, v1);
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH2::UH2_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method17(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure1() -> Rc<dyn Fn((i32)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US3> = Rc::new(move |mut v0: (i32)| -> US3 {
        let v1: i32 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn link_of_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US4 {
    let v3: bool = &*v0 == "direct";
    if v3 {
        let v4: US1 = US1::US1_Direct(v1);
        US4::US4_Some(v4)
    } else {
        let mut v6: bool = &*v0 == "jump";
        if v6 {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<UH2> = split_list_13(v7, v2);
            let mut v9: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v10: Rc<UH2> = method16(v8, v9);
            let v11: US1 = US1::US1_Jump(v10, v1);
            US4::US4_Some(v11)
        } else {
            let v13: bool = &*v0 == "reverse";
            if v13 {
                let v14: Option<i32> = (v2).trim().parse::<i32>().ok();
                let v15: Option<i32> = method17(v14);
                let mut v16: Rc<dyn Fn((i32)) -> US3> = closure1();
                let mut v17: Option<US3> = v15.map(|x| v16(x));
                let v19: US3 = US3::US3_None;
                let v20: US3 = v19.clone();
                let v21: US3 = v17.unwrap_or(v20);
                let v24: i32 = match v21 {
                    US3::US3_None => {
                        -1i32
                    }
                    US3::US3_Some(v22) => {
                        let v22: i32 = v22;
                        v22
                    }
                };
                let v25: bool = v24 > 0i32;
                if v25 {
                    let v26: US1 = US1::US1_Reverse(v1, v24);
                    US4::US4_Some(v26)
                } else {
                    US4::US4_None
                }
            } else {
                US4::US4_None
            }
        }
    }
}
fn machine_row_11(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<Rc<str>>>>) -> US0 {
    let v2: Rc<str> = v1.borrow()[2i32 as usize].clone();
    let v3: Rc<str> = v1.borrow()[6i32 as usize].clone();
    let v4: Rc<str> = v1.borrow()[3i32 as usize].clone();
    let v5: Rc<str> = v1.borrow()[4i32 as usize].clone();
    let mut v6: Rc<str> = v1.borrow()[5i32 as usize].clone();
    let v7: US4 = link_of_12(v4, v5, v6);
    match v7 {
        US4::US4_Some(v8) => {
            let v8: US1 = v8;
            let mut v9: bool = &*v2 == "hub";
            let v11: bool = if v9 {
                true
            } else {
                let v10: bool = &*v2 == "member";
                v10
            };
            let v15: bool = if v11 {
                let v12: bool = &*v3 == "windows";
                if v12 {
                    true
                } else {
                    let v13: bool = &*v3 == "linux";
                    v13
                }
            } else {
                false
            };
            if v15 {
                let mut v16: bool = &*v3 == "windows";
                let v19: US2 = if v16 {
                    US2::US2_WindowsHost
                } else {
                    US2::US2_LinuxHost
                };
                let v20: Rc<str> = v1.borrow()[0i32 as usize].clone();
                let v21: Rc<str> = v1.borrow()[1i32 as usize].clone();
                US0::US0_MachineRow(v20, v21, v9, v8, v19)
            } else {
                US0::US0_UnreadRow(v0)
            }
        }
        _ => {
            US0::US0_UnreadRow(v0)
        }
    }
}
fn row_of_fleet_line_8(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<Rc<str>>>>, mut v2: Rc<str>) -> US0 {
    let v3: bool = &*v0 == "network";
    if v3 {
        network_row_9(v2, v1)
    } else {
        let v5: bool = &*v0 == "machine";
        if v5 {
            machine_row_11(v2, v1)
        } else {
            US0::US0_UnreadRow(v2)
        }
    }
}
fn rows_from_6(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            return method7(v2, v5);
        } else {
            let v7: i32 = v1.wrapping_add(1i32);
            let (mut v8, mut v9, mut v10): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = v0.borrow()[v1 as usize].clone();
            let v11: US0 = row_of_fleet_line_8(v8, &v9, v10);
            let v12: Rc<UH1> = Rc::new(UH1::UH1_Cons(v11, v2));
            (v1, v2) = (v7, v12);
            continue;
        }
    }
}
fn fleet_rows_5(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> = rig_net_fleet_file::rig_net_fleet_lines(&v0);
    let v2: i32 = 0i32;
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    rows_from_6(v1, v2, v3)
}
fn method19(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: i32 = v4;
                let v5: Rc<UH3> = v5;
                let mut v6: Rc<UH3> = Rc::new(UH3::UH3_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn method20(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> Rc<UH4> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH4::UH4_Cons(v2, v3, v4, v5, v6, v7) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: bool = v4;
                let v5: US1 = v5;
                let mut v6: US2 = v6;
                let v7: Rc<UH4> = v7;
                let v8: Rc<UH4> = Rc::new(UH4::UH4_Cons(v2, v3, v4, v5, v6, v1));
                (v0, v1) = (v7, v8);
                continue;
            }
            UH4::UH4_Nil => {
                return v1;
            }
        }
    }
}
fn partitioned_18(mut v0: Rc<UH1>, mut v1: Rc<UH3>, mut v2: Rc<UH4>, mut v3: Rc<UH2>) -> (Rc<UH3>, Rc<UH4>, Rc<UH2>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v10, v11) => {
                let v10: US0 = v10;
                let v11: Rc<UH1> = v11;
                match v10 {
                    US0::US0_MachineRow(v19, v20, v21, v22, v23) => {
                        let v19: Rc<str> = v19;
                        let v20: Rc<str> = v20;
                        let v21: bool = v21;
                        let v22: US1 = v22;
                        let v23: US2 = v23;
                        let v24: Rc<UH4> = Rc::new(UH4::UH4_Cons(v19, v20, v21, v22, v23, v2));
                        (v0, v2) = (v11, v24);
                        continue;
                    }
                    US0::US0_NetworkRow(v12, v13, v14) => {
                        let v12: Rc<str> = v12;
                        let v13: i32 = v13;
                        let v14: i32 = v14;
                        let v15: Rc<UH3> = Rc::new(UH3::UH3_Cons(v12, v13, v14, v1));
                        (v0, v1) = (v11, v15);
                        continue;
                    }
                    US0::US0_UnreadRow(v28) => {
                        let v28: Rc<str> = v28;
                        let v29: Rc<UH2> = Rc::new(UH2::UH2_Cons(v28, v3));
                        (v0, v3) = (v11, v29);
                        continue;
                    }
                }
            }
            UH1::UH1_Nil => {
                let v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH3> = method19(v1, v4);
                let mut v6: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
                let v7: Rc<UH4> = method20(v2, v6);
                let v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                let mut v9: Rc<UH2> = method14(v3, v8);
                return (v5, v7, v9);
            }
        }
    }
}
fn method22(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let mut v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH2> = Rc::new(UH2::UH2_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method23(mut v0: Rc<UH3>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2;
            let v3: i32 = v3;
            let v4: i32 = v4;
            let v5: Rc<UH3> = v5;
            let mut v6: Rc<UH2> = method23(v5, v1);
            Rc::new(UH2::UH2_Cons(v2, v6))
        }
        UH3::UH3_Nil => {
            v1
        }
    }
}
fn method24(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH2>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH2> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn reach_word_26(mut v0: US1) -> Rc<str> {
    match v0 {
        US1::US1_Direct(v1) => {
            let v1: Rc<str> = v1;
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("direct"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US1::US1_Jump(v3, v4) => {
            let v3: Rc<UH2> = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("jump"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US1::US1_Reverse(v6, v7) => {
            let mut v6: Rc<str> = v6;
            let v7: i32 = v7;
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reverse"); } LIT.with(|lit| lit.clone()) };
            v8
        }
    }
}
fn method25(mut v0: Rc<UH4>, mut v1: Rc<UH5>) -> Rc<UH5> {
    match Rc::unwrap_or_clone(v0) {
        UH4::UH4_Cons(v2, v3, v4, v5, v6, v7) => {
            let v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: bool = v4;
            let v5: US1 = v5;
            let mut v6: US2 = v6;
            let v7: Rc<UH4> = v7;
            let v8: Rc<UH5> = method25(v7, v1);
            let mut v9: Rc<str> = reach_word_26(v5);
            Rc::new(UH5::UH5_Cons(v2, v3, v4, v9, v8))
        }
        UH4::UH4_Nil => {
            v1
        }
    }
}
fn method27(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, bool, Rc<str>)>>>, mut v1: Rc<UH5>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH5::UH5_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: bool = v5;
                let mut v6: Rc<str> = v6;
                let v7: Rc<UH5> = v7;
                v0.borrow_mut().push((v3, v4, v5, v6));
                let v8: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v7, v8);
                continue;
            }
            UH5::UH5_Nil => {
                return v2;
            }
        }
    }
}
fn problems_of_21(mut v0: Rc<UH3>, mut v1: Rc<UH4>) -> Rc<UH2> {
    let v2: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH2> = method23(v0, v2);
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v5: i32 = 0i32;
    let mut v6: i32 = method24(v4.clone(), v3, v5);
    let v7: Rc<Vec<Rc<str>>> = Rc::new(v4.borrow().clone());
    let v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    let mut v9: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH5> = method25(v1, v9);
    let v11: Rc<RefCell<Vec<(Rc<str>, Rc<str>, bool, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v12: i32 = 0i32;
    let v13: i32 = method27(v11.clone(), v10, v12);
    let v14: Rc<Vec<(Rc<str>, Rc<str>, bool, Rc<str>)>> = Rc::new(v11.borrow().clone());
    let v15: Rc<RefCell<Vec<(Rc<str>, Rc<str>, bool, Rc<str>)>>> = Rc::new(RefCell::new((v14).as_ref().clone()));
    let mut v16: Rc<RefCell<Vec<Rc<str>>>> = rig_net_fleet_rules::rig_net_fleet_problem_lines(v8, v15);
    let mut v17: Rc<Vec<Rc<str>>> = Rc::new((v16).borrow().clone());
    let v18: usize = (v17).len();
    let v19: i32 = ((v18) as i32);
    let v20: i32 = v19.wrapping_sub(1i32);
    let v21: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method22(v17, v20, v21)
}
fn method28(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH2> = method28(v3, v1);
            let v5: Rc<str> = string_concat(&["problem: ", &*v2]);
            Rc::new(UH2::UH2_Cons(v5, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method29(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH2> = method29(v3, v1);
            let v5: Rc<str> = string_concat(&["unread row: ", &*v2]);
            Rc::new(UH2::UH2_Cons(v5, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method30(mut v0: Rc<UH3>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: i32 = v4;
                let v5: Rc<UH3> = v5;
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn method32(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_31(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method32(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method33(mut v0: Rc<UH4>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH4::UH4_Cons(v2, v3, v4, v5, v6, v7) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: bool = v4;
                let v5: US1 = v5;
                let mut v6: US2 = v6;
                let v7: Rc<UH4> = v7;
                let v8: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v7, v8);
                continue;
            }
            UH4::UH4_Nil => {
                return v1;
            }
        }
    }
}
fn method34(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method37(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH2> = method37(v3, v1);
            Rc::new(UH2::UH2_Cons(v2, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method36(mut v0: Rc<UH6>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH6::UH6_Cons(v2, v3) => {
            let v2: Rc<UH2> = v2;
            let v3: Rc<UH6> = v3;
            let v4: Rc<UH2> = method36(v3, v1);
            method37(v2, v4)
        }
        UH6::UH6_Nil => {
            v1
        }
    }
}
fn hhmm_39(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = rig_net_fleet_file::rig_net_clock_text(v0);
    v1
}
fn method40(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH4>, mut v4: Rc<UH4>) -> Rc<UH4> {
    match Rc::unwrap_or_clone(v3) {
        UH4::UH4_Cons(v5, v6, v7, v8, v9, v10) => {
            let v5: Rc<str> = v5;
            let mut v6: Rc<str> = v6;
            let v7: bool = v7;
            let v8: US1 = v8;
            let mut v9: US2 = v9;
            let v10: Rc<UH4> = v10;
            let v11: Rc<UH4> = method40(v0.clone(), v1, v2, v10, v4);
            let v12: bool = &*v6 == &*v0;
            if v12 {
                Rc::new(UH4::UH4_Cons(v5, v6, v7, v8, v9, v11))
            } else {
                v11
            }
        }
        UH4::UH4_Nil => {
            v4
        }
    }
}
fn link_text_43(mut v0: US1) -> Rc<str> {
    match v0 {
        US1::US1_Direct(v1) => {
            let v1: Rc<str> = v1;
            let v2: Rc<str> = string_concat(&["direct ", &*v1]);
            v2
        }
        US1::US1_Jump(v3, v4) => {
            let v3: Rc<UH2> = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let mut v6: i32 = 0i32;
            let v7: i32 = method24(v5.clone(), v3, v6);
            let v8: Rc<Vec<Rc<str>>> = Rc::new(v5.borrow().clone());
            let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
            let v14: Rc<str> = string_concat(&["jump ", &*v11, " ", &*v4]);
            v14
        }
        US1::US1_Reverse(v15, v16) => {
            let v15: Rc<str> = v15;
            let mut v16: i32 = v16;
            let v19: Rc<str> = format_real_31(v16);
            let v20: Rc<str> = string_concat(&["reverse ", &*v15, ":", &*v19]);
            v20
        }
    }
}
fn machine_line_42(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool, mut v3: US1, mut v4: US2) -> Rc<str> {
    let v7: Rc<str> = if v2 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hub"); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("member"); } LIT.with(|lit| lit.clone()) };
        v6
    };
    let v11: Rc<str> = match v4 {
        US2::US2_LinuxHost => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("linux"); } LIT.with(|lit| lit.clone()) };
            v9
        }
        US2::US2_WindowsHost => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("windows"); } LIT.with(|lit| lit.clone()) };
            v8
        }
    };
    let v18: Rc<str> = link_text_43(v3);
    let v21: Rc<str> = string_concat(&["  ", &*v0, "  ", &*v1, "  ", &*v7, "  ", &*v18, "  ", &*v11]);
    v21
}
fn method41(mut v0: Rc<UH4>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH4::UH4_Cons(v2, v3, v4, v5, v6, v7) => {
            let v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: bool = v4;
            let v5: US1 = v5;
            let mut v6: US2 = v6;
            let v7: Rc<UH4> = v7;
            let v8: Rc<UH2> = method41(v7, v1);
            let mut v9: Rc<str> = machine_line_42(v2, v3, v4, v5, v6);
            Rc::new(UH2::UH2_Cons(v9, v8))
        }
        UH4::UH4_Nil => {
            v1
        }
    }
}
fn method38(mut v0: Rc<UH4>, mut v1: Rc<UH3>, mut v2: Rc<UH6>) -> Rc<UH6> {
    match Rc::unwrap_or_clone(v1) {
        UH3::UH3_Cons(v3, v4, v5, v6) => {
            let v3: Rc<str> = v3;
            let v4: i32 = v4;
            let v5: i32 = v5;
            let mut v6: Rc<UH3> = v6;
            let v7: Rc<UH6> = method38(v0.clone(), v6, v2);
            let v10: Rc<str> = hhmm_39(v4);
            let v12: Rc<str> = hhmm_39(v5);
            let v14: Rc<str> = string_concat(&["network ", &*v3, "  harvest ", &*v10, "-", &*v12]);
            let v15: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
            let mut v16: Rc<UH4> = method40(v3, v4, v5, v0, v15);
            let mut v17: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v18: Rc<UH2> = method41(v16, v17);
            let v19: Rc<UH2> = Rc::new(UH2::UH2_Cons(v14, v18));
            Rc::new(UH6::UH6_Cons(v19, v7))
        }
        UH3::UH3_Nil => {
            v2
        }
    }
}
fn fleet_lines_35(mut v0: Rc<UH3>, mut v1: Rc<UH4>) -> Rc<UH2> {
    let v2: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH6> = method38(v1, v0, v2);
    let v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method36(v3, v4)
}
fn method45(mut v0: bool, mut v1: Rc<UH2>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH2::UH2_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH2> = v4;
            let v5: Rc<UH0> = method45(v0, v4, v2);
            Rc::new(UH0::UH0_Cons(v0, v3, v5))
        }
        UH2::UH2_Nil => {
            v2
        }
    }
}
fn report_of_44(mut v0: bool, mut v1: Rc<UH2>) -> Rc<UH0> {
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method45(v0, v1, v2)
}
fn method46(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4) => {
            let v2: bool = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method46(v4, v1);
            Rc::new(UH0::UH0_Cons(v2, v3, v5))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method47(mut v0: Rc<RefCell<Vec<(bool, Rc<str>)>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5) => {
                let v3: bool = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                v0.borrow_mut().push((v3, v4));
                let mut v6: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn fleet_report_0() -> Rc<RefCell<Vec<(bool, Rc<str>)>>> {
    let mut v0: Rc<str> = config_dir_1();
    let v1: Rc<str> = string_concat(&[&*v0, "/fleet.spi"]);
    let v2: bool = file_exists_3(v1.clone());
    let v3: bool = v2 == false;
    let v65: Rc<UH0> = if v3 {
        let v4: Rc<str> = string_concat(&["rig error: machine fleet: no fleet file at ", &*v1]);
        let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(true, v4, v5))
    } else {
        let v8: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v1);
        let v10: Vec<u8> = v8.unwrap();
        let v11: Vec<u8> = method4(v10);
        let v13: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v11);
        let v15: std::string::String = v13.unwrap();
        let mut v17: Rc<str> = Rc::<str>::from(String::as_str(&v15));
        let v18: Rc<UH1> = fleet_rows_5(v17);
        let v19: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let v20: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
        let v21: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let (mut v22, mut v23, mut v24): (Rc<UH3>, Rc<UH4>, Rc<UH2>) = partitioned_18(v18, v19, v20, v21);
        let v25: Rc<UH2> = problems_of_21(v22.clone(), v23.clone());
        let v26: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let v27: Rc<UH2> = method28(v25, v26);
        let v28: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let v29: Rc<UH2> = method29(v24.clone(), v28);
        let v30: i32 = 0i32;
        let v31: i32 = method30(v22.clone(), v30);
        let v32: Rc<str> = format_real_31(v31);
        let v34: i32 = 0i32;
        let v35: i32 = method33(v23.clone(), v34);
        let v36: Rc<str> = format_real_31(v35);
        let v38: i32 = 0i32;
        let v39: i32 = method34(v27.clone(), v38);
        let v40: Rc<str> = format_real_31(v39);
        let v42: i32 = 0i32;
        let v43: i32 = method34(v24, v42);
        let v44: Rc<str> = format_real_31(v43);
        let v51: Rc<str> = string_concat(&["rig machine fleet ok networks=", &*v32, " machines=", &*v36, " problems=", &*v40, " unread=", &*v44, " file=", &*v1]);
        let v52: bool = false;
        let v53: Rc<UH2> = fleet_lines_35(v22, v23);
        let v54: Rc<UH0> = report_of_44(v52, v53);
        let v55: bool = true;
        let v56: Rc<UH0> = report_of_44(v55, v27);
        let v57: bool = true;
        let v58: Rc<UH0> = report_of_44(v57, v29);
        let v59: bool = false;
        let v60: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v61: Rc<UH0> = Rc::new(UH0::UH0_Cons(v59, v51, v60));
        let v62: Rc<UH0> = method46(v58, v61);
        let v63: Rc<UH0> = method46(v56, v62);
        method46(v54, v63)
    };
    let v66: Rc<RefCell<Vec<(bool, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v67: i32 = 0i32;
    let v68: i32 = method47(v66.clone(), v65, v67);
    let v69: Rc<Vec<(bool, Rc<str>)>> = Rc::new(v66.borrow().clone());
    let v70: Rc<RefCell<Vec<(bool, Rc<str>)>>> = Rc::new(RefCell::new((v69).as_ref().clone()));
    v70
}
fn closure0() -> Rc<dyn Fn() -> Rc<RefCell<Vec<(bool, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<RefCell<Vec<(bool, Rc<str>)>>>> = Rc::new(move || -> Rc<RefCell<Vec<(bool, Rc<str>)>>> {
        fleet_report_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_net_fleet_report() -> Rc<RefCell<Vec<(bool, Rc<str>)>>> {
    closure0()()
}
use rig_net_fleet_file::SpiralShape_97df63ebabc3 as Mut1;
pub type SpiralShape_0408665020a9 = Mut0;
