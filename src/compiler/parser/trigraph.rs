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
use std::{mem::transmute, slice::Windows, str::Utf8Chunk};

use num_traits::ops::bytes;


pub fn trigraph_convert(input: &str) -> String {
    // Most people don't use trigraphs, so soo much space is needed
    let mut res = String::with_capacity(input.len());

    let len = input.len();

    let mut u8chars = input.chars();

    let next_three = (u8chars.next(), u8chars.next(), u8chars.next());

    if next_three.2.is_none() {
        if let Some(c) = next_three.0 {res.push(c);}
        if let Some(c) = next_three.1 {res.push(c);}
        return res;
    };

    let mut next_three = (
        next_three.0.unwrap(),
        next_three.1.unwrap(),
        next_three.2.unwrap(),
    );
    loop {
        match next_three {
            ('?', '?', '=') => res.push('#'),
            ('?', '?', '(') => res.push('['),
            ('?', '?', '/') => res.push('\\'),
            ('?', '?', ')') => res.push(']'),
            ('?', '?', '\'') => res.push('^'),
            ('?', '?', '<') => res.push('{'),
            ('?', '?', '!') => res.push('|'),
            ('?', '?', '>') => res.push('}'),
            ('?', '?', '-') => res.push('~'),
            _ => {
                res.push(next_three.0);
                next_three.0 = next_three.1;
                next_three.1 = next_three.2;
                next_three.2 = match (u8chars.next()) {
                    Some(c) => c,
                    None => {
                        res.push(next_three.0);
                        res.push(next_three.1);
                        break;
                    }
                };
                continue;
            }
        }
        let next_three_opt = (u8chars.next(), u8chars.next(), u8chars.next());
        if (next_three_opt.2.is_some()) {
            next_three = (
                next_three_opt.0.unwrap(),
                next_three_opt.1.unwrap(),
                next_three_opt.2.unwrap(),
            );
            continue;
        }
        if let Some(c) = next_three_opt.0{ res.push(c) };
        if let Some(c) = next_three_opt.1{ res.push(c) };
        break;

    }

    res
}
