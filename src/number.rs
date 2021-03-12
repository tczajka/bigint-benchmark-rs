use std::{
    fmt::Display,
    ops::{Add, Div, Mul},
};

pub(crate) trait Number
where
    Self: Sized,
    Self: From<u32>,
    Self: Display,
    Self: Add<Self, Output = Self>,
    for<'a> Self: Add<&'a Self, Output = Self>,
    Self: Mul<Self, Output = Self>,
    for<'a> Self: Mul<&'a Self, Output = Self>,
    Self: Div<Self, Output = Self>,
{
    fn pow(&self, exp: u32) -> Self;
}

impl Number for ibig::UBig {
    fn pow(&self, exp: u32) -> Self {
        self.pow(exp as usize)
    }
}

impl Number for num_bigint::BigUint {
    fn pow(&self, exp: u32) -> Self {
        self.pow(exp)
    }
}

impl Number for ramp::Int {
    fn pow(&self, exp: u32) -> Self {
        self.pow(exp as usize)
    }
}

impl Number for rug::Integer {
    fn pow(&self, exp: u32) -> Self {
        rug::ops::Pow::pow(self, exp).into()
    }
}

impl Number for gmp::mpz::Mpz {
    fn pow(&self, exp: u32) -> Self {
        self.pow(exp)
    }
}
