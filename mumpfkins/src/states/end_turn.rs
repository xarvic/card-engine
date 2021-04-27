use engine_core::PlayerID;

pub struct EndTurn {
    player: PlayerID,
}

impl EndTurn {
    pub fn new(player: PlayerID) -> Box<Self> {
        Box::new(Self {
            player,
        })
    }
}