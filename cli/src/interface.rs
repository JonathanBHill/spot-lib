use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::Display;

use clap::{Arg, arg, ArgAction, ArgGroup, ArgMatches, ColorChoice, Command, value_parser, ValueEnum};
use clap::builder::{BoolValueParser, styling, TypedValueParser};
use clap_complete::{generate, Shell};
#[allow(unused_imports)]
use dialoguer::MultiSelect;

use spot_lib_general::playlists::update::ReleaseRadar;

macro_rules! generate_auto_complete {
    ($shell:expr, $cmd:expr) => {
        match $shell {
            ShellType::Bash => generate(Shell::Bash, &mut $cmd, "spotcli", &mut std::io::stdout()),
            ShellType::Zsh => generate(Shell::Zsh, &mut $cmd, "spotcli", &mut std::io::stdout()),
            ShellType::Fish => generate(Shell::Fish, &mut $cmd, "spotcli", &mut std::io::stdout()),
            ShellType::PowerShell => generate(
                Shell::PowerShell,
                &mut $cmd,
                "spotcli",
                &mut std::io::stdout(),
            ),
            ShellType::Elvish => {
                generate(Shell::Elvish, &mut $cmd, "spotcli", &mut std::io::stdout())
            }
        }
    };
}
macro_rules! handle_rr {
    ($matches:expr, $key:expr, $type:ty, $variant:ident) => {{
        let presence = match $matches.get_one::<$type>($key) {
            Some(val) => HashMapArgTypes::$variant(val.to_owned()),
            None => HashMapArgTypes::Bool(false),
        };
        presence
    }};
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Ord, ValueEnum)]
enum ShellType {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

impl std::fmt::Display for ShellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellType::Bash => write!(f, "bash"),
            ShellType::Zsh => write!(f, "zsh"),
            ShellType::Fish => write!(f, "fish"),
            ShellType::PowerShell => write!(f, "powershell"),
            ShellType::Elvish => write!(f, "elvish"),
        }
    }
    
}
pub struct TerminalApp {
    command: Command,
}

