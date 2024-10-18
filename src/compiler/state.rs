use std::{
    fs::read_to_string,
    path::{Path, PathBuf}, rc::Rc,
};

use num_traits::real;

use super::{error::CompilerError, parser::{string_patch_resolver::PatchString, tokenizer::Token}};

pub struct TranslationUnit<'a> {
    // List of lookup path priority for #include
    pub path: Vec<PathBuf>,


    // Must be Rc or rust freaks the fuck out
    //            (path,      og contents, mutated contents(scalped ps), mutated contents (string)    )
    pub files: Vec<(Rc<PathBuf>, String, PatchString, String)>,

    pub is_initialized : bool,

    pub tokens : Vec<Token<'a>>
    // TODO: CONSTANTS
    // TODO: FUNCTIONS
    // TODO: SYMBOLS
}

use crate::compiler::{
    error::ErrorVariety::*,
    parser::translation::{initial_translation_phases, trigraph_convert_str},
};
impl<'a> TranslationUnit<'a> {
    pub fn new() -> Self {
        Self {
            path: Vec::new(),
            files: Vec::new(),
            is_initialized: false,
            tokens: Vec::new(),
        }
    }
    // JUST INITS THE OBJECT, NOTHING ELSE
    pub fn seed_from_file(seed: &str) -> Result<Self, CompilerError> {
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

        let f = std::fs::File::open("a");
        let og_string = read_to_string(real_path.clone()).map_err(|e| CompilerError {
            error_variety: IoError(e),
            info: "Unable to read due to io error on initial seed: \"".to_owned()
                + real_path.as_path().to_str().unwrap()
                + "\"",
        })?;
        let mut translated = initial_translation_phases(&og_string);
        let translated_string = translated.scalp();
        Ok(Self {
            path: vec![parent.to_path_buf()],
            files: vec![(real_path.into(), og_string.into(), translated.into(), translated_string.into())],
            is_initialized: false,
            tokens: vec![],
        })
    }
}
