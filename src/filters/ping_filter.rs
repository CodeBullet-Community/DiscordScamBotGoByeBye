extern crate serenity;

use serenity::{model::channel::Message, client::Context};

use crate::filter_trait::FilterTrait;
pub struct EveryonePingFilter;

impl FilterTrait for EveryonePingFilter {
    fn should_delete(&self, message:Message, context:&Context)->bool{
        return message.content.contains("@everyone")
    }
}
