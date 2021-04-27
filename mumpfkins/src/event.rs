use engine_core::{CardID, PlayerID};

#[derive(Copy, Clone, Debug, Hash, Eq PartialEq)]
pub enum MumpfkinsEvent {
    EnterDoor,
    TakeCard,
    Provoke{monster: CardID},
    Equip{equipment: CardID},
    Unequip{equipment: CardID},
    PlayEffect{effect: CardID, player: PlayerID},
    SellCard{card: CardID},
    ConfirmSell,
    CancelSell,
    DonateCard{card: CardID, player: PlayerID},
    EndTurn,

    CreateOffer,
    OfferedTreasures{treasures: i32},
    AskPlayer {player: PlayerID},
    SendOffer{player: PlayerID, treasures: usize},
    AcceptOffer,
    RejectOffer,

    RollDice,
}