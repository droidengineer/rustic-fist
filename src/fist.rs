//! FIST
//! 
use nu_ansi_term::{AnsiString, AnsiStrings, Color};
use rand::Rng;
use serde::{Serialize,Deserialize};
use serde_json::Value;

use std::{collections::HashMap, fmt::Display};

use crate::break_into;


#[derive(Debug,Serialize, Deserialize)]
pub struct Trait {
    pub Number: u16,
    pub Name: String,
    pub Effect: String,
    pub Item: String,
    pub Stat: String,
}

//pub use crate::Role;
#[derive(Debug,Serialize, Deserialize)]
pub struct Role {
    pub Number: u16,
    pub Name: String,
    pub Text: String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Cassette {
    pub table: Vec<String>,
    pub links: HashMap<String,String>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Premade {
    pub Head: String,
    pub Features: Vec<String>,
    pub Notes: Vec<String>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Gadget {
    pub Title: String,
    pub Roll: String,
    
    #[serde(flatten)]
    pub Values: HashMap<String,Value>,
}

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct Matrix {
    pub Title: String,
    pub Roll: String,
    pub Values: HashMap<u16,Value>,
}


pub struct Item {
    name: String,
    roll: u16,
    count: i8,
    kind: String, // Title
    tags: Option<Vec<String>>,
}

#[derive(Default,Debug)]
pub struct Attributes {
    pub(crate) forceful: i8,
    pub(crate) creative: i8,
    pub(crate) tactical: i8,
    pub(crate) reflexive: i8,
    pub(crate) armor: i8,
    pub(crate) hp: (i8,i8),
    pub(crate) wardice: i8,
}

pub struct Character {
    pub stats: Attributes,
    traits: Vec<Trait>,
    pub codename: String,
    pub realname: String,
    role: Role,
    pub appearance: String,
    inventory: Vec<String>,
}


//////////////////////[ IMPLEMENTATIONS ]
impl Matrix {

    pub fn print_sorted_map(&self) {
        let mut keys: Vec<u16> = self.Values.keys().cloned().collect();
        keys.sort();

        for key in keys {
            if let Some(value) = self.Values.get(&key) {
                println!("  {} {} {}",key ,Color::White.bold().paint("⇒"),value);
            }
        }
    }
    pub fn rand_value(&self) -> (u16, Value) {
        let keys: Vec<u16> = self.Values.keys().cloned().collect();
        let mut rng = rand::thread_rng();
        let rnd_idx = rng.gen_range(0..keys.len()) as u16;
        let idx = keys[rnd_idx as usize];
        (idx, self.Values[&idx].clone())
    }
}
impl Into<Item> for Matrix {
    fn into(self) -> Item {
        let n = String::from("TODO");
        let i = Item::new(self.Title,n, self.Roll.parse::<u16>().unwrap());
        i
    }
}

impl Item {
    pub fn new(kind: String, name: String, roll: u16) -> Self {
        Item {
            name,
            kind,
            roll,
            count: 0,
            tags: None,
        }
    }
}

impl Character {
    pub fn new(cn: &str, rn: &str) -> Self {
        Character {
            stats: Attributes::new(),
            traits: vec![],
            codename: cn.to_string(),
            realname: rn.to_string(),
            role: Role { Number: 0, Name: "".to_string(), Text: "".to_string()},
            appearance: "".to_string(),
            inventory: vec![],
        }
    }

    pub fn mod_attr(&mut self, attr: &str, val: i8) {
        match attr.to_lowercase().as_str() {
            "for"|"forceful" => {self.stats.forceful = self.stats.forceful.wrapping_add(val);},
            "tac"|"tactical" => {self.stats.tactical = self.stats.tactical.wrapping_add(val);},
            "cre"|"creative" => {self.stats.creative = self.stats.creative.wrapping_add(val);},
            "ref"|"reflexive" => {self.stats.reflexive = self.stats.reflexive.wrapping_add(val);},
            "hp" => {self.stats.hp.0 = self.stats.hp.0.wrapping_add(val);},
            "hp_max" => {self.stats.hp.1 = self.stats.hp.1.wrapping_add(val);},
            "wd"|"war dice" => {self.stats.wardice = self.stats.wardice.wrapping_add(val);},
            "armor"|"arm" => {self.stats.armor = self.stats.armor.wrapping_add(val);},
            _ => {}
        }
    }

    pub fn add_trait(&mut self, t: Trait) {
        // modify stat
        // TODO

        // add to inventory
        self.inventory.push(t.Item.clone());

        self.traits.push(t);
    }
    pub fn add_role(&mut self, r: Role) {
        self.role = r;
    }
    pub fn add_item(&mut self, i: Item) {
        self.inventory.push(i.name);
    }
    pub fn remove_item(&mut self, i: Item) {
       if let result = self.inventory.iter().any(|f| {*f == i.name })  {
        //TODO

       }
    }

}



impl Attributes {
    pub fn new() -> Self {
        Attributes {
            forceful: 0,
            creative: 0,
            tactical: 0,
            reflexive: 0,
            armor: 0,
            hp: (6,6),
            wardice: 0,
        }
    }
    pub fn reset(&mut self) {
        self.forceful = 0;
        self.creative = 0;
        self.tactical = 0;
        self.reflexive = 0;
        self.armor = 0;
        self.wardice = 0;
        self.hp = (6,6);
    }
    pub fn forceful(mut self, val: i8) -> Self {
        self.forceful = val;
        self
    }
    pub fn tactical(mut self, val: i8) -> Self {
        self.tactical = val;
        self
    }
    pub fn creative(mut self, val: i8) -> Self {
        self.creative = val;
        self
    }
    pub fn reflexive(mut self, val: i8) -> Self {
        self.reflexive = val;
        self
    }
    pub fn hp(mut self, val: i8) -> Self {
        self.hp.0 = val;
        self
    }
    pub fn hp_max(mut self, val: i8) -> Self {
        self.hp.1 = val;
        self
    }
    pub fn armor(mut self, val: i8) -> Self {
        self.armor = val;
        self
    }
    pub fn war_dice(mut self, val: i8) -> Self {
        self.wardice = val;
        self
    }
    pub fn hp_change(&mut self, delta: i8) -> i8 {
        self.hp.0 = self.hp.0.wrapping_add(delta);
        self.hp.0
    }

}

impl Display for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("[ FOR: {:02} TAC: {:02} CRE: {:02} REF: {:02} ",self.forceful,self.tactical,self.creative,self.reflexive);  
        result.push_str(format!("ARMOR: {:02} WAR DICE: {:02} HP CURR: {:02} MAX: {:02} ]\n", self.armor,self.wardice,self.hp.0,self.hp.1).as_str());

        write!(f,"{}",result)
    }
}


impl Display for Trait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt: &[AnsiString<'static>] = &[
            Color::White.bold().paint(self.Name.clone()),
            Color::LightGray.paint(format!(" \u{1f3b2}\u{1f3b2}\u{1f3b2}  [{}]\n",self.Number)),
            Color::Default.paint(self.Effect.clone()),
            Color::White.bold().paint("\n  ⦿"),
            Color::Cyan.bold().paint(format!(" {}",self.Item)),
            Color::White.bold().paint("\n  ⦿"),
            Color::Purple.bold().paint(format!(" {}",self.Stat)),
        ];
        writeln!(f,"{}",AnsiStrings(fmt))
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let fmttxt = break_into(&self.Text, 84).concat();
        let fmt: &[AnsiString<'static>] = &[
            // Color::LightYellow.bold().paint("║\n"),
            // Color::LightYellow.bold().paint("╠═════╡ "),
            Color::White.bold().paint(self.Name.clone()),
            Color::LightGray.paint(format!(" \u{1f3b2}\u{1f3b2}  [{}]\n",self.Number)),
            Color::Default.paint(self.Text.clone()),
        ];
        writeln!(f,"{}",AnsiStrings(fmt))

