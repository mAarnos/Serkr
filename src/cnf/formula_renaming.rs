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

use std::collections::HashMap;
use num::{BigInt, Zero, One};
use num::traits::FromPrimitive;
use cnf::ast::Formula;
use cnf::free_variables::free_variables;
use cnf::renaming_info::{Polarity, RenamingInfo};

/// Tries to estimate how many clauses would be generated if this formula were turned into CNF.
fn estimate_size(cache: &mut HashMap<(Formula, bool), BigInt>,
                 f: &Formula,
                 polarity: bool)
                 -> BigInt {
    let f_p_pair = (f.clone(), polarity);
    if let Some(size) = cache.get(&f_p_pair) {
        return size.clone();
    }

    let s = match *f {
        Formula::Predicate(_, _) => One::one(),
        Formula::Not(ref p) => estimate_size(cache, p, !polarity),
        Formula::And(ref l) => estimate_size_list(cache, l, true, polarity),
        Formula::Or(ref l) => estimate_size_list(cache, l, false, polarity),
        Formula::Implies(ref p, ref q) => {
            if polarity {
                estimate_size(cache, p, false) * estimate_size(cache, q, true)
            } else {
                estimate_size(cache, p, true) + estimate_size(cache, q, false)
            }
        }
        Formula::Equivalent(ref p, ref q) => {
            if polarity {
                estimate_size(cache, p, true) * estimate_size(cache, q, false) +
                estimate_size(cache, p, false) * estimate_size(cache, q, true)
            } else {
                estimate_size(cache, p, true) * estimate_size(cache, q, true) +
                estimate_size(cache, p, false) * estimate_size(cache, q, false)
            }
        }
        Formula::Forall(_, ref p) | Formula::Exists(_, ref p) => estimate_size(cache, p, polarity),
        _ => panic!("True and false should've been eliminated already"),
    };

    if s > One::one() {
        cache.insert(f_p_pair, s.clone());
    }

    s
}

/// Convenience function for above.
fn estimate_size_list(cache: &mut HashMap<(Formula, bool), BigInt>,
                      l: &[Formula],
                      conjuction: bool,
                      polarity: bool)
                      -> BigInt {
    if (polarity && conjuction) || (!polarity && !conjuction) {
        l.iter().fold(Zero::zero(),
                      |acc, x| acc + estimate_size(cache, x, polarity))
    } else {
        l.iter().fold(One::one(), |acc, x| acc * estimate_size(cache, x, polarity))
    }
}

/// Checks if renaming the formula in the position of f decreases the amount of clauses.
/// Adjusting the limit changes how many clauses we must save until we rename.
/// With limit = 0 this is just 1, which isn't really optimal.
fn renaming_condition_fulfilled(cache: &mut HashMap<(Formula, bool), BigInt>,
                                f: &Formula,
                                a: BigInt,
                                b: BigInt,
                                polarity: Polarity,
                                limit: BigInt)
                                -> bool {
    match polarity {
        Polarity::Positive => {
            (a - BigInt::one()) * (estimate_size(cache, f, true) - BigInt::one()) >=
            BigInt::one() + limit
        }
        Polarity::Negative => {
            (b - BigInt::one()) * (estimate_size(cache, f, false) - BigInt::one()) >=
            BigInt::one() + limit
        }
        Polarity::Neutral => {
            (a - BigInt::one()) * (estimate_size(cache, f, true) - BigInt::one()) +
            (b - BigInt::one()) * (estimate_size(cache, f, false) - BigInt::one()) >=
            BigInt::one() + BigInt::one() + limit
        }
    }
}

/// Tries to rename a formula. If it does not succeed it calls rename_formula again
fn try_to_rename(cache: &mut HashMap<(Formula, bool), BigInt>,
                 ri: &mut RenamingInfo,
                 f: Formula,
                 a: BigInt,
                 b: BigInt,
                 polarity: Polarity,
                 limit: BigInt)
                 -> Formula {
    let renamed_f = rename_formulae(cache, ri, f, a.clone(), b.clone(), polarity, limit.clone());
    // Check if there already is a definition.
    if let Some(definition) = ri.get_definition(&renamed_f, polarity) {
        assert_eq!(free_variables(&definition), free_variables(&renamed_f));
        definition
    } else {
        if renaming_condition_fulfilled(cache, &renamed_f, a, b, polarity, limit) {
            ri.create_new_definition(&renamed_f, polarity)
        } else {
            renamed_f
        }
    }
}

