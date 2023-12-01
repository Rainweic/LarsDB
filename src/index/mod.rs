pub mod btree;

use crate::data::log_record::LogRecordPos;


// 索引接口 index下的所有数据结构都需要实现该特征
pub trait Indexer {

    fn put(&self, key: &Vec<u8>, pos: &LogRecordPos) -> bool;

    fn get(&self, key: &Vec<u8>) -> Option<LogRecordPos>;

    fn delete(&self, key: &Vec<u8>) -> bool;

}