        // writeln!(f,"{}", AnsiStrings(&[
        //     Color::White.bold().paint("╔═════╡ "),
        //     Color::Yellow.bold().paint(self.Name.clone()),
        // ]))?;        
        // writeln!(f,"{}", AnsiStrings(&[
        //     Color::White.bold().paint(format!("║ \u{1f3b2}\u{1f3b2}  [{}]",self.Number)),
        // ]))?;
        // writeln!(f,"{}", AnsiStrings(&[
        //     Color::White.bold().paint("╠════» "),
        // ]))?;
        // writeln!(f,"{}", AnsiStrings(&[
        //     Color::White.bold().paint("║ "),
        //     Color::Default.paint(break_into(&self.Text, 84).concat()),         //self.Text.clone()),
        // ]))?;

        // writeln!(f,"{}",AnsiStrings(&[
        //     Color::White.bold().paint("╚═══»")
        // ]))
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dice = " \u{1f3b2}";
        match self.Roll.as_str() {
            "D66"|"2D6" => dice = " \u{1f3b2}\u{1f3b2}",
            "D666" => dice = " \u{1f3b2}\u{1f3b2}\u{1f3b2}",
            _ => {}
        }
        let fmt: &[AnsiString<'static>] = &[
            Color::White.bold().paint(self.Title.clone()),
            Color::LightGray.paint(format!(" {}  [{}]",dice,self.Roll,)),
            //Color::Default.paint(self.Values.),

        ];
        writeln!(f,"{}",AnsiStrings(fmt))?;
        self.print_sorted_map();
        Ok(())
    }
}

impl Display for Premade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let fmt: &[AnsiString<'static>] = &[
        //     Color::White.bold().paint(self.Head.clone()),
        //     Color::Cyan.bold().paint(format!("{:?}",self.Features)),
        //     Color::Magenta.bold().paint(format!("{:?}",self.Notes)),
        // ];
        writeln!(f,"{}",Color::White.bold().paint(self.Head.clone()))?;
        
        for feat in &self.Features {
            writeln!(f, " ⦿ {}", Color::Cyan.bold().paint(break_into(feat,84).concat()))?;
        }
        let fnotes = break_into(&self.Notes.concat(), 84);
        for note in fnotes {
            writeln!(f, "{}", Color::Magenta.bold().paint(note))?;
        }
        //writeln!(f,"{}","")
        Ok(())
    }
}













#[cfg(test)]
pub mod test {
    use super::Attributes;

    #[test]
    fn test_role() {
        
    }

    #[test]
    fn test_trait() {
        
    }

    #[test]
    fn test_character() {
        
    }

    #[test]  
    fn file_test() {
        let mut core_stats = Attributes::new();
        println!("{}",core_stats);
        core_stats.hp_change(-1);
        println!("{}",core_stats);
    }
}