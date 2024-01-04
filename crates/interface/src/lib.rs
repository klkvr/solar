//! Source positions and related helper functions.
//!
//! Important concepts in this module include:
//!
//! - the *span*, represented by [`Span`] and related types;
//! - source code as represented by a [`SourceMap`]; and
//! - interned strings, represented by [`Symbol`]s, with some common symbols available statically in
//!   the [`sym`] module.
//!
//! ## Note
//!
//! This API is completely unstable and subject to change.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/danipopes/sulk/main/assets/logo.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/danipopes/sulk/main/assets/favicon.ico"
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(feature = "nightly", feature(min_specialization))]

use std::process::ExitCode;

pub mod diagnostics;
use diagnostics::{ErrorGuaranteed, FatalError};

mod globals;
pub use globals::SessionGlobals;

mod pos;
pub use pos::{BytePos, CharPos, Pos};

pub mod source_map;
pub use source_map::SourceMap;

mod span;
pub use span::Span;

mod symbol;
pub use symbol::{kw, sym, Ident, Symbol};

pub use anstream::ColorChoice;

/// Creates a new compiler session on the current thread if it doesn't exist already and then
/// executes the given closure, catching fatal errors and returning them as [`ErrorGuaranteed`].
///
/// # Errors
///
/// Returns [`ErrorGuaranteed`] if a [`FatalError`] was caught. Other panics are propagated.
pub fn enter<R>(f: impl FnOnce() -> R) -> Result<R, ErrorGuaranteed> {
    SessionGlobals::with_or_default(|_| FatalError::catch(f))
}

/// Creates a new compiler session on the current thread if it doesn't exist already and then
/// executes the given closure, catching fatal errors and returning them as [`ExitCode::FAILURE`].
pub fn enter_with_exit_code(f: impl FnOnce() -> Result<(), ErrorGuaranteed>) -> ExitCode {
    SessionGlobals::with_or_default(|_| FatalError::catch_with_exit_code(f))
}
