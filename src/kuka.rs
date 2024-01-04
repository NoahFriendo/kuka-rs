use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct KukaMessage {
    #[serde(rename = "@Type")]
    pub type_: String,
    #[serde(rename = "RIst")]
    pub r_ist: CartesianPosition, // Cartesian actual position
    #[serde(rename = "RSol")]
    pub r_sol: CartesianPosition, // Cartesian set position
    #[serde(rename = "AIPos")]
    pub ai_pos: AxisPosition, // Axis actual position
    #[serde(rename = "ASPos")]
    pub as_pos: AxisPosition, // Axis set position
    #[serde(rename = "Delay")]
    pub delay: Delay,
    #[serde(rename = "IPOC")]
    pub ipoc: i32,
}

#[derive(Debug, Serialize)]
pub struct KukaResponse {
    #[serde(rename = "@Type")]
    pub type_: String,
    #[serde(rename = "AK")]
    pub ak: AxisPosition,
    #[serde(rename = "Stop")]
    pub stop: i32,
    #[serde(rename = "IPOC")]
    pub ipoc: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CartesianPosition {
    #[serde(rename = "@X")]
    pub x: f64,
    #[serde(rename = "@Y")]
    pub y: f64,
    #[serde(rename = "@Z")]
    pub z: f64,
    #[serde(rename = "@A")]
    pub a: f64,
    #[serde(rename = "@B")]
    pub b: f64,
    #[serde(rename = "@C")]
    pub c: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AxisPosition {
    #[serde(rename = "@A1")]
    pub a1: f64,
    #[serde(rename = "@A2")]
    pub a2: f64,
    #[serde(rename = "@A3")]
    pub a3: f64,
    #[serde(rename = "@A4")]
    pub a4: f64,
    #[serde(rename = "@A5")]
    pub a5: f64,
    #[serde(rename = "@A6")]
    pub a6: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Delay {
    #[serde(rename = "@D")]
    pub d: i32,
}
