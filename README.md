# use-optics

Optics primitives for spectra, wavelengths, and visible-light boundaries.

`use-optics` is the natural optics sibling to `use-color` and `use-wave`. It starts with compact
helpers for classifying wavelengths, working with visible-light ranges, and turning spectral data
points into small Rust values.

The scope is intentionally narrow for v0.1: auditable wavelength primitives first, richer optical
models later.

## Workspace crates

| Crate        | Purpose                                           |
| ------------ | ------------------------------------------------- |
| `use-optics` | Wavelength classification and small spectral data |

## Installation

```toml
[dependencies]
use-optics = "0.1.0"
```

## Usage

```rust
use use_optics::{classify_wavelength_nm, SpectralBand};

assert_eq!(classify_wavelength_nm(532.0), Some(SpectralBand::Visible));
```

## Scope

- Visible-range helpers and spectral classification.
- Small wavelength-centric value types.
- No ray-tracing, lens design, or color appearance modeling yet.
