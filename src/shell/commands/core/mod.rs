//! Static command metadata shared by dispatch, help and completion.
//!
//! Phase 1 keeps every existing command body in place and wraps it in
//! [`Handler::Legacy`]. Command modules move under `builtin/` in later phases
//! without changing the registry shape.

pub(crate) mod completion;
pub(crate) mod help;
pub(crate) mod registry;
pub(crate) mod spec;
