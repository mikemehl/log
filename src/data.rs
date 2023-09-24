use anyhow::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Read};

use crate::cli::Period;

const LOG_FILE: &str = ".timelog.yaml";
const DATE_IN_FORMAT: &str = "%Y-%m-%d %H:%M";

#[derive(Serialize, Deserialize)]
pub struct ProjectDef {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct TimeEntry {
    pub id: i32,
    pub project: String,
    pub tag: Option<String>,
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
}

#[derive(Serialize, Deserialize)]
pub struct LogFile {
    pub projects: Vec<ProjectDef>,
    pub entries: Vec<TimeEntry>,
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

pub fn list_entries() -> Result<Vec<TimeEntry>> {
    let log_file = read_log_file()?;
    Ok(log_file.entries)
}

pub fn start_entry(project: String, tag: Option<String>, start: DateTime<Local>) -> Result<()> {
    let mut log_file = read_log_file()?;
    if !check_project_exists(&log_file, project.as_str()) {
        return Err(anyhow::anyhow!("Project does not exist"));
    }
    if check_project_started(&log_file, project.as_str()) {
        return Err(anyhow::anyhow!("Project already started"));
    }
    if let Some(entry) = check_any_project_started(&mut log_file) {
        entry.end = Some(start);
    }
    let id = log_file.entries.len() as i32;
    log_file.entries.push(TimeEntry {
        id,
        project,
        tag,
        start,
        end: None,
    });
    write_log_file(&log_file)
}

pub fn stop_entry() -> Result<()> {
    let mut log_file = read_log_file()?;
    if let Some(entry) = check_any_project_started(&mut log_file) {
        entry.end = Some(Local::now());
        write_log_file(&log_file)
    } else {
        Err(anyhow::anyhow!("No project started"))
    }
}

pub fn update_entry(id: i32, start: String, end: String) -> Result<()> {
    let start = DateTime::<FixedOffset>::parse_from_str(&start, DATE_IN_FORMAT)?;
    let end = DateTime::<FixedOffset>::parse_from_str(&end, DATE_IN_FORMAT)?;
    let mut log_file = read_log_file()?;
    if let Some(entry) = log_file.entries.iter_mut().find(|e| e.id == id) {
        entry.start = start.into();
        entry.end = Some(end.into());
        write_log_file(&log_file)
    } else {
        Err(anyhow::anyhow!("No entry with that id"))
    }
}

pub fn delete_entry(id: i32) -> Result<()> {
    let mut log_file = read_log_file()?;
    if log_file.entries.iter_mut().any(|e| e.id == id) {
        log_file.entries.retain(|e| e.id != id);
        write_log_file(&log_file)
    } else {
        Err(anyhow::anyhow!("No entry with that id"))
    }
}

pub fn fetch_entries(
    period: Period,
    project: String,
    tag: Option<String>,
) -> Result<Vec<TimeEntry>> {
    let all_entries = list_entries()?;
    Ok(all_entries
        .iter()
        .filter(|e| {
            let in_period = match period {
                Period::Day => e.start.date_naive() == Local::now().date_naive(),
                Period::Week => e.start.iso_week() == Local::now().iso_week(),
                Period::Month => e.start.month() == Local::now().month(),
                Period::Year => e.start.year() == Local::now().year(),
                Period::All => true,
            };
            let in_project = e.project == project;
            if let Some(tag) = &tag {
                let in_tag = e.tag == Some(tag.clone());
                in_period && in_project && in_tag
            } else {
                in_period && in_project
            }
        })
        .cloned()
        .collect::<Vec<TimeEntry>>())
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

fn check_project_started(log_file: &LogFile, project: &str) -> bool {
    !log_file.entries.is_empty()
        && log_file
            .entries
            .iter()
            .any(|e| e.project == project && e.end.is_none())
}

fn check_project_exists(log_file: &LogFile, project: &str) -> bool {
    log_file.projects.iter().any(|e| e.name == project)
}

fn check_any_project_started(log_file: &mut LogFile) -> Option<&mut TimeEntry> {
    log_file.entries.iter_mut().find(|e| e.end.is_none())
}
