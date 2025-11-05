//! Command implementations

pub mod add;
pub mod completion;
pub mod config;
pub mod edit;
pub mod info;
pub mod init;
pub mod open;
pub mod remove;
pub mod search;
pub mod validate;

// Re-export command handlers
pub use add::handle_add;
pub use completion::handle_completion;
pub use config::{handle_get, handle_set, handle_show};
pub use edit::handle_edit;
pub use info::handle_info;
pub use init::handle_init;
pub use open::handle_open;
pub use remove::handle_remove;
pub use search::handle_search;
pub use validate::handle_validate;
