use std::{collections::HashMap, fs, io::{self, stdout, Result, Write}, mem::size_of, str::FromStr};
use fist::Premade;
use nu_ansi_term::{enable_ansi_support, AnsiGenericString, AnsiString, AnsiStrings, Color::{self, *}, Style};



#[macro_use]
mod prelude;
use prelude::*;
mod inc;

use rand::Rng;
use serde_json::Value;


use crate::fist::{Role, Trait};
mod fist;

const APP_NAME: &str = "rustic-fist";
const VERSION: &str = "v0.1.1";

fn main() -> Result<()> {

    //let mut stdout = io::stdout();
    #[cfg(windows)]
    enable_ansi_support().unwrap();
    let mut dice_pool = DicePool::new_dice(vec![Die::d6(),Die::d6()]);

    banner();

    let traits = load_traits(TRAITS)?;
    let roles = load_roles(ROLES)?;
    let tl = traits.len();
    println!("Loaded {} {}, {} {}. Matrices loaded on demand.",Color::White.bold().paint(tl.to_string()),Color::Purple.bold().paint("traits"),Color::White.bold().paint(roles.len().to_string()),Color::Purple.bold().paint("roles"));
    println!();

    loop 
    {
        let mut input = String::new();

        prompt();

        io::stdin().read_line(&mut input)?;
        input = input.to_lowercase();
        let input = input.trim();

        if input == "quit" {
            break;
        }

        match evaluate_expression(input,&mut dice_pool) {
            Ok(_) => {},//println!("{}", result)
            Err(err) => println!("Error: {}", err),
        }
    }

    Ok(())
}

fn prompt() {
    let now = time::OffsetDateTime::now_local().unwrap().to_hms();       //now_utc().to_hms();
    //let tpb = format!("🤛👊╞═══════════════════════════════════════════╡");
    let ts = format!(" {:0>2}:{:0>2}:{:0>2}\n",now.0,now.1,now.2);
    //let tpb = "╔═══════════════════════════════════════════════════════════════════════════════════ \u{2728}\n";
    let p: &[AnsiString<'static>] = &[
        LightYellow.paint("╔════╡"),
        //LightYellow.paint("╔══╡✊🤜"),
        White.bold().paint("FIST:Ultimate Shell"),
        LightYellow.paint("╞═══════════════════════════════════════════╡"),
        Cyan.bold().paint(ts),
        //LightYellow.paint("║ "),
        //LightYellow.paint("╚»"),        
        LightYellow.paint("╚👊 "),
        //LightYellow.paint("╠╚✊️ " ),
        //Green.bold().paint("$ "),
    ];
    print!("{}",AnsiStrings(p));
    ok!(stdout().flush());
}

