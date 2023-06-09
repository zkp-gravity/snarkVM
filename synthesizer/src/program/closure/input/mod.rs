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

mod bytes;
mod parse;

use console::{
    network::prelude::*,
    program::{Register, RegisterType},
};

/// An input statement defines an input argument to a function, and is of the form
/// `input {register} as {register_type}`.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Input<N: Network> {
    /// The input register.
    register: Register<N>,
    /// The input register type.
    register_type: RegisterType<N>,
}

impl<N: Network> Input<N> {
    /// Returns the input register.
    #[inline]
    pub const fn register(&self) -> &Register<N> {
        &self.register
    }

    /// Returns the input register type.
    #[inline]
    pub const fn register_type(&self) -> &RegisterType<N> {
        &self.register_type
    }
}

impl<N: Network> TypeName for Input<N> {
    /// Returns the type name as a string.
    #[inline]
    fn type_name() -> &'static str {
        "input"
    }
}

impl<N: Network> Ord for Input<N> {
    /// Ordering is determined by the register (the register type is ignored).
    fn cmp(&self, other: &Self) -> Ordering {
        self.register().cmp(other.register())
    }
}

impl<N: Network> PartialOrd for Input<N> {
    /// Ordering is determined by the register (the register type is ignored).
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use console::network::Testnet3;

    type CurrentNetwork = Testnet3;

    #[test]
    fn test_input_type_name() -> Result<()> {
        assert_eq!(Input::<CurrentNetwork>::type_name(), "input");
        Ok(())
    }

    #[test]
    fn test_input_partial_ord() -> Result<()> {
        let input1 = Input::<CurrentNetwork>::from_str("input r0 as field;")?;
        let input2 = Input::<CurrentNetwork>::from_str("input r1 as field;")?;

        let input3 = Input::<CurrentNetwork>::from_str("input r0 as signature;")?;
        let input4 = Input::<CurrentNetwork>::from_str("input r1 as signature;")?;

        assert_eq!(input1.partial_cmp(&input1), Some(Ordering::Equal));
        assert_eq!(input1.partial_cmp(&input2), Some(Ordering::Less));
        assert_eq!(input1.partial_cmp(&input3), Some(Ordering::Equal));
        assert_eq!(input1.partial_cmp(&input4), Some(Ordering::Less));

        assert_eq!(input2.partial_cmp(&input1), Some(Ordering::Greater));
        assert_eq!(input2.partial_cmp(&input2), Some(Ordering::Equal));
        assert_eq!(input2.partial_cmp(&input3), Some(Ordering::Greater));
        assert_eq!(input2.partial_cmp(&input4), Some(Ordering::Equal));

        assert_eq!(input3.partial_cmp(&input1), Some(Ordering::Equal));
        assert_eq!(input3.partial_cmp(&input2), Some(Ordering::Less));
        assert_eq!(input3.partial_cmp(&input3), Some(Ordering::Equal));
        assert_eq!(input3.partial_cmp(&input4), Some(Ordering::Less));

        assert_eq!(input4.partial_cmp(&input1), Some(Ordering::Greater));
        assert_eq!(input4.partial_cmp(&input2), Some(Ordering::Equal));
        assert_eq!(input4.partial_cmp(&input3), Some(Ordering::Greater));
        assert_eq!(input4.partial_cmp(&input4), Some(Ordering::Equal));

        Ok(())
    }
}
