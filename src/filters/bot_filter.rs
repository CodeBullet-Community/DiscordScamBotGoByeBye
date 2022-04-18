extern crate serenity;
extern crate futures;

use crate::filter_trait::FilterTrait;
use log::*;
///filter that selects for discord bots based on what the api says
pub struct BotFilter;

#[async_trait::async_trait]
impl FilterTrait for BotFilter{
    async fn should_act(&self, message:&serenity::model::channel::Message, context:&serenity::client::Context) -> bool {
        let ret = message.member.clone().map(|mem|mem.user.map(|u|u.bot)).flatten().unwrap_or(false);
        trace!("is bot {}", ret);
        ret
    }
}
