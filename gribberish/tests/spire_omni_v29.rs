use gribberish::message::read_messages;
use gribberish::message_metadata::scan_message_metadata;
use gribberish::templates::product::tables::FixedSurfaceType;

// Fifteen 4 × 3 regular lat-lon messages, simple packing at 16 bits, values 200 + i + k
// for message i and point k, written with the eccodes Python bindings 2.50.0 in the
// form of Spire Omni v2.9 GRIB2 files: centre 255 (or 7 where noted), sub-centre 0,
// tablesVersion 32, localTablesVersion 1, forecast time 60 minutes. Messages:
//
//      0  centre 255  3/1/14   template 4.32, one band, wave number 96618 m-1 (tb_ir)
//      1  centre 255  0/16/5   composite reflectivity (refc)
//      2  centre 255  0/19/20  icing (icing potential)
//      3  centre 255  0/4/7    downward short-wave radiation flux (ghi)
//      4  centre 255  0/4/192  Spire local: direct normal irradiance (dni)
//      5  centre 255  0/1/192  Spire local: freezing rain accumulation (fzra1h)
//      6  centre 7    0/1/192  NCEP local: categorical rain
//      7  centre 7    0/4/192  NCEP local: downward short-wave radiation flux
//      8  centre 255  0/2/192  Spire local: low-level wind shear
//      9  centre 255  0/17/192 Spire local: lightning within 5 nm
//     10  centre 255  0/17/193 Spire local: lightning within 10 nm
//     11  centre 255  0/19/192 Spire local: flight category
//     12  centre 255  0/19/193 Spire local: fire weather joint event
//     13  centre 255  0/19/194 Spire local: turbulence category
//     14  centre 255  0/19/195 local number Spire does not define
//
// For each message (template 4.0 has typeOfFirstFixedSurface 1):
//
//     h = ec.codes_grib_new_from_samples("GRIB2")
//     for key, value in [
//         ("centre", centre), ("subCentre", 0), ("tablesVersion", 32),
//         ("localTablesVersion", 1 if centre == 255 else 0),
//         ("dataDate", 20260811), ("dataTime", 0),
//         ("gridType", "regular_ll"), ("Ni", 4), ("Nj", 3),
//         ("latitudeOfFirstGridPointInDegrees", 50.0),
//         ("longitudeOfFirstGridPointInDegrees", 0.0),
//         ("latitudeOfLastGridPointInDegrees", 48.0),
//         ("longitudeOfLastGridPointInDegrees", 3.0),
//         ("iDirectionIncrementInDegrees", 1.0), ("jDirectionIncrementInDegrees", 1.0),
//         ("discipline", d), ("productDefinitionTemplateNumber", pdt),
//         ("parameterCategory", c), ("parameterNumber", n),
//         ("indicatorOfUnitOfTimeRange", 0), ("forecastTime", 60),
//     ]:
//         ec.codes_set(h, key, value)
//     if pdt == 32:
//         ec.codes_set(h, "NB", 1)
//         for key, value in [("satelliteSeries", 0), ("satelliteNumber", 0),
//                            ("instrumentType", 0), ("scaleFactorOfCentralWaveNumber", 0),
//                            ("scaledValueOfCentralWaveNumber", 96618)]:
//             ec.codes_set_array(h, key, [value])
//     else:
//         ec.codes_set(h, "typeOfFirstFixedSurface", 1)
//     ec.codes_set(h, "packingType", "grid_simple")
//     ec.codes_set(h, "decimalScaleFactor", 0)
//     ec.codes_set(h, "bitsPerValue", 16)
//     ec.codes_set_values(h, [200.0 + i + k for k in range(12)])
const FIXTURE: &[u8] = include_bytes!("../../test-data/eccodes-spire-omni-v29-4x3.grib2");

/// (originating centre, (discipline, category, number), abbreviation, unit)
type Expected = (u16, (u8, u8, u8), &'static str, &'static str);

const EXPECTED: [Expected; 15] = [
    (255, (3, 1, 14), "CBTMP", "K"),
    (255, (0, 16, 5), "REFC", "dB"),
    (255, (0, 19, 20), "ICIP", "%"),
    (255, (0, 4, 7), "dswrf", "Wm-2"),
    (255, (0, 4, 192), "DNI", "Wm-2"),
    (255, (0, 1, 192), "FZRA", "kgm-2"),
    (7, (0, 1, 192), "CRAIN", "BOOL"),
    (7, (0, 4, 192), "dswrf", "Wm-2"),
    (255, (0, 2, 192), "LLWS", "ms-1"),
    (255, (0, 17, 192), "LTNG5NM", "0/1"),
    (255, (0, 17, 193), "LTNG10NM", "0/1"),
    (255, (0, 19, 192), "FLTCAT", "code"),
    (255, (0, 19, 193), "FIREWX", "0/1"),
    (255, (0, 19, 194), "TURBCAT", "code"),
    (255, (0, 19, 195), "missing", ""),
];

#[test]
fn names_follow_the_originating_centre() {
    let messages: Vec<_> = read_messages(FIXTURE).collect();
    assert_eq!(messages.len(), EXPECTED.len());

    for (i, (message, (centre, dcn, abbrev, unit))) in messages.iter().zip(EXPECTED).enumerate() {
        assert_eq!(message.originating_center().unwrap(), centre, "message {i}");
        let found = (
            message.discipline_value().unwrap(),
            message.category_value().unwrap(),
            message.parameter_value().unwrap(),
        );
        assert_eq!(found, dcn, "message {i}");
        assert_eq!(message.variable_abbrev().unwrap(), abbrev, "message {i}");
        assert_eq!(message.unit().unwrap(), unit, "message {i}");

        let values = message.data().unwrap();
        let want: Vec<f64> = (0..12).map(|k| (200 + i + k) as f64).collect();
        assert_eq!(values, want, "message {i}");
    }
}

#[test]
fn simulated_satellite_message_has_no_level() {
    let message = read_messages(FIXTURE).next().unwrap();
    assert_eq!(
        message.variable_name().unwrap(),
        "cloudybrightnesstemperature"
    );
    assert_eq!(
        message.first_fixed_surface().unwrap().0,
        FixedSurfaceType::Missing
    );
    assert_eq!(message.grid_dimensions().unwrap(), (3, 4));
    let metadata = scan_message_metadata(FIXTURE);
    assert!(metadata.values().any(|(_, _, m)| m.var == "CBTMP"));
}
