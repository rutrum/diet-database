use super::get_event_value;
use crate::api_call::ApiCall;
use diet_database::grocery_trip::*;
use diet_database::store::Store;
use seed::{prelude::*, *};

use super::*;
use crate::form::*;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<GroceryTrip>, PageError>),
    FetchedStores(Result<Vec<Store>, PageError>),
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
    err: Option<PageError>,
}

impl PageModel<Vec<GroceryTrip>, Msg> for Model {
    fn data(&self) -> &Vec<GroceryTrip> {
        &self.trips
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
            inputs: vec![
                Input::new("Date", InputType::Date),
                Input::new("Time", InputType::TimeOption),
                Input::new("Store", InputType::DropDown(vec![])),
            ]
        },
        ..Default::default()
    }
}

impl FromInputData for NewGroceryTrip {
    fn from_input_data(inputs: Vec<InputData>) -> Result<Self, PageError> {
        use InputData::*;
        let date = if let Date(d) = inputs[0] {
            d
        } else {
            return Err(PageError::form("date"));
        };
        let time = if let TimeOption(t) = inputs[1] {
            t
        } else {
            return Err(PageError::form("time"));
        };
        let store_id = if let Int(i) = inputs[2] {
            i
        } else {
            return Err(PageError::form("store id"));
        };
        Ok(NewGroceryTrip { date, time, store_id })
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
            Err(err) => model.err = Some(err),
        },
        FetchedStores(result) => match result {
            Ok(stores) => {
                model.form.inputs[2] = Input::new("Store", InputType::DropDown(
                    stores.into_iter().map(|store| (store.id, store.name.clone())).collect())
                )
            }
            Err(err) => model.err = Some(err),
        },
        FormUpdate(update_msg) => model.form.update(update_msg),
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
            Err(err) => model.err = Some(err),
        },
        Submit => match model.form.get_input_data() {
            Ok(inputs) => match NewGroceryTrip::from_input_data(inputs) {
                Ok(nb) => {
                    model.err = None;
                    orders.perform_cmd({
                        async move {
                            match ApiCall::GroceryTrip.post(nb).await {
                                Ok(s) if s.status().is_ok() => Submitted(Ok(())),
                                _ => Submitted(Err(PageError::Submit)),
                            }
                        }
                    });
                }
                Err(err) => model.err = Some(err),
            }
            Err(err) => model.err = Some(err),
        }
        Submitted(result) => match result {
            Ok(()) => {
                orders.send_msg(Fetch);
            }
            Err(err) => model.err = Some(err),
        },
    }
    log!(model);
}
