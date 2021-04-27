use std::num::NonZeroU64;
use crate::util::Counter;
use std::sync::Arc;
use crate::player::{TeamID, PlayerID};
use crate::context::Context;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CardID(NonZeroU64);

impl CardID {
    pub fn new() -> Self {
        static IDS: Counter = Counter::new();
        CardID(IDS.next())
    }

    pub fn raw(&self) -> u64 {
        self.0.get()
    }
}

pub trait Card {

}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct StackID(NonZeroU64);

impl StackID {
    pub fn new() -> Self {
        static IDS: Counter = Counter::new();
        StackID(IDS.next())
    }

    pub fn raw(&self) -> u64 {
        self.0.get()
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

pub struct Stack<E, C: Card> {
    pub(crate) owner: Option<(TeamID, ViewPermission)>,
    pub(crate) view_permission: ViewPermission,
    pub(crate) content: Vec<C>,
    on_action: Option<Box<dyn Fn(&Context<E, C>, PlayerID) -> Option<E>>>,
    on_drag_card: Option<Box<dyn Fn(&Context<E, C>, PlayerID, CardID, StackID) -> Option<E>>>,
}

impl<E, C: Card> Stack<E, C> {
    pub fn new(owner: Option<TeamID>, content: Vec<C>) -> Self {
        Self {
            owner: owner.map(|x|(x, ViewPermission::Show)),
            view_permission: ViewPermission::Show,
            content,
            on_action: None,
            on_drag_card: None
        }
    }
}