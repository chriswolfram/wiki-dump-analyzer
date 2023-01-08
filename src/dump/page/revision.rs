use chrono::DateTime;
use quick_xml::events::Event;

/// A revision of a page on a wiki
pub struct Revision {
    /// Revision ID.
    id: i64,
    /// Contributor ID (if specified).
    contributor_id: Option<i64>,
    /// Contributor username (if specified). A username might not be specified because the contributor was not logged in.
    contributor_username: Option<String>,
    /// Contributor IP address (if specified). The contributor's IP address is generally only included if they were not logged in.
    contributor_ip: Option<String>,
    /// Parent revision's ID (if this revision has a parent).
    parent_id: Option<i64>,
    /// Time when the revision was created.
    timestamp: DateTime<chrono::Utc>,
    /// Data model (usually 'wikitext', but not always).
    model: String,
    /// Data format (usually 'text/x-wiki', but not always).
    format: String,
    /// The body of the revision.
    body: String,
}

impl Revision {
    /// Revision ID.
    pub fn id(self: &Revision) -> i64 {
        self.id
    }

    /// Contributor ID (if specified).
    pub fn contributor_id(self: &Revision) -> Option<i64> {
        self.contributor_id
    }

    /// Contributor username (if specified). A username might not be specified because the contributor was not logged in.
    pub fn contributor_username(self: &Revision) -> Option<&String> {
        (&self.contributor_username).as_ref()
    }

    /// Contributor IP address (if specified). The contributor's IP address is generally only included if they were not logged in.
    pub fn contributor_ip(self: &Revision) -> Option<&String> {
        (&self.contributor_ip).as_ref()
    }

    /// Parent revision's ID (if this revision has a parent).
    pub fn parent_id(self: &Revision) -> Option<i64> {
        self.parent_id
    }

    /// Time when the revision was created.
    pub fn timestamp(self: &Revision) -> &DateTime<chrono::Utc> {
        &self.timestamp
    }

    /// Data model (usually 'wikitext', but not always).
    pub fn model(self: &Revision) -> &String {
        &self.model
    }

    /// Data format (usually 'text/x-wiki', but not always).
    pub fn format(self: &Revision) -> &String {
        &self.format
    }

    /// The body of the revision.
    pub fn body(self: &Revision) -> &String {
        &self.body
    }
}

impl std::fmt::Debug for Revision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Revision {{ id: {:?}, parent_id: {:?}, contributor_id: {:?}, contributor_username: {:?}, contributor_ip: {:?}, timestamp: {:?}, model: {:?}, format: {:?}, text: \"...\" }}",
            self.id, self.parent_id, self.contributor_id, self.contributor_username, self.contributor_ip, self.timestamp, self.model, self.format)
    }
}

pub struct RevisionIterator<'a, B: std::io::BufRead> {
    xml_reader: &'a mut quick_xml::Reader<B>,
    buf: Vec<u8>,
    last_page: bool,

    pub page_id: Option<i64>,
    pub page_namespace: Option<i64>,
    pub page_title: Option<String>,
}

impl<'a, B: std::io::BufRead> RevisionIterator<'a, B> {
    pub fn new(xml_reader: &'a mut quick_xml::Reader<B>) -> RevisionIterator<'a, B> {
        RevisionIterator {
            xml_reader,
            buf: Vec::new(),
            last_page: false,
            page_id: None,
            page_namespace: None,
            page_title: None,
        }
    }
}

impl<'a, B: std::io::BufRead> Iterator for RevisionIterator<'a, B> {
    type Item = Revision;

