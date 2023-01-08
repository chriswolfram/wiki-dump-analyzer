use rayon::prelude::*;

fn main() {
    let dump_store = wiki_dump_analyzer::DumpStore::new(std::path::PathBuf::from(
        "/home/christopher/Documents/wikipediaData/rawDumps",
    ))
    .expect("Failed to create dump store.");

    // Print paths in dump_store
    for p in dump_store.dumps() {
        println!("Path: {:?}", p);
    }

    // Get a path from the dump_store
    // let path = &dump_store.dump_paths().expect("Cannot open dump directory")[0];

    // Read from path
    // let pages = wiki_dump_analyzer::PageIterator::from_path(&path);
    // let pages = dump_store.pages();
    for p in dump_store.pages().take(10) {
        println!("Page: {:?}", p);
    }

    dump_store.pages().take(10).par_bridge().for_each(|p| {
        println!("Page: {:?}", p);
    });

    dump_store.par_pages().for_each(|p| {
        println!("Page: {:?}", p);
    });
}
