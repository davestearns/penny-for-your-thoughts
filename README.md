# Penny for your Thoughts (on Rust trait bounds)

[![CI](https://github.com/davestearns/penny-for-your-thoughts/actions/workflows/ci.yml/badge.svg)](https://github.com/davestearns/penny-for-your-thoughts/actions/workflows/ci.yml)

This library implements a `Money` datatype that supports both a statically-typed and dynamically-typed `Currency`. That is you can create a `Money<USD>` that is a totally different type than a `Money<JPY>`, or you can create a `Money<&dyn Currency>` that could be any `Currency` but still safely do math with it (i.e., `Money<&dyn Currency> + Money<&dyn Currency>` returns a fallible `Result` when the currencies are actually different).

My main motivation for building this was to learn more about Rust trait bounds and operator implementations. But I was also recently looking for a crate to represent an amount of money in a currency, and I noticed that the most popular one, [rusty_money](https://github.com/varunsrin/rusty_money), hasn't been updated in a while, and has several outstanding issues and pull requests that are more than a year old. It also has a rather un-ergonomic API and set of behaviors: for example, it requires the use of explicit lifetimes (which infect all types that use it), and it simply panics when you do math on instances with different currencies.

Although I'm fairly new to Rust, I felt like the powerful language features could support a better and more flexible experience, so I built something new, and learned a lot about Rust along the way!

## Requirements

I wanted a Money data type that offered the following features:

* Holds a [Decimal](https://docs.rs/rust_decimal/latest/rust_decimal/) amount in a particular currency.
* Supports instances with statically-typed currencies, so a `Money<USD>` would be a different type than a `Money<JPY>`. This makes it impossible to accidentally pass a USD amount to a function expecting a JPY amount, or add the two together. None of that would even compile.
* But also supports instances with dynamically-typed currencies: i.e., a `Money<&dyn Currency>`. This is often necessary in APIs where the caller passes a string amount and a currency code, so you need to lookup the currency dynamically.
* Allows equality and ordering comparisons on both statically and dynamically-typed instances. For example, it should be possible to ask if a `Money<USD>` is equal to a `Money<&dyn Currency>` since you can answer that in an infallible way.
* Allows math operations in a safe way. For example, when adding with `Money<USD>` instances, it should just return a `Money<USD>`, but when adding two `Money<&dyn Currency>` instances, it should return a `Result` since it might fail if the currencies are actually different.

Amazingly, Rust's language features do make all of this possible! In the rest of this README, I'll explain how I made this work, and discuss a few of the approaches I tried that didn't quite work.

