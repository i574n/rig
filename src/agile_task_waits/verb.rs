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
    US7_StateFile(Rc<str>),
    US7_HistoryFile(Rc<str>),
    US7_ArchiveFile(Rc<str>),
    US7_OtherFile,
}
#[derive(Clone)]
enum US9 {
    US9_HeadInl,
    US9_HeadUnion,
    US9_HeadOpen,
    US9_HeadType,
}
#[derive(Clone)]
enum US8 {
    US8_KeywordHead(US9),
    US8_SymbolHead(US9),
    US8_OtherLine,
}
#[derive(Clone)]
enum US10 {
    US10_SpiprojProject,
    US10_TomlProject,
}
#[derive(Clone)]
enum US11 {
    US11_WordsRoot,
    US11_SymbolsRoot,
}
#[derive(Clone)]
enum US12 {
    US12_Replaced,
    US12_TemporaryWriteFailed,
    US12_RenameFailed,
    US12_RenameFailedTemporaryKept,
    US12_ReadbackMismatch,
    US12_ChangedSinceRead,
}
#[derive(Clone)]
enum US13 {
    US13_SplitProject(Rc<UH0>, Rc<UH0>, Rc<UH0>),
    US13_NoModules,
}
#[derive(Clone)]
enum US14 {
    US14_ModuleListed,
    US14_ModuleAdded(Rc<str>),
    US14_NoModulesSection,
}
#[derive(Clone)]
enum US15 {
    US15_Some(i32),
    US15_None,
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
        let mut v1: usize = (v0);
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
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v10
        } else {
            let v11: i32 = v8.wrapping_add(1i32);
            let mut v12: i32 = (v0.clone().len() as i32);
            let mut v13: i32 = v12.wrapping_sub(v8);
            let v14: i32 = v13.wrapping_sub(1i32);
            let v15: usize = ((v11) as usize);
            let v16: i32 = v11.wrapping_add(v14);
            let mut v17: usize = ((v16) as usize);
            let v18: Rc<str> = Rc::<str>::from(&(v0)[v15..v17]);
            v18
        };
        let v20: Option<usize> = v19.find(&*v2);
        let v21: Option<usize> = method4(v20);
        let mut v22: Option<i32> = v21.map(|x| v5(x));
        let mut v24: i32 = v22.unwrap_or(-1i32);
        let v26: bool = if v9 {
            true
        } else {
            let mut v25: bool = v24 < 0i32;
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
                let mut v40: i32 = v1.wrapping_sub(1i32);
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
                let mut v10: bool = &*v0 == "lane";
                if v10 {
                    let v11: wait_kind = wait_kind::wait_kind_WaitsLane(v1);
                    US2::US2_Some(v11)
                } else {
                    let mut v13: bool = &*v0 == "user";
                    if v13 {
                        let v14: wait_kind = wait_kind::wait_kind_WaitsUser(v1);
                        US2::US2_Some(v14)
                    } else {
                        let v16: bool = &*v0 == "when";
                        if v16 {
                            let mut v17: wait_kind = wait_kind::wait_kind_ResumeWhen(v1);
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
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Waits (\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method4(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        US1::US1_None
    } else {
        let mut v10: i32 = v7.wrapping_add(3i32);
        let v11: i32 = (v0.clone().len() as i32);
        let mut v12: i32 = v11.wrapping_sub(v7);
        let mut v13: i32 = v12.wrapping_sub(3i32);
        let v14: usize = ((v10) as usize);
        let v15: i32 = v10.wrapping_add(v13);
        let v16: usize = ((v15) as usize);
        let mut v17: Rc<str> = Rc::<str>::from(&(v0)[v14..v16]);
        let v18: i32 = 0i32;
        let v19: Rc<str> = nth_quoted_5(v17.clone(), v18);
        let v20: i32 = 1i32;
        let v21: Rc<str> = nth_quoted_5(v17.clone(), v20);
        let mut v22: i32 = 2i32;
        let v23: Rc<str> = nth_quoted_5(v17, v22);
        let mut v24: US2 = kind_of_6(v21, v23);
        match v24 {
            US2::US2_Some(v25) => {
                let mut v25: wait_kind = v25;
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
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method9(v16, v17)
    }
}
fn waits_of_text_1(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
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
    let mut v1: Rc<UH1> = waits_of_text_1(v0);
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
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_15(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_13(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
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
            let mut v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let mut v12: Rc<UH0> = arguments_from_13(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn kind_of_flag_16(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method4(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 <= 0i32;
    if v8 {
        US2::US2_None
    } else {
        let mut v10: usize = ((0i32) as usize);
        let v11: usize = ((v7) as usize);
        let mut v12: Rc<str> = Rc::<str>::from(&(v0)[v10..v11]);
        let mut v13: i32 = v7.wrapping_add(1i32);
        let v14: i32 = (v0.clone().len() as i32);
        let v15: i32 = v14.wrapping_sub(v7);
        let v16: i32 = v15.wrapping_sub(1i32);
        let mut v17: usize = ((v13) as usize);
        let v18: i32 = v13.wrapping_add(v16);
        let v19: usize = ((v18) as usize);
        let v20: Rc<str> = Rc::<str>::from(&(v0)[v17..v19]);
        kind_of_6(v12, v20)
    }
}
fn loop_19(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn closure3() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_20(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(6i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_21(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_22(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(8i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method23(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method23(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US8 = if v6 {
                let v7: US9 = US9::US9_HeadType;
                US8::US8_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US9 = US9::US9_HeadType;
                    US8::US8_SymbolHead(v11)
                } else {
                    US8::US8_OtherLine
                }
            };
            let v27: US8 = match &v15 {
                US8::US8_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US9 = US9::US9_HeadInl;
                        US8::US8_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let mut v22: US9 = US9::US9_HeadInl;
                            US8::US8_SymbolHead(v22)
                        } else {
                            US8::US8_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US8 = match &v27 {
                US8::US8_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US9 = US9::US9_HeadUnion;
                        US8::US8_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US9 = US9::US9_HeadUnion;
                            US8::US8_SymbolHead(v34)
                        } else {
                            US8::US8_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US8 = match &v39 {
                US8::US8_OtherLine => {
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US9 = US9::US9_HeadOpen;
                        US8::US8_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US9 = US9::US9_HeadOpen;
                            US8::US8_SymbolHead(v46)
                        } else {
                            US8::US8_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US8::US8_SymbolHead(v52) => {
                    let v52: US9 = v52;
                    let v60: Rc<str> = match &v52 {
                        US9::US9_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US9::US9_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US9::US9_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US9::US9_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US9::US9_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US9::US9_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US9::US9_HeadType => {
                            v5
                        }
                        US9::US9_HeadUnion => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                    };
                    let v69: i32 = (v2.clone().len() as i32);
                    let v70: i32 = v69.wrapping_sub(v61);
                    let v71: usize = ((v61) as usize);
                    let v72: i32 = v61.wrapping_add(v70);
                    let v73: usize = ((v72) as usize);
                    let v74: Rc<str> = Rc::<str>::from(&(v2)[v71..v73]);
                    let v75: Rc<str> = string_concat(&[&*v68, &*v74]);
                    v75
                }
                _ => {
                    v2
                }
            };
            Rc::new(UH0::UH0_Cons(v76, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method24(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn text_of_18(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = std::path::Path::new(&*v0).is_file();
    if v1 {
        let (mut v2, mut v3): (bool, Rc<str>) = match std::fs::read(&*v0) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
        if v2 {
            let mut v4: Rc<str> = string_concat(&["rig state read failed: ", &*v0]);
            std::panic::panic_any::<std::string::String>(format!("{}", v4))
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
            let mut v9: Rc<str> = loop_19(v8);
            let mut v10: Option<usize> = v9.rfind(&*v7);
            let v11: Option<usize> = method4(v10);
            let mut v12: Rc<dyn Fn((usize)) -> i32> = closure3();
            let mut v13: Option<i32> = v11.map(|x| v12(x));
            let v15: i32 = v13.unwrap_or(-1i32);
            let v16: bool = v15 < 0i32;
            let v20: bool = if v16 {
                true
            } else {
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v18: bool = v9.ends_with(&*v17);
                let v19: bool = v18 == false;
                v19
            };
            let v41: US7 = if v20 {
                US7::US7_OtherFile
            } else {
                let mut v22: usize = ((0i32) as usize);
                let v23: usize = ((v15) as usize);
                let mut v24: Rc<str> = Rc::<str>::from(&(v9)[v22..v23]);
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v26: bool = v24.ends_with(&*v25);
                if v26 {
                    let v27: Rc<str> = loop_20(v24);
                    US7::US7_StateFile(v27)
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v30: bool = v24.ends_with(&*v29);
                    if v30 {
                        let v31: Rc<str> = loop_21(v24);
                        US7::US7_HistoryFile(v31)
                    } else {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v34: bool = v24.ends_with(&*v33);
                        if v34 {
                            let v35: Rc<str> = loop_22(v24);
                            US7::US7_ArchiveFile(v35)
                        } else {
                            US7::US7_OtherFile
                        }
                    }
                }
            };
            match v41 {
                US7::US7_OtherFile => {
                    v3
                }
                _ => {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<UH0> = split_list_8(v42.clone(), v3);
                    let v44: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v45: Rc<UH0> = method23(v43, v44);
                    let v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let mut v47: i32 = 0i32;
                    let v48: i32 = method24(v46.clone(), v45, v47);
                    let v49: Rc<Vec<Rc<str>>> = Rc::new(v46.borrow().clone());
                    let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v49).as_ref().clone()));
                    let v51: Rc<str> = Rc::<str>::from((*v50.borrow()).join(&*v42));
                    v51
                }
            }
        }
    } else {
        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v54
    }
}
fn change_item_25(mut v0: US5) -> Rc<str> {
    match v0 {
        US5::US5_WaitCleared(v3) => {
            let v3: Rc<str> = v3;
            v3
        }
        US5::US5_WaitSet(v1, v2) => {
            let mut v1: Rc<str> = v1;
            let mut v2: wait_kind = v2;
            v1
        }
    }
}
fn method27(mut v0: US5, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5) => {
            let v3: Rc<str> = v3;
            let mut v4: wait_kind = v4;
            let mut v5: Rc<UH1> = v5;
            let mut v6: Rc<UH1> = method27(v0.clone(), v5, v2);
            let v7: Rc<str> = change_item_25(v0);
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
fn waits_changed_26(mut v0: Rc<UH1>, mut v1: US5) -> Rc<UH1> {
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH1> = method27(v1.clone(), v0, v2);
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
fn quoted_31(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v5: Rc<str> = string_concat(&["\"", &*v3, "\""]);
    v5
}
fn kind_word_32(mut v0: wait_kind) -> Rc<str> {
    match v0 {
        wait_kind::wait_kind_ResumeWhen(v9) => {
            let mut v9: Rc<str> = v9;
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("when"); } LIT.with(|lit| lit.clone()) };
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
            let mut v1: Rc<str> = v1;
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
fn kind_value_33(mut v0: wait_kind) -> Rc<str> {
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
            let mut v1: Rc<str> = v1;
            v1
        }
        wait_kind::wait_kind_WaitsUser(v4) => {
            let mut v4: Rc<str> = v4;
            v4
        }
    }
}
fn slug_of_34(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v3));
    v6
}
fn row_of_wait_30(mut v0: Rc<str>, mut v1: wait_kind) -> Rc<str> {
    let mut v2: Rc<str> = quoted_31(v0.clone());
    let mut v4: Rc<str> = kind_word_32(v1.clone());
    let mut v5: Rc<str> = quoted_31(v4);
    let v8: Rc<str> = kind_value_33(v1);
    let mut v9: Rc<str> = quoted_31(v8);
    let v11: Rc<str> = slug_of_34(v0);
    let v15: Rc<str> = string_concat(&["inl wait_of_", &*v11, " () = Waits (", &*v2, ", ", &*v5, ", ", &*v9, ")"]);
    v15
}
fn method29(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let v3: wait_kind = v3;
            let mut v4: Rc<UH1> = v4;
            let mut v5: Rc<UH0> = method29(v4, v1);
            let mut v6: Rc<str> = row_of_wait_30(v2, v3);
            let v7: Rc<str> = string_concat(&[&*v6, "\n"]);
            Rc::new(UH0::UH0_Cons(v7, v5))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn waits_text_28(mut v0: Rc<UH1>) -> Rc<str> {
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH0> = method29(v0, v1);
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = method24(v3.clone(), v2, v4);
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from((*v7.borrow()).join(&*v8));
    let mut v10: Rc<str> = string_concat(&["///- --no-respell\nunion task_wait =\n    | Waits :: string * string * string -> task_wait\n", &*v9]);
    v10
}
fn method36(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method36(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            let v15: US8 = if v6 {
                let v7: US9 = US9::US9_HeadType;
                US8::US8_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US9 = US9::US9_HeadType;
                    US8::US8_SymbolHead(v11)
                } else {
                    US8::US8_OtherLine
                }
            };
            let v27: US8 = match &v15 {
                US8::US8_OtherLine => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US9 = US9::US9_HeadInl;
                        US8::US8_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let mut v22: US9 = US9::US9_HeadInl;
                            US8::US8_SymbolHead(v22)
                        } else {
                            US8::US8_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US8 = match &v27 {
                US8::US8_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US9 = US9::US9_HeadUnion;
                        US8::US8_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US9 = US9::US9_HeadUnion;
                            US8::US8_SymbolHead(v34)
                        } else {
                            US8::US8_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US8 = match &v39 {
                US8::US8_OtherLine => {
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US9 = US9::US9_HeadOpen;
                        US8::US8_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US9 = US9::US9_HeadOpen;
                            US8::US8_SymbolHead(v46)
                        } else {
                            US8::US8_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US8::US8_KeywordHead(v52) => {
                    let v52: US9 = v52;
                    let v59: Rc<str> = match &v52 {
                        US9::US9_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US9::US9_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US9::US9_HeadType => {
                            v5
                        }
                        US9::US9_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v60: i32 = (v59.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US9::US9_HeadInl => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v61
                        }
                        US9::US9_HeadOpen => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v63
                        }
                        US9::US9_HeadType => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US9::US9_HeadUnion => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                    };
                    let v69: i32 = (v2.clone().len() as i32);
                    let v70: i32 = v69.wrapping_sub(v60);
                    let v71: usize = ((v60) as usize);
                    let v72: i32 = v60.wrapping_add(v70);
                    let v73: usize = ((v72) as usize);
                    let v74: Rc<str> = Rc::<str>::from(&(v2)[v71..v73]);
                    let v75: Rc<str> = string_concat(&[&*v68, &*v74]);
                    v75
                }
                _ => {
                    v2
                }
            };
            Rc::new(UH0::UH0_Cons(v76, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn renamed_within_37(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn replaced_35(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = string_concat(&[&*v0, ".pending"]);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let mut v6: Rc<str> = loop_19(v5);
    let v7: Option<usize> = v6.rfind(&*v4);
    let v8: Option<usize> = method4(v7);
    let mut v9: Rc<dyn Fn((usize)) -> i32> = closure3();
    let mut v10: Option<i32> = v8.map(|x| v9(x));
    let mut v12: i32 = v10.unwrap_or(-1i32);
    let mut v13: bool = v12 < 0i32;
    let mut v17: bool = if v13 {
        true
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v15: bool = v6.ends_with(&*v14);
        let v16: bool = v15 == false;
        v16
    };
    let v38: US7 = if v17 {
        US7::US7_OtherFile
    } else {
        let v19: usize = ((0i32) as usize);
        let v20: usize = ((v12) as usize);
        let v21: Rc<str> = Rc::<str>::from(&(v6)[v19..v20]);
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v23: bool = v21.ends_with(&*v22);
        if v23 {
            let mut v24: Rc<str> = loop_20(v21);
            US7::US7_StateFile(v24)
        } else {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
            let v27: bool = v21.ends_with(&*v26);
            if v27 {
                let v28: Rc<str> = loop_21(v21);
                US7::US7_HistoryFile(v28)
            } else {
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                let v31: bool = v21.ends_with(&*v30);
                if v31 {
                    let v32: Rc<str> = loop_22(v21);
                    US7::US7_ArchiveFile(v32)
                } else {
                    US7::US7_OtherFile
                }
            }
        }
    };
    let v61: US11 = match v38 {
        US7::US7_ArchiveFile(v51) => {
            let v51: Rc<str> = v51;
            let v52: Rc<str> = string_concat(&[&*v51, "/state/spiral.toml"]);
            let v53: bool = std::path::Path::new(&*v52).is_file();
            if v53 {
                US11::US11_SymbolsRoot
            } else {
                US11::US11_WordsRoot
            }
        }
        US7::US7_HistoryFile(v45) => {
            let v45: Rc<str> = v45;
            let v46: Rc<str> = string_concat(&[&*v45, "/state/spiral.toml"]);
            let mut v47: bool = std::path::Path::new(&*v46).is_file();
            if v47 {
                US11::US11_SymbolsRoot
            } else {
                US11::US11_WordsRoot
            }
        }
        US7::US7_OtherFile => {
            US11::US11_WordsRoot
        }
        US7::US7_StateFile(v39) => {
            let v39: Rc<str> = v39;
            let mut v40: Rc<str> = string_concat(&[&*v39, "/state/spiral.toml"]);
            let v41: bool = std::path::Path::new(&*v40).is_file();
            if v41 {
                US11::US11_SymbolsRoot
            } else {
                US11::US11_WordsRoot
            }
        }
    };
    let v73: Rc<str> = match v61 {
        US11::US11_SymbolsRoot => {
            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v63: Rc<UH0> = split_list_8(v62.clone(), v1.clone());
            let v64: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v65: Rc<UH0> = method36(v63, v64);
            let v66: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v67: i32 = 0i32;
            let v68: i32 = method24(v66.clone(), v65, v67);
            let v69: Rc<Vec<Rc<str>>> = Rc::new(v66.borrow().clone());
            let v70: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v69).as_ref().clone()));
            let v71: Rc<str> = Rc::<str>::from((*v70.borrow()).join(&*v62));
            v71
        }
        US11::US11_WordsRoot => {
            v1.clone()
        }
    };
    let v74: bool = std::fs::write(&*v2, &*v73).is_ok();
    let v155: US12 = if v74 {
        let v75: i32 = 40i32;
        let v76: bool = renamed_within_37(v0.clone(), v2.clone(), v75);
        if v76 {
            let v77: Rc<str> = Rc::<str>::from(v0.replace(&*v3, &*v4));
            let mut v78: Rc<str> = loop_19(v77);
            let v79: Option<usize> = v78.rfind(&*v4);
            let v80: Option<usize> = method4(v79);
            let mut v81: Option<i32> = v80.map(|x| v9(x));
            let v83: i32 = v81.unwrap_or(-1i32);
            let v84: bool = v83 < 0i32;
            let v88: bool = if v84 {
                true
            } else {
                let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v86: bool = v78.ends_with(&*v85);
                let v87: bool = v86 == false;
                v87
            };
            let v109: US7 = if v88 {
                US7::US7_OtherFile
            } else {
                let v90: usize = ((0i32) as usize);
                let v91: usize = ((v83) as usize);
                let v92: Rc<str> = Rc::<str>::from(&(v78)[v90..v91]);
                let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v94: bool = v92.ends_with(&*v93);
                if v94 {
                    let v95: Rc<str> = loop_20(v92);
                    US7::US7_StateFile(v95)
                } else {
                    let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v98: bool = v92.ends_with(&*v97);
                    if v98 {
                        let v99: Rc<str> = loop_21(v92);
                        US7::US7_HistoryFile(v99)
                    } else {
                        let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v102: bool = v92.ends_with(&*v101);
                        if v102 {
                            let v103: Rc<str> = loop_22(v92);
                            US7::US7_ArchiveFile(v103)
                        } else {
                            US7::US7_OtherFile
                        }
                    }
                }
            };
            let v132: US11 = match v109 {
                US7::US7_ArchiveFile(v122) => {
                    let v122: Rc<str> = v122;
                    let v123: Rc<str> = string_concat(&[&*v122, "/state/spiral.toml"]);
                    let v124: bool = std::path::Path::new(&*v123).is_file();
                    if v124 {
                        US11::US11_SymbolsRoot
                    } else {
                        US11::US11_WordsRoot
                    }
                }
                US7::US7_HistoryFile(v116) => {
                    let v116: Rc<str> = v116;
                    let v117: Rc<str> = string_concat(&[&*v116, "/state/spiral.toml"]);
                    let v118: bool = std::path::Path::new(&*v117).is_file();
                    if v118 {
                        US11::US11_SymbolsRoot
                    } else {
                        US11::US11_WordsRoot
                    }
                }
                US7::US7_OtherFile => {
                    US11::US11_WordsRoot
                }
                US7::US7_StateFile(v110) => {
                    let v110: Rc<str> = v110;
                    let v111: Rc<str> = string_concat(&[&*v110, "/state/spiral.toml"]);
                    let v112: bool = std::path::Path::new(&*v111).is_file();
                    if v112 {
                        US11::US11_SymbolsRoot
                    } else {
                        US11::US11_WordsRoot
                    }
                }
            };
            let v144: Rc<str> = match v132 {
                US11::US11_SymbolsRoot => {
                    let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v134: Rc<UH0> = split_list_8(v133.clone(), v1);
                    let v135: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v136: Rc<UH0> = method36(v134, v135);
                    let v137: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v138: i32 = 0i32;
                    let v139: i32 = method24(v137.clone(), v136, v138);
                    let v140: Rc<Vec<Rc<str>>> = Rc::new(v137.borrow().clone());
                    let v141: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v140).as_ref().clone()));
                    let v142: Rc<str> = Rc::<str>::from((*v141.borrow()).join(&*v133));
                    v142
                }
                US11::US11_WordsRoot => {
                    v1
                }
            };
            let v145: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v144);
            if v145 {
                US12::US12_Replaced
            } else {
                US12::US12_ReadbackMismatch
            }
        } else {
            let v149: bool = std::fs::remove_file(&*v2).is_ok() || !std::path::Path::new(&*v2).exists();
            if v149 {
                US12::US12_RenameFailed
            } else {
                US12::US12_RenameFailedTemporaryKept
            }
        }
    } else {
        US12::US12_TemporaryWriteFailed
    };
    match v155 {
        US12::US12_Replaced => {
            true
        }
        _ => {
            false
        }
    }
}
fn method40(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method41(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
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
fn method39(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method39(v3, v1);
            let mut v5: i32 = (v2.clone().len() as i32);
            let mut v6: i32 = 0i32;
            let v7: i32 = method40(v2.clone(), v5, v6);
            let v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2.clone(), v7 as i64, v8 as i64);
            let mut v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method41(v9.clone(), v10);
            let mut v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            let mut v13: bool = &*v12 != "";
            if v13 {
                Rc::new(UH0::UH0_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn section_lines_43(mut v0: US10, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> US13 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v8, v9) => {
                let v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH0> = v9.clone();
                let v26: bool = match &v0 {
                    US10::US10_SpiprojProject => {
                        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v11: bool = v8.starts_with(&*v10);
                        let v14: bool = if v11 {
                            true
                        } else {
                            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v13: bool = v8.starts_with(&*v12);
                            v13
                        };
                        let v15: bool = v14 == false;
                        v15
                    }
                    US10::US10_TomlProject => {
                        let v16: i32 = (v8.clone().len() as i32);
                        let mut v17: i32 = 0i32;
                        let v18: i32 = method40(v8.clone(), v16, v17);
                        let v19: i32 = v16.wrapping_sub(1i32);
                        let v20: Rc<str> = string_slice(&v8.clone(), v18 as i64, v19 as i64);
                        let v21: i32 = (v20.clone().len() as i32);
                        let mut v22: i32 = method41(v20.clone(), v21);
                        let v23: Rc<str> = string_slice(&v20, 0i32 as i64, v22 as i64);
                        let mut v24: bool = &*v23 == "'''";
                        v24
                    }
                };
                if v26 {
                    let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v28: Rc<UH0> = method9(v3, v27);
                    return US13::US13_SplitProject(v2, v28, v1);
                } else {
                    let v30: i32 = (v8.clone().len() as i32);
                    let v31: i32 = 0i32;
                    let v32: i32 = method40(v8.clone(), v30, v31);
                    let v33: i32 = v30.wrapping_sub(1i32);
                    let v34: Rc<str> = string_slice(&v8, v32 as i64, v33 as i64);
                    let v35: i32 = (v34.clone().len() as i32);
                    let v36: i32 = method41(v34.clone(), v35);
                    let v37: Rc<str> = string_slice(&v34, 0i32 as i64, v36 as i64);
                    let v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v37, v3));
                    (v1, v3) = (v9, v38);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v5: Rc<UH0> = method9(v3, v4);
                let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return US13::US13_SplitProject(v2, v5, v6);
            }
        }
    }
}
fn split_lines_42(mut v0: US10, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> US13 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: i32 = (v4.clone().len() as i32);
                let v7: i32 = 0i32;
                let v8: i32 = method40(v4.clone(), v6, v7);
                let mut v9: i32 = v6.wrapping_sub(1i32);
                let mut v10: Rc<str> = string_slice(&v4.clone(), v8 as i64, v9 as i64);
                let v11: i32 = (v10.clone().len() as i32);
                let mut v12: i32 = method41(v10.clone(), v11);
                let mut v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
                let mut v17: Rc<str> = match &v0 {
                    US10::US10_SpiprojProject => {
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                        v14
                    }
                    US10::US10_TomlProject => {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules = '''"); } LIT.with(|lit| lit.clone()) };
                        v15
                    }
                };
                let v18: bool = &*v13 == &*v17;
                if v18 {
                    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v19));
                    let v21: Rc<UH0> = method9(v2, v20);
                    let mut v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    return section_lines_43(v0, v5, v21, v22);
                } else {
                    let mut v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v2));
                    (v1, v2) = (v5, v24);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US13::US13_NoModules;
            }
        }
    }
}
fn closure4(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US15> {
    Rc::new(move |mut v2: Rc<UH0>| -> US15 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US15::US15_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US15> = method44(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US15::US15_None
            }
        }
    })
}
fn method44(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US15> {
    closure4(v0, v1)
}
fn method45(mut v0: US10, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let mut v5: Rc<UH0> = method45(v0.clone(), v4, v2);
            let mut v13: Rc<str> = match v0 {
                US10::US10_SpiprojProject => {
                    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                    v6.borrow_mut().push(v7);
                    v6.borrow_mut().push(v3);
                    let v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
                    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
                    v11
                }
                US10::US10_TomlProject => {
                    v3
                }
            };
            Rc::new(UH0::UH0_Cons(v13, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method46(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method46(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn module_added_38(mut v0: US10, mut v1: Rc<str>) -> US3 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waits"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v1.replace(&*v4, &*v5));
    let v7: Rc<UH0> = split_list_8(v3.clone(), v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method39(v7, v8);
    let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v11: US13 = split_lines_42(v0.clone(), v9, v10);
    let mut v47: US14 = match v11 {
        US13::US13_NoModules => {
            US14::US14_NoModulesSection
        }
        US13::US13_SplitProject(v13, v14, v15) => {
            let mut v13: Rc<UH0> = v13;
            let v14: Rc<UH0> = v14;
            let v15: Rc<UH0> = v15;
            let v16: i32 = 0i32;
            let mut v17: Rc<dyn Fn(Rc<UH0>) -> US15> = method44(v2.clone(), v16);
            let v18: US15 = v17(v14.clone());
            let v19: bool = match v18 {
                US15::US15_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v20: bool = v19 != true;
            if v20 {
                US14::US14_ModuleListed
            } else {
                let mut v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v23: Rc<UH0> = method45(v0.clone(), v14, v22);
                let v30: Rc<str> = match v0 {
                    US10::US10_SpiprojProject => {
                        let mut v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                        v24.borrow_mut().push(v25);
                        v24.borrow_mut().push(v2);
                        let v26: Rc<Vec<Rc<str>>> = Rc::new(v24.borrow().clone());
                        let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v26).as_ref().clone()));
                        let v28: Rc<str> = Rc::<str>::from((*v27.borrow()).join(&*v5));
                        v28
                    }
                    US10::US10_TomlProject => {
                        v2
                    }
                };
                let v31: Rc<UH0> = method46(v23, v15);
                let v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v30, v31));
                let v33: Rc<UH0> = method46(v13, v32);
                let v34: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v35: i32 = 0i32;
                let v36: i32 = method24(v34.clone(), v33, v35);
                let v37: Rc<Vec<Rc<str>>> = Rc::new(v34.borrow().clone());
                let v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v37).as_ref().clone()));
                let v39: Rc<str> = Rc::<str>::from((*v38.borrow()).join(&*v3));
                let mut v40: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v40.borrow_mut().push(v39);
                v40.borrow_mut().push(v3);
                let v41: Rc<Vec<Rc<str>>> = Rc::new(v40.borrow().clone());
                let v42: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v41).as_ref().clone()));
                let v43: Rc<str> = Rc::<str>::from((*v42.borrow()).join(&*v5));
                US14::US14_ModuleAdded(v43)
            }
        }
    };
    match v47 {
        US14::US14_ModuleAdded(v49) => {
            let v49: Rc<str> = v49;
            US3::US3_Some(v49)
        }
        US14::US14_ModuleListed => {
            US3::US3_Some(v1)
        }
        US14::US14_NoModulesSection => {
            US3::US3_None
        }
    }
}
fn method47(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
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
    let mut v10: Rc<str> = change_item_25(v1.clone());
    let v11: Rc<UH1> = waits_of_text_1(v9.clone());
    let mut v12: Rc<UH1> = waits_changed_26(v11, v1);
    let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v13.borrow_mut().push(v5.clone());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.toml"); } LIT.with(|lit| lit.clone()) };
    v13.borrow_mut().push(v14.clone());
    let v15: Rc<Vec<Rc<str>>> = Rc::new(v13.borrow().clone());
    let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
    let mut v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v3));
    let v18: bool = std::path::Path::new(&*v17).is_file();
    let v21: US10 = if v18 {
        US10::US10_TomlProject
    } else {
        US10::US10_SpiprojProject
    };
    let mut v24: Rc<str> = match v21 {
        US10::US10_SpiprojProject => {
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
            v22
        }
        US10::US10_TomlProject => {
            v14.clone()
        }
    };
    let mut v25: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v25.borrow_mut().push(v5.clone());
    v25.borrow_mut().push(v24);
    let v26: Rc<Vec<Rc<str>>> = Rc::new(v25.borrow().clone());
    let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v26).as_ref().clone()));
    let v28: Rc<str> = Rc::<str>::from((*v27.borrow()).join(&*v3));
    let v29: bool = &*v7 == "";
    if v29 {
        let v30: Rc<str> = string_concat(&["no backlog state at ", &*v5]);
        US6::US6_WaitsRefused(v30)
    } else {
        let v33: Rc<str> = string_concat(&["Task (\"", &*v10, "\""]);
        let v34: bool = v7.contains(&*v33);
        let v35: bool = v34 == false;
        if v35 {
            let v36: Rc<str> = string_concat(&["no task ", &*v10]);
            US6::US6_WaitsRefused(v36)
        } else {
            let v38: Rc<str> = text_of_18(v8.clone());
            let v39: bool = &*v38 != &*v9;
            if v39 {
                let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/waits.spi changed since it was read; run again"); } LIT.with(|lit| lit.clone()) };
                US6::US6_WaitsRefused(v40)
            } else {
                let v42: Rc<str> = waits_text_28(v12.clone());
                let v43: bool = replaced_35(v8, v42);
                let v44: bool = v43 == false;
                if v44 {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/waits.spi could not be written"); } LIT.with(|lit| lit.clone()) };
                    US6::US6_WaitsRefused(v45)
                } else {
                    let mut v47: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    v47.borrow_mut().push(v5);
                    v47.borrow_mut().push(v14);
                    let v48: Rc<Vec<Rc<str>>> = Rc::new(v47.borrow().clone());
                    let v49: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v48).as_ref().clone()));
                    let v50: Rc<str> = Rc::<str>::from((*v49.borrow()).join(&*v3));
                    let v51: bool = std::path::Path::new(&*v50).is_file();
                    let v54: US10 = if v51 {
                        US10::US10_TomlProject
                    } else {
                        US10::US10_SpiprojProject
                    };
                    let v55: Rc<str> = text_of_18(v28.clone());
                    let v56: US3 = module_added_38(v54, v55);
                    match v56 {
                        US3::US3_None => {
                            let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the state project has no modules section to list waits in"); } LIT.with(|lit| lit.clone()) };
                            US6::US6_WaitsRefused(v57)
                        }
                        US3::US3_Some(v59) => {
                            let v59: Rc<str> = v59;
                            let v60: Rc<str> = text_of_18(v28.clone());
                            let v61: bool = &*v59 == &*v60;
                            let v63: bool = if v61 {
                                true
                            } else {
                                replaced_35(v28, v59)
                            };
                            if v63 {
                                let v64: i32 = 0i32;
                                let v65: i32 = method47(v12, v64);
                                US6::US6_WaitsWritten(v65)
                            } else {
                                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the state project was not updated"); } LIT.with(|lit| lit.clone()) };
                                US6::US6_WaitsRefused(v67)
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method49(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_48(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method49(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn wait_run_12() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_13(v0);
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
                                            let mut v10: Rc<str> = v10;
                                            let v11: Rc<UH0> = v11;
                                            match Rc::unwrap_or_clone(v11) {
                                                UH0::UH0_Cons(v12, v13) => {
                                                    let mut v12: Rc<str> = v12;
                                                    let mut v13: Rc<UH0> = v13;
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
                                                                let mut v17: US5 = US5::US5_WaitCleared(v8);
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
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: usage: agile edit <root> <ID> --wait <train|item|lane|user|when>:<value>|--clear"); } LIT.with(|lit| lit.clone()) };
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
                    let v54: Rc<str> = string_concat(&["rig error: agile edit --wait: ", &*v53]);
                    eprintln!("{}", v54);
                    2i32
                }
                US6::US6_WaitsWritten(v47) => {
                    let mut v47: i32 = v47;
                    let v48: Rc<str> = change_item_25(v45);
                    let v51: Rc<str> = format_real_48(v47);
                    let v52: Rc<str> = string_concat(&["rig agile edit ok id=", &*v48, " waits=", &*v51]);
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
