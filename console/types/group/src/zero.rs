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

impl<E: Environment> Zero for Group<E> {
    /// Returns the `0` element of the group.
    fn zero() -> Self {
        Self::from_projective(E::Projective::zero())
    }

    /// Returns `true` if the element is zero.
    fn is_zero(&self) -> bool {
        self.group.is_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_console_network_environment::Console;

    type CurrentEnvironment = Console;

    const ITERATIONS: u64 = 100;

    #[test]
    fn test_zero() {
        let zero = Group::<CurrentEnvironment>::zero();

        for bit in zero.to_bits_le().iter() {
            assert!(!bit)
        }
    }

    #[test]
    fn test_is_zero() {
        assert!(Group::<CurrentEnvironment>::zero().is_zero());

        let mut rng = TestRng::default();

        // Note: This test technically has a `1 / MODULUS` probability of being flaky.
        for _ in 0..ITERATIONS {
            let group: Group<CurrentEnvironment> = Uniform::rand(&mut rng);
            assert!(!group.is_zero());
        }
    }
}
