use imgui_sys::*;
use std::ptr;

use crate::imgui_android_backend;
// use crate::vulkan_hooks;
use log::{error, info};
use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void, CStr, CString};
use std::mem;
use std::sync::OnceLock;

// use crate::imgui_opengl3;
// pub static g_Allocator: OnceLock<VkAllocationCallbacks> = OnceLock::new();
// pub static g_Instance: OnceLock<u64> = OnceLock::new();
// pub static g_PhysicalDevice: OnceLock<VkPhysicalDevice> = OnceLock::new();
// pub static g_Device: OnceLock<VkDevice> = OnceLock::new();
// pub static g_QueueFamily: OnceLock<u32> = OnceLock::new();
// pub static g_Queue: OnceLock<VkQueue> = OnceLock::new();
// pub static g_DebugReport: OnceLock<VkDebugReportCallbackEXT> = OnceLock::new();
// pub static g_PipelineCache: OnceLock<VkPipelineCache> = OnceLock::new();
// pub static g_DescriptorPool: OnceLock<VkDescriptorPool> = OnceLock::new();
// pub static g_ImageCount: OnceLock<u32> = OnceLock::new();
// pub static IMGUI_INIT: OnceLock<bool> = OnceLock::new();

pub static G_ALLOCATOR: OnceLock<u64> = OnceLock::new();
pub static G_INSTANCE: OnceLock<u64> = OnceLock::new();
pub static G_PHYSICAL_DEVICE: OnceLock<u64> = OnceLock::new();
pub static G_DEVICE: OnceLock<u64> = OnceLock::new();
pub static G_QUEUE_FAMILY: OnceLock<u32> = OnceLock::new();
pub static G_QUEUE: OnceLock<u64> = OnceLock::new();
pub static G_DEBUG_REPORT: OnceLock<u64> = OnceLock::new();
pub static G_PIPELINE_CACHE: OnceLock<u64> = OnceLock::new();
pub static G_DESCRIPTOR_POOL: OnceLock<u64> = OnceLock::new();
pub static G_IMAGE_COUNT: OnceLock<u32> = OnceLock::new();
pub static G_SWAPCHAIN: OnceLock<u64> = OnceLock::new();
pub static G_RENDERPASS: OnceLock<u64> = OnceLock::new();

pub fn draw_menu() {
    let mut SHOW_DEMO_WINDOW: bool = true;
    static SHOW_ANOTHER_WINDOW: bool = false;
    static CLEAR_COLOR: [f32; 4] = [0.45, 0.55, 0.60, 1.00];
    if SHOW_DEMO_WINDOW {
        unsafe {
            // dbg_info!("Drawing ImGui demo window");
            imgui_sys::igShowDemoWindow(&mut SHOW_DEMO_WINDOW);
        }
    }
}

pub fn draw_imgui_opengl3() {
    let init = imgui_android_backend::IMGUI_INIT.get();
    if let Some(i) = init {
        if *i {
            // dbg_info!("ImGui initialized, lets draw it");
            let io = unsafe { igGetIO() };
            // dbg_info!("ImGui IO created");
            let io = unsafe { &mut *io };
            // dbg_info!("ImGui IO created");
            // dbg_info!("PRAY FOR RNG GODS!!!!!!!!!!!!!!!!");
            unsafe {
                imgui_sys::ImGui_ImplOpenGL3_NewFrame();
                // dbg_info!("ImGui new frame created");
                imgui_android_backend::imgui_impl_android_newframe(1920, 1080);
                // dbg_info!("ImGui android new frame created");
                igNewFrame();
                // dbg_info!("ImGui new frame created");
                draw_menu();
                // dbg_info!("ImGui menu drawn");
                igRender();
                // dbg_info!("ImGui rendered");
                imgui_sys::ImGui_ImplOpenGL3_RenderDrawData(imgui_sys::igGetDrawData());
                // dbg_info!("ImGui draw data rendered");
                igEndFrame();
                // dbg_info!("ImGui frame ended");
            }
            // dbg_info!("WE DIDD ITTTTTTTTTTTT!");
        } else {
            dbg_info!("ImGui not initialized");
            return;
        }
    } else {
        dbg_info!("ImGui not initialized");
        return;
    }
}

