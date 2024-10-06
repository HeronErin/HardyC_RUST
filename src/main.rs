#![allow(unused)]

mod compiler;
use std::{collections::HashMap, hint};

use compiler::parser::{translation::{non_logical_newline_striping, strip_single_line_style_comments, strip_star_style_comments}, trigraph::trigraph_convert};

const SAMPLE : &str = "


#define CONST_INT 5
#define ADD(XXX, YYY) ((XXX) + (YYY))
#define SUB(XXX, YYY) ((XXX) - (YYY))
#define ADD_SUB(XXXX, YYYY)  ADD(XXXX) - SUB(YYYY)


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
    
    let p1 = trigraph_convert("Hello //??/a");
    let p2 = non_logical_newline_striping(&p1);
    let p3 = strip_star_style_comments(&p2);
    let p4 = strip_single_line_style_comments(&p3);
    dbg!(p1);
    dbg!(p2);
    dbg!(p3);
    dbg!(p4);

    let x : f64 = "+2e2".parse().unwrap();
    dbg!(x);
    // println!("{:?}", p2.iter().map(|x| String::from_utf8_lossy(x.1)).collect::<Vec<_>>().concat());
    // println!("{:?}", t);
    // println!("{:?}", test_tokens_against(FUNCTION_DECLARATION, &ts));

    // SAMPLE.to_string()
    
}
