use seed::{prelude::*, *};
use crate::Msg as SuperMsg;
use crate::api_call;
use super::get_event_value;
use diet_database::store::NewStore;

pub enum Msg {
    UpdateName(String),
    UpdateErrMsg(String),
    Submit,
}

#[derive(Debug, Clone, Default)]
pub struct Model {
    name: String,
    err_msg: String,
}

impl Model {
    fn to_new_store(&self) -> Result<NewStore, &'static str> {
        Ok(NewStore{ name: self.name.clone() })
    }
}

pub fn init() -> Model {
    Model {
        ..Default::default()
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<SuperMsg>) {
    use Msg::*;
    match msg {
        UpdateName(s) => model.name = s,
        UpdateErrMsg(s) => model.err_msg = s,
        Submit => {
            
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            label![ "name of store: " ],
            input![ attrs!(At::Type => "text") ],
            ev(Ev::Change, |ev| Msg::UpdateName(get_event_value(ev))),
        ],
        button![
            "Submit",
            ev(Ev::Click, |_| Msg::Submit),
        ],
        &model.err_msg,
    ]
}
