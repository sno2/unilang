use itertools::Itertools;

use crate::{Language, Scope, ToCode};

#[derive(Debug)]
pub struct ConditionalBuilder {
	first: If,
	middle: Vec<If>,
	last: Option<Else>,
}

impl ConditionalBuilder {
	pub fn with_if(mut self, block: If) -> Self {
		self.first = block;
		self
	}

	pub fn with_else_if(mut self, block: If) -> Self {
		self.middle.push(block);
		self
	}

	pub fn with_else(mut self, block: Else) -> Self {
		self.last = Some(block);
		self
	}
}

impl ToCode for ConditionalBuilder {
	fn to_code(&self, language: Language) -> String {
		let Self {
			first,
			middle,
			last,
		} = self;
		match language {
			Language::Rust | Language::TypeScript | Language::CPP => {
				format!(
					"{}{}{}{}",
					first.to_code(language),
					middle
						.iter()
						.map(|itm| format!("else {}", itm.to_code(language)))
						.join(""),
					match last {
						Some(last) => format!("else{{{}}}", last.scope.to_code(language)),
						None => String::new(),
					},
					match language {
						Language::Rust => ";",
						_ => "",
					}
				)
			}
			Language::Python { .. } => Scope::default()
				.with(self.first.to_code(language))
				.with(
					self.middle
						.iter()
						.map(|itm| format!("el{}", itm.to_code(language)))
						.join(""),
				)
				.with(match self.last {
					Some(ref val) => val.to_code(language),
					_ => String::new(),
				})
				.to_code(language),
		}
	}
}

impl Default for ConditionalBuilder {
	fn default() -> Self {
		Self {
			first: If::default(),
			middle: Vec::new(),
			last: None,
		}
	}
}

#[derive(Debug)]
pub struct If {
	condition: Box<dyn ToCode>,
	scope: Scope,
}

impl ToCode for If {
	fn to_code(&self, language: Language) -> String {
		let Self { condition, scope } = self;
		match language {
			Language::Rust => format!(
				"if {}{{{}}}",
				condition.to_code(language),
				scope.to_code(language)
			),
			Language::TypeScript | Language::CPP => format!(
				"if({}){{{}}}",
				condition.to_code(language),
				scope.to_code(language)
			),
			Language::Python {
				include_types,
				indent_level,
				indent_type,
			} => format!(
				"if {}:{}",
				condition.to_code(language),
				scope.to_code(Language::Python {
					include_types,
					indent_level: Some(match indent_level {
						Some(lvl) => lvl + 1,
						None => 1,
					}),
					indent_type
				})
			),
		}
	}
}

impl Default for If {
	fn default() -> Self {
		Self {
			condition: Box::new("1"),
			scope: Scope::default(),
		}
	}
}

impl If {
	pub fn with_condition(mut self, condition: impl ToCode + 'static) -> Self {
		self.condition = Box::new(condition);
		self
	}

	pub fn with_scope(mut self, scope: Scope) -> Self {
		self.scope = scope;
		self
	}
}

#[derive(Debug)]
pub struct Else {
	scope: Scope,
}

impl ToCode for Else {
	fn to_code(&self, language: Language) -> String {
		match language {
			Language::Rust | Language::TypeScript | Language::CPP => {
				format!("else{{{}}}", self.scope.to_code(language))
			}
			Language::Python {
				include_types,
				indent_level,
				indent_type,
			} => {
				format!(
					"else:{}",
					self.scope.to_code(Language::Python {
						include_types,
						indent_level: Some(match indent_level {
							Some(lvl) => lvl + 1,
							None => 1,
						}),
						indent_type
					})
				)
			}
		}
	}
}

impl Default for Else {
	fn default() -> Self {
		Self {
			scope: Scope::default(),
		}
	}
}

impl Else {
	pub fn with_scope(mut self, scope: Scope) -> Self {
		self.scope = scope;
		self
	}
}
