use context::{Context, HashMapContext};
use value::Value;
pub mod calculate;
pub mod context;
pub mod error;
pub mod formula;
mod function;
mod parse_deps;
pub mod parser;
pub mod value;

pub struct FormulaEngine;
pub use formula::*;
pub use parser::parse_string_to_formula;

impl FormulaEngine {
    pub fn new() -> Self {
        FormulaEngine {}
    }
    pub fn evaluate(&self, formula: &str, ref_function: impl Fn(&str) -> Value) -> Value {
        let formula = parser::parse_string_to_formula(formula, None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, Some(&ref_function));
        result
    }
}

pub fn evaluate(expression: &str, context: HashMapContext) -> Value {
    let ref_function = |r: &str| -> Value {
        match context.get_value(r) {
            Some(v) => v.clone(),
            _ => Value::Blank,
        }
    };
    let formula = parser::parse_string_to_formula(expression, None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, Some(&ref_function));
    result
}

#[cfg(test)]
mod test {

    use super::*;
    use chrono::DateTime;
    use context::ContextWithMutableVariables;
    use error::ValueError;
    use value::Value;

    #[test]
    fn test_simple_formula() {
        let formula = parser::parse_string_to_formula(&"=1+1", None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, None::<NoReference>);
        assert_eq!(result.str_from().as_str(), "2");
    }

    #[test]
    fn test_complex_formula() {
        let formula = parser::parse_string_to_formula(&"=(1+1)*2", None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, None::<NoReference>);
        assert_eq!(result.str_from().as_str(), "4");
    }

    #[test]
    fn test_date_functions() {
        use chrono::{DateTime, FixedOffset};

        let start: DateTime<FixedOffset> =
            DateTime::parse_from_rfc3339("2019-03-01T02:00:00.000Z").unwrap();
        let end: DateTime<FixedOffset> =
            DateTime::parse_from_rfc3339("2019-08-30T02:00:00.000Z").unwrap();
        let data_function = |s: &str| match s {
            "start" => Value::Date(start),
            "end" => Value::Date(end),
            _ => Value::Error(ValueError::Value),
        };

        let formula =
            parser::parse_string_to_formula(&"=DAYS(end, start)", None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, Some(&data_function));
        assert_eq!(result.str_from().as_str(), "182");

        let formula = parser::parse_string_to_formula(&"=start+1", None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, Some(&data_function));
        assert!(result.str_from().as_str().contains("2019-03-02"));

        let formula = parser::parse_string_to_formula(&"=end-3", None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, Some(&data_function));
        assert!(result.str_from().as_str().contains("2019-08-27"));
    }

    #[test]
    fn test_custom_function() {
        let custom_functions = |s: &str, params: Vec<f64>| match s {
            "Increase" => Value::Number(params[0] + 1.0),
            "SimpleSum" => Value::Number(params[0] + params[1]),
            "EqualFive" => Value::Number(5.0),
            _ => Value::Error(ValueError::Value),
        };

        let formula = parser::parse_string_to_formula(&"=Increase(1)+1", Some(&custom_functions));
        let result = calculate::calculate_formula(formula, None::<NoReference>);
        assert_eq!(result.str_from().as_str(), "3");

        let formula = parser::parse_string_to_formula(&"=EqualFive()+1", Some(&custom_functions));
        let result = calculate::calculate_formula(formula, None::<NoReference>);
        assert_eq!(result.str_from().as_str(), "6");

        let formula = parser::parse_string_to_formula(&"=SimpleSum(1,2)", Some(&custom_functions));
        let result = calculate::calculate_formula(formula, None::<NoReference>);
        assert_eq!(result.str_from().as_str(), "3");
    }
    #[test]
    fn test_formula() {
        let formula = parser::parse_string_to_formula(&"=1+1", None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, None::<NoReference>);
        assert_eq!(result.str_from().as_str(), "2");

        let formula = parser::parse_string_to_formula(&"=1+1+1", None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, None::<NoReference>);
        assert_eq!(result.str_from().as_str(), "3");

        let formula = parser::parse_string_to_formula(&"=1+1+1+1", None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, None::<NoReference>);
        assert_eq!(result.str_from().as_str(), "4");

        let formula = parser::parse_string_to_formula(&"=1+1+1+1+1", None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, None::<NoReference>);
        assert_eq!(result.str_from().as_str(), "5");

        let formula = parser::parse_string_to_formula(&"=1+1+1+1+1+1", None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, None::<NoReference>);
        assert_eq!(result.str_from().as_str(), "6");

        let formula = parser::parse_string_to_formula(&"=1+1+1+1+1+1+1", None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, None::<NoReference>);
        assert_eq!(result.str_from().as_str(), "7");

        let formula =
            parser::parse_string_to_formula(&"=1+1+1+1+1+1+1+1", None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, None::<NoReference>);
        assert_eq!(result.str_from().as_str(), "8");

        let formula =
            parser::parse_string_to_formula(&"=1+1+1+1+1+1+1+1+1", None::<NoCustomFunction>);
        let result = calculate::calculate_formula(formula, None::<NoReference>);
        assert_eq!(result.str_from().as_str(), "9");
    }

