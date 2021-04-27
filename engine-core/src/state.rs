use crate::context::Context;
use crate::scene::Scene;

pub trait State {
    type Event: Clone;
    type Card: Clone;

    fn update(self: Box<Self>, context: &mut Context<Self::Event, Self::Card>, event: Self::Event) -> Box<dyn State<Event= Self::Event, Card = Self::Card>>;

    fn show(&self, context: &Context<Self::Event, Self::Card>, scene: &mut Scene);
}