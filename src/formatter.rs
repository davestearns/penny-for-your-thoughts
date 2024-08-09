use rust_decimal::{Decimal, RoundingStrategy};

/// Formats [Money] instances into Strings.
#[derive(Debug, Clone, PartialEq)]
pub struct Formatter {
    /// An explicit number of decimal places to round to and display.
    /// If this is None, the [Currency::minor_units] will be used.
    /// This defaults to None.
    pub decimal_places: Option<u32>,
    /// The RoundingStrategy to use when rounding is necessary. This
    /// defaults to RoundingStrategy::MidpointNearestEven
    /// (aka "Banker's Rounding").
    pub rounding_strategy: RoundingStrategy,
    /// The characters to use between the whole and fractional parts
    /// of the amount. This defaults to ".".
    pub decimal_separator: &'static str,
    /// The digit grouping size. This defaults to 3.
    /// Digits in the whole portion will be grouped by this size,
    /// no matter how many there are.
    pub digit_grouping: usize,
    /// Custom digit groupings, expressed in right-to-left order.
    /// For example, `&[3,2,2]` will group the right-most 3 digits,
    /// then the 2 digits to the left of those, then the 2 digits
    /// to the left of those, and then display any remaining digits
    /// without any grouping. This defaults to None.
    /// If specified, these override `digit_grouping`.
    pub digit_groupings: Option<&'static [usize]>,
    /// The characters to use between digit groups. This
    /// defaults to ",".
    pub digit_group_separator: &'static str,
    /// The format template to use for positive amounts.
    pub positive_template: &'static str,
    /// The format template to use for negative amounts.
    pub negative_template: &'static str,
    /// An optional format template to use for zero amounts. This defaults
    /// to None. If None, the positive_format will be used instead.
    pub zero_template: Option<&'static str>,
}

impl Default for Formatter {
    fn default() -> Self {
        Self {
            decimal_places: None,
            rounding_strategy: RoundingStrategy::MidpointNearestEven,
            decimal_separator: ".",
            digit_grouping: 3,
            digit_groupings: None,
            digit_group_separator: ",",
            positive_template: "{s}{a}",
            negative_template: "-{s}{a}",
            zero_template: None,
        }
    }
}

impl Formatter {
    /// Formats a Decimal amount. If `self.decimal_places` is `None` the
    /// `default_decimal_places` will be used, which is typically the number
    /// of minor units in the currency.
    pub fn format_amount(&self, amount: Decimal, default_decimal_places: u32) -> String {
        // round to the desired number of decimal places
        let dp = self.decimal_places.unwrap_or(default_decimal_places);
        let rounded_amount = amount.round_dp_with_strategy(dp, self.rounding_strategy);
        let amount_string = rounded_amount.to_string();

        // Trim off any leading negative sign and spaces, and split on `.`
        // since that seems to be the decimal separator always regardless of
        // system locale. This also seems to be a explicit design choice in
        // the default Rust numeric formatting as well. See
        // https://doc.rust-lang.org/std/fmt/index.html#localization
        let mut split = amount_string
            .trim_start_matches('-')
            .trim_start()
            .split('.');

        // We should always get at least some whole digits (even zero
        // is converted to the string "0"), but there might not be any
        // fractional digits.
        let whole = split.next().expect("expected at least some whole digits");
        let maybe_frac = split.next();

        // Break up the whole digits according to the digit_groupings.
        // If `digit_groupings` is None, create a temporary vector of
        // enough `digit_grouping` values given the length of `whole`
        let digit_groupings = if let Some(dgs) = self.digit_groupings {
            dgs
        } else {
            &vec![self.digit_grouping; (whole.len() / self.digit_grouping) + 1]
        };

        let mut groups: Vec<&str> = Vec::new();
        let mut group_start = whole.len();

        for &group_len in digit_groupings {
            if group_len >= group_start {
                break;
            }
            groups.push(&whole[(group_start - group_len)..group_start]);
            group_start -= group_len;
        }
        if group_start > 0 {
            groups.push(&whole[0..group_start]);
        }

        // Since we built the groups in reverse,
        // reverse them in-place before we join them.
        groups.reverse();
        let formatted_whole = groups.join(self.digit_group_separator);

        // Right-pad the fractional digits with zeros if necessary
        let frac = format!("{:0<1$}", maybe_frac.unwrap_or_default(), dp as usize);

        // Only include the decimal separator if dp > 0
        let decimal_sep = if dp > 0 { self.decimal_separator } else { "" };

        format!("{}{}{}", &formatted_whole, decimal_sep, frac)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_amount_simple() {
        assert_eq!(
            Formatter::default().format_amount(Decimal::new(123456789123456789, 2), 2),
            "1,234,567,891,234,567.89".to_string()
        );
    }

    #[test]
    fn format_amount_no_frac() {
        assert_eq!(
            Formatter::default().format_amount(Decimal::new(123456789123456789, 0), 2),
            "123,456,789,123,456,789.00".to_string()
        );
    }

    #[test]
    fn format_amount_no_frac_no_dp() {
        assert_eq!(
            Formatter::default().format_amount(Decimal::new(123456789123456789, 0), 0),
            "123,456,789,123,456,789".to_string()
        );
    }

    #[test]
    fn format_amount_zero() {
        assert_eq!(
            Formatter::default().format_amount(Decimal::ZERO, 2),
            "0.00".to_string()
        );
    }

    #[test]
    fn format_amount_decimal_padding() {
        assert_eq!(
            Formatter::default().format_amount(Decimal::new(12, 1), 4),
            "1.2000".to_string()
        );
    }

    #[test]
    fn format_amount_custom_separators() {
        let formatter = Formatter {
            decimal_separator: ",",
            digit_group_separator: " ",
            ..Default::default()
        };
        assert_eq!(
            formatter.format_amount(Decimal::new(123456789123456789, 2), 2),
            "1 234 567 891 234 567,89".to_string()
        );
    }

    #[test]
    fn format_amount_rounding() {
        assert_eq!(
            Formatter::default().format_amount(Decimal::new(123456789123456789, 4), 2),
            "12,345,678,912,345.68".to_string()
        );
    }

    #[test]
    fn format_amount_rounding_strategy() {
        let formatter = Formatter {
            rounding_strategy: RoundingStrategy::ToZero,
            ..Default::default()
        };
        assert_eq!(
            formatter.format_amount(Decimal::new(123456789123456789, 4), 2),
            "12,345,678,912,345.67".to_string()
        );
    }

    #[test]
    fn format_amount_custom_digit_grouping() {
        let formatter = Formatter {
            digit_groupings: Some(&[3, 2, 2]),
            ..Default::default()
        };
        assert_eq!(
            formatter.format_amount(Decimal::new(123456789123456789, 2), 2),
            "123456789,12,34,567.89".to_string()
        );
    }

    #[test]
    fn format_amount_no_digit_grouping() {
        let formatter = Formatter {
            digit_groupings: Some(&[]),
            ..Default::default()
        };
        assert_eq!(
            formatter.format_amount(Decimal::new(123456789123456789, 2), 2),
            "1234567891234567.89".to_string()
        );
    }
}