pub fn draw_imgui_vulkan() {
    // dbg_info!("Drawing ImGui");
    // let is_init = imgui_android_backend::IMGUI_INIT.get();
    // if let None = is_init {
    //     return;
    // }
    // let if_ = *is_init.unwrap();
    // if if_ {
    //     dbg_info!("ImGui already initialized");
    //     let io = unsafe { igGetIO() };
    //     dbg_info!("ImGui IO created");
    //     let io = unsafe { &mut *io };
    //     dbg_info!("ImGui IO created");
    //     unsafe {
    //         imgui_sys::ImGui_ImplVulkan_NewFrame();
    //     }

    //     // imgui_opengl3::imgui_impl_opengl3_new_frame();
    //     imgui_android_backend::imgui_impl_android_newframe(1920, 1080);
    //     unsafe {
    //         igNewFrame();
    //     }
    //     draw_menu();
    //     unsafe {
    //         igRender();
    //     }
    //     unsafe {

    //         // imgui_sys::ImGui_ImplOpenGL3_RenderDrawData(imgui_sys::igGetDrawData());
    //     }
    //     unsafe { igEndFrame() };
    // } else {
    //     dbg_info!("ImGui not initialized");
    // }
}

pub fn frame_render() {}

pub fn setup_imgui_opengl3() {
    let init = imgui_android_backend::IMGUI_INIT.get();
    if let Some(ii) = init {
        // dbg_info!("ImGui is initialized returning from setup");
        return;
    }
    dbg_info!("Initializing ImGui opengl3");
    let ctx = unsafe { imgui_sys::igCreateContext(0 as *mut imgui_sys::ImFontAtlas) };
    dbg_info!("ImGui context created");
    if ctx as usize == 0 {
        error!("Failed to initialize ImGui context");
        return;
    }

    // dbg_info!("ImGui context created");
    // unsafe{imgui_sys::igSetCurrentContext(ctx)};
    let io = unsafe { imgui_sys::igGetIO() };
    if io as usize == 0 {
        error!("Failed to initialize ImGui IO");
        return;
    }
    // Get inner io
    dbg_info!("ImGui IO created");
    let io = unsafe { &mut *io };
    io.IniFilename = ptr::null();
    io.KeyMap[ImGuiKey_ImGuiKey_Tab as usize] = 61;
    io.KeyMap[ImGuiKey_ImGuiKey_UpArrow as usize] = 19;
    io.KeyMap[ImGuiKey_ImGuiKey_DownArrow as usize] = 20;
    io.KeyMap[ImGuiKey_ImGuiKey_LeftArrow as usize] = 21;
    io.KeyMap[ImGuiKey_ImGuiKey_RightArrow as usize] = 22;
    io.KeyMap[ImGuiKey_ImGuiKey_Enter as usize] = 66;
    io.KeyMap[ImGuiKey_ImGuiKey_Backspace as usize] = 67;
    io.KeyMap[ImGuiKey_ImGuiKey_PageUp as usize] = 92;
    io.KeyMap[ImGuiKey_ImGuiKey_PageDown as usize] = 93;
    io.KeyMap[ImGuiKey_ImGuiKey_Escape as usize] = 111;
    io.KeyMap[ImGuiKey_ImGuiKey_Delete as usize] = 112;
    io.KeyMap[ImGuiKey_ImGuiKey_Home as usize] = 122;
    io.KeyMap[ImGuiKey_ImGuiKey_End as usize] = 123;
    io.KeyMap[ImGuiKey_ImGuiKey_Insert as usize] = 124;
    io.KeyMap[ImGuiKey_ImGuiKey_UpArrow as usize] = 19;
    io.KeyMap[ImGuiKey_ImGuiKey_DownArrow as usize] = 20;
    io.KeyMap[ImGuiKey_ImGuiKey_LeftArrow as usize] = 21;
    io.KeyMap[ImGuiKey_ImGuiKey_RightArrow as usize] = 22;
    io.KeyMap[ImGuiKey_ImGuiKey_Enter as usize] = 66;
    io.KeyMap[ImGuiKey_ImGuiKey_Backspace as usize] = 67;
    io.KeyMap[ImGuiKey_ImGuiKey_PageUp as usize] = 92;
    io.KeyMap[ImGuiKey_ImGuiKey_PageDown as usize] = 93;
    io.KeyMap[ImGuiKey_ImGuiKey_Escape as usize] = 111;
    io.KeyMap[ImGuiKey_ImGuiKey_Delete as usize] = 112;
    io.KeyMap[ImGuiKey_ImGuiKey_Home as usize] = 122;
    io.KeyMap[ImGuiKey_ImGuiKey_End as usize] = 123;
    io.KeyMap[ImGuiKey_ImGuiKey_Insert as usize] = 124;
    dbg_info!("ImGui IO keymap set");
    set_black_gold_theme();
    dbg_info!("ImGui theme set");

    let style = unsafe { igGetStyle() };
    if style.is_null() {
        error!("Failed to get ImGui style");
    }
    dbg_info!("ImGui style set");
    unsafe { ImGuiStyle_ScaleAllSizes(style, 3.0f32) };
    dbg_info!("ImGui style scaled");
    imgui_android_backend::imgui_impl_android_init();
    dbg_info!("ImGui android backend initialized");

    unsafe { imgui_sys::ImGui_ImplOpenGL3_Init(std::ptr::null()) };
    dbg_info!("Vulkan init info set");
    let ii = imgui_android_backend::IMGUI_INIT.get_or_init(|| true);
    dbg_info!("ImGui initialized : {}", *ii);
}

