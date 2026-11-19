use gribberish_macros::{DisplayDescription, FromValue, ToParameter};
use gribberish_types::Parameter;

#[repr(u8)]
#[derive(Eq, PartialEq, Debug, DisplayDescription, FromValue, ToParameter)]
pub enum SpaceNcepLocal {
    #[description = "simulated brightness temperature for GOES 12, channel 3"]
    #[abbrev = "SBT123"]
    #[unit = "K"]
    SimulatedBrightnessTempGOES12Ch3 = 1,
    #[description = "simulated brightness temperature for GOES 12, channel 4"]
    #[abbrev = "SBT124"]
    #[unit = "K"]
    SimulatedBrightnessTempGOES12Ch4 = 2,
    #[description = "simulated brightness temperature for GOES 11, channel 3"]
    #[abbrev = "SBT113"]
    #[unit = "K"]
    SimulatedBrightnessTempGOES11Ch3 = 7,
    #[description = "simulated brightness temperature for GOES 11, channel 4"]
    #[abbrev = "SBT114"]
    #[unit = "K"]
    SimulatedBrightnessTempGOES11Ch4 = 8,
    Missing = 255,
}

/// GRIB2 table 4.2-3-1: space products, quantitative. Abbreviations are NCEP's.
/// Reference: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2-3-1.shtml
#[repr(u8)]
#[derive(Eq, PartialEq, Debug, DisplayDescription, FromValue, ToParameter)]
pub enum SpaceQuantitative {
    #[description = "estimated precipitation"]
    #[abbrev = "ESTP"]
    #[unit = "kg m-2"]
    EstimatedPrecipitation = 0,
    #[description = "instantaneous rain rate"]
    #[abbrev = "IRRATE"]
    #[unit = "kg m-2 s-1"]
    InstantaneousRainRate = 1,
    #[description = "cloud top height"]
    #[abbrev = "CTOPH"]
    #[unit = "m"]
    CloudTopHeight = 2,
    #[description = "cloud top height quality indicator"]
    #[abbrev = "CTOPHQI"]
    #[unit = "code table 4.219"]
    CloudTopHeightQualityIndicator = 3,
    #[description = "estimated u-component of wind"]
    #[abbrev = "ESTUGRD"]
    #[unit = "m s-1"]
    EstimatedUComponentOfWind = 4,
    #[description = "estimated v-component of wind"]
    #[abbrev = "ESTVGRD"]
    #[unit = "m s-1"]
    EstimatedVComponentOfWind = 5,
    #[description = "number of pixels used"]
    #[abbrev = "NPIXU"]
    #[unit = "numeric"]
    NumberOfPixelsUsed = 6,
    #[description = "solar zenith angle"]
    #[abbrev = "SOLZA"]
    #[unit = "deg"]
    SolarZenithAngle = 7,
    #[description = "relative azimuth angle"]
    #[abbrev = "RAZA"]
    #[unit = "deg"]
    RelativeAzimuthAngle = 8,
    #[description = "reflectance in 0.6 micron channel"]
    #[abbrev = "RFL06"]
    #[unit = "%"]
    Reflectance06 = 9,
    #[description = "reflectance in 0.8 micron channel"]
    #[abbrev = "RFL08"]
    #[unit = "%"]
    Reflectance08 = 10,
    #[description = "reflectance in 1.6 micron channel"]
    #[abbrev = "RFL16"]
    #[unit = "%"]
    Reflectance16 = 11,
    #[description = "reflectance in 3.9 micron channel"]
    #[abbrev = "RFL39"]
    #[unit = "%"]
    Reflectance39 = 12,
    #[description = "atmospheric divergence"]
    #[abbrev = "ATMDIV"]
    #[unit = "s-1"]
    AtmosphericDivergence = 13,
    #[description = "cloudy brightness temperature"]
    #[abbrev = "CBTMP"]
    #[unit = "K"]
    CloudyBrightnessTemperature = 14,
    #[description = "clear sky brightness temperature"]
    #[abbrev = "CSBTMP"]
    #[unit = "K"]
    ClearSkyBrightnessTemperature = 15,
    #[description = "cloudy radiance (with respect to wave number)"]
    #[abbrev = "CLDRAD"]
    #[unit = "W m-1 sr-1"]
    CloudyRadiance = 16,
    #[description = "clear sky radiance (with respect to wave number)"]
    #[abbrev = "CSKYRAD"]
    #[unit = "W m-1 sr-1"]
    ClearSkyRadiance = 17,
    #[description = "wind speed"]
    #[abbrev = "WINDS"]
    #[unit = "m s-1"]
    WindSpeed = 19,
    #[description = "aerosol optical thickness at 0.635 um"]
    #[abbrev = "AOT06"]
    #[unit = "numeric"]
    AerosolOpticalThickness06 = 20,
    #[description = "aerosol optical thickness at 0.810 um"]
    #[abbrev = "AOT08"]
    #[unit = "numeric"]
    AerosolOpticalThickness08 = 21,
    #[description = "aerosol optical thickness at 1.640 um"]
    #[abbrev = "AOT16"]
    #[unit = "numeric"]
    AerosolOpticalThickness16 = 22,
    #[description = "angstrom coefficient"]
    #[abbrev = "ANGCOE"]
    #[unit = "numeric"]
    AngstromCoefficient = 23,
    #[description = "cosine of the solar zenith angle"]
    #[abbrev = "COSOLZA"]
    #[unit = "numeric"]
    CosineOfSolarZenithAngle = 24,
    #[description = "bidirectional reflectance factor"]
    #[abbrev = "BRFLF"]
    #[unit = "numeric"]
    BidirectionalReflectanceFactor = 27,
    #[description = "brightness temperature"]
    #[abbrev = "SPBRT"]
    #[unit = "K"]
    BrightnessTemperature = 28,
    #[description = "scaled radiance"]
    #[abbrev = "SCRAD"]
    #[unit = "numeric"]
    ScaledRadiance = 29,
    #[description = "reflectance in 0.4 micron channel"]
    #[abbrev = "RFL04"]
    #[unit = "%"]
    Reflectance04 = 30,
    #[description = "cloudy reflectance"]
    #[abbrev = "CLDREF"]
    #[unit = "%"]
    CloudyReflectance = 31,
    #[description = "clear reflectance"]
    #[abbrev = "CLRREF"]
    #[unit = "%"]
    ClearReflectance = 32,
    Missing = 255,
}

