use crate::{statement::Return, types::Integer, Function, Language, Scope, ToCode};

#[derive(Debug)]
pub struct WithSemi<T: ToCode>(pub T);

impl<T: ToCode> ToCode for WithSemi<T> {
	fn to_code(&self, language: Language) -> String {
		format!("{};", self.0.to_code(language))
	}
}

#[derive(Debug)]
pub struct RunScope(pub Scope);

impl ToCode for RunScope {
	fn to_code(&self, language: Language) -> String {
		match language {
			Language::CPP => Function::default()
				.with_name("main")
				.with_return_type(Integer)
				.with_scope(
					Scope::default()
						.with(self.0.to_code(language))
						.with(Return(Some(1))),
				)
				.to_code(language),
			Language::Rust => Function::default()
				.with_name("main")
				.with_scope(Scope::default().with(self.0.to_code(language)))
				.to_code(language),
			Language::TypeScript => self.0.to_code(language),
			Language::Python { .. } => self.0.to_code(language),
		}
	}
}
