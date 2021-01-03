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
    fn edit(_: usize) -> Self;
    fn submit() -> Self;
    fn load() -> Self;
}

pub trait PageModel<T: Tabular, M: 'static + PageMsg> {
    fn data(&self) -> &T;
    fn form_fields(&self) -> Vec<Node<M>>;
    fn error(&self) -> Option<&PageError>;

    fn view(&self) -> Node<M> {
        div![C!["page"], self.view_form(), self.view_table(),]
    }

    fn view_table(&self) -> Node<M> {
        let headers = self.data().headers();
        let matrix = self.data().matrix();
        table![
            tr![headers.iter().map(|header| { th![header] }),],
            matrix
                .iter()
                .enumerate()
                .map(|(i, row)| { tr![row.iter().map(|cell| { td![cell] }), edit_button(i), delete_button(i)] }),
        ]
    }

    fn view_form(&self) -> Node<M> {
        div![
            C!["form"],
            self.form_fields(),
            submit_button(),
            view_error_msg(self.error()),
        ]
    }
}

fn confirm<T>(item: T) -> bool
where
    Vec<T>: Tabular,
{
    let items = vec![item];
    let info = items
        .headers()
        .iter()
        .zip(items.matrix()[0].iter())
        .map(|(head, val)| format!("{}: {}", head, val))
        .collect::<Vec<String>>()
        .join("\n");
    web_sys::window()
        .map(|x| {
            x.confirm_with_message(&format!(
                "Are you sure you want to delete this item?\n{}",
                info
            ))
            .ok()
        })
        .flatten()
        .unwrap()
}

fn delete_button<T: 'static + PageMsg>(i: usize) -> Node<T> {
    button!["delete", ev(Ev::Click, move |_| T::delete(i)),]
}

fn edit_button<T: 'static + PageMsg>(i: usize) -> Node<T> {
    button!["edit", ev(Ev::Click, move |_| T::edit(i)),]
}

fn submit_button<T: 'static + PageMsg>() -> Node<T> {
    button!["Submit", ev(Ev::Click, move |_| T::submit()),]
}

fn view_error_msg<T>(error: Option<&PageError>) -> Node<T> {
    div![C!["error-msg"], error.map(|x| x.to_string())]
}

pub fn get_event_value(ev: web_sys::Event) -> String {
    ev.prevent_default();
    ev.target()
        .unwrap()
        .unchecked_into::<web_sys::HtmlInputElement>()
        .value()
}

#[derive(Clone, Debug)]
pub enum PageError {
    Submit,
    Delete,
    Load,
    Form(String),
    Developer,
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
            Developer => "The developer made a mistake!".to_string(),
        };
        write!(f, "{}", s)
    }
}
