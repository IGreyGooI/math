use std::convert::From;
use std::ops::{Add, Sub};
use std::num::Wrapping;

macro_rules! implement_angle {
    ($a:ident, $u:ident) => {
        #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
        pub struct $a {
            d: $u
        }
        impl $a {
            pub fn new(d: $u) -> Self {
                $a {
                    d
                }
            }
        }
        impl Add<$a> for $a {
            type Output = $a;
            fn add(self, _rhs: $a) -> $a {
                let d = (Wrapping(self.d) + Wrapping(_rhs.d)).0;
                $a {
                    d
                }
            }
        }
        
        impl Sub<$a> for $a {
            type Output = $a;
            fn sub(self, _rhs: $a) -> $a {
                let d = (Wrapping(self.d) - Wrapping(_rhs.d)).0;
                $a {
                    d
                }
            }
        }
    };
}

implement_angle!(a128, u128);
implement_angle!(a64, u64);
implement_angle!(a32, u32);
implement_angle!(a16, u16);
implement_angle!(a8, u8);

macro_rules! implement_convert_more_bit_to_less {
    ($from: ident, $to: ident, $bit_shift: expr, $to_data_type: ty) => {
        impl From<$from> for $to {
            fn from(angle: $from) -> $to {
                let d = (angle.d >> $bit_shift) as $to_data_type;
                $to::new(d)
            }
        }
    };
}

macro_rules! implement_convert_less_bit_to_more {
    ($from: ident, $to: ident, $bit_shift: expr, $to_data_type: ty) => {
        impl From<$from> for $to {
            fn from(angle: $from) -> $to {
                let d = (angle.d as $to_data_type) << $bit_shift;
                $to::new(d)
            }
        }
    };
}

implement_convert_more_bit_to_less!(a128, a64, 64, u64);
implement_convert_more_bit_to_less!(a128, a32, 96, u32);
implement_convert_more_bit_to_less!(a128, a16, 112, u16);
implement_convert_more_bit_to_less!(a128, a8, 120, u8);

implement_convert_more_bit_to_less!(a64, a32, 32, u32);
implement_convert_more_bit_to_less!(a64, a16, 48, u16);
implement_convert_more_bit_to_less!(a64, a8, 56, u8);

implement_convert_more_bit_to_less!(a32, a16, 16, u16);
implement_convert_more_bit_to_less!(a32, a8, 24, u8);

implement_convert_more_bit_to_less!(a16, a8, 8, u8);

implement_convert_less_bit_to_more!(a8, a16, 8, u16);
implement_convert_less_bit_to_more!(a8, a32, 24, u32);
implement_convert_less_bit_to_more!(a8, a64, 56, u64);
implement_convert_less_bit_to_more!(a8, a128, 120, u128);

implement_convert_less_bit_to_more!(a16, a32, 16, u32);
implement_convert_less_bit_to_more!(a16, a64, 48, u64);
implement_convert_less_bit_to_more!(a16, a128, 112, u128);

implement_convert_less_bit_to_more!(a32, a64, 32, u64);
implement_convert_less_bit_to_more!(a32, a128, 96, u128);

implement_convert_less_bit_to_more!(a64, a128, 64, u128);

pub trait Length {}

type RadLength = u64;

pub trait Trigonometry<L> {
    fn sine() -> L;
    fn cosine() -> L;
    fn sine_times(number: L) -> L;
    fn cosine_times(number: L) -> L;
}

impl Trigonometry<f64> for a64 {
    fn sine() -> f64 { 0.0f64 }
    fn cosine() -> f64 { 0.0f64 }
    fn sine_times(number: f64) -> f64 { 0.0f64 }
    fn cosine_times(number: f64) -> f64 { 0.0f64 }
}

impl Trigonometry<f64> for a32 {
    fn sine() -> f64 { 0.0f64 }
    
    fn cosine() -> f64 { 0.0f64 }
    fn sine_times(number: f64) -> f64 { 0.0f64 }
    fn cosine_times(number: f64) -> f64 { 0.0f64 }
}

impl Length for f64 {}







