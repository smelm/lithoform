use std::collections::HashMap;

type PlayerID = usize;

struct Player {
    name: String,
}

impl Player {
    fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
        }
    }
}

struct Game {
    players: Vec<Player>,
    active_player: PlayerID,
}

impl Game {
    fn new() -> Game {
        Game {
            players: vec![],
            active_player: 0,
        }
    }

    fn add_player(&mut self, player: Player) -> PlayerID {
        self.players.push(player);
        return self.players.len() - 1;
    }
}

/// rule 500.1
// TODO missing from 500.1 is that these happen every round
mod Phases {
    mod Beginning {
        const UNTAP: u16 = 0;
        const UPKEEP: u16 = 1;
        const DRAW: u16 = 2;
    }
    const PRE_COMBAT_MAIN: u16 = 3;
    mod Combat {
        const BEGINNING_OF_COMBAT: u16 = 4;
        const DECLARE_ATTACKERS: u16 = 5;
        const DECLARE_BLOCKERS: u16 = 6;
        const COMBAT_DAMAGE: u16 = 7;
        const END_OF_COMBAT: u16 = 8;
    }
    const POST_COMBAT_MAIN: u16 = 9;
    mod Ending {
        const END: u16 = 10;
        const CLEANUP: u16 = 11;
    }
}

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

type Zone = Vec<ObjectCharacteristic>;

struct PlayerZones {
    library: Zone,
    hand: Zone,
    graveyard: Zone,
}

struct SharedZones {
    battlefield: Zone,
    stack: Zone,
    exile: Zone,
    command: Zone,
    ante: Zone,
}

/// 400.1
struct Zones {
    player_zones: HashMap<PlayerID, PlayerZones>,
    shared_zones: SharedZones,
}

#[cfg(test)]
mod _1_game_concepts {
    use super::*;

    mod _102_players {
        use super::*;

        fn setup_game() -> (Game, PlayerID, PlayerID, PlayerID, PlayerID) {
            let mut game = Game::new();
            let a = game.add_player(Player::new("a"));
            let b = game.add_player(Player::new("b"));
            let c = game.add_player(Player::new("c"));
            let d = game.add_player(Player::new("d"));

            return (game, a, b, c, d);
        }

        #[test]
        fn _102_1_active_player() {
            let (game, a, b, c, d) = setup_game();
            assert_eq!(game.active_player, a);
            assert_ne!(game.active_player, b);
            assert_ne!(game.active_player, c);
            assert_ne!(game.active_player, d);
        }
    }
}
