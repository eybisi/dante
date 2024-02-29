// use crate::imgui_menu;
use dobby_rs::hook;
use crate::utils::logger::dbg_info;
use log::info;
use std::ffi::c_void;
use std::mem::transmute;
use std::sync::Mutex;
use std::sync::OnceLock;

use egui;
use std::sync::mpsc::Receiver;
static ORIG_EGL_GETPROCADDR: OnceLock<u64> = OnceLock::new();
static ORIG_EGL_SWAPBUFFERS: OnceLock<u64> = OnceLock::new();

pub static EGUI_EVENT_RECEIVER: OnceLock<Mutex<Receiver<egui::Event>>> = OnceLock::new();

// EGLBoolean eglSwapBuffers( 	EGLDisplay display,
//     EGLSurface surface);

#[inline(never)]
fn hk_egl_swap_buffers(display: *mut c_void, surface: *mut c_void) -> i32 {
    // dbg_info!(
    //     "eglSwapBuffers triggered with display : {:p} and surface : {:p}",
    //     display, surface
    // );
    // imgui_menu::setup_imgui_opengl3();
    // imgui_menu::draw_imgui_opengl3();

    let mutex_ctx = {
        CONTEXT.get_or_init(|| {
            // query for client dimensions
            let mut output_width = 0;
            if !egl::query_surface(
                display as *mut _,
                surface as *mut _,
                egl::EGL_WIDTH,
                &mut output_width,
            ) {
                panic!("Couldn't query surface for width")
            }

            let mut output_height = 0;
            if !egl::query_surface(
                display as *mut _,
                surface as *mut _,
                egl::EGL_HEIGHT,
                &mut output_height,
            ) {
                panic!("Couldn't query surface for height")
            }

            // create new context and stash old one
            let old_context = egl::get_current_context().expect("current context");

            let configs = egl::get_configs(display as *mut _, 1);
            let config = configs.configs;
            let render_context = egl::create_context(
                display as *mut _,
                config,
                egl::EGL_NO_CONTEXT,
                &[egl::EGL_CONTEXT_CLIENT_VERSION, 3, egl::EGL_NONE],
            )
            .expect("renderer");

            egl::make_current(
                display as *mut _,
                surface as *mut _,
                surface as *mut _,
                render_context,
            );

            let glow_context = unsafe {
                glow::Context::from_loader_function(|i| egl::get_proc_address(i) as *const _)
            };
            let glow_context = std::sync::Arc::new(glow_context);
            let egui_ctx = egui::Context::default();

            let painter =
                egui_glow::Painter::new(glow_context, "", None).expect("failed to create renderer");

            // restore old context after
            egl::make_current(
                display as *mut _,
                surface as *mut _,
                surface as *mut _,
                old_context,
            );

            Mutex::new(PayloadContext {
                painter,
                egui_ctx,
                shapes: Default::default(),
                textures_delta: Default::default(),
                dimensions: [
                    output_width.try_into().unwrap(),
                    output_height.try_into().unwrap(),
                ],
                render_context,
            })
        })
        //CONTEXT.get().expect("context set")
    };
    let mut context = mutex_ctx.lock().unwrap();
    // store old context
    let old_context = egl::get_current_context().expect("current context");
    egl::make_current(
        display as *mut _,
        surface as *mut _,
        surface as *mut _,
        context.render_context,
    );

    context.render();

    egl::make_current(
        display as *mut _,
        surface as *mut _,
        surface as *mut _,
        old_context,
    );

    let orig_egl_swap_buffers = ORIG_EGL_SWAPBUFFERS.get().unwrap();
    let orig_egl_swap_buffers = unsafe {
        transmute::<usize, extern "system" fn(*mut c_void, *mut c_void) -> i32>(
            *orig_egl_swap_buffers as usize,
        )
    };
    orig_egl_swap_buffers(display, surface)
}

// COMMON RENDERING CONTEXT
static CONTEXT: OnceLock<Mutex<PayloadContext>> = OnceLock::new();
struct PayloadContext {
    painter: egui_glow::Painter,
    egui_ctx: egui::Context,

    shapes: Vec<egui::epaint::ClippedShape>,
    textures_delta: egui::TexturesDelta,

    dimensions: [u32; 2],

    #[cfg(any(target_os = "linux", target_os = "android"))]
    render_context: egl::EGLContext,
}

