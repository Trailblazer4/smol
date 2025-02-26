//! Common definitions that are shared between different parts of the compiler.

// Use sorted sets and maps for consistent output
pub use std::collections::{BTreeMap as Map, BTreeSet as Set};

/// Identifiers.
pub type Id = internment::Intern<String>;

/// Identifier factory
pub fn id(name: &str) -> Id {
    Id::from_ref(name)
}
