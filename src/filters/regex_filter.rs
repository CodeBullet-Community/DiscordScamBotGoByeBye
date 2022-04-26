extern crate regex;
extern crate serenity;

use crate::filter_trait::FilterTrait;
use regex::Regex;
use serenity::{model::channel::Message, client::Context};
use log::*;

//the fact I have to stick a pub inside of here is silly
pub struct RegexFilter(pub Regex);

#[async_trait::async_trait]
impl FilterTrait for RegexFilter {
    async fn should_act(&self, message:&Message, _context:&Context)->bool{
        let did_match = self.0.is_match(message.content.as_str());
        trace!("{} ||| {}? - {}",message.content, self.0.as_str(), did_match);
        return did_match
    }
}