    fn next(&mut self) -> Option<Revision> {
        loop {
            // Run until we reach the first <revision> tag
            loop {
                match &self.xml_reader.read_event(&mut self.buf) {
                    // Loop until we reach the start of a new <revision>
                    Ok(Event::Start(ref e)) if e.name() == b"revision" => break,
                    // End the iterator if we reach the EOF
                    Ok(Event::Eof) => {
                        self.last_page = true;
                        return None;
                    }
                    // Stop if we reach the end of the page
                    Ok(Event::End(ref e)) if e.name() == b"page" => return None,
                    // If we see a tag we want, set next_page_field so that we will capture it
                    Ok(Event::Start(ref e)) => match e.name() {
                        b"id" => {
                            if self.page_id.is_none() {
                                self.page_id = self
                                    .xml_reader
                                    .read_text(b"id", &mut self.buf)
                                    .expect("No page ID")
                                    .parse()
                                    .ok();
                            }
                        }
                        b"ns" => {
                            if self.page_namespace.is_none() {
                                self.page_namespace = self
                                    .xml_reader
                                    .read_text(b"ns", &mut self.buf)
                                    .expect("No namespace")
                                    .parse()
                                    .ok();
                            }
                        }
                        b"title" => {
                            if self.page_title.is_none() {
                                self.page_title = Some(
                                    self.xml_reader
                                        .read_text(b"title", &mut self.buf)
                                        .expect("No title"),
                                );
                            }
                        }
                        _ => {}
                    },
                    Ok(_) => {}
                    Err(e) => panic!("Error {:?}", e),
                };
                let _ = &self.buf.clear();
            }

            let mut id = None;
            let mut parent_id = None;
            let mut timestamp = None;
            let mut model = None;
            let mut format = None;
            let mut text = None;
            let mut contributor_id = None;
            let mut contributor_username = None;
            let mut contributor_ip = None;

            let mut in_contributor = false;

            // Run until we get the </revision> tag
            loop {
                match &self.xml_reader.read_event(&mut self.buf) {
                    Ok(Event::Start(ref e)) => match e.name() {
                        b"id" => {
                            let text = self.xml_reader.read_text(b"id", &mut self.buf);
                            if in_contributor {
                                if contributor_id.is_none() {
                                    contributor_id = text.expect("No contributor ID").parse().ok();
                                }
                            } else {
                                if id.is_none() {
                                    id = text.expect("No ID").parse().ok();
                                }
                            }
                        }
                        b"username" => {
                            if in_contributor && contributor_username.is_none() {
                                contributor_username = Some(
                                    self.xml_reader
                                        .read_text(b"username", &mut self.buf)
                                        .expect("No contributor username"),
                                );
                            }
                        }
                        b"ip" => {
                            if in_contributor && contributor_ip.is_none() {
                                contributor_ip = Some(
                                    self.xml_reader
                                        .read_text(b"ip", &mut self.buf)
                                        .expect("No contributor IP"),
                                );
                            }
                        }
                        b"parentid" => {
                            if parent_id.is_none() {
                                parent_id = self
                                    .xml_reader
                                    .read_text(b"parentid", &mut self.buf)
                                    .expect("No parent ID")
                                    .parse()
                                    .ok();
                            }
                        }
                        b"timestamp" => {
                            if timestamp.is_none() {
                                timestamp = Some(DateTime::from(
                                    DateTime::parse_from_rfc3339(
                                        &self
                                            .xml_reader
                                            .read_text(b"timestamp", &mut self.buf)
                                            .expect("No timestamp"),
                                    )
                                    .expect("Bad timestamp"),
                                ));
                            }
                        }
                        b"model" => {
                            if model.is_none() {
                                model = Some(
                                    self.xml_reader
                                        .read_text(b"model", &mut self.buf)
                                        .expect("No model"),
                                );
                            }
                        }
                        b"format" => {
                            if format.is_none() {
                                format = Some(
                                    self.xml_reader
                                        .read_text(b"format", &mut self.buf)
                                        .expect("No format"),
                                );
                            }
                        }
                        b"text" => {
                            if text.is_none() {
                                text = Some(
                                    self.xml_reader
                                        .read_text(b"text", &mut self.buf)
                                        .expect("No text"),
                                );
                            }
                        }
                        b"contributor" => in_contributor = true,
                        _ => {}
                    },
                    Ok(Event::End(ref e)) if e.name() == b"contributor" => in_contributor = false,
                    // End if this is the end of the revision
                    Ok(Event::End(ref e)) if e.name() == b"revision" => break,
                    // End the iterator if we reach the EOF (which we shouldn't ever reach in this loop)
                    Ok(Event::Eof) => {
                        self.last_page = true;
                        return None;
                    }
                    Ok(_) => {}
                    Err(e) => panic!("Error {:?}", e),
                };
                let _ = &self.buf.clear();
            }

            // If we are missing any of the mandatory fields, ignore this revision
            if id.is_none()
                || timestamp.is_none()
                || model.is_none()
                || format.is_none()
                || text.is_none()
            {
                continue;
            };

            return Some(Revision {
                id: id.unwrap(),
                parent_id,
                contributor_id,
                contributor_username,
                contributor_ip,
                timestamp: timestamp.unwrap(),
                model: model.unwrap(),
                format: format.unwrap(),
                body: text.unwrap(),
            });
        }
    }
}
