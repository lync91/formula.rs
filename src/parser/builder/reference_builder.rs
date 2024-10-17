use crate::parser::grammar::Rule;
use crate::Formula;
use pest::iterators::Pair;

pub fn build_formula_reference(pair: Pair<Rule>) -> Formula {
    let string = pair.as_str().parse::<String>().unwrap();
    Formula::Reference(string)
}

// fn build_formula_char_reference(pair: Pair<Rule>) -> Formula {
//     let string = pair.as_str().parse::<String>().unwrap();
//     Formula::Reference(Reference::CharReference(string))
// }

// fn build_formula_sheet_range(pair: Pair<Rule>) -> Formula {
//     let string = pair.as_str().parse::<String>().unwrap();
//     Formula::Reference(Reference::SheetRange(string))
// }

// fn build_formula_sheet_cell(pair: Pair<Rule>) -> Formula {
//     let string = pair.as_str().parse::<String>().unwrap();
//     Formula::Reference(Reference::SheetCell(string))
// }

// fn build_formula_range(pair: Pair<Rule>) -> Formula {
//     let mut coords = vec![];
//     for p in pair.into_inner() {
//         if p.as_rule() == Rule::cell {
//             let cell_str = p.as_str();
//             let cell_coords = cell_string_to_usize(cell_str);
//             coords.push(cell_coords);
//         }
//     }

//     if coords.len() == 2 {
//         let (row1, col1) = coords[0].unwrap();
//         let (row2, col2) = coords[1].unwrap();
//         Formula::Reference(Reference::Range((row1, col1), (row2, col2)))
//     } else {
//         Formula::Value(Value::Error(Error::Value))
//     }
//     // Formula::Reference(Reference::SheetCell("string".to_string()))
// }

// fn build_formula_cell(pair: Pair<Rule>) -> Formula {
//     let string = pair.as_str();
//     match cell_string_to_usize(string) {
//         Some((row, col)) => Formula::Reference(Reference::Cell(row, col)),
//         _ => Formula::Value(Value::Error(Error::Reference)),
//     }
//     // Formula::Reference(Reference::SheetCell("string".to_string()))
// }

#[cfg(test)]
mod test {
    use pest::Parser;

    use super::*;
    // use crate::formula::grammar::GrammarParser;

    // #[test]
    // fn test_build_formula_cell() {
    //     let parse_result = GrammarParser::parse(Rule::cell, "A1+A2");
    //     let pairs = parse_result.unwrap();
    //     for pair in pairs {
    //         let cell = build_formula_cell(pair);
    //         match cell {
    //             Formula::Reference(reff) => match reff {
    //                 Reference::Cell(r, c) => assert_eq!((r, c), (1, 1)),
    //                 _ => {}
    //             },
    //             _ => {}
    //         }
    //     }
    // }

    // #[test]
    // fn parse_formula_range() {
    //     let parse_result = GrammarParser::parse(Rule::range, "A1:B1");
    //     assert!(parse_result.is_ok());
    // }
}
