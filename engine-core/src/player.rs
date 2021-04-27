use std::num::NonZeroU64;
use crate::util::{Counter, AnyID};
use std::collections::HashSet;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct PlayerID(TeamID);

impl PlayerID {
    pub fn new() -> Self {
        PlayerID(TeamID::new())
    }

    pub fn raw(&self) -> u64 {
        self.0.raw()
    }

    pub fn team(&self) -> TeamID {
        self.0
    }

    pub fn any(&self) -> AnyID {
        self.0.any()
    }
}

pub struct Player {
    name: String,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TeamID(AnyID);

impl TeamID {
    pub fn new() -> Self {
        Self(AnyID::new())
    }

    pub fn raw(&self) -> u64 {
        self.0.raw()
    }

    pub fn any(&self) -> AnyID {
        self.0
    }
}

impl From<PlayerID> for TeamID {
    fn from(id: PlayerID) -> Self {
        TeamID(id.0)
    }
}

pub struct Team {
    name: String,
    members: HashSet<PlayerID>,
}

impl Team {
    pub fn new(name: &str, initial_members: HashSet<PlayerID>) -> Self {
        Team {
            name: name.to_string(),
            members: initial_members,
        }
    }

    pub fn contains(&self, player: PlayerID) -> bool {
        self.members.contains(&player)
    }
}