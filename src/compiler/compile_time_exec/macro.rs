// Expand out and process compiler directives

use std::{collections::HashMap, path::PathBuf};

use crate::compiler::{
    error::{CompilerError, ErrorVariety},
    operators::Operator,
    parser::{
        string_patch_resolver::PatchString,
        tokenizer::{tokenize, Token, TokenData},
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

struct MacroExpandState<'a> {
    unit: &'a mut TranslationUnit<'a>,
    expanded_tokens: Vec<Token<'a>>,
    pre_token_stash : Vec<Token<'a>>,
    macro_map: HashMap<&'a str, Macro<'a>>,

    was_handled: bool,
}

use crate::compiler::error::ErrorVariety::*;
impl<'a> MacroExpandState<'a> {
    fn init(unit: &'a mut TranslationUnit<'a>) -> Result<Self, CompilerError> {
        assert!(
            !unit.is_initialized,
            "Refusal to initialize expand initialized TranslationUnit"
        );
        assert!(
            unit.files.len() == 1,
            "TranslationUnit has not been properly seeded"
        );
        unit.is_initialized = true;

        let (path, original_text, translated_pstr, translated_str) =
            unsafe { unit.files.last().unwrap_unchecked() };
        let translated_str = translated_str.as_str();

        // Capacity note: token_basis.capacity() != token_basis.len()
        // Therefore this is a large overestimate, but thats alright

        Ok(Self {
            unit,
            expanded_tokens: Vec::new(),
            macro_map: HashMap::new(),
            was_handled: false,
            pre_token_stash: Vec::new(),
        })
    }
    fn handle_include<'b>(&mut self, toks: &'b [Token<'a>]) -> Result<Vec<&'b Token<'a>>, CompilerError> {
        todo!()
    }
    fn  handle_directive<'b>(&mut self, toks: &'b [Token<'a>]) -> Result<(&'b [Token<'a>], Vec<Token<'a>>), CompilerError> {
            let directive = match toks.get(0){
                // EOF
                None => Err(CompilerError{ error_variety: ErrorVariety::UnexpectedEof, info: "Stray '#' found in code during macro expansion followed by an EOF".to_owned() })?,
                // Good path
                Some(Token { data : TokenData::TextCluster(directive), noncanonical_start: _, noncanonical_end : _, origin : _}) => directive,
                // Invalid directive type
                _ => Err(CompilerError::from_tokens(self.unit, &toks[0..1], InvalidPreprocessorDirective, "Please use text for preprocessor directives (or you left a stray '#' in your code)".to_owned()))?
            };
            let eol = toks
                .into_iter()
                .enumerate()
                .filter(|tok| matches!(tok.1.data, TokenData::NLstyleWs))
                .map(|tok| tok.0)
                .next()
                .unwrap_or(toks.len());
            let line = &toks[1..eol];
            println!("{}", directive);
            // todo!();
            let l = match *directive {
                "include" => self.handle_include(line)?,

                _ => Err(CompilerError::from_tokens(self.unit, &toks[0..1], InvalidPreprocessorDirective, "Not a recognized preprocessor directive: \"".to_owned() + directive + "\""))?
            };

            Ok((
                &toks[eol..],
                todo!()
            ))
        }

    fn handle(&mut self) -> Result<(), CompilerError> {
        debug_assert!(!self.was_handled);
        self.was_handled = true;
        
        let (mut path, mut og, mut ps, mut translated) = (
            self.unit.files[0].0.clone(),
            self.unit.files[0].1.as_str(),
            &self.unit.files[0].2,
            self.unit.files[0].3.as_str(),
        );
        let mut file_stack = Vec::new();
        
        let initial_tokens_vec = tokenize(translated, 0)?;
        let mut current_tokens = initial_tokens_vec.as_slice();
       
        

        
        loop {
            if current_tokens.len() == 0 {
                if let Some((_path, _og, _ps, tok)) = file_stack.pop() {
                    path = _path;
                    og = _og;
                    ps = _ps;
                    current_tokens = tok;
                } else {
                    break;
                }
            }
            // Test for compiler directive (Works with start of files due to implicit new lines added in tokenizer)
            if matches!(
                (&current_tokens[0].data, &current_tokens[1].data),
                (TokenData::NLstyleWs, TokenData::Operator(Operator::Pound),)
            ) {
                current_tokens = &current_tokens[2..];
                self.handle_directive(&current_tokens)?;
            } else {
                current_tokens = &current_tokens[1..];
            }
        }

        todo!()
    }
}

pub fn macro_evaluation<'a>(mut unit: &'a mut TranslationUnit<'a>) -> Result<(), CompilerError> {
    let mut x = MacroExpandState::init(unit)?;
    x.handle()?;

    todo!()
}
