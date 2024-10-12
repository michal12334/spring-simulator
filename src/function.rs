#[derive(Debug)]
pub enum Function {
    Const(ConstFunction),
    Step(StepFunction),
}

impl Function {
    pub fn get_value(&self, t: f64) -> f64 {
        match self {
            Function::Const(f) => f.value,
            Function::Step(f) => if t >= f.min_t { f.value } else { 0.0 },
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

#[derive(Debug)]
pub struct StepFunction {
    value: f64,
    min_t: f64,
}

impl StepFunction {
    pub fn new(value: f64, min_t: f64) -> Self {
        Self { value, min_t }
    }
}
