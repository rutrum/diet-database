use crate::page::{get_event_value, PageError};
use chrono::naive::{NaiveDate, NaiveTime};
use seed::{prelude::*, *};

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
        }
    }

    pub fn view(&self) -> Vec<Node<FormMsg>> {
        self
            .inputs
            .iter()
            .enumerate()
            .map(|(i, input)| input.view(i))
            .collect()
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
        Self {
            name: name.to_string(),
            value: String::new(),
            typ,
        }
    }

    fn set(&mut self, value: String) {
        self.value = value;
    }

    fn view(&self, i: usize) -> Node<FormMsg> {
        div![label![format!("{}:", self.name)], self.typ.view(i),]
    }

    fn get_data(&self) -> Result<InputData, PageError> {
        self.typ.to_data(&self.value)
    }
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
    DropDown(Vec<(i32, String)>)
}

impl InputType {
    fn to_data(&self, s: &str) -> Result<InputData, PageError> {
        use InputType::*;
        match self {
            Date => NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map(|d| InputData::Date(d))
                .map_err(|_| PageError::form("date")),
            Time => NaiveTime::parse_from_str(s, "%H:%M")
                .map(|d| InputData::Time(d))
                .map_err(|_| PageError::form("time")),
            TimeOption => Ok(InputData::TimeOption(
                NaiveTime::parse_from_str(s, "%H:%M").ok(),
            )),
            Range(_, _) => s
                .parse::<i8>()
                .map(|d| InputData::Byte(d))
                .map_err(|_| PageError::form("date")),
            Int => s
                .parse::<i32>()
                .map(|d| InputData::Int(d))
                .map_err(|_| PageError::form("integer")),
            IntOption => Ok(InputData::IntOption(s.parse::<i32>().ok())),
            Text => Ok(InputData::Text(s.to_string())),
            DropDown(options) => if options.iter().any(|(i, _)| i.to_string() == s) {
                s.parse::<i32>()
                    .map(|d| InputData::Int(d))
                    .map_err(|_| PageError::form("foreign key"))
            } else {
                Err(PageError::form("foreign key"))
            }
        }
    }

    fn view(&self, i: usize) -> Node<FormMsg> {
        use InputType::*;
        let attrs = match self {
            Date => attrs!(At::Type => "date"),
            Time | TimeOption => attrs!(At::Type => "time"),
            Range(min, max) => attrs!(At::Type => "range", At::Min => min, At::Max => max),
            Int | IntOption => attrs!(At::Type => "number"),
            Text => attrs!(At::Type => "text"),
            DropDown(options) => { attrs!() }
        };
        if let DropDown(options) = self {
            select![
                option![],
                options.iter()
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

pub enum InputData {
    Date(NaiveDate),
    Time(NaiveTime),
    TimeOption(Option<NaiveTime>),
    Byte(i8),
    Int(i32),
    IntOption(Option<i32>),
    Text(String),
}

pub enum FormMsg {
    UpdateValue(usize, String),
}

pub trait FromInputData {
    fn from_input_data(_: Vec<InputData>) -> Result<Self, PageError>
    where
        Self: Sized;
}
