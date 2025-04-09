/// Configuration options for password generation
#[derive(Debug)]
pub(crate) struct PasswordOptions {
	/// Length of the password
	pub(crate) length: usize,
	/// Whether to include uppercase letters
	pub(crate) include_uppercase: bool,
	/// Whether to include lowercase letters
	pub(crate) include_lowercase: bool,
	/// Whether to include digits
	pub(crate) include_digits: bool,
	/// Whether to include special characters
	pub(crate) include_specials: bool,
	/// Minimum number of digits required
	pub(crate) min_digits: usize,
	/// Minimum number of special characters required
	pub(crate) min_specials: usize,
	/// Whether to avoid ambiguous characters
	pub(crate) avoid_ambiguous: bool,
}
