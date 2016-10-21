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

use prover::data_structures::term::Term;
use prover::ordering::precedence::Precedence;
use prover::ordering::weight::Weight;
use utils::hash_map::HashMap;

/// Checks if s is greater than t according to the ordering.
pub fn kbo_gt(precedence: &Precedence,
              weight: &Weight,
              only_unary_func: &Option<i64>,
              s: &Term,
              t: &Term)
              -> bool {
    if s.is_function() && t.is_function() {
        let s_weight = weight.weight(only_unary_func, s);
        let t_weight = weight.weight(only_unary_func, t);
        if s_weight > t_weight {
            variable_domination(s, t)
        } else if s_weight == t_weight {
            if kbo_precedence(precedence, only_unary_func, s, t) ||
               (s.get_id() == t.get_id() &&
                lexical_ordering(precedence, weight, only_unary_func, s, t)) {
                variable_domination(s, t)
            } else {
                false
            }
        } else {
            false
        }
    } else if s.is_function() && t.is_variable() {
        s.occurs_proper(t)
    } else {
        false
    }
}

/// Checks if s is greater than or equal to t according to the ordering.
pub fn kbo_ge(precedence: &Precedence,
              weight: &Weight,
              only_unary_func: &Option<i64>,
              s: &Term,
              t: &Term)
              -> bool {
    s == t || kbo_gt(precedence, weight, only_unary_func, s, t)
}

/// Checks if for every variable x the amount of x in s is greater than or equal to the amount in t.
fn variable_domination(s: &Term, t: &Term) -> bool {
    let mut variable_counts = HashMap::default();
    variable_count(&mut variable_counts, s, 1);
    variable_count(&mut variable_counts, t, -1);
    variable_counts.values().all(|&count| count >= 0)
}

fn variable_count(counts: &mut HashMap<i64, i64>, t: &Term, weight: i64) {
    if t.is_variable() {
        let v = counts.entry(t.get_id()).or_insert(0);
        *v += weight;
    } else {
        for x in t.iter() {
            variable_count(counts, x, weight);
        }
    }
}

fn lexical_ordering(precedence: &Precedence,
                    weight: &Weight,
                    only_unary_func: &Option<i64>,
                    s: &Term,
                    t: &Term)
                    -> bool {
    assert_eq!(s.get_id(), t.get_id());
    assert_eq!(s.get_arity(), t.get_arity());

    for i in 0..s.get_arity() {
        if kbo_gt(precedence, weight, only_unary_func, &s[i], &t[i]) {
            return true;
        } else if s[i] != t[i] {
            return false;
        }
    }

    false
}

/// Expands the precedence so that it is suitable for KBO.
/// If there is exactly one unary function in the problem, it is greater than all other functions.
fn kbo_precedence(precedence: &Precedence,
                  only_unary_func: &Option<i64>,
                  s: &Term,
                  t: &Term)
                  -> bool {
    if let Some(id) = *only_unary_func {
        if s.get_id() == id && t.get_id() != id {
            return true;
        } else if s.get_id() != id && t.get_id() == id {
            return false;
        } 
    }

    precedence.gt(s, t)
}

#[cfg(test)]
mod test {
    use super::kbo_gt;
    use prover::data_structures::term::Term;
    use prover::ordering::precedence::Precedence;
    use prover::ordering::weight::Weight;

    #[test]
    fn kbo_gt_1() {
        let precedence = Precedence::default();
        let weight = Weight::SimpleWeight;
        let only_unary_func = None;

        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        assert!(!kbo_gt(&precedence, &weight, &only_unary_func, &x, &y));
        assert!(!kbo_gt(&precedence, &weight, &only_unary_func, &y, &x));
    }

    #[test]
    fn kbo_gt_2() {
        let precedence = Precedence::default();
        let weight = Weight::SimpleWeight;
        let only_unary_func = None;

        let x = Term::new_variable(-1);
        let f_x = Term::new_function(1, vec![x.clone()]);
        let f_f_x = Term::new_function(1, vec![f_x.clone()]);

        assert!(kbo_gt(&precedence, &weight, &only_unary_func, &f_f_x, &f_x));
        assert!(kbo_gt(&precedence, &weight, &only_unary_func, &f_x, &x));
        assert!(kbo_gt(&precedence, &weight, &only_unary_func, &f_f_x, &x));
        assert!(!kbo_gt(&precedence, &weight, &only_unary_func, &x, &f_f_x));
        assert!(!kbo_gt(&precedence, &weight, &only_unary_func, &x, &f_x));
        assert!(!kbo_gt(&precedence, &weight, &only_unary_func, &f_x, &f_f_x));
    }

    #[test]
    fn kbo_gt_3() {
        let precedence = Precedence::default();
        let weight = Weight::SimpleWeight;
        let only_unary_func = Some(1);

        let x = Term::new_variable(-1);
        let f_x = Term::new_function(1, vec![x.clone()]);
        let f_f_x = Term::new_function(1, vec![f_x.clone()]);
        let f_f_f_x = Term::new_function(1, vec![f_f_x]);

        assert!(kbo_gt(&precedence, &weight, &only_unary_func, &f_f_f_x, &f_x));
        assert!(!kbo_gt(&precedence, &weight, &only_unary_func, &f_x, &f_f_f_x));
    }
}
