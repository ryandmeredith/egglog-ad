use egglog_ad::{deriv::diff, dsl::D};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = diff(|v| D::ifold(|a, i| a + v.get(i), D::constant(0.), v.length()))?;
    f.to_svg_file("test.svg")
}
