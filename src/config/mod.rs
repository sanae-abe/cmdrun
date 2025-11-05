pub mod loader;
pub mod schema;
pub mod validation;

pub use loader::ConfigLoader;
pub use schema::{Command, CommandSpec, CommandsConfig, GlobalConfig, Language, Platform};
pub use validation::{ConfigValidator, DependencyGraph, ValidationError};
