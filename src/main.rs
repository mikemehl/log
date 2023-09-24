pub(crate) mod cli;
mod data;
use crate::cli::{Cli, Command, ProjectCommand};
use anyhow::Result;
use chrono::prelude::*;
use clap::Parser;
use cli::ListCommand;

fn main() {
    let args = Cli::parse();
    if let Err(err) = match args.cmd {
        Command::Project { cmd } => do_project_cmd(cmd),
        Command::List { what } => do_list_cmd(what),
        Command::Start { project, tag } => do_start_cmd(project, tag),
        Command::Stop {} => do_stop_cmd(),
        Command::Update { id, start, end } => do_update_cmd(id, start, end),
        Command::Report {
            period,
            project,
            tag,
        } => do_report_cmd(period, project, tag),
        Command::Delete { id } => do_delete_cmd(id),
    } {
        println!("Error: {}", err);
    }
}

fn do_project_cmd(cmd: ProjectCommand) -> Result<()> {
    match cmd {
        ProjectCommand::New { name } => {
            data::create_project(name.clone()).map(|_| println!("Project {} created", name))
        }
        ProjectCommand::Delete { name } => {
            data::delete_project(name.clone()).map(|_| println!("Project {} deleted", name))
        }
    }
}

fn do_list_cmd(what: ListCommand) -> Result<()> {
    match what {
        ListCommand::Projects => {
            for project in data::list_projects()? {
                println!("Project: {}", project);
            }
            Ok(())
        }
        ListCommand::Entries { period } => {
            let all_entries = data::list_entries()?;
            let entries = all_entries.iter().filter(|e| match period {
                cli::Period::Day => e.start.date_naive() == Local::now().date_naive(),
                cli::Period::Week => e.start.iso_week() == Local::now().iso_week(),
                cli::Period::Month => e.start.month() == Local::now().month(),
                cli::Period::Year => e.start.year() == Local::now().year(),
                cli::Period::All => true,
            });
            for entry in entries {
                println!(
                    "Project: {}, Tag: {}, Start: {}, End: {}",
                    entry.project,
                    if let Some(tag) = &entry.tag { tag } else { "" },
                    entry.start.format("%H:%M"),
                    if let Some(end) = entry.end {
                        end.format("%H:%M").to_string()
                    } else {
                        "ongoing".to_string()
                    }
                );
            }
            Ok(())
        }
    }
}

fn do_start_cmd(project: String, tag: Option<String>) -> Result<()> {
    let time = Local::now();
    data::start_entry(project.clone(), tag.clone(), time).map(|_| {
        if let Some(tag) = tag {
            println!(
                "Started {} with tag {} at {} ",
                project,
                tag,
                time.format("%H:%M")
            )
        } else {
            println!("Started {} at {} ", project, time.format("%H:%M"))
        }
    })
}

fn do_stop_cmd() -> Result<()> {
    data::stop_entry().map(|_| println!("Stopped"))
}

fn do_update_cmd(id: i32, start: String, end: String) -> Result<()> {
    data::update_entry(id, start, end).map(|_| println!("Updated"))
}

fn do_report_cmd(period: cli::Period, project: String, tag: Option<String>) -> Result<()> {
    let _entries = data::fetch_entries(period, project, tag);
    todo!()
}

fn do_delete_cmd(id: i32) -> Result<()> {
    data::delete_entry(id)
}
