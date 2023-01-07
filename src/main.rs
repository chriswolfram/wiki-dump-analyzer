mod dump;
use dump::{DumpStore, Page, PageIterator, Revision};

fn main() {
    let dump_store = DumpStore::new(std::path::PathBuf::from(
        "/home/christopher/Documents/wikipediaData/rawDumps",
    ));
    for p in dump_store.dump_paths().expect("Cannot open dump directory") {
        println!("Path: {:?}", p);
    }
}
