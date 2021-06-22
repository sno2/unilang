use unilang::*;

fn main() {
	let program = Scope::default()
		.with(Comment::Doc(String::from(
			"Generates the sum of the series of numbers in the fibonacci set.",
		)))
		.with(
			Function::default()
				.with_visibility(Visibility::Public)
				.with_name("fibonacci")
				.with_return_type(types::Integer)
				.with_param(Parameter(Box::new("num"), Some(Box::new(types::Integer))))
				.with_scope(
					Scope::default()
						.with(
							block::ConditionalBuilder::default().with_if(
								block::If::default()
									.with_condition(condition::Or(
										condition::Equal("num", 0),
										condition::Equal("num", 1),
									))
									.with_scope(Scope::default().with(statement::Return(Some(1)))),
							),
						)
						.with(statement::Return(Some(operation::Add(
							expression::FunctionCall(
								"fibonacci",
								vec![Box::new(operation::Subtract("num", 1))],
							),
							expression::FunctionCall(
								"fibonacci",
								vec![Box::new(operation::Subtract("num", 2))],
							),
						)))),
				),
		);
	for language in vec![
		Language::Rust,
		Language::TypeScript,
		Language::CPP,
		Language::Python {
			include_types: true,
			indent_level: None,
		},
	]
	.iter()
	{
		std::fs::write(
			format!(
				"./examples/__fibonacci.{}",
				match language {
					Language::Rust => "rs",
					Language::TypeScript => "ts",
					Language::CPP => "cpp",
					Language::Python { .. } => "py",
				}
			),
			program.to_code(*language),
		)
		.unwrap()
	}
}
