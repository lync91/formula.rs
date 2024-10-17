use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./parser/grammar/grammar.pest"]
pub struct Grammar;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    // #[test]
    // fn test_reference() {
    //     // Kiểm tra tham chiếu ô từ sheet khác
    //     let parse_result = GrammarParser::parse(Rule::reference, "A6");
    //     assert!(parse_result.is_ok(), "Failed to parse reference");
    // }

    // #[test]
    // fn test_internal_reference() {
    //     // Kiểm tra tham chiếu ô từ sheet khác
    //     let parse_result = GrammarParser::parse(Rule::sheet_inner, "A6");
    //     assert!(parse_result.is_ok(), "Failed to parse reference");
    // }

    // #[test]
    // fn test_internal_range_reference() {
    //     // Kiểm tra tham chiếu ô từ sheet khác
    //     let parse_result = GrammarParser::parse(Rule::sheet_inner, "A6:A9");
    //     assert!(parse_result.is_ok(), "Failed to parse reference");
    // }

    // #[test]
    // fn test_range_reference() {
    //     // Kiểm tra tham chiếu khoảng từ sheet khác
    //     let parse_result = GrammarParser::parse(Rule::range_reference, "'Sheet 1'!A1:B5");
    //     assert!(parse_result.is_ok(), "Failed to parse range reference");
    // }

    // #[test]
    // fn test_sheet_name() {
    //     // Kiểm tra tên sheet với khoảng trắng và không khoảng trắng
    //     let parse_result_1 = GrammarParser::parse(Rule::sheet_name, "'Sheet 1'");
    //     assert!(
    //         parse_result_1.is_ok(),
    //         "Failed to parse sheet name with space"
    //     );

    //     let parse_result_2 = GrammarParser::parse(Rule::sheet_name, "Data2023");
    //     assert!(
    //         parse_result_2.is_ok(),
    //         "Failed to parse sheet name without space"
    //     );
    // }
    #[test]
    fn test_formula_sum() {
        // Kiểm tra hàm SUM với tham chiếu khoảng từ sheet khác
        let parse_result = Grammar::parse(Rule::formula, "SUM('Sheet 1'!A1:A5)");
        assert!(parse_result.is_ok(), "Failed to parse SUM formula");
    }

    #[test]
    fn test_formula_average() {
        // Kiểm tra hàm SUM với tham chiếu khoảng từ sheet khác
        let parse_result = Grammar::parse(Rule::formula, "AVERAGE('Sheet 1'!A1:A5)");
        assert!(parse_result.is_ok(), "Failed to parse SUM formula");
    }

    #[test]
    fn test_formula() {
        // Kiểm tra hàm SUM với tham chiác
        let parse_result = Grammar::parse(Rule::formula, "SUM(A1:A5)");
        assert!(parse_result.is_ok(), "Failed to parse SUM formula");
    }

    #[test]
    fn test_function() {
        // Kiểm tra hàm SUM với tham chiác
        let parse_result = Grammar::parse(Rule::formula, "=ABS");
        assert!(parse_result.is_ok(), "Failed to parse formula");
    }

    #[test]
    fn test_operator() {
        // Kiểm tra hàm SUM với tham chiác
        let parse_result = Grammar::parse(Rule::formula, "+");
        assert!(parse_result.is_ok(), "Failed to parse formula");
    }

    #[test]
    fn test_cell_reference() {
        // Kiểm tra hàm SUM với tham chiác
        let parse_result = Grammar::parse(Rule::formula, "A1");
        assert!(parse_result.is_ok(), "Failed to parse formula");
    }

    #[test]
    fn test_cell_range_reference() {
        // Kiểm tra hàm SUM với tham chiác
        let parse_result = Grammar::parse(Rule::range, "A1:A5");
        assert!(parse_result.is_ok(), "Failed to parse formula");
    }

    #[test]
    fn test_sheet_reference() {
        // Kiểm tra hàm SUM với tham chiác
        let parse_result = Grammar::parse(Rule::sheet_cell, "'Sheet 1'!A1");
        assert!(parse_result.is_ok(), "Failed to parse formula");
    }

    #[test]
    fn test_sheet_range_reference() {
        // Kiểm tra hàm SUM với tham chiác
        let parse_result = Grammar::parse(Rule::sheet_range, "'Sheet 1'!A1:A5");
        assert!(parse_result.is_ok(), "Failed to parse formula");
    }
    #[test]
    fn test_sheet_range_reference_unicode_name() {
        // Kiểm tra hàm SUM với tham chiác
        let parse_result = Grammar::parse(Rule::sheet_range, "'Công trình'!A1:A5");
        assert!(parse_result.is_ok(), "Failed to parse formula");
    }

    #[test]
    fn test_custom_function() {
        // Kiểm tra hàm SUM với tham chiác
        let parse_result = Grammar::parse(Rule::custom_function, "cus_fn(A1)");
        assert!(parse_result.is_ok(), "Failed to parse formula");
    }

    #[test]
    fn test_reference() {
        // Kiểm tra hàm SUM với tham chiác
        let parse_result = Grammar::parse(Rule::reference, "A1");
        assert!(parse_result.is_ok(), "Failed to parse formula");
        let parse_result = Grammar::parse(Rule::reference, "A1:A1");
        assert!(parse_result.is_ok(), "Failed to parse formula");
        let parse_result = Grammar::parse(Rule::reference, "Sheet1!A1");
        assert!(parse_result.is_ok(), "Failed to parse formula");
        let parse_result = Grammar::parse(Rule::reference, "Sheet1!A1:A1");
        assert!(parse_result.is_ok(), "Failed to parse formula");
        let parse_result = Grammar::parse(Rule::reference, "'Công trình'!A1");
        assert!(parse_result.is_ok(), "Failed to parse formula");
        let parse_result = Grammar::parse(Rule::reference, "'Công trình'!A1:A2");
        assert!(parse_result.is_ok(), "Failed to parse formula");
        let parse_result = Grammar::parse(Rule::reference, "Cong_Viec");
        assert!(parse_result.is_ok(), "Failed to parse formula");
    }

    #[test]
    fn test_reference_parts() {
        // Định nghĩa một bảng chứa các tham chiếu cần kiểm tra và loại kết quả mong đợi
        let test_cases = vec![
            ("A1", Rule::cell, "single cell reference"),
            ("A1:B2", Rule::range, "range reference"),
            ("Sheet1!A1", Rule::sheet_cell, "sheet cell reference"),
            ("Sheet1!A1:B2", Rule::sheet_range, "sheet range reference"),
        ];

        // Lặp qua từng trường hợp và kiểm tra
        for (input, expected_rule, description) in test_cases {
            let parse_result = Grammar::parse(Rule::reference, input);
            assert!(
                parse_result.is_ok(),
                "Failed to parse {}: {}",
                description,
                input
            );

            // Kiểm tra xem kết quả parse có chứa phần tử mong đợi không
            let parsed_pair = parse_result.unwrap().next().unwrap(); // Lấy kết quả đầu tiên
            assert_eq!(
                parsed_pair.as_rule(),
                Rule::reference,
                "Expected reference rule for {}",
                description
            );

            // Kiểm tra các phần bên trong của reference
            let inner_pair = parsed_pair.into_inner().next().unwrap(); // Lấy phần tử bên trong reference
            assert_eq!(
                inner_pair.as_rule(),
                expected_rule,
                "Expected {:?} for {}",
                expected_rule,
                description
            );
        }
    }

    // #[test]
    // fn split_range() {
    //     let parse_result = GrammarParser::parse(Rule::reference, "A1:A2");
    //     for range in parse_result {}
    // }
}
