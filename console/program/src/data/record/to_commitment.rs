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

impl<N: Network> Record<N, Plaintext<N>> {
    /// Returns the record commitment.
    pub fn to_commitment(&self, program_id: &ProgramID<N>, record_name: &Identifier<N>) -> Result<Field<N>> {
        // Construct the input as `(program_id || record_name || record)`.
        let mut input = program_id.to_bits_le();
        input.extend(record_name.to_bits_le());
        input.extend(self.to_bits_le());
        // Compute the BHP hash of the program record.
        N::hash_bhp1024(&input)
    }
}

impl<N: Network> Record<N, Ciphertext<N>> {
    /// Returns the record commitment.
    pub fn to_commitment(&self, _program_id: &ProgramID<N>, _record_name: &Identifier<N>) -> Result<Field<N>> {
        bail!("Illegal operation: Record::to_commitment() cannot be invoked on the `Ciphertext` variant.")
    }
}
