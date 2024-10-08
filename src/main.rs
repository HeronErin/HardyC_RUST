#![allow(unused)]

mod compiler;
use std::{collections::HashMap, hint};

use compiler::parser::{tokenizer::tokenize, translation::{non_logical_newline_striping, strip_single_line_style_comments, strip_star_style_comments}, trigraph::trigraph_convert};

const SAMPLE : &str = "


#define CONST_INT 5
#define ADD(XXX, YYY) ((XXX) + (YYY))
#define SUB(XXX, YYY) ((XXX) - (YYY)) // bar
#define ADD_SUB(XXXX, YYYY)  ADD(XXXX) - SUB(YYYY) /* foo */ 


#define CONCAT(a, b) a##b


#if CONST_INT == 1
int foo(){
#else
void bar(){
#endif
    int xy = 100;
    int x = ADD_SUB(1, -1) + CONST_INT - CONCAT(x, y);
}

int main()<%
    printf(\"foo\");
%>


";
fn main() {
    let logical = compiler::parser::translation::initial_translation_phases(SAMPLE);
    println!("{:?}", tokenize(&logical).unwrap());
    // println!("{:?}", p2.iter().map(|x| String::from_utf8_lossy(x.1)).collect::<Vec<_>>().concat());
    // println!("{:?}", t);
    // println!("{:?}", test_tokens_against(FUNCTION_DECLARATION, &ts));

    // SAMPLE.to_string()
    
}
