use gribberish::message::read_messages;

// Sixteen 100 × 50 regular lat-lon messages (0/1/19 precipitation type), CCSDS packing
// (template 5.42, eccodes defaults: flags 14, block size 32, reference sample interval
// 128) at 1 to 16 bits per value, decimal scale 0. 5000 points, so each message crosses
// a reference sample interval boundary (32 × 128 = 4096). The values are the integers
// from `expected`; eccodes decodes them exactly. Generated with the eccodes Python
// bindings 2.50.0:
//
//     import eccodes as ec
//
//     def values(bits):  # same pattern as `expected` below
//         top = (1 << bits) - 1
//         out = []
//         for i in range(5000):
//             if i < 600: v = 0
//             elif i < 1000: v = i % 2
//             elif i < 1400: v = top
//             else: v = (((i * 1103515245 + 12345) % (1 << 31)) >> 8) % (top + 1)
//             out.append(float(v))
//         out[-1] = float(top)
//         return out
//
//     with open("eccodes-ccsds-bits-1-16.grib2", "wb") as f:
//         for bits in range(1, 17):
//             h = ec.codes_grib_new_from_samples("GRIB2")
//             for key, value in [
//                 ("centre", 7), ("dataDate", 20260811), ("dataTime", 0),
//                 ("gridType", "regular_ll"), ("Ni", 100), ("Nj", 50),
//                 ("latitudeOfFirstGridPointInDegrees", 50.0),
//                 ("longitudeOfFirstGridPointInDegrees", 0.0),
//                 ("latitudeOfLastGridPointInDegrees", 1.0),
//                 ("longitudeOfLastGridPointInDegrees", 99.0),
//                 ("iDirectionIncrementInDegrees", 1.0),
//                 ("jDirectionIncrementInDegrees", 1.0),
//                 ("discipline", 0), ("parameterCategory", 1), ("parameterNumber", 19),
//                 ("typeOfFirstFixedSurface", 1), ("packingType", "grid_ccsds"),
//                 ("decimalScaleFactor", 0), ("bitsPerValue", bits),
//             ]:
//                 ec.codes_set(h, key, value)
//             ec.codes_set_values(h, values(bits))
//             assert ec.codes_get(h, "bitsPerValue") == bits
//             ec.codes_write(h, f)
//             ec.codes_release(h)
const FIXTURE: &[u8] = include_bytes!("../../test-data/eccodes-ccsds-bits-1-16.grib2");

fn expected(bits: usize) -> Vec<f64> {
    let top = (1u64 << bits) - 1;
    let mut out: Vec<f64> = (0..5000u64)
        .map(|i| match i {
            0..=599 => 0,
            600..=999 => i % 2,
            1000..=1399 => top,
            _ => (((i * 1103515245 + 12345) % (1 << 31)) >> 8) % (top + 1),
        } as f64)
        .collect();
    out[4999] = top as f64;
    out
}

#[test]
fn ccsds_decodes_every_bit_width_from_1_to_16() {
    let messages: Vec<_> = read_messages(FIXTURE).collect();
    assert_eq!(messages.len(), 16);

    for (index, message) in messages.iter().enumerate() {
        let bits = index + 1;
        let values = message
            .data()
            .unwrap_or_else(|e| panic!("{bits} bits per value: {e}"));
        let want = expected(bits);
        assert_eq!(values.len(), want.len(), "{bits} bits per value");
        let wrong = values.iter().zip(&want).filter(|(a, b)| a != b).count();
        assert_eq!(wrong, 0, "{bits} bits per value: {wrong} values differ");
    }
}
