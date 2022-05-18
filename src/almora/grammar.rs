use crate::{define_grammar, range, word, choice};

define_grammar!(almora, |grammar: &mut GrammarBuilder<R>| {
    // Basic tokens
    let plus  = word!("+");
    let minus  = word!("-");
    let times  = word!("*");
    let divide = word!("/");
    let modulo = word!("%");

    let digit = range!('0', '9');
    let alpha = choice!(range!('a', 'z'), range!('A', 'Z'));
    let alpha_num = choice!(alpha, digit);
    let integer = digit.at_least(1);

    let operator = choice!(plus, minus, times, divide, modulo);

    let term = choice!(integer, operator);

    let expression = term.at_least(1);

    // Save the root rule.
    expression
});