fn evaluate_expression(expr: &str, dice_pool: &mut DicePool) -> Result<String> {
    let mut tokens = expr.split_whitespace();
    let mut rng = rand::thread_rng();

    match tokens.next().unwrap() {
        "banner" => {
            println!("{}",Cyan.bold().paint(random_banner()));
        }
        "bullet" => {
            cmd_bullet();
        }
        "promo" => {
            random_promo();
        }
        "dicepool" => {
            match tokens.next() {
                Some(subcmd) => {
                    match subcmd {
                        "add" => {}
                        "clear" => {}
                        _ => {}
                    }
                }
                None => {
                    println!("{dice_pool}");
                }
            }
        }
        "roll" => {
            match tokens.next() {
                Some(dice_str) => {
                    println!("dice_str = {}",dice_str);
                    let dp = DicePool::default();
                    for dice in dice_str.split('+') {
                        println!("dice = {}",dice);
                        let dp2 = ok!(DicePool::from_str(dice));
                        println!("{dp2} = DicePool::from_str({})",dice);
                        println!("{dp2} throws {}",&dp2.rand(&mut rng));
                        stdout().flush().unwrap();

                        dp.add_dice(dp2.dice());
                        println!("{dp} after adding {dice}");        stdout().flush().unwrap();


                    }
                    println!("{dp} = {}",dp.rand(&mut rng));
                }
                None => {
                    //let pool = DicePool::new_dice(vec![Die::d6(),Die::d6()]);
                    let result = dice_pool.rand(&mut rng);
                    println!("Dice Pool: {dice_pool} Throw: {}",Color::White.bold().paint(result.to_string()))
                }
            }
        }
        "help" => {
            match tokens.next() {
                Some(subcmd) => {
                    match subcmd.to_lowercase().as_str() {
                        "ls" => {print_help(inc::HELP_CMD_LS)},
                        "search" => {print_help(inc::HELP_CMD_SEARCH)},
                        "random" => {print_help(inc::HELP_CMD_RANDOM)},
                        "roll" => {print_help(inc::HELP_CMD_ROLL)},
                        "dicepool" => {print_help(inc::HELP_CMD_DICEPOOL)}
                        _ => {
                            println!("There is no specific help for {subcmd}");
                        }
                    }
                },
                None => cmd_help(),
            }
        }
        "search" => {
            // if tokens.len() != 3 {
            //     return Ok(format!("Wrong number of arguments({}): search <arg> <target>",tokens.len()));
            // }
        if let Some(tok) = tokens.next() {
            match tok { //tokens.next().unwrap() {
                "role"|"roles" => {
                    if let Some(role) = tokens.next() { 
                        let role = role.to_uppercase();
                        print!("Searching {} for {}...",Color::White.bold().paint("ROLES"),Color::White.bold().paint(role.as_str()));
                        //let r: Vec<_> = load_roles(ROLES)?.iter().map(|role| role.Name == tokens[2].to_uppercase()).collect();
                        match load_roles(ROLES)?.iter().find(|r| r.Name == role.as_str()) {
                            Some(val) => println!("✅\n{}",val),
                            None => println!("Not found"),
                        }
                    } else {
                        println!("Wrong number of arguments.");
                    }
                }
                "trait"|"traits" => {
                    let traits = tokens.next().unwrap().to_uppercase();

                    print!("Searching {} for {}...",Color::White.bold().paint("TRAITS"),Color::White.bold().paint(&traits));
                    match load_traits(TRAITS)?.iter().find(|r| r.Name == traits) {
                        Some(val) => println!("✅\n{}",val),
                        None => println!("Not found"),
                    }
                }
                "gear" => {

                }
                "character" => {

                }
                "matrix" => {
                    let mut search = String::new();

                    match tokens.next() {
                        Some(srch) => {
                            search.push_str(srch.to_uppercase().as_str());
                            for t in tokens.by_ref() {
                                search.push(' ');
                                search.push_str(t.to_uppercase().as_str());
                            }
                        }
                        None => {
                            println!("Wrong number of arguments.");
                            return Ok("()".to_string());
                        }
                    }

                    println!("Searching {} for {}...",Color::White.bold().paint("MATRICES"),Color::White.bold().paint(&search));
                    match get_matrix_by_title(&search) {
                    // match get_matrix_by_title(tokens[2]) {
                        Some(t) => println!("{}",t),
                        None => println!("Not found"),
                    }
                }
                _ => {

                }
            }
        }}
    
        "test" => {


        }
        "random"|"rnd" => {
            match tokens.next() {
                Some(r) => {
                    match r {              
                        "role"|"roles" => {
                            println!("{}",rnd_role(load_roles(ROLES)?.as_ref()));
                        },
                        "trait"|"traits" => {
                            println!("{}",rnd_trait(load_traits(TRAITS)?.as_ref()));
                        },
                        "matrix" => {
                            let mut search = String::new();

                            match tokens.next() {
                                Some(srch) => {
                                    search.push_str(srch.to_uppercase().as_str());
                                    for t in tokens.by_ref() {
                                        search.push(' ');
                                        search.push_str(t.to_uppercase().as_str());
                                    }
                                }
                                None => {
                                    println!("Wrong number of arguments.");
                                    return Ok("()".to_string());
                                }
                            }
        
                            match get_matrix_by_title(&search) {
                                Some(m) => {
                                    println!("Found {search}");
                                    let (idx,value) = m.rand_value();
                                    println!(" 🎲🎲 {idx} => {}",value);
                                },
                                None => println!("Not found"),
                            }
            
                        },
                        "enemy" => {
                            //let mut data = fs::read_to_string(CHARACTERS_PREMADE_ENEMIES).unwrap();
                            //let npcs: Vec<Premade> = serde_json::from_str(&data)?;
                            let npcs = load_premade(CHARACTERS_PREMADE_ENEMIES)?;
                            let mut rng = rand::thread_rng();
                            let rnd_idx = rng.gen_range(0..npcs.len());
                            
                            println!("{}",npcs[rnd_idx]);
                        },
                        "npc" => {
                                let npcs = load_premade(CHARACTERS_PREMADE_NPCS)?;
                                let mut rng = rand::thread_rng();
                                let rnd_idx = rng.gen_range(0..npcs.len());
                                
                                println!("{}",npcs[rnd_idx]);
                        },
                        "mission_prompt" => {
                            let mp = load_matrix(MISSION_PROMPTS)?;
                            // let mut rng = rand::thread_rng();
                            // let rnd_idx = rng.gen_range(0..mp[0].rand_value());

                            println!(" 🕵️ {}",Color::Green.bold().paint(mp[0].rand_value().1.as_str().unwrap()));

                        },
                        "mission" => {
                            let m = load_matrix(MISSION_GENERATOR)?;
                            //let mut rng = rand::thread_rng();
                            //let rnd_idx = rng.gen_range(0..m[0].Values.len());
                            //m.iter().take(m.len() -1)
                            for i in 0..m.len() {
                                let (k,v) = m[i].rand_value();
                                println!(" {} {} 🎲 [{}]",
                                    Color::White.bold().paint(m[i].Title.clone()),
                                    Color::Green.bold().paint(v.as_str().unwrap()),
                                    Color::Cyan.bold().paint(format!("{}",k)),
                                );                               
                            }

                        }
                        "banner" => {
                            for line in random_banner().split('\n') {
                                println!("{}",Color::Cyan.bold().paint(line));
                            }                           
                        }
                        _ => {}
                    }
                }
                None => {println!("Not found.")}
            }           
            // if tokens.len() != 2 {
            //     return Ok(format!("Wrong number of arguments({}): random <arg>",tokens.len()));               
            // }
        }
        "ls" | "dir" => {
            match tokens.next() {
                Some(t) => {
                    match t {
                        "roles" => {
                            let r = load_roles(ROLES)?;
                            for role in r.as_slice() {
                                println!("{}",role);
                            }
                            println!(" \x1b[1;37m{}\x1b[0m total roles.",r.len());
                        }
                        "traits" => {
                            let t = load_traits(TRAITS)?;
                            for tr in t.as_slice() {
                                println!("{}",tr);
                            }
                            println!(" \x1b[1;37m{}\x1b[0m total traits.",t.len());                    
                        }
                        "cassettes" => {
                            let mut data = fs::read_to_string(CASSETTES).unwrap();

                            let v: Vec<String> = serde_json::from_str(&data)?;
                            for c in v {
                                println!("{c}");
                            }
                        }
                        "cassette_links" => {
                            let data = fs::read_to_string(CASSETTE_LINKS).unwrap();                           
                            let t:HashMap<String, String> = serde_json::from_str(&data).unwrap();
                            for tr in &t {
                                println!("\x1b[1;32m{} 🔗 \x1b[1;34m{}\x1b[0m",tr.0,tr.1);
                            }
                            stdout().flush()?;
                            println!(" \x1b[1;37m{}\x1b[0m total cassette links.",t.len());                    
                        }                        

        
                        d@_ => {
                            // TODO need path to json
                            //let p = t.clone();
                            let path = ALL_FILES.iter().find(|p| get_filename(p) == d).unwrap();
                            let tr = load_matrix(path)?;
                            for trm in tr.as_slice() {
                                println!("{}",trm);
                            }
                            println!(" \x1b[1;37m{}\x1b[0m total tables.",tr.len())                      

                        }
                    }
                },
                None => {
                    println!("{}",White.bold().paint("FIST Shell Root Directory"));
                    //println!("/{}",Blue.bold().paint("root"));
                    // for file in ALL_FILES {  //CLI_DIR.split('\n').enumerate() {
                    //     gen_byte_entries(file);
                    // }
                    cmd_ls("");
                    //println!("total {}",ALL_FILES.len());
                    return Ok("ls".to_string());
     
                }
            }
        } 

        // Custom traits/roles/characters. These are v0.1.2 goals.           
        "create" => {
            println!("This feature available in {}",Cyan.paint("v0.1.2"));
        }
        "edit" => {
            match tokens.next().unwrap() {
                "role" => {},
                "trait"|"traits" => {},

                _ => {}
            }
            println!("This feature available in {}",Cyan.paint("v0.1.2"));

        }
        "delete" => {
            match tokens.next().unwrap() {
                "role" => {},
                "trait"|"traits" => {},

              
                _ => {}
            }        
            println!("This feature available in {}",Cyan.paint("v0.1.2"));

        }
        "cd" => {
            println!("This feature available in {}",Cyan.paint("v0.1.2"));
        }
        "validate" => {
            validate_data(1); 
        }
        "version" => {
            println!("{}", AnsiStrings( &[
                Color::Yellow.paint("rustic-fist "),
                Color::Cyan.paint(VERSION),
            ]));
        }
        "credits" => {
            credits();
        }
        "rgb" => {rgb_colors();}
        _ => {return Ok("Unknown Command.".to_string());}
    }


    Ok("ok.".to_string())
}

