use std::num::NonZeroU64;
use crate::util::{Counter, AnyID};
use std::sync::Arc;
use crate::player::{TeamID, PlayerID};
use crate::context::{GameContext, Context};

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CardID(AnyID);

impl CardID {
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

pub trait Card {
    pub fn id(&self) -> CardID;
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct StackID(AnyID);

impl StackID {
    pub fn new() -> Self {
        StackID(AnyID::new())
    }

    pub fn raw(&self) -> u64 {
        self.0.raw()
    }

    pub fn any(&self) -> AnyID {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ViewPermission {
    /// The player is allowed to see every Card in the stack
    Show,
    /// The player is allowed to see the topmost Card.
    ShowTopmost,
    /// The player is allowed the know the remaining cards in this stack
    ShowSize,
    /// The player has no information about this stack.
    Hide
}

pub struct Stack<C: Context> {
    pub(crate) owner: Option<(TeamID, ViewPermission)>,
    pub(crate) view_permission: ViewPermission,
    pub(crate) content: Vec<C>,
    on_action: Option<Box<dyn Fn(&GameContext<C>, PlayerID) -> Option<E>>>,
    on_drag_card: Option<Box<dyn Fn(&GameContext<C>, PlayerID, CardID, StackID) -> Option<E>>>,
}

impl<C: Card> Stack<C> {
    pub fn new(owner: Option<TeamID>, content: Vec<C>) -> Self {
        Self {
            owner: owner.map(|x|(x, ViewPermission::Show)),
            view_permission: ViewPermission::Show,
            content,
            on_action: None,
            on_drag_card: None
        }
    }

    pub fn push(&mut self, card: C) {
        self.content.push(card)
    }

    pub fn pop(&mut self) -> Option<C> {
        self.content.pop()
    }
}