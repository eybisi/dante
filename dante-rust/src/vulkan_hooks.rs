use std::ffi::{c_char, c_int, c_uint, c_void, CStr};
use std::mem::transmute;
use crate::utils::logger::dbg_info;
// use crate::imgui_menu;
use crate::my_plt_hook;
use dobby_rs::hook;
use libc::getpid;
use log::info;
use proc_maps::get_process_maps;
use std::sync::OnceLock;
use goldberg::goldberg_string;

static ORIG_VK_QUEUE_SUBMIT: OnceLock<usize> = OnceLock::new();
static ORIG_VK_CREATEDEVICE: OnceLock<usize> = OnceLock::new();
static ORIG_VK_CREATEINSTANCE: OnceLock<usize> = OnceLock::new();
pub static ORIG_VK_CREATEDESCRIPTORPOOL: OnceLock<usize> = OnceLock::new();
static ORIG_VK_GETDEVICEQUEUE: OnceLock<usize> = OnceLock::new();
pub static ORIG_VK_GETSWAPCHAINIMAGESKHR: OnceLock<usize> = OnceLock::new();
static ORIG_VK_GETINSTANCEPROCADDR: OnceLock<usize> = OnceLock::new();
static ORIG_EGL_SWAP_BUFFERS: OnceLock<usize> = OnceLock::new();
static ORIG_VK_GET_DEVICE_PROC_ADDR: OnceLock<usize> = OnceLock::new();
static ORIG_VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES: OnceLock<usize> = OnceLock::new();
static ORIG_VK_CREATESWAPCHAINKHR: OnceLock<usize> = OnceLock::new();
static ORIG_VK_CREATERENDERPASS: OnceLock<usize> = OnceLock::new();

#[inline(never)]
fn hk_vk_get_device_proc_addr(device: *mut c_void, p_name: *const c_char) -> *mut c_void {
    // dbg_info!("vkGetDeviceProcAddr called");
    let px_name = unsafe { CStr::from_ptr(p_name) }.to_str().unwrap();
    // dbg_info!("vkGetDeviceProcAddr called with symbol : {}", px_name);
    let orig_vk_get_device_proc_addr = ORIG_VK_GET_DEVICE_PROC_ADDR.get().unwrap();
    let orig_vk_get_device_proc_addr_fn = unsafe {
        transmute::<usize, extern "system" fn(*const c_void, *const c_char) -> *mut c_void>(
            *orig_vk_get_device_proc_addr,
        )
    };
    let ret = orig_vk_get_device_proc_addr_fn(device, p_name);
    // dbg_info!("vkGetDeviceProcAddr returned : {:x}", ret as usize);
    if ret != 0 as *mut c_void {
        // ret will be the real offset of target function. We dont noeed to go inner function, just hook it with Dobby
        // If p_name is function that we want, we can hook it here
        // dbg_info!("vkGetDeviceProcAddr returned : {:x}", ret as usize);
        if px_name == "vkCreateDevice" {
            let orig_create_device =
                unsafe { hook(ret as *mut c_void, hk_vk_create_device as *mut c_void) }.unwrap();
            ORIG_VK_CREATEDEVICE.get_or_init(|| orig_create_device as usize);
            dbg_info!("Hooking vkCreateDevice");
        } else if px_name == goldberg_string!("vkCreateDescriptorPool") {
            let orig_create_descriptor_pool = unsafe {
                hook(
                    ret as *mut c_void,
                    hk_vk_create_descriptor_pool as *mut c_void,
                )
            }
            .unwrap();
            ORIG_VK_CREATEDESCRIPTORPOOL.get_or_init(|| orig_create_descriptor_pool as usize);
            dbg_info!("Hooking vkCreateDescriptorPool");
        } else if px_name == goldberg_string!("vkGetDeviceQueue") {
            let orig_get_device_queue =
                unsafe { hook(ret as *mut c_void, hk_vk_get_device_queue as *mut c_void) }.unwrap();
            ORIG_VK_GETDEVICEQUEUE.get_or_init(|| orig_get_device_queue as usize);
            dbg_info!("Hooking vkGetDeviceQueue");
        } else if px_name == goldberg_string!("vkQueueSubmit") {
            let orig_queue_submit =
                unsafe { hook(ret as *mut c_void, hk_vk_queue_submit as *mut c_void) }.unwrap();
            ORIG_VK_QUEUE_SUBMIT.get_or_init(|| orig_queue_submit as usize);
            dbg_info!("Hooking vkQueueSubmit");
        } else if px_name == goldberg_string!("vkCreateRenderPass") {
            // dbg_info!(
            //     "Hooking vkCreateRenderPass with retval : {:x}",
            //     ret as usize
            // );
            // let orig_create_render_pass =
            //     unsafe { hook(ret as *mut c_void, hk_vk_create_render_pass as *mut c_void) }
            //         .unwrap();
            // ORIG_VK_CREATERENDERPASS.get_or_init(|| orig_create_render_pass as usize);
            // dbg_info!("Hooking vkCreateRenderPass");
        }

        // else if px_name == "vkCreateRenderPass2KHR" {
        //     let orig_create_render_pass = unsafe {
        //         hook(
        //             ret as *mut c_void,
        //             hk_vk_create_render_pass as *mut c_void,
        //         )
        //     }
        //     .unwrap();
        //     ORIG_VK_CREATERENDERPASS.get_or_init(|| orig_create_render_pass as usize);
        //     dbg_info!("Hooking vkCreateRenderPass2KHR");
        // }
    }
    ret
}

