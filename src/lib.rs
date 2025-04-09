//! A password generation library that provides customizable password creation.
//!
//! This library allows generating secure passwords with various options including:
//! - Configurable length
//! - Character set inclusion (uppercase, lowercase, digits, special characters)
//! - Minimum requirements for specific character types
//! - Option to avoid ambiguous characters
//!
//! # Example
//! ```
//! use vaultkey::PasswordBuilder;
//!
//! let password = PasswordBuilder::default()
//!     .length(16)
//!     .min_digits(2)
//!     .min_specials(2)
//!     .avoid_ambiguous(true)
//!     .build()
//!     .unwrap();
//!
//! ```

#![forbid(unsafe_code)]
#![deny(
	clippy::all,
	clippy::pedantic,
	clippy::nursery,
	clippy::cargo,
	missing_docs,
	unreachable_pub,
	unused_crate_dependencies
)]
#![warn(
	rust_2018_idioms,
	rust_2021_compatibility,
	missing_debug_implementations,
	trivial_casts,
	trivial_numeric_casts,
	unused_import_braces,
	unused_qualifications
)]
#![allow(
	clippy::module_name_repetitions,
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	clippy::must_use_candidate,
	clippy::doc_markdown,
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss,
	clippy::cast_precision_loss,
	clippy::similar_names,
	clippy::struct_excessive_bools
)]
#![doc(
	html_logo_url = "https://raw.githubusercontent.com/t1ltxz-gxd/vaultkey/main/assets/images/logo.png"
)]
#![doc(
	html_favicon_url = "https://raw.githubusercontent.com/t1ltxz-gxd/vaultkey/main/assets/images/favicon.png"
)]
/// Module providing the `PasswordBuilder` for constructing passwords with customizable options.
pub mod builder;
/// Module containing error types and utilities for the password generation library.
pub mod error;
pub use builder::*;

/// Module containing constants used throughout the password generation library.
pub mod constants;
/// Module defining various options and configurations for password generation.
pub mod options;
