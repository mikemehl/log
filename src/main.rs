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
        _ => unimplemented!(),
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
        ListCommand::Entries { period: _ } => {
            todo!()
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