#[inline(never)]
fn hk_vk_get_instance_proc_addr(instance: *mut c_void, p_name: *const c_char) -> *mut c_void {
    let px_name = unsafe { CStr::from_ptr(p_name) }.to_str().unwrap();
    // dbg_info!("vkGetInstanceProcAddr called with symbol : {}", px_name);
    let orig_vk_get_instance_proc_addr = ORIG_VK_GETINSTANCEPROCADDR.get().unwrap();
    let orig_vk_get_instance_proc_addr_fn = unsafe {
        transmute::<usize, extern "system" fn(*const c_void, *const c_char) -> *mut c_void>(
            *orig_vk_get_instance_proc_addr,
        )
    };
    let ret = orig_vk_get_instance_proc_addr_fn(instance, p_name);
    // dbg_info!("vkGetInstanceProcAddr returned : {:x}", ret as usize);
    if ret != 0 as *mut c_void {
        // ret will be the real offset of target function. We dont noeed to go inner function, just hook it with Dobby
        // If p_name is function that we want, we can hook it here
        // dbg_info!("vkGetInstanceProcAddr returned : {:x}", ret as usize);
        if px_name == goldberg_string!("vkGetSwapchainImagesKHR") {
            let orig_get_swapchain_images_khr = unsafe {
                hook(
                    ret as *mut c_void,
                    hk_vk_get_swapchain_images_khr as *mut c_void,
                )
            }
            .unwrap();
            ORIG_VK_GETSWAPCHAINIMAGESKHR.get_or_init(|| orig_get_swapchain_images_khr as usize);
            dbg_info!("Hooking vkGetSwapchainImagesKHR");
        } else if px_name == goldberg_string!("vkCreateSharedSwapchainsKHR") {
            let orig_create_swapchain_khr = unsafe {
                hook(
                    ret as *mut c_void,
                    hk_vk_create_swapchain_khr as *mut c_void,
                )
            }
            .unwrap();
            ORIG_VK_CREATESWAPCHAINKHR.get_or_init(|| orig_create_swapchain_khr as usize);
            dbg_info!("Hooking vkCreateSharedSwapchainsKHR");
        } else if px_name == goldberg_string!("vkCreateSwapchainKHR") {
            let orig_create_swapchain_khr = unsafe {
                hook(
                    ret as *mut c_void,
                    hk_vk_create_swapchain_khr as *mut c_void,
                )
            }
            .unwrap();
            ORIG_VK_CREATESWAPCHAINKHR.get_or_init(|| orig_create_swapchain_khr as usize);
            dbg_info!("Hooking vkCreateSwapchainKHR");
        } else if px_name == goldberg_string!("vkCreateDevice") {
            let orig_create_device =
                unsafe { hook(ret as *mut c_void, hk_vk_create_device as *mut c_void) }.unwrap();
            ORIG_VK_CREATEDEVICE.get_or_init(|| orig_create_device as usize);
            dbg_info!("Hooking vkCreateDevice");
        } else if px_name == goldberg_string!("vkCreateInstance") {
            // check if ORIG_VK_CREATEINSTANCE is already set if so return
            if ORIG_VK_CREATEINSTANCE.get().is_some() {
                return ret;
            }
            let orig_create_instance =
                unsafe { hook(ret as *mut c_void, hk_vk_create_instance as *mut c_void) }.unwrap();
            ORIG_VK_CREATEINSTANCE.get_or_init(|| orig_create_instance as usize);
            dbg_info!("Hooking vkCreateInstance");
        } else if px_name == goldberg_string!("vkGetPhysicalDeviceQueueFamilyProperties") {
            let orig_get_physical_device_queue_family_properties = unsafe {
                hook(
                    ret as *mut c_void,
                    hk_vk_get_physical_device_queue_family_properties as *mut c_void,
                )
            }
            .unwrap();
            ORIG_VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES
                .get_or_init(|| orig_get_physical_device_queue_family_properties as usize);
            dbg_info!("Hooking vkGetPhysicalDeviceQueueFamilyProperties");
        }
    }
    // dbg_info!("vkGetInstanceProcAddr returned");
    ret
}
pub fn hook_vk_instanceproc_and_deviceproc() {
    dbg_info!("Hooking vkGetInstanceProcAddr and vkGetDeviceProcAddr");

    let lib = unsafe { libloading::Library::new(goldberg_string!("libvulkan.so")).expect("load payload") };

    let orig_vk_get_instance_proc_addr = unsafe {
        let func: libloading::Symbol<
            unsafe extern "C" fn(*mut c_void, *mut c_char) -> *mut c_void,
        > = lib.get(goldberg_string!("vkGetInstanceProcAddr").as_bytes()).expect("get pid function");
        func.into_raw().into_raw() as usize
    };

    let orig_vk_get_device_proc_addr = unsafe {
        let func: libloading::Symbol<
            unsafe extern "C" fn(*mut c_void, *mut c_char) -> *mut c_void,
        > = lib.get(goldberg_string!("vkGetDeviceProcAddr").as_bytes()).expect("get pid function");
        func.into_raw().into_raw() as usize
    };

    let orig_vk_get_instance_proc_addr = my_plt_hook::get_branch_addr_from_inst_addr(
        orig_vk_get_instance_proc_addr as *mut *const c_void,
    );
    let orig_get_instance_proc_addr = unsafe {
        hook(
            orig_vk_get_instance_proc_addr as *mut c_void,
            hk_vk_get_instance_proc_addr as *mut c_void,
        )
    }
    .unwrap();

    let orig_vk_get_device_proc_addr = my_plt_hook::get_branch_addr_from_inst_addr(
        orig_vk_get_device_proc_addr as *mut *const c_void,
    );
    let orig_get_device_proc_addr = unsafe {
        hook(
            orig_vk_get_device_proc_addr as *mut c_void,
            hk_vk_get_device_proc_addr as *mut c_void,
        )
    }
    .unwrap();

    ORIG_VK_GETINSTANCEPROCADDR.get_or_init(|| orig_get_instance_proc_addr as usize);
    ORIG_VK_GET_DEVICE_PROC_ADDR.get_or_init(|| orig_get_device_proc_addr as usize);
    dbg_info!("HOOKED VK_GETINSTANCEPROCADDR VIA DOBBY");
}

