use crate::Currency;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct USD;
impl Currency for USD {
    fn code(&self) -> &'static str {
        "USD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn symbol(&self) -> &'static str {
        "$"
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JPY;
impl Currency for JPY {
    fn code(&self) -> &'static str {
        "JPY"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn symbol(&self) -> &'static str {
        "¥"
    }
}
