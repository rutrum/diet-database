use seed::{prelude::*, *};
use diet_database::bowel::Bowel;
use diet_database::Tabular;

const API_URL: &'static str = "http://localhost:8000";

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

#[derive(Default)]
struct Model {
    bowels: Vec<Bowel>,
}

enum Msg {
    LoadBowels,
    FetchedBowels(Vec<Bowel>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::LoadBowels => {
            log!("Fetching bowels");
            orders.perform_cmd({
                async move {
                    let bowels = fetch_bowels().await.unwrap_or_default();
                    Msg::FetchedBowels(bowels)
                }
            });
        }
        Msg::FetchedBowels(bowels) => {
            model.bowels = bowels;
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["counter"],
        button![
            "Load Bowels",
            ev(Ev::Click, |_| Msg::LoadBowels),
        ],
        format!("{:?}", model.bowels),
        view_tabular(&model.bowels),
        model.bowels.headers(),
    ]
}

fn view_tabular<T: Tabular>(table: &T) -> Node<Msg> {
    let headers = table.headers();
    let matrix = table.matrix();
    table![
        tr![
            headers.iter().map(|header| {
                th![header]
            })
        ],
        matrix.iter().map(|row| {
            tr![
                row.iter().map(|cell| {
                    td![cell]
                })
            ]
        })
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

pub async fn fetch_bowels() -> fetch::Result<Vec<Bowel>> {
    fetch(format!("{}/bowel", API_URL))
        .await?
        .json()
        .await
}
