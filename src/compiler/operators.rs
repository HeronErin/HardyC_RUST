

// A.1.1.6 Operators

use crate::genStrType;

genStrType!(Operators, OPERATORS,
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

    SizeOf => "sizeof";

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