pub fn setup_imgui_vulkan() {
    unsafe {
        dbg_info!("Initializing ImGui");
        let ctx = imgui_sys::igCreateContext(0 as *mut imgui_sys::ImFontAtlas);

        if ctx as usize == 0 {
            error!("Failed to initialize ImGui context");
        }
        dbg_info!("ImGui context created");
        dbg_info!("Initializing ImGui");
        imgui_sys::igSetCurrentContext(ctx);
        let io = imgui_sys::igGetIO();
        if io as usize == 0 {
            error!("Failed to initialize ImGui IO");
        }
        // Get inner io
        dbg_info!("ImGui IO created");
        let io = &mut *io;
        io.IniFilename = ptr::null();
        io.KeyMap[ImGuiKey_ImGuiKey_Tab as usize] = 61;
        io.KeyMap[ImGuiKey_ImGuiKey_UpArrow as usize] = 19;
        io.KeyMap[ImGuiKey_ImGuiKey_DownArrow as usize] = 20;
        io.KeyMap[ImGuiKey_ImGuiKey_LeftArrow as usize] = 21;
        io.KeyMap[ImGuiKey_ImGuiKey_RightArrow as usize] = 22;
        io.KeyMap[ImGuiKey_ImGuiKey_Enter as usize] = 66;
        io.KeyMap[ImGuiKey_ImGuiKey_Backspace as usize] = 67;
        io.KeyMap[ImGuiKey_ImGuiKey_PageUp as usize] = 92;
        io.KeyMap[ImGuiKey_ImGuiKey_PageDown as usize] = 93;
        io.KeyMap[ImGuiKey_ImGuiKey_Escape as usize] = 111;
        io.KeyMap[ImGuiKey_ImGuiKey_Delete as usize] = 112;
        io.KeyMap[ImGuiKey_ImGuiKey_Home as usize] = 122;
        io.KeyMap[ImGuiKey_ImGuiKey_End as usize] = 123;
        io.KeyMap[ImGuiKey_ImGuiKey_Insert as usize] = 124;
        io.KeyMap[ImGuiKey_ImGuiKey_UpArrow as usize] = 19;
        io.KeyMap[ImGuiKey_ImGuiKey_DownArrow as usize] = 20;
        io.KeyMap[ImGuiKey_ImGuiKey_LeftArrow as usize] = 21;
        io.KeyMap[ImGuiKey_ImGuiKey_RightArrow as usize] = 22;
        io.KeyMap[ImGuiKey_ImGuiKey_Enter as usize] = 66;
        io.KeyMap[ImGuiKey_ImGuiKey_Backspace as usize] = 67;
        io.KeyMap[ImGuiKey_ImGuiKey_PageUp as usize] = 92;
        io.KeyMap[ImGuiKey_ImGuiKey_PageDown as usize] = 93;
        io.KeyMap[ImGuiKey_ImGuiKey_Escape as usize] = 111;
        io.KeyMap[ImGuiKey_ImGuiKey_Delete as usize] = 112;
        io.KeyMap[ImGuiKey_ImGuiKey_Home as usize] = 122;
        io.KeyMap[ImGuiKey_ImGuiKey_End as usize] = 123;
        io.KeyMap[ImGuiKey_ImGuiKey_Insert as usize] = 124;
        dbg_info!("ImGui IO keymap set");
        // set_black_gold_theme();
        dbg_info!("ImGui theme set");

        let style = igGetStyle();
        if style.is_null() {
            error!("Failed to get ImGui style");
        }
        dbg_info!("ImGui style set");
        ImGuiStyle_ScaleAllSizes(style, 3.0f32);
        dbg_info!("ImGui style scaled");
        imgui_android_backend::imgui_impl_android_init();
        dbg_info!("ImGui android backend initialized");

        //TODO Handle g_swapchainrebuild
        if !set_vulkan_init_info() {
            error!("Failed to set Vulkan init info");
            return;
        }
        dbg_info!("Vulkan init info set");
        let ii = imgui_android_backend::IMGUI_INIT.get_or_init(|| true);
        dbg_info!("ImGui initialized : {}", *ii);
    }
}

