//! Task 3.2: Pokemon

fn main() {

    let player1 = "Red";
    let player2 = "Blue";
    let pokemon1 = choose_pokemon(&player1);
    let pokemon2 = choose_pokemon(&player2);

    let mut poki1 = Pokemon::with_level(pokemon1, 5);
    let mut poki2 = Pokemon::with_level(pokemon2, 5);

    if poki2.stats().speed > poki1.stats().speed {
        let help = poki2;
        poki2 = poki1;
        poki1 = help;
    }

    while poki1.is_alive() && poki2.is_alive() {

        let name1 = poki1.model().name;
        let name2 = poki2.model().name;

        let model1 = poki1.model();

        println!(">>>>> Status: {} has {} HP, {} has {} HP",
            name1, poki1.stats().hp,
            name2, poki2.stats().hp);
        println!(">>>>> {} is about to attack! Which move shall it execute?",
            name1);
        for i in 0..model1.attacks.len() {
            println!("{}: {:?}", i, model1.attacks[i].name)
        }
        loop {
            println!("!!! Please give me the attack ID:");
            let id = read_string();
            let index;
            if id == "0" {
                index = 0;
            } else if id == "1" {
                index = 1;
            } else {
                continue;
            }
            if index == 1 && model1.attacks.len() == 1 {
                continue;
            }
            let someattack = model1.attacks[index];
            poki2.endure_attack(&poki1, *someattack);
            println!("{} uses {}! ({} has {} HP left)",
                name1, someattack.name, name2, poki2.stats().hp);
            break;
        }
        let help = poki2;
        poki2 = poki1;
        poki1 = help;
        println!("");
    }
    if poki1.is_alive() {
        println!(">>>>> {} fainted!", poki2.model().name);
    } else {
        println!(">>>>> {} fainted!", poki1.model().name);
    }
}



/// Describes an attack with all its properties. This type is similar to
/// `PokemonModel`, as there are finite many, immutable instances of this type
/// in a database. This is not a type whose instances change over time.
#[derive(Debug, Clone, Copy)]
struct Attack {
    category: AttackCategory,
    name: &'static str,
    /// Base power of the move. The actual inflicted damage is calculated with
    /// a formula using the move's power and a few other parameters.
    base_power: u8,
    type_: Type,
}

/// Category of an attack.
///
/// Note: currently, the category 'status' is missing.
#[derive(Debug, Clone, Copy)]
enum AttackCategory {
    /// Attacks with body contact, like "Tackle" or "Bite"
    Physical,
    /// Attacks without body contact, like "Bubble Beam" or "Thunderbolt"
    Special,
}

/// Describes how effective an attack of one type is on a Pokemon of another
/// type.
///
/// Note that a Pokemon can have two types. In order to determine the
/// effectiveness, the multipliers of the effectivenesses on both types
/// are multiplied. As such, there can be 0.25 and 4.0 multipliers!
#[derive(Debug, Clone, Copy)]
enum TypeEffectiveness {
    NotEffective,
    NotVeryEffective,
    Normal,
    SuperEffective,
}

impl TypeEffectiveness {
    /// Returns the type effectiveness of an attack from one attacker type
    /// on one defender type.
    fn of_attack(attacker: Type, defender: Type) -> Self {
        use Type::*;
        use TypeEffectiveness as Te;

        // TODO: complete this
        match (attacker, defender) {
            (Fire, Water) => Te::NotVeryEffective,
            (Fire, Grass) => Te::SuperEffective,
            (Water, Fire) => Te::SuperEffective,
            (Water, Grass) => Te::NotVeryEffective,
            (Grass, Fire) => Te::NotVeryEffective,
            (Grass, Water) => Te::SuperEffective,
            _ => Te::Normal,
        }
    }

    /// Returns the corresponding multiplier for the damage formula.
    fn multiplier(&self) -> f64 {
        match *self {
            TypeEffectiveness::NotEffective => 0.0,
            TypeEffectiveness::NotVeryEffective => 0.5,
            TypeEffectiveness::Normal => 1.0,
            TypeEffectiveness::SuperEffective => 2.0,
        }
    }
}


