use crate::utils::jni_helpers::*;
use crate::utils::logger::{dbg_error, dbg_info};
use crate::{egui_android_backend};
use dobby_rs::hook;
use jni::objects::{JMethodID, JObject, JValue, JValueGen};
use jni::sys::{jboolean, jint, JNINativeMethod, JNI_VERSION_1_6};
use jni::JNIEnv;
use log::{error, info};
use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void, CStr, CString};
use std::mem::transmute;
use std::sync::OnceLock;
use goldberg::goldberg_string;

static ORIG_NATIVEINJECTEVENT: OnceLock<u64> = OnceLock::new();
static MOTION_GET_ACTION_ID: OnceLock<JMethodID> = OnceLock::new();
static MOTION_GET_X_ID: OnceLock<JMethodID> = OnceLock::new();
static MOTION_GET_Y_ID: OnceLock<JMethodID> = OnceLock::new();
static KEY_EVENT_GET_ACTION_ID: OnceLock<JMethodID> = OnceLock::new();
static KEY_EVENT_GET_KEYCODE_ID: OnceLock<JMethodID> = OnceLock::new();
static KEY_EVENT_GET_UNICODE_CHAR_ID: OnceLock<JMethodID> = OnceLock::new();
static KEY_EVENT_GET_META_STATE_ID: OnceLock<JMethodID> = OnceLock::new();

#[inline(never)]
fn hk_native_inject_event<'local>(mut env: JNIEnv<'local>, obj: JObject, event: JObject) -> i32 {
    // dbg_info!("nativeInjectEvent triggered!");

    let init = egui_android_backend::IMGUI_INIT.get();
    if let None = init {
        dbg_error!("IMGUI_INIT is none");
        return 0;
    }
    if let Some(init) = init {
        if !init {
        } else {
            let motion_event_cls = checked_find_class!(env, goldberg_string!("android/view/MotionEvent")).unwrap();
            let key_event_cls = checked_find_class!(env, goldberg_string!("android/view/KeyEvent")).unwrap();
            if env.is_instance_of(&event, motion_event_cls).unwrap() {
                // dbg_info!("Motion event");

                // if MOTION_GET_ACTION_ID.get().is_none() {
                //     let motion_get_action_id = checked_get_method_id!(env, motion_event_cls, "getAction", "()I");
                //     MOTION_GET_ACTION_ID.set(motion_get_action_id);
                // }
                // if MOTION_GET_X_ID.get().is_none() {
                //     let motion_get_x_id = checked_get_method_id!(env, motion_event_cls, "getX", "()F");
                //     MOTION_GET_X_ID.set(motion_get_x_id);
                // }
                // if MOTION_GET_Y_ID.get().is_none() {
                //     let motion_get_y_id = checked_get_method_id!(env, motion_event_cls, "getY", "()F");
                //     MOTION_GET_Y_ID.set(motion_get_y_id);
                // }

                // let io = unsafe { imgui_sys::igGetIO() };
                // let mut io = unsafe { &mut *io };
                let x = checked_call_method!(env, &event, goldberg_string!("getX"), goldberg_string!("()F"), &[], JNI_VERSION_1_6)
                    .f()
                    .unwrap();
                let y = checked_call_method!(env, &event, goldberg_string!("getY"), goldberg_string!("()F"), &[], JNI_VERSION_1_6)
                    .f()
                    .unwrap();
                let action =
                    checked_call_method!(env, &event, goldberg_string!("getAction"), goldberg_string!("()I"), &[], JNI_VERSION_1_6)
                        .i()
                        .unwrap();
                // dbg_info!("X: {}", x);
                // dbg_info!("Y: {}", y);
                // dbg_info!("Action: {}", action);

                egui_android_backend::imgui_impl_android_handle_inputevent_x_y_event(
                    x as i32, y as i32, action,
                );

            } else if env.is_instance_of(&event, key_event_cls).unwrap() {
                dbg_info!("Key event");

                // if KEY_EVENT_GET_ACTION_ID.get().is_none() {
                //     let key_get_action_id = checked_get_method_id!(env, key_event_cls, "getAction", "()I");
                //     KEY_EVENT_GET_ACTION_ID.set(key_get_action_id);
                // }
                // if KEY_EVENT_GET_KEYCODE_ID.get().is_none() {
                //     let key_get_keycode_id = checked_get_method_id!(env, key_event_cls, "getKeyCode", "()I");
                //     KEY_EVENT_GET_KEYCODE_ID.set(key_get_keycode_id);
                // }
                // // getunicodechar
                // if KEY_EVENT_GET_UNICODE_CHAR_ID.get().is_none() {
                //     let key_get_unicode_char_id = checked_get_method_id!(env, key_event_cls, "getUnicodeChar", "(I)I");
                //     KEY_EVENT_GET_UNICODE_CHAR_ID.set(key_get_unicode_char_id);
                // }
                // // getmetastate
                // if KEY_EVENT_GET_META_STATE_ID.get().is_none() {
                //     let key_get_meta_state_id = checked_get_method_id!(env, key_event_cls, "getMetaState", "()I");
                //     KEY_EVENT_GET_META_STATE_ID.set(key_get_meta_state_id);
                // }
                let action =
                    checked_call_method!(env, &event, goldberg_string!("getAction"), goldberg_string!("()I"), &[], JNI_VERSION_1_6)
                        .i()
                        .unwrap();
                // dbg_info!("Action: {}", action);
                if action == 0 {
                    let key_code = checked_call_method!(
                        env,
                        &event,
                        goldberg_string!("getKeyCode"),
                        "()I",
                        &[],
                        JNI_VERSION_1_6
                    )
                    .i()
                    .unwrap();
                }
            }
        }
    }

    let orig_native_inject_event = ORIG_NATIVEINJECTEVENT.get().unwrap();
    let orig_native_inject_event = unsafe {
        transmute::<usize, extern "system" fn(JNIEnv, JObject, JObject) -> i32>(
            *orig_native_inject_event as usize,
        )
    };

    orig_native_inject_event(env, obj, event)
}

pub static ORIG_REGISTERNAITIVES: OnceLock<u64> = OnceLock::new();
#[inline(never)]
pub fn hook_register_natives(
    env: JNIEnv,
    class: JObject,
    methods: *const JNINativeMethod,
    method_count: jint,
) -> jint {
    // dbg_info!("Hooking RegisterNatives");
    for i in 0..method_count {
        let method = unsafe { methods.offset(i as isize) };
        let name = unsafe { CStr::from_ptr((*method).name) };
        let sig = unsafe { CStr::from_ptr((*method).signature) };
        dbg_info!("Method: {:?}, Sig: {:?}", name, sig);
        // check nativeInjectEvent
        if name.to_str().unwrap() == goldberg_string!("nativeInjectEvent") {
            dbg_info!("Hooking nativeInjectEvent");
            let orig_native_inject_event = unsafe {
                hook(
                    (*method).fnPtr as *mut c_void,
                    hk_native_inject_event as *mut c_void,
                )
                .unwrap()
            };
            dbg_info!("hooked nativeInjectEvent");
            ORIG_NATIVEINJECTEVENT.get_or_init(|| orig_native_inject_event as u64);
        }
    }
    let orig_register_natives = ORIG_REGISTERNAITIVES.get().unwrap();
    let orig_register_natives = unsafe {
        transmute::<usize, extern "system" fn(JNIEnv, JObject, *const JNINativeMethod, jint) -> jint>(
            *orig_register_natives as usize,
        )
    };

    orig_register_natives(env, class, methods, method_count)
}
