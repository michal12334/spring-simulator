use egui::{DragValue, Widget};

use crate::function::{ConstFunction, Function};

#[derive(Debug, Default)]
pub struct FunctionBuilder {
    const_function_builder: ConstFunctionBuilder,
}

impl FunctionBuilder {
    pub fn show(&mut self, ui: &mut egui::Ui) -> bool {
        self.const_function_builder.show(ui)
    }

    pub fn build(&self) -> Function {
        Function::Const(self.const_function_builder.build())
    }
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
            result = DragValue::new(&mut self.value).ui(ui).changed();
        });

        result
    }

    pub fn build(&self) -> ConstFunction {
        ConstFunction::new(self.value)
    }
}
