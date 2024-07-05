//! PRELUDE
//! TODO docs


use std::{collections::HashMap, env, fmt::{Display, Error}, fs, io::{stdout, Write}, path::Path, str::FromStr};

use nu_ansi_term::{Color, Rgb};
use regex::Regex;
use serde::{Serialize,Deserialize};
use serde_json::{Result, Value};
use rand::Rng;

use crate::fist::{Matrix, Premade, Role, Trait};
use crate::inc::*;

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

macro_rules! ok(($result:expr) => ($result.unwrap()));

//const SteelBlue: Rgb = Rgb { r:70,g:130,b:180};

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

pub fn load_matrix(json: &str) -> Result<Vec<Matrix>> {
    let data = fs::read_to_string(json).unwrap();
    serde_json::from_str(&data)
}
pub fn load_premade(json: &'static str) -> Result<Vec<Premade>> {
    let data = ok!(fs::read_to_string(json));
    serde_json::from_str(&data)
}
pub fn load_roles(json: &'static str) -> Result<Vec<Role>> {
    //TODO validation
    let data = fs::read_to_string(json).unwrap();
    serde_json::from_str(&data)
}

pub fn load_traits(json: &'static str) -> Result<Vec<Trait>> {
    //TODO validation
    let data = fs::read_to_string(json).unwrap();
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
        write!(f,"1{}{}",Color::Yellow.paint("d"), Color::Blue.bold().paint(self.0.to_string()))
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
                Color::Green.bold().paint((counts[s]).to_string()), // * self.multiplier
                Color::Yellow.paint("d"),
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


pub fn home_banner() {
    println!("{}{}\x1b[0m {}\x1b[0m {}{}\x1b[0m  {}{}\x1b[0m      {}{}\x1b[0m {}\x1b[0m      {}\x1b[0m",
        Color::White.bold().on(Color::LightBlue).paint(":::"),
        Color::Red.bold().on(Color::White).paint("====="),
        Color::White.bold().on(Color::LightBlue).paint(":::"),
        Color::White.bold().on(Color::LightBlue).paint(":::"),
        Color::Red.bold().on(Color::White).paint("==="),
        Color::White.bold().on(Color::LightBlue).paint(":::"),
        Color::Red.bold().on(Color::White).paint("===="),

        Color::White.bold().on(Color::LightBlue).paint(":::"),
        Color::Red.bold().on(Color::White).paint("====="),
        Color::White.bold().on(Color::LightBlue).paint(":::"),
        Color::White.bold().on(Color::LightBlue).paint(":::"),
    );
    println!("{}{} {} {}{}  {}{}      {}{} {}      {}",
    Color::White.bold().on(Color::LightBlue).paint(":::"),
    "     ", //Color::Red.bold().on(Color::White).paint("     "),
    Color::White.bold().on(Color::LightBlue).paint(":::"),
    Color::White.bold().on(Color::LightBlue).paint(":::"),
    "   ", //Color::Red.bold().on(Color::White).paint("   "),
    Color::White.bold().on(Color::LightBlue).paint(":::"),
    //"    ", //Color::Red.bold().on(Color::White).paint("    "),
    Color::Red.bold().on(Color::White).paint("===="),

    Color::White.bold().on(Color::LightBlue).paint(":::"),
    "     ", //Color::Red.bold().on(Color::White).paint("     "),
    Color::White.bold().on(Color::LightBlue).paint(":::"),
    Color::White.bold().on(Color::LightBlue).paint(":::"),
    );
    // 3
    println!("{}   {} {}    {}        {}      {}      {}",
    Color::Red.bold().on(Color::White).paint("======"),
    Color::Red.bold().on(Color::White).paint("==="),
    Color::Red.bold().on(Color::White).paint("======"),
    Color::Red.bold().on(Color::White).paint("==="),
    Color::Red.bold().on(Color::White).paint("==="),
    Color::Red.bold().on(Color::White).paint("==="),
    Color::Red.bold().on(Color::White).paint("==="),
    );
    println!("{}      {}     {}   {}        {}      {}      {}",
    Color::Red.bold().on(Color::White).paint("==="),
    Color::Red.bold().on(Color::White).paint("==="),
    Color::Red.bold().on(Color::White).paint("==="),
    Color::Red.bold().on(Color::White).paint("==="),
    Color::Red.bold().on(Color::White).paint("==="),
    Color::Red.bold().on(Color::White).paint("==="),
    Color::Red.bold().on(Color::White).paint("==="),
    );
    println!("{}      {} {}    {}         {} {} {}",
    Color::Red.bold().on(Color::White).paint("==="),
    Color::Red.bold().on(Color::White).paint("==="),
    Color::Red.bold().on(Color::White).paint("======"),
    Color::Red.bold().on(Color::White).paint("==="),
    Color::Red.bold().on(Color::White).paint("======="),
    Color::Red.bold().on(Color::White).paint("========"),
    Color::Red.bold().on(Color::White).paint("==="),
    );
    
// for line in banner.split('\n') {
//     println!("{line}");
// }

}

pub fn print_help(help: &str) {
    for line in help.split('\n') {
        println!("{}",Color::Default.paint(line));
    }    
}

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

    use crate::{all_matrices, fist::{Matrix, Role}, get_matrix_by_title, json_exists, load_matrix, load_roles, load_traits, DicePool, Die, ALL_FILES, CYCLOPS_GADGETS, JSON_DIR, ROLES, TRAITS};


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

        for json in ALL_FILES {
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
