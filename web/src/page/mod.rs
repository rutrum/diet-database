use seed::prelude::*;

pub mod bowel;
pub mod store;

pub trait FormView<T> {
    fn err_msg(&self) -> String;
    fn form_view(&self) -> Node<T>;
}

pub fn get_event_value(ev: web_sys::Event) -> String {
    ev.prevent_default();
    ev.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value()
}
