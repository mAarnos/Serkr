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
    
}    
