use crate::{settings::SETTINGS, settings,*};

// Mutatable global integer
static GENIE_STOP_COUNT:Mutex<i32> = Mutex::<i32>::new(0);


#[inline(never)]
fn hk_lupstate(some_instance:*mut c_void, ison: u32) {
    dbg_info!("lupstate called");
    if ORIG_LUPSTATE.get().is_some() {
        dbg_info!("LupState parameter : {}", ison);
        let orig_lupstate = ORIG_LUPSTATE.get().unwrap();
        // Cast orig_lupstate to function pointer
        let orig_lupstate =
            unsafe { transmute::<usize, extern "system" fn(*mut c_void,c_int) -> c_int>(*orig_lupstate) };
        if ison != 1 {
            dbg_info!("Calling orig_lupstate with 1");
        }
        orig_lupstate(some_instance,1);
    } else {
        dbg_info!("ORIG_LUPSTATE is none");
    }
}
#[inline(never)]
fn hk_isemulator() -> bool {
    dbg_info!("IsEmulator called");
    if ORIG_ISEMULATOR.get().is_some() {
        // let orig_isemulator = ORIG_ISEMULATOR.get().unwrap();
        // // Cast orig_isemulator to function pointer
        // let orig_isemulator =
        //     unsafe { transmute::<usize, extern "system" fn() -> c_int>(*orig_isemulator) };
        // let ret = orig_isemulator();
        // dbg_info!("IsEmulator returned {}", ret);
        // if ret == 1 {
        //     return false;
        // }
    } else {
        dbg_info!("ORIG_ISEMULATOR is none");
    }

    false
}
#[inline(never)]
fn hk_genie_stop(instance:*mut c_void, _pkt_reader: *mut c_void) {


    dbg_info!("GenieController.ReceivedGenieStop called");
    let g_settings = SETTINGS.get().unwrap().lock().unwrap();
    if *g_settings.get(&settings::Setting::InfiniteGenie).unwrap(){
        let mut count = GENIE_STOP_COUNT.lock().unwrap();
        *count += 1;
        if *count % 2 == 1 {
            dbg_info!("Even stop : do nothing")
        }else{
            dbg_info!("Odd stop : call original");
            let orig_geniestop = ORIG_GENIESTOP.get().unwrap();
            dbg_info!("Odd stop : {}", *orig_geniestop);
            let orig_geniestop_fn = unsafe
            {
                transmute::<usize, extern "system" fn(*mut c_void, *mut c_void)>(*orig_geniestop)
            };
            orig_geniestop_fn(instance,_pkt_reader);
            dbg_info!("Odd stop : called");
        }        
    }else{
        dbg_info!("InfiniteGenie is disabled, call original");
        let orig_geniestop = ORIG_GENIESTOP.get().unwrap();
        dbg_info!("Odd stop : {}", *orig_geniestop);
        let orig_geniestop_fn = unsafe
        {
            transmute::<usize, extern "system" fn(*mut c_void, *mut c_void)>(*orig_geniestop)
        };
        orig_geniestop_fn(instance,_pkt_reader);
    }
}
// #[inline(never)]
// fn hk_genie_send_update(_pkt_reader: *mut c_void) {
//     dbg_info!("GenieController.SendGenieUpdate called");
//     let g_settings = SETTINGS.get().unwrap().lock().unwrap();
//     if *g_settings.get(&settings::Setting::InfiniteGenie).unwrap(){
//         dbg_info!("InfiniteGenie is enabled, doing nothing");
        
//     }else{
//         dbg_info!("InfiniteGenie is disabled, call original");
//         let orig_geniestop = ORIG_GENIESTOP.get().unwrap();
//         let orig_geniestop = unsafe
//         {
//             transmute::<usize, extern "system" fn(*mut c_void)>(*orig_geniestop)
//         };
//         orig_geniestop(_pkt_reader);
//     }
// }
#[inline(never)]
fn hk_robot(instance:*mut c_void,_pkt_reader: *mut c_void) {
    dbg_info!("UIRobotController.ReceivedRobotPackets called");
    let g_settings = SETTINGS.get().unwrap().lock().unwrap();
    if *g_settings.get(&settings::Setting::ImnotrobotBypass).unwrap(){
        dbg_info!("ImnotrobotBypass is enabled, doing nothing");
    }else{
        dbg_info!("ImnotrobotBypass is disabled, call original");
        let orig_robot = ORIG_ROBOT.get().unwrap();
        let orig_robot = unsafe
        {
            transmute::<usize, extern "system" fn(*mut c_void, *mut c_void)>(*orig_robot)
        };
        orig_robot(instance,_pkt_reader);
    }
}