fn set_black_gold_theme() {
    unsafe {
        let style = igGetStyle();
        if style.is_null() {
            error!("Failed to get ImGui style");
        }
        let mut colors = (*style).Colors;
        colors[ImGuiCol__ImGuiCol_Text as usize] = ImVec4 {
            x: 0.92f32,
            y: 0.92f32,
            z: 0.92f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_TextDisabled as usize] = ImVec4 {
            x: 0.44f32,
            y: 0.44f32,
            z: 0.44f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_WindowBg as usize] = ImVec4 {
            x: 0.06f32,
            y: 0.06f32,
            z: 0.06f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_ChildBg as usize] = ImVec4 {
            x: 0.00f32,
            y: 0.00f32,
            z: 0.00f32,
            w: 0.00f32,
        };
        colors[ImGuiCol__ImGuiCol_PopupBg as usize] = ImVec4 {
            x: 0.08f32,
            y: 0.08f32,
            z: 0.08f32,
            w: 0.94f32,
        };
        colors[ImGuiCol__ImGuiCol_Border as usize] = ImVec4 {
            x: 0.51f32,
            y: 0.36f32,
            z: 0.15f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_BorderShadow as usize] = ImVec4 {
            x: 0.00f32,
            y: 0.00f32,
            z: 0.00f32,
            w: 0.00f32,
        };
        colors[ImGuiCol__ImGuiCol_FrameBg as usize] = ImVec4 {
            x: 0.11f32,
            y: 0.11f32,
            z: 0.11f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_FrameBgHovered as usize] = ImVec4 {
            x: 0.51f32,
            y: 0.36f32,
            z: 0.15f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_FrameBgActive as usize] = ImVec4 {
            x: 0.78f32,
            y: 0.55f32,
            z: 0.21f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_TitleBg as usize] = ImVec4 {
            x: 0.51f32,
            y: 0.36f32,
            z: 0.15f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_TitleBgActive as usize] = ImVec4 {
            x: 0.91f32,
            y: 0.64f32,
            z: 0.13f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_TitleBgCollapsed as usize] = ImVec4 {
            x: 0.00f32,
            y: 0.00f32,
            z: 0.00f32,
            w: 0.51f32,
        };
        colors[ImGuiCol__ImGuiCol_MenuBarBg as usize] = ImVec4 {
            x: 0.11f32,
            y: 0.11f32,
            z: 0.11f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_ScrollbarBg as usize] = ImVec4 {
            x: 0.06f32,
            y: 0.06f32,
            z: 0.06f32,
            w: 0.53f32,
        };
        colors[ImGuiCol__ImGuiCol_ScrollbarGrab as usize] = ImVec4 {
            x: 0.21f32,
            y: 0.21f32,
            z: 0.21f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_ScrollbarGrabHovered as usize] = ImVec4 {
            x: 0.47f32,
            y: 0.47f32,
            z: 0.47f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_ScrollbarGrabActive as usize] = ImVec4 {
            x: 0.81f32,
            y: 0.83f32,
            z: 0.81f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_CheckMark as usize] = ImVec4 {
            x: 0.78f32,
            y: 0.55f32,
            z: 0.21f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_SliderGrab as usize] = ImVec4 {
            x: 0.91f32,
            y: 0.64f32,
            z: 0.13f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_SliderGrabActive as usize] = ImVec4 {
            x: 0.91f32,
            y: 0.64f32,
            z: 0.13f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_Button as usize] = ImVec4 {
            x: 0.51f32,
            y: 0.36f32,
            z: 0.15f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_ButtonHovered as usize] = ImVec4 {
            x: 0.91f32,
            y: 0.64f32,
            z: 0.13f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_ButtonActive as usize] = ImVec4 {
            x: 0.78f32,
            y: 0.55f32,
            z: 0.21f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_Header as usize] = ImVec4 {
            x: 0.51f32,
            y: 0.36f32,
            z: 0.15f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_HeaderHovered as usize] = ImVec4 {
            x: 0.91f32,
            y: 0.64f32,
            z: 0.13f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_HeaderActive as usize] = ImVec4 {
            x: 0.93f32,
            y: 0.65f32,
            z: 0.14f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_Separator as usize] = ImVec4 {
            x: 0.21f32,
            y: 0.21f32,
            z: 0.21f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_SeparatorHovered as usize] = ImVec4 {
            x: 0.91f32,
            y: 0.64f32,
            z: 0.13f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_SeparatorActive as usize] = ImVec4 {
            x: 0.78f32,
            y: 0.55f32,
            z: 0.21f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_ResizeGrip as usize] = ImVec4 {
            x: 0.21f32,
            y: 0.21f32,
            z: 0.21f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_ResizeGripHovered as usize] = ImVec4 {
            x: 0.91f32,
            y: 0.64f32,
            z: 0.13f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_ResizeGripActive as usize] = ImVec4 {
            x: 0.78f32,
            y: 0.55f32,
            z: 0.21f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_Tab as usize] = ImVec4 {
            x: 0.51f32,
            y: 0.36f32,
            z: 0.15f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_TabHovered as usize] = ImVec4 {
            x: 0.91f32,
            y: 0.64f32,
            z: 0.13f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_TabActive as usize] = ImVec4 {
            x: 0.78f32,
            y: 0.55f32,
            z: 0.21f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_TabUnfocused as usize] = ImVec4 {
            x: 0.07f32,
            y: 0.10f32,
            z: 0.15f32,
            w: 0.97f32,
        };
        colors[ImGuiCol__ImGuiCol_TabUnfocusedActive as usize] = ImVec4 {
            x: 0.14f32,
            y: 0.26f32,
            z: 0.42f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_PlotLines as usize] = ImVec4 {
            x: 0.61f32,
            y: 0.61f32,
            z: 0.61f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_PlotLinesHovered as usize] = ImVec4 {
            x: 1.00f32,
            y: 0.43f32,
            z: 0.35f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_PlotHistogram as usize] = ImVec4 {
            x: 0.90f32,
            y: 0.70f32,
            z: 0.00f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_PlotHistogramHovered as usize] = ImVec4 {
            x: 1.00f32,
            y: 0.60f32,
            z: 0.00f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_TextSelectedBg as usize] = ImVec4 {
            x: 0.26f32,
            y: 0.59f32,
            z: 0.98f32,
            w: 0.35f32,
        };
        colors[ImGuiCol__ImGuiCol_DragDropTarget as usize] = ImVec4 {
            x: 1.00f32,
            y: 1.00f32,
            z: 0.00f32,
            w: 0.90f32,
        };
        colors[ImGuiCol__ImGuiCol_NavHighlight as usize] = ImVec4 {
            x: 0.26f32,
            y: 0.59f32,
            z: 0.98f32,
            w: 1.00f32,
        };
        colors[ImGuiCol__ImGuiCol_NavWindowingHighlight as usize] = ImVec4 {
            x: 1.00f32,
            y: 1.00f32,
            z: 1.00f32,
            w: 0.70f32,
        };
        colors[ImGuiCol__ImGuiCol_NavWindowingDimBg as usize] = ImVec4 {
            x: 0.80f32,
            y: 0.80f32,
            z: 0.80f32,
            w: 0.20f32,
        };
        colors[ImGuiCol__ImGuiCol_ModalWindowDimBg as usize] = ImVec4 {
            x: 0.80f32,
            y: 0.80f32,
            z: 0.80f32,
            w: 0.35f32,
        };

        // style->FramePadding = ImVec2(4, 2);
        // style->ItemSpacing = ImVec2(10, 2);
        // style->IndentSpacing = 12;
        // style->ScrollbarSize = 10;

        // style->WindowRounding = 4;
        // style->FrameRounding = 4;
        // style->PopupRounding = 4;
        // style->ScrollbarRounding = 6;
        // style->GrabRounding = 4;
        // style->TabRounding = 4;

        (*style).FramePadding = ImVec2 {
            x: 4.0f32,
            y: 2.0f32,
        };
        (*style).ItemSpacing = ImVec2 {
            x: 10.0f32,
            y: 2.0f32,
        };
        (*style).IndentSpacing = 12.0f32;
        (*style).ScrollbarSize = 10.0f32;

        (*style).WindowRounding = 4.0f32;
        (*style).FrameRounding = 4.0f32;
        (*style).PopupRounding = 4.0f32;
        (*style).ScrollbarRounding = 6.0f32;
        (*style).GrabRounding = 4.0f32;
        (*style).TabRounding = 4.0f32;

        (*style).WindowMenuButtonPosition = ImGuiDir__ImGuiDir_Right;
        (*style).DisplaySafeAreaPadding = ImVec2 {
            x: 4.0f32,
            y: 4.0f32,
        };
    }
}

