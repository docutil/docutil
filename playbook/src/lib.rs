use std::process;

use tracing::info;
use serde::{Deserialize, Serialize};

pub trait Runnable {
    fn name(&self);
    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintTask {
    pub name: String,

    #[serde(rename(serialize = "print", deserialize = "print"))]
    pub command_line: String,
}

impl Runnable for PrintTask {
    fn name(&self) {
        info!("[TASK] {}", self.name.clone());
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("{}", self.command_line.clone());
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandTask {
    pub name: String,

    #[serde(rename(serialize = "command", deserialize = "command"))]
    pub command_line: String,
}

impl Runnable for CommandTask {
    fn name(&self) {
        info!("[TASK] {}", self.name.clone());
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let output = process::Command::new("sh")
            .arg("-c")
            .arg(&self.command_line)
            .output()
            .expect(&format!("unable to run command: {}", self.command_line));

        info!("{}", std::str::from_utf8(&output.stdout)?);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Task {
    Print(PrintTask),
    Command(CommandTask),
    // TODO 实现调用插件
    // External(HashMap<String, String>),
}

impl Task {
    pub fn as_runnable(&self) -> Box<dyn Runnable> {
        match self {
            Self::Print(print_task) => Box::new(print_task.clone().to_owned()),
            Self::Command(command_task) => Box::new(command_task.clone().to_owned()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Play {
    pub name: String,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayBook(Vec<Play>);

impl TryFrom<&str> for PlayBook {
    type Error = serde_yaml::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_yaml::from_str::<PlayBook>(value)
    }
}

impl PlayBook {
    pub fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        self.0.into_iter().for_each(|plays| {
            info!("[PLAYBOOK] running plays: {}", plays.name.clone());

            plays.tasks.iter().for_each(|task| {
                let runnable = task.as_runnable();
                runnable.name();
                runnable.run().unwrap()
            })
        });

        Ok(())
    }
}

pub fn try_from(doc: &str) -> Result<PlayBook, serde_yaml::Error> {
    PlayBook::try_from(doc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_read_a_playbook_file() {
        let doc = include_str!("../examples/demo.yml");
        let playbook = PlayBook::try_from(doc).unwrap();

        println!("{:?}", playbook);
    }

    #[test]
    fn run_playbook() {
        let doc = include_str!("../examples/demo.yml");
        let playbook = PlayBook::try_from(doc).unwrap();

        playbook.start().unwrap();
    }

    #[test]
    fn test_command_task() {
        let command_task = CommandTask {
            name: "test".into(),
            command_line: "echo hello, world".into(),
        };

        command_task.name();
        command_task.run().unwrap();
    }

    #[test]
    fn test_print_task() {
        let print_task = PrintTask {
            name: "test".into(),
            command_line: "hello, world".into(),
        };

        print_task.name();
        print_task.run().unwrap();
    }
}
