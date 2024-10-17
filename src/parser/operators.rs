use super::function::Function;
use super::grammar::Rule;

/// Defines Excel Operators.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Concat,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
    Function(Function),
}

pub fn rule_to_function_operator(collective_operation: Rule) -> Operator {
    match collective_operation {
        Rule::sum => Operator::Function(Function::Sum),
        Rule::product => Operator::Function(Function::Product),
        Rule::average => Operator::Function(Function::Average),
        Rule::or => Operator::Function(Function::Or),
        Rule::and => Operator::Function(Function::And),
        Rule::xor => Operator::Function(Function::Xor),
        Rule::days => Operator::Function(Function::Days),
        Rule::right => Operator::Function(Function::Right),
        Rule::left => Operator::Function(Function::Left),
        Rule::iff => Operator::Function(Function::Iff),
        Rule::isblank => Operator::Function(Function::IsBlank),
        _ => unreachable!(),
    }
}
