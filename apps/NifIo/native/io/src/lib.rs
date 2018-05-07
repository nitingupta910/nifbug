#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;

extern crate graph;

use rustler::{Encoder, Env, Error, Term};

rustler_export_nifs!("Elixir.NifIo", [("foo", 0, graph_foo)], Some(on_load));

fn on_load(_env: Env, _info: Term) -> bool {
    true
}

fn graph_foo<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    println!("graph_foo from NifIo");
    let ret: u32 = graph::foo();
    Ok(ret.encode(env))
}
