#![warn(missing_docs)]
//! This crate formats numbers in a compact form similar to that used on social media sites:
//! ```
//! use pretty_num::PrettyNumber;
//! 
//! assert_eq!(23_520_123.pretty_format(), String::from("23.5M"));
//! ```

const SUFFIXES: [char; 4] = ['k', 'M', 'B', 'T'];

/// A number that can be formatted prettily.
pub trait PrettyNumber {
    /// Formats an integer to be more compact. The resulting string will have a maximum of 3 significant digits with no more than one decimal point.
    /// # Examples
    /// ```
    /// # use pretty_num::PrettyNumber;
    /// // Integers with a magnitude less than 1,000 do not get compacted.
    /// assert_eq!(534.pretty_format(), String::from("534"));
    /// 
    /// // Integers with a magnitude greater than or equal to 1,000 get compacted.
    /// assert_eq!(15_000.pretty_format(), String::from("15k"));
    /// 
    /// // Integers will have a single decimal point when rounded.
    /// assert_eq!(4_230_542.pretty_format(), String::from("4.2M"));
    /// 
    /// // Formatted numbers get rounded to a number without a decimal place when appropriate.
    /// assert_eq!(5_031.pretty_format(), String::from("5k"));
    /// 
    /// // Also works with negative numbers.
    /// assert_eq!((-25_621_783).pretty_format(), String::from("-25.6M"));
    /// 
    /// // Can go as high as trillions!
    /// assert_eq!(36_777_121_590_100i64.pretty_format(), String::from("36.8T"));
    /// ```
    /// # Panics
    /// This function panics if it is passed a number greater than 1 quadrillion or less than negative 1 quadrillion.
    fn pretty_format(self) -> String;
}

impl<N: Into<i64>> PrettyNumber for N {
    fn pretty_format(self) -> String {
        let number: i64 = self.into();

        if number.abs() < 1000 {
            number.to_string()
        } else {
            let sign: i8 = if number < 0 { -1 } else { 1 };
            let mut number_as_float = number.abs() as f32;
            for suffix in SUFFIXES {
                number_as_float /= 1000f32;

                if number_as_float < 1000f32 {
                    return format!(
                        "{:.*}{suffix}",
                        if (number_as_float - number_as_float.floor()) < 0.1
                            || number_as_float >= 100f32
                        {
                            0
                        } else {
                            1
                        },
                        sign as f32 * number_as_float
                    );
                }
            }

            panic!("Number {number} is larger than 1 quadrillion!");
        }
    }
}

#[cfg(test)]
mod test {
    use crate::PrettyNumber;
    use rstest::rstest;

    #[rstest]
    #[case(7, "7")]
    #[case(42, "42")]
    #[case(717, "717")]
    #[case(-5, "-5")]
    #[case(-76, "-76")]
    #[case(-224, "-224")]
    #[case(1_001, "1k")]
    #[case(1_624, "1.6k")]
    #[case(-5_020, "-5k")]
    #[case(-9_505, "-9.5k")]
    #[case(19_007, "19k")]
    #[case(73_444, "73.4k")]
    #[case(-55_033, "-55k")]
    #[case(-42_780, "-42.8k")]
    #[case(469_070, "469k")]
    #[case(945_661, "946k")]
    #[case(-223_090, "-223k")]
    #[case(-671_522, "-672k")]
    #[case(3_001_500, "3M")]
    #[case(7_926_400, "7.9M")]
    #[case(-4_030_115, "-4M")]
    #[case(-3_333_221, "-3.3M")]
    #[case(75_032_115, "75M")]
    #[case(23_333_452, "23.3M")]
    #[case(-54_012_560, "-54M")]
    #[case(-11_740_662, "-11.7M")]
    #[case(555_067_885, "555M")]
    #[case(352_344_120, "352M")]
    #[case(-222_000_554, "-222M")]
    #[case(-434_875_500, "-435M")]
    #[case(2_004_254_578, "2B")]
    #[case(7_667_973_223, "7.7B")]
    #[case(-4_002_154_900, "-4B")]
    #[case(-6_534_664_725, "-6.5B")]
    #[case(87_050_671_768, "87B")]
    #[case(44_444_333_222, "44.4B")]
    #[case(-32_010_345_093, "-32B")]
    #[case(-65_420_132_543, "-65.4B")]
    #[case(899_055_111_032, "899B")]
    #[case(723_999_324_999, "724B")]
    #[case(-666_000_142_543, "-666B")]
    #[case(-400_601_897_231, "-401B")]
    #[case(5_000_023_667_158, "5T")]
    #[case(1_222_333_444_555, "1.2T")]
    #[case(-4_000_354_984_333, "-4T")]
    #[case(-6_923_000_178_126, "-6.9T")]
    #[case(66_001_789_809_223, "66T")]
    #[case(93_723_000_151_300, "93.7T")]
    #[case(-50_032_745_113_006, "-50T")]
    #[case(-11_444_653_221_094, "-11.4T")]
    #[case(343_003_766_091_322, "343T")]
    #[case(357_455_634_091_722, "357T")]
    #[case(-567_023_400_999_234, "-567T")]
    #[case(-871_567_223_222_546, "-872T")]
    fn pretty_format_test(#[case] input: i64, #[case] expected: &str) {
        assert_eq!(input.pretty_format().as_str(), expected);
    }

    #[rstest]
    #[case(1_000_000_000_000_000)]
    #[case(-1_000_000_000_000_000)]
    #[should_panic]
    fn format_quadrillion_should_panic(#[case] num: i64) {
        let _ = num.pretty_format();
    }
}
