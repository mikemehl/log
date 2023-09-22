use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "log")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Start {
        project: String,
        tag: Option<String>,
    },
    Stop {},
    List {
        #[arg(value_enum)]
        period: Period,
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
    Projects,
}

#[derive(ValueEnum, Clone)]
enum Period {
    Day,
    Week,
    Month,
    Year,
    All,
}

fn main() {
    let args = Cli::parse();
}
