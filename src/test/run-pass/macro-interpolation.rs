// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[feature(macro_rules)];

macro_rules! overly_complicated (
    ($fnname:ident, $arg:ident, $ty:ty, $body:block, $val:expr, $pat:pat, $res:path) =>
    ({
        fn $fnname($arg: $ty) -> Option<$ty> $body
        match $fnname($val) {
          Some($pat) => {
            $res
          }
          _ => { fail!(); }
        }
    })

)
pub fn main() {
    assert!(overly_complicated!(f, x, Option<uint>, { return Some(x); },
                               Some(8u), Some(y), y) == 8u)

}
