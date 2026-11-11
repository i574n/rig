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
    US0_StartPlain,
    US0_StartHeavy,
    US0_StartCompile,
}
#[derive(Clone)]
enum US1 {
    US1_Launched(Rc<str>, Rc<str>),
    US1_LaunchRefused(Rc<str>),
}
pub struct Mut0 { pub l0: Rc<str> }
fn shape_of_word_1(mut v0: Rc<str>) -> US0 {
    let v1: bool = "plain" == &*v0;
    if v1 {
        US0::US0_StartPlain
    } else {
        let v3: bool = "heavy" == &*v0;
        if v3 {
            US0::US0_StartHeavy
        } else {
            let v5: bool = "compile" == &*v0;
            if v5 {
                US0::US0_StartCompile
            } else {
                US0::US0_StartPlain
            }
        }
    }
}
fn now_ms_3() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn method6(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_5(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method6(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn decimal_text_4(mut v0: u64) -> Rc<str> {
    format_real_5(v0)
}
fn heavy_gate_script_7() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_HEAVY_GATE").or_else(|| std::env::var_os("KIT_HEAVY_GATE")).map(std::path::PathBuf::from).filter(|path| path.is_file()).or_else(|| std::env::current_dir().ok().and_then(|cwd| cwd.ancestors().find_map(|base| { let compiler = base.join("spiral").join("apps").join("compiler"); [compiler.join("scripts").join("heavy.ps1"), compiler.join("tmp").join("scripts").join("heavy.ps1")].into_iter().find(|candidate| candidate.is_file()) }))).map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
    v0
}
fn joined_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_10() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("RIG_JOBS_HOME").or_else(|| std::env::var_os("KIT_JOBS_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_10();
    joined_9(v1, v0)
}
fn job_file_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_8(v0);
    joined_9(v2, v1)
}
fn text_written_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2
}
fn job_argv_11(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> Rc<str> {
    match v0 {
        US0::US0_StartCompile => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = job_file_12(v1, v5);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = text_written_13(v6, v7);
            v3
        }
        US0::US0_StartHeavy => {
            let v9: Rc<str> = heavy_gate_script_7();
            let v10: bool = &*v9 == "";
            if v10 {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v11
            } else {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv.inner"); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = job_file_12(v1.clone(), v12.clone());
                let v14: Rc<str> = text_written_13(v13, v3);
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = job_file_12(v1.clone(), v15);
                let v17: Rc<str> = job_file_12(v1.clone(), v12);
                let v18: Rc<str> = rig_machine_windows::rig_heavy_wrapper(&*v9, &*v17, &*v2, &*v1, v4);
                let v19: Rc<str> = text_written_13(v16.clone(), v18);
                let v20: Rc<str> = string_concat(&["pwsh\n-NoProfile\n-File\n", &*v16]);
                v20
            }
        }
        US0::US0_StartPlain => {
            v3
        }
    }
}
fn environment_snapshot_14() -> Rc<str> {
    let mut v0: Rc<str> = Rc::<str>::from(std::env::vars_os().map(|(key, value)| format!("{}={}", key.to_string_lossy(), value.to_string_lossy())).collect::<Vec<_>>().join(&char::from(0u8).to_string()).as_str());
    v0
}
fn get_environment_variable_15(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn job_launched_2(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: bool, mut v11: Rc<str>, mut v12: u64, mut v13: Rc<str>, mut v14: Rc<str>) -> US1 {
    let v15: u64 = now_ms_3();
    let v16: Rc<str> = decimal_text_4(v15);
    let v19: Rc<str> = std::rc::Rc::<str>::from(v11.chars().map(|c| if c.is_ascii_alphanumeric() || c == char::from(45u8) { c } else { char::from(95u8) }).take(40).collect::<String>());
    let v20: Rc<str> = string_concat(&["job-", &*v16, "-", &*v19]);
    let v21: bool = match &v0 {
        US0::US0_StartHeavy => {
            true
        }
        _ => {
            false
        }
    };
    let v24: bool = if v21 {
        let v22: Rc<str> = heavy_gate_script_7();
        let v23: bool = &*v22 == "";
        v23
    } else {
        false
    };
    let v28: Rc<str> = if v24 {
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v25
    } else {
        let v26: Rc<str> = job_directory_8(v20.clone());
        let v27: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v26).err().map(|error| format!("create {}: {error}", &*v26)).unwrap_or_default());
        v27
    };
    let v30: bool = if v24 {
        true
    } else {
        let v29: bool = &*v28 != "";
        v29
    };
    if v30 {
        if v24 {
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--heavy needs spiral's scripts/heavy.ps1: set KIT_HEAVY_GATE or run from inside a tree that has a spiral checkout"); } LIT.with(|lit| lit.clone()) };
            US1::US1_LaunchRefused(v31)
        } else {
            US1::US1_LaunchRefused(v28)
        }
    } else {
        let v35: Rc<str> = job_argv_11(v0, v20.clone(), v11.clone(), v13, v12);
        let v36: bool = &*v35 == "";
        if v36 {
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the job command line came out empty"); } LIT.with(|lit| lit.clone()) };
            US1::US1_LaunchRefused(v37)
        } else {
            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
            let v40: Rc<str> = job_file_12(v20.clone(), v39);
            let v41: Rc<str> = text_written_13(v40, v11);
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = job_file_12(v20.clone(), v42);
            let v44: Rc<str> = decimal_text_4(v12);
            let v45: Rc<str> = text_written_13(v43, v44);
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cwd"); } LIT.with(|lit| lit.clone()) };
            let v48: Rc<str> = job_file_12(v20.clone(), v47);
            let v49: Rc<str> = text_written_13(v48, v14);
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv"); } LIT.with(|lit| lit.clone()) };
            let v52: Rc<str> = job_file_12(v20.clone(), v51);
            let v53: Rc<str> = text_written_13(v52, v35);
            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
            let v56: Rc<str> = job_file_12(v20.clone(), v55);
            let v57: u64 = now_ms_3();
            let v58: Rc<str> = decimal_text_4(v57);
            let v59: Rc<str> = text_written_13(v56, v58);
            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
            let v62: Rc<str> = job_file_12(v20.clone(), v61);
            let v63: Rc<str> = environment_snapshot_14();
            let v64: Rc<str> = text_written_13(v62, v63);
            let v65: Rc<str> = string_concat(&[&*v41, &*v45, &*v49, &*v53, &*v59, &*v64]);
            let v66: bool = &*v1 == "";
            let v73: Rc<str> = if v66 {
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v67
            } else {
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                let v69: Rc<str> = job_file_12(v20.clone(), v68);
                let v71: Rc<str> = string_concat(&[&*v2, "\t", &*v1]);
                text_written_13(v69, v71)
            };
            let v74: bool = v4 == 0u64;
            let v80: Rc<str> = if v74 {
                let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v75
            } else {
                let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_mb"); } LIT.with(|lit| lit.clone()) };
                let v77: Rc<str> = job_file_12(v20.clone(), v76);
                let v78: Rc<str> = decimal_text_4(v4);
                text_written_13(v77, v78)
            };
            let v81: bool = v5 == 0u64;
            let v86: u64 = if v81 {
                v6
            } else {
                let v82: bool = v6 == 0u64;
                let v84: bool = if v82 {
                    true
                } else {
                    let v83: bool = v5 < v6;
                    v83
                };
                if v84 {
                    v5
                } else {
                    v6
                }
            };
            let v87: bool = v86 == 0u64;
            let v96: Rc<str> = if v87 {
                let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v88
            } else {
                let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_by_ms"); } LIT.with(|lit| lit.clone()) };
                let v90: Rc<str> = job_file_12(v20.clone(), v89);
                let v91: u64 = now_ms_3();
                let v92: u64 = v86.wrapping_mul(60000u64);
                let v93: u64 = v91.wrapping_add(v92);
                let v94: Rc<str> = decimal_text_4(v93);
                text_written_13(v90, v94)
            };
            let v97: bool = v6 == 0u64;
            let v106: Rc<str> = if v97 {
                let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v98
            } else {
                let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done_by_ms"); } LIT.with(|lit| lit.clone()) };
                let v100: Rc<str> = job_file_12(v20.clone(), v99);
                let v101: u64 = now_ms_3();
                let v102: u64 = v6.wrapping_mul(60000u64);
                let v103: u64 = v101.wrapping_add(v102);
                let v104: Rc<str> = decimal_text_4(v103);
                text_written_13(v100, v104)
            };
            let v107: bool = v7 == 0u64;
            let v118: Rc<str> = if v107 {
                let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v108
            } else {
                let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_hold_min"); } LIT.with(|lit| lit.clone()) };
                let v110: Rc<str> = job_file_12(v20.clone(), v109);
                let v111: Rc<str> = decimal_text_4(v7);
                let v112: Rc<str> = text_written_13(v110, v111);
                let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_wait_min"); } LIT.with(|lit| lit.clone()) };
                let v114: Rc<str> = job_file_12(v20.clone(), v113);
                let v115: Rc<str> = decimal_text_4(v8);
                let v116: Rc<str> = text_written_13(v114, v115);
                let v117: Rc<str> = string_concat(&[&*v112, &*v116]);
                v117
            };
            let v119: bool = v9 == 0u64;
            let v125: Rc<str> = if v119 {
                let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v120
            } else {
                let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("min_free_mb"); } LIT.with(|lit| lit.clone()) };
                let v122: Rc<str> = job_file_12(v20.clone(), v121);
                let v123: Rc<str> = decimal_text_4(v9);
                text_written_13(v122, v123)
            };
            let v131: Rc<str> = if v10 {
                let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("front"); } LIT.with(|lit| lit.clone()) };
                let v127: Rc<str> = job_file_12(v20.clone(), v126);
                let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                text_written_13(v127, v128)
            } else {
                let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v130
            };
            let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RIG_AUTHOR"); } LIT.with(|lit| lit.clone()) };
            let v133: Rc<str> = get_environment_variable_15(v132);
            let v134: bool = &*v133 != "";
            let v137: Rc<str> = if v134 {
                v133
            } else {
                let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AUTHOR"); } LIT.with(|lit| lit.clone()) };
                get_environment_variable_15(v135)
            };
            let v138: bool = &*v137 == "";
            let v143: Rc<str> = if v138 {
                let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v139
            } else {
                let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("author"); } LIT.with(|lit| lit.clone()) };
                let v141: Rc<str> = job_file_12(v20.clone(), v140);
                text_written_13(v141, v137)
            };
            let v144: bool = &*v65 != "";
            let v146: bool = if v144 {
                true
            } else {
                let v145: bool = &*v73 != "";
                v145
            };
            let v148: bool = if v146 {
                true
            } else {
                let v147: bool = &*v80 != "";
                v147
            };
            let v150: bool = if v148 {
                true
            } else {
                let v149: bool = &*v96 != "";
                v149
            };
            let v152: bool = if v150 {
                true
            } else {
                let v151: bool = &*v106 != "";
                v151
            };
            let v154: bool = if v152 {
                true
            } else {
                let v153: bool = &*v118 != "";
                v153
            };
            let v156: bool = if v154 {
                true
            } else {
                let v155: bool = &*v125 != "";
                v155
            };
            let v158: bool = if v156 {
                true
            } else {
                let v157: bool = &*v131 != "";
                v157
            };
            let v160: bool = if v158 {
                true
            } else {
                let v159: bool = &*v143 != "";
                v159
            };
            if v160 {
                let v168: Rc<str> = string_concat(&[&*v125, &*v65, &*v73, &*v80, &*v96, &*v106, &*v118, &*v131, &*v143]);
                US1::US1_LaunchRefused(v168)
            } else {
                let v172: Rc<str> = if v3 {
                    let v170: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
                    v170
                } else {
                    let v171: Rc<str> = rig_job_host_request::rig_job_host_request(&*v20);
                    v171
                };
                let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error:"); } LIT.with(|lit| lit.clone()) };
                let v174: bool = v172.starts_with(&*v173);
                if v174 {
                    US1::US1_LaunchRefused(v172)
                } else {
                    US1::US1_Launched(v20, v172)
                }
            }
        }
    }
}
fn job_created_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: bool, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let v15: US0 = shape_of_word_1(v14);
    let v16: US1 = job_launched_2(v15, v13, v12, v11, v10, v9, v8, v7, v6, v5, v4, v3, v2, v1, v0);
    match v16 {
        US1::US1_LaunchRefused(v20) => {
            let v20: Rc<str> = v20;
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v21.clone(), v21, v20)
        }
        US1::US1_Launched(v17, v18) => {
            let v17: Rc<str> = v17;
            let v18: Rc<str> = v18;
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v17, v18, v19)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool, u64, u64, u64, u64, u64, u64, bool, Rc<str>, u64, Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool, u64, u64, u64, u64, u64, u64, bool, Rc<str>, u64, Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: bool, mut v11: Rc<str>, mut v12: u64, mut v13: Rc<str>, mut v14: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>) {
        job_created_0(v14, v13, v12, v11, v10, v9, v8, v7, v6, v5, v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_job_create(v0: &str, v1: &str, v2: &str, v3: bool, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: bool, v11: &str, v12: u64, v13: &str, v14: &str) -> (Rc<str>, Rc<str>, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3, v4, v5, v6, v7, v8, v9, v10, Rc::<str>::from(v11), v12, Rc::<str>::from(v13), Rc::<str>::from(v14))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
