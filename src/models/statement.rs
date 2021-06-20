pub use crate::{Language, ToCode};

#[derive(Debug)]
pub struct VariableInit<T: ToCode> {
	pub name: String,
	pub mutable: Option<bool>,
	pub typ: Option<String>,
	pub value: T,
}

impl<T: ToCode> ToCode for VariableInit<T> {
	fn to_code(&self, language: Language) -> String {
		let Self {
			name,
			mutable,
			typ,
			value,
		} = self;

		match language {
			Language::Rust => format!(
				"let {}{}{}={};",
				match mutable {
					Some(true) => {
						String::from("mut ")
					}
					_ => String::new(),
				},
				name,
				match typ {
					Some(typ) => format!(":{}", typ),
					None => String::new(),
				},
				value.to_code(language)
			),
			Language::TypeScript => {
				format!(
					"{} {}{}={};",
					match mutable {
						Some(true) => String::from("let"),
						Some(false) | None => String::from("const"),
					},
					name,
					match typ {
						Some(typ) => format!(":{}", typ),
						None => String::new(),
					},
					value.to_code(language)
				)
			}
		}
	}
}

#[derive(Debug)]
pub struct AssignVariable<T: ToCode> {
	pub name: String,
	pub value: T,
}

impl<T: ToCode> ToCode for AssignVariable<T> {
	fn to_code(&self, language: Language) -> String {
		let Self { name, value } = self;
		match language {
			Language::Rust | Language::TypeScript => {
				format!("{}={};", name, value.to_code(language))
			}
		}
	}
}

#[derive(Debug)]
pub struct Return<T: ToCode>(pub Option<T>);

impl<T: ToCode> ToCode for Return<T> {
	fn to_code(&self, language: Language) -> String {
		let Self(expr) = self;
		match language {
			Language::Rust | Language::TypeScript => match expr {
				Some(expr) => format!("return {};", expr.to_code(language)),
				None => format!("return;"),
			},
		}
	}
}

pub use crate::expression::Raw;
