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

use crate::{polycommit::sonic_pc, snark::marlin::ahp, SNARKError};

use snarkvm_curves::PairingEngine;
use snarkvm_fields::PrimeField;
use snarkvm_utilities::{
    error,
    io::{self, Read, Write},
    serialize::*,
    FromBytes,
    ToBytes,
};

#[derive(Clone, Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct Commitments<E: PairingEngine> {
    pub witness_commitments: Vec<WitnessCommitments<E>>,
    /// Commitment to the masking polynomial.
    pub mask_poly: Option<sonic_pc::Commitment<E>>,
    /// Commitments to plookup-related polynomials.
    pub lookup_commitments: Vec<LookupCommitments<E>>,
    /// Commitment to the lookup table polynomial.
    pub table: sonic_pc::Commitment<E>,
    /// Commitment to the shifted lookup table polynomial, multiplied by delta.
    pub delta_table_omega: sonic_pc::Commitment<E>,
    /// Commitment to the `g_1` polynomial.
    pub g_1: sonic_pc::Commitment<E>,
    /// Commitment to the `h_1` polynomial.
    pub h_1: sonic_pc::Commitment<E>,
    /// Commitment to the `g_a` polynomial.
    pub g_a: sonic_pc::Commitment<E>,
    /// Commitment to the `g_b` polynomial.
    pub g_b: sonic_pc::Commitment<E>,
    /// Commitment to the `g_c` polynomial.
    pub g_c: sonic_pc::Commitment<E>,
    /// Commitment to the `h_2` polynomial.
    pub h_2: sonic_pc::Commitment<E>,
}

impl<E: PairingEngine> Commitments<E> {
    fn serialize_with_mode<W: snarkvm_utilities::Write>(
        &self,
        mut writer: W,
        compress: Compress,
    ) -> Result<(), snarkvm_utilities::SerializationError> {
        for comm in &self.witness_commitments {
            comm.serialize_with_mode(&mut writer, compress)?;
        }
        for comm in &self.lookup_commitments {
            comm.serialize_with_mode(&mut writer, compress)?;
        }
        CanonicalSerialize::serialize_with_mode(&self.mask_poly, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.table, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.delta_table_omega, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.g_1, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.h_1, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.g_a, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.g_b, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.g_c, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.h_2, &mut writer, compress)?;
        Ok(())
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        let mut size = 0;
        size += self.witness_commitments.len()
            * CanonicalSerialize::serialized_size(&self.witness_commitments[0], compress);
        size +=
            self.lookup_commitments.len() * CanonicalSerialize::serialized_size(&self.lookup_commitments[0], compress);
        size += CanonicalSerialize::serialized_size(&self.mask_poly, compress);
        size += CanonicalSerialize::serialized_size(&self.table, compress);
        size += CanonicalSerialize::serialized_size(&self.delta_table_omega, compress);
        size += CanonicalSerialize::serialized_size(&self.g_1, compress);
        size += CanonicalSerialize::serialized_size(&self.h_1, compress);
        size += CanonicalSerialize::serialized_size(&self.g_a, compress);
        size += CanonicalSerialize::serialized_size(&self.g_b, compress);
        size += CanonicalSerialize::serialized_size(&self.g_c, compress);
        size += CanonicalSerialize::serialized_size(&self.h_2, compress);
        size
    }

    fn deserialize_with_mode<R: snarkvm_utilities::Read>(
        batch_size: usize,
        mut reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, snarkvm_utilities::SerializationError> {
        let mut witness_commitments = Vec::new();
        for _ in 0..batch_size {
            witness_commitments.push(CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?);
        }
        let mut lookup_commitments = Vec::new();
        for _ in 0..batch_size {
            lookup_commitments.push(CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?);
        }
        Ok(Commitments {
            witness_commitments,
            mask_poly: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            lookup_commitments,
            table: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            delta_table_omega: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            g_1: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            h_1: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            g_a: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            g_b: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            g_c: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            h_2: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
        })
    }
}
/// Commitments to the `w`, `z_a`, `z_b` and `z_c` polynomials.
#[derive(Clone, Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct WitnessCommitments<E: PairingEngine> {
    /// Commitment to the `w` polynomial.
    pub w: sonic_pc::Commitment<E>,
    /// Commitment to the `z_a` polynomial.
    pub z_a: sonic_pc::Commitment<E>,
    /// Commitment to the `z_b` polynomial.
    pub z_b: sonic_pc::Commitment<E>,
    /// Commitment to the `z_c` polynomial.
    pub z_c: sonic_pc::Commitment<E>,
}

