extern crate serenity;
extern crate futures;

use crate::filter_trait::FilterTrait;
use futures::executor;
use log::*;
///filter that selects for messages from a member with a particular role
pub struct RoleFilter{role_name:String}

impl FilterTrait for RoleFilter{
    fn should_act(&self, message:&serenity::model::channel::Message, context:&serenity::client::Context) -> bool {
        if let Some(guild) = executor::block_on(message.guild(context)){
            let ret = if let Some(role) = guild.clone().role_by_name(self.role_name.as_str()).clone().as_ref(){
                let has_role_monad = message.member.clone().map(|mem|mem.user.map(|u| executor::block_on(u.has_role(context, guild.id, role.id)))).flatten();
                //annoyed I can't use flatten for this
                has_role_monad.ok_or(serenity::prelude::SerenityError::Other("hush")).unwrap_or(Ok(false)).unwrap_or(false)
            }
            else{
                false
            };
            trace!("is role {} had? {}",self.role_name,ret);
            return ret
        }
        return false
    }
}
impl From<&str> for RoleFilter {
    fn from(s: &str) -> Self {
        RoleFilter { role_name: s.to_string() }
    }
}
