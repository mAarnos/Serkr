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
    GaveUp,
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
    
    /// Returns a String which describes the type of the ProofResult.
    /// The reason for this is function is trouble with SZS format output.
    pub fn display_type(&self) -> String {
        match *self {
            ProofResult::Theorem => "Theorem".to_owned(),
            ProofResult::CounterSatisfiable => "CounterSatisfiable".to_owned(),
            ProofResult::Unsatisfiable => "Unsatisfiable".to_owned(),
            ProofResult::Satisfiable => "Satisfiable".to_owned(),
            ProofResult::GaveUp => "GaveUp".to_owned(),
            ProofResult::Timeout => "Timeout".to_owned(),
            ProofResult::Error(_) => "Error".to_owned()
        }
    }
    
    /// Is the current result successful (in the sense that a proof of some kind was found)?
    pub fn is_successful(&self) -> bool {
        match *self {
            ProofResult::Timeout | ProofResult::Error(_) => false,
            _ => true
        }
    }
    
    /// Is the current result an error?
    pub fn is_err(&self) -> bool {
        match *self {
            ProofResult::Error(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {}
