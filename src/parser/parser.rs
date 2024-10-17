use super::grammar::{Grammar, Rule};
use crate::error::ValueError;
use crate::parser::builder::build_formula_with_pratt;
use crate::value::Value;
use crate::{Formula, NoCustomFunction};
use pest::Parser;

pub fn parse_string(s: &str) -> Option<pest::iterators::Pair<Rule>> {
    let parse_result = Grammar::parse(Rule::formula, s);
    match parse_result {
        Ok(mut result) => {
            let parse_result = result.next().unwrap();
            Some(parse_result)
        }
        Err(_) => None,
    }
}

fn parse_string_constant(parse_result: pest::iterators::Pair<Rule>) -> Formula {
    let string = parse_result
        .into_inner()
        .as_str()
        .parse::<String>()
        .unwrap();
    Formula::Value(Value::Text(string.trim_start_matches('\'').to_string()))
}

pub fn parse_formula(s: &str, f: Option<&impl Fn(&str, Vec<f64>) -> Value>) -> Formula {
    match parse_string(&s) {
        Some(parse_result) => match parse_result.as_rule() {
            Rule::expr => build_formula_with_pratt(parse_result.into_inner(), f),
            Rule::string_constant => parse_string_constant(parse_result),
            _ => Formula::Value(Value::Error(ValueError::Parse)),
        },
        None => Formula::Value(Value::Error(ValueError::Parse)),
    }
}

pub fn parse_deps(s: &str) -> Vec<String> {
    let formula = parse_formula(s, None::<NoCustomFunction>);
    formula.get_references()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_deps() {
        let deps = super::parse_deps("=A1 + B2 + C3");
        assert_eq!(deps, vec!["A1", "B2", "C3"]);
    }

    #[test]
    fn test_parse_range_deps() {
        let deps = super::parse_deps("=SUM(A1:B2)");
        assert_eq!(deps, vec!["A1:B2"]);
    }

    #[test]
    fn test_parse_deps_sheet_cell() {
        let deps = super::parse_deps("=Sheet1!A1 + Sheet2!B2 + Sheet3!C3");
        assert_eq!(deps, vec!["Sheet1!A1", "Sheet2!B2", "Sheet3!C3"]);
    }

    #[test]
    fn test_parse_deps_sheet_range() {
        let deps = super::parse_deps("=Sheet1!A1:A2 + Sheet2!B2:B4 + Sheet3!C3:D4");
        assert_eq!(deps, vec!["Sheet1!A1:A2", "Sheet2!B2:B4", "Sheet3!C3:D4"]);
    }
}
