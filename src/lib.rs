//! This library implements a `Money` datatype that supports both a
//! statically-typed and dynamically-typed `Currency`. That is to say,
//! you can create a `Money<USD>` that is a totally different type than
//! a `Money<JPY>`, or you can create a `Money<&dyn Currency>` where
//! the currency is determined at runtime, but still safely do math with
//! it (i.e., `Money<&dyn Currency> + Money<&dyn Currency>` returns a
//! fallible `Result` because the currencies might be different).
//!
//! For example:
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use rust_decimal::Decimal;
//! use doubloon::{
//!     {Money, Currency, MoneyMathError},
//!     iso_currencies::{USD, JPY, EUR},
//!     currency_map::CurrencyMap,
//! };
//!
//! // Instances with statically-typed currencies.
//! let m_usd = Money::new(Decimal::ONE, USD);
//! let m_jpy = Money::new(Decimal::ONE, JPY);
//!
//! // This won't even compile because they are two different types.
//! // let no_compile = m_usd + m_jpy;
//!
//! // But you can add same currencies together.
//! assert_eq!(
//!   m_usd + m_usd,
//!   Money::new(Decimal::TWO, USD)
//! );
//!
//! // If you don't know the currency until runtime, just use a
//! // dynamically-typed Currency (&dyn Currency).
//! let currency_map = CurrencyMap::from_collection(vec![&USD as &dyn Currency, &JPY]);
//! let m_dyn_usd = Money::new(Decimal::ONE, currency_map.get("USD").unwrap());
//! let m_dyn_jpy = Money::new(Decimal::ONE, currency_map.get("JPY").unwrap());
//!
//! // Adding same currencies produces an Ok Result.
//! assert_eq!(
//!     m_dyn_usd + m_dyn_usd,
//!     Ok(Money::new(Decimal::TWO, currency_map.get("USD").unwrap()))
//! );
//!
//! // Adding different currencies produces an Err Result.
//! assert_eq!(
//!     m_dyn_usd + m_dyn_jpy,
//!     Err(MoneyMathError::IncompatibleCurrencies("USD", "JPY"))
//! );
//!
//! // Locale-aware formatting is provided via the icu crate
//! // when the "formatting" feature of this crate is enabled.
//! # #[cfg(feature = "formatting")]
//! use icu::locale::locale;
//! let m = Money::new(Decimal::new(123456789, 2), EUR);
//! // en-US uses comma for group separator, period for decimal separator,
//! // with the symbol at the left with no spacing.
//! # #[cfg(feature = "formatting")]
//! assert_eq!(m.format(locale!("en-US")), "€1,234,567.89");
//!
//! // ir-IR is like en-US except there is a narrow non-breaking space
//! // between the symbol and the amount.
//! # #[cfg(feature = "formatting")]
//! assert_eq!(m.format(locale!("ir-IR")), "€\u{a0}1,234,567.89");
//!
//! // tr-TR is similar to ir-IR but uses period for the group separator
//! // and comma for the decimal separator.
//! # #[cfg(feature = "formatting")]
//! assert_eq!(m.format(locale!("tr-TR")), "€1.234.567,89");
//!
//! // fr-FR puts the symbol at the end, and uses non-breaking spaces
//! // between digit groups, comma as a decimal separator,
//! // and a narrow non-breaking space between the amount and symbol.
//! # #[cfg(feature = "formatting")]
//! assert_eq!(
//!     m.format(locale!("fr-FR")),
//!     "1\u{202f}234\u{202f}567,89\u{a0}€"
//! );
//!
//! # Ok(())
//! # }
//! ```

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use rust_decimal::{Decimal, MathematicalOps};
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{ser::SerializeStruct, Serialize};

/// Strategies for use with the [Money::round] method.
pub use rust_decimal::RoundingStrategy;

pub mod currency_map;
pub mod iso_currencies;

#[cfg(feature = "formatting")]
pub mod formatting;

/// Common trait for all currencies.
pub trait Currency {
    /// Returns the unique ISO alphabetic code for this currency
    /// (e.g., "USD" or "JPY").
    fn code(&self) -> &'static str;
    /// Returns the number of minor units supported by the currency.
    /// Currencies like USD and EUR currently support 2, but others
    /// like JPY or KRW support zero.
    fn minor_units(&self) -> u32;
    /// Returns the unique ISO numeric code for this currency.
    fn numeric_code(&self) -> u32;
}

/// Debug output for a dynamically-typed Currency.
/// Only prints the code since that is unique.
impl std::fmt::Debug for &dyn Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Currency")
            .field("code", &self.code())
            .finish()
    }
}

