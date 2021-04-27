use std::collections::HashMap;

pub struct Equipment {
    name: String,
    location: Option<EquipmentLocation>,
    bonus: i32,
    value: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum EquipmentLocation {
    Head,
    Foot,
    Legs,
    Body,
    Hand,
    BothHands,
}

impl EquipmentLocation {
    pub fn overlap(&self, other: Self) -> bool {
        match (self, other) {
            (Self::Hand, Self::BothHands) |
            (Self::BothHands, Self::Hand) => true,
            (x, y) => x == y,
        }
    }
}

pub struct PlayerEquipment {
    pieces: HashMap<CardID, Equipment>,
}

impl PlayerEquipment {
    pub fn new() -> Self {
        PlayerEquipment {
            pieces: HashMap::new()
        }
    }

    pub fn is_free(&self, location: EquipmentLocation) -> bool {
        let mut first_hand_free = true;
        for (_, eq) in self.pieces.iter() {
            if eq.overlap(location) && (location != EquipmentLocation::Hand || !first_hand_free) {
                return false;
            }
            if eq.overlap(EquipmentLocation::Hand) {
                first_hand_free = false;
            }
        }
        true
    }

    pub fn clear(&mut self) {
        self.pieces.clear();
    }

    pub fn clear_location(&mut self, location: EquipmentLocation) {
        self.pieces.retain(|k, v|!v.overlap(location))
    }

    pub fn clear_card(&mut self, card: CardID) {
        self.pieces.remove(&card);
    }

    pub fn insert_equipment(&mut self, card: CardID, equipment: Equipment) {
        if equipment.location.map_or(true, |location|self.is_free(location)) {
            self.pieces.insert(card, equipment);
        }
    }

    pub fn strength(&self) -> i32 {
        self.pieces.iter().map(|(_, eq)|eq.bonus).sum()
    }
}