// https://github.com/ocornut/imgui/blob/master/backends/imgui_impl_opengl3_loader.h
use std::sync::OnceLock;

use gl33::GlFns;

use libc::{dlclose, dlerror, dlopen, dlsym, getpid, size_t};
use libc::{RTLD_LAZY, RTLD_LOCAL};
use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void, CStr, CString};
static GLX_GET_PROC_ADDRESS: OnceLock<unsafe extern "C" fn(*const u8) -> *const c_void> = OnceLock::new();
static LIBGL_HANDLE: OnceLock<usize> = OnceLock::new();
static G_GLFNS: OnceLock<GlFns> = OnceLock::new();
static G_VERSION: OnceLock<(u32, u32)> = OnceLock::new();

pub type GLenum = u32;
pub type GLboolean = u8;
pub type GLbitfield = u32;
pub type GLuint = u32;
pub type GLint = i32;
pub type GLsizei = i32;




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gl3wRetCodes {
    Gl3wOk = 0,
    Gl3wErrorInit = -1,
    Gl3wErrorLibraryOpen = -2,
    Gl3wErrorOpenglVersion = -3,
    Gl3wErrorOpenglMissing = -4,
    Gl3wErrorProcMissing = -5,
}

pub fn imgl3_w_init()->Gl3wRetCodes{
    let r = open_libgl();
    if r != Gl3wRetCodes::Gl3wOk {
        panic!("Failed to open libGLESv3.so");
    }
    imgl3_w_init2();
    parse_version()
}

fn imgl3_w_init2(){
    let gl_fns = unsafe { gl33::GlFns::load_from(&|s| get_proc(s))}.unwrap();

    G_GLFNS.get_or_init(|| {
        gl_fns
    });
}

pub fn parse_version()->Gl3wRetCodes{

    let gl_fns = G_GLFNS.get().unwrap();
    let mut major = 0;
    
    unsafe {gl_fns.GetIntegerv(gl33::GL_MAJOR_VERSION, &mut major)};
    let mut minor = 0;
    unsafe{gl_fns.GetIntegerv(gl33::GL_MINOR_VERSION, &mut minor)};
    G_VERSION.get_or_init(|| {
        (major as u32, minor as u32)
    });
    if major < 3 {
        return Gl3wRetCodes::Gl3wErrorOpenglVersion
    }
    Gl3wRetCodes::Gl3wOk
}

fn open_libgl()->Gl3wRetCodes{
    let libgl = unsafe { dlopen(b"libGLESv3.so\0".as_ptr(), RTLD_LAZY|RTLD_LOCAL) };
    if libgl.is_null() {
        return Gl3wRetCodes::Gl3wErrorLibraryOpen;
    }
    LIBGL_HANDLE.get_or_init(|| libgl as usize);

    GLX_GET_PROC_ADDRESS.get_or_init(|| unsafe {
        let glx_get_proc_address = dlsym(libgl as *mut c_void, b"glXGetProcAddress\0".as_ptr());
        if glx_get_proc_address.is_null() {
            panic!("Failed to load glXGetProcAddress");
        }
        std::mem::transmute(glx_get_proc_address)
    });
    Gl3wRetCodes::Gl3wOk
}

fn get_proc(name: *const u8) -> *const c_void {
    let res = unsafe{ GLX_GET_PROC_ADDRESS.get().unwrap()(name)};
    if res.is_null() {
        panic!("Failed to load");
        // Resolve via libhandle if possible
        let libgl = LIBGL_HANDLE.get().unwrap();
        let res = unsafe { dlsym(*libgl as *mut c_void, name) };
        if res.is_null() {
            panic!("Failed to load");
        }
    }
    res
}
