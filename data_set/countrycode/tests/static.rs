#![cfg(feature = "once_cell")]

use country_code::CountryCode;
use countrycode::RECORDS_ISO2_MAP;

#[test]
fn test_static() {
    let record = RECORDS_ISO2_MAP.get(&CountryCode::US).unwrap();
    println!("{:?}", record);
    assert_eq!(record.iso3, "USA");
}
