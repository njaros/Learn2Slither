use convenient_lib::Res;
use qlearning::Model;
use std::fs;
use std::fs::read_dir;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

pub fn get_model_names(base_path: &str) -> Res<Vec<String>> {
    Ok(read_dir(Path::new(base_path))?
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
        .collect())
}

pub fn get_model_bests(path: &PathBuf) -> Res<Vec<String>> {
    Ok(read_dir(path)?
        .map(|entry| {
            entry
                .expect("couldn't get an entry ?")
                .path()
                .file_prefix()
                .expect("couldn't get a file/folder name?")
                .to_str()
                .unwrap()
                .into()
        })
        .collect())
}

pub fn get_model(path: &mut PathBuf) -> Res<Model> {
    path.add_extension("json");
    let file = fs::File::open(path)?;
    let mut contents = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut contents)?;

    Ok(serde_json::from_str::<Model>(&contents)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_model_names() {
        assert_eq!(
            get_model_names("../models").unwrap(),
            vec![
                String::from("pouet"),
                String::from("pouet_v2"),
                String::from("pouet_v3"),
                String::from("saucisse"),
                String::from("saucisse_v2"),
                String::from("saucisse_v3"),
                String::from("saucisse_v4")
            ]
        );
    }

    #[test]
    fn test_get_model_bests() {
        assert_eq!(
            get_model_bests(&Path::new("../models").join("pouet")).unwrap(),
            vec![
                String::from("0"),
                String::from("1"),
                String::from("2"),
                String::from("3"),
                String::from("4"),
                String::from("5"),
                String::from("6"),
                String::from("7"),
                String::from("8"),
                String::from("9"),
            ]
        )
    }
}
