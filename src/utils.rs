extern crate serde;
extern crate dirs;
use serde::{Serialize,Deserialize};
use lazy_static::lazy_static;
use dirs::config_dir;

include!(concat!(env!("OUT_DIR"), "/feature_gen.rs"));

lazy_static! {
    static ref CONFIG_LOC:Vec<String> = 
        CONFIG_FILE_TYPES.iter().map(dirs::config_dir().unwrap()
        .join(env!("CARGO_PKG_NAME"))
        .join({
            let t = "config.".to_string();
            t.push_str(CONFIG_FILE_TYPE);
            t
        })
        .to_str().unwrap().to_string()).collect();
}
#[derive(Serialize,Deserialize)]
pub struct Config{
    token:String
}
//level of priority for each config type if multiple are enabled and available
// env > yaml > toml > ini > json
pub fn get_config()->Config{
    
}
