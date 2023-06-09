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

impl<N: Network> FromBytes for PuzzleCommitment<N> {
    /// Reads the puzzle commitment from the buffer.
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        let commitment = KZGCommitment::read_le(&mut reader)?;

        Ok(Self::new(commitment))
    }
}

impl<N: Network> ToBytes for PuzzleCommitment<N> {
    /// Writes the puzzle commitment to the buffer.
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        self.commitment.write_le(&mut writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use console::network::Testnet3;

    type CurrentNetwork = Testnet3;

    #[test]
    fn test_bytes() -> Result<()> {
        let mut rng = TestRng::default();
        // Sample a new puzzle commitment.
        let expected = PuzzleCommitment::<CurrentNetwork>::new(KZGCommitment(rng.gen()));

        // Check the byte representation.
        let expected_bytes = expected.to_bytes_le()?;
        assert_eq!(expected_bytes.len(), 48);
        assert_eq!(expected, PuzzleCommitment::read_le(&expected_bytes[..])?);
        assert!(PuzzleCommitment::<CurrentNetwork>::read_le(&expected_bytes[1..]).is_err());

        Ok(())
    }
}
