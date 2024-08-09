use rust_decimal::{Decimal, RoundingStrategy};

#[derive(Debug, Clone, PartialEq)]
pub struct Formatter {
    pub decimal_places: Option<u32>,
    pub rounding_strategy: RoundingStrategy,
    pub decimal_separator: &'static str,
    pub digit_groupings: &'static [usize],
    pub digit_group_separator: &'static str,
    pub positive_format: &'static str,
    pub negative_format: &'static str,
    pub zero_format: Option<&'static str>,
}

impl Default for Formatter {
    fn default() -> Self {
        Self {
            decimal_places: None,
            rounding_strategy: RoundingStrategy::MidpointNearestEven,
            decimal_separator: ".",
            digit_groupings: &[3, 3, 3, 3, 3],
            digit_group_separator: ",",
            positive_format: "{s}{a}",
            negative_format: "-{s}{a}",
            zero_format: None,
        }
    }
}

impl Formatter {
    pub fn format_amount(&self, amount: Decimal, default_decimal_places: u32) -> String {
        // round to the desired number of decimal places
        let rounded_amount = amount.round_dp_with_strategy(
            self.decimal_places.unwrap_or(default_decimal_places),
            self.rounding_strategy,
        );
        let amount_string = rounded_amount.to_string();

        // Trim off any leading negative sign and spaces, and split on `.`
        // since that seems to be the decimal separator always regardless of
        // system locale.
        let mut split = amount_string
            .trim_start_matches('-')
            .trim_start()
            .split('.');

        // We should always get at least some whole digits (even zero
        // is converted to the string "0"), but there might not be any
        // fractional digits.
        let whole = split.next().expect("expected at least some whole digits");
        let maybe_frac = split.next();

        // Break up the whole digits according to the digit_groupings
        let mut groups: Vec<&str> = Vec::new();
        let mut group_start = whole.len();
        for &group_len in self.digit_groupings {
            if group_len >= group_start {
                break;
            }
            groups.push(&whole[group_start - group_len..group_start]);
            group_start -= group_len;
        }
        if group_start > 0 {
            groups.push(&whole[0..group_start]);
        }

        // Since we built the groups in reverse,
        // reverse them in-place before we join them.
        groups.reverse();
        let formatted_whole = groups.join(self.digit_group_separator);

        // Include the decimal separator only if there are fractional digits.
        let decimal_sep = if maybe_frac.is_some() {
            self.decimal_separator
        } else {
            ""
        };

        format!(
            "{}{}{}",
            &formatted_whole,
            decimal_sep,
            maybe_frac.unwrap_or_default()
        )
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
            "123,456,789,123,456,789".to_string()
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
            digit_groupings: &[3, 2, 2],
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
            digit_groupings: &[],
            ..Default::default()
        };
        assert_eq!(
            formatter.format_amount(Decimal::new(123456789123456789, 2), 2),
            "1234567891234567.89".to_string()
        );
    }
}
