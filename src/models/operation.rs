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
