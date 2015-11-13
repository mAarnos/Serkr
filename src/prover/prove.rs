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

use std::collections::HashMap;
use std::collections::BinaryHeap;
use prover::flatten_cnf::flatten_cnf;
use prover::term::Term;
use prover::literal::Literal;
use prover::clause::Clause;
use prover::literal_deletion::simplify;
use prover::tautology_deletion::trivial;
use prover::subsumption::subsumes_clause;
use prover::paramodulation::paramodulate_clauses;
use prover::equality_resolution::equality_resolution;
use prover::positive_factoring::positive_factoring;
use utils::formula::Formula;
use utils::stopwatch::Stopwatch;
use cnf::naive_cnf::cnf;
use parser::internal_parser::parse;

/// Rename a clause so that it contains no variables in common with any other clause we currently have.
fn rename_clause(cl: &mut Clause, var_cnt: &mut i64) {
    let mut var_map = HashMap::<i64, i64>::new();
    for l in cl.iter_mut() {
        l.rename_no_common(&mut var_map, var_cnt);
    }
}

fn serkr_loop(mut used: Vec<Clause>, mut unused: BinaryHeap<Clause>, mut var_cnt: i64) -> Result<bool, &'static str> {
    let mut sw = Stopwatch::new();
    let mut ms_count = 1000;
    sw.start();
    
    while let Some(mut chosen_clause) = unused.pop() {
        if sw.elapsed_ms() > ms_count {
            println!("{} seconds have elapsed, used clauses = {}, unused clauses = {}", sw.elapsed_ms() / 1000, used.len(), unused.len());
            ms_count += 1000;
        }
        
        // If we derived a contradiction we are done.
        if chosen_clause.is_empty() {
            return Ok(true);
        }
        
        if !used.iter().any(|cl| subsumes_clause(cl, &chosen_clause)) {
            // println!("Chosen clause: {:?}", clause_to_string(&chosen_clause));
            used.push(chosen_clause.clone());
            rename_clause(&mut chosen_clause, &mut var_cnt);
            
            let mut inferred_clauses = Vec::new();
            for cl in &used {
                paramodulate_clauses(&chosen_clause, cl, &mut inferred_clauses);
                paramodulate_clauses(cl, &chosen_clause, &mut inferred_clauses);
            }
            equality_resolution(&chosen_clause, &mut inferred_clauses);
            positive_factoring(&chosen_clause, &mut inferred_clauses);
            
            for x in &mut inferred_clauses {
                simplify(x);
            }
            
            // Forward subsumption.
            inferred_clauses = inferred_clauses.into_iter().filter(|cl| !unused.iter().any(|cl2| subsumes_clause(cl2, cl))).collect();
            
            // Finally add everything to the queue-
            for x in inferred_clauses.into_iter() {
                unused.push(x);
            }
        }
    }
    
    Err("No proof found.")
}

/// Attempts to prove the FOL formula passed in.
pub fn prove(s: &str) -> Result<bool, &'static str> {
    let cnf_f = cnf(Formula::Not(box parse(s).unwrap()));
    if cnf_f == Formula::False {
        Ok(true)
    } else if cnf_f == Formula::True {
        Ok(false)
    } else {
        let (flattened_cnf_f, renaming_info) = flatten_cnf(cnf_f);
        let nontrivial_flattened_cnf_f = flattened_cnf_f.into_iter()
                                                        .filter(|cl| !trivial(cl))
                                                        .map(|mut cl| { simplify(&mut cl); cl })
                                                        .collect();                                  
        serkr_loop(Vec::new(), nontrivial_flattened_cnf_f, renaming_info.var_cnt)
    }
}

fn term_to_string(t: &Term) -> String {
    if t.get_id() == 0 {
        "T".to_owned()
    } else {
        if t.is_function() {
            let mut s = format!("f_{}(", t.get_id());
            for (i, st) in t.iter().enumerate() {
                s = s + &term_to_string(st);
                if i != t.get_arity() - 1 {
                    s = s + ", ";
                }    
            }
            s + ")"
        } else {
            format!("x_{}", -t.get_id())
        }
    }    
}

