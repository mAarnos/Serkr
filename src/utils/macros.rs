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

/// Same as try! except for options.
#[macro_export]
macro_rules! get {
    ($e:expr) => (match $e {
                      Some(e) => e,
                      None => return None
                  })
}

/// Same as assert_eq! except for inequality.
#[macro_export]
macro_rules! assert_neq {
    ($left:expr , $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    panic!("assertion failed: `(left != right)` \
                           (left: `{:?}`, right: `{:?}`)", left_val, right_val)
                }
            }
        }
    })
}
