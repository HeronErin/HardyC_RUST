macro_rules! genKeyword {
    ($($element:ident, $text: literal), *) => {
        const KEYWORDS : &[&'static str] = &[$($text, )*];

        #[derive(Debug, Clone, PartialEq)]
        pub enum Keyword{
            $(
                $element,
            )*
        }
        impl Keyword{
            pub fn try_from_string(x : &str) ->Option<Self>{
                if x.len() <= 1 { return None }

                $(
                    // This _should_ be evaluated at compile time
                    if Self::$element != Self::DUMMY{
                        if x.starts_with($text){
                            return Some(Self::$element)
                        }
                    }
                )*
                None
            }
        }
    };
}

genKeyword!(
    DUMMY, "INVALID_KEYWORD",
    If, "if", 
    ElseIf, "else if",
    Else, "else",
    While, "while",
    Do, "do",
    For, "for"
);