
use std::{fs, error::Error};

fn main() -> Result<(),Box<dyn Error>>{
    println!("cargo:rerun-if-changed=build.rs");
   // std::io::stderr().write(format!("{}",std::env::vars_os().map(|s|s.0.to_str().unwrap().to_owned()+","+s.1.to_str().unwrap()).collect::<Vec<String>>().join("\n")).as_bytes())?;
    feature_handle()?;
    Ok(())
}

const SUPPORTED_CONF_FORMATS:[&str;4] = ["yaml","toml", "json", "ini"];
fn feature_handle()->Result<(),Box<dyn Error>> {
    println!("cargo:rerun-if-changed=Cargo.toml");

    let config_file_features:Vec<String> = std::env::vars().map(|v|v.0)
        .filter(|v|SUPPORTED_CONF_FORMATS
            .map(|f|"CARGO_FEATURE_".to_owned()+f.to_uppercase().as_str())
            .contains(v))
        .map(|v|SUPPORTED_CONF_FORMATS
            .map(|f|"CARGO_FEATURE_".to_owned()+f.to_uppercase().as_str())
            .iter().position(|m|m==&v).unwrap())
        .map(|i|SUPPORTED_CONF_FORMATS[i].to_string())
        .collect();
    // this is a rather hacky way to do this but oh well
    let config_declarations = format!("const CONFIG_FILE_TYPES:[&str;{}] = [{}];",config_file_features.len(),config_file_features.iter().map(|feat|"\"".to_owned()+feat+"\"").collect::<Vec<String>>().join(","));
    fs::write({let mut  x = std::env::var_os("OUT_DIR").unwrap().into_string().unwrap();x.push_str("/feature_gen.rs"); x}, config_declarations)?;
    Ok(())
}
