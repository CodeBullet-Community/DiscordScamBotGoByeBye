extern crate serenity;
extern crate async_trait;

use serenity::{client::Context, model::channel::Message};

//TODO:split into 2 traits 1 &self one &mut self after dealing with dumb trait objects
#[async_trait::async_trait]
pub trait FilterTrait where Self:Send+Sync{
    async fn should_act(&self, message:&Message ,context:&Context)->bool;
    //don't know why 'static is needed so that could be a problem in the future
    fn and<T>(self, other:T)->AndFilter<Self,T> where T:FilterTrait, Self:Sized{
        return AndFilter(self,other)
    }
    fn negate(self)->NegateFilter<Self> where Self:Sized{
        return NegateFilter(self)
    }
    fn or<T>(self, other:T)->OrFilter<Self,T> where T:FilterTrait, Self:Sized{
        return OrFilter(self, other)
    }
    fn xor<T>(self, other:T)->XorFilter<Self,T> where T:FilterTrait, Self:Sized{
        return XorFilter(self, other)
    }
}

pub struct AndFilter<T,U>(T, U) where T:FilterTrait, U:FilterTrait;
#[async_trait::async_trait]
impl<T,U> FilterTrait for AndFilter<T,U> where T:FilterTrait, U:FilterTrait {
    async fn should_act(&self, message:&Message, context:&Context) ->bool {
        return self.0.should_act(message, context).await && self.1.should_act(message, context).await
    }
}

//unneeded and enables less readable code but I can't help myself
//impl

pub struct NegateFilter<T>(T) where T:FilterTrait;
#[async_trait::async_trait]
impl<T> FilterTrait for NegateFilter<T> where T:FilterTrait{
    async fn should_act(&self, message:&Message, context:&Context) ->bool {
        !self.0.should_act(message, context).await
    }
}

pub struct OrFilter<T,U>(T,U) where T:FilterTrait, U:FilterTrait;
#[async_trait::async_trait]
impl<T,U> FilterTrait for OrFilter<T,U> where T:FilterTrait, U:FilterTrait{
    async fn should_act(&self, message:&Message, context:&Context) ->bool {
        self.0.should_act(message, context).await || self.1.should_act(message, context).await
    }
}

pub struct XorFilter<T,U>(T,U) where T:FilterTrait, U:FilterTrait;
#[async_trait::async_trait]
impl<T,U> FilterTrait for XorFilter<T,U> where T:FilterTrait, U:FilterTrait{
    async fn should_act(&self, message:&Message, context:&Context) ->bool {
        self.0.should_act(message, context).await ^ self.1.should_act(message, context).await
    }
}

//this may be dumb but I don't think it is
#[async_trait::async_trait]
impl<T> FilterTrait for T where T:Fn(&Message,&Context)->bool + Send + Sync{
    async fn should_act(&self, message:&Message, context:&Context) ->bool {
        self(message,context)
    }
}

