use std::process;

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::runnable::Runnable;

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
        let output = process::Command::new("nu")
            .arg("-c")
            .arg(&self.command_line)
            .output()
            .expect(&format!("unable to run command: {}", self.command_line));

        info!("{}", std::str::from_utf8(&output.stdout)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        let command_task = CommandTask {
            name: "test".into(),
            command_line: "echo hello, world".into(),
        };

        command_task.name();
        command_task.run().unwrap();
    }
}
