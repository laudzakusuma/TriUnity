// 1. Declare the 'blocks.rs' file as a public module.
pub mod blocks;
// You likely also want to declare your other files in this directory.
pub mod database;

// 2. Re-export all public items from the 'blocks' module.
pub use blocks::*;
