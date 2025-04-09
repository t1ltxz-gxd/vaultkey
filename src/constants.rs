use once_cell::sync::Lazy;

/// Uppercase letters used for password generation
pub(crate) static UPPERCASE: Lazy<&str> = Lazy::new(|| "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
/// Lowercase letters used for password generation
pub(crate) static LOWERCASE: Lazy<&str> = Lazy::new(|| "abcdefghijklmnopqrstuvwxyz");
/// Digits used for password generation
pub(crate) static DIGITS: Lazy<&str> = Lazy::new(|| "0123456789");
/// Special characters used for password generation
pub(crate) static SPECIALS: Lazy<&str> = Lazy::new(|| "!@#$%^&*()-_=+[]{}|;:,.<>?/");
/// Characters considered ambiguous and potentially confusing to read
pub(crate) static AMBIGUOUS: Lazy<&str> = Lazy::new(|| "Il1O0");
