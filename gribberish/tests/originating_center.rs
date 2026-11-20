use gribberish::message::read_messages;

fn read(name: &str) -> Vec<u8> {
    std::fs::read(format!(
        "{}/../test-data/{name}",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}

#[test]
fn originating_centre_of_grib2_and_grib1_messages() {
    for (file, centre) in [
        ("hrrr.t06z.wrfsfcf01-CFRZR.grib2", 7u16),
        ("ecmwf-ifs-oper-surface.grib2", 98),
        ("meteofrance.arome-TCDC.grib2", 85),
        ("ecmwf_soil_8vars_tiny.grib1", 98),
    ] {
        let data = read(file);
        let message = read_messages(&data).next().unwrap();
        assert_eq!(message.originating_center().unwrap(), centre, "{file}");
    }
}

#[test]
fn ncep_local_numbers_keep_their_meaning() {
    // 0/1/193 is a local number; with no local tables it keeps NCEP's meaning.
    let data = read("hrrr.t06z.wrfsfcf01-CFRZR.grib2");
    let message = read_messages(&data).next().unwrap();
    assert_eq!(message.variable_abbrev().unwrap(), "CFRZR");
}
