use std::collections::BTreeMap;
use std::ops::Index;
use std::sync::Arc;
use parking_lot::RwLock;
use crate::data::log_record::LogRecordPos;
use crate::index::Indexer;

pub struct BTree {
    tree: Arc<RwLock<BTreeMap<Vec<u8>, LogRecordPos>>>,
}

impl BTree {
    pub fn new() -> Self {
        Self {
            tree: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}

impl Indexer for BTree {
    fn put(&self, key: &Vec<u8>, pos: &LogRecordPos) -> bool {
        let mut write_guard = self.tree.write();
        write_guard.insert(key.clone(), pos.clone());
        true
    }

    fn get(&self, key: &Vec<u8>) -> Option<LogRecordPos> {
        let read_guard = self.tree.read();
        read_guard.get(key).copied()
    }

    fn delete(&self, key: &Vec<u8>) -> bool {
        let mut write_guard = self.tree.write();
        let remove_res = write_guard.remove(key);
        remove_res.is_some()
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_btree_put() {
        let bt = BTree::new();
        let key = "".as_bytes().to_vec();
        let pos = LogRecordPos{ file_id: 1, offset: 10 };
        let res = bt.put(&key, &pos);
        assert_eq!(res, true);
    }

    #[test]
    fn test_btree_get() {
        let bt = BTree::new();
        let key = "".as_bytes().to_vec();
        let pos = LogRecordPos{ file_id: 1, offset: 10 };
        bt.put(&key, &pos);
        let res = bt.get(&key).expect("BTree获取value失败");
        assert_eq!(res, pos);
    }

    #[test]
    fn test_btree_remove() {
        let bt = BTree::new();
        let key = "".as_bytes().to_vec();
        let pos = LogRecordPos{ file_id: 1, offset: 10 };
        bt.put(&key, &pos);
        bt.delete(&key);
        let res = bt.get(&key);
        assert_eq!(res, Option::None);
    }
}