fn matches_arg(arg: &str) -> bool { false }

fn cmd_help() {
    let fmtstr: &[AnsiString<'static>] = &[
        Blue.bold().paint("help   "),

    ];
    println!("{}", AnsiStrings( &[
        Color::Yellow.paint("rustic-fist "),
        Color::Cyan.paint(VERSION),
        Color::Yellow.paint("\nA FIST CLI Shell"),
    ]));
    for line in CLI_HELP.split('\n') {
        println!("{line}");
    }
}

fn cmd_ls(path: &str) {
    if path.is_empty() {
        println!("{}",Color::Yellow.bold().paint(" Mode Owner Bytes Tables  Matrix Name"));
        println!("{}",Color::White.bold().paint("┌────────────────────────────────────────────"));
        
        for i in ALL_FILES {
            gen_byte_entries(i)
        }
        println!("\x1b[1;37m{}\x1b[0m total matrices.", ALL_FILES.len())
    }
}

fn banner() {
    print!("{}[2J",27 as char);
    println!("\n\n\n{} {}",Yellow.paint(APP_NAME),Cyan.bold().paint(VERSION));
    println!("{}",Yellow.paint("Copyright (c) Convoluted Systems, LLC."));
    println!("This is {} software.",White.bold().paint("OPEN SOURCE"));
    println!();

    home_banner();
    println!();
    println!("Type {} for commands or {} to exit the shell.",Blue.bold().paint("help"),Blue.bold().paint("quit"));
}

