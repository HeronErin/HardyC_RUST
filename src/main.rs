#![allow(unused)]

mod compiler;
use std::{collections::HashMap, hint};

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
use compiler::parser::string_patch_resolver::RebuildAction::*;
fn main() {
    let og = SAMPLE;
    
    let ps = initial_translation_phases(og);
    println!("{}", ps.get_str());
    
}
