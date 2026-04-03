use std::fs::read_dir;
use std::path::Path;
use convenient_lib::Res;

const BASE_PATH: &str = "models";

pub fn get_model_names() -> Res<Vec<String>> {
    Ok(
        read_dir(Path::new(BASE_PATH))?
            .map(|entry| {
                entry
                    .expect("couldn't get an entry ?")
                    .path()
                    .file_name()
                    .expect("couldn't get a file/folder name?")
                    .to_str()
                    .unwrap()
                    .into()
            })
            .collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_model_names() {
        assert_eq!(get_model_names().unwrap(), vec![
            String::from("pouet"),
            String::from("pouet_v2"),
            String::from("pouet_v3"),
            String::from("saucisse"),
            String::from("saucisse_v2"),
            String::from("saucisse_v3"),
            String::from("saucisse_v4")
        ]);
    }
}
