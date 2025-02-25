use anyhow::Result;
use reqwest::StatusCode;
use serde_json::Value;
use worker::Response;

use discord::embed::{OpenBuildings, OpenFloors, OpenRooms, OpenTimes};
use d1::SQLRes;

pub(crate) fn ordinal(n: i32) -> String {
    let suffix = match n % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    format!("{}{}", n, suffix)
}