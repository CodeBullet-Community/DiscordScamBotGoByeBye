extern crate serenity;
extern crate futures;

use crate::filter_trait::FilterTrait;
use futures::executor;
use log::*;
///filter that selects for discord bots based on what the api says
pub struct BotFilter;

impl FilterTrait for BotFilter{
    fn should_act(&self, message:&serenity::model::channel::Message, context:&serenity::client::Context) -> bool {
        let ret = executor::block_on(message.member(context)).map(|mem|mem.user.bot).unwrap_or(false);
        trace!("is bot {}", ret);
        ret
    }
}
