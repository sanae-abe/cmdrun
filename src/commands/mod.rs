//! Command implementations

pub mod add;
pub mod completion;
pub mod config;
pub mod edit;
pub mod env;
pub mod history;
pub mod info;
pub mod init;
pub mod open;
pub mod plugin;
pub mod remove;
pub mod search;
pub mod template;
pub mod validate;
pub mod watch;

// Re-export command handlers
pub use add::handle_add;
pub use completion::handle_completion;
pub use config::{handle_get, handle_set, handle_show};
pub use edit::handle_edit;
pub use env::{handle_create, handle_current, handle_info as handle_env_info, handle_list as handle_env_list, handle_set as handle_env_set, handle_use};
pub use history::{
    handle_history, handle_history_clear, handle_history_export, handle_history_search,
    handle_retry, ExportFormat,
};
pub use info::handle_info;
pub use init::handle_init;
pub use open::handle_open;
pub use remove::handle_remove;
pub use search::handle_search;
pub use template::{
    handle_template_add, handle_template_export, handle_template_import, handle_template_list,
    handle_template_remove, handle_template_use,
};
pub use validate::handle_validate;
pub use watch::handle_watch;
