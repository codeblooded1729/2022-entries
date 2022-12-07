// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::traits::{FftParameters, Field};

/// The interface for fields that are able to be used in FFTs.
pub trait FftField: Field + From<u128> + From<u64> + From<u32> + From<u16> + From<u8> {
    type FftParameters: FftParameters;

    /// Returns the 2^s root of unity.
    fn two_adic_root_of_unity() -> Self;

    /// Returns the 2^s * small_subgroup_base^small_subgroup_base_adicity root of unity
    /// if a small subgroup is defined.
    fn large_subgroup_root_of_unity() -> Option<Self>;

    /// Returns the multiplicative generator of `char()` - 1 order.
    fn multiplicative_generator() -> Self;

    /// Returns the root of unity of order n, if one exists.
    /// If no small multiplicative subgroup is defined, this is the 2-adic root of unity of order n
    /// (for n a power of 2).
    /// If a small multiplicative subgroup is defined, this is the root of unity of order n for
    /// the larger subgroup generated by `FftParams::LARGE_SUBGROUP_ROOT_OF_UNITY`
    /// (for n = 2^i * FftParams::SMALL_SUBGROUP_BASE^j for some i, j).
    fn get_root_of_unity(n: usize) -> Option<Self> {
        let mut omega: Self;
        if let Some(large_subgroup_root_of_unity) = Self::large_subgroup_root_of_unity() {
            let q = Self::FftParameters::SMALL_SUBGROUP_BASE
                .expect("LARGE_SUBGROUP_ROOT_OF_UNITY should only be set in conjunction with SMALL_SUBGROUP_BASE")
                as usize;
            let small_subgroup_base_adicity = Self::FftParameters::SMALL_SUBGROUP_BASE_ADICITY.expect(
                "LARGE_SUBGROUP_ROOT_OF_UNITY should only be set in conjunction with SMALL_SUBGROUP_BASE_ADICITY",
            );

            let q_adicity = Self::k_adicity(q, n);
            let q_part = q.pow(q_adicity);

            let two_adicity = Self::k_adicity(2, n);
            let two_part = 1 << two_adicity;

            if n != two_part * q_part
                || (two_adicity > Self::FftParameters::TWO_ADICITY)
                || (q_adicity > small_subgroup_base_adicity)
            {
                return None;
            }

            omega = large_subgroup_root_of_unity;
            for _ in q_adicity..small_subgroup_base_adicity {
                omega = omega.pow(&[q as u64]);
            }

            for _ in two_adicity..Self::FftParameters::TWO_ADICITY {
                omega.square_in_place();
            }
        } else {
            // Compute the next power of 2.
            let size = n.next_power_of_two() as u64;
            let log_size_of_group = size.trailing_zeros();

            if n != size as usize || log_size_of_group > Self::FftParameters::TWO_ADICITY {
                return None;
            }

            // Compute the generator for the multiplicative subgroup.
            // It should be 2^(log_size_of_group) root of unity.
            omega = Self::two_adic_root_of_unity();
            for _ in log_size_of_group..Self::FftParameters::TWO_ADICITY {
                omega.square_in_place();
            }
        }
        Some(omega)
    }

    /// Calculates the k-adicity of n, i.e., the number of trailing 0s in a base-k
    /// representation.
    fn k_adicity(k: usize, mut n: usize) -> u32 {
        let mut r = 0;
        while n > 1 {
            if n % k == 0 {
                r += 1;
                n /= k;
            } else {
                return r;
            }
        }
        r
    }
}