impl TerminalApp {
    const ARGS : &'static [&'static str] = &[
        "ttest",];
    
    pub fn new() -> Self {
        let app_cmd = Command::new("tspot")
            .version("0.2.0")
            .author("Jonathan Hill <jonathans-git@pm.me>")
            .about("A terminal app for Spotify")
            .args (
                &[
                    arg!(ttest: -t --test <KEYWORD> "Injects keyword from terminal into the app")
                ]
            )
            .subcommands(
                &[
                    Self::playlist_command(),
                    Self::release_radar_command(),
                    Self::config_command(),
                ]
            );

        TerminalApp { command: app_cmd }
    }
    pub async fn run(&self) {
        let matches = self.command.clone().get_matches();
        self.check_subcommand_conflicts(&matches);
        
        if let Some(matches) = matches.subcommand_matches("config") {
            if let Some(shell) = matches.get_one::<ShellType>("cshell") {
                let mut cmd = self.command.clone();
                generate_auto_complete!(shell, cmd);
                return;
            }
        }
        if let Some(rrc_cmds) = matches.clone().subcommand_matches("releaseradar") {
            if let Some(list) = rrc_cmds.subcommand_matches("queryrr") {
                let rl_spot = list.get_one::<bool>("rlspot").unwrap_or(&false).to_string();
                let rl_custom = list.get_one::<bool>("rlcustom").unwrap_or(&false).to_string();
                println!("Spotify flag: {:?}; Custom flag: {:?}", &rl_spot, &rl_custom);
                if rl_spot == "true" {
                    let rr = ReleaseRadar::new().await;
                    rr.query_rr(false).await;
                } else if rl_custom == "true" {
                    let rr = ReleaseRadar::new().await;
                    rr.query_rr(true).await;
                }
            }
            if let Some(update) = rrc_cmds.subcommand_matches("rrupdate") {
                let print_rr = update.get_one::<bool>("printrr").unwrap_or(&false).to_string();
                let rr = ReleaseRadar::new().await;
                if print_rr == "true" {
                    rr.update_rr(true).await;
                    // println!("Printing the update progress");
                } else {
                    rr.update_rr(false).await;
                    println!("Your personal Release Radar has been updated.");
                }
            }
        };
        
        self.command.get_subcommands().for_each(|cmd| {
            let name = cmd.get_name();
            match matches.subcommand_matches(name) {
                None => {}
                Some(sub_cmd) => {}
            }
            if let Some(subcommand) = matches.subcommand_matches(name) {
                match name {
                    "playlist" => {
                        self.scan_playlist_command(&subcommand);
                    }
                    "config" => {
                        self.scan_config_command(&subcommand);
                    }
                    _ => {}
                }
                println!("Subcommand: {:?}", &name);
                println!("Subcommand matches: {:?}", &subcommand);
            }
        });
        // if let Some(rr_cmds) = rr_cmds {
        //     println!("{:?}", rr_cmds.subcommand_matches("list").unwrap());
        // }
        // println!("{:?}", rr_cmds.unwrap().subcommand_matches("list").unwrap());
        
        // let test_val = matches.subcommand_matches("releaseradar").clone();
        // let rr_args = self.run_releaseradar(&matches).await;
        // if let Some(testing_val) = Some(matches.get_one::<String>("ttest").unwrap_or(&"test".to_string()).to_string()) {
        //     println!("Test flag: {:?}", &testing_val);
        //     let testing = HashMapArgTypes::from_gen(test_val, testing_val);
        //     println!("{:?}", testing);
        // };
        // let hashm = self.check_presence(&matches);
        // println!("hashmap: {:?}", hashm);
    }
    async fn run_releaseradar(&self, matches: &ArgMatches) -> HashMap<&str, HashMapArgTypes> {
        let rr_args = match matches.clone().subcommand_matches("releaseradar") {
            Some(rr_arg_matches) => {
                let mut hsh: HashMap<&str, HashMapArgTypes> = HashMap::new();
                for arg_id in vec!["rrupdate", "rrlist", "rrcompare"].iter().copied() {
                    match arg_id {
                        "rrupdate" => {
                            let presence = handle_rr!(rr_arg_matches, arg_id, bool, Bool);
                            if presence == HashMapArgTypes::Bool(true) {
                                let rr = ReleaseRadar::new().await;
                                println!("Updating Release Radar playlist - {:?}", &presence);
                                // rr.update_rr(true).await;
                            }
                            hsh.insert("update", presence);
                        },
                        // "rrlist" => {
                        //     let presence = handle_rr!(rr_arg_matches, arg_id, String, String);
                        //     if presence.variant_type() == "String" {
                        //         println!("Listing release radar playlist tracks (default: custom) - {:?}", &presence);
                        //     };
                        //     hsh.insert("list", presence);
                        // },
                        "rrcompare" => {
                            let presence = handle_rr!(rr_arg_matches, arg_id, String, String);
                            // let presence = match rr_matches.get_one::<String>(val) {
                            //     Some(&ref val) => HashMapArgTypes::String(val.to_string()),
                            //     _ => HashMapArgTypes::Bool(false),
                            // };
                            if presence.variant_type() == "String" {
                                println!("Comparing the Release Radar playlist to another playlist - {:?}", &presence);
                            };
                            // if get_type(&presence, true) == "HashMapArgTypes"{
                            //     println!("Comparing the Release Radar playlist to another playlist - {:?}", &presence);
                            // }
                            hsh.insert("compare", presence);
                        },
                        _ => {}
                    }
                }
                // vec!["update", "list", "compare"].iter().copied().for_each(|id| {
                //     match id {
                //         "update" => {
                //
                //             let presence = match rr_matches.get_one::<bool>(id) {
                //                 Some(&val) => HashMapArgTypes::Bool(val),
                //                 _ => HashMapArgTypes::Bool(false),
                //             };
                //             println!("Updating Release Radar playlist - {:?}", &presence);
                //             let rr = ReleaseRadar::new();
                //             // rr.update_rr(true);
                //             hsh.insert("update", presence);
                //         },
                //         "list" => {
                //             let presence = match rr_matches.get_one::<String>(id) {
                //                 Some(&ref val) => HashMapArgTypes::String(val.to_string()),
                //                 _ => HashMapArgTypes::Bool(false),
                //             };
                //             println!("Listing release radar playlist tracks (default: custom) - {:?}", &presence);
                //             hsh.insert("list", presence);
                //         },
                //         "compare" => {
                //             let presence = match rr_matches.get_one::<String>(id) {
                //                 Some(&ref val) => HashMapArgTypes::String(val.to_string()),
                //                 _ => HashMapArgTypes::Bool(false),
                //             };
                //             println!("Comparing the Release Radar playlist to another playlist - {:?}", &presence);
                //             hsh.insert("compare", presence);
                //         },
                //         _ => {}
                //     }
                // });
                hsh
            }
            None => {
                println!("The releaseradar subcommand was not used");
                HashMap::new()
            }
        };
        println!("Releaseradar Flags: {:?}\n", &rr_args);
        rr_args
    }
    fn scan_playlist_command(&self, matches: &ArgMatches) {
        let plist = matches.get_one::<bool>("plist").unwrap_or(&false).to_string();
        let pcreate = matches.get_one::<bool>("pcreate").unwrap_or(&false).to_string();
        let pmove = matches.get_one::<String>("pmove").unwrap_or(&"none".to_string()).to_string();
        let pdelete = matches.get_one::<bool>("pdelete").unwrap_or(&false).to_string();
        println!("Playlist list: {:?}; Playlist create: {:?}; Playlist move: {:?}; Playlist delete: {:?}", &plist, &pcreate, &pmove, &pdelete);
    }
    fn scan_config_command(&self, matches: &ArgMatches) {
        let cset = matches.get_one::<String>("cset").unwrap_or(&"false".to_string()).to_string();
        let cunset = matches.get_one::<String>("cunset").unwrap_or(&"false".to_string()).to_string();
        let cget = matches.get_one::<String>("cget").unwrap_or(&"false".to_string()).to_string();
        let cshell = matches.get_one::<ShellType>("cshell").unwrap_or(&ShellType::Bash).to_string();
        println!("Config set: {:?}; Config unset: {:?}; Config get: {:?}; Config shell: {:?}", &cset, &cunset, &cget, &cshell);
    }
    fn config_command() -> Command {
        Command::new("config")
            .about("Configuration subcommand")
            .arg(
                Arg::new("cset")
                    .help("Set a configuration value within config")
                    .long("set")
                    .help("Set a configuration value within config"),
            )
            .arg(
                Arg::new("cunset")
                    .help("Unset a configuration value within config")
                    .long("unset")
                    .help("Unset a configuration value within config"),
            )
            .arg(
                Arg::new("cget")
                    .help("Get a configuration value within config")
                    .long("get")
                    .help("Get a configuration value within config"),
            )
            .arg(
                Arg::new("cshell")
                    .value_parser(value_parser!(ShellType))
                    .help("The shell to generate the script for"),
            )
            .group(
                ArgGroup::new("config_sub")
                    .args(&["cset", "cshell", "cunset", "cget"])
                    .required(true),
            )
    }
    fn playlist_command() -> Command {
        Command::new("playlist")
            .short_flag('p')
            .about("Manage playlists")
            .arg(
                Arg::new("plist")
                    .short('l')
                    .long("list")
                    .help("List all playlists"),
            )
            .arg(
                Arg::new("pcreate")
                    .short('c')
                    .long("create")
                    .help("Create a new playlist"),
            )
            .arg(
                Arg::new("pmove")
                    .short('m')
                    .long("move")
                    .value_name("folder-name")
                    .help("Move a playlist"),
            )
            .arg(
                Arg::new("pdelete")
                    .short('d')
                    .long("delete")
                    .help("Delete a playlist"),
            )
    }
    fn release_radar_command() -> Command {
        Command::new("releaseradar")
            .short_flag_alias('R')
            .subcommand(
                Command::new("queryrr")
                    .short_flag('q')
                    .long_flag("query")
                    .arg(
                        Arg::new("rlspot")
                            .short('s')
                            .long("spotify")
                            .value_parser(BoolValueParser::new()
                                .map(|b| if b { true } else { false })
                            )
                            .action(ArgAction::SetTrue)
                            .help("List all songs in the Release Radar playlist"),
                    )
                    .arg(
                        Arg::new("rlcustom")
                            .short('c')
                            .long("custom")
                            .value_parser(BoolValueParser::new()
                                .map(|b| if b { true } else { false })
                            )
                            .action(ArgAction::SetTrue)
                            .help("List all songs in the full Release Radar playlist"),
                    )
                    .group(
                        ArgGroup::new("queryrrgrp")
                            .args(&["rlspot", "rlcustom"])
                            .required(true)
                    )
                    // .arg_required_else_help(true)
                    .styles(styling::Styles::styled()
                        .header(styling::AnsiColor::BrightGreen.on_default()
                            |styling::Effects::BOLD|styling::Effects::ITALIC)
                        .usage(styling::AnsiColor::BrightGreen.on_default()
                            |styling::Effects::BOLD|styling::Effects::ITALIC)
                        .valid(styling::RgbColor(255,193,0).on_default()
                            |styling::Effects::ITALIC)
                        .literal(styling::AnsiColor::BrightBlue.on_default()
                            |styling::Effects::BOLD)
                    )
                    .after_help("This command will list all songs in the specified Release Radar playlist"),
                
                // .value_parser(["spotify", "custom"])
                // .default_missing_values(["custom"].into_iter())
                // .value_name("STOCK | CUSTOM")
                // .help("List all songs in the Release Radar playlist"),
            )
            .subcommand(
                Command::new("rrupdate")
                    .short_flag('U')
                    .long_flag("Update")
                    .color(ColorChoice::Always)
                    .about("Update the Release Radar playlist")
                    .arg(
                        Arg::new("printrr")
                            .short('p')
                            .long("print")
                            .value_parser(BoolValueParser::new()
                                .map(|b| if b { false } else { true })
                            )
                            .action(ArgAction::SetFalse)
                            .help("Print the update progress"),
                    )
            )
            .arg(
                Arg::new("rrcompare")
                    .short('c')
                    .long("compare")
                    .value_name("PLAYLIST-NAME")
                    .help("Compare the Release Radar playlist to another playlist"),
            )
    }
    fn use_test_value(&self, matches: &ArgMatches) {
        let test_val = matches.get_one::<String>("ttest").unwrap_or(&"test".to_string()).to_string();
        println!("Test flag: {:?}", &test_val);
    }
    fn check_presence(&self, matches: &ArgMatches) -> HashMap<&str, bool> {
        let mut presence = HashMap::new();
        let args = Self::ARGS.to_vec();
        args.iter().for_each(|&id| {
            let id_clone = id.clone().to_string();
            println!("{:?}: {:?} ", id, matches.contains_id(id));
            // presence.insert(id_clone.as_str(), matches.contains_id(id.clone().to_string().as_str()));
        });
        presence
    }
    fn check_subcommand_conflicts(&self, matches: &clap::ArgMatches) {
        let subcommands = matches.subcommand_name();
        let mut subcommand_count = 0;
        if matches.subcommand_matches("config").is_some() {
            subcommand_count += 1;
        }
        if matches.subcommand_matches("releaseradar").is_some() {
            subcommand_count += 1;
        }
        if matches.subcommand_matches("playlist").is_some() {
            subcommand_count += 1;
        }
        if subcommand_count > 1 {
            eprintln!("Error: Only one subcommand can be used at a time");
            std::process::exit(1);
        }
    }
}
#[derive(Debug, PartialEq)]
enum HashMapArgTypes {
    String(String),
    Bool(bool),
}