/// Updates a and b as we go along the positions of a given formula while trying to rename stuff.
fn rename_formulae(cache: &mut HashMap<(Formula, bool), BigInt>,
                   ri: &mut RenamingInfo,
                   f: Formula,
                   a: BigInt,
                   b: BigInt,
                   polarity: Polarity,
                   limit: BigInt)
                   -> Formula {
    match f {
        Formula::Not(p) => {
            Formula::Not(Box::new(try_to_rename(cache, ri, *p, b, a, polarity.flip(), limit)))
        }
        Formula::And(l) => {
            let prod: BigInt = l.iter()
                                .fold(One::one(), |acc, x| acc * estimate_size(cache, x, false));
            Formula::And(l.into_iter()
                          .map(|x| {
                              let v = b.clone() * prod.clone() / estimate_size(cache, &x, false);
                              try_to_rename(cache, ri, x, a.clone(), v, polarity, limit.clone())
                          })
                          .collect())
        }
        Formula::Or(l) => {
            Formula::Or(l.iter()
                         .cloned()
                         .enumerate()
                         .map(|(i, x)| {
                             let v = a.clone() * product(cache, &l, i, true);
                             try_to_rename(cache, ri, x, v, b.clone(), polarity, limit.clone())
                         })
                         .collect())
        }
        Formula::Implies(p, q) => {
            let a_1 = b.clone();
            let b_1 = a.clone() * estimate_size(cache, &q, true);
            let a_2 = a * estimate_size(cache, &p, false);
            let b_2 = b;
            let new_p = try_to_rename(cache, ri, *p, a_1, b_1, polarity.flip(), limit.clone());
            let new_q = try_to_rename(cache, ri, *q, a_2, b_2, polarity, limit);
            Formula::Implies(Box::new(new_p), Box::new(new_q))
        }
        Formula::Equivalent(p, q) => {
            let a_1 = a.clone() * estimate_size(cache, &q, false) +
                      b.clone() * estimate_size(cache, &q, true);
            let b_1 = a.clone() * estimate_size(cache, &q, true) +
                      b.clone() * estimate_size(cache, &q, false);
            let a_2 = a.clone() * estimate_size(cache, &p, false) +
                      b.clone() * estimate_size(cache, &p, true);
            let b_2 = a * estimate_size(cache, &p, true) + b * estimate_size(cache, &p, false);
            let new_p = try_to_rename(cache, ri, *p, a_1, b_1, Polarity::Neutral, limit.clone());
            let new_q = try_to_rename(cache, ri, *q, a_2, b_2, Polarity::Neutral, limit);
            Formula::Equivalent(Box::new(new_p), Box::new(new_q))
        }
        Formula::Forall(id, p) => {
            Formula::Forall(id,
                            Box::new(rename_formulae(cache, ri, *p, a, b, polarity, limit)))
        }
        Formula::Exists(id, p) => {
            Formula::Exists(id,
                            Box::new(rename_formulae(cache, ri, *p, a, b, polarity, limit)))
        }
        _ => f,
    }
}

/// Convenience function for above.
fn product(cache: &mut HashMap<(Formula, bool), BigInt>,
           l: &[Formula],
           i: usize,
           polarity: bool)
           -> BigInt {
    l.iter()
     .enumerate()
     .fold(One::one(), |acc, (j, x)| {
         if j == i {
             acc
         } else {
             estimate_size(cache, x, polarity) * acc
         }
     })
}

/// Renames subformulae of f to avoid an exponential increase in generated clauses.
/// If renaming_limit is 0, we do not do any renaming.
pub fn rename_formula(f: Formula,
                      renaming_info: &mut RenamingInfo,
                      renaming_limit: u64)
                      -> Formula {
    if renaming_limit == 0 {
        return f;
    }

    // If we do not do caching formula renaming is faaaar too slow to do any good.
    let mut cache = HashMap::with_capacity(256);
    let mut renamed_formulas = vec![];
    let mut formulas_to_rename = if let Formula::And(v) = f {
        v
    } else {
        vec![f]
    };

    while let Some(f) = formulas_to_rename.pop() {
        let new_f = rename_formulae(&mut cache,
                                    renaming_info,
                                    f,
                                    One::one(),
                                    Zero::zero(),
                                    Polarity::Positive,
                                    BigInt::from_u64(renaming_limit).expect("should not happen"));
        renamed_formulas.push(new_f);
        formulas_to_rename.extend(renaming_info.clear_definitions());
    }

    if renamed_formulas.len() > 1 {
        Formula::And(renamed_formulas)
    } else {
        renamed_formulas.into_iter().next().expect("should never happen")
    }
}

#[cfg(test)]
mod test {
    // How are you even supposed to test something like this?
}
