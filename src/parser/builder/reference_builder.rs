use crate::parser::grammar::Rule;
use crate::{Formula, Reference};
use pest::iterators::Pair;

pub fn build_formula_reference(pair: Pair<Rule>) -> Formula {
    // let string = pair.as_str().parse::<String>().unwrap();
    // Formula::Reference(string)
    // let rule = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::sheet_cell => build_formula_sheet_cell(pair),
        Rule::sheet_range => build_formula_sheet_range(pair),
        Rule::variable => build_formula_variable(pair),
        Rule::range => build_formula_range(pair),
        Rule::cell => build_formula_cell(pair),
        _ => Formula::Value(crate::Value::Error(crate::error::ValueError::Reference)),
    }
}

fn build_formula_variable(pair: Pair<Rule>) -> Formula {
    let string = pair.as_str().parse::<String>().unwrap();
    Formula::Reference(Reference::Variable(string))
}

fn build_formula_sheet_range(pair: Pair<Rule>) -> Formula {
    let string = pair.as_str().parse::<String>().unwrap();
    Formula::Reference(Reference::SheetRange(string))
}

fn build_formula_sheet_cell(pair: Pair<Rule>) -> Formula {
    let string = pair.as_str().parse::<String>().unwrap();
    Formula::Reference(Reference::SheetCell(string))
}
fn build_formula_range(pair: Pair<Rule>) -> Formula {
    let string = pair.as_str().parse::<String>().unwrap();
    Formula::Reference(Reference::Range(string))
}
fn build_formula_cell(pair: Pair<Rule>) -> Formula {
    let string = pair.as_str().parse::<String>().unwrap();
    Formula::Reference(Reference::Cell(string))
}

#[cfg(test)]
mod test {

    // use crate::xfeval::grammar::GrammarParser;

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
