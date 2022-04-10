extern crate serenity;
use serenity::{client::Context, model::channel::Message};

pub trait FilterTrait where Self:Send+Sync{
    fn should_act(&self, message:&Message ,context:&Context)->bool;
    //don't know why 'static is needed so that could be a problem in the future
    fn chain<T: 'static>(self, other:T)->ChainFilter where T:FilterTrait, Self:'static + Sized{
        return ChainFilter(Box::new(self),Box::new(other))
    }
    fn negate(self)->NegateFilter<Self> where Self:'static +Sized{
        return NegateFilter(self)
    }
}
//in hindsight these should be generics not trait objects
pub struct ChainFilter(Box<dyn FilterTrait>, Box<dyn FilterTrait>);
impl FilterTrait for ChainFilter {
    fn should_act(&self, message:&Message, context:&Context) ->bool {
        return self.0.should_act(message, context) && self.1.should_act(message, context)
    }
}
pub struct NegateFilter<T>(T) where T:FilterTrait;
impl<T> FilterTrait for NegateFilter<T> where T:FilterTrait{
    fn should_act(&self, message:&Message, context:&Context) ->bool {
        !self.0.should_act(message, context)
    }
}
//this may be dumb
impl<T> FilterTrait for T where T:Fn(&Message,&Context)->bool + Send + Sync{
    fn should_act(&self, message:&Message, context:&Context) ->bool {
        self(message,context)
    }
}
