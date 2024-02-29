use crate::knight_unity::ksend::LoginMainServerBuilder;
use crate::knight_unity::{ksend, KSend};
use crate::*;
use knight_unity;
use knight_unity::{KRecvInner, KSendInner};
use rand::Rng;
use std::iter;

use binrw::{BinRead, BinWrite};
use std::io::Cursor;

static ORIG_SEND: OnceLock<SendFnType> = OnceLock::new();
static ORIG_RECV: OnceLock<RecvFnType> = OnceLock::new();

#[inline(never)]
unsafe fn hk_send(fd: c_int, buf: *mut c_void, len: c_int, flags: c_int) -> c_int {
    if ORIG_SEND.get().is_some() {
        // dbg_info!("send called xD");
        // Convert buf to &[u8:len]
        let bufz = std::slice::from_raw_parts(buf as *const u8, len as usize);
        // dbg_info!("send data:\n{}", pretty_hex::pretty_hex(&bufz));

        if bufz[0] == 0xaa && bufz[1] == 0x55{
            let mut c = Cursor::new(bufz);
            let res = knight_unity::KSend::read_le(&mut c);
            // dbg_info!("data parsed");
            match res {
                Ok(ksend) => {
                    // dbg_info!("inner parsing");
                    let parsed = ksend.parse_inner();
                    match parsed {
                        Ok(e) => match e {
                            KSendInner::LoginMainServer(login_main_server) => {
                                dbg_info!("LoginMainServer");
                                // let user = LoginMainServer.username;
                                // let pw = LoginMainServer.password;
                                // dbg_info!("User: {}\nPassword: {}", user, pw);
                                dbg_info!("{:?}",login_main_server);


                                let gpu = login_main_server.graphic_mem_size;
                                let cpu = login_main_server.device_mem_size;
                                if cpu != gpu {
                                    const CHARSETX: &[u8] = b"abcdef0123456789";
                                    let mut rng = rand::thread_rng();
                                    let one_char = || CHARSETX[rng.gen_range(0..CHARSETX.len())] as char;
                                    let device_id: String = iter::repeat_with(one_char).take(16).collect();
                                    // Generate new packet with same gpu and cpu
                                    let new_login = LoginMainServerBuilder::default()
                                        .graphic_mem_size(cpu)
                                        .device_mem_size(cpu)
                                        .username(login_main_server.username)
                                        .password(login_main_server.password)
                                        .dev_id(device_id.into())
                                        .dev_model("SM-F750U".into())
                                        .dev_name("Samsung S19".into())
                                        .dev_gpu("Vulkan|_|- False -|Mali-P10".into())
                                        .device_id(login_main_server.device_id)
                                        .build()
                                        .unwrap();
                                
                                    
                                    let mut w = Cursor::new(Vec::new());
                                    new_login.write_le(&mut w).unwrap();
                                    let kreq = KSend::generate_encrypted_from_inner(w.into_inner());
                                    let mut c = Cursor::new(Vec::new());
                                    kreq.write_le(&mut c).unwrap();
                                    let len = c.get_ref().len();
                                    let encrypted_ksend = c.get_mut().to_vec();
                                    let orig_send = ORIG_SEND.get().unwrap();
                                    let encrypted_ksend = encrypted_ksend.as_ptr();
                                    dbg_info!("Sending new login packet");
                                    return orig_send(fd, encrypted_ksend as *mut c_void, len as i32, flags);


                                }
                                

                            }
                            KSendInner::DropItemList(_drop_item_list) => {
                                dbg_info!("DropItemList");
                                dbg_info!("send data:\n{}", pretty_hex::pretty_hex(&bufz));
                                dbg_info!("to fd : {} with len : {} flags : {}", fd, len, flags);
                            }
                            KSendInner::DeviceInfo(device_info) => {
                                dbg_info!("DeviceInfo");
                                dbg_info!("{:?}",device_info);
                            }
                            KSendInner::GeniePacket(genie_packet) => {
                                match genie_packet.type_id {
                                    0 => {
                                        // Genie Start
                                        dbg_info!("Genie Start");
                                        let g_settings = settings::SETTINGS.get().unwrap().lock().unwrap();
                                        if *g_settings.get(&settings::Setting::InfiniteGenie).unwrap(){
                                            std::thread::spawn( move || {
                                                // Random sleep between 20-100 seconds
                                                let sleep_time = rand::thread_rng().gen_range(20..100);
                                                std::thread::sleep(Duration::from_millis(sleep_time));
                                                let genie_stop = KSendInner::GeniePacket(ksend::GeniePacket {
                                                    type_id: 1,
                                                });
                                                let mut w = Cursor::new(Vec::new());
                                                genie_stop.write_le(&mut w).unwrap();
                                                let kreq = KSend::generate_encrypted_from_inner(w.into_inner());
                                                let mut c = Cursor::new(Vec::new());
                                                kreq.write_le(&mut c).unwrap();
                                                let len = c.get_ref().len();
                                                let encrypted_ksend = c.get_mut().to_vec();
                                                let orig_send = ORIG_SEND.get().unwrap();
                                                let encrypted_ksend = encrypted_ksend.as_ptr();
                                                dbg_info!("Sending genie stop after {} seconds", sleep_time);
                                                orig_send(fd, encrypted_ksend as *mut c_void, len as i32, flags);
                                                
                                            });
                                        }
                                    },
                                    1 => {
                                        let g_settings = settings::SETTINGS.get().unwrap().lock().unwrap();
                                        if *g_settings.get(&settings::Setting::InfiniteGenie).unwrap(){
                                            dbg_info!("Genie Stop. Do not send genie stop");
                                            // Before sending genie stop send genie start since we always send genie stop aftet start
                                            let genie_start = KSendInner::GeniePacket(ksend::GeniePacket {
                                                type_id: 0,
                                            });
                                            let mut w = Cursor::new(Vec::new());
                                            genie_start.write_le(&mut w).unwrap();
                                            let kreq = KSend::generate_encrypted_from_inner(w.into_inner());
                                            let mut c = Cursor::new(Vec::new());
                                            kreq.write_le(&mut c).unwrap();
                                            let len = c.get_ref().len();
                                            let encrypted_ksend = c.get_mut().to_vec();
                                            let orig_send = ORIG_SEND.get().unwrap();
                                            let encrypted_ksend = encrypted_ksend.as_ptr();
                                            dbg_info!("Sending genie start");
                                            orig_send(fd, encrypted_ksend as *mut c_void, len as i32, flags);
                                        }



                                    }
                                    2 => {
                                        // Genie update time
                                        dbg_info!("Genie update time");
                                    }
                                    _ => {

                                    }
                                }
                            }

                            _ => {}
                        },
                        Err(_e) => {
                            // error!("failed : {}", e);
                        }
                    }
                }
                Err(_e) => {
                    // error!("failed : {}", e);
                }
            }
        } else {
            dbg_info!("Not a ku packet!");
        }

        let orig_send = ORIG_SEND.get().unwrap();
        let ret = orig_send(fd, buf, len, flags);
        // dbg_info!("send returned {}", ret);
        return ret;
    } else {
        dbg_info!("orig_send is none");
    }

    -1
}
#[inline(never)]
unsafe fn hk_recvfrom(
    fd: c_int,
    buf: *mut c_void,
    len: c_int,
    flags: c_int,
    addr: *mut c_void,
    addrlen: *mut c_void,
) -> c_int {
    if ORIG_RECV.get().is_some() {
        // dbg_info!("recvfrom called xD");
        let orig_recv = ORIG_RECV.get().unwrap();
        let ret = orig_recv(fd, buf, len, flags, addr, addrlen);

        let bufz = std::slice::from_raw_parts(buf as *const u8, ret as usize);
        // dbg_info!("data received:\n{}", pretty_hex::pretty_hex(&bufz));
        if bufz[0] == 0xaa {
            let mut c = Cursor::new(bufz);
            let mut last_location = 0;
            loop {
                last_location = c.position();
                let res = knight_unity::KRecv::read_le(&mut c);
                // dbg_info!("data parsed current cursor location : {}", c.position());

                match res {
                    Ok(mut krecv) => {
                        let inner = krecv.parse_inner();
                        match inner {
                            Ok(inner) => {
                                let g_settings = settings::SETTINGS.get().unwrap();
                                let g_settings = g_settings.lock().unwrap();
                                if !*g_settings.get(&settings::Setting::AutoLoot).unwrap() {
                                    break;
                                }

                                if let KRecvInner::DropLoot(dl) = inner {
                                    let loot_id = dl.unit_name.to_string().clone();

                                    let orig_send = ORIG_SEND.get().unwrap();
                                    let buf_open_chest =
                                        KSendInner::DropItemList(ksend::DropItemList {
                                            drop_key: loot_id.into(),
                                            receive_type: true.into(),
                                        });

                                    let mut w = Cursor::new(Vec::new());
                                    buf_open_chest.write_le(&mut w).unwrap();
                                    let kreq = KSend::generate_encrypted_from_inner(w.into_inner());

                                    let mut c = Cursor::new(Vec::new());
                                    kreq.write_le(&mut c).unwrap();

                                    let len = c.get_ref().len();
                                    let encrypted_ksend = c.get_mut().to_vec();

                                    let loot_id = dl.unit_name.to_string().clone();
                                    let buf_recv_item =
                                        KSendInner::DropReceiveItem(ksend::DropReceiveItem {
                                            drop_key: loot_id.into(),
                                            receive_type: 100u32,
                                        });
                                    let mut w = Cursor::new(Vec::new());
                                    buf_recv_item.write_le(&mut w).unwrap();
                                    let kreq = KSend::generate_encrypted_from_inner(w.into_inner());
                                    let mut c_recv = Cursor::new(Vec::new());
                                    kreq.write_le(&mut c_recv).unwrap();

                                    let len_recv = c_recv.get_ref().len();
                                    let encrypted_ksend_recv = c_recv.get_mut().to_vec();

                                    std::thread::spawn(move || {
                                        std::thread::sleep(Duration::from_millis(100));
                                        let encrypted_ksend = encrypted_ksend.as_ptr();
                                        orig_send(
                                            fd,
                                            encrypted_ksend as *mut c_void,
                                            len as i32,
                                            flags,
                                        );

                                        std::thread::sleep(Duration::from_millis(100));
                                        let encrypted_ksend_recv = encrypted_ksend_recv.as_ptr();
                                        orig_send(
                                            fd,
                                            encrypted_ksend_recv as *mut c_void,
                                            len_recv as i32,
                                            flags,
                                        );
                                    });
                                    // orig_send(fd, *encrypted_ksend as *mut c_void, len as i32, flags);
                                }
                            }
                            Err(_e) => {
                                // dbg_error!("failed : {}",e)
                            }
                        }
                    }
                    Err(_e) => {
                        // dbg_error!("failed : {}",e)
                    }
                }
                if c.position() == c.get_ref().len() as u64 {
                    break;
                }
                if c.position() == last_location {
                    // dbg_error!("stuck at {} breaking away", c.position());
                    break;
                }
            }
        } else {
            dbg_info!("Not a ku packet!");
        }

        return ret;
    } else {
        dbg_info!("orig_recv is none");
    }
    -1
}

