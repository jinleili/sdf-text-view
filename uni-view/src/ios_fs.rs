
extern crate objc;
use self::objc::{*, runtime::{Class, Object}, rc::StrongPtr};
extern crate objc_foundation;
use self::objc_foundation::{NSString, INSString};

extern crate lazy_static;
use self::lazy_static::*;

use std::path::PathBuf;

lazy_static! {
    static ref BUNDLE_PATH: &'static str = get_bundle_url();
}
fn get_bundle_url() -> &'static str {
    let cls = class!(NSBundle);
        // Allocate an instance
    let bundle = unsafe { StrongPtr::new(msg_send![cls, mainBundle]) };
    let path: &str = unsafe { 
        // let url: *mut Object = msg_send![*bundle, resourcePath];
        // 资源路径要用 resourcePath 
        let path: &NSString  = msg_send![*bundle, resourcePath];
        path.as_str()
    };

    path
}

pub struct FileSystem {}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {

        }
    }

    pub fn get_bundle_url() -> &'static str {
        &BUNDLE_PATH
    }

    pub fn get_shader_path(name: &str, suffix: &str) -> String {
        FileSystem::get_spirv_file_path(name, suffix)
    }

    fn get_spirv_file_path(name: &str, suffix: &str) -> String {
        let mut p = name.to_string().replace("/", "_");
        p = BUNDLE_PATH.to_string() + "/shader-gen/" + &p;
        p += &format!("_{}.spv", suffix);

        p
    }

    pub fn get_texture_file_path(name: &str) -> PathBuf {
        let p = BUNDLE_PATH.to_string() + "/assets/" + name;
        PathBuf::from(&p)      
    }
}