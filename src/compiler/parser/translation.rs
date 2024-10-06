
// For more info; 2.1.1.2 Translation phases

// Phases:
// 1. Trigraphs
// 2. Non-logical newline striping (I.e "\\\n" -> "")



// Turns "\\\n" -> ""
pub fn non_logical_newline_striping<'a>(input: Vec<(usize, &'a [u8])>) -> Vec<(usize, &'a [u8])>{
    let mut result = Vec::with_capacity(input.capacity());
    
    let mut itr = input.iter();
    let (mut last_file_index, mut bytes) = match itr.next() {
        Some(x) => x,
        None => return result
    };

    let mut is_after_backslash = false;
    let mut i = 0;
    loop {
        if i >= bytes.len(){
            result.push((last_file_index, bytes));
            i = 0;

            (last_file_index, bytes) = match itr.next() {
                Some(x) => *x,
                None => break
            };
        }
        let char = bytes[i];
        let is_backslash = char == b'\\';
        let is_nl = char == b'\n';

        if is_backslash && is_after_backslash{ is_after_backslash = false;}
        else if is_backslash{is_after_backslash = true;}
        else if is_nl && is_after_backslash{
            if i == 0{
                // Safety: We dispose of the last value quickly so being mutable is not a problem
                // Safety: is_after_backslash can only be set if something precedes is
                let last = unsafe{ result.last_mut().unwrap_unchecked() };
                last.1 = &last.1[..last.1.len()-1];
                bytes = &bytes[1..];
                last_file_index += 1;
            }else{
               let before = &bytes[..i - 1];
               bytes = &bytes[i+1..];
               
               result.push((last_file_index, before));

               last_file_index += i+1;

               is_after_backslash = false;
            }
            i = 0;
            continue;
        } else if is_after_backslash{
            is_after_backslash = false;
        }
        
        i += 1;
    }



    return result;
}

// Turns "/*   */" -> " "
pub fn star_comment_striper<'a>(input: Vec<(usize, &'a [u8])>) -> Vec<(usize, &'a [u8])>{
    let mut result = Vec::with_capacity(input.capacity());
    let mut itr = input.iter();
    let (mut last_file_index, mut bytes) = match itr.next() {
        Some(x) => x,
        None => return result
    };

    let mut is_within_comment = false;
    let mut i = 0;
    loop{
        if i+1 >= bytes.len(){
            if !is_within_comment{
                result.push((last_file_index, bytes));
            }
            i = 0;

            (last_file_index, bytes) = match itr.next() {
                Some(x) => *x,
                None => break
            };
        }
        if (!is_within_comment && bytes[i] == b'/' && bytes[i+1] == b'*'){
            is_within_comment = true;
            
        }
        


        i+=1;
    }


    return result;
}

// Not that fast, but good for tests
fn back_to_string(pieces :  Vec<(usize, &[u8])>) -> String{
    pieces.iter().map(|x| String::from_utf8_lossy(x.1)).collect::<Vec<_>>().concat()
}











#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_escape() {
        let input = vec![(0, b"Hello\nWorld\n".as_ref())];
        let expected = "Hello\nWorld\n";
        let result = back_to_string(non_logical_newline_striping(input));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_simple_escape() {
        let input = vec![(0, b"Hello\\\nWorld\n".as_ref())];
        let expected = "HelloWorld\n";
        let result = back_to_string(non_logical_newline_striping(input));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_multiple_escapes() {
        let input = vec![(0, b"Line1\\\nLine2\\\nLine3\n".as_ref())];
        let expected = "Line1Line2Line3\n";
        let result = back_to_string(non_logical_newline_striping(input));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_leading_escape() {
        let input = vec![(0, b"\\\nLine1\n".as_ref())];
        let expected = "Line1\n";
        let result = back_to_string(non_logical_newline_striping(input));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_trailing_escape() {
        let input = vec![(0, b"Line1\\\n".as_ref())];
        let expected = "Line1";
        let result = back_to_string(non_logical_newline_striping(input));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_consecutive_escapes() {
        let input = vec![(0, b"Line1\\\\\nLine2\n".as_ref())];
        let expected = "Line1\\\\\nLine2\n";
        let result = back_to_string(non_logical_newline_striping(input));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_empty_input() {
        let input: Vec<(usize, &[u8])> = vec![];
        let expected = "";
        let result = back_to_string(non_logical_newline_striping(input));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_no_newlines() {
        let input = vec![(0, b"HelloWorld".as_ref())];
        let expected = "HelloWorld";
        let result = back_to_string(non_logical_newline_striping(input));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_only_escaped_newlines() {
        let input = vec![(0, b"\\\n\\\n".as_ref())];
        let expected = "";
        let result = back_to_string(non_logical_newline_striping(input));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_mixed_input() {
        let input = vec![(0, b"Line1\\\nLine2\nLine3\\\nLine4\n".as_ref())];
        let expected = "Line1Line2\nLine3Line4\n";
        let result = back_to_string(non_logical_newline_striping(input));
        assert_eq!(expected, result);
    }
}
