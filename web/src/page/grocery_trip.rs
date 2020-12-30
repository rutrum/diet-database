use super::get_event_value;
use crate::api_call::ApiCall;
use chrono::naive::{NaiveDate, NaiveTime};
use diet_database::grocery_trip::*;
use diet_database::store::Store;
use seed::{prelude::*, *};

use super::*;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<GroceryTrip>, String>),
    FetchedStores(Result<Vec<Store>, String>),
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

impl PageModel<Vec<GroceryTrip>> for Model {
    fn data(&self) -> &Vec<GroceryTrip> {
        &self.trips
    }

    fn error_msg(&self) -> &String {
        &self.err_msg
    }

    fn form_fields<T: 'static>(&self) -> Vec<Node<T>> {
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
                    self.stores.iter().map(|store| {
                        option![ attrs!(At::Value => store.id), &store.name ]
                    }),
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
    fn to_new_grocery_trip(&self) -> Result<NewGroceryTrip, &'static str> {
        let date =
            NaiveDate::parse_from_str(&self.date, "%Y-%m-%d").map_err(|_| "Wrong date format")?;
        let time = NaiveTime::parse_from_str(self.time.as_str(), "%H:%M").ok();
        let store_id = self.store_id.parse::<i32>().map_err(|_| "Please select a store")?;

        Ok(NewGroceryTrip { date, time, store_id })
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
                    let trips = ApiCall::GroceryTrip.get().await.unwrap_or_default();
                    Msg::Fetched(Ok(trips))
                }
            });
            orders.perform_cmd({
                async move {
                    let stores = ApiCall::Store.get().await.unwrap_or_default();
                    Msg::FetchedStores(Ok(stores))
                }
            });
        }
        Fetched(result) => match result {
            Ok(trips) => model.trips = trips,
            Err(msg) => model.err_msg = msg,
        },
        FetchedStores(result) => match result {
            Ok(stores) => model.stores = stores,
            Err(msg) => model.err_msg = msg,
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
        Submit => match model.form.to_new_grocery_trip() {
            Ok(nb) => {
                model.err_msg = String::new();
                orders.perform_cmd({
                    async move {
                        match ApiCall::GroceryTrip.post(nb).await {
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
