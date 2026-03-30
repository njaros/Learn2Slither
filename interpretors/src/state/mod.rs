pub mod ets_lib;

pub struct ETSFactory {}

impl ETSFactory {
	pub fn create(ets_name: String) -> Result<Box<dyn ets_lib::ETS>, String> {
		match ets_name.to_lowercase().as_str() {
			"dummy" => Ok(Box::new(ets_lib::Dummy {})),
			"jaja_v1" => Ok(Box::new(ets_lib::JajaV1 {})),
			_ => Err(format!("{ets_name} not implemented. choose one of those: [{}]", list_all_ets()))
		}
	}
}

pub fn list_all_ets() -> String {
	vec![
		"jaja_v1",
		"dummy"
	]
	.join(", ")
}