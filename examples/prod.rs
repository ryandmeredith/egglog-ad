use egglog_ad::{deriv::grad, dsl::D, optim::optim};
use env_logger;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::try_init()?;
    let f = grad(D::lam(1, D::var(0).prod()))?;
    let f = optim(f)?;
    f.to_dot_file("test.dot")?;
    Ok(())
}
