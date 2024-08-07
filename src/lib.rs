use std::ops::{Add, Div, Mul, Neg, Sub};

use rust_decimal::Decimal;
use thiserror::Error;

/// Various Currency definitions used in tests below
#[cfg(test)]
pub mod currencies;

/// The CurrencyMap, which provides `currency code -> &dyn Currency` lookup.
pub mod currency_map;

/// Common trait for all currencies.
pub trait Currency {
    /// Returns the unique alphabetic code for this currency
    /// (e.g., "USD" or "JPY").
    fn code(&self) -> &'static str;
    /// Returns the number of minor units supported by the currency.
    /// Currencies like USD and EUR currently support 2, but others
    /// like JPY or KRW support zero.
    fn minor_units(&self) -> u32;
    /// Returns the symbol used to represent this currency.
    /// For example `$` for USD or `¥` for JPY. Some currencies
    /// use a series of letters instead of a special symbol
    /// (e.g., `CHF` or `Lek`).
    fn symbol(&self) -> &'static str;
}

/// Debug output for a dynamically-typed Currency.
/// Only prints the code since that is unique.
impl<'c> std::fmt::Debug for &'c dyn Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Currency")
            .field("code", &self.code())
            .finish()
    }
}

/// Allows comparing dynamically-typed Currency instances.
/// They are equal of their `code()` methods return the same value.
impl<'c> PartialEq for &'c dyn Currency {
    fn eq(&self, other: &Self) -> bool {
        self.code() == other.code()
    }
}

/// Used as a trait bound when constructing new instances of Money
/// from minor units.
pub trait MinorUnits {
    fn minor_units(&self) -> u32;
}

/// Blanket implementation for any static [Currency] instance.
impl<C> MinorUnits for C
where
    C: Currency,
{
    fn minor_units(&self) -> u32 {
        self.minor_units()
    }
}

/// Implementation for an `&dyn Currency`.
impl<'c> MinorUnits for &'c dyn Currency {
    fn minor_units(&self) -> u32 {
        (*self).minor_units()
    }
}

/// An amount of money in a particular currency.
#[derive(Debug, Clone)]
pub struct Money<C> {
    amount: Decimal,
    currency: C,
}

/// Common functions for statically and dynamically-typed currencies.
impl<C> Money<C> {
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
impl<'c> Money<&'c dyn Currency> {
    /// Returns the reference to the dynamically-typed Currency.
    pub fn currency(&self) -> &'c dyn Currency {
        self.currency
    }
}

/// Allows equality comparisons between Money instances with statically-typed
/// currencies.
impl<C> PartialEq for Money<C>
where
    C: Currency + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency == other.currency
    }
}

/// Allows equality comparisons between Money instances with dynamically-typed
/// currencies and those with statically-typed currencies
impl<'c, C> PartialEq<Money<&'c dyn Currency>> for Money<C>
where
    C: Currency + PartialEq,
{
    fn eq(&self, other: &Money<&'c dyn Currency>) -> bool {
        self.amount == other.amount && self.currency.code() == other.currency.code()
    }
}

/// Allows equality comparisons between Money instances with dynamically-typed
/// currencies.
impl<'c> PartialEq for Money<&'c dyn Currency> {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency == other.currency
    }
}

/// Allows equality comparisons between Money instances with dynamically-typed
/// currencies and those with statically-typed currencies
impl<'c, C> PartialEq<Money<C>> for Money<&'c dyn Currency>
where
    C: Currency,
{
    fn eq(&self, other: &Money<C>) -> bool {
        self.amount == other.amount && self.currency.code() == other.currency.code()
    }
}

/// Adds two Money instances with the same statically-typed currencies.
/// Attempting to add two instances with _different_ statically-typed
/// Currencies simply won't compile.
impl<C> Add for Money<C>
where
    C: Currency,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            amount: self.amount + rhs.amount,
            currency: self.currency,
        }
    }
}

/// Errors that can occur when doing math with Money instances that
/// have dynamically-typed currencies
#[derive(Debug, Error, PartialEq, Clone)]
pub enum MoneyMathError {
    #[error("the money instances have incompatible currencies ({0}, {1})")]
    IncompatibleCurrencies(&'static str, &'static str),
}

