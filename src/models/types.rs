use crate::{Language, ToCode};

#[derive(Debug)]
pub struct Number;

impl ToCode for Number {
	fn to_code(&self, language: Language) -> String {
		match language {
			Language::Rust => String::from("i32"),
			Language::TypeScript => String::from("number"),
		}
	}
}

#[derive(Debug)]
pub struct Boolean;

impl ToCode for Boolean {
	fn to_code(&self, language: Language) -> String {
		match language {
			Language::Rust => String::from("bool"),
			Language::TypeScript => String::from("boolean"),
		}
	}
}
