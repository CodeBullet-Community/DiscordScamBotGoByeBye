extern crate serenity;
use serenity::{client::Context, model::channel::Message};

//split into 2 traits 1 &self one &mut self after dealing with dumb trait objects
pub trait FilterTrait where Self:Send+Sync{
    fn should_act(&self, message:&Message ,context:&Context)->bool;
    //don't know why 'static is needed so that could be a problem in the future
    fn and<T: 'static>(self, other:T)->AndFilter where T:FilterTrait, Self:'static + Sized{
        return AndFilter(Box::new(self),Box::new(other))
    }
    fn negate(self)->NegateFilter<Self> where Self:Sized{
        return NegateFilter(self)
    }
    fn or<T>(self, other:T)->OrFilter<Self,T> where T:FilterTrait, Self:Sized{
        return OrFilter(self, other)
    }

}
//in hindsight these should be generics not trait objects
pub struct AndFilter(Box<dyn FilterTrait>, Box<dyn FilterTrait>);
impl FilterTrait for AndFilter {
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

pub struct OrFilter<T,U>(T,U) where T:FilterTrait, U:FilterTrait;
impl<T,U> FilterTrait for OrFilter<T,U> where T:FilterTrait, U:FilterTrait{
    fn should_act(&self, message:&Message, context:&Context) ->bool {
        self.0.should_act(message, context) || self.1.should_act(message, context)
    }
}
//this may be dumb
impl<T> FilterTrait for T where T:Fn(&Message,&Context)->bool + Send + Sync{
    fn should_act(&self, message:&Message, context:&Context) ->bool {
        self(message,context)
    }
}
