use std::env;
use std::path::Path;
use std::fs::{self, File};

fn main() {
    env::set_var("OUT_DIR", "gen");
    let _ = fs::create_dir("gen");
    let src = "schema/vault.json";
    println!("cargo:rerun-if-changed={}", src);
    let file = File::open(src).unwrap();
    let spec = serde_json::from_reader(file).unwrap();
    let mut generator = progenitor::Generator::new();

    let content = generator.generate_text(&spec).unwrap();

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("api.rs");

    fs::write(out_file, content).unwrap();
}
