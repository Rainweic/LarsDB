use crate::data::log_record::LogRecordPos;

pub trait Indexer {

    fn put(&self, key: Vec<u8>, pos: LogRecordPos) -> bool;

}