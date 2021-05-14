use engine_core::{GameContext, State, PlayerID, StackID, Stack, Context};
use crate::event::MumpfkinsEvent;
use crate::cards::{MumpfkinsCard, PlayerEquipment};
use std::collections::HashMap;

mod start_turn;
mod end_turn;
mod fight;
mod offer;

pub enum Gender {
    Male,
    Female,
}

pub struct MumpfkinsPlayer {
    equipment: PlayerEquipment,
    equipment_cards: StackID,

    hand_cards: StackID,
    level: u32,
    gender: Gender,

}

pub struct MumpfkinsData {
    players: HashMap<PlayerID, MumpfkinsPlayer>,

    card_stack: StackID,
    winning_level: u32,
}

impl Context for MumpfkinsData {
    type Card = MumpfkinsCard;
    type Event = MumpfkinsEvent;

    fn add_player(&mut self) -> PlayerID {
        unimplemented!()
    }
}


impl GameContext<MumpfkinsData> {
    pub fn player(&self, player: PlayerID) -> &MumpfkinsPlayer {
        &self.data().players[player]
    }

    pub fn player_mut(&mut self, player: PlayerID) -> &mut MumpfkinsPlayer {
        self.data_mut().players.get_mut(&player).unwrap()
    }

    pub fn handcards(&self, player: PlayerID) -> &Stack<MumpfkinsData> {
        self.stack(self.player(player).hand_cards)
    }

    pub fn handcards_mut(&mut self, player: PlayerID) -> &mut Stack<MumpfkinsData> {
        self.stack_mut(self.player(player).hand_cards)
    }

    pub fn card_stack(&self) -> &Stack<MumpfkinsData> {
        self.stack(self.data().card_stack)
    }

    pub fn card_stack_mut(&mut self) -> &Stack<MumpfkinsData> {
        self.stack_mut(self.data().card_stack)
    }
}

pub type MumpfkinsContext = GameContext<MumpfkinsData>;

pub type MumpfkinsState = dyn State<Context = MumpfkinsData>;