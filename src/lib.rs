//! High-performance framework for reading and analyzing wiki(pedia) dump files.
//! # Description
//!

pub mod page;
pub use page::{Page, PageIterator, Revision};

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