/// Allows comparing dynamically-typed Currency instances.
/// They are equal of their `code()` methods return the same value.
impl PartialEq for &dyn Currency {
    fn eq(&self, other: &Self) -> bool {
        self.code() == other.code()
    }
}

/// Used as a trait bound when constructing new instances of Money
/// from minor units.
pub trait MinorUnits {
    fn minor_units(&self) -> u32;
}

/// Blanket implementation of [MinorUnits] for any static [Currency] instance.
impl<C> MinorUnits for C
where
    C: Currency,
{
    fn minor_units(&self) -> u32 {
        self.minor_units()
    }
}

/// Blanket implementation of [MinorUnits] for an `&dyn Currency`.
impl MinorUnits for &dyn Currency {
    fn minor_units(&self) -> u32 {
        (*self).minor_units()
    }
}

/// An amount of money in a particular currency.
///
/// Money instances are immutable. All operations that would
/// alter the state return a new instance with that new state,
/// leaving the original instance unaltered.
///
/// Money instances also support Copy semantics. The amount
/// Decimal is 128 bits, but statically-typed Currency implementations
/// are typically unit structs, so they don't add any more. References
/// to a dynamic currency add the size of a pointer.
#[derive(Debug, Clone, Copy)]
pub struct Money<C> {
    amount: Decimal,
    currency: C,
}

/// Common functions for statically and dynamically-typed currencies.
impl<C> Money<C>
where
    C: Copy,
{
    /// Constructs a new Money given a decimal amount and Currency.
    /// The currency argument can be either an owned statically-typed
    /// Currency instance, or a dynamically-typed reference
    /// to a Currency instance (i.e., `&dyn Currency`).
    pub fn new(amount: Decimal, currency: C) -> Self {
        Self { amount, currency }
    }

    /// Returns a copy of the amount as a Decimal.
    pub fn amount(&self) -> Decimal {
        self.amount
    }

    /// Returns true if the amount is zero.
    pub fn is_zero(&self) -> bool {
        self.amount.is_zero()
    }

    /// Returns true if the amount is positive.
    pub fn is_positive(&self) -> bool {
        self.amount.is_sign_positive()
    }

    /// Returns true if the amount is negative.
    pub fn is_negative(&self) -> bool {
        self.amount.is_sign_negative()
    }

    /// Returns a new instance raised to the specified power.
    pub fn pow(&self, exponent: i64) -> Self {
        Self {
            amount: self.amount.powi(exponent),
            currency: self.currency,
        }
    }

    /// Returns a new instance rounded to the specified number
    /// of decimal places, using the specified strategy.
    pub fn round(&self, decimal_places: u32, strategy: RoundingStrategy) -> Self {
        Self {
            amount: self.amount.round_dp_with_strategy(decimal_places, strategy),
            currency: self.currency,
        }
    }
}

/// Methods that require knowing the `minor_units` of the currency.
impl<C> Money<C>
where
    C: MinorUnits,
{
    /// Constructs a Money from some number of minor units in the
    /// specified Currency. For example, 100 USD minor units is one USD,
    /// but 100 JPY minor units is 100 JPY.
    pub fn from_minor_units(minor_units: i64, currency: C) -> Self {
        Self {
            amount: Decimal::new(minor_units, currency.minor_units()),
            currency,
        }
    }
}

/// Functions specifically for owned statically-typed Currency instances.
impl<C> Money<C>
where
    C: Currency + Copy, // owned Currency instances can be Copy
{
    /// Returns a copy of the Money's Currency.
    pub fn currency(&self) -> C {
        self.currency
    }
}

/// Functions specifically for borrowed dynamically-typed currencies.
impl Money<&dyn Currency> {
    /// Returns the reference to the dynamically-typed Currency.
    pub fn currency(&self) -> &dyn Currency {
        self.currency
    }
}

/// Allows equality comparisons between Money instances with statically-typed
/// currencies. The compiler will already ensure that `C` is the same for
/// both instances, so only the amounts must match.
impl<C> PartialEq for Money<C>
where
    C: Currency + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

/// Allows equality comparisons between Money instances with dynamically-typed
/// currencies and those with statically-typed currencies. Both the amounts
/// and the currency codes must match.
impl<C> PartialEq<Money<&dyn Currency>> for Money<C>
where
    C: Currency + PartialEq,
{
    fn eq(&self, other: &Money<&dyn Currency>) -> bool {
        self.amount == other.amount && self.currency.code() == other.currency.code()
    }
}

