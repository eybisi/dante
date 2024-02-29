#![no_main]
use goldberg::goldberg_string;
use log::{error, info};
// mod imgui_opengl3;
// mod imgui_opengl3_loader;
mod utils;

use jni::objects::{JClass, JObject, JString, JValueGen};
use log::LevelFilter;
use proc_maps::get_process_maps;

// #[cfg(debug_assertions)]
// use backtrace::Backtrace;
use dobby_rs::{hook, Address};
use jni::sys::{jboolean, jint, JNI_VERSION_1_6};
use jni::{JNIEnv, JavaVM, NativeMethod};
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::time::Duration;

use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void, CStr, CString};
use std::mem::transmute;

use libc::{dlclose, dlerror, dlopen, dlsym, getpid, size_t};
use plt_rs::{LinkMapBacked, MutableLinkMap};

use std::sync::OnceLock;



use android_logger::Config;

use ctor::ctor;

type SendFnType = fn(c_int, *mut c_void, c_int, c_int) -> c_int;
type RecvFnType = fn(c_int, *mut c_void, c_int, c_int, *mut c_void, *mut c_void) -> c_int;
// type EglSwapBuffersFnType = fn(*mut c_void, c_uint, *mut c_void, *mut c_void) -> c_int;
// type LupStateFnType = unsafe fn(c_int);

mod bd;
// mod egui_android_backend;
mod fake_libmain;
mod game_hook;
mod il2cpp_stuff;
// mod imgui_android_backend;
// mod imgui_menu;
// mod jni_hooks;
mod knight_unity;
// mod my_plt_hook;
// mod opengl_hooks;
// mod vulkan_hooks;
mod settings;
mod encrypted_file {
    include!(concat!(env!("OUT_DIR"), "/encrypted_file.rs"));
}

pub static LIBUNITY_HANDLE: OnceLock<usize> = OnceLock::new();
pub static LIBIL2CPP_HANDLE: OnceLock<usize> = OnceLock::new();

pub static ORIG_SEND: OnceLock<SendFnType> = OnceLock::new();
pub static ORIG_RECV: OnceLock<RecvFnType> = OnceLock::new();
pub static ORIG_LUPSTATE: OnceLock<usize> = OnceLock::new();
pub static ORIG_ISEMULATOR: OnceLock<usize> = OnceLock::new();
pub static ORIG_GENIESTOP: OnceLock<usize> = OnceLock::new();
pub static ORIG_ROBOT: OnceLock<usize> = OnceLock::new();
pub static ORIG_ROBOTCONTROLLER_RECEIVEDROBOTPACKETS: OnceLock<usize> = OnceLock::new();
pub static ORIG_ROBOTCONTROLLER_SHOW_ROBOT: OnceLock<usize> = OnceLock::new();
pub static ORIG_ROBOTCONTROLLER_ON_USING_SKILL: OnceLock<usize> = OnceLock::new();
pub static ORIG_SENDDEVICEINFO: OnceLock<usize> = OnceLock::new();
pub static ORIG_IS_IN_SKILL_ANIM: OnceLock<usize> = OnceLock::new();
pub static ORIG_IS_IN_CASTING: OnceLock<usize> = OnceLock::new();
pub static ORIG_CANCEL_IF_CASTING: OnceLock<usize> = OnceLock::new();

// vkCreateDevice(g_PhysicalDevice, &create_info, g_Allocator, &g_Device);  4th to get g_Device
// vkCreateInstance(&create_info, g_Allocator, &g_Instance);  3rd to get g_Instance
// vkEnumeratePhysicalDevices; return value g_PhysicalDevice
// vkCreateDescriptorPool(g_Device, &pool_info, g_Allocator, &g_DescriptorPool); to get g_DescriptorPool
// vkGetDeviceQueue(g_Device, g_QueueFamily, 0, &g_Queue); to get g_Queue
// vkGetSwapchainImagesKHR(device, wd->Swapchain, &wd->ImageCount, backbuffers); get g_ImageCount

