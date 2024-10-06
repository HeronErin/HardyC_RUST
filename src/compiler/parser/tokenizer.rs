use crate::compiler::{keywords::Keyword, operators::Bracket};

#[derive(Debug, Clone)]
pub enum Token<'a>{
    Keyword(Keyword),
    TextCluster(&'a str),

    SepStyleWS, // Separation style white space: Whitespace without newlines
    NLstyleWs, // Newline style whitespace: Whitespace containing newlines


    
    OpenBracket(Bracket),
    CloseBracket(Bracket),

    Semicolon,
    Colon
}
