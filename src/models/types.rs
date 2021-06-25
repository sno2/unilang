use crate::{Language, ToCode};
use itertools::Itertools;

#[derive(Debug)]
pub struct Number;

impl ToCode for Number {
	fn to_code(&self, language: Language) -> String {
		String::from(match language {
			Language::Rust => "f32",
			Language::TypeScript => "number",
			Language::CPP => "double",
			Language::Python { .. } => "float",
		})
	}
}

#[derive(Debug)]
pub struct Integer;

impl ToCode for Integer {
	fn to_code(&self, language: Language) -> String {
		String::from(match language {
			Language::CPP => "int",
			Language::Rust => "i32",
			Language::TypeScript => "number",
			Language::Python { .. } => "int",
		})
	}
}

#[derive(Debug)]
pub struct Float;

impl ToCode for Float {
	fn to_code(&self, language: Language) -> String {
		String::from(match language {
			Language::CPP => "double",
			Language::Rust => "f32",
			Language::TypeScript => "number",
			Language::Python { .. } => "float",
		})
	}
}
#[derive(Debug)]
pub struct Boolean;

impl ToCode for Boolean {
	fn to_code(&self, language: Language) -> String {
		String::from(match language {
			Language::Rust | Language::CPP | Language::Python { .. } => "bool",
			Language::TypeScript => "boolean",
		})
	}
}

#[derive(Debug)]
pub struct Generic<'a>(pub String, pub Vec<&'a dyn ToCode>);

impl<'a> ToCode for Generic<'a> {
	fn to_code(&self, language: Language) -> String {
		let Self(name, args) = self;
		let args = args.iter().map(|itm| itm.to_code(language)).join(",");
		match language {
			Language::Rust | Language::TypeScript | Language::CPP => {
				format!("{}<{}>", name, args)
			}
			Language::Python { .. } => {
				format!("{}[{}]", name, args)
			}
		}
	}
}

#[derive(Debug)]
pub struct Future<T: ToCode>(pub T);

impl<T: ToCode + 'static> ToCode for Future<T> {
	fn to_code(&self, language: Language) -> String {
		let Self(value) = self;
		Generic(
			String::from(match language {
				Language::Rust => "Future",
				Language::TypeScript => "Promise",
				Language::CPP | Language::Python { .. } => unimplemented!(),
			}),
			vec![value as &dyn ToCode],
		)
		.to_code(language)
	}
}
