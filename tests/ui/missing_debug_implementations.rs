#![allow(unused)]
#![warn(clippy::missing_debug_implementations)]

struct NoDebug {}

#[derive(Debug)]
struct YesDebug {}

fn main() {
    let yes_debug = YesDebug {};
    let no_debug = NoDebug {};

    dbg!(yes_debug);
}
