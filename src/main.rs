extern crate serenity;
extern crate tokio;
extern crate serde;
extern crate log;
extern crate env_logger;
mod utils;
use serenity::{Client, framework, client::{EventHandler, Context}, model::channel::Message};
use log::error;


#[tokio::main]
async fn main() {
    env_logger::init();
    let config = utils::get_config();
    let client = Client::builder(std::env::var("DISCORD_TOKEN").expect("no env var DISCORD_TOKEN"))
        .framework(framework::StandardFramework::new());
}

struct Handler {}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message:Message){
        let prob = calc_spam_probability(&message).await;
        if prob >= 1.0 {
            match message.delete(context).await {
                Ok(_)=>{},
                Err(err)=> error!("Error on message delete(due to spam): {}",err),
            }
        }
    }
}
//I don't like how I'm returning an f64 instead of a value restricted to 0 <= ret <= 1 so I may do
//something about that in the future
async fn calc_spam_probability(message:&Message)->f64 {
    0.0
}
