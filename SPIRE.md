# Spire build of gribberish

This `spire` branch (in the fork `afarrspire/gribberish`) is the build the
aviation-weather hackathon lake pins. It exists only until upstream merges
and releases the table additions below; after that the lake goes back to a
PyPI release of `gribberish` and this branch is dropped.

## What it contains

Upstream release 1.8.4 (`5e039e1`) plus, each as its own merge:

1. Upstream PR [#212](https://github.com/mpiannucci/gribberish/pull/212)
   (mgrover1, open): read ECMWF model-level CAT and lnsp, expose the `pv`
   coordinate array.
2. Branch `edr-echo-top-parameters` (to go upstream as its own PR): eddy
   dissipation parameters 0/19/28 MWTURB, 0/19/30 EDPARM, 0/19/31 MXEDPRM and
   NCEP echo top 0/16/197 RETOP, with a test fixture.
3. A spire build commit: version `1.8.4+spire.N`, the
   `.github/workflows/spire-wheels.yml` workflow, and a guard so upstream's
   PyPI publish job in `python.yml` only runs in `mpiannucci/gribberish`.
4. Spire Omni v2.9 GRIB2 support (branch `spire-omni-v29`, one commit each, so
   that the first four can go upstream as separate PRs):
   1. `a892f67` CCSDS at 1 to 4 bits per value (to go upstream). The pure-Rust
      decoder used a 2-bit code option id at 4 bits and a 1-bit id at 2 and 3
      bits, and rejected 1 bit; libaec uses a 3-bit id for every width up to 8
      unless `AEC_RESTRICTED` is set. Omni's 4-bit precipitation type and
      freezing rain failed on the 3 km grid and decoded wrong on small grids.
      Fixture `test-data/eccodes-ccsds-bits-1-16.grib2` (1 to 16 bits).
   2. `4f79219` WMO parameters (to go upstream): 0/16/4 REFD and 0/16/5 REFC
      (WMO numbers for NCEP's 195 and 196), 0/19/20 ICIP (icing), and table
      4.2-3-1 (space products, quantitative) with 3/1/14 CBTMP, the cloudy
      brightness temperature of simulated IR imagery. NCEP abbreviations.
   3. `0487524` Product definition template 4.32, simulated satellite data
      (to go upstream), with its spectral bands. Omni's `tb_ir` is 3/1/14 in
      template 4.32.
   4. `4c1e90f` Local parameters per originating centre (to go upstream).
      `Message::originating_center`, `IdentificationSection::originating_center`,
      `originating_subcenter` and `local_tables_version`; the parameter lookup
      takes the originator (centre, sub-centre, local tables version), and a
      `LocalTable` in `LOCAL_TABLES` gives one centre's local numbers (192-254)
      their own meanings. `LOCAL_TABLES` is empty in this commit, so every
      message decodes as upstream does (NCEP meanings for every centre).
   5. `d446373` Spire local table (**Spire only, the one change on this branch not
      headed upstream**: it reads centre 255, "missing", as Spire). The Omni
      catalogue's `local_table`, plugged in through item 4.4 and keyed on centre
      255, sub-centre 0 and local tables version 1 (the values in Omni's files):
      0/1/192 FZRA freezing rain accumulation, 0/2/192 LLWS, 0/4/192 DNI
      direct normal irradiance, 0/17/192 LTNG5NM, 0/17/193 LTNG10NM, 0/19/192
      FLTCAT, 0/19/193 FIREWX, 0/19/194 TURBCAT; other local numbers are
      "missing". Every other originator, including centre 255 with another
      sub-centre or local tables version, keeps the NCEP meanings (0/1/192
      CRAIN, 0/4/192 DSWRF under centre 7).
      Fixture `test-data/eccodes-spire-omni-v29-4x3.grib2`.
   Upstream order: item 4.1 and item 4.4 apply cleanly to upstream `main`;
   item 4.2 conflicts in `meteorological.rs` (this branch carries the
   `edr-echo-top-parameters` change) and needs a rebase or to follow the EDR PR;
   item 4.3 goes after item 4.2, as its test reads CBTMP. Item 4.4 uses
   `Option::is_none_or` (Rust 1.82 or later); say so in its PR.
   Omni v2.9's sample files are named `omni_v2.8_*`: v2.9 code writes the
   release name `omni_v2.8`.

## Releasing

Bump `N` in `gribberish/Cargo.toml` and `python/Cargo.toml` (and
`Cargo.lock`), commit, then:

```
git tag v1.8.4+spire.N && git push fork v1.8.4+spire.N
```

`spire-wheels.yml` runs `cargo test`, builds CPython 3.12 wheels for
manylinux x86_64 and aarch64 and macOS x86_64/arm64 plus an sdist, and attaches them to a
GitHub Release for the tag. Nothing goes to PyPI.

## Rule

Every addition on this branch must also go upstream as its own PR. Don't put
anything here that isn't headed upstream; keep each addition a separate merge
so it can be dropped once upstream has it. Exception: the Spire local table
(item 4.5, the table only) stays here until Spire has a WMO centre id; the way
to plug it in (item 4.4) goes upstream.