fn set_vulkan_init_info() -> bool {
    true
    // let ix:ImVector_int = ImVector::new();

    // // Get current image count from G_SWAPCHAIN via vkGetSwapchainImagesKHR
    // dbg_info!("image_count: {}", 0);

    // // let orig_vk_getswapchain = vulkan_hooks::ORIG_VK_GETSWAPCHAINIMAGESKHR.get().unwrap();
    // // let vk_getswapchain: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) -> c_int = unsafe { std::mem::transmute(orig_vk_getswapchain) };
    // // let mut image_count = std::ptr::null_mut();
    // // unsafe {vk_getswapchain(*G_DEVICE.get().unwrap() as *mut c_void, *G_SWAPCHAIN.get().unwrap() as *mut c_void, image_count, ptr::null_mut())};
    // // dbg_info!("image_count: {:p}", image_count);
    // // G_IMAGE_COUNT.get_or_init(|| image_count as u32);

    // let g_instance = *G_INSTANCE.get().unwrap();
    // dbg_info!("g_instance: {:x}", g_instance);
    // let g_physical_device = *G_PHYSICAL_DEVICE.get().unwrap();
    // dbg_info!("g_physical_device: {:x}", g_physical_device);
    // let g_device = *G_DEVICE.get().unwrap();
    // dbg_info!("g_device: {:x}", g_device);
    // let g_queue = *G_QUEUE.get().unwrap();
    // dbg_info!("g_queue: {:x}", g_queue);
    // let g_descriptor_pool = G_DESCRIPTOR_POOL.get();
    // if let None = g_descriptor_pool {
    //     return false;
    // }
    // // let g_renderpass = G_RENDERPASS.get();
    // // if let None = g_renderpass {
    // // return false;
    // // }
    // let g_allocator = G_ALLOCATOR.get();
    // if let None = g_allocator {
    //     return false;
    // }
    // // let g_renderpass = *g_renderpass.unwrap();
    // // dbg_info!("g_renderpass: {:x}", g_renderpass);

    // let g_descriptor_pool = *g_descriptor_pool.unwrap();
    // dbg_info!("g_descriptor_pool: {:x}", g_descriptor_pool);

    // let g_allocator = *g_allocator.unwrap();
    // dbg_info!("g_allocator: {:x}", g_allocator);

    // dbg_info!("Setting Vulkan init info");
    // let mut init_info = ImGui_ImplVulkan_InitInfo {
    //     Instance: *G_INSTANCE.get().unwrap() as VkInstance,
    //     PhysicalDevice: *G_PHYSICAL_DEVICE.get().unwrap() as VkPhysicalDevice,
    //     Device: *G_DEVICE.get().unwrap() as VkDevice,
    //     QueueFamily: 31,
    //     Queue: *G_QUEUE.get().unwrap() as VkQueue,
    //     PipelineCache: std::ptr::null_mut(),
    //     DescriptorPool: *G_DESCRIPTOR_POOL.get().unwrap() as VkDescriptorPool,
    //     MinImageCount: 2,
    //     ImageCount: 3 as u32,
    //     Allocator: *G_ALLOCATOR.get().unwrap() as *const VkAllocationCallbacks,
    //     CheckVkResultFn: Option::None,
    //     Subpass: 0,
    //     MSAASamples: VkSampleCountFlagBits_VK_SAMPLE_COUNT_1_BIT,
    // };
    // dbg_info!("init_info created");

    // // let mut imgui_renderpass = create_renderpass_for_imgui();
    // // dbg_info!("imgui_renderpass created :");
    // // unsafe { ImGui_ImplVulkan_LoadFunctions()};
    // let res = unsafe { ImGui_ImplVulkan_Init(&mut init_info) };
    // dbg_info!("ImGui_ImplVulkan_Init called : {:?}", res);
    // true
}

