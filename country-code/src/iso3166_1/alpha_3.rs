//! [ISO 3166-1 alpha-3 - Wikipedia](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3)

//
country_code! {
    length = 3;
    #[derive(Debug, Clone)]
    pub enum CountryCode {
        ABW,
        AFG,
        AGO,
        AIA,
        ALA,
        ALB,
        AND,
        ARE,
        ARG,
        ARM,
        ASM,
        ATA,
        ATF,
        ATG,
        AUS,
        AUT,
        AZE,
        BDI,
        BEL,
        BEN,
        BES,
        BFA,
        BGD,
        BGR,
        BHR,
        BHS,
        BIH,
        BLM,
        BLR,
        BLZ,
        BMU,
        BOL,
        BRA,
        BRB,
        BRN,
        BTN,
        BVT,
        BWA,
        CAF,
        CAN,
        CCK,
        CHE,
        CHL,
        CHN,
        CIV,
        CMR,
        COD,
        COG,
        COK,
        COL,
        COM,
        CPV,
        CRI,
        CUB,
        CUW,
        CXR,
        CYM,
        CYP,
        CZE,
        DEU,
        DJI,
        DMA,
        DNK,
        DOM,
        DZA,
        ECU,
        EGY,
        ERI,
        ESH,
        ESP,
        EST,
        ETH,
        FIN,
        FJI,
        FLK,
        FRA,
        FRO,
        FSM,
        GAB,
        GBR,
        GEO,
        GGY,
        GHA,
        GIB,
        GIN,
        GLP,
        GMB,
        GNB,
        GNQ,
        GRC,
        GRD,
        GRL,
        GTM,
        GUF,
        GUM,
        GUY,
        HKG,
        HMD,
        HND,
        HRV,
        HTI,
        HUN,
        IDN,
        IMN,
        IND,
        IOT,
        IRL,
        IRN,
        IRQ,
        ISL,
        ISR,
        ITA,
        JAM,
        JEY,
        JOR,
        JPN,
        KAZ,
        KEN,
        KGZ,
        KHM,
        KIR,
        KNA,
        KOR,
        KWT,
        LAO,
        LBN,
        LBR,
        LBY,
        LCA,
        LIE,
        LKA,
        LSO,
        LTU,
        LUX,
        LVA,
        MAC,
        MAF,
        MAR,
        MCO,
        MDA,
        MDG,
        MDV,
        MEX,
        MHL,
        MKD,
        MLI,
        MLT,
        MMR,
        MNE,
        MNG,
        MNP,
        MOZ,
        MRT,
        MSR,
        MTQ,
        MUS,
        MWI,
        MYS,
        MYT,
        NAM,
        NCL,
        NER,
        NFK,
        NGA,
        NIC,
        NIU,
        NLD,
        NOR,
        NPL,
        NRU,
        NZL,
        OMN,
        PAK,
        PAN,
        PCN,
        PER,
        PHL,
        PLW,
        PNG,
        POL,
        PRI,
        PRK,
        PRT,
        PRY,
        PSE,
        PYF,
        QAT,
        REU,
        ROU,
        RUS,
        RWA,
        SAU,
        SDN,
        SEN,
        SGP,
        SGS,
        SHN,
        SJM,
        SLB,
        SLE,
        SLV,
        SMR,
        SOM,
        SPM,
        SRB,
        SSD,
        STP,
        SUR,
        SVK,
        SVN,
        SWE,
        SWZ,
        SXM,
        SYC,
        SYR,
        TCA,
        TCD,
        TGO,
        THA,
        TJK,
        TKL,
        TKM,
        TLS,
        TON,
        TTO,
        TUN,
        TUR,
        TUV,
        TWN,
        TZA,
        UGA,
        UKR,
        UMI,
        URY,
        USA,
        UZB,
        VAT,
        VCT,
        VEN,
        VGB,
        VIR,
        VNM,
        VUT,
        WLF,
        WSM,
        YEM,
        ZAF,
        ZMB,
        ZWE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    use csv::ReaderBuilder;

    #[test]
    fn test_country_code() {
        // Wikipedia
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b' ')
            .flexible(true)
            .from_reader(
                include_str!(
                    "../../tests/ISO_3166-1_alpha-3/Officially_assigned_code_elements.txt"
                )
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
            "ZZZ".parse::<CountryCode>().unwrap(),
            CountryCode::Other("ZZZ".into())
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
        assert_eq!(CountryCode::USA, CountryCode::USA);
        assert_eq!(CountryCode::USA, "USA");

        match CountryCode::USA {
            x if x == "USA" => {}
            _ => panic!(),
        }

        #[cfg(feature = "std")]
        {
            // Hash
            let mut h = std::collections::HashSet::new();
            h.insert(CountryCode::USA);
            h.insert(CountryCode::Other("USA".into()));
            assert_eq!(h.len(), 1);
        }

        #[cfg(feature = "serde")]
        {
            #[derive(serde::Serialize, serde::Deserialize)]
            struct Foo {
                code: CountryCode,
            }

            assert_eq!(
                serde_json::from_str::<Foo>(r#"{"code":"USA"}"#)
                    .unwrap()
                    .code,
                CountryCode::USA
            );
            assert_eq!(
                serde_json::to_string(&Foo {
                    code: CountryCode::USA
                })
                .unwrap(),
                r#"{"code":"USA"}"#
            );
        }
    }
}
