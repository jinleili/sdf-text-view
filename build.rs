use shaderc::ShaderKind;

use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

// 参考： https://falseidolfactory.com/2018/06/23/compiling-glsl-to-spirv-at-build-time.html
// 所有 GL_ 打头的宏名称都是 glsl 保留的，不能自定义
const SHADER_VERSION_GL: &str = "#version 450\n";
const SHADER_IMPORT: &str = "#include ";

// build.rs 配置：https://blog.csdn.net/weixin_33910434/article/details/87943334
fn main() -> Result<(), Box<dyn Error>> {
    // 只在编译为移动端的库文件时，才编译 spv
    let shader_files: Vec<&str> = match std::env::var("TARGET") {
        Ok(target) => {
            if target.contains("ios") {
                vec!["none", "sdf/text", "clear_color"]
            } else {
                vec![]
            }
        }
        _ => vec![],
    };

    let compute_shader: Vec<&str> = match std::env::var("TARGET") {
        Ok(target) => {
            if target.contains("ios") {
                vec!["sdf/sdf", "sdf/sdf_x", "sdf/sdf_y"]
            } else {
                vec![]
            }
        }
        _ => vec![],
    };

    // Tell the build script to only run again if we change our source shaders
    // println!("cargo:rerun-if-changed=shader");

    // Create destination path if necessary
    std::fs::create_dir_all("shader-gen")?;
    for name in shader_files {
        let _ = generate_shader_spirv(name, ShaderKind::Vertex);
        let _ = generate_shader_spirv(name, ShaderKind::Fragment);
    }

    for comp in compute_shader {
        let _ = generate_shader_spirv(comp, ShaderKind::Compute);
    }

    Ok(())
}

fn generate_shader_spirv(name: &str, ty: ShaderKind) -> Result<(), Box<dyn Error>> {
    let suffix = match ty {
        ShaderKind::Vertex => "vs",
        ShaderKind::Fragment => "fs",
        _ => "comp",
    };

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("shader")
        .join(format!("{}.{}.glsl", name, suffix));
    let mut out_path = "shader-gen/".to_string();
    out_path += &format!("{}_{}.spv", (name.to_string().replace("/", "_")), suffix);

    let code = match read_to_string(&path) {
        Ok(code) => code,
        Err(e) => {
            if ty == ShaderKind::Vertex {
                load_common_vertex_shader()
            } else {
                panic!("Unable to read {:?}: {:?}", path, e)
            }
        }
    };

    let mut shader_source = String::new();
    shader_source.push_str(SHADER_VERSION_GL);
    parse_shader_source(&code, &mut shader_source);
    // panic!("--panic--");

    let mut compiler = shaderc::Compiler::new().unwrap();
    let options = shaderc::CompileOptions::new().unwrap();
    let binary_result = compiler
        .compile_into_spirv(&shader_source, ty, "shader.glsl", "main", Some(&options))
        .unwrap();

    let _ =
        std::fs::File::create(&std::path::Path::new(&env!("CARGO_MANIFEST_DIR")).join(&out_path))
            .unwrap();
    std::fs::write(&out_path, binary_result.as_binary_u8()).unwrap();

    Ok(())
}

fn load_common_vertex_shader() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("shader").join("common.vs.glsl");

    let code = match read_to_string(&path) {
        Ok(code) => code,
        Err(e) => panic!("Unable to read {:?}: {:?}", path, e),
    };

    code
}

// Parse a shader string for imports. Imports are recursively processed, and
// prepended to the list of outputs.
fn parse_shader_source(source: &str, output: &mut String) {
    for line in source.lines() {
        match line.find("//") {
            Some(_) => (),
            None => {
                if line.starts_with(SHADER_IMPORT) {
                    let imports = line[SHADER_IMPORT.len()..].split(',');
                    // For each import, get the source, and recurse.
                    for import in imports {
                        if let Some(include) = get_shader_funcs(import) {
                            parse_shader_source(include, output);
                        } else {
                            println!("shader parse error -------");
                            println!("can't find shader functions: {}", import);
                            println!("--------------------------");
                        }
                    }
                } else {
                    output.push_str(line);
                    // output.push_str("\n");
                }
            }
        }
    }
    println!("line: {:?}", output);
}

// 获取通用 shader function
// 将着色器代码预先静态加载进程序，避免打包成 .a 静态库时找不到文件
fn get_shader_funcs(key: &str) -> Option<&str> {
    match key {
        "color_space_convert" => Some(COLOR_SPACE_CONVERT),
        "vs_micros" => Some(VS_MICROS),
        "fs_micros" => Some(FS_MICROS),
        "fluid_layout_and_fn" => Some(FLUID_DEFINE),
        "sdf_layout_and_fn" => Some(SDF_COMMON),
        _ => None,
    }
}

#[allow(dead_code)]
static VS_MICROS: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader/func/vs_micros.glsl"));

#[allow(dead_code)]
static FS_MICROS: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader/func/fs_micros.glsl"));

#[allow(dead_code)]
static COLOR_SPACE_CONVERT: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader/func/color_space_convert.glsl"));

#[allow(dead_code)]
static FLUID_DEFINE: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader/func/fluid.glsl"));

#[allow(dead_code)]
static SDF_COMMON: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader/sdf/common.glsl"));
