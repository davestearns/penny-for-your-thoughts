use std::ops::Add;

use rust_decimal::Decimal;
use thiserror::Error;

/// Definitions for various currencies.
pub mod currencies;

/// CurrencyMap
pub mod currency_map;

/// Common trait for all currencies.
pub trait Currency: Send + Sync {
    fn code(&self) -> &'static str;
    fn minor_units(&self) -> u32;
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
        Money { amount, currency }
    }

    /// Returns a copy of the amount as a Decimal.
    pub fn amount(&self) -> Decimal {
        self.amount
    }
}

/// Functions specifically for owned statically-typed Currency instances.
impl<C> Money<C>
where
    C: Currency + Copy,
{
    pub fn currency(&self) -> C {
        self.currency
    }
}

/// Functions specifically for borrowed dynamically-typed currencies.
impl<'c> Money<&'c dyn Currency> {
    pub fn from_minor_units(
        minor_units: i64,
        currency: &'c dyn Currency,
    ) -> Money<&'c dyn Currency> {
        Money {
            amount: Decimal::new(minor_units, currency.minor_units()),
            currency,
        }
    }

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
/// The Output is a Result instead a Money since the operation can
/// fail if the currencies are incompatible.
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
/// a Money instance with a statically-typed Currency. The output
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

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;
    use crate::currencies::*;
    use currency_map::CurrencyMap;
    use rust_decimal::Decimal;

    const CURRENCIES: LazyLock<CurrencyMap> =
        LazyLock::new(|| CurrencyMap::from_slice([&USD as &dyn Currency, &JPY]));

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
            Money::new(Decimal::ONE, CURRENCIES.get("JPY").unwrap()),
            Money::new(Decimal::ONE, JPY),
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
}
