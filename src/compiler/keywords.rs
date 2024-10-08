use crate::genStrType;


// See '3.1.1 Keywords' in ansi.c.txt
genStrType!(Keyword, KEYWORDS,
    AUTO => "auto";
    DOUBLE => "double";
    INT => "int";
    STRUCT => "struct";
    BREAK => "break";
    ELSE => "else";
    LONG => "long";
    SWITCH => "switch";
    CASE => "case";
    ENUM => "enum";
    REGISTER => "register";
    TYPEDEF => "typedef";
    CHAR => "char";
    EXTERN => "extern";
    RETURN => "return";
    UNION => "union";
    CONST => "const";
    FLOAT => "float";
    SHORT => "short";
    UNSIGNED => "unsigned";
    CONTINUE => "continue";
    FOR => "for";
    SIGNED => "signed";
    VOID => "void";
    DEFAULT => "default";
    GOTO => "goto";
    SIZEOF => "sizeof";
    VOLATILE => "volatile";
    DO => "do";
    IF => "if";
    STATIC => "static";
    WHILE => "while"
);


// impl Keyword{
//     #[inline]
//     pub fn is_primitive_type_keyword(&self) -> bool{
//         matches!(self, Self::INT | Self::DOUBLE | Self::AUTO | Self::FLOAT | Self::SHORT | Self::CHAR)
//     }
// }