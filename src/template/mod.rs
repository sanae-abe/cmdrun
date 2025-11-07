//! Template management module
//!
//! Provides functionality to manage command configuration templates

pub mod builtin;
pub mod manager;
pub mod schema;

pub use builtin::BuiltinTemplate;
pub use manager::TemplateManager;
pub use schema::{TemplateMetadata, UserTemplate};
