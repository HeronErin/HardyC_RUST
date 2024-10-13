// Expand out and process compiler directives

use crate::compiler::parser::tokenizer::Token;

pub struct MacroDefinition<'a>{
    name : &'a str,
    arguments : Option<Vec<&'a str>>,
    replacement : Vec<Token<'a>>
}





