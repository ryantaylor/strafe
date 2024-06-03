mod command_type;

use crate::command_type::CommandType;
use anyhow::Result;
use clap::Parser;
use colored::*;
use inquire::{MultiSelect, Select};
use std::fmt::Write;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use strum::IntoEnumIterator;
use vault::{Player, RawCommand, Replay};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Strafe {
    /// Path to a CoH3 replay file
    file: PathBuf,
}

fn main() -> Result<()> {
    let strafe = Strafe::parse();

    let file = File::open(strafe.file)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    match Replay::from_bytes(&buffer) {
        Ok(replay) => {
            let player = select_player(replay.players());
            let command_types = select_command_types();
            let commands = filter_commands(&player, &command_types);

            for command in commands.iter() {
                println!("{}", format_display(command));
            }

            Ok(())
        }
        Err(_) => Ok(println!("{{\"error\":\"Parsing failed!\"}}")),
    }
}

fn select_player(players: Vec<Player>) -> Player {
    let selected_player = Select::new("Whose commands would you like to view?", players).prompt();

    match selected_player {
        Ok(player) => player,
        Err(_) => panic!("Error selecting player!"),
    }
}

fn select_command_types() -> Vec<CommandType> {
    let command_types = CommandType::iter().collect();

    let selected_types = MultiSelect::new(
        "Select command types to filter, or none to show all",
        command_types,
    )
    .prompt();

    match selected_types {
        Ok(types) => types,
        Err(_) => panic!("Error selecting command types!"),
    }
}

fn filter_commands(player: &Player, command_types: &[CommandType]) -> Vec<RawCommand> {
    if command_types.is_empty() {
        return player.raw_commands();
    }

    let commands: Vec<u8> = command_types
        .iter()
        .map(|command_type| *command_type as u8)
        .collect();
    player
        .raw_commands()
        .into_iter()
        .filter(|raw| commands.contains(&raw.action_type.into()))
        .collect()
}

fn format_display(command: &RawCommand) -> String {
    let command_type = format!("{}", CommandType::from_u8(command.action_type.into()));
    let command_padding = " ".repeat(35 - command_type.len());
    let contents = command
        .bytes
        .iter()
        .map(hexify)
        .map(whiten)
        .enumerate()
        .map(colour_core)
        .collect::<Vec<ColoredString>>();
    let mut buffer = String::new();
    for part in contents {
        write!(buffer, "{part} ").unwrap();
    }
    let tick_padding = " ".repeat(5 - command.tick.to_string().len());

    format!(
        "{}{} {}{}: {}",
        command_type.green(),
        command_padding,
        tick_padding,
        command.tick,
        buffer
    )
}

fn hexify(byte: &u8) -> String {
    format!("{:02X?}", byte)
}

fn whiten(byte: String) -> ColoredString {
    byte.white()
}

fn colour_core((idx, byte): (usize, ColoredString)) -> ColoredString {
    if idx == 2 {
        byte.magenta()
    } else if idx == 3 {
        byte.purple()
    } else if (35..=38).contains(&idx) {
        byte.red()
    } else {
        byte
    }
}
