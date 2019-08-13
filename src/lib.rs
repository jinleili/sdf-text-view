use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern crate libc;

#[no_mangle]
pub extern "C" fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn rust_greeting_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

pub use uni_view::*;

mod surface_view;
pub use surface_view::*;

mod depth_stencil;
mod geometry;
pub mod math;
pub mod matrix_helper;
mod node;
mod texture;
mod utils;
mod vertex;

// #[cfg(not(target_os = "ios"))]
mod shader;

mod sdf_text_view;
pub use sdf_text_view::SDFTextView;

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub extern "C" fn create_triangle(view: uni_view::AppViewObj) -> *mut libc::c_void {
    // let v = AppViewWrapper(view);
    let rust_view = uni_view::AppView::new(view);
    let obj = Triangle::new(rust_view);
    box_obj(obj)
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub extern "C" fn create_fluid(view: uni_view::AppViewObj) -> *mut libc::c_void {
    // let v = AppViewWrapper(view);
    let rust_view = uni_view::AppView::new(view);
    let obj = fluid2::PoiseuilleFlow::new(rust_view);
    box_obj(obj)
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub extern "C" fn create_blur_filter(view: uni_view::AppViewObj) -> *mut libc::c_void {
    let rust_view = uni_view::AppView::new(view);
    let obj = filters::BlurFilter::new(rust_view);
    box_obj(obj)
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub extern "C" fn create_gray_filter(view: uni_view::AppViewObj) -> *mut libc::c_void {
    let rust_view = uni_view::AppView::new(view);
    let obj = filters::GrayFilter::new(rust_view);
    box_obj(obj)
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub extern "C" fn create_page_turning(view: uni_view::AppViewObj) -> *mut libc::c_void {
    let rust_view = uni_view::AppView::new(view);
    let obj = PageTurning::new(rust_view);
    box_obj(obj)
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub extern "C" fn create_roll_animation(view: uni_view::AppViewObj) -> *mut libc::c_void {
    let rust_view = uni_view::AppView::new(view);
    let obj = RollAnimation::new(rust_view);
    box_obj(obj)
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub extern "C" fn create_brush_view(view: uni_view::AppViewObj) -> *mut libc::c_void {
    let rust_view = uni_view::AppView::new(view);
    let obj = BrushView::new(rust_view);
    box_obj(obj)
}

#[cfg(not(target_os = "macos"))]
fn box_obj(obj: impl SurfaceView) -> *mut libc::c_void {
    let boxed_trait: Box<dyn SurfaceView> = Box::new(obj);
    let boxed_boxed_trait = Box::new(boxed_trait);
    let heap_pointer = Box::into_raw(boxed_boxed_trait);
    // let boxed_boxed_trait = Box::new(v);
    // let heap_pointer = Box::into_raw(boxed_boxed_trait);
    heap_pointer as *mut libc::c_void
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub unsafe extern "C" fn enter_frame(obj: *mut libc::c_void) -> *mut libc::c_void {
    let mut obj: Box<Box<dyn SurfaceView>> = Box::from_raw(obj as *mut _);
    obj.enter_frame();

    // 重新将所有权移出
    Box::into_raw(obj) as *mut libc::c_void
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub unsafe extern "C" fn touch_move(obj: *mut libc::c_void, p: TouchPoint) {
    let mut obj: Box<Box<dyn SurfaceView>> = Box::from_raw(obj as *mut _);
    obj.touch_moved(crate::math::Position::new(p.x, p.y));

    // 重新将所有权移出
    let _ = Box::into_raw(obj) as *mut libc::c_void;
}