/// Commitments to the `f`, `s_1`, `s_2`, `z_2`, `delta_s_1_omega` and `z_2_omega` polynomials.
#[derive(Clone, Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct LookupCommitments<E: PairingEngine> {
    /// Commitment to the `f` polynomial.
    pub f: sonic_pc::Commitment<E>,
    /// Commitment to the `s_1` polynomial.
    pub s_1: sonic_pc::Commitment<E>,
    /// Commitment to the `s_2` polynomial.
    pub s_2: sonic_pc::Commitment<E>,
    /// Commitment to the `z_2` polynomial.
    pub z_2: sonic_pc::Commitment<E>,
    /// Commitment to the `delta_s_1_omega` polynomial.
    pub delta_s_1_omega: sonic_pc::Commitment<E>,
    /// Commitment to the `z_2_omega` polynomial.
    pub z_2_omega: sonic_pc::Commitment<E>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Evaluations<F: PrimeField> {
    /// Evaluation of `z_b_i`'s at `beta`.
    pub z_b_evals: Vec<F>,
    /// Evaluation of `f_i`'s at `beta`.
    pub f_evals: Vec<F>,
    /// Evaluation of `s_1_i`'s at `beta`.
    pub s_1_evals: Vec<F>,
    /// Evaluation of `s_2_i`'s at `beta`.
    pub s_2_evals: Vec<F>,
    /// Evaluation of `z_2_i`'s at `beta`.
    pub z_2_evals: Vec<F>,
    /// Evaluation of `delta_s_1_omega_i`'s at `beta`.
    pub delta_s_1_omega_evals: Vec<F>,
    /// Evaluation of `s_m` at `beta`.
    pub s_m_eval: F,
    /// Evaluation of `s_l` at `beta`.
    pub s_l_eval: F,
    /// Evaluation of `table` at `beta`.
    pub table_eval: F,
    /// Evaluation of `delta_table_omega` at `beta`.
    pub delta_table_omega_eval: F,
    /// Evaluation of `g_1` at `beta`.
    pub g_1_eval: F,
    /// Evaluation of `g_a` at `beta`.
    pub g_a_eval: F,
    /// Evaluation of `g_b` at `gamma`.
    pub g_b_eval: F,
    /// Evaluation of `g_c` at `gamma`.
    pub g_c_eval: F,
}

