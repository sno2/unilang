use crate::{new_comparator, Language, ToCode};

#[derive(Debug)]
pub struct Equal<T: ToCode, F: ToCode>(pub T, pub F);

impl<T: ToCode, F: ToCode> ToCode for Equal<T, F> {
	fn to_code(&self, language: Language) -> String {
		let Self(a, b) = self;
		format!(
			"({}{}{})",
			a.to_code(language),
			match language {
				Language::TypeScript | Language::CPP => "===",
				Language::Rust | Language::Python { .. } => "==",
			},
			b.to_code(language)
		)
	}
}

new_comparator!(GreaterThan, ">");
new_comparator!(LessThan, "<");
new_comparator!(LessThanOrEq, "<=");
new_comparator!(GreaterThanOrEq, ">=");

#[derive(Debug)]
pub struct And<T: ToCode, F: ToCode>(pub T, pub F);

impl<T: ToCode, F: ToCode> ToCode for And<T, F> {
	fn to_code(&self, language: Language) -> String {
		let Self(a, b) = self;
		format!(
			"({}{}{})",
			a.to_code(language),
			match language {
				Language::Python { .. } => " and ",
				_ => "&&",
			},
			b.to_code(language)
		)
	}
}

#[derive(Debug)]
pub struct Or<T: ToCode, F: ToCode>(pub T, pub F);

impl<T: ToCode, F: ToCode> ToCode for Or<T, F> {
	fn to_code(&self, language: Language) -> String {
		let Self(a, b) = self;
		format!(
			"({}{}{})",
			a.to_code(language),
			match language {
				Language::Python { .. } => " or ",
				_ => "||",
			},
			b.to_code(language)
		)
	}
}