fn old_credits() {
    println!("{} by Gian James.",Color::Yellow.paint("rustic-fist"));
    let link = Color::Blue.bold().underline().paint("https://github.com/droidengineer/rustic-fist").hyperlink("https://github.com/droidengineer/");
    println!(" 🔗 {link}");

    println!("{} written by B. Everett Dutton.",Color::White.bold().paint("FIST:Ultimate"));
    let link = Color::Blue.bold().underline().paint("https://claymorerpgs.itch.io/fist").hyperlink("https://claymorerpgs.itch.io/fist");
    println!(" 🔗 {link}");

    println!("{} by ululu.",Color::Purple.bold().paint("FIST JSON Data"));
    let link = Color::Blue.bold().underline().paint("https://ululu.itch.io/fist-json-data").hyperlink("https://ululu.itch.io/fist-json-data");
    println!(" 🔗 {link}");

    println!("🏳️‍🌈 Developed during Pride Month 🏳️‍🌈");
    println!();  
}
fn credits() {
    // println!("{}", AnsiStrings(&[
    //     LightYellow.bold().paint("╔═════╡ "),
    //     Yellow.bold().paint("CREDITS"),
    // ]));
    //old_credits();

    println!("{}", AnsiStrings(&[
        LightYellow.bold().paint("╔═════» "),
        Yellow.paint("rustic-fist "),
        Color::Default.paint("by Gian James"),
    ]));
    //let link = Color::Blue.bold().underline().paint("https://github.com/droidengineer/rustic-fist").hyperlink("https://github.com/droidengineer/");
    println!("{}", AnsiStrings(&[
        LightYellow.bold().paint("║  🔗 "),
        Color::Blue.bold().underline().paint("https://github.com/droidengineer/rustic-fist").hyperlink("https://github.com/droidengineer/"),                
    ]));
    println!("{}", AnsiStrings(&[
        LightYellow.bold().paint("╠═════» "),
        White.bold().paint("FIST:Ultimate "),
        Color::Default.paint("by B. Everett Dutton"),
    ]));
    println!("{}", AnsiStrings(&[
        LightYellow.bold().paint("║  🔗 "),
        Color::Blue.bold().underline().paint("https://claymorerpgs.itch.io/fist").hyperlink("https://claymorerpgs.itch.io/fist"),
    ]));
    println!("{}", AnsiStrings(&[                
        LightYellow.bold().paint("╠═════» "),
        Purple.bold().paint("FIST JSON Data "),
        Color::Default.paint("by ululu")
    ]));
    println!("{}", AnsiStrings(&[
        LightYellow.bold().paint("║  🔗 "),
        Color::Blue.bold().underline().paint("https://ululu.itch.io/fist-json-data").hyperlink("https://ululu.itch.io/fist-json-data"),
    ]));
    println!("{}", AnsiStrings(&[
        Yellow.bold().paint("╚═════╡ "),
        Default.paint("🏳️‍🌈"),
        White.bold().paint(" Developed during Pride Month "),
        Default.paint("🏳️‍🌈"),
    ]));
    println!();
}

