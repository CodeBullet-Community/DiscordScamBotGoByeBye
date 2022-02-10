extern crate serde;
extern crate dirs;
#[cfg(feature = "json")]
extern crate serde_json;
#[cfg(feature = "ini")]
extern crate serde_ini;
#[cfg(feature = "toml")]
extern crate toml;
#[cfg(feature = "yaml")]
extern crate serde_yaml;
#[cfg(feature = "env")]
extern crate envy;
use std::{fmt::Debug};

use serde::{Serialize,Deserialize};
use dirs::config_dir;
use std::fs::read_to_string;

#[derive(Serialize,Deserialize,Clone)]
struct InternConfig{
    token:Option<String>
}
pub struct Config {
    pub token:String
}
#[derive(Debug)]
enum ConfigAttrMissing{TOKEN}
impl InternConfig{
    fn verify(self)->Result<Config,ConfigAttrMissing> {
        let token = match self.token {
            Some(token)=>token,
            None=> return Err(ConfigAttrMissing::TOKEN)
        };
        Ok(Config{token})
    }
}
trait Combinable {
    fn union(self, other:Self)->Self;
}
impl Combinable for InternConfig {
    //there may be a double backflippy macro way of handling this but for now the effort isn't
    //worth it
    fn union(mut self, other: Self)->Self{
        if other.token.is_some(){
            self.token = Some(self.token.unwrap_or(other.token.unwrap()));
        }
        self
    }
}
impl<T:Combinable> Combinable for Option<T>{
    fn union(self, other:Self)->Self{
        if other.is_none(){
            self
        }
        else if self.is_none() {
            other
        }
        else {
            Some(self.unwrap().union(other.unwrap()))
        }
    }
}
//level of priority for each config type if multiple are enabled and available
// env > yaml > toml > ini > json
pub fn get_config()->Config{
    let mut config:Option<InternConfig> = None;
    #[cfg(feature = "env")]
    {
        config = match envy::from_env::<InternConfig>() {
            Ok(cfg)=>Some(cfg),
            Err(_)=>None
        }
    }
    #[cfg(feature = "yaml")]
    {
        if let Ok(file_dat) = read_to_string(config_file_loc("yaml")){
            config = config.union(serde_yaml::from_str(file_dat.as_str()).unwrap());
        }
    }
    #[cfg(feature = "toml")]
    {
        if let Ok(file_dat) = read_to_string(config_file_loc("toml")){
            config = config.union(toml::from_str(file_dat.as_str()).unwrap());
        }
    }
    #[cfg(feature = "ini")]
    {
        if let Ok(file_dat) = read_to_string(config_file_loc("ini")){
            config = config.union(serde_ini::from_str(file_dat.as_str()).unwrap());
        }
    }
    #[cfg(feature = "json")]
    {
        if let Ok(file_dat) = read_to_string(config_file_loc("json")){
            config = config.union(serde_json::from_str(file_dat.as_str()).unwrap());
        }
    }
    //this should be rewritten so users don't get bad error messages when they don't have a config
    config.expect("no config provided").verify().expect("config was missing a value or otherwise couldn't be verified")
}
pub fn config_file_loc(filetype:&str) -> String {
    config_dir().unwrap()
        .join(env!("CARGO_PKG_NAME"))
        .join(format!("config.{}",filetype)).to_str().unwrap().to_string()

}
