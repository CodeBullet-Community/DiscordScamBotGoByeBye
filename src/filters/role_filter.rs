extern crate serenity;
extern crate futures;

use crate::filter_trait::FilterTrait;
use futures::executor;
use log::*;
///filter that selects for messages from a member with a particular role
pub struct RoleFilter{role_name:String}

impl FilterTrait for RoleFilter{
    fn should_act(&self, message:&serenity::model::channel::Message, context:&serenity::client::Context) -> bool {
        if let Some(v) = executor::block_on(message.guild(context)){
            let ret = v.clone().role_by_name(self.role_name.as_str()).clone().as_ref().is_some();
            trace!("is role {} had? {}",self.role_name,ret);
            return ret
        }
        false
    }
}
impl From<&str> for RoleFilter {
    fn from(s: &str) -> Self {
        RoleFilter { role_name: s.to_string() }
    }
}
