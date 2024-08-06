use std::collections::HashMap;

use crate::Currency;

#[derive(Debug, Default)]
pub struct CurrencyMap<'c> {
    map: HashMap<&'c str, &'c dyn Currency>,
}

impl<'c> CurrencyMap<'c> {
    pub fn new() -> Self {
        CurrencyMap {
            map: HashMap::new(),
        }
    }

    pub fn from_collection<I>(currencies: I) -> Self
    where
        I: IntoIterator<Item = &'c dyn Currency>,
    {
        CurrencyMap {
            map: currencies.into_iter().map(|c| (c.code(), c)).collect(),
        }
    }

    pub fn insert(&mut self, currency: &'c dyn Currency) {
        self.map.insert(currency.code(), currency);
    }

    pub fn get(&self, code: &'c str) -> Option<&'c dyn Currency> {
        self.map.get(code).copied()
    }
}
