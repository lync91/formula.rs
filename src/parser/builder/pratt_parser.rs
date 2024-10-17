use super::*;
use crate::Formula;
use crate::{parser::grammar::Rule, Value};
use pest::pratt_parser::{Assoc, Op, PrattParser};

pub fn build_formula_with_pratt(
    expression: pest::iterators::Pairs<Rule>,
    f: Option<&impl Fn(&str, Vec<f64>) -> Value>,
) -> Formula {
    let pratt = create_pratt_parser();
    pratt
        .map_primary(|pair: pest::iterators::Pair<Rule>| map_primary(pair, f))
        .map_infix(|lhs, op, rhs| map_infix(lhs, op, rhs))
        .parse(expression)
}

fn create_pratt_parser() -> PrattParser<Rule> {
    PrattParser::new()
        .op(Op::infix(Rule::concat, Assoc::Left))
        .op(Op::infix(Rule::equal, Assoc::Left) | Op::infix(Rule::not_equal, Assoc::Left))
        .op(Op::infix(Rule::greater, Assoc::Left)
            | Op::infix(Rule::less, Assoc::Left)
            | Op::infix(Rule::greater_or_equal, Assoc::Left)
            | Op::infix(Rule::less_or_equal, Assoc::Left))
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::subtract, Assoc::Left))
        .op(Op::infix(Rule::multiply, Assoc::Left) | Op::infix(Rule::divide, Assoc::Left))
        .op(Op::infix(Rule::power, Assoc::Right))
}

fn map_primary(
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(&str, Vec<f64>) -> Value>,
) -> Formula {
    match pair.as_rule() {
        Rule::number => build_formula_number(pair),
        Rule::string_double_quote => build_formula_string_double_quote(pair),
        Rule::string_single_quote => build_formula_string_single_quote(pair),
        Rule::t => build_formula_boolean(true),
        Rule::f => build_formula_boolean(false),
        Rule::abs => build_formula_unary_operator(Rule::abs, pair, f),
        Rule::sum => build_formula_collective_operator(Rule::sum, pair, f),
        Rule::product => build_formula_collective_operator(Rule::product, pair, f),
        Rule::average => build_formula_collective_operator_average(Rule::average, pair, f),
        Rule::or => build_formula_collective_operator(Rule::or, pair, f),
        Rule::and => build_formula_collective_operator_and(Rule::and, pair, f),
        Rule::xor => build_formula_collective_operator(Rule::xor, pair, f),
        Rule::not => build_formula_unary_operator(Rule::not, pair, f),
        Rule::reference => build_formula_reference(pair.into_inner().next().unwrap()),
        Rule::iterator => build_formula_iterator(pair, f),
        Rule::negate => build_formula_unary_operator(Rule::negate, pair, f),
        Rule::expr => build_formula_with_pratt(pair.into_inner(), f),
        Rule::days => build_formula_collective_operator(Rule::days, pair, f),
        Rule::right => build_formula_collective_operator(Rule::right, pair, f),
        Rule::left => build_formula_collective_operator(Rule::left, pair, f),
        Rule::custom_function => build_formula_custom_function(pair, f),
        Rule::iff => build_formula_iff(pair, f),
        Rule::isblank => build_formula_collective_operator(Rule::isblank, pair, f),
        Rule::atomic_expr => build_formula_with_pratt(pair.into_inner(), f),
        _ => unreachable!(),
    }
}

fn map_infix(lhs: Formula, op: pest::iterators::Pair<Rule>, rhs: Formula) -> Formula {
    match op.as_rule() {
        Rule::add => build_formula_binary_operator(Rule::add, lhs, rhs),
        Rule::subtract => build_formula_binary_operator(Rule::subtract, lhs, rhs),
        Rule::multiply => build_formula_binary_operator(Rule::multiply, lhs, rhs),
        Rule::divide => build_formula_binary_operator(Rule::divide, lhs, rhs),
        Rule::power => build_formula_binary_operator(Rule::power, lhs, rhs),
        Rule::concat => build_formula_binary_operator(Rule::concat, lhs, rhs),
        Rule::equal => build_formula_binary_operator(Rule::equal, lhs, rhs),
        Rule::not_equal => build_formula_binary_operator(Rule::not_equal, lhs, rhs),
        Rule::greater => build_formula_binary_operator(Rule::greater, lhs, rhs),
        Rule::less => build_formula_binary_operator(Rule::less, lhs, rhs),
        Rule::greater_or_equal => build_formula_binary_operator(Rule::greater_or_equal, lhs, rhs),
        Rule::less_or_equal => build_formula_binary_operator(Rule::less_or_equal, lhs, rhs),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_build_prarr() {}
}
