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

impl<N: Network> Parser for Output<N> {
    /// Parses a string into an output statement.
    /// The output statement is of the form `output {operand} as {value_type};`.
    #[inline]
    fn parse(string: &str) -> ParserResult<Self> {
        // Parse the whitespace and comments from the string.
        let (string, _) = Sanitizer::parse(string)?;
        // Parse the output keyword from the string.
        let (string, _) = tag(Self::type_name())(string)?;
        // Parse the whitespace from the string.
        let (string, _) = Sanitizer::parse_whitespaces(string)?;
        // Parse the operand from the string.
        let (string, operand) = Operand::parse(string)?;
        // Parse the whitespace from the string.
        let (string, _) = Sanitizer::parse_whitespaces(string)?;
        // Parse the "as" from the string.
        let (string, _) = tag("as")(string)?;
        // Parse the whitespace from the string.
        let (string, _) = Sanitizer::parse_whitespaces(string)?;
        // Parse the value type from the string.
        let (string, value_type) = ValueType::parse(string)?;
        // Parse the whitespace from the string.
        let (string, _) = Sanitizer::parse_whitespaces(string)?;
        // Parse the semicolon from the string.
        let (string, _) = tag(";")(string)?;
        // Return the output statement.
        Ok((string, Self { operand, value_type }))
    }
}

impl<N: Network> FromStr for Output<N> {
    type Err = Error;

    /// Parses a string into an output statement.
    #[inline]
    fn from_str(string: &str) -> Result<Self> {
        match Self::parse(string) {
            Ok((remainder, object)) => {
                // Ensure the remainder is empty.
                ensure!(remainder.is_empty(), "Failed to parse string. Found invalid character in: \"{remainder}\"");
                // Return the object.
                Ok(object)
            }
            Err(error) => bail!("Failed to parse string. {error}"),
        }
    }
}

impl<N: Network> Debug for Output<N> {
    /// Prints the output as a string.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<N: Network> Display for Output<N> {
    /// Prints the output statement as a string.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{type_} {operand} as {value_type};",
            type_ = Self::type_name(),
            operand = self.operand,
            value_type = self.value_type
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use console::{
        network::Testnet3,
        program::{Literal, Register, U8},
    };

    type CurrentNetwork = Testnet3;

    #[test]
    fn test_output_parse() -> Result<()> {
        // Register
        let output = Output::<CurrentNetwork>::parse("output r0 as field.private;").unwrap().1;
        assert_eq!(output.operand(), &Operand::Register(Register::<CurrentNetwork>::Locator(0)));
        assert_eq!(output.value_type(), &ValueType::<CurrentNetwork>::from_str("field.private")?);

        // Literal
        let output = Output::<CurrentNetwork>::parse("output 0u8 as u8.public;").unwrap().1;
        assert_eq!(output.operand(), &Operand::Literal(Literal::<CurrentNetwork>::U8(U8::new(0))));
        assert_eq!(output.value_type(), &ValueType::<CurrentNetwork>::from_str("u8.public")?);

        // Struct
        let output = Output::<CurrentNetwork>::parse("output r1 as signature.private;").unwrap().1;
        assert_eq!(output.operand(), &Operand::Register(Register::<CurrentNetwork>::Locator(1)));
        assert_eq!(output.value_type(), &ValueType::<CurrentNetwork>::from_str("signature.private")?);

        // Record
        let output = Output::<CurrentNetwork>::parse("output r2 as token.record;").unwrap().1;
        assert_eq!(output.operand(), &Operand::Register(Register::<CurrentNetwork>::Locator(2)));
        assert_eq!(output.value_type(), &ValueType::<CurrentNetwork>::from_str("token.record")?);

        Ok(())
    }

    #[test]
    fn test_output_display() {
        // Register
        let output = Output::<CurrentNetwork>::parse("output r0 as field.private;").unwrap().1;
        assert_eq!(format!("{output}"), "output r0 as field.private;");

        // Literal
        let output = Output::<CurrentNetwork>::parse("output 0u8 as u8.public;").unwrap().1;
        assert_eq!(format!("{output}"), "output 0u8 as u8.public;");

        // Struct
        let output = Output::<CurrentNetwork>::parse("output r1 as signature.private;").unwrap().1;
        assert_eq!(format!("{output}"), "output r1 as signature.private;");

        // Record
        let output = Output::<CurrentNetwork>::parse("output r2 as token.record;").unwrap().1;
        assert_eq!(format!("{output}"), "output r2 as token.record;");
    }
}
