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

 use utils::formula::Formula;

/// Distributes ORs inwards over ANDs.
pub fn distribute_ors_over_ands(f: Formula) -> Formula {
    match f {
        Formula::And(p, q) => Formula::And(Box::new(distribute_ors_over_ands(*p)), Box::new(distribute_ors_over_ands(*q))),
        Formula::Or(p, q) => distribute_or(*p, *q),
        _ => f,
    }
}

fn distribute_or(f1: Formula, f2: Formula) -> Formula {
    let new_f1 = distribute_ors_over_ands(f1);
    let new_f2 = distribute_ors_over_ands(f2);
    match (new_f1, new_f2) {
        (p, Formula::And(q, r)) | 
        (Formula::And(q, r), p) => Formula::And(Box::new(distribute_ors_over_ands(Formula::Or(Box::new(p.clone()), q))),
                                                Box::new(distribute_ors_over_ands(Formula::Or(Box::new(p), r)))),
        (p, q) => Formula::Or(Box::new(p), Box::new(q))
    }
}

#[cfg(test)]
mod test {
    use super::distribute_ors_over_ands;
    use parser::internal_parser::parse;
    
    #[test]
    fn distribute_1() {
        let f = parse("((Animal(f(x)) /\\ ~Loves(x, f(x))) \\/ Loves(g(x), x))").unwrap();
        let correct_f = parse("((Loves(g(x), x) \\/ Animal(f(x))) /\\ (Loves(g(x), x) \\/ ~Loves(x, f(x))))").unwrap();
        assert_eq!(distribute_ors_over_ands(f), correct_f);
    }
    
    #[test]
    fn distribute_2() {
        let f = parse("((P /\\ Q) \\/ (R /\\ S))").unwrap();
        let correct_f = parse("(((R \\/ P) /\\ (R \\/ Q)) /\\ ((S \\/ P) /\\ (S \\/ Q)))").unwrap();
        assert_eq!(distribute_ors_over_ands(f), correct_f);
    }
    
    #[test]
    fn distribute_3() {
        let f = parse("(Stays(x) /\\ (~HeavyItem(x) \\/ ~ExpensiveItem(x)))").unwrap();
        assert_eq!(distribute_ors_over_ands(f.clone()), f);
    }
    
    #[test]
    fn distribute_4() {
        let f = parse("(~Q \\/ (~P \\/ (P /\\ ~Q)))").unwrap();
        let correct_f = parse("((~Q \\/ (~P \\/ P)) /\\ (~Q \\/ (~P \\/ ~Q)))").unwrap();
        assert_eq!(distribute_ors_over_ands(f), correct_f);
    }
}    
