#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Speed of light in meters per second.
pub const SPEED_OF_LIGHT_MPS: f64 = 299_792_458.0;

/// Lower bound of the conventional visible spectrum in nanometers.
pub const VISIBLE_MIN_NM: f64 = 380.0;

/// Upper bound of the conventional visible spectrum in nanometers.
pub const VISIBLE_MAX_NM: f64 = 780.0;

/// Coarse spectral bands used by the scaffold API.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectralBand {
    Ultraviolet,
    Visible,
    Infrared,
}

impl SpectralBand {
    /// Returns a lowercase label for the spectral band.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ultraviolet => "ultraviolet",
            Self::Visible => "visible",
            Self::Infrared => "infrared",
        }
    }
}

/// A simple wavelength-plus-intensity sample.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpectralSample {
    pub wavelength_nm: f64,
    pub intensity: f64,
}

impl SpectralSample {
    /// Creates a new spectral sample.
    #[must_use]
    pub const fn new(wavelength_nm: f64, intensity: f64) -> Self {
        Self {
            wavelength_nm,
            intensity,
        }
    }

    /// Returns the coarse spectral band for the sample wavelength.
    #[must_use]
    pub fn band(self) -> Option<SpectralBand> {
        classify_wavelength_nm(self.wavelength_nm)
    }
}

/// Classifies a wavelength in nanometers into a coarse spectral band.
#[must_use]
pub fn classify_wavelength_nm(wavelength_nm: f64) -> Option<SpectralBand> {
    if !wavelength_nm.is_finite() || wavelength_nm <= 0.0 {
        return None;
    }

    if wavelength_nm < VISIBLE_MIN_NM {
        Some(SpectralBand::Ultraviolet)
    } else if wavelength_nm <= VISIBLE_MAX_NM {
        Some(SpectralBand::Visible)
    } else {
        Some(SpectralBand::Infrared)
    }
}

/// Returns whether the wavelength falls inside the conventional visible range.
#[must_use]
pub fn is_visible_wavelength_nm(wavelength_nm: f64) -> bool {
    matches!(
        classify_wavelength_nm(wavelength_nm),
        Some(SpectralBand::Visible)
    )
}

/// Clamps a valid wavelength into the visible range.
#[must_use]
pub fn clamp_visible_wavelength_nm(wavelength_nm: f64) -> Option<f64> {
    if !wavelength_nm.is_finite() || wavelength_nm <= 0.0 {
        return None;
    }

    Some(wavelength_nm.clamp(VISIBLE_MIN_NM, VISIBLE_MAX_NM))
}

/// Converts a wavelength in nanometers to frequency in hertz.
#[must_use]
pub fn wavelength_to_frequency_hz(wavelength_nm: f64) -> Option<f64> {
    if !wavelength_nm.is_finite() || wavelength_nm <= 0.0 {
        return None;
    }

    Some(SPEED_OF_LIGHT_MPS / (wavelength_nm * 1.0e-9))
}

/// Common optics primitives.
pub mod prelude {
    pub use super::{
        SPEED_OF_LIGHT_MPS, SpectralBand, SpectralSample, VISIBLE_MAX_NM, VISIBLE_MIN_NM,
        clamp_visible_wavelength_nm, classify_wavelength_nm, is_visible_wavelength_nm,
        wavelength_to_frequency_hz,
    };
}

#[cfg(test)]
mod tests {
    use super::{
        SpectralBand, SpectralSample, VISIBLE_MAX_NM, VISIBLE_MIN_NM, clamp_visible_wavelength_nm,
        classify_wavelength_nm, is_visible_wavelength_nm, wavelength_to_frequency_hz,
    };

    #[test]
    fn classifies_common_wavelength_ranges() {
        assert_eq!(
            classify_wavelength_nm(350.0),
            Some(SpectralBand::Ultraviolet)
        );
        assert_eq!(classify_wavelength_nm(532.0), Some(SpectralBand::Visible));
        assert_eq!(classify_wavelength_nm(900.0), Some(SpectralBand::Infrared));
        assert_eq!(classify_wavelength_nm(0.0), None);
    }

    #[test]
    fn detects_visible_wavelengths() {
        assert!(is_visible_wavelength_nm(VISIBLE_MIN_NM));
        assert!(is_visible_wavelength_nm(VISIBLE_MAX_NM));
        assert!(!is_visible_wavelength_nm(300.0));
    }

    #[test]
    fn clamps_into_visible_range() {
        assert_eq!(clamp_visible_wavelength_nm(300.0), Some(VISIBLE_MIN_NM));
        assert_eq!(clamp_visible_wavelength_nm(500.0), Some(500.0));
        assert_eq!(clamp_visible_wavelength_nm(900.0), Some(VISIBLE_MAX_NM));
    }

    #[test]
    fn converts_wavelength_to_frequency() {
        let frequency = wavelength_to_frequency_hz(500.0).expect("500 nm should be valid");

        assert!((frequency - 5.995_849_16e14).abs() < 1.0e7);
    }

    #[test]
    fn sample_reports_its_band() {
        let sample = SpectralSample::new(450.0, 0.75);

        assert_eq!(sample.band(), Some(SpectralBand::Visible));
    }
}
