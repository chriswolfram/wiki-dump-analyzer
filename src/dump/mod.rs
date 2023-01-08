use rayon::prelude::*;

pub mod page;
pub use page::{Page, PageIterator, Revision};

// The regex for file paths that can be interpreted as dumps
const DUMP_REGEX: &str = r"[^\.]*\.xml-p([^p]+)p([^\.]+)\.7z";

#[derive(Debug)]
pub struct Dump {
    path: std::path::PathBuf,
    page_id_range: (i64, i64),
}

impl Dump {
    /// Create a [`Dump`] from a path to a `.7z` file.
    pub fn new(path: std::path::PathBuf) -> Option<Dump> {
        let file_name_str = path.file_name().and_then(|name| name.to_str());
        if let Some(file_name) = file_name_str {
            let re = regex::Regex::new(DUMP_REGEX).unwrap();
            if let Some(caps) = re.captures(file_name) {
                let page_id_range = (caps[1].parse().ok()?, caps[2].parse().ok()?);
                Some(Dump {
                    path,
                    page_id_range,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Return the path of the dump file.
    pub fn path(self: &Dump) -> &std::path::PathBuf {
        &self.path
    }

    /// Return an iterator over the pages stored in the dump.
    pub fn pages(self: &Dump) -> impl Iterator<Item = Page> {
        PageIterator::from_path(&self.path)
    }

    /// Return the range of page IDs covered by this dump.
    pub fn page_id_range(self: &Dump) -> (i64, i64) {
        self.page_id_range
    }

    /// Returns [`true`] when the dump purports to claim the specified page ID.
    /// However, the dump will need to be parsed to confirm that the page is actually present.
    pub fn contains_page_id(self: &Dump, page_id: i64) -> bool {
        let (min_id, max_id) = self.page_id_range();
        min_id <= page_id && page_id <= max_id
    }
}

#[derive(Debug)]
pub struct DumpStore {
    dump_dir: std::path::PathBuf,
    dumps: Vec<Dump>,
}

impl DumpStore {
    /// Create a [`DumpStore`] from a path to a directory containing `.7z` files.
    pub fn new(dump_dir: std::path::PathBuf) -> std::io::Result<DumpStore> {
        let dumps = directory_dumps(&dump_dir)?;
        Ok(DumpStore { dump_dir, dumps })
    }

    /// Return the path of the directory containing the dump files [`DumpStore`].
    pub fn dump_dir(self: &DumpStore) -> &std::path::PathBuf {
        &self.dump_dir
    }

    /// Return a vector containing the paths of dump files in the [`DumpStore`].
    pub fn dumps(self: &DumpStore) -> &Vec<Dump> {
        &self.dumps
    }

    /// Return an iterator over the all the pages stored in all the dumps.
    pub fn pages(self: &DumpStore) -> impl Iterator<Item = Page> + '_ {
        self.dumps().iter().flat_map(Dump::pages)
    }

    /// Return a parallel iterator over the all the pages stored in all the dumps.
    pub fn par_pages(self: &DumpStore) -> impl rayon::iter::ParallelIterator<Item = Page> + '_ {
        self.dumps().par_iter().flat_map(|d| d.pages().par_bridge())
    }

    /// Return a collection of pages with the specified page IDs. There is no guarantee that a requested
    /// page will be included, and the order of the returned vector is independent of the input vector.
    pub fn pages_by_id(self: &DumpStore, page_ids: Vec<i64>) -> Vec<Page> {
        self.dumps()
            .iter()
            .filter(|d| page_ids.iter().any(|id| d.contains_page_id(*id)))
            .flat_map(|d| d.pages())
            .filter(|p| page_ids.contains(&p.id()))
            .collect()
    }

    /// Return a [`Page`] with the specified page ID, if it can be found in the dumps.
    pub fn page_by_id(self: &DumpStore, page_id: i64) -> Option<Page> {
        self.dumps()
            .iter()
            .filter(|d| d.contains_page_id(page_id))
            .flat_map(|d| d.pages())
            .filter(|p| p.id() == page_id)
            .next()
    }
}

fn directory_dumps(dump_dir: &std::path::Path) -> std::io::Result<Vec<Dump>> {
    Ok(std::fs::read_dir(dump_dir)?
        .into_iter()
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .flat_map(Dump::new)
        .collect())
}
