#[cfg(test)]
mod tests;
extern crate serde;
extern crate dirs;
extern crate log;
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

use std::fmt::Debug;
use serde::{Serialize,Deserialize};
use dirs::config_dir;
use log::*;
//read_to_string is used but depending on specified features rust may give a warning
#[allow(unused_imports)]
use std::fs::read_to_string;

#[derive(Serialize,Deserialize,Clone)]
/// An mirror of Config that has attrs that are optional to allow for relatively easy unification
/// between multiple of them to support multiple config files at the same time
struct InternConfig{
    token:Option<String>
}
pub struct Config {
    pub token:String
}
#[derive(Debug)]
enum ConfigAttrMissing{TOKEN}
impl InternConfig{
    /// Convert from InternConfig to config
    fn verify(self)->Result<Config,ConfigAttrMissing> {
        let token = match self.token {
            Some(token)=>token,
            None=> return Err(ConfigAttrMissing::TOKEN)
        };
        Ok(Config{token})
    }
}
impl Debug for InternConfig {
    fn fmt(&self, formatter:&mut std::fmt::Formatter)->Result<(),std::fmt::Error>{
        if self.token.is_some() {
            formatter.write_str("{")?;
            formatter.write_str(" token: ")?;
            formatter.write_str(self.token.clone().unwrap().as_str())?;
            formatter.write_str(" }")?;
        }
        Ok(())
    }
}
/// Trait that allows for 2 of the same type struct to merge in some meaningful way
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
/// Provides a Config object generated from the config file right now defaulting to reading from
/// $(config_dir)/$(crate_name)/config.$(file_type) read dirs config_dir documentation for more
/// info on what/where that is, file_type is all file_types that were compiled as features into the
/// bot with a priority order(earlier is higher) between configs of env_vars(not a file, read envy documentation),
/// yaml, toml, ini and finally json with higher priority configuration overriding lover priority
/// configuration
pub fn get_config()->Config{
    #[allow(unused_assignments)]
    let mut config:Option<InternConfig> = None;
    #[cfg(feature = "env")]
    {
        config = match envy::from_env::<InternConfig>() {
            Ok(cfg)=>Some(cfg),
            Err(_)=>None
        };
        trace!("config after env vars {:?}",config);
    }
    #[cfg(feature = "yaml")]
    {
        if let Ok(file_dat) = read_to_string(config_file_loc("yaml")){
            config = config.union(serde_yaml::from_str(file_dat.as_str()).unwrap());
        }
        trace!("config after yaml vars {:?}",config);
    }
    #[cfg(feature = "toml")]
    {
        if let Ok(file_dat) = read_to_string(config_file_loc("toml")){
            config = config.union(toml::from_str(file_dat.as_str()).unwrap());
        }
        trace!("config after toml vars {:?}",config);
    }
    #[cfg(feature = "ini")]
    {
        if let Ok(file_dat) = read_to_string(config_file_loc("ini")){
            config = config.union(serde_ini::from_str(file_dat.as_str()).unwrap());
        }
        trace!("config after ini vars {:?}",config);
    }
    #[cfg(feature = "json")]
    {
        if let Ok(file_dat) = read_to_string(config_file_loc("json")){
            config = config.union(serde_json::from_str(file_dat.as_str()).unwrap());
        }
        trace!("config after json vars {:?}",config);
    }
    debug!("complete pre-verified config {:?}",config);
    //this should be rewritten so users don't get bad error messages when they don't have a config
    config.expect("no config provided").verify().expect("config was missing a value or otherwise couldn't be verified")
}

#[allow(dead_code)]
/// provides the default config file location(currently the only config file location) to get_config based on some file type
pub fn config_file_loc(filetype:&str) -> String {
    let loc = config_dir().unwrap()
        .join(env!("CARGO_PKG_NAME"))
        .join(format!("config.{}",filetype)).to_str().unwrap().to_string();
    debug!("{} filetype is located at {}",filetype,loc);
    loc
}
