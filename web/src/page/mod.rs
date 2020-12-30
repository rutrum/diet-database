use seed::{prelude::*, *};
use diet_database::Tabular;

pub mod bowel;
pub mod store;

pub trait PageMsg {
    /// Returns the msg cooresponding to deleting the ith item
    fn delete(_: usize) -> Self;
    fn submit() -> Self;
}

pub trait PageModel<T: Tabular> {
    fn data(&self) -> &T;
    fn error_msg(&self) -> &String;
    fn form_fields<G: 'static + PageMsg>(&self) -> Vec<Node<G>>;

    fn view<G: 'static + PageMsg>(&self) -> Node<G> {
        div![
            C!["page"],
            self.view_form(),
            self.view_table(),
        ]
    }

    fn view_table<G: 'static + PageMsg>(&self) -> Node<G> {
        let headers = self.data().headers();
        let matrix = self.data().matrix();
        table![
            tr![headers.iter().map(|header| { th![header] }),],
            matrix.iter().enumerate().map(|(i, row)| {
                tr![
                    row.iter().map(|cell| { td![cell] }),
                    delete_button(i)
                ]
            }),
        ]
    }

    fn view_form<G: 'static + PageMsg>(&self) -> Node<G> {
        div![
            C!["form"],
            self.form_fields(),
            submit_button(),
            view_error_msg(&self.error_msg()),
        ]
    }
}

fn delete_button<T: 'static + PageMsg>(i: usize) -> Node<T> {
    button!["delete", ev(Ev::Click, move |_| T::delete(i)),]
}

fn submit_button<T: 'static + PageMsg>() -> Node<T> {
    button!["Submit", ev(Ev::Click, move |_| T::submit()),]
}

fn view_error_msg<T>(msg: &String) -> Node<T> {
    div![C!["error-msg"], msg]
}

pub fn get_event_value(ev: web_sys::Event) -> String {
    ev.prevent_default();
    ev.target()
        .unwrap()
        .unchecked_into::<web_sys::HtmlInputElement>()
        .value()
}
