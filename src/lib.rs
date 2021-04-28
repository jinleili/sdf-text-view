use zerocopy::{AsBytes, FromBytes};

#[cfg(any(target_os = "ios", target_os = "android"))]
use std::{os::raw::c_char, ffi::CStr};

pub use idroid::utils::{depth_stencil, matrix_helper};
pub use uni_view::*;

mod compute_node;
mod render_node;

mod sdf_text_view;
pub use sdf_text_view::SDFTextView;

#[derive(Copy, Clone, AsBytes, FromBytes)]
#[repr(C)]
pub struct PicInfoUniform {
    info: [i32; 4],
    // only for requested 256 alignment: (256 - 16) / 4 = 60
    any: [i32; 60],
}

#[derive(Copy, Clone, AsBytes, FromBytes)]
#[repr(C)]
pub struct PicInfoUniform2 {
    info: [i32; 4],
    threshold: [f32; 4],
    any: [i32; 56],
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub extern "C" fn create_sdf_view(view: uni_view::AppViewObj) -> *mut libc::c_void {
    let rust_view = uni_view::AppView::new(view);
    let obj = SDFTextView::new(rust_view);
    idroid::box_obj(obj)
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub unsafe extern "C" fn sdf_view_set_bundle_image(
    obj: *mut libc::c_void, image_name: *mut c_char,
) {
    let c_str = CStr::from_ptr(image_name);
    let name = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };

    let mut obj: Box<Box<SDFTextView>> = Box::from_raw(obj as *mut _);
    obj.bundle_image(name.to_string(), false);
    let _ = Box::into_raw(obj) as *mut libc::c_void;
}
