use crate::value::Value;

pub fn calculate_right(number_string: (Value, Value)) -> Value {
    let (number, string) = number_string;
    let trim_length = match number {
        Value::Number(x) => x as usize,
        _ => 0,
    };

    let trimmed_string = match string {
        Value::Text(s) => {
            let temp: &'static str = Box::leak(s.into_boxed_str());
            &temp[(temp.len() - trim_length)..]
        }
        _ => "",
    };
    Value::Text(trimmed_string.to_string())
}

pub fn calculate_left(number_string: (Value, Value)) -> Value {
    let (number, string) = number_string;
    let trim_length = match number {
        Value::Number(x) => x as usize,
        _ => 0,
    };

    let trimmed_string = match string {
        Value::Text(s) => {
            let temp: &'static str = Box::leak(s.into_boxed_str());
            &temp[..trim_length]
        }
        _ => "",
    };
    Value::Text(trimmed_string.to_string())
}
