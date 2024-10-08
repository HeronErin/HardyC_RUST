
#[derive(Debug)]
pub enum ErrorVariety{
    BracketCountError
}

#[derive(Debug)]
pub struct CompilerError{
    error_variety : ErrorVariety,
    info : String,
}