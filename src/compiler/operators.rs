

// A.1.1.6 Operators

use crate::genStrType;

#[derive(Debug, Clone, PartialEq)]
pub enum Bracket{
    Parenthesis, // ()
    CurlyBracket, // {}  ALSO digraphs: <% %>
    SquareBracket // [] ALSO digraphs: <: :>
}

impl Bracket {
    #[inline]
    pub fn try_from(i : &str) -> Option<(usize, bool, Bracket)>{
        let mut chrs = i.chars();
        let first = chrs.next()?;
        match first {
            '(' | ')' => return Some((1, first == '(', Bracket::Parenthesis)),
            '{' | '}' => return Some((1, first == '{', Bracket::CurlyBracket)),
            '[' | ']' => return Some((1, first == '[', Bracket::SquareBracket)),
            _ => {}
        };
        let second = chrs.next()?;
        match (first, second) {
            ('<','%') => Some((2, true, Bracket::CurlyBracket)),
            ('%','>') => Some((2, false, Bracket::CurlyBracket)),
            ('<',':') => Some((2, true, Bracket::SquareBracket)),
            (':','>') => Some((2, false, Bracket::SquareBracket)),
            _ => None
        }        
    }
}


genStrType!(Operator, OPERATORS,
    Period => ".";
    Arrow => "->";
    Increment => "++";
    Decrement => "--";

    AddEq => "+=";
    Add => "+";

    SubEq => "-=";
    Sub => "-";

    AndEq => "&=";
    LogicalAnd => "&&";
    Ampersand => "&"; // Might be ref OR bitwise and

    OrEq => "|=";
    LogicalOr => "||";
    BitwiseOr => "|";

    Asterisk => "*"; // Might be multiplication OR deref
    MultEq => "*=";

    BitwiseComplement => "~";
    LogicalNegation => "!";
    NotEq => "!=";

    BitwiseXorEq => "^=";
    BitwiseXor => "^";

    DivideEq => "/=";
    Divide => "/";

    ModEq => "%=";
    Mod => "%";

    LeftShiftEq => "<<=";
    RightShiftEq => ">>=";

    LeftShift => "<<";
    RightShift => ">>";

    LesserThanOrEq => "<=";
    LesserThan => "<";
    GreaterThanOrEq => ">=";
    GreaterThan => ">";

    QuestionMark => "?";
    Colon => ":";
    Comma => ",";
    LogicalEquals => "==";
    Assignment => "=";

    DoublePound => "##", "%:%:";
    Pound => "#", "%:";
);
