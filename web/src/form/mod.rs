use crate::page::PageError;
use seed::{prelude::*, *};

mod input_data;
mod input_type;
pub use input_data::*;
pub use input_type::*;

#[derive(Clone, Default, Debug)]
pub struct Form {
    pub inputs: Vec<Input>,
}

impl Form {
    pub fn update(&mut self, msg: FormMsg) {
        match msg {
            FormMsg::UpdateValue(i, s) => {
                self.inputs[i].set(s);
            }
            FormMsg::Clear(i) => {
                log!("clarning");
                self.inputs[i].set(String::new());
            }
        }
    }

    pub fn view(&self) -> Vec<Node<FormMsg>> {
        self.inputs
            .iter()
            .enumerate()
            .map(|(i, input)| input.view(i))
            .collect()
    }

    pub fn set_all(&mut self, values: &Vec<String>) {
        self.inputs.iter_mut().zip(values.iter()).for_each(|(input, new)| {
            input.set(new.to_owned())
        })
    }

    pub fn get_input_data(&self) -> Result<Vec<InputData>, PageError> {
        self.inputs.iter().map(|input| input.get_data()).collect() // shouldn't work?!?!?
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    name: String,
    value: String,
    typ: InputType,
}

impl Input {
    pub fn new(name: &str, typ: InputType) -> Self {
        let default = typ.default_value();
        Self::with_initial(name, typ, &default)
    }

    pub fn with_initial(name: &str, typ: InputType, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            typ,
        }
    }

    fn set(&mut self, value: String) {
        self.value = value;
    }

    fn view(&self, i: usize) -> Node<FormMsg> {
        let label = match self.typ {
            InputType::Range(_, _) => format!("{}: {}", self.name, self.value),
            _ => format!("{}: ", self.name),
        };
        div![label![label], self.typ.view(i, &self.value),]
    }

    fn get_data(&self) -> Result<InputData, PageError> {
        self.typ
            .to_data(&self.value)
            .map_err(|_| PageError::form(&self.name))
    }
}
