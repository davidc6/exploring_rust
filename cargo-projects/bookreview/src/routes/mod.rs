//! src/routes/mod.rs
// mod health_check;
mod follows;

// a public use declaration can redirect public name to a diff target definition
// pub use health_check::*;
pub use follows::*;
