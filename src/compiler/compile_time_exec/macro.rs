// Expand out and process compiler directives

use std::{collections::HashMap, fs, path::PathBuf};

use crate::compiler::{
    error::{CompilerError, ErrorVariety},
    operators::Operator,
    parser::{
        string_patch_resolver::PatchString,
        tokenizer::{skip_ws, tokenize, Token, TokenData},
        translation::{self, initial_translation_phases},
    },
    state::TranslationUnit,
};

pub struct FunctionMacro<'a> {
    arguments: Vec<&'a str>,

    // Just the token data, no need for location info
    replacement: Vec<TokenData<'a>>,
}
pub enum Macro<'a> {
    FunctionStyle(FunctionMacro<'a>),
    VariableStyle(Vec<TokenData<'a>>),
}

fn token_gen<'a>(
    origin_file: &PathBuf,
    determined_name: &str,
    name_origin: &[Token<'_>],
    unit: &'a mut TranslationUnit,
) -> Result<Vec<Token<'a>>, CompilerError> {
    let closest = unit
        .resolve_to_closest(determined_name, origin_file.parent())
        .ok_or_else(|| {
            CompilerError::from_tokens(
                unit,
                name_origin,
                ErrorVariety::FileNotFoundError,
                "Failed to resolve import: \"".to_owned() + determined_name + "\"",
            )
        })?;
    let s = fs::read_to_string(closest.clone()).map_err(|e| CompilerError {
        error_variety: ErrorVariety::IoError(e),
        info: "Is error".to_owned(),
    })?;
    let ps = initial_translation_phases(&s);
    unit.files.push((closest, s, ps));
    let info_ref = unsafe { unit.files.last().unwrap_unchecked() };
    return tokenize(&info_ref.2.get_str(), unit.files.len() - 1);
}

// Assume we are **immediately** after include, simply skip ws find <> and "" and extract the true contents
// option is set to None if include either requires expansion or is invalid
fn resolve_include_name<'a>(
    unit: &TranslationUnit,
    mod_file_cont: &'a str,
    toks: &[Token<'a>],
) -> Result<Option<&'a str>, CompilerError> {
    let (first_tok, new_toks) = skip_ws(toks)
        .ok_or_else(|| CompilerError::from_tokens(unit, toks, ErrorVariety::UnexpectedEof, "Can't find first token of #include".to_owned()))?;
    Ok(match first_tok {
        Token { data : TokenData::StringLiteral(str), .. } => Some(&str[1..str.len()-1]),
        // This is not as simple...
        Token { data : TokenData::Operator(Operator::LesserThan), noncanonical_start : start, ..} => {
            let after_start = &mod_file_cont[*start+1..];
            let end_ind = after_start
                .char_indices()
                .filter(|c| c.1 == '>')
                .map(|c|c.0)
                .next()
                .ok_or_else(|| CompilerError::from_tokens(unit, new_toks, ErrorVariety::UnexpectedEof, "Unable to find end of include string".to_owned()))?;
            Some(&after_start[..end_ind])
        }
        _ => None
    })
}



macro_rules! clone_mut_ref {
    ($x : expr) => {
        unsafe{
            let ptr = $x as *mut _;
            std::mem::transmute(ptr)
        }
        
        
    };
}

pub fn macro_evaluation(unit: &mut TranslationUnit) -> Result<(), CompilerError> {
    assert!(!unit.is_initialized);
    unit.is_initialized = true;

    let seed = unit.files.last().expect("Unit was not seeded!");
    let (mut file, mut og_cont, mut mut_cont) = (&seed.0, &seed.1, &seed.2);

    let og_token_vec = tokenize(&mut_cont.get_str(), 0)?;
    let mut current_tokens = og_token_vec.as_slice();

    let mut file_stack: Vec<(&PathBuf, &String, &PatchString, &[Token<'_>])> = Vec::new();
    let mut just_started_file = true;


    fn expand(){

    }

    loop {
        // Handle file transitions
        if current_tokens.len() == 0 {
            if let Some((_file, _og_cont, _mut_cont, _tok)) = file_stack.pop() {
                file = &*_file;
                og_cont = &*_og_cont;
                mut_cont = &*_mut_cont;
                current_tokens = &*_tok;
            } else {
                break;
            }
        }

        let is_procedure = match (&current_tokens[0], current_tokens.get(1)) {
            // At start of file
            (
                Token { data: TokenData::Operator(Operator::Pound), .. },
                _,
            ) if just_started_file => {
                current_tokens = &current_tokens[1..];
                true
            }
            // In midst of file
            (
                Token { data: TokenData::NLstyleWs, .. },
                Some(Token { data: TokenData::Operator(Operator::Pound), .. }),
            ) => {
                current_tokens = &current_tokens[2..];
                true
            }
            // Something else that we don't care about
            _ => false,
        };
        if is_procedure {
            let eol = current_tokens
                .into_iter()
                .enumerate()
                .filter(|x| {
                    matches!(&x.1, Token { data: TokenData::NLstyleWs, .. } )
                })
                .map(|x| x.0)
                .next()
                .unwrap_or(current_tokens.len());

            let mut current_line = &current_tokens[..eol];

            let proc = match current_line.get(0) {
                Some(Token {
                    data: TokenData::TextCluster(text),
                    ..
                }) => *text,
                Some(_) => Err(CompilerError::from_tokens(
                    unit,
                    &current_line[..1],
                    ErrorVariety::InvalidPreprocessorDirective,
                    "Unexpected preprocessor procedure type".to_owned(),
                ))?,
                None => Err(CompilerError {
                    error_variety: ErrorVariety::UnexpectedEof,
                    info: "Stray '#' at end of file".to_owned(),
                })?,
            };
            current_line = &current_line[1..];
            match proc {
                "include" => {
                    // TODO: HANDLE EXPAND REQUIRING INCLUDES!
                    let include_str = resolve_include_name(&*unit, &mut_cont.get_str(), &current_line)?
                        .expect("Unsupported include type"); 
                    token_gen(
                        file,
                        include_str,
                        current_line,
                        clone_mut_ref!(unit)
                    );
                    
                }

                _ => Err(CompilerError::from_tokens(
                    unit,
                    &current_line,
                    ErrorVariety::InvalidPreprocessorDirective,
                    "Unknown preprocessor procedure: ".to_owned() + proc,
                ))?,
            }

            just_started_file = false;
            continue;
        }
        // TODO: ACTUAL EXPANSION

        just_started_file = false;
    }

    todo!()
}
