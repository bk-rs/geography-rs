//! [ISO 3166-1 alpha-2 - Wikipedia](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)

//
country_code! {
    length = 2;
    #[derive(Debug, Clone)]
    pub enum CountryCode {
        AD,
        AE,
        AF,
        AG,
        AI,
        AL,
        AM,
        AO,
        AQ,
        AR,
        AS,
        AT,
        AU,
        AW,
        AX,
        AZ,
        BA,
        BB,
        BD,
        BE,
        BF,
        BG,
        BH,
        BI,
        BJ,
        BL,
        BM,
        BN,
        BO,
        BQ,
        BR,
        BS,
        BT,
        BV,
        BW,
        BY,
        BZ,
        CA,
        CC,
        CD,
        CF,
        CG,
        CH,
        CI,
        CK,
        CL,
        CM,
        CN,
        CO,
        CR,
        CU,
        CV,
        CW,
        CX,
        CY,
        CZ,
        DE,
        DJ,
        DK,
        DM,
        DO,
        DZ,
        EC,
        EE,
        EG,
        EH,
        ER,
        ES,
        ET,
        FI,
        FJ,
        FK,
        FM,
        FO,
        FR,
        GA,
        GB,
        GD,
        GE,
        GF,
        GG,
        GH,
        GI,
        GL,
        GM,
        GN,
        GP,
        GQ,
        GR,
        GS,
        GT,
        GU,
        GW,
        GY,
        HK,
        HM,
        HN,
        HR,
        HT,
        HU,
        ID,
        IE,
        IL,
        IM,
        IN,
        IO,
        IQ,
        IR,
        IS,
        IT,
        JE,
        JM,
        JO,
        JP,
        KE,
        KG,
        KH,
        KI,
        KM,
        KN,
        KP,
        KR,
        KW,
        KY,
        KZ,
        LA,
        LB,
        LC,
        LI,
        LK,
        LR,
        LS,
        LT,
        LU,
        LV,
        LY,
        MA,
        MC,
        MD,
        ME,
        MF,
        MG,
        MH,
        MK,
        ML,
        MM,
        MN,
        MO,
        MP,
        MQ,
        MR,
        MS,
        MT,
        MU,
        MV,
        MW,
        MX,
        MY,
        MZ,
        NA,
        NC,
        NE,
        NF,
        NG,
        NI,
        NL,
        NO,
        NP,
        NR,
        NU,
        NZ,
        OM,
        PA,
        PE,
        PF,
        PG,
        PH,
        PK,
        PL,
        PM,
        PN,
        PR,
        PS,
        PT,
        PW,
        PY,
        QA,
        RE,
        RO,
        RS,
        RU,
        RW,
        SA,
        SB,
        SC,
        SD,
        SE,
        SG,
        SH,
        SI,
        SJ,
        SK,
        SL,
        SM,
        SN,
        SO,
        SR,
        SS,
        ST,
        SV,
        SX,
        SY,
        SZ,
        TC,
        TD,
        TF,
        TG,
        TH,
        TJ,
        TK,
        TL,
        TM,
        TN,
        TO,
        TR,
        TT,
        TV,
        TW,
        TZ,
        UA,
        UG,
        UM,
        US,
        UY,
        UZ,
        VA,
        VC,
        VE,
        VG,
        VI,
        VN,
        VU,
        WF,
        WS,
        YE,
        YT,
        ZA,
        ZM,
        ZW,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    use csv::Reader;

    #[test]
    fn test_country_code() {
        // Wikipedia
        let mut rdr = Reader::from_reader(
            include_str!("../../tests/ISO_3166-1_alpha-2/Officially_assigned_code_elements.csv")
                .as_bytes(),
        );

        let mut n = 0;
        for record in rdr.records() {
            let record = record.unwrap();
            let code = &record[0];
            assert_eq!(code.parse::<CountryCode>().unwrap().to_string(), code);
            n += 1;
        }

        assert_eq!(CountryCode::VARS.len(), n);

        // FromStr
        assert_eq!(
            "ZZ".parse::<CountryCode>().unwrap(),
            CountryCode::Other("ZZ".into())
        );
        assert_eq!(
            "x".parse::<CountryCode>().err().unwrap(),
            crate::error::ParseError::Invalid("x".into())
        );
        #[cfg(feature = "std")]
        {
            std::println!("{}", "x".parse::<CountryCode>().err().unwrap());
        }

        // PartialEq
        assert_eq!(CountryCode::US, CountryCode::US);
        assert_eq!(CountryCode::US, "US");

        match CountryCode::US {
            x if x == "US" => {}
            _ => panic!(),
        }

        #[cfg(feature = "std")]
        {
            // Hash
            let mut h = std::collections::HashSet::new();
            h.insert(CountryCode::US);
            h.insert(CountryCode::Other("US".into()));
            assert_eq!(h.len(), 1);
        }

        #[cfg(feature = "serde")]
        {
            #[derive(serde::Serialize, serde::Deserialize)]
            struct Foo {
                code: CountryCode,
            }

            assert_eq!(
                serde_json::from_str::<Foo>(r#"{"code":"US"}"#)
                    .unwrap()
                    .code,
                CountryCode::US
            );
            assert_eq!(
                serde_json::to_string(&Foo {
                    code: CountryCode::US
                })
                .unwrap(),
                r#"{"code":"US"}"#
            );
        }
    }
}
