pub mod parser;
pub mod keywords;
pub mod operators;


pub mod error;
pub mod compile_time_exec;


pub mod state;




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
      
            pub fn try_from_string(x: &str) -> Option<(usize, Self)> {
                if x.len() <= 1 { return None; }

                $(
                    $(
                        if x.len() >=  $text.len(){
                            match &x[0..$text.len()]{
                                $text => return Some(($text.len(), Self::$element)),
                                _ => {}
                            }
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
