use crate::states::fight::Fight;
use engine_core::PlayerID;

struct CreateOffer {
    fight: Box<Fight>,

    asking: PlayerID,
    offering: i32,
}

struct AwaitOffer {
    fight: Box<Fight>,

    asking: PlayerID,
    offering: i32,
}