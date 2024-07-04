//! PRELUDE
//! TODO docs

use std::{collections::HashMap, env, fmt::{Display, Error}, fs, path::Path, str::FromStr};

use nu_ansi_term::{Color, Rgb};
use regex::Regex;
use serde::{Serialize,Deserialize};
use serde_json::{Result, Value};
use rand::Rng;

use crate::fist::{Matrix, Premade, Role, Trait};

pub const JSON_DIR: &str = "fist_jsons/";
pub const MATRICES_DIR: &str = "fist_jsons/matrices/";
pub const ROLES: &str = "fist_jsons/roles.json";
pub const TRAITS: &str = "fist_jsons/traits.json";
pub const CASSETTES: &str = "fist_jsons/matrices/cassettes.json";
pub const MISC: &str = "fist_jsons/matrices/misc.json";
pub const CASSETTE_LINKS: &str = "fist_jsons/matrices/cassette_links.json";
pub const MISSION_GENERATOR: &str = "fist_jsons/matrices/mission_generator.json";
pub const MISSION_PROMPTS: &str = "fist_jsons/matrices/mission_prompts.json";
pub const CHARACTERS_ANIMALS: &str = "fist_jsons/matrices/characters/animals.json";
pub const CHARACTERS_ANOMALIES: &str = "fist_jsons/matrices/characters/anomalies.json";
pub const CHARACTERS_CELEBRITIES: &str = "fist_jsons/matrices/characters/celebrities.json";
pub const CHARACTERS_CIVILIANS: &str = "fist_jsons/matrices/characters/civilians.json";
pub const CHARACTERS_EXPERIMENTS: &str = "fist_jsons/matrices/characters/experiments.json";
pub const CHARACTERS_MONSTERS: &str = "fist_jsons/matrices/characters/monsters.json";
pub const CHARACTERS_POLITICIANS: &str = "fist_jsons/matrices/characters/politicians.json";
pub const CHARACTERS_PREMADE_ENEMIES: &str = "fist_jsons/matrices/characters/premade_enemies.json";
pub const CHARACTERS_PREMADE_NPCS: &str = "fist_jsons/matrices/characters/premade_npcs.json";
pub const CHARACTERS_ROBOTS: &str = "fist_jsons/matrices/characters/robots.json";
pub const CHARACTERS_SCIENTISTS: &str = "fist_jsons/matrices/characters/scientists.json";
pub const CHARACTERS_SOLDIERS: &str = "fist_jsons/matrices/characters/soldiers.json";
pub const CHARACTERS_SPIES: &str = "fist_jsons/matrices/characters/spies.json";
pub const CHARACTERS_SQUADS: &str = "fist_jsons/matrices/characters/squads.json";
pub const CYCLOPS_GADGETS: &str = "fist_jsons/matrices/cyclops/gadgets.json";
pub const CYCLOPS_RUMORS: &str = "fist_jsons/matrices/cyclops/rumors.json";
pub const FACTIONS_AGENCIES: &str = "fist_jsons/matrices/factions/agencies.json";
pub const FACTIONS_ALIENS: &str = "fist_jsons/matrices/factions/aliens.json";
pub const FACTIONS_CORPORATIONS: &str = "fist_jsons/matrices/factions/corporations.json";
pub const FACTIONS_CRIMINALS: &str = "fist_jsons/matrices/factions/criminals.json";
pub const FACTIONS_CULTS: &str = "fist_jsons/matrices/factions/cults.json";
pub const FACTIONS_INSURGENTS: &str = "fist_jsons/matrices/factions/insurgents.json";
pub const GEAR_BASE_UPGRADES: &str = "fist_jsons/matrices/gear/base_upgrades.json";
pub const GEAR_ITEMS: &str = "fist_jsons/matrices/gear/items.json";
pub const GEAR_VEHICLES: &str = "fist_jsons/matrices/gear/vehicles.json";
pub const GEAR_WEAPONS_AND_ARMOR: &str = "fist_jsons/matrices/gear/weapons_and_armor.json";
pub const LOCATIONS_BATTLEFIELDS: &str = "fist_jsons/matrices/locations/battlefields.json";
pub const LOCATIONS_CITIES: &str = "fist_jsons/matrices/locations/cities.json";
pub const LOCATIONS_NATURE: &str = "fist_jsons/matrices/locations/nature.json";
pub const LOCATIONS_ROOMS: &str = "fist_jsons/matrices/locations/rooms.json";
pub const LOCATIONS_STRUCTURES: &str = "fist_jsons/matrices/locations/structures.json";
pub const LOCATIONS_ZONES: &str = "fist_jsons/matrices/locations/zones.json";
pub const LORE_ARTIFACTS: &str = "fist_jsons/matrices/lore/artifacts.json";
pub const LORE_COVERUPS: &str = "fist_jsons/matrices/lore/coverups.json";
pub const LORE_DIPLOMACY: &str = "fist_jsons/matrices/lore/diplomacy.json";
pub const LORE_DISASTER: &str = "fist_jsons/matrices/lore/disasters.json";
pub const LORE_LEGENDS: &str = "fist_jsons/matrices/lore/legends.json";
pub const LORE_SPELLS: &str = "fist_jsons/matrices/lore/spells.json";
pub const WORLD_HAZARDS: &str = "fist_jsons/matrices/world/hazards.json";

pub const ALL_FILES: &[&str] = &[
    ROLES, TRAITS,
    CASSETTES, CASSETTE_LINKS, MISC, MISSION_GENERATOR, MISSION_PROMPTS,
    CHARACTERS_ANIMALS, CHARACTERS_ANOMALIES, CHARACTERS_CELEBRITIES,
    CHARACTERS_CIVILIANS, CHARACTERS_EXPERIMENTS, CHARACTERS_MONSTERS,
    CHARACTERS_POLITICIANS, CHARACTERS_ROBOTS, CHARACTERS_SCIENTISTS, CHARACTERS_SOLDIERS,
    CHARACTERS_SPIES, CHARACTERS_SQUADS, CHARACTERS_PREMADE_ENEMIES, CHARACTERS_PREMADE_NPCS,
    CYCLOPS_RUMORS, CYCLOPS_GADGETS,
    FACTIONS_AGENCIES, FACTIONS_ALIENS, FACTIONS_CORPORATIONS, FACTIONS_CRIMINALS,
    FACTIONS_CULTS, FACTIONS_INSURGENTS,
    GEAR_ITEMS, GEAR_VEHICLES, GEAR_WEAPONS_AND_ARMOR, GEAR_BASE_UPGRADES,
    LOCATIONS_BATTLEFIELDS, LOCATIONS_CITIES, LOCATIONS_NATURE, LOCATIONS_ROOMS, 
    LOCATIONS_STRUCTURES, LOCATIONS_ZONES, 
    LORE_ARTIFACTS, LORE_COVERUPS, LORE_DIPLOMACY, LORE_DISASTER, LORE_LEGENDS, LORE_SPELLS, 
    WORLD_HAZARDS,
];

pub const MATRIX_LIST: &[&str] = &[
    MISC, MISSION_GENERATOR, MISSION_PROMPTS,
    CHARACTERS_ANIMALS, CHARACTERS_ANOMALIES, CHARACTERS_CELEBRITIES,
    CHARACTERS_CIVILIANS, CHARACTERS_EXPERIMENTS, CHARACTERS_MONSTERS,
    CHARACTERS_POLITICIANS, CHARACTERS_ROBOTS, CHARACTERS_SCIENTISTS, CHARACTERS_SOLDIERS,
    CHARACTERS_SPIES, CHARACTERS_SQUADS, //CHARACTERS_PREMADE_ENEMIES,
    CYCLOPS_RUMORS, //CYCLOPS_GADGETS
    FACTIONS_AGENCIES, FACTIONS_ALIENS, FACTIONS_CORPORATIONS, FACTIONS_CRIMINALS,
    FACTIONS_CULTS, FACTIONS_INSURGENTS,
    GEAR_ITEMS, GEAR_VEHICLES, GEAR_WEAPONS_AND_ARMOR, // GEAR_BASE_UPGRADES
    LOCATIONS_BATTLEFIELDS, LOCATIONS_CITIES, LOCATIONS_NATURE, LOCATIONS_ROOMS, 
    LOCATIONS_STRUCTURES, LOCATIONS_ZONES, 
    LORE_ARTIFACTS, LORE_COVERUPS, LORE_DIPLOMACY, LORE_DISASTER, LORE_LEGENDS, LORE_SPELLS, 
    WORLD_HAZARDS,
];

