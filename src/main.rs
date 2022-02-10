extern crate serenity;
extern crate tokio;
extern crate serde;
extern crate log;
extern crate env_logger;
extern crate regex;
extern crate lazy_static;
mod utils;
use regex::Regex;
use serenity::{Client, framework, client::{EventHandler, Context}, model::{channel::Message, prelude::Ready}, Error};
use log::{info,error};
use lazy_static::lazy_static;

lazy_static!{
    static ref LINK_REGEX:Regex = Regex::new("http[s]?://(?:[a-zA-Z]|[0-9]|[$-_@.&+]|)+\\.([a-zA-Z]+)").expect("Regex failed to compile unexpectedly");
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = utils::get_config();
    let client = Client::builder(config.token)
        .framework(framework::StandardFramework::new())
        .event_handler(Handler);
    client.await.expect("Client build failed").start().await.expect("client start failed");
}

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _context:Context, _ready:Ready){
        info!("bot is ready");
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
                error!("got error calculating spam probability: {}", e);
                return
            }
        };
        if prob > 0.5 {
            match message.delete(context).await {
                Ok(_)=>{},
                Err(err)=> error!("Error on message delete(due to spam): {}",err),
            }
        }
    }
}
//I don't like how I'm returning an f64 instead of a value restricted to 0 <= ret <= 1 so I may do
//something about that in the future
//note I'm intending on having the handled at least in part by a naive bayes classifier which is
//why a probability is returned
// context requirement should only be temporary for this
/// Currently a reimplementation of the naive regex algo that was in the original aaaaaaaaaaaa
async fn calc_spam_probability(message:&Message,context:&Context)->Result<f64,Error> {

    let author_mod = match message.guild(context).await.ok_or("Missing Guild").unwrap().role_by_name("Moderator"){
        Some(role)=>message.author.has_role(context,
            message.guild(context).await.ok_or("Missing Guild").unwrap().id,
            role).await?,
        None => false
    };

    if message.author.bot {
        Ok(0.0)
    }
    else if message.content.contains("@everyone") 
        && LINK_REGEX.is_match(message.content.as_str())
        && author_mod{
        Ok(1.0)
    }
    else {Ok(0.0)}
}
