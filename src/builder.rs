use crate::constants::{AMBIGUOUS, DIGITS, LOWERCASE, SPECIALS, UPPERCASE};
use crate::error::VaultKeyError;
use crate::options::PasswordOptions;
use anyhow::Result;
use rand::{seq::SliceRandom, Rng};

/// Builder for creating passwords with customizable options
#[derive(Debug)]
pub struct PasswordBuilder {
	/// Configuration options for the password
	options: PasswordOptions,
}

impl Default for PasswordBuilder {
	/// Creates a new `PasswordBuilder` with default options:
	/// - Length: 12 characters
	/// - Uppercase letters: included
	/// - Lowercase letters: included
	/// - Digits: included
	/// - Special characters: included
	/// - Minimum digits: 1
	/// - Minimum special characters: 1
	/// - Avoid ambiguous characters: false
	fn default() -> Self {
		Self {
			options: PasswordOptions {
				length: 12,
				include_uppercase: true,
				include_lowercase: true,
				include_digits: true,
				include_specials: true,
				min_digits: 1,
				min_specials: 1,
				avoid_ambiguous: false,
			},
		}
	}
}

impl PasswordBuilder {
	/// Sets the desired length of the password.
	///
	/// # Arguments
	/// * `len` - The length of the password in characters
	#[must_use]
	pub const fn length(mut self, len: usize) -> Self {
		self.options.length = len;
		self
	}

	/// Controls the inclusion of uppercase letters in the password.
	///
	/// # Arguments
	/// * `include` - Whether to include uppercase letters
	#[must_use]
	pub const fn with_uppercase(mut self, include: bool) -> Self {
		self.options.include_uppercase = include;
		self
	}

	/// Controls the inclusion of lowercase letters in the password.
	///
	/// # Arguments
	/// * `include` - Whether to include lowercase letters
	#[must_use]
	pub const fn with_lowercase(mut self, include: bool) -> Self {
		self.options.include_lowercase = include;
		self
	}

	/// Controls the inclusion of digits in the password.
	///
	/// # Arguments
	/// * `include` - Whether to include digits
	#[must_use]
	pub const fn with_digits(mut self, include: bool) -> Self {
		self.options.include_digits = include;
		self
	}

	/// Controls the inclusion of special characters in the password.
	///
	/// # Arguments
	/// * `include` - Whether to include special characters
	#[must_use]
	pub const fn with_specials(mut self, include: bool) -> Self {
		self.options.include_specials = include;
		self
	}

	/// Sets the minimum number of digits required in the password.
	///
	/// # Arguments
	/// * `min` - The minimum number of digits to include
	#[must_use]
	pub const fn min_digits(mut self, min: usize) -> Self {
		self.options.min_digits = min;
		self
	}

	/// Sets the minimum number of special characters required in the password.
	///
	/// # Arguments
	/// * `min` - The minimum number of special characters to include
	#[must_use]
	pub const fn min_specials(mut self, min: usize) -> Self {
		self.options.min_specials = min;
		self
	}

	/// Controls whether to avoid ambiguous characters (I, l, 1, O, 0) in the password.
	///
	/// # Arguments
	/// * `avoid` - Whether to avoid ambiguous characters
	#[must_use]
	pub const fn avoid_ambiguous(mut self, avoid: bool) -> Self {
		self.options.avoid_ambiguous = avoid;
		self
	}

	/// Builds the password with the configured options.
	///
	/// # Returns
	/// A string containing the generated password
	pub fn build(self) -> Result<String> {
		generate_password(&self.options)
	}
}

