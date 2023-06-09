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

impl<N: Network> ToBits for Entry<N, Plaintext<N>> {
    /// Returns this entry as a list of **little-endian** bits.
    fn to_bits_le(&self) -> Vec<bool> {
        let mut bits_le = match self {
            Self::Constant(..) => vec![false, false],
            Self::Public(..) => vec![false, true],
            Self::Private(..) => vec![true, false],
        };
        match self {
            Self::Constant(plaintext) => bits_le.extend(plaintext.to_bits_le()),
            Self::Public(plaintext) => bits_le.extend(plaintext.to_bits_le()),
            Self::Private(plaintext) => bits_le.extend(plaintext.to_bits_le()),
        }
        bits_le
    }

    /// Returns this entry as a list of **big-endian** bits.
    fn to_bits_be(&self) -> Vec<bool> {
        let mut bits_be = match self {
            Self::Constant(..) => vec![false, false],
            Self::Public(..) => vec![false, true],
            Self::Private(..) => vec![true, false],
        };
        match self {
            Self::Constant(plaintext) => bits_be.extend(plaintext.to_bits_be()),
            Self::Public(plaintext) => bits_be.extend(plaintext.to_bits_be()),
            Self::Private(plaintext) => bits_be.extend(plaintext.to_bits_be()),
        }
        bits_be
    }
}

impl<N: Network> ToBits for Entry<N, Ciphertext<N>> {
    /// Returns this entry as a list of **little-endian** bits.
    fn to_bits_le(&self) -> Vec<bool> {
        let mut bits_le = match self {
            Self::Constant(..) => vec![false, false],
            Self::Public(..) => vec![false, true],
            Self::Private(..) => vec![true, false],
        };
        match self {
            Self::Constant(plaintext) => bits_le.extend(plaintext.to_bits_le()),
            Self::Public(plaintext) => bits_le.extend(plaintext.to_bits_le()),
            Self::Private(plaintext) => bits_le.extend(plaintext.to_bits_le()),
        }
        bits_le
    }

    /// Returns this entry as a list of **big-endian** bits.
    fn to_bits_be(&self) -> Vec<bool> {
        let mut bits_be = match self {
            Self::Constant(..) => vec![false, false],
            Self::Public(..) => vec![false, true],
            Self::Private(..) => vec![true, false],
        };
        match self {
            Self::Constant(plaintext) => bits_be.extend(plaintext.to_bits_be()),
            Self::Public(plaintext) => bits_be.extend(plaintext.to_bits_be()),
            Self::Private(plaintext) => bits_be.extend(plaintext.to_bits_be()),
        }
        bits_be
    }
}
