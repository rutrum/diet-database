use super::get_event_value;
use crate::api_call::ApiCall;
use chrono::naive::{NaiveDate, NaiveTime};
use diet_database::grocery_trip::*;
use diet_database::store::Store;
use seed::{prelude::*, *};

use super::*;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<GroceryTrip>, PageError>),
    FetchedStores(Result<Vec<Store>, PageError>),
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
    Date(String),
    Time(String),
    StoreId(String),
}

#[derive(Debug, Clone, Default)]
pub struct Model {
    trips: Vec<GroceryTrip>,
    stores: Vec<Store>,
    form: Form,
    err_msg: String,
}

impl PageModel<Vec<GroceryTrip>, Msg> for Model {
    fn data(&self) -> &Vec<GroceryTrip> {
        &self.trips
    }

    fn error_msg(&self) -> &String {
        &self.err_msg
    }

    fn form_fields(&self) -> Vec<Node<Msg>> {
        nodes![
            div![
                label!["Date: "],
                input![attrs!(At::Type => "Date")],
                ev(Ev::Change, |ev| Msg::FormUpdate(FormUpdateMsg::Date(
                    get_event_value(ev)
                ))),
            ],
            div![
                label!["Time: "],
                input![attrs!(At::Type => "Time")],
                ev(Ev::Change, |ev| Msg::FormUpdate(FormUpdateMsg::Time(
                    get_event_value(ev)
                ))),
            ],
            div![
                label!["Store: "],
                select![
                    option![],
                    self.stores
                        .iter()
                        .map(|store| { option![attrs!(At::Value => store.id), &store.name] }),
                    ev(Ev::Change, |ev| Msg::FormUpdate(FormUpdateMsg::StoreId(
                        get_event_value(ev)
                    ))),
                ]
            ],
        ]
    }
}

#[derive(Debug, Clone, Default)]
pub struct Form {
    date: String,
    time: String,
    store_id: String,
}

impl Form {
    fn to_new_grocery_trip(&self) -> Result<NewGroceryTrip, PageError> {
        let date = parse_date_input(&self.date)?;
        let time = parse_time_input(&self.time).ok();
        let store_id = self
            .store_id
            .parse::<i32>()
            .map_err(|_| PageError::form("store id"))?;

        Ok(NewGroceryTrip {
            date,
            time,
            store_id,
        })
    }
}

pub fn init() -> Model {
    Model {
        ..Default::default()
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    use Msg::*;
    match msg {
        Fetch => {
            orders.perform_cmd({
                async move {
                    match ApiCall::GroceryTrip.get().await {
                        Ok(s) => Fetched(Ok(s)),
                        Err(_) => Fetched(Err(PageError::Load)),
                    }
                }
            });
            orders.perform_cmd({
                async move {
                    match ApiCall::Store.get().await {
                        Ok(s) => FetchedStores(Ok(s)),
                        Err(_) => FetchedStores(Err(PageError::Load)),
                    }
                }
            });
        }
        Fetched(result) => match result {
            Ok(trips) => model.trips = trips,
            Err(msg) => model.err_msg = msg.to_string(),
        },
        FetchedStores(result) => match result {
            Ok(stores) => model.stores = stores,
            Err(msg) => model.err_msg = msg.to_string(),
        },
        FormUpdate(update_msg) => {
            use FormUpdateMsg::*;
            match update_msg {
                Date(s) => model.form.date = s,
                Time(s) => model.form.time = s,
                StoreId(s) => model.form.store_id = s,
            }
        }
        Delete(idx) => {
            let b = model.trips[idx].clone();
            orders.perform_cmd({
                async move {
                    match ApiCall::GroceryTrip.delete(b).await {
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
        Submit => match model.form.to_new_grocery_trip() {
            Ok(nb) => {
                model.err_msg = String::new();
                orders.perform_cmd({
                    async move {
                        match ApiCall::GroceryTrip.post(nb).await {
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
