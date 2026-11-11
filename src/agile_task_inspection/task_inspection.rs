#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;

pub fn spiral_json_quote(s: &str) -> std::rc::Rc<str> {
    let (q, b) = (char::from(34u8), char::from(92u8));
    let mut out = String::from(q);
    for c in s.chars() {
        match c as u32 {
            34 | 92 => { out.push(b); out.push(c); }
            n @ (8 | 9 | 10 | 12 | 13) => { out.push(b); out.push(char::from(b"btn_fr"[(n - 8) as usize])); }
            n if n < 32 => out.push_str(&format!("{b}u{n:04x}")),
            _ => out.push(c),
        }
    }
    out.push(q);
    std::rc::Rc::from(out.as_str())
}

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
enum US0 {
    US0_ShowText,
    US0_ShowJson,
    US0_ShowFiltered(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_FileReadable(Rc<str>),
    US1_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_StateFile(Rc<str>),
    US2_HistoryFile(Rc<str>),
    US2_ArchiveFile(Rc<str>),
    US2_OtherFile,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US4 {
    US4_HeadInl,
    US4_HeadUnion,
    US4_HeadOpen,
    US4_HeadType,
}
#[derive(Clone)]
enum US3 {
    US3_KeywordHead(US4),
    US3_SymbolHead(US4),
    US3_OtherLine,
}
#[derive(Clone)]
enum US5 {
    US5_ScanPlain,
    US5_ScanQuoted,
    US5_ScanEscaped,
}
#[derive(Clone)]
enum US6 {
    US6_Valid(US0),
    US6_Invalid,
}
fn task_summary_3(mut v0: Rc<str>, mut v1: &Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: &Rc<str>, mut v8: &Rc<str>) -> Rc<str> {
    let v18: Rc<str> = string_concat(&[&*v0, "  ", &*v2, "  ", &*v3, "  ", &*v4, "  ", &*v5, "%  ", &*v6]);
    v18
}
fn show_receipt_4(mut v0: Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: &Rc<str>, mut v4: &Rc<str>, mut v5: &Rc<str>, mut v6: &Rc<str>, mut v7: &Rc<str>, mut v8: &Rc<str>, mut v9: u64) -> Rc<str> {
    let v12: i32 = (v1.clone().len() as i32);
    let mut v13: Rc<str> = std::rc::Rc::<str>::from((v12).to_string());
    let mut v16: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v9);
    let mut v17: Rc<str> = std::rc::Rc::<str>::from((v16).to_string());
    let v18: Rc<str> = string_concat(&["rig agile show ok id=", &*v0, " title_bytes=", &*v13, " elapsed_ms=", &*v17]);
    v18
}
fn show_task_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: u64) -> i32 {
    let v12: Rc<str> = task_summary_3(v0.clone(), &v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), &v7, &v8);
    let mut v13: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v12.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v14: bool = &*v7 == "";
    let mut v16: Rc<str> = if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        v15
    } else {
        v7.clone()
    };
    let mut v17: Rc<str> = string_concat(&["deps: ", &*v16]);
    let v18: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v17.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v19: bool = &*v8 == "";
    let v21: Rc<str> = if v19 {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        v20
    } else {
        v8.clone()
    };
    let mut v22: Rc<str> = string_concat(&["tags: ", &*v21]);
    let v23: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v22.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v24: Rc<str> = string_concat(&["planning: ", &*v9]);
    let mut v25: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v24.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v26: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v1.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v27: bool = &*v10 == "";
    let v29: i32 = if v27 {
        0i32
    } else {
        let v28: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v10.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        v28
    };
    let v30: Rc<str> = show_receipt_4(v0, &v1, &v2, &v3, &v4, &v5, &v6, &v7, &v8, v11);
    let v31: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v30.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v31
}
fn concat5_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = string_concat(&[&*v0, &*v1, &*v2, &*v3, &*v4]);
    v8
}
fn loop_6(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method7(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_8(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(6i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_9(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_10(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(8i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method13(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method13(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method12(v16, v17)
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method14(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            let v15: US3 = if v6 {
                let v7: US4 = US4::US4_HeadType;
                US3::US3_KeywordHead(v7)
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v2.starts_with(&*v9);
                if v10 {
                    let v11: US4 = US4::US4_HeadType;
                    US3::US3_SymbolHead(v11)
                } else {
                    US3::US3_OtherLine
                }
            };
            let v27: US3 = match &v15 {
                US3::US3_OtherLine => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v2.starts_with(&*v16);
                    if v17 {
                        let v18: US4 = US4::US4_HeadInl;
                        US3::US3_KeywordHead(v18)
                    } else {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = v2.starts_with(&*v20);
                        if v21 {
                            let mut v22: US4 = US4::US4_HeadInl;
                            US3::US3_SymbolHead(v22)
                        } else {
                            US3::US3_OtherLine
                        }
                    }
                }
                _ => {
                    v15
                }
            };
            let v39: US3 = match &v27 {
                US3::US3_OtherLine => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union "); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v2.starts_with(&*v28);
                    if v29 {
                        let v30: US4 = US4::US4_HeadUnion;
                        US3::US3_KeywordHead(v30)
                    } else {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = v2.starts_with(&*v32);
                        if v33 {
                            let v34: US4 = US4::US4_HeadUnion;
                            US3::US3_SymbolHead(v34)
                        } else {
                            US3::US3_OtherLine
                        }
                    }
                }
                _ => {
                    v27
                }
            };
            let v51: US3 = match &v39 {
                US3::US3_OtherLine => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v2.starts_with(&*v40);
                    if v41 {
                        let v42: US4 = US4::US4_HeadOpen;
                        US3::US3_KeywordHead(v42)
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v2.starts_with(&*v44);
                        if v45 {
                            let v46: US4 = US4::US4_HeadOpen;
                            US3::US3_SymbolHead(v46)
                        } else {
                            US3::US3_OtherLine
                        }
                    }
                }
                _ => {
                    v39
                }
            };
            let v76: Rc<str> = match v51 {
                US3::US3_SymbolHead(v52) => {
                    let v52: US4 = v52;
                    let v60: Rc<str> = match &v52 {
                        US4::US4_HeadInl => {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            v53
                        }
                        US4::US4_HeadOpen => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                            v55
                        }
                        US4::US4_HeadType => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                            v56
                        }
                        US4::US4_HeadUnion => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                            v54
                        }
                    };
                    let v61: i32 = (v60.len() as i32);
                    let v68: Rc<str> = match v52 {
                        US4::US4_HeadInl => {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                            v62
                        }
                        US4::US4_HeadOpen => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                            v64
                        }
                        US4::US4_HeadType => {
                            v5
                        }
                        US4::US4_HeadUnion => {
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
fn method15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
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
fn archived_text_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = string_concat(&[&*v0, "/archive/done-notes.spi"]);
    let v2: bool = std::path::Path::new(&*v1).is_file();
    let v5: US1 = if v2 {
        US1::US1_FileReadable(v1)
    } else {
        US1::US1_FileMissing(v1)
    };
    match v5 {
        US1::US1_FileMissing(v6) => {
            let v6: Rc<str> = v6;
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US1::US1_FileReadable(v8) => {
            let v8: Rc<str> = v8;
            let mut v9: Rc<str> = std::fs::read_to_string(&*v8).unwrap_or_default().into();
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = Rc::<str>::from(v8.replace(&*v10, &*v11));
            let mut v13: Rc<str> = loop_6(v12);
            let v14: Option<usize> = v13.rfind(&*v11);
            let v15: Option<usize> = method7(v14);
            let mut v16: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v17: Option<i32> = v15.map(|x| v16(x));
            let v19: i32 = v17.unwrap_or(-1i32);
            let v20: bool = v19 < 0i32;
            let v24: bool = if v20 {
                true
            } else {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let mut v22: bool = v13.ends_with(&*v21);
                let v23: bool = v22 == false;
                v23
            };
            let v45: US2 = if v24 {
                US2::US2_OtherFile
            } else {
                let mut v26: usize = ((0i32) as usize);
                let v27: usize = ((v19) as usize);
                let v28: Rc<str> = Rc::<str>::from(&(v13)[v26..v27]);
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v30: bool = v28.ends_with(&*v29);
                if v30 {
                    let v31: Rc<str> = loop_8(v28);
                    US2::US2_StateFile(v31)
                } else {
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v34: bool = v28.ends_with(&*v33);
                    if v34 {
                        let v35: Rc<str> = loop_9(v28);
                        US2::US2_HistoryFile(v35)
                    } else {
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v38: bool = v28.ends_with(&*v37);
                        if v38 {
                            let v39: Rc<str> = loop_10(v28);
                            US2::US2_ArchiveFile(v39)
                        } else {
                            US2::US2_OtherFile
                        }
                    }
                }
            };
            let v56: Rc<str> = match v45 {
                US2::US2_OtherFile => {
                    v9
                }
                _ => {
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v47: Rc<UH0> = split_list_11(v46.clone(), v9);
                    let v48: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v49: Rc<UH0> = method14(v47, v48);
                    let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v51: i32 = 0i32;
                    let v52: i32 = method15(v50.clone(), v49, v51);
                    let v53: Rc<Vec<Rc<str>>> = Rc::new(v50.borrow().clone());
                    let v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v53).as_ref().clone()));
                    let v55: Rc<str> = Rc::<str>::from((*v54.borrow()).join(&*v46));
                    v55
                }
            };
            let v57: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v56);
            v57
        }
    }
}
fn state_segments_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = string_concat(&[&*v0, "/state"]);
    let v2: Rc<str> = agile_note_listing::rig_agile_state_segments(&*v1);
    v2
}
fn notes_joined_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v0 == "";
    if v2 {
        v1
    } else {
        let v3: bool = &*v1 == "";
        if v3 {
            v0
        } else {
            let v5: Rc<str> = string_concat(&[&*v0, "\n", &*v1]);
            v5
        }
    }
}
fn field_end_21(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US5) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US5::US5_ScanPlain => {
                    let v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let v10: i32 = v1.wrapping_add(1i32);
                let mut v25: US5 = match v3 {
                    US5::US5_ScanEscaped => {
                        US5::US5_ScanQuoted
                    }
                    US5::US5_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US5::US5_ScanQuoted
                        } else {
                            US5::US5_ScanPlain
                        }
                    }
                    US5::US5_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US5::US5_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US5::US5_ScanPlain
                            } else {
                                US5::US5_ScanQuoted
                            }
                        }
                    }
                };
                (v1, v3) = (v10, v25);
                continue;
            }
        }
    }
}
fn skip_spaces_22(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_20(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US5 = US5::US5_ScanPlain;
            let v6: i32 = field_end_21(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_22(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn unescaped_from_24(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 <= v1;
        if v5 {
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = string_concat(&[&*v4, &*v6]);
            return v7;
        } else {
            let v8: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = v8 == 92i32;
            if v9 {
                let v10: i32 = v1.wrapping_add(1i32);
                let v11: i32 = v10.wrapping_add(1i32);
                let v12: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v13: Rc<str> = string_concat(&[&*v4, &*v12]);
                let v14: i32 = v0.as_bytes().get(v10 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v15: bool = v14 == 110i32;
                let v24: Rc<str> = if v15 {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16
                } else {
                    let mut v17: bool = v14 == 114i32;
                    if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18
                    } else {
                        let v19: bool = v14 == 116i32;
                        if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20
                        } else {
                            let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v11 as usize).unwrap_or(""));
                            v21
                        }
                    }
                };
                let mut v25: Rc<str> = string_concat(&[&*v13, &*v24]);
                (v1, v2, v4) = (v11, v11, v25);
                continue;
            } else {
                let v27: i32 = v1.wrapping_add(1i32);
                v1 = v27;
                continue;
            }
        }
    }
}
fn unquoted_23(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_24(v3, v4, v5, v6, v7)
}
fn task_record_at_19(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v3: i32 = 0i32;
    let v4: i32 = field_start_20(v0.clone(), v1, v2, v3);
    let v5: US5 = US5::US5_ScanPlain;
    let v6: i32 = field_end_21(v0.clone(), v4, v2, v5);
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let v8: Rc<str> = unquoted_23(v7);
    let mut v9: i32 = 1i32;
    let v10: i32 = field_start_20(v0.clone(), v1, v2, v9);
    let v11: US5 = US5::US5_ScanPlain;
    let v12: i32 = field_end_21(v0.clone(), v10, v2, v11);
    let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v12 as usize).unwrap_or(""));
    let v14: Rc<str> = unquoted_23(v13);
    let v15: i32 = 2i32;
    let mut v16: i32 = field_start_20(v0.clone(), v1, v2, v15);
    let mut v17: US5 = US5::US5_ScanPlain;
    let v18: i32 = field_end_21(v0.clone(), v16, v2, v17);
    let v19: Rc<str> = std::rc::Rc::<str>::from(v0.get(v16 as usize..v18 as usize).unwrap_or(""));
    let v20: i32 = 3i32;
    let v21: i32 = field_start_20(v0.clone(), v1, v2, v20);
    let mut v22: US5 = US5::US5_ScanPlain;
    let v23: i32 = field_end_21(v0.clone(), v21, v2, v22);
    let v24: Rc<str> = std::rc::Rc::<str>::from(v0.get(v21 as usize..v23 as usize).unwrap_or(""));
    let mut v25: i32 = 4i32;
    let mut v26: i32 = field_start_20(v0.clone(), v1, v2, v25);
    let v27: US5 = US5::US5_ScanPlain;
    let v28: i32 = field_end_21(v0.clone(), v26, v2, v27);
    let v29: Rc<str> = std::rc::Rc::<str>::from(v0.get(v26 as usize..v28 as usize).unwrap_or(""));
    let v30: i32 = 5i32;
    let v31: i32 = field_start_20(v0.clone(), v1, v2, v30);
    let v32: US5 = US5::US5_ScanPlain;
    let v33: i32 = field_end_21(v0.clone(), v31, v2, v32);
    let v34: Rc<str> = std::rc::Rc::<str>::from(v0.get(v31 as usize..v33 as usize).unwrap_or(""));
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v37: Rc<str> = Rc::<str>::from(v34.replace(&*v35, &*v36));
    let v38: i32 = 6i32;
    let v39: i32 = field_start_20(v0.clone(), v1, v2, v38);
    let v40: US5 = US5::US5_ScanPlain;
    let v41: i32 = field_end_21(v0.clone(), v39, v2, v40);
    let v42: Rc<str> = std::rc::Rc::<str>::from(v0.get(v39 as usize..v41 as usize).unwrap_or(""));
    let v43: i32 = 7i32;
    let v44: i32 = field_start_20(v0.clone(), v1, v2, v43);
    let v45: US5 = US5::US5_ScanPlain;
    let v46: i32 = field_end_21(v0.clone(), v44, v2, v45);
    let v47: Rc<str> = std::rc::Rc::<str>::from(v0.get(v44 as usize..v46 as usize).unwrap_or(""));
    let v48: Rc<str> = unquoted_23(v47);
    let v49: i32 = 8i32;
    let v50: i32 = field_start_20(v0.clone(), v1, v2, v49);
    let v51: US5 = US5::US5_ScanPlain;
    let v52: i32 = field_end_21(v0.clone(), v50, v2, v51);
    let v53: Rc<str> = std::rc::Rc::<str>::from(v0.get(v50 as usize..v52 as usize).unwrap_or(""));
    let v54: Rc<str> = unquoted_23(v53);
    (v8, v14, v19, v24, v29, v37, v42, v48, v54)
}
fn notes_or_archived_25(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v1 == "";
    if v2 {
        v0
    } else {
        v1
    }
}
fn task_json_record_at_26(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v3: i32 = 0i32;
    let v4: i32 = field_start_20(v0.clone(), v1, v2, v3);
    let v5: US5 = US5::US5_ScanPlain;
    let v6: i32 = field_end_21(v0.clone(), v4, v2, v5);
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let v8: i32 = (v7.clone().len() as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let v11: i32 = 1i32;
    let v12: i32 = field_start_20(v0.clone(), v1, v2, v11);
    let mut v13: US5 = US5::US5_ScanPlain;
    let v14: i32 = field_end_21(v0.clone(), v12, v2, v13);
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let mut v16: i32 = (v15.clone().len() as i32);
    let mut v17: i32 = v16.wrapping_sub(1i32);
    let v18: Rc<str> = std::rc::Rc::<str>::from(v15.get(1i32 as usize..v17 as usize).unwrap_or(""));
    let v19: i32 = 2i32;
    let v20: i32 = field_start_20(v0.clone(), v1, v2, v19);
    let v21: US5 = US5::US5_ScanPlain;
    let mut v22: i32 = field_end_21(v0.clone(), v20, v2, v21);
    let v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let v24: i32 = 3i32;
    let mut v25: i32 = field_start_20(v0.clone(), v1, v2, v24);
    let mut v26: US5 = US5::US5_ScanPlain;
    let v27: i32 = field_end_21(v0.clone(), v25, v2, v26);
    let v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let v29: i32 = 4i32;
    let v30: i32 = field_start_20(v0.clone(), v1, v2, v29);
    let v31: US5 = US5::US5_ScanPlain;
    let v32: i32 = field_end_21(v0.clone(), v30, v2, v31);
    let v33: Rc<str> = std::rc::Rc::<str>::from(v0.get(v30 as usize..v32 as usize).unwrap_or(""));
    let v34: i32 = 5i32;
    let v35: i32 = field_start_20(v0.clone(), v1, v2, v34);
    let v36: US5 = US5::US5_ScanPlain;
    let v37: i32 = field_end_21(v0.clone(), v35, v2, v36);
    let v38: Rc<str> = std::rc::Rc::<str>::from(v0.get(v35 as usize..v37 as usize).unwrap_or(""));
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v41: Rc<str> = Rc::<str>::from(v38.replace(&*v39, &*v40));
    let v42: i32 = 6i32;
    let v43: i32 = field_start_20(v0.clone(), v1, v2, v42);
    let v44: US5 = US5::US5_ScanPlain;
    let v45: i32 = field_end_21(v0.clone(), v43, v2, v44);
    let v46: Rc<str> = std::rc::Rc::<str>::from(v0.get(v43 as usize..v45 as usize).unwrap_or(""));
    let v47: i32 = 7i32;
    let v48: i32 = field_start_20(v0.clone(), v1, v2, v47);
    let v49: US5 = US5::US5_ScanPlain;
    let v50: i32 = field_end_21(v0.clone(), v48, v2, v49);
    let v51: Rc<str> = std::rc::Rc::<str>::from(v0.get(v48 as usize..v50 as usize).unwrap_or(""));
    let v52: i32 = (v51.clone().len() as i32);
    let v53: i32 = v52.wrapping_sub(1i32);
    let v54: Rc<str> = std::rc::Rc::<str>::from(v51.get(1i32 as usize..v53 as usize).unwrap_or(""));
    let v55: i32 = 8i32;
    let v56: i32 = field_start_20(v0.clone(), v1, v2, v55);
    let v57: US5 = US5::US5_ScanPlain;
    let v58: i32 = field_end_21(v0.clone(), v56, v2, v57);
    let v59: Rc<str> = std::rc::Rc::<str>::from(v0.get(v56 as usize..v58 as usize).unwrap_or(""));
    let v60: i32 = (v59.clone().len() as i32);
    let v61: i32 = v60.wrapping_sub(1i32);
    let v62: Rc<str> = std::rc::Rc::<str>::from(v59.get(1i32 as usize..v61 as usize).unwrap_or(""));
    (v10, v18, v23, v28, v33, v41, v46, v54, v62)
}
fn json_notes_of_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = agile_note_listing::rig_agile_notes_json(&*v1, &*v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = agile_note_listing::rig_agile_notes_json(&*v4, &*v2);
    let v6: bool = &*v3 == &*v5;
    if v6 {
        let v7: Rc<str> = agile_note_listing::rig_agile_notes_json(&*v0, &*v2);
        v7
    } else {
        v3
    }
}
fn task_json_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: u64) -> Rc<str> {
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = spiral_json_quote(&*v0);
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = spiral_json_quote(&*v1);
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = spiral_json_quote(&*v2);
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority"); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = spiral_json_quote(&*v3);
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<str> = spiral_json_quote(&*v4);
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let v51: Rc<str> = spiral_json_quote(&*v6);
    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
    let v58: Rc<str> = spiral_json_quote(&*v7);
    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags"); } LIT.with(|lit| lit.clone()) };
    let v64: Rc<str> = spiral_json_quote(&*v8);
    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elapsed_ms"); } LIT.with(|lit| lit.clone()) };
    let v72: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v10);
    let v73: Rc<str> = std::rc::Rc::<str>::from((v72).to_string());
    let v82: Rc<str> = string_concat(&["{\"", &*v11, "\":", &*v12, ",\"", &*v18, "\":", &*v19, ",\"", &*v25, "\":", &*v26, ",\"", &*v31, "\":", &*v32, ",\"", &*v38, "\":", &*v39, ",\"", &*v44, "\":", &*v5, ",\"", &*v50, "\":", &*v51, ",\"", &*v57, "\":", &*v58, ",\"", &*v63, "\":", &*v64, ",", &*v9, ",\"", &*v71, "\":", &*v73, "}"]);
    v82
}
fn agile_show_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US0) -> i32 {
    let v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_5(v0.clone(), v4, v5.clone(), v5.clone(), v5.clone());
    let v7: bool = std::path::Path::new(&*v6).is_file();
    let v10: US1 = if v7 {
        US1::US1_FileReadable(v6)
    } else {
        US1::US1_FileMissing(v6)
    };
    match v10 {
        US1::US1_FileMissing(v11) => {
            let v11: Rc<str> = v11;
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = string_concat(&["agile show ", &*v1, " rejected: ", &*v12]);
            let mut v16: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v15 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v16
        }
        US1::US1_FileReadable(v17) => {
            let mut v17: Rc<str> = v17;
            let v18: Rc<str> = std::fs::read_to_string(&*v17).unwrap_or_default().into();
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = Rc::<str>::from(v17.replace(&*v19, &*v20));
            let mut v22: Rc<str> = loop_6(v21);
            let v23: Option<usize> = v22.rfind(&*v20);
            let v24: Option<usize> = method7(v23);
            let mut v25: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v26: Option<i32> = v24.map(|x| v25(x));
            let v28: i32 = v26.unwrap_or(-1i32);
            let v29: bool = v28 < 0i32;
            let v33: bool = if v29 {
                true
            } else {
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let v31: bool = v22.ends_with(&*v30);
                let v32: bool = v31 == false;
                v32
            };
            let v54: US2 = if v33 {
                US2::US2_OtherFile
            } else {
                let v35: usize = ((0i32) as usize);
                let v36: usize = ((v28) as usize);
                let v37: Rc<str> = Rc::<str>::from(&(v22)[v35..v36]);
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v39: bool = v37.ends_with(&*v38);
                if v39 {
                    let v40: Rc<str> = loop_8(v37);
                    US2::US2_StateFile(v40)
                } else {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/history"); } LIT.with(|lit| lit.clone()) };
                    let v43: bool = v37.ends_with(&*v42);
                    if v43 {
                        let v44: Rc<str> = loop_9(v37);
                        US2::US2_HistoryFile(v44)
                    } else {
                        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) };
                        let v47: bool = v37.ends_with(&*v46);
                        if v47 {
                            let v48: Rc<str> = loop_10(v37);
                            US2::US2_ArchiveFile(v48)
                        } else {
                            US2::US2_OtherFile
                        }
                    }
                }
            };
            let v65: Rc<str> = match v54 {
                US2::US2_OtherFile => {
                    v18
                }
                _ => {
                    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v56: Rc<UH0> = split_list_11(v55.clone(), v18);
                    let v57: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v58: Rc<UH0> = method14(v56, v57);
                    let v59: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v60: i32 = 0i32;
                    let v61: i32 = method15(v59.clone(), v58, v60);
                    let v62: Rc<Vec<Rc<str>>> = Rc::new(v59.borrow().clone());
                    let v63: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v62).as_ref().clone()));
                    let v64: Rc<str> = Rc::<str>::from((*v63.borrow()).join(&*v55));
                    v64
                }
            };
            let v66: Rc<str> = rig_agile_records::rig_agile_records_as_lines(&*v65);
            let v67: Rc<str> = archived_text_16(v0.clone());
            let v68: Rc<str> = state_segments_17(v0);
            let v69: Rc<str> = notes_joined_18(v66.clone(), v68);
            let v71: Rc<str> = string_concat(&["Task (\"", &*v1, "\","]);
            let v72: i32 = v66.find(&*v71).map(|offset| offset as i32).unwrap_or(-1);
            let v73: bool = v72 < 0i32;
            if v73 {
                let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                let v77: Rc<str> = string_concat(&["agile show ", &*v1, " rejected: ", &*v74]);
                let v78: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v77 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v78
            } else {
                let v79: i32 = v72.wrapping_add(6i32);
                match v2 {
                    US0::US0_ShowFiltered(v95, v96) => {
                        let v95: Rc<str> = v95;
                        let v96: Rc<str> = v96;
                        let v97: i32 = (v66.clone().len() as i32);
                        let (mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = task_record_at_19(v66.clone(), v79, v97);
                        let v107: Rc<str> = agile_planning_fields::rig_agile_planning_summary(&*v66, &*v1);
                        let v108: Rc<str> = agile_note_listing::rig_agile_notes_filtered_text(&*v67, &*v1, &*v95, &*v96);
                        let v109: Rc<str> = agile_note_listing::rig_agile_notes_filtered_text(&*v69, &*v1, &*v95, &*v96);
                        let v110: Rc<str> = notes_or_archived_25(v108, v109);
                        show_task_2(v98, v99, v100, v101, v102, v103, v104, v105, v106, v107, v110, v3)
                    }
                    US0::US0_ShowJson => {
                        let v112: i32 = (v66.clone().len() as i32);
                        let (mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = task_json_record_at_26(v66.clone(), v79, v112);
                        let v122: Rc<str> = agile_planning_fields::rig_agile_planning_json(&*v66, &*v1);
                        let v124: Rc<str> = json_notes_of_27(v67, v69, v1);
                        let v125: Rc<str> = string_concat(&[&*v122, ",", &*v124]);
                        let v126: Rc<str> = task_json_28(v113, v114, v115, v116, v117, v118, v119, v120, v121, v125, v3);
                        let v127: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v126.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v127
                    }
                    US0::US0_ShowText => {
                        let v80: i32 = (v66.clone().len() as i32);
                        let (mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = task_record_at_19(v66.clone(), v79, v80);
                        let v90: Rc<str> = agile_planning_fields::rig_agile_planning_summary(&*v66, &*v1);
                        let v91: Rc<str> = agile_note_listing::rig_agile_notes_text(&*v67, &*v1);
                        let v92: Rc<str> = agile_note_listing::rig_agile_notes_text(&*v69, &*v1);
                        let v93: Rc<str> = notes_joined_18(v91, v92);
                        show_task_2(v81, v82, v83, v84, v85, v86, v87, v88, v89, v90, v93, v3)
                    }
                }
            }
        }
    }
}
fn command_line_argument_count_30() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_29() -> i32 {
    let mut v0: i32 = command_line_argument_count_30();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn agile_show_entry_0() -> i32 {
    let mut v0: i32 = argument_count_29();
    let mut v1: bool = v0 == 3i32;
    if v1 {
        let v2: i32 = 1i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v4: i32 = 2i32;
        let v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v6: US0 = US0::US0_ShowText;
        agile_show_1(v3, v5, v6)
    } else {
        let v8: i32 = argument_count_29();
        let mut v9: bool = v8 == 5i32;
        let mut v13: Rc<str> = if v9 {
            let v10: i32 = 4i32;
            let v11: Rc<str> = usize::try_from(v10).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            v11
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v12
        };
        let v14: i32 = argument_count_29();
        let v15: bool = v14 > 5i32;
        let v37: US6 = if v15 {
            US6::US6_Invalid
        } else {
            let mut v17: i32 = 3i32;
            let v18: Rc<str> = usize::try_from(v17).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v19: bool = &*v18 == "--json";
            let v21: bool = if v19 {
                let v20: bool = &*v13 == "";
                v20
            } else {
                false
            };
            if v21 {
                let mut v22: US0 = US0::US0_ShowJson;
                US6::US6_Valid(v22)
            } else {
                let v24: bool = &*v18 == "--last";
                let mut v26: bool = if v24 {
                    true
                } else {
                    let mut v25: bool = &*v18 == "--grep";
                    v25
                };
                let v28: bool = if v26 {
                    let v27: bool = &*v13 != "";
                    v27
                } else {
                    false
                };
                if v28 {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v31: Rc<str> = Rc::<str>::from(v18.replace(&*v29, &*v30));
                    let v32: US0 = US0::US0_ShowFiltered(v31, v13);
                    US6::US6_Valid(v32)
                } else {
                    US6::US6_Invalid
                }
            }
        };
        match v37 {
            US6::US6_Invalid => {
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("options are --json, --last <n> or --grep <word>"); } LIT.with(|lit| lit.clone()) };
                let v45: i32 = 2i32;
                let v46: Rc<str> = usize::try_from(v45).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v49: Rc<str> = string_concat(&["agile show ", &*v46, " rejected: ", &*v44]);
                let v50: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("rig error: ".to_string() + &*v49 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v50
            }
            US6::US6_Valid(v38) => {
                let v38: US0 = v38;
                let v39: i32 = 1i32;
                let v40: Rc<str> = usize::try_from(v39).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v41: i32 = 2i32;
                let v42: Rc<str> = usize::try_from(v41).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                agile_show_1(v40, v42, v38)
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_show_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_show_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
