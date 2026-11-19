use crate::templates::template::{Template, TemplateType};
use crate::utils::{read_i32_from_bytes, read_u16_from_bytes, read_u32_from_bytes};
use chrono::{DateTime, Utc};

use super::product_template::ProductTemplate;
use super::tables::{FixedSurfaceType, GeneratingProcess, TimeUnit};

/// One contributing spectral band of product definition template 4.32.
#[derive(Clone, Debug, PartialEq)]
pub struct SpectralBand {
    pub satellite_series: u16,
    pub satellite_number: u16,
    pub instrument_type: u16,
    /// Central wave number in m-1 (None when missing).
    pub central_wave_number: Option<f64>,
}

/// Product definition template 4.32: analysis or forecast at a horizontal level or in a
/// horizontal layer at a point in time for simulated (synthetic) satellite data.
///
/// The template has no fixed surfaces; after the forecast time come the number of
/// contributing spectral bands (NB, octet 23) and NB groups of 11 octets: satellite
/// series, satellite number, instrument type (2 octets each), scale factor (1 octet)
/// and scaled value (4 octets) of the central wave number.
pub struct SimulatedSatelliteTemplate {
    data: Vec<u8>,
    discipline: u8,
}

const BAND_START: usize = 23;
const BAND_LEN: usize = 11;

impl Template for SimulatedSatelliteTemplate {
    fn data(&self) -> &[u8] {
        &self.data
    }

    fn template_number(&self) -> u16 {
        32
    }

    fn template_type(&self) -> TemplateType {
        TemplateType::Product
    }

    fn template_name(&self) -> &str {
        "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for simulated (synthetic) satellite data"
    }
}

impl SimulatedSatelliteTemplate {
    pub fn new(data: Vec<u8>, discipline: u8) -> Self {
        SimulatedSatelliteTemplate { data, discipline }
    }

    pub fn number_of_spectral_bands(&self) -> u8 {
        self.data.get(22).copied().unwrap_or(0)
    }

    pub fn spectral_bands(&self) -> Vec<SpectralBand> {
        (0..self.number_of_spectral_bands() as usize)
            .map(|i| BAND_START + i * BAND_LEN)
            .take_while(|start| start + BAND_LEN <= self.data.len())
            .map(|start| {
                let factor = self.data[start + 6];
                let scaled = read_u32_from_bytes(&self.data, start + 7).unwrap_or(u32::MAX);
                let central_wave_number = if factor == u8::MAX || scaled == u32::MAX {
                    None
                } else {
                    let factor = as_signed!(factor, 8, i8) as i32;
                    Some(scaled as f64 * 10_f64.powi(-factor))
                };
                SpectralBand {
                    satellite_series: read_u16_from_bytes(&self.data, start).unwrap_or(0),
                    satellite_number: read_u16_from_bytes(&self.data, start + 2).unwrap_or(0),
                    instrument_type: read_u16_from_bytes(&self.data, start + 4).unwrap_or(0),
                    central_wave_number,
                }
            })
            .collect()
    }
}

impl ProductTemplate for SimulatedSatelliteTemplate {
    fn discipline(&self) -> u8 {
        self.discipline
    }

    fn category_value(&self) -> u8 {
        self.data[9]
    }

    fn parameter_value(&self) -> u8 {
        self.data[10]
    }

    fn generating_process(&self) -> GeneratingProcess {
        self.data[11].into()
    }

    fn time_unit(&self) -> TimeUnit {
        self.data[17].into()
    }

    fn time_increment_unit(&self) -> Option<TimeUnit> {
        None
    }

    fn time_interval(&self) -> i32 {
        read_i32_from_bytes(&self.data, 18).unwrap_or(0)
    }

    fn time_increment_interval(&self) -> Option<u32> {
        None
    }

    fn forecast_end_datetime(&self, _reference_date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        None
    }

    fn first_fixed_surface_type(&self) -> FixedSurfaceType {
        FixedSurfaceType::Missing
    }

    fn first_fixed_surface_value(&self) -> Option<f64> {
        None
    }

    fn second_fixed_surface_type(&self) -> FixedSurfaceType {
        FixedSurfaceType::Missing
    }

    fn second_fixed_surface_value(&self) -> Option<f64> {
        None
    }

    fn derived_forecast_type(&self) -> Option<super::tables::DerivedForecastType> {
        None
    }

    fn statistical_process_type(&self) -> Option<super::tables::TypeOfStatisticalProcessing> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_one_spectral_band() {
        // Section 4 of a template 4.32 message with one band: satellite series 0,
        // satellite number 0, instrument 0, central wave number 96618 m-1 (10.35 um).
        let mut data = vec![0u8, 0, 0, 34, 4, 0, 0, 0, 32, 1, 14, 2, 0, 0, 0, 0, 0, 0];
        data.extend_from_slice(&60i32.to_be_bytes());
        data.push(1);
        data.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0]);
        data.extend_from_slice(&96618u32.to_be_bytes());
        let template = SimulatedSatelliteTemplate::new(data, 3);

        assert_eq!(
            (template.category_value(), template.parameter_value()),
            (1, 14)
        );
        assert_eq!(template.time_interval(), 60);
        assert_eq!(
            template.first_fixed_surface_type(),
            FixedSurfaceType::Missing
        );
        assert_eq!(
            template.spectral_bands(),
            vec![SpectralBand {
                satellite_series: 0,
                satellite_number: 0,
                instrument_type: 0,
                central_wave_number: Some(96618.0),
            }]
        );
        assert_eq!(template.parameter().unwrap().abbrev, "CBTMP");
    }
}
