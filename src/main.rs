#![allow(unused)]

mod compiler;
use std::fs::File;
use std::path::Path;
use std::{collections::HashMap, hint};

use compiler::parser::tokenizer::tokenize;
use compiler::parser::translation::{initial_translation_phases, strip_star_style_comments};
use compiler::parser::{translation::trigraph_convert};

const SAMPLE : &str = "


??=define CONST_INT 5\\
            +\\
            1

??=define ADD(XXX, YYY) ((XXX) + (YYY))
??=define SUB(XXX, YYY) ((XXX) - (YYY)) // bar
??=define ADD_SUB(XXXX, YYYY)  ADD(XXXX) - SUB(YYYY) /* foo */ 


??=define CONCAT(a, b) a??=??=b


??=if CONST_INT == 1
int foo(){
??=else
void bar(){
??=endif
    int xy = 100;
    int x = ADD_SUB(1, -1) + CONST_INT - CONCAT(x, y);
}

int main()<%
    printf(\"foo\");
%>";
// const x = [1, 2, 3];
use compiler::parser::string_patch_resolver::{PatchString, RebuildAction::*};
use compiler::state::TranslationUnit;
fn main() {
    let mut unit = TranslationUnit{ path: vec![Path::new(".")], files: Vec::new() };
    
    let og = "#include \"examples/main.h\"";
    let mut ps = PatchString::new(og.to_string());
    
    initial_translation_phases(&mut ps);
    
    unit.files.push(("main.c".to_string(), og.to_string(), ps));
    let r = unit.files.last().unwrap();


    let toks = tokenize(&r.2.get_str(), unit.files.len()-1).unwrap();
    
    
}
