use super::adapter::Parser;
use crate::utils::{dir_to_string, file_to_string};
use anyhow::Result;
use rush_core::blueprint::Blueprint;
use std::{fs::metadata, path::Path};

pub struct Loader {
    parser: Box<dyn Parser>,
}

impl Loader {
    pub fn new(parser: impl Parser) -> Self {
        Self {
            parser: Box::new(parser),
        }
    }

    /// Load Blueprint
    ///
    /// Loads [`Blueprint`] from a specific [`Path`]
    ///
    /// [`Path`] can be a **file** or **directory**;
    pub fn load_blueprint(&self, path: &Path) -> Result<Blueprint> {
        // expecting a valid path
        let md = metadata(path).expect("invalid path");

        // get blueprint string from file or directory
        let blueprint_string = match md.is_dir() {
            true => dir_to_string(path),
            false => file_to_string(path),
        };

        self.parser.parse_string(blueprint_string)
    }
}
