// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use super::*;

impl<N: Network> Eq for Value<N> {}

impl<N: Network> PartialEq for Value<N> {
    /// Returns `true` if `self` and `other` are equal.
    fn eq(&self, other: &Self) -> bool {
        *self.is_equal(other)
    }
}

impl<N: Network> Equal<Self> for Value<N> {
    type Output = Boolean<N>;

    /// Returns `true` if `self` and `other` are equal.
    fn is_equal(&self, other: &Self) -> Self::Output {
        match (self, other) {
            (Self::Plaintext(a), Self::Plaintext(b)) => a.is_equal(b),
            (Self::Record(a), Self::Record(b)) => a.is_equal(b),
            (Self::Plaintext(..), _) | (Self::Record(..), _) => Boolean::new(false),
        }
    }

    /// Returns `true` if `self` and `other` are *not* equal.
    fn is_not_equal(&self, other: &Self) -> Self::Output {
        match (self, other) {
            (Self::Plaintext(a), Self::Plaintext(b)) => a.is_not_equal(b),
            (Self::Record(a), Self::Record(b)) => a.is_not_equal(b),
            (Self::Plaintext(..), _) | (Self::Record(..), _) => Boolean::new(true),
        }
    }
}
