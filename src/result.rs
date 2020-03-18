//! Definisi untuk system result internal

use crate::error;

/// Generic type of our result
pub type Result<T> = ::std::result::Result<T, error::Error>;
