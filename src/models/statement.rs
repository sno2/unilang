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
			Language::CPP => {
				format!(
					"{} {}={};",
					typ.as_ref().unwrap().to_code(language),
					name.to_code(language),
					value.to_code(language)
				)
			}
			Language::Python { include_types, .. } => {
				format!(
					"{}{}={}",
					name.to_code(language),
					if include_types && typ.is_some() {
						format!(":{}", typ.as_ref().unwrap().to_code(language))
					} else {
						String::new()
					},
					value.to_code(language)
				)
			}
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
			Language::Rust | Language::TypeScript | Language::CPP => {
				format!("{}={};", name.to_code(language), value.to_code(language))
			}
			Language::Python { .. } => {
				format!("{}={}", name.to_code(language), value.to_code(language))
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
			Language::Rust | Language::TypeScript | Language::CPP => match expr {
				Some(expr) => format!("return {};", expr.to_code(language)),
				None => String::from("return;"),
			},
			Language::Python { .. } => match expr {
				Some(expr) => format!("return {}", expr.to_code(language)),
				None => String::from("return"),
			},
		}
	}
}

#[derive(Debug)]
pub enum Comment {
	/// ## Notes
	/// * inserts newline after comment if the language's comment spans rest of
	///   the line
	Regular(String),
	MultiLine(String),
	Doc(String),
}

impl ToCode for Comment {
	fn to_code(&self, language: Language) -> String {
		match self {
			Self::Regular(content) => match language {
				Language::Rust | Language::TypeScript | Language::CPP => format!("//{}\n", content),
				Language::Python { .. } => format!("#{}", content),
			},
			Self::MultiLine(content) => match language {
				Language::Rust | Language::TypeScript | Language::CPP => format!("/*{}*/", content),
				Language::Python { .. } => format!("\"\"\"{}\"\"\"", content),
			},
			Self::Doc(content) => match language {
				Language::Rust => format!("///{}\n", content),
				Language::TypeScript => format!("/**{}*/", content),
				Language::CPP => Self::Regular(content.clone()).to_code(language),
				Language::Python { .. } => Self::MultiLine(content.clone()).to_code(language),
			},
		}
	}
}
