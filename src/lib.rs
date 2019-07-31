#![allow(non_snake_case)]

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IPData {
    #[serde(rename = "as")]
    pub asn: String,
    pub city: String,
    pub country: String,
    pub countryCode: String,
    pub isp: String,
    pub lat: f64,
    pub lon: f64,
    pub org: String,
    pub query: String,
    pub region: String,
    pub regionName: String,
    pub status: String,
    pub timezone: String,
    pub zip: String,
}

pub fn get() -> Result<IPData, Error> {
    let url = "http://ip-api.com/json";

    let json: IPData = reqwest::get(url)?.json()?;

    return Ok(json);
}
