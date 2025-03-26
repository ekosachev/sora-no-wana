use std::{default, fmt::Display, ops::RangeInclusive};

use bevy::prelude::{default, Bundle, Component};
use rand::{rng, thread_rng, Rng};

use super::common::{Mass, Name, Radius, Temperature, ID};

#[derive(Component, Default)]
pub struct Star;

#[derive(Bundle, Default)]
pub struct StarBundle {
    pub id: ID,
    pub name: Name,
    pub star_type: CStarType,
    pub star_class: CStarClass,
    pub luminosity: Luminosity,
    pub temperature: Temperature,
    pub radius: Radius,
    pub mass: Mass,
    pub star: Star,
}

#[derive(Component, Default)]
pub struct Luminosity(pub f32);

#[derive(PartialEq, Clone, Copy, Default)]
pub enum StarType {
    O,
    B,
    A,
    F,
    G,
    K,
    #[default]
    M,
}

impl Display for StarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::O => "O type (blue)",
            Self::B => "B type (blue)",
            Self::A => "A type (white blue)",
            Self::F => "F type (white yellow)",
            Self::G => "G type (yellow orange)",
            Self::K => "K type (orange red)",
            Self::M => "M type (red)",
        };
        write!(f, "{}", symbol);

        Ok(())
    }
}

#[derive(PartialEq, Default, Clone, Copy)]
pub enum StarClass {
    O,
    I,
    II,
    III,
    IV,
    #[default]
    V,
}

impl Display for StarClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::O => "O class (hypergiant)",
            Self::I => "I class (supergiant)",
            Self::II => "II class (bright giant)",
            Self::III => "III class (giant)",
            Self::IV => "IV class (subdwarf)",
            Self::V => "V class (dwarf)",
        };
        write!(f, "{}", symbol);

        Ok(())
    }
}

#[derive(Component, Default)]
pub struct CStarType(pub StarType);

#[derive(Component, Default)]
pub struct CStarClass(pub StarClass);

impl StarBundle {
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Name(name.to_string());
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Temperature(temperature);
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = Radius(radius);
        self
    }
}

impl Star {
    pub fn generate() -> StarBundle {
        let star_type = Self::get_star_type();
        let star_class = Self::get_star_class(star_type);
        let luminosity = Self::get_luminosity(star_class, star_type);
        let temperature_classification =
            Self::get_temperature_classification(star_class, star_type);
        let temperature = Self::get_temperature(star_type, temperature_classification);
        let mass = Self::get_mass(luminosity);
        let radius = Self::get_radius(luminosity, temperature);

        StarBundle {
            id: ID::default(),
            name: Name("Star".to_string()),
            star_type: CStarType(star_type),
            star_class: CStarClass(star_class),
            temperature: Temperature(temperature),
            radius: Radius(radius),
            luminosity: Luminosity(luminosity),
            mass: Mass(mass),
            star: Star,
        }
    }

    fn get_star_type() -> StarType {
        let mut rng = rng();
        let star_type_proba = rng.random_range(1..=1000);

        match star_type_proba {
            1..=2 => StarType::O,
            3..=4 => StarType::B,
            5..=11 => StarType::A,
            12..=13 => StarType::F,
            14..=68 => StarType::G,
            69..=148 => StarType::K,
            _ => StarType::M,
        }
    }

    fn get_star_class(star_type: StarType) -> StarClass {
        let mut rng = rng();
        let star_class_proba = rng.random_range(1..=1000);

        match star_class_proba {
            1..=2 => {
                if star_type != StarType::O {
                    StarClass::O
                } else {
                    StarClass::V
                }
            }
            3..=10 => StarClass::I,
            11..=60 => StarClass::II,
            61..=190 => StarClass::III,
            191..=200 => {
                if star_type != StarType::M {
                    StarClass::IV
                } else {
                    StarClass::V
                }
            }
            _ => StarClass::V,
        }
    }

    fn get_lum_range(star_class: StarClass, star_type: StarType) -> RangeInclusive<f32> {
        match star_class {
            StarClass::V => Self::get_lum_v(star_type),
            StarClass::IV => Self::get_lum_iv(star_type),
            StarClass::III => Self::get_lum_iii(star_type),
            StarClass::II => Self::get_lum_ii(star_type),
            StarClass::I => Self::get_lum_i(star_type),
            StarClass::O => Self::get_lum_o(star_type),
        }
    }

    fn get_lum_v(star_type: StarType) -> RangeInclusive<f32> {
        match star_type {
            StarType::O => 20_000.0..=800_000.0,
            StarType::B => 80.0..=20_000.0,
            StarType::A => 6.5..=80.0,
            StarType::F => 1.26..=6.5,
            StarType::G => 0.42..=1.26,
            StarType::K => 0.072..=0.42,
            StarType::M => 0.000_015..=0.072,
        }
    }

