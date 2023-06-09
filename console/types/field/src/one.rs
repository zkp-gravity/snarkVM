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

impl<E: Environment> One for Field<E> {
    /// Returns the `1` element of the field.
    fn one() -> Self {
        Self::new(E::Field::one())
    }

    /// Returns `true` if the element is one.
    fn is_one(&self) -> bool {
        self.field.is_one()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_console_network_environment::Console;

    type CurrentEnvironment = Console;

    const ITERATIONS: u64 = 100;

    #[test]
    fn test_one() {
        let one = Field::<CurrentEnvironment>::one();

        for (index, bit) in one.to_bits_le().iter().enumerate() {
            match index == 0 {
                true => assert!(bit),
                false => assert!(!bit),
            }
        }
    }

    #[test]
    fn test_is_one() {
        assert!(Field::<CurrentEnvironment>::one().is_one());

        let mut rng = TestRng::default();

        // Note: This test technically has a `1 / MODULUS` probability of being flaky.
        for _ in 0..ITERATIONS {
            let field: Field<CurrentEnvironment> = Uniform::rand(&mut rng);
            assert!(!field.is_one());
        }
    }
}