impl<F: PrimeField> Evaluations<F> {
    fn serialize_with_mode<W: snarkvm_utilities::Write>(
        &self,
        mut writer: W,
        compress: Compress,
    ) -> Result<(), snarkvm_utilities::SerializationError> {
        for z_b_eval in &self.z_b_evals {
            CanonicalSerialize::serialize_with_mode(z_b_eval, &mut writer, compress)?;
        }
        for f_eval in &self.f_evals {
            CanonicalSerialize::serialize_with_mode(f_eval, &mut writer, compress)?;
        }
        for s_1_eval in &self.s_1_evals {
            CanonicalSerialize::serialize_with_mode(s_1_eval, &mut writer, compress)?;
        }
        for s_2_eval in &self.s_2_evals {
            CanonicalSerialize::serialize_with_mode(s_2_eval, &mut writer, compress)?;
        }
        for z_2_eval in &self.z_2_evals {
            CanonicalSerialize::serialize_with_mode(z_2_eval, &mut writer, compress)?;
        }
        for delta_s_1_omega_eval in &self.delta_s_1_omega_evals {
            CanonicalSerialize::serialize_with_mode(delta_s_1_omega_eval, &mut writer, compress)?;
        }
        CanonicalSerialize::serialize_with_mode(&self.s_m_eval, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.s_l_eval, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.table_eval, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.delta_table_omega_eval, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.g_1_eval, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.g_a_eval, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.g_b_eval, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.g_c_eval, &mut writer, compress)?;
        Ok(())
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        let mut size = 0;
        size += self.z_b_evals.iter().map(|s| s.serialized_size(compress)).sum::<usize>();
        size += self.f_evals.iter().map(|s| s.serialized_size(compress)).sum::<usize>();
        size += self.s_1_evals.iter().map(|s| s.serialized_size(compress)).sum::<usize>();
        size += self.s_2_evals.iter().map(|s| s.serialized_size(compress)).sum::<usize>();
        size += self.z_2_evals.iter().map(|s| s.serialized_size(compress)).sum::<usize>();
        size += self.delta_s_1_omega_evals.iter().map(|s| s.serialized_size(compress)).sum::<usize>();
        size += CanonicalSerialize::serialized_size(&self.s_m_eval, compress);
        size += CanonicalSerialize::serialized_size(&self.s_l_eval, compress);
        size += CanonicalSerialize::serialized_size(&self.table_eval, compress);
        size += CanonicalSerialize::serialized_size(&self.delta_table_omega_eval, compress);
        size += CanonicalSerialize::serialized_size(&self.g_1_eval, compress);
        size += CanonicalSerialize::serialized_size(&self.g_a_eval, compress);
        size += CanonicalSerialize::serialized_size(&self.g_b_eval, compress);
        size += CanonicalSerialize::serialized_size(&self.g_c_eval, compress);
        size
    }

    fn deserialize_with_mode<R: snarkvm_utilities::Read>(
        batch_size: usize,
        mut reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, snarkvm_utilities::SerializationError> {
        let mut z_b_evals = Vec::with_capacity(batch_size);
        for _ in 0..batch_size {
            z_b_evals.push(CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?);
        }
        let mut f_evals = Vec::with_capacity(batch_size);
        for _ in 0..batch_size {
            f_evals.push(CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?);
        }
        let mut s_1_evals = Vec::with_capacity(batch_size);
        for _ in 0..batch_size {
            s_1_evals.push(CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?);
        }
        let mut s_2_evals = Vec::with_capacity(batch_size);
        for _ in 0..batch_size {
            s_2_evals.push(CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?);
        }
        let mut z_2_evals = Vec::with_capacity(batch_size);
        for _ in 0..batch_size {
            z_2_evals.push(CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?);
        }
        let mut delta_s_1_omega_evals = Vec::with_capacity(batch_size);
        for _ in 0..batch_size {
            delta_s_1_omega_evals.push(CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?);
        }
        Ok(Evaluations {
            z_b_evals,
            f_evals,
            s_1_evals,
            s_2_evals,
            z_2_evals,
            delta_s_1_omega_evals,
            s_m_eval: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            s_l_eval: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            table_eval: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            delta_table_omega_eval: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            g_1_eval: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            g_a_eval: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            g_b_eval: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            g_c_eval: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
        })
    }
}

impl<F: PrimeField> Evaluations<F> {
    pub(crate) fn from_map(map: &std::collections::BTreeMap<String, F>, batch_size: usize) -> Self {
        let z_b_evals = map.iter().filter_map(|(k, v)| k.starts_with("z_b_").then_some(*v)).collect::<Vec<_>>();
        let f_evals = map.iter().filter_map(|(k, v)| k.starts_with("f_").then(|| *v)).collect::<Vec<_>>();
        let s_1_evals = map.iter().filter_map(|(k, v)| k.starts_with("s_1_").then(|| *v)).collect::<Vec<_>>();
        let s_2_evals = map.iter().filter_map(|(k, v)| k.starts_with("s_2_").then(|| *v)).collect::<Vec<_>>();
        let z_2_evals = map.iter().filter_map(|(k, v)| k.starts_with("z_2_").then(|| *v)).collect::<Vec<_>>();
        let delta_s_1_omega_evals =
            map.iter().filter_map(|(k, v)| k.starts_with("delta_omega_s_1_").then(|| *v)).collect::<Vec<_>>();
        assert_eq!(z_b_evals.len(), batch_size);
        Self {
            z_b_evals,
            f_evals,
            s_1_evals,
            s_2_evals,
            z_2_evals,
            delta_s_1_omega_evals,
            s_m_eval: map["s_m"],
            s_l_eval: map["s_l"],
            table_eval: map["table"],
            delta_table_omega_eval: map["delta_table_omega"],
            g_1_eval: map["g_1"],
            g_a_eval: map["g_a"],
            g_b_eval: map["g_b"],
            g_c_eval: map["g_c"],
        }
    }