pub fn hook_vulkan_via_inner_fnc() {
    // vkCreateDevice(g_PhysicalDevice, &create_info, g_Allocator, &g_Device);  4th to get g_Device
    // vkCreateInstance(&create_info, g_Allocator, &g_Instance);  3rd to get g_Instance
    // vkEnumeratePhysicalDevices; return value g_PhysicalDevice
    // vkCreateDescriptorPool(g_Device, &pool_info, g_Allocator, &g_DescriptorPool); to get g_DescriptorPool
    // vkGetDeviceQueue(g_Device, g_QueueFamily, 0, &g_Queue); to get g_Queue
    // vkGetSwapchainImagesKHR(device, wd->Swapchain, &wd->ImageCount, backbuffers); get g_ImageCount

    dbg_info!("hook_vulkan_via_plt");
    let lib = unsafe { libloading::Library::new(goldberg_string!("libvulkan.so")).expect("load payload") };

    let orig_vk_create_device = unsafe {
        let func: libloading::Symbol<
            unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) -> c_int,
        > = lib.get(goldberg_string!("vkCreateDevice").as_bytes()).expect("get pid function");
        func.into_raw().into_raw() as usize
    };
    let orig_vk_create_instance = unsafe {
        let func: libloading::Symbol<
            unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int,
        > = lib.get(goldberg_string!("vkCreateInstance").as_bytes()).expect("get pid function");
        func.into_raw().into_raw() as usize
    };
    let orig_vk_create_descriptor_pool = unsafe {
        let func: libloading::Symbol<
            unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) -> c_int,
        > = lib
            .get(goldberg_string!("vkCreateDescriptorPool").as_bytes())
            .expect("get pid function");
        func.into_raw().into_raw() as usize
    };
    let orig_vk_get_device_queue = unsafe {
        let func: libloading::Symbol<
            unsafe extern "C" fn(*mut c_void, *mut c_void, c_uint, c_uint, *mut c_void) -> c_int,
        > = lib.get(goldberg_string!("vkGetDeviceQueue").as_bytes()).expect("get pid function");
        func.into_raw().into_raw() as usize
    };
    let orig_vk_get_swapchain_images_khr = unsafe {
        let func: libloading::Symbol<
            unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) -> c_int,
        > = lib
            .get(goldberg_string!("vkGetSwapchainImagesKHR").as_bytes())
            .expect("get pid function");
        func.into_raw().into_raw() as usize
    };

    let orig_queue_submit = unsafe {
        let func: libloading::Symbol<
            unsafe extern "C" fn(
                *mut c_void,
                c_uint,
                *mut *const c_void,
                *mut *const c_void,
                *mut *const c_void,
                *mut *const c_void,
                c_uint,
                *mut *const c_void,
            ) -> c_int,
        > = lib.get(goldberg_string!("vkQueueSubmit").as_bytes()).expect("get pid function");
        func.into_raw().into_raw() as usize
    };

    let orig_vk_get_device_proc_addr = unsafe {
        let func: libloading::Symbol<
            unsafe extern "C" fn(
                *mut c_void,
                c_uint,
                *mut *const c_void,
                *mut *const c_void,
                *mut *const c_void,
                *mut *const c_void,
                c_uint,
                *mut *const c_void,
            ) -> c_int,
        > = lib.get(goldberg_string!("vkGetDeviceProcAddr").as_bytes()).expect("get pid function");
        func.into_raw().into_raw() as usize
    };

    let orig_vk_create_device =
        my_plt_hook::get_branch_addr_from_inst_addr(orig_vk_create_device as *mut *const c_void);
    let orig_create_device = unsafe {
        hook(
            orig_vk_create_device as *mut c_void,
            hk_vk_create_device as *mut c_void,
        )
    }
    .unwrap();

    let orig_vk_create_instance =
        my_plt_hook::get_branch_addr_from_inst_addr(orig_vk_create_instance as *mut *const c_void);
    let orig_create_instance = unsafe {
        hook(
            orig_vk_create_instance as *mut c_void,
            hk_vk_create_instance as *mut c_void,
        )
    }
    .unwrap();

    let orig_vk_get_device_proc_addr = my_plt_hook::get_branch_addr_from_inst_addr(
        orig_vk_get_device_proc_addr as *mut *const c_void,
    );
    let orig_get_device_proc_addr = unsafe {
        hook(
            orig_vk_get_device_proc_addr as *mut c_void,
            hk_vk_get_device_proc_addr as *mut c_void,
        )
    }
    .unwrap();

    // let orig_vk_create_descriptor_pool = my_plt_hook::get_branch_addr_from_inst_addr(orig_vk_create_descriptor_pool as *mut *const c_void);
    let orig_create_descriptor_pool = unsafe {
        hook(
            orig_vk_create_descriptor_pool as *mut c_void,
            hk_vk_create_descriptor_pool as *mut c_void,
        )
    }
    .unwrap();

    // let orig_vk_get_device_queue = my_plt_hook::get_branch_addr_from_inst_addr(orig_vk_get_device_queue as *mut *const c_void);
    let orig_get_device_queue = unsafe {
        hook(
            orig_vk_get_device_queue as *mut c_void,
            hk_vk_get_device_queue as *mut c_void,
        )
    }
    .unwrap();

    // let orig_vk_get_swapchain_images_khr = my_plt_hook::get_branch_addr_from_inst_addr(orig_vk_get_swapchain_images_khr as *mut *const c_void);
    let orig_get_swapchain_images_khr = unsafe {
        hook(
            orig_vk_get_swapchain_images_khr as *mut c_void,
            hk_vk_get_swapchain_images_khr as *mut c_void,
        )
    }
    .unwrap();

    let orig_queue_submit = unsafe {
        hook(
            orig_queue_submit as *mut c_void,
            hk_vk_queue_submit as *mut c_void,
        )
    }
    .unwrap();

    ORIG_VK_CREATEDEVICE.get_or_init(|| orig_create_device as usize);
    ORIG_VK_CREATEINSTANCE.get_or_init(|| orig_create_instance as usize);
    ORIG_VK_CREATEDESCRIPTORPOOL.get_or_init(|| orig_create_descriptor_pool as usize);
    ORIG_VK_GETDEVICEQUEUE.get_or_init(|| orig_get_device_queue as usize);
    ORIG_VK_GETSWAPCHAINIMAGESKHR.get_or_init(|| orig_get_swapchain_images_khr as usize);
    ORIG_VK_QUEUE_SUBMIT.get_or_init(|| orig_queue_submit as usize);
    ORIG_VK_GET_DEVICE_PROC_ADDR.get_or_init(|| orig_get_device_proc_addr as usize);

    dbg_info!("HOOKED VK_CREATEDEVICE VIA DOBBY");
}

#[inline(never)]
fn hk_test_hook() {
    dbg_info!("called caller of create descriptor pool");
}

