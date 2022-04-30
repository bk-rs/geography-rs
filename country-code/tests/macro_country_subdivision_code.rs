// cargo expand --verbose --all-features --test macro_country_subdivision_code

use country_code::iso3166_1::alpha_2::CountryCode;

country_code::country_subdivision_code! {
    CountryCode, CountryCode::CN;

    #[derive(Debug, Clone)]
    pub enum MyCNCountrySubdivisionCode {
        BJ,
    }
}