/// Types (sometimes called "elements") of the Pokemon universe. Each
/// attack-move has exactly one type, Pokemons can have one or two types.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum Type {
    Normal,
    Fire,
    Fighting,
    Water,
    Flying,
    Grass,
    Poison,
    Electric,
    Ground,
    Psychic,
    Rock,
    Ice,
    Bug,
    Dragon,
    Ghost,
    Dark,
    Steel,
    Fairy,
}

/// Describes the type of a Pokemon. Pokemon can have one or two types.
#[derive(Debug, Clone, Copy)]
enum PokemonType {
    One(Type),
    Two(Type, Type),
}

/// Describes a kind of Pokemon, e.g. "Pikachu".
///
/// This is different than an actual, living Pokemon. This struct just
/// describes properties that are the same for every creature of this
/// Pokemon kind.
#[derive(Debug, Clone, Copy)]
struct PokemonModel {
    /// Name of the Pokemon
    name: &'static str,
    /// ID in the international Pokedex
    id: u16,
    type_: PokemonType,
    base_stats: Stats,
    /// This is different from the real Pokemon games: attacks are not part
    /// of the Pokemon model, but of the Pokemon itself (as they change over
    /// time). A pokemon just has an abstract learnset of potential attacks.
    /// But this is easier for now.
    attacks: &'static [&'static Attack]
}

struct Pokemon {
    model: PokemonModel,
    current_stats: Stats,
    current_level: u8
}

impl Pokemon {

    fn with_level(model: PokemonModel, mut level: u8) -> Self {
        if level > 100 {
            level = 100;
        }
        Pokemon {
            model: model,
            current_stats: Stats::at_level(model.base_stats, level),
            current_level: level,
        }
    }

    fn stats(&self) -> &Stats {
        &self.current_stats
    }

    fn model(&self) -> PokemonModel {
        self.model
    }

    fn name(&self) -> &'static str {
        self.model.name
    }

    fn level(&self) -> u8 {
        self.current_level
    }

    fn is_alive(&self) -> bool {
        if self.current_stats.hp > 0 {
            return true;
        }
        false
    }

    fn endure_attack(&mut self, opponent: &Pokemon, attack: Attack) {
        let damage = attack_damage(opponent, &self, attack);
        self.current_stats.hp = self.current_stats.hp.saturating_sub(damage);
    }

}

/// Describes the basic stats of a Pokemon.
///
/// Each living Pokemon has an actual stat value, but each Pokemon kind also
/// has so called "base stats". These base stats are used to calculate the
/// actual stats, whose depend on the Pokemon's current level. Stronger Pokemon
/// have higher base stats.
#[derive(Debug, Clone, Copy)]
struct Stats {
    /// Health points
    hp: u16,
    /// Speed, sometimes called initiative (INIT)
    speed: u16,
    /// Strength of physical attacks (like "Tackle")
    attack: u16,
    /// Strength of special attacks (like "Bubble Beam")
    special_attack: u16,
    /// Defense power against physical attacks (like "Tackle")
    defense: u16,
    /// Defense power against special attacks (like "Bubble Beam")
    special_defense: u16,
}

impl Stats {
    /// Given the base stats and a level, this function returns the actual
    /// stats for that level.
    ///
    /// This function doesn't implement the correct formula used by Pokemon
    /// games. It is a simplified version of the original formula for now: we
    /// ignore IVs, EVs and the Pokemon's nature). The complete formula can be
    /// found [here (HP)][1] and [here (other stats)][2].
    ///
    /// [1]: http://bulbapedia.bulbagarden.net/wiki/File:HPStatCalcGen34.png
    /// [2]: http://bulbapedia.bulbagarden.net/wiki/File:OtherStatCalcGen34.png
    fn at_level(base: Self, level: u8) -> Self {
        /// The formula is the same for all stats != hp
        fn stat_formula(base: u16, level: u8) -> u16 {
            ((base as f64 * level as f64) / 50.0 + 5.0) as u16
        }

        let hp = (
            (base.hp as f64 * level as f64) / 50.0
                + level as f64
                + 10.0
        ) as u16;

        Stats {
            hp: hp,
            speed: stat_formula(base.speed, level),
            attack: stat_formula(base.attack, level),
            special_attack: stat_formula(base.special_attack, level),
            defense: stat_formula(base.defense, level),
            special_defense: stat_formula(base.special_defense, level),
        }
    }
}

