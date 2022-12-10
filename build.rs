use std::env;
use std::{collections::HashMap, fs};
use std::path::Path;

use serde::Deserialize;
use toml::Value;

#[derive(Debug, Deserialize)]
struct VoltFile {
    config: HashMap<String, VoltConfig>,
}

#[derive(Debug, Deserialize)]
struct VoltConfig {
    default: Option<Value>,
}

fn main() {
    let volt_file_content = fs::read_to_string("volt.toml").unwrap();
    let volt_file: VoltFile = toml::from_str(&volt_file_content).unwrap();

    let mut content = String::from("macro_rules! volt_config {\n");

    for (name, volt_config) in volt_file.config {
        let Some(default) = volt_config.default else { continue };
        let value = match default {
            Value::String(s) => format!("\"{s}\""), // TODO: escape string
            Value::Boolean(b) => format!("{:?}", b),
            _ => continue,
        };
        let line = format!("    (\"{name}\") => {{ {value} }};\n");
        content.push_str(&line);
    }

    content.push_str("}\n");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("volt_config.rs");
    fs::write(&dest_path, &content).unwrap();

    println!("cargo:rerun-if-changed=volt.toml");
}
