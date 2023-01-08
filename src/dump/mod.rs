use rayon::prelude::*;

pub mod page;
pub use page::{Page, PageIterator, Revision};


#[derive(Debug)]
pub struct Dump {
    path: std::path::PathBuf
}

impl Dump {
    /// Create a [`Dump`] from a path to a `.7z` file.
    pub fn new(path: std::path::PathBuf) -> Dump {
        Dump { path }
    }

    /// Return the path of the dump file.
    pub fn path(self: &Dump) -> &std::path::PathBuf {
        &self.path
    }

    /// Return an iterator over the pages stored in the dump.
    pub fn pages(self: &Dump) -> impl Iterator<Item = Page> {
        PageIterator::from_path(&self.path)
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
}


fn directory_dumps(dump_dir: &std::path::Path) -> std::io::Result<Vec<Dump>> {
    Ok(std::fs::read_dir(dump_dir)?
        .into_iter()
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext_str| ext_str == "7z")
                .unwrap_or(false)
        })
        .map(Dump::new)
        .collect())
}