#[inline(never)]
fn hk_vk_create_render_pass(
    g_device: *mut c_void,
    create_info: *mut c_void,
    g_allocator: *mut c_void,
    g_render_pass: *mut c_void,
) -> i32 {
    dbg_info!(
        "vkCreateRenderPass called with device : {:x}",
        g_device as usize
    );
    let orig_vk_create_render_pass = ORIG_VK_CREATERENDERPASS.get().unwrap();
    let orig_vk_create_render_pass = unsafe {
        transmute::<
            usize,
            extern "system" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) -> c_int,
        >(*orig_vk_create_render_pass)
    };
    let res = orig_vk_create_render_pass(g_device, create_info, g_allocator, g_render_pass);
    // imgui_menu::G_RENDERPASS.get_or_init(|| g_render_pass as u64);
    res
}

pub fn hook_unity_vkcreatedescpool() {
    //7C27B0
    let maps = unsafe { get_process_maps(getpid()).expect("maps") };
    // Find base module of libil2cpp.so
    let base_addr = maps
        .iter()
        .find(|m| m.filename().is_some() && m.filename().unwrap().ends_with(goldberg_string!("libunity.so")))
        .expect("hm");
    let base_addr = base_addr.start();

    let vk_create_descriptor_pool_caller = base_addr + 0x7C27B0;
    let _orig_caller = unsafe {
        hook(
            vk_create_descriptor_pool_caller as *mut c_void,
            hk_test_hook as *mut c_void,
        )
    }
    .unwrap();
    dbg_info!("hooked vkCreateDescriptorPool caller");
}

#[inline(never)]
fn hk_egl_swap_buffers(
    _display: *mut c_void,
    _surface: c_uint,
    _c: *mut c_void,
    _d: *mut c_void,
) -> c_int {
    dbg_info!("eglSwapBuffers called! xD");
    // imgui_menu::setup_imgui();
    // imgui_menu::draw
    let orig_egl_swap = ORIG_EGL_SWAP_BUFFERS.get().unwrap();

    // mutate
    let orig = unsafe {
        transmute::<usize, extern "system" fn(*mut c_void, c_uint, *mut c_void, *mut c_void) -> c_int>(
            *orig_egl_swap,
        )
    };
    orig(_display, _surface, _c, _d)
}

#[inline(never)]
fn hk_vk_queue_submit(q: *mut c_void, s: c_uint, vk: *mut c_void, vkf: *mut c_void) -> i32 {
    dbg_info!("vkQueueSubmit called");
    // imgui_menu::setup_imgui_vulkan();
    // imgui_menu::draw_imgui_vulkan();
    // imgui_menu::draw_imgui_opengl3();
    // imgui_menu::setup_imgui_opengl3();
    // transmute
    let orig_queue_submit = ORIG_VK_QUEUE_SUBMIT.get().unwrap();
    let orig_queue_submit = unsafe {
        transmute::<usize, extern "system" fn(*mut c_void, c_uint, *mut c_void, *mut c_void) -> c_int>(
            *orig_queue_submit,
        )
    };
    orig_queue_submit(q, s, vk, vkf)
}

#[inline(never)]
fn hk_vk_create_device(
    g_physical_device: *mut c_void,
    create_info: *mut c_void,
    g_allocator: *mut c_void,
    g_device: *mut *mut c_void,
) -> i32 {
    let orig_vk_create_device = ORIG_VK_CREATEDEVICE.get().unwrap();
    let orig_vk_create_device = unsafe {
        transmute::<
            usize,
            extern "system" fn(*mut c_void, *mut c_void, *mut c_void, *mut *mut c_void) -> c_int,
        >(*orig_vk_create_device)
    };
    let res = orig_vk_create_device(g_physical_device, create_info, g_allocator, g_device);
    dbg_info!("vkCreateDevice called");
    dbg_info!("Dereferencing g_Device!");
    // g_device is pointer to device handle. we need to deref to get real handle
    let p_device = unsafe { *g_device as usize };
    dbg_info!("Device: {:x}", p_device as usize);
    // imgui_menu::G_DEVICE.get_or_init(|| p_device as u64);
    res
}

//vkCreateInstance(&create_info, g_Allocator, &g_Instance);  3rd to get g_Instance
#[inline(never)]
fn hk_vk_create_instance(
    create_info: *mut c_void,
    g_allocator: *mut c_void,
    g_instance: *mut *mut c_void,
) -> i32 {
    dbg_info!("vkCreateInstance called");

    let orig_vk_create_instance = ORIG_VK_CREATEINSTANCE.get().unwrap();
    let orig_vk_create_instance = unsafe {
        transmute::<usize, extern "system" fn(*mut c_void, *mut c_void, *mut *mut c_void) -> c_int>(
            *orig_vk_create_instance,
        )
    };
    let _res = orig_vk_create_instance(create_info, g_allocator, g_instance);
    let p_instance = unsafe { *g_instance as usize };

    dbg_info!("g_Instance: {:x}", p_instance as usize);
    dbg_info!("g_Allocator: {:x}", g_allocator as usize);
    // imgui_menu::G_INSTANCE.get_or_init(|| p_instance as u64);
    // imgui_menu::G_ALLOCATOR.get_or_init(|| g_allocator as u64);
    dbg_info!("Exit vkCreateInstance");
    #[cfg(feature = "vulkan")]
    return res;
    #[cfg(not(feature = "vulkan"))]
    return 1;
}

//vkCreateDescriptorPool(g_Device, &pool_info, g_Allocator, &g_DescriptorPool); to get g_DescriptorPool
#[inline(never)]
fn hk_vk_create_descriptor_pool(
    g_device: *mut c_void,
    pool_info: *mut c_void,
    g_allocator: *mut c_void,
    g_descriptor_pool: *mut *mut c_void,
) -> i32 {
    dbg_info!("vkCreateDescriptorPool called");

    let orig_vk_create_descriptor_pool = ORIG_VK_CREATEDESCRIPTORPOOL.get().unwrap();
    let orig_vk_create_descriptor_pool = unsafe {
        transmute::<
            usize,
            extern "system" fn(*mut c_void, *mut c_void, *mut c_void, *mut *mut c_void) -> c_int,
        >(*orig_vk_create_descriptor_pool)
    };
    let res = orig_vk_create_descriptor_pool(g_device, pool_info, g_allocator, g_descriptor_pool);

    let p_descriptor_pool = unsafe { *g_descriptor_pool as usize };
    dbg_info!("g_DescriptorPool: {:x}", p_descriptor_pool as usize);
    // imgui_menu::G_DESCRIPTOR_POOL.get_or_init(|| p_descriptor_pool as u64);
    res
}