impl HashMapArgTypes {
    pub fn variant_type(&self) -> &'static str {
        match self {
            HashMapArgTypes::String(_) => "String",
            HashMapArgTypes::Bool(_) => "Bool",
        }
    }
    fn from_gen(arg_val: Option<&ArgMatches>, id: String) -> Self {
        match arg_val {
            None => {
                println!("no arg matches; returning hsmbool");
                HashMapArgTypes::Bool(false)
            }
            Some(val) => {
                if id == "updaterr" {
                    if let Some(val) = val.get_one::<bool>(id.as_str()) {
                        println!("update true; returning hsmbool");
                        HashMapArgTypes::Bool(val.clone())
                        // t
                    } else {
                        println!("update false; returning hsmbool");
                        HashMapArgTypes::Bool(false)
                    }
                    // HashMapArgTypes::Bool(Some(val.get_one::<bool>(id.as_str()).clone()))
                } else if id == "queryrr" {
                    if let Some(val) = val.get_one::<String>(id.as_str()) {
                        println!("list true; returning hsmstring");
                        HashMapArgTypes::String(val.to_string())
                    } else {
                        println!("list false; returning hsmbool");
                        HashMapArgTypes::Bool(false)
                    }
                } else if id == "rrcompare" {
                    if let Some(val) = val.get_one::<String>(id.as_str()) {
                        println!("compare true; returning hsmstring");
                        HashMapArgTypes::String(val.to_string())
                    } else {
                        println!("compare false; returning hsmbool");
                        HashMapArgTypes::Bool(false)
                    }
                } else {
                    println!("invalid flag; returning hsmbool");
                    HashMapArgTypes::Bool(false)
                }
                // if let Some(val) = val.get_one::<String>(id.as_str()) {
                //     HashMapArgTypes::String(val.to_string())
                // } else if let Some(val) = val.get_one::<bool>(id.as_str()) {
                //     HashMapArgTypes::Bool(val.clone())
                // } else {
                //     HashMapArgTypes::Bool(false)
                // }
            }
        }
    }
}
