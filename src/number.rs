use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

pub(crate) trait Number
where
    Self: Sized,
    Self: From<u32>,
    Self: Display,
    Self: Add<Self, Output = Self>,
    Self: for<'a> Add<&'a Self, Output = Self>,
    Self: Sub<Self, Output = Self>,
    Self: for<'a> Sub<&'a Self, Output = Self>,
    Self: Mul<Self, Output = Self>,
    Self: for<'a> Mul<&'a Self, Output = Self>,
    Self: Div<Self, Output = Self>,
    Self: for<'a> Div<&'a Self, Output = Self>,
{
    fn pow(&self, exp: u32) -> Self;
    fn to_hex(&self) -> String;
    fn mul_ref(&self, rhs: &Self) -> Self;
}

impl Number for ibig::UBig {
    fn pow(&self, exp: u32) -> Self {
        self.pow(exp as usize)
    }

    fn to_hex(&self) -> String {
        format!("{:x}", self)
    }

    fn mul_ref(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Number for num_bigint::BigUint {
    fn pow(&self, exp: u32) -> Self {
        self.pow(exp)
    }

    fn to_hex(&self) -> String {
        format!("{:x}", self)
    }

    fn mul_ref(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Number for rug::Integer {
    fn pow(&self, exp: u32) -> Self {
        rug::ops::Pow::pow(self, exp).into()
    }

    fn to_hex(&self) -> String {
        format!("{:x}", self)
    }

    fn mul_ref(&self, rhs: &Self) -> Self {
        (self * rhs).into()
    }
}

impl Number for malachite::natural::Natural {
    fn pow(&self, exp: u32) -> Self {
        malachite::num::arithmetic::traits::Pow::pow(self, exp.into())
    }

    fn to_hex(&self) -> String {
        malachite::strings::ToLowerHexString::to_lower_hex_string(self)
    }

    fn mul_ref(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Number for dashu::Natural {
    fn pow(&self, exp: u32) -> Self {
        self.pow(exp as usize)
    }

    fn to_hex(&self) -> String {
        format!("{:x}", self)
    }

    fn mul_ref(&self, rhs: &Self) -> Self {
        self * rhs
    }
}
