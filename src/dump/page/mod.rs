pub mod revision;
pub use revision::Revision;

/// A page on a wiki
pub struct Page {
    /// Page ID.
    id: i64,
    /// Namespace (on Wikipedia, 0 is for articles, 1 is talk pages, 2 is user pages, etc.)
    /// More information for Wikipedia available [here](https://en.wikipedia.org/wiki/Wikipedia:Namespace).
    namespace: i64,
    /// Page title.
    title: String,
    /// Page revisions.
    revisions: Vec<Revision>,
}

impl Page {
    /// Page ID.
    pub fn id(self: &Page) -> i64 {
        self.id
    }

    /// Namespace (on Wikipedia, 0 is for articles, 1 is talk pages, 2 is user pages, etc.)
    /// More information for Wikipedia available [here](https://en.wikipedia.org/wiki/Wikipedia:Namespace).
    pub fn namespace(self: &Page) -> i64 {
        self.namespace
    }

    /// Page title.
    pub fn title(self: &Page) -> &String {
        &self.title
    }

    /// Page revisions.
    pub fn revisions(self: &Page) -> &Vec<Revision> {
        &&self.revisions
    }
}

impl std::fmt::Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Page {{ id: {:?}, title: {:?}, namespace: {:?}, revision_count: {:?} }}",
            self.id,
            self.title,
            self.namespace,
            self.revisions.len()
        )
    }
}

pub struct PageIterator<B: std::io::BufRead> {
    xml_reader: quick_xml::Reader<B>,
}

impl<B: std::io::BufRead> PageIterator<B> {
    /// Create a [`PageIterator`] from an [`std::io::BufRead`].
    pub fn from_reader(bufreader: B) -> PageIterator<B> {
        let xml_reader = quick_xml::Reader::from_reader(bufreader);
        PageIterator { xml_reader }
    }
}

impl PageIterator<std::io::BufReader<std::process::ChildStdout>> {
    pub fn from_path(path: &std::path::Path) -> PageIterator<impl std::io::BufRead> {
        // This requires that the 7z command be installed and on the $PATH
        // TODO: Replace unwraps
        let file = std::process::Command::new("7z")
            .arg("x")
            .arg("-so")
            .arg(path)
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap()
            .stdout
            .unwrap();

        PageIterator::from_reader(std::io::BufReader::new(file))
    }
}

impl<B: std::io::BufRead> Iterator for PageIterator<B> {
    type Item = Page;

    fn next(&mut self) -> Option<Page> {
        let mut rev_iter = revision::RevisionIterator::new(&mut self.xml_reader);

        let mut revisions = Vec::new();
        for rev in &mut rev_iter {
            revisions.push(rev);
        }

        // In rare cases, revisions are not stored in the order of their timestamps. This
        // fixes those cases.
        revisions.sort_by_cached_key(|rev| rev.timestamp().timestamp());

        if rev_iter.page_id.is_none()
            || rev_iter.page_namespace.is_none()
            || rev_iter.page_title.is_none()
        {
            return None;
        }

        Some(Page {
            id: rev_iter.page_id.unwrap(),
            namespace: rev_iter.page_namespace.unwrap(),
            title: rev_iter.page_title.unwrap(),
            revisions,
        })
    }
}
