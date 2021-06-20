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