    fn get_lum_iv(star_type: StarType) -> RangeInclusive<f32> {
        match star_type {
            StarType::O => 80_000.0..=100_000.0,
            StarType::B => 102.0..=34_000.0,
            StarType::A => 13.0..=102.0,
            StarType::F => 9.0..=13.0,
            StarType::G => 8.0..=9.0,
            StarType::K => 8.0..=9.0,
            StarType::M => 9.0..=10.0,
        }
    }

    fn get_lum_iii(star_type: StarType) -> RangeInclusive<f32> {
        match star_type {
            StarType::O => 50_000.0..=300_000.0,
            StarType::B => 170.0..=50_000.0,
            StarType::A => 97.0..=170.0,
            StarType::F => 95.0..=97.0,
            StarType::G => 95.0..=96.0,
            StarType::K => 96.0..=98.0,
            StarType::M => 98.0..=105.0,
        }
    }
    fn get_lum_ii(star_type: StarType) -> RangeInclusive<f32> {
        match star_type {
            StarType::O => 40_000.0..=52_000.0,
            StarType::B => 4000.0..=40000.0,
            StarType::A => 2000.0..=4000.0,
            StarType::F => 960.0..=2000.0,
            StarType::G => 950.0..=960.0,
            StarType::K => 950.0..=1000.0,
            StarType::M => 1000.0..=8000.0,
        }
    }
    fn get_lum_i(star_type: StarType) -> RangeInclusive<f32> {
        match star_type {
            StarType::O => 10_000.0..=1500_000.0,
            StarType::B => 10_000.0..=1500_000.0,
            StarType::A => 1000.0..=100_000.0,
            StarType::F => 1000.0..=100_000.0,
            StarType::G => 20_000.0..=500_000.0,
            StarType::K => 20_000.0..=500_000.0,
            StarType::M => 20_000.0..=500_000.0,
        }
    }
    fn get_lum_o(star_type: StarType) -> RangeInclusive<f32> {
        match star_type {
            StarType::O => 1000_000.0..=5000_000.0,
            StarType::B => 380_000.0..=2000_000.0,
            StarType::A => 300_000.0..=600_000.0,
            StarType::F => 300_000.0..=600_000.0,
            StarType::G => 100_000.0..=500_000.0,
            StarType::K => 100_000.0..=500_000.0,
            StarType::M => 86_000.0..=500_000.0,
        }
    }

    fn get_luminosity(star_class: StarClass, star_type: StarType) -> f32 {
        let mut rng = rng();
        rng.random_range(Self::get_lum_range(star_class, star_type))
    }

    fn get_temperature_classification(star_class: StarClass, star_type: StarType) -> i32 {
        let mut rng = rng();
        let temp_classification = rng.random_range(0..=9);
        if star_type == StarType::O && star_class == StarClass::V {
            return temp_classification.max(5);
        }
        if star_type == StarType::M {
            return temp_classification.min(6);
        }
        temp_classification
    }

    fn get_temp_maxmin(star_type: StarType) -> (f32, f32) {
        match star_type {
            StarType::O => (54_000.0, 33_200.0),
            StarType::B => (29_700.0, 10_700.0),
            StarType::A => (9790.0, 7323.0),
            StarType::F => (7300.0, 6033.0),
            StarType::G => (5940.0, 5335.0),
            StarType::K => (5150.0, 3880.0),
            StarType::M => (3840.0, 2376.0),
        }
    }

    fn get_temp_coeff(star_type: StarType) -> f32 {
        match star_type {
            StarType::O => 2400.0,
            StarType::B => 2111.0,
            StarType::A => 274.0,
            StarType::F => 141.0,
            StarType::G => 67.0,
            StarType::K => 141.0,
            StarType::M => 165.0,
        }
    }

    fn get_temperature(star_type: StarType, temp_classification: i32) -> f32 {
        let (temp_max, temp_min) = Self::get_temp_maxmin(star_type);
        let temp_coeff = Self::get_temp_coeff(star_type);
        let surfaceTemp = temp_min + (temp_coeff * (9.0 - temp_classification as f32));
        let mut rng = rng();

        if temp_classification == 0 {
            return surfaceTemp + rng.random_range(-temp_coeff / 6.0..=temp_coeff / 6.0);
        }

        return surfaceTemp + rng.random_range(-temp_coeff / 2.0..=temp_coeff / 2.0);
    }

    fn get_mass(luminosity: f32) -> f32 {
        if luminosity < 0.03418801 {
            return luminosity.powf(1.0 / 2.3);
        }
        if luminosity < 16.97056275 {
            return luminosity.powf(1.0 / 4.0);
        }
        if luminosity < 64_000.0 {
            return (luminosity / 1.5).powf(1.0 / 3.5);
        }
        if luminosity <= 160_000.0 {
            return (luminosity / 3200.0).powf(1.0);
        }
        (luminosity / 3200.0).powf(1.0 / 1.25)
    }

    fn get_radius(luminosity: f32, surface_temp: f32) -> f32 {
        (luminosity / (surface_temp / 5766.0).powi(4)).sqrt()
    }
}
