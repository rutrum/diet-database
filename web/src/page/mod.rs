use chrono::naive::{NaiveDate, NaiveTime};
use diet_database::Tabular;
use seed::{prelude::*, *};

pub mod bowel;
pub mod grocery_trip;
pub mod metric;
pub mod store;
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
        div![C!["page"], self.view_form(), self.view_table(),]
    }

    fn view_table<G: 'static + PageMsg>(&self) -> Node<G> {
        let headers = self.data().headers();
        let matrix = self.data().matrix();
        table![
            tr![headers.iter().map(|header| { th![header] }),],
            matrix
                .iter()
                .enumerate()
                .map(|(i, row)| { tr![row.iter().map(|cell| { td![cell] }), delete_button(i)] }),
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
    pub fn form(s: &str) -> PageError {
        PageError::Form(s.to_string())
    }
}

use std::fmt::{Display, Error, Formatter};

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
