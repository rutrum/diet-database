use crate::api_call::ApiCall;
use diet_database::metric::*;
use seed::{prelude::*, *};

use super::*;
use crate::form::*;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<Metric>, PageError>),
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
    metrics: Vec<Metric>,
    form: Form,
    err: Option<PageError>,
}

impl PageModel<Vec<Metric>, Msg> for Model {
    fn data(&self) -> &Vec<Metric> {
        &self.metrics
    }

    fn error(&self) -> Option<&PageError> {
        self.err.as_ref()
    }

    fn form_fields(&self) -> Vec<Node<Msg>> {
        self.form.view().map_msg(Msg::FormUpdate)
    }
}

impl FromInputData for NewMetric {
    fn from_input_data(inputs: Vec<InputData>) -> Result<Self, PageError> {
        Ok(NewMetric {
            date: inputs[0].try_date()?,
            time: inputs[1].try_time_option()?,
            body_fat: inputs[2].try_float_option()?,
            gut_circum: inputs[3].try_float_option()?,
            waist_circum: inputs[4].try_float_option()?,
            chest_circum: inputs[5].try_float_option()?,
            thigh_circum: inputs[6].try_float_option()?,
        })
    }
}

pub fn init() -> Model {
    Model {
        form: Form {
            inputs: vec![
                Input::new("Date", InputType::Date),
                Input::new("Time", InputType::TimeOption),
                Input::new("Body Fat", InputType::FloatOption),
                Input::new("Gut", InputType::FloatOption),
                Input::new("Waist", InputType::FloatOption),
                Input::new("Chest", InputType::FloatOption),
                Input::new("Thigh", InputType::FloatOption),
            ],
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
                    match ApiCall::Metric.get().await {
                        Ok(s) => Fetched(Ok(s)),
                        Err(_) => Fetched(Err(PageError::Load)),
                    }
                }
            });
        }
        Fetched(result) => match result {
            Ok(metrics) => model.metrics = metrics,
            Err(err) => model.err = Some(err),
        },
        FormUpdate(update_msg) => model.form.update(update_msg),
        Edit(idx) => {
            let b = model.metrics[idx];
            model.form.set_all(&vec![b].matrix()[0]);
        }
        Delete(idx) => {
            let b = model.metrics[idx];
            if confirm(b) {
                orders.perform_cmd({
                    async move {
                        match ApiCall::Metric.delete(b).await {
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
            Ok(inputs) => match NewMetric::from_input_data(inputs) {
                Ok(nb) => {
                    model.err = None;
                    orders.perform_cmd({
                        async move {
                            match ApiCall::Metric.post(nb).await {
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
