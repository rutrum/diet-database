use super::get_event_value;
use crate::api_call::ApiCall;
use diet_database::store::*;
use seed::{prelude::*, *};

use super::*;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<Store>, PageError>),
    FormUpdate(FormUpdateMsg),
    Delete(usize),
    Deleted(Result<(), PageError>),
    Submit,
    Submitted(Result<(), PageError>),
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
    Name(String),
}

#[derive(Debug, Clone, Default)]
pub struct Model {
    stores: Vec<Store>,
    form: Form,
    err_msg: String,
}

impl PageModel<Vec<Store>, Msg> for Model {
    fn data(&self) -> &Vec<Store> {
        &self.stores
    }

    fn error_msg(&self) -> &String {
        &self.err_msg
    }

    fn form_fields(&self) -> Vec<Node<Msg>> {
        nodes![div![
            label!["Name of store:"],
            input![attrs!(At::Type => "text")],
            ev(Ev::Change, |ev| Msg::FormUpdate(FormUpdateMsg::Name(
                get_event_value(ev)
            ))),
        ],]
    }
}

#[derive(Debug, Clone, Default)]
pub struct Form {
    name: String,
}

impl Form {
    fn to_new_store(&self) -> Result<NewStore, PageError> {
        if self.name.is_empty() {
            Err(PageError::form("name"))
        } else {
            Ok(NewStore {
                name: self.name.clone(),
            })
        }
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
                    let stores = ApiCall::Store.get().await.unwrap_or_default();
                    Msg::Fetched(Ok(stores))
                }
            });
        }
        Fetched(result) => match result {
            Ok(stores) => model.stores = stores,
            Err(msg) => model.err_msg = msg.to_string(),
        },
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
                    match ApiCall::Store.delete(s).await {
                        Ok(s) if s.status().is_ok() => Deleted(Ok(())),
                        _ => Deleted(Err(PageError::Delete)),
                    }
                }
            });
        }
        Deleted(result) => match result {
            Ok(()) => {
                orders.send_msg(Fetch);
            }
            Err(msg) => model.err_msg = msg.to_string(),
        },
        Submit => match model.form.to_new_store() {
            Ok(nb) => {
                model.err_msg = String::new();
                orders.perform_cmd({
                    async move {
                        match ApiCall::Store.post(nb).await {
                            Ok(s) if s.status().is_ok() => Submitted(Ok(())),
                            _ => Submitted(Err(PageError::Submit)),
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
            Err(msg) => model.err_msg = msg.to_string(),
        },
    }
    log!(model);
}
