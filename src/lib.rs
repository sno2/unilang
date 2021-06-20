mod models;

pub use models::*;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let func = Function {
			name: Box::new(String::from("subtract")),
			visibility: Visibility::Public,
			return_type: Some(Box::new(String::from("i32"))),
			params: vec![
				Parameter {
					name: Box::new("a"),
					typ: Some(Box::new("i32")),
				},
				Parameter {
					name: Box::new("b"),
					typ: Some(Box::new("i32")),
				},
			],
			content: vec![Box::new(statement::VariableInit {
				name: Box::new(String::from("difference")),
				mutable: Some(false),
				typ: None,
				value: Box::new(operation::Add("a", "b")),
			})],
		};

		let language = Language::TypeScript;

		let func2 = Function::default()
			.with_visibility(Visibility::Public)
			.with_name("add")
			.with_return_type(types::Number)
			.with_param(Parameter::default().with_name("a").with_type(types::Number))
			.with_param(Parameter::default().with_name("b").with_type(types::Number))
			.with_statement(
				statement::VariableInit::default()
					.with_name("sum")
					.with_value(operation::Add("a", "b")),
			)
			.with_statement(statement::Return(Some("sum")));

		let extension = match language {
			Language::Rust => "rs",
			Language::TypeScript => "ts",
		};

		std::fs::write(format!("./test.{}", extension), func2.to_code(language)).unwrap();
	}
}
