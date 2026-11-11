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
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_Removable(Rc<str>),
    US0_KeptCited(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US0, Rc<UH2>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn directory_exists_5(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn entries_of_3(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: bool = directory_exists_5(v0.clone());
    let mut v2: bool = v1 == false;
    if v2 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v5: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v7: Vec<std::string::String> = v5.unwrap();
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v10: bool = true; let _vec_fold_ = v7.into_iter().fold(v8, |acc, x| { //;
        let v12: Rc<UH0> = acc;
        let v14: std::string::String = x;
        let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
        let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v12));
        let v19: bool = true; v18 });
        let v21: Rc<UH0> = _vec_fold_;
        let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method4(v21, v22)
    }
}
fn is_state_file_7(mut v0: Rc<str>) -> bool {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.ends_with(&*v1);
    v2
}
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method6(v3, v1);
            let mut v5: bool = is_state_file_7(v2.clone());
            if v5 {
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
fn method8(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let mut v5: Rc<UH0> = method8(v0.clone(), v4, v2);
            let mut v7: Rc<str> = string_concat(&[&*v0, "/", &*v3]);
            let v8: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v7).unwrap_or_default().as_str());
            Rc::new(UH0::UH0_Cons(v8, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method9(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn state_text_2(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = string_concat(&[&*v3, "/state"]);
    let mut v5: Rc<UH0> = entries_of_3(v4.clone());
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method6(v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method8(v4, v7, v8);
    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v11: i32 = 0i32;
    let v12: i32 = method9(v10.clone(), v9, v11);
    let v13: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
    let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v13).as_ref().clone()));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from((*v14.borrow()).join(&*v15));
    v16
}
fn method11(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH1> = v4;
                let mut v5: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v1));
                (v0, v1) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method13(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method13(v7, &v9) {
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
        method4(v16, v17)
    }
}
fn note_marker_16() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= Note (\""); } LIT.with(|lit| lit.clone()) };
    v0
}
fn method17(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_item_15(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = note_marker_16();
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method17(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let v8: Rc<str> = note_marker_16();
    let mut v9: i32 = (v8.len() as i32);
    let v10: i32 = v7.wrapping_add(v9);
    let v11: bool = v7 < 0i32;
    if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v14: i32 = (v0.clone().len() as i32);
        let v15: i32 = v14.wrapping_sub(v10);
        let v16: usize = ((v10) as usize);
        let v17: i32 = v10.wrapping_add(v15);
        let v18: usize = ((v17) as usize);
        let v19: Rc<str> = Rc::<str>::from(&(v0)[v16..v18]);
        let v20: Rc<UH0> = split_list_12(v13, v19);
        match Rc::unwrap_or_clone(v20) {
            UH0::UH0_Cons(v21, v22) => {
                let v21: Rc<str> = v21;
                let v22: Rc<UH0> = v22;
                v21
            }
            UH0::UH0_Nil => {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v23
            }
        }
    }
}
fn boundaries_20() -> Rc<UH0> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("`"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v14, v15));
    let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v16));
    let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v17));
    let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v18));
    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v19));
    let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v20));
    let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v21));
    let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v22));
    let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v23));
    let v25: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v24));
    let v26: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v25));
    let v27: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v26));
    let v28: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v27));
    let v29: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v28));
    Rc::new(UH0::UH0_Cons(v0, v29))
}
fn method21(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let mut v5: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v4));
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn normalized_19(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = Rc::<str>::from(v3.to_lowercase());
    let mut v5: Rc<UH0> = boundaries_20();
    let mut v6: Rc<str> = method21(v5, v4);
    let v8: Rc<str> = string_concat(&["/", &*v6, "/"]);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(v8.replace(&*v9, &*v2));
    let v11: Rc<str> = Rc::<str>::from(v10.replace(&*v9, &*v2));
    v11
}
fn note_of_18(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    let mut v2: Rc<str> = normalized_19(v1);
    (v0, v2)
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl note_of_"); } LIT.with(|lit| lit.clone()) };
                let mut v5: bool = v2.starts_with(&*v4);
                let v8: Rc<str> = if v5 {
                    note_item_15(v2.clone())
                } else {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v7
                };
                let mut v9: bool = &*v8 == "";
                let v13: Rc<UH1> = if v9 {
                    v1
                } else {
                    let (mut v10, mut v11): (Rc<str>, Rc<str>) = note_of_18(v8, v2);
                    Rc::new(UH1::UH1_Cons(v10, v11, v1))
                };
                (v0, v1) = (v3, v13);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn notes_of_state_10(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_12(v1, v4);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH1> = method14(v5, v6);
    let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method11(v7, v8)
}
fn method22(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
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
fn method24(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let mut v5: Rc<UH0> = method24(v0.clone(), v4, v2);
            let mut v7: Rc<str> = string_concat(&[&*v0, "/", &*v3]);
            let v8: bool = directory_exists_5(v7);
            if v8 {
                Rc::new(UH0::UH0_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn folders_of_23(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<UH0> = entries_of_3(v3.clone());
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method24(v3, v4, v5)
}
fn closure2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v2: Rc<str>| -> bool {
        let v3: Option<usize> = v1.find(&*v2);
        let mut v4: Option<usize> = method17(v3);
        let mut v5: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v6: Option<i32> = v4.map(|x| v5(x));
        let v8: i32 = v6.unwrap_or(-1i32);
        let mut v9: bool = v8 >= 0i32;
        v9
    })
}
fn method30(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure2(v0, v1)
}
fn method31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<dyn Fn(Rc<str>) -> bool>, mut v3: Rc<UH0>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = v2(v4);
                if v6 {
                    return true;
                } else {
                    let mut v7: Rc<dyn Fn(Rc<str>) -> bool> = method30(v0.clone(), v1.clone());
                    (v2, v3) = (v7, v5);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn method29(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: US1) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH1> = v5;
                let v13: US1 = match &v2 {
                    US1::US1_None => {
                        let mut v7: Rc<dyn Fn(Rc<str>) -> bool> = method30(v3.clone(), v4.clone());
                        let v8: bool = method31(v3.clone(), v4, v7, v0.clone());
                        if v8 {
                            US1::US1_Some(v3)
                        } else {
                            US1::US1_None
                        }
                    }
                    US1::US1_Some(v6) => {
                        let mut v6: Rc<str> = v6.clone();
                        v2
                    }
                };
                (v1, v2) = (v5, v13);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn bare_name_length_33() -> i32 {
    16i32
}
fn keys_of_32(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v3: Rc<str> = string_concat(&[&*v0, "/", &*v1]);
    let mut v4: Rc<str> = normalized_19(v3);
    let mut v5: i32 = (v1.clone().len() as i32);
    let mut v6: i32 = bare_name_length_33();
    let mut v7: bool = v5 >= v6;
    if v7 {
        let v8: Rc<str> = normalized_19(v1);
        let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v9));
        Rc::new(UH0::UH0_Cons(v4, v10))
    } else {
        let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v4, v12))
    }
}
fn cited_by_28(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<str>) -> US1 {
    let v3: Rc<UH0> = keys_of_32(v1, v2);
    let mut v4: US1 = US1::US1_None;
    method29(v3, v0, v4)
}
fn fate_of_27(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let v3: US1 = cited_by_28(v0, v1, v2.clone());
    match v3 {
        US1::US1_None => {
            US0::US0_Removable(v2)
        }
        US1::US1_Some(v4) => {
            let mut v4: Rc<str> = v4;
            US0::US0_KeptCited(v2, v4)
        }
    }
}
fn method26(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4;
            let mut v5: Rc<UH0> = v5;
            let mut v6: Rc<UH2> = method26(v0.clone(), v1.clone(), v5, v3);
            let mut v7: US0 = fate_of_27(v0, v1, v4);
            Rc::new(UH2::UH2_Cons(v7, v6))
        }
        UH0::UH0_Nil => {
            v3
        }
    }
}
fn fates_25(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> Rc<UH2> {
    let v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method26(v0, v1, v2, v3)
}
fn fate_line_35(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_KeptCited(v4, v5) => {
            let mut v4: Rc<str> = v4;
            let mut v5: Rc<str> = v5;
            let v8: Rc<str> = string_concat(&["scratch archive / ", &*v4, " kept: cited by ", &*v5]);
            v8
        }
        US0::US0_Removable(v1) => {
            let v1: Rc<str> = v1;
            let v3: Rc<str> = string_concat(&["scratch archive / ", &*v1, " removable after the verify"]);
            v3
        }
    }
}
fn method34(mut v0: Rc<UH2>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v1, v2) => {
                let v1: US0 = v1;
                let mut v2: Rc<UH2> = v2;
                let v3: Rc<str> = fate_line_35(v1);
                println!("{}", v3);
                v0 = v2;
                continue;
            }
            UH2::UH2_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method36(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: US0 = v2;
                let v3: Rc<UH2> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method38(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_37(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method38(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method42(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method42(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method41(mut v0: Rc<UH2>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: US0 = v2;
                let v3: Rc<UH2> = v3;
                let v11: Rc<UH0> = match v2 {
                    US0::US0_KeptCited(v8, v9) => {
                        let v8: Rc<str> = v8;
                        let mut v9: Rc<str> = v9;
                        v1
                    }
                    US0::US0_Removable(v4) => {
                        let mut v4: Rc<str> = v4;
                        let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5));
                        method42(v1, v6)
                    }
                };
                (v0, v1) = (v3, v11);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn removable_names_40(mut v0: Rc<UH2>) -> Rc<UH0> {
    let v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method41(v0, v1)
}
fn method43(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn kept_count_39(mut v0: Rc<UH2>) -> i32 {
    let v1: i32 = 0i32;
    let mut v2: i32 = method36(v0.clone(), v1);
    let v3: Rc<UH0> = removable_names_40(v0);
    let mut v4: i32 = 0i32;
    let mut v5: i32 = method43(v3, v4);
    let mut v6: i32 = v2.wrapping_sub(v5);
    v6
}
fn cited_run_1(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = state_text_2(v0.clone());
    let v3: Rc<UH1> = notes_of_state_10(v2);
    let mut v4: i32 = 0i32;
    let mut v5: i32 = method22(v3.clone(), v4);
    let mut v6: bool = v5 == 0i32;
    if v6 {
        let v8: Rc<str> = string_concat(&["rig error: scratch cited: no backlog notes under ", &*v0, "/state, so nothing is removable"]);
        eprintln!("{}", v8);
        2i32
    } else {
        let mut v9: Rc<UH0> = folders_of_23(v1.clone());
        let v10: Rc<UH2> = fates_25(v3, v1, v9);
        method34(v10.clone());
        let v11: i32 = 0i32;
        let v12: i32 = method36(v10.clone(), v11);
        let v13: Rc<str> = format_real_37(v12);
        let v16: i32 = kept_count_39(v10);
        let v17: Rc<str> = format_real_37(v16);
        let v19: Rc<str> = format_real_37(v5);
        let v21: Rc<str> = string_concat(&["rig scratch cited ok notes=", &*v19, " folders=", &*v13, " kept=", &*v17]);
        println!("{}", v21);
        0i32
    }
}
fn command_line_argument_count_45() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_46(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_44(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_45();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US1 = if v4 {
        let mut v5: Rc<str> = command_line_argument_46(v0);
        US1::US1_Some(v5)
    } else {
        US1::US1_None
    };
    match v8 {
        US1::US1_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US1::US1_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_44(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn scratch_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_44(v0);
    match Rc::unwrap_or_clone(v1) {
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
                            let mut v7: Rc<UH0> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH0::UH0_Cons(v8, v9) => {
                                    let v8: Rc<str> = v8;
                                    let mut v9: Rc<UH0> = v9;
                                    let v10: bool = &*v4 == "cited";
                                    if v10 {
                                        cited_run_1(v6, v8)
                                    } else {
                                        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: scratch cited <backlog root> <scratch root>"); } LIT.with(|lit| lit.clone()) };
                                        eprintln!("{}", v12);
                                        2i32
                                    }
                                }
                                _ => {
                                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: scratch cited <backlog root> <scratch root>"); } LIT.with(|lit| lit.clone()) };
                                    eprintln!("{}", v14);
                                    2i32
                                }
                            }
                        }
                        _ => {
                            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: scratch cited <backlog root> <scratch root>"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v16);
                            2i32
                        }
                    }
                }
                _ => {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: scratch cited <backlog root> <scratch root>"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v18);
                    2i32
                }
            }
        }
        _ => {
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: scratch cited <backlog root> <scratch root>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v20);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        scratch_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_scratch_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
