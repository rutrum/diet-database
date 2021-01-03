use crate::api_call::ApiCall;
use diet_database::bowel::*;
use seed::{prelude::*, *};

use super::*;
use crate::form::*;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<Bowel>, PageError>),
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
    bowels: Vec<Bowel>,
    form: Form,
    err: Option<PageError>,
}

impl PageModel<Vec<Bowel>, Msg> for Model {
    fn data(&self) -> &Vec<Bowel> {
        &self.bowels
    }

    fn error(&self) -> Option<&PageError> {
        self.err.as_ref()
    }

    fn form_fields(&self) -> Vec<Node<Msg>> {
        self.form.view().map_msg(Msg::FormUpdate)
    }
}

pub fn init() -> Model {
    Model {
        form: Form {
            inputs: vec![
                Input::new("Date", InputType::Date),
                Input::new("Time", InputType::TimeOption),
                Input::with_initial("Scale", InputType::Range(1, 7), "4"),
            ],
        },
        ..Default::default()
    }
}

impl FromInputData for NewBowel {
    fn from_input_data(inputs: Vec<InputData>) -> Result<Self, PageError> {
        Ok(NewBowel {
            date: inputs[0].try_date()?,
            time: inputs[1].try_time_option()?,
            scale: inputs[2].try_byte()?,
        })
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
            Err(err) => model.err = Some(err),
        },
        FormUpdate(update_msg) => model.form.update(update_msg),
        Edit(idx) => {
            let b = model.bowels[idx];
            model.form.set_all(&vec![b].matrix()[0]);
        }
        Delete(idx) => {
            let b = model.bowels[idx];
            if confirm(b) {
                orders.perform_cmd({
                    async move {
                        match ApiCall::Bowel.delete(b).await {
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
            Ok(inputs) => match NewBowel::from_input_data(inputs) {
                Ok(nb) => {
                    model.err = None;
                    orders.perform_cmd({
                        async move {
                            match ApiCall::Bowel.post(nb).await {
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
