/// rule 300.1
pub enum CardType {
    Artifact,
    Conspiracy,
    Creature,
    Dungeon,
    Enchantment,
    Instant,
    Land,
    Phenomenon,
    Plane,
    Planeswalker,
    Scheme,
    Sorcery,
    Tribal,
    Vanguard,
}

/// rule 109.3
pub struct ObjectCharacteristic {
    pub name: String,
    pub mana_cost: String,
    pub color: String,
    pub color_indicator: String,
    pub card_type: String,
    pub subtype: String,
    pub supertype: String,
    pub rules_text: String,
    pub abilities: String,
    pub power: i32,
    pub toughness: i32,
    pub loyalty: i32,
    pub hand_modifier: i32,
    pub life_modifier: i32,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
