pub mod expression;
pub mod statement;

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
	pub name: String,
	pub typ: Option<String>,
}

#[derive(Debug)]
pub struct Function {
	pub name: String,
	pub return_type: Option<String>,
	pub visibility: Visibility,
	pub params: Vec<Parameter>,
	pub content: Vec<Box<dyn ToCode>>,
}

impl ToCode for Parameter {
	fn to_code(&self, language: Language) -> String {
		match language {
			Language::Rust => {
				format!("{}:{}", self.name, self.typ.clone().unwrap())
			}
			Language::TypeScript => match self.typ {
				Some(ref typ) => format!("{}:{}", self.name, typ),
				_ => format!("{}:any", self.name),
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
					self.name,
					self.params
						.iter()
						.map(|param| param.to_code(language))
						.join(","),
					match self.return_type {
						Some(ref r_type) => format!("->{}", r_type),
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
					self.name,
					self.params
						.iter()
						.map(|param| param.to_code(language))
						.join(","),
					match self.return_type {
						Some(ref typ) => format!(":{}", typ),
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