pub const HELP_CMD_LS: &str = r#"
\x1b[1;34mls\x1b[1;32m or \x1b[1;34mdir\x1b[1;32m is used to query and navigate the FIST JSON data as if it were
a native filesystem. The actual physical files on disk are represented as directories.

When used without arguments, \x1b[1;34mls\x1b[1;32m lists the 'root' directory and everything under it. 
When used with an argument, the command never takes more than two directories, and in 
multi-directory depth searches it is always the last two. 

Examples: 
  'dir' lists an expanded directory view at root.
  'ls roles' will list all roles. Note it's location on the vfs.
  'ls world/hazards' will list all tables in the hazards matrix.
  'dir gear/items' lists all tables from the gear/items matrix.

NOTE: If you want to find particular entries, use the 'search' command.

"#;

pub const HELP_CMD_SEARCH: &str = "
\x1b[1;34msearch\x1b[1;32m is used to query particular\x1b[1;34m roles, traits,\x1b[0m and\x1b[1;34m matrix\x1b[0m entries
found within their respective data tables. Searching a role or trait returns the full listing of
the role/trait, if found, while searching matrix tables returns the table itself and all entries.

Input is not case-sensitive. Multi-word searches \x1b[1;31mdo not\x1b[0m use quotes.

\x1b[1;34msearch \x1b[1;36m<roles|traits|matrix> \x1b[32m<search string>

Examples:
  '\x1b[1;34msearch\x1b[0m \x1b[1;36mroles\x1b[0m \x1b[1;32mcrusader\x1b[0m'      looks for the CRUSADER role and returns it if found.
  '\x1b[1;34msearch\x1b[0m \x1b[1;36mtraits\x1b[0m \x1b[1;32mxeno\x1b[0m'         looks for the XENO trait and returns it if found.
  '\x1b[1;34msearch\x1b[0m \x1b[1;36mmatrix\x1b[0m \x1b[1;32mweapons\x1b[0m'      searches the matrices and prints the weapons table.
  '\x1b[1;34msearch\x1b[0m \x1b[1;36mmatrix\x1b[0m \x1b[1;32mweapon skins\x1b[0m' searches matrices for `weapon skins.`

💡NOTE: 'role,' 'roles,' 'trait,' traits,' accepted.

";

const HELP_CMD_RANDOM: &str = r"

random <role | trait | matrix | npc | enemy | mission>
";
const HELP_CMD_ROLL: &str = r"

roll [dice]
";
const HELP_CMD_DICEPOOL: &str = r"

dicepool [args]
";
macro_rules! ok(($result:expr) => ($result.unwrap()));

//use SteelBlue as Rgb(70,130,180);
const SteelBlue: Rgb = Rgb { r:70,g:130,b:180};

pub fn break_into(s: &str, len: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut start = 0;

    while start < s.len() {
        let end = std::cmp::min(start + len, s.len());
        lines.push(s[start..end].to_string());
        //lines.push(format!("{}\n",&s[start..end]));
        start = end;
    }

    lines    
}

pub fn json_exists(json_file_path: &str) -> bool {
    fs::metadata(json_file_path).is_ok()
}

pub fn all_jsons() -> [&'static str; 21] {
    [
        ROLES, TRAITS,
        CASSETTES, CASSETTE_LINKS, MISC, MISSION_GENERATOR, MISSION_PROMPTS,
        CHARACTERS_ANIMALS, CHARACTERS_ANOMALIES, CHARACTERS_CELEBRITIES,
        CHARACTERS_CIVILIANS, CHARACTERS_EXPERIMENTS, CHARACTERS_MONSTERS,
        CHARACTERS_POLITICIANS, CHARACTERS_PREMADE_ENEMIES, CHARACTERS_PREMADE_NPCS,
        CHARACTERS_ROBOTS, CHARACTERS_SCIENTISTS, CHARACTERS_SOLDIERS,
        CHARACTERS_SPIES, CHARACTERS_SQUADS
    ]

}

pub fn all_matrices() -> [&'static str; 38] {
    [
        MISC, MISSION_GENERATOR, MISSION_PROMPTS,
        CHARACTERS_ANIMALS, CHARACTERS_ANOMALIES, CHARACTERS_CELEBRITIES,
        CHARACTERS_CIVILIANS, CHARACTERS_EXPERIMENTS, CHARACTERS_MONSTERS,
        CHARACTERS_POLITICIANS, 
        CHARACTERS_ROBOTS, CHARACTERS_SCIENTISTS, CHARACTERS_SOLDIERS,
        CHARACTERS_SPIES, CHARACTERS_SQUADS, CYCLOPS_RUMORS, //CYCLOPS_GADGETS, CYCLOPS_RUMORS,
        FACTIONS_AGENCIES, FACTIONS_ALIENS, FACTIONS_CORPORATIONS, FACTIONS_CRIMINALS,
        FACTIONS_CULTS, FACTIONS_INSURGENTS, /*GEAR_BASE_UPGRADES, */ GEAR_ITEMS,
        GEAR_VEHICLES, GEAR_WEAPONS_AND_ARMOR, LOCATIONS_BATTLEFIELDS,
        LOCATIONS_CITIES, LOCATIONS_NATURE, LOCATIONS_ROOMS, LOCATIONS_STRUCTURES,
        LOCATIONS_ZONES, LORE_ARTIFACTS, LORE_COVERUPS, LORE_DIPLOMACY,
        LORE_DISASTER, LORE_LEGENDS, LORE_SPELLS, WORLD_HAZARDS,       
    ]
}

//pub fn load_json(json: &str) -> Result<Vec

pub fn load_matrix(json: &'static str) -> Result<Vec<Matrix>> {
    //TODO validation
    if json_exists(json) { print!(""); }
    let data = fs::read_to_string(json).unwrap();
    serde_json::from_str(&data)
    // let result: Vec<Matrix> = serde_json::from_str(&data)?;
    // Ok(result)
}
pub fn load_premade(json: &'static str) -> Result<Vec<Premade>> {
    let mut data = fs::read_to_string(json).unwrap();
    serde_json::from_str(&data)

}
pub fn load_roles(json: &'static str) -> Result<Vec<Role>> {
    //TODO validation
    let mut data = fs::read_to_string(json).unwrap();
    serde_json::from_str(&data)
}

pub fn load_traits(json: &'static str) -> Result<Vec<Trait>> {
    //TODO validation
    let mut data = fs::read_to_string(json).unwrap();
    serde_json::from_str(&data)

}
pub fn get_matrix_by_title(title: &String) -> Option<Matrix> {
    for m in all_matrices() {
        let table = load_matrix(m).unwrap();
        println!("Loaded {} tables from {m}",table.len());
        if let Some(t) =  table.into_iter().find(|m| m.Title == *title) {
            return Some(t); 
        }
     }
    None
}
fn num_entries(file_path: &str) -> usize {
    if json_exists(file_path) {
        let data = fs::read_to_string(file_path).unwrap();
        if file_path == CASSETTE_LINKS {
            let m: HashMap<String,String> = serde_json::from_str(&data).unwrap();
            m.len()
        } else {
            let m: Vec<Value> = serde_json::from_str(&data).unwrap();
            m.len()
        }       
    } else { 0 }
}

pub fn get_filename(file_path: &str) -> String {
    let path = Path::new(file_path);
    match path.file_stem() {
        Some(stem) => stem.to_string_lossy().into_owned(),
        None => String::new(),
    }
}
pub fn gen_byte_entries(json_file_path: &str) {
//    if fs::metadata(json_file_path).is_ok() {
    let attr = fs::metadata(json_file_path).unwrap();
    let mut mode = "🔎";
    if attr.is_dir() {
        mode = "📁";
    }
    let len = num_entries(json_file_path);
    println!(" {:2}  fist \x1b[1;37m{:^8} {len:<5} {:<24}",
        mode,
        attr.len(),
        //format!("{}",Color::White.bold().paint(attr.len().to_string())),
        //len, 
        format!("{}",Color::Blue.paint(get_filename(json_file_path))));

}








#[derive(Clone)]
pub struct Die(u8);
impl Die {
    pub fn new(sides: u8) -> Self {Die(sides)}
    pub fn sides(&self) -> u8 {self.0}

    pub fn d4() -> Self { Die(4)}
    pub fn d6() -> Self { Self::new(6) }
    pub fn d8() -> Self { Self::new(8) }
    pub fn d10() -> Self { Self::new(10) }
    pub fn d12() -> Self { Self::new(12) }
    pub fn d20() -> Self { Self::new(20) }
    pub fn d100() -> Self { Self::new(100) }
}
impl Display for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"1{}{}",Color::Green.paint("d"), Color::Blue.bold().paint(self.0.to_string()))
    }
}

pub struct DicePool {
    dice: Vec<Die>,
    multiplier: i8,
    flat: i8,

}
impl DicePool {
    pub fn new(die: Die) -> Self {
        Self {
            dice: vec![die],
            multiplier: 1,
            flat: 0,
        }
    }
    pub fn new_dice(dice: Vec<Die>) -> Self {
        Self {
            dice,
            multiplier: 1,
            flat: 0,
        }
    }
    pub fn dice(&self) -> Vec<Die> { self.dice.clone() }
    pub fn add_die(&self, die: Die) -> DicePool {
        self.add_dice(vec![die])
    }
    pub fn add_dice(&self, dice: Vec<Die>) -> DicePool {
        let mut new_dice = self.dice.clone();
        new_dice.append(&mut dice.clone());
        new_dice.sort_by_key(|d| d.sides());
        DicePool {
            dice: new_dice,
            multiplier: self.multiplier,
            flat: self.flat,
        }
    }
    pub fn best() {}
    pub fn multiply(mut self, val: i8) -> Self {
        self.multiplier = val;
        self
    }
    pub fn flat_modifier(mut self, val: i8) -> Self {
        self.flat = val;
        self
    }
    fn calc_flat_modifier(&self) -> i8 {
        self.flat
    }
    // Creates a `DicePool` with the number of dice types as expressed `xdy` where
    // `x` is number of dice and `y` is the sides. e.g. 2d6 `xdy(2,6)` (from Rise)
    pub fn xdy(num: u8, sides: u8) -> DicePool {
        let mut dice: Vec<Die> = vec![];
        for _ in 0..num {
            dice.push(Die::new(sides));
        }
        Self::new_dice(dice)
    }

    pub fn rand<R: Rng>(&self, rnd: &mut R) -> u8 {
        let mut sum = 1;
        for die in self.dice.iter() {
            sum += rnd.gen_range(1..die.sides());
        }
        sum
    }
}
impl Display for DicePool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut counts = HashMap::new();
        for die in self.dice.iter() {
            counts
                .entry(die.sides())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut contained_sizes: Vec<&u8> = counts.keys().collect();
        contained_sizes.sort();

        let mut dice_texts: Vec<String> = contained_sizes
            .iter()
            .map(|s| format!("{}{}{}",
                Color::Green.bold().paint((counts[s] as i32).to_string()), // * self.multiplier
                Color::Default.paint("d"),
                Color::Green.bold().paint(s.to_string())))
            .collect();
        let modifier = self.calc_flat_modifier();
        if modifier != 0 {
            dice_texts.push(modifier.to_string());
        }


        let result = dice_texts.join(format!("{}",Color::White.bold().paint("+")).as_str());
        write!(f,"{result}")
    }
}

impl FromStr for DicePool {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+)d(\d+)").unwrap();
        let dice_pool = DicePool::default();

        for cap in re.captures_iter(s) {
            let quantity = cap[1].parse::<u8>().unwrap();
            let sides = cap[2].parse::<u8>().unwrap();
            print!("d{sides}: {quantity}\nAdded ");
            for _ in 0..quantity {
                dice_pool.add_die(Die(sides));
                print!("{} ",Die(sides));
            }

        }

        Ok(dice_pool)      
    }
}

impl Default for DicePool {
    fn default() -> Self {
        Self { dice: vec![], multiplier: 0, flat: 0 }
    }
}


pub const FIST_BANNER1: &str = r#"

███████╗██╗███████╗████████╗     ██████╗██╗     ██╗
██╔════╝██║██╔════╝╚══██╔══╝    ██╔════╝██║     ██║
█████╗  ██║███████╗   ██║       ██║     ██║     ██║
██╔══╝  ██║╚════██║   ██║       ██║     ██║     ██║
██║     ██║███████║   ██║       ╚██████╗███████╗██║
╚═╝     ╚═╝╚══════╝   ╚═╝        ╚═════╝╚══════╝╚═╝
"#;

pub const FIST_BANNER2: &str = r#"
 ______   __     ______     ______      ______     __         __    
/\  ___\ /\ \   /\  ___\   /\__  _\    /\  ___\   /\ \       /\ \   
\ \  __\ \ \ \  \ \___  \  \/_/\ \/    \ \ \____  \ \ \____  \ \ \  
 \ \_\    \ \_\  \/\_____\    \ \_\     \ \_____\  \ \_____\  \ \_\ 
  \/_/     \/_/   \/_____/     \/_/      \/_____/   \/_____/   \/_/ 
"#;

pub const FIST_BANNER3: &str = r#"
    ___       ___       ___       ___            ___       ___       ___   
   /\  \     /\  \     /\  \     /\  \          /\  \     /\__\     /\  \  
  /::\  \   _\:\  \   /::\  \    \:\  \        /::\  \   /:/  /    _\:\  \ 
 /::\:\__\ /\/::\__\ /\:\:\__\   /::\__\      /:/\:\__\ /:/__/    /\/::\__\
 \/\:\/__/ \::/\/__/ \:\:\/__/  /:/\/__/      \:\ \/__/ \:\  \    \::/\/__/
    \/__/   \:\__\    \::/  /   \/__/          \:\__\    \:\__\    \:\__\  
             \/__/     \/__/                    \/__/     \/__/     \/__/  
"#;

pub const FIST_BANNER4: &str = r#"
  _______  __      ________  ___________       ______   ___        __     
 /"     "||" \    /"       )("     _   ")     /" _  "\ |"  |      |" \    
(: ______)||  |  (:   \___/  )__/  \\__/     (: ( \___)||  |      ||  |   
 \/    |  |:  |   \___  \       \\_ /         \/ \     |:  |      |:  |   
 // ___)  |.  |    __/  \\      |.  |         //  \ _   \  |___   |.  |   
(:  (     /\  |\  /" \   :)     \:  |        (:   _) \ ( \_|:  \  /\  |\  
 \__/    (__\_|_)(_______/       \__|         \_______) \_______)(__\_|_) 
"#;

pub const FIST_BANNER5: &str = r#"
 _______  ___   _______  _______    _______  ___      ___  
|       ||   | |       ||       |  |       ||   |    |   | 
|    ___||   | |  _____||_     _|  |       ||   |    |   | 
|   |___ |   | | |_____   |   |    |       ||   |    |   | 
|    ___||   | |_____  |  |   |    |      _||   |___ |   | 
|   |    |   |  _____| |  |   |    |     |_ |       ||   | 
|___|    |___| |_______|  |___|    |_______||_______||___| 
"#;

pub const FIST_BANNER6: &str = r#"
 ______   ________  ______   _________   ______   __        ________     
/_____/\ /_______/\/_____/\ /________/\ /_____/\ /_/\      /_______/\    
\::::_\/_\__.::._\/\::::_\/_\__.::.__\/ \:::__\/ \:\ \     \__.::._\/    
 \:\/___/\  \::\ \  \:\/___/\  \::\ \    \:\ \  __\:\ \       \::\ \     
  \:::._\/  _\::\ \__\_::._\:\  \::\ \    \:\ \/_/\\:\ \____  _\::\ \__  
   \:\ \   /__\::\__/\ /____\:\  \::\ \    \:\_\ \ \\:\/___/\/__\::\__/\ 
    \_\/   \________\/ \_____\/   \__\/     \_____\/ \_____\/\________\/ 
"#;

pub const FIST_BANNER7: &str = r#"
:::===== ::: :::===  :::====      :::===== :::      :::
:::      ::: :::     :::====      :::      :::      :::
======   ===  =====    ===        ===      ===      ===
===      ===     ===   ===        ===      ===      ===
===      === ======    ===         ======= ======== ===                                                    
"#;

pub const FIST_BANNER8: &str = r#"
    _____________________   ________    ____
   / ____/  _/ ___/_  __/  / ____/ /   /  _/
  / /_   / / \__ \ / /    / /   / /    / /  
 / __/ _/ / ___/ // /    / /___/ /____/ /   
/_/   /___//____//_/     \____/_____/___/   
"#;

const FIST_BANNER9: &str = r#"
      ___                       ___           ___                    ___           ___             
     /\  \          ___        /\  \         /\  \                  /\  \         /\__\      ___   
    /::\  \        /\  \      /::\  \        \:\  \                /::\  \       /:/  /     /\  \  
   /:/\:\  \       \:\  \    /:/\ \  \        \:\  \              /:/\:\  \     /:/  /      \:\  \ 
  /::\~\:\  \      /::\__\  _\:\~\ \  \       /::\  \            /:/  \:\  \   /:/  /       /::\__\
 /:/\:\ \:\__\  __/:/\/__/ /\ \:\ \ \__\     /:/\:\__\          /:/__/ \:\__\ /:/__/     __/:/\/__/
 \/__\:\ \/__/ /\/:/  /    \:\ \:\ \/__/    /:/  \/__/          \:\  \  \/__/ \:\  \    /\/:/  /   
      \:\__\   \::/__/      \:\ \:\__\     /:/  /                \:\  \        \:\  \   \::/__/    
       \/__/    \:\__\       \:\/:/  /     \/__/                  \:\  \        \:\  \   \:\__\    
                 \/__/        \::/  /                              \:\__\        \:\__\   \/__/    
                               \/__/                                \/__/         \/__/            
"#;

const FIST_BANNER10: &str = r#"
__________________________     ___________   ____
7     77  77     77      7     7     77  7   7  7
|  ___!|  ||  ___!!__  __!     |  ___!|  |   |  |
|  __| |  |!__   7  7  7       |  7___|  !___|  |
|  7   |  |7     |  |  |       |     7|     7|  |
!__!   !__!!_____!  !__!       !_____!!_____!!__!
"#;
const FIST_BANNER11: &str = r#"
__________________________________       _______________ ________
___  ____/____  _/__  ___/___  __/       __  ____/___  / ____  _/
__  /_     __  /  _____ \ __  /          _  /     __  /   __  /  
_  __/    __/ /   ____/ / _  /           / /___   _  /_____/ /   
/_/       /___/   /____/  /_/            \____/   /_____//___/   
"#;
const FIST_BANNER12: &str = r#"
 ____  ____  ____  ____  _________  ____  ____  ____ 
||F ||||I ||||S ||||T ||||       ||||C ||||L ||||I ||
||__||||__||||__||||__||||_______||||__||||__||||__||
|/__\||/__\||/__\||/__\||/_______\||/__\||/__\||/__\|
"#;
const FIST_BANNER13: &str = r#"

============================================================
=        =    ==      ==        =======     ==  =======    =
=  ========  ==  ====  ====  =========  ===  =  ========  ==
=  ========  ==  ====  ====  ========  =======  ========  ==
=  ========  ===  =========  ========  =======  ========  ==
=      ====  =====  =======  ========  =======  ========  ==
=  ========  =======  =====  ========  =======  ========  ==
=  ========  ==  ====  ====  ========  =======  ========  ==
=  ========  ==  ====  ====  =========  ===  =  ========  ==
=  =======    ==      =====  ==========     ==        =    =
============================================================
"#;
const FIST_BANNER14: &str = r#"

ooooooo_oooo__ooooo__oooooooo_______oooo___oo______oooo_
oo_______oo__oo___oo____oo________oo____oo_oo_______oo__
oooo_____oo___oo________oo_______oo________oo_______oo__
oo_______oo_____oo______oo_______oo________oo_______oo__
oo_______oo__oo___oo____oo________oo____oo_oo_______oo__
oo______oooo__ooooo_____oo__________oooo___ooooooo_oooo_
________________________________________________________
"#;
const FIST_BANNER15: &str = r#"
                                                            
@@@@@@@@  @@@   @@@@@@  @@@@@@@     @@@@@@@  @@@       @@@  
@@@@@@@@  @@@  @@@@@@@  @@@@@@@    @@@@@@@@  @@@       @@@  
@@!       @@!  !@@        @@!      !@@       @@!       @@!  
!@!       !@!  !@!        !@!      !@!       !@!       !@!  
@!!!:!    !!@  !!@@!!     @!!      !@!       @!!       !!@  
!!!!!:    !!!   !!@!!!    !!!      !!!       !!!       !!!  
!!:       !!:       !:!   !!:      :!!       !!:       !!:  
:!:       :!:      !:!    :!:      :!:        :!:      :!:  
 ::        ::  :::: ::     ::       ::: :::   :: ::::   ::  
 :        :    :: : :      :        :: :: :  : :: : :  :   
"#;
const FIST_BANNER16: &str = r#"

     _/\/\/\/\/\/\_/\/\/\/\___/\/\/\/\/\_/\/\/\/\/\/\______/\/\/\/\/\_/\/\_______/\/\/\/\_
    _/\/\___________/\/\___/\/\_____________/\/\________/\/\_________/\/\_________/\/\___ 
   _/\/\/\/\/\_____/\/\_____/\/\/\/\_______/\/\________/\/\_________/\/\_________/\/\___  
  _/\/\___________/\/\___________/\/\_____/\/\________/\/\_________/\/\_________/\/\___   
 _/\/\_________/\/\/\/\_/\/\/\/\/\_______/\/\__________/\/\/\/\/\_/\/\/\/\/\_/\/\/\/\_    
_____________________________________________________________________________________
"#;
const FIST_BANNER17: &str = r#"
..-. .. ... -    -.-. .-.. ..
"#;
const FIST_BANNER18: &str = r#"
___________.___  ____________________ _________ .____    .___ 
\_   _____/|   |/   _____/\__    ___/ \_   ___ \|    |   |   |
 |    __)  |   |\_____  \   |    |    /    \  \/|    |   |   |
 |     \   |   |/        \  |    |    \     \___|    |___|   |
 \___  /   |___/_______  /  |____|     \______  /_______ \___|
     \/                \/                     \/        \/    
"#;
const FIST_BANNER19: &str = r#"
 ____     ______   ____     ______      ____     __     ______     
/\  _`\  /\__  _\ /\  _`\  /\__  _\    /\  _`\  /\ \   /\__  _\    
\ \ \L\_\\/_/\ \/ \ \,\L\_\\/_/\ \/    \ \ \/\_\\ \ \  \/_/\ \/    
 \ \  _\/   \ \ \  \/_\__ \   \ \ \     \ \ \/_/_\ \ \  __\ \ \    
  \ \ \/     \_\ \__ /\ \L\ \  \ \ \     \ \ \L\ \\ \ \L\ \\_\ \__ 
   \ \_\     /\_____\\ `\____\  \ \_\     \ \____/ \ \____//\_____\
    \/_/     \/_____/ \/_____/   \/_/      \/___/   \/___/ \/_____/

"#;
const FIST_BANNER20: &str = r#"
 _______  _______  _______  _______     _______  _______  _______ 
|\     /||\     /||\     /||\     /|   |\     /||\     /||\     /|
| +---+ || +---+ || +---+ || +---+ |   | +---+ || +---+ || +---+ |
| |   | || |   | || |   | || |   | |   | |   | || |   | || |   | |
| |F  | || |I  | || |S  | || |T  | |   | |C  | || |L  | || |I  | |
| +---+ || +---+ || +---+ || +---+ |   | +---+ || +---+ || +---+ |
|/_____\||/_____\||/_____\||/_____\|   |/_____\||/_____\||/_____\|

"#;
const FIST_BANNER21: &str = r#"

01000110 01001001 01010011 01010100  01000011 01001100 01001001 
"#;
const FIST_BANNER22: &str = r#"

                                     ,----,                      ,--,            
                                   ,/   .`|                   ,---.'|            
    ,---,.   ,---,  .--.--.      ,`   .'  :          ,----..  |   | :      ,---, 
  ,'  .' |,`--.' | /  /    '.  ;    ;     /         /   /   \ :   : |   ,`--.' | 
,---.'   ||   :  :|  :  /`. /.'___,/    ,'         |   :     :|   ' :   |   :  : 
|   |   .':   |  ';  |  |--` |    :     |          .   |  ;. /;   ; '   :   |  ' 
:   :  :  |   :  ||  :  ;_   ;    |.';  ;          .   ; /--` '   | |__ |   :  | 
:   |  |-,'   '  ; \  \    `.`----'  |  |          ;   | ;    |   | :.'|'   '  ; 
|   :  ;/||   |  |  `----.   \   '   :  ;          |   : |    '   :    ;|   |  | 
|   |   .''   :  ;  __ \  \  |   |   |  '          .   | '___ |   |  ./ '   :  ; 
'   :  '  |   |  ' /  /`--'  /   '   :  |          '   ; : .'|;   : ;   |   |  ' 
|   |  |  '   :  |'--'.     /    ;   |.'           '   | '/  :|   ,/    '   :  | 
|   :  \  ;   |.'   `--'---'     '---'             |   :    / '---'     ;   |.'  
|   | ,'  '---'                                     \   \ .'            '---'    
`----'                                               `---`                       

"#;
const FIST_BANNER23: &str = r#"

  █████▒ ██▓  ██████ ▄▄▄█████▓    ▄████▄   ██▓     ██▓
▓██   ▒ ▓██▒▒██    ▒ ▓  ██▒ ▓▒   ▒██▀ ▀█  ▓██▒    ▓██▒
▒████ ░ ▒██▒░ ▓██▄   ▒ ▓██░ ▒░   ▒▓█    ▄ ▒██░    ▒██▒
░▓█▒  ░ ░██░  ▒   ██▒░ ▓██▓ ░    ▒▓▓▄ ▄██▒▒██░    ░██░
░▒█░    ░██░▒██████▒▒  ▒██▒ ░    ▒ ▓███▀ ░░██████▒░██░
 ▒ ░    ░▓  ▒ ▒▓▒ ▒ ░  ▒ ░░      ░ ░▒ ▒  ░░ ▒░▓  ░░▓  
 ░       ▒ ░░ ░▒  ░ ░    ░         ░  ▒   ░ ░ ▒  ░ ▒ ░
 ░ ░     ▒ ░░  ░  ░    ░         ░          ░ ░    ▒ ░
         ░        ░              ░ ░          ░  ░ ░  
                                 ░                    

"#;
const FIST_BANNER24: &str = r#"
       ___        ___   ___  ___  ___   ___           ___                  
  .'|=|_.'   .'| |   |=|_.' `._|=|   |=|_.'      .'|=|_.'   .'|        .'| 
.'  |  ___ .'  | `.  |           |   |         .'  |      .'  |      .'  | 
|   |=|_.' |   |   `.|=|`.       |   |         |   |      |   |      |   | 
|   |      |   |  ___  |  `.     `.  |         `.  |  ___ |   |  ___ |   | 
|___|      |___|  `._|=|___|       `.|           `.|=|_.' |___|=|_.' |___| 

"#;
const FIST_BANNER25: &str = r#"
   . .    .       . .       . .    .          . .       .      .    
.+'|=|`+. |`+. .+'|=|`+. .+'|=|`+.=|`+.    .+'|=|`+. .+'|      |`+. 
|  | `+.| |  | |  | `+.| |.+' |  | `+.|    |  | `+.| |  |      |  | 
|  |=|`.  |  | |  | .         |  |         |  |      |  |      |  | 
|  | `.|  |  | `+.|=|`+.      |  |         |  |      |  |      |  | 
|  |      |  | .    |  |      |  |         |  |    . |  |    . |  | 
|  |      |  | |`+. |  |      |  |         |  | .+'| |  | .+'| |  | 
`+.|      |.+' `+.|=|.+'      |.+'         `+.|=|.+' `+.|=|.+' |.+' 

"#;
const FIST_BANNER26: &str = r#"

8"""" 8  8""""8 ""8""    8""""8 8     8  
8     8  8        8      8    " 8     8  
8eeee 8e 8eeeee   8e     8e     8e    8e 
88    88     88   88     88     88    88 
88    88 e   88   88     88   e 88    88 
88    88 8eee88   88     88eee8 88eee 88
"#;
const FIST_BANNER27: &str = r#"

.-:::::'::: .::::::. ::::::::::::      .,-:::::   :::     :::
;;;'''' ;;;;;;`    ` ;;;;;;;;''''    ,;;;'````'   ;;;     ;;;
[[[,,== [[['[==/[[[[,     [[         [[[          [[[     [[[
`$$$"`` $$$  '''    $     $$         $$$          $$'     $$$
 888    888 88b    dP     88,        `88bo,__,o, o88oo,.__888
 "MM,   MMM  "YMmMY"      MMM          "YUMMMMMP"""""YUMMMMMM
"#;
const FIST_BANNER28: &str = r#"
 _____  ____   _____ ______         __  _      ____ 
|     |l    j / ___/|      T       /  ]| T    l    j
|   __j |  T (   \_ |      |      /  / | |     |  T 
|  l_   |  |  \__  Tl_j  l_j     /  /  | l___  |  | 
|   _]  |  |  /  \ |  |  |      /   \_ |     T |  | 
|  T    j  l  \    |  |  |      \     ||     | j  l 
l__j   |____j  \___j  l__j       \____jl_____j|____j
"#;
const FIST_BANNER29: &str = r#"
  _______   ___   _______   _______       _______   ___       ___ 
 |   _   | |   | |   _   | |       |     |   _   | |   |     |   |
 |.  1___| |.  | |   1___| |.|   | |     |.  1___| |.  |     |.  |
 |.  __)   |.  | |____   | `-|.  |-'     |.  |___  |.  |___  |.  |
 |:  |     |:  | |:  1   |   |:  |       |:  1   | |:  1   | |:  |
 |::.|     |::.| |::.. . |   |::.|       |::.. . | |::.. . | |::.|
 `---'     `---' `-------'   `---'       `-------' `-------' `---'
"#;
const FIST_BANNER30: &str = r#"

     ,                                                                         
     Et                                                                        
     E#t                     .                         .,                      
     E##t      t            ;W                        ,Wt             i     t  
     E#W#t     Ej          f#E  GEEEEEEEL            i#D.            LE     Ej 
     E#tfL.    E#,       .E#f   ,;;L#K;;.           f#f             L#E     E#,
     E#t       E#t      iWW;       t#E            .D#i             G#W.     E#t
  ,ffW#Dffj.   E#t     L##Lffi     t#E           :KW,             D#K.      E#t
   ;LW#ELLLf.  E#t    tLLG##L      t#E           t#f             E#K.       E#t
     E#t       E#t      ,W#i       t#E            ;#G          .E#E.        E#t
     E#t       E#t     j#E.        t#E             :KE.       .K#E          E#t
     E#t       E#t   .D#j          t#E              .DW:     .K#D           E#t
     E#t       E#t  ,WK,           t#E                L#,   .W#G            E#t
     E#t       E#t  EG.             fE                 jt  :W##########Wt   E#t
     ;#t       ,;.  ,                :                     :,,,,,,,,,,,,,.  ,;.
      :;                                                                       

"#;
const FIST_BANNER31: &str = r#"

)`-.--. .'(    )\.--.  .-,.-.,-.         )\.-.   .')      .'(  
) ,-._( \  )  (   ._.' ) ,, ,. (       ,' ,-,_) ( /       \  ) 
\ `-._  ) (    `-.`.   \( |(  )/      (  .   _   ))       ) (  
 ) ,_(  \  )  ,_ (  \     ) \          ) '..' )  )'._.-.  \  ) 
(  \     ) \ (  '.)  )    \ (         (  ,   (  (       )  ) \ 
 ).'      )/  '._,_.'      )/          )/'._.'   )/,__.'    )/ 
"#;
const FIST_BANNER32: &str = r#"
    ____     __      ___     ____        ___       _         __  
   F ___J    FJ     F __".  /_  _\     ,"___".    FJ         FJ  
  J |___:   J  L   J (___|  [J  L]     FJ---L]   J |        J  L 
  | _____|  |  |   J\___ \   |  |     J |   LJ   | |        |  | 
  F |____J  F  J  .--___) \  F  J     | \___--.  F L_____   F  J 
 J__F      J____L J\______J J____L    J\_____/F J________L J____L
 |__|      |____|  J______F |____|     J_____F  |________| |____|
"#;

const FIST_BANNER33: &str = r#"

   ▄████████  ▄█     ▄████████     ███           ▄████████  ▄█        ▄█  
  ███    ███ ███    ███    ███ ▀█████████▄      ███    ███ ███       ███  
  ███    █▀  ███▌   ███    █▀     ▀███▀▀██      ███    █▀  ███       ███▌ 
 ▄███▄▄▄     ███▌   ███            ███   ▀      ███        ███       ███▌ 
▀▀███▀▀▀     ███▌ ▀███████████     ███          ███        ███       ███▌ 
  ███        ███           ███     ███          ███    █▄  ███       ███  
  ███        ███     ▄█    ███     ███          ███    ███ ███▌    ▄ ███  
  ███        █▀    ▄████████▀     ▄████▀        ████████▀  █████▄▄██ █▀   
                                                           ▀              
"#;

const FIST_BANNER34: &str = r#"

▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄▄▄       ▄▄▄▄▄▄▄▄▄▄▄ ▄           ▄▄▄▄▄▄▄▄▄▄▄ 
▐░░░░░░░░░░░▐░░░░░░░░░░░▐░░░░░░░░░░░▐░░░░░░░░░░░▌     ▐░░░░░░░░░░░▐░▌         ▐░░░░░░░░░░░▌
▐░█▀▀▀▀▀▀▀▀▀ ▀▀▀▀█░█▀▀▀▀▐░█▀▀▀▀▀▀▀▀▀ ▀▀▀▀█░█▀▀▀▀      ▐░█▀▀▀▀▀▀▀▀▀▐░▌          ▀▀▀▀█░█▀▀▀▀ 
▐░▌              ▐░▌    ▐░▌              ▐░▌          ▐░▌         ▐░▌              ▐░▌     
▐░█▄▄▄▄▄▄▄▄▄     ▐░▌    ▐░█▄▄▄▄▄▄▄▄▄     ▐░▌          ▐░▌         ▐░▌              ▐░▌     
▐░░░░░░░░░░░▌    ▐░▌    ▐░░░░░░░░░░░▌    ▐░▌          ▐░▌         ▐░▌              ▐░▌     
▐░█▀▀▀▀▀▀▀▀▀     ▐░▌     ▀▀▀▀▀▀▀▀▀█░▌    ▐░▌          ▐░▌         ▐░▌              ▐░▌     
▐░▌              ▐░▌              ▐░▌    ▐░▌          ▐░▌         ▐░▌              ▐░▌     
▐░▌          ▄▄▄▄█░█▄▄▄▄ ▄▄▄▄▄▄▄▄▄█░▌    ▐░▌          ▐░█▄▄▄▄▄▄▄▄▄▐░█▄▄▄▄▄▄▄▄▄ ▄▄▄▄█░█▄▄▄▄ 
▐░▌         ▐░░░░░░░░░░░▐░░░░░░░░░░░▌    ▐░▌          ▐░░░░░░░░░░░▐░░░░░░░░░░░▐░░░░░░░░░░░▌
 ▀           ▀▀▀▀▀▀▀▀▀▀▀ ▀▀▀▀▀▀▀▀▀▀▀      ▀            ▀▀▀▀▀▀▀▀▀▀▀ ▀▀▀▀▀▀▀▀▀▀▀ ▀▀▀▀▀▀▀▀▀▀▀ 
                                                                                           
"#;
const FIST_BANNER35: &str = r#"

·▄▄▄▪  .▄▄ · ▄▄▄▄▄     ▄▄· ▄▄▌  ▪  
▐▄▄·██ ▐█ ▀. •██      ▐█ ▌▪██•  ██ 
██▪ ▐█·▄▀▀▀█▄ ▐█.▪    ██ ▄▄██▪  ▐█·
██▌.▐█▌▐█▄▪▐█ ▐█▌·    ▐███▌▐█▌▐▌▐█▌
▀▀▀ ▀▀▀ ▀▀▀▀  ▀▀▀     ·▀▀▀ .▀▀▀ ▀▀▀


"#;
const FIST_BANNER36: &str = r#"

  █████▒ ██▓  ██████ ▄▄▄█████▓    ▄████▄   ██▓     ██▓
▓██   ▒ ▓██▒▒██    ▒ ▓  ██▒ ▓▒   ▒██▀ ▀█  ▓██▒    ▓██▒
▒████ ░ ▒██▒░ ▓██▄   ▒ ▓██░ ▒░   ▒▓█    ▄ ▒██░    ▒██▒
░▓█▒  ░ ░██░  ▒   ██▒░ ▓██▓ ░    ▒▓▓▄ ▄██▒▒██░    ░██░
░▒█░    ░██░▒██████▒▒  ▒██▒ ░    ▒ ▓███▀ ░░██████▒░██░
 ▒ ░    ░▓  ▒ ▒▓▒ ▒ ░  ▒ ░░      ░ ░▒ ▒  ░░ ▒░▓  ░░▓  
 ░       ▒ ░░ ░▒  ░ ░    ░         ░  ▒   ░ ░ ▒  ░ ▒ ░
 ░ ░     ▒ ░░  ░  ░    ░         ░          ░ ░    ▒ ░
         ░        ░              ░ ░          ░  ░ ░  
                                 ░                    

"#;
const FIST_BANNER37: &str = r#"

  sSSs   .S    sSSs  sdSS_SSSSSSbs          sSSs  S.       .S  
 d%%SP  .SS   d%%SP  YSSS~S%SSSSSP         d%%SP  SS.     .SS  
d%S'    S%S  d%S'         S%S             d%S'    S%S     S%S  
S%S     S%S  S%|          S%S             S%S     S%S     S%S  
S&S     S&S  S&S          S&S             S&S     S&S     S&S  
S&S_Ss  S&S  Y&Ss         S&S             S&S     S&S     S&S  
S&S~SP  S&S  `S&&S        S&S             S&S     S&S     S&S  
S&S     S&S    `S*S       S&S             S&S     S&S     S&S  
S*b     S*S     l*S       S*S             S*b     S*b     S*S  
S*S     S*S    .S*P       S*S             S*S.    S*S.    S*S  
S*S     S*S  sSS*S        S*S              SSSbs   SSSbs  S*S  
S*S     S*S  YSS'         S*S               YSSP    YSSP  S*S  
SP      SP                SP                              SP   
Y       Y                 Y                               Y    
"#;
const FIST_BANNER38: &str = r#"

.sSSSSs.    SSSSS .sSSSSs.       .sSSSSSSSSs.        .sSSSSs.    SSSSS       SSSSS 
SSSSSSSSSs. SSSSS SSSSSSSSSs. .sSSSSSSSSSSSSSs.      SSSSSSSSSs. SSSSS       SSSSS 
S SSS SSSS' S SSS S SSS SSSS' SSSSS S SSS SSSSS      S SSS SSSSS S SSS       S SSS 
S  SS       S  SS S  SS       SSSSS S  SS SSSSS      S  SS SSSS' S  SS       S  SS 
S..SSsss    S..SS `SSSSsSSSa. `:S:' S..SS `:S:'      S..SS       S..SS       S..SS 
S:::SSSS    S:::S .sSSS SSSSS       S:::S            S:::S SSSSS S:::S       S:::S 
S;;;S       S;;;S S;;;S SSSSS       S;;;S            S;;;S SSSSS S;;;S       S;;;S 
S%%%S       S%%%S S%%%S SSSSS       S%%%S            S%%%S SSSSS S%%%S SSSSS S%%%S 
SSSSS       SSSSS SSSSSsSSSSS       SSSSS            SSSSSsSSSSS SSSSSsSS;:' SSSSS 
"#;

const FIST_BANNER39: &str = r#"
 ________ .-./`)    .-'''-. ,---------.             _______    .---.    .-./`)  
|        |\ .-.')  / _     \\          \           /   __  \   | ,_|    \ .-.') 
|   .----'/ `-' \ (`' )/`--' `--.  ,---'          | ,_/  \__),-./  )    / `-' \ 
|  _|____  `-'`"`(_ o _).       |   \           ,-./  )      \  '_ '`)   `-'`"` 
|_( )_   | .---.  (_,_). '.     :_ _:           \  '_ '`)     > (_)  )   .---.  
(_ o._)__| |   | .---.  \  :    (_I_)            > (_)  )  __(  .  .-'   |   |  
|(_,_)     |   | \    `-'  |   (_(=)_)          (  .  .-'_/  )`-'`-'|___ |   |  
|   |      |   |  \       /     (_I_)            `-'`-'     /  |        \|   |  
'---'      '---'   `-...-'      '---'              `._____.'   `--------`'---'  
                                                                                
"#;

// const FIST_BANNER: &str = r#"

// "#;
// const FIST_BANNER: &str = r#"

// "#;

pub const MIAMI_FACTION_SETTINGS: &[&str] = &[
    MIAMI_NIGHTS, MIAMI_SPLICE, MIAMI_FRIGHTS,
    MIAMI_KNIGHTS, MIAMI_SLICE, MIAMI_TIGHTS,
    MIAMI_DICE,
];

pub const MIAMI_NIGHTS: &str = r#"
     e    e      888      e           e    e      888  
    d8b  d8b     888     d8b         d8b  d8b     888  
   d888bdY88b    888    /Y88b       d888bdY88b    888  
  / Y88Y Y888b   888   /  Y88b     / Y88Y Y888b   888  
 /   YY   Y888b  888  /____Y88b   /   YY   Y888b  888  
/          Y888b 888 /      Y88b /          Y888b 888  
                                                       
    888b    | 888  e88~~\  888   | ~~~888~~~ ,d88~~\   
    |Y88b   | 888 d888     888___|    888    8888      
    | Y88b  | 888 8888 __  888   |    888    `Y88b     
    |  Y88b | 888 8888   | 888   |    888     `Y88b,   
    |   Y88b| 888 Y888   | 888   |    888       8888   
    |    Y888 888  "88__/  888   |    888    \__88P'   
                                                       
"#;
pub const MIAMI_SPLICE: &str = r#"
     e    e      888      e           e    e      888 
    d8b  d8b     888     d8b         d8b  d8b     888 
   d888bdY88b    888    /Y88b       d888bdY88b    888 
  / Y88Y Y888b   888   /  Y88b     / Y88Y Y888b   888 
 /   YY   Y888b  888  /____Y88b   /   YY   Y888b  888 
/          Y888b 888 /      Y88b /          Y888b 888 
                                                      
      ,d88~~\ 888~-_   888     888  e88~-_  888~~     
      8888    888   \  888     888 d888   \ 888___    
      `Y88b   888    | 888     888 8888     888       
       `Y88b, 888   /  888     888 8888     888       
         8888 888_-~   888     888 Y888   / 888       
      \__88P' 888      888____ 888  "88_-~  888___    
                                                      
"#;
pub const MIAMI_FRIGHTS: &str = r#"
     e    e      888      e           e    e      888  
    d8b  d8b     888     d8b         d8b  d8b     888  
   d888bdY88b    888    /Y88b       d888bdY88b    888  
  / Y88Y Y888b   888   /  Y88b     / Y88Y Y888b   888  
 /   YY   Y888b  888  /____Y88b   /   YY   Y888b  888  
/          Y888b 888 /      Y88b /          Y888b 888  
                                                       
888~~  888~-_   888  e88~~\  888   | ~~~888~~~ ,d88~~\ 
888___ 888   \  888 d888     888___|    888    8888    
888    888    | 888 8888 __  888   |    888    `Y88b   
888    888   /  888 8888   | 888   |    888     `Y88b, 
888    888_-~   888 Y888   | 888   |    888       8888 
888    888 ~-_  888  "88__/  888   |    888    \__88P' 
                                                       
"#;
pub const MIAMI_SLICE: &str = r#"
     e    e      888      e           e    e      888 
    d8b  d8b     888     d8b         d8b  d8b     888 
   d888bdY88b    888    /Y88b       d888bdY88b    888 
  / Y88Y Y888b   888   /  Y88b     / Y88Y Y888b   888 
 /   YY   Y888b  888  /____Y88b   /   YY   Y888b  888 
/          Y888b 888 /      Y88b /          Y888b 888 
                                                      
        ,d88~~\ 888     888  e88~-_  888~~            
        8888    888     888 d888   \ 888___           
        `Y88b   888     888 8888     888              
         `Y88b, 888     888 8888     888              
           8888 888     888 Y888   / 888              
        \__88P' 888____ 888  "88_-~  888___           
                                                      
"#;
pub const MIAMI_TIGHTS: &str = r#"
     e    e      888      e           e    e      888 
    d8b  d8b     888     d8b         d8b  d8b     888 
   d888bdY88b    888    /Y88b       d888bdY88b    888 
  / Y88Y Y888b   888   /  Y88b     / Y88Y Y888b   888 
 /   YY   Y888b  888  /____Y88b   /   YY   Y888b  888 
/          Y888b 888 /      Y88b /          Y888b 888 
                                                      
  ~~~888~~~ 888  e88~~\  888   | ~~~888~~~ ,d88~~\    
     888    888 d888     888___|    888    8888       
     888    888 8888 __  888   |    888    `Y88b      
     888    888 8888   | 888   |    888     `Y88b,    
     888    888 Y888   | 888   |    888       8888    
     888    888  "88__/  888   |    888    \__88P'    
                                                      
"#;
pub const MIAMI_KNIGHTS: &str = r#"
     e    e      888      e           e    e      888     
    d8b  d8b     888     d8b         d8b  d8b     888     
   d888bdY88b    888    /Y88b       d888bdY88b    888     
  / Y88Y Y888b   888   /  Y88b     / Y88Y Y888b   888     
 /   YY   Y888b  888  /____Y88b   /   YY   Y888b  888     
/          Y888b 888 /      Y88b /          Y888b 888     
                                                          
888  /   888b    | 888  e88~~\  888   | ~~~888~~~ ,d88~~\ 
888 /    |Y88b   | 888 d888     888___|    888    8888    
888/\    | Y88b  | 888 8888 __  888   |    888    `Y88b   
888  \   |  Y88b | 888 8888   | 888   |    888     `Y88b, 
888   \  |   Y88b| 888 Y888   | 888   |    888       8888 
888    \ |    Y888 888  "88__/  888   |    888    \__88P' 
                                                          
"#;
pub const MIAMI_DICE: &str = r#"
     e    e      888      e           e    e      888 
    d8b  d8b     888     d8b         d8b  d8b     888 
   d888bdY88b    888    /Y88b       d888bdY88b    888 
  / Y88Y Y888b   888   /  Y88b     / Y88Y Y888b   888 
 /   YY   Y888b  888  /____Y88b   /   YY   Y888b  888 
/          Y888b 888 /      Y88b /          Y888b 888 
                                                      
            888~-_   888  e88~-_  888~~               
            888   \  888 d888   \ 888___              
            888    | 888 8888     888                 
            888    | 888 8888     888                 
            888   /  888 Y888   / 888                 
            888_-~   888  "88_-~  888___              
                                                      
"#;




// const FIST_BANNER: &str = r#"

// "#;
//💊💉
const NPC_EMOJIS: &[&str] = &[
    "👦","👧","👨","👩","👱","👲","👳","👴","👵","👷",
    "👸","🧔","🧔‍♂️","👨‍🦰","👨‍🦱","👨‍🦳","👨‍🦲","👩","👩‍🦰","🧑‍🦰",
    "👩‍🦱","🧑‍🦱","🧑‍🦳","👩‍🦲","🧓","🙎‍♂️","🙎‍♀️","🧑‍🌾","🧑‍🔧","🧑‍🏭",
    "🧑‍💼","🧑‍🔬","🧑‍🎤","👨‍🎤","🧑‍🎨","🧑‍✈️","🥷","👷","👲","🤵",
    "🧙","🧚","🧛","🧝","🧞","🧟","🧍","🧍‍♂️","🧍‍♀️","🧖",
    // "","","","","","","","","","",
    // "","","","","","","","","","",
    // "","","","","","","","","","",
    // "","","","","","","","","","",
];



pub fn print_help(help: &str) {
    for line in help.split('\n') {
        println!("{}",Color::Green.bold().paint(line));
    }    
}


pub const CLI_DIR: &str = r#"
Mode         Bytes    #    Name
-------------------------------------------------------
 🔎  1  fist 9949     36   roles
 🔎  1  fist 97512    216  traits
 📁  12 fist   -       -   matrices/
 🔎  1  fist 7794     00   matrices/cassettes
 🔎  1  fist 9024     00   matrices/cassette_links
 🔎  1  fist 8862     14   matrices/misc
 🔎  1  fist 8070     5    matrices/mission_generator
 🔎  1  fist 10241    1    matrices/mission_prompts
 📁  14 fist   -       -   matrices/characters/
 🔎  1  fist 3650     4    characters/animals
 🔎  1  fist 4265     4    characters/anomalies
 🔎  1  fist 4194     4    characters/celebrities
 🔎  1  fist 3988     4    characters/civilians
 🔎  1  fist 4619     4    characters/experiments
 🔎  1  fist 4058     4    characters/monster
 🔎  1  fist 3963     5    characters/politicians
 🔎  1  fist 24947    00   characters/premade_enemies
 🔎  1  fist 25261    00   characters/premade_npcs
 🔎  1  fist 4520     4    characters/robots
 🔎  1  fist 4215     5    characters/scientists
 🔎  1  fist 3750     4    characters/soldiers
 🔎  1  fist 4254     5    characters/spies
 🔎  1  fist 4588     5    characters/squads
 📁  2  fist   -       -   matrices/cyclops/
 🔎  -  fist 5226    matrices/cyclops/gadgets
 🔎  -  fist 2568    matrices/cyclops/rumors
 📁  6  fist   -       -     matrices/factions/
 🔎  1  fist     factions/agencies
 🔎  1  fist     matrices/factions/aliens
 🔎  1  fist     matrices/factions/corporations
 🔎  1  fist     matrices/factions/criminals
 🔎  1  fist     matrices/factions/cults
 🔎  1  fist     matrices/factions/insurgents
 📁  4  fist   -       -     matrices/gear/
 🔎  1  fist     matrices/gear/base_upgrades
 🔎  1  fist     matrices/gear/items
 🔎  1  fist     matrices/gear/vehicles
 🔎  1  fist     matrices/gear/weapons_and_armor
 📁  6  fist   -       -     matrices/locations/
 🔎  1  fist     matrices/locations/battlefields
 🔎  1  fist     matrices/locations/cities
 🔎  1  fist     matrices/locations/nature
 🔎  1  fist     matrices/locations/rooms
 🔎  1  fist     matrices/locations/structures
 🔎  1  fist     matrices/locations/zones
 📁  6  fist   -       -     matrices/lore/
 🔎  1  fist     matrices/lore/artifacts
 🔎  1  fist     matrices/lore/coverups
 🔎  1  fist     matrices/lore/diplomacy
 🔎  1  fist     matrices/lore/disasters
 🔎  1  fist     matrices/lore/legends
 🔎  1  fist     matrices/lore/spells
 📁  1  fist   -       -     matrices/world/
 🔎  1  fist     world/hazards
"#;

pub const PREMADE_ENEMY: &str = r#"
╔═══════════════════════════════════════════════════════════════════════════════════╗
║ ANTAREAN WARPRIEST (6 HP)                                                         ║
╠═══════════════════════════════════════════════════════════════════════════════════╣
║ * Laser rifle, long, slim, and chrome (1D6+1 DAMAGE)                              ║
║ * Metallic staff (1D6 DAMAGE, NON-LETHAL)                                         ║
║ * Battle-trance herbs (stop the trance, one use)                                  ║
╠═══════════════════════════════════════════════════════════════════════════════════╣
║ TRANCE: Antarean Warpriests can be found in stasis, frozen in drop pods planted   ║
║ on Earth from orbit 10,000 years ago, never opened due to a small but detrimental ║
║ programming error. Upon leaving stasis, Warpriests enter a battle-trance which    ║
║ grants them 3 ARMOR for as long as it lasts. Only the herbs can break the trance. ║
╚═══════════════════════════════════════════════════════════════════════════════════╝
"#;

// other diffs
// base_upgrades: Title: "Base Upgrades", Roll: "1D6", Text: 

pub const FIST_BANNERS: &[&str] = &[
    FIST_BANNER1, FIST_BANNER2, FIST_BANNER3, FIST_BANNER4,
    FIST_BANNER5, FIST_BANNER6, FIST_BANNER7, FIST_BANNER8,
    FIST_BANNER9, FIST_BANNER10, FIST_BANNER11, FIST_BANNER12,
    FIST_BANNER13, FIST_BANNER14, FIST_BANNER15, FIST_BANNER16,
    FIST_BANNER17, FIST_BANNER18, FIST_BANNER19, FIST_BANNER20,
    FIST_BANNER21, FIST_BANNER22, FIST_BANNER23, FIST_BANNER24,
    FIST_BANNER25, FIST_BANNER26, FIST_BANNER27, FIST_BANNER28,
    FIST_BANNER29, FIST_BANNER30, FIST_BANNER31, FIST_BANNER32,
    FIST_BANNER33, FIST_BANNER34, FIST_BANNER35, FIST_BANNER36,
    FIST_BANNER37, FIST_BANNER38, FIST_BANNER39,
    
];

pub fn random_banner() -> &'static str {
    let mut rng = rand::thread_rng();
    let rnd_idx = rng.gen_range(0..FIST_BANNERS.len());
    FIST_BANNERS[rnd_idx]
}
pub fn random_promo() {
    let mut rng = rand::thread_rng();
    let rnd_idx = rng.gen_range(0..MIAMI_FACTION_SETTINGS.len());
    
    match rnd_idx {
        0 => { // nights
            println!("{}{} 🧛👻 A {} Setting for neon \'80s {}{}{} and other {} creatures.",
                Color::LightYellow.paint("╠═════╡ "),
                Color::Magenta.bold().on(Color::Cyan).paint("MIAMI NIGHTS"),
                Color::White.bold().paint("FIST Factions"),
                Color::Blue.bold().paint("vampires, werewolves,\n"),
                Color::LightYellow.paint("╚═════╡ "),
                Color::Blue.bold().paint("witches"),
                Color::LightYellow.italic().paint("supernatural night"),
            );
        }
        1 => { // splice
            println!("{} 🧛 A {} Setting for neon \'80s {}, and other {} factions.",
                Color::Magenta.bold().on(Color::Cyan).paint("MIAMI SPLICE"),
                Color::White.bold().paint("FIST Factions"),
                Color::Blue.bold().paint("<TBD>, <TBD>,\n<TBD>"),
                Color::LightYellow.italic().paint("<TBD>"),
            ); 
        }
        2 => { // frights
            println!("{} 👹👺 A {} Setting for neon \'80s {}, and other {} horrors.",
                Color::Magenta.bold().on(Color::Cyan).paint("MIAMI FRIGHTS"),
                Color::White.bold().paint("FIST Factions"),
                Color::Blue.bold().paint("old gods, ancient ones,\ncults"),
                Color::LightYellow.italic().paint("ancient and eldritch"),
                
            );         
        }
        3 => { // knights
            println!("{} ⚔️ A {} Setting for neon \'80s {}, and other {} factions.",
                Color::Magenta.bold().on(Color::Cyan).paint("MIAMI KNIGHTS"),
                Color::White.bold().paint("FIST Factions"),
                Color::Blue.bold().paint("<TBD>, <TBD>,\n<TBD>"),
                Color::LightYellow.italic().paint("<TBD>"),
            ); 
        }
        4 => { // slice
            println!("{} 👩‍💻👾 A {} Setting for neon \'80s {}, and other {} factions.",
                Color::Magenta.bold().on(Color::Cyan).paint("MIAMI SLICE"),
                Color::White.bold().paint("FIST Factions"),
                Color::Blue.bold().paint("crackers, phreakers,\nwarez couriers"),
                Color::LightYellow.italic().paint("dialup cyberpunk"),
                
            ); 
        }
        5 => { // tights
            println!("{} 🦸‍♀️🦹‍♂️ A {} Setting for neon \'80s {}, and other {} vigilantes.",
                Color::Magenta.bold().on(Color::Cyan).paint("MIAMI TIGHTS"),
                Color::White.bold().paint("FIST Factions"),
                Color::Blue.bold().paint("super hero, villains,\nmasterminds"),
                Color::LightYellow.italic().paint("cape and tights"),
                
            ); 
        }
        6 => { // dice
            println!("{} 👠 A {} Setting for neon \'80s {}, and other {} organizations.",
                Color::Magenta.bold().on(Color::Cyan).paint("MIAMI DICE"),
                Color::White.bold().paint("FIST Factions"),
                Color::Blue.bold().paint("hookers, drug cartels,\nvice crimelords"),
                Color::LightYellow.italic().paint("seedy underbelly"),
                
            ); 
        }
        _ => {
            // println!("{} 🧛 A {} Setting for neon \'80s vampires, werewolves,\nwitches, and other night creatures.",
            //     Color::Magenta.bold().on(Color::Cyan).paint("MIAMI NIGHTS"),
            //     Color::White.bold().paint("FIST Factions")); 
        }
    }
    for (num,line) in MIAMI_FACTION_SETTINGS[rnd_idx].split('\n').enumerate() {
        if num <=6 {
            println!("{}",Color::Cyan.bold().paint(line));
        } else {
            println!("{}",Color::Magenta.bold().on(Color::Cyan).paint(line));
        }
    }    
}


#[cfg(test)]
pub mod test {
    use std::{env, fs};
    use regex::Regex;

    use crate::{all_jsons, all_matrices, fist::{Matrix, Role}, get_matrix_by_title, json_exists, load_matrix, load_roles, load_traits, DicePool, Die, CYCLOPS_GADGETS, JSON_DIR, ROLES, TRAITS};


    #[test]
    fn std_dice_vals() {
        let d6 = Die::d6();
        assert_eq!("1d6", d6.to_string());
        let d8 = Die::d8();
        assert_eq!("1d8", d8.to_string());
        let d10 = Die::d10();
        assert_eq!("1d10", d10.to_string());
        let d12 = Die::d12();
        assert_eq!("1d12", d12.to_string());
        let d20 = Die::d20();
        assert_eq!("1d20", d20.to_string());
        let d100 = Die::d100();
        assert_eq!("1d100", d100.to_string());

        let mut rng = rand::thread_rng();
        let pool = DicePool::new_dice(vec![Die::d6(),Die::d6(),Die::d10()]);
        let result = pool.rand(&mut rng) as u16;
        println!("Dice Pool: {pool} Throw: {result}")
        
    }
    #[test]
    fn test_all_jsons() {
        println!("pwd:{ROLES} json_exists() -> {}",json_exists(ROLES));
        assert!(json_exists(ROLES));

        for json in all_jsons() {
            assert!(json_exists(json));
            println!("{json} EXISTS!");
        }
        
        for json in all_matrices() {
            assert!(json_exists(json));
            println!("{json} EXISTS!");
        }
    }

    #[test]  
    fn title_test() {
        match get_matrix_by_title(&("WEAPONS".to_string())) {
            Some(m) => println!("{:?}",m),
            None => println!("Couldn't find it")
        }
        match get_matrix_by_title(&("COMMON ITEMS".to_string())) {
            Some(m) => println!("{:?}",m),
            None => println!("Couldn't find it")
        }

    }

    #[test]  
    fn traits_test() {
        let traits = ok!(load_traits(TRAITS));
        let re1 = Regex::new(r"^(.*?)\s(.*)$").unwrap();
        let re2 = Regex::new(r" ").unwrap();
        let str = traits[9].Stat.as_str();
        let result1: Vec<_> = re1.splitn(str,2).collect();
        println!("Stat parsed: {}({})", result1[0],result1[1]);        
        let result1: Vec<_> = re2.splitn(str,2).collect();
        println!("Stat parsed: {}({})", result1[1],result1[0]);
        let val: i8 = result1[0].parse().unwrap();
        let stat: u8 = 6;
        println!("{stat} + {val} = {}",stat.wrapping_add_signed(val));
    }


    #[test]  
    fn file_test() {
        let roles = ok!(load_roles(ROLES));
        let traits = ok!(load_traits(TRAITS));
        let mut tables: Vec<Matrix> = vec![];
        //TODO pre-mades, cassette, and cassette_links
        
        for m in all_matrices() {
            let mut matrices = load_matrix(m).unwrap();
            println!("Loaded {} entries from {m}",matrices.len());

            tables.append(&mut matrices);
        }
        let gadgets = load_matrix(CYCLOPS_GADGETS).unwrap();
        println!("Loaded {} entries from {CYCLOPS_GADGETS}",gadgets.len());
        println!("{:?}",gadgets);
        //tables.append(&mut gadgets);

        println!("Loaded {} traits, {} roles, {} matrices", traits.len(),roles.len(), tables.len());
        let rns: Vec<String> = roles.into_iter().map(|r| { format!("{} : {}\n{}", r.Number,r.Name,r.Text)  }).collect();
        for s in rns {
            println!("{}",s);
        }
    }

    #[test]  
    fn init_test() {
        let n = JSON_DIR.to_owned() + "roles.json";
        let mut data = fs::read_to_string(n).unwrap();
        let roles: Vec<Role> = serde_json::from_str(&data).unwrap();
    
        let r = &roles[12];
    
        println!("{} : {}\n{}",r.Number,r.Name,r.Text);




          
    }  
}
