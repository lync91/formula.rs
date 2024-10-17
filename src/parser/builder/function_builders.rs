use crate::error::ValueError;
use crate::parser::grammar::Rule;
use crate::value::Value;
use crate::Formula;

pub fn build_formula_custom_function(
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(&str, Vec<f64>) -> Value>,
) -> Formula {
    let mut vec = Vec::new();
    for field in pair.clone().into_inner() {
        match field.as_rule() {
            Rule::expr =>
            // vec.push(field.into_inner().as_str().parse::<f64>().unwrap()),
            // if custom formula is undefined, then don't have anything to push into vec
            {
                let x = field.into_inner().as_str();
                let y = x.parse::<f64>();
                match y {
                    Ok(_) => vec.push(y.unwrap()),
                    Err(_) => (),
                }
            }
            _ => (),
        }
    }
    let mut ref_string = "";
    for field in pair.clone().into_inner() {
        ref_string = match field.as_rule() {
            Rule::custom_function_name => field.as_str(),
            _ => ref_string,
        }
    }
    match f {
        Some(f) => match f(ref_string, vec) {
            Value::Number(x) => Formula::Value(Value::Number(x)),
            Value::Text(s) => Formula::Value(Value::Text(s)),
            Value::Boolean(x) => Formula::Value(Value::Boolean(x)),
            Value::Error(ValueError::Value) => Formula::Value(Value::Error(ValueError::Value)),
            Value::Iterator(v) => Formula::Value(Value::Iterator(v)),
            Value::Date(d) => Formula::Value(Value::Date(d)),
            Value::Blank => Formula::Value(Value::Blank),
            _ => Formula::Value(Value::Error(ValueError::Reference)),
        },
        None => Formula::Value(Value::Error(ValueError::Reference)),
    }
}
