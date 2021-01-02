use super::get_event_value;
use crate::api_call::ApiCall;
use diet_database::store::*;
use seed::{prelude::*, *};

use super::*;
use crate::form::*;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<Store>, PageError>),
    FormUpdate(FormMsg),
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

#[derive(Debug, Clone, Default)]
pub struct Model {
    stores: Vec<Store>,
    form: Form,
    err: Option<PageError>,
}

impl PageModel<Vec<Store>, Msg> for Model {
    fn data(&self) -> &Vec<Store> {
        &self.stores
    }
    fn error(&self) -> Option<&PageError> {
        self.err.as_ref()
    }

    fn form_fields(&self) -> Vec<Node<Msg>> {
        self.form.view().map_msg(Msg::FormUpdate)
    }
}

pub fn init() -> Model {
    Model {
        form: Form {
            inputs: vec![Input::new("Store name", InputType::Text)],
        },
        ..Default::default()
    }
}

impl FromInputData for NewStore {
    fn from_input_data(inputs: Vec<InputData>) -> Result<Self, PageError> {
        Ok(Self { 
            name: inputs[0].try_text()?,
        })
    }
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
            Err(err) => model.err = Some(err),
        },
        FormUpdate(update_msg) => model.form.update(update_msg),
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
            Err(err) => model.err = Some(err),
        },
        Submit => match model.form.get_input_data() {
            Ok(inputs) => match NewStore::from_input_data(inputs) {
                Ok(nb) => {
                    model.err = None;
                    orders.perform_cmd({
                        async move {
                            match ApiCall::Store.post(nb).await {
                                Ok(s) if s.status().is_ok() => Submitted(Ok(())),
                                _ => Submitted(Err(PageError::Submit)),
                            }
                        }
                    });
                }
                Err(err) => model.err = Some(err),
            },
            Err(err) => model.err = Some(err),
        },
        Submitted(result) => match result {
            Ok(()) => {
                orders.send_msg(Fetch);
            }
            Err(err) => model.err = Some(err),
        },
    }
    log!(model);
}
