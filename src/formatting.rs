use icu::{
    experimental::dimension::currency::{formatter::CurrencyFormatter, CurrencyCode},
    locale::Locale,
};
use rust_decimal::RoundingStrategy;
use std::str::FromStr;
use tinystr::TinyAsciiStr;

use crate::{Currency, Money};

/// Republished reference to the icu crate's CurrencyFormatterOptions.
/// If you `use doubloon::formatting::CurrencyFormatterOptions`, your
/// code will be protected against changes to module path within the icu
/// crate once currency formatting becomes stable.
pub use icu::experimental::dimension::currency::options::CurrencyFormatterOptions;

#[derive(Debug, Clone)]
pub struct FormattingOptions {
    /// The number of decimal places to include in the formatted string.
    /// By default this will be the number of minor units for the currency.
    /// If this is less than the current scale, the amount will be rounded
    /// using the specified rounding_strategy.
    pub decimal_places: u32,
    /// The rounding strategy to use when decimal_places is less than
    /// the current scale. By default this will use MidpointNearestEven,
    /// otherwise known as "banker's rounding."
    pub rounding_strategy: RoundingStrategy,
    /// Options for the icu [CurrencyFormatter].
    pub currency_formatter_options: CurrencyFormatterOptions,
}

impl<C> Money<C> {
    /// Returns a formatted version of this instance for the specified locale.
    fn format_helper(
        &self,
        locale: &Locale,
        currency_code_str: &'static str,
        options: FormattingOptions,
    ) -> String {
        // This could only fail for app-defined Currency instances that
        // return a code with non-ASCII characters, and it fail immediately
        // and always, so I think it's fine to use .expect() here.
        let currency_code = CurrencyCode(
            TinyAsciiStr::from_str(currency_code_str).expect("unsupported currency code"),
        );
        let formatter =
            CurrencyFormatter::try_new(locale.into(), CurrencyFormatterOptions::default()).unwrap();

        let mut rounded_amount = self
            .amount
            .round_dp_with_strategy(options.decimal_places, options.rounding_strategy);
        // rescale to force a minimum number of decimal places even when zero
        rounded_amount.rescale(options.decimal_places);
        let amount_string = rounded_amount.to_string();
        let amount = icu::decimal::input::Decimal::try_from_str(&amount_string).unwrap();

        let formatted = formatter.format_fixed_decimal(&amount, currency_code);
        formatted.to_string()
    }
}

/// Functions specifically for owned statically-typed Currency instances.
impl<C> Money<C>
where
    C: Currency + Copy,
{
    /// Formats this Money instance as a locale-aware string suitable for
    /// showing to a user. This uses the `icu` crate for CLDR formatting rules.
    pub fn format(&self, locale: &Locale) -> String {
        self.format_helper(
            locale,
            self.currency.code(),
            FormattingOptions {
                decimal_places: self.currency.minor_units(),
                rounding_strategy: RoundingStrategy::MidpointNearestEven,
                currency_formatter_options: CurrencyFormatterOptions::default(),
            },
        )
    }

    /// Same as [format] but allows the caller to specify [FormattingOptions].
    pub fn format_with_options(&self, locale: &Locale, options: FormattingOptions) -> String {
        self.format_helper(locale, self.currency.code(), options)
    }
}

/// Functions specifically for borrowed dynamically-typed currencies.
impl Money<&dyn Currency> {
    /// Formats this Money instance as a locale-aware string suitable for
    /// showing to a user. This uses the `icu` crate for CLDR formatting rules.
    pub fn format(&self, locale: &Locale) -> String {
        self.format_helper(
            locale,
            self.currency.code(),
            FormattingOptions {
                decimal_places: self.currency.minor_units(),
                rounding_strategy: RoundingStrategy::MidpointNearestEven,
                currency_formatter_options: CurrencyFormatterOptions::default(),
            },
        )
    }

    /// Same as [format] but allows the caller to specify [FormattingOptions].
    pub fn format_with_options(&self, locale: &Locale, options: FormattingOptions) -> String {
        self.format_helper(locale, self.currency.code(), options)
    }
}

#[cfg(test)]
mod tests {
    use crate::formatting::*;
    use crate::iso_currencies::{EUR, JPY, PLN, USD};
    use crate::*;
    use icu::locale::locale;

