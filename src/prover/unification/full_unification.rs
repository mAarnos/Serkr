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
use prover::unification::substitution::Substitution;
use std::mem::swap;

/// Tries to find the most general unifier of s and t.
pub fn mgu(s: &Term, t: &Term) -> Option<Substitution> {
    let mut sigma = Substitution::new();
    let mut eqs = vec![(s.clone(), t.clone())];

    while let Some((mut s, mut t)) = eqs.pop() {
        if s == t {
            continue; // delete
        }

        if s.is_function() && t.is_function() {
            if s.get_id() == t.get_id() {
                assert_eq!(s.get_arity(), t.get_arity());
                // decompose
                eqs.extend(s.into_iter().zip(t.into_iter()));
            } else {
                return None; // conflict
            }
        } else {
            if s.is_function() {
                // swap
                swap(&mut s, &mut t);
            } 
            
            if t.occurs(&s) {
                return None; // check
            } else {
                // Can't unify between two different sorts.
                if t.is_special_function() {
                    return None;
                }

                // eliminate
                // We soon add a mapping of the form s |-> t.
                // We might have mappings of the form x |-> s which need to be fixed to x |-> t.
                for (_, v) in sigma.iter_mut() {
                    v.subst_single(&s, &t);
                }

                // Then eliminate all occurences of s.
                for eq in &mut eqs {
                    eq.0.subst_single(&s, &t);
                    eq.1.subst_single(&s, &t);
                }

                // And finally add the new mapping.
                sigma.insert(s.get_id(), t);
            }
        }
    }
    
    debug_assert!(unifier_ok(&sigma, s, t));

    Some(sigma)
}

/// Tests that the calculated unifier actually unifies the two terms.
fn unifier_ok(sigma: &Substitution, s: &Term, t: &Term) -> bool {
    let mut new_s = s.clone();
    let mut new_t = t.clone();
    new_s.subst(sigma);
    new_t.subst(sigma);
    new_s == new_t
}

#[cfg(test)]
mod test {
    use super::mgu;
    use prover::data_structures::term::Term;
    
    #[test]
    fn mgu_1() {
        // c = c
        let c = Term::new_constant(1);
        let sigma = mgu(&c, &c).expect("MGU should exist");
        assert_eq!(sigma.size(), 0);
    }
    
    #[test]
    fn mgu_2() {
        // c = d
        let c = Term::new_constant(1);
        let d = Term::new_constant(2);
        assert!(mgu(&c, &d).is_none());
    }
    
    #[test]
    fn mgu_3() {
        // x = x
        let x = Term::new_variable(-1);
        let sigma = mgu(&x, &x).expect("MGU should exist");
        assert_eq!(sigma.size(), 0);
    }  
        
    #[test]
    fn mgu_4() {
        // x = c
        let x = Term::new_variable(-1);
        let c = Term::new_constant(1);
        let sigma = mgu(&x, &c).expect("MGU should exist");
        assert_eq!(sigma.size(), 1);
        assert_eq!(sigma.get(&x), Some(&c));
    }

    #[test]
    fn mgu_4_sort() {
        // x = P
        let x = Term::new_variable(-1);
        let p = Term::new_special_function(1, Vec::new());
        assert!(mgu(&x, &p).is_none());
    }
    
    #[test]
    fn mgu_5() {
        // x = y
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let sigma = mgu(&x, &y).expect("MGU should exist");
        assert_eq!(sigma.size(), 1);
        assert_eq!(sigma.get(&x), Some(&y));
    } 

    #[test]
    fn mgu_6() {
        // f(c, x) = f(c, d)
        let x = Term::new_variable(-1);
        let c = Term::new_constant(1);
        let d = Term::new_constant(2);
        let f_c_x = Term::new_function(3, vec![c.clone(), x.clone()]);
        let f_c_d = Term::new_function(3, vec![c.clone(), d.clone()]);
        let sigma = mgu(&f_c_x, &f_c_d).expect("MGU should exist");
        assert_eq!(sigma.size(), 1);
        assert_eq!(sigma.get(&x), Some(&d));
    }    

