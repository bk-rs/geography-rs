// cargo expand --verbose --all-features --test macro_country_code

country_code::country_code! {
    length = 2;
    #[derive(Debug, Clone)]
    pub enum MyCountryCode {
        CN,
    }
}
