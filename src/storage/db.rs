//! redb database schema.

pub struct Database;

impl Database {
    pub fn open(_path: &std::path::Path) -> anyhow::Result<Self> {
        Ok(Self)
    }
}
