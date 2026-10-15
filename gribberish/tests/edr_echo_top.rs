use gribberish::message::read_messages;
use gribberish::message_metadata::scan_message_metadata;
use gribberish::templates::product::tables::FixedSurfaceType;

// Two 4 × 3 regular lat-lon messages, simple packing at 16 bits: eddy dissipation
// parameter (0/19/30) at 250 hPa and NCEP local echo top (0/16/197) at cloud top (fixed
// surface type 3). Expected values are eccodes' decode of the file. Generated with the
// eccodes Python bindings 2.48.0:
//
//     import eccodes as ec
//
//     def msg(cat, num, level_type, level, values, decimal):
//         h = ec.codes_grib_new_from_samples("GRIB2")
//         for key, value in [
//             ("centre", 7), ("dataDate", 20261015), ("dataTime", 1200),
//             ("gridType", "regular_ll"), ("Ni", 4), ("Nj", 3),
//             ("latitudeOfFirstGridPointInDegrees", 50.0),
//             ("longitudeOfFirstGridPointInDegrees", 0.0),
//             ("latitudeOfLastGridPointInDegrees", 48.0),
//             ("longitudeOfLastGridPointInDegrees", 3.0),
//             ("iDirectionIncrementInDegrees", 1.0), ("jDirectionIncrementInDegrees", 1.0),
//             ("discipline", 0), ("parameterCategory", cat), ("parameterNumber", num),
//             ("typeOfFirstFixedSurface", level_type),
//         ]:
//             ec.codes_set(h, key, value)
//         if level is not None:
//             ec.codes_set(h, "scaleFactorOfFirstFixedSurface", 0)
//             ec.codes_set(h, "scaledValueOfFirstFixedSurface", level)
//         ec.codes_set(h, "forecastTime", 1)
//         ec.codes_set(h, "packingType", "grid_simple")
//         ec.codes_set(h, "decimalScaleFactor", decimal)
//         ec.codes_set(h, "bitsPerValue", 16)
//         ec.codes_set_values(h, values)
//         return h
//
//     edr = [i * 0.05 for i in range(12)]
//     retop = [i * 1500.0 for i in range(12)]
//     with open("eccodes-edr-echo-top-4x3.grib2", "wb") as f:
//         for h in (msg(19, 30, 100, 25000, edr, 3), msg(16, 197, 3, None, retop, 0)):
//             ec.codes_write(h, f)
//             ec.codes_release(h)
const FIXTURE: &[u8] = include_bytes!("../../test-data/eccodes-edr-echo-top-4x3.grib2");

const EDPARM: [f64; 12] = [
    0.0,
    0.0500030517578125,
    0.100006103515625,
    0.149993896484375,
    0.1999969482421875,
    0.25,
    0.3000030517578125,
    0.350006103515625,
    0.399993896484375,
    0.4499969482421875,
    0.5,
    0.5500030517578125,
];

const RETOP: [f64; 12] = [
    0.0, 1500.0, 3000.0, 4500.0, 6000.0, 7500.0, 9000.0, 10500.0, 12000.0, 13500.0, 15000.0,
    16500.0,
];

fn assert_values(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (a, e) in actual.iter().zip(expected) {
        assert!((a - e).abs() < 1e-9, "{a} != {e}");
    }
}

#[test]
fn read_eddy_dissipation_parameter_and_echo_top() {
    let messages: Vec<_> = read_messages(FIXTURE).collect();
    assert_eq!(messages.len(), 2);

    let edparm = &messages[0];
    assert_eq!(
        (
            edparm.discipline_value().unwrap(),
            edparm.category_value().unwrap(),
            edparm.parameter_value().unwrap()
        ),
        (0, 19, 30)
    );
    assert_eq!(edparm.variable_abbrev().unwrap(), "EDPARM");
    assert_eq!(edparm.variable_name().unwrap(), "eddydissipationparameter");
    assert_eq!(edparm.unit().unwrap(), "m2/3s-1");
    assert_eq!(
        edparm.first_fixed_surface().unwrap(),
        (FixedSurfaceType::IsobaricSurface, Some(25000.0))
    );
    assert_eq!(edparm.grid_dimensions().unwrap(), (3, 4));
    assert_values(&edparm.data().unwrap(), &EDPARM);

    let retop = &messages[1];
    assert_eq!(
        (
            retop.discipline_value().unwrap(),
            retop.category_value().unwrap(),
            retop.parameter_value().unwrap()
        ),
        (0, 16, 197)
    );
    assert_eq!(retop.variable_abbrev().unwrap(), "RETOP");
    assert_eq!(retop.unit().unwrap(), "m");
    assert_eq!(
        retop.first_fixed_surface().unwrap().0,
        FixedSurfaceType::CloudTop
    );
    assert_values(&retop.data().unwrap(), &RETOP);
}

#[test]
fn scan_keeps_both_messages() {
    // Without these table entries both messages come back with var "missing".
    let metadata = scan_message_metadata(FIXTURE);
    let mut vars: Vec<_> = metadata.values().map(|(_, _, m)| m.var.clone()).collect();
    vars.sort();
    assert_eq!(vars, ["EDPARM", "RETOP"]);
}
