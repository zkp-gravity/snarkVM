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

#[macro_use]
extern crate criterion;

use snarkvm_algorithms::{
    crypto_hash::PoseidonSponge,
    snark::marlin::{ahp::AHPForR1CS, CircuitVerifyingKey, MarlinHidingMode, MarlinSNARK},
    AlgebraicSponge,
    SNARK,
};
use snarkvm_curves::bls12_377::{Bls12_377, Fq, Fr};
use snarkvm_fields::{Field, One, Zero};
use snarkvm_r1cs::{errors::SynthesisError, ConstraintSynthesizer, ConstraintSystem, LinearCombination, LookupTable};
use snarkvm_utilities::{ops::MulAssign, CanonicalDeserialize, CanonicalSerialize, TestRng, Uniform};

use criterion::Criterion;
use rand::{self, thread_rng, Rng};

use std::ops::{AddAssign, SubAssign};

type MarlinInst = MarlinSNARK<Bls12_377, FS, MarlinHidingMode>;
type FS = PoseidonSponge<Fq, 2, 1>;

#[derive(Copy, Clone)]
pub struct Benchmark<F: Field> {
    pub a: Option<F>,
    pub b: Option<F>,
    pub num_constraints: usize,
    pub num_variables: usize,
}

#[derive(Copy, Clone)]
pub struct BenchmarkXOR<F: Field> {
    pub x: Option<F>,
    pub y: Option<F>,
    pub z: Option<F>,
    pub one: Option<F>,
    pub sym_1: Option<F>,
    pub sym_2: Option<F>,
    pub sym_3: Option<F>,
    pub num_xors: usize,
    pub num_variables: usize,
}

#[derive(Clone)]
pub struct BenchmarkWithLookup<F: Field> {
    pub x: Option<F>,
    pub y: Option<F>,
    pub z: Option<F>,
    pub num_xors: usize,
    pub num_variables: usize,
    pub tables: Vec<LookupTable<F>>,
}

impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for Benchmark<ConstraintF> {
    fn generate_constraints<CS: ConstraintSystem<ConstraintF>>(&self, cs: &mut CS) -> Result<(), SynthesisError> {
        let a = cs.alloc(|| "a", || self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = cs.alloc(|| "b", || self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let c = cs.alloc_input(
            || "c",
            || {
                let mut a = self.a.ok_or(SynthesisError::AssignmentMissing)?;
                let b = self.b.ok_or(SynthesisError::AssignmentMissing)?;

                a.mul_assign(&b);
                Ok(a)
            },
        )?;

        for i in 0..(self.num_variables - 3) {
            let _ = cs.alloc(|| format!("var {i}"), || self.a.ok_or(SynthesisError::AssignmentMissing))?;
        }

        for i in 0..(self.num_constraints - 1) {
            cs.enforce(|| format!("constraint {i}"), |lc| lc + a, |lc| lc + b, |lc| lc + c);
        }

        Ok(())
    }
}

impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for BenchmarkXOR<ConstraintF> {
    // We will prove the operation `a ^ b = c` to R1CS constraints
    // We assume that all variables are bits, either 1 or
    // First, we break up the XOR operation `x + y - 2*x*y = z` into smaller pieces:
    //      sym_1 = x + y
    //      sym_2 = 2*x
    //      sym_3 = sym_2*y
    //      z = sym_1 - sym_3
    // variables:1 x y z sym_1 sym_2 sym_3
    // Gate 1 A:   1 1
    // Gate 1 B: 1
    // Gate 1 C:         1
    // Gate 2 A: 2
    // Gate 2 B:   1
    // Gate 2 C:               1
    // Gate 3 A:               1
    // Gate 3 B:     1
    // Gate 3 C:                    1
    // Gate 4 A:         1
    // Gate 4 B: 1
    // Gate 4 C:       1            1
    fn generate_constraints<CS: ConstraintSystem<ConstraintF>>(&self, cs: &mut CS) -> Result<(), SynthesisError> {
        let x = cs.alloc_input(|| "x", || self.x.ok_or(SynthesisError::AssignmentMissing))?;
        let y = cs.alloc_input(|| "y", || self.y.ok_or(SynthesisError::AssignmentMissing))?;
        let z = cs.alloc_input(|| "z", || self.z.ok_or(SynthesisError::AssignmentMissing))?;
        let one = cs.alloc(|| "one", || self.one.ok_or(SynthesisError::AssignmentMissing))?;
        let sym_1 = cs.alloc(|| "sym_1", || self.sym_1.ok_or(SynthesisError::AssignmentMissing))?;
        let sym_2 = cs.alloc(|| "sym_2", || self.sym_2.ok_or(SynthesisError::AssignmentMissing))?;
        let sym_3 = cs.alloc(|| "sym_3", || self.sym_3.ok_or(SynthesisError::AssignmentMissing))?;

        for i in 0..(self.num_variables - 7) {
            let _ = cs.alloc(|| format!("var {}", i), || self.x.ok_or(SynthesisError::AssignmentMissing))?;
        }

        for i in 0..(self.num_xors) {
            cs.enforce(|| format!("constraint gate 1 {}", i), |lc| lc + x + y, |lc| lc + one, |lc| lc + sym_1);
            cs.enforce(|| format!("constraint gate 2 {}", i), |lc| lc + one + one, |lc| lc + x, |lc| lc + sym_2);
            cs.enforce(|| format!("constraint gate 3 {}", i), |lc| lc + sym_2, |lc| lc + y, |lc| lc + sym_3);
            cs.enforce(|| format!("constraint gate 4 {}", i), |lc| lc + sym_1, |lc| lc + one, |lc| lc + z + sym_3);
        }

        Ok(())
    }
}

impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for BenchmarkWithLookup<ConstraintF> {
    fn generate_constraints<C: ConstraintSystem<ConstraintF>>(&self, cs: &mut C) -> Result<(), SynthesisError> {
        for table in &self.tables {
            cs.add_lookup_table(table.clone());
        }
        let x = cs.alloc_input(|| "x", || self.x.ok_or(SynthesisError::AssignmentMissing))?;
        let y = cs.alloc_input(|| "y", || self.y.ok_or(SynthesisError::AssignmentMissing))?;
        let z = cs.alloc_input(|| "z", || self.z.ok_or(SynthesisError::AssignmentMissing))?;
        for i in 0..self.num_xors {
            let mut table_index = 0;
            // this is a very silly test and assumes we have two equal tables
            if i % 2 == 0 {
                table_index = 0;
            }
            cs.enforce_lookup(
                || format!("c_lookup {}", i),
                |lc| lc + LinearCombination::from(x),
                |lc| lc + LinearCombination::from(y),
                |lc| lc + LinearCombination::from(z),
                table_index,
            )?;
        }

        for i in 0..(self.num_variables - 3) {
            let _ = cs.alloc(|| format!("var {}", i), || self.x.ok_or(SynthesisError::AssignmentMissing))?;
        }

        Ok(())
    }
}

fn snark_universal_setup(c: &mut Criterion) {
    let max_degree = AHPForR1CS::<Fr, MarlinHidingMode>::max_degree(1000000, 1000000, 1000000).unwrap();

    c.bench_function("snark_universal_setup", move |b| {
        b.iter(|| {
            MarlinInst::universal_setup(&max_degree).unwrap();
        })
    });
}

fn snark_circuit_setup(c: &mut Criterion) {
    let rng = &mut TestRng::default();

    let x = Fr::rand(rng);
    let y = Fr::rand(rng);

    let max_degree = AHPForR1CS::<Fr, MarlinHidingMode>::max_degree(100000, 100000, 100000).unwrap();
    let universal_srs = MarlinInst::universal_setup(&max_degree).unwrap();

    for size in [100, 1_000, 10_000] {
        let num_constraints = size;
        let num_variables = size;
        let circuit = Benchmark::<Fr> { a: Some(x), b: Some(y), num_constraints, num_variables };
        c.bench_function(&format!("snark_circuit_setup_{size}"), |b| {
            b.iter(|| MarlinInst::circuit_setup(&universal_srs, &circuit).unwrap())
        });
    }
}

fn snark_prove(c: &mut Criterion) {
    c.bench_function("snark_prove", move |b| {
        let num_constraints = 100;
        let num_variables = 100;
        let rng = &mut TestRng::default();

        let x = Fr::rand(rng);
        let y = Fr::rand(rng);
        let mut z = x;
        z.mul_assign(&y);

        let max_degree = AHPForR1CS::<Fr, MarlinHidingMode>::max_degree(1000, 1000, 1000).unwrap();
        let universal_srs = MarlinInst::universal_setup(&max_degree).unwrap();
        let fs_parameters = FS::sample_parameters();

        let circuit = Benchmark::<Fr> { a: Some(x), b: Some(y), num_constraints, num_variables };
        let params = MarlinInst::circuit_setup(&universal_srs, &circuit).unwrap();
        b.iter(|| MarlinInst::prove(&fs_parameters, &params.0, &circuit, rng).unwrap())
    });
}

fn snark_xor_prove(c: &mut Criterion) {
    c.bench_function("snark_xor_prove", move |b| {
        let num_xors = 10000;
        let num_variables = 100;
        let rng = &mut thread_rng();

        // We will prove the operation `a ^ b = c` to R1CS constraints
        // We assume that all variables are bits, either 1 or
        // First, we break up the XOR operation `x + y - 2*x*y = z` into smaller pieces:
        //      sym_1 = x + y
        //      sym_2 = 2*x
        //      sym_3 = sym_2*y
        //      z = sym_1 - sym_3
        let one = Fr::one();
        let zero = Fr::zero();
        let x = if rng.gen::<bool>() { one } else { zero };
        let y = if rng.gen::<bool>() { one } else { zero };
        let mut sym_1 = x;
        sym_1.add_assign(&y);
        let mut sym_2 = x;
        sym_2.mul_assign(&one.double());
        let mut sym_3 = sym_2;
        sym_3.mul_assign(&y);
        let mut z = sym_1;
        z.sub_assign(&sym_3);

        let max_degree = AHPForR1CS::<Fr, MarlinHidingMode>::max_degree(100000, 100000, 100000).unwrap();
        let universal_srs = MarlinInst::universal_setup(&max_degree).unwrap();
        let fs_parameters = FS::sample_parameters();

        let circuit = BenchmarkXOR::<Fr> {
            x: Some(x),
            y: Some(y),
            z: Some(z),
            one: Some(one),
            sym_1: Some(sym_1),
            sym_2: Some(sym_2),
            sym_3: Some(sym_3),
            num_xors,
            num_variables,
        };

        let params = MarlinInst::circuit_setup(&universal_srs, &circuit).unwrap();

        b.iter(|| MarlinInst::prove(&fs_parameters, &params.0, &circuit, rng).unwrap())
    });
}

fn snark_lookup_prove(c: &mut Criterion) {
    c.bench_function("snark_lookup_prove", move |b| {
        let num_xors = 10000;
        let num_variables = 100;
        let rng = &mut thread_rng();

        // We will prove the operation `a ^ b = c` to R1CS constraints
        // We assume that all variables are bits, either 1 or
        // First, we break up the XOR operation `x + y - 2*x*y = z` into smaller pieces:
        //      sym_1 = x + y
        //      sym_2 = 2*x
        //      sym_3 = sym_2*y
        //      z = sym_1 - sym_3
        let one = Fr::one();
        let zero = Fr::zero();
        let x = if rng.gen::<bool>() { one } else { zero };
        let y = if rng.gen::<bool>() { one } else { zero };
        let mut sym_1 = x;
        sym_1.add_assign(&y);
        let mut sym_2 = x;
        sym_2.mul_assign(&one.double());
        let mut sym_3 = sym_2;
        sym_3.mul_assign(&y);
        let mut z = sym_1;
        z.sub_assign(&sym_3);

        let mut tables = vec![];
        let num_tables = 2;
        for _ in 0..num_tables {
            let mut table = LookupTable::default();
            let lookup_value = [x, y];
            table.fill(lookup_value, z);
            tables.push(table);
        }

        let max_degree = AHPForR1CS::<Fr, MarlinHidingMode>::max_degree(100000, 100000, 100000).unwrap();
        let universal_srs = MarlinInst::universal_setup(&max_degree).unwrap();
        let fs_parameters = FS::sample_parameters();

        let circuit = BenchmarkWithLookup::<Fr> { x: Some(x), y: Some(y), z: Some(z), num_xors, num_variables, tables };

        let params = MarlinInst::circuit_setup(&universal_srs, &circuit).unwrap();

        b.iter(|| MarlinInst::prove(&fs_parameters, &params.0, &circuit, rng).unwrap())
    });
}

fn snark_verify(c: &mut Criterion) {
    c.bench_function("snark_verify", move |b| {
        let num_constraints = 100;
        let num_variables = 25;
        let rng = &mut TestRng::default();

        let x = Fr::rand(rng);
        let y = Fr::rand(rng);
        let mut z = x;
        z.mul_assign(&y);

        let max_degree = AHPForR1CS::<Fr, MarlinHidingMode>::max_degree(100, 100, 100).unwrap();
        let universal_srs = MarlinInst::universal_setup(&max_degree).unwrap();
        let fs_parameters = FS::sample_parameters();

        let circuit = Benchmark::<Fr> { a: Some(x), b: Some(y), num_constraints, num_variables };

        let (pk, vk) = MarlinInst::circuit_setup(&universal_srs, &circuit).unwrap();

        let proof = MarlinInst::prove(&fs_parameters, &pk, &circuit, rng).unwrap();
        b.iter(|| {
            let verification = MarlinInst::verify(&fs_parameters, &vk, [z], &proof).unwrap();
            assert!(verification);
        })
    });
}

fn snark_vk_serialize(c: &mut Criterion) {
    use snarkvm_utilities::serialize::Compress;
    let mut group = c.benchmark_group("snark_vk_serialize");
    for mode in [Compress::Yes, Compress::No] {
        let name = match mode {
            Compress::No => "uncompressed",
            Compress::Yes => "compressed",
        };
        let num_constraints = 100;
        let num_variables = 25;
        let rng = &mut TestRng::default();

        let x = Fr::rand(rng);
        let y = Fr::rand(rng);
        let mut z = x;
        z.mul_assign(&y);

        let max_degree = AHPForR1CS::<Fr, MarlinHidingMode>::max_degree(100, 100, 100).unwrap();
        let universal_srs = MarlinInst::universal_setup(&max_degree).unwrap();
        let circuit = Benchmark::<Fr> { a: Some(x), b: Some(y), num_constraints, num_variables };

        let (_, vk) = MarlinInst::circuit_setup(&universal_srs, &circuit).unwrap();
        let mut bytes = Vec::with_capacity(10000);
        group.bench_function(name, |b| {
            b.iter(|| {
                vk.serialize_with_mode(&mut bytes, mode).unwrap();
                bytes.clear()
            })
        });
    }
    group.finish();
}

fn snark_vk_deserialize(c: &mut Criterion) {
    use snarkvm_utilities::serialize::{Compress, Validate};
    let mut group = c.benchmark_group("snark_vk_deserialize");
    for compress in [Compress::Yes, Compress::No] {
        let compress_name = match compress {
            Compress::No => "uncompressed",
            Compress::Yes => "compressed",
        };
        for validate in [Validate::Yes, Validate::No] {
            let validate_name = match validate {
                Validate::No => "unchecked",
                Validate::Yes => "checked",
            };
            let name = format!("{compress_name}_{validate_name}");
            let num_constraints = 100;
            let num_variables = 25;
            let rng = &mut TestRng::default();

            let x = Fr::rand(rng);
            let y = Fr::rand(rng);
            let mut z = x;
            z.mul_assign(&y);

            let max_degree = AHPForR1CS::<Fr, MarlinHidingMode>::max_degree(100, 100, 100).unwrap();
            let universal_srs = MarlinInst::universal_setup(&max_degree).unwrap();
            let circuit = Benchmark::<Fr> { a: Some(x), b: Some(y), num_constraints, num_variables };

            let (_, vk) = MarlinInst::circuit_setup(&universal_srs, &circuit).unwrap();
            let mut bytes = Vec::with_capacity(10000);
            vk.serialize_with_mode(&mut bytes, compress).unwrap();
            group.bench_function(name, |b| {
                b.iter(|| {
                    let _vk = CircuitVerifyingKey::<Bls12_377, MarlinHidingMode>::deserialize_with_mode(
                        &*bytes, compress, validate,
                    )
                    .unwrap();
                })
            });
        }
    }
    group.finish();
}

fn snark_certificate_prove(c: &mut Criterion) {
    let rng = &mut TestRng::default();

    let x = Fr::rand(rng);
    let y = Fr::rand(rng);

    let max_degree = AHPForR1CS::<Fr, MarlinHidingMode>::max_degree(100000, 100000, 100000).unwrap();
    let universal_srs = MarlinInst::universal_setup(&max_degree).unwrap();
    let fs_parameters = FS::sample_parameters();
    let fs_p = &fs_parameters;

    for size in [100, 1_000, 10_000, 100_000] {
        c.bench_function(&format!("snark_certificate_prove_{size}"), |b| {
            let num_constraints = size;
            let num_variables = size;
            let circuit = Benchmark::<Fr> { a: Some(x), b: Some(y), num_constraints, num_variables };
            let (pk, vk) = MarlinInst::circuit_setup(&universal_srs, &circuit).unwrap();

            b.iter(|| MarlinInst::prove_vk(fs_p, &vk, &pk).unwrap())
        });
    }
}

fn snark_certificate_verify(c: &mut Criterion) {
    let rng = &mut TestRng::default();

    let x = Fr::rand(rng);
    let y = Fr::rand(rng);

    let max_degree = AHPForR1CS::<Fr, MarlinHidingMode>::max_degree(100_000, 100_000, 100_000).unwrap();
    let universal_srs = MarlinInst::universal_setup(&max_degree).unwrap();
    let fs_parameters = FS::sample_parameters();
    let fs_p = &fs_parameters;

    for size in [100, 1_000, 10_000, 100_000] {
        c.bench_function(&format!("snark_certificate_verify_{size}"), |b| {
            let num_constraints = size;
            let num_variables = size;
            let circuit = Benchmark::<Fr> { a: Some(x), b: Some(y), num_constraints, num_variables };
            let (pk, vk) = MarlinInst::circuit_setup(&universal_srs, &circuit).unwrap();
            let certificate = MarlinInst::prove_vk(fs_p, &vk, &pk).unwrap();

            b.iter(|| MarlinInst::verify_vk(fs_p, &circuit, &vk, &certificate).unwrap())
        });
    }
}

criterion_group! {
    name = marlin_snark;
    config = Criterion::default().sample_size(10);
    //targets = snark_universal_setup, snark_circuit_setup, snark_prove, snark_verify, snark_vk_serialize, snark_vk_deserialize, snark_certificate_prove, snark_certificate_verify,
    targets = snark_xor_prove, snark_lookup_prove
}

criterion_main!(marlin_snark);
