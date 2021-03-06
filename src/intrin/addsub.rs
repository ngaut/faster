// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};

pub trait PackedAddSub {
    fn addsub(&self, other: Self) -> Self;
}

// impl PackedAddSub for f32x4 {
//     #[inline(always)]
//     fn addsub(&self, other: Self) -> Self {
//         unsafe { _mm_addsub_ps(*self, other) }
//     }
// }

// impl PackedAddSub for f64x2 {
//     #[inline(always)]
//     fn addsub(&self, other: Self) -> Self {
//         unsafe { _mm_addsub_pd(*self, other) }
//     }
// }

// impl PackedAddSub for f32x8 {
//     #[inline(always)]
//     fn addsub(&self, other: Self) -> Self {
//         unsafe { _mm256_addsub_ps(*self, other) }
//     }
// }

// impl PackedAddSub for f64x4 {
//     #[inline(always)]
//     fn addsub(&self, other: Self) -> Self {
//         unsafe { _mm256_addsub_pd(*self, other) }
//     }
// }
