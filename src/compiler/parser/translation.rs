
// For more info; 2.1.1.2 Translation phases

// Phases:
// 1. Trigraphs (see: trigraph.rs)
// 2. Non-logical newline striping (I.e "\\\n" -> "")
// 3. Comments



// Turns "\\\n" -> ""
pub fn non_logical_newline_striping(input: &str) -> String{
    let mut res = String::with_capacity(input.len());


    let mut utf8chars = input.chars();

    let next_two = (utf8chars.next(), utf8chars.next());
    
    
    if next_two.1.is_none() {
        if let Some(c) = next_two.0 {res.push(c);}
        return res;
    }
    let mut next_two = (next_two.0.unwrap(), next_two.1.unwrap());
    loop{
        if ('\\', '\n') == next_two{
            let next_two_opt = (utf8chars.next(), utf8chars.next());
            if next_two_opt.1.is_none() {
                if let Some(c) = next_two_opt.0 {res.push(c);}
                return res;
            }
            next_two = (next_two_opt.0.unwrap(), next_two_opt.1.unwrap());
            continue;
        }
        res.push(next_two.0);
        next_two.0 = next_two.1;
        next_two.1 = match utf8chars.next() {
            Some(c) => c,
            None => {res.push(next_two.0); break;},
        }
        
    }


    res
}


pub fn strip_star_style_comments(input : &str) -> String{
    let mut res = String::with_capacity(input.len());


    let mut utf8chars = input.chars();
    let next_two = (utf8chars.next(), utf8chars.next());
    
    if next_two.1.is_none() {
        if let Some(c) = next_two.0 {res.push(c);}
        return res;
    }

    let mut is_in_comment = false;
    let mut next_two = (next_two.0.unwrap(), next_two.1.unwrap());

   
    macro_rules! move_forward_from_match {
        () => {
            let next_two_opt = (utf8chars.next(), utf8chars.next());
            if next_two_opt.1.is_none() {
                if let Some(c) = next_two_opt.0 {if !is_in_comment { res.push(c); }}
                return res;
            }
            next_two = (next_two_opt.0.unwrap(), next_two_opt.1.unwrap());
            continue;

        };
    }
    loop{
       
        if !is_in_comment && ('/', '*') == next_two{
            is_in_comment = true;
            res.push(' '); // "Each comment is replaced by one space character" ok...
            move_forward_from_match!();
        }
        // "21. Thus comments do not nest." saves me the work
        else if is_in_comment && ('*', '/') == next_two{
            is_in_comment = false;
            move_forward_from_match!();
        }


        if !is_in_comment{ res.push(next_two.0); }
        next_two.0 = next_two.1;
        next_two.1 = match utf8chars.next() {
            Some(c) => c,
            None => {if !is_in_comment {res.push(next_two.0);} break;},
        }
        
    }


    res
}



// Not technically in the ansi.c spec, but i want it!
pub fn strip_single_line_style_comments(input : &str) -> String{
    let mut res = String::with_capacity(input.len());


    let mut utf8chars = input.chars();
    let next_two = (utf8chars.next(), utf8chars.next());
    
    if next_two.1.is_none() {
        if let Some(c) = next_two.0 {res.push(c);}
        return res;
    }

    let mut is_in_comment = false;
    let mut next_two = (next_two.0.unwrap(), next_two.1.unwrap());
   
    loop{
        if !is_in_comment && next_two == ('/', '/'){
            is_in_comment = true;
            // Don't think a space is needed as a newline is inserted soon
            // (assuming this comment is not at the end of a file)
            let next_two_opt = (utf8chars.next(), utf8chars.next());
            if next_two_opt.1.is_none() { return res; }
            next_two = (next_two_opt.0.unwrap(), next_two_opt.1.unwrap());
            continue;
        }   
        if next_two.0 == '\n'{
            is_in_comment = false;
        }

        if !is_in_comment{ res.push(next_two.0); }
        next_two.0 = next_two.1;
        next_two.1 = match utf8chars.next() {
            Some(c) => c,
            None => {if !is_in_comment {res.push(next_two.0);} break;},
        }
        
    }


    res
}




pub fn initial_translation_phases(inputc : &str) -> String{
    let p1 = super::trigraph::trigraph_convert(inputc);
    let p2 = non_logical_newline_striping(&p1);
    let p3 = strip_star_style_comments(&p2);
    strip_single_line_style_comments(&p3)
}