fn literal_to_string(l: &Literal) -> String {
    let eqn_sign = if l.is_positive() { " = " } else { " <> " }; 
    term_to_string(l.get_lhs()) + eqn_sign + &term_to_string(l.get_rhs())
}

fn clause_to_string(cl: &Clause) -> String {
    let mut s = "{ ".to_owned();
    for (i, l) in cl.iter().enumerate() {
        s = s + &literal_to_string(l);
        if i != cl.size() - 1 {
            s = s + ", ";
        }    
    }
    s + " }"
}

#[cfg(test)]
mod test {
    use super::prove;
    
    // Contains some pelletier problems negated so that we can make sure we don't prove a theorem not actually provable.
    
    #[test]
    fn pelletier_1() {
        let result = prove("((P ==> Q) <=> (~Q ==> ~P))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_1_negated() {
        let result = prove("~((P ==> Q) <=> (~Q ==> ~P))");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_2() {
        let result = prove("(~~P <=> P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_3() {
        let result = prove("(~(P ==> Q) ==> (Q ==> P))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_4() {
        let result = prove("((~P ==> Q) <=> (~Q ==> P))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_5() {
        let result = prove("(((P \\/ Q) ==> (P \\/ R)) ==> (P \\/ (Q ==> R)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_6() {
        let result = prove("(P \\/ ~P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_7() {
        let result = prove("(P \\/ ~~~P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_8() {
        let result = prove("(((P ==> Q) ==> P) ==> P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_8_negated() {
        let result = prove("~(((P ==> Q) ==> P) ==> P)");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_9() {
        let result = prove("((((P \\/ Q) /\\ (~P \\/ Q)) /\\ (P \\/ ~Q)) ==> ~(~P \\/ ~Q))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_10() {
        let result = prove("(((((Q ==> R) /\\ (R ==> (P /\\ Q))) /\\ (P ==> (Q \\/ R)))) ==> (P <=> Q))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_11() {
        let result = prove("(P <=> P)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_12() {
        let result = prove("(((P <=> Q) <=> R) <=> (P <=> (Q <=> R)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_13() {
        let result = prove("((P \\/ (Q /\\ R)) <=> ((P \\/ Q) /\\ (P \\/ R)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_14() {
        let result = prove("((P <=> Q) <=> ((~P \\/ Q) /\\ (~Q \\/ P)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_15() {
        let result = prove("((P ==> Q) <=> (~P \\/ Q))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_16() {
        let result = prove("((P ==> Q) \\/ (Q ==> P))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_17() {
        let result = prove("(((P /\\ (Q ==> R)) ==> S) <=>
                            (((~P \\/ Q) \\/ S) /\\
                              (~P \\/ (~R \\/ S))))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_18() {
        let result = prove("exists y. forall x. (F(y) ==> F(x))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_18_negated() {
        let result = prove("~exists y. forall x. (F(y) ==> F(x))");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_19() {
        let result = prove("exists x. forall y. forall z. ((P(y) ==> Q(z)) ==> (P(x) ==> Q(x)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_20() {
        let result = prove("((forall x. forall y. exists z. forall w. ((P(x) /\\ Q(y)) ==> (R(z) /\\ S(w))))
                                   ==> ((exists x. exists y. (P(x) /\\ Q(y))) ==> exists z. R(z)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_21() {
        let result = prove("(((exists x. (P ==> F(x))) /\\ 
                                   (exists x. (F(x) ==> P))) 
                                    ==> (exists x. (P <=> F(x))))");
        assert!(result.is_ok());
    }

    #[test]
    fn pelletier_22() {
        let result = prove("((forall x. (P <=> F(x))) ==> (P <=> forall x. F(x)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_23() {
        let result = prove("((forall x. (P \\/ F(x))) <=> (P \\/ forall x. F(x)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_24() {
        let result = prove("(((((~exists x. (S(x) /\\ Q(x))) /\\
                                     (forall x. (P(x) ==> (Q(x) \\/ R(x))))) /\\
                                    ((~exists x. P(x)) ==> exists x. Q(x))) /\\
                                     (forall x. ((Q(x) \\/ R(x)) ==> S(x))))
                                      ==> exists x. (P(x) /\\ R(x)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_25() {
        let result = prove("(((((exists x. P(x)) /\\ 
                                   (forall x. (F(x) ==> (~G(x) /\\ R(x))))) /\\
                                   (forall x. (P(x) ==> (G(x) /\\ F(x))))) /\\
                                   ((forall x. (P(x) ==> Q(x))) \\/ exists x. (P(x) /\\ R(x))))
                                      ==> (exists x. (Q(x) /\\ P(x))))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_26() {
        let result = prove("((((exists x. P(x)) <=> (exists x. Q(x))) /\\
                               (forall x. forall y. ((P(x) /\\ Q(y)) ==> (R(x) <=> S(y)))))
                                 ==> ((forall x. (P(x) ==> R(x))) <=> (forall x. (Q(x) ==> S(x)))))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_27() {
        let result = prove("(((((exists x. (F(x) /\\ ~G(x))) /\\ 
                                     (forall x. (F(x) ==> H(x)))) /\\ 
                                     (forall x. ((J(x) /\\ I(x)) ==> F(x)))) /\\ 
                                     ((exists x. (H(x) /\\ ~G(x))) ==> forall x. (I(x) ==> ~H(x))))
                                     ==> (forall x. (J(x) ==> ~I(x))))");
        assert!(result.is_ok());
    }

    #[test]
    fn pelletier_28_orig() {
        let result = prove("((((forall x. P(x)) ==> forall x. Q(x)) /\\
                                    (((forall x. (Q(x) \\/ R(x))) ==> exists x. (Q(x) /\\ S(x))) /\\
                                   ((exists x. S(x)) ==> forall x. (F(x) ==> G(x))))) 
                                    ==>(forall x. ((P(x) /\\ F(x)) ==> G(x))))");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_28_errata() {
        let result = prove("(((forall x. (P(x) ==> forall x. Q(x))) /\\
                                    (((forall x. (Q(x) \\/ R(x))) ==> exists x. (Q(x) /\\ S(x))) /\\
                                   ((exists x. S(x)) ==> forall x. (F(x) ==> G(x))))) 
                                    ==>(forall x. ((P(x) /\\ F(x)) ==> G(x))))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_29() {
        let result = prove("(((exists x. F(x)) /\\ (exists x. G(x)))
                               ==> (((forall x. (F(x) ==> H(x))) /\\ forall x. (G(x) ==> J(x))) <=> forall x. forall y. ((F(x) /\\ G(y)) ==> (H(x) /\\ J(y)))))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_30() {
        let result = prove("(((forall x. ((F(x) \\/ G(x)) ==> ~H(x))) /\\ (forall x. ((G(x) ==> ~I(x)) ==> (F(x) /\\ H(x))))) ==> (forall x. I(x)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_30_negated() {
        let result = prove("~(((forall x. ((F(x) \\/ G(x)) ==> ~H(x))) /\\ (forall x. ((G(x) ==> ~I(x)) ==> (F(x) /\\ H(x))))) ==> (forall x. I(x)))");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_31() {
        let result = prove("(((~exists x. (F(x) /\\ (G(x) \\/ H(x)))) /\\ 
                                  ((exists x. (I(x) /\\ F(x))) /\\ 
                                   (forall x. (~H(x) ==> J(x))))) 
                                    ==> (exists x. (I(x) /\\ J(x))))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_32() {
        let result = prove("(((forall x. ((F(x) /\\ (G(x) \\/ H(x))) ==> I(x))) /\\ 
                                  ((forall x. (I(x) /\\ (H(x) ==> J(x)))) /\\ 
                                   (forall x. (K(x) ==> H(x))))) 
                                    ==> (forall x. ((F(x) /\\ K(x)) ==> J(x))))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_33() {
        let result = prove("((forall x. ((P(a) /\\ (P(x) ==> P(b))) ==> P(c))) <=>
                                  forall x. ((~P(a) \\/ (P(x) \\/ P(c))) /\\ 
                                             (~P(a) \\/ (~P(b) \\/ P(c)))))");
        assert!(result.is_ok());
    }

    #[test]
    fn pelletier_35() {
        let result = prove("exists x. exists y. (P(x, y) ==> forall x. forall y. P(x, y))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_35_negated() {
        let result = prove("~exists x. exists y. (P(x, y) ==> forall x. forall y. P(x, y))");
        assert!(result.is_err());
    }
    
    #[test]
    fn pelletier_36() {
        let result = prove("((((forall x. exists y. F(x, y)) /\\ 
                                    (forall x. exists y. G(x, y))) /\\ 
                                    (forall x. forall y. ((F(x, y) \\/ G(x, y)) ==> forall z. ((F(y, z) \\/ G(y, z)) ==> H(x, z)))))
                                    ==> forall x. exists y. H(x, y))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_37() {
        let result = prove("((((forall x. exists y. F(x, y)) /\\ 
                                    (forall x. exists y. G(x, y))) /\\ 
                                    (forall x. forall y. ((F(x, y) \\/ G(x, y)) ==> forall z. ((F(y, z) \\/ G(y, z)) ==> H(x, z)))))
                                    ==> forall x. exists y. H(x, y))");
        assert!(result.is_ok());
    }
    
    /*
    #[test]
    fn pelletier_38() {
        let result = prove("((forall x. ((P(a()) /\\ (P(x) ==> exists y. (P(y) /\\ R(x, y)))) ==> exists z. exists w. (P(z) /\\ (R(x, w) /\\ R(w, z))))) <=>
                                  (forall x. ((~P(a()) \\/ (P(x) \\/ exists z. exists w. (P(z) /\\ (R(x, w) /\\ R(w, z))))) /\\ 
                                              (~P(a()) \\/ (~exists y. (P(y) /\\ R(x, y)) \\/ exists z. exists w. (P(z) /\\ (R(x, w) /\\ R(w, z))))))))");
        assert!(result.is_ok());
    }
    */
    
    #[test]
    fn pelletier_39() {
        let result = prove("~exists x. forall y. (F(y, x) <=> ~F(y, y))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_40() {
        let result = prove("((exists y. forall x. (F(x, y) <=> F(x, x))) ==> ~forall x. exists y. forall z. (F(x, y) <=> ~F(z, x)))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_41() {
        let result = prove("(forall z. exists y. forall x. (F(x, y) <=> (F(x, z) /\\ ~F(x, x))) 
                                   ==> ~exists z. forall x. F(x, z))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_42() {
        let result = prove("~exists y. forall x. (F(x, y) <=> ~exists z. (F(x, z) /\\ F(z, x)))");
        assert!(result.is_ok());
    }
    
    /*
    #[test]
    fn pelletier_42_negated() {
        let result = prove("exists y. forall x. (F(x, y) <=> ~exists z. (F(x, z) /\\ F(z, x)))");
        assert!(result.is_err());
    }
    */

    /*
    #[test]
    fn pelletier_43() {
        let result = prove("((forall x. forall y. Q(x, y) <=> forall z. (F(z, x) <=> F(z, y))) 
                                   ==> (forall x. forall y. (Q(x, y) <=> Q(y, x))))");
        assert!(result.is_ok());
    }
    */

    #[test]
    fn pelletier_44() {
        let result = prove("(((forall x. (F(x) ==> (exists y. (G(y) /\\ H(x, y)) /\\ exists y. (G(y) /\\ ~H(x, y))))) /\\ 
                                    exists x. (J(x) /\\ forall y. (G(y) ==> H(x, y))))
                                    ==> exists x. (J(x) /\\ ~F(x)))");
        assert!(result.is_ok());
    }   

    #[test]
    fn pelletier_45() {
        let result = prove("((((forall x. ((F(x) /\\ forall y. ((G(y) /\\ H(x, y)) ==> J(x, y)))
                                             ==> (forall y. ((G(y) /\\ H(x, y)) ==> K(y))))) /\\ 
                                   ~exists y. (L(y) /\\ K(y))) /\\ 
                                   (exists x. ((F(x) /\\ forall y. (H(x, y) ==> L(y))) /\\
                                                (forall y. ((G(y) /\\ H(x, y)) ==> J(x, y))))))
                                    ==> exists x. (F(x) /\\ ~exists y. (G(y) /\\ H(x, y))))");
        assert!(result.is_ok());
    }
    
    /*
    #[test]
    fn pelletier_46() {
        let result = prove("((((forall x. ((F(x) /\\ forall y. ((F(y) /\\ H(y, x)) ==> G(y))) ==> G(x))) /\\
                                 ((exists x. (F(x) /\\ ~G(x))) ==> ((exists x. ((F(x) /\\ ~G(x)) /\\ 
                                                                    (forall y. (F(y) /\\ ~G(y)) ==> J(x, y))))))) /\\
                                   (forall x. forall y. (((F(x) /\\ F(y)) /\\ H(x, y)) ==> ~J(y, x))))
                                   ==> forall x. (F(x) ==> G(x)))");
        assert!(result.is_ok());
    }
    */
    
    #[test]
    fn pelletier_48() {
        let result = prove("(((a() = b() \\/ c() = d()) /\\ (a() = c() \\/ b() = d())) ==> (a() = d() \\/ b() = c()))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_49() {
        let result = prove("((exists x. exists y. forall z. (z = x \\/ z = y) /\\ ((P(a()) /\\ P(b)) /\\ ~(a() = b())))
                              ==> forall x. P(x))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn pelletier_50() {
        let result = prove("((forall x. (F(a(), x) \\/ forall y. F(x, y))) ==> exists x. forall y. F(x, y))");
        assert!(result.is_ok());
    }
    
    /*
    #[test]
    fn pelletier_51() {
        let result = prove("((exists z. exists w. forall x. forall y. (F(x, y) <=> (x = z /\\ y = w)))
                             ==> (exists z. forall x. ((exists w. forall y. (F(x, y) <=> y = w)) <=> x = z)))");
        assert!(result.is_ok());
    }
    */

    #[test]
    fn los() {
        let result = prove("(((((forall x. forall y. forall z. ((P(x, y) /\\ P(y, z)) ==> P(x, z))) /\\
                                (forall x. forall y. forall z. ((Q(x, y) /\\ Q(y, z)) ==> Q(x, z)))) /\\
                               ((forall x. forall y. Q(x, y) ==> Q(y, x)) /\\
                                (forall x. forall y. P(x, y) \\/ Q(x, y)))))
                                 ==> ((forall x. forall y. P(x, y)) \\/ (forall x. forall y. Q(x, y))))");
        assert!(result.is_ok());
    }
       
    #[test]
    fn davis_putnam() {
        let result = prove("exists x. exists y. forall z. ((F(x, y) ==> (F(y, z) /\\ F(z, z))) /\\ ((F(x, y) /\\ G(x, y)) ==> (G(x, z) /\\ G(z, z))))");
        assert!(result.is_ok());
    }
    
    #[test]
    fn djikstra() {
        let result = prove("(((forall x. f(f(x)) = f(x)) /\\ (forall x. exists y. f(y) = x)) ==> forall x. f(x) = x)");
        assert!(result.is_ok());
    }
    
    /*
    #[test]
    fn group_x_times_x_equals_1_abelian() {
        let result = prove("(((forall x. forall y. forall z. mult(x, mult(y, z)) = mult(mult(x, y), z)) /\\
                             ((forall x. mult(i(), x) = x) /\\
                              (forall x. mult(i(x), x) = i())))
                               ==> (forall x. mult(x, i()) = x))");
        assert!(result.is_ok());
    }
    */
} 
