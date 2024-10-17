// use context::HashMapContext;
// use types::{Error, Value};
// mod calculate;
// pub mod context;
// pub mod error;
// mod formula_parser;
// mod parse_deps;
// pub mod types;
// pub mod value;

// pub struct FormulaEngine;
// pub use types::*;

// impl FormulaEngine {
//     pub fn new() -> Self {
//         FormulaEngine {}
//     }
//     pub fn evaluate(&self, formula: &str, ref_function: impl Fn(String) -> Value) -> Value {
//         let formula = formula_parser::parse_string_to_formula(formula, None::<NoCustomFunction>);
//         let result = calculate::calculate_formula(formula, Some(&ref_function));
//         result
//     }
// }

// pub fn evaluate(expression: &str, context: HashMapContext) {}

// #[cfg(test)]
// mod test {

//     use calculate::utils;

//     use super::*;
//     use crate::{cell::CellCoords, Workbook};
//     fn get_workbook(values: Option<Vec<Vec<Value>>>) -> Workbook {
//         let data = match values {
//             Some(v) => v,
//             None => vec![
//                 vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)],
//                 vec![Value::Number(4.0), Value::Number(5.0), Value::Number(6.0)],
//                 vec![Value::Number(7.0), Value::Number(8.0), Value::Number(9.0)],
//             ],
//         };
//         let workbook = Workbook::new();

//         workbook.add_sheet("Sheet1".to_string());
//         let sheet = workbook.get_sheet("Sheet1").unwrap();
//         for (row, r) in data.iter().enumerate() {
//             for (col, c) in r.iter().enumerate() {
//                 sheet.set_cell_value(&CellCoords::new(row + 1, col + 1), c.clone());
//             }
//         }

//         workbook
//     }

//     #[test]
//     fn test_basic_calculate() {
//         let formula = formula_parser::parse_string_to_formula(&"=1+1", None::<NoCustomFunction>);
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(result, types::Value::Number(2.0));
//     }

//     #[test]
//     fn test_basic_math() {
//         let formula = formula_parser::parse_string_to_formula(&"=1+2", None::<NoCustomFunction>);
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "3");

//         let formula =
//             formula_parser::parse_string_to_formula(&"=(1*(2+3))*2", None::<NoCustomFunction>);
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "10");

//         let formula =
//             formula_parser::parse_string_to_formula(&"=300/1.2", None::<NoCustomFunction>);
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "250");

//         let formula = formula_parser::parse_string_to_formula(&"=1+3/0", None::<NoCustomFunction>);
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "#DIV/0!");
//     }

//     #[test]
//     fn test_string_concatenation() {
//         let formula = formula_parser::parse_string_to_formula(
//             &"=\"Hello \" & \" World!\"",
//             None::<NoCustomFunction>,
//         );
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "Hello  World!");

//         let formula =
//             formula_parser::parse_string_to_formula(&"=1 + \"Hello\"", None::<NoCustomFunction>);
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "#CAST!");
//     }

//     #[test]
//     fn test_excel_functions() {
//         let formula =
//             formula_parser::parse_string_to_formula(&"=ABS(-1)", None::<NoCustomFunction>);
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "1");

//         let formula =
//             formula_parser::parse_string_to_formula(&"=SUM(1,2,\"3\")", None::<NoCustomFunction>);
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "6");

//         let formula = formula_parser::parse_string_to_formula(
//             &"=PRODUCT(ABS(1),2*1, 3,4*1)",
//             None::<NoCustomFunction>,
//         );
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "24");
//     }

//     #[test]
//     fn test_logical_expressions() {
//         let formula = formula_parser::parse_string_to_formula(&"=2>=1", None::<NoCustomFunction>);
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "TRUE");

//         let formula =
//             formula_parser::parse_string_to_formula(&"=OR(1>1,1<>1)", None::<NoCustomFunction>);
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "FALSE");
//     }

//     // #[test]
//     // fn test_date_functions() {
//     //     use chrono::{DateTime, FixedOffset};

//     //     let start: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-03-01T02:00:00.000Z").unwrap();
//     //     let end: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-08-30T02:00:00.000Z").unwrap();
//     //     let data_function = |s: String| match s.as_str() {
//     //         "start" => types::Value::Date(start),
//     //         "end" => types::Value::Date(end),
//     //         _ => types::Value::Error(types::Error::Value),
//     //     };

//     //     let formula = formula_parser::parse_string_to_formula(&"=DAYS(end, start)", None::<NoCustomFunction>);
//     //     let result = calculate::calculate_formula(formula, Some(&data_function));
//     //     assert_eq!(utils::result_to_string(result), "182");

//     //     let formula = formula_parser::parse_string_to_formula(&"=start+1", None::<NoCustomFunction>);
//     //     let result = calculate::calculate_formula(formula, Some(&data_function));
//     //     assert!(utils::result_to_string(result).contains("2019-03-02"));

//     //     let formula = formula_parser::parse_string_to_formula(&"=end-3", None::<NoCustomFunction>);
//     //     let result = calculate::calculate_formula(formula, Some(&data_function));
//     //     assert!(utils::result_to_string(result).contains("2019-08-27"));
//     // }

//     #[test]
//     fn test_custom_function() {
//         let custom_functions = |s: String, params: Vec<f64>| match s.as_str() {
//             "Increase" => types::Value::Number(params[0] + 1.0),
//             "SimpleSum" => types::Value::Number(params[0] + params[1]),
//             "EqualFive" => types::Value::Number(5.0),
//             _ => types::Value::Error(types::Error::Value),
//         };

//         let formula =
//             formula_parser::parse_string_to_formula(&"=Increase(1)+1", Some(&custom_functions));
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "3");

//         let formula =
//             formula_parser::parse_string_to_formula(&"=EqualFive()+1", Some(&custom_functions));
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "6");

//         let formula =
//             formula_parser::parse_string_to_formula(&"=SimpleSum(1,2)", Some(&custom_functions));
//         let result = calculate::calculate_formula(formula, None::<NoReference>);
//         assert_eq!(utils::result_to_string(result), "3");
//     }

//     // #[test]
//     // fn test_get_cell_by_address() {
//     //     let cell_a1_address = &CellAddress::from_str("A1").unwrap();
//     //     let _workbook = get_workbook();
//     //     tmp_workshet(None);
//     //     let wb = _workbook.read().unwrap();
//     //     let sheet = wb.get_sheet("Sheet1").unwrap();
//     //     let cell = sheet.get_cell(cell_a1_address).unwrap();
//     //     assert_eq!(cell.value, types::Value::Number(1.0));
//     // }

//     // #[test]
//     // fn test_evaluate_with_cell_ref() {
//     //     let _workbook = get_workbook();
//     //     tmp_workshet(None);
//     //     let cell = FormulaEngine::evaluate("=A1+A2");
//     //     assert_eq!(cell, types::Value::Number(5.0));
//     // }

//     // #[test]
//     // fn test_evaluate_with_range_ref() {
//     //     let _workbook = get_workbook();
//     //     tmp_workshet(None);
//     //     let cell = FormulaEngine::evaluate("=SUM(A1:A2)");
//     //     assert_eq!(cell, types::Value::Number(5.0));
//     // }
// }
