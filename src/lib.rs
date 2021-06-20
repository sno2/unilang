mod models;

pub use models::*;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let func = Function {
			name: String::from("subtract"),
			visibility: Visibility::Public,
			return_type: Some(String::from("i32")),
			params: vec![
				Parameter {
					name: String::from("a"),
					typ: Some(String::from("i32")),
				},
				Parameter {
					name: String::from("b"),
					typ: Some(String::from("i32")),
				},
			],
			content: vec![Box::new(statement::VariableInit {
				name: String::from("difference"),
				mutable: Some(false),
				typ: None,
				value: expression::operation::Add("a", "b"),
			})],
		};

		std::fs::write("./asdf.rs", func.to_code(Language::Rust)).unwrap();
	}
}
