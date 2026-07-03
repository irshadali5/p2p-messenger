// storage/db.rs
use redb::{Database, ReadableTable, TableDefinition, WritableTable};

const MESSAGES_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("messages");
const CONTACTS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("contacts");

pub struct MessageStore {
    db: Database,
}

impl MessageStore {
    pub fn open(path: &std::path::Path) -> redb::Result<Self> {
        let db = Database::create(path)?;
        // Schema initialization in write txn
        let txn = db.begin_write()?;
        txn.open_table(MESSAGES_TABLE)?;
        txn.open_table(CONTACTS_TABLE)?;
        txn.commit()?;
        Ok(Self { db })
    }

    pub fn insert_message(&self, msg: &Message) -> redb::Result<()> {
        let txn = self.db.begin_write()?;
        let mut table = txn.open_table(MESSAGES_TABLE)?;
        let key = msg.id.to_string();
        let value = postcard::to_stdvec(msg).unwrap();
        table.insert(&*key, &*value)?;
        txn.commit()
    }
}
