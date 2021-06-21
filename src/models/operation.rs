use crate::{Language, ToCode};

macro_rules! new_operation {
	($name:ident, $operator:expr) => {
		#[derive(Debug)]
		pub struct $name<T: ToCode, F: ToCode>(pub T, pub F);

		impl<T: ToCode + std::fmt::Display, F: ToCode + std::fmt::Display> ToCode for $name<T, F> {
			fn to_code(&self, language: Language) -> String {
				let Self(a, b) = self;
				format!(
					"{}{}{}",
					a.to_code(language),
					$operator,
					b.to_code(language)
				)
			}
		}
	};
}

new_operation!(Add, "+");
new_operation!(Subtract, "-");
new_operation!(Multiply, "*");
new_operation!(Divide, "/");
new_operation!(Modulus, "%");
