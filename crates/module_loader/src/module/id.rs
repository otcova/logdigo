#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ModuleId {
    pub name: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BookId {
    pub module_id: ModuleId,
    pub title: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ChapterId {
    pub book_id: BookId,
    pub title: String,
}

/// Index of `Module::blocks`
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BlockDescIndex(pub usize);
