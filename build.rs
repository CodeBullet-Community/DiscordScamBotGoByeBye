extern crate const_gen;
extern crate toml;
extern crate serde;
extern crate cargo_toml;

use std::{fs, error::Error, io::Write, ffi::OsString};

fn main() -> Result<(),Box<dyn Error>>{
    println!("cargo:rerun-if-changed=build.rs");
   // std::io::stderr().write(format!("{}",std::env::vars_os().map(|s|s.0.to_str().unwrap().to_owned()+","+s.1.to_str().unwrap()).collect::<Vec<String>>().join("\n")).as_bytes())?;
    feature_handle()?;
    Ok(())
}

const SUPPORTED_CONF_FORMATS:[&str;4] = ["yaml","toml", "json", "ini"];
fn feature_handle()->Result<(),Box<dyn Error>> {
    println!("cargo:rerun-if-changed=Cargo.toml");

    let features:Vec<String> = toml::from_str::<cargo_toml::Manifest>(fs::read_to_string("Cargo.toml")?.as_str())?
        //I think the fact I have to derefence and clone the keys is stupid as well
        .features.keys().map(|v|(*v).clone()).collect();
    let config_file_features:Vec<String> = features.iter()
        .filter(|s|SUPPORTED_CONF_FORMATS.contains(&s.as_str()))
        .map(|s|(*s).clone()).collect();
    // this is a rather hacky way to do this but oh well
    let config_declarations = format!("const CONFIG_FILE_TYPES:[&str;{}] = [{}]",config_file_features.len(),config_file_features.join(","));
    fs::write({let mut  x = std::env::var_os("OUT_DIR").unwrap().into_string().unwrap();x.push_str("/feature_gen.rs"); x}, config_declarations)?;
    Ok(())
}
