pub trait Tabular: std::default::Default {
    fn headers(&self) -> Vec<String>;
    fn matrix(&self) -> Vec<Vec<String>>;
}
