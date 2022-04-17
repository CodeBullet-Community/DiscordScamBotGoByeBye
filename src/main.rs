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
use filters::{regex_filter,ping_filter,role_filter, bot_filter};
use crate::filter_trait::FilterTrait;

#[tokio::main]
async fn main() {
    let filter = ping_filter::EveryonePingFilter
        .and(regex_filter::RegexFilter(
            regex::Regex::new("http[s]?://(?:[a-zA-Z]|[0-9]|[$-_@.&+]|)+\\.([a-zA-Z]+)")
            .expect("Regex failed to compile unexpectedly")))
        .and(role_filter::RoleFilter::from("Moderator").negate())
        .and(bot_filter::BotFilter.negate());

    env_logger::init();
    debug!("Logger initialized");
    let config = utils::get_config();
    debug!("Config gotten");
    let client = Client::builder(config.token)
        .framework(framework::StandardFramework::new())
        //using default value temporarily
        .event_handler(Handler::new(filter));
    client.await.expect("Client build failed").start().await.expect("client start failed");
}

/// The event handler
struct Handler<T>where T:FilterTrait{
    filter: T
}

impl<T> Handler<T> where T:FilterTrait {
    fn new(filter:T)->Self{
        Handler{filter}
    }
}

#[serenity::async_trait]
impl<T> EventHandler for Handler<T> where T:FilterTrait {
    async fn ready(&self, _context:Context, _ready:Ready){
        info!("Bot is ready");
    }
    async fn message(&self, context: Context, message:Message){
        info!("Message: {}\t[#{}]({}): {}",
            message.guild(&context).await.expect("The guild or message vanished as we queried it").name,
            message.channel(&context).await.unwrap().guild().unwrap().name,
            message.author.name,
            message.content);
        if self.filter.should_act(&message, &context){
            match message.delete(&context).await{
                //we don't care if result is okay or err
                Ok(_)=>{},
                Err(_)=>{}
            }
            debug!("deleting message '{}' by user '{}'",message.content, message.member(&context).await.unwrap().display_name());
        }
    }
}