// fn create_descriptor_pool() {
//     let pool_sizes = [vk::DescriptorPoolSize {
//         ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
//         descriptor_count: 1000,
//     }];
//     // descriptor_pool_info.

//     let pool_info = vk::DescriptorPoolCreateInfo {
//         s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
//         p_next: ptr::null(),
//         flags: vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET,
//         max_sets: 1,
//         pool_size_count: pool_sizes.len() as u32,
//         p_pool_sizes: pool_sizes.as_ptr(),
//     };

//     dbg_info!("pool_info created!!");

//     let orig_vk_create_descriptor_pool = vulkan_hooks::ORIG_VK_CREATEDESCRIPTORPOOL.get().unwrap();
//     let vk_create_descriptor_pool: unsafe extern "C" fn(
//         *mut c_void,
//         *const vk::DescriptorPoolCreateInfo,
//         *mut c_void,
//         *mut c_void,
//     ) -> c_int = unsafe { std::mem::transmute(*orig_vk_create_descriptor_pool) };
//     let mut descriptor_pool = unsafe { std::mem::zeroed() };
//     dbg_info!("calling vkCreateDescriptorPool");
//     unsafe {
//         vk_create_descriptor_pool(
//             *G_DEVICE.get().unwrap() as *mut c_void,
//             &pool_info,
//             0 as *mut c_void,
//             &mut descriptor_pool,
//         )
//     };
//     dbg_info!("descriptor_pool created");
//     // dbg_info!("descriptor_pool: {:x}", descriptor_pool as usize);
//     G_DESCRIPTOR_POOL.get_or_init(|| descriptor_pool as u64);
//     dbg_info!("descriptor_pool created");
// }

// #[inline(never)]
// extern "C" fn check_vk_result(result: VkResult) {
//     if result != VkResult_VK_SUCCESS {
//         error!("VkResult is not VK_SUCCESS!")
//     }
// }
