use itertools::Itertools;

use crate::{Language, ToCode};

#[derive(Debug)]
pub struct Scope {
	children: Vec<Box<dyn ToCode>>,
	indent_level: Option<u32>,
}

impl std::default::Default for Scope {
	#[inline]
	fn default() -> Self {
		Self {
			children: Vec::new(),
			indent_level: None,
		}
	}
}

impl Scope {
	#[inline]
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
				Language::Python {
					indent_level,
					indent_type,
					..
				} => {
					let indent: String = indent_type.into();
					format!(
						"\n{}{}",
						indent.repeat(indent_level.unwrap_or(0) as usize),
						itm.to_code(language)
					)
				}
				_ => itm.to_code(language),
			})
			.join("")
	}
}
