use engine_core::{PlayerID, AnyID};
use crate::cards::Monster;

pub struct Fight {
    fighter: Fighter,
    helper: Option<Helper>,
    enemies: Vec<Enemy>,

    current_turn: PlayerID,
    last_action: PlayerID,
    current: PlayerID,
}

impl Fight {
    pub fn normal(player: PlayerID, monster: Monster) -> Box<Self> {
        Box::new(Self {
            fighter: Fighter::new(player),
            helper: None,
            enemies: vec![Enemy::new(monster)]

            current_turn: player,
            last_action: player,
            current: player,
        })
    }
}

struct Fighter {
    player: PlayerID,
    strength_change: i32,
}

impl Fighter {
    pub fn new(player: PlayerID) -> Self {
        Self {
            player,
            strength_change: 0,
        }
    }
}

struct Helper {
    fighter: Fighter,
    treasures: i32,
}

impl Helper {

}

struct Enemy {
    monster: Monster,
    strength_change: i32,
    treasure_change: i32,
    id: AnyID,
}

impl Enemy {
    pub fn new(monster: Monster) -> Self {
        Enemy {
            monster,
            strength_change: 0,
            treasure_change: 0,
            id: AnyID::new(),
        }
    }
}

pub struct FightLost {
    current: Fighter,
    other: Option<Fighter>,

    enemies: Vec<Enemy>,
}