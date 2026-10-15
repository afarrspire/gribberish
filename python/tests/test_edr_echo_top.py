"""Eddy dissipation parameter (0/19/30) and NCEP local echo top (0/16/197).

The fixture holds two 4 x 3 regular lat-lon messages written with eccodes: EDPARM at
250 hPa and RETOP at cloud top. Expected values are eccodes' decode of the file.
"""

from pathlib import Path

import gribberish
import numpy as np
import pytest

FIXTURE = Path(__file__).parent.parent.parent / "test-data" / "eccodes-edr-echo-top-4x3.grib2"
EDPARM = [
    0.0, 0.0500030517578125, 0.100006103515625, 0.149993896484375,
    0.1999969482421875, 0.25, 0.3000030517578125, 0.350006103515625,
    0.399993896484375, 0.4499969482421875, 0.5, 0.5500030517578125,
]
RETOP = [0.0, 1500.0, 3000.0, 4500.0, 6000.0, 7500.0, 9000.0, 10500.0, 12000.0, 13500.0, 15000.0, 16500.0]


def test_metadata_names_both_parameters():
    raw = FIXTURE.read_bytes()
    first = gribberish.parse_grib_message_metadata(raw, 0)
    assert first.var_abbrev == "EDPARM"
    assert first.units == "m2/3s-1"
    second = gribberish.parse_grib_message_metadata(raw, first.message_size)
    assert second.var_abbrev == "RETOP"
    assert second.units == "m"


def test_xarray_backend_reads_edparm_and_retop():
    xr = pytest.importorskip("xarray")
    ds = xr.open_dataset(FIXTURE, engine="gribberish", collapse_groups=True)
    assert set(ds.data_vars) == {"edparm", "retop"}
    assert ds["edparm"].attrs["unit"] == "m2/3s-1"
    assert ds["retop"].attrs["unit"] == "m"
    np.testing.assert_allclose(np.sort(ds["edparm"].values.ravel()), EDPARM, atol=1e-9)
    np.testing.assert_allclose(np.sort(ds["retop"].values.ravel()), RETOP, atol=1e-9)