#[inline(never)]
fn hk_robotcontroller_receivedrobot(instance: *mut c_void ,_pkt_reader: *mut c_void) {
    dbg_info!("UIRobotController.ReceivedRobotPackets called");
    let g_settings = SETTINGS.get().unwrap().lock().unwrap();
    if *g_settings.get(&settings::Setting::ImnotrobotBypass).unwrap(){
        dbg_info!("ImnotrobotBypass is enabled, doing nothing");
    }else{
        dbg_info!("ImnotrobotBypass is disabled, call original");
        let orig_robot = ORIG_ROBOTCONTROLLER_RECEIVEDROBOTPACKETS.get().unwrap();
        let orig_robot = unsafe
        {
            transmute::<usize, extern "system" fn(*mut c_void, *mut c_void)>(*orig_robot)
        };
        orig_robot(instance,_pkt_reader);
    }
}

#[inline(never)]
fn hk_robotcontroller_show_robot(instance :*mut c_void) {
    dbg_info!("UIRobotController.ShowRobot called");
    let g_settings = SETTINGS.get().unwrap().lock().unwrap();
    if *g_settings.get(&settings::Setting::ImnotrobotBypass).unwrap(){
        dbg_info!("ImnotrobotBypass is enabled, doing nothing");
    }else{
        dbg_info!("ImnotrobotBypass is disabled, call original");
        let orig_robot = ORIG_ROBOTCONTROLLER_SHOW_ROBOT.get().unwrap();
        let orig_robot = unsafe
        {
            transmute::<usize, extern "system" fn(*mut c_void)>(*orig_robot)
        };
        orig_robot(instance);
    }
}
#[inline(never)]
fn hk_robotcontroller_on_using_skill(instance :*mut c_void) {
    dbg_info!("UIRobotController.OnUsingSkill called");
    let g_settings = SETTINGS.get().unwrap().lock().unwrap();
    if *g_settings.get(&settings::Setting::ImnotrobotBypass).unwrap(){
        dbg_info!("ImnotrobotBypass is enabled, doing nothing");
    }else{
        dbg_info!("ImnotrobotBypass is disabled, call original");
        let orig_robot = ORIG_ROBOTCONTROLLER_ON_USING_SKILL.get().unwrap();
        let orig_robot = unsafe
        {
            transmute::<usize, extern "system" fn(*mut c_void)>(*orig_robot)
        };
        orig_robot(instance);
    }
}


fn hk_send_device_info(){
    dbg_info!("GameClientController.SendDeviceInfo called, do not call anything");
}

#[inline(never)]
fn hk_is_in_skill_anim(cls:*mut c_void) -> c_int {
    // dbg_info!("PlayerAnimationController.IsInSkillAnimation called");
    let g_settings = SETTINGS.get().unwrap().lock().unwrap();
    if *g_settings.get(&settings::Setting::MoveInAnimation).unwrap(){
        // dbg_info!("MoveInAnimation is enabled, returning false");
        return 0;
    }
    // dbg_info!("MoveInAnimation is disabled, call original");
    let orig_is_in_skill_anim = ORIG_IS_IN_SKILL_ANIM.get().unwrap();
    let orig_is_in_skill_anim = unsafe
    {
        transmute::<usize, extern "system" fn(*mut c_void) -> c_int>(*orig_is_in_skill_anim)
    };
    orig_is_in_skill_anim(cls)
}


#[inline(never)]
fn hk_is_in_casting(cls:*mut c_void) -> c_int {
    // dbg_info!("SkillController.IsInCasting called");
    let g_settings = SETTINGS.get().unwrap().lock().unwrap();
    if *g_settings.get(&settings::Setting::MoveInAnimation).unwrap(){
        // dbg_info!("MoveInAnimation is enabled, returning false");
        return 0;
    }
    // dbg_info!("MoveInAnimation is disabled, call original");
    let orig_is_in_casting = ORIG_IS_IN_CASTING.get().unwrap();
    let orig_is_in_casting = unsafe
    {
        transmute::<usize, extern "system" fn(*mut c_void) -> c_int>(*orig_is_in_casting)
    };
    orig_is_in_casting(cls)
}


