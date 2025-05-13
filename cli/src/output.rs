//! Output formatting for the CLI.

use colored::Colorize;
use samp_query::{DetailedPlayerList, PlayerList, ServerInfo, ServerRules};
use tabled::{Table, Tabled};

pub fn format_server_info(info: &ServerInfo) -> String {
    let mut output = String::new();

    output.push_str(&format!("{}\n", "Server Information".green().bold()));
    output.push_str(&format!("{}: {}\n", "Hostname".blue().bold(), info.hostname));
    output.push_str(&format!(
        "{}: {}/{}\n",
        "Players".blue().bold(),
        info.players,
        info.max_players
    ));
    output.push_str(&format!("{}: {}\n", "Gamemode".blue().bold(), info.gamemode));
    output.push_str(&format!("{}: {}\n", "Language".blue().bold(), info.language));
    output.push_str(&format!(
        "{}: {}\n",
        "Password".blue().bold(),
        if info.password { "Yes".red() } else { "No".green() }
    ));

    output
}

pub fn format_rules(rules: &ServerRules) -> String {
    let mut output = String::new();

    output.push_str(&format!("{}\n", "Server Rules".green().bold()));

    // Create a table for the rules
    #[derive(Tabled)]
    struct RuleRow {
        #[tabled(rename = "Rule")]
        name: String,
        #[tabled(rename = "Value")]
        value: String,
    }

    let mut rule_rows = Vec::new();
    for (name, value) in &rules.rules {
        rule_rows.push(RuleRow {
            name: name.clone(),
            value: value.clone(),
        });
    }

    let table = Table::new(rule_rows);
    let formatted_table = table.to_string();
    output.push_str(&formatted_table);

    output
}

pub fn format_player_list(players: &PlayerList) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "{} ({})\n",
        "Players".green().bold(),
        players.players.len()
    ));

    #[derive(Tabled)]
    struct PlayerRow {
        #[tabled(rename = "Name")]
        name: String,
        #[tabled(rename = "Score")]
        score: i32,
    }

    let mut player_rows = Vec::new();
    for player in &players.players {
        player_rows.push(PlayerRow {
            name: player.name.clone(),
            score: player.score,
        });
    }

    let table = Table::new(player_rows);
    let formatted_table = table.to_string();
    output.push_str(&formatted_table);

    output
}

pub fn format_detailed_player_list(players: &DetailedPlayerList) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "{} ({})\n",
        "Players".green().bold(),
        players.players.len()
    ));

    #[derive(Tabled)]
    struct PlayerRow {
        #[tabled(rename = "ID")]
        id: u8,
        #[tabled(rename = "Name")]
        name: String,
        #[tabled(rename = "Score")]
        score: i32,
        #[tabled(rename = "Ping")]
        ping: u32,
    }

    let mut player_rows = Vec::new();
    for player in &players.players {
        player_rows.push(PlayerRow {
            id: player.id,
            name: player.name.clone(),
            score: player.score,
            ping: player.ping,
        });
    }

    let table = Table::new(player_rows);
    let formatted_table = table.to_string();
    output.push_str(&formatted_table);

    output
}
