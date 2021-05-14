mod equipment;
mod monster;
mod effect;

pub use std::sync::Arc;
pub use crate::cards::equipment::{Equipment, EquipmentLocation, PlayerEquipment};
pub use crate::cards::monster::Monster;
pub use crate::cards::effect::{Effect, FightEffect};
use engine_core::Card;

pub enum MumpfkinsCard {
    Equipment(Equipment),
    Monster(Monster),
    Effect(Effect),
    FightEffect(FightEffect),
}

impl Card for MumpfkinsCard {

}