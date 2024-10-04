#![allow(unused)]

mod compiler;
use std::collections::HashMap;

use compiler::parser::line_parser::build_recursive;


const SAMPLE : &str = "
struct Foo{
    char x;
    int y;
    short z;
    union{
        char* x2;
        int* y2;
        char nil;
    }
};

typedef Dtyp char;

static unsigned char** doShit(void** input, struct Foo bar, Dtyp d){
    return (unsigned char**)0;
}
int main(){
    char x = ***&*&doShit(((void**) 0, struct Foo{
        x : 1,
        y : 2,
        z : 4,
        nil : 0
    }, 4) * 2);

    return x;
}

";
fn main() {
    
    println!("{:?}", build_recursive(SAMPLE))
    // println!("{:?}", ts);
    // println!("{:?}", test_tokens_against(FUNCTION_DECLARATION, &ts));
    
}
