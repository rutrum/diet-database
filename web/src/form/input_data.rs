use chrono::naive::{NaiveDate, NaiveTime};
use crate::page::PageError;

pub enum InputData {
    Date(NaiveDate),
    Time(NaiveTime),
    TimeOption(Option<NaiveTime>),
    Byte(i8),
    Int(i32),
    IntOption(Option<i32>),
    Text(String),
    Float(f32),
    FloatOption(Option<f32>),
}

impl InputData {
    pub fn try_date(&self) -> Result<NaiveDate, PageError> {
        match self {
            InputData::Date(d) => Ok(*d),
            _ => Err(PageError::Developer),
        }
    }
    pub fn try_time(&self) -> Result<NaiveTime, PageError> {
        match self {
            InputData::Time(d) => Ok(*d),
            _ => Err(PageError::Developer),
        }
    }
    pub fn try_time_option(&self) -> Result<Option<NaiveTime>, PageError> {
        match self {
            InputData::TimeOption(d) => Ok(*d),
            _ => Err(PageError::Developer),
        }
    }
    pub fn try_byte(&self) -> Result<i8, PageError> {
        match self {
            InputData::Byte(d) => Ok(*d),
            _ => Err(PageError::Developer),
        }
    }
    pub fn try_int(&self) -> Result<i32, PageError> {
        match self {
            InputData::Int(d) => Ok(*d),
            _ => Err(PageError::Developer),
        }
    }
    pub fn try_int_option(&self) -> Result<Option<i32>, PageError> {
        match self {
            InputData::IntOption(d) => Ok(*d),
            _ => Err(PageError::Developer),
        }
    }
    pub fn try_text(&self) -> Result<String, PageError> {
        match self {
            InputData::Text(d) => Ok(d.clone()),
            _ => Err(PageError::Developer),
        }
    }
    pub fn try_float(&self) -> Result<f32, PageError> {
        match self {
            InputData::Float(d) => Ok(*d),
            _ => Err(PageError::Developer),
        }
    }
    pub fn try_float_option(&self) -> Result<Option<f32>, PageError> {
        match self {
            InputData::FloatOption(d) => Ok(*d),
            _ => Err(PageError::Developer),
        }
    }
}

pub trait FromInputData {
    fn from_input_data(_: Vec<InputData>) -> Result<Self, PageError>
    where
        Self: Sized;
}
