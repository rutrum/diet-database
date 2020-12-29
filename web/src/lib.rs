use seed::{prelude::*, *};
use diet_database::bowel::*;
//use diet_database::Tabular;

mod api_call;
mod form;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        page: Page::Bowel(form::bowel::init()),
    }
}

struct Model {
    page: Page,
}

pub enum FormType {
    Bowel,
}

pub enum Page {
    Bowel(form::bowel::Model),
//    Store(form::store::Model),
}

pub enum Msg {
    LoadPage(Page),
    BowelPageUpdate(form::bowel::Msg),
}

/*
pub enum Msg {
    LoadBowels,
    FetchedBowels(Vec<Bowel>),
    ShowAddForm(FormType),
    BowelFormUpdate(form::bowel::Msg),
    SubmitBowelSuccess,
    SubmitBowelFailure,
    DeleteBowel(usize),
    DeleteBowelSuccess,
    DeleteBowelFailure,
    InitPage(Page),
}
*/

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::LoadPage(page) => match page {
            Page::Bowel(model) => {}
        }
        Msg::BowelPageUpdate(msg) => {
            if let Page::Bowel(model) = &mut model.page {
                form::bowel::update(msg, model, &mut orders.proxy(Msg::BowelPageUpdate));
            }
        }
        /*
        Msg::InitPage(page) => {
            orders.send_msg(Msg::LoadBowels);
        }
        Msg::LoadBowels => {
            log!("Fetching bowels");
            orders.perform_cmd({
                async move {
                    let bowels = api_call::bowel::get().await.unwrap_or_default();
                    Msg::FetchedBowels(bowels)
                }
            });
        }
        Msg::FetchedBowels(bowels) => {
            model.bowels = bowels;
        }
        Msg::ShowAddForm(form_type) => {
            model.form = Some(form_type);
        }
        Msg::BowelFormUpdate(msg) => {
            form::bowel::update(msg, &mut model.bowel_form, orders);
        }
        Msg::SubmitBowelSuccess => {
            model.form = None;
            orders.send_msg(Msg::LoadBowels);
        }
        Msg::SubmitBowelFailure => {
            let msg = form::bowel::Msg::UpdateErrorMsg("Failed to submit to database".to_string());
            form::bowel::update(msg, &mut model.bowel_form, orders);
        }
        Msg::DeleteBowel(idx) => {
            log!("deleting bowel");
            let b = model.bowels[idx];
            orders.perform_cmd({
                async move {
                    match api_call::bowel::delete(b).await {
                        Ok(s) if s.status().is_ok() => Msg::DeleteBowelSuccess,
                        _ => Msg::DeleteBowelFailure,
                    }
                }
            });
        }
        Msg::DeleteBowelSuccess => {
            orders.send_msg(Msg::LoadBowels);
        }
        Msg::DeleteBowelFailure => log!("Failed to delete!"),
        */
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
        Page::Bowel(model) => form::bowel::view(&model).map_msg(Msg::BowelPageUpdate),
    }
}

fn view_page_selector(model: &Model) -> Node<Msg> {
    nav![
        div![
            "Bowel",
            ev(Ev::Click, |_| Msg::LoadPage(Page::Bowel(form::bowel::init())))
        ],
        /*
        div![
            "Store",
            ev(Ev::Click, |_| Msg::InitPage(Page::Store(form::store::init())))
        ],
        */
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