// ===========================================================================
// ===========================================================================
// ===========================================================================
// Formulas to calculate stuff
// ===========================================================================
// ===========================================================================
// ===========================================================================

/// Calculates the damage of an attack. We don't use the exact formula, but
/// a simplified version of it. In particular, we simplified the "Modifier"
/// term quite a bit. The correct and complete formula can be found [here][1].
///
/// [1]: http://bulbapedia.bulbagarden.net/wiki/Damage#Damage_formula
 fn attack_damage(attacker: &Pokemon, defender: &Pokemon, attack: Attack) -> u16 {
     // Depending on the attack category, get the correct stats
     let (attack_mod, defense_mod) = match attack.category {
         AttackCategory::Physical => {
             (attacker.stats().attack, defender.stats().defense)
         }
         AttackCategory::Special => {
             (attacker.stats().special_attack, defender.stats().special_defense)
         }
     };

     // Cast everything to f64 to reduce noise in actual formula
     let (attack_mod, defense_mod) = (attack_mod as f64, defense_mod as f64);
     let base_power = attack.base_power as f64;
     let attacker_level = attacker.level() as f64;

     // The modifier only depends on the type effectiveness (in our simplified
     // version!).
     let modifier = match defender.model().type_ {
         PokemonType::One(ty) => {
             TypeEffectiveness::of_attack(attack.type_, ty).multiplier()
         }
         PokemonType::Two(ty_a, ty_b) => {
             TypeEffectiveness::of_attack(attack.type_, ty_a).multiplier()
                 * TypeEffectiveness::of_attack(attack.type_, ty_b).multiplier()
         }
     };

     // With every parameter prepared above, here is the formula
     (
         (
             ((2.0 * attacker_level + 10.0) / 250.0)
                 * (attack_mod / defense_mod)
                 * base_power
                 + 2.0
         ) * modifier
     ) as u16
 }

// ===========================================================================
// ===========================================================================
// ===========================================================================
// This is just constant data!
// ===========================================================================
// ===========================================================================
// ===========================================================================

/// The Pokemon database!
///
/// Apart from the "attacks" field, all values are correct.
const POKEDEX: &'static [PokemonModel] = &[
    PokemonModel {
        name: "Bulbasaur",
        id: 001,
        type_: PokemonType::Two(Type::Poison, Type::Grass),
        base_stats: Stats {
            hp: 45,
            attack: 49,
            defense: 49,
            special_attack: 65,
            special_defense: 65,
            speed: 45,
        },
        attacks: &[TACKLE],
    },
    PokemonModel {
        name: "Ivysaur",
        id: 002,
        type_: PokemonType::Two(Type::Poison, Type::Grass),
        base_stats: Stats {
            hp: 60,
            attack: 62,
            defense: 63,
            special_attack: 80,
            special_defense: 80,
            speed: 60,
        },
        attacks: &[TACKLE, VINE_WHIP],
    },
    PokemonModel {
        name: "Venusaur",
        id: 003,
        type_: PokemonType::Two(Type::Poison, Type::Grass),
        base_stats: Stats {
            hp: 80,
            attack: 82,
            defense: 83,
            special_attack: 100,
            special_defense: 100,
            speed: 80,
        },
        attacks: &[TACKLE, VINE_WHIP],
    },
    PokemonModel {
        name: "Charmander",
        id: 004,
        type_: PokemonType::One(Type::Fire),
        base_stats: Stats {
            hp: 39,
            attack: 52,
            defense: 43,
            special_attack: 60,
            special_defense: 50,
            speed: 65,
        },
        attacks: &[TACKLE],
    },
    PokemonModel {
        name: "Charmeleon",
        id: 005,
        type_: PokemonType::One(Type::Fire),
        base_stats: Stats {
            hp: 58,
            attack: 64,
            defense: 58,
            special_attack: 80,
            special_defense: 65,
            speed: 80,
        },
        attacks: &[TACKLE, EMBER],
    },
    PokemonModel {
        name: "Charizard",
        id: 006,
        type_: PokemonType::Two(Type::Fire, Type::Flying),
        base_stats: Stats {
            hp: 78,
            attack: 84,
            defense: 78,
            special_attack: 109,
            special_defense: 85,
            speed: 100,
        },
        attacks: &[TACKLE, EMBER],
    },
    PokemonModel {
        name: "Squirtle",
        id: 007,
        type_: PokemonType::One(Type::Water),
        base_stats: Stats {
            hp: 44,
            attack: 48,
            defense: 65,
            special_attack: 50,
            special_defense: 64,
            speed: 43,
        },
        attacks: &[TACKLE],
    },
    PokemonModel {
        name: "Wartortle",
        id: 008,
        type_: PokemonType::One(Type::Water),
        base_stats: Stats {
            hp: 59,
            attack: 63,
            defense: 80,
            special_attack: 65,
            special_defense: 80,
            speed: 58,
        },
        attacks: &[TACKLE, WATER_GUN],
    },
    PokemonModel {
        name: "Blastoise",
        id: 009,
        type_: PokemonType::One(Type::Water),
        base_stats: Stats {
            hp: 79,
            attack: 83,
            defense: 100,
            special_attack: 85,
            special_defense: 105,
            speed: 78,
        },
        attacks: &[TACKLE, WATER_GUN],
    },
];

