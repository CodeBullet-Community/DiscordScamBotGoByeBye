extern crate serenity;
use serenity::{client::Context, model::channel::Message};


pub trait FilterTrait where Self:Send+Sync{
    fn should_act(&self, message:&Message ,context:&Context)->bool;
    //don't know why 'static is needed so that could be a problem in the future
    fn chain<T: 'static>(self, other:T)->ChainFilter where T:FilterTrait, Self:'static + Sized{
        return ChainFilter(Box::new(self),Box::new(other))
    }
}
pub struct ChainFilter(Box<dyn FilterTrait>, Box<dyn FilterTrait>);
impl FilterTrait for ChainFilter {
    fn should_act(&self, message:&Message, context:&Context) ->bool {
        return self.0.should_act(message, context) && self.1.should_act(message, context)
    }
}
impl<T> FilterTrait for T where T:Fn(&Message,&Context)->bool + Send + Sync{
    fn should_act(&self, message:&Message, context:&Context) ->bool {
        self(message,context)
    }
}
