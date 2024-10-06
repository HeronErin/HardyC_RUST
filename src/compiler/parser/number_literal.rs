// See "3.1.3.1 Floating constants", and "3.1.3.2 Integer constants"

use std::mem::transmute;



pub struct NumberLiteral<'a>{
    sign : bool, // true = negative
    base : NumberBase,
    suffix : NumberSuffix,

    number_portion : &'a str
}

// Simply counts potential number length, no validity testing
fn get_number_extent(input : &str) -> usize{
    let mut chrs = input.char_indices();
    while let Some((i, char)) = chrs.next(){
        // Preceding sign
        if i == 0 && (char == '-' || char == '+') { continue };
        
        // Assume all bases valid (also include the e in 1e1 and suffixes)
        if char.is_alphanumeric() { continue };
        
        if char == '.' {continue};
        
        return i;
    }
    return input.len();
}

impl<'a> NumberLiteral<'a>{
    pub fn try_from(input : &'a str) -> Option<NumberLiteral<'a>>{
        let extent = get_number_extent(&input);
        
        


        
        



        todo!()
    }
}








// Its nice to have the radix available
pub enum NumberBase{
    Octal = 8,
    Decimal = 10,
    Hex = 16
}
pub enum NumberSuffix{
    Unsigned,
    Long,
    Float
}