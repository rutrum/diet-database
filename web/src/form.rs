use crate::page::{get_event_value, PageError};
use chrono::naive::{NaiveDate, NaiveTime};
use seed::{prelude::*, *};

struct Form {
    inputs: Vec<Input>,
}

impl Form {
    fn update(&mut self, msg: FormMsg) {
        match msg {
            FormMsg::UpdateValue(i, s) => {
                self.inputs[i].set(s);
            }
        }
    }

    fn view(&self) -> Node<FormMsg> {
        div![self
            .inputs
            .iter()
            .enumerate()
            .map(|(i, input)| input.view(i))]
    }

    fn get_input_data(&self) -> Result<Vec<InputData>, PageError> {
        self.inputs.iter().map(|input| input.get_data()).collect() // shouldn't work?!?!?
    }
}

struct Input {
    name: String,
    value: String,
    nullable: bool,
    typ: InputType,
}

impl Input {
    fn new(name: String, nullable: bool, typ: InputType) -> Self {
        Self {
            name,
            value: String::new(),
            typ,
            nullable,
        }
    }

    fn set(&mut self, value: String) {
        self.value = value;
    }

    fn view(&self, i: usize) -> Node<FormMsg> {
        div![label![&self.name], self.typ.view(i),]
    }

    fn get_data(&self) -> Result<InputData, PageError> {
        self.typ.to_data(&self.value)
    }
}

#[derive(Clone, Copy, Debug)]
enum InputType {
    Date,
    Time,
    TimeOption,
    Range(usize, usize),
    Int,
    IntOption,
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
                .map_err(|_| PageError::form("date")),
            IntOption => Ok(InputData::IntOption(s.parse::<i32>().ok())),
        }
    }

    fn view(&self, i: usize) -> Node<FormMsg> {
        use InputType::*;
        let attrs = match self {
            Date => attrs!(At::Type => "date"),
            Time | TimeOption => attrs!(At::Type => "time"),
            Range(min, max) => attrs!(At::Type => "range", At::Min => min, At::Max => max),
            Int | IntOption => attrs!(At::Type => "number"),
        };
        input![
            attrs,
            ev(Ev::Change, move |ev| FormMsg::UpdateValue(
                i,
                get_event_value(ev)
            )),
        ]
    }
}

enum InputData {
    Date(NaiveDate),
    Time(NaiveTime),
    TimeOption(Option<NaiveTime>),
    Byte(i8),
    Int(i32),
    IntOption(Option<i32>),
}

enum FormMsg {
    UpdateValue(usize, String),
}

pub trait FromInputData {
    fn from_input_data(_: Vec<InputData>) -> Result<Self, PageError>
    where
        Self: Sized;
}

use diet_database::bowel::NewBowel;
impl FromInputData for NewBowel {
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
        let scale = if let Byte(b) = inputs[2] {
            b
        } else {
            return Err(PageError::form("scale"));
        };
        Ok(NewBowel { date, time, scale })
    }
    /*
    fn to_new_bowel(&self) -> Result<NewBowel, PageError> {
        let date = parse_date_input(&self.date)?;
        let time = parse_time_input(&self.time).ok();
        let scale = self.scale.parse::<i8>().map_err(|_| PageError::form("scale"))?;

        Ok(NewBowel { date, time, scale })
    }
    */
}
