use super::get_event_value;
use crate::api_call::ApiCall;
use chrono::naive::{NaiveDate, NaiveTime};
use diet_database::metric::*;
use seed::{prelude::*, *};

use super::*;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<Metric>, PageError>),
    FormUpdate(FormUpdateMsg),
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
    err_msg: String,
}

impl PageModel<Vec<Metric>> for Model {
    fn data(&self) -> &Vec<Metric> {
        &self.metrics
    }

    fn error_msg(&self) -> &String {
        &self.err_msg
    }

    fn form_fields<T: 'static>(&self) -> Vec<Node<T>> {
        nodes![
            div![
                label!["Date: "],
                input![attrs!(At::Type => "Date")],
                ev(Ev::Change, |ev| Msg::FormUpdate(FormUpdateMsg::Date(
                    get_event_value(ev)
                ))),
            ],
            div![
                label!["Time: "],
                input![attrs!(At::Type => "Time")],
                ev(Ev::Change, |ev| Msg::FormUpdate(FormUpdateMsg::Time(
                    get_event_value(ev)
                ))),
            ],
            div![
                label!["Weight: "],
                input![attrs!(At::Type => "Text")],
                ev(Ev::Change, |ev| Msg::FormUpdate(FormUpdateMsg::Weight(
                    get_event_value(ev)
                ))),
            ],
            div![
                label!["Body Fat: "],
                input![attrs!(At::Type => "Text")],
                ev(Ev::Change, |ev| Msg::FormUpdate(FormUpdateMsg::BodyFat(
                    get_event_value(ev)
                ))),
            ],
            div![
                label!["Gut: "],
                input![attrs!(At::Type => "Text")],
                ev(Ev::Change, |ev| Msg::FormUpdate(FormUpdateMsg::GutCircum(
                    get_event_value(ev)
                ))),
            ],
            div![
                label!["Waist: "],
                input![attrs!(At::Type => "Text")],
                ev(Ev::Change, |ev| Msg::FormUpdate(
                    FormUpdateMsg::WaistCircum(get_event_value(ev))
                )),
            ],
            div![
                label!["Chest: "],
                input![attrs!(At::Type => "Text")],
                ev(Ev::Change, |ev| Msg::FormUpdate(
                    FormUpdateMsg::ChestCircum(get_event_value(ev))
                )),
            ],
            div![
                label!["Thigh: "],
                input![attrs!(At::Type => "Text")],
                ev(Ev::Change, |ev| Msg::FormUpdate(
                    FormUpdateMsg::ThighCircum(get_event_value(ev))
                )),
            ],
        ]
    }
}

#[derive(Debug, Clone, Default)]
pub struct Form {
    date: String,
    time: String,
    weight: String,
    body_fat: String,
    gut_circum: String,
    waist_circum: String,
    chest_circum: String,
    thigh_circum: String,
}

impl Form {
    fn to_new_metric(&self) -> Result<NewMetric, PageError> {
        let date = parse_date_input(&self.date)?;
        let time = parse_time_input(&self.time).ok();
        let weight = self.weight.parse::<f32>().ok();
        let body_fat = self.body_fat.parse::<f32>().ok();
        let gut_circum = self.gut_circum.parse::<f32>().ok();
        let waist_circum = self.waist_circum.parse::<f32>().ok();
        let chest_circum = self.chest_circum.parse::<f32>().ok();
        let thigh_circum = self.thigh_circum.parse::<f32>().ok();

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
            Err(msg) => model.err_msg = msg.to_string(),
        },
        FormUpdate(update_msg) => {
            use FormUpdateMsg::*;
            match update_msg {
                Date(s) => model.form.date = s,
                Time(s) => model.form.time = s,
                Weight(s) => model.form.weight = s,
                BodyFat(s) => model.form.body_fat = s,
                GutCircum(s) => model.form.gut_circum = s,
                WaistCircum(s) => model.form.waist_circum = s,
                ChestCircum(s) => model.form.chest_circum = s,
                ThighCircum(s) => model.form.thigh_circum = s,
            }
        }
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
            Err(msg) => model.err_msg = msg.to_string(),
        },
        Submit => match model.form.to_new_metric() {
            Ok(nb) => {
                model.err_msg = String::new();
                orders.perform_cmd({
                    async move {
                        match ApiCall::Metric.post(nb).await {
                            Ok(s) if s.status().is_ok() => Submitted(Ok(())),
                            _ => Submitted(Err(PageError::Submit)),
                        }
                    }
                });
            }
            Err(err_msg) => model.err_msg = err_msg.to_string(),
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