impl PayloadContext {
    fn render(&mut self) {
        let rx = EGUI_EVENT_RECEIVER.get().unwrap();
        let mut events = vec![];
        while let Ok(event) = rx.lock().unwrap().try_recv() {
            // dbg_info!("received event : {:?}", event);
            events.push(event);
        }
        

        let raw_input = egui::RawInput {
            events,
            ..Default::default()
        };
        
        let egui::FullOutput {
            platform_output: _platform_output,
            textures_delta,
            pixels_per_point,
            viewport_output: _viewport_output,
            shapes,
        } = self.egui_ctx.run(raw_input, |ui| {
            
            
            // let mut visuals = Visuals::dark();
            // visuals.widgets.active.weak_bg_fill = Color32::BLUE;
            // visuals.widgets.open.weak_bg_fill = Color32::BLUE;
            // // visuals.widgets.noninteractive.weak_bg_fill = Color32::BLUE;
            // // visuals.widgets.inactive.weak_bg_fill = Color32::BLUE;
            // let s = egui::Style {
            //     visuals: visuals.clone(),
            //     ..Default::default()
            // };
            // // ui.settings_ui(ui);
            // ui.set_style(s);
            // // egui::TopBottomPanel::top("my_top_panel").show(ui, |ui| {
            // //     ui.heading("~~WTS Lobbies 200gpa EA~~~!");
            // // });
            // // egui::SidePanel::left("my_side_panel").show(ui, |ui| {
            // //     ui.heading("~~WTS Lobbies 200gpa EA~~~!");
            // // });
            egui::Window::new("🔧 Settings")
            .open(&mut true)
            .vscroll(true)
            .show(&ui, |ui| {
                // self.egui_ctx.set_pixels_per_point(2.0);
                &self.egui_ctx.settings_ui(ui);
            });
        });

        self.shapes = shapes;
        self.textures_delta.append(textures_delta);

        let shapes = std::mem::take(&mut self.shapes);
        let mut textures_delta = std::mem::take(&mut self.textures_delta);


        for (id, image_delta) in textures_delta.set {
            self.painter.set_texture(id, &image_delta);
        }


        let clipped_primitives = self.egui_ctx.tessellate(shapes, pixels_per_point);
        self.painter.paint_primitives(
            self.dimensions,
            self.egui_ctx.pixels_per_point(),
            &clipped_primitives,
        );

        for id in textures_delta.free.drain(..) {
            self.painter.free_texture(id);
        }
    }
}

unsafe impl Send for PayloadContext {}
unsafe impl Sync for PayloadContext {}

#[inline(never)]
fn hk_egl_get_proc_addr(p_procname: *const u8) -> *mut c_void {
    // dbg_info!("eglGetProcAddress triggered!");
    // Convert the procname to a string
    let procname = unsafe { std::ffi::CStr::from_ptr(p_procname as *const u8) };
    let procname = procname.to_str().unwrap();
    let orig_egl_swap_buffers = ORIG_EGL_GETPROCADDR.get().unwrap();
    let orig_egl_swap_buffers = unsafe {
        transmute::<usize, extern "system" fn(*const u8) -> *mut c_void>(
            *orig_egl_swap_buffers as usize,
        )
    };
    let ret = orig_egl_swap_buffers(p_procname);
    // dbg_info!("procname: {}", procname);
    if procname == "eglSwapBuffers" {
        dbg_info!("hooking eglSwapBuffers");
        let orig_egl_swap_buffers =
            unsafe { hook(ret, hk_egl_swap_buffers as *mut c_void) }.unwrap();
        ORIG_EGL_SWAPBUFFERS.get_or_init(|| orig_egl_swap_buffers as u64);
    }
    ret
}

#[inline(never)]
pub fn hook_egl_proc_addr() {
    // dbg_info!("Hooking eglGetProcAddress");

    let lib = unsafe { libloading::Library::new("libEGL.so").expect("load payload") };

    let orig_egl_get_proc_addr = unsafe {
        let func: libloading::Symbol<unsafe extern "C" fn(*mut c_void)> =
            lib.get(b"eglGetProcAddress").expect("get pid function");
        func.into_raw().into_raw() as usize
    };
    let orig_egl_get_proc_addr = unsafe {
        hook(
            orig_egl_get_proc_addr as *mut c_void,
            hk_egl_get_proc_addr as *mut c_void,
        )
    }
    .unwrap();
    // dbg_info!("hooked eglGetProcAddress");
    ORIG_EGL_GETPROCADDR.get_or_init(|| orig_egl_get_proc_addr as u64);

    let orig_egl_swap_buffers = unsafe {
        let func: libloading::Symbol<unsafe extern "C" fn(*mut c_void)> =
            lib.get(b"eglSwapBuffers").expect("get pid function");
        func.into_raw().into_raw() as usize
    };
    let orig_egl_swap_buffers = unsafe {
        hook(
            orig_egl_swap_buffers as *mut c_void,
            hk_egl_swap_buffers as *mut c_void,
        )
    }
    .unwrap();
    // dbg_info!("hooked eglGetProcAddress");
    ORIG_EGL_SWAPBUFFERS.get_or_init(|| orig_egl_swap_buffers as u64);
}