//PFN_vkVoidFunction vkGetInstanceProcAddr(
// VkInstance                                  instance,
// const char*                                 pName);

#[ctor]
fn init_func() {
    // initialize_logging();
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Info)
            .with_tag("dante-native"),
    );

    dbg_info!("Hello, world!");
    // let (tx, rx) = channel::<egui::Event>();
    // egui_android_backend::EGUI_EVENT_SENDER.get_or_init(|| Mutex::new(tx));
    // egui_android_backend::IMGUI_INIT.get_or_init(|| true);
    // opengl_hooks::EGUI_EVENT_RECEIVER.get_or_init(|| Mutex::new(rx));

    // vulkan_hooks::hook_vk_instanceproc_and_deviceproc();
    // opengl_hooks::hook_egl_proc_addr();
}



#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    dbg_info!("JNI_OnLoad called");
    dbg_info!("Registering unity native functions");
    fake_libmain::register_unity_native_functions(vm.get_env().expect("get_env"));
    
    dbg_info!("Trying to hook register native pointer");
    let mut env = vm.get_env().unwrap();
    let unityplayer_cls =
        checked_find_class!(env, goldberg_string!("com/unity3d/player/UnityPlayer"), JNI_VERSION_1_6);
    let _curr_act_fid = env
        .get_static_field_id(unityplayer_cls, goldberg_string!("currentActivity"), goldberg_string!("Landroid/app/Activity;"))
        .unwrap();
    // let raw_env = unsafe { *env.get_raw() };
    // let raw_env = unsafe { *raw_env };
    // let _pp = raw_env.RegisterNatives.unwrap() as *mut c_void;

    // //0x6B8
    // let orig_regnatives = unsafe{hook(pp, jni_hooks::hook_register_natives as *mut c_void).unwrap()};
    // jni_hooks::ORIG_REGISTERNAITIVES.get_or_init(|| orig_regnatives as u64);
    // dbg_info!("Hooked RegisterNatives at {:p}", orig_regnatives);

    let (tx, rx) = channel();

    // Create a new thread for the hooking
    let _ = std::thread::spawn(move || {
        if let Err(_e) = tx.send(()) {
            dbg_error!("ERROR: thread starter sending channel {:?}", _e);
            return;
        };
        std::thread::sleep(Duration::from_secs(5));
        dbg_info!("Hooking thread started");

        bd::hook_il2cpp_send_recv();
        game_hook::hook_game_functions();

        dbg_info!("il2cpp functions hooked");
    });
    if let Err(_e) = rx.recv() {
        dbg_error!("ERROR: thread waiting receiver:  {:?}", _e);
    }
    load_plugin_apk_trigger_init(vm);
    JNI_VERSION_1_6
}

