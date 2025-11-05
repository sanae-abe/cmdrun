//! Configuration loader
//! To be implemented in Day 3

use crate::config::schema::CommandsConfig;
use anyhow::Result;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn new() -> Self {
        Self
    }

    pub async fn load(&self) -> Result<CommandsConfig> {
        // Placeholder implementation
        todo!("Configuration loader to be implemented in Day 3")
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}
