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
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US0 {
    US0_NextText,
    US0_NextJson,
}
fn method2(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_1(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method2(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn row_json_4(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: bool, mut v7: i32, mut v8: i32, mut v9: Rc<str>) -> Rc<str> {
    let v10: Rc<str> = format_real_1(v0);
    let v13: Rc<str> = string_concat(&["{\"rank\":", &*v10, ",\"id\":\"", &*v1]);
    let v14: Rc<str> = format_real_1(v3);
    let v15: Rc<str> = string_concat(&["\",\"priority\":", &*v14]);
    let v16: Rc<str> = format_real_1(v4);
    let v19: Rc<str> = format_real_1(v5);
    let v21: Rc<str> = string_concat(&[",\"difficulty\":", &*v16, ",\"minutes\":", &*v19, ",\"estimated\":"]);
    let v24: Rc<str> = if v6 {
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("true"); } LIT.with(|lit| lit.clone()) };
        v22
    } else {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("false"); } LIT.with(|lit| lit.clone()) };
        v23
    };
    let v26: Rc<str> = format_real_1(v8);
    let v29: Rc<str> = format_real_1(v7);
    let v38: Rc<str> = string_concat(&[&*v13, &*v15, &*v21, &*v24, ",\"wsjf\":", &*v26, ",\"unblocks\":", &*v29, ",\"parent\":\"", &*v9, "\",\"title\":\"", &*v2, "\"}"]);
    v38
}
fn json_items_3(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v1.wrapping_add(1i32);
            let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>) = v0.borrow()[v1 as usize].clone();
            let v15: Rc<str> = row_json_4(v5, v6, v7, v8, v9, v10, v11, v12, v13, v14);
            let v16: bool = &*v2 == "";
            let v19: Rc<str> = if v16 {
                v15
            } else {
                let v18: Rc<str> = string_concat(&[&*v2, ",", &*v15]);
                v18
            };
            (v1, v2) = (v5, v19);
            continue;
        }
    }
}
fn spaces_7(mut v0: i32) -> Rc<str> {
    let v1: bool = v0 <= 0i32;
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: i32 = v0.wrapping_sub(1i32);
        let v4: Rc<str> = spaces_7(v3);
        let v5: Rc<str> = string_concat(&[" ", &*v4]);
        v5
    }
}
fn row_line_6(mut v0: bool, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: bool, mut v8: i32, mut v9: i32, mut v10: Rc<str>) -> Rc<str> {
    let v11: Rc<str> = format_real_1(v6);
    let v12: Rc<str> = string_concat(&[&*v11, "m"]);
    let v15: Rc<str> = if v7 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        v13
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
        v14
    };
    let v16: Rc<str> = string_concat(&[&*v12, &*v15]);
    let v24: Rc<str> = if v0 {
        let v17: Rc<str> = format_real_1(v9);
        let v18: i32 = (v17.clone().len() as i32);
        let v19: i32 = 7i32.wrapping_sub(v18);
        let v20: Rc<str> = spaces_7(v19);
        let v22: Rc<str> = string_concat(&["  wsjf=", &*v17, &*v20]);
        v22
    } else {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v23
    };
    let v25: Rc<str> = format_real_1(v1);
    let v26: i32 = (v25.clone().len() as i32);
    let v27: i32 = 3i32.wrapping_sub(v26);
    let v28: Rc<str> = spaces_7(v27);
    let v31: i32 = (v2.clone().len() as i32);
    let v32: i32 = 28i32.wrapping_sub(v31);
    let v33: Rc<str> = spaces_7(v32);
    let v37: Rc<str> = format_real_1(v4);
    let v38: Rc<str> = string_concat(&[&*v28, &*v25, "  ", &*v2, &*v33, " P", &*v37]);
    let v39: Rc<str> = format_real_1(v5);
    let v42: i32 = (v16.clone().len() as i32);
    let v43: i32 = 7i32.wrapping_sub(v42);
    let v44: Rc<str> = spaces_7(v43);
    let v47: Rc<str> = string_concat(&[" D", &*v39, "  ", &*v44, &*v16, &*v24]);
    let v48: bool = &*v10 == "";
    let v51: Rc<str> = if v48 {
        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v49
    } else {
        let v50: Rc<str> = string_concat(&["  parent=", &*v10]);
        v50
    };
    let v52: Rc<str> = format_real_1(v8);
    let v53: i32 = (v52.clone().len() as i32);
    let v54: i32 = 5i32.wrapping_sub(v53);
    let v55: Rc<str> = spaces_7(v54);
    let v62: Rc<str> = { let text = &*v3; if text.chars().count() <= 72i32 as usize { std::rc::Rc::<str>::from(text) } else { std::rc::Rc::<str>::from(text.chars().take((72i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
    let v63: Rc<str> = string_concat(&[&*v38, &*v47, "  unblocks=", &*v52, &*v55, &*v51, "  ", &*v62]);
    v63
}
fn lines_printed_5(mut v0: bool, mut v1: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>>>, mut v2: i32) -> () {
    loop {
        let v3: i32 = (v1.clone().borrow().len() as i32);
        let v4: bool = v2 >= v3;
        if v4 {
            return ();
        } else {
            let v5: i32 = v2.wrapping_add(1i32);
            let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>) = v1.borrow()[v2 as usize].clone();
            let v15: Rc<str> = row_line_6(v0, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14);
            println!("{}", v15);
            v2 = v5;
            continue;
        }
        return ();
    }
}
fn format_real_8(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method2(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn next_printed_0(mut v0: i32, mut v1: u64, mut v2: Rc<str>, mut v3: bool, mut v4: bool, mut v5: Rc<str>, mut v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>>>) -> i32 {
    let v7: i32 = (v6.clone().borrow().len() as i32);
    let v8: Rc<str> = format_real_1(v7);
    let v11: US0 = if v3 {
        US0::US0_NextJson
    } else {
        US0::US0_NextText
    };
    match v11 {
        US0::US0_NextJson => {
            let v14: Rc<str> = format_real_1(v0);
            let v19: i32 = 0i32;
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = json_items_3(&v6, v19, v20);
            let v23: Rc<str> = string_concat(&["{\"role\":\"", &*v5, "\",\"ready\":", &*v14, ",\"shown\":", &*v8, ",\"items\":[", &*v21, "]}"]);
            println!("{}", v23);
            ()
        }
        US0::US0_NextText => {
            let v24: i32 = 0i32;
            lines_printed_5(v4, &v6, v24);
            let v25: bool = &*v2 == "all";
            let v28: Rc<str> = if v25 {
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v26
            } else {
                let v27: Rc<str> = string_concat(&[" load=", &*v2]);
                v27
            };
            let v29: Rc<str> = format_real_1(v0);
            let v34: Rc<str> = format_real_8(v1);
            let v38: Rc<str> = string_concat(&["rig agile next ok role=", &*v5, " ready=", &*v29, " shown=", &*v8, &*v28, " elapsed_ms=", &*v34]);
            println!("{}", v38);
            ()
        }
    };
    0i32
}
fn closure0() -> Rc<dyn Fn(Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>>>, Rc<str>, bool, bool, Rc<str>, u64, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>>>, Rc<str>, bool, bool, Rc<str>, u64, i32) -> i32> = Rc::new(move |mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>>>, mut v1: Rc<str>, mut v2: bool, mut v3: bool, mut v4: Rc<str>, mut v5: u64, mut v6: i32| -> i32 {
        next_printed_0(v6, v5, v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_next_printed(v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<str>)>>>, v1: &str, v2: bool, v3: bool, v4: &str, v5: u64, v6: i32) -> i32 {
    closure0()(v0, Rc::<str>::from(v1), v2, v3, Rc::<str>::from(v4), v5, v6)
}
pub type SpiralShape_97df63ebabc3 = Mut0;
