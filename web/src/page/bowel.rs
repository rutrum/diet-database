use super::get_event_value;
use crate::api_call::ApiCall;
use chrono::naive::{NaiveDate, NaiveTime};
use diet_database::bowel::*;
use seed::{prelude::*, *};

use super::*;
use crate::form::*;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<Bowel>, PageError>),
    FormUpdate(FormMsg),
    Delete(usize),
    Deleted(Result<(), PageError>),
    Submit,
    Submitted(Result<(), PageError>),
}

impl PageMsg for Msg {
    fn delete(i: usize) -> Self {
        Msg::Delete(i)
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
    bowels: Vec<Bowel>,
    form: Form,
    err_msg: String,
}

impl PageModel<Vec<Bowel>, Msg> for Model {
    fn data(&self) -> &Vec<Bowel> {
        &self.bowels
    }

    fn error_msg(&self) -> &String {
        &self.err_msg
    }

    fn form_fields(&self) -> Vec<Node<Msg>> {
        nodes![
            vec![self.form.view().map_msg(Msg::FormUpdate)],
        ]
    }
}

pub fn init() -> Model {
    Model {
        form: Form {
            inputs: vec![
                Input::new("Date", InputType::Date),
                Input::new("Time", InputType::TimeOption),
                Input::new("Scale", InputType::Range(1, 7)),
            ]
        },
        ..Default::default()
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    use Msg::*;
    match msg {
        Fetch => {
            orders.perform_cmd({
                async move {
                    match ApiCall::Bowel.get().await {
                        Ok(s) => Fetched(Ok(s)),
                        Err(_) => Fetched(Err(PageError::Load)),
                    }
                }
            });
        }
        Fetched(result) => match result {
            Ok(bowels) => model.bowels = bowels,
            Err(msg) => model.err_msg = msg.to_string(),
        },
        FormUpdate(update_msg) => {
            model.form.update(update_msg);
        }
        Delete(idx) => {
            let b = model.bowels[idx];
            orders.perform_cmd({
                async move {
                    match ApiCall::Bowel.delete(b).await {
                        Ok(s) if s.status().is_ok() => Deleted(Ok(())),
                        _ => Deleted(Err(PageError::Delete)),
                    }
                }
            });
        }
        Deleted(result) => match result {
            Ok(()) => {
                orders.send_msg(Fetch);
            }
            Err(msg) => model.err_msg = msg.to_string(),
        },
        Submit => {
            match model.form.get_input_data() {
                Ok(inputs) => {
                    match NewBowel::from_input_data(inputs) {
                        Ok(nb) => {
                            model.err_msg = String::new();
                            orders.perform_cmd({
                                async move {
                                    match ApiCall::Bowel.post(nb).await {
                                        Ok(s) if s.status().is_ok() => Submitted(Ok(())),
                                        _ => Submitted(Err(PageError::Submit)),
                                    }
                                }
                            });
                        }
                        Err(err_msg) => model.err_msg = err_msg.to_string(),
                    }
                }
                Err(e) => model.err_msg = e.to_string(),
            }
        },
        Submitted(result) => match result {
            Ok(()) => {
                orders.send_msg(Fetch);
            }
            Err(msg) => model.err_msg = msg.to_string(),
        },
    }
    log!(model);
}
