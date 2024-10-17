// Expand out and process compiler directives

use crate::compiler::{parser::tokenizer::TokenData, state::TranslationUnit};

pub struct FunctionMacro<'a>{
    arguments : Vec<&'a str>,

    // Just the token data, no need for location info
    replacement : Vec<TokenData<'a>>
}
pub enum Macro<'a>{
    FunctionStyle(FunctionMacro<'a>),
    VariableStyle(Vec<TokenData<'a>>)
}



