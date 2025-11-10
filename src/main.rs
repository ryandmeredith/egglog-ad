mod builtins;

use builtins::add_builtins;
use egglog::{CommandOutput, EGraph, Error, UserDefinedCommand, ast::Expr, cli};
use std::sync::Arc;

struct FSmooth;

impl UserDefinedCommand for FSmooth {
    fn update(&self, egraph: &mut EGraph, _args: &[Expr]) -> Result<Option<CommandOutput>, Error> {
        egraph.parse_and_run_program(Some("f-smooth.egg".into()), include_str!("f-smooth.egg"))?;
        Ok(None)
    }
}

fn main() -> Result<(), Error> {
    let mut eg = EGraph::default();
    eg.add_command("f-smooth".into(), Arc::new(FSmooth))?;
    add_builtins(&mut eg.parser);
    cli(eg);
    Ok(())
}