/// Allows equality comparisons between Money instances with dynamically-typed
/// currencies. Both the amounts and currency codes must match.
impl PartialEq for Money<&dyn Currency> {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency.code() == other.currency.code()
    }
}

/// Allows equality comparisons between Money instances with dynamically-typed
/// currencies and those with statically-typed currencies. Both the amounts
/// and currency codes must match.
impl<C> PartialEq<Money<C>> for Money<&dyn Currency>
where
    C: Currency,
{
    fn eq(&self, other: &Money<C>) -> bool {
        self.amount == other.amount && self.currency.code() == other.currency.code()
    }
}

/// Errors that can occur when doing math with Money instances that
/// have dynamically-typed currencies
#[derive(Debug, Error, PartialEq, Clone)]
pub enum MoneyMathError {
    #[error("the money instances have incompatible currencies ({0}, {1})")]
    IncompatibleCurrencies(&'static str, &'static str),
}

macro_rules! impl_binary_op {
    ($trait:ident, $method:ident) => {
        /// Supports $trait for Money instances with the same statically-typed currency.
        impl<C> $trait for Money<C>
        where
            C: Currency,
        {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                Self {
                    amount: self.amount.$method(rhs.amount),
                    currency: self.currency,
                }
            }
        }

        /// Supports $trait for two Money instances with dynamically-typed currencies.
        /// The Output is a Result instead of a Money since the operation
        /// can fail if the currencies are incompatible.
        impl $trait for Money<&dyn Currency> {
            type Output = Result<Self, MoneyMathError>;

            fn $method(self, rhs: Self) -> Self::Output {
                if self.currency.code() == rhs.currency.code() {
                    Ok(Self {
                        amount: self.amount.$method(rhs.amount),
                        currency: self.currency,
                    })
                } else {
                    Err(MoneyMathError::IncompatibleCurrencies(
                        self.currency.code(),
                        rhs.currency.code(),
                    ))
                }
            }
        }

        /// Support $trait for a Money instance with a dynamically-typed Currency
        /// and a Money instance with a statically-typed Currency. The Output
        /// is a Result since the operation can fail if the currencies are
        /// incompatible.
        impl<C> $trait<Money<C>> for Money<&dyn Currency>
        where
            C: Currency,
        {
            type Output = Result<Self, MoneyMathError>;

            fn $method(self, rhs: Money<C>) -> Self::Output {
                if self.currency.code() == rhs.currency.code() {
                    Ok(Self {
                        amount: self.amount.$method(rhs.amount),
                        currency: self.currency,
                    })
                } else {
                    Err(MoneyMathError::IncompatibleCurrencies(
                        self.currency.code(),
                        rhs.currency.code(),
                    ))
                }
            }
        }

        /// Supports $trait for a Money instance with a statically-typed Currency
        /// and a Money instance with a dynamically-typed Currency. The output
        /// is a Result since the operation can fail if the currencies are
        /// incompatible.
        impl<C> $trait<Money<&dyn Currency>> for Money<C>
        where
            C: Currency,
        {
            type Output = Result<Self, MoneyMathError>;

            fn $method(self, rhs: Money<&dyn Currency>) -> Self::Output {
                if self.currency.code() == rhs.currency.code() {
                    Ok(Self {
                        amount: self.amount.$method(rhs.amount),
                        currency: self.currency,
                    })
                } else {
                    Err(MoneyMathError::IncompatibleCurrencies(
                        self.currency.code(),
                        rhs.currency.code(),
                    ))
                }
            }
        }
    };
}

impl_binary_op!(Add, add);
impl_binary_op!(Sub, sub);
impl_binary_op!(Mul, mul);
impl_binary_op!(Div, div);
impl_binary_op!(Rem, rem);

macro_rules! impl_unary_op {
    ($trait:ident, $method:ident) => {
        /// Supports $trait for Money instances with statically-typed currencies.
        impl<C> $trait for Money<C>
        where
            C: Currency,
        {
            type Output = Self;

            fn $method(self) -> Self::Output {
                Self {
                    amount: self.amount.$method(),
                    currency: self.currency,
                }
            }
        }

        /// Supports $trait for Money instances with dynamically-typed currencies.
        impl $trait for Money<&dyn Currency> {
            type Output = Self;

            fn $method(self) -> Self::Output {
                Self {
                    amount: self.amount.$method(),
                    currency: self.currency,
                }
            }
        }
    };
}

impl_unary_op!(Neg, neg);

