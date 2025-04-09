use vaultkey::PasswordBuilder;

fn main() {
	for i in 0..10 {
		let password = PasswordBuilder::default()
			.length(50)
			.with_uppercase(true)
			.min_digits(2)
			.min_specials(2)
			.avoid_ambiguous(true)
			.build()
			.unwrap();

		println!("Generated password {}: {}", i + 1, password);
	}
}