    pub(crate) fn get(&self, label: &str) -> Option<F> {
        if label.starts_with("z_b_") {
            let index = label.strip_prefix("z_b_").expect("should be able to strip identified prefix");
            self.z_b_evals.get(index.parse::<usize>().unwrap()).copied()
        } else if label.starts_with("f_") {
            let index = label.strip_prefix("f_").expect("should be able to strip identified prefix");
            self.f_evals.get(index.parse::<usize>().unwrap()).copied()
        } else if label.starts_with("s_1_") {
            let index = label.strip_prefix("s_1_").expect("should be able to strip identified prefix");
            self.s_1_evals.get(index.parse::<usize>().unwrap()).copied()
        } else if label.starts_with("s_2_") {
            let index = label.strip_prefix("s_2_").expect("should be able to strip identified prefix");
            self.s_2_evals.get(index.parse::<usize>().unwrap()).copied()
        } else if label.starts_with("z_2_") {
            let index = label.strip_prefix("z_2_").expect("should be able to strip identified prefix");
            self.z_2_evals.get(index.parse::<usize>().unwrap()).copied()
        } else if label.starts_with("delta_omega_s_1_") {
            let index = label.strip_prefix("delta_omega_s_1_").expect("should be able to strip identified prefix");
            self.delta_s_1_omega_evals.get(index.parse::<usize>().unwrap()).copied()
        } else {
            match label {
                "s_m" => Some(self.s_m_eval),
                "s_l" => Some(self.s_l_eval),
                "table" => Some(self.table_eval),
                "delta_table_omega" => Some(self.delta_table_omega_eval),
                "g_1" => Some(self.g_1_eval),
                "g_a" => Some(self.g_a_eval),
                "g_b" => Some(self.g_b_eval),
                "g_c" => Some(self.g_c_eval),
                _ => None,
            }
        }
    }
}

impl<F: PrimeField> Valid for Evaluations<F> {
    fn check(&self) -> Result<(), snarkvm_utilities::SerializationError> {
        self.z_b_evals.check()?;
        self.f_evals.check()?;
        self.s_1_evals.check()?;
        self.s_2_evals.check()?;
        self.z_2_evals.check()?;
        self.delta_s_1_omega_evals.check()?;
        self.s_m_eval.check()?;
        self.s_l_eval.check()?;
        self.table_eval.check()?;
        self.delta_table_omega_eval.check()?;
        self.g_1_eval.check()?;
        self.g_a_eval.check()?;
        self.g_b_eval.check()?;
        self.g_c_eval.check()
    }
}

impl<F: PrimeField> Evaluations<F> {
    pub fn to_field_elements(&self) -> Vec<F> {
        let mut result = self.z_b_evals.clone();
        result.extend(self.f_evals.iter());
        result.extend(self.s_1_evals.iter());
        result.extend(self.s_2_evals.iter());
        result.extend(self.z_2_evals.iter());
        result.extend(self.delta_s_1_omega_evals.iter());
        result.extend([
            self.s_m_eval,
            self.s_l_eval,
            self.table_eval,
            self.delta_table_omega_eval,
            self.g_1_eval,
            self.g_a_eval,
            self.g_b_eval,
            self.g_c_eval,
        ]);
        result
    }
}

/// A zkSNARK proof.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proof<E: PairingEngine> {
    /// The number of instances being proven in this proof.
    batch_size: usize,

