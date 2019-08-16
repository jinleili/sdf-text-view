use std::fs::File;
use std::io::Write;
use std::vec::Vec;

use getopts::Options;

mod sdf;
use sdf::SDF;

mod sdf2;
use sdf2::SDF2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program_name = args[0].clone();
    if args.len() == 1 {
        println!("Error: \nUsage: cargo run input.png output.png");
        return;
    }
    let mut opts = Options::new();
    let parsed_opts = match opts.parse(&args[1..]) {
        Ok(v) => v,
        Err(e) => panic!(e.to_string()),
    };
    let input_image_path = &parsed_opts.free[0];
    let output_image_path = &parsed_opts.free[1];

    let mut sdf = SDF2::new(input_image_path, output_image_path);
    sdf.generate();
}
