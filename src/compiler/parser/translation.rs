
// For more info; 2.1.1.2 Translation phases

// Phases:
// 1. Trigraphs (see: trigraph.rs)
// 2. Non-logical newline striping (I.e "\\\n" -> "")
// 3. Comments

use super::string_patch_resolver::PatchString;

use super::string_patch_resolver::RebuildAction;
use super::string_patch_resolver::RebuildAction::*;


const TRIGRAPH_MAP_FUNCTION : &dyn Fn([char; 3]) -> RebuildAction = &(|window : [char; 3]|{
    match window {
        ['?', '?', '='] => DiscardAndInsert(3, "#"),
        ['?', '?', '('] => DiscardAndInsert(3, "["),
        ['?', '?', '/'] => DiscardAndInsert(3, "\\"),
        ['?', '?', ')'] => DiscardAndInsert(3, "]"),
        ['?', '?', '\''] => DiscardAndInsert(3, "^"),
        ['?', '?', '<'] => DiscardAndInsert(3, "{"),
        ['?', '?', '!'] => DiscardAndInsert(3, "|"),
        ['?', '?', '>'] => DiscardAndInsert(3, "}"),
        ['?', '?', '-'] => DiscardAndInsert(3, "~"),
        _ => Keep
    }
});


// Replaces all trigraphs with their canonical characters
pub fn trigraph_convert(input:  &mut PatchString){
    input.rebuild_string_windowed(TRIGRAPH_MAP_FUNCTION);
}
// Ditto
pub fn trigraph_convert_str(input:  &str) -> PatchString{
    PatchString::construct_from(input, TRIGRAPH_MAP_FUNCTION)
}

// Turns "\\\n" -> ""
pub fn non_logical_newline_striping(input: &mut PatchString){
    let mut was_previous_escape = false;
    input.rebuild_string_windowed( move |window : [char; 2]|{
        if window == ['\\', '\n'] && !was_previous_escape{
            return DiscardAmount(2)    
        }
        if window[0] == '\\'{
            was_previous_escape = true;
        }else{
            was_previous_escape = false;
        }
        Keep
    });
}

fn string_heuristic(curr : char, was_backslash : &mut bool, is_in_string : &mut bool){
    if curr == '\\'{
        *was_backslash = !*was_backslash;
        return;
    }
    if curr == '\"' && !*is_in_string{
        *is_in_string = true;
        return;
    }
    if curr == '\"' && *is_in_string && !*was_backslash{
        *is_in_string = false;
    }

}
pub fn strip_star_style_comments(input : &mut PatchString){
    let mut is_in_comment = false;
    let mut is_in_string = false;
    let mut was_backslash = false;
    input.rebuild_string_windowed(move |window : [char; 2]|{
        string_heuristic(window[0], &mut was_backslash, &mut is_in_string);
        if is_in_string && !is_in_comment{
            return Keep;
        }
        
        if is_in_comment{
            if window == ['*', '/']{
                is_in_comment = false;
                DiscardAmount(2)
            }else{ DiscardAmount(1) }
        }
        else if window == ['/', '*']{
            is_in_comment = true;
            DiscardAndInsert(2, " ")
        }else{ Keep }
    
    });
}



// Not technically in the ansi.c spec, but I want it!
pub fn strip_single_line_style_comments(input : &mut PatchString){
    let mut is_in_comment = false;
    let mut is_in_string = false;
    let mut was_backslash: bool = false;
    input.rebuild_string_windowed(move |window : [char; 2]|{
        string_heuristic(window[0], &mut was_backslash, &mut is_in_string);
        if is_in_string && !is_in_comment{
            return Keep;
        }

        if is_in_comment && window[1] == '\n'{
            is_in_comment = false;
            return DiscardAmount(1);
        }
        if window == ['/', '/']{
            is_in_comment = true;
        }
        if is_in_comment{
            return DiscardAmount(1);
        }

        Keep
    });
}



#[inline]
pub fn apply_initial_translation_phases(inputc : &mut PatchString){
    trigraph_convert(&mut *inputc);
    non_logical_newline_striping(&mut *inputc);
    strip_star_style_comments(&mut *inputc);
    strip_single_line_style_comments(&mut *inputc);
}

#[inline]
pub fn initial_translation_phases(inputc : &str) -> PatchString{
    let mut ps = trigraph_convert_str(inputc);
    non_logical_newline_striping(&mut ps);
    strip_star_style_comments(&mut ps);
    strip_single_line_style_comments(&mut ps);
    ps
}

