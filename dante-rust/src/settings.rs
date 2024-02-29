use std::collections::HashMap;
use jni::sys::jint;
use crate::*;
use std::sync::Mutex;

// Global settings that holds the settings and value of the settings
// This is a hashmap that holds the setting and the value of the setting

// lazy_static::lazy_static! {
//     static ref SETTINGS: Arc<Mutex<HashMap<Setting, bool>>> = {
//         Arc::new(Mutex::new({
//             let mut settings = HashMap::new();
//             settings.insert(Setting::EmulatorBypass, false);
//             settings.insert(Setting::InfiniteGenie, false);
//             settings.insert(Setting::AutoLoot, false);
//             settings.insert(Setting::ImnotrobotBypass, false);
//             settings.insert(Setting::MoveInAnimation, false);
//             settings.insert(Setting::LuponHack, false);
//             settings
//         }))
//     };
// }
pub static SETTINGS: OnceLock<Mutex<HashMap<Setting, bool>>> = OnceLock::new();



#[derive(Debug,Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Setting {
    EmulatorBypass = 0,
    InfiniteGenie,
    AutoLoot,
    ImnotrobotBypass,
    MoveInAnimation,
    LuponHack,
}

impl From<u32> for Setting {
    fn from(n: u32) -> Self {
        match n {
            0 => Setting::EmulatorBypass,
            1 => Setting::InfiniteGenie,
            2 => Setting::AutoLoot,
            3 => Setting::ImnotrobotBypass,
            4 => Setting::MoveInAnimation,
            5 => Setting::LuponHack,
            _ => panic!("Invalid setting number"),
        }
    }
}

#[inline(never)]
pub fn get_settings_from_java(mut env: JNIEnv, _class: JClass, settings_idx: jint, settings_value: JObject) {
    dbg_info!("get_setting called with int : {}", settings_idx);
    // return;
    // let mut global_settings = SETTINGS.lock().unwrap();
    let global_settings = SETTINGS.get_or_init(|| {
            let mut settings = HashMap::new();
            settings.insert(Setting::EmulatorBypass, false);
            settings.insert(Setting::InfiniteGenie, false);
            settings.insert(Setting::AutoLoot, false);
            settings.insert(Setting::ImnotrobotBypass, false);
            settings.insert(Setting::MoveInAnimation, false);
            settings.insert(Setting::LuponHack, false);
            Mutex::new(settings)
    });    
    let curr_set = Setting::from(settings_idx as u32);
    // Check if settings_value is a java boolean
    let bool_cls = env.find_class(goldberg_string!("java/lang/Boolean")).unwrap();
    if env.is_instance_of(&settings_value, bool_cls).unwrap() {
        let bool_val = checked_call_method!(env, &settings_value, goldberg_string!("booleanValue"), goldberg_string!("()Z"), &[])
            .unwrap()
            .z()
            .unwrap();

        dbg_info!("Setting {:?} is {}", curr_set.clone() as u32, bool_val);
        // global_settings.insert(curr_set.clone(), bool_val);
        global_settings.lock().unwrap().insert(curr_set.clone(), bool_val);
    }

}

