use crate::error::ValueError;
use crate::value::Value;
use rust_decimal::{prelude::*, Decimal};

pub fn calculate_divide_operator(num1: f64, num2: f64) -> f64 {
    // Chuyển đổi f64 sang Decimal
    if num2 == 0.0 {
        return f64::INFINITY;
    }
    let decimal_num1 = Decimal::from_f64(num1).unwrap();
    let decimal_num2 = Decimal::from_f64(num2).unwrap();

    // Thực hiện phép chia
    let res = decimal_num1 / decimal_num2;
    res.to_f64().unwrap()
}

pub fn is_float_int(num: f64) -> bool {
    (((num as i32) as f64) - num).abs() == 0.0
}

pub fn calculate_power_operator(num1: f64, num2: f64) -> f64 {
    if is_float_int(num2) {
        num1.powi(num2 as i32)
    } else {
        num1.powf(num2)
    }
}

pub fn calculate_numeric_operator_rhs_number(
    l: f64,
    lhs: Value,
    rhs: Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Value {
    match rhs {
        Value::Boolean(_) => rhs,
        Value::Error(_) => rhs,
        Value::Text(t) => match t.parse::<f64>() {
            Ok(nr) => Value::Number(f(l, nr)),
            Err(_) => Value::Error(ValueError::Cast),
        },
        Value::Number(r) => Value::Number(f(l, r)),
        Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_numeric_operator(temp, top, f);
                }
                calculate_numeric_operator(lhs, temp, f)
            } else {
                Value::Error(ValueError::Argument)
            }
        }
        Value::Date(_) => Value::Error(ValueError::Value),
        Value::Blank => Value::Number(f(l, 0.0)),
    }
}

pub fn calculate_numeric_operator_product_rhs_number(
    l: f64,
    lhs: Value,
    rhs: Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Value {
    match rhs {
        Value::Boolean(_) => rhs,
        Value::Error(_) => rhs,
        Value::Text(t) => match t.parse::<f64>() {
            Ok(nr) => Value::Number(f(l, nr)),
            Err(_) => Value::Error(ValueError::Cast),
        },
        Value::Number(r) => Value::Number(f(l, r)),
        Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_numeric_product_operator(temp, top, f);
                }
                calculate_numeric_product_operator(lhs, temp, f)
            } else {
                Value::Error(ValueError::Argument)
            }
        }
        Value::Date(_) => Value::Error(ValueError::Value),
        Value::Blank => match lhs {
            Value::Blank => Value::Blank,
            _ => Value::Number(l),
        },
    }
}

pub fn calculate_numeric_operator_rhs_iterator(
    mut lhs_vec: Vec<Value>,
    rhs: Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Value {
    match rhs {
        Value::Number(_) => {
            if let Some(mut temp) = lhs_vec.pop() {
                while let Some(top) = lhs_vec.pop() {
                    temp = calculate_numeric_operator(temp, top, f);
                }
                calculate_numeric_operator(temp, rhs, f)
            } else {
                Value::Error(ValueError::Argument)
            }
        }
        Value::Iterator(mut rhs_vec) => {
            let mut result_vec = Vec::new();
            loop {
                match (lhs_vec.pop(), rhs_vec.pop()) {
                    (Some(x), Some(y)) => {
                        result_vec.push(calculate_numeric_operator(x, y, f));
                    }
                    (Some(_), None) => result_vec.push(Value::Error(ValueError::Argument)),
                    (None, Some(_)) => result_vec.push(Value::Error(ValueError::Argument)),
                    (None, None) => break,
                };
            }
            Value::Iterator(result_vec)
        }
        _ => Value::Error(ValueError::Value),
    }
}

pub fn calcualte_numeric_operator_rhs_text(
    t: String,
    rhs: Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Value {
    match t.parse::<f64>() {
        Ok(nl) => match rhs {
            Value::Boolean(_) => rhs,
            Value::Error(_) => rhs,
            Value::Text(t) => match t.parse::<f64>() {
                Ok(nr) => Value::Number(f(nl, nr)),
                Err(_) => Value::Error(ValueError::Cast),
            },
            Value::Number(r) => Value::Number(f(nl, r)),
            Value::Iterator(_) => Value::Error(ValueError::Value),
            Value::Date(_) => Value::Error(ValueError::Value),
            Value::Blank => Value::Error(ValueError::Value),
        },
        Err(_) => Value::Error(ValueError::Cast),
    }
}

pub fn calculate_numeric_operator(
    lhs: Value,
    rhs: Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Value {
    match lhs {
        Value::Boolean(_) => lhs,
        Value::Error(_) => lhs,
        Value::Text(t) => calcualte_numeric_operator_rhs_text(t, rhs, f),
        Value::Number(l) => calculate_numeric_operator_rhs_number(l, lhs, rhs, f),
        Value::Iterator(lhs_vec) => calculate_numeric_operator_rhs_iterator(lhs_vec, rhs, f),
        Value::Date(_) => Value::Error(ValueError::Value),
        Value::Blank => calculate_numeric_operator_rhs_number(0.0, lhs, rhs, f),
    }
}

pub fn calculate_numeric_product_operator(
    lhs: Value,
    rhs: Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Value {
    //println!("{:?}::{:?}", lhs, rhs);
    match lhs {
        Value::Boolean(_) => lhs,
        Value::Error(_) => lhs,
        Value::Text(t) => calcualte_numeric_operator_rhs_text(t, rhs, f),
        Value::Number(l) => calculate_numeric_operator_product_rhs_number(l, lhs, rhs, f),
        Value::Iterator(lhs_vec) => calculate_numeric_operator_rhs_iterator(lhs_vec, rhs, f),
        Value::Date(_) => Value::Error(ValueError::Value),
        Value::Blank => calculate_numeric_operator_product_rhs_number(1.0, lhs, rhs, f),
    }
}

pub fn calculate_negate(value: Value) -> Value {
    match value {
        Value::Number(l) => Value::Number(-l),
        Value::Iterator(mut value_vec) => {
            let mut result_vec = Vec::new();
            while let Some(top) = value_vec.pop() {
                result_vec.push(calculate_negate(top));
            }
            Value::Iterator(result_vec)
        }
        Value::Blank => Value::Blank,
        _ => Value::Error(ValueError::Value),
    }
}