fn glow(c: u8, light_bg: bool) {
    let base = if light_bg { Color::Black } else { Color::White };
    let style = base.on(Color::Fixed(c));
    print!("{}",style.paint(&format!(" {:3} ",c)));
}

fn rgb_colors() {
    let height = 24;
    let width = 80;

    for row in 0..height {
        for col in 0..width {
            let r = (row * 255 / height) as u8;
            let g = (col * 255/width) as u8;        
            let b = 128;

            print!("{}",Style::default().on(Color::Rgb(r, g, b)).paint(" "));
        }
        println!();
    }
}

fn validate_data(lvl:u8) {
   for path in ALL_FILES {
    let file = get_filename(path);
    print!("{file}..."); stdout().flush().unwrap();
    if json_exists(path) {
        print!("{}",Color::LightGreen.paint("found! ")); 
        stdout().flush().unwrap();
        if lvl > 0 {
            match *path {
                ROLES => {
                    let len:usize = load_roles(ROLES).unwrap().len();
                    println!("Loaded {len} roles.");
                }
                TRAITS => {
                    let len:usize = load_traits(TRAITS).unwrap().len();
                    println!("Loaded {len} traits.");
                }
                CASSETTES => {
                    let data = fs::read_to_string(path).unwrap();
                    let upgrades: Vec<Value> = serde_json::from_str(&data).unwrap();
                    println!("Found {} entries.",upgrades.len());
                }
                CASSETTE_LINKS => {
                    let data = fs::read_to_string(path).unwrap();
                    let upgrades: HashMap<String,String> = serde_json::from_str(&data).unwrap();
                    println!("Found {} entries.",upgrades.len());
                }
                CHARACTERS_PREMADE_ENEMIES => {
                    let len = load_premade(CHARACTERS_PREMADE_ENEMIES).unwrap().len();
                    println!("Found {len} premade enemies.");
                }
                CHARACTERS_PREMADE_NPCS => {
                    let len = load_premade(path).unwrap().len();
                    println!("Found {len} premade npcs.");
                }
                GEAR_BASE_UPGRADES => {
                    let data = fs::read_to_string(path).unwrap();
                    let upgrades: Vec<Value> = serde_json::from_str(&data).unwrap();
                    println!("Found {} entries.",upgrades.len());         
                }
                _ => {
                    let len: usize = load_matrix(path).unwrap().len();
                    println!("Loaded {len} tables.");
                }
            }    
        } else { println!(); }
    } else {
        println!("{}",Color::LightRed.paint("missing!"));
    }
}

}

