use seed::{prelude::*, *};

mod api_call;
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
}

impl PageName {
    fn init_with_data(self) -> Page {
        use PageName::*;
        match self {
            Bowel => Page::Bowel(page::bowel::init()),
            Store => Page::Store(page::store::init()),
        }
    }

    fn display_name(&self) -> String {
        use PageName::*;
        match self {
            Bowel => "Bowel Movements",
            Store => "Grocery Stores",
        }.to_string()
    }
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
        Msg::LoadPage(page) => {
            match &page {
                Page::Bowel(_) => orders.send_msg(Msg::BowelPageUpdate(page::bowel::Msg::load())),
                Page::Store(_) => orders.send_msg(Msg::StorePageUpdate(page::store::Msg::load())),
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
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![C!["content"], view_page_selector(model), view_page(model),]
}

fn view_page(model: &Model) -> Node<Msg> {
    match &model.page {
        Page::Bowel(model) => model.view().map_msg(Msg::BowelPageUpdate),
        Page::Store(model) => model.view().map_msg(Msg::StorePageUpdate),
    }
}

fn view_page_selector(_model: &Model) -> Node<Msg> {
    let page_names = vec![PageName::Bowel, PageName::Store];
    nav![
        C!["page-selector"],
        page_names.into_iter().map(|page_name| {
            div![
                page_name.display_name(),
                ev(Ev::Click, move |_| Msg::LoadPage(page_name.init_with_data()))
            ]
        }),
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
