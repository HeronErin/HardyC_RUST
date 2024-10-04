// 2.2.1.1 Trigraph sequences
// All occurrences in a source file of the following sequences of
// three characters (called trigraph sequences /5/)are replaced with the
// corresponding single character.

//          ??=      #
//          ??(      [
//          ??/      \
//          ??)      ]
//          ??'      ^
//          ??<      {
//          ??!      |
//          ??>      }
//          ??-      ~

use core::str;
use std::{slice::Windows, str::Utf8Chunk};

use num_traits::ops::bytes;

// Fast AF trigraph converter
pub fn trigraph_convert<'a>(input: &'a str) -> Vec<&'a str> {
    let bytes = input.as_bytes();

    // Theoretically the max size for vec. 
    let mut res = Vec::with_capacity(bytes.len() / 3);

    let mut i = 0;
    let mut oldest_non_trigraph = 0;
    let len = bytes.len();

    while i <= len - 3 {
        let window = &bytes[i..i + 3];

        if window[0] != b'?' || window[1] != b'?' {
            i += 1;
            continue;
        }


        let replacement = match window[2] {
            b'=' => "#",
            b'(' => "[",
            b'/' => "\\",
            b')' => "]",
            b'\'' => "^",
            b'<' => "{",
            b'!' => "|",
            b'>' => "}",
            b'-' => "~",
            b'?' => "?",
            _ => {
                i += 1;
                continue;
            }
        };

        // Push non-trigraph part
        if oldest_non_trigraph != i {
            res.push(unsafe { std::str::from_utf8_unchecked(&bytes[oldest_non_trigraph..i]) });
        }

        // Push trigraph replacement
        res.push(replacement);

        // Skip the 3 characters of the trigraph
        i += 3;
        oldest_non_trigraph = i;
    }

    // Add any remaining non-trigraph text
    if oldest_non_trigraph < len {
        res.push(unsafe { std::str::from_utf8_unchecked(&bytes[oldest_non_trigraph..len]) });
    }

    res
}


#[cfg(test)]
mod tests{
    use super::*;
    fn simple(){
        assert_eq!(trigraph_convert("//          ??=      #
    //          ??(      [
    //          ??/      \\
    //          ??)      ]
    //          ??'      ^
    //          ??<      {
    //          ??!      |
    //          ??>      }
    //          ??-      ~").concat(), "//          #      #
    //          [      [
    //          \\      \\
    //          ]      ]
    //          ^      ^
    //          {      {
    //          |      |
    //          }      }
    //          ~      ~")
    }
}
