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
so it can be dropped once upstream has it.
