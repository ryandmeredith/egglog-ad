use egglog_ad::{deriv::grad, dsl::Arg, optim::optim};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = grad(Arg::sum)?;
    let f = optim(f)?;
    f.to_svg_file("test.svg")
}
