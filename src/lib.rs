//! `tiny-uom` is a small version of the [`uom`] library.
//!
//! This crate is mostly based on [this] proof-of-concept using const generics.
//! `tiny-uom` is a port of `uom` to use const generics and to be a faster and smaller version.
//! It provides type-safe and zero-cost [dimensional-analysis].
//! `tiny-uom` provides all units that are specified in the [International System of Units][SI]
//! and all quantities that are specified in the [International System of Quantities][ISQ].
//!
//! ## Usage
//! ```
//! // use tiny_uom::values::{kg, m, s};
//!
//! # fn main() {
//! // let distance = 10.0 * m;
//! // let time = 2.0 * s;
//!
//! // let velocity = distance / time;
//! // assert_eq!(velocity, 5.0 * (m / s));
//! # }
//! ```
//!
//! [`uom`]: https://docs.rs/uom
//! [this]: https://docs.rs/const_unit_poc
//! [dimensional-analysis]: https://en.wikipedia.org/wiki/Dimensional_analysis
//! [SI]: https://jcgm.bipm.org/vim/en/1.16.html
//! [ISQ]: https://jcgm.bipm.org/vim/en/1.6.html
#![deny(
    rust_2021_compatibility,
    warnings,
    clippy::pedantic,
    missing_docs,
    missing_debug_implementations,
    rustdoc::broken_intra_doc_links,
    unsafe_code
)]
#![allow(non_upper_case_globals)]

use std::clone::Clone;

// pub use si::{units, values};

mod si;

/// The `Unit` struct can represent every possible unit
/// that is defined in the [`SI`] system.
///
/// It is able to do so because it contains a list of all
/// 7 base units and a number which represents the exponent
/// of that unit.
///
/// # Example
///
/// ## Newton
/// ```no_rust
/// kg * m * s⁻²
/// ```
///
/// would be represented using the following `Unit`:
/// ```no_rust
/// Unit {
///     m: 1,
///     kg: 1,
///     s: -2,
/// }
/// ```
///
/// [`SI`]: https://jcgm.bipm.org/vim/en/1.16.html

/// Implement all methods and traits for a quantity type.
macro_rules! quantity_impl {
    ($backing_ty:ty, $quantity:ident, $unit_exp_ty:ty, $($unit:ident),+) => {
        /// A `Quantity` represents a raw value and it's unit
        /// that is represented as a const generic parameter.
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(transparent)]
        pub struct $quantity<$(const $unit: $unit_exp_ty,)*> {
            /// The raw value of this `Quantity`
            pub value: $backing_ty,
        }
        impl<$(const $unit: $unit_exp_ty,)*> $quantity<$($unit,)*> {
            /// Create a new `Quantity` with the given value.
            #[must_use]
            pub const fn new(value: $backing_ty) -> Self {
                Self { value }
            }
        }

        impl<$(const $unit: $unit_exp_ty,)*> ::std::fmt::Display for $quantity<$($unit,)*> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{} * {:?}", self.value, &[$($unit,)*])
            }
        }

        // ============================
        // Add implementations
        // ============================
        impl<$(const $unit: $unit_exp_ty,)*> ::std::ops::Add<$quantity<$($unit,)*>> for $quantity<$($unit,)*> {
            type Output = Self;

            /// Add the value of two equal units.
            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    value: self.value + rhs.value,
                }
            }
        }

        impl<$(const $unit: $unit_exp_ty,)*> ::std::ops::AddAssign<$quantity<$($unit,)*>> for $quantity<$($unit,)*> {
            /// Add the value of two equal units.
            fn add_assign(&mut self, rhs: Self) {
                self.value += rhs.value;
            }
        }

        // ============================
        // Sub implementations
        // ============================
        impl<$(const $unit: $unit_exp_ty,)*> ::std::ops::Sub<$quantity<$($unit,)*>> for $quantity<$($unit,)*> {
            type Output = Self;

            /// Subtract the value of two equal units.
            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    value: self.value - rhs.value,
                }
            }
        }

        impl<$(const $unit: $unit_exp_ty,)*> ::std::ops::SubAssign<$quantity<$($unit,)*>> for $quantity<$($unit,)*> {
            /// Subtract the value of two equal units.
            fn sub_assign(&mut self, rhs: Self) {
                self.value -= rhs.value;
            }
        }

        // ============================
        // Mul implementations
        // ============================
        impl<$(const $unit: $unit_exp_ty,)*> ::std::ops::Mul<$backing_ty> for $quantity<$($unit,)*> {
            type Output = Self;

            /// Multiply the value of this unit with a number.
            fn mul(self, rhs: $backing_ty) -> Self::Output {
                Self {
                    value: self.value * rhs,
                }
            }
        }

        impl<$(const $unit: $unit_exp_ty,)*> ::std::ops::Mul<$quantity<$($unit,)*>> for $backing_ty {
            type Output = $quantity<$($unit,)*>;

            /// Multiply the value of this unit with a number.
            fn mul(self, rhs: $quantity<$($unit,)*>) -> Self::Output {
                $quantity {
                    value: self * rhs.value,
                }
            }
        }

        impl<$(const $unit: $unit_exp_ty,)*> ::std::ops::MulAssign<$backing_ty> for $quantity<$($unit,)*> {
            /// Multiply the value of this unit with a number.
            fn mul_assign(&mut self, rhs: $backing_ty) {
                self.value *= rhs;
            }
        }

        // ============================
        // Div implementations
        // ============================
        impl<$(const $unit: $unit_exp_ty,)*> ::std::ops::Div<$backing_ty> for $quantity<$($unit,)*> {
            type Output = Self;

            /// Divides the value of this unit with a number.
            fn div(self, rhs: $backing_ty) -> Self::Output {
                Self {
                    value: self.value / rhs,
                }
            }
        }

        impl<$(const $unit: $unit_exp_ty,)*> ::std::ops::DivAssign<$backing_ty> for $quantity<$($unit,)*> {
            /// Divides the value of this unit with a number.
            fn div_assign(&mut self, rhs: $backing_ty) {
                self.value /= rhs;
            }
        }
    };
}
quantity_impl!(f32, Quantity, i8, m, kg, s, A, K, mol, cd);
