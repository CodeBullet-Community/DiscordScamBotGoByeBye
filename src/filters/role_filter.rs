extern crate serenity;
extern crate futures;

use crate::filter_trait::FilterTrait;
use log::*;
///filter that selects for messages from a member with a particular role
pub struct RoleFilter{role_name:String}

#[async_trait::async_trait]
impl FilterTrait for RoleFilter{
    async fn should_act(&self, message:&serenity::model::channel::Message, context:&serenity::client::Context) -> bool {
        if let Some(guild) = message.guild(context).await{
            let ret = if let Some(role) = guild.clone().role_by_name(self.role_name.as_str()).clone().as_ref(){
                if let Ok(v) =  message.member(context).await{
                    v.roles(context).await.map(|roles|roles.contains(role)).unwrap_or(false)
                }
                else {false}
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
