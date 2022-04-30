// cargo expand --verbose --all-features --test macro_continent_code

continent_code::continent_code! {
    #[derive(Debug, Clone)]
    pub enum MyContinentCode {
        AS,
    }
}
