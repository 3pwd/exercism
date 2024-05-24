const ONE_YEAR_DAYS: f64 = 365.25;
const ONE_EARTH_YEAR_SECONDS: f64 = ONE_YEAR_DAYS * 24.0 * 60.0 * 60.0;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 / (Self::ORBITAL_PERIOD * ONE_EARTH_YEAR_SECONDS)
    }
}

macro_rules! planets {
    ($($n:ident $p:expr);*) => {
        $(pub struct $n;
        impl Planet for $n {
            const ORBITAL_PERIOD: f64 = $p;
        })*
    };
}

planets!(
    Mercury 0.2408467;
    Venus 0.61519726;
    Earth 1.0;
    Mars 1.8808158;
    Jupiter 11.862615;
    Saturn 29.447498;
    Uranus 84.016846;
    Neptune 164.79132
);
