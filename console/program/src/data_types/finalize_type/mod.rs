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
mod serialize;

use crate::{Identifier, Locator, PlaintextType};
use snarkvm_console_network::prelude::*;

use enum_index::EnumIndex;

#[derive(Copy, Clone, PartialEq, Eq, Hash, EnumIndex)]
pub enum FinalizeType<N: Network> {
    /// A publicly-visible type.
    Public(PlaintextType<N>),
    /// A record type inherits its visibility from the record definition.
    Record(Identifier<N>),
    /// An external record type inherits its visibility from its record definition.
    ExternalRecord(Locator<N>),
}
