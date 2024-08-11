<div align="center">
    <img src="https://img.shields.io/github/license/SleeplessOne1917/pretty-num?style=for-the-badge" alt="License"/>
    <img src="https://img.shields.io/github/last-commit/SleeplessOne1917/pretty-num?style=for-the-badge&logo=github" alt="Last commit"/>
    <img src="https://img.shields.io/github/stars/SleeplessOne1917/pretty-num?style=for-the-badge&logo=github" alt="GitHub stars"/>
    <img src="https://img.shields.io/crates/v/pretty-num?style=for-the-badge" alt="Latest version" />
    <img src="https://img.shields.io/crates/dv/pretty-num?style=for-the-badge" alt="Latest version downloads" />
    <img src="https://img.shields.io/crates/size/pretty-num?style=for-the-badge&logo=rust&label=Code%20Size" alt="Code size" />
</div>

# prettty-num

Format integers into a compact social media style format, similar to using `Intl.NumberFormat("en", { notation: "compact" });` as a number formatter in Javascript.

## Usage

Simply import the `PrettyNumber` trait into your module to use the `pretty_format` method on any number that can convert into an `i64`:

```rust
use pretty_num::PrettyNumber;
// Integers with a magnitude less than 1,000 do not get compacted.
assert_eq!(534.pretty_format(), String::from("534"));

// Integers with a magnitude greater than or equal to 1,000 get compacted.
assert_eq!(15_000.pretty_format(), String::from("15k"));

// Integers will have a single decimal point when rounded.
assert_eq!(4_230_542.pretty_format(), String::from("4.2M"));

// Formatted numbers get rounded to a number without a decimal place when appropriate.
assert_eq!(5_031.pretty_format(), String::from("5k"));

// Also works with negative numbers.
assert_eq!((-25_621_783).pretty_format(), String::from("-25.6M"));

// Can go as high as trillions!
assert_eq!(36_777_121_590_100i64.pretty_format(), String::from("36.8T"));
```

## Why use this instead of another number formatting crate?

There are several other number formatting libraries for Rust such as [`numfmt`](https://crates.io/crates/numfmt), [`human_format`](https://crates.io/crates/human_format), [`si_format`](https://crates.io/crates/si_format), and [`si-scale`](https://crates.io/crates/si-scale). All of these crates are more flexible than this one. However, all of them have a fixed number of decimals. If you want to, for example, have 12 formatted as "12" and 1500 formatted as "1.5k", you will not be able to do so: you can get "2.0" and "1.5k" or "2" and "2k", but they all use an exact number of significant digits/decimal points. If compact numbers that omit the decimal when appropriate is all you need, this is the crate for you. Otherwise, the crates mentioned above are likely more appropriate for your usecase.
