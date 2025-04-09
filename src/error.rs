use thiserror::Error;

/// Errors that can occur when working with vault keys.
#[derive(Debug, Error)]
pub(crate) enum VaultKeyError {
	/// Error indicating that the provided password is too short.
	#[error("Password length must be at least 5")]
	PasswordTooShort,

	/// Error indicating that no character types were selected for password generation.
	#[error("No character types selected for password generation")]
	NoCharacterTypesSelected,
}
