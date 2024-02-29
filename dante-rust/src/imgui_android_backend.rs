use imgui_sys::*;
use ndk::event::InputEvent;
use ndk::event::KeyAction;
use ndk::event::Keycode;
use std::sync::OnceLock;
pub static IMGUI_INIT: OnceLock<bool> = OnceLock::new();

static mut G_TIME: f32 = 0.0f32;
use log::{error, info};
pub fn imgui_impl_android_init() {
    // G_WINDOW.get_or_init(|| window.clone());
    // let time = unsafe { &mut G_TIME };
    // *time = time::OffsetDateTime::now_utc().unix_timestamp() as f32;

    let io = unsafe { igGetIO() };
    if io.is_null() {
        panic!("Failed to initialize ImGui IO");
    }
    // Get inner io
    let io = unsafe { &mut *io };
    io.BackendPlatformName = "imgui_impl_android\0".as_ptr();
    true;
}

pub fn imgui_impl_android_shutdown() {}

pub fn imgui_impl_android_newframe(width: i32, height: i32) {
    let io = unsafe { igGetIO() };
    if io.is_null() {
        panic!("Failed to initialize ImGui IO");
    }
    let io = unsafe { &mut *io };
    // let window = G_WINDOW.get().unwrap();
    let time = unsafe { &mut G_TIME };
    // let window_width = NativeWindow::width(window);
    // let window_height = NativeWindow::height(window);
    let display_width = width;
    let display_height = height;

    io.DisplaySize = ImVec2 {
        x: display_width as f32,
        y: display_height as f32,
    };
    if display_width > 0 && display_height > 0 {
        io.DisplayFramebufferScale = ImVec2 { x: 1.0, y: 1.0 };
    }
    // let current_time = time::OffsetDateTime::now_utc().unix_timestamp() as f32;
    let mut current_time = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut current_time) };
    let converted_time = current_time.tv_sec as f32 + current_time.tv_nsec as f32 / 1_000_000_000.0;

    if *time > 0.0 {
        io.DeltaTime = (converted_time - *time) as f32;
    } else {
        io.DeltaTime = 1.0 / 60.0;
    }
    // dbg_info!("delta time : {}", io.DeltaTime);
    // get current time
    unsafe {
        // dbg_info!("Current time: {}", converted_time);
        *time = converted_time;
    }
}

pub fn imgui_impl_android_handle_inputevent_x_y_event(x: i32, y: i32, event_type: i32) -> i32 {
    let io = unsafe { igGetIO() };
    if io.is_null() {
        panic!("Failed to initialize ImGui IO");
    }
    let io = unsafe { &mut *io };
    match event_type {
        0 => unsafe {
            ImGuiIO_AddMouseButtonEvent(io, 0, true);
        },
        1 => unsafe {
            ImGuiIO_AddMouseButtonEvent(io, 0, false);
        },
        2 => unsafe {},
        _ => {}
    }
    io.MousePos = ImVec2 {
        x: x as f32,
        y: y as f32,
    };
    0
}

