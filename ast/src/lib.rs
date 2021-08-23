pub mod meta;
pub mod opcode;

pub use meta::AstMeta;
pub use opcode::BinaryOpcode;

/// The AST items that may be in an AST tree generated by Flycatcher's parser.
pub enum Ast {}
