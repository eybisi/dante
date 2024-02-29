use egui::Pos2;
// use imgui_sys::*;
use log::info;
use ndk::event::InputEvent;
use ndk::event::KeyAction;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::sync::OnceLock;
pub static IMGUI_INIT: OnceLock<bool> = OnceLock::new();

pub static EGUI_EVENT_SENDER: OnceLock<Mutex<Sender<egui::Event>>> = OnceLock::new();

static mut G_TIME: f32 = 0.0f32;
use crate::utils::logger::dbg_info;


enum ActionType {
    Down, // 0
    Up,   //  1
    Move, // 2
}

pub fn imgui_impl_android_handle_inputevent_x_y_event(x: i32, y: i32, event_type: i32) -> i32 {
    // dbg_info!("handling x: {}, y: {}, event_type: {}", x, y, event_type);
    // let io = unsafe { igGetIO() };
    // if io.is_null() {
    //     panic!("Failed to initialize ImGui IO");
    // }
    // let io = unsafe { &mut *io };
    match event_type {
        0 => unsafe {
            EGUI_EVENT_SENDER
                .get()
                .unwrap()
                .lock()
                .unwrap()
                .send(egui::Event::PointerButton {
                    pressed: true,
                    pos: Pos2::new(x as f32, y as f32),
                    button: egui::PointerButton::Primary,
                    modifiers: egui::Modifiers::NONE,
                });
            // ImGuiIO_AddMouseButtonEvent(io, 0, true);
        },
        1 => unsafe {
            // ImGuiIO_AddMouseButtonEvent(io, 0, false);
            EGUI_EVENT_SENDER
                .get()
                .unwrap()
                .lock()
                .unwrap()
                .send(egui::Event::PointerButton {
                    pressed: false,
                    pos: Pos2::new(x as f32, y as f32),
                    button: egui::PointerButton::Primary,
                    modifiers: egui::Modifiers::NONE,
                });
        },
        2 => unsafe {
            EGUI_EVENT_SENDER
                .get()
                .unwrap()
                .lock()
                .unwrap()
                .send(egui::Event::PointerMoved(Pos2::new(x as f32, y as f32)));
            // ImGuiIO_AddMousePosEvent(io, x as f32, y as f32);
        },
        _ => {}
    }

    0
}

pub fn imgui_impl_android_handle_inputevent(input_event: InputEvent) {

    match input_event {
        InputEvent::MotionEvent(e) => {
            let action = e.action();
            let meta_state = e.meta_state();
            let ptr_idx = e.pointer_index();
            match action {
                ndk::event::MotionAction::Down | ndk::event::MotionAction::Up => unsafe {
                    // ImGuiIO_AddMousePosEvent(
                    //     io,
                    //     e.pointer_at_index(ptr_idx).x(),
                    //     e.pointer_at_index(ptr_idx).y(),
                    // );
                    // ImGuiIO_AddMouseButtonEvent(io, 0, action == ndk::event::MotionAction::Down);
                },
                ndk::event::MotionAction::HoverMove | ndk::event::MotionAction::Move => unsafe {
                    // ImGuiIO_AddMousePosEvent(
                    //     io,
                    //     e.pointer_at_index(ptr_idx).x(),
                    //     e.pointer_at_index(ptr_idx).y(),
                    // );
                },
                ndk::event::MotionAction::Scroll => unsafe {
                    // ImGuiIO_AddMouseWheelEvent(
                    //     io,
                    //     e.pointer_at_index(ptr_idx)
                    //         .axis_value(ndk::event::Axis::Hscroll),
                    //     e.pointer_at_index(ptr_idx)
                    //         .axis_value(ndk::event::Axis::Vscroll),
                    // )
                },
                ndk::event::MotionAction::ButtonPress | ndk::event::MotionAction::ButtonRelease => {
                    let button_state = e.button_state();
                    // unsafe {
                        // ImGuiIO_AddMouseButtonEvent(io, 0, button_state.primary());
                        // ImGuiIO_AddMouseButtonEvent(io, 1, button_state.secondary());
                        // ImGuiIO_AddMouseButtonEvent(io, 2, button_state.teriary());
                    // }
                }
                _ => {}
            }
        }
        InputEvent::KeyEvent(e) => {
            // dbg_info!("Key event: {:?}", e);
            let key_code = e.key_code();
            let scan_code = e.scan_code();
            let action = e.action();
            let meta_state = e.meta_state();

            // unsafe {
                // ImGuiIO_AddKeyEvent(io, ImGuiKey_ImGuiMod_Ctrl, meta_state.ctrl_on());
                // ImGuiIO_AddKeyEvent(io, ImGuiKey_ImGuiMod_Shift, meta_state.shift_on());
                // ImGuiIO_AddKeyEvent(io, ImGuiKey_ImGuiMod_Alt, meta_state.alt_on());
                // ImGuiIO_AddKeyEvent(io, ImGuiKey_ImGuiMod_Super, meta_state.meta_on());
            // }
            match action {
                KeyAction::Down | KeyAction::Up => {
                    // let key_code = imgui_impl_android_keycode_to_imguikey(key_code);
                    // if key_code != ImGuiKey_ImGuiKey_None {
                    //     unsafe {
                    //         // ImGuiIO_AddKeyEvent(io, key_code as u32, action == KeyAction::Down);
                    //         // ImGuiIO_SetKeyEventNativeData(
                    //         //     io,
                    //         //     key_code as u32,
                    //         //     key_code as i32,
                    //         //     scan_code,
                    //         //     0,
                    //         // )
                    //     }
                    // }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

