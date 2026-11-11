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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(i32),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_SpiprojFile,
    US1_TomlFile,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US1, Rc<UH2>),
}
#[derive(Clone)]
enum US2 {
    US2_Other,
    US2_Packages,
    US2_Modules,
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(i32, Rc<str>, Rc<UH3>),
}
fn joined_with_2(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<str> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match &*v3 {
                UH0::UH0_Nil => {
                    v2
                }
                _ => {
                    let mut v4: Rc<str> = string_concat(&[&*v2, &*v0]);
                    let v5: Rc<str> = joined_with_2(v0, v3);
                    let mut v6: Rc<str> = string_concat(&[&*v4, &*v5]);
                    v6
                }
            }
        }
        UH0::UH0_Nil => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v8
        }
    }
}
fn method3(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let mut v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method5(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn up_one_6(mut v0: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v1, v2) => {
            let v1: Rc<str> = v1.clone();
            let v2: Rc<UH0> = v2.clone();
            let v3: bool = &*v1 == "..";
            if v3 {
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".."); } LIT.with(|lit| lit.clone()) };
                Rc::new(UH0::UH0_Cons(v4, v0))
            } else {
                v2
            }
        }
        UH0::UH0_Nil => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".."); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v7, v8))
        }
    }
}
fn normal_parts_4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v2 == "";
                let mut v6: bool = if v4 {
                    true
                } else {
                    let v5: bool = &*v2 == ".";
                    v5
                };
                if v6 {
                    v0 = v3;
                    continue;
                } else {
                    let v8: bool = &*v2 == "..";
                    if v8 {
                        let mut v9: Rc<UH0> = up_one_6(v1);
                        (v0, v1) = (v3, v9);
                        continue;
                    } else {
                        let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                        (v0, v1) = (v3, v11);
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method5(v1, v15);
            }
        }
    }
}
fn normal_path_1(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v5: Rc<Vec<Rc<str>>> = Rc::new((v4).borrow().clone());
    let mut v6: usize = (v5).len();
    let v7: i32 = ((v6) as i32);
    let v8: i32 = v7.wrapping_sub(1i32);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH0> = method3(v5, v8, v9);
    let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH0> = normal_parts_4(v10, v11);
    joined_with_2(v2, v12)
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    Rc::new(move |mut v2: Rc<UH0>| -> US0 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US0::US0_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US0> = method9(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US0::US0_None
            }
        }
    })
}
fn method9(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    closure1(v0, v1)
}
fn directory_exists_10(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
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
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v7: Rc<str> = string_concat(&[&*v0, "/", &*v4]);
                let v8: i32 = 0i32;
                let mut v9: Rc<dyn Fn(Rc<UH0>) -> US0> = method9(v4, v8);
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target"); } LIT.with(|lit| lit.clone()) };
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".git"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules"); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".cache"); } LIT.with(|lit| lit.clone()) };
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_build"); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bin"); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("obj"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17, v18));
                let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v19));
                let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v20));
                let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v14, v21));
                let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v22));
                let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v23));
                let v25: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v24));
                let v26: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v25));
                let v27: US0 = v9(v26);
                let v28: bool = match v27 {
                    US0::US0_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v29: bool = v28 != true;
                let v32: bool = if v29 {
                    true
                } else {
                    let v30: bool = directory_exists_10(v7.clone());
                    let v31: bool = v30 == false;
                    v31
                };
                let v35: Rc<UH1> = if v32 {
                    v3
                } else {
                    let v33: i32 = v1.wrapping_add(1i32);
                    manifests_in_7(v7, v33, v3)
                };
                (v2, v3) = (v5, v35);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn file_exists_11(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
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
fn method12(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn indent_of_19(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v3: bool = v0.starts_with(&*v2);
        if v3 {
            let mut v4: i32 = (v0.clone().len() as i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 1i32 as i64, v5 as i64);
            let v7: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v6, v7);
            continue;
        } else {
            return v1;
        }
    }
}
fn value_after_20(mut v0: &Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: i32 = (v1.clone().len() as i32);
    let mut v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v1, v2 as i64, v4 as i64);
    let mut v6: Rc<str> = Rc::<str>::from((v5).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    v6
}
fn header_of_21(mut v0: Rc<str>) -> US2 {
    let v1: bool = &*v0 == "packages:";
    if v1 {
        US2::US2_Packages
    } else {
        let v3: bool = &*v0 == "modules:";
        if v3 {
            US2::US2_Modules
        } else {
            US2::US2_Other
        }
    }
}
fn popped_22(mut v0: Rc<UH3>, mut v1: i32) -> Rc<UH3> {
    loop {
        match &*v0 {
            UH3::UH3_Cons(v2, v3, v4) => {
                let v2: i32 = *v2;
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH3> = v4.clone();
                let v5: bool = v2 >= v1;
                if v5 {
                    v0 = v4;
                    continue;
                } else {
                    return v0;
                }
            }
            UH3::UH3_Nil => {
                return { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn prefix_of_23(mut v0: Rc<UH3>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_Cons(v1, v2, v3) => {
            let v1: i32 = v1;
            let v2: Rc<str> = v2;
            let v3: Rc<UH3> = v3;
            let mut v4: Rc<str> = prefix_of_23(v3);
            let v5: Rc<str> = string_concat(&[&*v4, &*v2]);
            v5
        }
        UH3::UH3_Nil => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v6
        }
    }
}
fn dep_name_24(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    let mut v6: Rc<str> = if v2 {
        let v3: i32 = (v0.clone().len() as i32);
        let mut v4: i32 = v3.wrapping_sub(1i32);
        let v5: Rc<str> = string_slice(&v0, 1i32 as i64, v4 as i64);
        v5
    } else {
        v0
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v8: bool = v6.ends_with(&*v7);
    if v8 {
        let mut v9: i32 = (v6.clone().len() as i32);
        let v10: i32 = v9.wrapping_sub(2i32);
        let v11: Rc<str> = string_slice(&v6, 0i32 as i64, v10 as i64);
        v11
    } else {
        v6
    }
}
enum Group16 {
    M16(Rc<UH0>, US2, Rc<UH3>, Rc<str>, Rc<UH0>, Rc<UH0>),
    M17(Rc<UH0>, Rc<str>, Rc<UH0>, Rc<UH0>, Rc<str>),
    M18(Rc<UH0>, US2, Rc<UH3>, Rc<str>, Rc<UH0>, Rc<UH0>, i32, Rc<str>),
}
fn group_16(mut st: Group16) -> (Rc<str>, Rc<UH0>, Rc<UH0>) {
    loop {
        match st {
            Group16::M16(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5) => {
                match Rc::unwrap_or_clone(v0) {
                    UH0::UH0_Cons(v10, v11) => {
                        let v10: Rc<str> = v10;
                        let v11: Rc<UH0> = v11;
                        let v12: i32 = 0i32;
                        let v13: i32 = indent_of_19(v10.clone(), v12);
                        let v14: Rc<str> = Rc::<str>::from((v10).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
                        let v15: bool = &*v14 == "";
                        if v15 {
                            st = Group16::M16(v11, v1, v2, v3, v4, v5);
                            continue;
                        } else {
                            let v19: bool = v13 == 0i32;
                            if v19 {
                                st = Group16::M17(v11, v3, v4, v5, v14);
                                continue;
                            } else {
                                st = Group16::M18(v11, v1, v2, v3, v4, v5, v13, v14);
                                continue;
                            }
                        }
                    }
                    UH0::UH0_Nil => {
                        let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v7: Rc<UH0> = method5(v4, v6);
                        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let mut v9: Rc<UH0> = method5(v5, v8);
                        return (v3, v7, v9);
                    }
                }
            }
            Group16::M17(mut v0, mut v1, mut v2, mut v3, mut v4) => {
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packageDir:"); } LIT.with(|lit| lit.clone()) };
                let mut v6: bool = v4.starts_with(&*v5);
                if v6 {
                    let v7: US2 = US2::US2_Other;
                    let v8: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                    let mut v9: Rc<str> = value_after_20(&v5, v4);
                    st = Group16::M16(v0, v7, v8, v9, v2, v3);
                    continue;
                } else {
                    let v13: US2 = header_of_21(v4);
                    let v14: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                    st = Group16::M16(v0, v13, v14, v1, v2, v3);
                    continue;
                }
            }
            Group16::M18(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7) => {
                match &v1 {
                    US2::US2_Modules => {
                        let v13: Rc<UH3> = popped_22(v2, v6);
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                        let v15: bool = v7.ends_with(&*v14);
                        if v15 {
                            let v16: Rc<UH3> = Rc::new(UH3::UH3_Cons(v6, v7, v13));
                            st = Group16::M16(v0, v1, v16, v3, v4, v5);
                            continue;
                        } else {
                            let v20: Rc<str> = prefix_of_23(v13.clone());
                            let v21: Rc<str> = string_concat(&[&*v20, &*v7]);
                            let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v21, v5));
                            st = Group16::M16(v0, v1, v13, v3, v4, v22);
                            continue;
                        }
                    }
                    US2::US2_Other => {
                        st = Group16::M16(v0, v1, v2, v3, v4, v5);
                        continue;
                    }
                    US2::US2_Packages => {
                        let v8: Rc<str> = dep_name_24(v7);
                        let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v4));
                        st = Group16::M16(v0, v1, v2, v3, v9, v5);
                        continue;
                    }
                }
            }
        }
    }
}
fn read_entry_18(mut v0: Rc<UH0>, mut v1: US2, mut v2: Rc<UH3>, mut v3: Rc<str>, mut v4: Rc<UH0>, mut v5: Rc<UH0>, mut v6: i32, mut v7: Rc<str>) -> (Rc<str>, Rc<UH0>, Rc<UH0>) {
    group_16(Group16::M18(v0, v1, v2, v3, v4, v5, v6, v7))
}
fn read_header_17(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH0>, mut v4: Rc<str>) -> (Rc<str>, Rc<UH0>, Rc<UH0>) {
    group_16(Group16::M17(v0, v1, v2, v3, v4))
}
fn read_lines_16(mut v0: Rc<UH0>, mut v1: US2, mut v2: Rc<UH3>, mut v3: Rc<str>, mut v4: Rc<UH0>, mut v5: Rc<UH0>) -> (Rc<str>, Rc<UH0>, Rc<UH0>) {
    group_16(Group16::M16(v0, v1, v2, v3, v4, v5))
}
fn text_lines_25(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new((v5).borrow().clone());
    let v7: usize = (v6).len();
    let v8: i32 = ((v7) as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method3(v6, v9, v10)
}
fn manifest_of_15(mut v0: Rc<str>) -> (Rc<str>, Rc<UH0>, Rc<UH0>) {
    let v1: Rc<UH0> = text_lines_25(v0);
    let v2: US2 = US2::US2_Other;
    let v3: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    read_lines_16(v1, v2, v3, v4, v5, v6)
}
fn joined_path_28(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = string_concat(&[&*v0, "/", &*v1]);
    normal_path_1(v3)
}
fn resolved_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.starts_with(&*v3);
    if v4 {
        joined_path_28(v0, v2)
    } else {
        let mut v6: Rc<str> = joined_path_28(v0, v1);
        joined_path_28(v6, v2)
    }
}
fn method26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            let mut v6: Rc<UH0> = method26(v0.clone(), v1.clone(), v5, v3);
            let v7: Rc<str> = resolved_27(v0, v1, v4);
            Rc::new(UH0::UH0_Cons(v7, v6))
        }
        UH0::UH0_Nil => {
            v3
        }
    }
}
fn method30(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn listed_29(mut v0: Rc<UH0>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v2: i32 = 0i32;
    let v3: i32 = method30(v1.clone(), v0, v2);
    let mut v4: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v4).as_ref().clone()));
    v5
}
fn row_of_14(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) {
    let (mut v2, mut v3, mut v4): (Rc<str>, Rc<UH0>, Rc<UH0>) = manifest_of_15(v1);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = method26(v0.clone(), v2, v3, v5);
    let v7: Rc<RefCell<Vec<Rc<str>>>> = listed_29(v6);
    let v8: Rc<str> = normal_path_1(v0);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = listed_29(v4);
    (v8, v7, v9)
}
fn method32(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method32(v3, v1);
            let v5: Rc<str> = dep_name_24(v2);
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: &Rc<RefCell<Vec<Rc<str>>>>, mut v3: &Rc<RefCell<Vec<Rc<str>>>>, mut v4: Rc<UH0>, mut v5: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v4) {
        UH0::UH0_Cons(v6, v7) => {
            let mut v6: Rc<str> = v6;
            let v7: Rc<UH0> = v7;
            let v8: Rc<UH0> = method33(v0.clone(), v1.clone(), v2, v3, v7, v5);
            let mut v9: Rc<str> = resolved_27(v0, v1, v6);
            Rc::new(UH0::UH0_Cons(v9, v8))
        }
        UH0::UH0_Nil => {
            v5
        }
    }
}
fn row_of_toml_31(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) {
    let v2: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = rig_proof_scope_toml::rig_proof_scope_toml_package(&v0, &v1);
    let (mut v3, mut v4, mut v5): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) = v2.borrow()[0i32 as usize].clone();
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new((v4).borrow().clone());
    let v7: usize = (v6).len();
    let v8: i32 = ((v7) as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH0> = method3(v6, v9, v10);
    let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH0> = method32(v11, v12);
    let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v15: Rc<UH0> = method33(v0.clone(), v3, &v4, &v5, v13, v14);
    let v16: Rc<str> = normal_path_1(v0);
    let v17: Rc<RefCell<Vec<Rc<str>>>> = listed_29(v15);
    (v16, v17, v5)
}
fn row_from_13(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) {
    let (mut v12, mut v13, mut v14): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) = match &v0 {
        US1::US1_SpiprojFile => {
            row_of_14(v1, v2)
        }
        _ => {
            let (mut v9, mut v10, mut v11): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) = match v0 {
                US1::US1_TomlFile => {
                    row_of_toml_31(v1, v2)
                }
                _ => unreachable!(),
            };
            (v9, v10, v11)
        }
    };
    (v12, v13, v14)
}
fn child_names_34(mut v0: Rc<str>) -> Rc<UH0> {
    let v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Vec<std::string::String> = v2.unwrap();
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: bool = true; let _vec_fold_ = v4.into_iter().fold(v5, |acc, x| { //;
    let mut v9: Rc<UH0> = acc;
    let v11: std::string::String = x;
    let v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v9));
    let v16: bool = true; v15 });
    let v18: Rc<UH0> = _vec_fold_;
    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method5(v18, v19)
}
fn manifests_in_7(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    let v3: Rc<str> = string_concat(&[&*v0, "/spiral.toml"]);
    let mut v4: bool = file_exists_11(v3.clone());
    let mut v9: Rc<UH2> = if v4 {
        let v5: US1 = US1::US1_TomlFile;
        let mut v6: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH2::UH2_Cons(v5, v6))
    } else {
        { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
    };
    let v10: Rc<str> = string_concat(&[&*v0, "/package.spiproj"]);
    let v11: bool = file_exists_11(v10.clone());
    let v14: Rc<UH2> = if v11 {
        let v12: US1 = US1::US1_SpiprojFile;
        Rc::new(UH2::UH2_Cons(v12, v9))
    } else {
        v9
    };
    let v35: Rc<UH1> = match Rc::unwrap_or_clone(v14) {
        UH2::UH2_Cons(v15, v16) => {
            let v15: US1 = v15;
            let v16: Rc<UH2> = v16;
            let v18: Rc<str> = match &v15 {
                US1::US1_SpiprojFile => {
                    v10
                }
                US1::US1_TomlFile => {
                    v3
                }
            };
            let v20: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v18);
            let v22: Vec<u8> = v20.unwrap();
            let v23: Vec<u8> = method12(v22);
            let v25: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v23);
            let v27: std::string::String = v25.unwrap();
            let v29: Rc<str> = Rc::<str>::from(String::as_str(&v27));
            let (mut v30, mut v31, mut v32): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) = row_from_13(v15, v0.clone(), v29);
            Rc::new(UH1::UH1_Cons(v30, v31, v32, v2))
        }
        UH2::UH2_Nil => {
            v2
        }
    };
    let v36: bool = v1 >= 8i32;
    if v36 {
        v35
    } else {
        let v37: Rc<UH0> = child_names_34(v0.clone());
        method8(v0, v1, v37, v35)
    }
}
fn method35(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = v4;
                let v5: Rc<RefCell<Vec<Rc<str>>>> = v5;
                let mut v6: Rc<UH1> = v6;
                v0.borrow_mut().push((v3, v4, v5));
                let v7: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn manifests_under_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    let v1: Rc<str> = normal_path_1(v0);
    let v2: i32 = 0i32;
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH1> = manifests_in_7(v1, v2, v3);
    let v5: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v6: i32 = 0i32;
    let v7: i32 = method35(v5.clone(), v4, v6);
    let v8: Rc<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>> = Rc::new(v5.borrow().clone());
    let mut v9: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
    v9
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
        manifests_under_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn anchored_38(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        true
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v1.contains(&*v4);
        v5
    };
    if v6 {
        normal_path_1(v1)
    } else {
        joined_path_28(v0, v1)
    }
}
fn method37(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method37(v0.clone(), v4, v2);
            let mut v6: Rc<str> = anchored_38(v0, v3);
            Rc::new(UH0::UH0_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn anchored_paths_36(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v2: Rc<str> = normal_path_1(v1);
    let v3: Rc<Vec<Rc<str>>> = Rc::new((v0).borrow().clone());
    let mut v4: usize = (v3).len();
    let v5: i32 = ((v4) as i32);
    let mut v6: i32 = v5.wrapping_sub(1i32);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = method3(v3, v6, v7);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH0> = method37(v2, v8, v9);
    listed_29(v10)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>| -> Rc<RefCell<Vec<Rc<str>>>> {
        anchored_paths_36(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_proof_scope_manifests_under(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_proof_scope_anchored_paths(v0: &str, v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure2()(Rc::<str>::from(v0), v1)
}
