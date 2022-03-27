//! Final Conversion of code into an appropriate architecture

use crate::intermediate::BrainFuck;

mod c99;

pub enum Backend {
    C99,
}

pub fn compile(backend: Backend, bf: &BrainFuck,  pre: u32, post: u32) -> (String, &'static str) {
    let (compile_fn, ext) = match backend {
        Backend::C99 => (c99::compile, "c"),
    };
    (compile_fn.call((bf, pre, post)), ext)
}