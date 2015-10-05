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
use cnf::rename::rename;

pub fn skolemize(f: Formula) -> Formula {
    let mut n = 0;
    rename(f, &mut n)
}

/*
fn skolemize1(f: Formula, mut bound_variables: Vec<String>) -> Formula {
    match f {
        Formula::And(box p, box q) => Formula::And(box skolemize1(p, bound_variables.clone()), box skolemize1(q, bound_variables)),
        Formula::Or(box p, box q) => Formula::Or(box skolemize1(p, bound_variables.clone()), box skolemize1(q, bound_variables)),
        Formula::Forall(s, box p) => { bound_variables.push(s.clone()); Formula::Forall(s, box skolemize1(p, bound_variables)) },
        Formula::Exists(_, box p) => { unimplemented!(); p },
        _ => f,
    }
}
*/

#[cfg(test)]
mod test {
    
}    
