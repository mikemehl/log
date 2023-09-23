pub(crate) mod cli;
mod data;
use crate::cli::{Cli, Command, ProjectCommand};
use anyhow::Result;
use clap::Parser;
use cli::ListCommand;

fn main() {
    let args = Cli::parse();
    if let Err(err) = match args.cmd {
        Command::Project { cmd } => do_project_cmd(cmd),
        Command::List { what } => do_list_cmd(what),
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
        _ => unimplemented!(),
    }
}
