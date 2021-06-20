pub use crate::{Language, ToCode};

#[derive(Debug)]
pub struct VariableInit {
	pub name: Box<dyn ToCode>,
	pub mutable: Option<bool>,
	pub typ: Option<Box<dyn ToCode>>,
	pub value: Box<dyn ToCode>,
}

impl std::default::Default for VariableInit {
	fn default() -> Self {
		Self {
			name: Box::new("foo"),
			mutable: None,
			typ: None,
			value: Box::new("bar"),
		}
	}
}

impl VariableInit {
	pub fn with_name(mut self, name: impl ToCode + 'static) -> Self {
		self.name = Box::new(name);
		self
	}

	pub fn with_mutable(mut self, is_mutable: bool) -> Self {
		self.mutable = Some(is_mutable);
		self
	}

	pub fn with_type(mut self, typ: impl ToCode + 'static) -> Self {
		self.typ = Some(Box::new(typ));
		self
	}

	pub fn with_value(mut self, value: impl ToCode + 'static) -> Self {
		self.value = Box::new(value);
		self
	}
}

impl ToCode for VariableInit {
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
				name.to_code(language),
				match typ {
					Some(typ) => format!(":{}", typ.to_code(language)),
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
					name.to_code(language),
					match typ {
						Some(typ) => format!(":{}", typ.to_code(language)),
						None => String::new(),
					},
					value.to_code(language)
				)
			}
		}
	}
}

#[derive(Debug)]
pub struct AssignVariable<T: ToCode, F: ToCode>(T, F);

impl<T: ToCode, F: ToCode> ToCode for AssignVariable<T, F> {
	fn to_code(&self, language: Language) -> String {
		let Self(name, value) = self;
		match language {
			Language::Rust | Language::TypeScript => {
				format!("{}={};", name.to_code(language), value.to_code(language))
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
