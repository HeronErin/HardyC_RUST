use crate::compiler::{error::CompilerError, keywords::Keyword, operators::{Bracket, Operator}};

use super::string_patch_resolver::PatchString;

#[inline]
pub fn test_number_suffix(s : char) -> bool{
    matches!(
        s,
        'l' | 'L' |
        'u' | 'U' |
        'f' | 'F'
    )
}


// Tests if something **could** be a number, ands is length
pub fn test_number_extent(input : &str) -> usize{
    let mut chrs = input.char_indices();

    let mut is_zero_prefixed = false;
    while let Some((i, char)) = chrs.next(){
        // Preceding sign
        if i == 0 && char == '0' { is_zero_prefixed = true; };
        if i == 1 && is_zero_prefixed && ( char == 'x'  || char == 'b') { continue };
        
        // Assume all bases valid (also include the e in 1e1 and suffixes)
        match char {
            '0'..='9' 
                | 'a'..='f'  // Hex lowercase
                | 'A'..='F'  // Hex uppercase
                | 'e' | 'E' // Exponents
                | '.' => continue,

            // Assume all suffixes valid
            'l' | 'L' | 'u' | 'U' | 'f' | 'F' => continue,
            _ => {}
        }
        
        return i;
    }
    return input.len();
}






#[derive(Debug, Clone)]
pub enum TokenData<'a>{
    UNKNOWN(char),


    Keyword(Keyword),
    TextCluster(&'a str),
    NumberLiteral(&'a str),
    StringLiteral(&'a str),

    SepStyleWS, // Separation style white space: Whitespace without newlines
    NLstyleWs, // Newline style whitespace: Whitespace containing newlines

    Operator(Operator),
    
    OpenBracket(Bracket),
    CloseBracket(Bracket),

    Semicolon,
    Colon
}
#[derive(Debug, Clone)]
pub struct Token<'a>{
    pub data : TokenData<'a>,
    
    // Location to 
    pub noncanonical_start : usize,
    pub noncanonical_end : usize,

    // Index into where the file exists
    // in TranslationUnit.files
    pub origin : usize
}



fn consume_ws<'a>(inputc : &'a str) -> Option<(usize, bool)>{
    let first = inputc.chars().next()?;
    if !first.is_ascii_whitespace(){ return None };

    let mut is_newline_style = false;

    
    for (i, c) in inputc.char_indices(){
        if c == '\n' || c == '\r'{ is_newline_style = true; }
        else if !c.is_ascii_whitespace(){
            return Some((
                i, 
                is_newline_style
            ));
        }
    }
    Some((
        inputc.len(), 
        is_newline_style
    ))
}
// Assume we already have found a cluster, this gets that cluster and the following string
fn consume_char_cluster<'a>(input : &'a str) -> (&'a str,  &'a str){
    for (i, c) in input.char_indices(){
        if c != '_' && !c.is_alphanumeric(){
            return (&input[..i], &input[i..]);
        }
    }
    (input, &input[input.len()..])
}
// Assume the literal is itself valid. No validity checking
fn consume_string_literal<'a>(input : &'a str) -> (&'a str,  &'a str){
    let mut itr =  input.char_indices();
    let first = itr.next().unwrap().1;
    
    let mut last_was_escape = false;
    for (i, c) in itr{
        if c == '\\' && !last_was_escape{
            last_was_escape = true;
        }
        else if c == first && !last_was_escape{
            return (&input[..i+1], &input[i+1..]);
        }else{
            last_was_escape = false;
        }
    }
    (input, &input[input.len()..])
}



pub fn tokenize<'a>(input_str : &'a str, origin : usize) -> Result<Vec<Token<'a>>, CompilerError>{
    let mut inputc = input_str;
    let mut ret = Vec::new();

    macro_rules! getIndex {
        () => {
            inputc.as_ptr() as usize - input_str.as_ptr() as usize
        };
    }
    
    loop {
        if let Some((count, is_newline)) = consume_ws(&inputc){
            let start = getIndex!();
            ret.push(Token{
                data: if is_newline {TokenData::NLstyleWs} else {TokenData::SepStyleWS},
                noncanonical_start: start,
                noncanonical_end: start + count,
                origin
            });
            inputc = &inputc[count..];
        }
        let chr = inputc.chars().next();
        
        if let None = chr { break };
        let chr = unsafe { chr.unwrap_unchecked() };
        
        let i = getIndex!();

        match chr {
            ';'  => ret.push(Token { data: TokenData::Semicolon, noncanonical_start: i, noncanonical_end: i+1, origin }),
            ','  => ret.push(Token { data: TokenData::Colon, noncanonical_start: i, noncanonical_end: i+1, origin }),
            '0'..='9' => {
                let extent = test_number_extent(&inputc);
                if extent != 0{
                    ret.push(
                        Token {
                            data: TokenData::NumberLiteral(&inputc[..extent]),
                            noncanonical_start: i,
                            noncanonical_end: i + extent, 
                            origin
                        }
                    );
                    inputc = &inputc[extent..];
                    continue;
                }
            },
            '\'' | '\"' => {
                let (literal, new_inputc) = consume_string_literal(&inputc);
                inputc = new_inputc;
                let end = getIndex!();

                ret.push(Token { data: TokenData::StringLiteral(literal), noncanonical_start: i, noncanonical_end: end, origin });
                continue;
            },
            '_' | 'a'..='z' | 'A'..='Z' => {
                let (cluster, new_inputc) = consume_char_cluster(&inputc);
                inputc = new_inputc;
                ret.push(
                    Token { data: TokenData::TextCluster(cluster), noncanonical_start: i, noncanonical_end: i + cluster.len(), origin}
                );
                continue;
            },
            // Taken from is_ascii_punctuation
            '!'..='/' | ':'..='@' | '['..='`' | '{'..='~'  => {
                if let Some((size, is_open, bracket)) = Bracket::try_from(&inputc){
                    ret.push(
                        Token {
                            data: if is_open { TokenData::OpenBracket(bracket) } else  { TokenData::CloseBracket(bracket) },
                            noncanonical_start: i,
                            noncanonical_end: i + size,
                            origin
                        }
                        
                    );
                    inputc = &inputc[size..];
                    continue;
                }
                if let Some((size, operator)) = Operator::try_from_string(inputc){
                    ret.push(Token { data: TokenData::Operator(operator), noncanonical_start: i, noncanonical_end: i + size, origin });
                    inputc = &inputc[size..];
                    continue;
                }
            },
            _ => {ret.push(Token { data: TokenData::UNKNOWN(chr), noncanonical_start: i, noncanonical_end: i+1, origin }) }   
        }
        
        inputc = &inputc[1..];
    }
    
    
    Ok(ret)
}

