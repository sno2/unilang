pub mod expression;
pub mod operation;
pub mod statement;
pub mod types;

use std::fmt::Debug;

use itertools::{free::join, Itertools};

#[derive(Debug, Clone, Copy)]
pub enum Language {
	Rust,
	TypeScript,
}

pub trait ToCode: Debug {
	fn to_code(&self, language: Language) -> String;
}

impl ToCode for String {
	fn to_code(&self, _: Language) -> String {
		self.clone()
	}
}

impl ToCode for &str {
	fn to_code(&self, _: Language) -> String {
		self.to_string()
	}
}

#[derive(Debug)]
pub enum Visibility {
	Public,
	Private,
}

#[derive(Debug)]
pub struct Parameter {
	pub name: Box<dyn ToCode>,
	pub typ: Option<Box<dyn ToCode>>,
}

impl std::default::Default for Parameter {
	fn default() -> Self {
		Self {
			name: Box::new(String::from("foo")),
			typ: None,
		}
	}
}

impl Parameter {
	pub fn with_name(mut self, name: impl ToCode + 'static) -> Self {
		self.name = Box::new(name);
		self
	}

	pub fn with_type(mut self, typ: impl ToCode + 'static) -> Self {
		self.typ = Some(Box::new(typ));
		self
	}
}

#[derive(Debug)]
pub struct Function {
	pub name: Box<dyn ToCode>,
	pub return_type: Option<Box<dyn ToCode>>,
	pub visibility: Visibility,
	pub params: Vec<Parameter>,
	pub content: Vec<Box<dyn ToCode>>,
}

impl Function {
	pub fn with_name(mut self, name: impl ToCode + 'static) -> Self {
		self.name = Box::new(name);
		self
	}

	pub fn with_return_type(mut self, return_type: impl ToCode + 'static) -> Self {
		self.return_type = Some(Box::new(return_type));
		self
	}

	pub fn with_visibility(mut self, visibility: Visibility) -> Self {
		self.visibility = visibility;
		self
	}

	pub fn with_param(mut self, param: Parameter) -> Self {
		self.params.push(param);
		self
	}

	pub fn with_statement(mut self, statement: impl ToCode + 'static) -> Self {
		self.content.push(Box::new(statement));
		self
	}
}

impl std::default::Default for Function {
	fn default() -> Self {
		Self {
			name: Box::new(String::from("foo")),
			return_type: None,
			visibility: Visibility::Private,
			params: vec![],
			content: vec![],
		}
	}
}

impl ToCode for Parameter {
	fn to_code(&self, language: Language) -> String {
		match language {
			Language::Rust => {
				format!(
					"{}:{}",
					self.name.to_code(language),
					self.typ.as_ref().unwrap().to_code(language)
				)
			}
			Language::TypeScript => match self.typ {
				Some(ref typ) => {
					format!("{}:{}", self.name.to_code(language), typ.to_code(language))
				}
				_ => format!("{}:any", self.name.to_code(language)),
			},
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
					self.content
						.iter()
						.map(|itm| itm.to_code(language))
						.join("")
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
					self.content
						.iter()
						.map(|itm| itm.to_code(language))
						.join("")
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
		}
	}
}

impl<T: ToCode> ToCode for Vec<T> {
	fn to_code(&self, language: Language) -> String {
		join(self.iter().map(|field| field.to_code(language)), "")
	}
}
