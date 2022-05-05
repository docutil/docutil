use serde::{Deserialize, Serialize};
use tracing::info;

use crate::runnable::Runnable;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        let print_task = PrintTask {
            name: "test".into(),
            command_line: "hello, world".into(),
        };

        print_task.name();
        print_task.run().unwrap();
    }
}
