use engine_core::{PlayerID, State, Context, Scene}
use std::alloc::Global;
use crate::event::MumpfkinsEvent;
use crate::cards::MumpfkinsCard;
use crate::states::end_turn::EndTurn;
use crate::states::fight::Fight;

pub struct EnterDoor {
    player: PlayerID
}

impl EnterDoor {
    pub fn new(player: PlayerID) -> Box<Self> {
        Box::new(EnterDoor {
            player
        })
    }
}

impl State for EnterDoor {
    type Event = MumpfkinsEvent;
    type Card = MumpfkinsCard;

    fn update(self: Box<Self>, context: &mut Context<Self::Event, Self::Card>, event: MumpfkinsEvent) -> Box<dyn State<Event=Self::Event, Card=Self::Card>> {
        match event {
            MumpfkinsEvent::EnterDoor => Provoke::new(self.player),
            _ => self,
        }
    }

    fn show(&self, context: &Context<Self::Event, Self::Card>, scene: &mut Scene) {
        unimplemented!()
    }
}

pub struct Provoke {
    player: PlayerID
}

impl Provoke {
    pub fn new(player: PlayerID) -> Box<Self> {
        Box::new(Provoke {
            player
        })
    }
}

impl State for Provoke {
    type Event = MumpfkinsEvent;
    type Card = MumpfkinsCard;

    fn update(self: Box<Self>, context: &mut Context<Self::Event, Self::Card>, event: MumpfkinsEvent) -> Box<dyn State<Event=Self::Event, Card=Self::Card>> {
        match event {
            MumpfkinsEvent::TakeCard => {
                EndTurn::new(self.player)
            },
            MumpfkinsEvent::Provoke {monster} => {
                Fight::normal(self.player, monster)
            },
            _ => self
        }
    }

    fn show(&self, context: &Context<Self::Event, Self::Card>, scene: &mut Scene) {
        unimplemented!()
    }
}