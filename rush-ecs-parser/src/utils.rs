use rush_ecs_core::blueprint::BlueprintString;
use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};

/// Load Blueprint from file
///
/// Loads blueprint from a file directly. Panics when
/// path is invalid
///
/// Returns a [`BlueprintString`] of the entire blueprint
pub fn file_to_string(path: &Path) -> BlueprintString {
    // expecting valid path
    read_to_string(path).expect("invalid path")
}

/// Load Blueprint from Directory
///
/// Loads blueprint from combination of files in
/// a given directory. Panics when path is invalid.
///
/// Returns a [`BlueprintString`] of the entire combined
/// blueprint separated with an empty line (\n)
///
/// Example:
///
/// file1_contents
/// -> Empty line
/// file2_contents
///
pub fn dir_to_string(path: &Path) -> BlueprintString {
    let list_of_files = read_dir(path).expect("invalid path");

    // holds the entire blueprint string
    let mut loaded_string = String::default();

    // loop over files in given directory
    for de in list_of_files {
        // expecting valid path
        let dir_entry = de.expect("invalid directory entry");
        let filepath = dir_entry.path();
        let content = read_to_string(filepath).expect("invalid path");

        // combine
        loaded_string += format!("{content}\n").as_str();
    }

    loaded_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_to_string() {
        let path = Path::new("mock/fixtures/utils/file_to_string");
        let string = file_to_string(path);
        assert_eq!(string, "abcd\n");
    }

    #[test]
    fn test_dir_to_string() {
        let path = Path::new("mock/fixtures/utils/dir_to_string");
        let string = dir_to_string(path);
        assert_eq!(string, "a\n\nb\n\n");
    }
}
