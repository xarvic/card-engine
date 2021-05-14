use engine_core::{PlayerID, State, Scene};
use crate::states::{MumpfkinsData, MumpfkinsContext, MumpfkinsState};
use crate::event::MumpfkinsEvent;
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
    type Context = MumpfkinsData;

    fn update(self: Box<Self>, context: &mut MumpfkinsContext, event: MumpfkinsEvent) -> Box<MumpfkinsState> {
        match event {
            MumpfkinsEvent::EnterDoor => {
                context.card_stack_mut()


                Provoke::new(self.player)
            },
            _ => self,
        }
    }

    fn show(&self, context: &MumpfkinsContext, scene: &mut Scene) {
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
    type Context = MumpfkinsData;



    fn update(self: Box<Self>, context: &mut MumpfkinsContext, event: MumpfkinsEvent) -> Box<MumpfKinsState> {
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

    fn show(&self, context: &MumpfkinsContext, scene: &mut Scene) {
        unimplemented!()
    }
}