    #[test]
    fn locale_aware_formatting() {
        let m = Money::new(Decimal::new(123456789, 2), EUR);
        // en-US uses comma for group separator, period for decimal separator,
        // with the symbol at the left with no spacing.
        assert_eq!(m.format(&locale!("en-US")), "€1,234,567.89");

        // ir-IR is like en-US except there is a narrow non-breaking space between the symbol
        // and the amount.
        assert_eq!(m.format(&locale!("ir-IR")), "€\u{a0}1,234,567.89");

        // tr-TR is similar to ir-IR but uses period for the group separator
        // and comma for the decimal separator.
        assert_eq!(m.format(&locale!("tr-TR")), "€1.234.567,89");

        // fr-FR puts the symbol at the end, and uses non-breaking spaces between digit groups,
        // comma as a decimal separator, and a narrow non-breaking space between the amount and symbol.
        assert_eq!(
            m.format(&locale!("fr-FR")),
            "1\u{202f}234\u{202f}567,89\u{a0}€"
        );

        // pl-PL is like fr-FR except it uses all narrow non-breaking spaces.
        assert_eq!(m.format(&locale!("pl-PL")), "1\u{a0}234\u{a0}567,89\u{a0}€");
    }

    #[test]
    fn format_pln() {
        // For my friend https://github.com/CodeServant
        let m = Money::new(Decimal::new(123456789, 2), PLN);
        // in pl-PL the symbol is zł, on the right
        assert_eq!(
            m.format(&locale!("pl-PL")),
            "1\u{a0}234\u{a0}567,89\u{a0}zł"
        );
        // in en-US the ISO code is used instead, on the left
        assert_eq!(m.format(&locale!("en-US")), "PLN\u{a0}1,234,567.89");
    }

    #[test]
    fn format_dyn_currency() {
        let c: &dyn Currency = &USD;
        let m = Money::new(Decimal::new(123456789, 2), c);
        assert_eq!(m.format(&locale!("en-US")), "$1,234,567.89");
    }

    #[test]
    fn format_zero_decimals_with_minor_units() {
        let m = Money::new(Decimal::ONE, USD);
        assert_eq!(m.format(&locale!("en-US")), "$1.00");
    }

    #[test]
    fn format_zero_decimals_with_no_minor_units() {
        let m = Money::new(Decimal::ONE, JPY);
        // JPY has no minor units, so it shouldn't have any decimals
        assert_eq!(m.format(&locale!("ja-JP")), "￥1");
    }

    #[test]
    fn format_foreign_currency_in_euro_locales() {
        let m = Money::new(Decimal::new(123456789, 2), USD);
        assert_eq!(m.format(&locale!("en-US")), "$1,234,567.89");
        assert_eq!(
            m.format(&locale!("fr-FR")),
            "1\u{202f}234\u{202f}567,89\u{a0}$US"
        );
        assert_eq!(m.format(&locale!("tr-TR")), "$1.234.567,89");
        assert_eq!(
            m.format(&locale!("pl-PL")),
            "1\u{a0}234\u{a0}567,89\u{a0}USD"
        );
    }

    #[test]
    fn format_with_options() {
        let m = Money::new(Decimal::ONE_HUNDRED, USD);
        assert_eq!(
            m.format_with_options(
                &locale!("en-US"),
                FormattingOptions {
                    decimal_places: 0, // force zero decimal places
                    rounding_strategy: RoundingStrategy::MidpointNearestEven,
                    currency_formatter_options: CurrencyFormatterOptions::default(),
                }
            ),
            "$100"
        );
    }

    #[test]
    fn format_rounding() {
        let m = Money::new(Decimal::new(123456750, 2), USD);
        assert_eq!(
            m.format_with_options(
                &locale!("en-US"),
                FormattingOptions {
                    decimal_places: 0, // force zero decimal places
                    rounding_strategy: RoundingStrategy::MidpointNearestEven,
                    currency_formatter_options: CurrencyFormatterOptions::default(),
                }
            ),
            "$1,234,568" // should round to nearest even
        );

        assert_eq!(
            m.format_with_options(
                &locale!("en-US"),
                FormattingOptions {
                    decimal_places: 0, // force zero decimal places
                    rounding_strategy: RoundingStrategy::MidpointTowardZero,
                    currency_formatter_options: CurrencyFormatterOptions::default(),
                }
            ),
            "$1,234,567" // should round toward zero
        );
    }

    #[test]
    fn format_negative() {
        let m = Money::new(Decimal::new(-123456789, 2), USD);
        assert_eq!(m.format(&locale!("en-US")), "$-1,234,567.89");
    }
}
