extern crate serenity;

use serenity::{model::channel::Message, client::Context};
use crate::filter_trait::FilterTrait;

///filter that selects for messages which contain an everyone ping
pub struct EveryonePingFilter;

impl FilterTrait for EveryonePingFilter {
    fn should_act(&self, message:&Message, _context:&Context)->bool{
        return message.mention_everyone// .content.contains("@everyone")
    }
}
