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
pub enum wait_kind {
    wait_kind_WaitsTrain(Rc<str>),
    wait_kind_WaitsItem(Rc<str>),
    wait_kind_WaitsLane(Rc<str>),
    wait_kind_WaitsUser(Rc<str>),
    wait_kind_ResumeWhen(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, wait_kind, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, wait_kind),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(wait_kind),
    US2_None,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>),
    US3_None,
}
#[derive(Clone)]
enum US5 {
    US5_WaitSet(Rc<str>, wait_kind),
    US5_WaitCleared(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>, US5),
    US4_None,
}
#[derive(Clone)]
enum US6 {
    US6_WaitsWritten(i32),
    US6_WaitsRefused(Rc<str>),
}
#[derive(Clone)]
enum US7 {
    US7_Replaced,
    US7_TemporaryWriteFailed,
    US7_RenameFailed,
    US7_RenameFailedTemporaryKept,
    US7_ReadbackMismatch,
    US7_ChangedSinceRead,
}
pub struct Mut1 { pub l0: Rc<str> }
impl wait_kind {
    pub fn tag(&self) -> i32 {
        match self {
            wait_kind::wait_kind_WaitsTrain(..) => 0,
            wait_kind::wait_kind_WaitsItem(..) => 1,
            wait_kind::wait_kind_WaitsLane(..) => 2,
            wait_kind::wait_kind_WaitsUser(..) => 3,
            wait_kind::wait_kind_ResumeWhen(..) => 4,
        }
    }
}
impl Default for wait_kind {
    fn default() -> Self {
        wait_kind::wait_kind_WaitsTrain({ thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) })
    }
}
fn method4(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn nth_quoted_5(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v3: Option<usize> = v0.find(&*v2);
        let mut v4: Option<usize> = method4(v3);
        let mut v5: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v6: Option<i32> = v4.map(|x| v5(x));
        let v8: i32 = v6.unwrap_or(-1i32);
        let mut v9: bool = v8 < 0i32;
        let v19: Rc<str> = if v9 {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v10
        } else {
            let v11: i32 = v8.wrapping_add(1i32);
            let v12: i32 = (v0.clone().len() as i32);
            let v13: i32 = v12.wrapping_sub(v8);
            let v14: i32 = v13.wrapping_sub(1i32);
            let v15: usize = ((v11) as usize);
            let v16: i32 = v11.wrapping_add(v14);
            let v17: usize = ((v16) as usize);
            let v18: Rc<str> = Rc::<str>::from(&(v0)[v15..v17]);
            v18
        };
        let v20: Option<usize> = v19.find(&*v2);
        let v21: Option<usize> = method4(v20);
        let mut v22: Option<i32> = v21.map(|x| v5(x));
        let v24: i32 = v22.unwrap_or(-1i32);
        let v26: bool = if v9 {
            true
        } else {
            let v25: bool = v24 < 0i32;
            v25
        };
        if v26 {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v27;
        } else {
            let v28: bool = v1 == 0i32;
            if v28 {
                let v29: usize = ((0i32) as usize);
                let v30: usize = ((v24) as usize);
                let v31: Rc<str> = Rc::<str>::from(&(v19)[v29..v30]);
                return v31;
            } else {
                let v32: i32 = v24.wrapping_add(1i32);
                let v33: i32 = (v19.clone().len() as i32);
                let v34: i32 = v33.wrapping_sub(v24);
                let v35: i32 = v34.wrapping_sub(1i32);
                let v36: usize = ((v32) as usize);
                let v37: i32 = v32.wrapping_add(v35);
                let v38: usize = ((v37) as usize);
                let v39: Rc<str> = Rc::<str>::from(&(v19)[v36..v38]);
                let v40: i32 = v1.wrapping_sub(1i32);
                (v0, v1) = (v39, v40);
                continue;
            }
        }
    }
}
fn kind_of_6(mut v0: Rc<str>, mut v1: Rc<str>) -> US2 {
    let mut v2: bool = &*v1 == "";
    if v2 {
        US2::US2_None
    } else {
        let mut v4: bool = &*v0 == "train";
        if v4 {
            let mut v5: wait_kind = wait_kind::wait_kind_WaitsTrain(v1);
            US2::US2_Some(v5)
        } else {
            let v7: bool = &*v0 == "item";
            if v7 {
                let v8: wait_kind = wait_kind::wait_kind_WaitsItem(v1);
                US2::US2_Some(v8)
            } else {
                let v10: bool = &*v0 == "lane";
                if v10 {
                    let v11: wait_kind = wait_kind::wait_kind_WaitsLane(v1);
                    US2::US2_Some(v11)
                } else {
                    let v13: bool = &*v0 == "user";
                    if v13 {
                        let v14: wait_kind = wait_kind::wait_kind_WaitsUser(v1);
                        US2::US2_Some(v14)
                    } else {
                        let v16: bool = &*v0 == "when";
                        if v16 {
                            let v17: wait_kind = wait_kind::wait_kind_ResumeWhen(v1);
                            US2::US2_Some(v17)
                        } else {
                            US2::US2_None
                        }
                    }
                }
            }
        }
    }
}
fn wait_of_line_3(mut v0: Rc<str>) -> US1 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Waits (\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method4(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        US1::US1_None
    } else {
        let v10: i32 = v7.wrapping_add(3i32);
        let v11: i32 = (v0.clone().len() as i32);
        let v12: i32 = v11.wrapping_sub(v7);
        let v13: i32 = v12.wrapping_sub(3i32);
        let v14: usize = ((v10) as usize);
        let v15: i32 = v10.wrapping_add(v13);
        let v16: usize = ((v15) as usize);
        let v17: Rc<str> = Rc::<str>::from(&(v0)[v14..v16]);
        let v18: i32 = 0i32;
        let v19: Rc<str> = nth_quoted_5(v17.clone(), v18);
        let v20: i32 = 1i32;
        let v21: Rc<str> = nth_quoted_5(v17.clone(), v20);
        let mut v22: i32 = 2i32;
        let v23: Rc<str> = nth_quoted_5(v17, v22);
        let v24: US2 = kind_of_6(v21, v23);
        match v24 {
            US2::US2_Some(v25) => {
                let v25: wait_kind = v25;
                let v26: bool = &*v19 != "";
                if v26 {
                    US1::US1_Some(v19, v25)
                } else {
                    US1::US1_None
                }
            }
            _ => {
                US1::US1_None
            }
        }
    }
}
fn method7(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let v3: wait_kind = v3;
            let mut v4: Rc<UH1> = v4;
            let mut v5: Rc<UH1> = method7(v4, v1);
            Rc::new(UH1::UH1_Cons(v2, v3, v5))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: US1 = wait_of_line_3(v2);
                let v11: Rc<UH1> = match v4 {
                    US1::US1_None => {
                        v1
                    }
                    US1::US1_Some(v5, v6) => {
                        let mut v5: Rc<str> = v5;
                        let mut v6: wait_kind = v6;
                        let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v8: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v6, v7));
                        method7(v1, v8)
                    }
                };
                (v0, v1) = (v3, v11);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
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
        method9(v16, v17)
    }
}
fn waits_of_text_1(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_8(v1, v4);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method2(v5, v6)
}
fn method11(mut v0: Rc<RefCell<Vec<(Rc<str>, wait_kind)>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5) => {
                let v3: Rc<str> = v3;
                let mut v4: wait_kind = v4;
                let mut v5: Rc<UH1> = v5;
                v0.borrow_mut().push((v3, v4));
                let mut v6: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v5, v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn waiting_of_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, wait_kind)>>> {
    let v1: Rc<UH1> = waits_of_text_1(v0);
    let mut v2: Rc<RefCell<Vec<(Rc<str>, wait_kind)>>> = Rc::new(RefCell::new(Vec::new()));
    let v3: i32 = 0i32;
    let mut v4: i32 = method11(v2.clone(), v1, v3);
    let mut v5: Rc<Vec<(Rc<str>, wait_kind)>> = Rc::new(v2.borrow().clone());
    let mut v6: Rc<RefCell<Vec<(Rc<str>, wait_kind)>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    v6
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, wait_kind)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, wait_kind)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, wait_kind)>>> {
        waiting_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn command_line_argument_count_14() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_15(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_13(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_14();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US3 = if v4 {
        let mut v5: Rc<str> = command_line_argument_15(v0);
        US3::US3_Some(v5)
    } else {
        US3::US3_None
    };
    match v8 {
        US3::US3_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US3::US3_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_13(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn kind_of_flag_16(mut v0: Rc<str>) -> US2 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method4(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 <= 0i32;
    if v8 {
        US2::US2_None
    } else {
        let v10: usize = ((0i32) as usize);
        let v11: usize = ((v7) as usize);
        let v12: Rc<str> = Rc::<str>::from(&(v0)[v10..v11]);
        let v13: i32 = v7.wrapping_add(1i32);
        let v14: i32 = (v0.clone().len() as i32);
        let v15: i32 = v14.wrapping_sub(v7);
        let v16: i32 = v15.wrapping_sub(1i32);
        let v17: usize = ((v13) as usize);
        let v18: i32 = v13.wrapping_add(v16);
        let v19: usize = ((v18) as usize);
        let v20: Rc<str> = Rc::<str>::from(&(v0)[v17..v19]);
        kind_of_6(v12, v20)
    }
}
fn file_exists_19(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn method20(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn text_of_18(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = file_exists_19(v0.clone());
    if v1 {
        let v3: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let mut v5: Vec<u8> = v3.unwrap();
        let mut v6: Vec<u8> = method20(v5);
        let v8: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v6);
        let v10: std::string::String = v8.unwrap();
        let v12: Rc<str> = Rc::<str>::from(String::as_str(&v10));
        v12
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    }
}
fn change_item_21(mut v0: US5) -> Rc<str> {
    match v0 {
        US5::US5_WaitCleared(v3) => {
            let v3: Rc<str> = v3;
            v3
        }
        US5::US5_WaitSet(v1, v2) => {
            let v1: Rc<str> = v1;
            let mut v2: wait_kind = v2;
            v1
        }
    }
}
fn method23(mut v0: US5, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5) => {
            let v3: Rc<str> = v3;
            let mut v4: wait_kind = v4;
            let mut v5: Rc<UH1> = v5;
            let mut v6: Rc<UH1> = method23(v0.clone(), v5, v2);
            let v7: Rc<str> = change_item_21(v0);
            let v8: bool = &*v3 != &*v7;
            if v8 {
                Rc::new(UH1::UH1_Cons(v3, v4, v6))
            } else {
                v6
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn waits_changed_22(mut v0: Rc<UH1>, mut v1: US5) -> Rc<UH1> {
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH1> = method23(v1.clone(), v0, v2);
    match v1 {
        US5::US5_WaitCleared(v9) => {
            let mut v9: Rc<str> = v9;
            v3
        }
        US5::US5_WaitSet(v4, v5) => {
            let mut v4: Rc<str> = v4;
            let mut v5: wait_kind = v5;
            let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v4, v5, v6));
            method7(v3, v7)
        }
    }
}
fn quoted_27(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v5: Rc<str> = string_concat(&["\"", &*v3, "\""]);
    v5
}
fn kind_word_28(mut v0: wait_kind) -> Rc<str> {
    match v0 {
        wait_kind::wait_kind_ResumeWhen(v9) => {
            let mut v9: Rc<str> = v9;
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("when"); } LIT.with(|lit| lit.clone()) };
            v10
        }
        wait_kind::wait_kind_WaitsItem(v3) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("item"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        wait_kind::wait_kind_WaitsLane(v5) => {
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lane"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        wait_kind::wait_kind_WaitsTrain(v1) => {
            let v1: Rc<str> = v1;
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("train"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        wait_kind::wait_kind_WaitsUser(v7) => {
            let v7: Rc<str> = v7;
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user"); } LIT.with(|lit| lit.clone()) };
            v8
        }
    }
}
fn kind_value_29(mut v0: wait_kind) -> Rc<str> {
    match v0 {
        wait_kind::wait_kind_ResumeWhen(v5) => {
            let mut v5: Rc<str> = v5;
            v5
        }
        wait_kind::wait_kind_WaitsItem(v2) => {
            let mut v2: Rc<str> = v2;
            v2
        }
        wait_kind::wait_kind_WaitsLane(v3) => {
            let v3: Rc<str> = v3;
            v3
        }
        wait_kind::wait_kind_WaitsTrain(v1) => {
            let v1: Rc<str> = v1;
            v1
        }
        wait_kind::wait_kind_WaitsUser(v4) => {
            let mut v4: Rc<str> = v4;
            v4
        }
    }
}
fn slug_of_30(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v3));
    v6
}
fn row_of_wait_26(mut v0: Rc<str>, mut v1: wait_kind) -> Rc<str> {
    let mut v2: Rc<str> = quoted_27(v0.clone());
    let mut v4: Rc<str> = kind_word_28(v1.clone());
    let mut v5: Rc<str> = quoted_27(v4);
    let v8: Rc<str> = kind_value_29(v1);
    let mut v9: Rc<str> = quoted_27(v8);
    let v11: Rc<str> = slug_of_30(v0);
    let v15: Rc<str> = string_concat(&["inl wait_of_", &*v11, " () = Waits (", &*v2, ", ", &*v5, ", ", &*v9, ")"]);
    v15
}
fn method25(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let v3: wait_kind = v3;
            let mut v4: Rc<UH1> = v4;
            let mut v5: Rc<UH0> = method25(v4, v1);
            let mut v6: Rc<str> = row_of_wait_26(v2, v3);
            let v7: Rc<str> = string_concat(&[&*v6, "\n"]);
            Rc::new(UH0::UH0_Cons(v7, v5))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method31(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn waits_text_24(mut v0: Rc<UH1>) -> Rc<str> {
    let v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH0> = method25(v0, v1);
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = method31(v3.clone(), v2, v4);
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from((*v7.borrow()).join(&*v8));
    let v10: Rc<str> = string_concat(&["///- --no-respell\nunion task_wait =\n    | Waits :: string * string * string -> task_wait\n", &*v9]);
    v10
}
fn renamed_within_33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: bool = std::fs::rename(&*v1, &*v0).is_ok();
        if v3 {
            return true;
        } else {
            let mut v4: bool = v2 <= 1i32;
            if v4 {
                return false;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(50u64));
                let mut v5: i32 = v2.wrapping_sub(1i32);
                v2 = v5;
                continue;
            }
        }
    }
}
fn replaced_32(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = string_concat(&[&*v0, ".pending"]);
    let v3: bool = std::fs::write(&*v2, &*v1).is_ok();
    let v16: US7 = if v3 {
        let mut v4: i32 = 40i32;
        let mut v5: bool = renamed_within_33(v0.clone(), v2.clone(), v4);
        if v5 {
            let mut v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
            if v6 {
                US7::US7_Replaced
            } else {
                US7::US7_ReadbackMismatch
            }
        } else {
            let v10: bool = std::fs::remove_file(&*v2).is_ok() || !std::path::Path::new(&*v2).exists();
            if v10 {
                US7::US7_RenameFailed
            } else {
                US7::US7_RenameFailedTemporaryKept
            }
        }
    } else {
        US7::US7_TemporaryWriteFailed
    };
    match v16 {
        US7::US7_Replaced => {
            true
        }
        _ => {
            false
        }
    }
}
fn module_added_34(mut v0: Rc<str>) -> US3 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n    waits\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.contains(&*v1);
    let mut v5: bool = if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n    waits"); } LIT.with(|lit| lit.clone()) };
        let mut v4: bool = v0.ends_with(&*v3);
        v4
    };
    if v5 {
        US3::US3_Some(v0)
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v0.contains(&*v7);
        if v8 {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n    waits\n"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v9));
            US3::US3_Some(v10)
        } else {
            US3::US3_None
        }
    }
}
fn method35(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let v3: wait_kind = v3;
                let mut v4: Rc<UH1> = v4;
                let mut v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn waits_written_17(mut v0: Rc<str>, mut v1: US5) -> US6 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<str> = string_concat(&[&*v4, "/state"]);
    let mut v6: Rc<str> = string_concat(&[&*v5, "/agile.spi"]);
    let v7: Rc<str> = text_of_18(v6);
    let v8: Rc<str> = string_concat(&[&*v5, "/waits.spi"]);
    let mut v9: Rc<str> = text_of_18(v8.clone());
    let v10: Rc<str> = change_item_21(v1.clone());
    let v11: Rc<UH1> = waits_of_text_1(v9.clone());
    let v12: Rc<UH1> = waits_changed_22(v11, v1);
    let v13: Rc<str> = string_concat(&[&*v5, "/package.spiproj"]);
    let v14: bool = &*v7 == "";
    if v14 {
        let v15: Rc<str> = string_concat(&["no backlog state at ", &*v5]);
        US6::US6_WaitsRefused(v15)
    } else {
        let v18: Rc<str> = string_concat(&["Task (\"", &*v10, "\""]);
        let v19: bool = v7.contains(&*v18);
        let v20: bool = v19 == false;
        if v20 {
            let v21: Rc<str> = string_concat(&["no task ", &*v10]);
            US6::US6_WaitsRefused(v21)
        } else {
            let v23: Rc<str> = text_of_18(v8.clone());
            let v24: bool = &*v23 != &*v9;
            if v24 {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/waits.spi changed since it was read; run again"); } LIT.with(|lit| lit.clone()) };
                US6::US6_WaitsRefused(v25)
            } else {
                let v27: Rc<str> = waits_text_24(v12.clone());
                let v28: bool = replaced_32(v8, v27);
                let v29: bool = v28 == false;
                if v29 {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/waits.spi could not be written"); } LIT.with(|lit| lit.clone()) };
                    US6::US6_WaitsRefused(v30)
                } else {
                    let v32: Rc<str> = text_of_18(v13.clone());
                    let v33: US3 = module_added_34(v32);
                    match v33 {
                        US3::US3_None => {
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj has no modules section to list waits in"); } LIT.with(|lit| lit.clone()) };
                            US6::US6_WaitsRefused(v34)
                        }
                        US3::US3_Some(v36) => {
                            let v36: Rc<str> = v36;
                            let v37: Rc<str> = text_of_18(v13.clone());
                            let v38: bool = &*v36 == &*v37;
                            let v40: bool = if v38 {
                                true
                            } else {
                                replaced_32(v13, v36)
                            };
                            if v40 {
                                let v41: i32 = 0i32;
                                let v42: i32 = method35(v12, v41);
                                US6::US6_WaitsWritten(v42)
                            } else {
                                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj not updated"); } LIT.with(|lit| lit.clone()) };
                                US6::US6_WaitsRefused(v44)
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method37(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_36(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method37(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn wait_run_12() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_13(v0);
    let v42: US4 = match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let mut v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH0::UH0_Cons(v8, v9) => {
                                    let v8: Rc<str> = v8;
                                    let mut v9: Rc<UH0> = v9;
                                    match Rc::unwrap_or_clone(v9) {
                                        UH0::UH0_Cons(v10, v11) => {
                                            let v10: Rc<str> = v10;
                                            let v11: Rc<UH0> = v11;
                                            match Rc::unwrap_or_clone(v11) {
                                                UH0::UH0_Cons(v12, v13) => {
                                                    let v12: Rc<str> = v12;
                                                    let v13: Rc<UH0> = v13;
                                                    match Rc::unwrap_or_clone(v13) {
                                                        UH0::UH0_Nil => {
                                                            let v14: bool = &*v10 == "--wait";
                                                            let v16: bool = if v14 {
                                                                let v15: bool = &*v12 == "--clear";
                                                                v15
                                                            } else {
                                                                false
                                                            };
                                                            if v16 {
                                                                let v17: US5 = US5::US5_WaitCleared(v8);
                                                                US4::US4_Some(v6, v17)
                                                            } else {
                                                                if v14 {
                                                                    let v19: US2 = kind_of_flag_16(v12);
                                                                    match v19 {
                                                                        US2::US2_None => {
                                                                            US4::US4_None
                                                                        }
                                                                        US2::US2_Some(v20) => {
                                                                            let v20: wait_kind = v20;
                                                                            let v21: US5 = US5::US5_WaitSet(v8, v20);
                                                                            US4::US4_Some(v6, v21)
                                                                        }
                                                                    }
                                                                } else {
                                                                    US4::US4_None
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            US4::US4_None
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    US4::US4_None
                                                }
                                            }
                                        }
                                        _ => {
                                            US4::US4_None
                                        }
                                    }
                                }
                                _ => {
                                    US4::US4_None
                                }
                            }
                        }
                        _ => {
                            US4::US4_None
                        }
                    }
                }
                _ => {
                    US4::US4_None
                }
            }
        }
        _ => {
            US4::US4_None
        }
    };
    match v42 {
        US4::US4_None => {
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile edit <root> <ID> --wait <train|item|lane|user|when>:<value>|--clear"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v43);
            2i32
        }
        US4::US4_Some(v44, v45) => {
            let v44: Rc<str> = v44;
            let v45: US5 = v45;
            let v46: US6 = waits_written_17(v44, v45.clone());
            match v46 {
                US6::US6_WaitsRefused(v53) => {
                    let v53: Rc<str> = v53;
                    let v54: Rc<str> = string_concat(&["kit error: agile edit --wait: ", &*v53]);
                    eprintln!("{}", v54);
                    2i32
                }
                US6::US6_WaitsWritten(v47) => {
                    let v47: i32 = v47;
                    let v48: Rc<str> = change_item_21(v45);
                    let v51: Rc<str> = format_real_36(v47);
                    let v52: Rc<str> = string_concat(&["kit agile edit ok id=", &*v48, " waits=", &*v51]);
                    println!("{}", v52);
                    0i32
                }
            }
        }
    }
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        wait_run_12()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_waits_of(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, wait_kind)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_agile_wait_run() -> i32 {
    closure2()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