/// Allows ordering comparisons for Money instances with the same
/// statically-typed currency.
impl<C> PartialOrd for Money<C>
where
    C: Currency + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.amount.partial_cmp(&other.amount)
    }
}

/// Allows ordering comparisons for Money instances with
/// dynamically-typed currencies.
impl PartialOrd for Money<&dyn Currency> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.currency.code() == other.currency.code() {
            self.amount.partial_cmp(&other.amount)
        } else {
            None
        }
    }
}

#[cfg(feature = "serde")]
impl<C> Serialize for Money<C>
where
    C: Currency,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut struct_serializer = serializer.serialize_struct("money", 2)?;
        struct_serializer.serialize_field("amount", &self.amount.to_string())?;
        struct_serializer.serialize_field("currency", &self.currency.code())?;
        struct_serializer.end()
    }
}

#[cfg(feature = "serde")]
impl Serialize for Money<&dyn Currency> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut struct_serializer = serializer.serialize_struct("money", 2)?;
        struct_serializer.serialize_field("amount", &self.amount.to_string())?;
        struct_serializer.serialize_field("currency", &self.currency.code())?;
        struct_serializer.end()
    }
}

/// [Display::fmt] is supposed to be infallible, so this just writes the amount
/// followed by the currency code. For more sophisticated formatting, use the
/// [Money::format] method, which may return an Err result.
impl<C> Display for Money<C>
where
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.currency.code())
    }
}