pub fn hook_il2cpp_send_recv() {
    let lib = unsafe { libloading::Library::new(goldberg_string!("libil2cpp.so")).expect("load payload") };

    let il2cpp_init = unsafe {
        let func: libloading::Symbol<unsafe extern "C" fn() -> c_int> =
            lib.get(goldberg_string!("il2cpp_init").as_bytes()).expect("get pid function");
        func.into_raw().into_raw() as usize
    };

    let maps = unsafe { get_process_maps(getpid()).expect("maps") };
    // Find base module of libil2cpp.so
    let base_addr = maps
        .iter()
        .find(|m| m.filename().is_some() && m.filename().unwrap().ends_with(goldberg_string!("libil2cpp.so")))
        .expect("hm");
    let base_addr = base_addr.start();

    let map_name = maps
        .iter()
        .flat_map(|m| m.filename())
        .find(|name| name.ends_with(goldberg_string!("libil2cpp.so")))
        .expect("find payload");

    let map_name = format!("{}\0", map_name.to_str().expect("to str"));
    dbg_info!("Target lib {map_name}");
    dbg_info!("base_addr {:#x}", base_addr);
    dbg_info!("il2cpp_init {:#x}", il2cpp_init);

    let link_map = plt_rs::LinkMapView::from_address(il2cpp_init);
    if link_map.is_none() {
        error!("link map is none");
        return;
    }
    let mut link_map = link_map.unwrap();
    // let mut link_map = plt_rs::LinkMapView::from_shared_library(&map_name).expect("open link map");

    let mut mutable_link_map: MutableLinkMap = MutableLinkMap::from_view(link_map);
    let mut _orig_send = mutable_link_map
        .hook::<fn(c_int, *mut c_void, c_int, c_int) -> c_int>(goldberg_string!("send"), hk_send as *const _);

    if let Err(e) = _orig_send {
        error!("Error: {:?}", e);
        return;
    }
    let _orig_send = _orig_send.unwrap().unwrap();
    dbg_info!("Send hooked at {:p}", *_orig_send.cached_function());
    link_map = plt_rs::LinkMapView::from_address(il2cpp_init).expect("open link map");
    mutable_link_map = MutableLinkMap::from_view(link_map);
    let _orig_recvfrom =
        mutable_link_map
            .hook::<fn(c_int, *mut c_void, c_int, c_int, *mut c_void, *mut c_void) -> c_int>(
                goldberg_string!("recvfrom"),
                hk_recvfrom as *const _,
            );

    if let Err(e) = _orig_recvfrom {
        error!("Error: {:?}", e);
        return;
    }
    let _orig_recvfrom = _orig_recvfrom.unwrap().unwrap();
    dbg_info!("Recvfrom hooked at {:p}", *_orig_recvfrom.cached_function());

    ORIG_SEND.get_or_init(|| *_orig_send.cached_function());
    ORIG_RECV.get_or_init(|| *_orig_recvfrom.cached_function());
}
