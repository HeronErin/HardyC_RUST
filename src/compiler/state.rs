use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use num_traits::real;

use super::{error::CompilerError, parser::string_patch_resolver::PatchString};

pub struct TranslationUnit {
    // List of lookup path priority for #include
    pub path: Vec<PathBuf>,

    //            (path,  og contents,  mut contents)
    pub files: Vec<(PathBuf, String, PatchString)>,
    // TODO: CONSTANTS
    // TODO: FUNCTIONS
    // TODO: SYMBOLS
}

use crate::compiler::{
    error::ErrorVariety::*,
    parser::translation::{initial_translation_phases, trigraph_convert_str},
};
impl TranslationUnit {
    pub fn new() -> Self {
        Self {
            path: Vec::new(),
            files: Vec::new(),
        }
    }
    fn seed_from_file(seed: &str) -> Result<Self, CompilerError> {
        let p = Path::new(seed);
        if !p.exists() {
            Err(CompilerError {
                error_variety: FileNotFoundError,
                info: "Can't open seed file: \"".to_owned() + seed + "\"",
            })?;
        }
        if p.is_dir() {
            Err(CompilerError {
                error_variety: IsDirectoryError,
                info: "Can't open directory as text file: \"".to_owned() + seed + "\"",
            })?;
        }
        let real_path = p.canonicalize().map_err(|e| CompilerError {
            error_variety: IoError(e),
            info: "Unspecified io error when resolving path: \"".to_owned() + seed + "\"",
        })?;
        // All the ways that parent() can return None have been checked above
        let parent = unsafe { real_path.parent().unwrap_unchecked() };

        let new_self = Self {
            path: Vec::with_capacity(1),
            files: Vec::with_capacity(1),
        };
        let f = std::fs::File::open("a");
        let og_string = read_to_string(real_path.clone()).map_err(|e| CompilerError {
            error_variety: IoError(e),
            info: "Unable to read due to io error on initial seed: \"".to_owned()
                + real_path.as_path().to_str().unwrap()
                + "\"",
        })?;
        let translated = initial_translation_phases(&og_string);

        Ok(Self {
            path: vec![parent.to_path_buf()],
            files: vec![(real_path, og_string, translated)],
        })
    }
}
