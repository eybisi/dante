use crate::*;

pub fn hook_il2cpp_method(
    target_class_name: &str,
    target_function_name: &str,
    target_fnc_offset: usize,
    target_global_variable: &OnceLock<usize>, 
) -> usize {
    let lib = unsafe { libloading::Library::new("libil2cpp.so").expect("load payload") };

    let il2cpp_domain_get = unsafe {
        let func: libloading::Symbol<unsafe extern "C" fn() -> c_int> =
            lib.get(goldberg_string!("il2cpp_domain_get").as_bytes()).expect("get pid function");
        func.into_raw().into_raw() as usize
    };
    // Call il2cpp_domain_get
    let domain = unsafe { transmute::<usize, extern "system" fn() -> c_int>(il2cpp_domain_get)() };
    // dbg_info!("domain: {}", domain);
    let il2cpp_domain_get_assemblies = unsafe {
        let func: libloading::Symbol<unsafe extern "C" fn(c_int, *mut c_int) -> *mut c_int> = lib
            .get(goldberg_string!("il2cpp_domain_get_assemblies").as_bytes())
            .expect("get pid function");
        func.into_raw().into_raw() as usize
    };
    // Walk image and find image with name "Assembly-CSharp"
    let mut size = -1;
    // assemblies is pointer to pointer of assembly
    let assemblies = unsafe {
        transmute::<usize, extern "system" fn(c_int, *mut c_int) -> *mut *mut c_void>(
            il2cpp_domain_get_assemblies,
        )(domain, &mut size)
    };
    let il2cpp_assembly_get_image = unsafe {
        let func: libloading::Symbol<unsafe extern "system" fn(c_int) -> c_int> = lib
            .get(goldberg_string!("il2cpp_assembly_get_image").as_bytes())
            .expect("il2cpp_assembly_get_image not found");
        func.into_raw().into_raw() as usize
    };

    for i in 0..size {
        // il2cpp_assembly_get_image
        let ptr_ptr_image = unsafe {
            transmute::<usize, extern "system" fn(*mut c_void) -> *mut c_uint>(
                il2cpp_assembly_get_image,
            )((assemblies as usize + (i * std::mem::size_of::<usize>() as i32) as usize) as *mut c_void)
        };
        unsafe {
            let ptr_image = std::ptr::read((ptr_ptr_image as usize) as *const usize);
            let image_name_ptr = std::ptr::read((ptr_image + std::mem::size_of::<usize>()) as *const *const c_char);
            let image_name = CStr::from_ptr(image_name_ptr).to_str();
            if image_name.is_err() {
                dbg_error!("image name is err");
                continue;
            }
            let image_name = image_name.unwrap();
            if image_name.eq(goldberg_string!("Assembly-CSharp")) {
                dbg_info!("Found Assembly-CSharp");
                let il2cpp_image_get_class_count = {
                    let func: libloading::Symbol<unsafe extern "system" fn(*mut c_void) -> size_t> =
                        lib.get(goldberg_string!("il2cpp_image_get_class_count").as_bytes())
                            .expect("il2cpp_image_get_class_count not found");
                    func.into_raw().into_raw() as usize
                };

                let class_count = {
                    transmute::<usize, extern "system" fn(*mut c_void) -> size_t>(
                        il2cpp_image_get_class_count,
                    )(ptr_image as *mut c_void)
                };
                let il2cpp_image_get_class = {
                    let func: libloading::Symbol<unsafe extern "system" fn(c_int, c_int) -> c_int> =
                        lib.get(goldberg_string!("il2cpp_image_get_class").as_bytes())
                            .expect("il2cpp_image_get_class not found");
                    func.into_raw().into_raw() as usize
                };

                let il2cpp_class_get_methods = {
                    let func: libloading::Symbol<
                        unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> *mut c_void,
                    > = lib
                        .get(goldberg_string!("il2cpp_class_get_methods").as_bytes())
                        .expect("il2cpp_class_get_methods not found");
                    func.into_raw().into_raw() as usize
                };

                let il2cpp_method_get_name = {
                    let func: libloading::Symbol<unsafe extern "system" fn(c_int) -> c_int> = lib
                        .get(goldberg_string!("il2cpp_method_get_name").as_bytes())
                        .expect("il2cpp_method_get_name not found");
                    func.into_raw().into_raw() as usize
                };

                for i in 0..class_count {
                    let p_class = {
                        transmute::<usize, extern "system" fn(*mut c_void, size_t) -> *mut c_void>(
                            il2cpp_image_get_class,
                        )(ptr_image as *mut c_void, i)
                    };

                    // Read p_class + 0x10
                    let p_class_name_ptr =
                        std::ptr::read((p_class as usize + 2*std::mem::size_of::<usize>()) as *const *const c_char);
                    // Convert it to str
                    let p_class_name = CStr::from_ptr(p_class_name_ptr).to_str();
                    if p_class_name.is_err() {
                        dbg_error!("class name is err");
                        continue;
                    }
                    let class_name = p_class_name.unwrap();
                    if !class_name.eq(target_class_name) {
                        continue;
                    }

                    // dbg_info!("class ptr: {:x}",p_class as usize);
                    let mut function_iter = 0u64;
                    loop {
                        let method = {
                            transmute::<
                                usize,
                                extern "system" fn(*mut c_void, *mut c_ulong) -> *mut c_void,
                            >(il2cpp_class_get_methods)(
                                p_class, &mut function_iter
                            )
                        };
                        if method.is_null() {
                            // dbg_info!("method is null");
                            break;
                        }
                        // dbg_info!("method: {:x}",method as usize);
                        // let method_name = CStr::from_ptr(method).to_str().unwrap();
                        // dbg_info!("method name: {}",method_name);
                        // // Get method addr
                        let method_addr = std::ptr::read((method as usize) as *const usize);
                        // dbg_info!("method addr: {:x}",method_addr);

                        // get il2cpp_method_get_name
                        let method_name_ptr = {
                            transmute::<usize, extern "system" fn(*mut c_void) -> *mut c_char>(
                                il2cpp_method_get_name,
                            )(method as *mut c_void)
                        };
                        let method_name = CStr::from_ptr(method_name_ptr).to_str().unwrap();
                        if !method_name.eq(target_function_name) {
                            continue;
                        } else {
                            // let method_name = CStr::from_ptr(method).to_str().unwrap();
                            dbg_info!("method name: {}", method_name);
                            let orig_target_function =
                                hook(method_addr as Address, target_fnc_offset as *mut c_void);
                            if orig_target_function.is_err() {
                                dbg_error!("Error: {:?}", orig_target_function);
                                return 0;
                            }

                            let orig_target_function = orig_target_function.unwrap();

                            target_global_variable.get_or_init(|| orig_target_function as usize);


                            // if method_name.eq("LupState") {
                            //     ORIG_LUPSTATE.get_or_init(|| orig_target_function as usize);
                            //     // ORIG_LUPSTATE = Some(orig_target_function as usize);
                            // }
                            // if method_name.eq("IsEmulator") {
                            //     ORIG_ISEMULATOR.get_or_init(|| orig_target_function as usize);
                            //     // ORIG_ISEMULATOR = Some(orig_target_function as usize);
                            // }
                            // if method_name.eq("ReceivedGenieStop")
                            //     || method_name.eq("ReceivedRobotPackets")
                            //     || method_name.eq("IsInSkillAnimation")
                            //     || method_name.eq("CancelIfCasting")
                            // {
                            //     dbg_info!(
                            //         "Not setting orig func to global variable,since we dont use orig function for {}",
                            //         method_name
                            //     );
                            // }
                            dbg_info!(
                                "Hooked {} {} at {:p}",
                                class_name,
                                method_name,
                                orig_target_function
                            );
                        }
                    }
                }
                break;
            }
        }
    }
    0
}
