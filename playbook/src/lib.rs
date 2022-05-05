use serde::{Deserialize, Serialize};
use tracing::info;

mod builtin;
mod runnable;

use crate::{
    builtin::{CommandTask, PrintTask},
    runnable::Runnable,
};

pub use runnable::*;

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

            plays.tasks.into_iter().for_each(|task| {
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
    fn try_read_playbook_file() {
        let doc = include_str!("../examples/demo.yml");
        let playbook = PlayBook::try_from(doc).unwrap();

        println!("{:?}", playbook);
    }

    #[test]
    fn run_playbook() {
        let doc = include_str!("../examples/demo.yml");
        let playbook = try_from(doc).unwrap();

        playbook.start().unwrap();
    }
}
