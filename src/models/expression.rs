use crate::{Language, ToCode};

#[derive(Debug)]
pub struct Ternary<T: ToCode, F: ToCode, E: ToCode>(pub T, pub F, pub E);

impl<T: ToCode, F: ToCode, E: ToCode> ToCode for Ternary<T, F, E> {
	fn to_code(&self, language: Language) -> String {
		let Self(condition, opt_if, opt_else) = self;
		match language {
			Language::Rust => {
				format!(
					"if {}{{{}}}else{{{}}}",
					condition.to_code(language),
					opt_if.to_code(language),
					opt_else.to_code(language)
				)
			}
			Language::TypeScript => {
				format!(
					"{}?{}:{}",
					condition.to_code(language),
					opt_if.to_code(language),
					opt_else.to_code(language)
				)
			}
		}
	}
}

#[derive(Debug)]
pub struct Raw(pub String);

impl ToCode for Raw {
	fn to_code(&self, _: Language) -> String {
		self.0.clone()
	}
}

pub mod operation {
	use crate::{Language, ToCode};

	// TODO: create macro for this

	#[derive(Debug)]
	pub struct Add<T: ToCode, F: ToCode>(pub T, pub F);

	impl<T: ToCode, F: ToCode> ToCode for Add<T, F> {
		fn to_code(&self, language: Language) -> String {
			let Self(a, b) = self;
			match language {
				Language::Rust | Language::TypeScript => {
					format!("{}+{}", a.to_code(language), b.to_code(language))
				}
			}
		}
	}

	#[derive(Debug)]
	pub struct Subtract<T: ToCode, F: ToCode>(pub T, pub F);

	impl<T: ToCode, F: ToCode> ToCode for Subtract<T, F> {
		fn to_code(&self, language: Language) -> String {
			let Self(a, b) = self;
			match language {
				Language::Rust | Language::TypeScript => {
					format!("{}-{}", a.to_code(language), b.to_code(language))
				}
			}
		}
	}

	#[derive(Debug)]
	pub struct Multiply<T: ToCode, F: ToCode>(pub T, pub F);

	impl<T: ToCode, F: ToCode> ToCode for Multiply<T, F> {
		fn to_code(&self, language: Language) -> String {
			let Self(a, b) = self;
			match language {
				Language::Rust | Language::TypeScript => {
					format!("{}*{}", a.to_code(language), b.to_code(language))
				}
			}
		}
	}

	#[derive(Debug)]
	pub struct Divide<T: ToCode, F: ToCode>(pub T, pub F);

	impl<T: ToCode, F: ToCode> ToCode for Divide<T, F> {
		fn to_code(&self, language: Language) -> String {
			let Self(a, b) = self;
			match language {
				Language::Rust | Language::TypeScript => {
					format!("{}*{}", a.to_code(language), b.to_code(language))
				}
			}
		}
	}
}
