use super::{flow::Flow, interpreter::Interpreter, value::Value};

pub trait TCall {
    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Value>) -> Result<Value, Flow>;
}
