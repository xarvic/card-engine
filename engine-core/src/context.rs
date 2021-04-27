use crate::state::State;
use crate::player::{Player, Team, TeamID, PlayerID};
use crate::card::{Stack, StackID, CardID, SharedCard, ViewPermission};
use std::collections::{HashSet, HashMap, BTreeMap};
use std::any::Any;

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
pub struct StateKey<T> (&'static str, T);

impl<T> StateKey<T> {
    pub const fn new(name: &'static str, default_value: T) -> Self {
        StateKey(name, default_value)
    }
}

pub struct Context<E> {
    players: BTreeMap<PlayerID, Player>,
    player_order: Vec<PlayerID>,

    teams: HashMap<TeamID, Team>,
    team_states: HashMap<(&'static str, TeamID), Box<dyn Any>>,

    stacks: HashMap<StackID, Stack>,
}

impl<E> Context<E> {
    pub fn new(context: C) -> Self {
        Context {
            players: BTreeMap::new(),
            player_order: Vec::new(),
            teams: HashMap::new(),
            team_states: HashMap::new(),
            stacks: HashMap::new(),
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
        stack.owner.and_then(|(team, view_permission)| { if self.contains_player(team.0, player) {
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

    pub fn get_team_state<T: Clone>(&self, team: TeamID, state: StateKey<T>) -> T {
        self.team_states
            .get(&(state.0, player)).cloned()
            .unwrap_or(Box::new(state.1.clone()))
            .downcast()
            .unwrap()
    }

    pub fn update_team_state<T: Clone>(&mut self, team: TeamID, state: StateKey<T>) -> &mut T {
        self.team_states
            .entry((state.0, player))
            .or_insert(Box::new(state.1.clone()))
            .downcast_mut()
            .unwrap()
    }

    //TODO: fix me
    pub fn inspect_team_state<T>(&self, team: TeamID, state: &'static StateKey<T>) -> &T {
        self.team_states
            .get(&(state.0, team)).map(|data|data.downcast_ref().unwrap())
            .unwrap_or(&state.1)
    }


    pub fn set_team_state<T>(&mut self, team: TeamID, state: StateKey<T>, value: T) {
        self.team_states.insert((state.0, player), Box::new(value));
    }

    pub fn team(&self, team: TeamID) -> &Team {
        &self.teams[team]
    }

    pub fn team_mut(&mut self, team: TeamID) -> &mut Team {
        &mut self.teams[team]
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

    pub fn get_player_state<T: Clone>(&self, player: PlayerID, state: StateKey<T>) -> T {
        self.get_team_state(player.into(), state)
    }

    pub fn set_player_state<T>(&mut self, player: PlayerID, state: StateKey<T>, value: T) {
        self.set_team_state(player.into(), state, value)
    }

    pub fn update_player_state<T: Clone>(&mut self, player: PlayerID, state: StateKey<T>) -> &mut T {
        self.update_team_state(player.into(), state)
    }

    //TODO: fix me
    pub fn inspect_player_state<T>(&self, player: PlayerID, state: &'static StateKey<T>) -> &T {
        self.inspect_team_state(player.into(), state)
    }

    pub fn player(&self, player: PlayerID) -> &Player {
        &self.players[player]
    }

    pub fn player_mut(&mut self, player: PlayerID) -> &mut Player {
        &mut self.players[player]
    }

    pub fn next(&self, player: PlayerID) -> PlayerID {
        let index = self.player_order.iter().position(|p|*p == player);
        self.player_order[(index + 1) % self.player_order.len()]
    }

    pub fn previous(&self, player: PlayerID) -> PlayerID {
        let index = self.player_order.iter().position(|p|*p == player);
        self.player_order[(index - 1 + self.player_order.len()) % self.player_order.len()]
    }

    pub fn add_stack(&mut self, cards: Vec<SharedCard>, owner: impl Into<Option<TeamID>>) -> StackID {
        let stack = Stack {
            owner: owner.into().map(|x|(x, ViewPermission::Show)),
            view_permission: ViewPermission::Show,
            content: cards,
        };
        let id = StackID::new();

        self.stacks.insert(id, stack);

        id
    }

    pub fn remove_stack(&mut self, stack: StackID) {
        self.stacks.remove(&stack);
    }

    pub fn stack(&self, stack: StackID) -> &Stack {
        &self.stacks[stack]
    }

    pub fn stack_mut(&mut self, stack: StackID) -> &mut Stack {
        &mut self.stacks[stack]
    }
}