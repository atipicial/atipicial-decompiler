//! Error types returned by the library.
//!
//! Most public APIs return [`crate::Result`], which uses [`enum@Error`] as the error
//! type. The variants provide access to more specific error categories when
//! needed.

mod disassembly;
mod manifest;
mod aef;

use std::io;

use thiserror::Error;

pub use disassembly::DisassemblyError;
pub use manifest::ManifestError;
pub use aef::AefError;

/// Convenient result alias for the library.
pub type Result<T> = std::result::Result<T, Error>;

/// Top level error surfaced by the library APIs.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// Errors encountered while parsing a AEF container.
    #[error(transparent)]
    Aef(#[from] AefError),

    /// Errors encountered while decoding Atipicial VM bytecode.
    #[error(transparent)]
    Disassembly(#[from] DisassemblyError),

    /// I/O failures when reading inputs.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Errors encountered while parsing a contract manifest.
    #[error(transparent)]
    Manifest(#[from] ManifestError),
}
