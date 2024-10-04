#![allow(unused)]

mod compiler;
use std::collections::HashMap;

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
    
    
    // println!("{:?}", ts);
    // println!("{:?}", test_tokens_against(FUNCTION_DECLARATION, &ts));
    
}
