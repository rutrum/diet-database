use seed::{prelude::*, *};
use crate::Msg as SuperMsg;
use crate::api_call;
use super::get_event_value;
use diet_database::store::*;
use diet_database::Tabular;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<Store>, String>),
    FormUpdate(FormUpdateMsg),
    Delete(usize),
    Deleted(Result<(), String>),
    Submit,
    Submitted(Result<(), String>),
}

pub enum FormUpdateMsg {
    Name(String),
}

#[derive(Debug, Clone, Default)]
pub struct Model {
    stores: Vec<Store>,
    form: Form,
    err_msg: String,
}

#[derive(Debug, Clone, Default)]
pub struct Form {
    name: String,
}

impl Form {
    fn to_new_store(&self) -> Result<NewStore, &'static str> {
        Ok(NewStore{ name: self.name.clone() })
    }
}

pub fn init() -> Model {
    Default::default()
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    use Msg::*;
    match msg {
        Fetch => {
            log!("Fetching stores");
            orders.perform_cmd({
                async move {
                    let stores = api_call::store::get().await.unwrap_or_default();
                    Msg::Fetched(Ok(stores))
                }
            });
        }
        Fetched(result) => match result {
            Ok(stores) => model.stores = stores,
            Err(msg) => model.err_msg = msg,
        }
        FormUpdate(update_msg) => {
            use FormUpdateMsg::*;
            match update_msg {
                Name(s) => model.form.name = s,
            }
        }
        Delete(idx) => {
            log!("deleting store");
            let s = model.stores[idx].clone();
            orders.perform_cmd({
                async move {
                    match api_call::store::delete(s).await {
                        Ok(s) if s.status().is_ok() => Deleted(Ok(())),
                        _ => Deleted(Err("Error deleting on server".to_string())),
                    }
                }
            });
        }
        Deleted(result) => match result {
            Ok(()) => {orders.send_msg(Fetch); },
            Err(msg) => model.err_msg = msg,
        }
        Submit => {
            log!("Submitting poo");
            match model.form.to_new_store() {
                Ok(nb) => {
                    model.err_msg = String::new();
                    orders.perform_cmd({
                        async move {
                            match api_call::store::post(nb).await {
                                Ok(s) if s.status().is_ok() => Submitted(Ok(())),
                                _ => Submitted(Err("Error submitting to server".to_string())),
                            }
                        }
                    });
                }
                Err(err_msg) => model.err_msg = err_msg.to_string(),
            }
        }
        Submitted(result) => match result {
            Ok(()) => {orders.send_msg(Fetch); },
            Err(msg) => model.err_msg = msg,
        }
    }
    log!(model);
}

pub fn view(model: &Model) -> Node<Msg> {
    let headers = model.stores.headers();
    let matrix = model.stores.matrix();
    div![
        button![
            "Load Stores",
            ev(Ev::Click, |_| Msg::Fetch),
        ],
        view_form(),
        table![
            tr![
                headers.iter().map(|header| {
                    th![header]
                }),
            ],
            matrix.iter().enumerate().map(|(i, row)| {
                tr![
                    row.iter().map(|cell| {
                        td![cell]
                    }),
                    button![
                        "delete",
                        ev(Ev::Click, move |_| Msg::Delete(i)),
                    ]
                ]
            }),
        ]
    ]
}

pub fn view_form() -> Node<Msg> {
    div![
        div![
            label![ "Name of store:" ],
            input![ attrs!(At::Type => "text") ],
            ev(Ev::Change, |ev| Msg::FormUpdate(FormUpdateMsg::Name(get_event_value(ev)))),
        ],
        button![
            "Submit Poo",
            ev(Ev::Click, |_| Msg::Submit),
        ],
    ]
}
