use std::num::NonZeroU64;
use crate::util::Counter;
use std::collections::HashSet;

// Players and teams use the same ID counter therefore we can convert a playerID into a new unique teamID.
static IDS: Counter = Counter::new();

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct PlayerID(NonZeroU64);

impl PlayerID {
    pub fn new() -> Self {
        PlayerID(IDS.next())
    }

    pub fn raw(&self) -> u64 {
        self.0.get()
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
pub struct TeamID(NonZeroU64);

impl TeamID {
    pub fn new() -> Self {
        TeamID(IDS.next())
    }

    pub fn raw(&self) -> u64 {
        self.0.get()
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