// Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.
//
// Serkr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Serkr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Serkr. If not, see <http://www.gnu.org/licenses/>.
//

/// Contains the result of a proof attempt.
#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(missing_docs)]
pub enum ProofResult {
    Theorem,
    CounterSatisfiable,
    Unsatisfiable,
    Satisfiable,
    Timeout,
    Error(String),
}

impl ProofResult {
    /// If the problem has conjectures, we should report Theorem instead of Unsatisfiable.
    pub fn new_refutation(contains_conjectures: bool) -> ProofResult {
        if contains_conjectures {
            ProofResult::Theorem
        } else {
            ProofResult::Unsatisfiable
        }
    }

    /// If the problem has conjectures, we should report CounterSatisfiable instead of Satisfiable.
    pub fn new_saturation(contains_conjectures: bool) -> ProofResult {
        if contains_conjectures {
            ProofResult::CounterSatisfiable
        } else {
            ProofResult::Satisfiable
        }
    }
}

#[cfg(test)]
mod test {}
