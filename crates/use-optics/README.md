# use-optics

Small wavelength and spectral helpers for RustUse.

`use-optics` provides visible-spectrum boundaries, wavelength classification helpers, and a compact
`SpectralSample` value type for lightweight optical calculations.

## What this crate provides

| Item                           | Purpose                                           |
| ------------------------------ | ------------------------------------------------- |
| `SpectralBand`                 | Ultraviolet, visible, and infrared classification |
| `VISIBLE_MIN_NM` / `MAX_NM`    | Visible-light range constants                     |
| `classify_wavelength_nm()`     | Wavelength band classification                    |
| `wavelength_to_frequency_hz()` | Frequency helper for a wavelength in nanometers   |
| `SpectralSample`               | Small wavelength-plus-intensity value type        |

## Installation

```toml
[dependencies]
use-optics = "0.1.0"
```

## Example

```rust
use use_optics::{is_visible_wavelength_nm, SpectralSample, SpectralBand};

let sample = SpectralSample::new(450.0, 0.8);

assert_eq!(sample.band(), Some(SpectralBand::Visible));
assert!(is_visible_wavelength_nm(sample.wavelength_nm));
```

## Scope

- Wavelength-first helpers.
- Small immutable values.
- No polarization, interference, or rendering pipeline yet.
