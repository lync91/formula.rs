use super::pratt_parser::build_formula_with_pratt;
use crate::parser::function::Function;
use crate::parser::grammar::Rule;
use crate::parser::operators::{rule_to_function_operator, Operator};
use crate::value::{Boolean, Value};
use crate::{Expression, Formula};

pub fn build_formula_binary_operator(binary_operator: Rule, lhs: Formula, rhs: Formula) -> Formula {
    let op_type = match binary_operator {
        Rule::add => Operator::Plus,
        Rule::subtract => Operator::Minus,
        Rule::multiply => Operator::Multiply,
        Rule::divide => Operator::Divide,
        Rule::power => Operator::Power,
        Rule::concat => Operator::Concat,
        Rule::equal => Operator::Equal,
        Rule::not_equal => Operator::NotEqual,
        Rule::greater => Operator::Greater,
        Rule::less => Operator::Less,
        Rule::greater_or_equal => Operator::GreaterOrEqual,
        Rule::less_or_equal => Operator::LessOrEqual,
        _ => unreachable!(),
    };
    let operation = Expression {
        op: op_type,
        values: vec![lhs, rhs],
    };
    Formula::Operation(operation)
}

pub fn build_formula_unary_operator(
    unary_operation: Rule,
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(&str, Vec<f64>) -> Value>,
) -> Formula {
    let op_type = match unary_operation {
        Rule::abs => Operator::Function(Function::Abs),
        Rule::not => Operator::Function(Function::Not),
        Rule::negate => Operator::Function(Function::Negate),
        _ => unreachable!(),
    };
    let operation = Expression {
        op: op_type,
        values: vec![build_formula_with_pratt(pair.into_inner(), f)],
    };
    Formula::Operation(operation)
}

pub fn build_formula_collective_operator(
    collective_operation: Rule,
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(&str, Vec<f64>) -> Value>,
) -> Formula {
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        if (term.as_str().parse::<String>().unwrap() == "")
            | (term.as_str().parse::<String>().unwrap() == ",")
            | (term.as_str().parse::<String>().unwrap() == ", ")
            | (term.as_str().parse::<String>().unwrap() == " ,")
        {
            vec.push(Formula::Value(Value::Blank))
        } else {
            vec.push(build_formula_with_pratt(term.into_inner(), f))
        }
    }
    let op_type = rule_to_function_operator(collective_operation);
    let operation = Expression {
        op: op_type,
        values: vec,
    };
    Formula::Operation(operation)
}

pub fn build_formula_collective_operator_average(
    collective_operation: Rule,
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(&str, Vec<f64>) -> Value>,
) -> Formula {
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        if (term.as_str().parse::<String>().unwrap() == "")
            | (term.as_str().parse::<String>().unwrap() == ",")
            | (term.as_str().parse::<String>().unwrap() == ", ")
            | (term.as_str().parse::<String>().unwrap() == " ,")
        {
            vec.push(Formula::Value(Value::Number(0.0)))
        } else {
            vec.push(build_formula_with_pratt(term.into_inner(), f))
        }
    }
    let op_type = rule_to_function_operator(collective_operation);
    let operation = Expression {
        op: op_type,
        values: vec,
    };
    Formula::Operation(operation)
}

pub fn build_formula_collective_operator_and(
    collective_operation: Rule,
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(&str, Vec<f64>) -> Value>,
) -> Formula {
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        if (term.as_str().parse::<String>().unwrap() == "")
            | (term.as_str().parse::<String>().unwrap() == ",")
            | (term.as_str().parse::<String>().unwrap() == ", ")
            | (term.as_str().parse::<String>().unwrap() == " ,")
        {
            vec.push(Formula::Value(Value::Boolean(Boolean::False)))
        } else {
            vec.push(build_formula_with_pratt(term.into_inner(), f))
        }
    }
    let op_type = rule_to_function_operator(collective_operation);
    let operation = Expression {
        op: op_type,
        values: vec,
    };
    Formula::Operation(operation)
}