//vkGetDeviceQueue(g_Device, g_QueueFamily, 0, &g_Queue); to get g_Queue
#[inline(never)]
fn hk_vk_get_device_queue(
    g_device: *mut c_void,
    g_queue_family: c_uint,
    _c: c_uint,
    g_queue: *mut *mut c_void,
) {
    let orig_vk_get_device_queue = ORIG_VK_GETDEVICEQUEUE.get().unwrap();
    let orig_vk_get_device_queue = unsafe {
        transmute::<usize, extern "system" fn(*mut c_void, c_uint, c_uint, *mut *mut c_void) -> c_int>(
            *orig_vk_get_device_queue,
        )
    };
    orig_vk_get_device_queue(g_device, g_queue_family, _c, g_queue);
    dbg_info!("vkGetDeviceQueue called");
    let p_queue = unsafe { *g_queue as usize };
    // imgui_menu::G_QUEUE.get_or_init(|| p_queue as u64);
    dbg_info!("g_Queue: {:x}", p_queue as usize);
}

// vkGetSwapchainImagesKHR(device, wd->Swapchain, &wd->ImageCount, backbuffers); get g_ImageCount
#[inline(never)]
fn hk_vk_get_swapchain_images_khr(
    device: *mut c_void,
    swapchain: *mut c_void,
    image_count: *mut c_uint,
    backbuffers: *mut c_void,
) {
    let orig_vk_get_swapchain_images_khr = ORIG_VK_GETSWAPCHAINIMAGESKHR.get().unwrap();
    let orig_vk_get_swapchain_images_khr = unsafe {
        transmute::<
            usize,
            extern "system" fn(*mut c_void, *mut c_void, *mut c_uint, *mut c_void) -> c_int,
        >(*orig_vk_get_swapchain_images_khr)
    };
    orig_vk_get_swapchain_images_khr(device, swapchain, image_count, backbuffers);
    dbg_info!("vkGetSwapchainImagesKHR called");
    // unsafe { imgui_menu::G_IMAGE_COUNT.get_or_init(|| *image_count as u32) };
    dbg_info!("g_ImageCount: {:x}", image_count as usize);
}
// Provided by VK_KHR_swapchain
// VkResult vkCreateSwapchainKHR(
//     VkDevice                                    device,
//     const VkSwapchainCreateInfoKHR*             pCreateInfo,
//     const VkAllocationCallbacks*                pAllocator,
//     VkSwapchainKHR*                             pSwapchain);
#[inline(never)]
fn hk_vk_create_swapchain_khr(
    device: *mut c_void,
    p_create_info: *mut c_void,
    p_allocator: *mut c_void,
    p_swapchain: *mut *mut c_void,
) -> i32 {
    let orig_vk_create_swapchain_khr = ORIG_VK_CREATESWAPCHAINKHR.get().unwrap();
    let orig_vk_create_swapchain_khr = unsafe {
        transmute::<
            usize,
            extern "system" fn(*mut c_void, *mut c_void, *mut c_void, *mut *mut c_void) -> c_int,
        >(*orig_vk_create_swapchain_khr)
    };
    let res = orig_vk_create_swapchain_khr(device, p_create_info, p_allocator, p_swapchain);
    let p_swapchain = unsafe { *p_swapchain as usize };
    dbg_info!("vkCreateSwapchainKHR called");
    // imgui_menu::G_SWAPCHAIN.get_or_init(|| p_swapchain as u64);
    dbg_info!("g_Swapchain: {:x}", p_swapchain as usize);
    res
}

#[inline(never)]
fn hk_vk_get_physical_device_queue_family_properties(
    physical_device: *mut c_void,
    queue_family_count: *mut c_uint,
    queue_families: *mut c_void,
) {
    dbg_info!(
        "vkGetPhysicalDeviceQueueFamilyProperties called with device : {:x}",
        physical_device as usize
    );
    // imgui_menu::G_PHYSICAL_DEVICE.get_or_init(|| physical_device as u64);
    let orig_vk_get_physical_device_queue_family_properties =
        ORIG_VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES
            .get()
            .unwrap();
    let orig_vk_get_physical_device_queue_family_properties = unsafe {
        transmute::<usize, extern "system" fn(*mut c_void, *mut c_uint, *mut c_void) -> c_int>(
            *orig_vk_get_physical_device_queue_family_properties,
        )
    };
    orig_vk_get_physical_device_queue_family_properties(
        physical_device,
        queue_family_count,
        queue_families,
    );
}

