
pub enum ErrorVariety{
    BracketCountError
}
pub struct CompilerError{
    error_variety : ErrorVariety,
    info : String,
}