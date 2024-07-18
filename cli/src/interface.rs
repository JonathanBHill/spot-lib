use std::cmp::PartialEq;
use std::collections::HashMap;

use clap::{Arg, arg, ArgAction, ArgGroup, ArgMatches, ColorChoice, Command, value_parser, ValueEnum};
use clap::builder::{BoolValueParser, styling, TypedValueParser};
use clap_complete::{generate, Shell};
#[allow(unused_imports)]
use dialoguer::MultiSelect;
use futures::stream::{self, StreamExt};
use spot_lib_general::playlists::playlists::ComparePlaylists;
use spot_lib_general::playlists::query::PlaylistQuery;

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
#[allow(unused_macros)]
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
        let subcommands_stream = stream::iter(self.command.get_subcommands());
        self.check_subcommand_conflicts(&matches);
        
        subcommands_stream.for_each(|cmd| async {
            let name = cmd.get_name();
            match matches.subcommand_matches(name) {
                None => {
                    println!("No matches for subcommand, {:?}", &name);
                }
                Some(subcommand) => {
                    match name {
                        "playlist" => {
                            self.scan_playlist_command(&subcommand);
                        }
                        "config" => {
                            self.scan_config_command(&subcommand);
                        }
                        "releaseradar" => {
                            self.run_rr_command(&subcommand).await;
                        }
                        _ => {}
                    }
                    println!("Subcommand: {:?}", &name);
                    println!("Subcommand matches: {:?}", &subcommand);
                }
            }
        }).await;
    }
    
    fn scan_playlist_command(&self, matches: &ArgMatches) {
        let plist = matches.get_one::<bool>("plist").unwrap_or(&false).to_string();
        let pcreate = matches.get_one::<bool>("pcreate").unwrap_or(&false).to_string();
        let pmove = matches.get_one::<String>("pmove").unwrap_or(&"none".to_string()).to_string();
        let pdelete = matches.get_one::<bool>("pdelete").unwrap_or(&false).to_string();
        println!("Playlist list: {:?}; Playlist create: {:?}; Playlist move: {:?}; Playlist delete: {:?}", &plist, &pcreate, &pmove, &pdelete);
    }
    
    fn scan_config_command(&self, matches: &ArgMatches) {
        if let Some(shell) = matches.get_one::<ShellType>("cshell") {
            let mut cmd = self.command.clone();
            generate_auto_complete!(shell, cmd);
            return;
        }
        let cset = matches.get_one::<String>("cset").unwrap_or(&"false".to_string()).to_string();
        let cunset = matches.get_one::<String>("cunset").unwrap_or(&"false".to_string()).to_string();
        let cget = matches.get_one::<String>("cget").unwrap_or(&"false".to_string()).to_string();
        let cshell = matches.get_one::<ShellType>("cshell").unwrap_or(&ShellType::Bash).to_string();
        println!("Config set: {:?}; Config unset: {:?}; Config get: {:?}; Config shell: {:?}", &cset, &cunset, &cget, &cshell);
    }
    
    async fn run_rr_command(&self, rr_subcommand: &ArgMatches) {
        let rr = ReleaseRadar::new().await;
        if let Some(list) = rr_subcommand.subcommand_matches("queryrr") {
            let rl_spot = list.get_one::<bool>("rlspot").unwrap_or(&false).to_string();
            let rl_custom = list.get_one::<bool>("rlcustom").unwrap_or(&false).to_string();
            println!("Spotify flag: {:?}; Custom flag: {:?}", &rl_spot, &rl_custom);
            if rl_spot == "true" {
                rr.query_rr(false).await;
            } else if rl_custom == "true" {
                rr.query_rr(true).await;
            }
        }
        if let Some(update) = rr_subcommand.subcommand_matches("updaterr") {
            let print_rr = update.get_one::<bool>("printrr").unwrap_or(&false).to_string();
            if print_rr == "true" {
                rr.update_rr(true).await;
            } else {
                rr.update_rr(false).await;
                println!("Your personal Release Radar has been updated.");
            }
        }
        if let Some(compare) = rr_subcommand.subcommand_matches("comparerr") {
            let rr_obj = rr.get_rr(true).await;
            let comp_obj = ComparePlaylists::new(rr_obj).await;
            if let Some(playlist) = compare.get_one::<String>("playlisttocompare") {
                let test = PlaylistQuery::new().await;
                if let Ok(queried_pl) = test.query_playlist(playlist.to_string()).await {
                    let comp_obj_2 = ComparePlaylists::new(queried_pl).await;
                    let comp_tracks = comp_obj.comp_tracks(&comp_obj_2);
                    comp_obj.print_comp(comp_tracks);
                }
                println!("Comparing the Release Radar playlist to the {:?} playlist", &playlist);
            }
        }
    }
    
    fn config_command() -> Command {
        Command::new("config")
            .short_flag('C')
            .long_flag("config")
            .about("Configuration subcommand")
            .arg(
                Arg::new("cset")
                    .help("Set a configuration value within config")
                    .short('s')
                    .long("set")
                    .help("Set a configuration value within config"),
            )
            .arg(
                Arg::new("cunset")
                    .help("Unset a configuration value within config")
                    .short('u')
                    .long("unset")
                    .help("Unset a configuration value within config"),
            )
            .arg(
                Arg::new("cget")
                    .help("Get a configuration value within config")
                    .short('g')
                    .long("get")
                    .help("Get a configuration value within config"),
            )
            .arg(
                Arg::new("cshell")
                    .short('S')
                    .long("shell")
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
                    .value_parser(BoolValueParser::new()
                        .map(|b| if b { true } else { false })
                    )
                    .action(ArgAction::SetTrue)
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
            )
            .subcommand(
                Command::new("updaterr")
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
            .subcommand(
                Command::new("comparerr")
                    .short_flag('C')
                    .long_flag("compare")
                    .color(ColorChoice::Always)
                    .about("Compare the Release Radar playlist to another playlist")
                    .arg(
                        Arg::new("playlisttocompare")
                            .short('p')
                            .long("playlist")
                            .value_name("PLAYLIST-NAME")
                            .help("The exact name of the playlist to use for comparison"),
                    )
            )
            // .arg(
            //     Arg::new("rrcompare")
            //         .short('c')
            //         .long("compare")
            //         .value_name("PLAYLIST-NAME")
            //         .help("Compare the Release Radar playlist to another playlist"),
            // )
    }
    
    #[allow(dead_code)]
    fn use_test_value(&self, matches: &ArgMatches) {
        let test_val = matches.get_one::<String>("ttest").unwrap_or(&"test".to_string()).to_string();
        println!("Test flag: {:?}", &test_val);
    }
    
    #[allow(dead_code)]
    fn check_presence(&self, matches: &ArgMatches) -> HashMap<&str, bool> {
        let presence = HashMap::new();
        let args = Self::ARGS.to_vec();
        args.iter().for_each(|&id| {
            let _id_clone = id.to_string();
            println!("{:?}: {:?} ", id, matches.contains_id(id));
        });
        presence
    }
    
    fn check_subcommand_conflicts(&self, matches: &ArgMatches) {
        let _subcommands = matches.subcommand_name();
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
#[allow(dead_code)]
enum HashMapArgTypes {
    String(String),
    Bool(bool),
}

impl HashMapArgTypes {
    #[allow(dead_code)]
    pub fn variant_type(&self) -> &'static str {
        match self {
            HashMapArgTypes::String(_) => "String",
            HashMapArgTypes::Bool(_) => "Bool",
        }
    }
    
    #[allow(dead_code)]
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
