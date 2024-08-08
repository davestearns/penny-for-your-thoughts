# Penny for your Thoughts (on Rust trait bounds)

[![CI](https://github.com/davestearns/penny-for-your-thoughts/actions/workflows/ci.yml/badge.svg)](https://github.com/davestearns/penny-for-your-thoughts/actions/workflows/ci.yml)

This library implements a `Money` datatype that supports both a statically-typed and dynamically-typed `Currency`. That is to say, you can create a `Money<USD>` that is a totally different type than a `Money<JPY>`, or you can create a `Money<&dyn Currency>` where the currency is determined at runtime, but still safely do math with it (i.e., `Money<&dyn Currency> + Money<&dyn Currency>` returns a fallible `Result` because the currencies might be different).

My main motivation for building this was to learn more about Rust trait bounds and custom operators. But I was also recently looking for a crate to represent an amount of money in a currency, and I noticed that the most popular one, [rusty_money](https://github.com/varunsrin/rusty_money), hasn't been updated in a while, and has several outstanding issues and pull requests that are more than a year old. It also has a rather un-ergonomic API and set of behaviors: for example, it requires the use of explicit lifetimes (which naturally infect all types that use it), and it simply panics when you do math on instances with different currencies.

Although I'm fairly new to Rust, I felt like the powerful language features could support a better and more flexible experience, so I built something new, and learned a lot about Rust along the way!

## Requirements

I wanted a Money data type that offered the following features:

* **Tracks the amount as a [high-precision decimal](https://docs.rs/rust_decimal/latest/rust_decimal/):** The standard floating point data types can't be used for monetary amounts because even simple addition can produce rather [strange results](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8a75fcb20fcbf7f9df12947dea347720). A common alternative is to track the amount in currency minor units (e.g., cents of USD), but this becomes awkward when a currency decides to change its number of minor units, as [Iceland did in 2007](https://www.ibm.com/support/pages/apar/PK52556). It also makes it difficult to represent fractional minor units, such as a stock price expressed in eighths of a cent.
* **Supports instances with statically-typed currencies:** In some applications you know the currency at compile time, and you want to ensure that an amount of `Money` in one currency can't accidentally be passed to a function expecting an amount in a different currency. In other words, you want `Money<USD>` and `Money<JPY>` to be totally different _types_, so that it becomes a _compile error_ to mix them up.
* **Supports instances with dynamically-typed currencies:** In other applications, you don't know the currency until runtime, so we need to support that as well. For example, you might get an API request with an amount and a three-character currency code, so you need to lookup the currency in a map and create a `Money<&dyn Currency>`.
* **Allows equality comparisons:** Regardless of whether the currency is statically or dynamically-typed, you should be able to test two instances for equality since that can never fail--they might be unequal, but the comparison is always a valid thing to do.
* **Supports math operations in a safe way:** If you add two `Money<USD>` instances, you should get a `Money<USD>` since the compiler ensures the currencies are the same. But if you add two `Money<&dyn Currency>` instances, or a mix of statically and dynamically-typed currencies, you should get a `Result` since the operation could fail if the currencies are actually different. The `Result` type supports chaining through the `.and_then()` method, so one can still work with multiple terms in a safe way.

Amazingly, Rust's language features do make all of this possible! In the rest of this README, I'll explain how I made this work, and discuss a few of the approaches I tried that didn't quite work.

## Currency Trait and Implementations

The first step was to define a `Currency` trait that all currencies must implement. I kept it simple for now, but one could expand this in the future to include other details:

```rust
/// Common trait for all currencies.
pub trait Currency {
    /// Returns the unique ISO alphabetic code for this currency
    /// (e.g., "USD" or "JPY").
    fn code(&self) -> &'static str;
    /// Returns the number of minor units supported by the currency.
    /// Currencies like USD and EUR currently support 2, but others
    /// like JPY or KRW support zero.
    fn minor_units(&self) -> u32;
    /// Returns the symbol used to represent this currency.
    /// For example `$` for USD or `¥` for JPY. Some currencies
    /// use a series of letters instead of a special symbol
    /// (e.g., `CHF` or `Lek`). If the currency has no defined
    /// symbol, this will return an empty string.
    fn symbol(&self) -> &'static str;
    /// Returns the informal name for this currency.
    fn name(&self) -> &'static str;
    /// Returns the unique ISO numeric code for this currency.
    fn numeric_code(&self) -> u32;
}
```

Initially I didn't include `&self` as an argument on these methods because I figured the implementations would just return static data, but this created a problem when I tried to build a reference to a dynamically-typed currency: `&dyn Currency`. To do this, Rust requires the trait to be "object safe," which means the compiler can build a v-table and do dynamic dispatch. Without a reference to `&self`, there would be no way to know which implementation of the trait method to call at runtime, so `&self` must be an argument, even if you never refer to it in your implementations.

For instances of `Currency` my first inclination was to declare an `enum` with the code as the variant name, as that must be unique. But in Rust an `enum` is a type and the variants of that enum are all _instances of the same type_. So if I declared the currencies as something like `enum CurrencySet` all the money instances would end up being `Money<CurrencySet>`, which would defeat our desire to support statically-typed currencies. The same would be true if I declared just one `CurrencyImpl` struct and declared constant instances of it for the various currencies.

Instead, we need each `Currency` implementation to be it's own _type_. The easiest way to do that is to declare them as separate `struct`s, each of which `impl Currency`:

```rust
/// US Dollar
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct USD;
impl Currency for USD {
    fn code(&self) -> &'static str {
        "USD"
    }

    fn symbol(&self) -> &'static str {
        "$"
    }

    fn name(&self) -> &'static str {
        "US Dollar"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        840
    }
}

/// Yen
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JPY;
impl Currency for JPY {
    fn code(&self) -> &'static str {
        "JPY"
    }

    fn symbol(&self) -> &'static str {
        "¥"
    }

    fn name(&self) -> &'static str {
        "Yen"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        392
    }
}
```

Declaring `USD` and `JPY` as separate `struct`s makes them separate _types_, which will enable us to create statically-typed `Money<USD>` vs `Money<JPY>`.

## Money Type

Now that we have some currencies defined, we can build our `Money` type:

```rust
use rust_decimal::Decimal;

/// An amount of money in a particular currency.
#[derive(Debug, Clone)]
pub struct Money<C> {
    amount: Decimal,
    currency: C,
}
```

We define a generic type argument `C` for the currency, but notice that I don't add a trait bound here in the struct definition. That is, I just declare `Money<C>` not `Money<C: Currency>`. When I first started learning Rust I tended to add trait bounds on my `struct` definitions, but I realized this was both unnecessary and restrictive. Since you must add trait bounds on the `impl` blocks when referring to trait methods, and because they only way to create or interact with the type is through methods defined in the `impl` blocks, it's typically unnecessary to add trait bounds on the struct itself. But it's also overly restrictive: we don't want to restrict `C` to be only a `Currency` as we also want to support an `&dyn Currency` or maybe even a `Box<dyn Currency>`. We can do that using separate `impl` blocks with different trait bounds and types for `C`.

At first I tried to construct a single impl block with a trait bound that allowed either an owned `Currency` implementation OR a reference to a dynamic `Currency`, but that doesn't actually make sense, as an `&dyn Currency` is actually a _type_ not a _trait_, so it can't be used as a trait bound. But it can be used as the type for a generic type argument in a separate `impl` block, which you'll see below.

I also considered implementing `Currency` for `&dyn Currency`, which is possible in Rust, but that would erase the distinction between the two: it would then be possible to pass a `Money` with a dynamically-typed `Currency` to a function expecting a statically-typed `Currency`, and the actual currencies might not be the same!

So our first `impl` block should be for methods that don't really care what type `C` actually is:

```rust
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
```

The `new()` and `amount()` method don't really need to know what type `C` actually is, so we can define them once. This does have an interesting drawback, however: one can pass _any_ type for the `currency` argument, so one could construct a `Money<String>` or `Money<Foo>` where `Foo` is not a `Currency`. Although that's strange, it's probably fine since you can't do much with that `Money` instance without calling methods defined in the other `impl` blocks, which will establish bounds on the type of `C`. But if you find this distasteful, see the "Marker Trait for New" section below for an interesting solution.

## Statically-Typed Currencies

The next `impl` block defines methods that are specific to owned statically-typed `Currency` instances:

```rust
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
```

Here we add a trait bound on `C` of `Currency + Copy`, meaning that whatever the caller is using for `C` it must be an owned `Currency` instance that also supports copy semantics. This allows us to return a copy of the `Currency` instance from the `currency()` method. Since `USD` and `JPY` are unit structs, copying them doesn't require any significant work, so it's fine and convenient to just return a copy instead of a reference.

Now we can create Money instances with a statically-typed `Currency`:

```rust
// m_usd is type Money<USD>
let m_usd = Money::new(Decimal::ONE, USD);
assert_eq!(m_usd.currency(), USD);
assert_eq!(m_usd.amount(), Decimal::ONE);

// m_jpy is type Money<JPY>
let m_jpy = Money::new(Decimal::ONE, JPY);
assert_eq!(m_jpy.currency(), JPY);
assert_eq!(m_jpy.amount(), Decimal::ONE);

// This won't even compile because they are totally different types
// assert_eq!(m_usd, m_jpy);
```

## Dynamically-Typed Currencies

To support references to dynamically-typed currencies, we can add another `impl` block where we provide a specific type for the generic `C` type argument:

```rust
/// Functions specifically for borrowed dynamically-typed currencies.
impl<'c> Money<&'c dyn Currency> {
    /// Returns the reference to the dynamically-typed Currency.
    pub fn currency(&self) -> &'c dyn Currency {
        self.currency
    }
}
```

There are a few subtleties to note here. First, we can't do this with a trait bound like we did above because `&'c dyn Currency` is a _type_ not a _trait_. But that's okay because we can simply use that as the explicit type for `C` in this `impl` block.

Second, we declare a lifetime argument `'c` for the `impl` block, and use that as the lifetime of the `Currency` references. This will make compiler enforce that the `Currency` instance lives for at least as long as the `Money` instance does, which is good because we are holding a reference to it. Thankfully, callers won't have to deal with this lifetime argument in their code, as the compiler can work it out from context. One will be able to simply do something like this:

```rust
// CURRENCIES is a HashMap<'static str, &'static dyn Currency>
// so dynamic_currency is of type `&dyn Currency`
let dynamic_currency = CURRENCIES.get("USD").unwrap();

// money is of type `Money<&dyn Currency>`
let money = Money::new(Decimal::ONE, dynamic_currency);
assert_eq!(money.currency().code(), "USD");

let other_money = Money::new(Decimal::ONE, CURRENCIES.get("JPY").unwrap());
assert_eq!(other_money.currency().code(), "JPY");
```

Third, you might be surprised that we can declare another method with the same name as the method we just declared in the previous `impl` block. Rust allows this for methods that take `&self` as an argument because it can use that to determine the correct implementation. And in this case we can redefine the return type to be the same reference we are holding rather than a copy of an owned statically-typed `Currency` value.

The implication here is that methods that do not take `&self` as an argument cannot be "overloaded" (so to speak). For example, I initially tried defining different versions of the `new()` method in the different `impl` blocks, the first taking an owned `Currency` value and the second taking a `&dyn Currency` reference, but Rust doesn't currently allow that: the declaration will work, but when you try to use one of those `new()` methods you'll get an error saying there are multiple candidates and it can't figure out which one you want to call. There might be a syntax to disambiguate, but I couldn't figure it out, which means my callers probably won't be able to either.

## Supporting Safe Money Math

We can now create `Money` instances with static or dynamically-typed Currencies, so let's make it possible to add them in a safe way.

* `Money<USD> + Money<USD>` should return `Money<USD>` since that's infallible (though it can still overflow).
* `Money<USD> + Money<JPY>` shouldn't even compile.
* `Money<&dyn Currency> + Money<&dyn Currency>` should return a `Result` since the currencies might be different.
* `Money<USD> + Money<&dyn Currency>` and `Money<&dyn Currency> + Money<USD>` should also be possible, returning a `Result` with the `Ok` type being whatever the left-hand side's type was.

Amazingly, Rust makes all of this possible. The `Add` trait not only allows you to specify a different type for the right-hand side term, but also for the `Output` of the operation!

The statically-typed implementation is pretty straightforward:

```rust
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
```

For the dynamically-typed version, we define a `MoneyMathError` enum and set the `Output` associated type to be a `Result<Self, MoneyMathError>`:

```rust
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
```

We again specify `&'c dyn Currency` as the explicit type for the generic type argument because it's a type, not a trait, so we can't express it as a trait bound. We also check whether the currencies are the same, and return an error if they are not.

Supporting a mix of statically and dynamically-typed currencies is also possible by specifying the right-hand side type in the `Add` trait (it defaults to `Self`):

```rust
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
```

With all of this we can now do Money math like so:

```rust
// statically-typed
assert_eq!(
    Money::new(Decimal::ONE, USD) + Money::new(Decimal::ONE, USD),
    Money::new(Decimal::TWO, USD),
);

// dynamically-typed, same currency -> Ok
let currency_usd = CURRENCIES.get("USD").unwrap();
assert_eq!(
    Money::new(Decimal::ONE, currency_usd) + Money::new(Decimal::ONE, currency_usd),
    Ok(Money::new(Decimal::TWO, currency_usd)),
);

// dynamically-typed, different currencies -> Err
let currency_jpy = CURRENCIES.get("JPY").unwrap();
assert_eq!(
    Money::new(Decimal::ONE, currency_usd) + Money::new(Decimal::ONE, currency_jpy),
    Err(MoneyMathError::IncompatibleCurrencies(
        currency_usd.code(),
        currency_jpy.code(),
    )),
);

// dynamically-typed + statically-typed, same currency -> Ok(dynamically-typed)
assert_eq!(
    Money::new(Decimal::ONE, currency_usd) + Money::new(Decimal::ONE, USD),
    Ok(Money::new(Decimal::TWO, currency_usd)),
);

// dynamically-typed + statically-typed, different currencies -> Err
assert_eq!(
    Money::new(Decimal::ONE, currency_usd) + Money::new(Decimal::ONE, JPY),
    Err(MoneyMathError::IncompatibleCurrencies(
        currency_usd.code(),
        JPY.code()
    )),
);

// statically-typed, multi-term
assert_eq!(
    Money::new(Decimal::ONE, USD)
        + Money::new(Decimal::ONE, USD)
        + Money::new(Decimal::ONE, USD),
    Money::new(Decimal::new(3, 0), USD),
);

// dynamically-typed, multi-term using Result::and_then()
// (if an error occurs, closures are skipped and final result is an error)
assert_eq!(
    (Money::new(Decimal::ONE, currency_usd) + Money::new(Decimal::ONE, currency_usd))
        .and_then(|m| m + Money::new(Decimal::ONE, currency_usd)),
    Ok(Money::new(Decimal::new(3, 0), currency_usd)),
);
```

The same approach is used to support `Sub`, `Mul` and `Div` as well.

## Equality Comparisons

The asserts above rely on the ability to compare `Money` instances for equality, which requires implementing the `PartialEq` trait for both statically and dynamically-typed currencies:

```rust
/// Allows equality comparisons between Money instances with statically-typed
/// currencies. Both the amount and currency must be the same.
impl<C> PartialEq for Money<C>
where
    C: Currency + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency == other.currency
    }
}

/// Allows equality comparisons between Money instances with dynamically-typed
/// currencies. Both the amount and the currency codes must be the same.
impl<'c> PartialEq for Money<&'c dyn Currency> {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency.code() == other.currency.code()
    }
}
```

Just as with the math operations, we can also support comparing a mix of statically and dynamically-typed currencies by specifying the right-hand side type in the `PartialEq` trait (defaults to `Self`):

```rust
/// Allows equality comparisons between Money instances with dynamically-typed
/// currencies and those with statically-typed currencies
impl<'c, C> PartialEq<Money<&'c dyn Currency>> for Money<C>
where
    C: Currency,
{
    fn eq(&self, other: &Money<&'c dyn Currency>) -> bool {
        self.amount == other.amount && self.currency.code() == other.currency.code()
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
```

## Marker Trait for New

When we first saw the `Money::new()` method, I noted that it technically allows one to construct a `Money` with something that isn't actually a `Currency`. At first I tried to work around this by putting `new()` into the specific `impl` blocks like so, but this doesn't compile:

```rust
// DOES NOT COMPILE!
impl<C> Money<C>
where
    C: Currency,
{
    pub fn new(amount: Decimal, currency: C) -> Self {
        Self { amount, currency }
    }
}

impl<'c> Money<&'c dyn Currency>
{
    pub fn new(amount: Decimal, currency: &'c dyn Currency) -> Self {
        Self { amount, currency }
    }
}

fn main() {
    // COMPILE ERROR: multiple candidates
    let m_static = Money::new(Decimal::ONE, USD);
    let m_dynamic = Money::new(Decimal::ONE, &USD as &dyn Currency);
}
```
[rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1c78db9b128cad740076d19a0cc02478)

I'm not sure why the compiler can't figure out which version of `new()` to call given that the argument types are different, but it doesn't work for now.

Although we can't construct a single trait bound that allows either an owned implementation of `Currency` or a reference to a dynamic one, we can define a new trait and do a blanket implementation for those two things. For example:

```rust
// New marker trait, with blanket implementations for anything that 
// implements Currency, and any `&'c dyn Currency`
pub trait CurrencyOrRef {}
impl<C> CurrencyOrRef for C where C: Currency {}
impl<'c> CurrencyOrRef for &'c dyn Currency {}

// Single impl block using CurrencyOrRef as trait bound
impl<C> Money<C>
where
    C: CurrencyOrRef,
{
    pub fn new(amount: Decimal, currency: C) -> Self {
        Self { amount, currency }
    }
}

fn main() {
    // Now this compiles
    let m_static = Money::new(Decimal::ONE, USD);
    let m_dynamic = Money::new(Decimal::ONE, &USD as &dyn Currency);
}
```

Now it's impossible to construct a `Money<String>` or `Money<Foo>` where `Foo` is not a `Currency`. But it's also not impossible for a caller to just implement the `CurrencyOrRef` marker trait on their own `Foo` type, so it's unclear to me if this is really worth it in the end.

But this technique does make it easier to support other kinds of constructors that might need a subset of the `Currency` trait. For example, say we wanted to support creating a `Money` from some amount of currency minor units. To do that, we need to know how many minor units the currency supports, which is a method on the `Currency` trait. We could do that by making the marker trait here a bit smarter:

```rust
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

/// Methods that require knowing the `minor_units` of the currency.
impl<C> Money<C>
where
    C: MinorUnits,
{
    /// Construct a Money from a decimal amount and currency.
    /// (This doesn't strictly need the minor units but we include
    /// it here to take advantage of the marker trait).
    pub fn new(amount: Decimal, currency: C) -> Self {
        Self { amount, currency }
    }

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
```

This makes the marker trait a bit more useful and perhaps worth it.

## TODO

I still need to finish the following:

* **Currency Formatting:** Proper locale-sensitive money formatting is _very_ complicated, but this is something I'd like to add.
* **Helper Methods:** Might be useful to add various helpers, such as `split()` for minor-unit aware splitting (e.g., remainder pennies gets assigned to a subset of the buckets).

## Corrections or Suggestions?

Is there a better way to do this? I'm fairly new to Rust, so perhaps there's a mechanism I haven't run across yet that would provide a better solution. If you know of something, please open an issue and tell me about it! I'll update the code and README accordingly.
