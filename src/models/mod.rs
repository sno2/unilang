pub mod block;
pub mod condition;
pub mod expression;
mod import;
pub mod operation;
pub mod statement;
pub mod types;

pub use statement::Comment;

pub use import::Import;

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

use std::{
	any::{Any, TypeId},
	fmt::Debug,
};

use itertools::{free::join, Itertools};

#[derive(Debug, Clone, Copy)]
pub enum Language {
	Rust,
	TypeScript,
	CPP,
	Python {
		include_types: bool,
		indent_level: Option<u32>,
	},
}

pub trait ToCode: std::fmt::Debug {
	fn to_code(&self, language: Language) -> String;
}

impl ToCode for isize {
	fn to_code(&self, _: Language) -> String {
		self.to_string()
	}
}

impl ToCode for i32 {
	fn to_code(&self, _: Language) -> String {
		self.to_string()
	}
}

impl ToCode for usize {
	fn to_code(&self, _: Language) -> String {
		self.to_string()
	}
}

impl ToCode for u32 {
	fn to_code(&self, _: Language) -> String {
		self.to_string()
	}
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
pub struct Parameter(pub Box<dyn ToCode>, pub Option<Box<dyn ToCode>>);

#[derive(Debug)]
pub struct Scope {
	children: Vec<Box<dyn ToCode>>,
	indent_level: Option<u32>,
}

impl std::default::Default for Scope {
	fn default() -> Self {
		Self {
			children: Vec::new(),
			indent_level: None,
		}
	}
}

impl Scope {
	pub fn with<T>(mut self, child: T) -> Self
	where
		T: ToCode + 'static,
	{
		self.children.push(Box::new(child));
		self
	}
}

impl ToCode for Scope {
	fn to_code(&self, language: Language) -> String {
		self.children
			.iter()
			.map(|itm| match language {
				Language::Python { indent_level, .. } => {
					format!(
						"\n{}{}",
						"\t".repeat(indent_level.unwrap_or(0) as usize),
						itm.to_code(language)
					)
				}
				_ => itm.to_code(language),
			})
			.join("")
	}
}

#[derive(Debug)]
pub struct Function {
	pub name: Box<dyn ToCode>,
	pub return_type: Option<Box<dyn ToCode>>,
	pub visibility: Visibility,
	pub params: Vec<Parameter>,
	pub scope: Scope,
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

	pub fn with_scope(mut self, scope: Scope) -> Self {
		self.scope = scope;
		self
	}
}

impl std::default::Default for Function {
	fn default() -> Self {
		Self {
			name: Box::new(String::from("foo")),
			return_type: None,
			visibility: Visibility::Private,
			params: Vec::new(),
			scope: Scope::default(),
		}
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
						indent_level: Some(indent_level.unwrap_or(0) + 1)
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
