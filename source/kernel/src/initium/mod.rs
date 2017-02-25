//! # Initium parser module

mod tag;
mod memory_map;
mod elf_sections;
mod core_information;

pub mod initium;

pub use self::initium::load;
