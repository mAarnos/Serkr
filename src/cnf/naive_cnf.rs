/*
    Serkr - An automated theorem prover. Copyright (C) 2015 Mikko Aarnos.

    Serkr is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Serkr is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Serkr. If not, see <http://www.gnu.org/licenses/>.
*/

use parser::formula::{Formula};
use cnf::nnf::nnf;
use cnf::skolemize::skolemize;

/// Turns a formula into CNF.
pub fn cnf(f: Formula) -> Formula {
    let nnf_f = nnf(f);
    let skolemized_f = skolemize(nnf_f);
    skolemized_f
}

/// Drops all universal quantifiers from the start of the formula.
fn drop_universal_quantifiers(f: Formula) -> Formula {
    match f {
        Formula::Forall(_, box p) => drop_universal_quantifiers(p),
        _ => f,
    }
}

/// Distributes ORs inwards over ANDs.
fn distribute_ors_over_ands(f: Formula) -> Formula {
    match f {
        Formula::And(box p, box q) => Formula::And(box distribute_ors_over_ands(p), box distribute_ors_over_ands(q)),
        Formula::Or(box ref p, box Formula::And(box ref q, box ref r)) | 
        Formula::Or(box Formula::And(box ref q, box ref r), box ref p) => Formula::And(box distribute_ors_over_ands(Formula::Or(box p.clone(), box q.clone())),
                                                                                       box distribute_ors_over_ands(Formula::Or(box p.clone(), box r.clone()))),
        Formula::Or(box p, box q) => Formula::Or(box distribute_ors_over_ands(p), box distribute_ors_over_ands(q)),
        _ => f,
    }
}

#[cfg(test)]
mod test {
    use super::{drop_universal_quantifiers};
    use parser::formula::{Formula};
    
    #[test]
    fn drop_universal_quantifiers_1() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(drop_universal_quantifiers(Formula::Forall("x".to_string(), box Formula::Forall("y".to_string(), box pred.clone()))), pred);
    }
    
    #[test]
    fn drop_universal_quantifiers_2() {
        let pred = Formula::Predicate("P".to_string(), Vec::new());
        assert_eq!(drop_universal_quantifiers(pred.clone()), pred);
    }
}    