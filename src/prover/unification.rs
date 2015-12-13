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

use prover::term::Term;
use prover::substitution::Substitution;

/// Tries to find a substitution so that s and t are equal.
fn unify(s: &Term, t: &Term) -> Option<Substitution> {
    let mut env = Substitution::new();
    let mut eqs = vec!((s.clone(), t.clone()));

    while let Some((eq1, eq2)) = eqs.pop() {
        if eq1 == eq2 {
            continue; // delete
        } 
        
        if eq1.is_function() && eq2.is_function() {
            if eq1.get_id() == eq2.get_id() {
                // decompose  
                for eq in eq1.into_iter().zip(eq2.into_iter()) {
                    eqs.push(eq);
                }
            } else {
                return None; // conflict
            }
        } else if eq1.is_function() {
            // swap
            eqs.push((eq2, eq1));
        } else {
            if eq2.occurs(&eq1) {
                return None; // check
            } else {
                // Can't unify between two different sorts.
                if eq2.get_sort_predicate() {
                    return None;
                }
                
                // eliminate
                // We soon add a mapping of the form eq1 |-> eq2.
                // We might have mappings of the form x |-> eq1 which need to be fixed to x |-> eq2.
                for (_, v) in env.iter_mut() {
                    v.subst_single(&eq1, &eq2);
                }
                
                // Then eliminate all occurences of eq1.
                for eq in &mut eqs {
                    eq.0.subst_single(&eq1, &eq2);
                    eq.1.subst_single(&eq1, &eq2);
                }
                
                // And finally add the new mapping.
                env.insert(eq1, eq2);
            }
        }
    }
    
    Some(env)
}

/// Tries to find the most general unifier of two terms.
pub fn mgu(s: &Term, t: &Term) -> Option<Substitution> {
    unify(s, t)
}

#[cfg(test)]
mod test {
    use super::mgu;
    use prover::term::Term;
    
    #[test]
    fn mgu_1() {
        // x = f_p()
        let t1 = Term::new(-1, false, Vec::new());
        let t2 = Term::new(1, true, Vec::new());
        assert!(mgu(&t1, &t2).is_none());
    }
    
    #[test]
    fn mgu_2() {
        // f(x, g(y)) = f(g(z), w)
        let x = Term::new(-1, false, Vec::new());
        let y = Term::new(-2, false, Vec::new());
        let z = Term::new(-3, false, Vec::new());
        let w = Term::new(-4, false, Vec::new());   
        let g_y = Term::new(2, false, vec!(y.clone()));
        let g_z = Term::new(2, false, vec!(z.clone()));
        
        let t1 = Term::new(1, false, vec!(x.clone(), g_y.clone()));
        let t2 = Term::new(1, false, vec!(g_z.clone(), w.clone()));
        let theta = mgu(&t1, &t2).unwrap();
        assert_eq!(theta.size(), 2);
        assert_eq!(theta.get(&x), Some(&g_z));
        assert_eq!(theta.get(&w), Some(&g_y));
    }

    #[test]
    fn mgu_3() {
        // f(x, y) = f(y, x)
        let x = Term::new(-1, false, Vec::new());
        let y = Term::new(-2, false, Vec::new());         
        let t1 = Term::new(1, false, vec!(x.clone(), y.clone()));
        let t2 = Term::new(1, false, vec!(y.clone(), x.clone()));
        
        let theta = mgu(&t1, &t2).unwrap();
        assert_eq!(theta.size(), 1);
        assert!(theta.get(&y) == Some(&x) || theta.get(&x) == Some(&y));
    }
    
    #[test]
    fn mgu_4() {
        // In HOL this would work.
        // f(x) = g(x)
        let x = Term::new(-1, false, Vec::new());
        let t1 = Term::new(1, false, vec!(x.clone()));
        let t2 = Term::new(2, false, vec!(x));
        assert!(mgu(&t1, &t2).is_none());
    }
    
    #[test]
    fn mgu_5() {
        // f(y) = f(g(y))
        let y = Term::new(-1, false, Vec::new());
        let t1 = Term::new(1, false, vec!(y.clone()));
        let t2 = Term::new(1, false, vec!(Term::new(2, false, vec!(y))));
        assert!(mgu(&t1, &t2).is_none());
    }
    
    #[test]
    fn mgu_6() {
        // f(x) = f(g(y))
        let x = Term::new(-1, false, Vec::new());
        let y = Term::new(-2, false, Vec::new());
        let t1 = Term::new(1, false, vec!(x));
        let t2 = Term::new(1, false, vec!(Term::new(2, false, vec!(y))));
        assert!(mgu(&t1, &t2).is_some());
        assert!(mgu(&t2, &t1).is_some());
    }
    
    #[test]
    fn mgu_7() {
        // f(y, y) = f(g(x), x)
        let x = Term::new(-1, false, Vec::new());
        let y = Term::new(-2, false, Vec::new());
        let t1 = Term::new(1, false, vec!(x.clone(), x));
        let t2 = Term::new(1, false, vec!(Term::new(2, false, vec!(y.clone())), y));
        assert!(mgu(&t1, &t2).is_none());
    }
    
    #[test]
    fn mgu_8() {
        // f(x, g(x)) = f(c, y)
        let x = Term::new(-1, false, Vec::new());
        let y = Term::new(-2, false, Vec::new());
        let c = Term::new(1, false, Vec::new());
        let t1 = Term::new(2, false, vec!(x.clone(), Term::new(3, false, vec!(x.clone()))));
        let t2 = Term::new(2, false, vec!(c.clone(), y.clone()));
        let theta = mgu(&t1, &t2).unwrap();
        assert_eq!(theta.size(), 2);
        assert_eq!(theta.get(&x), Some(&c));
        assert_eq!(theta.get(&y), Some(&Term::new(3, false, vec!(c))));
    }
}    