impl Display for Money<&dyn Currency> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.currency.code())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;
    use crate::iso_currencies::{JPY, USD};
    use currency_map::CurrencyMap;
    use rust_decimal::Decimal;

    const CURRENCIES: LazyLock<CurrencyMap> =
        LazyLock::new(|| CurrencyMap::from_collection([&USD as &dyn Currency, &JPY]));

    #[test]
    fn new_static() {
        let m1 = Money::new(Decimal::ONE, USD);
        assert_eq!(m1.amount(), Decimal::ONE);
        assert_eq!(m1.currency, USD);

        let m2 = Money::new(Decimal::TWO, JPY);
        assert_eq!(m2.amount(), Decimal::TWO);
        assert_eq!(m2.currency, JPY);
    }

    #[test]
    fn equality_static() {
        assert_eq!(Money::new(Decimal::ONE, USD), Money::new(Decimal::ONE, USD));
        assert_eq!(Money::new(Decimal::ONE, JPY), Money::new(Decimal::ONE, JPY));

        // This won't even compile because they are different types...
        // assert_eq!(Money::new(Decimal::ONE, USD), Money::new(Decimal::ONE, JPY));

        assert_ne!(Money::new(Decimal::ONE, USD), Money::new(Decimal::TWO, USD));
        assert_ne!(Money::new(Decimal::ONE, JPY), Money::new(Decimal::TWO, JPY));
    }

    #[test]
    fn new_dynamic() {
        let m1 = Money::new(Decimal::ONE, CURRENCIES.get("USD").unwrap());
        assert_eq!(m1.amount(), Decimal::ONE);
        assert_eq!(m1.currency(), &USD);

        let m2 = Money::new(Decimal::ONE, CURRENCIES.get("JPY").unwrap());
        assert_eq!(m2.amount(), Decimal::ONE);
        assert_eq!(m2.currency(), &JPY);
    }

    #[test]
    fn from_minor_units() {
        let m1 = Money::from_minor_units(100, USD);
        assert_eq!(m1.amount(), Decimal::ONE);
        assert_eq!(m1.currency(), USD);

        let m2 = Money::from_minor_units(100, JPY);
        assert_eq!(m2.amount(), Decimal::ONE_HUNDRED);
        assert_eq!(m2.currency(), JPY);

        let currency_jpy = CURRENCIES.get("JPY").unwrap();
        let m3 = Money::from_minor_units(100, currency_jpy);
        assert_eq!(m3.amount(), Decimal::ONE_HUNDRED);
        assert_eq!(m3.currency(), currency_jpy);
    }

    #[test]
    fn is_zero() {
        assert!(Money::new(Decimal::ZERO, USD).is_zero());
        assert!(Money::new(Decimal::ZERO, CURRENCIES.get("USD").unwrap()).is_zero());
    }

    #[test]
    fn equality_dynamic() {
        assert_eq!(
            Money::new(Decimal::ONE, CURRENCIES.get("USD").unwrap()),
            Money::new(Decimal::ONE, CURRENCIES.get("USD").unwrap()),
        );
        assert_eq!(
            Money::new(Decimal::ONE, CURRENCIES.get("JPY").unwrap()),
            Money::new(Decimal::ONE, CURRENCIES.get("JPY").unwrap()),
        );
        assert_ne!(
            Money::new(Decimal::ONE, CURRENCIES.get("USD").unwrap()),
            Money::new(Decimal::ONE, CURRENCIES.get("JPY").unwrap()),
        );
        assert_ne!(
            Money::new(Decimal::ONE, CURRENCIES.get("JPY").unwrap()),
            Money::new(Decimal::ONE, CURRENCIES.get("USD").unwrap()),
        );
        assert_ne!(
            Money::new(Decimal::ONE, CURRENCIES.get("USD").unwrap()),
            Money::new(Decimal::TWO, CURRENCIES.get("USD").unwrap()),
        );
        assert_ne!(
            Money::new(Decimal::ONE, CURRENCIES.get("JPY").unwrap()),
            Money::new(Decimal::TWO, CURRENCIES.get("JPY").unwrap()),
        );
    }

    #[test]
    fn equality_mixed() {
        assert_eq!(
            Money::new(Decimal::ONE, CURRENCIES.get("USD").unwrap()),
            Money::new(Decimal::ONE, USD),
        );
        assert_eq!(
            Money::new(Decimal::ONE, USD),
            Money::new(Decimal::ONE, CURRENCIES.get("USD").unwrap()),
        );
        assert_eq!(
            Money::new(Decimal::ONE, CURRENCIES.get("JPY").unwrap()),
            Money::new(Decimal::ONE, JPY),
        );
        assert_eq!(
            Money::new(Decimal::ONE, JPY),
            Money::new(Decimal::ONE, CURRENCIES.get("JPY").unwrap()),
        );
        assert_ne!(
            Money::new(Decimal::ONE, CURRENCIES.get("USD").unwrap()),
            Money::new(Decimal::ONE, JPY),
        );
        assert_ne!(
            Money::new(Decimal::ONE, CURRENCIES.get("JPY").unwrap()),
            Money::new(Decimal::ONE, USD),
        );
        assert_ne!(
            Money::new(Decimal::ONE, CURRENCIES.get("USD").unwrap()),
            Money::new(Decimal::TWO, USD),
        );
    }

    #[test]
    fn add_static() {
        assert_eq!(
            Money::new(Decimal::ONE, USD) + Money::new(Decimal::ONE, USD),
            Money::new(Decimal::TWO, USD),
        );
        assert_eq!(
            Money::new(Decimal::ONE, JPY) + Money::new(Decimal::ONE, JPY),
            Money::new(Decimal::TWO, JPY),
        );
        assert_eq!(
            Money::new(Decimal::ONE, USD)
                + Money::new(Decimal::ONE, USD)
                + Money::new(Decimal::ONE, USD),
            Money::new(Decimal::new(3, 0), USD),
        );

        // this won't compile...
        // let x = Money::new(Decimal::ONE, USD) + Money::new(Decimal::ONE, JPY);
    }

    #[test]
    fn add_dynamic() {
        let currency_usd = CURRENCIES.get("USD").unwrap();
        let currency_jpy = CURRENCIES.get("JPY").unwrap();

        // Attempting to add compatible currencies should produce the correct Ok result.
        assert_eq!(
            Money::new(Decimal::ONE, currency_usd) + Money::new(Decimal::ONE, currency_usd),
            Ok(Money::new(Decimal::TWO, currency_usd)),
        );
        assert_eq!(
            Money::new(Decimal::ONE, currency_jpy) + Money::new(Decimal::ONE, currency_jpy),
            Ok(Money::new(Decimal::TWO, currency_jpy)),
        );

        // Attempting to add incompatible currencies should produce an error.
        assert_eq!(
            Money::new(Decimal::ONE, currency_usd) + Money::new(Decimal::ONE, currency_jpy),
            Err(MoneyMathError::IncompatibleCurrencies(
                currency_usd.code(),
                currency_jpy.code(),
            )),
        );
        assert_eq!(
            Money::new(Decimal::ONE, currency_jpy) + Money::new(Decimal::ONE, currency_usd),
            Err(MoneyMathError::IncompatibleCurrencies(
                currency_jpy.code(),
                currency_usd.code(),
            )),
        );

        // To add more than two instances together, use `Result.and_then()`, which
        // will skip the closure when the initial Result in an error.
        assert_eq!(
            (Money::new(Decimal::ONE, currency_usd) + Money::new(Decimal::ONE, currency_usd))
                .and_then(|m| m + Money::new(Decimal::ONE, currency_usd)),
            Ok(Money::new(Decimal::new(3, 0), currency_usd)),
        );
        assert_eq!(
            (Money::new(Decimal::ONE, currency_usd) + Money::new(Decimal::ONE, currency_jpy))
                .and_then(|m| m + Money::new(Decimal::ONE, currency_usd)),
            Err(MoneyMathError::IncompatibleCurrencies(
                currency_usd.code(),
                currency_jpy.code()
            )),
        );
    }

    #[test]
    fn add_mixed() {
        let currency_usd = CURRENCIES.get("USD").unwrap();
        let currency_jpy = CURRENCIES.get("JPY").unwrap();

        // Attempting to add compatible currencies should produce the correct Ok result.
        // The Ok type should be the same as the left-hand side.
        assert_eq!(
            Money::new(Decimal::ONE, currency_usd) + Money::new(Decimal::ONE, USD),
            Ok(Money::new(Decimal::TWO, currency_usd)),
        );
        assert_eq!(
            Money::new(Decimal::ONE, currency_jpy) + Money::new(Decimal::ONE, JPY),
            Ok(Money::new(Decimal::TWO, currency_jpy)),
        );
        assert_eq!(
            Money::new(Decimal::ONE, JPY) + Money::new(Decimal::ONE, currency_jpy),
            Ok(Money::new(Decimal::TWO, JPY)),
        );
        assert_eq!(
            Money::new(Decimal::ONE, USD) + Money::new(Decimal::ONE, currency_usd),
            Ok(Money::new(Decimal::TWO, USD)),
        );

        // Attempting to add incompatible currencies should produce an error result.
        assert_eq!(
            Money::new(Decimal::ONE, currency_usd) + Money::new(Decimal::ONE, JPY),
            Err(MoneyMathError::IncompatibleCurrencies(
                currency_usd.code(),
                JPY.code()
            )),
        );
        assert_eq!(
            Money::new(Decimal::ONE, USD) + Money::new(Decimal::ONE, currency_jpy),
            Err(MoneyMathError::IncompatibleCurrencies(
                USD.code(),
                currency_jpy.code()
            )),
        );
    }

    #[test]
    fn subtract() {
        // static
        assert_eq!(
            Money::new(Decimal::TWO, USD) - Money::new(Decimal::ONE, USD),
            Money::new(Decimal::ONE, USD)
        );
        assert_eq!(
            Money::new(Decimal::ONE, USD) - Money::new(Decimal::TWO, USD),
            Money::new(Decimal::NEGATIVE_ONE, USD)
        );

        //dynamic, same currency
        let currency_usd = CURRENCIES.get("USD").unwrap();
        let currency_jpy = CURRENCIES.get("JPY").unwrap();

        assert_eq!(
            Money::new(Decimal::TWO, currency_usd) - Money::new(Decimal::ONE, currency_usd),
            Ok(Money::new(Decimal::ONE, currency_usd))
        );
        assert_eq!(
            Money::new(Decimal::ONE, currency_usd) - Money::new(Decimal::TWO, currency_usd),
            Ok(Money::new(Decimal::NEGATIVE_ONE, currency_usd))
        );

        // dynamic, different currencies
        assert_eq!(
            Money::new(Decimal::TWO, currency_jpy) - Money::new(Decimal::ONE, currency_usd),
            Err(MoneyMathError::IncompatibleCurrencies("JPY", "USD"))
        );

        // mixed, same currency
        assert_eq!(
            Money::new(Decimal::TWO, currency_usd) - Money::new(Decimal::ONE, USD),
            Ok(Money::new(Decimal::ONE, currency_usd))
        );
        assert_eq!(
            Money::new(Decimal::ONE, USD) - Money::new(Decimal::TWO, currency_usd),
            Ok(Money::new(Decimal::NEGATIVE_ONE, USD))
        );

        // mixed, different currencies
        assert_eq!(
            Money::new(Decimal::TWO, JPY) - Money::new(Decimal::ONE, currency_usd),
            Err(MoneyMathError::IncompatibleCurrencies("JPY", "USD"))
        );
        assert_eq!(
            Money::new(Decimal::TWO, currency_jpy) - Money::new(Decimal::ONE, USD),
            Err(MoneyMathError::IncompatibleCurrencies("JPY", "USD"))
        );
    }

    #[test]
    fn multiply() {
        // static
        assert_eq!(
            Money::new(Decimal::TEN, USD) * Money::new(Decimal::TEN, USD),
            Money::new(Decimal::ONE_HUNDRED, USD)
        );

        //dynamic, same currency
        let currency_usd = CURRENCIES.get("USD").unwrap();
        let currency_jpy = CURRENCIES.get("JPY").unwrap();

        assert_eq!(
            Money::new(Decimal::TEN, currency_usd) * Money::new(Decimal::TEN, currency_usd),
            Ok(Money::new(Decimal::ONE_HUNDRED, currency_usd))
        );

        // dynamic, different currencies
        assert_eq!(
            Money::new(Decimal::TEN, currency_jpy) * Money::new(Decimal::TEN, currency_usd),
            Err(MoneyMathError::IncompatibleCurrencies("JPY", "USD"))
        );

        // mixed, same currency
        assert_eq!(
            Money::new(Decimal::TEN, currency_usd) * Money::new(Decimal::TEN, USD),
            Ok(Money::new(Decimal::ONE_HUNDRED, currency_usd))
        );
        assert_eq!(
            Money::new(Decimal::TEN, USD) * Money::new(Decimal::TEN, currency_usd),
            Ok(Money::new(Decimal::ONE_HUNDRED, USD))
        );

        // mixed, different currencies
        assert_eq!(
            Money::new(Decimal::TEN, JPY) * Money::new(Decimal::TEN, currency_usd),
            Err(MoneyMathError::IncompatibleCurrencies("JPY", "USD"))
        );
        assert_eq!(
            Money::new(Decimal::TEN, currency_jpy) * Money::new(Decimal::TEN, USD),
            Err(MoneyMathError::IncompatibleCurrencies("JPY", "USD"))
        );
    }

    #[test]
    fn divide() {
        // static
        assert_eq!(
            Money::new(Decimal::TEN, USD) / Money::new(Decimal::TWO, USD),
            Money::new(Decimal::new(5, 0), USD)
        );
        assert_eq!(
            Money::new(Decimal::TWO, USD) / Money::new(Decimal::TEN, USD),
            Money::new(Decimal::new(2, 1), USD)
        );

        //dynamic, same currency
        let currency_usd = CURRENCIES.get("USD").unwrap();
        let currency_jpy = CURRENCIES.get("JPY").unwrap();

        assert_eq!(
            Money::new(Decimal::TEN, currency_usd) / Money::new(Decimal::TWO, currency_usd),
            Ok(Money::new(Decimal::new(5, 0), currency_usd))
        );
        assert_eq!(
            Money::new(Decimal::TWO, currency_usd) / Money::new(Decimal::TEN, currency_usd),
            Ok(Money::new(Decimal::new(2, 1), currency_usd))
        );

        // dynamic, different currencies
        assert_eq!(
            Money::new(Decimal::TEN, currency_jpy) / Money::new(Decimal::TWO, currency_usd),
            Err(MoneyMathError::IncompatibleCurrencies("JPY", "USD"))
        );

        // mixed, same currency
        assert_eq!(
            Money::new(Decimal::TEN, currency_usd) / Money::new(Decimal::TWO, USD),
            Ok(Money::new(Decimal::new(5, 0), currency_usd))
        );
        assert_eq!(
            Money::new(Decimal::TWO, USD) / Money::new(Decimal::TEN, currency_usd),
            Ok(Money::new(Decimal::new(2, 1), USD))
        );

        // mixed, different currencies
        assert_eq!(
            Money::new(Decimal::TEN, JPY) / Money::new(Decimal::TWO, currency_usd),
            Err(MoneyMathError::IncompatibleCurrencies("JPY", "USD"))
        );
        assert_eq!(
            Money::new(Decimal::TEN, currency_jpy) / Money::new(Decimal::TWO, USD),
            Err(MoneyMathError::IncompatibleCurrencies("JPY", "USD"))
        );
    }

    #[test]
    fn rem() {
        // static
        assert_eq!(
            Money::new(Decimal::TEN, USD) % Money::new(Decimal::TEN, USD),
            Money::new(Decimal::ZERO, USD)
        );

        //dynamic, same currency
        let currency_usd = CURRENCIES.get("USD").unwrap();
        let currency_jpy = CURRENCIES.get("JPY").unwrap();

        assert_eq!(
            Money::new(Decimal::TEN, currency_usd) % Money::new(Decimal::TEN, currency_usd),
            Ok(Money::new(Decimal::ZERO, currency_usd))
        );

        // dynamic, different currencies
        assert_eq!(
            Money::new(Decimal::TEN, currency_jpy) % Money::new(Decimal::TEN, currency_usd),
            Err(MoneyMathError::IncompatibleCurrencies("JPY", "USD"))
        );

        // mixed, same currency
        assert_eq!(
            Money::new(Decimal::TEN, currency_usd) % Money::new(Decimal::TEN, USD),
            Ok(Money::new(Decimal::ZERO, currency_usd))
        );

        // mixed, different currencies
        assert_eq!(
            Money::new(Decimal::TEN, JPY) % Money::new(Decimal::TEN, currency_usd),
            Err(MoneyMathError::IncompatibleCurrencies("JPY", "USD"))
        );
        assert_eq!(
            Money::new(Decimal::TEN, currency_jpy) % Money::new(Decimal::TEN, USD),
            Err(MoneyMathError::IncompatibleCurrencies("JPY", "USD"))
        );
    }

    #[test]
    fn negate() {
        // static
        assert_eq!(
            -Money::new(Decimal::ONE, USD),
            Money::new(Decimal::NEGATIVE_ONE, USD)
        );

        // dynamic
        let currency_usd = CURRENCIES.get("USD").unwrap();
        assert_eq!(
            -Money::new(Decimal::ONE, currency_usd),
            Money::new(Decimal::NEGATIVE_ONE, currency_usd)
        );
    }

    #[test]
    fn pow() {
        assert_eq!(
            Money::new(Decimal::TEN, USD).pow(2),
            Money::new(Decimal::ONE_HUNDRED, USD)
        );
        let currency_usd = CURRENCIES.get("USD").unwrap();
        assert_eq!(
            Money::new(Decimal::TEN, currency_usd).pow(2),
            Money::new(Decimal::ONE_HUNDRED, currency_usd)
        );
    }

    #[test]
    fn is_positive_negative() {
        assert!(Money::new(Decimal::ONE, USD).is_positive());
        assert!((-Money::new(Decimal::ONE, USD)).is_negative());

        let currency_usd = CURRENCIES.get("USD").unwrap();
        assert!(Money::new(Decimal::ONE, currency_usd).is_positive());
        assert!((-Money::new(Decimal::ONE, currency_usd)).is_negative());

        // Decimal zero is considered positive, but not negative.
        assert!(Money::new(Decimal::ZERO, USD).is_positive());
        assert!(!Money::new(Decimal::ZERO, USD).is_negative());
    }

    #[test]
    fn round() {
        assert_eq!(
            Money::new(Decimal::new(15, 1), USD).round(0, RoundingStrategy::MidpointNearestEven),
            Money::new(Decimal::TWO, USD)
        );
        assert_eq!(
            Money::new(Decimal::new(15, 1), USD).round(0, RoundingStrategy::MidpointTowardZero),
            Money::new(Decimal::ONE, USD)
        );
    }

    #[test]
    fn partial_ord() {
        assert!(Money::new(Decimal::ONE, USD) < Money::new(Decimal::TWO, USD));
        assert!(Money::new(Decimal::TWO, USD) > Money::new(Decimal::ONE, USD));

        let currency_usd = CURRENCIES.get("USD").unwrap();
        let currency_jpy = CURRENCIES.get("JPY").unwrap();
        assert!(Money::new(Decimal::ONE, currency_usd) < Money::new(Decimal::TWO, currency_usd));
        assert!(Money::new(Decimal::TWO, currency_usd) > Money::new(Decimal::ONE, currency_usd));

        // different currencies -> incomparable
        assert_eq!(
            Money::new(Decimal::ONE, currency_usd)
                .partial_cmp(&Money::new(Decimal::TWO, currency_jpy)),
            None
        );

        // different currencies -> neither greater than nor less than
        assert!(!(Money::new(Decimal::ONE, currency_usd) < Money::new(Decimal::TWO, currency_jpy)));
        assert!(!(Money::new(Decimal::TWO, currency_usd) > Money::new(Decimal::ONE, currency_jpy)));
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Money::new(Decimal::ONE_THOUSAND, USD).to_string(),
            "1000 USD"
        );
        assert_eq!(
            Money::new(Decimal::ONE_THOUSAND, &USD as &dyn Currency).to_string(),
            "1000 USD"
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serialize() {
        let expected = "{\"amount\":\"1\",\"currency\":\"USD\"}".to_string();
        let json = serde_json::to_string(&Money::new(Decimal::ONE, USD)).unwrap();
        assert_eq!(json, expected);

        let json = serde_json::to_string(&Money::new(Decimal::ONE, &USD as &dyn Currency)).unwrap();
        assert_eq!(json, expected);
    }
}