/// Generates a password based on the given options.
///
/// This function constructs a password that satisfies all the requirements specified
/// in the provided options, including:
/// - Using only the character types that are specified for inclusion
/// - Meeting the minimum requirements for specific character types
/// - Avoiding ambiguous characters if specified
/// - Matching the requested password length
///
/// The generation process works as follows:
/// 1. Build a character pool from the selected character types
/// 2. Filter out ambiguous characters if requested
/// 3. Add the minimum required number of digits and special characters
/// 4. Fill the remaining length with random characters from the pool
/// 5. Shuffle the resulting password for randomness
///
/// # Arguments
/// * `options` - Configuration parameters that control password generation
///
/// # Returns
/// A string containing the generated password, or an empty string if the
/// requested length is 0 or no character types are selected
fn generate_password(options: &PasswordOptions) -> Result<String> {
	let mut rng = rand::rng();
	let mut pool = String::new();

	// Build the character pool based on selected options
	if options.include_uppercase {
		pool.push_str(&UPPERCASE);
	}
	if options.include_lowercase {
		pool.push_str(&LOWERCASE);
	}
	if options.include_digits {
		pool.push_str(&DIGITS);
	}
	if options.include_specials {
		pool.push_str(&SPECIALS);
	}
	if options.avoid_ambiguous {
		pool = pool.chars().filter(|c| !AMBIGUOUS.contains(*c)).collect();
	}

	// Handle edge cases
	if options.length < 5 {
		return Err(VaultKeyError::PasswordTooShort.into());
	}

	if pool.is_empty() {
		return Err(VaultKeyError::NoCharacterTypesSelected.into());
	}

	let mut password = String::with_capacity(options.length);

	// Calculate minimum requirements, ensuring they don't exceed the password length
	let available_length = options.length;
	let min_digits = options.min_digits.min(if options.include_digits {
		available_length
	} else {
		0
	});
	let min_specials = options.min_specials.min(if options.include_specials {
		available_length.saturating_sub(min_digits)
	} else {
		0
	});

	// Helper function to filter ambiguous characters if needed
	let filter_ambiguous = |chars: &str| -> Vec<char> {
		if options.avoid_ambiguous {
			chars.chars().filter(|c| !AMBIGUOUS.contains(*c)).collect()
		} else {
			chars.chars().collect()
		}
	};

	// Prepare filtered character sets
	let digits_chars: Vec<char> = filter_ambiguous(&DIGITS);
	let special_chars: Vec<char> = filter_ambiguous(&SPECIALS);

	// Add required minimum digits
	if options.include_digits && min_digits > 0 && !digits_chars.is_empty() {
		for _ in 0..min_digits {
			let idx = rng.random_range(0..digits_chars.len());
			password.push(digits_chars[idx]);
		}
	}

	// Add required minimum special characters
	if options.include_specials && min_specials > 0 && !special_chars.is_empty() {
		for _ in 0..min_specials {
			let idx = rng.random_range(0..special_chars.len());
			password.push(special_chars[idx]);
		}
	}

	// Fill the remaining length with random characters from the pool
	while password.len() < options.length {
		let idx = rng.random_range(0..pool.len());
		password.push(pool.chars().nth(idx).unwrap());
	}

	// Shuffle the password characters for randomness
	let mut password_chars: Vec<char> = password.chars().collect();
	password_chars.shuffle(&mut rng);
	Ok(password_chars.iter().collect::<String>())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn password_matches_requested_length() {
		let options = PasswordOptions {
			length: 16,
			include_uppercase: true,
			include_lowercase: true,
			include_digits: true,
			include_specials: true,
			min_digits: 1,
			min_specials: 1,
			avoid_ambiguous: false,
		};

		let password = generate_password(&options).unwrap();
		assert_eq!(password.len(), 16);
	}

	#[test]
	fn password_contains_only_uppercase_when_specified() {
		let options = PasswordOptions {
			length: 10,
			include_uppercase: true,
			include_lowercase: false,
			include_digits: false,
			include_specials: false,
			min_digits: 0,
			min_specials: 0,
			avoid_ambiguous: false,
		};

		let password = generate_password(&options).unwrap();
		assert!(password.chars().all(|c| UPPERCASE.contains(c)));
	}

	#[test]
	fn password_contains_only_lowercase_when_specified() {
		let options = PasswordOptions {
			length: 10,
			include_uppercase: false,
			include_lowercase: true,
			include_digits: false,
			include_specials: false,
			min_digits: 0,
			min_specials: 0,
			avoid_ambiguous: false,
		};

		let password = generate_password(&options).unwrap();
		assert!(password.chars().all(|c| LOWERCASE.contains(c)));
	}

	#[test]
	fn password_contains_minimum_required_digits() {
		let options = PasswordOptions {
			length: 20,
			include_uppercase: true,
			include_lowercase: true,
			include_digits: true,
			include_specials: true,
			min_digits: 5,
			min_specials: 2,
			avoid_ambiguous: false,
		};

		let password = generate_password(&options).unwrap();
		let digit_count = password.chars().filter(|c| DIGITS.contains(*c)).count();
		assert!(digit_count >= 5);
	}

	#[test]
	fn password_contains_minimum_required_specials() {
		let options = PasswordOptions {
			length: 20,
			include_uppercase: true,
			include_lowercase: true,
			include_digits: true,
			include_specials: true,
			min_digits: 2,
			min_specials: 7,
			avoid_ambiguous: false,
		};

		let password = generate_password(&options).unwrap();
		let special_count = password.chars().filter(|c| SPECIALS.contains(*c)).count();
		assert!(special_count >= 7);
	}

	#[test]
	fn password_excludes_ambiguous_characters_when_specified() {
		let options = PasswordOptions {
			length: 100,
			include_uppercase: true,
			include_lowercase: true,
			include_digits: true,
			include_specials: false,
			min_digits: 10,
			min_specials: 0,
			avoid_ambiguous: true,
		};

		let password = generate_password(&options).unwrap();
		assert!(!password.chars().any(|c| AMBIGUOUS.contains(c)));
	}

	#[test]
	fn zero_length_returns_empty_string() {
		let options = PasswordOptions {
			length: 0,
			include_uppercase: true,
			include_lowercase: true,
			include_digits: true,
			include_specials: true,
			min_digits: 0,
			min_specials: 0,
			avoid_ambiguous: false,
		};

		let result = generate_password(&options);
		assert!(result.is_err());
		assert_eq!(
			result.unwrap_err().to_string(),
			"Password length must be at least 5"
		);
	}

	#[test]
	fn handles_large_password_lengths() {
		let options = PasswordOptions {
			length: 1000,
			include_uppercase: true,
			include_lowercase: true,
			include_digits: true,
			include_specials: true,
			min_digits: 100,
			min_specials: 100,
			avoid_ambiguous: false,
		};

		let password = generate_password(&options).unwrap();
		assert_eq!(password.len(), 1000);
	}

	#[test]
	fn handles_minimum_requirements_exceeding_length() {
		let options = PasswordOptions {
			length: 5,
			include_uppercase: true,
			include_lowercase: true,
			include_digits: true,
			include_specials: true,
			min_digits: 3,
			min_specials: 4,
			avoid_ambiguous: false,
		};

		let password = generate_password(&options).unwrap();
		assert_eq!(password.len(), 5);
	}
}
