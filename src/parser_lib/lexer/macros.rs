// #[macro_export]
// macro_rules! define_keywords {
//     ($($word:expr => $name: ident), *) => {
//         enum TokenType {
//             define_tokens_inner!($($word => $name), *);
//         }
//     };
// }
// macro_rules! define_tokens_inner {

//     ($word:expr => $name: ident, $($rest:tt)*) => {
//         name,
//         define_tokens!($($rest)*);
//     };

// }

// define_keywords! {
//     // Keywords (reserved, can't be used for identifier)
//     If => "if",
// }

// fn test() {
//     TokenType::
// }