    #[test]
    fn test_evaluate() {
        let context = HashMapContext::new();
        let result = evaluate("=1+1", context);
        assert_eq!(result.str_from(), String::from("2"));

        let context = HashMapContext::new();
        let result = evaluate("=1+1+1", context);
        assert_eq!(result.str_from().as_str(), "3");

        let context = HashMapContext::new();
        let result = evaluate("=1+1+1+1", context);
        assert_eq!(result.str_from().as_str(), "4");

        let context = HashMapContext::new();
        let result = evaluate("=1+1+1+1+1", context);
        assert_eq!(result.str_from().as_str(), "5");

        let context = HashMapContext::new();
        let result = evaluate("=1+1+1+1+1+1", context);
        assert_eq!(result.str_from().as_str(), "6");

        let context = HashMapContext::new();
        let result = evaluate("=1+1+1+1+1+1+1", context);
        assert_eq!(result.str_from().as_str(), "7");

        let context = HashMapContext::new();
        let result = evaluate("=1+1+1+1+1+1+1+1", context);
        assert_eq!(result.str_from().as_str(), "8");

        let context = HashMapContext::new();
        let result = evaluate("=1+1+1+1+1+1+1+1+1", context);
        assert_eq!(result.str_from().as_str(), "9");
    }

    #[test]
    fn test_evaluate_with_date_context() {
        let mut context = HashMapContext::new();
        context.set_value(
            "start".to_string(),
            Value::Date(DateTime::parse_from_rfc3339("2019-03-01T02:00:00.000Z").unwrap()),
        );
        context.set_value(
            "end".to_string(),
            Value::Date(DateTime::parse_from_rfc3339("2019-03-04T02:00:00.000Z").unwrap()),
        );
        let result = evaluate("=start+1", context);
        assert!(result.str_from().as_str().contains("2019-03-02"));
    }

    #[test]
    fn test_evaluate_with_cell_context() {
        let mut context = HashMapContext::new();
        context.set_value("A1".to_string(), Value::Number(1.0));
        context.set_value("A2".to_string(), Value::Number(2.0));
        let result = evaluate("=A1+A2", context);
        assert_eq!(result.str_from().as_str(), "3");
    }

    #[test]
    fn test_evaluate_with_range_context() {
        let mut context = HashMapContext::new();
        context.set_value(
            "A1:A4".to_string(),
            Value::Iterator(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
            ]),
        );
        let result = evaluate("=SUM(A1:A4)", context);
        assert_eq!(result.str_from().as_str(), "10");
    }

    // #[test]
    // fn test_evaluate_with_custom_function() {
    //     let mut context = HashMapContext::new();
    //     let custom_functions = hashmap! {
    //         "Increase".to_string() => Box::new(|args: Vec<Value>| {
    //             let a = args[0].as_number().unwrap();
    //             Value::Number(a + 1.0)
    //         }),
    //     };

    //     let result = evaluate("=Increase(1)+1", context);
    //     assert_eq!(result.str_from().as_str(), "3");
    // }
    #[test]
    fn test_evaluate_with_sheet_cell_context() {
        let mut context = HashMapContext::new();
        context.set_value("Sheet1!A1".to_string(), Value::Number(1.0));
        context.set_value("Sheet1!A2".to_string(), Value::Number(2.0));
        let result = evaluate("=Sheet1!A1+Sheet1!A2", context);
        assert_eq!(result.str_from().as_str(), "3");
    }
}
