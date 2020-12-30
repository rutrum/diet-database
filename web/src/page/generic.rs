use super::get_event_value;
use crate::api_call;
use diet_database::store::*;
use diet_database::Tabular;
use seed::{prelude::*, *};

use super::{PageMsg, PageModel};

pub enum Msg<DATA: Tabular> {
    Fetch,
    Fetched(Result<DATA, String>),
    Delete(usize),
    Submit,
}

impl<DATA: Tabular> PageMsg for Msg<DATA> {
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
pub struct Model<DATA: Tabular> {
    data: DATA,
    err_msg: String,
}

pub fn init<DATA: Tabular>() -> Model<DATA> {
    Default::default()
}

pub fn update<DATA: 'static + Tabular>(msg: Msg<DATA>, model: &mut Model<DATA>, orders: &mut impl Orders<Msg<DATA>>) {
    use Msg::*;
    match msg {
        Fetch => {
            orders.perform_cmd({
                async move {
                    let data = api_call::store::get().await.unwrap_or_default();
                    Msg::Fetched(Ok(data))
                }
            });
        }
        Fetched(result) => match result {
            Ok(data) => model.data = data,
            Err(msg) => model.err_msg = msg,
        },
        Delete(_) => {}
        Submit => {}
    }
    log!(model);
}
