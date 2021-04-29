use std::error::Error;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::path::PathBuf;

// build.rs 配置：https://blog.csdn.net/weixin_33910434/article/details/87943334
fn main() -> Result<(), Box<dyn Error>> {
    let shader_files: Vec<&str> = match std::env::var("TARGET") {
        Ok(target) => {
            // if target.contains("ios") {
            if target.contains("os") {
                vec!["sdf/sdf_y", "sdf/sdf_x", "sdf/sdf", "clear_color", "text"]
            } else {
                vec![]
            }
        }
        _ => vec![],
    };
    // 创建目录
    std::fs::create_dir_all("shader-preprocessed-wgsl")?;
    for name in shader_files {
        let _ = regenerate_shader(name);
    }
    Ok(())
}

fn regenerate_shader(shader_name: &str) -> Result<(), Box<dyn Error>> {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let path = PathBuf::from(&base_dir).join("shader-wgsl").join(format!("{}.wgsl", shader_name));
    let mut out_path = "shader-preprocessed-wgsl/".to_string();
    out_path += &format!("{}.wgsl", shader_name.replace("/", "_"));

    let code = match read_to_string(&path) {
        Ok(code) => code,
        Err(e) => {
            panic!("Unable to read {:?}: {:?}", path, e)
        }
    };

    let mut shader_source = String::new();
    parse_shader_source(&code, &mut shader_source, &base_dir);

    let mut f = std::fs::File::create(&std::path::Path::new(&base_dir).join(&out_path))?;
    f.write_all(shader_source.as_bytes())?;

    Ok(())
}

fn parse_shader_source(source: &str, output: &mut String, base_path: &str) {
    let include: &str = "#include ";
    for line in source.lines() {
        if line.starts_with(include) {
            let imports = line[include.len()..].split(',');
            // For each import, get the source, and recurse.
            for import in imports {
                if let Some(include) = get_shader_funcs(import, base_path) {
                    parse_shader_source(&include, output, base_path);
                } else {
                    println!("shader parse error -------");
                    println!("can't find shader functions: {}", import);
                    println!("--------------------------");
                }
            }
        } else {
            // 移除注释
            let need_delete = match line.find("//") {
                Some(_) => {
                    let segments: Vec<&str> = line.split("//").collect();
                    segments.len() > 1 && segments.first().unwrap().trim().is_empty()
                }
                None => false,
            };
            if !need_delete {
                output.push_str(line);
                output.push_str("\n");
            }
        }
    }
}

fn get_shader_funcs(key: &str, base_path: &str) -> Option<String> {
    let path = PathBuf::from(base_path).join("shader-wgsl").join(key.replace('"', ""));
    let shader = match read_to_string(&path) {
        Ok(code) => code,
        Err(e) => panic!("Unable to read {:?}: {:?}", path, e),
    };
    Some(shader)
}
