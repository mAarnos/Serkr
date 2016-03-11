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

use cnf::ast::Formula;

/// Distributes ORs inwards over ANDs.
pub fn distribute_ors_over_ands(f: Formula) -> Formula {
    match f {
        Formula::And(l) => {
            Formula::And(l.into_iter()
                          .map(distribute_ors_over_ands)
                          .collect())
        }
        Formula::Or(l) => distribute_or(l),
        _ => f,
    }
}

fn distribute_or(l: Vec<Formula>) -> Formula {
    let mut distributed_l = l.into_iter()
                             .map(distribute_ors_over_ands)
                             .collect::<Vec<_>>();

    if let Some(i) = distributed_l.iter().position(|x| {
        match *x {
            Formula::And(_) => true,
            _ => false,
        }
    }) {
        let and_f = distributed_l.swap_remove(i);
        if let Formula::And(and_l) = and_f {
            Formula::And(and_l.into_iter()
                              .map(|x| {
                                  let mut rest = distributed_l.clone();
                                  rest.push(x);
                                  Formula::Or(rest)
                              })
                              .map(distribute_ors_over_ands)
                              .collect())
        } else {
            panic!("Should not be possible, see above if condition");
        }
    } else {
        Formula::Or(distributed_l)
    }
}

#[cfg(test)]
mod test {
}
