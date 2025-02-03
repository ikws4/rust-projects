use super::{flow::Flow, interpreter::Interpreter, value::Value};

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Value>) -> Result<Value, Flow>;
}
