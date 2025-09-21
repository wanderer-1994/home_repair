//! Strong typed entity IDs.
//! Natively, all database entity modals having same numeric type i64, which can be mistakenly swapped together.
//! This module defines strong typed to differentiate them out.

mod macros;

mod entity_ids;
pub use entity_ids::*;

mod account;
pub use account::*;
