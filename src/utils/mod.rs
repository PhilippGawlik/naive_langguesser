use regex::Regex;
use std::fs;
use std::path::Path;
use utils::errors::UtilError;

pub mod errors;

pub fn get_probability(nominator: f32, denominator: f32) -> Result<f32, UtilError> {
    if denominator > 0.0 {
        Ok(nominator / (denominator as f32))
    } else {
        Err(UtilError::new(
            "get_probability: Division by zero or negative number!",
        ))
    }
}

pub fn get_model_paths(dir: &Path) -> Result<Vec<String>, UtilError> {
    let mut model_paths = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                // just check the paths on current folder level
                continue;
            } else {
                let path_str = String::from(
                    path.to_str()
                        // Option to Result type
                        .ok_or(UtilError::new(
                            "get_model_paths: Can't convert path to string.",
                        ))?,
                );
                // only build regex once
                lazy_static! {
                    static ref IS_MODEL: Regex =
                        Regex::new(r".*\.model").expect("get_model_path: Can't initialise regex.");
                }
                if IS_MODEL.is_match(&path_str[..]) {
                    model_paths.push(String::from(
                        path.to_str()
                            // Option to Result type
                            .ok_or(UtilError::new(
                                "get_model_paths: Can't convert path to string.",
                            ))?,
                    ));
                }
            }
        }
    };
    Ok(model_paths)
}

pub fn sort_by_second_element<T: PartialOrd>(
    mut vec: Vec<(String, T)>,
) -> Result<Vec<(String, T)>, UtilError> {
    vec.sort_by(|tuple1, tuple2| { tuple1.1.partial_cmp(&tuple2.1).unwrap() }.reverse());
    Ok(vec)
}
