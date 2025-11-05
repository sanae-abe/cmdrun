//! Command implementations

pub mod init;
pub mod validate;
pub mod completion;
pub mod remove;
pub mod add;
pub mod open;
pub mod edit;
pub mod info;
pub mod search;
pub mod config;

// Re-export command handlers
pub use init::handle_init;
pub use validate::handle_validate;
pub use completion::handle_completion;
pub use remove::handle_remove;
pub use add::handle_add;
pub use open::handle_open;
pub use edit::handle_edit;
pub use info::handle_info;
pub use search::handle_search;
pub use config::{handle_get, handle_set, handle_show};