// Instead of hooking symbols directly, hook vkGetInstanceProcAddr and vkGetDeviceProcAddr functions to get real offset of the function
// applications use vkGetInstanceProcAddr to get
// vkGetInstanceProcAddr vkCreateInstance retval: 0x7b72d88790
// vkGetInstanceProcAddr vkDestroyInstance retval: 0x7b72d89370
// vkGetInstanceProcAddr vkGetPhysicalDeviceProperties retval: 0x7ae1211d7c
// vkGetInstanceProcAddr vkEnumeratePhysicalDevices retval: 0x7b72d90dcc
// vkGetInstanceProcAddr vkGetDeviceProcAddr retval: 0x7b72d8de28
// vkGetInstanceProcAddr vkGetPhysicalDeviceFeatures retval: 0x7ae1212af0
// vkGetInstanceProcAddr vkGetPhysicalDeviceFormatProperties retval: 0x7ae1212464
// vkGetInstanceProcAddr vkGetPhysicalDeviceImageFormatProperties retval: 0x7ae12124dc
// vkGetInstanceProcAddr vkGetPhysicalDeviceQueueFamilyProperties retval: 0x7ae1212a4c
// vkGetInstanceProcAddr vkGetPhysicalDeviceMemoryProperties retval: 0x7ae1212364
// vkGetInstanceProcAddr vkCreateDevice retval: 0x7b72d89458
// vkGetInstanceProcAddr vkEnumerateDeviceExtensionProperties retval: 0x7b72d89e00
// vkGetInstanceProcAddr vkEnumerateDeviceLayerProperties retval: 0x7b72d89d5c
// vkGetInstanceProcAddr vkGetPhysicalDeviceSparseImageFormatProperties retval: 0x7ae1212de8
// vkGetInstanceProcAddr vkDestroySurfaceKHR retval: 0x7b72d95f58
// vkGetInstanceProcAddr vkGetPhysicalDeviceSurfaceSupportKHR retval: 0x7b72d95fd4
// vkGetInstanceProcAddr vkGetPhysicalDeviceSurfaceCapabilitiesKHR retval: 0x7b72d960c0
// vkGetInstanceProcAddr vkGetPhysicalDeviceSurfaceFormatsKHR retval: 0x7b72d9627c
// vkGetInstanceProcAddr vkGetPhysicalDeviceSurfacePresentModesKHR retval: 0x7b72d965a4
// vkGetInstanceProcAddr vkCreateSwapchainKHR retval: 0x7b72d8ef2c
// vkGetInstanceProcAddr vkDestroySwapchainKHR retval: 0x7b72d8ef44
// vkGetInstanceProcAddr vkGetSwapchainImagesKHR retval: 0x7b72d8ef5c
// vkGetInstanceProcAddr vkAcquireNextImageKHR retval: 0x7b72d8ef74
// vkGetInstanceProcAddr vkQueuePresentKHR retval: 0x7b72d8ef8c
// vkGetInstanceProcAddr vkGetPhysicalDeviceDisplayPropertiesKHR retval: 0x0
// vkGetInstanceProcAddr vkGetPhysicalDeviceDisplayPlanePropertiesKHR retval: 0x0
// vkGetInstanceProcAddr vkGetDisplayPlaneSupportedDisplaysKHR retval: 0x0
// vkGetInstanceProcAddr vkGetDisplayModePropertiesKHR retval: 0x0
// vkGetInstanceProcAddr vkCreateDisplayModeKHR retval: 0x0
// vkGetInstanceProcAddr vkGetDisplayPlaneCapabilitiesKHR retval: 0x0
// vkGetInstanceProcAddr vkCreateDisplayPlaneSurfaceKHR retval: 0x0
// vkGetInstanceProcAddr vkCreateSharedSwapchainsKHR retval: 0x0
// vkGetInstanceProcAddr vkCreateAndroidSurfaceKHR retval: 0x7b72d95dc8
// vkGetInstanceProcAddr vkSetHdrMetadataEXT retval: 0x7b72d938a8
// vkGetInstanceProcAddr vkGetPhysicalDeviceFeatures2KHR retval: 0x7ae1212bf8
// vkGetInstanceProcAddr vkGetPhysicalDeviceProperties2KHR retval: 0x7ae121235c
// vkGetInstanceProcAddr vkGetPhysicalDeviceFormatProperties2KHR retval: 0x7ae121249c
// vkGetInstanceProcAddr vkGetPhysicalDeviceImageFormatProperties2KHR retval: 0x7ae1212760
// vkGetInstanceProcAddr vkGetPhysicalDeviceQueueFamilyProperties2KHR retval: 0x7ae1212a98
// vkGetInstanceProcAddr vkGetPhysicalDeviceMemoryProperties2KHR retval: 0x7ae12123e4
// vkGetInstanceProcAddr vkGetPhysicalDeviceSparseImageFormatProperties2KHR retval: 0x7ae120f2c0

