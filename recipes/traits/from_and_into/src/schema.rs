pub struct Celsius(pub f64);
pub struct Farenheit(pub f64);

impl From<f64> for Celsius {
    fn from(t: f64) -> Self {
        Celsius(t)
    }
}

impl From<f64> for Farenheit {
    fn from(t: f64) -> Self {
        Farenheit(t)
    }
}

impl Into<f64> for Celsius {
    fn into(self) -> f64 {
        self.0
    }
}

impl Into<f64> for Farenheit {
    fn into(self) -> f64 {
        self.0
    }
}

impl From<Celsius> for Farenheit {
    fn from(c: Celsius) -> Self {
        Farenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<Farenheit> for Celsius {
    fn from(c: Farenheit) -> Self {
        Celsius((c.0 - 32.0) / (9.0 / 5.0))
    }
}
