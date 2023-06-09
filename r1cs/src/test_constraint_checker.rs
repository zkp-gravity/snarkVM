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

use crate::{errors::SynthesisError, ConstraintSystem, Index, LinearCombination, LookupTable, Variable};
use snarkvm_fields::Field;

/// Constraint system for testing purposes.
pub struct TestConstraintChecker<F: Field> {
    // the list of currently applicable input variables
    public_variables: Vec<F>,
    // the list of currently applicable auxiliary variables
    private_variables: Vec<F>,
    // the currently applicable lookup table
    lookup_table: Option<LookupTable<F>>,
    // whether or not unsatisfactory constraint has been found
    found_unsatisfactory_constraint: bool,
    // number of constraints
    num_constraints: usize,
    // constraint path segments in the stack
    segments: Vec<String>,
    // the first unsatisfied constraint
    first_unsatisfied_constraint: Option<String>,
}

impl<F: Field> Default for TestConstraintChecker<F> {
    fn default() -> Self {
        Self {
            public_variables: vec![F::one()],
            private_variables: vec![],
            lookup_table: None,
            found_unsatisfactory_constraint: false,
            num_constraints: 0,
            segments: vec![],
            first_unsatisfied_constraint: None,
        }
    }
}

impl<F: Field> TestConstraintChecker<F> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn which_is_unsatisfied(&self) -> Option<String> {
        self.first_unsatisfied_constraint.clone()
    }

    pub fn eval_lc(&self, lc: &LinearCombination<F>) -> F {
        lc.0.iter()
            .map(|(var, coeff)| {
                let value = match var.get_unchecked() {
                    Index::Public(index) => self.public_variables[index],
                    Index::Private(index) => self.private_variables[index],
                };
                value * coeff
            })
            .sum::<F>()
    }

    #[inline]
    pub fn is_satisfied(&self) -> bool {
        !self.found_unsatisfactory_constraint
    }

    #[inline]
    pub fn num_constraints(&self) -> usize {
        self.num_constraints
    }

    #[inline]
    pub fn public_inputs(&self) -> Vec<F> {
        self.public_variables[1..].to_vec()
    }
}

impl<F: Field> ConstraintSystem<F> for TestConstraintChecker<F> {
    type Root = Self;

    fn add_lookup_table(&mut self, lookup_table: LookupTable<F>) {
        self.lookup_table = Some(lookup_table);
    }

    fn alloc<Fn, A, AR>(&mut self, _annotation: A, f: Fn) -> Result<Variable, SynthesisError>
    where
        Fn: FnOnce() -> Result<F, SynthesisError>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        let index = self.private_variables.len();
        self.private_variables.push(f()?);
        let var = Variable::new_unchecked(Index::Private(index));

        Ok(var)
    }

    fn alloc_input<Fn, A, AR>(&mut self, _annotation: A, f: Fn) -> Result<Variable, SynthesisError>
    where
        Fn: FnOnce() -> Result<F, SynthesisError>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        let index = self.public_variables.len();
        self.public_variables.push(f()?);
        let var = Variable::new_unchecked(Index::Public(index));

        Ok(var)
    }

    fn enforce<A, AR, LA, LB, LC>(&mut self, annotation: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LB: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LC: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
    {
        self.num_constraints += 1;

        let a = self.eval_lc(&a(LinearCombination::zero()));
        let b = self.eval_lc(&b(LinearCombination::zero()));
        let c = self.eval_lc(&c(LinearCombination::zero()));

        if a * b != c && self.first_unsatisfied_constraint.is_none() {
            self.found_unsatisfactory_constraint = true;

            let new = annotation().as_ref().to_string();
            assert!(!new.contains('/'), "'/' is not allowed in names");

            let mut path = self.segments.clone();
            path.push(new);
            self.first_unsatisfied_constraint = Some(path.join("/"));
        }
    }

    fn enforce_lookup<A, AR, LA, LB, LC>(
        &mut self,
        _: A,
        a: LA,
        b: LB,
        c: LC,
        _table_index: usize,
    ) -> Result<(), SynthesisError>
    where
        A: FnOnce() -> AR,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LB: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LC: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
    {
        let a = self.eval_lc(&a(LinearCombination::zero()));
        let b = self.eval_lc(&b(LinearCombination::zero()));
        let c = self.eval_lc(&c(LinearCombination::zero()));

        let res = if let Some(lookup_table) = &self.lookup_table {
            *lookup_table.lookup(&[a, b]).ok_or(SynthesisError::LookupValueMissing)?
        } else {
            if self.first_unsatisfied_constraint.is_none() {
                self.found_unsatisfactory_constraint = true;
                self.first_unsatisfied_constraint = Some("lookup".to_string());
            }
            return Err(SynthesisError::LookupTableMissing);
        };

        if c == res {
            self.num_constraints += 1;
            Ok(())
        } else {
            Err(SynthesisError::LookupValueMissing)
        }
    }

    fn push_namespace<NR: AsRef<str>, N: FnOnce() -> NR>(&mut self, name_fn: N) {
        let new = name_fn().as_ref().to_string();
        assert!(!new.contains('/'), "'/' is not allowed in names");

        self.segments.push(new)
    }

    fn pop_namespace(&mut self) {
        self.segments.pop();
    }

    #[inline]
    fn get_root(&mut self) -> &mut Self::Root {
        self
    }

    #[inline]
    fn num_constraints(&self) -> usize {
        self.num_constraints()
    }

    #[inline]
    fn num_public_variables(&self) -> usize {
        self.public_variables.len()
    }

    #[inline]
    fn num_private_variables(&self) -> usize {
        self.private_variables.len()
    }

    #[inline]
    fn is_in_setup_mode(&self) -> bool {
        false
    }
}