    /// Commitments to prover polynomials.
    pub commitments: Commitments<E>,

    /// Evaluations of some of the committed polynomials.
    pub evaluations: Evaluations<E::Fr>,

    /// Prover message: sum_a, sum_b, sum_c
    pub msg: ahp::prover::FifthMessage<E::Fr>,

    /// An evaluation proof from the polynomial commitment.
    pub pc_proof: sonic_pc::BatchLCProof<E>,
}

impl<E: PairingEngine> Proof<E> {
    /// Construct a new proof.
    pub fn new(
        batch_size: usize,
        commitments: Commitments<E>,
        evaluations: Evaluations<E::Fr>,
        msg: ahp::prover::FifthMessage<E::Fr>,
        pc_proof: sonic_pc::BatchLCProof<E>,
    ) -> Result<Self, SNARKError> {
        if commitments.witness_commitments.len() != batch_size {
            return Err(SNARKError::BatchSizeMismatch);
        }
        if evaluations.z_b_evals.len() != batch_size {
            return Err(SNARKError::BatchSizeMismatch);
        }
        Ok(Self { batch_size, commitments, evaluations, msg, pc_proof })
    }

    pub fn batch_size(&self) -> Result<usize, SNARKError> {
        if self.commitments.witness_commitments.len() != self.batch_size {
            return Err(SNARKError::BatchSizeMismatch);
        }
        if self.evaluations.z_b_evals.len() != self.batch_size {
            return Err(SNARKError::BatchSizeMismatch);
        }
        Ok(self.batch_size)
    }
}

impl<E: PairingEngine> CanonicalSerialize for Proof<E> {
    fn serialize_with_mode<W: Write>(&self, mut writer: W, compress: Compress) -> Result<(), SerializationError> {
        CanonicalSerialize::serialize_with_mode(&self.batch_size, &mut writer, compress)?;
        Commitments::serialize_with_mode(&self.commitments, &mut writer, compress)?;
        Evaluations::serialize_with_mode(&self.evaluations, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.msg, &mut writer, compress)?;
        CanonicalSerialize::serialize_with_mode(&self.pc_proof, &mut writer, compress)?;
        Ok(())
    }

    fn serialized_size(&self, mode: Compress) -> usize {
        let mut size = 0;
        size += CanonicalSerialize::serialized_size(&self.batch_size, mode);
        size += Commitments::serialized_size(&self.commitments, mode);
        size += Evaluations::serialized_size(&self.evaluations, mode);
        size += CanonicalSerialize::serialized_size(&self.msg, mode);
        size += CanonicalSerialize::serialized_size(&self.pc_proof, mode);
        size
    }
}

impl<E: PairingEngine> Valid for Proof<E> {
    fn check(&self) -> Result<(), SerializationError> {
        self.batch_size.check()?;
        self.commitments.check()?;
        self.evaluations.check()?;
        self.msg.check()?;
        self.pc_proof.check()
    }
}

impl<E: PairingEngine> CanonicalDeserialize for Proof<E> {
    fn deserialize_with_mode<R: Read>(
        mut reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        let batch_size = CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?;
        Ok(Proof {
            batch_size,
            commitments: Commitments::deserialize_with_mode(batch_size, &mut reader, compress, validate)?,
            evaluations: Evaluations::deserialize_with_mode(batch_size, &mut reader, compress, validate)?,
            msg: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            pc_proof: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
        })
    }
}

impl<E: PairingEngine> ToBytes for Proof<E> {
    fn write_le<W: Write>(&self, mut w: W) -> io::Result<()> {
        Self::serialize_compressed(self, &mut w).map_err(|_| error("could not serialize Proof"))
    }
}

impl<E: PairingEngine> FromBytes for Proof<E> {
    fn read_le<R: Read>(mut r: R) -> io::Result<Self> {
        Self::deserialize_compressed(&mut r).map_err(|_| error("could not deserialize Proof"))
    }
}
