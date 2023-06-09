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

impl<N: Network> FromBytes for Operand<N> {
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        match u8::read_le(&mut reader) {
            Ok(0) => Ok(Self::Literal(Literal::read_le(&mut reader)?)),
            Ok(1) => Ok(Self::Register(Register::read_le(&mut reader)?)),
            Ok(2) => Ok(Self::ProgramID(ProgramID::read_le(&mut reader)?)),
            Ok(3) => Ok(Self::Caller),
            Ok(variant) => Err(error(format!("Failed to deserialize operand variant {variant}"))),
            Err(err) => Err(err),
        }
    }
}

impl<N: Network> ToBytes for Operand<N> {
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        match self {
            Self::Literal(literal) => {
                0u8.write_le(&mut writer)?;
                literal.write_le(&mut writer)
            }
            Self::Register(register) => {
                1u8.write_le(&mut writer)?;
                register.write_le(&mut writer)
            }
            Self::ProgramID(program_id) => {
                2u8.write_le(&mut writer)?;
                program_id.write_le(&mut writer)
            }
            Self::Caller => 3u8.write_le(&mut writer),
        }
    }
}
