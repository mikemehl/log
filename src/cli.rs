use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "log")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start tracking time for a project with an optional tag
    Start {
        /// Name of the project to start tracking
        project: String,
        /// Optional tag to add to the time entry
        tag: Option<String>,
    },
    /// Stop the current time tracking
    Stop {},
    /// List projects or time entries
    List {
        #[command(subcommand)]
        /// What to list
        what: ListCommand,
    },
    /// Delete a time entry
    Delete { id: i32 },
    /// Update a time entry
    Update { id: i32, start: String, end: String },
    /// Generate a report for a time period, project, and optional tag
    Report {
        #[arg(value_enum)]
        /// Time period to report on
        period: Period,
        /// Name of the project to report on
        project: String,
        /// Optional tag to filter on
        tag: Option<String>,
    },
    /// Create or delete a project
    Project {
        #[command(subcommand)]
        cmd: ProjectCommand,
    },
}

#[derive(Subcommand)]
pub enum ListCommand {
    /// List all projects
    Projects,
    /// List all time entries for a period
    Entries {
        #[arg(value_enum)]
        /// Time period to list entries for
        period: Period,
    },
}

#[derive(Subcommand)]
pub enum ProjectCommand {
    /// Create a new project
    New { name: String },
    /// Delete a project
    Delete { name: String },
}

#[derive(ValueEnum, Clone)]
/// Time period to report on
pub enum Period {
    /// Report on the current day
    Day,
    /// Report on the current week
    Week,
    /// Report on the current month
    Month,
    /// Report on the current year
    Year,
    /// Report on all time
    All,
}
