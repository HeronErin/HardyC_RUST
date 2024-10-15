#![allow(unused)]

mod compiler;
use std::{collections::HashMap, hint};

use compiler::parser::{string_patch_resolver::PatchString, tokenizer::tokenize, translation::{non_logical_newline_striping, strip_single_line_style_comments, strip_star_style_comments}, trigraph::trigraph_convert};

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
// const x = [1, 2, 3];
use compiler::parser::string_patch_resolver::RebuildAction::*;
fn main() {
    let og = "Boo Far Faz!";
    let mut ps = PatchString::new(String::from(og));
    
    ps.rebuild_string_windowed(|window : [char; 2]|{
        if window[0] == 'F'{
            return DiscardAndInsert(1, "B");
        }
        if window[0] == 'B'{
            return DiscardAndInsert(1, "F");
        }
        if window == ['z', '!']{
            return DiscardAndInsert(1, "sh");
        }

        return Keep
    });
 


    dbg!(ps.get_str());
    println!("from_mod_index:");


    let old: Vec<char> = og.chars().collect();
    for (i, chr) in ps.get_str().char_indices(){
        println!("{} {} - {} @ {}", chr, old[ps.from_mod_index(i)], ps.from_mod_index(i), i);
    }

    println!("____________________\nto_mod_index:");
    let start : Vec<char> = ps.get_str().chars().collect();
    for (i, chr) in og.char_indices(){
        println!("{} {} - {} @ {}", chr, start[ps.to_mod_index(i)], ps.to_mod_index(i), i);
    }

    
}
