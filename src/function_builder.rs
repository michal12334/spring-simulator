use egui::{Checkbox, DragValue, Widget};

use crate::function::{ConstFunction, Function, StepFunction};

#[derive(Debug, Default)]
pub struct FunctionBuilder {
    const_function_builder: ConstFunctionBuilder,
    step_function_builder: StepFunctionBuilder,
    builder_type: FunctionBuilderType,
}

impl FunctionBuilder {
    pub fn show(&mut self, ui: &mut egui::Ui) -> bool {
        let mut result = false;

        let mut checked = self.builder_type == FunctionBuilderType::Const;
        if Checkbox::new(&mut checked, "f(t) = A").ui(ui).changed() {
            if checked {
                self.builder_type = FunctionBuilderType::Const;
                result = true;
            }
        }
        result = result || (self.const_function_builder.show(ui) && self.builder_type == FunctionBuilderType::Const);

        let mut checked = self.builder_type == FunctionBuilderType::Step;
        if Checkbox::new(&mut checked, "f(t) =\n0 for t < min_t,\nA for t >= min_t").ui(ui).changed() {
            if checked {
                self.builder_type = FunctionBuilderType::Step;
                result = true;
            }
        }
        result = result || (self.step_function_builder.show(ui) && self.builder_type == FunctionBuilderType::Step);

        result
    }

    pub fn build(&self) -> Function {
        match self.builder_type {
            FunctionBuilderType::Const => Function::Const(self.const_function_builder.build()),
            FunctionBuilderType::Step => Function::Step(self.step_function_builder.build()),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
enum FunctionBuilderType {
    #[default]
    Const,
    Step,
}

#[derive(Debug, Default)]
pub struct ConstFunctionBuilder {
    value: f64,
}

impl ConstFunctionBuilder {
    pub fn show(&mut self, ui: &mut egui::Ui) -> bool {
        let mut result = false;

        ui.horizontal(|ui| {
            ui.label("A: ");
            result = DragValue::new(&mut self.value).speed(0.1).ui(ui).changed();
        });

        result
    }

    pub fn build(&self) -> ConstFunction {
        ConstFunction::new(self.value)
    }
}

#[derive(Debug, Default)]
pub struct StepFunctionBuilder {
    value: f64,
    min_t: f64,
}

impl StepFunctionBuilder {
    pub fn show(&mut self, ui: &mut egui::Ui) -> bool {
        let mut result = false;

        ui.horizontal(|ui| {
            ui.label("A: ");
            result = result || DragValue::new(&mut self.value).speed(0.1).ui(ui).changed();
        });
        ui.horizontal(|ui| {
            ui.label("min_t: ");
            result = result || DragValue::new(&mut self.min_t).speed(0.1).ui(ui).changed();
        });

        result
    }

    pub fn build(&self) -> StepFunction {
        StepFunction::new(self.value, self.min_t)
    }
}
