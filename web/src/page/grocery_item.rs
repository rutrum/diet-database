use crate::api_call::ApiCall;
use diet_database::grocery_item::*;
use diet_database::grocery_trip::GroceryTrip;
use seed::{prelude::*, *};

use super::*;
use crate::form::*;

pub enum Msg {
    Fetch,
    Fetched(Result<Vec<GroceryItem>, PageError>),
    FetchedTrips(Result<Vec<GroceryTrip>, PageError>),
    FormUpdate(FormMsg),
    Edit(usize),
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
    fn edit(i: usize) -> Self {
        Msg::Edit(i)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Model {
    trips: Vec<GroceryItem>,
    form: Form,
    err: Option<PageError>,
}

impl PageModel<Vec<GroceryItem>, Msg> for Model {
    fn data(&self) -> &Vec<GroceryItem> {
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
                Input::new("Trip", InputType::DropDown(vec![])),
                Input::new("Name", InputType::Text),
                Input::new("Amount", InputType::FloatOption),
                Input::new("Measure", InputType::TextOption),
            ],
        },
        ..Default::default()
    }
}

impl FromInputData for NewGroceryItem {
    fn from_input_data(inputs: Vec<InputData>) -> Result<Self, PageError> {
        Ok(NewGroceryItem {
            trip_id: inputs[0].try_int()?,
            name: inputs[1].try_text()?,
            amount: inputs[2].try_float_option()?,
            measure: inputs[3].try_text_option()?,
        })
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    use Msg::*;
    match msg {
        Fetch => {
            orders.perform_cmd({
                async move {
                    match ApiCall::GroceryItem.get().await {
                        Ok(s) => Fetched(Ok(s)),
                        Err(_) => Fetched(Err(PageError::Load)),
                    }
                }
            });
            orders.perform_cmd({
                async move {
                    match ApiCall::GroceryTrip.get().await {
                        Ok(s) => FetchedTrips(Ok(s)),
                        Err(_) => FetchedTrips(Err(PageError::Load)),
                    }
                }
            });
        }
        Fetched(Ok(trips)) => model.trips = trips,
        Fetched(Err(err)) => model.err = Some(err),
        FetchedTrips(Ok(trips)) => {
            model.form.inputs[0] = Input::new(
                "Trip",
                InputType::DropDown(
                    trips
                        .into_iter()
                        .map(|trip| (trip.id, trip.date.to_string()))
                        .collect(),
                ),
            )
        }
        FetchedTrips(Err(err)) => model.err = Some(err),
        FormUpdate(update_msg) => model.form.update(update_msg),
        Edit(idx) => {
            let b = model.trips[idx].clone();
            model.form.set_all(&vec![b].matrix()[0]);
        }
        Delete(idx) => {
            let b = model.trips[idx].clone();
            if confirm(b.clone()) {
                orders.perform_cmd({
                    async move {
                        match ApiCall::GroceryItem.delete(b).await {
                            Ok(s) if s.status().is_ok() => Deleted(Ok(())),
                            _ => Deleted(Err(PageError::Delete)),
                        }
                    }
                });
            }
        }
        Deleted(Ok(_)) => {
            orders.send_msg(Fetch);
        }
        Deleted(Err(err)) => model.err = Some(err),
        Submit => match model.form.get_input_data() {
            Ok(inputs) => match NewGroceryItem::from_input_data(inputs) {
                Ok(nb) => {
                    model.err = None;
                    orders.perform_cmd({
                        async move {
                            match ApiCall::GroceryItem.post(nb).await {
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
        Submitted(Ok(_)) => {
            orders.send_msg(Fetch);
        }
        Submitted(Err(err)) => model.err = Some(err),
    }
    log!(model);
}
