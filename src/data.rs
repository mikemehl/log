use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Read};

const LOG_FILE: &str = ".timelog.yaml";

#[derive(Serialize, Deserialize)]
pub struct ProjectDef {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: i32,
    pub project: String,
    pub tag: Option<String>,
    // pub start: DateTime<Local>,
    // pub end: Option<DateTime<Local>>,
}

#[derive(Serialize, Deserialize)]
pub struct LogFile {
    projects: Vec<ProjectDef>,
    entries: Vec<TimeEntry>,
}

pub fn create_project(name: String) -> Result<()> {
    let mut log_file = read_log_file()?;
    if !log_file.projects.iter_mut().any(|e| e.name == name) {
        log_file.projects.push(ProjectDef { name });
        write_log_file(&log_file)
    } else {
        Err(anyhow::anyhow!("Project already exists"))
    }
}

pub fn delete_project(name: String) -> Result<()> {
    let mut log_file = read_log_file()?;
    match log_file.projects.iter_mut().find(|e| e.name == name) {
        Some(_) => {
            log_file.projects.retain(|e| e.name != name);
            write_log_file(&log_file)
        }
        None => Err(anyhow::anyhow!("Project already exists")),
    }
}

pub fn list_projects() -> Result<Vec<String>> {
    let log_file = read_log_file()?;
    Ok(log_file
        .projects
        .iter()
        .map(|e| e.name.clone())
        .collect::<Vec<String>>())
}

fn read_log_file() -> Result<LogFile> {
    let mut log_string = String::new();
    if let Ok(log_file) = OpenOptions::new().read(true).open(LOG_FILE) {
        let mut log_file = std::io::BufReader::new(log_file);
        log_file.read_to_string(&mut log_string)?;
    } else {
        let log_file = create_log_file()?;
        let mut log_file = std::io::BufReader::new(log_file);
        log_file.read_to_string(&mut log_string)?;
    }
    Ok(serde_yaml::from_str(&log_string)?)
}

fn create_log_file() -> Result<std::fs::File> {
    let log_file = LogFile {
        projects: Vec::new(),
        entries: Vec::new(),
    };
    let starter = serde_yaml::to_string(&log_file)?;
    std::fs::write(LOG_FILE, starter)?;
    Ok(std::fs::File::open(LOG_FILE)?)
}

fn write_log_file(log_file: &LogFile) -> Result<()> {
    let log_file = serde_yaml::to_string(log_file)?;
    Ok(std::fs::write(LOG_FILE, log_file)?)
}
