//! High-performance framework for reading and analyzing wiki(pedia) dump files.
//! # Description
//!

pub mod dump;
pub use dump::{DumpStore, Page, PageIterator, Revision};

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn print_dumps() {
//         let dump_store = DumpStore::new(std::path::PathBuf::from(
//             "/home/christopher/Documents/wikipediaData/rawDumps",
//         ));
//         for p in dump_store.dump_paths().expect("Cannot open dump directory") {
//             println!("Path: {:?}", p);
//         }
//     }
// }
