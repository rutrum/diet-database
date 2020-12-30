use super::get_event_value;
use crate::api_call;
use diet_database::store::*;
use diet_database::Tabular;
use seed::{prelude::*, *};

use super::{PageMsg, PageModel};

/*
pub enum GMsg<DATA, MSG> {
    Fetch,
    Fetched(Result<DATA, String>),
    Update(MSG),
    Delete(usize),
    Deleted(Result<(), String>),
    Submit,
    Submitted(Result<(), String>),
}

impl PageMsg for GMsg<Vec<Store>, Msg> {
    fn delete(i: usize) -> Self {
        GMsg::Delete(i)
    }
    fn submit() -> Self {
        GMsg::Submit
    }
    fn load() -> Self {
        GMsg::Fetch
    }
}

pub struct GModel<DATA: Tabular, FORM> {
    data: DATA,
    form: FORM,
    err_msg: String,
}

impl PageModel<Vec<Store>> for GModel<Vec<Store>, Form> {
    fn data(&self) -> &Vec<Store> {
        &self.data
    }
}
*/

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<Store>, String>),
    FormUpdate(FormUpdateMsg),
    Delete(usize),
    Deleted(Result<(), String>),
    Submit,
    Submitted(Result<(), String>),
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

impl PageModel<Vec<Store>> for Model {
    fn data(&self) -> &Vec<Store> {
        &self.stores
    }

    fn error_msg(&self) -> &String {
        &self.err_msg
    }

    fn form_fields<G: 'static + PageMsg>(&self) -> Vec<Node<G>> {
        nodes![
            div![
                label!["Name of store:"],
                input![attrs!(At::Type => "text")],
                ev(Ev::Change, |ev| Msg::FormUpdate(FormUpdateMsg::Name(
                    get_event_value(ev)
                ))),
            ],
        ]
    }

}

#[derive(Debug, Clone, Default)]
pub struct Form {
    name: String,
}

impl Form {
    fn to_new_store(&self) -> Result<NewStore, &'static str> {
        if self.name.is_empty() {
            Err("Name is required")
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
                    let stores = api_call::store::get().await.unwrap_or_default();
                    Msg::Fetched(Ok(stores))
                }
            });
        }
        Fetched(result) => match result {
            Ok(stores) => model.stores = stores,
            Err(msg) => model.err_msg = msg,
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
                    match api_call::store::delete(s).await {
                        Ok(s) if s.status().is_ok() => Deleted(Ok(())),
                        _ => Deleted(Err("Error deleting on server".to_string())),
                    }
                }
            });
        }
        Deleted(result) => match result {
            Ok(()) => {
                orders.send_msg(Fetch);
            }
            Err(msg) => model.err_msg = msg,
        },
        Submit => match model.form.to_new_store() {
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
        },
        Submitted(result) => match result {
            Ok(()) => {
                orders.send_msg(Fetch);
            }
            Err(msg) => model.err_msg = msg,
        },
    }
    log!(model);
}
