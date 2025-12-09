use egglog_ad::{deriv::grad, dsl::Arg};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = grad(Arg::sum)?;
    f.to_svg_file("test.svg")
}
