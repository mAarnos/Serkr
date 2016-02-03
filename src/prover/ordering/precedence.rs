/*
    Serkr - An automated theorem prover. Copyright (C) 2015-2016 Mikko Aarnos.

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

use std::default::Default;
use std::collections::HashMap;
use prover::term::Term;

/// Defines a partial orders on the function symbols.
/// Different enums represent different partial orders.
#[allow(dead_code)]
pub enum Precedence {
    ArityId,
    ArityFrequency(HashMap<i64, i64>),
    Frequency(HashMap<i64, i64>),
    Arity,
    Id,
}

impl Precedence {
    /// Checks if s has precedence over t.
    pub fn gt(&self, s: &Term, t: &Term) -> bool {
        assert!(s.is_function());
        assert!(t.is_function());
        
        match *self {
            Precedence::ArityId => arity_id_gt(s, t),
            Precedence::ArityFrequency(ref frequency_table) => arity_frequency_gt(frequency_table, s, t),
            Precedence::Frequency(ref frequency_table) => frequency_gt(frequency_table, s, t),
            Precedence::Arity => arity_gt(s, t),
            Precedence::Id => id_gt(s, t)
        }
    }
}

impl Default for Precedence {
    fn default() -> Precedence { 
        Precedence::ArityId 
    }
}

/// Orders function symbols first based on arity and then by ID number.
fn arity_id_gt(s: &Term, t: &Term) -> bool {
    if s.get_arity() == t.get_arity() {
        id_gt(s, t)
    } else {
        arity_gt(s, t)
    }
}

/// Orders function symbols first based on arity and then based on rarity according to some count table.
fn arity_frequency_gt(frequency_table: &HashMap<i64, i64>, s: &Term, t: &Term) -> bool {
    if s.get_arity() == t.get_arity() {
        frequency_gt(frequency_table, s, t)
    } else {
        arity_gt(s, t)
    }
}

/// Orders function symbols based on their rarity according to some count table.
fn frequency_gt(frequency_table: &HashMap<i64, i64>, s: &Term, t: &Term) -> bool {
    let s_freq = frequency_table.get(&s.get_id()).expect("Symbol not found in frequency table");
    let t_freq = frequency_table.get(&t.get_id()).expect("Symbol not found in frequency table");
    s_freq < t_freq
}

/// Orders function symbols based on their arity. 
fn arity_gt(s: &Term, t: &Term) -> bool {
    s.get_arity() > t.get_arity()
}

/// Orders function symbols based on their ID. 
fn id_gt(s: &Term, t: &Term) -> bool {
    s.get_id() > t.get_id()
}

#[cfg(test)]
mod test {
    use super::Precedence;
    use prover::term::Term;
    
    #[test]
    fn arity_id_1() {
        let precedence = Precedence::ArityId;
        let x = Term::new_variable(-1);
        let y = Term::new_variable(-2);
        let z = Term::new_variable(-3);
        let f_x_y = Term::new_function(1, vec!(x, y));
        let g_z = Term::new_function(2, vec!(z));
        assert!(precedence.gt(&f_x_y, &g_z));
        assert!(!precedence.gt(&g_z, &f_x_y));
    }
    
    #[test]
    fn arity_id_2() {
        let precedence = Precedence::ArityId;
        let t = Term::new_truth();
        let c1 = Term::new_function(1, Vec::new());
        let c2 = Term::new_function(2, Vec::new());
        assert!(!precedence.gt(&t, &c1));
        assert!(!precedence.gt(&c1, &c2));
        assert!(precedence.gt(&c2, &c1));
        assert!(precedence.gt(&c1, &t));
        assert!(precedence.gt(&c2, &t));
    }
} 