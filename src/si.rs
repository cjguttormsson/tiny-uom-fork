//! All SI base units and more constants.

#![allow(non_upper_case_globals, dead_code)]


// TODO: Use a macro to parameterize this?
/// Constants for the multiplicative identities of each unit
pub mod values {
    use crate::Quantity;

    /// Time in seconds
    pub const s: Quantity<0, 0, 1, 0, 0, 0, 0> = Quantity { value: 1.0 };
    /// Length in metre
    pub const m: Quantity<1, 0, 0, 0, 0, 0, 0> = Quantity { value: 1.0 };
    /// Mass in kilogram
    pub const kg: Quantity<0, 1, 0, 0, 0, 0, 0> = Quantity { value: 1.0 };
    /// Electric current in ampere
    pub const A: Quantity<0, 0, 0, 0, 0, 0, 0> = Quantity { value: 1.0 };
    /// Temperature in kelvin
    pub const K: Quantity<0, 0, 0, 0, 0, 0, 0> = Quantity { value: 1.0 };
    /// Amount of substance in mole
    pub const mol: Quantity<0, 0, 0, 0, 0, 0, 0> = Quantity { value: 1.0 };
    /// Luminous intensity in candela
    pub const cd: Quantity<0, 0, 0, 0, 0, 0, 0> = Quantity { value: 1.0 };
}
