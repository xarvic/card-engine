use crate::context::Context;
use crate::scene::Scene;

pub trait State {
    type Event: Clone;

    fn update(self: Box<Self>, context: &mut Context<Self::Event>, event: Self::Event) -> Box<dyn State<Event= Self::Event>>;

    fn show(&self, context: &Context<Self::Event>, scene: &mut Scene);
}