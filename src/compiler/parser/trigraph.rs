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
// This requires little 
pub fn trigraph_convert<'a>(input: &'a [u8]) -> Vec<(usize, &'a [u8])> {
    // Most people don't use trigraphs, so reserving is not necessary
    let mut res = Vec::new();

    let mut i = 0;
    let mut oldest_non_trigraph = 0;
    let len = input.len();

    while i+3 <= len {
        let window = &input[i..i + 3];

        if window[0] != b'?' || window[1] != b'?' {
            i += 1;
            continue;
        }

        let replacement = match window[2] {
            b'=' => b"#",
            b'(' => b"[",
            b'/' => b"\\",
            b')' => b"]",
            b'\'' =>b"^",
            b'<' => b"{",
            b'!' => b"|",
            b'>' => b"}",
            b'-' => b"~",
            _ => {
                i += 1;
                continue;
            }
        };

        // Push non-trigraph part
        if oldest_non_trigraph != i {
            res.push((oldest_non_trigraph, (&input[oldest_non_trigraph..i])));
        }

        // Push trigraph replacement
        res.push((i, replacement));

        // Skip the 3 characters of the trigraph
        i += 3;
        oldest_non_trigraph = i;
    }

    // Add any remaining non-trigraph text
    if oldest_non_trigraph < len {
        res.push((oldest_non_trigraph, &input[oldest_non_trigraph..len]));
    }

    res
}


#[cfg(test)]
mod tests{
    use super::*;
    fn simple(){
        assert_eq!(trigraph_convert(b"//          ??=      #
    //          ??(      [
    //          ??/      \\
    //          ??)      ]
    //          ??'      ^
    //          ??<      {
    //          ??!      |
    //          ??>      }
    //          ??-      ~").iter().map(|t| t.1).collect::<Vec<_>>().concat(), b"//          #      #
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
