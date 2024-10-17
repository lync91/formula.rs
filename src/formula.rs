use crate::{
    calculate::calculate_formula, parse_formula, parser::operators::Operator, value::Value,
};

#[derive(Debug)]
pub enum Formula {
    Operation(Expression),
    Value(Value),
    Reference(Reference),
    Iterator(Vec<Formula>),
}

impl Formula {
    pub fn new(s: &str) -> Self {
        parse_formula(s, None::<NoCustomFunction>)
    }
    pub fn new_with_custom_function(s: &str, f: NoCustomFunction) -> Self {
        parse_formula(s, Some(f))
    }
    pub fn get_references(&self) -> Vec<String> {
        let mut references = Vec::new();
        match self {
            Formula::Reference(ref_name) => {
                references.push(ref_name.as_str().to_string());
            }
            Formula::Operation(expression) => {
                for value in &expression.values {
                    references.extend(value.get_references());
                }
            }
            Formula::Iterator(formulas) => {
                for formula in formulas {
                    references.extend(formula.get_references());
                }
            }
            _ => {}
        }
        references
    }
    pub fn evaluate(self) -> Value {
        let value = calculate_formula(self, None::<NoReference>);
        value
    }
    pub fn get_deps(&self) -> Vec<Dependents> {
        let mut deps = Vec::new();
        match self {
            Formula::Reference(ref_name) => {
                deps.push(ref_name.as_dependent());
            }
            Formula::Operation(expression) => {
                for value in &expression.values {
                    deps.extend(value.get_deps());
                }
            }
            Formula::Iterator(formulas) => {
                for formula in formulas {
                    deps.extend(formula.get_deps());
                }
            }
            _ => {}
        }
        deps
    }
}

#[derive(Debug)]
pub struct Expression {
    pub op: Operator,
    pub values: Vec<Formula>,
}

#[derive(Debug)]
pub enum Reference {
    Cell(String),
    Range(String),
    SheetCell(String),
    SheetRange(String),
    Variable(String),
}

impl Reference {
    pub fn as_str(&self) -> &str {
        match self {
            Reference::Cell(s) => s.as_str(),
            Reference::Range(s) => s.as_str(),
            Reference::SheetCell(s) => s.as_str(),
            Reference::SheetRange(s) => s.as_str(),
            Reference::Variable(s) => s.as_str(),
        }
    }

    pub fn as_dependent(&self) -> Dependents {
        match self {
            Reference::Cell(s) => Dependents::Dep(s.clone()),
            Reference::Range(s) => Dependents::Dep(s.clone()),
            Reference::SheetCell(s) => Dependents::SheetDep(s.clone()),
            Reference::SheetRange(s) => Dependents::SheetDep(s.clone()),
            Reference::Variable(s) => Dependents::Variable(s.clone()),
        }
    }
}

pub enum Dependents {
    Dep(String),
    SheetDep(String),
    Variable(String),
}

pub type NoReference<'a> = &'a fn(&str) -> Value;
pub type NoCustomFunction<'a> = &'a fn(&str, Vec<f64>) -> Value;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_formula() {
        let formula = Formula::new("=1+2");
        assert_eq!(formula.evaluate().str_from(), "3");
    }
}
