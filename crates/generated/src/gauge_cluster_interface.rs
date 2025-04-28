use serde::{Deserialize, Serialize};

pub trait DataObject<'a>: serde::Serialize + serde::Deserialize<'a> {
    fn serilize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_bytes(slice: &'a [u8]) -> Self {
        serde_json::from_slice(slice).unwrap()
    }
}

pub trait GaugeClusterInterface {
    fn new() -> Self;
    fn get_rpm(&self) -> Rpm;
    fn get_speed(&self) -> Speed;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Speed {
    pub value: u16,
}

impl DataObject<'_> for Speed{}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rpm {
    pub value: u16,
}

impl DataObject<'_> for Rpm{}