/// List of all attacks.
///
/// Of course, these are not all attacks. We will probably provide a much
/// bigger database with the next sheet.
const ATTACK_DB: &'static [Attack] = &[
    Attack {
        category: AttackCategory::Physical,
        name: "Tackle",
        base_power: 50,
        type_: Type::Normal,
    },
    Attack {
        category: AttackCategory::Special,
        name: "Vine Whip",
        base_power: 45,
        type_: Type::Grass,
    },
    Attack {
        category: AttackCategory::Special,
        name: "Ember",
        base_power: 40,
        type_: Type::Fire,
    },
    Attack {
        category: AttackCategory::Special,
        name: "Water Gun",
        base_power: 40,
        type_: Type::Water,
    },
];

// These are just some easy names to be more expressive in the Pokedex.
const TACKLE: &'static Attack = &ATTACK_DB[0];
const VINE_WHIP: &'static Attack = &ATTACK_DB[1];
const EMBER: &'static Attack = &ATTACK_DB[2];
const WATER_GUN: &'static Attack = &ATTACK_DB[3];




// ===========================================================================
// ===========================================================================
// ===========================================================================
// Helper functions (you don't need to understand how they work yet)
// ===========================================================================
// ===========================================================================
// ===========================================================================

/// Reads a string from the terminal/user.
fn read_string() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}

/// Reads a valid `usize` integer from the terminal/user.
fn read_usize() -> usize {
    loop {
        match read_string().parse() {
            Ok(res) => return res,
            Err(_) => println!("That was not an integer! Please try again!"),
        }
    }
}

// ===========================================================================
// ===========================================================================
// ===========================================================================
// Task 2.1
// ===========================================================================
// ===========================================================================
// ===========================================================================

fn print_pokemon_list() {
    for pokemon in POKEDEX {
        println!("#{} {}", pokemon.id, pokemon.name)
    }
}

fn find_pokemon_by_name(name: &str) -> Option<PokemonModel> {
    for pokemon in POKEDEX {
        if name == pokemon.name {
            return Some(*pokemon);
        }
    }
    None
}

fn choose_pokemon(name: &str) -> PokemonModel {
    let mut pokemon;
    loop {
        println!("Player {}, please choose a Pokemon (or type '?' to get a complete list)", name);
        let string = read_string();
        if string == "?" {
            print_pokemon_list();
            continue;
        }
        pokemon = find_pokemon_by_name(&string);
        if let Some(poki) = pokemon {
            return poki;
        }
    }
}
