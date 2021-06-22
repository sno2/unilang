use itertools::Itertools;

use crate::{Language, ToCode};

#[derive(Debug)]
pub struct FunctionCall<T: ToCode>(pub T, pub Vec<Box<dyn ToCode>>);

impl<T: ToCode> ToCode for FunctionCall<T> {
	fn to_code(&self, language: Language) -> String {
		let Self(fn_name, args) = self;
		match language {
			Language::TypeScript | Language::Rust | Language::CPP | Language::Python { .. } => {
				format!(
					"{}({})",
					fn_name.to_code(language),
					args.iter().map(|itm| itm.to_code(language)).join(",")
				)
			}
		}
	}
}

#[derive(Debug)]
pub struct Ternary<T: ToCode, F: ToCode, E: ToCode>(pub T, pub F, pub E);

impl<T: ToCode, F: ToCode, E: ToCode> ToCode for Ternary<T, F, E> {
	fn to_code(&self, language: Language) -> String {
		let Self(condition, opt_if, opt_else) = self;
		match language {
			Language::Rust => {
				format!(
					"if {}{{{}}}else{{{}}}",
					condition.to_code(language),
					opt_if.to_code(language),
					opt_else.to_code(language)
				)
			}
			Language::TypeScript | Language::CPP => {
				format!(
					"{}?{}:{}",
					condition.to_code(language),
					opt_if.to_code(language),
					opt_else.to_code(language)
				)
			}
			Language::Python { .. } => {
				format!(
					"if {} {} else {}",
					condition.to_code(language),
					opt_if.to_code(language),
					opt_else.to_code(language)
				)
			}
		}
	}
}
