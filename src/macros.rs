#[macro_export]
macro_rules! new_comparator {
	($name:ident, $operator:expr) => {
		#[derive(Debug)]
		pub struct $name<T: ToCode, F: ToCode>(pub T, pub F);

		impl<T: ToCode + std::fmt::Debug, F: ToCode + std::fmt::Debug> ToCode for $name<T, F> {
			fn to_code(&self, language: Language) -> String {
				let Self(a, b) = self;
				format!(
					"({}{}{})",
					a.to_code(language),
					$operator,
					b.to_code(language)
				)
			}
		}
	};
}
