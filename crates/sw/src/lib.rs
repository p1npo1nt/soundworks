pub mod ast;
pub mod parser;
pub mod runtime;
pub mod compiler;

pub use ast::{
    Instrument, Pattern, Composition, Effect,
    instrument::{DrumInstrument, SynthInstrument},
    pattern::{NotePattern, DrumPattern},
};
pub use parser::parse_file;
pub use compiler::validate;
pub use runtime::Engine;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");