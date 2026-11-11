#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)] #![recursion_limit = "256"]
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
fn agile_state_mutation_target_decision_binding_0(mut v0: i32, mut v1: i32) -> i32 {
    let v2: bool = v0 == 1i32;
    if v2 {
        let v3: bool = v1 == 1i32;
        if v3 {
            2i32
        } else {
            1i32
        }
    } else {
        0i32
    }
}
fn closure0() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_state_mutation_target_decision_binding_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_lease_budget_decision_binding_1(mut v0: i32, mut v1: i32) -> i32 {
    let v2: bool = v0 == 1i32;
    if v2 {
        let v3: bool = v1 == 1i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    } else {
        0i32
    }
}
fn closure1() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_lease_budget_decision_binding_1(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_lease_readback_decision_binding_2(mut v0: i32, mut v1: i32) -> i32 {
    let v2: bool = v0 == 1i32;
    if v2 {
        let v3: bool = v1 == 1i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    } else {
        0i32
    }
}
fn closure2() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_lease_readback_decision_binding_2(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_prompt_title_code_3(mut v0: Rc<str>) -> u64 {
    let v1: bool = &*v0 == "";
    if v1 {
        0u64
    } else {
        1u64
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_prompt_title_code_3(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_status_name_code_4(mut v0: Rc<str>) -> u64 {
    let v1: u64 = u64::MAX;
    let v2: bool = "Planned" == &*v0;
    if v2 {
        let v3: u64 = 0i32 as u64;
        v3
    } else {
        let v4: bool = "Active" == &*v0;
        if v4 {
            let v5: u64 = 1i32 as u64;
            v5
        } else {
            let v6: bool = "Blocked" == &*v0;
            if v6 {
                let v7: u64 = 2i32 as u64;
                v7
            } else {
                let v8: bool = "Paused" == &*v0;
                if v8 {
                    let v9: u64 = 3i32 as u64;
                    v9
                } else {
                    let v10: bool = "Done" == &*v0;
                    if v10 {
                        let v11: u64 = 4i32 as u64;
                        v11
                    } else {
                        v1
                    }
                }
            }
        }
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_status_name_code_4(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_task_line_candidate_code_5(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v1.starts_with(&*v2);
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v1.contains(&*v4);
        if v5 {
            1u64
        } else {
            0u64
        }
    } else {
        0u64
    }
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_task_line_candidate_code_5(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn receipt_row_kind_code_7(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("snapshot"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0 == v1 ;
    if v2 {
        1u64
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("module"); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0 == v3 ;
        if v4 {
            2u64
        } else {
            0u64
        }
    }
}
fn agile_receipt_row_kind_code_6(mut v0: Rc<str>) -> u64 {
    receipt_row_kind_code_7(v0)
}
fn closure6() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_receipt_row_kind_code_6(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn receipt_row_kind_text_9(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0 == v1 ;
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("snapshot"); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2"); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v0 == v4 ;
        if v5 {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("module"); } LIT.with(|lit| lit.clone()) };
            v6
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7
        }
    }
}
fn agile_receipt_row_kind_text_8(mut v0: Rc<str>) -> Rc<str> {
    receipt_row_kind_text_9(v0)
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        agile_receipt_row_kind_text_8(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn starts_with_loop_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
    loop {
        let v5: bool = v3 == v4 ;
        if v5 {
            return true;
        } else {
            let v6: u8 = v0.clone().as_bytes()[v2 as usize];
            let v7: u8 = v1.clone().as_bytes()[v3 as usize];
            let v8: bool = v6 == v7 ;
            if v8 {
                let v9: i32 = v2 + 1i32 ;
                let v10: i32 = v3 + 1i32 ;
                (v2, v3) = (v9, v10);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn receipt_attestation_kind_code_11(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runtime"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0 == v1 ;
    if v2 {
        1u64
    } else {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v3 < 6i32;
        let v10: bool = if v4 {
            false
        } else {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("build:"); } LIT.with(|lit| lit.clone()) };
            let v6: i32 = 0i32;
            let v7: i32 = 0i32;
            let v8: i32 = 6i32;
            starts_with_loop_12(v0, v5, v6, v7, v8)
        };
        if v10 {
            2u64
        } else {
            0u64
        }
    }
}
fn agile_receipt_attestation_kind_code_10(mut v0: Rc<str>) -> u64 {
    receipt_attestation_kind_code_11(v0)
}
fn closure8() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_receipt_attestation_kind_code_10(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn receipt_attestation_text_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0 == v1 ;
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runtime"); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let v4: Rc<str> = string_concat(&["build:", &*v0]);
        v4
    }
}
fn agile_receipt_attestation_text_13(mut v0: Rc<str>) -> Rc<str> {
    receipt_attestation_text_14(v0)
}
fn closure9() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        agile_receipt_attestation_text_13(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_prompt_lease_status_code_15(mut v0: Rc<str>) -> u64 {
    let v1: bool = &*v0 == "LeaseActive";
    if v1 {
        1u64
    } else {
        0u64
    }
}
fn closure10() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_prompt_lease_status_code_15(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_prompt_lease_marker_text_16(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = &*v0 == "1";
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PromptLease ("); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3
    }
}
fn closure11() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        agile_prompt_lease_marker_text_16(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_prompt_lease_shape_binding_17(mut v0: i32, mut v1: i32) -> i32 {
    let v2: bool = v0 == 7i32;
    if v2 {
        let v3: bool = v1 == 1i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    } else {
        0i32
    }
}
fn closure12() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_prompt_lease_shape_binding_17(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_lease_clock_phase_binding_18(mut v0: i32, mut v1: i32) -> i32 {
    let v2: bool = v1 == 1i32;
    if v2 {
        2i32
    } else {
        let v3: bool = v0 == 1i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    }
}
fn closure13() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_lease_clock_phase_binding_18(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_lease_policy_from_text_19(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v2: bool = &*v1 == "bounded";
    if v2 {
        0u64
    } else {
        1u64
    }
}
fn closure14() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_lease_policy_from_text_19(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_lease_effective_phase_binding_20(mut v0: i32, mut v1: i32) -> i32 {
    let v2: bool = v0 == 1i32;
    if v2 {
        0i32
    } else {
        v1
    }
}
fn closure15() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        agile_lease_effective_phase_binding_20(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_status_text_21(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((0i32).to_string());
    let v2: bool = &*v0 == &*v1;
    let v5: Rc<str> = if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    };
    let v6: Rc<str> = std::rc::Rc::<str>::from((1i32).to_string());
    let v7: bool = &*v0 == &*v6;
    let v9: Rc<str> = if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
        v8
    } else {
        v5
    };
    let v10: Rc<str> = std::rc::Rc::<str>::from((2i32).to_string());
    let v11: bool = &*v0 == &*v10;
    let v13: Rc<str> = if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) };
        v12
    } else {
        v9
    };
    let v14: Rc<str> = std::rc::Rc::<str>::from((3i32).to_string());
    let v15: bool = &*v0 == &*v14;
    let v17: Rc<str> = if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Paused"); } LIT.with(|lit| lit.clone()) };
        v16
    } else {
        v13
    };
    let v18: Rc<str> = std::rc::Rc::<str>::from((4i32).to_string());
    let v19: bool = &*v0 == &*v18;
    if v19 {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
        v20
    } else {
        v17
    }
}
fn closure16() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        agile_status_text_21(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_spi_escape_text_22(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(v6.replace(&*v7, &*v8));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v11));
    v12
}
fn closure17() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        agile_spi_escape_text_22(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_state_package_line_code_23(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v2: bool = &*v1 == "modules:";
    if v2 {
        1u64
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.starts_with(&*v3);
        let v7: bool = if v4 {
            true
        } else {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v0.starts_with(&*v5);
            v6
        };
        if v7 {
            let v8: bool = &*v1 == "";
            let v11: bool = if v8 {
                true
            } else {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//"); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v1.starts_with(&*v9);
                v10
            };
            if v11 {
                0u64
            } else {
                2u64
            }
        } else {
            3u64
        }
    }
}
fn closure18() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_state_package_line_code_23(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_25(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(1i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn loop_26(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(1i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn agile_state_package_module_name_24(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v2: Rc<str> = loop_25(v1);
    let v3: Rc<str> = loop_26(v2);
    let v4: Rc<str> = Rc::<str>::from((v3).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v5: bool = &*v4 == "";
    let v8: bool = if v5 {
        true
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v7: bool = v4.contains(&*v6);
        v7
    };
    let v11: bool = if v8 {
        true
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v10: bool = v4.contains(&*v9);
        v10
    };
    if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    } else {
        let v13: Rc<str> = string_concat(&[&*v4, ".spi"]);
        v13
    }
}
fn closure19() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        agile_state_package_module_name_24(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_state_toml_line_code_27(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = Rc::<str>::from((v0).trim_matches(|c: char| { let n = c as u32; n == 32 || n == 9 || n == 13 || n == 10 }));
    let v2: bool = &*v1 == "modules = '''";
    if v2 {
        1u64
    } else {
        let v3: bool = &*v1 == "'''";
        if v3 {
            3u64
        } else {
            let v4: bool = &*v1 == "";
            let v7: bool = if v4 {
                true
            } else {
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
                let v6: bool = v1.starts_with(&*v5);
                v6
            };
            if v7 {
                0u64
            } else {
                2u64
            }
        }
    }
}
fn closure20() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_state_toml_line_code_27(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_agile_state_mutation_target_decision_binding(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
pub fn rig_agile_lease_budget_decision_binding(v0: i32, v1: i32) -> i32 {
    closure1()(v0, v1)
}
pub fn rig_agile_lease_readback_decision_binding(v0: i32, v1: i32) -> i32 {
    closure2()(v0, v1)
}
pub fn rig_agile_prompt_title_code(v0: &str) -> u64 {
    closure3()(Rc::<str>::from(v0))
}
pub fn rig_agile_status_name_code(v0: &str) -> u64 {
    closure4()(Rc::<str>::from(v0))
}
pub fn rig_agile_task_line_candidate_code(v0: &str) -> u64 {
    closure5()(Rc::<str>::from(v0))
}
pub fn rig_agile_receipt_row_kind_code(v0: &str) -> u64 {
    closure6()(Rc::<str>::from(v0))
}
pub fn rig_agile_receipt_row_kind_text(v0: &str) -> Rc<str> {
    closure7()(Rc::<str>::from(v0))
}
pub fn rig_agile_receipt_attestation_kind_code(v0: &str) -> u64 {
    closure8()(Rc::<str>::from(v0))
}
pub fn rig_agile_receipt_attestation_text(v0: &str) -> Rc<str> {
    closure9()(Rc::<str>::from(v0))
}
pub fn rig_agile_prompt_lease_status_code(v0: &str) -> u64 {
    closure10()(Rc::<str>::from(v0))
}
pub fn rig_agile_prompt_lease_marker_text(v0: &str) -> Rc<str> {
    closure11()(Rc::<str>::from(v0))
}
pub fn rig_agile_prompt_lease_shape_binding(v0: i32, v1: i32) -> i32 {
    closure12()(v0, v1)
}
pub fn rig_agile_lease_clock_phase_binding(v0: i32, v1: i32) -> i32 {
    closure13()(v0, v1)
}
pub fn rig_agile_lease_policy_from_text(v0: &str) -> u64 {
    closure14()(Rc::<str>::from(v0))
}
pub fn rig_agile_lease_effective_phase_binding(v0: i32, v1: i32) -> i32 {
    closure15()(v0, v1)
}
pub fn rig_agile_status_text(v0: &str) -> Rc<str> {
    closure16()(Rc::<str>::from(v0))
}
pub fn rig_agile_spi_escape_text(v0: &str) -> Rc<str> {
    closure17()(Rc::<str>::from(v0))
}
pub fn rig_agile_state_package_line_code(v0: &str) -> u64 {
    closure18()(Rc::<str>::from(v0))
}
pub fn rig_agile_state_package_module_name(v0: &str) -> Rc<str> {
    closure19()(Rc::<str>::from(v0))
}
pub fn rig_agile_state_toml_line_code(v0: &str) -> u64 {
    closure20()(Rc::<str>::from(v0))
}
