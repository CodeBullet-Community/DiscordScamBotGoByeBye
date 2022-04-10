extern crate regex;
extern crate serenity;

use crate::filter_trait::FilterTrait;
use regex::Regex;
use serenity::{model::channel::Message, client::Context};

//the fact I have to stick a pub inside of here is silly
pub struct RegexFilter(pub Regex);

impl FilterTrait for RegexFilter {
    fn should_delete(&self, message:Message, context:&Context)->bool{
        return self.0.is_match(message.content.as_str())
    }
}
