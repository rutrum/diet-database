use super::get_event_value;
use crate::api_call::ApiCall;
use diet_database::metric::*;
use seed::{prelude::*, *};

use super::*;
use crate::form::*;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<Metric>, PageError>),
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

pub enum FormUpdateMsg {
    Date(String),
    Time(String),
    Weight(String),
    BodyFat(String),
    GutCircum(String),
    WaistCircum(String),
    ChestCircum(String),
    ThighCircum(String),
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
        use InputData::*;
        let date = if let Date(d) = inputs[0] {
            d
        } else {
            return Err(PageError::form("date"));
        };
        let time = if let TimeOption(t) = inputs[1] {
            t
        } else {
            return Err(PageError::form("time"));
        };
        let weight = if let FloatOption(i) = inputs[2] {
            i
        } else {
            return Err(PageError::form("weight"));
        };
        let body_fat = if let FloatOption(i) = inputs[3] {
            i
        } else {
            return Err(PageError::form("body_fat"));
        };
        let gut_circum = if let FloatOption(i) = inputs[4] {
            i
        } else {
            return Err(PageError::form("gut_circum"));
        };
        let waist_circum = if let FloatOption(i) = inputs[5] {
            i
        } else {
            return Err(PageError::form("waist_circum"));
        };
        let chest_circum = if let FloatOption(i) = inputs[6] {
            i
        } else {
            return Err(PageError::form("chest_circum"));
        };
        let thigh_circum = if let FloatOption(i) = inputs[7] {
            i
        } else {
            return Err(PageError::form("thigh_circum"));
        };
        Ok(NewMetric {
            date,
            time,
            weight,
            body_fat,
            gut_circum,
            waist_circum,
            chest_circum,
            thigh_circum,
        })
    }
}

pub fn init() -> Model {
    Model {
        form: Form {
            inputs: vec![
                Input::new("Date", InputType::Date),
                Input::new("Time", InputType::TimeOption),
                Input::new("Weight", InputType::FloatOption),
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
        Delete(idx) => {
            let b = model.metrics[idx];
            orders.perform_cmd({
                async move {
                    match ApiCall::Metric.delete(b).await {
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
