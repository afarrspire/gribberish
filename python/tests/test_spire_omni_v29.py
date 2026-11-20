"""Spire Omni v2.9 GRIB2 form: CCSDS at low bit widths, template 4.32, WMO and Spire
local parameters under centre 255, NCEP local parameters under centre 7.

The fixtures and the eccodes snippets that generate them are described at the top of
gribberish/tests/ccsds_bit_widths.rs and gribberish/tests/spire_omni_v29.rs.
"""

from pathlib import Path

import gribberish
import numpy as np
import pytest

TEST_DATA = Path(__file__).parent.parent.parent / "test-data"
TABLES = TEST_DATA / "eccodes-spire-omni-v29-4x3.grib2"
CCSDS = TEST_DATA / "eccodes-ccsds-bits-1-16.grib2"

ABBREVS = [
    "CBTMP", "REFC", "ICIP", "dswrf", "DNI", "FZRA", "CRAIN", "dswrf",
    "LLWS", "LTNG5NM", "LTNG10NM", "FLTCAT", "FIREWX", "TURBCAT", "missing",
]


def offsets(raw):
    offset = 0
    while offset < len(raw):
        md = gribberish.parse_grib_message_metadata(raw, offset)
        yield offset, md
        offset += md.message_size


def test_names_and_values():
    raw = TABLES.read_bytes()
    found = list(offsets(raw))
    assert [md.var_abbrev for _, md in found] == ABBREVS
    for i, (offset, _) in enumerate(found):
        values = gribberish.parse_grib_array(raw, offset)
        np.testing.assert_array_equal(values, 200.0 + i + np.arange(12))


def ccsds_expected(bits):
    top = (1 << bits) - 1
    i = np.arange(5000, dtype=np.int64)
    v = ((((i * 1103515245 + 12345) % (1 << 31)) >> 8) % (top + 1)).astype(float)
    v[i < 1400] = top
    v[i < 1000] = i[i < 1000] % 2
    v[i < 600] = 0
    v[-1] = top
    return v


@pytest.mark.parametrize("bits", range(1, 17))
def test_ccsds_bit_widths(bits):
    raw = CCSDS.read_bytes()
    offset, md = list(offsets(raw))[bits - 1]
    assert md.var_abbrev == "PTYPE"
    np.testing.assert_array_equal(gribberish.parse_grib_array(raw, offset), ccsds_expected(bits))


def test_xarray_backend_reads_spire_names():
    xr = pytest.importorskip("xarray")
    ds = xr.open_dataset(TABLES, engine="gribberish", collapse_groups=True)
    for name in ("cbtmp", "refc", "icip", "dni", "fzra", "crain", "dswrf"):
        assert name in ds.data_vars, name
