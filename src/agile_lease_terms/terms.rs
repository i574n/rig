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
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_LeaseRunning,
    US1_LeaseWrapping,
    US1_LeaseExpired,
}
pub struct Mut0 { pub l0: Rc<str> }
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn process_argument_1(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_2();
        let v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US0 = if v5 {
        let v6: Rc<str> = command_line_argument_3(v1);
        US0::US0_Some(v6)
    } else {
        US0::US0_None
    };
    match v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            v10
        }
    }
}
fn lease_effect_entry_0() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = process_argument_1(v0);
    let mut v3: i32 = 2i32;
    let v4: Rc<str> = process_argument_1(v3);
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(std::env::args().count().saturating_sub(3).to_string());
    let v8: Rc<str> = string_concat(&[&*v1, ":", &*v4, "/", &*v7]);
    let v9: u64 = command_spec_domain::rig_command_subcommand_effect_code(&v8);
    let v10: bool = v9 <= 3u64;
    if v10 {
        let v11: i32 = v9 as i32;
        v11
    } else {
        let v12: i32 = 1i32;
        let v13: Rc<str> = process_argument_1(v12);
        let v15: i32 = 2i32;
        let v16: Rc<str> = process_argument_1(v15);
        let v19: i32 = 3i32;
        let v20: Rc<str> = process_argument_1(v19);
        let v21: Rc<str> = string_concat(&[&*v13, ":", &*v16, ":", &*v20]);
        let v22: u64 = command_spec_domain::rig_command_subcommand_effect_code(&v21);
        let v23: bool = v22 <= 3u64;
        if v23 {
            let v24: i32 = v22 as i32;
            v24
        } else {
            let v25: i32 = 1i32;
            let v26: Rc<str> = process_argument_1(v25);
            let v28: i32 = 2i32;
            let v29: Rc<str> = process_argument_1(v28);
            let v30: Rc<str> = string_concat(&[&*v26, ":", &*v29]);
            let v31: u64 = command_spec_domain::rig_command_subcommand_effect_code(&v30);
            let v32: bool = v31 <= 3u64;
            if v32 {
                let v33: i32 = v31 as i32;
                v33
            } else {
                let v34: i32 = 1i32;
                let v35: Rc<str> = process_argument_1(v34);
                let v37: i32 = 2i32;
                let v38: Rc<str> = process_argument_1(v37);
                let v40: u64 = command_spec_domain::rig_command_effect_code(&v35);
                let v41: bool = v40 <= 3u64;
                if v41 {
                    let v42: i32 = v40 as i32;
                    v42
                } else {
                    let v43: i32 = 4u64 as i32;
                    v43
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        lease_effect_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn get_environment_variable_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn lease_policy_entry_4() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_LEASE_POLICY"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = get_environment_variable_5(v0);
    let mut v2: bool = v1.is_empty() == false;
    if v2 {
        v1
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lease-policy"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = get_environment_variable_5(v4);
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("XDG_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = get_environment_variable_5(v6);
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("APPDATA"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = get_environment_variable_5(v8);
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("USERPROFILE"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = get_environment_variable_5(v10);
        let v12: bool = v5.is_empty() == false;
        let v31: Rc<str> = if v12 {
            let v14: Rc<str> = string_concat(&[&*v5, "/", &*v3]);
            v14
        } else {
            let v15: bool = v7.is_empty() == false;
            if v15 {
                let v17: Rc<str> = string_concat(&[&*v7, "/kit/", &*v3]);
                v17
            } else {
                let v18: bool = v9.is_empty() == false;
                if v18 {
                    let v20: Rc<str> = string_concat(&[&*v9, "/kit/", &*v3]);
                    v20
                } else {
                    let v21: bool = v11.is_empty() == false;
                    if v21 {
                        let v23: Rc<str> = string_concat(&[&*v11, "/.config/kit/", &*v3]);
                        v23
                    } else {
                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HOME"); } LIT.with(|lit| lit.clone()) };
                        let v25: Rc<str> = get_environment_variable_5(v24);
                        let v27: Rc<str> = string_concat(&[&*v25, "/.config/kit/", &*v3]);
                        v27
                    }
                }
            }
        };
        let v32: Rc<str> = std::fs::read_to_string(&*v31).map(|text| std::rc::Rc::<str>::from(text.trim())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        v32
    }
}
fn closure1() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        lease_policy_entry_4()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = string_concat(&[&*v2, &*v1]);
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_7(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method8(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn lease_status_entry_6(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: bool, mut v6: bool) -> Rc<str> {
    let mut v7: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v9: Rc<str> = string_concat(&["kit agile lease status root=", &*v0, " phase="]);
    let v14: US1 = if v6 {
        US1::US1_LeaseExpired
    } else {
        if v5 {
            US1::US1_LeaseWrapping
        } else {
            US1::US1_LeaseRunning
        }
    };
    let v18: Rc<str> = match v14 {
        US1::US1_LeaseExpired => {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired"); } LIT.with(|lit| lit.clone()) };
            v17
        }
        US1::US1_LeaseRunning => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v15
        }
        US1::US1_LeaseWrapping => {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wrapping"); } LIT.with(|lit| lit.clone()) };
            v16
        }
    };
    let v20: Rc<str> = string_concat(&[&*v9, &*v18, " policy="]);
    let v21: Rc<str> = lease_policy_entry_4();
    let v22: bool = v21.is_empty() == false;
    let v24: Rc<str> = if v22 {
        v21
    } else {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("renewing"); } LIT.with(|lit| lit.clone()) };
        v23
    };
    let v25: Rc<str> = string_concat(&[&*v24, " started="]);
    let v26: bool = v1 <= v7;
    let v89: Rc<str> = if v26 {
        let v27: u64 = v7.saturating_sub(v1);
        let v28: u64 = v27.wrapping_div(1000u64);
        let v29: u64 = v28.wrapping_div(3600u64);
        let v30: u64 = v28.wrapping_div(60u64);
        let v31: u64 = v30.wrapping_rem(60u64);
        let v32: u64 = v28.wrapping_rem(60u64);
        let v33: bool = 0u64 < v29;
        let v56: Rc<str> = if v33 {
            let v34: Rc<str> = format_real_7(v29);
            let v35: Rc<str> = string_concat(&[&*v34, "h"]);
            let v36: bool = v31 <= 9u64;
            let v40: Rc<str> = if v36 {
                let v37: Rc<str> = format_real_7(v31);
                let v38: Rc<str> = string_concat(&["0", &*v37]);
                v38
            } else {
                format_real_7(v31)
            };
            let v42: Rc<str> = string_concat(&[&*v35, &*v40, "m"]);
            v42
        } else {
            let v43: bool = 0u64 < v31;
            if v43 {
                let v44: Rc<str> = format_real_7(v31);
                let v45: Rc<str> = string_concat(&[&*v44, "m"]);
                let v46: bool = v32 <= 9u64;
                let v50: Rc<str> = if v46 {
                    let v47: Rc<str> = format_real_7(v32);
                    let v48: Rc<str> = string_concat(&["0", &*v47]);
                    v48
                } else {
                    format_real_7(v32)
                };
                let v52: Rc<str> = string_concat(&[&*v45, &*v50, "s"]);
                v52
            } else {
                let v53: Rc<str> = format_real_7(v32);
                let v54: Rc<str> = string_concat(&[&*v53, "s"]);
                v54
            }
        };
        let v57: Rc<str> = string_concat(&[&*v56, "_ago"]);
        v57
    } else {
        let v58: u64 = v1.saturating_sub(v7);
        let v59: u64 = v58.wrapping_div(1000u64);
        let v60: u64 = v59.wrapping_div(3600u64);
        let v61: u64 = v59.wrapping_div(60u64);
        let v62: u64 = v61.wrapping_rem(60u64);
        let v63: u64 = v59.wrapping_rem(60u64);
        let v64: bool = 0u64 < v60;
        let v87: Rc<str> = if v64 {
            let v65: Rc<str> = format_real_7(v60);
            let v66: Rc<str> = string_concat(&[&*v65, "h"]);
            let v67: bool = v62 <= 9u64;
            let v71: Rc<str> = if v67 {
                let v68: Rc<str> = format_real_7(v62);
                let v69: Rc<str> = string_concat(&["0", &*v68]);
                v69
            } else {
                format_real_7(v62)
            };
            let v73: Rc<str> = string_concat(&[&*v66, &*v71, "m"]);
            v73
        } else {
            let v74: bool = 0u64 < v62;
            if v74 {
                let v75: Rc<str> = format_real_7(v62);
                let v76: Rc<str> = string_concat(&[&*v75, "m"]);
                let v77: bool = v63 <= 9u64;
                let v81: Rc<str> = if v77 {
                    let v78: Rc<str> = format_real_7(v63);
                    let v79: Rc<str> = string_concat(&["0", &*v78]);
                    v79
                } else {
                    format_real_7(v63)
                };
                let v83: Rc<str> = string_concat(&[&*v76, &*v81, "s"]);
                v83
            } else {
                let v84: Rc<str> = format_real_7(v63);
                let v85: Rc<str> = string_concat(&[&*v84, "s"]);
                v85
            }
        };
        let v88: Rc<str> = string_concat(&["in_", &*v87]);
        v88
    };
    let v91: Rc<str> = string_concat(&[&*v25, &*v89, " wrap_at="]);
    let v92: bool = v2 <= v7;
    let v155: Rc<str> = if v92 {
        let v93: u64 = v7.saturating_sub(v2);
        let v94: u64 = v93.wrapping_div(1000u64);
        let v95: u64 = v94.wrapping_div(3600u64);
        let v96: u64 = v94.wrapping_div(60u64);
        let v97: u64 = v96.wrapping_rem(60u64);
        let v98: u64 = v94.wrapping_rem(60u64);
        let v99: bool = 0u64 < v95;
        let v122: Rc<str> = if v99 {
            let v100: Rc<str> = format_real_7(v95);
            let v101: Rc<str> = string_concat(&[&*v100, "h"]);
            let v102: bool = v97 <= 9u64;
            let v106: Rc<str> = if v102 {
                let v103: Rc<str> = format_real_7(v97);
                let v104: Rc<str> = string_concat(&["0", &*v103]);
                v104
            } else {
                format_real_7(v97)
            };
            let v108: Rc<str> = string_concat(&[&*v101, &*v106, "m"]);
            v108
        } else {
            let v109: bool = 0u64 < v97;
            if v109 {
                let v110: Rc<str> = format_real_7(v97);
                let v111: Rc<str> = string_concat(&[&*v110, "m"]);
                let v112: bool = v98 <= 9u64;
                let v116: Rc<str> = if v112 {
                    let v113: Rc<str> = format_real_7(v98);
                    let v114: Rc<str> = string_concat(&["0", &*v113]);
                    v114
                } else {
                    format_real_7(v98)
                };
                let v118: Rc<str> = string_concat(&[&*v111, &*v116, "s"]);
                v118
            } else {
                let v119: Rc<str> = format_real_7(v98);
                let v120: Rc<str> = string_concat(&[&*v119, "s"]);
                v120
            }
        };
        let v123: Rc<str> = string_concat(&[&*v122, "_ago"]);
        v123
    } else {
        let v124: u64 = v2.saturating_sub(v7);
        let v125: u64 = v124.wrapping_div(1000u64);
        let v126: u64 = v125.wrapping_div(3600u64);
        let v127: u64 = v125.wrapping_div(60u64);
        let v128: u64 = v127.wrapping_rem(60u64);
        let v129: u64 = v125.wrapping_rem(60u64);
        let v130: bool = 0u64 < v126;
        let v153: Rc<str> = if v130 {
            let v131: Rc<str> = format_real_7(v126);
            let v132: Rc<str> = string_concat(&[&*v131, "h"]);
            let v133: bool = v128 <= 9u64;
            let v137: Rc<str> = if v133 {
                let v134: Rc<str> = format_real_7(v128);
                let v135: Rc<str> = string_concat(&["0", &*v134]);
                v135
            } else {
                format_real_7(v128)
            };
            let v139: Rc<str> = string_concat(&[&*v132, &*v137, "m"]);
            v139
        } else {
            let v140: bool = 0u64 < v128;
            if v140 {
                let v141: Rc<str> = format_real_7(v128);
                let v142: Rc<str> = string_concat(&[&*v141, "m"]);
                let v143: bool = v129 <= 9u64;
                let v147: Rc<str> = if v143 {
                    let v144: Rc<str> = format_real_7(v129);
                    let v145: Rc<str> = string_concat(&["0", &*v144]);
                    v145
                } else {
                    format_real_7(v129)
                };
                let v149: Rc<str> = string_concat(&[&*v142, &*v147, "s"]);
                v149
            } else {
                let v150: Rc<str> = format_real_7(v129);
                let v151: Rc<str> = string_concat(&[&*v150, "s"]);
                v151
            }
        };
        let v154: Rc<str> = string_concat(&["in_", &*v153]);
        v154
    };
    let v156: Rc<str> = string_concat(&[&*v91, &*v155]);
    let v157: bool = v3 <= v7;
    let v220: Rc<str> = if v157 {
        let v158: u64 = v7.saturating_sub(v3);
        let v159: u64 = v158.wrapping_div(1000u64);
        let v160: u64 = v159.wrapping_div(3600u64);
        let v161: u64 = v159.wrapping_div(60u64);
        let v162: u64 = v161.wrapping_rem(60u64);
        let v163: u64 = v159.wrapping_rem(60u64);
        let v164: bool = 0u64 < v160;
        let v187: Rc<str> = if v164 {
            let v165: Rc<str> = format_real_7(v160);
            let v166: Rc<str> = string_concat(&[&*v165, "h"]);
            let v167: bool = v162 <= 9u64;
            let v171: Rc<str> = if v167 {
                let v168: Rc<str> = format_real_7(v162);
                let v169: Rc<str> = string_concat(&["0", &*v168]);
                v169
            } else {
                format_real_7(v162)
            };
            let v173: Rc<str> = string_concat(&[&*v166, &*v171, "m"]);
            v173
        } else {
            let v174: bool = 0u64 < v162;
            if v174 {
                let v175: Rc<str> = format_real_7(v162);
                let v176: Rc<str> = string_concat(&[&*v175, "m"]);
                let v177: bool = v163 <= 9u64;
                let v181: Rc<str> = if v177 {
                    let v178: Rc<str> = format_real_7(v163);
                    let v179: Rc<str> = string_concat(&["0", &*v178]);
                    v179
                } else {
                    format_real_7(v163)
                };
                let v183: Rc<str> = string_concat(&[&*v176, &*v181, "s"]);
                v183
            } else {
                let v184: Rc<str> = format_real_7(v163);
                let v185: Rc<str> = string_concat(&[&*v184, "s"]);
                v185
            }
        };
        let v188: Rc<str> = string_concat(&[&*v187, "_ago"]);
        v188
    } else {
        let v189: u64 = v3.saturating_sub(v7);
        let v190: u64 = v189.wrapping_div(1000u64);
        let v191: u64 = v190.wrapping_div(3600u64);
        let v192: u64 = v190.wrapping_div(60u64);
        let v193: u64 = v192.wrapping_rem(60u64);
        let v194: u64 = v190.wrapping_rem(60u64);
        let v195: bool = 0u64 < v191;
        let v218: Rc<str> = if v195 {
            let v196: Rc<str> = format_real_7(v191);
            let v197: Rc<str> = string_concat(&[&*v196, "h"]);
            let v198: bool = v193 <= 9u64;
            let v202: Rc<str> = if v198 {
                let v199: Rc<str> = format_real_7(v193);
                let v200: Rc<str> = string_concat(&["0", &*v199]);
                v200
            } else {
                format_real_7(v193)
            };
            let v204: Rc<str> = string_concat(&[&*v197, &*v202, "m"]);
            v204
        } else {
            let v205: bool = 0u64 < v193;
            if v205 {
                let v206: Rc<str> = format_real_7(v193);
                let v207: Rc<str> = string_concat(&[&*v206, "m"]);
                let v208: bool = v194 <= 9u64;
                let v212: Rc<str> = if v208 {
                    let v209: Rc<str> = format_real_7(v194);
                    let v210: Rc<str> = string_concat(&["0", &*v209]);
                    v210
                } else {
                    format_real_7(v194)
                };
                let v214: Rc<str> = string_concat(&[&*v207, &*v212, "s"]);
                v214
            } else {
                let v215: Rc<str> = format_real_7(v194);
                let v216: Rc<str> = string_concat(&[&*v215, "s"]);
                v216
            }
        };
        let v219: Rc<str> = string_concat(&["in_", &*v218]);
        v219
    };
    let v222: Rc<str> = string_concat(&[" deadline=", &*v220, " remaining="]);
    let v223: u64 = v4.wrapping_div(1000u64);
    let v224: u64 = v223.wrapping_div(3600u64);
    let v225: u64 = v223.wrapping_div(60u64);
    let v226: u64 = v225.wrapping_rem(60u64);
    let v227: u64 = v223.wrapping_rem(60u64);
    let v228: bool = 0u64 < v224;
    let v251: Rc<str> = if v228 {
        let v229: Rc<str> = format_real_7(v224);
        let v230: Rc<str> = string_concat(&[&*v229, "h"]);
        let v231: bool = v226 <= 9u64;
        let v235: Rc<str> = if v231 {
            let v232: Rc<str> = format_real_7(v226);
            let v233: Rc<str> = string_concat(&["0", &*v232]);
            v233
        } else {
            format_real_7(v226)
        };
        let v237: Rc<str> = string_concat(&[&*v230, &*v235, "m"]);
        v237
    } else {
        let v238: bool = 0u64 < v226;
        if v238 {
            let v239: Rc<str> = format_real_7(v226);
            let v240: Rc<str> = string_concat(&[&*v239, "m"]);
            let v241: bool = v227 <= 9u64;
            let v245: Rc<str> = if v241 {
                let v242: Rc<str> = format_real_7(v227);
                let v243: Rc<str> = string_concat(&["0", &*v242]);
                v243
            } else {
                format_real_7(v227)
            };
            let v247: Rc<str> = string_concat(&[&*v240, &*v245, "s"]);
            v247
        } else {
            let v248: Rc<str> = format_real_7(v227);
            let v249: Rc<str> = string_concat(&[&*v248, "s"]);
            v249
        }
    };
    let v252: Rc<str> = string_concat(&[&*v222, &*v251]);
    let v255: Rc<str> = if v5 {
        let v253: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v253
    } else {
        let v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v254
    };
    let v257: Rc<str> = string_concat(&[" should_wrap=", &*v255, " should_yield="]);
    let v260: Rc<str> = if v6 {
        let v258: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v258
    } else {
        let v259: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v259
    };
    let v262: Rc<str> = format_real_7(v1);
    let v265: Rc<str> = format_real_7(v2);
    let v267: Rc<str> = format_real_7(v3);
    let v270: Rc<str> = format_real_7(v4);
    let v276: Rc<str> = string_concat(&[&*v20, &*v156, &*v252, &*v257, &*v260, " started_unix_ms=", &*v262, " wrap_at_unix_ms=", &*v265, " deadline_unix_ms=", &*v267, " remaining_ms=", &*v270]);
    v276
}
fn closure2() -> Rc<dyn Fn(Rc<str>, u64, u64, u64, u64, bool, bool) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64, u64, u64, u64, bool, bool) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: bool, mut v6: bool| -> Rc<str> {
        lease_status_entry_6(v0, v1, v2, v3, v4, v5, v6)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rig_lease_effect_code() -> i32 {
    closure0()()
}
pub fn rig_lease_policy_text() -> Rc<str> {
    closure1()()
}
pub fn rig_lease_status_text(v0: &str, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: bool) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), v1, v2, v3, v4, v5, v6)
}
pub type SpiralShape_97df63ebabc3 = Mut0;
