#[macro_use]
pub(crate) mod macros;
mod models;

pub use models::*;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic_function() {
		let scope = Scope::default()
			.with(Import::Members(
				String::from("foo"),
				vec![String::from("Hello")],
			))
			.with(
				Function::default()
					.with_visibility(Visibility::Public)
					.with_name("add")
					.with_return_type(types::Number)
					.with_param(Parameter(Box::new("a"), Some(Box::new(types::Number))))
					.with_param(Parameter(Box::new("b"), Some(Box::new(types::Number))))
					.with_scope(
						Scope::default().with(statement::Return(Some(operation::Add("a", "b")))),
					),
			);

		assert_eq!(
			scope.to_code(Language::Rust),
			"pub fn add(a:i32,b:i32)->i32{return a+b;}"
		);

		assert_eq!(
			scope.to_code(Language::TypeScript),
			"export function add(a:number,b:number):number{return a+b;}"
		);
	}
}
