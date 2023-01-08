use rayon::prelude::*;

fn main() {
    let dump_store = wiki_dump_analyzer::DumpStore::new(std::path::PathBuf::from(
        "/home/christopher/Documents/wikipediaData/rawDumps",
    ))
    .expect("Failed to create dump store.");

    // Print dumps in dump_store
    for d in dump_store.dumps() {
        println!("Dump: {:?}", d);
    }

    // Get a path from the dump_store
    // let path = &dump_store.dump_paths().expect("Cannot open dump directory")[0];

    // Read from path
    // let pages = wiki_dump_analyzer::PageIterator::from_path(&path);
    // let pages = dump_store.pages();
    for p in dump_store.pages().take(10) {
        println!("Page: {:?}", p);
    }

    println!("Target page: {:?}", dump_store.page_by_id(4648));

    // dump_store.par_pages().for_each(|p| {
    //     println!("Page: {:?}", p);
    // });
}
