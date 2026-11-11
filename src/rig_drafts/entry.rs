#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
fn command_line_argument_count_2() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_1(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_2();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_3(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_1(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn drafts_run_0() -> i32 {
    let v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_1(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    let v6: bool = &*v4 == "list";
                    if v6 {
                        let v7: i32 = rig_drafts_list::rig_drafts_list_run();
                        v7
                    } else {
                        let v8: bool = &*v4 == "put";
                        if v8 {
                            let v9: i32 = rig_drafts_put::rig_drafts_put_run();
                            v9
                        } else {
                            let v10: bool = &*v4 == "promote";
                            if v10 {
                                let v11: i32 = rig_drafts_promote::rig_drafts_promote_run();
                                v11
                            } else {
                                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: drafts put|list|promote"); } LIT.with(|lit| lit.clone()) };
                                eprintln!("{}", v12);
                                2i32
                            }
                        }
                    }
                }
                _ => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: drafts put|list|promote"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v16);
                    2i32
                }
            }
        }
        _ => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rig error: drafts put|list|promote"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v18);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        drafts_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_drafts_run() -> i32 {
    closure0()()
}
