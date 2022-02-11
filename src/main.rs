extern crate serenity;
extern crate tokio;
extern crate serde;
extern crate log;
extern crate env_logger;
mod utils;
mod prob;

use serenity::{Client, framework, client::{EventHandler, Context}, model::{channel::Message, prelude::Ready}};
use log::*;
use prob::calc_spam_probability;

#[tokio::main]
async fn main() {
    env_logger::init();
    debug!("Logger initialized");
    let config = utils::get_config();
    debug!("Config gotten");
    let client = Client::builder(config.token)
        .framework(framework::StandardFramework::new())
        .event_handler(Handler);
    client.await.expect("Client build failed").start().await.expect("client start failed");
}

struct Handler;

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
        
        let prob:f64 = match calc_spam_probability(&message,&context).await{
            Ok(v)=>v,
            Err(e)=>{
                error!("got error calculating spam probability: {} with message {}[{}]({}):{}", e,
                    message.guild(&context).await.expect("The guild or message vanished as we queried it").name,
                    message.channel(&context).await.unwrap().guild().unwrap().name,
                    message.author.name,
                    message.content);
                return
            }
        };
        if prob > 0.5 {
            match message.delete(context).await {
                Ok(_)=>{},
                Err(err)=> error!("Error on message delete: {}",err),
            }
        }
    }
}
