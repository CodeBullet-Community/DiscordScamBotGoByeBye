extern crate serde;
extern crate dirs;
#[cfg(feature = "json")]
extern crate serde_json;
#[cfg(feature = "ini")]
extern crate serde_ini;
#[cfg(feature = "toml")]
extern crate serde_yaml;
#[cfg(feature = "yaml")]
extern crate serde_yaml;
#[cfg(feature = "env")]
extern crate envy;
use serde::{Serialize,Deserialize};
use lazy_static::lazy_static;
use dirs::config_dir;


//sets a bunch of consts up namely
// CONFIG_FILE_TYPES
include!(concat!(env!("OUT_DIR"), "/feature_gen.rs"));

lazy_static! {
    static ref CONFIG_LOCS:Vec<String> = 
        CONFIG_FILE_TYPES.iter().map(
            |cft|{
                config_dir().unwrap()
                    .join(env!("CARGO_PKG_NAME"))
                    .join({
                        let mut t = "config.".to_string();
                        t.push_str(cft);
                        t
                    })
                    .to_str().unwrap().to_string()
            }).collect();
}
#[derive(Serialize,Deserialize)]
pub struct Config{
    token:String
}
//level of priority for each config type if multiple are enabled and available
// env > yaml > toml > ini > json
pub fn get_config()->Option<Config>{
    let mut config:Option<Config> = None;
    #[cfg(feature = "json")]
    {
        let tmp = 
    }
    
    config
}