// Then calls vkCreateDevice to get device and use vkGetDeviceProcAddr with device object to get other functions
//
// vkCreateDevice called
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Thread ID: 14197
// vkGetDeviceProcAddr called with pName: vkDestroyDevice returned : 0x7b72d89afc
// vkGetDeviceProcAddr called with pName: vkGetDeviceQueue returned : 0x7b72d911a4
// vkGetDeviceProcAddr called with pName: vkQueueSubmit returned : 0x7ae1216c8c
// vkGetDeviceProcAddr called with pName: vkQueueWaitIdle returned : 0x7ae12176b8
// vkGetDeviceProcAddr called with pName: vkDeviceWaitIdle returned : 0x7ae120caa4
// vkGetDeviceProcAddr called with pName: vkAllocateMemory returned : 0x7ae120cce0
// vkGetDeviceProcAddr called with pName: vkFreeMemory returned : 0x7ae120d0b0
// vkGetDeviceProcAddr called with pName: vkMapMemory returned : 0x7ae120d354
// vkGetDeviceProcAddr called with pName: vkUnmapMemory returned : 0x7ae120d3c0
// vkGetDeviceProcAddr called with pName: vkFlushMappedMemoryRanges returned : 0x7ae120d128
// vkGetDeviceProcAddr called with pName: vkInvalidateMappedMemoryRanges returned : 0x7ae120d334
// vkGetDeviceProcAddr called with pName: vkGetDeviceMemoryCommitment returned : 0x7ae120d3c8
// vkGetDeviceProcAddr called with pName: vkBindBufferMemory returned : 0x7ae11ffee0
// vkGetDeviceProcAddr called with pName: vkBindImageMemory returned : 0x7ae120f098
// vkGetDeviceProcAddr called with pName: vkGetBufferMemoryRequirements returned : 0x7ae11ffddc
// vkGetDeviceProcAddr called with pName: vkGetImageMemoryRequirements returned : 0x7ae120efd8
// vkGetDeviceProcAddr called with pName: vkGetImageSparseMemoryRequirements returned : 0x7ae120f2c0
// vkGetDeviceProcAddr called with pName: vkQueueBindSparse returned : 0x7ae112e178
// vkGetDeviceProcAddr called with pName: vkCreateFence returned : 0x7ae120dadc
// vkGetDeviceProcAddr called with pName: vkDestroyFence returned : 0x7ae120dc10
// vkGetDeviceProcAddr called with pName: vkResetFences returned : 0x7ae120dc9c
// vkGetDeviceProcAddr called with pName: vkGetFenceStatus returned : 0x7ae120dc80
// vkGetDeviceProcAddr called with pName: vkWaitForFences returned : 0x7ae120dcf8
// vkGetDeviceProcAddr called with pName: vkCreateSemaphore returned : 0x7ae121b794
// vkGetDeviceProcAddr called with pName: vkDestroySemaphore returned : 0x7ae121b8b0
// vkGetDeviceProcAddr called with pName: vkCreateEvent returned : 0x7ae120d844
// vkGetDeviceProcAddr called with pName: vkDestroyEvent returned : 0x7ae120d974
// vkGetDeviceProcAddr called with pName: vkGetEventStatus returned : 0x7ae120d9e4
// vkGetDeviceProcAddr called with pName: vkSetEvent returned : 0x7ae120da04
// vkGetDeviceProcAddr called with pName: vkResetEvent returned : 0x7ae120da70
// vkGetDeviceProcAddr called with pName: vkCreateQueryPool returned : 0x7ae1216a14
// vkGetDeviceProcAddr called with pName: vkDestroyQueryPool returned : 0x7ae1216ba8
// vkGetDeviceProcAddr called with pName: vkGetQueryPoolResults returned : 0x7ae1216bf0
// vkGetDeviceProcAddr called with pName: vkCreateBuffer returned : 0x7ae11ffbec
// vkGetDeviceProcAddr called with pName: vkDestroyBuffer returned : 0x7ae11ffd74
// vkGetDeviceProcAddr called with pName: vkCreateBufferView returned : 0x7ae11fffa0
// vkGetDeviceProcAddr called with pName: vkDestroyBufferView returned : 0x7ae1200144
// vkGetDeviceProcAddr called with pName: vkCreateImage returned : 0x7ae120e4dc
// vkGetDeviceProcAddr called with pName: vkDestroyImage returned : 0x7ae120ef8c
// vkGetDeviceProcAddr called with pName: vkGetImageSubresourceLayout returned : 0x7ae120f210
// vkGetDeviceProcAddr called with pName: vkCreateImageView returned : 0x7ae1210d18
// vkGetDeviceProcAddr called with pName: vkDestroyImageView returned : 0x7ae12110e4
// vkGetDeviceProcAddr called with pName: vkCreateShaderModule returned : 0x7ae121bb3c
// vkGetDeviceProcAddr called with pName: vkDestroyShaderModule returned : 0x7ae121bc78
// vkGetDeviceProcAddr called with pName: vkCreatePipelineCache returned : 0x7ae121577c
// vkGetDeviceProcAddr called with pName: vkDestroyPipelineCache returned : 0x7ae12159d0
// vkGetDeviceProcAddr called with pName: vkGetPipelineCacheData returned : 0x7ae1215a40
// vkGetDeviceProcAddr called with pName: vkMergePipelineCaches returned : 0x7ae1215cac
// vkGetDeviceProcAddr called with pName: vkCreateGraphicsPipelines returned : 0x7ae1213028
// vkGetDeviceProcAddr called with pName: vkCreateComputePipelines returned : 0x7ae1214b60
// vkGetDeviceProcAddr called with pName: vkDestroyPipeline returned : 0x7ae120ef8c
// vkGetDeviceProcAddr called with pName: vkCreatePipelineLayout returned : 0x7ae12161e4
// vkGetDeviceProcAddr called with pName: vkDestroyPipelineLayout returned : 0x7ae12169a0
// vkGetDeviceProcAddr called with pName: vkCreateSampler returned : 0x7ae121b19c
// vkGetDeviceProcAddr called with pName: vkDestroySampler returned : 0x7ae11ffd74
// vkGetDeviceProcAddr called with pName: vkCreateDescriptorSetLayout returned : 0x7ae120b650
// vkGetDeviceProcAddr called with pName: vkDestroyDescriptorSetLayout returned : 0x7ae120bd78
// vkGetDeviceProcAddr called with pName: vkCreateDescriptorPool returned : 0x7ae1209a0c
// vkGetDeviceProcAddr called with pName: vkDestroyDescriptorPool returned : 0x7ae1209ddc
// vkGetDeviceProcAddr called with pName: vkResetDescriptorPool returned : 0x7ae1209f14
// vkGetDeviceProcAddr called with pName: vkAllocateDescriptorSets returned : 0x7ae120a130
// vkGetDeviceProcAddr called with pName: vkFreeDescriptorSets returned : 0x7ae120a224
// vkGetDeviceProcAddr called with pName: vkUpdateDescriptorSets returned : 0x7ae120a268
// vkGetDeviceProcAddr called with pName: vkCreateFramebuffer returned : 0x7ae120e19c
// vkGetDeviceProcAddr called with pName: vkDestroyFramebuffer returned : 0x7ae120e2ec
// vkGetDeviceProcAddr called with pName: vkCreateRenderPass returned : 0x7ae1217bf8
// vkGetDeviceProcAddr called with pName: vkDestroyRenderPass returned : 0x7ae121a8c0
// vkGetDeviceProcAddr called with pName: vkGetRenderAreaGranularity returned : 0x7ae121a994
// vkGetDeviceProcAddr called with pName: vkCreateCommandPool returned : 0x7ae120933c
// vkGetDeviceProcAddr called with pName: vkDestroyCommandPool returned : 0x7ae1209598
// vkGetDeviceProcAddr called with pName: vkResetCommandPool returned : 0x7ae12095c4
// vkGetDeviceProcAddr called with pName: vkAllocateCommandBuffers returned : 0x7b72d91280
// vkGetDeviceProcAddr called with pName: vkFreeCommandBuffers returned : 0x7ae1200ba4
// vkGetDeviceProcAddr called with pName: vkBeginCommandBuffer returned : 0x7ae1200d04
// vkGetDeviceProcAddr called with pName: vkEndCommandBuffer returned : 0x7ae1200ff4
// vkGetDeviceProcAddr called with pName: vkResetCommandBuffer returned : 0x7ae1200be4
// vkGetDeviceProcAddr called with pName: vkCmdBindPipeline returned : 0x7ae1201ecc
// vkGetDeviceProcAddr called with pName: vkCmdSetViewport returned : 0x7ae12057cc
// vkGetDeviceProcAddr called with pName: vkCmdSetScissor returned : 0x7ae12056b4
// vkGetDeviceProcAddr called with pName: vkCmdSetLineWidth returned : 0x7ae11352c0
// vkGetDeviceProcAddr called with pName: vkCmdSetDepthBias returned : 0x7ae12051d0
// vkGetDeviceProcAddr called with pName: vkCmdSetBlendConstants returned : 0x7ae1205930
// vkGetDeviceProcAddr called with pName: vkCmdSetDepthBounds returned : 0x7ae11352c0
// vkGetDeviceProcAddr called with pName: vkCmdSetStencilCompareMask returned : 0x7ae1205300
// vkGetDeviceProcAddr called with pName: vkCmdSetStencilWriteMask returned : 0x7ae1205578
// vkGetDeviceProcAddr called with pName: vkCmdSetStencilReference returned : 0x7ae120543c
// vkGetDeviceProcAddr called with pName: vkCmdBindDescriptorSets returned : 0x7ae1204390
// vkGetDeviceProcAddr called with pName: vkCmdBindIndexBuffer returned : 0x7ae1202bc0
// vkGetDeviceProcAddr called with pName: vkCmdBindVertexBuffers returned : 0x7ae1202d04
// vkGetDeviceProcAddr called with pName: vkCmdDraw returned : 0x7ae1202410
// vkGetDeviceProcAddr called with pName: vkCmdDrawIndexed returned : 0x7ae12025ec
// vkGetDeviceProcAddr called with pName: vkCmdDrawIndirect returned : 0x7ae12029d4
// vkGetDeviceProcAddr called with pName: vkCmdDrawIndexedIndirect returned : 0x7ae12027e4
// vkGetDeviceProcAddr called with pName: vkCmdDispatch returned : 0x7ae1202040
// vkGetDeviceProcAddr called with pName: vkCmdDispatchIndirect returned : 0x7ae1202244
// vkGetDeviceProcAddr called with pName: vkCmdCopyBuffer returned : 0x7ae1203140
// vkGetDeviceProcAddr called with pName: vkCmdCopyImage returned : 0x7ae12038fc
// vkGetDeviceProcAddr called with pName: vkCmdBlitImage returned : 0x7ae12060a4
// vkGetDeviceProcAddr called with pName: vkCmdCopyBufferToImage returned : 0x7ae12035d0
// vkGetDeviceProcAddr called with pName: vkCmdCopyImageToBuffer returned : 0x7ae1203764
// vkGetDeviceProcAddr called with pName: vkCmdUpdateBuffer returned : 0x7ae1203388
// vkGetDeviceProcAddr called with pName: vkCmdFillBuffer returned : 0x7ae1202f98
// vkGetDeviceProcAddr called with pName: vkCmdClearColorImage returned : 0x7ae1204020
// vkGetDeviceProcAddr called with pName: vkCmdClearDepthStencilImage returned : 0x7ae1204300
// vkGetDeviceProcAddr called with pName: vkCmdClearAttachments returned : 0x7ae1205c78
// vkGetDeviceProcAddr called with pName: vkCmdResolveImage returned : 0x7ae1203b60
// vkGetDeviceProcAddr called with pName: vkCmdSetEvent returned : 0x7ae1206518
// vkGetDeviceProcAddr called with pName: vkCmdResetEvent returned : 0x7ae1206700
// vkGetDeviceProcAddr called with pName: vkCmdWaitEvents returned : 0x7ae1206718
// vkGetDeviceProcAddr called with pName: vkCmdPipelineBarrier returned : 0x7ae12049dc
// vkGetDeviceProcAddr called with pName: vkCmdBeginQuery returned : 0x7ae120716c
// vkGetDeviceProcAddr called with pName: vkCmdEndQuery returned : 0x7ae12072a4
// vkGetDeviceProcAddr called with pName: vkCmdResetQueryPool returned : 0x7ae1206fe0
// vkGetDeviceProcAddr called with pName: vkCmdWriteTimestamp returned : 0x7ae1207424
// vkGetDeviceProcAddr called with pName: vkCmdCopyQueryPoolResults returned : 0x7ae1206d9c
// vkGetDeviceProcAddr called with pName: vkCmdPushConstants returned : 0x7ae1205a38
// vkGetDeviceProcAddr called with pName: vkCmdBeginRenderPass returned : 0x7ae1201428
// vkGetDeviceProcAddr called with pName: vkCmdNextSubpass returned : 0x7ae1201138
// vkGetDeviceProcAddr called with pName: vkCmdEndRenderPass returned : 0x7ae12019bc
// vkGetDeviceProcAddr called with pName: vkCmdExecuteCommands returned : 0x7ae1204694
// vkGetDeviceProcAddr called with pName: vkGetImageMemoryRequirements2KHR returned : 0x7ae120f018
// vkGetDeviceProcAddr called with pName: vkGetBufferMemoryRequirements2KHR returned : 0x7ae11ffe40
// vkGetDeviceProcAddr called with pName: vkGetImageSparseMemoryRequirements2KHR returned : 0x7ae120f2c8
// vkGetDeviceProcAddr called with pName: vkCreateDescriptorUpdateTemplateKHR returned : 0x7ae120c0f4
// vkGetDeviceProcAddr called with pName: vkDestroyDescriptorUpdateTemplateKHR returned : 0x7ae120c210
// vkGetDeviceProcAddr called with pName: vkUpdateDescriptorSetWithTemplateKHR returned : 0x7ae120c288
// vkGetDeviceProcAddr called with pName: vkCmdBeginRenderPass2KHR returned : 0x7ae12019b4f
// vkGetDeviceProcAddr called with pName: vkCmdEndRenderPass2KHR returned : 0x7ae1201ec4
// vkGetDeviceProcAddr called with pName: vkCmdNextSubpass2KHR returned : 0x7ae1201420
// vkGetDeviceProcAddr called with pName: vkCreateRenderPass2KHR returned : 0x7ae121a71c
// vkGetDeviceProcAddr called with pName: vkQueuePresentKHR returned : 0x7b72d97b2c
// vkGetDeviceProcAddr called with pName: vkGetRefreshCycleDurationGOOGLE returned : 0x7b72d98318
// vkGetDeviceProcAddr called with pName: vkGetPastPresentationTimingGOOGLE returned : 0x7b72d98328
// vkGetDeviceProcAddr called with pName: vkQueuePresentKHR returned : 0x7b72d97b2c
// vkGetDeviceProcAddr called with pName: vkGetRefreshCycleDurationGOOGLE returned : 0x7b72d98318
// vkGetDeviceProcAddr called with pName: vkGetPastPresentationTimingGOOGLE returned : 0x7b72d98328