/// Adds two Money instances with dynamically-typed currencies.
/// The Output is a Result instead of a Money since the operation
/// can fail if the currencies are incompatible.
impl<'c> Add for Money<&'c dyn Currency> {
    type Output = Result<Self, MoneyMathError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.currency.code() == rhs.currency.code() {
            Ok(Self {
                amount: self.amount + rhs.amount,
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

/// Adds a Money instance with a dynamically-typed Currency to
/// a Money instance with a statically-typed Currency. The Output
/// is a Result since the operation can fail if the currencies are
/// incompatible.
impl<'c, C> Add<Money<C>> for Money<&'c dyn Currency>
where
    C: Currency,
{
    type Output = Result<Self, MoneyMathError>;

    fn add(self, rhs: Money<C>) -> Self::Output {
        if self.currency.code() == rhs.currency.code() {
            Ok(Self {
                amount: self.amount + rhs.amount,
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

/// Adds a Money instance with a statically-typed Currency to
/// a Money instance with a dynamically-typed Currency. The output
/// is a Result since the operation can fail if the currencies are
/// incompatible.
impl<'c, C> Add<Money<&'c dyn Currency>> for Money<C>
where
    C: Currency,
{
    type Output = Result<Self, MoneyMathError>;

    fn add(self, rhs: Money<&'c dyn Currency>) -> Self::Output {
        if self.currency.code() == rhs.currency.code() {
            Ok(Self {
                amount: self.amount + rhs.amount,
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

/// Subtracts Money instances with statically-typed currencies.
impl<C> Sub for Money<C>
where
    C: Currency,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            amount: self.amount - rhs.amount,
            currency: self.currency,
        }
    }
}

/// Subtracts Money instances with dynamically-typed currencies.
impl<'c> Sub for Money<&'c dyn Currency> {
    type Output = Result<Self, MoneyMathError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.currency.code() == rhs.currency.code() {
            Ok(Self {
                amount: self.amount - rhs.amount,
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

/// Subtracts instances of Money with statically-typed currencies
/// from instances with dynamically-typed currencies.
impl<'c, C> Sub<Money<C>> for Money<&'c dyn Currency>
where
    C: Currency,
{
    type Output = Result<Self, MoneyMathError>;

    fn sub(self, rhs: Money<C>) -> Self::Output {
        if self.currency.code() == rhs.currency.code() {
            Ok(Self {
                amount: self.amount - rhs.amount,
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

/// Subtracts instances of Money with dynamically-typed currencies
/// from instances with statically-typed currencies.
impl<'c, C> Sub<Money<&'c dyn Currency>> for Money<C>
where
    C: Currency,
{
    type Output = Result<Self, MoneyMathError>;

    fn sub(self, rhs: Money<&'c dyn Currency>) -> Self::Output {
        if self.currency.code() == rhs.currency.code() {
            Ok(Self {
                amount: self.amount - rhs.amount,
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

/// Multiplies instances of Money with statically-typed currencies.
impl<C> Mul for Money<C>
where
    C: Currency,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            amount: self.amount * rhs.amount,
            currency: self.currency,
        }
    }
}

/// Multiplies instances of Money with dynamically-typed currencies.
impl<'c> Mul for Money<&'c dyn Currency> {
    type Output = Result<Self, MoneyMathError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.currency.code() == rhs.currency.code() {
            Ok(Self {
                amount: self.amount * rhs.amount,
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

/// Multiplies instances of Money with dynamically and
/// statically-typed currencies.
impl<'c, C> Mul<Money<C>> for Money<&'c dyn Currency>
where
    C: Currency,
{
    type Output = Result<Self, MoneyMathError>;

    fn mul(self, rhs: Money<C>) -> Self::Output {
        if self.currency.code() == rhs.currency.code() {
            Ok(Self {
                amount: self.amount * rhs.amount,
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

/// Multiplies instances of Money with dynamically and
/// statically-typed currencies.
impl<'c, C> Mul<Money<&'c dyn Currency>> for Money<C>
where
    C: Currency,
{
    type Output = Result<Self, MoneyMathError>;

    fn mul(self, rhs: Money<&'c dyn Currency>) -> Self::Output {
        if self.currency.code() == rhs.currency.code() {
            Ok(Self {
                amount: self.amount * rhs.amount,
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

/// Divides instances of Money with statically-typed currencies.
impl<C> Div for Money<C>
where
    C: Currency,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            amount: self.amount / rhs.amount,
            currency: self.currency,
        }
    }
}

/// Divides instances of Money with dynamically-typed currencies.
impl<'c> Div for Money<&'c dyn Currency> {
    type Output = Result<Self, MoneyMathError>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.currency.code() == rhs.currency.code() {
            Ok(Self {
                amount: self.amount / rhs.amount,
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

/// Divides instances of Money with dynamically and statically-typed
/// currencies.
impl<'c, C> Div<Money<C>> for Money<&'c dyn Currency>
where
    C: Currency,
{
    type Output = Result<Self, MoneyMathError>;

    fn div(self, rhs: Money<C>) -> Self::Output {
        if self.currency.code() == rhs.currency.code() {
            Ok(Self {
                amount: self.amount / rhs.amount,
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

/// Divides instances of Money with dynamically and statically-typed
/// currencies.
impl<'c, C> Div<Money<&'c dyn Currency>> for Money<C>
where
    C: Currency,
{
    type Output = Result<Self, MoneyMathError>;

    fn div(self, rhs: Money<&'c dyn Currency>) -> Self::Output {
        if self.currency.code() == rhs.currency.code() {
            Ok(Self {
                amount: self.amount / rhs.amount,
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

/// Negates Money instances with statically-typed currencies.
impl<C> Neg for Money<C>
where
    C: Currency,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            amount: -self.amount,
            currency: self.currency,
        }
    }
}

/// Negates Money instances with dynamically-typed currencies.
impl<'c> Neg for Money<&'c dyn Currency> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            amount: -self.amount,
            currency: self.currency,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;
    use crate::currencies::*;
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
}
