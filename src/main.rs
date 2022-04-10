#[cfg(test)]
mod tests;
extern crate serenity;
extern crate tokio;
extern crate serde;
extern crate log;
extern crate env_logger;
extern crate regex;
mod utils;
mod prob;
mod filter_trait;
mod filters;


use serenity::{Client, framework, client::{EventHandler, Context}, model::prelude::*};
use log::*;
use filters::{regex_filter,ping_filter};
use crate::filter_trait::FilterTrait;

#[tokio::main]
async fn main() {
    let filters = {
        //sadly this seems the cleanest way to initialize this
        let mut tmp:Vec<Box<dyn FilterTrait>> = Vec::new();
        //such nested method calls it's disgusting
        tmp.push(Box::new(ping_filter::EveryonePingFilter
                .chain(regex_filter::RegexFilter(
                        regex::Regex::new("http[s]?://(?:[a-zA-Z]|[0-9]|[$-_@.&+]|)+\\.([a-zA-Z]+)")
                        .expect("Regex failed to compile unexpectedly")))));
        tmp
    };

    env_logger::init();
    debug!("Logger initialized");
    let config = utils::get_config();
    debug!("Config gotten");
    let client = Client::builder(config.token)
        .framework(framework::StandardFramework::new())
        //using default value temporarily
        .event_handler(Handler::new(filters));
    client.await.expect("Client build failed").start().await.expect("client start failed");
}

/// The event handler
struct Handler{
    filters: Vec<Box<dyn FilterTrait>>
}

impl Handler {
    fn new(filters:Vec<Box<dyn FilterTrait>>)->Self{
        Handler{filters}
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _context:Context, _ready:Ready){
        info!("Bot is ready");
    }
    async fn message(&self, context: Context, message:Message){
        info!("Message: {}\t[#{}]({}): {}",
            message.guild(&context).await.expect("The guild or message vanished as we queried it").name,
            message.channel(&context).await.unwrap().guild().unwrap().name,
            message.author.name,
            message.content);
        for filter in &self.filters{
            if filter.should_act(&message, &context){
                match message.delete(&context).await{
                    Ok(_)=>{},
                    Err(_)=>break
                }
                debug!("deleting message '{}' by user '{}'",message.content, message.member(&context).await.unwrap().display_name());
            }
        }
    }
}
