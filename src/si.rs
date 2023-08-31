//! All SI base units and more constants.

#![allow(non_upper_case_globals)]

/// Helper macro to generate two constants for every unit,
/// a unit const, and a value const in two different modules.
macro_rules! units_impl {
    ($(
        $(#[$attr:meta])*
        $name:ident => $unit:expr
    ),*$(,)?) => {
    };
}

units_impl! {
    /// Time in seconds
    s => Unit { s: 1, ..NONE },
    /// Length in metre
    m => Unit { m: 1, ..NONE },
    /// Mass in kilogram
    kg => Unit { kg: 1, ..NONE },
    /// Electric current in ampere
    A => Unit { A: 1, ..NONE },
    /// Temperature in kelvin
    K => Unit { K: 1, ..NONE },
    /// Amount of substance in mole
    mol => Unit { mol: 1, ..NONE },
    /// Luminous intensity in candela
    cd => Unit { cd: 1, ..NONE },
}