pub fn imgui_impl_android_handle_inputevent(input_event: InputEvent) {
    let io = unsafe { igGetIO() };
    if io.is_null() {
        panic!("Failed to initialize ImGui IO");
    }
    let io = unsafe { &mut *io };
    match input_event {
        InputEvent::MotionEvent(e) => {
            let action = e.action();
            let meta_state = e.meta_state();
            let ptr_idx = e.pointer_index();
            match action {
                ndk::event::MotionAction::Down | ndk::event::MotionAction::Up => unsafe {
                    ImGuiIO_AddMousePosEvent(
                        io,
                        e.pointer_at_index(ptr_idx).x(),
                        e.pointer_at_index(ptr_idx).y(),
                    );
                    ImGuiIO_AddMouseButtonEvent(io, 0, action == ndk::event::MotionAction::Down);
                },
                ndk::event::MotionAction::HoverMove | ndk::event::MotionAction::Move => unsafe {
                    ImGuiIO_AddMousePosEvent(
                        io,
                        e.pointer_at_index(ptr_idx).x(),
                        e.pointer_at_index(ptr_idx).y(),
                    );
                },
                ndk::event::MotionAction::Scroll => unsafe {
                    ImGuiIO_AddMouseWheelEvent(
                        io,
                        e.pointer_at_index(ptr_idx)
                            .axis_value(ndk::event::Axis::Hscroll),
                        e.pointer_at_index(ptr_idx)
                            .axis_value(ndk::event::Axis::Vscroll),
                    )
                },
                ndk::event::MotionAction::ButtonPress | ndk::event::MotionAction::ButtonRelease => {
                    let button_state = e.button_state();
                    unsafe {
                        ImGuiIO_AddMouseButtonEvent(io, 0, button_state.primary());
                        ImGuiIO_AddMouseButtonEvent(io, 1, button_state.secondary());
                        ImGuiIO_AddMouseButtonEvent(io, 2, button_state.teriary());
                    }
                }
                _ => {}
            }
        }
        InputEvent::KeyEvent(e) => {
            dbg_info!("Key event: {:?}", e);
            let key_code = e.key_code();
            let scan_code = e.scan_code();
            let action = e.action();
            let meta_state = e.meta_state();

            unsafe {
                ImGuiIO_AddKeyEvent(io, ImGuiKey_ImGuiMod_Ctrl, meta_state.ctrl_on());
                ImGuiIO_AddKeyEvent(io, ImGuiKey_ImGuiMod_Shift, meta_state.shift_on());
                ImGuiIO_AddKeyEvent(io, ImGuiKey_ImGuiMod_Alt, meta_state.alt_on());
                ImGuiIO_AddKeyEvent(io, ImGuiKey_ImGuiMod_Super, meta_state.meta_on());
            }
            match action {
                KeyAction::Down | KeyAction::Up => {
                    let key_code = imgui_impl_android_keycode_to_imguikey(key_code);
                    if key_code != ImGuiKey_ImGuiKey_None {
                        unsafe {
                            ImGuiIO_AddKeyEvent(io, key_code as u32, action == KeyAction::Down);
                            ImGuiIO_SetKeyEventNativeData(
                                io,
                                key_code as u32,
                                key_code as i32,
                                scan_code,
                                0,
                            )
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

pub fn imgui_impl_android_keycode_to_imguikey(key_code: Keycode) -> ImGuiKey {
    match key_code {
        Keycode::Tab => ImGuiKey_ImGuiKey_Tab,
        Keycode::DpadLeft => ImGuiKey_ImGuiKey_LeftArrow,
        Keycode::DpadRight => ImGuiKey_ImGuiKey_RightArrow,
        Keycode::DpadUp => ImGuiKey_ImGuiKey_UpArrow,
        Keycode::DpadDown => ImGuiKey_ImGuiKey_DownArrow,
        Keycode::PageUp => ImGuiKey_ImGuiKey_PageUp,
        Keycode::PageDown => ImGuiKey_ImGuiKey_PageDown,
        Keycode::MoveHome => ImGuiKey_ImGuiKey_Home,
        Keycode::MoveEnd => ImGuiKey_ImGuiKey_End,
        Keycode::Insert => ImGuiKey_ImGuiKey_Insert,
        Keycode::ForwardDel => ImGuiKey_ImGuiKey_Delete,
        Keycode::ForwardDel => ImGuiKey_ImGuiKey_Backspace,
        Keycode::Space => ImGuiKey_ImGuiKey_Space,
        Keycode::Enter => ImGuiKey_ImGuiKey_Enter,
        Keycode::Escape => ImGuiKey_ImGuiKey_Escape,
        Keycode::Apostrophe => ImGuiKey_ImGuiKey_Apostrophe,
        Keycode::Comma => ImGuiKey_ImGuiKey_Comma,
        Keycode::Minus => ImGuiKey_ImGuiKey_Minus,
        Keycode::Period => ImGuiKey_ImGuiKey_Period,
        Keycode::Slash => ImGuiKey_ImGuiKey_Slash,
        Keycode::Semicolon => ImGuiKey_ImGuiKey_Semicolon,
        Keycode::Equals => ImGuiKey_ImGuiKey_Equal,
        Keycode::LeftBracket => ImGuiKey_ImGuiKey_LeftBracket,
        Keycode::Backslash => ImGuiKey_ImGuiKey_Backslash,
        Keycode::RightBracket => ImGuiKey_ImGuiKey_RightBracket,
        Keycode::Grave => ImGuiKey_ImGuiKey_GraveAccent,
        Keycode::CapsLock => ImGuiKey_ImGuiKey_CapsLock,
        Keycode::ScrollLock => ImGuiKey_ImGuiKey_ScrollLock,
        Keycode::NumLock => ImGuiKey_ImGuiKey_NumLock,
        Keycode::Sysrq => ImGuiKey_ImGuiKey_PrintScreen,
        Keycode::Break => ImGuiKey_ImGuiKey_Pause,
        Keycode::Numpad0 => ImGuiKey_ImGuiKey_Keypad0,
        Keycode::Numpad1 => ImGuiKey_ImGuiKey_Keypad1,
        Keycode::Numpad2 => ImGuiKey_ImGuiKey_Keypad2,
        Keycode::Numpad3 => ImGuiKey_ImGuiKey_Keypad3,
        Keycode::Numpad4 => ImGuiKey_ImGuiKey_Keypad4,
        Keycode::Numpad5 => ImGuiKey_ImGuiKey_Keypad5,
        Keycode::Numpad6 => ImGuiKey_ImGuiKey_Keypad6,
        Keycode::Numpad7 => ImGuiKey_ImGuiKey_Keypad7,
        Keycode::Numpad8 => ImGuiKey_ImGuiKey_Keypad8,
        Keycode::Numpad9 => ImGuiKey_ImGuiKey_Keypad9,
        Keycode::NumpadDot => ImGuiKey_ImGuiKey_KeypadDecimal,
        Keycode::NumpadDivide => ImGuiKey_ImGuiKey_KeypadDivide,
        Keycode::NumpadMultiply => ImGuiKey_ImGuiKey_KeypadMultiply,
        Keycode::NumpadSubtract => ImGuiKey_ImGuiKey_KeypadSubtract,
        Keycode::NumpadAdd => ImGuiKey_ImGuiKey_KeypadAdd,
        Keycode::NumpadEnter => ImGuiKey_ImGuiKey_KeypadEnter,
        Keycode::NumpadEquals => ImGuiKey_ImGuiKey_KeypadEqual,
        Keycode::CtrlLeft => ImGuiKey_ImGuiKey_LeftCtrl,
        Keycode::ShiftLeft => ImGuiKey_ImGuiKey_LeftShift,
        Keycode::AltLeft => ImGuiKey_ImGuiKey_LeftAlt,
        Keycode::MetaLeft => ImGuiKey_ImGuiKey_LeftSuper,
        Keycode::CtrlRight => ImGuiKey_ImGuiKey_RightCtrl,
        Keycode::ShiftRight => ImGuiKey_ImGuiKey_RightShift,
        Keycode::AltRight => ImGuiKey_ImGuiKey_RightAlt,
        Keycode::MetaRight => ImGuiKey_ImGuiKey_RightSuper,
        Keycode::Menu => ImGuiKey_ImGuiKey_Menu,
        Keycode::Keycode0 => ImGuiKey_ImGuiKey_0,
        Keycode::Keycode1 => ImGuiKey_ImGuiKey_1,
        Keycode::Keycode2 => ImGuiKey_ImGuiKey_2,
        Keycode::Keycode3 => ImGuiKey_ImGuiKey_3,
        Keycode::Keycode4 => ImGuiKey_ImGuiKey_4,
        Keycode::Keycode5 => ImGuiKey_ImGuiKey_5,
        Keycode::Keycode6 => ImGuiKey_ImGuiKey_6,
        Keycode::Keycode7 => ImGuiKey_ImGuiKey_7,
        Keycode::Keycode8 => ImGuiKey_ImGuiKey_8,
        Keycode::Keycode9 => ImGuiKey_ImGuiKey_9,
        Keycode::A => ImGuiKey_ImGuiKey_A,
        Keycode::B => ImGuiKey_ImGuiKey_B,
        Keycode::C => ImGuiKey_ImGuiKey_C,
        Keycode::D => ImGuiKey_ImGuiKey_D,
        Keycode::E => ImGuiKey_ImGuiKey_E,
        Keycode::F => ImGuiKey_ImGuiKey_F,
        Keycode::G => ImGuiKey_ImGuiKey_G,
        Keycode::H => ImGuiKey_ImGuiKey_H,
        Keycode::I => ImGuiKey_ImGuiKey_I,
        Keycode::J => ImGuiKey_ImGuiKey_J,
        Keycode::K => ImGuiKey_ImGuiKey_K,
        Keycode::L => ImGuiKey_ImGuiKey_L,
        Keycode::M => ImGuiKey_ImGuiKey_M,
        Keycode::N => ImGuiKey_ImGuiKey_N,
        Keycode::O => ImGuiKey_ImGuiKey_O,
        Keycode::P => ImGuiKey_ImGuiKey_P,
        Keycode::Q => ImGuiKey_ImGuiKey_Q,
        Keycode::R => ImGuiKey_ImGuiKey_R,
        Keycode::S => ImGuiKey_ImGuiKey_S,
        Keycode::T => ImGuiKey_ImGuiKey_T,
        Keycode::U => ImGuiKey_ImGuiKey_U,
        Keycode::V => ImGuiKey_ImGuiKey_V,
        Keycode::W => ImGuiKey_ImGuiKey_W,
        Keycode::X => ImGuiKey_ImGuiKey_X,
        Keycode::Y => ImGuiKey_ImGuiKey_Y,
        Keycode::Z => ImGuiKey_ImGuiKey_Z,
        _ => ImGuiKey_ImGuiKey_None,
    }
}
