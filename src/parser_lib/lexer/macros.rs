#[macro_export]
macro_rules! define_tokens {
    ($($word:expr => $name: ident), *) => {
        enum TokenType {
            define_tokens_inner!($($word => $name), *);
        }
    };
}
macro_rules! define_tokens_inner {

    ($word:expr => $name: ident, $($rest:tt)*) => {
        name,
        define_tokens!($($rest)*);
    };

}

define_tokens! {
    // Keywords
    "if" => If,
    "else" => Else,
    "while" => While
}

fn test() {
    TokenType::While
}