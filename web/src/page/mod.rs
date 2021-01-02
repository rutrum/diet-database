use seed::{prelude::*, *};
use diet_database::Tabular;
use chrono::naive::{NaiveDate, NaiveTime};

pub mod bowel;
pub mod store;
pub mod grocery_trip;
pub mod metric;
//pub mod generic;

pub trait PageMsg {
    /// Returns the msg cooresponding to deleting the ith item
    fn delete(_: usize) -> Self;
    fn submit() -> Self;
    fn load() -> Self;
}

pub trait PageModel<T: Tabular> {
    fn data(&self) -> &T;
    fn error_msg(&self) -> &String;
    fn form_fields<G: 'static + PageMsg>(&self) -> Vec<Node<G>>;

    fn view<G: 'static + PageMsg>(&self) -> Node<G> {
        div![
            C!["page"],
            self.view_form(),
            self.view_table(),
        ]
    }

    fn view_table<G: 'static + PageMsg>(&self) -> Node<G> {
        let headers = self.data().headers();
        let matrix = self.data().matrix();
        table![
            tr![headers.iter().map(|header| { th![header] }),],
            matrix.iter().enumerate().map(|(i, row)| {
                tr![
                    row.iter().map(|cell| { td![cell] }),
                    delete_button(i)
                ]
            }),
        ]
    }

    fn view_form<G: 'static + PageMsg>(&self) -> Node<G> {
        div![
            C!["form"],
            self.form_fields(),
            submit_button(),
            view_error_msg(&self.error_msg()),
        ]
    }
}

fn delete_button<T: 'static + PageMsg>(i: usize) -> Node<T> {
    button!["delete", ev(Ev::Click, move |_| T::delete(i)),]
}

fn submit_button<T: 'static + PageMsg>() -> Node<T> {
    button!["Submit", ev(Ev::Click, move |_| T::submit()),]
}

fn view_error_msg<T>(msg: &String) -> Node<T> {
    div![C!["error-msg"], msg]
}

pub fn get_event_value(ev: web_sys::Event) -> String {
    ev.prevent_default();
    ev.target()
        .unwrap()
        .unchecked_into::<web_sys::HtmlInputElement>()
        .value()
}

struct Form {
    inputs: Vec<Input>
}

impl Form {
    fn view(&self) -> Node<FormMsg> {
        div![
            self.inputs.iter().enumerate().map(|(i, input)| input.view(i))
        ]
    }

    fn get_input_data(&self) -> Result<Vec<InputData>, PageError> {
        self.inputs.iter().map(|input| input.get_data()).collect() // shouldn't work?!?!?
    }
}

struct Input {
    name: String,
    value: String,
    typ: InputType,
}

impl Input {
    fn view(&self, i: usize) -> Node<FormMsg> {
        div![
            label![&self.name],
            self.typ.view(i),
        ]
    }

    fn get_data(&self) -> Result<InputData, PageError> {
        self.typ.to_data(&self.value)
    }
}

#[derive(Clone, Copy, Debug)]
enum InputType {
    Date,
    Time,
    Range(usize, usize),
    Int,
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
            Range(_, _) => s.parse::<i8>()
                         .map(|d| InputData::Byte(d))
                         .map_err(|_| PageError::form("date")),
            Int => s.parse::<i32>()
                         .map(|d| InputData::Int(d))
                         .map_err(|_| PageError::form("date")),
        }
    }

    fn view(&self, i: usize) -> Node<FormMsg> {
        use InputType::*;
        let attrs = match self {
            Date => attrs!(At::Type => "date"),
            Time => attrs!(At::Type => "time"),
            Range(min, max) => attrs!(At::Type => "range", At::Min => min, At::Max => max),
            Int => attrs!(At::Type => "number"),
        };
        input![
            attrs,
            ev(Ev::Change, move |ev| FormMsg::UpdateValue(i, get_event_value(ev))),
        ]
    }
}

enum InputData {
    Date(NaiveDate),
    Time(NaiveTime),
    Byte(i8),
    Int(i32),
}

enum FormMsg {
    UpdateValue(usize, String)
}

pub fn parse_date_input(s: &str) -> Result<NaiveDate, PageError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| PageError::form("date"))
}

pub fn parse_time_input(s: &str) -> Result<NaiveTime, PageError> {
    NaiveTime::parse_from_str(s, "%H:%M").map_err(|_| PageError::form("time"))
}

#[derive(Clone, Debug)]
pub enum PageError {
    Submit,
    Delete,
    Load,
    Form(String),
}

impl PageError {
    fn form(s: &str) -> PageError {
        PageError::Form(s.to_string())
    }
}

use std::fmt::{Display, Formatter, Error};

impl Display for PageError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use PageError::*;
        let s = match self {
            Submit => "Error submitting to api".to_string(),
            Delete => "Unable to delete item".to_string(),
            Load => "Cannot retrieve data".to_string(),
            Form(s) => format!("Field {} is invalid", s),
        };
        write!(f, "{}", s)
    }
}