pub fn space_parameter(category: u8, parameter: u8) -> Option<Parameter> {
    match category {
        1 => Some(Parameter::from(SpaceQuantitative::from(parameter))),
        192 => Some(Parameter::from(SpaceNcepLocal::from(parameter))),
        _ => None,
    }
}

pub fn space_category(category: u8) -> &'static str {
    match category {
        0 => "image format",
        1 => "quantitative",
        192 => "ncep local",
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use crate::templates::product::parameters::parameter;

    #[test]
    fn space_quantitative_brightness_temperatures() {
        // 3/1/14 is how simulated infrared imagery is written with template 4.32.
        let cases = [
            (14u8, "CBTMP", "cloudybrightnesstemperature"),
            (15, "CSBTMP", "clearskybrightnesstemperature"),
            (28, "SPBRT", "brightnesstemperature"),
        ];
        for (number, abbrev, name) in cases {
            let param = parameter(3, 1, number).unwrap();
            assert_eq!(param.abbrev, abbrev, "parameter number {number}");
            assert_eq!(param.name, name, "parameter number {number}");
            assert_eq!(param.unit, "K", "parameter number {number}");
        }
        assert_eq!(parameter(3, 1, 18).unwrap().abbrev, "missing");
        assert_eq!(parameter(3, 192, 1).unwrap().abbrev, "SBT123");
    }
}
