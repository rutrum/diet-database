use super::get_event_value;
use crate::api_call;
use crate::Msg as SuperMsg;
use chrono::naive::{NaiveDate, NaiveTime};
use diet_database::bowel::*;
use diet_database::Tabular;
use seed::{prelude::*, *};

use super::*;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<Bowel>, String>),
    FormUpdate(FormUpdateMsg),
    Delete(usize),
    Deleted(Result<(), String>),
    Submit,
    Submitted(Result<(), String>),
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
    Scale(String),
}

#[derive(Debug, Clone, Default)]
pub struct Model {
    bowels: Vec<Bowel>,
    form: Form,
    err_msg: String,
}

impl PageModel<Vec<Bowel>> for Model {
    fn data(&self) -> &Vec<Bowel> {
        &self.bowels
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
                label!["Scale: "],
                input![attrs!(At::Type => "Range", At::Min => 1, At::Max => 7)],
                ev(Ev::Change, |ev| Msg::FormUpdate(FormUpdateMsg::Scale(
                    get_event_value(ev)
                ))),
            ],
        ]
    }
}

#[derive(Debug, Clone, Default)]
pub struct Form {
    date: String,
    time: String,
    scale: String,
}

impl Form {
    fn to_new_bowel(&self) -> Result<NewBowel, &'static str> {
        let date =
            NaiveDate::parse_from_str(&self.date, "%Y-%m-%d").map_err(|_| "Wrong date format")?;
        let time = NaiveTime::parse_from_str(self.time.as_str(), "%H:%M").ok();
        let scale = self
            .scale
            .parse::<i8>()
            .map_err(|_| "Scale is not an 8 bit integer")?;

        Ok(NewBowel { date, time, scale })
    }
}

pub fn init() -> Model {
    Model {
        form: Form {
            scale: 7.to_string(),
            ..Default::default()
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
                    let bowels = api_call::bowel::get().await.unwrap_or_default();
                    Msg::Fetched(Ok(bowels))
                }
            });
        }
        Fetched(result) => match result {
            Ok(bowels) => model.bowels = bowels,
            Err(msg) => model.err_msg = msg,
        },
        FormUpdate(update_msg) => {
            use FormUpdateMsg::*;
            match update_msg {
                Date(s) => model.form.date = s,
                Time(s) => model.form.time = s,
                Scale(s) => model.form.scale = s,
            }
        }
        Delete(idx) => {
            let b = model.bowels[idx];
            orders.perform_cmd({
                async move {
                    match api_call::bowel::delete(b).await {
                        Ok(s) if s.status().is_ok() => Deleted(Ok(())),
                        _ => Deleted(Err("Error deleting on server".to_string())),
                    }
                }
            });
        }
        Deleted(result) => match result {
            Ok(()) => {
                orders.send_msg(Fetch);
            }
            Err(msg) => model.err_msg = msg,
        },
        Submit => match model.form.to_new_bowel() {
            Ok(nb) => {
                model.err_msg = String::new();
                orders.perform_cmd({
                    async move {
                        match api_call::bowel::post(nb).await {
                            Ok(s) if s.status().is_ok() => Submitted(Ok(())),
                            _ => Submitted(Err("Error submitting to server".to_string())),
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
            Err(msg) => model.err_msg = msg,
        },
    }
    log!(model);
}
