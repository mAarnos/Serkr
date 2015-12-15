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

use std::default::Default;
use prover::term::Term;

/// Defines a partial orders on the function symbols.
/// Different enums represent different partial orders.
pub enum Precedence {
    ArityId,
}

impl Precedence {
    /// Checks if s has precedence over t.
    pub fn gt(&self, s: &Term, t: &Term) -> bool {
        match *self {
            Precedence::ArityId => arity_id_gt(s, t),
        }
    }
}

impl Default for Precedence {
    fn default() -> Precedence { 
        Precedence::ArityId 
    }
}

/// Orders terms based on arity. In case the two terms have the same arity we break the tie with the id number.
fn arity_id_gt(s: &Term, t: &Term) -> bool {
    if s.get_arity() == t.get_arity()  {
        s.get_id() > t.get_id()
    } else {
        s.get_arity() > t.get_arity()
    }
}

#[cfg(test)]
mod test {
    use super::Precedence;
    use prover::term::Term;
    
    #[test]
    fn arity_id_1() {
        let precedence = Precedence::ArityId;
        let x = Term::new(-1, false, Vec::new());
        let y = Term::new(-2, false, Vec::new());
        let z = Term::new(-3, false, Vec::new());
        let f_x_y = Term::new(1, false, vec!(x, y));
        let g_z = Term::new(2, false, vec!(z));
        assert!(precedence.gt(&f_x_y, &g_z));
        assert!(!precedence.gt(&g_z, &f_x_y));
    }
    
    #[test]
    fn arity_id_2() {
        let precedence = Precedence::ArityId;
        let t = Term::new_truth();
        let c1 = Term::new(1, false, Vec::new());
        let c2 = Term::new(2, false, Vec::new());
        assert!(!precedence.gt(&t, &c1));
        assert!(!precedence.gt(&c1, &c2));
        assert!(precedence.gt(&c2, &c1));
        assert!(precedence.gt(&c1, &t));
        assert!(precedence.gt(&c2, &t));
    }
} 