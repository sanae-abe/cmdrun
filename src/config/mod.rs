pub mod loader;
pub mod schema;
pub mod validation;

pub use loader::ConfigLoader;
pub use schema::{CommandsConfig, Command, CommandSpec, GlobalConfig, Language, Platform};
pub use validation::{ConfigValidator, ValidationError, DependencyGraph};
