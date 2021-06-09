// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Point at the captured immutable outer variable

fn foo(mut f: Box<FnMut()>) {
    f();
}

fn main() {
    let y = true;
    foo(Box::new(move || y = false) as Box<_>); //~ ERROR cannot assign to captured outer variable
}
