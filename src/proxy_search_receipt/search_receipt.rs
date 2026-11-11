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
    US0_AllHitsShown,
    US0_HitsTruncated,
}
#[derive(Clone)]
enum US1 {
    US1_SliceFilled,
    US1_SliceReachedEnd,
}
fn search_clock_ms_0() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    v0
}
fn closure0() -> Rc<dyn Fn() -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> u64> = Rc::new(move || -> u64 {
        search_clock_ms_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn parallel_search_receipt_text_2(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: u64, mut v4: u64, mut v5: u64) -> Rc<str> {
    let v6: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v7: Rc<str> = string_concat(&["rig proxy parallel-search ok workers=", &*v6]);
    let v8: bool = v1 > v2;
    let v11: US0 = if v8 {
        US0::US0_HitsTruncated
    } else {
        US0::US0_AllHitsShown
    };
    let v20: Rc<str> = match v11 {
        US0::US0_AllHitsShown => {
            let v12: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let v13: Rc<str> = string_concat(&[" hits=", &*v12]);
            v13
        }
        US0::US0_HitsTruncated => {
            let v14: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let v17: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let v18: Rc<str> = string_concat(&[" hits=", &*v14, " truncated=true found_at_least=", &*v17]);
            v18
        }
    };
    let v23: u64 = (v4).saturating_sub(v3);
    let v24: Rc<str> = std::rc::Rc::<str>::from((v23).to_string());
    let v27: u64 = (v5).saturating_sub(v4);
    let v28: Rc<str> = std::rc::Rc::<str>::from((v27).to_string());
    let v31: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let v32: Rc<str> = std::rc::Rc::<str>::from((v31).to_string());
    let v34: Rc<str> = string_concat(&[&*v7, &*v20, " parity=exact ordering=deterministic excludes=.git,node_modules,build-output,symlinks serial_ms=", &*v24, " parallel_ms=", &*v28, " elapsed_ms=", &*v32]);
    v34
}
fn parallel_search_receipt_1(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: u64, mut v4: u64, mut v5: u64) -> i32 {
    let v6: Rc<str> = parallel_search_receipt_text_2(v0, v1, v2, v3, v4, v5);
    let v7: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v6.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v7
}
fn closure1() -> Rc<dyn Fn(i32, i32, i32, u64, u64, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32, i32, u64, u64, u64) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32, mut v2: i32, mut v3: u64, mut v4: u64, mut v5: u64| -> i32 {
        parallel_search_receipt_1(v0, v1, v2, v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn fs_search_receipt_text_4(mut v0: i32, mut v1: i32, mut v2: u64) -> Rc<str> {
    let v3: bool = v0 > v1;
    let v6: US0 = if v3 {
        US0::US0_HitsTruncated
    } else {
        US0::US0_AllHitsShown
    };
    let v15: Rc<str> = match v6 {
        US0::US0_AllHitsShown => {
            let v7: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
            let v8: Rc<str> = string_concat(&[" hits=", &*v7]);
            v8
        }
        US0::US0_HitsTruncated => {
            let v9: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let v12: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
            let v13: Rc<str> = string_concat(&[" hits=", &*v9, " truncated=true found_at_least=", &*v12]);
            v13
        }
    };
    let v18: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
    let v19: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
    let v21: Rc<str> = string_concat(&["rig proxy fs-search ok", &*v15, " excludes=.git,node_modules,build-output,symlinks elapsed_ms=", &*v19]);
    v21
}
fn fs_search_receipt_3(mut v0: i32, mut v1: i32, mut v2: u64) -> i32 {
    let v3: Rc<str> = fs_search_receipt_text_4(v0, v1, v2);
    let v4: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v3.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v4
}
fn closure2() -> Rc<dyn Fn(i32, i32, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32, u64) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32, mut v2: u64| -> i32 {
        fs_search_receipt_3(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn fs_slice_receipt_text_6(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> Rc<str> {
    let v4: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v7: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v10: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let v11: Rc<str> = string_concat(&["rig proxy fs-slice ok start=", &*v4, " lines=", &*v7, " requested=", &*v10]);
    let v12: bool = v1 < v2;
    let v15: US1 = if v12 {
        US1::US1_SliceReachedEnd
    } else {
        US1::US1_SliceFilled
    };
    let v19: Rc<str> = match v15 {
        US1::US1_SliceFilled => {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v16
        }
        US1::US1_SliceReachedEnd => {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" end_of_file=true"); } LIT.with(|lit| lit.clone()) };
            v17
        }
    };
    let v21: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let v22: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
    let v24: Rc<str> = string_concat(&[&*v11, &*v19, " elapsed_ms=", &*v22]);
    v24
}
fn fs_slice_receipt_5(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> i32 {
    let v4: Rc<str> = fs_slice_receipt_text_6(v0, v1, v2, v3);
    let v5: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v4.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v5
}
fn closure3() -> Rc<dyn Fn(u64, u64, u64, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, u64, u64, u64) -> i32> = Rc::new(move |mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64| -> i32 {
        fs_slice_receipt_5(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn fs_context_receipt_text_8(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> Rc<str> {
    let v4: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v7: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v10: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let v12: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let v13: Rc<str> = std::rc::Rc::<str>::from((v12).to_string());
    let v15: Rc<str> = string_concat(&["rig proxy fs-context ok offset=", &*v4, " window_bytes=", &*v7, " file_bytes=", &*v10, " elapsed_ms=", &*v13]);
    v15
}
fn fs_context_receipt_7(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> i32 {
    let v4: Rc<str> = fs_context_receipt_text_8(v0, v1, v2, v3);
    let v5: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v4.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v5
}
fn closure4() -> Rc<dyn Fn(u64, u64, u64, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, u64, u64, u64) -> i32> = Rc::new(move |mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64| -> i32 {
        fs_context_receipt_7(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn hash_receipt_text_10(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: u64 = std::fs::metadata(&*v0).map(|meta| meta.len()).unwrap_or(0);
    let v3: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let v5: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v1);
    let v6: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
    let v8: Rc<str> = string_concat(&["rig proxy hash ok bytes=", &*v3, " elapsed_ms=", &*v6]);
    v8
}
fn hash_receipt_9(mut v0: Rc<str>, mut v1: u64) -> i32 {
    let v2: Rc<str> = hash_receipt_text_10(v0, v1);
    let v3: i32 = std::io::Write::write_all(&mut std::io::stderr(), (v2.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v3
}
fn closure5() -> Rc<dyn Fn(Rc<str>, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> i32 {
        hash_receipt_9(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn hash_tree_receipt_text_12(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v1);
    let v4: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v6: Rc<str> = string_concat(&["rig proxy hash-tree ok relative=", &*v0, " elapsed_ms=", &*v4]);
    v6
}
fn hash_tree_receipt_11(mut v0: Rc<str>, mut v1: u64) -> i32 {
    let v2: Rc<str> = hash_tree_receipt_text_12(v0, v1);
    let v3: i32 = std::io::Write::write_all(&mut std::io::stderr(), (v2.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v3
}
fn closure6() -> Rc<dyn Fn(Rc<str>, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> i32 {
        hash_tree_receipt_11(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn parallel_manifest_receipt_text_14(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64) -> Rc<str> {
    let v6: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v9: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v14: u64 = (v4).saturating_sub(v3);
    let v15: Rc<str> = std::rc::Rc::<str>::from((v14).to_string());
    let v18: u64 = (v5).saturating_sub(v4);
    let v19: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
    let v22: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let v23: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
    let v25: Rc<str> = string_concat(&["rig proxy parallel-manifest ok workers=", &*v6, " entries=", &*v9, " sha256=", &*v2, " parity=exact ordering=deterministic serial_ms=", &*v15, " parallel_ms=", &*v19, " elapsed_ms=", &*v23]);
    v25
}
fn parallel_manifest_receipt_13(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64) -> i32 {
    let v6: Rc<str> = parallel_manifest_receipt_text_14(v0, v1, v2, v3, v4, v5);
    let v7: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v6.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v7
}
fn closure7() -> Rc<dyn Fn(i32, i32, Rc<str>, u64, u64, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32, Rc<str>, u64, u64, u64) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64| -> i32 {
        parallel_manifest_receipt_13(v0, v1, v2, v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_search_clock_ms() -> u64 {
    closure0()()
}
pub fn rig_parallel_search_receipt(v0: i32, v1: i32, v2: i32, v3: u64, v4: u64, v5: u64) -> i32 {
    closure1()(v0, v1, v2, v3, v4, v5)
}
pub fn rig_fs_search_receipt(v0: i32, v1: i32, v2: u64) -> i32 {
    closure2()(v0, v1, v2)
}
pub fn rig_fs_slice_receipt(v0: u64, v1: u64, v2: u64, v3: u64) -> i32 {
    closure3()(v0, v1, v2, v3)
}
pub fn rig_fs_context_receipt(v0: u64, v1: u64, v2: u64, v3: u64) -> i32 {
    closure4()(v0, v1, v2, v3)
}
pub fn rig_hash_receipt(v0: &str, v1: u64) -> i32 {
    closure5()(Rc::<str>::from(v0), v1)
}
pub fn rig_hash_tree_receipt(v0: &str, v1: u64) -> i32 {
    closure6()(Rc::<str>::from(v0), v1)
}
pub fn rig_parallel_manifest_receipt(v0: i32, v1: i32, v2: &str, v3: u64, v4: u64, v5: u64) -> i32 {
    closure7()(v0, v1, Rc::<str>::from(v2), v3, v4, v5)
}