    #[test]
    fn mgu_7() {
        // f(c) = g(c)
        let c = Term::new_constant(1);
        let f_c = Term::new_function(2, vec![c.clone()]);
        let g_c = Term::new_function(3, vec![c.clone()]);
        assert!(mgu(&f_c, &g_c).is_none());
    } 

    #[test]
    fn mgu_8() {
        // f(x) = f(y)
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let f_x = Term::new_function(2, vec![x.clone()]);
        let f_y = Term::new_function(2, vec![y.clone()]);
        let sigma = mgu(&f_x, &f_y).expect("MGU should exist");
        assert_eq!(sigma.size(), 1);
        assert_eq!(sigma.get(&x), Some(&y));
    }    

    #[test]
    fn mgu_9() {
        // f(x) = g(y)
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let f_x = Term::new_function(2, vec![x.clone()]);
        let g_y = Term::new_function(3, vec![y.clone()]);
        assert!(mgu(&f_x, &g_y).is_none());
    } 

    #[test]
    fn mgu_10() {
        // f(g(x)) = f(y)
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let g_x = Term::new_function(2, vec![x.clone()]);
        let f_g_x = Term::new_function(1, vec![g_x.clone()]);
        let f_y = Term::new_function(1, vec![y.clone()]);
        let sigma = mgu(&f_g_x, &f_y).expect("MGU should exist");
        assert_eq!(sigma.size(), 1);
        assert_eq!(sigma.get(&y), Some(&g_x));
    } 

    #[test]
    fn mgu_11() {
        // f(g(x), x) = f(y, c)
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let c = Term::new_constant(3);
        let g_x = Term::new_function(2, vec![x.clone()]);
        let g_c = Term::new_function(2, vec![c.clone()]);
        let f_g_x_x = Term::new_function(1, vec![g_x.clone(), x.clone()]);
        let f_y_c = Term::new_function(1, vec![y.clone(), c.clone()]);
        let sigma = mgu(&f_g_x_x, &f_y_c).expect("MGU should exist");
        assert_eq!(sigma.size(), 2);
        assert_eq!(sigma.get(&x), Some(&c));
        assert_eq!(sigma.get(&y), Some(&g_c));
    }
 
    #[test]
    fn mgu_12() {
        // f(x) = x
        let x = Term::new_variable(-1);
        let f_x = Term::new_function(1, vec![x.clone()]);
        assert!(mgu(&f_x, &x).is_none());
    }    

    #[test]
    fn mgu_13() {
        // f(x, g(y)) = f(x, z)
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let g_y = Term::new_function(2, vec![y.clone()]);

        let t1 = Term::new_function(1, vec![x.clone(), g_y.clone()]);
        let t2 = Term::new_function(1, vec![x.clone(), z.clone()]);
        let sigma = mgu(&t1, &t2).expect("MGU should exist");
        
        assert_eq!(sigma.size(), 1);
        assert_eq!(sigma.get(&z), Some(&g_y));
    }
    
    #[test]
    fn mgu_13_sort() {
        // P(x, g(x)) = P(f(y), z)
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let f_y = Term::new_function(1, vec!(y.clone()));
        let g_x = Term::new_function(2, vec![x.clone()]);
        let g_f_y = Term::new_function(2, vec![f_y.clone()]);

        let t1 = Term::new_special_function(3, vec![x.clone(), g_x.clone()]);
        let t2 = Term::new_special_function(3, vec![f_y.clone(), z.clone()]);
        let sigma = mgu(&t1, &t2).expect("MGU should exist");
        
        assert_eq!(sigma.size(), 2);
        assert_eq!(sigma.get(&x), Some(&f_y));
        assert_eq!(sigma.get(&z), Some(&g_f_y));
    }
}
