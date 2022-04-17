extern crate serenity;

use serenity::{model::channel::Message, client::Context};
use crate::filter_trait::FilterTrait;
use log::*;
///filter that selects for messages which contain an everyone ping
pub struct EveryonePingFilter;

impl FilterTrait for EveryonePingFilter {
    fn should_act(&self, message:&Message, _context:&Context)->bool{
        trace!("Does the message mention everyone? {}", message.mention_everyone);
        return message.mention_everyone// .content.contains("@everyone")
    }
}
