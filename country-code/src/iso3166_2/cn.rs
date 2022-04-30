//! [ISO 3166-2:CN](https://en.wikipedia.org/wiki/ISO_3166-2:CN)

use crate::iso3166_1::alpha_2::CountryCode;

//
country_subdivision_code! {
    CountryCode, CountryCode::CN;

    #[derive(Debug, Clone)]
    pub enum CountrySubdivisionCode {
        AH,
        BJ,
        CQ,
        FJ,
        GD,
        GS,
        GX,
        GZ,
        HA,
        HB,
        HE,
        HI,
        HK,
        HL,
        HN,
        JL,
        JS,
        JX,
        LN,
        MO,
        NM,
        NX,
        QH,
        SC,
        SD,
        SH,
        SN,
        SX,
        TJ,
        TW,
        XJ,
        XZ,
        YN,
        ZJ,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    use csv::Reader;

    #[test]
    fn test_country_subdivision_code() {
        // Wikipedia
        let mut rdr = Reader::from_reader(include_str!("../../tests/ISO_3166-2/CN.csv").as_bytes());

        let mut n = 0;
        for record in rdr.records() {
            let record = record.unwrap();
            let code = &record[0];
            assert_eq!(
                code.parse::<CountrySubdivisionCode>().unwrap().to_string(),
                code
            );
            n += 1;
        }

        assert_eq!(CountrySubdivisionCode::COUNTRY_CODE, CountryCode::CN);

        assert_eq!(CountrySubdivisionCode::VARS.len(), n);

        // FromStr
        assert_eq!(
            "CN-ZZ".parse::<CountrySubdivisionCode>().unwrap(),
            CountrySubdivisionCode::Other("ZZ".into())
        );
        assert_eq!(
            "x-y".parse::<CountrySubdivisionCode>().err().unwrap(),
            crate::error::CountrySubdivisionCodeParseError::CountryCodeInvalid("x".into())
        );
        assert_eq!(
            "ZZ-y".parse::<CountrySubdivisionCode>().err().unwrap(),
            crate::error::CountrySubdivisionCodeParseError::CountryCodeMismatch("ZZ".into())
        );
        assert_eq!(
            "CN-".parse::<CountrySubdivisionCode>().err().unwrap(),
            crate::error::CountrySubdivisionCodeParseError::SubdivisionCodeMissing
        );
        assert_eq!(
            "CN-y".parse::<CountrySubdivisionCode>().err().unwrap(),
            crate::error::CountrySubdivisionCodeParseError::SubdivisionCodeInvalid("y".into())
        );

        // PartialEq
        assert_eq!(CountrySubdivisionCode::BJ, CountrySubdivisionCode::BJ);
        assert_eq!(CountrySubdivisionCode::BJ, "CN-BJ");

        match CountrySubdivisionCode::BJ {
            x if x == "CN-BJ" => {}
            _ => panic!(),
        }

        #[cfg(feature = "std")]
        {
            // Hash
            let mut h = std::collections::HashSet::new();
            h.insert(CountrySubdivisionCode::BJ);
            h.insert(CountrySubdivisionCode::Other("BJ".into()));
            assert_eq!(h.len(), 1);
        }

        #[cfg(feature = "serde")]
        {
            #[derive(serde::Serialize, serde::Deserialize)]
            struct Foo {
                code: CountrySubdivisionCode,
            }

            assert_eq!(
                serde_json::from_str::<Foo>(r#"{"code":"CN-BJ"}"#)
                    .unwrap()
                    .code,
                CountrySubdivisionCode::BJ
            );
            assert_eq!(
                serde_json::to_string(&Foo {
                    code: CountrySubdivisionCode::BJ
                })
                .unwrap(),
                r#"{"code":"CN-BJ"}"#
            );
        }
    }
}
