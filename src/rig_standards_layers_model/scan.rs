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
    US1_Meta,
    US1_Core,
    US1_Prim,
    US1_Std,
    US1_Os,
    US1_Host(Rc<str>),
    US1_Ecosystem,
}
#[derive(Clone)]
enum US0 {
    US0_Some(US1),
    US0_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, US0, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum US2 {
    US2_Allowed,
    US2_UpwardImport,
    US2_CrossBackend,
}
fn method1(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: US0 = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method4(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closed_piece_6(mut v0: Rc<UH1>, mut v1: i32) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                match Rc::unwrap_or_clone(v3) {
                    UH1::UH1_Cons(v4, v5) => {
                        let v4: Rc<str> = v4;
                        let v5: Rc<UH1> = v5;
                        match Rc::unwrap_or_clone(v5) {
                            UH1::UH1_Cons(v6, v7) => {
                                let v6: Rc<str> = v6;
                                let mut v7: Rc<UH1> = v7;
                                let mut v8: bool = v1 == 0i32;
                                if v8 {
                                    return v4;
                                } else {
                                    let mut v9: Rc<UH1> = Rc::new(UH1::UH1_Cons(v6, v7));
                                    let v10: i32 = v1.wrapping_sub(1i32);
                                    (v0, v1) = (v9, v10);
                                    continue;
                                }
                            }
                            _ => {
                                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                return v13;
                            }
                        }
                    }
                    _ => {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        return v15;
                    }
                }
            }
            _ => {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v17;
            }
        }
    }
}
fn method8(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn method9(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method9(v7, &v9) {
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
        method8(v16, v17)
    }
}
fn nth_quoted_5(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH1> = split_list_7(v2, v0);
    closed_piece_6(v3, v1)
}
fn layer_named_10(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let v2: bool = &*v0 == "Meta";
    if v2 {
        let v3: US1 = US1::US1_Meta;
        US0::US0_Some(v3)
    } else {
        let v5: bool = &*v0 == "Core";
        if v5 {
            let v6: US1 = US1::US1_Core;
            US0::US0_Some(v6)
        } else {
            let mut v8: bool = &*v0 == "Prim";
            if v8 {
                let mut v9: US1 = US1::US1_Prim;
                US0::US0_Some(v9)
            } else {
                let v11: bool = &*v0 == "Std";
                if v11 {
                    let v12: US1 = US1::US1_Std;
                    US0::US0_Some(v12)
                } else {
                    let v14: bool = &*v0 == "Os";
                    if v14 {
                        let v15: US1 = US1::US1_Os;
                        US0::US0_Some(v15)
                    } else {
                        let v17: bool = &*v0 == "Host";
                        let v19: bool = if v17 {
                            let v18: bool = &*v1 != "";
                            v18
                        } else {
                            false
                        };
                        if v19 {
                            let v20: US1 = US1::US1_Host(v1);
                            US0::US0_Some(v20)
                        } else {
                            let v22: bool = &*v0 == "Ecosystem";
                            if v22 {
                                let v23: US1 = US1::US1_Ecosystem;
                                US0::US0_Some(v23)
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
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2;
            let v3: US0 = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            let v6: Rc<UH0> = method11(v5, v1);
            Rc::new(UH0::UH0_Cons(v2, v3, v4, v6))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method3(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = LayerRow ("); } LIT.with(|lit| lit.clone()) };
                let v5: Option<usize> = v2.find(&*v4);
                let v6: Option<usize> = method4(v5);
                let mut v7: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v8: Option<i32> = v6.map(|x| v7(x));
                let v10: i32 = v8.unwrap_or(-1i32);
                let v11: bool = v10 < 0i32;
                let v34: Rc<UH0> = if v11 {
                    v1
                } else {
                    let v12: i32 = v10.wrapping_add(13i32);
                    let v13: i32 = (v2.clone().len() as i32);
                    let v14: i32 = v13.wrapping_sub(v10);
                    let v15: i32 = v14.wrapping_sub(13i32);
                    let v16: usize = ((v12) as usize);
                    let v17: i32 = v12.wrapping_add(v15);
                    let v18: usize = ((v17) as usize);
                    let v19: Rc<str> = Rc::<str>::from(&(v2)[v16..v18]);
                    let v20: i32 = 0i32;
                    let v21: Rc<str> = nth_quoted_5(v19.clone(), v20);
                    let v22: i32 = 1i32;
                    let v23: Rc<str> = nth_quoted_5(v19.clone(), v22);
                    let v24: i32 = 2i32;
                    let v25: Rc<str> = nth_quoted_5(v19, v24);
                    let v26: US0 = layer_named_10(v23.clone(), v25);
                    let v27: bool = &*v21 == "";
                    let v29: bool = if v27 {
                        true
                    } else {
                        let v28: bool = &*v23 == "";
                        v28
                    };
                    if v29 {
                        v1
                    } else {
                        let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v31: Rc<UH0> = Rc::new(UH0::UH0_Cons(v21, v26, v23, v30));
                        method11(v1, v31)
                    }
                };
                (v0, v1) = (v3, v34);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn table_rows_2(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH1> = split_list_7(v1, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method3(v5, v6)
}
fn table_count_0(mut v0: Rc<str>) -> i32 {
    let v1: Rc<UH0> = table_rows_2(v0);
    let v2: i32 = 0i32;
    method1(v1, v2)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        table_count_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method14(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method14(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: US0 = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v14: Rc<UH1> = match v3 {
                    US0::US0_None => {
                        let mut v8: Rc<str> = string_concat(&["layers / table: unknown layer ", &*v4, " for ", &*v2]);
                        let mut v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v10: Rc<UH1> = Rc::new(UH1::UH1_Cons(v8, v9));
                        method14(v1, v10)
                    }
                    US0::US0_Some(v12) => {
                        let v12: US1 = v12;
                        v1
                    }
                };
                (v0, v1) = (v5, v14);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn table_problems_12(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v1: Rc<UH0> = table_rows_2(v0);
    let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH1> = method13(v1, v2);
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v5: i32 = 0i32;
    let v6: i32 = method15(v4.clone(), v3, v5);
    let mut v7: Rc<Vec<Rc<str>>> = Rc::new(v4.borrow().clone());
    let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    v8
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<Rc<str>>>> {
        table_problems_12(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn code_of_18(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = rig_standards_prim_bindings::rig_standards_code_of(&v0);
    v1
}
fn own_macros_19(mut v0: Rc<str>) -> i32 {
    let v1: i32 = rig_standards_prim_bindings::rig_standards_own_macros(&v0);
    v1
}
fn layer_of_module_20(mut v0: Rc<UH0>, mut v1: Rc<str>) -> US0 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: US0 = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<UH0> = v6;
                let mut v7: bool = &*v3 == &*v1;
                if v7 {
                    return v4;
                } else {
                    v0 = v6;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US0::US0_None;
            }
        }
    }
}
fn opens_module_22(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: bool = rig_standards_prim_bindings::rig_standards_module_opened(&v0, &v1);
    v2
}
fn layer_rank_25(mut v0: US1) -> i32 {
    match v0 {
        US1::US1_Core => {
            1i32
        }
        US1::US1_Ecosystem => {
            6i32
        }
        US1::US1_Host(v1) => {
            let v1: Rc<str> = v1;
            5i32
        }
        US1::US1_Meta => {
            0i32
        }
        US1::US1_Os => {
            4i32
        }
        US1::US1_Prim => {
            2i32
        }
        US1::US1_Std => {
            3i32
        }
    }
}
fn ranked_verdict_24(mut v0: US1, mut v1: US1) -> US2 {
    let v2: i32 = layer_rank_25(v1);
    let v3: i32 = layer_rank_25(v0);
    let v4: bool = v2 > v3;
    if v4 {
        US2::US2_UpwardImport
    } else {
        US2::US2_Allowed
    }
}
fn opening_verdict_23(mut v0: US1, mut v1: US1) -> US2 {
    match &v1 {
        US1::US1_Meta => {
            US2::US2_Allowed
        }
        _ => {
            match &v0 {
                US1::US1_Host(v3) => {
                    let v3: Rc<str> = v3.clone();
                    match &v1 {
                        US1::US1_Host(v4) => {
                            let v4: Rc<str> = v4.clone();
                            let v5: bool = &*v3 != &*v4;
                            if v5 {
                                US2::US2_CrossBackend
                            } else {
                                ranked_verdict_24(v0, v1)
                            }
                        }
                        _ => {
                            ranked_verdict_24(v0, v1)
                        }
                    }
                }
                _ => {
                    ranked_verdict_24(v0, v1)
                }
            }
        }
    }
}
fn layer_word_27(mut v0: US1) -> Rc<str> {
    match v0 {
        US1::US1_Core => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Core"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US1::US1_Ecosystem => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Ecosystem"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US1::US1_Host(v6) => {
            let v6: Rc<str> = v6;
            let mut v7: Rc<str> = string_concat(&["Host ", &*v6]);
            v7
        }
        US1::US1_Meta => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Meta"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US1::US1_Os => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Os"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US1::US1_Prim => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Prim"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US1::US1_Std => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Std"); } LIT.with(|lit| lit.clone()) };
            v4
        }
    }
}
fn opened_text_26(mut v0: Rc<str>, mut v1: US1) -> Rc<str> {
    let v4: Rc<str> = layer_word_27(v1);
    let v6: Rc<str> = string_concat(&["opens ", &*v0, " (", &*v4, ")"]);
    v6
}
fn method21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US1, mut v3: Rc<UH0>, mut v4: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH0::UH0_Cons(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5;
                let v6: US0 = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<UH0> = v8;
                let v28: Rc<UH1> = match v6 {
                    US0::US0_Some(v9) => {
                        let mut v9: US1 = v9;
                        let v10: bool = &*v5 != &*v0;
                        let v12: bool = if v10 {
                            opens_module_22(v1.clone(), v5.clone())
                        } else {
                            false
                        };
                        if v12 {
                            let v13: US2 = opening_verdict_23(v2.clone(), v9.clone());
                            match v13 {
                                US2::US2_Allowed => {
                                    v4
                                }
                                US2::US2_CrossBackend => {
                                    let v19: Rc<str> = opened_text_26(v5, v9);
                                    let v20: Rc<str> = string_concat(&[&*v19, " of another backend"]);
                                    let v21: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                                    let v22: Rc<UH1> = Rc::new(UH1::UH1_Cons(v20, v21));
                                    method14(v4, v22)
                                }
                                US2::US2_UpwardImport => {
                                    let v14: Rc<str> = opened_text_26(v5, v9);
                                    let v15: Rc<str> = string_concat(&[&*v14, " above its rank"]);
                                    let v16: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                                    let v17: Rc<UH1> = Rc::new(UH1::UH1_Cons(v15, v16));
                                    method14(v4, v17)
                                }
                            }
                        } else {
                            v4
                        }
                    }
                    _ => {
                        v4
                    }
                };
                (v3, v4) = (v8, v28);
                continue;
            }
            UH0::UH0_Nil => {
                return v4;
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
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method29(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn macros_allowed_30(mut v0: US1) -> bool {
    let v1: bool = match &v0 {
        US1::US1_Prim => {
            true
        }
        _ => {
            false
        }
    };
    if v1 {
        true
    } else {
        let v2: i32 = layer_rank_25(v0);
        let v3: US1 = US1::US1_Std;
        let v4: i32 = layer_rank_25(v3);
        let v5: bool = v2 > v4;
        v5
    }
}
fn list_from_base_32(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v2));
            (v1, v2) = (v4, v6);
            continue;
        }
    }
}
fn prim_binding_rows_31(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<RefCell<Vec<Rc<str>>>> = rig_standards_prim_bindings::rig_standards_prim_binding_texts(&v0);
    let v2: i32 = (v1.clone().borrow().len() as i32);
    let v3: i32 = v2.wrapping_sub(1i32);
    let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    list_from_base_32(v1, v3, v4)
}
fn module_checked_17(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, Rc<str>, i32, Rc<UH1>) {
    let v3: Rc<str> = code_of_18(v2);
    let v4: i32 = own_macros_19(v3.clone());
    let v5: US0 = layer_of_module_20(v0.clone(), v1.clone());
    let (mut v34, mut v35, mut v36, mut v37): (Rc<str>, Rc<str>, i32, Rc<UH1>) = match &v5 {
        US0::US0_None => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unassigned"); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            (v1, v6, v4, v7)
        }
        _ => {
            let (mut v30, mut v31, mut v32, mut v33): (Rc<str>, Rc<str>, i32, Rc<UH1>) = match v5 {
                US0::US0_Some(v8) => {
                    let mut v8: US1 = v8;
                    let mut v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v10: Rc<UH1> = method21(v1.clone(), v3.clone(), v8.clone(), v0, v9);
                    let v11: Rc<str> = format_real_28(v4);
                    let v13: Rc<str> = layer_word_27(v8.clone());
                    let v14: Rc<str> = string_concat(&[&*v11, " own host macro(s) in ", &*v13]);
                    let v15: bool = v4 > 0i32;
                    let v18: bool = if v15 {
                        let v16: bool = macros_allowed_30(v8.clone());
                        let v17: bool = v16 == false;
                        v17
                    } else {
                        false
                    };
                    let v22: Rc<UH1> = if v18 {
                        let v19: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v14, v19))
                    } else {
                        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
                    };
                    let v23: bool = match &v8 {
                        US1::US1_Prim => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v26: Rc<UH1> = if v23 {
                        prim_binding_rows_31(v3)
                    } else {
                        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
                    };
                    let v27: Rc<UH1> = method14(v22, v26);
                    let v28: Rc<UH1> = method14(v10, v27);
                    let v29: Rc<str> = layer_word_27(v8);
                    (v1, v29, v4, v28)
                }
                _ => unreachable!(),
            };
            (v30, v31, v32, v33)
        }
    };
    (v34, v35, v36, v37)
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" own host macro(s) in "); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.contains(&*v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method35() -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure4()
}
fn method36(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<UH1>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: bool = v0(v2);
                if v4 {
                    return true;
                } else {
                    let v5: Rc<dyn Fn(Rc<str>) -> bool> = method35();
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
fn method37(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("opens "); } LIT.with(|lit| lit.clone()) };
                let v5: bool = v2.starts_with(&*v4);
                let mut v8: bool = if v5 {
                    true
                } else {
                    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("partial backend binding"); } LIT.with(|lit| lit.clone()) };
                    let mut v7: bool = v2.starts_with(&*v6);
                    v7
                };
                let v10: i32 = if v8 {
                    let mut v9: i32 = v1.wrapping_add(1i32);
                    v9
                } else {
                    v1
                };
                (v0, v1) = (v3, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn violation_count_34(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<UH1>) -> i32 {
    let v4: Rc<dyn Fn(Rc<str>) -> bool> = method35();
    let v5: bool = method36(v4, v3.clone());
    let v6: i32 = 0i32;
    let mut v7: i32 = method37(v3, v6);
    let mut v8: i32 = if v5 {
        v2
    } else {
        0i32
    };
    let mut v9: i32 = v7.wrapping_add(v8);
    v9
}
fn method38(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method38(v3, v1);
            let v5: Rc<str> = string_concat(&["    ", &*v2]);
            Rc::new(UH1::UH1_Cons(v5, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn report_lines_33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<UH1>) -> Rc<UH1> {
    let mut v8: Rc<str> = format_real_28(v2);
    let v10: Rc<str> = string_concat(&["layers / ", &*v0, " ", &*v1, " macros=", &*v8, " violations="]);
    let v11: i32 = violation_count_34(v0, v1, v2, v3.clone());
    let v12: Rc<str> = format_real_28(v11);
    let v13: Rc<str> = string_concat(&[&*v10, &*v12]);
    let v14: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v15: Rc<UH1> = method38(v3, v14);
    Rc::new(UH1::UH1_Cons(v13, v15))
}
fn module_lines_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v3: Rc<UH0> = table_rows_2(v2);
    let (mut v4, mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, i32, Rc<UH1>) = module_checked_17(v3, v1, v0);
    let mut v8: Rc<UH1> = report_lines_33(v4, v5, v6, v7);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v10: i32 = 0i32;
    let v11: i32 = method15(v9.clone(), v8, v10);
    let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
    let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
    v13
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> Rc<RefCell<Vec<Rc<str>>>> {
        module_lines_16(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn module_violations_39(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: Rc<UH0> = table_rows_2(v2);
    let (mut v4, mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, i32, Rc<UH1>) = module_checked_17(v3, v1, v0);
    violation_count_34(v4, v5, v6, v7)
}
fn closure5() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> i32 {
        module_violations_39(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_layers_table_count(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_layers_table_problems(v0: &str) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure2()(Rc::<str>::from(v0))
}
pub fn rig_layers_module_lines(v0: &str, v1: &str, v2: &str) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub fn rig_layers_module_violations(v0: &str, v1: &str, v2: &str) -> i32 {
    closure5()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
use rig_standards_prim_bindings::SpiralShape_97df63ebabc3 as Mut1;
pub type SpiralShape_accbf6de6842 = Mut0;
