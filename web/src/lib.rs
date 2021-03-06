use seed::{prelude::*, *};

mod api_call;
mod form;
mod page;
use page::{PageModel, PageMsg};

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.send_msg(Msg::BowelPageUpdate(page::bowel::Msg::load()));
    Model {
        page: Page::Bowel(page::bowel::init()),
    }
}

struct Model {
    page: Page,
}

#[derive(Clone, Copy, Debug)]
pub enum PageName {
    Bowel,
    Store,
    GroceryTrip,
    Metric,
    Weight,
    GroceryItem,
}

impl PageName {
    fn init_with_data(self) -> Page {
        use PageName::*;
        match self {
            Bowel => Page::Bowel(page::bowel::init()),
            Store => Page::Store(page::store::init()),
            GroceryTrip => Page::GroceryTrip(page::grocery_trip::init()),
            GroceryItem => Page::GroceryItem(page::grocery_item::init()),
            Metric => Page::Metric(page::metric::init()),
            Weight => Page::Weight(page::weight::init()),
        }
    }

    fn display_name(&self) -> String {
        use PageName::*;
        match self {
            Bowel => "Bowel Movements",
            Store => "Grocery Stores",
            GroceryTrip => "Grocery Trips",
            GroceryItem => "Grocery Items",
            Metric => "Body Metrics",
            Weight => "Weight",
        }
        .to_string()
    }
}

pub enum Page {
    Bowel(page::bowel::Model),
    Store(page::store::Model),
    GroceryTrip(page::grocery_trip::Model),
    GroceryItem(page::grocery_item::Model),
    Metric(page::metric::Model),
    Weight(page::weight::Model),
}

pub enum Msg {
    LoadPage(Page),
    BowelPageUpdate(page::bowel::Msg),
    StorePageUpdate(page::store::Msg),
    GroceryTripPageUpdate(page::grocery_trip::Msg),
    GroceryItemPageUpdate(page::grocery_item::Msg),
    MetricPageUpdate(page::metric::Msg),
    WeightPageUpdate(page::weight::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::LoadPage(page) => {
            match &page {
                Page::Bowel(_) => orders.send_msg(Msg::BowelPageUpdate(page::bowel::Msg::load())),
                Page::Store(_) => orders.send_msg(Msg::StorePageUpdate(page::store::Msg::load())),
                Page::GroceryTrip(_) => {
                    orders.send_msg(Msg::GroceryTripPageUpdate(page::grocery_trip::Msg::load()))
                }
                Page::GroceryItem(_) => {
                    orders.send_msg(Msg::GroceryItemPageUpdate(page::grocery_item::Msg::load()))
                }
                Page::Metric(_) => {
                    orders.send_msg(Msg::MetricPageUpdate(page::metric::Msg::load()))
                }
                Page::Weight(_) => {
                    orders.send_msg(Msg::WeightPageUpdate(page::weight::Msg::load()))
                }
            };
            model.page = page;
        }
        Msg::BowelPageUpdate(msg) => {
            if let Page::Bowel(model) = &mut model.page {
                page::bowel::update(msg, model, &mut orders.proxy(Msg::BowelPageUpdate));
            }
        }
        Msg::StorePageUpdate(msg) => {
            if let Page::Store(model) = &mut model.page {
                page::store::update(msg, model, &mut orders.proxy(Msg::StorePageUpdate));
            }
        }
        Msg::GroceryItemPageUpdate(msg) => {
            if let Page::GroceryItem(model) = &mut model.page {
                page::grocery_item::update(
                    msg,
                    model,
                    &mut orders.proxy(Msg::GroceryItemPageUpdate),
                );
            }
        }
        Msg::GroceryTripPageUpdate(msg) => {
            if let Page::GroceryTrip(model) = &mut model.page {
                page::grocery_trip::update(
                    msg,
                    model,
                    &mut orders.proxy(Msg::GroceryTripPageUpdate),
                );
            }
        }
        Msg::MetricPageUpdate(msg) => {
            if let Page::Metric(model) = &mut model.page {
                page::metric::update(msg, model, &mut orders.proxy(Msg::MetricPageUpdate));
            }
        }
        Msg::WeightPageUpdate(msg) => {
            if let Page::Weight(model) = &mut model.page {
                page::weight::update(msg, model, &mut orders.proxy(Msg::WeightPageUpdate));
            }
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![C!["content"], view_page_selector(model), view_page(model),]
}

fn view_page(model: &Model) -> Node<Msg> {
    match &model.page {
        Page::Bowel(model) => model.view().map_msg(Msg::BowelPageUpdate),
        Page::Store(model) => model.view().map_msg(Msg::StorePageUpdate),
        Page::GroceryTrip(model) => model.view().map_msg(Msg::GroceryTripPageUpdate),
        Page::GroceryItem(model) => model.view().map_msg(Msg::GroceryItemPageUpdate),
        Page::Metric(model) => model.view().map_msg(Msg::MetricPageUpdate),
        Page::Weight(model) => model.view().map_msg(Msg::WeightPageUpdate),
    }
}

fn view_page_selector(_model: &Model) -> Node<Msg> {
    let page_names = vec![
        PageName::Bowel,
        PageName::Store,
        PageName::GroceryTrip,
        PageName::GroceryItem,
        PageName::Metric,
        PageName::Weight,
    ];
    nav![
        C!["page-selector"],
        page_names.into_iter().map(|page_name| {
            div![
                page_name.display_name(),
                ev(Ev::Click, move |_| Msg::LoadPage(
                    page_name.init_with_data()
                ))
            ]
        }),
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
