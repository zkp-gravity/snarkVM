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

impl<A: Aleo> Plaintext<A> {
    /// Encrypts `self` to the given address under the given randomizer.
    pub fn encrypt(&self, address: &Address<A>, randomizer: Scalar<A>) -> Ciphertext<A> {
        // Compute the plaintext view key.
        let plaintext_view_key = (address.to_group() * randomizer).to_x_coordinate();
        // Encrypt the plaintext.
        self.encrypt_symmetric(plaintext_view_key)
    }

    /// Encrypts `self` under the given plaintext view key.
    pub fn encrypt_symmetric(&self, plaintext_view_key: Field<A>) -> Ciphertext<A> {
        // Determine the number of randomizers needed to encrypt the plaintext.
        let num_randomizers = self.num_randomizers();
        // Prepare a randomizer for each field element.
        let randomizers = A::hash_many_psd8(&[A::encryption_domain(), plaintext_view_key], num_randomizers);
        // Encrypt the plaintext.
        self.encrypt_with_randomizers(&randomizers)
    }

    /// Encrypts `self` under the given randomizers.
    pub(crate) fn encrypt_with_randomizers(&self, randomizers: &[Field<A>]) -> Ciphertext<A> {
        // Encrypt the plaintext.
        Ciphertext::from_fields(
            &self
                .to_fields()
                .into_iter()
                .zip_eq(randomizers)
                .map(|(plaintext, randomizer)| plaintext + randomizer)
                .collect::<Vec<_>>(),
        )
    }
}
