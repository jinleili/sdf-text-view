use std::path::PathBuf;

pub struct FileSystem {}

impl FileSystem {
    pub fn get_bundle_url() -> &'static str {
        env!("CARGO_MANIFEST_DIR")
    }

     pub fn get_texture_file_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../assets")
        .join(name)
    }
}