use core::fmt;
use serde::Serialize;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Serialize)]
pub enum Projects {
    PythonTutor,
}

impl FromStr for Projects {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PythonTutor" => Ok(Projects::PythonTutor),
            _ => Err("Project not found".to_string()),
        }
    }
}

impl Display for Projects {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Projects::PythonTutor => write!(f, "- Python Tutor"),
        }
    }
}

impl Projects {
    pub fn init(&self) {
        match self {
            Projects::PythonTutor => println!("Setting up Python Tutor project..."),
        }
    }
}

pub fn list() {
    println!("Available projects:");
    println!("{}", Projects::PythonTutor);
}

pub fn init(project: &Projects) {
    project.init();
}
