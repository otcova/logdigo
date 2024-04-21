pub struct ModuleInfo {
    /// In case of a group of authors, author[0] should be:
    ///     - The principal creator or founder
    /// This is because "author[0]" is used as part of the primary key.
    pub author: Vec<String>,
    pub title: String,
    pub description: String,
    pub used_blocks: Vec<BlockTemplate>,
}

pub struct BookInfo {
    pub title: String,
}

pub struct ChapterInfo {
    pub title: String,
    pub allowed_blocks: Vec<BlockTemplateId>,
}