fn load_plugin_apk_trigger_init(vm: JavaVM) -> i32 {
    // get JNIEnv
    let mut env = vm.get_env().expect("get_env");
    let cls = checked_find_class!(env, goldberg_string!("android/app/ActivityThread"), JNI_VERSION_1_6);

    let a = checked_call_static_method!(
        env,
        &cls,
        goldberg_string!("currentApplication"),
        goldberg_string!("()Landroid/app/Application;"),
        &[],
        JNI_VERSION_1_6
    );

    let ctx = match a.l() {
        Ok(a) => a,
        Err(_e) => {
            dbg_error!("ERROR: cannot get context obj {:?}", _e);
            return JNI_VERSION_1_6;
        }
    };

    let files = checked_call_method!(
        env,
        &ctx,
        goldberg_string!("getFilesDir"),
        goldberg_string!("()Ljava/io/File;"),
        &[],
        JNI_VERSION_1_6
    );

    let file = match files.l() {
        Ok(a) => a,
        Err(_e) => {
            dbg_error!("ERROR: cannot get files dir obj {:?}", _e);
            return JNI_VERSION_1_6;
        }
    };

    let filesdirpath = checked_call_method!(
        env,
        &file,
        goldberg_string!("getAbsolutePath"),
        goldberg_string!("()Ljava/lang/String;"),
        &[],
        JNI_VERSION_1_6
    );

    let files_path: String = env
        .get_string(&filesdirpath.l().unwrap().into())
        .unwrap()
        .into();
    dbg_info!("filesdirpath: {}", files_path);

    // Get nativeLibraryDir from ctx.applicationInfo.nativeLibraryDir
    let j_app_info = checked_call_method!(
        env,
        &ctx,
        goldberg_string!("getApplicationInfo"),
        goldberg_string!("()Landroid/content/pm/ApplicationInfo;"),
        &[],
        JNI_VERSION_1_6
    );

    let app_info = match j_app_info.l() {
        Ok(a) => a,
        Err(_e) => {
            dbg_error!("ERROR: cannot get app info obj {:?}", _e);
            return JNI_VERSION_1_6;
        }
    };

    let native_library_dir = env
        .get_field(app_info, goldberg_string!("nativeLibraryDir"), goldberg_string!("Ljava/lang/String;"))
        .unwrap();
    let native_library_dir = match native_library_dir.l() {
        Ok(a) => a,
        Err(_e) => {
            dbg_error!("ERROR: cannot get nativeLibraryDir obj {:?}", _e);
            return JNI_VERSION_1_6;
        }
    };
    let native_library_dir: String = env.get_string(&native_library_dir.into()).unwrap().into();
    dbg_info!("nativeLibraryDir: {}", native_library_dir);

    let cls_loader = checked_find_class!(env, goldberg_string!("java/lang/ClassLoader"), JNI_VERSION_1_6);

    let class_loader = checked_call_static_method!(
        env,
        &cls_loader,
        goldberg_string!("getSystemClassLoader"),
        goldberg_string!("()Ljava/lang/ClassLoader;"),
        &[],
        JNI_VERSION_1_6
    );

    let class_loader = match class_loader.l() {
        Ok(a) => a,
        Err(_e) => {
            dbg_error!("ERROR: cannot get class loader obj {:?}", _e);
            return JNI_VERSION_1_6;
        }
    };
    dbg_error!("class_loader: loaded");

    // Read payload/app-debug.apk and write to filesPath/plugin.apk
    //let payload = utils::xor_runt(include_bytes!("payload/app-debug.apk"));
    let payload = utils::xor_runt(encrypted_file::ENCRYPTED_FILE_CONTENTS);
    let mut file = File::create(format!("{}/{}", files_path, goldberg_string!("plugin.apk"))).unwrap();
    file.write_all(payload.as_slice()).unwrap();

    dbg_info!("plugin.apk written to filesPath");
    let plugin_path = format!("{}/{}", files_path,goldberg_string!("plugin.apk"));
    // Create new PathClassLoader
    let plugin_class_loader = env.new_object(
        goldberg_string!("dalvik/system/PathClassLoader"),
        goldberg_string!("(Ljava/lang/String;Ljava/lang/String;Ljava/lang/ClassLoader;)V"),
        &[
            JValueGen::from(&env.new_string(plugin_path).unwrap()),
            JValueGen::from(&env.new_string(native_library_dir).unwrap()),
            JValueGen::Object(&class_loader),
        ],
    );
    if plugin_class_loader.is_err() {
        dbg_error!("ERROR: cannot create plugin class loader");
        return JNI_VERSION_1_6;
    }
    let plugin_class_loader = plugin_class_loader.unwrap();

    // Call findClass on plugin class loader
    let u_plugin = checked_call_method!(
        env,
        plugin_class_loader,
        goldberg_string!("findClass"),
        goldberg_string!("(Ljava/lang/String;)Ljava/lang/Class;"),
        &[JValueGen::from(
            &env.new_string(goldberg_string!("me.underworld.dante.DantePlugin")).unwrap()
        )]
    );

    if u_plugin.is_err() {
        dbg_error!("ERROR: cannot find main activity class");
        return JNI_VERSION_1_6;
    }
    let u_plugin = u_plugin.unwrap();
    let u_plugin = match u_plugin.l() {
        Ok(a) => a,
        Err(_e) => {
            dbg_error!("ERROR: cannot get main activity class obj {:?}", _e);
            return JNI_VERSION_1_6;
        }
    };

    // public static native void pushSettings(int settingIndex, Object obj);
    let methods: [NativeMethod; 1] = [NativeMethod {
        name: goldberg_string!("pushSettings").to_string().into(),
        sig: goldberg_string!("(ILjava/lang/Object;)V").into(),
        fn_ptr: settings::get_settings_from_java as *mut c_void,
    }];

    let res = env.register_native_methods(JClass::from(u_plugin), &methods);
    if res.is_err() {
        dbg_error!("ERROR: cannot register native methods");
        return JNI_VERSION_1_6;
    }
    dbg_info!("Registered native methods");

    // Get nativeLibraryDir from ctx.applicationInfo.nativeLibraryDir
    let j_app_info = checked_call_method!(
        env,
        &ctx,
        goldberg_string!("getApplicationInfo"),
        goldberg_string!("()Landroid/content/pm/ApplicationInfo;"),
        &[],
        JNI_VERSION_1_6
    );

    let _app_info = match j_app_info.l() {
        Ok(a) => a,
        Err(_e) => {
            dbg_error!("ERROR: cannot get app info obj {:?}", _e);
            return JNI_VERSION_1_6;
        }
    };
    let u_plugin = checked_call_method!(
        env,
        plugin_class_loader,
        goldberg_string!("findClass"),
        goldberg_string!("(Ljava/lang/String;)Ljava/lang/Class;"),
        &[JValueGen::from(
            &env.new_string(goldberg_string!("me.underworld.dante.DantePlugin")).unwrap()
        )]
    );

    if u_plugin.is_err() {
        dbg_error!("ERROR: cannot find main activity class");
        return JNI_VERSION_1_6;
    }
    let u_plugin = u_plugin.unwrap();
    let u_plugin = match u_plugin.l() {
        Ok(a) => a,
        Err(_e) => {
            dbg_error!("ERROR: cannot get main activity class obj {:?}", _e);
            return JNI_VERSION_1_6;
        }
    };
    let plugin_path = format!("{}/{}", files_path, goldberg_string!("plugin.apk"));
    let res = checked_call_static_method!(
        env,
        JClass::from(u_plugin),
        goldberg_string!("initNative"),
        goldberg_string!("(Landroid/app/Application;Ljava/lang/String;)V"),
        &[
            JValueGen::from(&ctx),
            JValueGen::from(&env.new_string(plugin_path.clone()).unwrap())
        ]
    );
    if res.is_err() {
        dbg_error!("ERROR: cannot call initNative");
        env.exception_check().unwrap();
        return JNI_VERSION_1_6;
    }

    dbg_info!("initNative called");
    dbg_info!("Removing apk from folder");
    let r = std::fs::remove_file(plugin_path);
    match r {
        Ok(_) => {
            dbg_info!("apk removed");
        }
        Err(e) => {
            dbg_error!("ERROR: cannot remove apk {:?}", e);
        }
    }
    JNI_VERSION_1_6
}

// fn initialize_logging() {

//     #[cfg(debug_assertions)] {
//         use tracing_subscriber::layer::SubscriberExt;
//         let stdout_log = tracing_subscriber::fmt::layer().compact();
//         let subscriber = tracing_subscriber::Registry::default().with(stdout_log);
    
//         // Upgrade logger on android
//         #[cfg(target_os = "android")]
//         let subscriber = {
//             let android_layer =
//                 tracing_android::layer("dante-native").expect("Unable to create android tracing layer");
//             subscriber.with(android_layer)
//         };
    
//         tracing::subscriber::set_global_default(subscriber).expect("Unable to set global subscriber");
    
//         // Add panic hook
//         std::panic::set_hook(Box::new(|panic_info| {
//             let backtrace = Backtrace::new();
//             tracing::error!("{backtrace:?}");
//             tracing::error!("{panic_info}");
//         }));
    
//         #[cfg(target_os = "android")]
//         {
//             tracing::warn!("Android logging enabled! Layer created");
//         }
//     }

// }
