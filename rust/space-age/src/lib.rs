
const SEC_PER_EARTH_YEAR: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl Duration {
    fn to_earth_years(&self) -> f64 {
        self.seconds as f64 / SEC_PER_EARTH_YEAR
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    const RATIO: f64 = 0.0;

    fn years_during(d: &Duration) -> f64 {
        d.to_earth_years() / Self::RATIO
    }
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

#[macro_export(local_inner_macros)]
macro_rules! planet {
    ($name:ident, $ratio:expr) => {
        pub struct $name;
        impl Planet for $name {
            const RATIO: f64 = $ratio;
        }
    };
}