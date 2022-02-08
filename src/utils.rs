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
use dirs::config_dir;



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
        let tmp:Config = serde_json::from_str();

    }
    config
}
fn config_file_loc(filetype:&str) -> String {
    config_dir().unwrap()
        .join(env!("CARGO_PKG_NAME"))
        .join(format!("config.{}",filetype)).to_str().unwrap().to_string()

}
