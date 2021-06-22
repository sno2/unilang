use itertools::Itertools;

use crate::{Language, ToCode};

#[derive(Debug)]
pub enum Import {
	/// A file path.
	Module(String),
	Members(String, Vec<String>),
}

impl ToCode for Import {
	fn to_code(&self, language: Language) -> String {
		match self {
			Self::Module(location) => match language {
				Language::TypeScript => format!("import \"{}\";", location),
				Language::Rust => format!("mod {};", location),
				Language::CPP | Language::Python { .. } => unimplemented!(),
			},
			Self::Members(location, members) => match language {
				Language::TypeScript => {
					format!(
						"import {{{}}} from \"{}\";",
						members.iter().join(","),
						location
					)
				}
				Language::Rust => {
					format!("use {}::{{{}}};", location, members.iter().join(","))
				}
				Language::CPP | Language::Python { .. } => unimplemented!(),
			},
		}
	}
}
