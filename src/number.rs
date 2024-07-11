use std::str::FromStr;
use std::num::ParseFloatError;

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    SIGNED(i64),
    UNSIGNED(u64),
    FLOAT(f64),
}

impl Number {
    pub fn add(&self, other: &Number) -> Number {
        match (self, other) {
            (Number::SIGNED(a), Number::SIGNED(b)) => Number::SIGNED(a + b),
            (Number::UNSIGNED(a), Number::UNSIGNED(b)) => Number::UNSIGNED(a + b),
            (Number::FLOAT(a), Number::FLOAT(b)) => Number::FLOAT(a + b),
            (Number::SIGNED(a), Number::UNSIGNED(b)) => Number::SIGNED(*a + *b as i64),
            (Number::UNSIGNED(a), Number::SIGNED(b)) => Number::SIGNED(*a as i64 + *b),
            (Number::SIGNED(a), Number::FLOAT(b)) => Number::FLOAT(*a as f64 + b),
            (Number::UNSIGNED(a), Number::FLOAT(b)) => Number::FLOAT(*a as f64 + b),
            (Number::FLOAT(a), Number::SIGNED(b)) => Number::FLOAT(a + *b as f64),
            (Number::FLOAT(a), Number::UNSIGNED(b)) => Number::FLOAT(a + *b as f64),
        }
    }

    pub fn sub(&self, other: &Number) -> Number {
        match (self, other) {
            (Number::SIGNED(a), Number::SIGNED(b)) => Number::SIGNED(a - b),
            (Number::UNSIGNED(a), Number::UNSIGNED(b)) => Number::UNSIGNED(a - b),
            (Number::FLOAT(a), Number::FLOAT(b)) => Number::FLOAT(a - b),
            (Number::SIGNED(a), Number::UNSIGNED(b)) => Number::SIGNED(*a - *b as i64),
            (Number::UNSIGNED(a), Number::SIGNED(b)) => Number::SIGNED(*a as i64 - *b),
            (Number::SIGNED(a), Number::FLOAT(b)) => Number::FLOAT(*a as f64 - b),
            (Number::UNSIGNED(a), Number::FLOAT(b)) => Number::FLOAT(*a as f64 - b),
            (Number::FLOAT(a), Number::SIGNED(b)) => Number::FLOAT(a - *b as f64),
            (Number::FLOAT(a), Number::UNSIGNED(b)) => Number::FLOAT(a - *b as f64),
        }
    }

    pub fn mul(&self, other: &Number) -> Number {
        match (self, other) {
            (Number::SIGNED(a), Number::SIGNED(b)) => Number::SIGNED(a * b),
            (Number::UNSIGNED(a), Number::UNSIGNED(b)) => Number::UNSIGNED(a * b),
            (Number::FLOAT(a), Number::FLOAT(b)) => Number::FLOAT(a * b),
            (Number::SIGNED(a), Number::UNSIGNED(b)) => Number::SIGNED(*a * *b as i64),
            (Number::UNSIGNED(a), Number::SIGNED(b)) => Number::SIGNED(*a as i64 * *b),
            (Number::SIGNED(a), Number::FLOAT(b)) => Number::FLOAT(*a as f64 * b),
            (Number::UNSIGNED(a), Number::FLOAT(b)) => Number::FLOAT(*a as f64 * b),
            (Number::FLOAT(a), Number::SIGNED(b)) => Number::FLOAT(a * *b as f64),
            (Number::FLOAT(a), Number::UNSIGNED(b)) => Number::FLOAT(a * *b as f64),
        }
    }

    pub fn div(&self, other: &Number) -> Number {
        match (self, other) {
            (Number::SIGNED(a), Number::SIGNED(b)) => Number::SIGNED(a / b),
            (Number::UNSIGNED(a), Number::UNSIGNED(b)) => Number::UNSIGNED(a / b),
            (Number::FLOAT(a), Number::FLOAT(b)) => Number::FLOAT(a / b),
            (Number::SIGNED(a), Number::UNSIGNED(b)) => Number::SIGNED(*a / *b as i64),
            (Number::UNSIGNED(a), Number::SIGNED(b)) => Number::SIGNED(*a as i64 / *b),
            (Number::SIGNED(a), Number::FLOAT(b)) => Number::FLOAT(*a as f64 / b),
            (Number::UNSIGNED(a), Number::FLOAT(b)) => Number::FLOAT(*a as f64 / b),
            (Number::FLOAT(a), Number::SIGNED(b)) => Number::FLOAT(a / *b as f64),
            (Number::FLOAT(a), Number::UNSIGNED(b)) => Number::FLOAT(a / *b as f64),
        }
    }
}

impl From<i64> for Number {
    fn from(i: i64) -> Self {
        Number::SIGNED(i)
    }
}

impl From<u64> for Number {
    fn from(u: u64) -> Self {
        Number::UNSIGNED(u)
    }
}

impl From<f64> for Number {
    fn from(f: f64) -> Self {
        Number::FLOAT(f)
    }
}

impl From<Number> for i64 {
    fn from(n: Number) -> Self {
        match n {
            Number::SIGNED(i) => i,
            Number::UNSIGNED(u) => u as i64,
            Number::FLOAT(f) => f as i64,
        }
    }
}

impl From<Number> for u64 {
    fn from(n: Number) -> Self {
        match n {
            Number::SIGNED(i) => i as u64,
            Number::UNSIGNED(u) => u,
            Number::FLOAT(f) => f as u64,
        }
    }
}

impl From<Number> for f64 {
    fn from(n: Number) -> Self {
        match n {
            Number::SIGNED(i) => i as f64,
            Number::UNSIGNED(u) => u as f64,
            Number::FLOAT(f) => f,
        }
    }
}

impl FromStr for Number {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(u) = s.parse::<u64>() {
            Ok(Number::UNSIGNED(u))
        } else if let Ok(i) = s.parse::<i64>() {
            Ok(Number::SIGNED(i))
        } else if let Ok(f) = s.parse::<f64>() {
            Ok(Number::FLOAT(f))
        } else {
            Err(s.parse::<f64>().unwrap_err())
        }
    }
}