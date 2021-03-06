// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.




// -*- rust -*-
enum color {
    rgb(int, int, int),
    rgba(int, int, int, int),
    hsl(int, int, int),
}

fn process(c: color) -> int {
    let mut x: int;
    match c {
      rgb(r, _, _) => { debug!("rgb"); log(debug, r); x = r; }
      rgba(_, _, _, a) => { debug!("rgba"); log(debug, a); x = a; }
      hsl(_, s, _) => { debug!("hsl"); log(debug, s); x = s; }
    }
    return x;
}

fn main() {
    let gray: color = rgb(127, 127, 127);
    let clear: color = rgba(50, 150, 250, 0);
    let red: color = hsl(0, 255, 255);
    assert (process(gray) == 127);
    assert (process(clear) == 0);
    assert (process(red) == 255);
}
