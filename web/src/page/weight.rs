use crate::api_call::ApiCall;
use diet_database::weight::*;
use seed::{prelude::*, *};

use super::*;
use crate::form::*;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<Weight>, PageError>),
    FormUpdate(FormMsg),
    Edit(usize),
    Delete(usize),
    Deleted(Result<(), PageError>),
    Submit,
    Submitted(Result<(), PageError>),
}

impl PageMsg for Msg {
    fn delete(i: usize) -> Self {
        Msg::Delete(i)
    }
    fn edit(i: usize) -> Self {
        Msg::Edit(i)
    }
    fn submit() -> Self {
        Msg::Submit
    }
    fn load() -> Self {
        Msg::Fetch
    }
}

#[derive(Debug, Clone, Default)]
pub struct Model {
    data: Vec<Weight>,
    form: Form,
    err: Option<PageError>,
}

impl PageModel<Vec<Weight>, Msg> for Model {
    fn data(&self) -> &Vec<Weight> {
        &self.data
    }

    fn error(&self) -> Option<&PageError> {
        self.err.as_ref()
    }

    fn form_fields(&self) -> Vec<Node<Msg>> {
        self.form.view().map_msg(Msg::FormUpdate)
    }
}

impl FromInputData for NewWeight {
    fn from_input_data(inputs: Vec<InputData>) -> Result<Self, PageError> {
        Ok(Self {
            date: inputs[0].try_date()?,
            time: inputs[1].try_time_option()?,
            value: inputs[2].try_float()?,
        })
    }
}

pub fn init() -> Model {
    Model {
        form: Form {
            inputs: vec![
                Input::new("Date", InputType::Date),
                Input::new("Time", InputType::TimeOption),
                Input::new("Weight", InputType::Float),
            ],
        },
        ..Default::default()
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    use Msg::*;
    let api_call = ApiCall::Weight;
    match msg {
        Fetch => {
            orders.perform_cmd({
                async move {
                    match api_call.get().await {
                        Ok(s) => Fetched(Ok(s)),
                        Err(_) => Fetched(Err(PageError::Load)),
                    }
                }
            });
        }
        Fetched(result) => match result {
            Ok(data) => model.data = data,
            Err(err) => model.err = Some(err),
        },
        FormUpdate(update_msg) => model.form.update(update_msg),
        Edit(idx) => {
            let b = model.data[idx];
            model.form.set_all(&vec![b].matrix()[0]);
        }
        Delete(idx) => {
            let b = model.data[idx];
            if confirm(b) {
                orders.perform_cmd({
                    async move {
                        match api_call.delete(b).await {
                            Ok(s) if s.status().is_ok() => Deleted(Ok(())),
                            _ => Deleted(Err(PageError::Delete)),
                        }
                    }
                });
            }
        }
        Deleted(result) => match result {
            Ok(()) => {
                orders.send_msg(Fetch);
            }
            Err(err) => model.err = Some(err),
        },
        Submit => match model.form.get_input_data() {
            Ok(inputs) => match NewWeight::from_input_data(inputs) {
                Ok(nb) => {
                    model.err = None;
                    orders.perform_cmd({
                        async move {
                            match api_call.post(nb).await {
                                Ok(s) if s.status().is_ok() => Submitted(Ok(())),
                                _ => Submitted(Err(PageError::Submit)),
                            }
                        }
                    });
                }
                Err(err) => model.err = Some(err),
            },
            Err(err) => model.err = Some(err),
        },
        Submitted(result) => match result {
            Ok(()) => {
                orders.send_msg(Fetch);
            }
            Err(err) => model.err = Some(err),
        },
    }
    log!(model);
}
