use chrono_tz::Tz;
use country_code::{CountryCode, iso3166_1::alpha_3::CountryCode as CountryCodeIso3};
use serde::Deserialize;

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    pub country_name: Box<str>,
    pub iso2: CountryCode,
    pub iso3: CountryCodeIso3,
    pub top_level_domain: Box<str>,
    pub fips: Box<str>,
    pub iso_mumeric: Box<str>,
    pub geo_name_id: Option<u32>,
    pub e164: u32,
    pub phone_code: Box<str>,
    pub continent: Box<str>,
    pub capital: Box<str>,
    #[serde(deserialize_with = "serde_field_with::from_str")]
    pub time_zone_in_capital: Tz,
    pub currency: Box<str>,
    pub language_codes: Box<str>,
    pub languages: Box<str>,
    pub area: u32,
    pub internet_hosts: Option<u32>,
    pub internet_users: Option<u32>,
    pub phones_mobile: Option<u32>,
    pub phones_landline: Option<u32>,
    pub gdp: Option<u64>,
}
