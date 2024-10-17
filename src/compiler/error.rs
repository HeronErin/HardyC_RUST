use std::io;


#[derive(Debug)]
pub enum ErrorVariety{
    FileNotFoundError,
    IsDirectoryError,

    BracketCountError,
    IoError(io::Error),
    
    // Macro errors
    RecursiveMacroError,
    MacroMissingCorresponding
}

#[derive(Debug)]
pub struct CompilerError{
    pub error_variety : ErrorVariety,
    pub info : String,
}