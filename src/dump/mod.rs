pub mod page;
pub use page::{Page, PageIterator, Revision};

pub struct DumpStore {
    dump_dir: std::path::PathBuf,
}

impl DumpStore {
    /// Create a [`DumpStore`] from a [`std::path::Path`] containing `.7z` files.
    pub fn new(dump_dir: std::path::PathBuf) -> DumpStore {
        DumpStore { dump_dir }
    }

    /// Return an iterator over the paths to dump files in the [`DumpStore`].
    pub fn dump_paths(
        self: &DumpStore,
    ) -> std::io::Result<impl Iterator<Item = std::path::PathBuf>> {
        Ok(std::fs::read_dir(&self.dump_dir)?
            .into_iter()
            .filter_map(|res| res.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext_str| ext_str == "7z")
                    .unwrap_or(false)
            }))
    }
}
