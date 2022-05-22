// Define helper macros to reduce the amount of boilerplate needed to define rules

#[allow(unused)]
use crate::parser_lib::Rule;

/// Matches a sequence of rules
#[macro_export(rule_macros)]
macro_rules! seq {
    ($($rule:expr),*) => {
        Rule::seq(vec![$(&$rule),*])
    };
}

/// Chooses between several rules
#[macro_export]
macro_rules! choice {
    ($($rule:expr),*) => {
        Rule::choice(vec![$(&$rule),*])
    };
}

/// Makes the rule optional
#[macro_export]
macro_rules! opt {
    ($rule:expr) => {
        $rule.optional()
    };
}

/// Matches a char within a range
#[macro_export]
macro_rules! range {
    ($start:expr, $end:expr) => {
        Rule::range($start, $end)
    };
}

/// Matches an exact word
#[macro_export]
macro_rules! word {
    ($word:expr) => {
        Rule::word($word)
    };
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::StringCharReader;

    use super::*;

    #[test]
    fn test_seq() {
        let x: Rule<StringCharReader> = word!("X");
        let y = word!("Y");
        let val = seq![x, y];
        assert_eq!(val.to_string(), "(\"X\" \"Y\")");
    }

    #[test]
    fn test_choice() {
        let x: Rule<StringCharReader> = word!("X");
        let y = word!("Y");
        let val = choice![x, y];
        assert_eq!(val.to_string(), "(\"X\" | \"Y\")");
    }

    #[test]
    fn test_opt() {
        let x: Rule<StringCharReader> = word!("X");
        let val = opt!(x);
        assert_eq!(val.to_string(), "\"X\"?");
    }

    #[test]
    fn test_range() {
        let val: Rule<StringCharReader> = range!('a', 'z');
        assert_eq!(val.to_string(), "[a-z]");
    }

    #[test]
    fn test_word() {
        let val: Rule<StringCharReader> = word!("X");
        assert_eq!(val.to_string(), "\"X\"");
    }

    #[test]
    fn test_repetition() {
        let x: Rule<StringCharReader> = word!("X").at_least(2);
        assert_eq!(x.to_string(), "\"X\"{2,...}");
    }
}
