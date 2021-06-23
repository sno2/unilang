use crate::{expression::FunctionCall, utils::WithSemi, Language, ToCode};

#[derive(Debug)]
/// ## Requirements
/// - Golang requires the "fmt" package.
/// - C++ requires the "std" namespace and the inclusion of "iostream".
pub struct Println<T: ToCode>(pub T);

impl<T: 'static + ToCode> ToCode for Println<T> {
	fn to_code(&self, language: Language) -> String {
		let Self(val) = self;
		let item = Box::new(val.to_code(language)) as Box<dyn ToCode>;
		(match language {
			Language::Python { .. } => return FunctionCall("print", vec![item]).to_code(language),
			Language::TypeScript => WithSemi(FunctionCall("console.log", vec![item])),
			Language::Rust => WithSemi(FunctionCall("println!", vec![Box::new("\"{:?}\""), item])),
			Language::CPP => return format!("cout << {};", val.to_code(language)),
		})
		.to_code(language)
	}
}
