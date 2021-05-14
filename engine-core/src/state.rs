use crate::context::{GameContext, Context};
use crate::scene::Scene;
use crate::card::Card;

pub trait State {
    type Context: Context;

    fn update(
        self: Box<Self>,
        context: &mut GameContext<Self::Context>,
        event: Self::Event
    ) -> Box<dyn State<Context = Self::Context>>;

    fn show(&self, context: &GameContext<Self::Context>, scene: &mut Scene);
}