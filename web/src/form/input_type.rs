use chrono::naive::{NaiveDate, NaiveTime};
use crate::page::{get_event_value, PageError};
use seed::{prelude::*, *};

use super::*;

pub enum FormMsg {
    UpdateValue(usize, String),
}

#[derive(Clone, Debug)]
pub enum InputType {
    Date,
    Time,
    TimeOption,
    Range(usize, usize),
    Int,
    Text,
    IntOption,
    DropDown(Vec<(i32, String)>),
    Float,
    FloatOption,
}

impl InputType {
    pub fn to_data(&self, s: &str) -> Result<InputData, PageError> {
        use InputType::*;
        match self {
            Date => NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map(InputData::Date)
                .map_err(|_| PageError::form("date")),
            Time => NaiveTime::parse_from_str(s, "%H:%M")
                .map(InputData::Time)
                .map_err(|_| PageError::form("time")),
            TimeOption => Ok(InputData::TimeOption(
                NaiveTime::parse_from_str(s, "%H:%M").ok(),
            )),
            Range(_, _) => s
                .parse::<i8>()
                .map(InputData::Byte)
                .map_err(|_| PageError::form("date")),
            Int => s
                .parse::<i32>()
                .map(InputData::Int)
                .map_err(|_| PageError::form("integer")),
            Float => s
                .parse::<f32>()
                .map(InputData::Float)
                .map_err(|_| PageError::form("float")),
            FloatOption => Ok(InputData::FloatOption(s.parse::<f32>().ok())),
            IntOption => Ok(InputData::IntOption(s.parse::<i32>().ok())),
            Text => Ok(InputData::Text(s.to_string())),
            DropDown(options) => {
                if options.iter().any(|(i, _)| i.to_string() == s) {
                    s.parse::<i32>()
                        .map(InputData::Int)
                        .map_err(|_| PageError::form("foreign key"))
                } else {
                    Err(PageError::form("foreign key"))
                }
            }
        }
    }

    pub fn view(&self, i: usize) -> Node<FormMsg> {
        use InputType::*;
        let attrs = match self {
            Date => attrs!(At::Type => "date"),
            Time | TimeOption => attrs!(At::Type => "time"),
            Range(min, max) => attrs!(At::Type => "range", At::Min => min, At::Max => max),
            Int | IntOption => attrs!(At::Type => "number"),
            Text | Float | FloatOption => attrs!(At::Type => "text"),
            DropDown(_) => attrs!(),
        };
        if let DropDown(options) = self {
            select![
                option![],
                options
                    .iter()
                    .map(|option| { option![attrs!(At::Value => option.0), &option.1] }),
                ev(Ev::Change, move |ev| FormMsg::UpdateValue(
                    i,
                    get_event_value(ev)
                )),
            ]
        } else {
            input![
                attrs,
                ev(Ev::Change, move |ev| FormMsg::UpdateValue(
                    i,
                    get_event_value(ev)
                )),
            ]
        }
    }
}
