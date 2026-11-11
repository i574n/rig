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
    US0_TermMacro,
    US0_TypeMacro,
}
#[derive(Clone)]
enum US1 {
    US1_InlineMacro,
    US1_GlobalDeclaration,
    US1_GlobalBody,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(i32, Rc<str>, US0, Rc<str>, bool, US1, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_MacroLong,
    US2_MacroBlock,
    US2_MacroFlow,
    US2_MacroCast,
    US2_MacroArith,
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US3 {
    US3_Some(US2),
    US3_None,
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5, v6, v7, v8) => {
                let v2: i32 = v2;
                let mut v3: Rc<str> = v3;
                let v4: US0 = v4;
                let v5: Rc<str> = v5;
                let mut v6: bool = v6;
                let mut v7: US1 = v7;
                let v8: Rc<UH0> = v8;
                let v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v4, v5, v6, v7, v1));
                (v0, v1) = (v8, v9);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn definition_name_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v0 == "";
    let v5: bool = if v2 {
        true
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.starts_with(&*v3);
        v4
    };
    if v5 {
        v1
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v6).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v8: i32 = (v7.clone().borrow().len() as i32);
        let v9: bool = v8 < 2i32;
        let v27: bool = if v9 {
            true
        } else {
            let v10: Rc<str> = v7.borrow()[0i32 as usize].clone();
            let mut v11: bool = &*v10 == "inl";
            let v13: bool = if v11 {
                true
            } else {
                let v12: bool = &*v10 == "let";
                v12
            };
            let mut v15: bool = if v13 {
                true
            } else {
                let mut v14: bool = &*v10 == ":=";
                v14
            };
            let v17: bool = if v15 {
                true
            } else {
                let v16: bool = &*v10 == ":~";
                v16
            };
            let v19: bool = if v17 {
                true
            } else {
                let v18: bool = &*v10 == ":!";
                v18
            };
            let mut v21: bool = if v19 {
                true
            } else {
                let v20: bool = &*v10 == "nominal";
                v20
            };
            let v23: bool = if v21 {
                true
            } else {
                let v22: bool = &*v10 == "type";
                v22
            };
            let v25: bool = if v23 {
                true
            } else {
                let mut v24: bool = &*v10 == "union";
                v24
            };
            let v26: bool = v25 == false;
            v26
        };
        if v27 {
            v1
        } else {
            let v28: Rc<str> = v7.borrow()[1i32 as usize].clone();
            let v29: bool = &*v28 == "rec";
            let v31: bool = if v29 {
                let v30: bool = v8 >= 3i32;
                v30
            } else {
                false
            };
            if v31 {
                let v32: Rc<str> = v7.borrow()[2i32 as usize].clone();
                v32
            } else {
                let v33: Rc<str> = v7.borrow()[1i32 as usize].clone();
                v33
            }
        }
    }
}
fn method6(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn global_macro_5(mut v0: i32, mut v1: Rc<str>) -> (i32, Rc<str>, US0, Rc<str>, bool, US1) {
    let v2: Rc<str> = Rc::<str>::from((v1).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v4: Option<usize> = v2.find(&*v3);
    let v5: Option<usize> = method6(v4);
    let mut v6: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v7: Option<i32> = v5.map(|x| v6(x));
    let v9: i32 = v7.unwrap_or(-1i32);
    let v10: i32 = (v2.clone().len() as i32);
    let mut v11: bool = v9 < 0i32;
    let v20: Rc<str> = if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    } else {
        let v13: i32 = v9.wrapping_add(1i32);
        let mut v14: i32 = v10.wrapping_sub(v9);
        let mut v15: i32 = v14.wrapping_sub(1i32);
        let v16: usize = ((v13) as usize);
        let v17: i32 = v13.wrapping_add(v15);
        let v18: usize = ((v17) as usize);
        let v19: Rc<str> = Rc::<str>::from(&(v2)[v16..v18]);
        v19
    };
    let mut v21: bool = v9 >= 0i32;
    let mut v24: bool = if v21 {
        let v22: bool = v20.starts_with(&*v3);
        let v23: bool = v22 == false;
        v23
    } else {
        false
    };
    let v39: bool = if v24 {
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("use "); } LIT.with(|lit| lit.clone()) };
        let v26: bool = v20.starts_with(&*v25);
        let v29: bool = if v26 {
            true
        } else {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import "); } LIT.with(|lit| lit.clone()) };
            let v28: bool = v20.starts_with(&*v27);
            v28
        };
        let v32: bool = if v29 {
            true
        } else {
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("extern "); } LIT.with(|lit| lit.clone()) };
            let v31: bool = v20.starts_with(&*v30);
            v31
        };
        let v35: bool = if v32 {
            true
        } else {
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mod "); } LIT.with(|lit| lit.clone()) };
            let v34: bool = v20.starts_with(&*v33);
            v34
        };
        if v35 {
            true
        } else {
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
            let v37: bool = v20.starts_with(&*v36);
            v37
        }
    } else {
        false
    };
    let v42: US1 = if v39 {
        US1::US1_GlobalDeclaration
    } else {
        US1::US1_GlobalBody
    };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("global"); } LIT.with(|lit| lit.clone()) };
    let v44: US0 = US0::US0_TermMacro;
    (v0, v43, v44, v2, true, v42)
}
fn place_of_8(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let v2: Rc<str> = Rc::<str>::from((v1).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nominal "); } LIT.with(|lit| lit.clone()) };
    let v4: bool = v0.starts_with(&*v3);
    let mut v7: bool = if v4 {
        true
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = v0.starts_with(&*v5);
        v6
    };
    let v10: bool = if v7 {
        true
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":! "); } LIT.with(|lit| lit.clone()) };
        let v9: bool = v0.starts_with(&*v8);
        v9
    };
    if v10 {
        US0::US0_TypeMacro
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let v13: bool = v2.ends_with(&*v12);
        let v16: bool = if v13 {
            true
        } else {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("->"); } LIT.with(|lit| lit.clone()) };
            let mut v15: bool = v2.ends_with(&*v14);
            v15
        };
        if v16 {
            US0::US0_TypeMacro
        } else {
            US0::US0_TermMacro
        }
    }
}
fn line_macros_7(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v5: i32 = (v2.clone().len() as i32);
        let mut v6: i32 = v5.wrapping_sub(v3);
        let mut v7: usize = ((v3) as usize);
        let v8: i32 = v3.wrapping_add(v6);
        let v9: usize = ((v8) as usize);
        let v10: Rc<str> = Rc::<str>::from(&(v2)[v7..v9]);
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
        let v12: Option<usize> = v10.find(&*v11);
        let v13: Option<usize> = method6(v12);
        let mut v14: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v15: Option<i32> = v13.map(|x| v14(x));
        let v17: i32 = v15.unwrap_or(-1i32);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$\""); } LIT.with(|lit| lit.clone()) };
        let v19: Option<usize> = v10.find(&*v18);
        let v20: Option<usize> = method6(v19);
        let mut v21: Option<i32> = v20.map(|x| v14(x));
        let v23: i32 = v21.unwrap_or(-1i32);
        let mut v24: bool = v17 < 0i32;
        let v29: i32 = if v24 {
            v23
        } else {
            let v25: bool = v23 < 0i32;
            let v27: bool = if v25 {
                true
            } else {
                let v26: bool = v17 < v23;
                v26
            };
            if v27 {
                v17
            } else {
                v23
            }
        };
        let v30: bool = v29 == v17;
        let v33: Rc<str> = if v30 {
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
            v31
        } else {
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            v32
        };
        let v34: bool = v29 < 0i32;
        if v34 {
            return v4;
        } else {
            let v35: i32 = v3.wrapping_add(v29);
            let v36: usize = ((0i32) as usize);
            let v37: usize = ((v35) as usize);
            let v38: Rc<str> = Rc::<str>::from(&(v2)[v36..v37]);
            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v41: Rc<str> = Rc::<str>::from(v38.replace(&*v39, &*v40));
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = Rc::<str>::from(v41.replace(&*v42, &*v40));
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v45: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v43.split(&*v44).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v46: i32 = (v45.borrow().len() as i32);
            let v47: i32 = v46.wrapping_sub(1i32);
            let v48: i32 = v47.wrapping_rem(2i32);
            let v49: bool = v48 == 1i32;
            if v49 {
                let v50: i32 = v35.wrapping_add(1i32);
                v3 = v50;
                continue;
            } else {
                let v52: usize = ((0i32) as usize);
                let v53: usize = ((v35) as usize);
                let v54: Rc<str> = Rc::<str>::from(&(v2)[v52..v53]);
                let v55: US0 = place_of_8(v2.clone(), v54);
                let v56: i32 = v35.wrapping_add(2i32);
                let v57: i32 = v5.wrapping_sub(v35);
                let v58: i32 = v57.wrapping_sub(2i32);
                let v59: usize = ((v56) as usize);
                let v60: i32 = v56.wrapping_add(v58);
                let v61: usize = ((v60) as usize);
                let v62: Rc<str> = Rc::<str>::from(&(v2)[v59..v61]);
                let v63: Option<usize> = v62.find(&*v33);
                let v64: Option<usize> = method6(v63);
                let mut v65: Option<i32> = v64.map(|x| v14(x));
                let v67: i32 = v65.unwrap_or(-1i32);
                let v68: bool = v67 < 0i32;
                if v68 {
                    let v69: usize = ((v35) as usize);
                    let v70: i32 = v35.wrapping_add(v57);
                    let v71: usize = ((v70) as usize);
                    let v72: Rc<str> = Rc::<str>::from(&(v2)[v69..v71]);
                    let v73: US1 = US1::US1_InlineMacro;
                    return Rc::new(UH0::UH0_Cons(v0, v1, v55, v72, false, v73, v4));
                } else {
                    let v75: i32 = v56.wrapping_add(v67);
                    let v76: i32 = v75.wrapping_sub(v35);
                    let v77: i32 = v76.wrapping_add(1i32);
                    let v78: usize = ((v35) as usize);
                    let v79: i32 = v35.wrapping_add(v77);
                    let v80: usize = ((v79) as usize);
                    let v81: Rc<str> = Rc::<str>::from(&(v2)[v78..v80]);
                    let v82: i32 = v75.wrapping_add(1i32);
                    let v83: bool = true;
                    let v84: US1 = US1::US1_InlineMacro;
                    let v85: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v1.clone(), v55, v81, v83, v84, v4));
                    (v3, v4) = (v82, v85);
                    continue;
                }
            }
        }
    }
}
fn macros_from_2(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v4 <= v1;
        if v5 {
            let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method3(v3, v6);
        } else {
            let v8: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v9: Rc<str> = definition_name_4(v8.clone(), v2);
            let v10: Rc<str> = Rc::<str>::from((v8).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("global "); } LIT.with(|lit| lit.clone()) };
            let v12: bool = v10.starts_with(&*v11);
            let mut v15: bool = if v12 {
                true
            } else {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("global("); } LIT.with(|lit| lit.clone()) };
                let mut v14: bool = v10.starts_with(&*v13);
                v14
            };
            let v18: bool = if v15 {
                true
            } else {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust_global"); } LIT.with(|lit| lit.clone()) };
                let v17: bool = v10.starts_with(&*v16);
                v17
            };
            if v18 {
                let v19: i32 = v1.wrapping_add(1i32);
                let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (i32, Rc<str>, US0, Rc<str>, bool, US1) = global_macro_5(v19, v8);
                let v26: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20, v21, v22, v23, v24, v25, v3));
                (v1, v2, v3) = (v19, v9, v26);
                continue;
            } else {
                let v28: i32 = v1.wrapping_add(1i32);
                let v29: i32 = 0i32;
                let v30: Rc<UH0> = line_macros_7(v28, v9.clone(), v8, v29, v3);
                (v1, v2, v3) = (v28, v9, v30);
                continue;
            }
        }
    }
}
fn file_macros_1(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = 0i32;
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    macros_from_2(v5, v6, v2, v7)
}
fn closure2() -> Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1| -> i32 {
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5, v6, v7, v8) => {
                let v2: i32 = v2;
                let mut v3: Rc<str> = v3;
                let v4: US0 = v4;
                let v5: Rc<str> = v5;
                let mut v6: bool = v6;
                let mut v7: US1 = v7;
                let v8: Rc<UH0> = v8;
                let v9: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v8, v9);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method11(mut v0: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32>, mut v1: &Rc<RefCell<Vec<i32>>>, mut v2: Rc<UH0>, mut v3: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5, v6, v7, v8, v9, v10) => {
                let v4: i32 = v4;
                let v5: Rc<str> = v5;
                let mut v6: US0 = v6;
                let mut v7: Rc<str> = v7;
                let v8: bool = v8;
                let v9: US1 = v9;
                let v10: Rc<UH0> = v10;
                let mut v11: i32 = v0(v4, v5, v6, v7, v8, v9);
                v1.borrow_mut()[v3 as usize] = v11;
                let v12: i32 = v3.wrapping_add(1i32);
                (v2, v3) = (v10, v12);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn filled_9(mut v0: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32>, mut v1: Rc<UH0>) -> Rc<RefCell<Vec<i32>>> {
    let v2: i32 = 0i32;
    let mut v3: i32 = method10(v1.clone(), v2);
    let v4: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![<i32>::default(); v3 as usize]));
    let v5: i32 = 0i32;
    let mut v6: i32 = method11(v0, &v4, v1, v5);
    v4
}
fn closure3() -> Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1| -> Rc<str> {
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method13(mut v0: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>>, mut v1: &Rc<RefCell<Vec<Rc<str>>>>, mut v2: Rc<UH0>, mut v3: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5, v6, v7, v8, v9, v10) => {
                let v4: i32 = v4;
                let v5: Rc<str> = v5;
                let mut v6: US0 = v6;
                let mut v7: Rc<str> = v7;
                let v8: bool = v8;
                let v9: US1 = v9;
                let v10: Rc<UH0> = v10;
                let mut v11: Rc<str> = v0(v4, v5, v6, v7, v8, v9);
                v1.borrow_mut()[v3 as usize] = v11;
                let v12: i32 = v3.wrapping_add(1i32);
                (v2, v3) = (v10, v12);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn filled_12(mut v0: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>>, mut v1: Rc<UH0>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v2: i32 = 0i32;
    let mut v3: i32 = method10(v1.clone(), v2);
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v3 as usize]));
    let v5: i32 = 0i32;
    let mut v6: i32 = method13(v0, &v4, v1, v5);
    v4
}
fn closure4() -> Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1| -> Rc<str> {
        v3
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> bool> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1| -> bool {
        v4
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15(mut v0: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> bool>, mut v1: &Rc<RefCell<Vec<bool>>>, mut v2: Rc<UH0>, mut v3: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5, v6, v7, v8, v9, v10) => {
                let v4: i32 = v4;
                let v5: Rc<str> = v5;
                let mut v6: US0 = v6;
                let mut v7: Rc<str> = v7;
                let v8: bool = v8;
                let v9: US1 = v9;
                let v10: Rc<UH0> = v10;
                let mut v11: bool = v0(v4, v5, v6, v7, v8, v9);
                v1.borrow_mut()[v3 as usize] = v11;
                let v12: i32 = v3.wrapping_add(1i32);
                (v2, v3) = (v10, v12);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn filled_14(mut v0: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> bool>, mut v1: Rc<UH0>) -> Rc<RefCell<Vec<bool>>> {
    let v2: i32 = 0i32;
    let mut v3: i32 = method10(v1.clone(), v2);
    let v4: Rc<RefCell<Vec<bool>>> = Rc::new(RefCell::new(vec![<bool>::default(); v3 as usize]));
    let v5: i32 = 0i32;
    let mut v6: i32 = method15(v0, &v4, v1, v5);
    v4
}
fn closure6() -> Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1| -> i32 {
        match v2 {
            US0::US0_TermMacro => {
                0i32
            }
            US0::US0_TypeMacro => {
                1i32
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1| -> i32 {
        match v5 {
            US1::US1_GlobalBody => {
                2i32
            }
            US1::US1_GlobalDeclaration => {
                1i32
            }
            US1::US1_InlineMacro => {
                0i32
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn scanned_0(mut v0: Rc<str>) -> (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) {
    let mut v1: Rc<UH0> = file_macros_1(v0);
    let mut v3: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> = closure2();
    let v4: Rc<RefCell<Vec<i32>>> = filled_9(v3, v1.clone());
    let mut v6: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>> = closure3();
    let mut v7: Rc<RefCell<Vec<Rc<str>>>> = filled_12(v6, v1.clone());
    let v8: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>> = closure4();
    let v9: Rc<RefCell<Vec<Rc<str>>>> = filled_12(v8, v1.clone());
    let mut v11: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> bool> = closure5();
    let v12: Rc<RefCell<Vec<bool>>> = filled_14(v11, v1.clone());
    let mut v14: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> = closure6();
    let mut v15: Rc<RefCell<Vec<i32>>> = filled_9(v14, v1.clone());
    let v17: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> = closure7();
    let v18: Rc<RefCell<Vec<i32>>> = filled_9(v17, v1);
    (v4, v7, v9, v12, v15, v18)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) {
        scanned_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH1> = v3;
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
fn added_from_17(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            return method18(v2, v5);
        } else {
            let mut v7: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v8: i32 = (v7.clone().len() as i32);
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("+"); } LIT.with(|lit| lit.clone()) };
            let v10: bool = v7.starts_with(&*v9);
            let mut v14: bool = if v10 {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("+++"); } LIT.with(|lit| lit.clone()) };
                let v12: bool = v7.starts_with(&*v11);
                let v13: bool = v12 == false;
                v13
            } else {
                false
            };
            let mut v15: i32 = v1.wrapping_add(1i32);
            let v22: Rc<UH1> = if v14 {
                let v16: i32 = v8.wrapping_sub(1i32);
                let v17: usize = ((1i32) as usize);
                let v18: i32 = 1i32.wrapping_add(v16);
                let v19: usize = ((v18) as usize);
                let v20: Rc<str> = Rc::<str>::from(&(v7)[v17..v19]);
                Rc::new(UH1::UH1_Cons(v20, v2))
            } else {
                v2
            };
            (v1, v2) = (v15, v22);
            continue;
        }
    }
}
fn method19(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
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
fn added_text_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = 0i32;
    let mut v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH1> = added_from_17(&v5, v6, v7);
    let v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v10: i32 = 0i32;
    let mut v11: i32 = method19(v9.clone(), v8, v10);
    let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
    let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
    let mut v14: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v4));
    v14
}
fn closure8() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        added_text_16(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure10(mut v0: i32, mut v1: Rc<str>, mut v2: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    Rc::new(move |mut v3: Rc<str>| -> Rc<str> {
        let v4: bool = v2 >= v0;
        if v4 {
            v3
        } else {
            let v5: i32 = v2.wrapping_add(1i32);
            let mut v6: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method22(v0, v1.clone(), v5);
            let mut v7: Rc<str> = string_concat(&[&*v3, &*v1]);
            v6(v7)
        }
    })
}
fn method22(mut v0: i32, mut v1: Rc<str>, mut v2: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    closure10(v0, v1, v2)
}
fn method24(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_23(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method24(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn violation_row_21(mut v0: i32, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: US2, mut v5: Rc<str>) -> Rc<str> {
    let mut v6: i32 = v0.wrapping_add(2i32);
    let mut v7: Rc<str> = Rc::<str>::from(((b' ') as char).to_string());
    let v8: i32 = (v3.clone().len() as i32);
    let v9: i32 = v6.wrapping_sub(v8);
    let v10: i32 = 0i32;
    let mut v11: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method22(v9, v7.clone(), v10);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = v11(v12.clone());
    let mut v14: Rc<str> = string_concat(&[&*v3, &*v13]);
    let v20: Rc<str> = match v4 {
        US2::US2_MacroArith => {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arith"); } LIT.with(|lit| lit.clone()) };
            v19
        }
        US2::US2_MacroBlock => {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("block"); } LIT.with(|lit| lit.clone()) };
            v16
        }
        US2::US2_MacroCast => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cast"); } LIT.with(|lit| lit.clone()) };
            v18
        }
        US2::US2_MacroFlow => {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("flow"); } LIT.with(|lit| lit.clone()) };
            v17
        }
        US2::US2_MacroLong => {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("long"); } LIT.with(|lit| lit.clone()) };
            v15
        }
    };
    let mut v21: i32 = (v20.clone().len() as i32);
    let v22: i32 = 7i32.wrapping_sub(v21);
    let v23: i32 = 0i32;
    let mut v24: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method22(v22, v7, v23);
    let v25: Rc<str> = v24(v12);
    let v28: Rc<str> = format_real_23(v2);
    let v33: Rc<str> = string_concat(&[&*v1, ":", &*v28, "  ", &*v14, &*v20, &*v25, &*v5]);
    v33
}
fn row_of_20(mut v0: i32, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>) -> Rc<str> {
    let mut v6: bool = v4 == 0i32;
    let v26: US3 = if v6 {
        let mut v7: US2 = US2::US2_MacroLong;
        US3::US3_Some(v7)
    } else {
        let v9: bool = v4 == 1i32;
        if v9 {
            let v10: US2 = US2::US2_MacroBlock;
            US3::US3_Some(v10)
        } else {
            let v12: bool = v4 == 2i32;
            if v12 {
                let v13: US2 = US2::US2_MacroFlow;
                US3::US3_Some(v13)
            } else {
                let mut v15: bool = v4 == 3i32;
                if v15 {
                    let v16: US2 = US2::US2_MacroCast;
                    US3::US3_Some(v16)
                } else {
                    let v18: bool = v4 == 4i32;
                    if v18 {
                        let v19: US2 = US2::US2_MacroArith;
                        US3::US3_Some(v19)
                    } else {
                        US3::US3_None
                    }
                }
            }
        }
    };
    match v26 {
        US3::US3_None => {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v29
        }
        US3::US3_Some(v27) => {
            let v27: US2 = v27;
            violation_row_21(v0, v1, v2, v3, v27, v5)
        }
    }
}
fn closure9() -> Rc<dyn Fn(i32, Rc<str>, i32, Rc<str>, i32, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, i32, Rc<str>, i32, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>| -> Rc<str> {
        row_of_20(v0, v1, v2, v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn tally_head_26(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let v4: bool = v3 == 0i32;
    let mut v7: Rc<str> = if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
        v6
    };
    let v9: Rc<str> = format_real_23(v0);
    let v12: Rc<str> = format_real_23(v1);
    let mut v15: Rc<str> = format_real_23(v2);
    let v18: Rc<str> = format_real_23(v3);
    let v19: Rc<str> = string_concat(&[&*v7, "  files ", &*v9, "  macros ", &*v12, "  violations ", &*v15, "  new ", &*v18]);
    v19
}
fn tally_text_25(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    tally_head_26(v0, v1, v2, v3)
}
fn closure11() -> Rc<dyn Fn(i32, i32, i32, i32) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32, i32, i32) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32| -> Rc<str> {
        tally_text_25(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_lint_scan(v0: &str) -> (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) {
    closure0()(Rc::<str>::from(v0))
}
pub fn rig_lint_added_text(v0: &str) -> Rc<str> {
    closure8()(Rc::<str>::from(v0))
}
pub fn rig_lint_row(v0: i32, v1: &str, v2: i32, v3: &str, v4: i32, v5: &str) -> Rc<str> {
    closure9()(v0, Rc::<str>::from(v1), v2, Rc::<str>::from(v3), v4, Rc::<str>::from(v5))
}
pub fn rig_lint_tally(v0: i32, v1: i32, v2: i32, v3: i32) -> Rc<str> {
    closure11()(v0, v1, v2, v3)
}
pub type SpiralShape_97df63ebabc3 = Mut0;
