use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "log")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Start {
        project: String,
        tag: Option<String>,
    },
    Stop {},
    List {
        #[command(subcommand)]
        what: ListCommand,
    },
    Delete {
        id: i32,
    },
    Update {
        id: i32,
        start: String,
        end: String,
    },
    Report {
        #[arg(value_enum)]
        period: Period,
        project: String,
        tag: Option<String>,
    },
    Project {
        #[command(subcommand)]
        cmd: ProjectCommand,
    },
}

#[derive(Subcommand)]
pub enum ListCommand {
    Projects,
    Entries {
        #[arg(value_enum)]
        period: Period,
    },
}

#[derive(Subcommand)]
pub enum ProjectCommand {
    New { name: String },
    Delete { name: String },
}

#[derive(ValueEnum, Clone)]
pub enum Period {
    Day,
    Week,
    Month,
    Year,
    All,
}
