pub mod parser;
pub mod keywords;
pub mod operators;


pub mod error;


#[macro_export]
macro_rules! genStrType {
    ($name:ident, $arr_name:ident, $($element:ident => $($text:literal),+);* $(;)?) => {
        const $arr_name: &[&'static str] = &[
            $(
                $(
                    $text,
                )+
            )*
        ];

        #[derive(Debug, Clone, PartialEq)]
        pub enum $name {
            INVALID,
            $(
                $element,
            )*
        }

        impl $name {
            pub fn try_from_string(x: &str) -> Option<Self> {
                if x.len() <= 1 { return None; }

                $(
                    $(
                        if x.starts_with($text) {
                            return Some(Self::$element);
                        }
                    )+
                )*

                None
            }

            pub fn to_string(&self) -> &'static str {
                match self {
                    $(
                        $(
                            Self::$element => $text,
                        )+
                    )*
                    _ => unreachable!(),
                }
            }
        }
    };
}
