use errors::UtilError;
use regex::Regex;
use std::fs;
use std::path::Path;

/// Collect paths to all files of a type `.model` from a folder
pub fn get_model_paths(dir: &str) -> Result<Vec<String>, UtilError> {
    let model_path = Path::new(dir);
    let mut model_paths = Vec::new();
    if model_path.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                // just check the paths on current folder level
                continue;
            } else {
                let path_str = String::from(
                    path.to_str()
                        // Option to Result type
                        .ok_or(UtilError::new("Can't convert path to string."))?,
                );
                // only build regex once
                lazy_static! {
                    static ref IS_MODEL: Regex =
                        Regex::new(r".*\.model").expect("Can't initialise regex.");
                }
                if IS_MODEL.is_match(&path_str[..]) {
                    model_paths.push(String::from(
                        path.to_str()
                            // Option to Result type
                            .ok_or(UtilError::new("Can't convert path to string."))?,
                    ));
                }
            }
        }
    };
    Ok(model_paths)
}

/// Sort list of tuples by second element
pub fn sort_by_second_element<T: PartialOrd>(
    mut vec: Vec<(String, T)>,
) -> Result<Vec<(String, T)>, UtilError> {
    vec.sort_by(|tuple1, tuple2| { tuple1.1.partial_cmp(&tuple2.1).unwrap() }.reverse());
    Ok(vec)
}
