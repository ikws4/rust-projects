use super::{array::Array, flow::Flow, value::Value};

fn str_internal(value: &Value) -> Result<String, Flow> {
    Ok(format!("{}", value))
}

pub fn str(values: &Vec<Value>) -> Result<Value, Flow> {
    Ok(Value::String(str_internal(&values[0])?))
}

pub fn assert(values: &Vec<Value>) -> Result<Value, Flow> {
    let ret = values[0].eq(&values[1])?.as_bool()?;
    let a = str_internal(&values[0])?;
    let b = str_internal(&values[1])?;
    assert!(ret, "Assertion fialed: {} == {}", a, b);
    Ok(Value::Void)
}

pub fn addr(values: &Vec<Value>) -> Result<Value, Flow> {
    let address = format!("{:p}", &values[0]);
    Ok(Value::String(address))
}

pub fn print(values: &Vec<Value>) -> Result<Value, Flow> {
    let output: Vec<String> = values
        .iter()
        .map(|value| str_internal(value).unwrap())
        .collect();

    println!("{}", output.join(" "));

    Ok(Value::Void)
}

pub fn length(values: &Vec<Value>) -> Result<Value, Flow> {
    match &values[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        Value::Array(arr) => Ok(Value::Number(arr.elements.borrow().len() as f64)),
        _ => Err(Flow::Error(
            "Cannot get length of non-string/array value".to_string(),
        )),
    }
}

pub fn range(values: &Vec<Value>) -> Result<Value, Flow> {
    let start = values[0].as_number()?;
    let end = values[1].as_number()?;
    let step = if values.len() == 3 {
        values[2].as_number()?
    } else {
        1.0
    };

    let mut range = Vec::new();
    let mut x = start;
    while x < end {
        range.push(Value::Number(x));
        x += step;
    }

    Ok(Value::Array(Array::new(range)))
}
