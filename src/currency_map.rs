//! CurrencyMap providing simplified currency code to `&dyn Currency` lookup.
use std::collections::HashMap;

use crate::Currency;

/// Provides a map from currency code to `&dyn Currency`.
///
/// Use this to lookup currencies dynamically based on a provided code.
#[derive(Debug, Default)]
pub struct CurrencyMap<'c> {
    map: HashMap<&'c str, &'c dyn Currency>,
}

impl<'c> CurrencyMap<'c> {
    /// Construct a new empty [CurrencyMap]
    pub fn new() -> Self {
        CurrencyMap {
            map: HashMap::new(),
        }
    }

    /// Constructs a new [CurrencyMap] populated with all the
    /// currencies returned from the provided collection's iterator.
    pub fn from_collection<I>(currencies: I) -> Self
    where
        I: IntoIterator<Item = &'c dyn Currency>,
    {
        CurrencyMap {
            map: currencies.into_iter().map(|c| (c.code(), c)).collect(),
        }
    }

    /// Inserts a [Currency] into the map. If there was an existing
    /// entry already in the map for the same code, it will be returned.
    pub fn insert(&mut self, currency: &'c dyn Currency) -> Option<&'c dyn Currency> {
        self.map.insert(currency.code(), currency)
    }

    /// Returns the [Currency] associated with the provided `code`, or
    /// None if no currency matching that code exists in the map.
    pub fn get(&self, code: &'c str) -> Option<&'c dyn Currency> {
        self.map.get(code).copied()
    }
}
