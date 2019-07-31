#![allow(non_snake_case)]

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IPData {
    #[serde(rename = "as")] 
    asn: String,
    city: String,
    country: String,
    countryCode: String,
    isp: String,
    lat: f64,
    lon: f64,
    org: String,
    query: String,
    region: String,
    regionName: String,
    status: String,
    timezone: String,
    zip: String,
}

pub fn get() -> Result<IPData, Error> {
    let url = "http://ip-api.com/json";

    let json: IPData = reqwest::get(url)?.json()?;

    return Ok(json);
}