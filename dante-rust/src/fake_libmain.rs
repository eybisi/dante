use crate::*;

fn load_native_library(env: &JNIEnv, targetlibpath: String, libhandle: &OnceLock<usize>) {
    if libhandle.get().is_none() {
        dbg_info!("libhandle is some");
        // GetJavaVM
        let vm = env.get_java_vm();
        if vm.is_err() {
            dbg_error!("Unable to retrieve Java VM");
            env.fatal_error("Unable to retrieve Java VM");
        }
        let vm = vm.unwrap();
        let c_str = CString::new(targetlibpath).unwrap();
        let c_world: *const c_char = c_str.as_ptr() as *const c_char;
        let handle = unsafe { dlopen(c_world, 1) };
        if handle.is_null() {
            let dl_open_err = unsafe { dlerror() };
            // Convert to rust string
            let dl_open_err = unsafe { CStr::from_ptr(dl_open_err) }.to_str().unwrap();
            dbg_error!("dlopen error: {}", dl_open_err);
            env.fatal_error("Unable to load library");
        }
        dbg_info!("dlopen success");
        let jni_onload_cstr = CString::new("JNI_OnLoad").unwrap();
        let jni_onload_sym = unsafe { dlsym(handle, jni_onload_cstr.as_ptr() as *const c_char) };
        // If JNI_OnLoad is found in the library, call it
        if !jni_onload_sym.is_null() {
            let jni_onload_sym = unsafe {
                transmute::<*mut c_void, extern "system" fn(JavaVM, *mut c_void) -> jint>(
                    jni_onload_sym,
                )
            };
            let res = jni_onload_sym(vm, std::ptr::null_mut());
            if res != JNI_VERSION_1_6 {
                dbg_error!("JNI_OnLoad returned unexpected version {}", res);
                env.fatal_error("JNI_OnLoad returned unexpected version");
            }
            dbg_info!("JNI_OnLoad called");
        }
        libhandle.get_or_init(|| handle as usize);
    }
}

fn load_library(mut env: JNIEnv, _class: JClass, path: JString) -> jboolean {
    let path: String = env.get_string(&path).unwrap().into();
    dbg_info!("load_library called with path : {}", path);
    let libunity_path = format!("{}/{}.so", path,goldberg_string!("libunity"));
    let libil2cpp_path = format!("{}/{}.so", path,goldberg_string!("libil2cpp"));
    dbg_info!("libunity_path: {}", libunity_path);
    dbg_info!("libil2cpp_path: {}", libil2cpp_path);
    load_native_library(&env, libunity_path, &LIBUNITY_HANDLE);
    dbg_info!("libunity loaded");

    load_native_library(&env, libil2cpp_path, &LIBIL2CPP_HANDLE);
    dbg_info!("libil2cpp loaded");
    true as jboolean
}

fn unload_library(env: JNIEnv) {
    dbg_info!("unload_native_library called");

    let libunity_handle = LIBUNITY_HANDLE.get();
    let libil2cpp_handle = LIBIL2CPP_HANDLE.get();

    if let Some(libunity_handle) = libunity_handle {
        let handle = libunity_handle;
        let vm = env.get_java_vm();
        if vm.is_err() {
            dbg_error!("Unable to retrieve Java VM");
            env.fatal_error(goldberg_string!("Unable to retrieve Java VM"));
        }
        let vm = vm.unwrap();
        let jni_onunload_cstr = CString::new(goldberg_string!("JNI_OnUnload")).unwrap();

        let jni_onunload_sym = unsafe {
            dlsym(
                *handle as *mut c_void,
                jni_onunload_cstr.as_ptr() as *const c_char,
            )
        };
        // If JNI_OnLoad is found in the library, call it
        if !jni_onunload_sym.is_null() {
            let jni_onunload_sym = unsafe {
                transmute::<*mut c_void, extern "system" fn(JavaVM, *mut c_void) -> jint>(
                    jni_onunload_sym,
                )
            };
            let res = jni_onunload_sym(vm, std::ptr::null_mut());
            if res != JNI_VERSION_1_6 {
                dbg_error!("JNI_OnUnload returned unexpected version {}", res);
                env.fatal_error(goldberg_string!("JNI_OnUnload returned unexpected version"));
            }
            dbg_info!("JNI_OnUnload called");
        }
        unsafe { dlclose(*handle as *mut c_void) };
    }
    if let Some(libil2cpp_handle) = libil2cpp_handle {
        let handle = libil2cpp_handle;
        let vm = env.get_java_vm();
        if vm.is_err() {
            dbg_error!("Unable to retrieve Java VM");
            env.fatal_error(goldberg_string!("Unable to retrieve Java VM"));
        }
        let vm = vm.unwrap();
        let jni_onunload_cstr = CString::new(goldberg_string!("JNI_OnUnload")).unwrap();
        let jni_onunload_sym = unsafe {
            dlsym(
                *handle as *mut c_void,
                jni_onunload_cstr.as_ptr() as *const c_char,
            )
        };
        // If JNI_OnLoad is found in the library, call it
        if !jni_onunload_sym.is_null() {
            let jni_onunload_sym = unsafe {
                transmute::<*mut c_void, extern "system" fn(JavaVM, *mut c_void) -> jint>(
                    jni_onunload_sym,
                )
            };
            let res = jni_onunload_sym(vm, std::ptr::null_mut());
            if res != JNI_VERSION_1_6 {
                dbg_error!("JNI_OnUnload returned unexpected version {}", res);
                env.fatal_error(goldberg_string!("JNI_OnUnload returned unexpected version"));
            }
            dbg_info!("JNI_OnUnload called");
        }
        unsafe { dlclose(*handle as *mut c_void) };
    }
}

pub fn register_unity_native_functions(mut env: JNIEnv) -> i32 {
    let methods: [NativeMethod; 2] = [
        NativeMethod {
            name: goldberg_string!("load").to_string().into(),
            sig: goldberg_string!("(Ljava/lang/String;)Z").into(),
            fn_ptr: load_library as *mut c_void,
        },
        NativeMethod {
            name: goldberg_string!("unload").to_string().into(),
            sig: goldberg_string!("()Z").into(),
            fn_ptr: unload_library as *mut c_void,
        },
    ];
    let cls = checked_find_class!(env, goldberg_string!("com/unity3d/player/NativeLoader"), JNI_VERSION_1_6);
    let res = env.register_native_methods(cls, &methods);
    if res.is_err() {
        dbg_error!("ERROR: cannot register native methods");
        env.fatal_error(goldberg_string!("com/unity3d/player/NativeLoader"));
    }
    dbg_info!("Registered native methods");
    JNI_VERSION_1_6
}
