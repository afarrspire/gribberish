//! Local parameter tables per originating centre.
//!
//! In the WMO parameter tables (4.2-d-c) the numbers 192 to 254 are reserved for local
//! use, so their meaning depends on the centre that wrote the message. The built-in
//! tables give them NCEP's meanings, for every centre. A [`LocalTable`] gives the local
//! numbers of one centre (optionally one sub-centre and one local tables version) their
//! own meanings; the tables in [`LOCAL_TABLES`] apply when a message is decoded.
//! Spire build: `LOCAL_TABLES` holds the Spire Omni table (see `spire.rs`).
use gribberish_types::Parameter;

/// The originating centre, sub-centre and local tables version of a GRIB2 message
/// (section 1, octets 6-7, 8-9 and 11).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Originator {
    pub centre: u16,
    pub subcentre: u16,
    pub local_tables_version: u8,
}

/// One entry of a local table.
#[derive(Clone, Copy, Debug)]
pub struct LocalParameter {
    pub discipline: u8,
    pub category: u8,
    pub number: u8,
    pub abbrev: &'static str,
    pub name: &'static str,
    pub unit: &'static str,
}

/// The local parameters of one originating centre.
///
/// The table applies to messages from `centre`, and, when set, only to those with this
/// `subcentre` and `local_tables_version`. Under a table that applies, a local number
/// (192 to 254) in a WMO discipline and category comes from `parameters`, and is
/// "missing" when the table does not define it; it never falls back to NCEP's meaning.
/// WMO numbers (0 to 191) and local disciplines and categories (192 and above) are not
/// affected.
#[derive(Clone, Copy, Debug)]
pub struct LocalTable {
    pub centre: u16,
    pub subcentre: Option<u16>,
    pub local_tables_version: Option<u8>,
    pub parameters: &'static [LocalParameter],
}

impl LocalTable {
    pub fn applies_to(&self, originator: &Originator) -> bool {
        self.centre == originator.centre
            && self.subcentre.is_none_or(|s| s == originator.subcentre)
            && self
                .local_tables_version
                .is_none_or(|v| v == originator.local_tables_version)
    }
}

/// Local tables used when messages are decoded. Spire build: the Spire Omni table (see
/// `spire.rs` and SPIRE.md); every other originator gets NCEP's meanings.
pub const LOCAL_TABLES: &[LocalTable] = &[super::spire::SPIRE_OMNI_LOCAL_TABLE];

/// True for a number in the local-use range of a WMO parameter table (WMO discipline
/// and category, number 192 to 254).
pub fn is_local_number(discipline: u8, category: u8, number: u8) -> bool {
    discipline < 192 && category < 192 && (192..=254).contains(&number)
}

/// Parameter lookup with the local tables `tables`: the first table that applies to
/// `originator` gives the meaning of a local number; otherwise the built-in tables do.
pub fn parameter_with_local_tables(
    tables: &[LocalTable],
    originator: &Originator,
    discipline: u8,
    category: u8,
    number: u8,
) -> Option<Parameter> {
    if is_local_number(discipline, category, number) {
        if let Some(table) = tables.iter().find(|t| t.applies_to(originator)) {
            let entry = table
                .parameters
                .iter()
                .find(|p| (p.discipline, p.category, p.number) == (discipline, category, number));
            return Some(match entry {
                Some(p) => Parameter {
                    name: p.name.to_string(),
                    abbrev: p.abbrev.to_string(),
                    unit: p.unit.to_string(),
                },
                None => Parameter {
                    name: "missing".to_string(),
                    abbrev: "missing".to_string(),
                    unit: "".to_string(),
                },
            });
        }
    }
    super::parameter(discipline, category, number)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::product::parameters::parameter;

    fn key(p: Option<Parameter>) -> Option<(String, String, String)> {
        p.map(|p| (p.abbrev, p.name, p.unit))
    }

    fn originator(centre: u16, subcentre: u16, local_tables_version: u8) -> Originator {
        Originator {
            centre,
            subcentre,
            local_tables_version,
        }
    }

    const TEST_TABLE: LocalTable = LocalTable {
        centre: 255,
        subcentre: Some(0),
        local_tables_version: Some(1),
        parameters: &[LocalParameter {
            discipline: 0,
            category: 1,
            number: 192,
            abbrev: "TEST",
            name: "testparameter",
            unit: "1",
        }],
    };

    #[test]
    fn with_no_local_tables_every_centre_gets_the_built_in_meanings() {
        for o in [
            originator(7, 0, 1),
            originator(98, 0, 0),
            originator(255, 0, 1),
            originator(255, 65535, 255),
        ] {
            for d in [0u8, 1, 2, 3, 10, 192, 209] {
                for c in 0..=255u8 {
                    for n in 0..=255u8 {
                        assert_eq!(
                            key(parameter_with_local_tables(&[], &o, d, c, n)),
                            key(parameter(d, c, n)),
                            "{o:?} {d}/{c}/{n}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn a_local_table_gives_local_numbers_of_its_centre() {
        let tables = [TEST_TABLE];
        let o = originator(255, 0, 1);
        let found = parameter_with_local_tables(&tables, &o, 0, 1, 192).unwrap();
        assert_eq!(
            (
                found.abbrev.as_str(),
                found.name.as_str(),
                found.unit.as_str()
            ),
            ("TEST", "testparameter", "1")
        );
        // A local number the table does not define is missing, not NCEP's entry.
        let undefined = parameter_with_local_tables(&tables, &o, 0, 4, 192).unwrap();
        assert_eq!(undefined.abbrev, "missing");
        // WMO numbers and local disciplines or categories keep the built-in meanings.
        for (d, c, n) in [
            (0u8, 4u8, 7u8),
            (0, 1, 191),
            (0, 1, 255),
            (0, 192, 192),
            (192, 128, 139),
        ] {
            assert_eq!(
                key(parameter_with_local_tables(&tables, &o, d, c, n)),
                key(parameter(d, c, n)),
                "{d}/{c}/{n}"
            );
        }
    }

    #[test]
    fn a_local_table_applies_only_to_its_key() {
        let tables = [TEST_TABLE];
        for o in [
            originator(7, 0, 1),
            originator(255, 1, 1),
            originator(255, 0, 0),
            originator(255, 0, 2),
        ] {
            assert_eq!(
                key(parameter_with_local_tables(&tables, &o, 0, 1, 192)),
                key(parameter(0, 1, 192)),
                "{o:?}"
            );
        }
        let any_subcentre = [LocalTable {
            subcentre: None,
            local_tables_version: None,
            ..TEST_TABLE
        }];
        assert_eq!(
            parameter_with_local_tables(&any_subcentre, &originator(255, 3, 0), 0, 1, 192)
                .unwrap()
                .abbrev,
            "TEST"
        );
    }
}
