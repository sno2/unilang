pub mod block;
pub mod condition;
pub mod expression;
mod import;
pub mod operation;
pub mod print;
mod scope;
pub mod statement;
pub mod types;
pub(crate) mod utils;

pub use import::Import;
pub use scope::Scope;
pub use statement::Comment;
pub use utils::RunScope;

#[derive(Debug)]
pub enum Value {
	False,
	True,
}

impl ToCode for Value {
	fn to_code(&self, language: Language) -> String {
		String::from(match self {
			Self::True => match language {
				Language::Rust | Language::TypeScript | Language::CPP => "true",
				Language::Python { .. } => "True",
			},
			Self::False => match language {
				Language::Rust | Language::TypeScript | Language::CPP => "false",
				Language::Python { .. } => "False",
			},
		})
	}
}

use std::fmt::Debug;

use itertools::{free::join, Itertools};

#[derive(Debug, Clone, Copy)]
pub enum IndentType {
	Tab,
	/// Include the amount of spaces for each indentation here.
	Space(u16),
}

impl Into<String> for IndentType {
	fn into(self) -> String {
		match self {
			Self::Tab => String::from("\t"),
			Self::Space(times) => " ".repeat(times as usize),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Language {
	Rust,
	TypeScript,
	CPP,
	Python {
		include_types: bool,
		/// **ALWAYS** set this to [`None`]
		indent_level: Option<u32>,
		indent_type: IndentType,
	},
}

pub trait ToCode: std::fmt::Debug {
	fn to_code(&self, language: Language) -> String;
}

impl ToCode for isize {
	#[inline]
	fn to_code(&self, _: Language) -> String {
		self.to_string()
	}
}

impl ToCode for i32 {
	#[inline]
	fn to_code(&self, _: Language) -> String {
		self.to_string()
	}
}

impl ToCode for usize {
	#[inline]
	fn to_code(&self, _: Language) -> String {
		self.to_string()
	}
}

impl ToCode for u32 {
	#[inline]
	fn to_code(&self, _: Language) -> String {
		self.to_string()
	}
}

impl ToCode for String {
	#[inline]
	fn to_code(&self, _: Language) -> String {
		self.clone()
	}
}

impl ToCode for &str {
	#[inline]
	fn to_code(&self, _: Language) -> String {
		self.to_string()
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Visibility {
	Public,
	Private,
}

#[derive(Debug)]
pub struct Parameter(pub Box<dyn ToCode>, pub Option<Box<dyn ToCode>>);

#[derive(Debug)]
pub struct Function {
	pub name: String,
	pub return_type: Option<Box<dyn ToCode>>,
	pub visibility: Visibility,
	pub params: Vec<Parameter>,
	pub scope: Scope,
}

#[derive(Debug)]
pub struct FunctionBuilder {
	pub name: Option<String>,
	pub return_type: Option<Box<dyn ToCode>>,
	pub visibility: Option<Visibility>,
	pub params: Vec<Parameter>,
	pub scope: Option<Scope>,
}

impl FunctionBuilder {
	#[inline]
	pub fn new() -> Self {
		FunctionBuilder {
			name: None,
			return_type: None,
			visibility: None,
			params: Vec::new(),
			scope: None,
		}
	}

	#[inline]
	pub fn name<T: AsRef<str>>(mut self, name: T) -> Self {
		self.name = Some(name.as_ref().to_owned());
		self
	}

	#[inline]
	pub fn with_return_type(mut self, return_type: impl ToCode + 'static) -> Self {
		self.return_type = Some(Box::new(return_type));
		self
	}

	#[inline]
	pub fn with_visibility(mut self, visibility: Visibility) -> Self {
		self.visibility = Some(visibility);
		self
	}

	#[inline]
	pub fn with_param(mut self, param: Parameter) -> Self {
		self.params.push(param);
		self
	}

	#[inline]
	pub fn with_scope(mut self, scope: Scope) -> Self {
		self.scope = Some(scope);
		self
	}

	pub fn build(self) -> Option<Function> {
		Some(Function {
			name: self.name?,
			visibility: self.visibility.unwrap_or(Visibility::Private),
			return_type: self.return_type,
			params: self.params,
			scope: self.scope?,
		})
	}
}

impl ToCode for Parameter {
	fn to_code(&self, language: Language) -> String {
		let Self(name, typ) = self;
		match language {
			Language::Rust => {
				format!(
					"{}:{}",
					name.to_code(language),
					typ.as_ref().unwrap().to_code(language)
				)
			}
			Language::TypeScript => match typ {
				Some(typ) => {
					format!("{}:{}", name.to_code(language), typ.to_code(language))
				}
				_ => format!("{}:any", name.to_code(language)),
			},
			Language::CPP => match typ {
				Some(typ) => {
					format!("{} {}", typ.to_code(language), name.to_code(language))
				}
				_ => panic!("A type is required for parameters in C++."),
			},
			Language::Python { include_types, .. } => format!(
				"{}{}",
				name.to_code(language),
				if include_types && typ.is_some() {
					format!(":{}", typ.as_ref().unwrap().to_code(language))
				} else {
					String::new()
				}
			),
		}
	}
}

impl ToCode for Function {
	fn to_code(&self, language: Language) -> String {
		match language {
			Language::Rust => {
				format!(
					"{}{}fn {}({}){}{{{}}}",
					self.visibility.to_code(language),
					match self.visibility {
						Visibility::Public => " ",
						_ => "",
					},
					self.name.to_code(language),
					self.params
						.iter()
						.map(|param| param.to_code(language))
						.join(","),
					match self.return_type {
						Some(ref r_type) => format!("->{}", r_type.to_code(language)),
						None => String::new(),
					},
					self.scope.to_code(language)
				)
			}
			Language::TypeScript => {
				format!(
					"{}{}function {}({}){}{{{}}}",
					self.visibility.to_code(language),
					match self.visibility {
						Visibility::Public => String::from(" "),
						Visibility::Private => String::new(),
					},
					self.name.to_code(language),
					self.params
						.iter()
						.map(|param| param.to_code(language))
						.join(","),
					match self.return_type {
						Some(ref typ) => format!(":{}", typ.to_code(language)),
						None => String::from(":void"),
					},
					self.scope.to_code(language)
				)
			}
			Language::CPP => {
				format!(
					"{} {}({}){{{}}}",
					match self.return_type {
						Some(ref typ) => typ.to_code(language),
						None => String::from("void"),
					},
					self.name.to_code(language),
					self.params
						.iter()
						.map(|param| {
							let Parameter(name, typ) = param;
							let typ = typ.as_ref().expect("A parameter type is required");
							format!("{} {}", typ.to_code(language), name.to_code(language))
						})
						.join(","),
					self.scope.to_code(language)
				)
			}
			Language::Python {
				include_types,
				indent_level,
				indent_type,
			} => {
				format!(
					"def {}({}){}:{}",
					self.name.to_code(language),
					self.params
						.iter()
						.map(|itm| itm.to_code(language))
						.join(","),
					if include_types && self.return_type.is_some() {
						format!("->{}", self.return_type.as_ref().unwrap().to_code(language))
					} else {
						String::new()
					},
					self.scope.to_code(Language::Python {
						include_types,
						indent_level: Some(indent_level.unwrap_or(0) + 1),
						indent_type
					})
				)
			}
		}
	}
}

impl ToCode for Visibility {
	fn to_code(&self, language: Language) -> String {
		match language {
			Language::Rust => match self {
				Visibility::Public => String::from("pub"),
				Visibility::Private => String::new(),
			},
			Language::TypeScript => match self {
				Visibility::Public => String::from("export"),
				Visibility::Private => String::new(),
			},
			_ => unimplemented!(),
		}
	}
}

impl<T: ToCode> ToCode for Vec<T> {
	fn to_code(&self, language: Language) -> String {
		join(self.iter().map(|field| field.to_code(language)), "")
	}
}
