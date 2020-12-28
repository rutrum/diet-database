pub trait Tabular {
    fn headers(&self) -> Vec<String>;
    fn matrix(&self) -> Vec<Vec<String>>;
}
