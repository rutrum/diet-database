use seed::{prelude::*, *};
use diet_database::bowel::*;
//use diet_database::Tabular;

mod api_call;
mod page;

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        page: Page::Bowel(page::bowel::init()),
    }
}

struct Model {
    page: Page,
}

pub enum Page {
    Bowel(page::bowel::Model),
    Store(page::store::Model),
}

pub enum Msg {
    LoadPage(Page),
    BowelPageUpdate(page::bowel::Msg),
    StorePageUpdate(page::store::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::LoadPage(page) => model.page = page,
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
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        view_page_selector(model),
        view_page(model),
        /*
        button![
            "Load Bowels",
            ev(Ev::Click, |_| Msg::LoadBowels),
        ],
        button![
            "Add Bowel",
            ev(Ev::Click, |_| Msg::ShowAddForm(FormType::Bowel)),
        ],
        view_tabular(&model.bowels),
        model.form.as_ref().map(|form_type| {
            match form_type {
                FormType::Bowel => {
                    form::bowel::view(&model.bowel_form).map_msg(Msg::BowelFormUpdate)
                }
            }
        })
        */
    ]
}

fn view_page(model: &Model) -> Node<Msg> {
    match &model.page {
        Page::Bowel(model) => page::bowel::view(&model).map_msg(Msg::BowelPageUpdate),
        Page::Store(model) => page::store::view(&model).map_msg(Msg::StorePageUpdate),
    }
}

fn view_page_selector(model: &Model) -> Node<Msg> {
    nav![
        div![
            "Bowel",
            ev(Ev::Click, |_| Msg::LoadPage(Page::Bowel(page::bowel::init())))
        ],
        div![
            "Store",
            ev(Ev::Click, |_| Msg::LoadPage(Page::Store(page::store::init())))
        ],
    ]
}

/*
pub fn view_tabular<T: Tabular>(table: &T) -> Node<Msg> {
    let headers = table.headers();
    let matrix = table.matrix();
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
                    ev(Ev::Click, move |_| Msg::DeleteBowel(i)),
                ]
            ]
        }),
    ]
}
*/

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
