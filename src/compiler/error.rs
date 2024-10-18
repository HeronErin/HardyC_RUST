use std::io;

use super::{parser::{gen_line_map, tokenizer::Token}, state::TranslationUnit};


#[derive(Debug)]
pub enum ErrorVariety{
    FileNotFoundError,
    IsDirectoryError,

    BracketCountError,
    IoError(io::Error),

    UnexpectedEof,
    
    // Macro errors
    InvalidPreprocessorDirective,
    RecursiveMacroError,
    MacroMissingCorresponding
}

#[derive(Debug)]
pub struct CompilerError{
    pub error_variety : ErrorVariety,
    pub info : String,
}


impl CompilerError {
    pub fn from_tokens(unit : &TranslationUnit, token : &[Token<'_>], error_variety : ErrorVariety, info : String) -> Self{
        if token.len() == 0{
            return CompilerError{
                error_variety,
                info,
            };
        }
        let mut real_info = "An error has occurred in ".to_owned();
        let mut _o = usize::MAX;
        for o in token.into_iter().map(|o| o.origin){
            if o == _o { continue; }
            if _o != usize::MAX { real_info += ", "; }

            _o = o;

            real_info = real_info + "\"" + &unit.files[o].0.display().to_string() + "\"";
        }
        
        
        let fid_1 = token.first().unwrap().origin;
        let fid_2 = token.last().unwrap().origin;
        let is_multi_file = fid_1 != fid_2;

        if  is_multi_file{real_info += " It spans many files!"; }
        
        let file_1 = &unit.files[fid_1];
        let file_2 = &unit.files[fid_2];
    
        let line_map_1 = gen_line_map(&file_1.1);
        let line_map_2 = gen_line_map(&file_2.1);
        let maybe_start = file_1.2.from_mod_index(token.first().unwrap().noncanonical_start);
        let maybe_end = file_2.2.from_mod_index(token.last().unwrap().noncanonical_end);

        let line1 = line_map_1.into_iter().enumerate().filter(|x| x.1 <= maybe_start).last();
        let line2 = line_map_2.into_iter().enumerate().filter(|x| x.1 <= maybe_start).last();
        


        real_info = real_info + "It starts on line: ";
        if let Some(line1) = line1 {
            real_info = real_info + &(1+line1.0).to_string() + ", chr: " + &(maybe_start - line1.1).to_string();
        }else{
            real_info = real_info + "Unknown";
        }

        real_info = real_info + " and end on line: ";

        if let Some(line2) = line2 {
            real_info = real_info + &(1+line2.0).to_string() + ", chr: " + &(maybe_end - line2.1).to_string();
        }else{
            real_info = real_info + "Unknown";
        }

        CompilerError{
            error_variety,
            info: info + "\n" + &real_info,
        }
    }
}