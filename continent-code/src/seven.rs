continent_code! {
    #[derive(Debug, Clone)]
    pub enum ContinentCode {
        AS => "Asia",
        AF => "Africa",
        NA => "North America",
        SA => "South America",
        AN => "Antarctica",
        EU => "Europe",
        OC | AU => "Oceania",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    use csv::ReaderBuilder;

    #[test]
    fn test_continent_code() {
        // Wikipedia
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(include_str!("../tests/Continent/Area_and_population.csv").as_bytes());

        let mut n = 0;
        for record in rdr.records().skip(2) {
            let record = record.unwrap();
            let en_name = &record[0];
            assert_eq!(en_name.parse::<ContinentCode>().unwrap().en_name(), en_name);
            n += 1;
        }

        assert_eq!("AU".parse::<ContinentCode>().unwrap().to_string(), "OC");

        assert_eq!(ContinentCode::VARS.len(), n);

        assert_eq!(ContinentCode::from_en_name("Asia"), Some(ContinentCode::AS));
        assert_eq!(ContinentCode::AS.en_name(), "Asia");

        // PartialEq
        assert_eq!(ContinentCode::AS, ContinentCode::AS);
        assert_eq!(ContinentCode::AS, "AS");

        #[cfg(feature = "serde")]
        {
            #[derive(serde::Serialize, serde::Deserialize)]
            struct Foo {
                code: ContinentCode,
            }

            assert_eq!(
                serde_json::from_str::<Foo>(r#"{"code":"AS"}"#)
                    .unwrap()
                    .code,
                ContinentCode::AS
            );
            assert_eq!(
                serde_json::to_string(&Foo {
                    code: ContinentCode::AS
                })
                .unwrap(),
                r#"{"code":"AS"}"#
            );
        }
    }
}
