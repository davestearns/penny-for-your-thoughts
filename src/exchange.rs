use rust_decimal::Decimal;
use thiserror::Error;

use crate::{Currency, Money};

/// Represents an exchange rate for converting monetary amounts
/// in the 'from' [Currency] into amounts in the 'to' [Currency].
pub struct ExchangeRate<F, T>
where
    F: Currency,
    T: Currency,
{
    pub from: F,
    pub to: T,
    pub rate: Decimal,
}

impl<F, T> ExchangeRate<F, T>
where
    F: Currency,
    T: Currency,
{
    /// Constructs a new instance given the rate for converting
    /// monetary amounts in the 'from' [Currency] into the amounts
    /// in the 'to' [Currency].
    pub fn new(from: F, to: T, rate: Decimal) -> Self {
        ExchangeRate { from, to, rate }
    }
}

/// Errors that can occur when converting Money instances with
/// dynamic currencies into another currency.
#[derive(Debug, Error, PartialEq, Clone)]
pub enum MoneyConversionError {
    #[error(
        "the exchange rate's 'from' currency does not match the Money instance's currency ({0}, {1})"
    )]
    IncorrectExchangeRate(&'static str, &'static str),
}

impl<C> Money<C>
where
    C: Currency,
{
    /// Returns a new [Money] in the [ExchangeRate]'s 'to' currency after multiplying
    /// the amount by the exchange rate.
    pub fn convert<T: Currency + Copy>(&self, exchange_rate: &ExchangeRate<C, T>) -> Money<T> {
        Money {
            amount: self.amount * exchange_rate.rate,
            currency: exchange_rate.to,
        }
    }
}

impl Money<&dyn Currency> {
    /// If the current dynamic [Currency] is the same as the [ExchangeRate]'s
    /// 'from' currency, this returns an Ok value with a new [Money] in the
    /// [ExchangeRate]'s 'to' currency after multiplying the amount by the
    /// exchange rate. If the currencies don't match, this returns an Err
    /// value of type [MoneyConversionError::IncorrectExchangeRate].
    pub fn convert<C: Currency, T: Currency + Copy>(
        &self,
        exchange_rate: &ExchangeRate<C, T>,
    ) -> Result<Money<T>, MoneyConversionError> {
        if self.currency.code() == exchange_rate.from.code() {
            Ok(Money {
                amount: self.amount * exchange_rate.rate,
                currency: exchange_rate.to,
            })
        } else {
            Err(MoneyConversionError::IncorrectExchangeRate(
                exchange_rate.from.code(),
                self.currency().code(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iso_currencies::{EUR, JPY, USD};

    #[test]
    fn convert_static() {
        let exchange_rate = ExchangeRate::new(USD, EUR, Decimal::new(85, 2));
        assert_eq!(
            Money::new(1, USD).convert(&exchange_rate),
            Money::new(Decimal::new(85, 2), EUR)
        );
    }

    #[test]
    fn convert_dynamic() {
        let exchange_rate = ExchangeRate::new(USD, EUR, Decimal::new(85, 2));
        assert_eq!(
            Money::new(1, &USD as &dyn Currency).convert(&exchange_rate),
            Ok(Money::new(Decimal::new(85, 2), EUR))
        );

        assert_eq!(
            Money::new(1, &JPY as &dyn Currency).convert(&exchange_rate),
            Err(MoneyConversionError::IncorrectExchangeRate("USD", "JPY"))
        );
    }
}