fn cmd_bullet() {

}
fn cmd_promo(str: &str) {
    for (num,line) in str.split('\n').enumerate() {
        if num <=6 {
            println!("{}",Color::Cyan.bold().paint(line));
        } else {
            println!("{}",Color::Magenta.bold().on(Cyan).paint(line));
        }
    }
    // println!("{} 🧛 A {} Setting for neon \'80s vampires, werewolves,\n      witches, and other night creatures.",
    //     Color::Magenta.bold().on(Cyan).paint("MIAMI NIGHTS"),
    //     Color::White.bold().paint("FIST Factions"));  

}

fn rnd_trait(traits: &[Trait]) -> &Trait {
    let mut rng = rand::thread_rng();
    let rnd_idx = rng.gen_range(0..traits.len());
    traits.get(rnd_idx).expect("Problems getting random trait.")  
}
fn rnd_role(roles: &[Role]) -> &Role {
    let mut rng = rand::thread_rng();
    let rnd_idx = rng.gen_range(0..roles.len());
    roles.get(rnd_idx).expect("Problems getting random role.")  
}


const EMOJI_CODES: &[&str] = &[
    "✊️","👊","🤛","🤜","❌","✖", "✔",
    "🕵️", "🗨️", "🗯️","📁","🔎","✅️","🆗",
    "⚔️", "⚠️", "⛔️", "❓️", "❗️", "⭐️", "🏷️", "‼️",
    "🎖️","🏅","🐾","💀","💾","📌","🔗","📒",
    "📑","🗂️","🦀","🎲","📼","🔦","🔥","🛠️",
    "🥇","🥈","🥉","🥷","🦸","🦹","🪪","➾",
    "🚗","","","","","","","","",
    "\u{2139}","☁","☑","⛅","","","","",
    
];
const BULLET_CODES: &[&str] = &[
    "▪","🔸","🔹","⦿","=>","➾","☑","",
];

const CLI_HELP: &str = "
The following commands are accepted within the shell. Other commands may be run
on cmdline. Where input is not case-sensitive:
    <arg> is one of <trait | role | character | npc | enemy | mission | gear>

    \x1b[32mhelp\x1b[0m   [cmd]           Prints this help or specific help for [cmd]
    \x1b[1;34mrandom\x1b[0m <arg>           Generates random selection for <arg>
    \x1b[1;34mcreate\x1b[0m <arg>           Create a new <arg>
    \x1b[1;34medit\x1b[0m   <arg> <tgt>     Edit <tgt> in <arg>
    \x1b[1;34mdelete\x1b[0m <arg> <tgt>     Delete <tgt> from <arg>
    \x1b[1;34msearch\x1b[0m <arg> <tgt>     Search <arg> for <tgt>

    cd     <dir>           Change directory to <dir>
    \x1b[34mls\x1b[0m|\x1b[34mdir\x1b[0m [pattern]       List contents of directory or [pattern] match   
    
    \x1b[1;34mcredits\x1b[0m                Show credits, contributors, thanks.
    \x1b[1;34mpromo\x1b[0m                  Show a random promotion for upcoming FIST expansion
    \x1b[1;34mbanner\x1b[0m                 Show a random startup banner.
    validate               Checks integrity of software data.

    \x1b[1;34mroll\x1b[0m
    \x1b[1;34mdicepool\x1b[0m

    \x1b[31mquit\x1b[0m|\x1b[31mexit\x1b[0m               Exits the FIST CLI shell.

";

