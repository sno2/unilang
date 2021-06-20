mod models;

pub use models::*;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic_function() {
		let func = Function::default()
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

		assert_eq!(
			func.to_code(Language::Rust),
			"pub fn add(a:i32,b:i32)->i32{let sum=a+b;return sum;}"
		);

		assert_eq!(
			func.to_code(Language::TypeScript),
			"export function add(a:number,b:number):number{const sum=a+b;return sum;}"
		);
	}
}
