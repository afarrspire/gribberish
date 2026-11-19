/// Spire local GRIB2 parameters (Spire build only, see SPIRE.md)
///
/// Spire Omni writes its GRIB2 files with originating centre 255 (missing: Spire has no
/// WMO centre id), sub-centre 0, local tables version 1. The table is the `local_table`
/// of the Omni catalogue (`projects/modelling/omni/config/catalog/catalog_v1.3.1.json`
/// in apex, on Tom's `v2.9.6-int` branch since commit 2749ad4a; not on apex `main`). It is keyed on all three values, so other data under centre 255 keeps the
/// NCEP meanings unless it also has sub-centre 0 and local tables version 1.
use super::local::{LocalParameter, LocalTable};

const fn entry(
    category: u8,
    number: u8,
    abbrev: &'static str,
    name: &'static str,
    unit: &'static str,
) -> LocalParameter {
    LocalParameter {
        discipline: 0,
        category,
        number,
        abbrev,
        name,
        unit,
    }
}

pub const SPIRE_OMNI_LOCAL_TABLE: LocalTable = LocalTable {
    centre: 255,
    subcentre: Some(0),
    local_tables_version: Some(1),
    parameters: &[
        entry(1, 192, "FZRA", "freezingrainaccumulation", "kgm-2"),
        entry(
            2,
            192,
            "LLWS",
            "lowlevelwindshearmaxvectordifference",
            "ms-1",
        ),
        entry(4, 192, "DNI", "directnormalirradiance", "Wm-2"),
        entry(17, 192, "LTNG5NM", "lightningwithin5nm", "0/1"),
        entry(17, 193, "LTNG10NM", "lightningwithin10nm", "0/1"),
        entry(19, 192, "FLTCAT", "flightcategory", "code"),
        entry(19, 193, "FIREWX", "fireweatherjointevent", "0/1"),
        entry(19, 194, "TURBCAT", "turbulencecategoryedr", "code"),
    ],
};

#[cfg(test)]
mod tests {
    use crate::templates::product::parameters::local::Originator;
    use crate::templates::product::parameters::{parameter, parameter_for_originator};
    use gribberish_types::Parameter;

    fn key(p: Option<Parameter>) -> Option<(String, String, String)> {
        p.map(|p| (p.abbrev, p.name, p.unit))
    }

    fn omni() -> Originator {
        Originator {
            centre: 255,
            subcentre: 0,
            local_tables_version: 1,
        }
    }

    #[test]
    fn spire_local_entries_under_centre_255() {
        let fzra = parameter_for_originator(&omni(), 0, 1, 192).unwrap();
        assert_eq!(fzra.abbrev, "FZRA");
        assert_eq!(fzra.unit, "kgm-2");
        let dni = parameter_for_originator(&omni(), 0, 4, 192).unwrap();
        assert_eq!(dni.abbrev, "DNI");
        assert_eq!(dni.name, "directnormalirradiance");
        assert_eq!(
            parameter_for_originator(&omni(), 0, 19, 195)
                .unwrap()
                .abbrev,
            "missing"
        );
    }

    #[test]
    fn ncep_local_entries_outside_the_omni_key() {
        let others = [
            Originator {
                centre: 7,
                ..omni()
            },
            Originator {
                centre: 98,
                ..omni()
            },
            Originator {
                subcentre: 1,
                ..omni()
            },
            Originator {
                local_tables_version: 0,
                ..omni()
            },
            Originator {
                local_tables_version: 2,
                ..omni()
            },
        ];
        for o in others {
            assert_eq!(
                parameter_for_originator(&o, 0, 1, 192).unwrap().abbrev,
                "CRAIN",
                "{o:?}"
            );
            assert_eq!(
                key(parameter_for_originator(&o, 0, 4, 192)),
                key(parameter(0, 4, 192)),
                "{o:?}"
            );
        }
    }

    #[test]
    fn wmo_entries_do_not_depend_on_centre() {
        for (d, c, n) in [(0u8, 4u8, 7u8), (0, 16, 5), (3, 1, 14), (192, 128, 139)] {
            assert_eq!(
                key(parameter_for_originator(&omni(), d, c, n)),
                key(parameter(d, c, n))
            );
        }
    }
}
