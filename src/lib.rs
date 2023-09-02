/*
MoKa Reads Core Library
Developed by Mustafif Khan

This is built to serve as a core library for
various MoKa Reads application/tools.

All code is under the GPLv2 License (LICENSE.md).
 */

#[cfg(feature = "experimental")]
/// This is still experimental
pub mod latex;
/// The different MoKa Reads Resources
pub mod resources;
/// Awesome Lists of various topics in GitHub
pub mod awesome_lists;

pub use rss::*;
/// A generic Result type
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;