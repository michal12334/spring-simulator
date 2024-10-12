#[derive(Debug)]
pub enum Function {
    Const(ConstFunction),
}

impl Function {
    pub fn get_value(&self, t: f64) -> f64 {
        match self {
            Function::Const(f) => f.value,
        }
    }
}

#[derive(Debug)]
pub struct ConstFunction {
    value: f64,
}

impl ConstFunction {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}
