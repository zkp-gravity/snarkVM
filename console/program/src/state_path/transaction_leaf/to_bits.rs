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

impl<N: Network> ToBits for TransactionLeaf<N> {
    /// Returns the little-endian bits of the Merkle leaf.
    fn to_bits_le(&self) -> Vec<bool> {
        // Construct the leaf as (variant || index || ID).
        self.variant
            .to_bits_le()
            .into_iter()
            .chain(self.index.to_bits_le().into_iter())
            .chain(self.id.to_bits_le().into_iter())
            .collect()
    }

    /// Returns the big-endian bits of the Merkle leaf.
    fn to_bits_be(&self) -> Vec<bool> {
        // Construct the leaf as (variant || index || ID).
        self.variant
            .to_bits_be()
            .into_iter()
            .chain(self.index.to_bits_be().into_iter())
            .chain(self.id.to_bits_be().into_iter())
            .collect()
    }
}
