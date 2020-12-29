use seed::{prelude::*, *};
use chrono::naive::{NaiveDate, NaiveTime};
use crate::Msg as SuperMsg;
use diet_database::bowel::*;
use crate::api_call;
use super::get_event_value;

pub enum Msg {
    UpdateDate(String),
    UpdateTime(String),
    UpdateScale(String),
    UpdateErrorMsg(String),
    SubmitPoo,
}

#[derive(Debug, Clone, Default)]
pub struct Model {
    date: String,
    time: String,
    scale: String,
    err_msg: String,
}

pub fn init() -> Model {
    Model {
        scale: 3.to_string(),
        ..Default::default()
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<SuperMsg>) {
    use Msg::*;
    match msg {
        UpdateDate(s) => model.date = s,
        UpdateTime(s) => model.time = s,
        UpdateScale(s) => model.scale = s,
        UpdateErrorMsg(s) => model.err_msg = s,
        SubmitPoo => {
            log!("Submitting poo");
            match model.to_new_bowel() {
                Ok(nb) => {
                    model.err_msg = String::new();
                    orders.perform_cmd({
                        async move {
                            match api_call::bowel::post(nb).await {
                                Ok(s) if s.status().is_ok() => SuperMsg::SubmitBowelSuccess,
                                _ => SuperMsg::SubmitBowelFailure,
                            }
                        }
                    });
                }
                Err(err_msg) => model.err_msg = err_msg.to_string(),
            }
        }
    }
    log!(model);
}

impl Model {
    fn to_new_bowel(&self) -> Result<NewBowel, &'static str> {
        let date = NaiveDate::parse_from_str(&self.date, "%Y-%m-%d")
            .map_err(|_| "Wrong date format")?;
        let time = NaiveTime::parse_from_str(self.time.as_str(), "%H:%M").ok();
        let scale = self.scale.parse::<i8>().map_err(|_| "Scale is not an 8 bit integer")?;

        Ok(NewBowel{ date, time, scale })
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            label![ "Date of poo: " ],
            input![ attrs!(At::Type => "Date") ],
            ev(Ev::Change, |ev| Msg::UpdateDate(get_event_value(ev))),
        ],
        div![
            label![ "Time of poo: " ],
            input![ attrs!(At::Type => "Time") ],
            ev(Ev::Change, |ev| Msg::UpdateTime(get_event_value(ev))),
        ],
        div![
            label![ "Scale of poo: " ],
            input![ attrs!(At::Type => "Range", At::Min => 1, At::Max => 7) ],
            ev(Ev::Change, |ev| Msg::UpdateScale(get_event_value(ev))),
        ],
        button![
            "Submit Poo",
            ev(Ev::Click, |_| Msg::SubmitPoo),
        ],
        &model.err_msg,
    ]
}
