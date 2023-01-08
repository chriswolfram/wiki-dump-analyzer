//! High-performance framework for reading and analyzing wiki(pedia) dump files.
//! # Description
//! Periodic dumps of wikipedia are available from [Wikimedia](https://www.wikimedia.org/), an example
//! of which can be seen [here](https://dumps.wikimedia.org/enwiki/20230101/). This crate is designed
//! to process the most complete dumps available, containing the complete edit history of all pages on a wiki.
//! (For this reason, the `.7z` versions of the dumps are used because they are far more efficient.)
//! ## Dependency
//! This crate currently assumes that the `7z` command line application is installed and available to be
//! called with [`std::process::Command::new`].

pub mod dump;
pub use dump::{Dump, DumpStore, Page, PageIterator, Revision};

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
