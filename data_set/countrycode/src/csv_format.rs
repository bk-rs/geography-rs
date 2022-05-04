use core::{fmt, ops::Deref};
use std::io::Read;

use csv::{Error as CsvError, Reader, StringRecord};

use crate::record::Record;

//
pub const CSV_HEADER: &[&str] = &[
    "country_name",
    "iso2",
    "iso3",
    "top_level_domain",
    "fips",
    "iso_mumeric",
    "geo_name_id",
    "e164",
    "phone_code",
    "continent",
    "capital",
    "time_zone_in_capital",
    "currency",
    "language_codes",
    "languages",
    "area",
    "internet_hosts",
    "internet_users",
    "phones_mobile",
    "phones_landline",
    "gdp",
];

//
#[cfg(feature = "once_cell")]
pub static RECORDS: once_cell::sync::Lazy<Records> = once_cell::sync::Lazy::new(|| {
    let csv = include_str!("../data/countrycode.csv");
    Records::from_csv(csv.as_bytes()).unwrap()
});

#[cfg(feature = "once_cell")]
pub static RECORDS_ISO2_MAP: once_cell::sync::Lazy<
    std::collections::HashMap<country_code::CountryCode, Record>,
> = once_cell::sync::Lazy::new(|| {
    RECORDS
        .iter()
        .cloned()
        .map(|x| (x.iso2.to_owned(), x))
        .collect()
});

//
#[derive(Debug, Clone)]
pub struct Records(pub Vec<Record>);

impl Deref for Records {
    type Target = Vec<Record>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//
impl Records {
    pub fn from_csv<R: Read>(rdr: R) -> Result<Self, RecordsFromCsvError> {
        let mut rdr = Reader::from_reader(rdr);

        let header = StringRecord::from(CSV_HEADER);

        let mut inner = vec![];

        for record in rdr.records() {
            let record = record.map_err(RecordsFromCsvError::CsvParseFailed)?;
            let row: Record = record
                .deserialize(Some(&header))
                .map_err(RecordsFromCsvError::RecordDeFailed)?;
            inner.push(row);
        }

        Ok(Self(inner))
    }
}

//
#[derive(Debug)]
pub enum RecordsFromCsvError {
    CsvParseFailed(CsvError),
    RecordDeFailed(CsvError),
}

impl fmt::Display for RecordsFromCsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RecordsFromCsvError {}
