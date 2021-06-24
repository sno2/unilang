#[macro_use]
pub(crate) mod macros;
mod models;

pub(crate) use models::utils;
pub use models::*;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic_function() {
		let scope = Scope::default().with(
			Function::default()
				.with_visibility(Visibility::Public)
				.with_name("add")
				.with_return_type(types::Integer)
				.with_param(Parameter(Box::new("a"), Some(Box::new(types::Integer))))
				.with_param(Parameter(Box::new("b"), Some(Box::new(types::Integer))))
				.with_scope(
					Scope::default().with(statement::Return(Some(operation::Add("a", "b")))),
				),
		);

		assert_eq!(
			scope.to_code(Language::Rust),
			"pub fn add(a:i32,b:i32)->i32{return (a+b);}"
		);

		assert_eq!(
			scope.to_code(Language::TypeScript),
			"export function add(a:number,b:number):number{return (a+b);}"
		);

		assert_eq!(
			scope.to_code(Language::CPP),
			"int add(int a,int b){return (a+b);}"
		);

		assert_eq!(
			scope.to_code(Language::Python {
				include_types: true,
				indent_level: None,
				indent_type: IndentType::Tab
			}),
			"\ndef add(a:int,b:int)->int:\n\treturn (a+b)"
		);
	}
}
