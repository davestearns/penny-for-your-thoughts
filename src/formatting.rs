use icu::{
    experimental::dimension::currency::{
        formatter::CurrencyFormatter, options::CurrencyFormatterOptions, CurrencyCode,
    },
    locale::Locale,
};
use rust_decimal::RoundingStrategy;
use std::str::FromStr;
use tinystr::TinyAsciiStr;

use crate::{Currency, Money};

impl<C> Money<C> {
    /// Returns a formatted version of this instance for the specified locale.
    fn format_helper(
        &self,
        locale: Locale,
        currency_code: CurrencyCode,
        decimal_places: u32,
    ) -> String {
        let formatter =
            CurrencyFormatter::try_new(locale.into(), CurrencyFormatterOptions::default()).unwrap();

        let mut rounded_amount = self
            .amount
            .round_dp_with_strategy(decimal_places, RoundingStrategy::MidpointNearestEven);
        // rescale to force a minimum number of decimal places even when zero
        rounded_amount.rescale(decimal_places);
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
    #[cfg(feature = "formatting")]
    /// Formats this Money instance as a locale-aware string suitable for
    /// showing to a user. This uses the `icu` crate for CLDR formatting rules.
    pub fn format(&self, locale: Locale) -> String {
        let currency_code = CurrencyCode(
            TinyAsciiStr::from_str(self.currency.code()).expect("unsupported currency code"),
        );
        self.format_helper(locale, currency_code, self.currency.minor_units())
    }
}

/// Functions specifically for borrowed dynamically-typed currencies.
impl Money<&dyn Currency> {
    #[cfg(feature = "formatting")]
    /// Formats this Money instance as a locale-aware string suitable for
    /// showing to a user. This uses the `icu` crate for CLDR formatting rules.
    pub fn format(&self, locale: Locale) -> String {
        let currency_code = CurrencyCode(
            TinyAsciiStr::from_str(self.currency.code()).expect("unsupported currency code"),
        );
        self.format_helper(locale, currency_code, self.currency.minor_units())
    }
}

#[cfg(test)]
mod tests {
    use crate::iso_currencies::{EUR, JPY, PLN, USD};
    use crate::*;
    use icu::locale::locale;

    #[test]
    fn locale_aware_formatting() {
        let m = Money::new(Decimal::new(123456789, 2), EUR);
        // en-US uses comma for group separator, period for decimal separator,
        // with the symbol at the left with no spacing.
        assert_eq!(m.format(locale!("en-US")), "€1,234,567.89");

        // ir-IR is like en-US except there is a narrow non-breaking space between the symbol
        // and the amount.
        assert_eq!(m.format(locale!("ir-IR")), "€\u{a0}1,234,567.89");

        // tr-TR is similar to ir-IR but uses period for the group separator
        // and comma for the decimal separator.
        assert_eq!(m.format(locale!("tr-TR")), "€1.234.567,89");

        // fr-FR puts the symbol at the end, and uses non-breaking spaces between digit groups,
        // comma as a decimal separator, and a narrow non-breaking space between the amount and symbol.
        assert_eq!(
            m.format(locale!("fr-FR")),
            "1\u{202f}234\u{202f}567,89\u{a0}€"
        );

        // pl-PL is like fr-FR except it uses all narrow non-breaking spaces.
        assert_eq!(m.format(locale!("pl-PL")), "1\u{a0}234\u{a0}567,89\u{a0}€");
    }

    #[test]
    fn format_pln() {
        // For my friend https://github.com/CodeServant
        let m = Money::new(Decimal::new(123456789, 2), PLN);
        // in pl-PL the symbol is zł, on the right
        assert_eq!(m.format(locale!("pl-PL")), "1\u{a0}234\u{a0}567,89\u{a0}zł");
        // in en-US the ISO code is used instead, on the left
        assert_eq!(m.format(locale!("en-US")), "PLN\u{a0}1,234,567.89");
    }

    #[test]
    fn format_dyn_currency() {
        let c: &dyn Currency = &USD;
        let m = Money::new(Decimal::new(123456789, 2), c);
        assert_eq!(m.format(locale!("en-US")), "$1,234,567.89");
    }

    #[test]
    fn format_zero_decimals_with_minor_units() {
        let m = Money::new(Decimal::ONE, USD);
        assert_eq!(m.format(locale!("en-US")), "$1.00");
    }

    #[test]
    fn format_zero_decimals_with_no_minor_units() {
        let m = Money::new(Decimal::ONE, JPY);
        // JPY has no minor units, so it shouldn't have any decimals
        assert_eq!(m.format(locale!("ja-JP")), "￥1");
    }

    #[test]
    fn format_foreign_currency_in_euro_locales() {
        let m = Money::new(Decimal::new(123456789, 2), USD);
        assert_eq!(m.format(locale!("en-US")), "$1,234,567.89");
        assert_eq!(
            m.format(locale!("fr-FR")),
            "1\u{202f}234\u{202f}567,89\u{a0}$US"
        );
        assert_eq!(m.format(locale!("tr-TR")), "$1.234.567,89");
        assert_eq!(
            m.format(locale!("pl-PL")),
            "1\u{a0}234\u{a0}567,89\u{a0}USD"
        );
    }
}
