use egglog_ad::{deriv::grad, dsl::D, optim::optim};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = grad(D::lam(1, D::var(0).sum()))?;
    let f = optim(f)?;
    f.to_dot_file("test.dot")
}
