#[cfg(test)]
mod tests;
extern crate log;
extern crate serenity;
extern crate lazy_static;
extern crate regex;
use log::*;
use serenity::{Error, client::Context, model::prelude::*};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
    static ref LINK_REGEX:Regex = Regex::new("http[s]?://(?:[a-zA-Z]|[0-9]|[$-_@.&+]|)+\\.([a-zA-Z]+)").expect("Regex failed to compile unexpectedly");
}

//I don't like how I'm returning an f64 instead of a value restricted to 0 <= ret <= 1 so I may do
//something about that in the future
//note I'm intending on having the handled at least in part by a naive bayes classifier which is
//why a probability is returned
// probably want to create a custom error type so ? can be used for all results not just serenity
// results
/// Currently a reimplementation of the naive regex algo that was in the original aaaaaaaaaaaa
pub async fn calc_spam_probability(message:&Message,context:&Context)->Result<f64,Error> {
    //naive method of determining modmin status but it's fine for now
    let author_mod = match message.guild(context).await.ok_or("Missing Guild").unwrap().role_by_name("Moderator"){
        Some(role)=>message.author.has_role(context,
            message.guild(context).await.ok_or("Missing Guild").unwrap().id,
            role).await?,
        None => false
    };

    if message.author.bot {
        trace!("{} is a bot",message.author);
        Ok(0.0)
    }
    else if author_mod {
        trace!("{} is a mod on {}",message.author,
            message.guild(context).await.ok_or("Missing Guild").unwrap().name);
        Ok(0.0)
    }
    else { 
        trace!("Basic user checks complete {} is not a mod or bot on {}",message.author.name, 
            message.guild(context).await.ok_or("Missing Guild").unwrap().name);
        let probability = calc_content_spam_prob(message.content.as_str());
        if probability>0.9{
            debug!("\"{}\" from {} has been determined to be a spam message",message.content,message.author.name);
        }
        Ok(probability)
    }
}
pub fn calc_content_spam_prob(msg:&str)->f64 {
        if msg.contains("@everyone") 
            && LINK_REGEX.is_match(msg){
            1.0
        }
        else {0.0}
    
}