#[inline(never)]
fn hk_cancel_if_casting(cls:*mut c_void,send_packet:c_int) {
    // dbg_info!("SkillController.CancelIfCasting called");
    let g_settings = SETTINGS.get().unwrap().lock().unwrap();
    if *g_settings.get(&settings::Setting::MoveInAnimation).unwrap(){
        // dbg_info!("MoveInAnimation is enabled, doing nothing");
        return;
    }
    // dbg_info!("MoveInAnimation is disabled, call original");
    let orig_cancel_if_casting = ORIG_CANCEL_IF_CASTING.get().unwrap();
    let orig_cancel_if_casting = unsafe
    {
        transmute::<usize, extern "system" fn(*mut c_void, c_int)>(*orig_cancel_if_casting)
    };
    orig_cancel_if_casting(cls,send_packet)
}

#[inline(never)]
fn hk_max_use_skill_count()->i32{
    dbg_info!("UIRobotController get_MaxUseSkillCount called! returning 1");
    100000  
    // let g_settings = SETTINGS.get().unwrap().lock().unwrap();
    // if *g_settings.get(&settings::Setting::ImnotrobotBypass).unwrap(){
    //     dbg_info!("ImnotrobotBypass is enabled, returning 1");
    //     return 1;
    // }
    // dbg_info!("ImnotrobotBypass is disabled, call original");
    // let orig_max_use_skill_count = ORIG.get().unwrap();
    // let orig_max_use_skill_count = unsafe
    // {
    //     transmute::<usize, extern "system" fn() -> i32>(*orig_max_use_skill_count)
    // };
    // orig_max_use_skill_count()

}

pub fn hook_game_functions() {
    il2cpp_stuff::hook_il2cpp_method(goldberg_string!("MainPlayer"), goldberg_string!("LupState"), hk_lupstate as usize, &ORIG_LUPSTATE);
    il2cpp_stuff::hook_il2cpp_method(
        goldberg_string!("UILoginController"),
        goldberg_string!("IsEmulator"),
        hk_isemulator as usize,
        &ORIG_ISEMULATOR
    );
    il2cpp_stuff::hook_il2cpp_method(
        goldberg_string!("GenieController"),
        goldberg_string!("ReceivedGenieStop"),
        hk_genie_stop as usize,
        &ORIG_GENIESTOP
    );

    // il2cpp_stuff::hook_il2cpp_method(
    //     goldberg_string!("GenieController"),
    //     goldberg_string!("SendGenieUpdate"),
    //     hk_genie_send_update as usize,
    //     &ORIG_GENIESTOP_XDSYS
    // );
    il2cpp_stuff::hook_il2cpp_method(
        goldberg_string!("GameClientController"),
        goldberg_string!("ReceivedRobotPackets"),
        hk_robot as usize,
        &ORIG_ROBOT
    );

    il2cpp_stuff::hook_il2cpp_method(
        goldberg_string!("UIRobotController"),
        goldberg_string!("ReceivedRobotPacket"),
        hk_robotcontroller_receivedrobot as usize,
        &ORIG_ROBOTCONTROLLER_RECEIVEDROBOTPACKETS
    );

    il2cpp_stuff::hook_il2cpp_method(
        goldberg_string!("UIRobotController"),
        goldberg_string!("ShowRobot"),
        hk_robotcontroller_show_robot as usize,
        &ORIG_ROBOTCONTROLLER_SHOW_ROBOT
    );
    il2cpp_stuff::hook_il2cpp_method(
        goldberg_string!("UIRobotController"),
        goldberg_string!("OnUsingSkill"),
        hk_robotcontroller_on_using_skill as usize,
        &ORIG_ROBOTCONTROLLER_ON_USING_SKILL
    );


    il2cpp_stuff::hook_il2cpp_method(
        goldberg_string!("GameController"),
        goldberg_string!("SendDeviceInfoPacket"),
        hk_send_device_info as usize,
        &ORIG_SENDDEVICEINFO
    );
    
    // il2cpp_stuff::hook_il2cpp_method(
    //     "UIRobotController",
    //     "get_MaxUseSkillCount",
    //     hk_max_use_skill_count as usize,

    // );

    il2cpp_stuff::hook_il2cpp_method(
        goldberg_string!("PlayerAnimationController"),
        goldberg_string!("IsInSkillAnimation"),
        hk_is_in_skill_anim as usize,
        &ORIG_IS_IN_SKILL_ANIM
    );
    il2cpp_stuff::hook_il2cpp_method(
        goldberg_string!("SkillController"),
        goldberg_string!("IsInCasting"),
        hk_is_in_casting as usize,
        &ORIG_IS_IN_CASTING,
        
    );
    il2cpp_stuff::hook_il2cpp_method(
        goldberg_string!("SkillController"),
        goldberg_string!("CancelIfCasting"),
        hk_cancel_if_casting as usize,
        &ORIG_CANCEL_IF_CASTING
        
    );
}
