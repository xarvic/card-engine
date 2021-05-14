use crate::state::State;
use crate::player::{Player, Team, TeamID, PlayerID};
use crate::card::{Stack, StackID, CardID, ViewPermission, Card};
use std::collections::{HashSet, HashMap, BTreeMap};
use std::any::Any;

pub trait Context {
    type Card: Card;
    type Event;

    fn add_player(&mut self) -> PlayerID;

}

pub struct GameContext<C: Context> {
    players: BTreeMap<PlayerID, Player>,
    player_order: Vec<PlayerID>,
    teams: HashMap<TeamID, Team>,
    stacks: HashMap<StackID, Stack<C>>,
    data: C,
}

impl<C: Card> GameContext<C> {
    pub fn new(data: C) -> Self {
        GameContext {
            players: BTreeMap::new(),
            player_order: Vec::new(),
            teams: HashMap::new(),
            stacks: HashMap::new(),
            data,
        }
    }

    pub fn contains_player(&self, team: TeamID, player: PlayerID) -> bool {
        self.team(team).contains(player)
    }

    pub fn contains_card(&self, stack: StackID, card: CardID) -> bool {
        unimplemented!()
    }

    pub fn size(&self, stack: StackID) -> usize {
        self.stack(stack).content.len()
    }

    pub fn owner_of(&self, player: PlayerID, stack: StackID) -> bool {
        self.stack(stack).owner.map_or(false, |team|self.contains_player(team.0, player))
    }

    pub fn view_permission_for(&self, player: PlayerID, stack: StackID) -> ViewPermission {
        let stack = self.stack(stack);
        stack.owner.and_then(|(team, view_permission)| { if self.contains_player(team, player) {
            Some(view_permission)
        } else {
            None
        }}).unwrap_or(stack.view_permission)
    }

    pub fn add_team(&mut self, name: &str, players: HashSet<PlayerID>) -> TeamID {
        let team_id = TeamID::new();
        self.teams.insert(team_id, Team::new(name, players));
        team_id
    }

    pub fn team(&self, team: TeamID) -> &Team {
        &self.teams[&team]
    }

    pub fn team_mut(&mut self, team: TeamID) -> &mut Team {
        self.teams.get_mut(&team).unwrap()
    }

    pub fn add_player(&mut self, name: &str) -> PlayerID {
        let id = PlayerID::new();
        self.players.insert(id, Player::new(name));
        self.player_order.push(id);

        // Create a team for this specific player
        let mut this = HashSet::new();
        this.insert(id);
        self.teams.insert(id.into(), Team::new(name, this));

        id
    }

    pub fn player(&self, player: PlayerID) -> &Player {
        &self.players[&player]
    }

    pub fn player_mut(&mut self, player: PlayerID) -> &mut Player {
        self.players.get_mut(&player).unwrap()
    }

    pub fn next(&self, player: PlayerID) -> PlayerID {
        let index = self.player_order.iter().position(|p|*p == player).unwrap();
        self.player_order[(index + 1) % self.player_order.len()]
    }

    pub fn previous(&self, player: PlayerID) -> PlayerID {
        let index = self.player_order.iter().position(|p|*p == player).unwrap();
        self.player_order[(index - 1 + self.player_order.len()) % self.player_order.len()]
    }

    pub fn add_stack(&mut self, cards: Vec<C>, owner: impl Into<Option<TeamID>>) -> StackID {
        let id = StackID::new();
        self.stacks.insert(id, Stack::new(owner.into(), cards));

        id
    }

    pub fn remove_stack(&mut self, stack: StackID) {
        self.stacks.remove(&stack);
    }

    pub fn stack(&self, stack: StackID) -> &Stack<E, C> {
        &self.stacks[&stack]
    }

    pub fn stack_mut(&mut self, stack: StackID) -> &mut Stack<E, C> {
        self.stacks.get_mut(&stack).unwrap()
    }

    pub fn data(&self) -> &D {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut D {
        &mut self.data
    }
}